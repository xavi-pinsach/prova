use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Anchor {
    pub id: Uuid,
    pub proof_hash: String,
    pub vk_hash: String,
    pub valid: bool,
    pub prover: String,
    pub proof_system: String,
    pub chain: String,
    pub block_number: Option<i64>,
    pub block_hash: Option<String>,
    pub block_timestamp: Option<DateTime<Utc>>,
    pub tx_hash: Option<String>,
    pub explorer_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAnchor {
    pub proof_hash: String,
    pub vk_hash: String,
    pub valid: bool,
    pub prover: String,
    pub proof_system: String,
    pub chain: String,
    pub block_number: Option<i64>,
    pub block_hash: Option<String>,
    pub block_timestamp: Option<DateTime<Utc>>,
    pub tx_hash: Option<String>,
    pub explorer_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AnchorResponse {
    pub id: Uuid,
    pub proof_hash: String,
    pub vk_hash: String,
    pub valid: bool,
    pub prover: String,
    pub proof_system: String,
    pub chain: ChainInfo,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ChainInfo {
    pub name: String,
    pub block_number: Option<i64>,
    pub block_hash: Option<String>,
    pub block_timestamp: Option<DateTime<Utc>>,
    pub tx_hash: Option<String>,
    pub explorer_url: Option<String>,
}

impl From<Anchor> for AnchorResponse {
    fn from(anchor: Anchor) -> Self {
        AnchorResponse {
            id: anchor.id,
            proof_hash: anchor.proof_hash,
            vk_hash: anchor.vk_hash,
            valid: anchor.valid,
            prover: anchor.prover,
            proof_system: anchor.proof_system,
            chain: ChainInfo {
                name: anchor.chain,
                block_number: anchor.block_number,
                block_hash: anchor.block_hash,
                block_timestamp: anchor.block_timestamp,
                tx_hash: anchor.tx_hash,
                explorer_url: anchor.explorer_url,
            },
            created_at: anchor.created_at,
        }
    }
}
