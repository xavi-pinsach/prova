mod config;
mod db;
mod error;
mod middleware;
mod models;
mod routes;
mod services;

use sqlx::PgPool;
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use middleware::RateLimiter;
use services::{AnchorService, VerifierClient, VkService};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub verifier_client: VerifierClient,
    pub vk_service: VkService,
    pub anchor_service: AnchorService,
    pub rate_limiter: RateLimiter,
}



#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Err(e) = run().await {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    let db_pool = db::create_pool(&config.database_url).await?;
    db::run_migrations(&db_pool).await?;
    tracing::info!("Database migrations completed");

    let verifier_client = VerifierClient::new(config.zisk_service_url.clone()).await?;

    // Health check verifier with retries
    check_verifier_health(&verifier_client).await;

    let vk_service = VkService::new(db_pool.clone());
    let anchor_service = AnchorService::new(db_pool.clone());
    let rate_limiter = RateLimiter::new(config.rate_limit_requests, config.rate_limit_window_secs);

    let state = AppState {
        db: db_pool.clone(),
        verifier_client,
        vk_service,
        anchor_service,
        rate_limiter,
    };

    let cors = CorsLayer::new()
        .allow_origin(config.cors_origins)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = routes::create_router(state, cors);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Check verifier health with retries.
/// Logs warning if unreachable but doesn't block startup.
async fn check_verifier_health(client: &VerifierClient) {
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY: Duration = Duration::from_secs(2);

    for attempt in 1..=MAX_RETRIES {
        match client.health_check().await {
            Ok(version) => {
                tracing::info!(
                    version = %version,
                    "Verifier health check passed"
                );
                return;
            }
            Err(e) => {
                if attempt < MAX_RETRIES {
                    tracing::warn!(
                        attempt = attempt,
                        max_retries = MAX_RETRIES,
                        error = %e,
                        "Verifier health check failed, retrying..."
                    );
                    tokio::time::sleep(RETRY_DELAY).await;
                } else {
                    tracing::warn!(
                        error = %e,
                        "Verifier health check failed after {} attempts. Service may be unavailable.",
                        MAX_RETRIES
                    );
                }
            }
        }
    }
}
