import Link from 'next/link';

export default function Home() {
  return (
    <div className="max-w-4xl mx-auto">
      <div className="text-center py-16">
        <h1 className="text-5xl font-bold mb-4">Prova</h1>
        <p className="text-xl text-gray-600 mb-8">
          A universal, canonical proof verification layer
        </p>
        <p className="text-gray-500 mb-12 max-w-2xl mx-auto">
          Send us any proof, we verify it, you get a result and a canonical reference.
          No installation. No version management. Ever.
        </p>

        <div className="flex gap-4 justify-center">
          <Link
            href="/verify"
            className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
          >
            Verify a Proof
          </Link>
          <Link
            href="/provers"
            className="px-6 py-3 border border-gray-300 rounded-lg hover:bg-gray-50 transition"
          >
            View Supported Provers
          </Link>
        </div>
      </div>

      <div className="grid md:grid-cols-3 gap-8 py-16">
        <div className="text-center p-6">
          <div className="text-3xl mb-4">🔐</div>
          <h3 className="text-lg font-semibold mb-2">Universal</h3>
          <p className="text-gray-600 text-sm">
            Support for all major proof systems: Groth16, PLONK, FFLONK, Zisk, and more.
          </p>
        </div>
        <div className="text-center p-6">
          <div className="text-3xl mb-4">📋</div>
          <h3 className="text-lg font-semibold mb-2">Canonical</h3>
          <p className="text-gray-600 text-sm">
            Every verified proof gets a permanent, unique identifier you can reference.
          </p>
        </div>
        <div className="text-center p-6">
          <div className="text-3xl mb-4">⚡</div>
          <h3 className="text-lg font-semibold mb-2">Fast</h3>
          <p className="text-gray-600 text-sm">
            Sub-second verification. No SDK installation required. Just call our API.
          </p>
        </div>
      </div>
    </div>
  );
}
