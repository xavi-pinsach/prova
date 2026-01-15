-- Migration 007: Create anchors table
-- Stores on-chain anchor records created by chain clients

CREATE TABLE IF NOT EXISTS anchors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    proof_hash VARCHAR(66) NOT NULL,
    vk_hash VARCHAR(66) NOT NULL,
    valid BOOLEAN NOT NULL,
    prover VARCHAR(100) NOT NULL,
    proof_system VARCHAR(50) NOT NULL,
    chain VARCHAR(50) NOT NULL,
    block_number BIGINT,
    block_hash VARCHAR(66),
    block_timestamp TIMESTAMP WITH TIME ZONE,
    tx_hash VARCHAR(66),
    explorer_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Each proof can only be anchored once per chain
    UNIQUE(proof_hash, chain)
);

-- Index for proof hash lookups
CREATE INDEX IF NOT EXISTS idx_anchors_proof_hash ON anchors(proof_hash);

-- Index for chain filtering
CREATE INDEX IF NOT EXISTS idx_anchors_chain ON anchors(chain);

-- Index for time-based queries
CREATE INDEX IF NOT EXISTS idx_anchors_created_at ON anchors(created_at);
