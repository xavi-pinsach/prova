'use client';

import Link from 'next/link';
import { useSession, signIn, signOut } from 'next-auth/react';

export function Navigation() {
  const { data: session } = useSession();

  return (
    <nav className="border-b">
      <div className="container mx-auto px-4 py-4 flex items-center justify-between">
        <div className="flex items-center gap-8">
          <Link href="/" className="text-xl font-bold">
            Prova
          </Link>
          <div className="flex gap-6">
            <Link href="/verify" className="text-gray-600 hover:text-gray-900">
              Verify
            </Link>
            <Link href="/provers" className="text-gray-600 hover:text-gray-900">
              Provers
            </Link>
            {session && (
              <Link href="/dashboard" className="text-gray-600 hover:text-gray-900">
                Dashboard
              </Link>
            )}
          </div>
        </div>
        <div>
          {session ? (
            <div className="flex items-center gap-4">
              <span className="text-sm text-gray-600">{session.user?.email}</span>
              <button
                onClick={() => signOut()}
                className="text-sm text-gray-600 hover:text-gray-900"
              >
                Sign out
              </button>
            </div>
          ) : (
            <button
              onClick={() => signIn('github')}
              className="px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 text-sm"
            >
              Sign in with GitHub
            </button>
          )}
        </div>
      </div>
    </nav>
  );
}
