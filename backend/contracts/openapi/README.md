# OpenAPI Contracts

This directory contains the OpenAPI specifications for the Prova REST API.

## Versioning

- `v1/` - Current stable API version

## Files

- `prova.yaml` - Complete API specification

## Usage

### Generate TypeScript SDK

```bash
cd ../../../tools/codegen
./generate-sdk.sh
```

### View Documentation

```bash
npx @redocly/cli preview-docs v1/prova.yaml
```

### Validate Spec

```bash
npx @redocly/cli lint v1/prova.yaml
```

## Adding a New Version

1. Copy the latest version directory (e.g., `cp -r v1 v2`)
2. Update the `info.version` in the new spec
3. Make breaking changes only in the new version
4. Update `generate-sdk.sh` to generate both versions
5. Add deprecation notices to old version endpoints
