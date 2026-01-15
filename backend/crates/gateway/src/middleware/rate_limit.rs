use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::AppState;

const CLEANUP_THRESHOLD: usize = 10000;

#[derive(Clone)]
pub struct RateLimiter {
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
    rate: u32,
    window: Duration,
}

struct TokenBucket {
    tokens: u32,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(rate: u32, window_secs: u64) -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            rate,
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn check(&self, key: &str) -> bool {
        let now = Instant::now();

        let mut buckets = self.buckets.write().unwrap();

        if buckets.len() > CLEANUP_THRESHOLD {
            self.cleanup_expired(&mut buckets, now);
        }

        let bucket = buckets.entry(key.to_string()).or_insert(TokenBucket {
            tokens: self.rate,
            last_refill: now,
        });

        if now.duration_since(bucket.last_refill) >= self.window {
            bucket.tokens = self.rate;
            bucket.last_refill = now;
        }

        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            true
        } else {
            false
        }
    }

    fn cleanup_expired(&self, buckets: &mut HashMap<String, TokenBucket>, now: Instant) {
        let expiry = self.window * 2;
        buckets.retain(|_, bucket| now.duration_since(bucket.last_refill) < expiry);
    }
}

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let key = request
        .headers()
        .get("X-API-Key")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            request
                .headers()
                .get("X-Forwarded-For")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.split(',').next())
                .unwrap_or("unknown")
                .trim()
                .to_string()
        });

    if !state.rate_limiter.check(&key) {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}
