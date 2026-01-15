const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

export interface VkInfo {
  id: string;
  hash: string;
  alias: string | null;
  status: 'active' | 'deprecated' | 'revoked';
  deprecation_reason: string | null;
}

export interface VerifyResult {
  valid: boolean;
  prover: string;
  proof_system: string;
  proof_type: string | null;
  prover_version: string;
  proof_hash: string;
  public_inputs_hash: string | null;
  vk: VkInfo | null;
  verified_at: string;
  error?: string;
}

export interface VerificationKey {
  id: string;
  prover: string;
  version: string;
  proof_system: string;
  proof_type: string | null;
  hash: string;
  alias: string | null;
  status: 'active' | 'deprecated' | 'revoked';
  deprecation_reason: string | null;
  created_at: string;
}

export async function verifyProof(data: {
  proof: unknown;
  publicInputs?: string[];
  prover?: string;
  proofSystem?: string;
  vkId?: string;
}): Promise<VerifyResult> {
  const response = await fetch(`${API_URL}/v1/verify`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      proof: data.proof,
      public_inputs: data.publicInputs,
      prover: data.prover,
      proof_system: data.proofSystem,
      vk_id: data.vkId,
    }),
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || 'Verification failed');
  }

  return response.json();
}

export async function getProvers() {
  const response = await fetch(`${API_URL}/v1/provers`);

  if (!response.ok) {
    throw new Error('Failed to fetch provers');
  }

  return response.json();
}

export async function getVks(options?: {
  prover?: string;
  status?: string;
  limit?: number;
  offset?: number;
}): Promise<{ vks: VerificationKey[]; total: number }> {
  const params = new URLSearchParams();
  if (options?.prover) params.set('prover', options.prover);
  if (options?.status) params.set('status', options.status);
  if (options?.limit) params.set('limit', options.limit.toString());
  if (options?.offset) params.set('offset', options.offset.toString());

  const query = params.toString();
  const path = query ? `${API_URL}/v1/vks?${query}` : `${API_URL}/v1/vks`;

  const response = await fetch(path);

  if (!response.ok) {
    throw new Error('Failed to fetch verification keys');
  }

  return response.json();
}

export async function getVk(id: string, prover?: string): Promise<VerificationKey> {
  const params = new URLSearchParams();
  if (prover) params.set('prover', prover);

  const query = params.toString();
  const path = query
    ? `${API_URL}/v1/vks/${encodeURIComponent(id)}?${query}`
    : `${API_URL}/v1/vks/${encodeURIComponent(id)}`;

  const response = await fetch(path);

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || 'Failed to fetch verification key');
  }

  return response.json();
}
