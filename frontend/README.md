# Prova Frontend

Dashboard and web interface for Prova — universal ZK proof verification layer.

## Quick Start

### Prerequisites

- Node.js 20+
- Running Prova backend (see [prova-backend](../prova-backend))

### Development

```bash
npm install
npm run dev
```

Frontend available at http://localhost:3003

### Production

```bash
npm run build
npm start
```

## Environment Variables

| Variable               | Description                      | Default                 |
| ---------------------- | -------------------------------- | ----------------------- |
| `NEXT_PUBLIC_API_URL`  | Prova API gateway URL            | `http://localhost:3000` |
| `NEXTAUTH_URL`         | NextAuth callback URL            | `http://localhost:3003` |
| `NEXTAUTH_SECRET`      | NextAuth secret                  | Required                |
| `DATABASE_URL`         | PostgreSQL for NextAuth sessions | Required                |
| `INTERNAL_API_SECRET`  | Secret for API key provisioning  | Required                |
| `INTERNAL_API_URL`     | Gateway internal URL             | `http://localhost:3000` |
| `GITHUB_CLIENT_ID`     | GitHub OAuth client ID           | Required for auth       |
| `GITHUB_CLIENT_SECRET` | GitHub OAuth client secret       | Required for auth       |

## Project Structure

```
prova-frontend/
├── src/
│   ├── app/              # Next.js App Router pages
│   ├── components/       # React components
│   ├── lib/              # Utilities, auth config
│   └── hooks/            # Custom React hooks
├── config/               # Environment configuration
├── tests/
│   ├── unit/
│   └── e2e/
└── public/               # Static assets
```

## Authentication Flow

1. User clicks "Login with GitHub"
2. NextAuth handles OAuth flow
3. On success, frontend calls `POST /internal/api-keys/provision`
4. Gateway returns API key, stored in session
5. All subsequent API calls use `X-API-Key` header

## Docker

```bash
docker-compose up
```

Requires backend services running. See `docker-compose.yaml` for configuration.

## License

Proprietary — All rights reserved.
