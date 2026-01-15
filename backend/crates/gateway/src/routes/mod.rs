pub mod internal;
pub mod provers;
pub mod verify;
pub mod vk;

pub use internal::*;
pub use provers::*;
pub use verify::*;
pub use vk::*;

use axum::{
    Json, Router, middleware as axum_middleware,
    routing::{get, post},
};
use serde_json::{Value, json};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::{AppState, middleware};

// Health check handler
async fn health() -> Json<Value> {
    Json(json!({
        "service": "prova-gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

pub fn create_router(state: AppState, cors: CorsLayer) -> Router {
    Router::new()
        // == Public endpoints
        .route("/health", get(health))
        // Prover endpoints
        .route("/v1/provers", get(list_provers))
        .route("/v1/provers/:prover/versions", get(list_versions))
        .route("/v1/provers/:prover/:version", get(get_version))
        // Verification key endpoints
        .route("/v1/vks", get(list_vks).post(create_vk))
        .route("/v1/vks/:id", get(get_vk).patch(update_vk))
        // Verification endpoints
        .route("/v1/verify", post(verify))
        // == Internal endpoints
        .route("/internal/api-keys/provision", post(provision_api_key))
        .route("/internal/anchor", post(create_anchor))
        // == Middleware
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::rate_limit_middleware,
        ))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
