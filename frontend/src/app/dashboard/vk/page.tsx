'use client';

import { useSession } from 'next-auth/react';
import { redirect } from 'next/navigation';
import { useEffect, useState } from 'react';
import type { VerificationKey } from '@/lib/api';
import { getVks } from '@/lib/api';

function VkStatusBadge({ status }: { status: string }) {
  const colors = {
    active: 'bg-green-100 text-green-800',
    deprecated: 'bg-yellow-100 text-yellow-800',
    revoked: 'bg-red-100 text-red-800',
  };

  return (
    <span
      className={`inline-flex px-2 py-0.5 text-xs font-medium rounded ${colors[status as keyof typeof colors] || 'bg-gray-100 text-gray-800'}`}
    >
      {status}
    </span>
  );
}

export default function VkManagementPage() {
  const { data: session, status } = useSession();
  const [vks, setVks] = useState<VerificationKey[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchVks() {
      try {
        const data = await getVks();
        setVks(data.vks);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load verification keys');
      } finally {
        setLoading(false);
      }
    }

    fetchVks();
  }, []);

  if (status === 'loading') {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="text-gray-500">Loading...</div>
      </div>
    );
  }

  if (!session) {
    redirect('/');
  }

  return (
    <div className="max-w-4xl mx-auto">
      <h1 className="text-3xl font-bold mb-2">Verification Keys</h1>
      <p className="text-gray-600 mb-8">
        View and manage verification keys for provers.
      </p>

      {error && (
        <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          {error}
        </div>
      )}

      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Registered Verification Keys</h2>
        {loading ? (
          <div className="text-gray-500">Loading...</div>
        ) : vks.length === 0 ? (
          <div className="p-6 border rounded-lg text-gray-500 text-center">
            No verification keys registered yet.
          </div>
        ) : (
          <div className="space-y-4">
            {vks.map((vk) => (
              <div key={vk.id} className="p-4 border rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <div className="flex items-center gap-2">
                    <span className="font-medium">{vk.prover}</span>
                    <span className="text-gray-400">v{vk.version}</span>
                    <VkStatusBadge status={vk.status} />
                  </div>
                  <span className="text-xs text-gray-500">
                    {vk.proof_system}
                    {vk.proof_type && ` / ${vk.proof_type}`}
                  </span>
                </div>
                <div className="space-y-1 text-sm">
                  <div>
                    <span className="text-gray-500">Hash: </span>
                    <code className="text-xs bg-gray-100 px-1 py-0.5 rounded break-all">
                      {vk.hash}
                    </code>
                  </div>
                  {vk.alias && (
                    <div>
                      <span className="text-gray-500">Alias: </span>
                      <span className="font-medium">{vk.alias}</span>
                    </div>
                  )}
                  {vk.status !== 'active' && vk.deprecation_reason && (
                    <div className="mt-2 p-2 bg-yellow-50 border border-yellow-200 rounded text-yellow-800 text-xs">
                      Reason: {vk.deprecation_reason}
                    </div>
                  )}
                </div>
                <div className="mt-2 text-xs text-gray-400">
                  Registered {new Date(vk.created_at).toLocaleDateString()}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="p-6 border rounded-lg bg-gray-50">
        <h2 className="text-xl font-semibold mb-4">Register New VK</h2>
        <p className="text-gray-500 text-sm mb-4">
          VK registration requires admin or prover_manager role. Contact us to register as a prover.
        </p>

        <form className="space-y-4">
          <div className="grid md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium mb-2">Prover Name</label>
              <input
                type="text"
                placeholder="e.g., zisk"
                className="w-full p-3 border rounded-lg bg-white"
                disabled
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-2">Version</label>
              <input
                type="text"
                placeholder="e.g., 1.0.0"
                className="w-full p-3 border rounded-lg bg-white"
                disabled
              />
            </div>
          </div>
          <div className="grid md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium mb-2">Proof System</label>
              <select className="w-full p-3 border rounded-lg bg-white" disabled>
                <option>Select proof system</option>
                <option value="groth16">Groth16</option>
                <option value="plonk">PLONK</option>
                <option value="fflonk">FFLONK</option>
                <option value="zisk">Zisk</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium mb-2">Alias (optional)</label>
              <input
                type="text"
                placeholder="e.g., main-circuit-v1"
                className="w-full p-3 border rounded-lg bg-white"
                disabled
              />
            </div>
          </div>
          <div>
            <label className="block text-sm font-medium mb-2">Proof Type (optional)</label>
            <input
              type="text"
              placeholder="e.g., SNARK, STARK"
              className="w-full p-3 border rounded-lg bg-white"
              disabled
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-2">Verification Key (JSON)</label>
            <textarea
              placeholder="Paste VK JSON here..."
              className="w-full h-32 p-3 border rounded-lg font-mono text-sm bg-white"
              disabled
            />
          </div>
          <button
            type="submit"
            className="px-6 py-3 bg-gray-400 text-white rounded-lg cursor-not-allowed"
            disabled
          >
            Register VK (Admin/Prover Manager only)
          </button>
        </form>
      </div>
    </div>
  );
}
