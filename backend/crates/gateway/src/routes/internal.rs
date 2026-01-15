use axum::{Json, extract::State, http::HeaderMap};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::AppState;
use crate::error::ApiError;
use crate::models::{AnchorResponse, CreateAnchor};

#[derive(Debug, Deserialize)]
pub struct ProvisionApiKeyRequest {
    pub user_id: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ProvisionApiKeyResponse {
    pub api_key: String,
    pub created: bool,
}

pub async fn provision_api_key(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ProvisionApiKeyRequest>,
) -> Result<Json<ProvisionApiKeyResponse>, ApiError> {
    let internal_secret = std::env::var("INTERNAL_API_SECRET").map_err(|_| ApiError::Internal)?;

    let provided_secret = headers
        .get("X-Internal-Secret")
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    if provided_secret != internal_secret {
        return Err(ApiError::Unauthorized);
    }

    let existing_key: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT ak.key_hash
        FROM api_keys ak
        JOIN users u ON ak.user_id = u.id
        WHERE u.external_id = $1 AND NOT ak.revoked
        LIMIT 1
        "#,
    )
    .bind(&request.user_id)
    .fetch_optional(&state.db)
    .await?;

    if existing_key.is_some() {
        return Err(ApiError::BadRequest(
            "User already has an API key. Revoke existing key first.".to_string(),
        ));
    }

    let user_id: Uuid = get_or_create_user(&state, &request.user_id, &request.email).await?;

    let raw_key = generate_api_key();
    let mut hasher = Sha256::new();
    hasher.update(raw_key.as_bytes());
    let key_hash = hex::encode(hasher.finalize());

    sqlx::query(
        r#"
        INSERT INTO api_keys (id, user_id, key_hash, name, created_at)
        VALUES ($1, $2, $3, $4, NOW())
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&key_hash)
    .bind("auto-provisioned")
    .execute(&state.db)
    .await?;

    Ok(Json(ProvisionApiKeyResponse {
        api_key: raw_key,
        created: true,
    }))
}

async fn get_or_create_user(
    state: &AppState,
    external_id: &str,
    email: &str,
) -> Result<Uuid, ApiError> {
    let existing: Option<Uuid> = sqlx::query_scalar("SELECT id FROM users WHERE external_id = $1")
        .bind(external_id)
        .fetch_optional(&state.db)
        .await?;

    if let Some(id) = existing {
        return Ok(id);
    }

    let new_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO users (id, external_id, email, created_at)
        VALUES ($1, $2, $3, NOW())
        "#,
    )
    .bind(new_id)
    .bind(external_id)
    .bind(email)
    .execute(&state.db)
    .await?;

    Ok(new_id)
}

fn generate_api_key() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let random_bytes: [u8; 24] = rng.random();
    format!("prova_{}", hex::encode(random_bytes))
}

// Anchor endpoint for chain clients

#[derive(Debug, Deserialize)]
pub struct CreateAnchorRequest {
    pub proof_hash: String,
    pub vk_hash: String,
    pub valid: bool,
    pub prover: String,
    pub proof_system: String,
    pub chain: String,
    pub block_number: Option<i64>,
    pub block_hash: Option<String>,
    pub block_timestamp: Option<String>,
    pub tx_hash: Option<String>,
    pub explorer_url: Option<String>,
}

/// POST /internal/anchor - Create anchor record (chain clients only)
pub async fn create_anchor(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateAnchorRequest>,
) -> Result<Json<AnchorResponse>, ApiError> {
    // Validate internal secret
    let internal_secret = std::env::var("INTERNAL_API_SECRET").map_err(|_| ApiError::Internal)?;

    let provided_secret = headers
        .get("X-Internal-Secret")
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    if provided_secret != internal_secret {
        return Err(ApiError::Unauthorized);
    }

    // Parse block timestamp if provided
    let block_timestamp: Option<DateTime<Utc>> = request
        .block_timestamp
        .as_ref()
        .and_then(|ts| DateTime::parse_from_rfc3339(ts).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let create_anchor = CreateAnchor {
        proof_hash: request.proof_hash,
        vk_hash: request.vk_hash,
        valid: request.valid,
        prover: request.prover,
        proof_system: request.proof_system,
        chain: request.chain,
        block_number: request.block_number,
        block_hash: request.block_hash,
        block_timestamp,
        tx_hash: request.tx_hash,
        explorer_url: request.explorer_url,
    };

    let anchor = state.anchor_service.create_anchor(create_anchor).await?;

    Ok(Json(AnchorResponse::from(anchor)))
}
