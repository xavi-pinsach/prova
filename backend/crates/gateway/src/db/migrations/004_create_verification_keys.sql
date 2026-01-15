-- Create verification keys table
CREATE TABLE verification_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    prover VARCHAR(100) NOT NULL,
    version VARCHAR(50) NOT NULL,
    proof_system VARCHAR(50) NOT NULL,
    vk_hash VARCHAR(66) NOT NULL,
    vk_data JSONB NOT NULL,
    registered_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    active BOOLEAN DEFAULT TRUE,
    UNIQUE(prover, version, proof_system)
);

CREATE INDEX idx_vk_prover_version ON verification_keys(prover, version);
CREATE INDEX idx_vk_proof_system ON verification_keys(proof_system);
CREATE INDEX idx_vk_active ON verification_keys(active);
