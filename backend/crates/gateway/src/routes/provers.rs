use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

use crate::error::ApiError;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ProverInfo {
    pub name: String,
    pub proof_systems: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ListProversResponse {
    pub provers: Vec<ProverInfo>,
}

#[derive(Debug, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub active: bool,
    pub proof_systems: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ListVersionsResponse {
    pub prover: String,
    pub versions: Vec<VersionInfo>,
}

#[derive(Debug, Serialize)]
pub struct ProofSystemInfo {
    pub name: String,
    pub vk_hash: String,
    pub active: bool,
}

#[derive(Debug, Serialize)]
pub struct VersionDetailResponse {
    pub prover: String,
    pub version: String,
    pub proof_systems: Vec<ProofSystemInfo>,
    pub registered_at: String,
}

pub async fn list_provers(
    State(_state): State<AppState>,
) -> Result<Json<ListProversResponse>, ApiError> {
    // Currently only Zisk is supported
    let provers = vec![
        ProverInfo {
            name: "zisk".to_string(),
            proof_systems: vec!["stark".to_string()],
        },
    ];

    Ok(Json(ListProversResponse { provers }))
}

#[derive(FromRow)]
struct VersionRow {
    version: String,
    active: bool,
    proof_systems: Option<Vec<String>>,
}

pub async fn list_versions(
    State(state): State<AppState>,
    Path(prover): Path<String>,
) -> Result<Json<ListVersionsResponse>, ApiError> {
    let rows = sqlx::query_as::<_, VersionRow>(
        r#"
        SELECT DISTINCT version, active, array_agg(proof_system) as proof_systems
        FROM verification_keys
        WHERE prover = $1
        GROUP BY version, active
        ORDER BY version DESC
        "#,
    )
    .bind(&prover)
    .fetch_all(&state.db)
    .await?;

    let versions: Vec<VersionInfo> = rows
        .into_iter()
        .map(|row| VersionInfo {
            version: row.version,
            active: row.active,
            proof_systems: row.proof_systems.unwrap_or_default(),
        })
        .collect();

    Ok(Json(ListVersionsResponse { prover, versions }))
}

#[derive(FromRow)]
struct VkRow {
    proof_system: String,
    vk_hash: String,
    active: bool,
    created_at: DateTime<Utc>,
}

pub async fn get_version(
    State(state): State<AppState>,
    Path((prover, version)): Path<(String, String)>,
) -> Result<Json<VersionDetailResponse>, ApiError> {
    let rows = sqlx::query_as::<_, VkRow>(
        r#"
        SELECT proof_system, vk_hash, active, created_at
        FROM verification_keys
        WHERE prover = $1 AND version = $2
        "#,
    )
    .bind(&prover)
    .bind(&version)
    .fetch_all(&state.db)
    .await?;

    if rows.is_empty() {
        return Err(ApiError::VkNotFound);
    }

    let registered_at = rows.first().map(|r| r.created_at.to_rfc3339()).unwrap_or_default();

    let proof_systems: Vec<ProofSystemInfo> = rows
        .into_iter()
        .map(|row| ProofSystemInfo {
            name: row.proof_system,
            vk_hash: row.vk_hash,
            active: row.active,
        })
        .collect();

    Ok(Json(VersionDetailResponse {
        prover,
        version,
        proof_systems,
        registered_at,
    }))
}
