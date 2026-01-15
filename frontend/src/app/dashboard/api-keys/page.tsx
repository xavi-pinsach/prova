'use client';

import { useSession } from 'next-auth/react';
import { redirect } from 'next/navigation';
import { useState } from 'react';

export default function ApiKeysPage() {
  const { data: session, status } = useSession();
  const [newKeyName, setNewKeyName] = useState('');
  const [generatedKey, setGeneratedKey] = useState<string | null>(null);

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

  const handleCreateKey = async (e: React.FormEvent) => {
    e.preventDefault();
    // TODO: Implement API key creation
    // For MVP, show placeholder
    const mockKey = `prova_${Math.random().toString(36).substring(2, 15)}`;
    setGeneratedKey(mockKey);
    setNewKeyName('');
  };

  return (
    <div className="max-w-4xl mx-auto">
      <h1 className="text-3xl font-bold mb-2">API Keys</h1>
      <p className="text-gray-600 mb-8">
        Create and manage API keys for programmatic access.
      </p>

      <div className="space-y-8">
        {/* Create new key */}
        <div className="p-6 border rounded-lg">
          <h2 className="text-xl font-semibold mb-4">Create New API Key</h2>
          <form onSubmit={handleCreateKey} className="flex gap-4">
            <input
              type="text"
              value={newKeyName}
              onChange={(e) => setNewKeyName(e.target.value)}
              placeholder="Key name (e.g., Production)"
              className="flex-1 p-3 border rounded-lg"
              required
            />
            <button
              type="submit"
              className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
            >
              Create Key
            </button>
          </form>

          {generatedKey && (
            <div className="mt-4 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
              <p className="text-sm text-yellow-800 mb-2">
                Copy this key now. You won't be able to see it again!
              </p>
              <code className="block p-3 bg-white rounded border font-mono text-sm break-all">
                {generatedKey}
              </code>
            </div>
          )}
        </div>

        {/* Existing keys */}
        <div className="p-6 border rounded-lg">
          <h2 className="text-xl font-semibold mb-4">Your API Keys</h2>
          <p className="text-gray-500 text-sm">
            No API keys yet. Create one above to get started.
          </p>
        </div>
      </div>
    </div>
  );
}
