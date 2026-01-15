#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
CONTRACTS_DIR="$ROOT_DIR/contracts/openapi/v1"
SDK_DIR="$ROOT_DIR/sdk"

echo "Generating TypeScript SDK from OpenAPI spec..."

if ! command -v npx &> /dev/null; then
    echo "Error: npx not found. Please install Node.js"
    exit 1
fi

cd "$SDK_DIR"

npx openapi-typescript-codegen \
    --input "$CONTRACTS_DIR/prova.yaml" \
    --output src/generated \
    --client fetch \
    --name ProvaApiClient

echo "SDK generated at $SDK_DIR/src/generated"
echo ""
echo "Next steps:"
echo "  1. Review generated types in sdk/src/generated/"
echo "  2. Update sdk/src/index.ts exports if needed"
echo "  3. Run 'npm run build' in sdk/ to compile"
