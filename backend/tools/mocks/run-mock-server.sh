#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
CONTRACTS_DIR="$ROOT_DIR/contracts/openapi/v1"

echo "Starting mock server from OpenAPI spec..."

if ! command -v npx &> /dev/null; then
    echo "Error: npx not found. Please install Node.js"
    exit 1
fi

PORT="${MOCK_PORT:-4010}"

echo "Mock server starting on http://localhost:$PORT"
echo "Press Ctrl+C to stop"
echo ""

npx @stoplight/prism-cli mock "$CONTRACTS_DIR/prova.yaml" --port "$PORT"
