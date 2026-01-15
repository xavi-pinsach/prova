use axum::{
    extract::{Request, State},
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::AppState;

/// Routes that are fully public (all methods)
const PUBLIC_PATHS: &[&str] = &[
    "/health",
    "/v1/provers",
    "/internal/api-keys/provision",
];

/// Routes that are public only for GET requests
const PUBLIC_GET_PATHS: &[&str] = &[
    "/v1/vks",
];

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub role: String,
    pub managed_prover: Option<String>,
}

impl AuthenticatedUser {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    pub fn is_prover_manager(&self) -> bool {
        self.role == "prover_manager"
    }

    pub fn can_manage_vk(&self, prover: &str) -> bool {
        match self.role.as_str() {
            "admin" => true,
            "prover_manager" => self.managed_prover.as_deref() == Some(prover),
            _ => false,
        }
    }
}

#[derive(sqlx::FromRow)]
struct AuthQueryResult {
    user_id: Uuid,
    role: String,
    managed_prover: Option<String>,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path();
    let method = request.method().clone();

    if is_public_path(path, &method) {
        return Ok(next.run(request).await);
    }

    let api_key = request
        .headers()
        .get("X-API-Key")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if api_key.len() < 16 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let key_hash = hex::encode(hasher.finalize());

    let result: Option<AuthQueryResult> = sqlx::query_as(
        r#"SELECT u.id as user_id, u.role, u.managed_prover
           FROM api_keys ak
           JOIN users u ON ak.user_id = u.id
           WHERE ak.key_hash = $1 AND NOT ak.revoked"#
    )
    .bind(&key_hash)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error during auth: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let auth_result = result.ok_or(StatusCode::UNAUTHORIZED)?;

    if let Err(e) = sqlx::query("UPDATE api_keys SET last_used_at = NOW() WHERE key_hash = $1")
        .bind(&key_hash)
        .execute(&state.db)
        .await
    {
        tracing::warn!("Failed to update api key last_used_at: {}", e);
    }

    request.extensions_mut().insert(AuthenticatedUser {
        user_id: auth_result.user_id,
        role: auth_result.role,
        managed_prover: auth_result.managed_prover,
    });

    Ok(next.run(request).await)
}

fn is_public_path(path: &str, method: &Method) -> bool {
    let normalized = path.trim_end_matches('/');

    // Check fully public paths (all methods)
    for public in PUBLIC_PATHS {
        if normalized == *public || normalized.starts_with(&format!("{}/", public)) {
            return true;
        }
    }

    // Check GET-only public paths
    if *method == Method::GET {
        for public in PUBLIC_GET_PATHS {
            if normalized == *public || normalized.starts_with(&format!("{}/", public)) {
                return true;
            }
        }
    }

    false
}
