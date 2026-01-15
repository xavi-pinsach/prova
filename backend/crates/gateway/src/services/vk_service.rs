use crate::error::ApiError;
use crate::models::{CreateVerificationKey, UpdateVerificationKey, VerificationKey};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct VkService {
    pool: PgPool,
}

impl VkService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Check if the given ID looks like a hash (0x prefix or 64 hex chars)
    fn is_hash(id: &str) -> bool {
        if id.starts_with("0x") {
            let hex_part = &id[2..];
            hex_part.len() == 64 && hex_part.chars().all(|c| c.is_ascii_hexdigit())
        } else {
            id.len() == 64 && id.chars().all(|c| c.is_ascii_hexdigit())
        }
    }

    /// Resolve VK by hash (0x... or 64 hex chars) or alias
    pub async fn resolve_vk(
        &self,
        prover: &str,
        vk_id: &str,
    ) -> Result<Option<VerificationKey>, ApiError> {
        if Self::is_hash(vk_id) {
            self.get_by_hash(vk_id).await
        } else {
            self.get_by_alias(prover, vk_id).await
        }
    }

    /// Get VK by hash
    pub async fn get_by_hash(&self, hash: &str) -> Result<Option<VerificationKey>, ApiError> {
        let normalized_hash = if hash.starts_with("0x") {
            hash.to_string()
        } else {
            format!("0x{}", hash)
        };

        let vk = sqlx::query_as::<_, VerificationKey>(
            r#"SELECT id, prover, version, proof_system, proof_type, vk_hash, vk_data,
                      alias, status, deprecation_reason, registered_by, created_at, active
               FROM verification_keys
               WHERE vk_hash = $1"#,
        )
        .bind(&normalized_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(vk)
    }

    /// Get VK by prover-defined alias
    pub async fn get_by_alias(
        &self,
        prover: &str,
        alias: &str,
    ) -> Result<Option<VerificationKey>, ApiError> {
        let vk = sqlx::query_as::<_, VerificationKey>(
            r#"SELECT id, prover, version, proof_system, proof_type, vk_hash, vk_data,
                      alias, status, deprecation_reason, registered_by, created_at, active
               FROM verification_keys
               WHERE prover = $1 AND alias = $2"#,
        )
        .bind(prover)
        .bind(alias)
        .fetch_optional(&self.pool)
        .await?;

        Ok(vk)
    }

    /// Get VK by UUID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<VerificationKey>, ApiError> {
        let vk = sqlx::query_as::<_, VerificationKey>(
            r#"SELECT id, prover, version, proof_system, proof_type, vk_hash, vk_data,
                      alias, status, deprecation_reason, registered_by, created_at, active
               FROM verification_keys
               WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(vk)
    }

    /// Get VK by UUID string, hash, or alias
    pub async fn get_vk(&self, id: &str, prover: Option<&str>) -> Result<Option<VerificationKey>, ApiError> {
        // Try UUID first
        if let Ok(uuid) = Uuid::parse_str(id) {
            return self.get_by_id(uuid).await;
        }

        // Try hash
        if Self::is_hash(id) {
            return self.get_by_hash(id).await;
        }

        // Try alias (requires prover)
        if let Some(p) = prover {
            return self.get_by_alias(p, id).await;
        }

        Ok(None)
    }

    /// List VKs with optional filters, returns (vks, total_count)
    pub async fn list_vks(
        &self,
        prover: Option<&str>,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VerificationKey>, usize), ApiError> {
        let (vks, total): (Vec<VerificationKey>, i64) = match (prover, status) {
            (Some(p), Some(s)) => {
                let vks = sqlx::query_as::<_, VerificationKey>(
                    r#"SELECT id, prover, version, proof_system, proof_type, vk_hash, vk_data,
                              alias, status, deprecation_reason, registered_by, created_at, active
                       FROM verification_keys
                       WHERE prover = $1 AND status = $2
                       ORDER BY created_at DESC
                       LIMIT $3 OFFSET $4"#,
                )
                .bind(p)
                .bind(s)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

                let total: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM verification_keys WHERE prover = $1 AND status = $2",
                )
                .bind(p)
                .bind(s)
                .fetch_one(&self.pool)
                .await?;

                (vks, total.0)
            }
            (Some(p), None) => {
                let vks = sqlx::query_as::<_, VerificationKey>(
                    r#"SELECT id, prover, version, proof_system, proof_type, vk_hash, vk_data,
                              alias, status, deprecation_reason, registered_by, created_at, active
                       FROM verification_keys
                       WHERE prover = $1
                       ORDER BY created_at DESC
                       LIMIT $2 OFFSET $3"#,
                )
                .bind(p)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

                let total: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM verification_keys WHERE prover = $1",
                )
                .bind(p)
                .fetch_one(&self.pool)
                .await?;

                (vks, total.0)
            }
            (None, Some(s)) => {
                let vks = sqlx::query_as::<_, VerificationKey>(
                    r#"SELECT id, prover, version, proof_system, proof_type, vk_hash, vk_data,
                              alias, status, deprecation_reason, registered_by, created_at, active
                       FROM verification_keys
                       WHERE status = $1
                       ORDER BY created_at DESC
                       LIMIT $2 OFFSET $3"#,
                )
                .bind(s)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

                let total: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM verification_keys WHERE status = $1",
                )
                .bind(s)
                .fetch_one(&self.pool)
                .await?;

                (vks, total.0)
            }
            (None, None) => {
                let vks = sqlx::query_as::<_, VerificationKey>(
                    r#"SELECT id, prover, version, proof_system, proof_type, vk_hash, vk_data,
                              alias, status, deprecation_reason, registered_by, created_at, active
                       FROM verification_keys
                       ORDER BY created_at DESC
                       LIMIT $1 OFFSET $2"#,
                )
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

                let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM verification_keys")
                    .fetch_one(&self.pool)
                    .await?;

                (vks, total.0)
            }
        };

        Ok((vks, total as usize))
    }

    /// Create a new VK
    pub async fn create_vk(
        &self,
        request: CreateVerificationKey,
        registered_by: Uuid,
    ) -> Result<VerificationKey, ApiError> {
        // Compute hash of VK data
        let vk_hash = Self::compute_vk_hash(&request.vk_data);

        let vk = sqlx::query_as::<_, VerificationKey>(
            r#"INSERT INTO verification_keys
               (prover, version, proof_system, proof_type, vk_hash, vk_data, alias, status, registered_by, active)
               VALUES ($1, $2, $3, $4, $5, $6, $7, 'active', $8, true)
               RETURNING id, prover, version, proof_system, proof_type, vk_hash, vk_data,
                         alias, status, deprecation_reason, registered_by, created_at, active"#,
        )
        .bind(&request.prover)
        .bind(&request.version)
        .bind(&request.proof_system)
        .bind(&request.proof_type)
        .bind(&vk_hash)
        .bind(&request.vk_data)
        .bind(&request.alias)
        .bind(registered_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(vk)
    }

    /// Update VK status/alias
    pub async fn update_vk(
        &self,
        id: Uuid,
        request: UpdateVerificationKey,
    ) -> Result<VerificationKey, ApiError> {
        let status = request.status.map(|s| s.as_str().to_string());

        let vk = sqlx::query_as::<_, VerificationKey>(
            r#"UPDATE verification_keys
               SET status = COALESCE($2, status),
                   deprecation_reason = COALESCE($3, deprecation_reason),
                   alias = COALESCE($4, alias),
                   active = CASE WHEN COALESCE($2, status) = 'active' THEN true ELSE false END
               WHERE id = $1
               RETURNING id, prover, version, proof_system, proof_type, vk_hash, vk_data,
                         alias, status, deprecation_reason, registered_by, created_at, active"#,
        )
        .bind(id)
        .bind(&status)
        .bind(&request.deprecation_reason)
        .bind(&request.alias)
        .fetch_one(&self.pool)
        .await?;

        Ok(vk)
    }

    /// Compute SHA256 hash of VK data
    pub fn compute_vk_hash(vk_data: &serde_json::Value) -> String {
        let mut hasher = Sha256::new();
        hasher.update(vk_data.to_string().as_bytes());
        format!("0x{}", hex::encode(hasher.finalize()))
    }
}
