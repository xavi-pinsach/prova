'use client';

import { useEffect, useState } from 'react';

interface Prover {
  name: string;
  proof_systems: string[];
}

export default function ProversPage() {
  const [provers, setProvers] = useState<Prover[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchProvers() {
      try {
        const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';
        const response = await fetch(`${apiUrl}/v1/provers`);

        if (!response.ok) {
          throw new Error('Failed to fetch provers');
        }

        const data = await response.json();
        setProvers(data.provers);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load provers');
      } finally {
        setLoading(false);
      }
    }

    fetchProvers();
  }, []);

  if (loading) {
    return (
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold mb-8">Supported Provers</h1>
        <div className="text-gray-500">Loading...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold mb-8">Supported Provers</h1>
        <div className="p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          {error}
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto">
      <h1 className="text-3xl font-bold mb-2">Supported Provers</h1>
      <p className="text-gray-600 mb-8">
        Prova supports verification for the following proof systems.
      </p>

      <div className="grid md:grid-cols-2 gap-6">
        {provers.map((prover) => (
          <div
            key={prover.name}
            className="p-6 border rounded-lg hover:shadow-md transition"
          >
            <h2 className="text-xl font-semibold mb-3">{prover.name}</h2>
            <div>
              <span className="text-sm text-gray-500">Proof Systems:</span>
              <div className="flex flex-wrap gap-2 mt-2">
                {prover.proof_systems.map((system) => (
                  <span
                    key={system}
                    className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm"
                  >
                    {system}
                  </span>
                ))}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
