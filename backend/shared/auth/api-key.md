# API Key Authentication

Prova uses API keys for authentication. This document describes the implementation.

## Key Format

```
prova_<48 hex characters>
```

Example: `prova_a1b2c3d4e5f6...`

- Prefix `prova_` for identification
- 24 random bytes (48 hex chars) for entropy
- Minimum length: 16 characters (enforced by middleware)

## Storage

Keys are **never stored in plaintext**. The gateway stores a SHA-256 hash:

```rust
let mut hasher = Sha256::new();
hasher.update(api_key.as_bytes());
let key_hash = hex::encode(hasher.finalize());
```

## Authentication Flow

1. Client sends request with `X-API-Key: prova_...` header
2. Gateway computes SHA-256 hash of provided key
3. Gateway queries `api_keys` table for matching hash
4. If found and not revoked, request proceeds
5. `last_used_at` timestamp updated for analytics

## Public Paths

These paths do not require authentication:

- `/health`
- `/v1/provers/**`
- `/internal/api-keys/provision` (protected by `X-Internal-Secret`)

## Rate Limiting

Rate limiting is per-API-key, stored in memory:

- Default: 100 requests/minute
- Enterprise: Configurable

## Database Schema

```sql
CREATE TABLE api_keys (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    key_hash VARCHAR(64) NOT NULL UNIQUE,
    name VARCHAR(255),
    revoked BOOLEAN DEFAULT FALSE,
    last_used_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```
