export interface ProvaConfig {
  apiKey: string;
  /** @default 'https://api.prova.io' */
  baseUrl?: string;
  /** @default 30000 */
  timeout?: number;
}

export interface VerifyRequest {
  proof: unknown;
  publicInputs?: string[];
  prover?: string;
  proofSystem?: 'groth16' | 'plonk' | 'fflonk' | 'zisk' | string;
  /** VK identifier: hash (0x...) or alias */
  vkId?: string;
}

export interface VkInfo {
  id: string;
  hash: string;
  alias: string | null;
  status: 'active' | 'deprecated' | 'revoked';
  deprecationReason: string | null;
}

export interface VerifyResponse {
  valid: boolean;
  prover: string;
  proofSystem: string;
  proofType: string | null;
  proverVersion: string;
  proofHash: string;
  publicInputsHash: string | null;
  vk: VkInfo | null;
  verifiedAt: string;
  error?: string;
}

export interface VerificationKey {
  id: string;
  prover: string;
  version: string;
  proofSystem: string;
  proofType: string | null;
  hash: string;
  alias: string | null;
  status: 'active' | 'deprecated' | 'revoked';
  deprecationReason: string | null;
  createdAt: string;
}

export interface Prover {
  name: string;
  proofSystems: string[];
}

export interface ProverVersion {
  version: string;
  active: boolean;
  proofSystems: string[];
}

export interface ProofSystemInfo {
  name: string;
  vkHash: string;
  active: boolean;
}

// API response types (snake_case from server)
export interface ApiVkInfo {
  id: string;
  hash: string;
  alias: string | null;
  status: string;
  deprecation_reason: string | null;
}

export interface ApiVerifyResponse {
  valid: boolean;
  prover: string;
  proof_system: string;
  proof_type: string | null;
  prover_version: string;
  proof_hash: string;
  public_inputs_hash: string | null;
  vk: ApiVkInfo | null;
  verified_at: string;
  error?: string;
}

export interface ApiVerificationKey {
  id: string;
  prover: string;
  version: string;
  proof_system: string;
  proof_type: string | null;
  hash: string;
  alias: string | null;
  status: string;
  deprecation_reason: string | null;
  created_at: string;
}

export interface ApiProverResponse {
  name: string;
  proof_systems: string[];
}

export interface ApiProverVersionResponse {
  version: string;
  active: boolean;
  proof_systems: string[];
}

export interface ApiProofSystemInfoResponse {
  name: string;
  vk_hash: string;
  active: boolean;
}
