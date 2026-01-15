-- Create proofs table
CREATE TABLE proofs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    proof_id VARCHAR(255) UNIQUE NOT NULL,
    proof_system VARCHAR(50) NOT NULL,
    prover VARCHAR(100) NOT NULL,
    prover_version VARCHAR(50) NOT NULL,
    public_inputs_hash VARCHAR(66),
    valid BOOLEAN NOT NULL,
    verified_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_proofs_proof_id ON proofs(proof_id);
CREATE INDEX idx_proofs_prover ON proofs(prover, prover_version);
CREATE INDEX idx_proofs_verified_at ON proofs(verified_at);
CREATE INDEX idx_proofs_proof_system ON proofs(proof_system);
