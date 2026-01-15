import type {
  ProvaConfig,
  VerifyRequest,
  VerifyResponse,
  VerificationKey,
  Prover,
  ProverVersion,
  ProofSystemInfo,
  VkInfo,
  ApiVerifyResponse,
  ApiVerificationKey,
  ApiProverResponse,
  ApiProverVersionResponse,
  ApiProofSystemInfoResponse,
} from './types';
import {
  ProvaError,
  AuthenticationError,
  NotFoundError,
  ValidationError,
  TimeoutError,
} from './errors';

export class Prova {
  private apiKey: string;
  private baseUrl: string;
  private timeout: number;

  constructor(config: ProvaConfig) {
    if (!config.apiKey || config.apiKey.length < 16) {
      throw new ValidationError('API key is required and must be at least 16 characters');
    }

    this.apiKey = config.apiKey;
    this.baseUrl = (config.baseUrl || 'https://api.prova.io').replace(/\/$/, '');
    this.timeout = config.timeout || 30000;
  }

  async verify(request: VerifyRequest): Promise<VerifyResponse> {
    const response = await this.request<ApiVerifyResponse>('/v1/verify', {
      method: 'POST',
      body: JSON.stringify({
        proof: request.proof,
        public_inputs: request.publicInputs,
        prover: request.prover,
        proof_system: request.proofSystem,
        vk_id: request.vkId,
      }),
    });

    return {
      valid: response.valid,
      prover: response.prover,
      proofSystem: response.proof_system,
      proofType: response.proof_type,
      proverVersion: response.prover_version,
      proofHash: response.proof_hash,
      publicInputsHash: response.public_inputs_hash,
      vk: response.vk ? this.mapVkInfo(response.vk) : null,
      verifiedAt: response.verified_at,
      error: response.error,
    };
  }

  async getVks(options?: {
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
    const path = query ? `/v1/vks?${query}` : '/v1/vks';

    const response = await this.request<{ vks: ApiVerificationKey[]; total: number }>(path);

    return {
      vks: response.vks.map(this.mapVerificationKey),
      total: response.total,
    };
  }

  async getVk(id: string, prover?: string): Promise<VerificationKey> {
    const params = new URLSearchParams();
    if (prover) params.set('prover', prover);

    const query = params.toString();
    const path = query
      ? `/v1/vks/${encodeURIComponent(id)}?${query}`
      : `/v1/vks/${encodeURIComponent(id)}`;

    const response = await this.request<ApiVerificationKey>(path);
    return this.mapVerificationKey(response);
  }

  async getProvers(): Promise<{ provers: Prover[] }> {
    const response = await this.request<{ provers: ApiProverResponse[] }>('/v1/provers');

    return {
      provers: response.provers.map((p) => ({
        name: p.name,
        proofSystems: p.proof_systems,
      })),
    };
  }

  async getProverVersions(prover: string): Promise<{
    prover: string;
    versions: ProverVersion[];
  }> {
    const response = await this.request<{
      prover: string;
      versions: ApiProverVersionResponse[];
    }>(`/v1/provers/${encodeURIComponent(prover)}/versions`);

    return {
      prover: response.prover,
      versions: response.versions.map((v) => ({
        version: v.version,
        active: v.active,
        proofSystems: v.proof_systems,
      })),
    };
  }

  async getProverVersion(prover: string, version: string): Promise<{
    prover: string;
    version: string;
    proofSystems: ProofSystemInfo[];
    registeredAt: string;
  }> {
    const response = await this.request<{
      prover: string;
      version: string;
      proof_systems: ApiProofSystemInfoResponse[];
      registered_at: string;
    }>(`/v1/provers/${encodeURIComponent(prover)}/${encodeURIComponent(version)}`);

    return {
      prover: response.prover,
      version: response.version,
      proofSystems: response.proof_systems.map((ps) => ({
        name: ps.name,
        vkHash: ps.vk_hash,
        active: ps.active,
      })),
      registeredAt: response.registered_at,
    };
  }

  private mapVkInfo(api: { id: string; hash: string; alias: string | null; status: string; deprecation_reason: string | null }): VkInfo {
    return {
      id: api.id,
      hash: api.hash,
      alias: api.alias,
      status: api.status as 'active' | 'deprecated' | 'revoked',
      deprecationReason: api.deprecation_reason,
    };
  }

  private mapVerificationKey(api: ApiVerificationKey): VerificationKey {
    return {
      id: api.id,
      prover: api.prover,
      version: api.version,
      proofSystem: api.proof_system,
      proofType: api.proof_type,
      hash: api.hash,
      alias: api.alias,
      status: api.status as 'active' | 'deprecated' | 'revoked',
      deprecationReason: api.deprecation_reason,
      createdAt: api.created_at,
    };
  }

  private async request<T>(path: string, options: RequestInit = {}): Promise<T> {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeout);

    try {
      const response = await fetch(`${this.baseUrl}${path}`, {
        ...options,
        headers: {
          'Content-Type': 'application/json',
          'X-API-Key': this.apiKey,
          ...options.headers,
        },
        signal: controller.signal,
      });

      if (!response.ok) {
        await this.handleError(response);
      }

      return await response.json() as T;
    } catch (error) {
      if (error instanceof Error && error.name === 'AbortError') {
        throw new TimeoutError(`Request timed out after ${this.timeout}ms`);
      }
      throw error;
    } finally {
      clearTimeout(timeoutId);
    }
  }

  private async handleError(response: Response): Promise<never> {
    let errorMessage = 'Request failed';
    try {
      const data = await response.json() as { error?: string; message?: string };
      errorMessage = data.error || data.message || errorMessage;
    } catch {
      // Response body not JSON
    }

    switch (response.status) {
      case 401:
        throw new AuthenticationError(errorMessage);
      case 403:
        throw new ProvaError('Forbidden', 403);
      case 404:
        throw new NotFoundError(errorMessage);
      case 400:
        throw new ValidationError(errorMessage);
      case 409:
        throw new ProvaError('Conflict: resource already exists', 409);
      case 429:
        throw new ProvaError('Rate limit exceeded', 429);
      default:
        throw new ProvaError(errorMessage, response.status);
    }
  }
}
