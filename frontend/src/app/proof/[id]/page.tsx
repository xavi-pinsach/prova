'use client';

import Link from 'next/link';
import { useParams } from 'next/navigation';

export default function ProofPage() {
  const params = useParams();
  const proofId = params.id as string;

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-3xl font-bold mb-8">Proof Lookup Unavailable</h1>

      <div className="p-6 bg-yellow-50 border border-yellow-200 rounded-lg">
        <p className="text-yellow-800 mb-4">
          Proof storage has been removed. Prova now performs stateless verification
          and returns proof metadata directly in the verification response.
        </p>
        <p className="text-sm text-gray-600 mb-4">
          Proof hash: <code className="bg-white px-2 py-1 rounded">{proofId}</code>
        </p>
        <p className="text-sm text-gray-600">
          To verify a proof, use the{' '}
          <Link href="/verify" className="text-blue-600 hover:underline">
            verification page
          </Link>{' '}
          or the API directly. The proof hash is returned in the verification response
          for your records.
        </p>
      </div>
    </div>
  );
}
