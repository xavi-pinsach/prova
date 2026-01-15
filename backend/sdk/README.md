# @prova/sdk

Official TypeScript SDK for Prova - Universal Proof Verification.

## Installation

```bash
npm install @prova/sdk
```

## Quick Start

```typescript
import { Prova } from '@prova/sdk';

// Initialize the client
const prova = new Prova({
  apiKey: 'your-api-key',
  baseUrl: 'https://api.prova.io', // optional
});

// Verify a proof
const result = await prova.verify({
  proof: proofData,
  publicInputs: ['1', '2', '3'],
  proofSystem: 'groth16', // optional, auto-detected
  vkId: 'my-circuit-v1',  // optional, VK hash or alias
});

if (result.valid) {
  console.log('Proof verified!');
  console.log('Proof hash:', result.proofHash);
  if (result.vk) {
    console.log('VK status:', result.vk.status);
  }
} else {
  console.log('Verification failed:', result.error);
}
```

## API Reference

### `prova.verify(request)`

Verify a zero-knowledge proof. Returns rich metadata without storing the proof.

```typescript
const result = await prova.verify({
  proof: { /* proof data */ },
  publicInputs: ['1', '2', '3'],  // optional
  prover: 'snarkjs',              // optional, auto-detected
  proofSystem: 'groth16',         // optional, auto-detected
  vkId: 'my-circuit-v1',          // optional, VK hash (0x...) or alias
});

// Result:
// {
//   valid: true,
//   prover: 'snarkjs',
//   proofSystem: 'groth16',
//   proofType: 'SNARK',
//   proverVersion: '0.7.3',
//   proofHash: '0x7f3a...',
//   publicInputsHash: '0xabc...',
//   vk: {
//     id: 'uuid',
//     hash: '0xdef...',
//     alias: 'my-circuit-v1',
//     status: 'active',
//     deprecationReason: null
//   },
//   verifiedAt: '2026-01-14T12:00:00Z'
// }
```

### `prova.getVks(options?)`

List registered verification keys.

```typescript
const { vks, total } = await prova.getVks({
  prover: 'zisk',     // optional filter
  status: 'active',   // optional filter
  limit: 10,          // optional pagination
  offset: 0,          // optional pagination
});

// vks: [
//   {
//     id: 'uuid',
//     prover: 'zisk',
//     version: '1.0.0',
//     proofSystem: 'zisk',
//     proofType: 'STARK',
//     hash: '0xabc...',
//     alias: 'main-circuit',
//     status: 'active',
//     deprecationReason: null,
//     createdAt: '2026-01-14T12:00:00Z'
//   }
// ]
```

### `prova.getVk(id, prover?)`

Get a verification key by UUID, hash, or alias.

```typescript
// By UUID or hash
const vk = await prova.getVk('0xabc123...');

// By alias (requires prover name)
const vk = await prova.getVk('main-circuit', 'zisk');
```

### `prova.getProvers()`

List all supported provers.

```typescript
const { provers } = await prova.getProvers();
// [
//   { name: 'snarkjs', proofSystems: ['groth16', 'plonk', 'fflonk'] },
//   { name: 'zisk', proofSystems: ['zisk'] }
// ]
```

### `prova.getProverVersions(prover)`

List versions for a specific prover.

```typescript
const { prover, versions } = await prova.getProverVersions('snarkjs');
// versions: [
//   { version: '0.7.3', active: true, proofSystems: ['groth16', 'plonk', 'fflonk'] }
// ]
```

### `prova.getProverVersion(prover, version)`

Get details for a specific prover version.

```typescript
const info = await prova.getProverVersion('zisk', '1.0.0');
// {
//   prover: 'zisk',
//   version: '1.0.0',
//   proofSystems: [
//     { name: 'zisk', vkHash: '0x...', active: true }
//   ],
//   registeredAt: '2026-01-14T12:00:00Z'
// }
```

## Error Handling

```typescript
import {
  Prova,
  AuthenticationError,
  NotFoundError,
  ValidationError,
  RateLimitError,
  TimeoutError,
} from '@prova/sdk';

try {
  const result = await prova.verify({ proof: data });
} catch (error) {
  if (error instanceof AuthenticationError) {
    console.error('Invalid API key');
  } else if (error instanceof NotFoundError) {
    console.error('VK not found');
  } else if (error instanceof ValidationError) {
    console.error('Invalid request:', error.message);
  } else if (error instanceof RateLimitError) {
    console.error('Rate limit exceeded');
  } else if (error instanceof TimeoutError) {
    console.error('Request timed out');
  } else {
    console.error('Error:', error.message);
  }
}
```

## VK Status

Verification keys can have one of three statuses:

| Status | Description |
|--------|-------------|
| `active` | VK is valid and recommended for use |
| `deprecated` | VK works but should be replaced (e.g., newer version available) |
| `revoked` | VK is invalid (e.g., security issue) - verification will be rejected |

When a VK is deprecated or revoked, check `deprecationReason` for details.

## License

MIT
