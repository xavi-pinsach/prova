use axum::{extract::State, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::models::VkInfo;
use crate::services::{generate_proof_hash, hash_public_inputs};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub proof: serde_json::Value,
    pub public_inputs: Option<Vec<String>>,
    pub prover: Option<String>,
    pub proof_system: Option<String>,
    /// VK identifier: can be a hash (0x...) or a prover-defined alias
    pub vk_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub valid: bool,
    pub prover: String,
    pub proof_system: String,
    pub proof_type: Option<String>,
    pub prover_version: String,
    pub proof_hash: String,
    pub public_inputs_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vk: Option<VkInfo>,
    pub verified_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn verify(
    State(state): State<AppState>,
    Json(request): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, ApiError> {
    let prover = request.prover.clone().unwrap_or_else(|| "zisk".to_string());
    let proof_system = request.proof_system.clone().unwrap_or_else(|| "zisk".to_string());

    // Currently only zisk is supported
    if prover != "zisk" {
        return Err(ApiError::UnsupportedProver(prover));
    }

    // Resolve VK if vk_id is provided
    let vk = if let Some(vk_id) = &request.vk_id {
        let resolved = state
            .vk_service
            .resolve_vk(&prover, vk_id)
            .await?;

        if resolved.is_none() {
            return Err(ApiError::VkNotFound);
        }

        let vk = resolved.unwrap();

        // Check if VK is revoked
        if vk.is_revoked() {
            return Err(ApiError::BadRequest(format!(
                "Verification key is revoked: {}",
                vk.deprecation_reason.as_deref().unwrap_or("unknown reason")
            )));
        }

        Some(vk)
    } else {
        None
    };

    let proof_bytes = serde_json::to_vec(&request.proof)
        .map_err(|e| ApiError::BadRequest(format!("Invalid proof format: {}", e)))?;

    let verify_result = state
        .verifier_client
        .verify_zisk(proof_bytes, request.public_inputs.clone())
        .await?;

    let verified_at = Utc::now();
    let proof_hash = generate_proof_hash(&request.proof, &request.public_inputs);
    let public_inputs_hash = hash_public_inputs(&request.public_inputs);

    // Get proof_type from VK if available
    let proof_type = vk.as_ref().and_then(|v| v.proof_type.clone());

    // Build VK info for response
    let vk_info = vk.as_ref().map(VkInfo::from);

    Ok(Json(VerifyResponse {
        valid: verify_result.valid,
        prover,
        proof_system,
        proof_type,
        prover_version: verify_result.prover_version,
        proof_hash,
        public_inputs_hash,
        vk: vk_info,
        verified_at: verified_at.to_rfc3339(),
        error: verify_result.error,
    }))
}
