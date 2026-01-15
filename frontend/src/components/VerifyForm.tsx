'use client';

import { useState } from 'react';
import type { VerifyResult } from '@/lib/api';

interface Props {
  onResult: (result: VerifyResult | null) => void;
}

export function VerifyForm({ onResult }: Props) {
  const [proof, setProof] = useState('');
  const [publicInputs, setPublicInputs] = useState('');
  const [prover, setProver] = useState('');
  const [proofSystem, setProofSystem] = useState('');
  const [vkId, setVkId] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    onResult(null);

    try {
      let proofData;
      try {
        proofData = JSON.parse(proof);
      } catch {
        proofData = proof;
      }

      let publicInputsData;
      if (publicInputs.trim()) {
        try {
          publicInputsData = JSON.parse(publicInputs);
        } catch {
          throw new Error('Invalid public inputs JSON');
        }
      }

      const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';
      const response = await fetch(`${apiUrl}/v1/verify`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          proof: proofData,
          public_inputs: publicInputsData,
          prover: prover || undefined,
          proof_system: proofSystem || undefined,
          vk_id: vkId || undefined,
        }),
      });

      const result = await response.json();

      if (!response.ok) {
        throw new Error(result.error || 'Verification request failed');
      }

      onResult(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Verification failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div>
        <label className="block text-sm font-medium mb-2">
          Proof Data <span className="text-red-500">*</span>
        </label>
        <textarea
          value={proof}
          onChange={(e) => setProof(e.target.value)}
          placeholder="Paste proof JSON here..."
          className="w-full h-48 p-4 border rounded-lg font-mono text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          required
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">
          Public Inputs (optional)
        </label>
        <textarea
          value={publicInputs}
          onChange={(e) => setPublicInputs(e.target.value)}
          placeholder='["1", "2", "3"]'
          className="w-full h-24 p-4 border rounded-lg font-mono text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        />
      </div>

      <div className="grid md:grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium mb-2">
            Prover (optional)
          </label>
          <select
            value={prover}
            onChange={(e) => setProver(e.target.value)}
            className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          >
            <option value="">Auto-detect</option>
            <option value="snarkjs">snarkjs</option>
            <option value="zisk">Zisk</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium mb-2">
            Proof System (optional)
          </label>
          <select
            value={proofSystem}
            onChange={(e) => setProofSystem(e.target.value)}
            className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          >
            <option value="">Auto-detect</option>
            <option value="groth16">Groth16</option>
            <option value="plonk">PLONK</option>
            <option value="fflonk">FFLONK</option>
            <option value="zisk">Zisk</option>
          </select>
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">
          Verification Key (optional)
        </label>
        <input
          type="text"
          value={vkId}
          onChange={(e) => setVkId(e.target.value)}
          placeholder="VK hash (0x...) or alias"
          className="w-full p-3 border rounded-lg font-mono text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        />
        <p className="mt-1 text-xs text-gray-500">
          Specify a verification key by its hash or prover-defined alias
        </p>
      </div>

      {error && (
        <div className="p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          {error}
        </div>
      )}

      <button
        type="submit"
        disabled={loading || !proof.trim()}
        className="w-full py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition"
      >
        {loading ? 'Verifying...' : 'Verify Proof'}
      </button>
    </form>
  );
}
