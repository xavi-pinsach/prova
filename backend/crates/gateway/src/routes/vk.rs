use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::AuthenticatedUser;
use crate::models::{CreateVerificationKey, UpdateVerificationKey, VkListItem};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ListVksParams {
    pub prover: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListVksResponse {
    pub vks: Vec<VkListItem>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct VkDetailResponse {
    pub id: String,
    pub prover: String,
    pub version: String,
    pub proof_system: String,
    pub proof_type: Option<String>,
    pub hash: String,
    pub alias: Option<String>,
    pub status: String,
    pub deprecation_reason: Option<String>,
    pub created_at: String,
}

/// GET /v1/vks - List verification keys (public)
pub async fn list_vks(
    State(state): State<AppState>,
    Query(params): Query<ListVksParams>,
) -> Result<Json<ListVksResponse>, ApiError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);

    let (vks, total) = state
        .vk_service
        .list_vks(
            params.prover.as_deref(),
            params.status.as_deref(),
            limit,
            offset,
        )
        .await?;

    let vk_items: Vec<VkListItem> = vks.into_iter().map(VkListItem::from).collect();

    Ok(Json(ListVksResponse { vks: vk_items, total }))
}

/// GET /v1/vks/:id - Get VK by ID, hash, or alias (public)
pub async fn get_vk(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<ListVksParams>,
) -> Result<Json<VkDetailResponse>, ApiError> {
    let vk = state
        .vk_service
        .get_vk(&id, params.prover.as_deref())
        .await?
        .ok_or(ApiError::VkNotFound)?;

    Ok(Json(VkDetailResponse {
        id: vk.id.to_string(),
        prover: vk.prover,
        version: vk.version,
        proof_system: vk.proof_system,
        proof_type: vk.proof_type,
        hash: vk.vk_hash,
        alias: vk.alias,
        status: vk.status,
        deprecation_reason: vk.deprecation_reason,
        created_at: vk.created_at.to_rfc3339(),
    }))
}

/// POST /v1/vks - Create verification key (admin/prover_manager only)
pub async fn create_vk(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<CreateVerificationKey>,
) -> Result<Json<VkDetailResponse>, ApiError> {
    // Check authorization
    if !user.can_manage_vk(&request.prover) {
        return Err(ApiError::Forbidden);
    }

    let vk = state
        .vk_service
        .create_vk(request, user.user_id)
        .await
        .map_err(|e| {
            // Check for unique constraint violation
            if let ApiError::Database(ref db_err) = e {
                if db_err.to_string().contains("duplicate key") || db_err.to_string().contains("unique constraint") {
                    return ApiError::VkAlreadyExists;
                }
            }
            e
        })?;

    Ok(Json(VkDetailResponse {
        id: vk.id.to_string(),
        prover: vk.prover,
        version: vk.version,
        proof_system: vk.proof_system,
        proof_type: vk.proof_type,
        hash: vk.vk_hash,
        alias: vk.alias,
        status: vk.status,
        deprecation_reason: vk.deprecation_reason,
        created_at: vk.created_at.to_rfc3339(),
    }))
}

/// PATCH /v1/vks/:id - Update verification key (admin/prover_manager only)
pub async fn update_vk(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<String>,
    Json(request): Json<UpdateVerificationKey>,
) -> Result<Json<VkDetailResponse>, ApiError> {
    // First get the VK to check authorization
    let existing_vk = state
        .vk_service
        .get_vk(&id, None)
        .await?
        .ok_or(ApiError::VkNotFound)?;

    // Check authorization
    if !user.can_manage_vk(&existing_vk.prover) {
        return Err(ApiError::Forbidden);
    }

    let vk = state
        .vk_service
        .update_vk(existing_vk.id, request)
        .await?;

    Ok(Json(VkDetailResponse {
        id: vk.id.to_string(),
        prover: vk.prover,
        version: vk.version,
        proof_system: vk.proof_system,
        proof_type: vk.proof_type,
        hash: vk.vk_hash,
        alias: vk.alias,
        status: vk.status,
        deprecation_reason: vk.deprecation_reason,
        created_at: vk.created_at.to_rfc3339(),
    }))
}
