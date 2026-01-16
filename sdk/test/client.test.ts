import { Prova, ValidationError, AuthenticationError } from '../src';

describe('Prova SDK', () => {
  describe('constructor', () => {
    it('should throw if no API key provided', () => {
      expect(() => new Prova({ apiKey: '' })).toThrow(ValidationError);
    });

    it('should create client with valid config', () => {
      const client = new Prova({ apiKey: 'test-key' });
      expect(client).toBeInstanceOf(Prova);
    });

    it('should use default base URL', () => {
      const client = new Prova({ apiKey: 'test-key' });
      // @ts-ignore - accessing private property for testing
      expect(client.baseUrl).toBe('https://api.prova.io');
    });

    it('should use custom base URL', () => {
      const client = new Prova({
        apiKey: 'test-key',
        baseUrl: 'http://localhost:3000',
      });
      // @ts-ignore - accessing private property for testing
      expect(client.baseUrl).toBe('http://localhost:3000');
    });

    it('should strip trailing slash from base URL', () => {
      const client = new Prova({
        apiKey: 'test-key',
        baseUrl: 'http://localhost:3000/',
      });
      // @ts-ignore - accessing private property for testing
      expect(client.baseUrl).toBe('http://localhost:3000');
    });
  });

  describe('verify', () => {
    it('should make correct API call', async () => {
      const mockFetch = jest.fn().mockResolvedValue({
        ok: true,
        json: () =>
          Promise.resolve({
            valid: true,
            proof_id: 'prova:0x123',
            proof_system: 'groth16',
            prover: 'snarkjs',
            prover_version: '0.7.3',
            public_inputs_hash: '0xabc',
            verified_at: '2026-01-11T12:00:00Z',
          }),
      });

      global.fetch = mockFetch;

      const client = new Prova({
        apiKey: 'test-key',
        baseUrl: 'http://localhost:3000',
      });

      const result = await client.verify({
        proof: { test: 'proof' },
        publicInputs: ['1', '2'],
        proofSystem: 'groth16',
      });

      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:3000/v1/verify',
        expect.objectContaining({
          method: 'POST',
          headers: expect.objectContaining({
            'Content-Type': 'application/json',
            'X-API-Key': 'test-key',
          }),
        })
      );

      expect(result).toEqual({
        valid: true,
        proofId: 'prova:0x123',
        proofSystem: 'groth16',
        prover: 'snarkjs',
        proverVersion: '0.7.3',
        publicInputsHash: '0xabc',
        verifiedAt: '2026-01-11T12:00:00Z',
        error: undefined,
      });
    });
  });
});
