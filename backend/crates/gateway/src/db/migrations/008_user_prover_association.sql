-- Migration 008: Add prover association to users
-- Allows prover_manager role to be scoped to specific provers

ALTER TABLE users ADD COLUMN IF NOT EXISTS managed_prover VARCHAR(100);

-- Index for finding users who manage a specific prover
CREATE INDEX IF NOT EXISTS idx_users_managed_prover ON users(managed_prover);
