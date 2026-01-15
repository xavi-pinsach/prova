use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    ProverManager,
    Admin,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "user",
            UserRole::ProverManager => "prover_manager",
            UserRole::Admin => "admin",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "user" => Some(UserRole::User),
            "prover_manager" => Some(UserRole::ProverManager),
            "admin" => Some(UserRole::Admin),
            _ => None,
        }
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub managed_prover: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn role_enum(&self) -> UserRole {
        UserRole::from_str(&self.role).unwrap_or(UserRole::User)
    }

    pub fn is_admin(&self) -> bool {
        self.role_enum() == UserRole::Admin
    }

    pub fn is_prover_manager(&self) -> bool {
        self.role_enum() == UserRole::ProverManager
    }

    pub fn can_manage_vk(&self, prover: &str) -> bool {
        match self.role_enum() {
            UserRole::Admin => true,
            UserRole::ProverManager => self.managed_prover.as_deref() == Some(prover),
            UserRole::User => false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub name: Option<String>,
    pub role: Option<String>,
    pub managed_prover: Option<String>,
}
