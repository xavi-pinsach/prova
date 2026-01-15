use sha2::{Digest, Sha256};

/// Utility functions for proof hashing
/// Note: Proof storage has been removed. These are hash utilities only.

/// Generate a hash for a proof (used for proof_hash in responses)
pub fn generate_proof_hash(proof: &serde_json::Value, public_inputs: &Option<Vec<String>>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(proof.to_string().as_bytes());
    if let Some(inputs) = public_inputs {
        for input in inputs {
            hasher.update(input.as_bytes());
        }
    }
    format!("0x{}", hex::encode(hasher.finalize()))
}

/// Hash public inputs for the response
pub fn hash_public_inputs(public_inputs: &Option<Vec<String>>) -> Option<String> {
    public_inputs.as_ref().map(|inputs| {
        let mut hasher = Sha256::new();
        for input in inputs {
            hasher.update(input.as_bytes());
        }
        format!("0x{}", hex::encode(hasher.finalize()))
    })
}
