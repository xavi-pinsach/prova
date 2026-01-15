use crate::error::ApiError;
use crate::models::{Anchor, CreateAnchor};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AnchorService {
    pool: PgPool,
}

impl AnchorService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new anchor record
    /// Uses upsert to handle duplicate (proof_hash, chain) combinations
    pub async fn create_anchor(&self, request: CreateAnchor) -> Result<Anchor, ApiError> {
        let anchor = sqlx::query_as::<_, Anchor>(
            r#"INSERT INTO anchors
               (proof_hash, vk_hash, valid, prover, proof_system, chain,
                block_number, block_hash, block_timestamp, tx_hash, explorer_url)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               ON CONFLICT (proof_hash, chain) DO UPDATE SET
                   valid = EXCLUDED.valid,
                   block_number = EXCLUDED.block_number,
                   block_hash = EXCLUDED.block_hash,
                   block_timestamp = EXCLUDED.block_timestamp,
                   tx_hash = EXCLUDED.tx_hash,
                   explorer_url = EXCLUDED.explorer_url
               RETURNING id, proof_hash, vk_hash, valid, prover, proof_system, chain,
                         block_number, block_hash, block_timestamp, tx_hash, explorer_url, created_at"#,
        )
        .bind(&request.proof_hash)
        .bind(&request.vk_hash)
        .bind(request.valid)
        .bind(&request.prover)
        .bind(&request.proof_system)
        .bind(&request.chain)
        .bind(request.block_number)
        .bind(&request.block_hash)
        .bind(request.block_timestamp)
        .bind(&request.tx_hash)
        .bind(&request.explorer_url)
        .fetch_one(&self.pool)
        .await?;

        Ok(anchor)
    }

    /// Get anchor by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Anchor>, ApiError> {
        let anchor = sqlx::query_as::<_, Anchor>(
            r#"SELECT id, proof_hash, vk_hash, valid, prover, proof_system, chain,
                      block_number, block_hash, block_timestamp, tx_hash, explorer_url, created_at
               FROM anchors
               WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(anchor)
    }

    /// Get anchors by proof hash
    pub async fn get_by_proof_hash(&self, proof_hash: &str) -> Result<Vec<Anchor>, ApiError> {
        let anchors = sqlx::query_as::<_, Anchor>(
            r#"SELECT id, proof_hash, vk_hash, valid, prover, proof_system, chain,
                      block_number, block_hash, block_timestamp, tx_hash, explorer_url, created_at
               FROM anchors
               WHERE proof_hash = $1
               ORDER BY created_at DESC"#,
        )
        .bind(proof_hash)
        .fetch_all(&self.pool)
        .await?;

        Ok(anchors)
    }

    /// Get anchor by proof hash and chain
    pub async fn get_by_proof_hash_and_chain(
        &self,
        proof_hash: &str,
        chain: &str,
    ) -> Result<Option<Anchor>, ApiError> {
        let anchor = sqlx::query_as::<_, Anchor>(
            r#"SELECT id, proof_hash, vk_hash, valid, prover, proof_system, chain,
                      block_number, block_hash, block_timestamp, tx_hash, explorer_url, created_at
               FROM anchors
               WHERE proof_hash = $1 AND chain = $2"#,
        )
        .bind(proof_hash)
        .bind(chain)
        .fetch_optional(&self.pool)
        .await?;

        Ok(anchor)
    }
}
