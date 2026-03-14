---
name: xion-asset
description: |
  Asset Builder skill for CW721 NFT operations on Xion. Use this skill when users want to:
  - Create NFT collections
  - Mint NFT tokens (standard or with royalties)
  - Predict contract addresses before deployment
  - Batch mint multiple tokens
  - Query NFT contracts
  
  Supports 5 NFT types: cw721-base, cw2981-royalties, cw721-expiration, cw721-metadata-onchain, cw721-non-transferable.
  
  Triggers on: NFT, CW721, asset, mint, collection, royalty, soulbound, expiration, predict address, batch mint.
metadata:
  author: burnt-labs
  version: "1.0.0"
  recommends:
    - xion-oauth2
    - xion-toolkit-init
---

# xion-asset

CW721 NFT Asset Builder for Xion blockchain. Create, mint, and manage NFT collections with gasless transactions.

## Prerequisites

- xion-toolkit CLI installed (use `xion-toolkit-init` skill)
- Authenticated with OAuth2 (use `xion-oauth2` skill)

## Quick Start

```bash
# 1. List available NFT types
xion-toolkit asset types

# 2. Create collection
xion-toolkit asset create --type cw721-base --name "My Collection" --symbol "NFT"

# 3. Mint token
xion-toolkit asset mint --contract xion1... --token-id "1" --owner xion1...
```

## Asset Types

| Type | Code ID | Features |
|------|---------|----------|
| `cw721-base` | 522 | Standard NFT |
| `cw721-metadata-onchain` | 525 | On-chain metadata |
| `cw721-expiration` | 523 | Time-based expiry |
| `cw721-non-transferable` | 526 | Soulbound NFT |
| `cw2981-royalties` | 528 | Royalties at mint time |

## Commands

### List Asset Types

```bash
xion-toolkit asset types
```

Output:
```json
{
  "success": true,
  "types": [...],
  "count": 5
}
```

### Create Collection

```bash
xion-toolkit asset create \
  --type cw721-base \
  --name "My Collection" \
  --symbol "NFT" \
  [--minter xion1...] \
  [--salt "my-salt"]
```

### Mint Token

```bash
# Standard mint
xion-toolkit asset mint \
  --contract xion1... \
  --token-id "1" \
  --owner xion1... \
  [--token-uri "ipfs://..."]

# Mint with royalties (CW2981)
xion-toolkit asset mint \
  --contract xion1... \
  --token-id "1" \
  --owner xion1... \
  --asset-type cw2981-royalties \
  --royalty-address xion1... \
  --royalty-percentage 0.05

# Mint with expiration
xion-toolkit asset mint \
  --contract xion1... \
  --token-id "1" \
  --owner xion1... \
  --asset-type cw721-expiration \
  --expires-at "2025-12-31T23:59:59Z"
```

### Predict Address

```bash
xion-toolkit asset predict \
  --type cw721-base \
  --name "My Collection" \
  --symbol "NFT" \
  --salt "my-unique-salt"
```

### Batch Mint

```bash
# Create tokens.json
cat > tokens.json << 'EOF'
[
  {"token_id": "1", "owner": "xion1abc...", "token_uri": "ipfs://QmHash1"},
  {"token_id": "2", "owner": "xion1def...", "token_uri": "ipfs://QmHash2"}
]
EOF

# Batch mint
xion-toolkit asset batch-mint \
  --contract xion1... \
  --tokens-file tokens.json
```

### Query Contract

```bash
# Get NFT info
xion-toolkit asset query \
  --contract xion1... \
  --msg '{"nft_info": {"token_id": "1"}}'

# Get all tokens
xion-toolkit asset query \
  --contract xion1... \
  --msg '{"all_tokens": {}}'
```

## Scripts Reference

| Script | Description |
|--------|-------------|
| `types.sh` | List available asset types |
| `create.sh` | Create NFT collection |
| `mint.sh` | Mint NFT token |
| `predict.sh` | Predict contract address |
| `batch-mint.sh` | Batch mint from JSON |
| `query.sh` | Query NFT contract |

## Error Handling

All commands return JSON:

**Success:**
```json
{"success": true, ...}
```

**Error:**
```json
{"success": false, "error": "...", "error_code": "..."}
```

**Common Errors:**
- `INVALID_ASSET_TYPE` - Use one of the 5 supported types
- `INVALID_ROYALTY_PERCENTAGE` - Must be 0.0-1.0
- `INCOMPLETE_ROYALTY_INFO` - Both address and percentage required
- `INVALID_OPTION_FOR_TYPE` - Option not valid for this asset type

## Related Skills

- **xion-dev** - Unified entry point
- **xion-oauth2** - Authentication (use before this skill)
- **xion-toolkit-init** - CLI installation
- **xion-treasury** - Treasury management for funding

## Version

- Skill Version: 1.0.0
- Compatible CLI Version: >=0.1.0
