# Prova Backend

A universal, canonical proof verification layer.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                            Clients                              │
│                  (Web Dashboard, SDK, Direct API)               │
└────────────────────────────────┬────────────────────────────────┘
                                 │ HTTPS
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                          API Gateway                            │
│                         (Rust + Axum)                           │
│                                                                 │
│  • REST API            • Auth (API keys + roles)                │
│  • Rate Limiting       • VK Registry (PostgreSQL)               │
│  • Request Routing     • Stateless Verification                 │
└────────────────────────────────┬────────────────────────────────┘
                                 │ gRPC
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Verifier Services                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────┐       ┌─────────────────────┐          │
│  │    prova-rust       │       │    prova-node       │          │
│  │  (Generic Runner)   │       │  (Generic Runner)   │          │
│  │                     │       │                     │          │
│  │  Loads any Rust     │       │  Loads any Node.js  │          │
│  │  CLI verifier via   │       │  verifier via       │          │
│  │  manifest.yaml      │       │  manifest.yaml      │          │
│  └──────────┬──────────┘       └──────────┬──────────┘          │
│             │                             │                     │
│             ▼                             ▼                     │
│  ┌─────────────────────┐       ┌─────────────────────┐          │
│  │  Artifacts (Binary) │       │  Artifacts (Binary) │          │
│  │  • zisk-verifier    │       │  • snarkjs          │          │
│  │  • sp1-verifier     │       │                     │          │
│  │  • risc0-verifier   │       │                     │          │
│  └─────────────────────┘       └─────────────────────┘          │
│                                                                 │
│  PRIVATE: Binaries mounted at runtime, not in git              │
└─────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
backend/
├── Cargo.toml                    # Rust workspace
├── contracts/
│   ├── openapi/v1/prova.yaml     # REST API spec
│   └── protobuf/v1/verifier.proto # gRPC service definition
├── crates/
│   ├── gateway/                  # API Gateway (Axum)
│   └── services/
│       └── rust/                 # prova-rust runner
├── deployments/
│   ├── docker-compose.yml
│   └── docker-compose.dev.yml
├── sdk/                          # TypeScript SDK
└── shared/
    └── errors/error-codes.yaml
```

## Quick Start

### Prerequisites

- Docker and Docker Compose
- Rust (for local development)
- Node.js 20+ (for SDK development)

### Run with Docker (Development)

```bash
# Start PostgreSQL only
docker compose -f deployments/docker-compose.dev.yml up -d
```

### Run with Docker (Production)

Production deployments use the `verifiers/` directory for private artifacts:

```bash
cd ../verifiers/deployments
docker compose up
```

### Local Development

1. Start PostgreSQL:
```bash
docker compose -f deployments/docker-compose.dev.yml up -d
```

2. Run the API Gateway:
```bash
cd crates/gateway
cargo run
```

3. Run the Zisk Verifier Service:
```bash
cd crates/services/rust
ARTIFACTS_DIR=/path/to/verifiers/artifacts/zisk cargo run
```

## API Endpoints

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/health` | GET | None | Health check |
| `/v1/verify` | POST | API Key | Verify a proof (stateless) |
| `/v1/vks` | GET | None | List verification keys |
| `/v1/vks/{id}` | GET | None | Get VK by ID/hash/alias |
| `/v1/vks` | POST | Admin/Prover Manager | Register new VK |
| `/v1/vks/{id}` | PATCH | Admin/Prover Manager | Update VK status |
| `/v1/provers` | GET | None | List supported provers |
| `/v1/provers/{prover}/versions` | GET | None | List prover versions |
| `/internal/anchor` | POST | Internal Secret | Record chain anchor |

## Example: Verify a Proof

```bash
curl -X POST http://localhost:3000/v1/verify \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{
    "proof": { ... },
    "public_inputs": ["0x1234", "0x5678"],
    "prover": "zisk",
    "proof_system": "zisk",
    "vk_id": "main-circuit-v1"
  }'
```

Response:
```json
{
  "valid": true,
  "prover": "zisk",
  "proof_system": "zisk",
  "proof_type": "STARK",
  "prover_version": "0.1.0",
  "proof_hash": "0x7f3a2b...",
  "public_inputs_hash": "0xabc123...",
  "vk": {
    "id": "uuid",
    "hash": "0xdef456...",
    "alias": "main-circuit-v1",
    "status": "active",
    "deprecation_reason": null
  },
  "verified_at": "2026-01-14T12:00:00Z"
}
```

## Environment Variables

### Gateway

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | (required) | PostgreSQL connection string |
| `ZISK_SERVICE_URL` | `http://localhost:50051` | Zisk gRPC endpoint |
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `3000` | HTTP port |
| `CORS_ORIGINS` | `*` | Allowed origins |
| `RATE_LIMIT_REQUESTS` | `100` | Requests per window |
| `RATE_LIMIT_WINDOW_SECS` | `60` | Window duration |
| `INTERNAL_API_SECRET` | (required in prod) | Internal API auth |

### Verifier Services

| Variable | Default | Description |
|----------|---------|-------------|
| `ARTIFACTS_DIR` | `/artifacts` | Path to verifier artifacts |
| `GRPC_PORT` | `50051` | gRPC server port |
| `RUST_LOG` | `info` | Log level |

## Service Communication

Services communicate via gRPC (port 50051 for zisk, 50052 for snarkjs). The proto definition:

```protobuf
service Verifier {
  rpc Verify(VerifyRequest) returns (VerifyResponse);
  rpc Health(HealthRequest) returns (HealthResponse);
}
```

## Security

- **Binary Integrity**: SHA256 checksum verification at startup (required)
- **Secure Temp Files**: Uses `tempfile` crate for random filenames and automatic cleanup
- **API Keys**: Hashed with SHA256 before storage
- **Rate Limiting**: Per API key with configurable limits

## Development

### Running Tests

```bash
# Gateway
cd crates/gateway && cargo test

# SDK
cd sdk && npm test
```

### Building Docker Images

```bash
# Gateway
docker build -f crates/gateway/Dockerfile -t prova-gateway .

# Verifier runner
docker build -f crates/services/rust/Dockerfile -t prova-rust .
```

## Documentation

- [Engineering Design](../docs/engineering-design.md) - Full architecture details
- [Verifiers README](../verifiers/README.md) - Artifact configuration
- [SDK README](sdk/README.md) - TypeScript SDK usage
