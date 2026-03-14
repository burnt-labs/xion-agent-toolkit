# Asset Types Reference

## Available NFT Types

| Type | Code ID (Testnet) | Description | Use Case |
|------|-------------------|-------------|----------|
| `cw721-base` | 522 | Standard CW721 NFT | General-purpose NFTs |
| `cw721-metadata-onchain` | 525 | On-chain metadata | NFTs with on-chain attributes |
| `cw721-expiration` | 523 | Time-based expiry | Limited-time NFTs, tickets, passes |
| `cw721-non-transferable` | 526 | Soulbound NFT | Credentials, achievements, identity |
| `cw2981-royalties` | 528 | Royalty support | NFTs with creator royalties |

## Type-Specific Features

### cw721-base
Standard NFT with basic functionality.

```bash
xion-toolkit asset create --type cw721-base --name "My NFT" --symbol "NFT"
xion-toolkit asset mint --contract xion1... --token-id "1" --owner xion1...
```

### cw2981-royalties
NFT with CW2981 royalty standard. Royalties are set at mint time.

```bash
xion-toolkit asset mint --contract xion1... --token-id "1" --owner xion1... \
  --asset-type cw2981-royalties \
  --royalty-address xion1... \
  --royalty-percentage 0.05  # 5% royalty
```

### cw721-expiration
NFT with time-based expiration.

```bash
xion-toolkit asset mint --contract xion1... --token-id "1" --owner xion1... \
  --asset-type cw721-expiration \
  --expires-at "2025-12-31T23:59:59Z"
```

### cw721-non-transferable
Soulbound NFT that cannot be transferred after minting.

```bash
xion-toolkit asset create --type cw721-non-transferable --name "Achievements" --symbol "SBT"
xion-toolkit asset mint --contract xion1... --token-id "1" --owner xion1...
```

## Validation Rules

- `--royalty-percentage` must be between 0.0 and 1.0
- `--royalty-address` and `--royalty-percentage` must be provided together
- `--expires-at` must be a valid timestamp (ISO 8601 or Unix)
- Asset type options are validated against the specified type
