---
status: Done
created_at: 2026-03-13
updated_at: 2026-03-13
done_at: 2026-03-13
architecture_completed_at: 2026-03-13
architecture_by: @architect
phase1_completed_at: 2026-03-13
phase1_by: @fullstack-dev
phase1_qa_at: 2026-03-13
phase1_qa_by: @qa-engineer
---

# Asset Builder (CW721 NFT)

## Architecture Summary

**Designed by**: @architect (2026-03-13)

### Module Structure

```
src/
├── asset_builder/
│   ├── mod.rs          # Module exports
│   ├── types.rs        # AssetType enum, messages, results
│   ├── manager.rs      # AssetBuilderManager
│   └── code_ids.rs     # Code ID helpers
├── cli/
│   └── asset.rs        # CLI commands
└── build.rs            # NetworkConfig with code IDs
```

### Supported Asset Types (Testnet)

| Type | Code ID | Features |
|------|---------|----------|
| cw721-base | 522 | Standard NFT |
| cw721-metadata-onchain | 525 | On-chain metadata |
| cw721-expiration | 523 | Time-based expiry |
| cw721-fixed-price | 524 | ⚠️ Requires CW20 (deferred) |
| cw721-non-transferable | 526 | Soulbound |
| cw2981-royalties | 528 | Royalties at mint time |

## Goal

Support CW721 NFT contract deployment and minting via CLI.

## API Design

### Create Collection

```bash
xion-toolkit asset create \
  --type cw721-base \
  --name "My Collection" \
  --symbol "NFT" \
  [--description "..."] \
  [--code-id 522] \
  [--salt "..."] \
  --output json
```

### Mint Token

```bash
xion-toolkit asset mint \
  --contract xion1... \
  --token-id "1" \
  --owner xion1... \
  [--token-uri "ipfs://..."] \
  [--royalty-address xion1...] \
  [--royalty-percentage 5] \
  --output json
```

### Query Contract

```bash
xion-toolkit asset query \
  --contract xion1... \
  --msg '{"nft_info": {"token_id": "1"}}' \
  --output json
```

### List Types

```bash
xion-toolkit asset types
```

## Implementation Phases

### Phase 1: Core Functionality (Current)

**Goal**: Deploy cw721-base collection and mint tokens

**Tasks**:
- [x] Create `src/asset_builder/mod.rs`
- [x] Create `src/asset_builder/types.rs` with AssetType enum, input/result types
- [x] Create `src/asset_builder/manager.rs` with create/mint/query methods
- [x] Create `src/asset_builder/code_ids.rs` with code ID helpers
- [x] Modify `build.rs` to add asset code IDs to NetworkConfig
- [x] Create `src/cli/asset.rs` with commands
- [x] Update `src/cli/mod.rs` to include asset module
- [x] Update `src/lib.rs` to include asset_builder module
- [x] Add unit tests
- [ ] Test on testnet (requires authentication)

**Files**:
```
src/
├── asset_builder/
│   ├── mod.rs          # NEW
│   ├── types.rs        # NEW
│   ├── manager.rs      # NEW
│   └── code_ids.rs     # NEW
├── cli/
│   ├── asset.rs        # NEW
│   └── mod.rs          # MODIFY
└── build.rs            # MODIFY
```

### Phase 2: All Variants (Next)

- [ ] Add variant-specific instantiate messages
- [ ] Add variant-specific mint messages
- [ ] cw721-metadata-onchain support
- [ ] cw721-expiration support
- [ ] cw721-non-transferable support
- [ ] cw2981-royalties support (royalties at mint time)
- [ ] Add `asset types` command

### Phase 3: Advanced Features (Future)

- [ ] Address prediction (`asset predict`)
- [ ] Batch minting
- [ ] cw721-fixed-price support (requires CW20)
- [ ] Mainnet code ID configuration

## Key Design Decisions

### 1. Code ID Management

**Decision**: Extend NetworkConfig in build.rs

```rust
// build.rs - Added to NetworkConfig
pub cw721_base_code_id: u64,
pub cw721_metadata_onchain_code_id: u64,
pub cw721_expiration_code_id: u64,
pub cw721_fixed_price_code_id: u64,
pub cw721_non_transferable_code_id: u64,
pub cw2981_royalties_code_id: u64,
```

### 2. CW2981 Royalties

**Important**: Royalties are set at **mint time**, not instantiation.

```json
{
  "mint": {
    "token_id": "1",
    "owner": "xion1...",
    "extension": {},
    "royalty_info": {
      "payment_address": "xion1...",
      "share": "0.05"
    }
  }
}
```

### 3. OAuth2 API Encoding

**Critical**: `msg` and `salt` fields must be **number arrays**, not base64.

## Error Codes

| Code | Description |
|------|-------------|
| `INVALID_ASSET_TYPE` | Unknown asset type |
| `CODE_ID_NOT_FOUND` | Code ID not configured for network |
| `MISSING_REQUIRED_FIELD` | Required field not provided |
| `CW20_REQUIRED` | Fixed-price requires CW20 address |
| `INSTANTIATION_FAILED` | Contract deployment failed |
| `MINT_FAILED` | Token minting failed |
| `QUERY_FAILED` | Contract query failed |

## Acceptance Criteria

### Phase 1
- [x] `xion-toolkit asset create --type cw721-base` works
- [x] `xion-toolkit asset mint` works
- [x] `xion-toolkit asset query` works
- [x] `xion-toolkit asset types` lists available types
- [x] Unit tests pass (22 tests)
- [x] `cargo clippy` passes
- [ ] Tested on testnet (requires authentication - deferred to user)

### Phase 2
- [ ] All 5 variants supported (excluding fixed-price)
- [ ] Variant-specific options validated
- [ ] CW2981 royalties at mint time

## Dependencies

- No new dependencies required
- Uses existing OAuth2 API client
- Uses existing treasury instantiate2 patterns

## Reference

- Developer Portal: `src/config/assetBuilder/constants.ts`
- Developer Portal: `src/lib/assetBuilder/asset.ts`
- Toolkit: `src/treasury/manager.rs` (instantiate2 pattern)

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-13 | @project-manager | Plan created | Done |
| 2026-03-13 | @architect | Architecture completed | Done |
| 2026-03-13 | @fullstack-dev | Phase 1 implementation | Done |
| 2026-03-13 | @qa-engineer | QA verification passed | Done |
| 2026-03-13 | @project-manager | Final sign-off | **Done** |
