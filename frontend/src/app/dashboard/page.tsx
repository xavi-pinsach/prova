'use client';

import { useSession } from 'next-auth/react';
import Link from 'next/link';
import { redirect } from 'next/navigation';

export default function DashboardPage() {
  const { data: session, status } = useSession();

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
      <h1 className="text-3xl font-bold mb-2">Dashboard</h1>
      <p className="text-gray-600 mb-8">
        Welcome back, {session.user?.name || session.user?.email}
      </p>

      <div className="grid md:grid-cols-2 gap-6">
        <Link
          href="/dashboard/api-keys"
          className="p-6 border rounded-lg hover:shadow-md transition block"
        >
          <h2 className="text-xl font-semibold mb-2">API Keys</h2>
          <p className="text-gray-600 text-sm">
            Manage your API keys for programmatic access to Prova.
          </p>
        </Link>

        <Link
          href="/dashboard/vk"
          className="p-6 border rounded-lg hover:shadow-md transition block"
        >
          <h2 className="text-xl font-semibold mb-2">Verification Keys</h2>
          <p className="text-gray-600 text-sm">
            Register and manage verification keys for your provers.
          </p>
        </Link>
      </div>
    </div>
  );
}
