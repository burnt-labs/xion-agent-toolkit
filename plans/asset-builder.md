---
status: Todo
created_at: 2026-03-13
updated_at: 2026-03-13
---

# Asset Builder (CW721 NFT)

## Background

Developer Portal supports CW721 NFT contract deployment via Asset Builder feature with 8 contract variants.

## Goal

Support CW721 NFT contract deployment and minting via CLI.

## Contract Variants

| Asset Type | Code ID (Testnet) | Available | Features |
|------------|-------------------|-----------|----------|
| cw721-base | 522 | Yes | Standard NFT, transferable, off-chain metadata |
| cw721-metadata-onchain | 525 | Yes | On-chain metadata, immutable |
| cw721-expiration | 523 | Yes | Time-based expiry |
| cw721-fixed-price | 524 | Yes | Fixed price sales (CW20 only) |
| cw721-non-transferable | 526 | Yes | Soulbound, permanent |
| cw2981-royalties | 528 | Yes | Royalty support (CW2981) |
| cw721-updatable | 0 | No | Updatable metadata |
| cw721-soulbound | 0 | No | Identity/badges |

## API Design

### Deploy NFT Collection

```bash
xion-toolkit asset deploy \
  --type cw721-base \
  --name "My Collection" \
  --symbol "NFT" \
  [--description "..."] \
  [--max-supply 10000] \
  [--base-uri "ipfs://..."]
```

**Output:**
```json
{
  "success": true,
  "contract_address": "xion1...",
  "code_id": 522,
  "asset_type": "cw721-base"
}
```

### Mint NFT

```bash
xion-toolkit asset mint \
  --contract xion1... \
  --token-id "1" \
  --owner xion1... \
  [--token-uri "ipfs://..."] \
  [--royalty-percentage 5] \
  [--royalty-address xion1...]
```

**Output:**
```json
{
  "success": true,
  "tx_hash": "...",
  "token_id": "1",
  "owner": "xion1..."
}
```

### Query Contract

```bash
xion-toolkit asset query \
  --contract xion1... \
  --msg '{"nft_info": {"token_id": "1"}}'
```

## Implementation Phases

### Phase 1: Basic Deployment

- [ ] Add `src/asset/` module
- [ ] Support cw721-base deployment
- [ ] Use instantiate2 for predictable addresses

### Phase 2: Minting

- [ ] Implement mint command
- [ ] Support variant-specific mint messages
- [ ] Handle CW2981 royalty fields

### Phase 3: All Variants

- [ ] Add support for all 6 available variants
- [ ] Variant-specific instantiation messages
- [ ] Validation for variant-specific options

## Type Definitions

```rust
// src/asset/types.rs

#[derive(Debug, Clone, Deserialize)]
pub struct DeployRequest {
    pub asset_type: AssetType,
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub max_supply: Option<u32>,
    pub base_uri: Option<String>,
    pub custom_code_id: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MintRequest {
    pub contract: String,
    pub token_id: String,
    pub owner: String,
    pub token_uri: Option<String>,
    pub royalty_percentage: Option<u32>,
    pub royalty_address: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeployResult {
    pub success: bool,
    pub contract_address: String,
    pub code_id: u64,
    pub asset_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MintResult {
    pub success: bool,
    pub tx_hash: String,
    pub token_id: String,
    pub owner: String,
}
```

## Contract Configuration

Store code IDs in `src/config/constants.rs`:

```rust
pub const ASSET_CODE_IDS: &[(Network, AssetType, u64)] = &[
    (Network::Testnet, AssetType::Cw721Base, 522),
    (Network::Testnet, AssetType::Cw721MetadataOnchain, 525),
    (Network::Testnet, AssetType::Cw721Expiration, 523),
    (Network::Testnet, AssetType::Cw721FixedPrice, 524),
    (Network::Testnet, AssetType::Cw721NonTransferable, 526),
    (Network::Testnet, AssetType::Cw2981Royalties, 528),
    // Mainnet values TBD
];
```

## Instantiate Messages by Variant

### cw721-base
```json
{
  "name": "...",
  "symbol": "...",
  "minter": "xion1..."
}
```

### cw2981-royalties
```json
{
  "name": "...",
  "symbol": "...",
  "minter": "xion1...",
  "collection_info_extension": {
    "description": "...",
    "image": "...",
    "external_url": ""
  }
}
```

### cw721-expiration
```json
{
  "name": "...",
  "symbol": "...",
  "minter": "xion1...",
  "default_expiration": { "days": 365 }
}
```

## Files to Create/Modify

```
src/
├── asset/
│   ├── mod.rs          # NEW
│   ├── deploy.rs       # NEW - contract deployment
│   ├── mint.rs         # NEW - token minting
│   └── types.rs        # NEW - asset types
├── cli/
│   ├── asset.rs        # NEW
│   └── mod.rs          # MODIFY
└── config/
    └── constants.rs    # MODIFY - add code IDs
```

## Acceptance Criteria

- [ ] Deploy cw721-base contract successfully
- [ ] Mint tokens to recipient
- [ ] Query contract state
- [ ] Support CW2981 royalties (mint time configuration)
- [ ] Predictable addresses via instantiate2
- [ ] All 6 available variants supported

## Dependencies

- No new dependencies required
- Uses existing contract instantiate/execute infrastructure

## Reference

- Developer Portal: `src/config/assetBuilder/constants.ts`
- Developer Portal: `src/lib/assetBuilder/asset.ts`

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-13 | @project-manager | Plan created | Todo |
