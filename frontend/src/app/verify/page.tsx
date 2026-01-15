'use client';

import { useState } from 'react';
import { VerifyForm } from '@/components/VerifyForm';
import { ProofResult } from '@/components/ProofResult';
import type { VerifyResult } from '@/lib/api';

export default function VerifyPage() {
  const [result, setResult] = useState<VerifyResult | null>(null);

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-3xl font-bold mb-2">Verify Proof</h1>
      <p className="text-gray-600 mb-8">
        Paste your proof data below to verify it instantly.
      </p>

      <div className="space-y-8">
        <VerifyForm onResult={setResult} />

        {result && <ProofResult result={result} />}
      </div>
    </div>
  );
}
