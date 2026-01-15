use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication required")]
    Unauthorized,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Forbidden")]
    Forbidden,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Proof not found")]
    ProofNotFound,

    #[error("Unsupported prover: {0}")]
    UnsupportedProver(String),

    #[error("Unsupported proof system: {0}")]
    UnsupportedProofSystem(String),

    #[error("Verification key not found")]
    VkNotFound,

    #[error("Verification key already exists")]
    VkAlreadyExists,

    #[error("Verifier service error: {0}")]
    VerifierService(String),

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message, details) = match &self {
            ApiError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error", None)
            }
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Authentication required", None),
            ApiError::InvalidApiKey => (StatusCode::UNAUTHORIZED, "Invalid API key", None),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden", None),
            ApiError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded", None),
            ApiError::ProofNotFound => (StatusCode::NOT_FOUND, "Proof not found", None),
            ApiError::UnsupportedProver(p) => (StatusCode::BAD_REQUEST, "Unsupported prover", Some(p.clone())),
            ApiError::UnsupportedProofSystem(s) => (StatusCode::BAD_REQUEST, "Unsupported proof system", Some(s.clone())),
            ApiError::VkNotFound => (StatusCode::NOT_FOUND, "Verification key not found", None),
            ApiError::VkAlreadyExists => (StatusCode::CONFLICT, "Verification key already exists", None),
            ApiError::VerifierService(e) => {
                tracing::error!("Verifier service error: {}", e);
                (StatusCode::BAD_GATEWAY, "Verifier service unavailable", None)
            }
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "Invalid request", Some(msg.clone())),
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error", None),
        };

        let body = if let Some(d) = details {
            json!({ "error": message, "details": d })
        } else {
            json!({ "error": message })
        };

        (status, Json(body)).into_response()
    }
}
