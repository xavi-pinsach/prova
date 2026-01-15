use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VkStatus {
    Active,
    Deprecated,
    Revoked,
}

impl VkStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            VkStatus::Active => "active",
            VkStatus::Deprecated => "deprecated",
            VkStatus::Revoked => "revoked",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(VkStatus::Active),
            "deprecated" => Some(VkStatus::Deprecated),
            "revoked" => Some(VkStatus::Revoked),
            _ => None,
        }
    }
}

impl std::fmt::Display for VkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VerificationKey {
    pub id: Uuid,
    pub prover: String,
    pub version: String,
    pub proof_system: String,
    pub proof_type: Option<String>,
    pub vk_hash: String,
    pub vk_data: serde_json::Value,
    pub alias: Option<String>,
    pub status: String,
    pub deprecation_reason: Option<String>,
    pub registered_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub active: bool,
}

impl VerificationKey {
    pub fn status_enum(&self) -> VkStatus {
        VkStatus::from_str(&self.status).unwrap_or(VkStatus::Active)
    }

    pub fn is_active(&self) -> bool {
        self.status_enum() == VkStatus::Active
    }

    pub fn is_deprecated(&self) -> bool {
        self.status_enum() == VkStatus::Deprecated
    }

    pub fn is_revoked(&self) -> bool {
        self.status_enum() == VkStatus::Revoked
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateVerificationKey {
    pub prover: String,
    pub version: String,
    pub proof_system: String,
    pub proof_type: Option<String>,
    pub alias: Option<String>,
    pub vk_data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVerificationKey {
    pub status: Option<VkStatus>,
    pub deprecation_reason: Option<String>,
    pub alias: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VkInfo {
    pub id: Uuid,
    pub hash: String,
    pub alias: Option<String>,
    pub status: String,
    pub deprecation_reason: Option<String>,
}

impl From<&VerificationKey> for VkInfo {
    fn from(vk: &VerificationKey) -> Self {
        VkInfo {
            id: vk.id,
            hash: vk.vk_hash.clone(),
            alias: vk.alias.clone(),
            status: vk.status.clone(),
            deprecation_reason: vk.deprecation_reason.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct VkListItem {
    pub id: Uuid,
    pub prover: String,
    pub version: String,
    pub proof_system: String,
    pub proof_type: Option<String>,
    pub hash: String,
    pub alias: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<VerificationKey> for VkListItem {
    fn from(vk: VerificationKey) -> Self {
        VkListItem {
            id: vk.id,
            prover: vk.prover,
            version: vk.version,
            proof_system: vk.proof_system,
            proof_type: vk.proof_type,
            hash: vk.vk_hash,
            alias: vk.alias,
            status: vk.status,
            created_at: vk.created_at,
        }
    }
}
