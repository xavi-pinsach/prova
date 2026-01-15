use std::env;
use tower_http::cors::AllowOrigin;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVar(String),
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(String, String),
}

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub zisk_service_url: String,
    pub internal_api_secret: String,
    pub host: String,
    pub port: u16,
    pub cors_origins: AllowOrigin,
    pub rate_limit_requests: u32,
    pub rate_limit_window_secs: u64,
    pub max_db_connections: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let database_url = required_var("DATABASE_URL")?;

        let cors_origins = env::var("CORS_ORIGINS")
            .map(|s| {
                let origins: Vec<_> = s.split(',').map(|s| s.trim().to_string()).collect();
                if origins.iter().any(|o| o == "*") {
                    AllowOrigin::any()
                } else {
                    AllowOrigin::list(
                        origins.into_iter().filter_map(|o| o.parse().ok())
                    )
                }
            })
            .unwrap_or_else(|_| AllowOrigin::any());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("PORT".into(), "must be a number".into()))?;

        let rate_limit_requests = env::var("RATE_LIMIT_REQUESTS")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("RATE_LIMIT_REQUESTS".into(), "must be a number".into()))?;

        let rate_limit_window_secs = env::var("RATE_LIMIT_WINDOW_SECS")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("RATE_LIMIT_WINDOW_SECS".into(), "must be a number".into()))?;

        let max_db_connections = env::var("MAX_DB_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("MAX_DB_CONNECTIONS".into(), "must be a number".into()))?;

        Ok(Self {
            database_url,
            zisk_service_url: env::var("ZISK_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:50051".to_string()),
            internal_api_secret: env::var("INTERNAL_API_SECRET")
                .unwrap_or_else(|_| "dev-internal-secret".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port,
            cors_origins,
            rate_limit_requests,
            rate_limit_window_secs,
            max_db_connections,
        })
    }
}

fn required_var(name: &str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingVar(name.into()))
}
