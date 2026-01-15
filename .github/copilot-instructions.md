# Prova - AI Coding Instructions

Prova is a universal ZK proof verification layer. It abstracts multiple provers (snarkjs, Zisk) behind a single API, storing verified proofs with canonical IDs in PostgreSQL.

## Architecture

```
Clients → Gateway (Rust/Axum:3000) → Verifier Services (gRPC)
                ↓                         ├─ snarkjs (Node.js:50052)
          PostgreSQL                      └─ Zisk (Rust:50051)
```

- **Gateway** ([code/gateway/](code/gateway/)): Routes requests, validates API keys (SHA-256 hash), applies rate limiting, stores proof metadata
- **Verifier services** ([code/verifiers/](code/verifiers/)): Stateless gRPC services that perform actual proof verification against VKs
- **Frontend** ([code/frontend/](code/frontend/)): Next.js dashboard for manual verification and VK management
- **SDK** ([code/sdk/](code/sdk/)): TypeScript client wrapping the REST API

## Code Quality Rules

**Rust (Gateway)**:

- Never use `unwrap()`/`expect()` in production—propagate `Result<>` with `?`
- Use `ApiError` enum from [error.rs](code/gateway/src/error.rs) for typed HTTP responses
- Log errors internally, return generic messages to clients (see `IntoResponse` impl)
- Environment config validated at startup via `Config::from_env() -> Result<>`

**TypeScript (SDK/Verifiers)**:

- No `any` types—create interfaces in `types.ts`
- No `@ts-ignore`—use `.d.ts` files (see [snarkjs.d.ts](code/verifiers/snarkjs/src/snarkjs.d.ts))
- Structured logging: `process.stdout.write(JSON.stringify({...}) + '\n')`
- Export error classes from `index.ts` ([errors.ts](code/sdk/src/errors.ts) pattern)

## Key Patterns

**Auth middleware** ([auth.rs](code/gateway/src/middleware/auth.rs)):

- Public paths: `/health`, `/v1/provers/**`
- API key via `X-API-Key` header, SHA-256 hashed, checked against `api_keys` table
- Normalize paths before matching (strip trailing slashes)

**Proof verification flow** ([verify.rs](code/gateway/src/routes/verify.rs)):

1. Parse request → auto-detect prover/proof_system if missing
2. Call appropriate verifier gRPC service
3. If valid: generate `proof_id`, store in PostgreSQL
4. Return `VerifyResponse` with canonical ID

**Database migrations** ([migrations/](code/gateway/src/db/migrations/)): Raw SQL files run at startup, numbered `00X_*.sql`

## Development Commands

```bash
# Full stack via Docker
docker-compose up

# Local dev (requires PostgreSQL)
docker-compose -f docker-compose.dev.yml up -d  # Start Postgres only
cd gateway && cargo run                          # Gateway on :3000
cd verifiers/snarkjs && npm run dev              # snarkjs on :50052
cd verifiers/zisk && cargo run                   # Zisk on :50051
cd frontend && npm run dev                       # Frontend on :3003
```

## Service Communication

- Gateway ↔ Verifiers: gRPC using [verifier.proto](code/proto/verifier.proto)
- Frontend ↔ Gateway: REST API (internal), NextAuth for dashboard auth
- SDK ↔ Gateway: REST API with `X-API-Key` header

## Environment Variables

| Service  | Required Variables                                        |
| -------- | --------------------------------------------------------- |
| Gateway  | `DATABASE_URL`, `SNARKJS_SERVICE_URL`, `ZISK_SERVICE_URL` |
| snarkjs  | `GRPC_PORT`, `VK_DIR`                                     |
| Frontend | `NEXT_PUBLIC_API_URL`, `NEXTAUTH_SECRET`, `DATABASE_URL`  |

## Remaining Work

- Tests: Not yet implemented for gateway, SDK, verifiers
- Proof validation: Currently accepts any JSON—consider schema validation
- Idempotency: `ON CONFLICT DO UPDATE` exists; full idempotency needs `Idempotency-Key` header
