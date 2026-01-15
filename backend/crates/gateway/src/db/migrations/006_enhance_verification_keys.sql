-- Migration 006: Enhance verification_keys table
-- Adds alias, status, deprecation tracking, and proof_type

-- Add new columns
ALTER TABLE verification_keys
  ADD COLUMN IF NOT EXISTS alias VARCHAR(100),
  ADD COLUMN IF NOT EXISTS status VARCHAR(20) NOT NULL DEFAULT 'active',
  ADD COLUMN IF NOT EXISTS deprecation_reason TEXT,
  ADD COLUMN IF NOT EXISTS proof_type VARCHAR(50);

-- Create unique index for alias lookup (only for non-null aliases)
CREATE UNIQUE INDEX IF NOT EXISTS idx_vk_prover_alias
  ON verification_keys(prover, alias) WHERE alias IS NOT NULL;

-- Create index for hash lookups
CREATE INDEX IF NOT EXISTS idx_vk_hash ON verification_keys(vk_hash);

-- Add constraint for valid status values
DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1 FROM pg_constraint WHERE conname = 'chk_vk_status'
  ) THEN
    ALTER TABLE verification_keys
      ADD CONSTRAINT chk_vk_status CHECK (status IN ('active', 'deprecated', 'revoked'));
  END IF;
END $$;

-- Migrate existing data: convert active boolean to status
UPDATE verification_keys SET status = 'deprecated' WHERE active = false AND status = 'active';
