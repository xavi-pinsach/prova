'use client';

import type { VerifyResult } from '@/lib/api';

interface Props {
  result: VerifyResult;
}

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

export function ProofResult({ result }: Props) {
  return (
    <div
      className={`p-6 rounded-lg border-2 ${
        result.valid
          ? 'bg-green-50 border-green-200'
          : 'bg-red-50 border-red-200'
      }`}
    >
      <div className="flex items-center gap-3 mb-4">
        <div
          className={`text-3xl ${result.valid ? 'text-green-600' : 'text-red-600'}`}
        >
          {result.valid ? '✓' : '✗'}
        </div>
        <div>
          <h3
            className={`text-xl font-semibold ${
              result.valid ? 'text-green-800' : 'text-red-800'
            }`}
          >
            {result.valid ? 'Valid Proof' : 'Invalid Proof'}
          </h3>
          <p className="text-sm text-gray-600">
            Verified at {new Date(result.verified_at).toLocaleString()}
          </p>
        </div>
      </div>

      <div className="space-y-3">
        <div>
          <span className="text-sm font-medium text-gray-500">Proof Hash:</span>
          <p className="font-mono text-sm break-all bg-white p-2 rounded mt-1">
            {result.proof_hash}
          </p>
        </div>

        <div className="grid md:grid-cols-2 gap-4">
          <div>
            <span className="text-sm font-medium text-gray-500">Prover:</span>
            <p className="font-medium">{result.prover}</p>
          </div>
          <div>
            <span className="text-sm font-medium text-gray-500">Version:</span>
            <p className="font-medium">{result.prover_version}</p>
          </div>
          <div>
            <span className="text-sm font-medium text-gray-500">Proof System:</span>
            <p className="font-medium">{result.proof_system}</p>
          </div>
          {result.proof_type && (
            <div>
              <span className="text-sm font-medium text-gray-500">Proof Type:</span>
              <p className="font-medium">{result.proof_type}</p>
            </div>
          )}
          {result.public_inputs_hash && (
            <div>
              <span className="text-sm font-medium text-gray-500">
                Public Inputs Hash:
              </span>
              <p className="font-mono text-xs break-all">
                {result.public_inputs_hash}
              </p>
            </div>
          )}
        </div>

        {result.vk && (
          <div className="mt-4 p-4 bg-white rounded border">
            <div className="flex items-center gap-2 mb-2">
              <span className="text-sm font-medium text-gray-700">
                Verification Key
              </span>
              <VkStatusBadge status={result.vk.status} />
            </div>
            <div className="space-y-2 text-sm">
              <div>
                <span className="text-gray-500">Hash: </span>
                <span className="font-mono text-xs break-all">{result.vk.hash}</span>
              </div>
              {result.vk.alias && (
                <div>
                  <span className="text-gray-500">Alias: </span>
                  <span className="font-medium">{result.vk.alias}</span>
                </div>
              )}
              {result.vk.status === 'deprecated' && result.vk.deprecation_reason && (
                <div className="p-2 bg-yellow-50 border border-yellow-200 rounded text-yellow-800 text-xs">
                  Deprecation reason: {result.vk.deprecation_reason}
                </div>
              )}
            </div>
          </div>
        )}

        {result.error && (
          <div className="mt-4 p-3 bg-red-100 rounded text-red-700 text-sm">
            {result.error}
          </div>
        )}
      </div>
    </div>
  );
}
