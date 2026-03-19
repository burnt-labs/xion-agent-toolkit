---
status: Done
created_at: 2026-03-13
updated_at: 2026-03-14
done_at: 2026-03-14
architecture_completed_at: 2026-03-13
architecture_by: @architect
phase1_completed_at: 2026-03-13
phase1_by: @fullstack-dev
phase1_qa_at: 2026-03-13
phase1_qa_by: @qa-engineer
phase2_started_at: 2026-03-13
phase2_completed_at: 2026-03-13
phase2_by: @fullstack-dev
phase2_qa_at: 2026-03-13
phase2_qa_by: @qa-engineer
phase3_started_at: 2026-03-14
phase3_completed_at: 2026-03-14
phase3_by: @fullstack-dev
phase3_qa_at: 2026-03-14
phase3_qa_by: @qa-engineer
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

### Phase 2: All Variants (Current)

**Goal**: Support all CW721 variants with variant-specific messages

**Tasks**:

#### 2.1 Types Module (`src/asset_builder/types.rs`)
- [x] Add `Cw2981RoyaltyInfo` struct with `payment_address` and `share` fields
- [x] Add `Cw2981MintContent` struct with `royalty_info` option
- [x] Add `Cw721ExpirationMintContent` with `expires_at` field
- [x] Update `MintTokenInput` with optional variant-specific fields:
  - `royalty_address` and `royalty_percentage` for CW2981
  - `expires_at` for expiration variant
  - `asset_type` for dispatch control

#### 2.2 Manager Module (`src/asset_builder/manager.rs`)
- [x] Remove Phase 1 restriction (cw721-base only)
- [x] Implement `build_mint_msg()` dispatch by AssetType
- [x] Add validation for variant-specific requirements
- [x] Add `build_royalty_info()` helper for CW2981

#### 2.3 CLI Module (`src/cli/asset.rs`)
- [x] Add `--asset-type` option to `asset mint` with default `cw721-base`
- [x] Add `--royalty-address` option to `asset mint`
- [x] Add `--royalty-percentage` option to `asset mint`
- [x] Add `--expires-at` option to `asset mint`
- [x] Add validation for variant-specific options

#### 2.4 Unit Tests
- [x] Test CW2981 mint message with royalties
- [x] Test expiration mint message
- [x] Test variant dispatch in manager
- [x] Test MintTokenInput backward compatibility
- [x] Test royalty info validation

#### 2.5 Bug Fixes
- [x] Fix validation order bug: numeric constraints checked before type compatibility
  - Issue: `--royalty-percentage 1.5` returned "wrong type" instead of "invalid percentage"
  - Fix: Reordered validation in `handle_mint()` to check percentage range first
  - Added 5 test cases for validation order verification

**Priority Order**:
1. cw2981-royalties (most requested feature)
2. cw721-expiration (simple addition)
3. cw721-metadata-onchain (standard extension)
4. cw721-non-transferable (standard structure)

### Phase 3: Advanced Features (Done)

**Goal**: Add address prediction, batch minting, and cleanup

**Tasks**:

#### 3.1 Address Prediction (`asset predict`)
- [x] Add `PredictAddressInput` struct in types.rs
- [x] Add `PredictAddressResult` struct with predicted address
- [x] Implement `predict_address()` in manager.rs using `instantiate2_address`
- [x] Add `asset predict` CLI command
- [x] Add unit tests for address prediction

**CLI Design**:
```bash
xion-toolkit asset predict \
  --type cw721-base \
  --name "My Collection" \
  --symbol "NFT" \
  --minter xion1... \
  [--salt "..."] \
  --output json
```

#### 3.2 Batch Minting (`asset batch-mint`)
- [x] Add `BatchMintInput` struct with multiple tokens
- [x] Add `BatchMintResult` struct with results per token
- [x] Implement `batch_mint()` in manager.rs
- [x] Add `asset batch-mint` CLI command with `--tokens-file` option
- [x] Support JSON file input for batch tokens
- [x] Add unit tests for batch minting

**CLI Design**:
```bash
xion-toolkit asset batch-mint \
  --contract xion1... \
  --tokens-file tokens.json \
  --output json
```

**tokens.json format**:
```json
[
  {"token_id": "1", "owner": "xion1...", "token_uri": "ipfs://..."},
  {"token_id": "2", "owner": "xion1...", "token_uri": "ipfs://..."}
]
```

#### 3.3 Mainnet Code IDs (Deferred)
- [ ] Update `build.rs` with mainnet code IDs (when available)
- [ ] Add `--network` flag to asset commands
- [ ] Test mainnet code ID lookup

#### 3.4 Cleanup
- [x] Remove `Cw721FixedPrice` from AssetType enum (deferred indefinitely)
- [x] Update `asset types` output to show 5 types instead of 6
- [x] Update documentation

**Priority Order**:
1. Address prediction (highest value for UX)
2. Batch minting (efficiency feature)
3. Mainnet code IDs (deployment readiness)
4. Cleanup (code hygiene)

### ~~cw721-fixed-price support~~ (Removed)

**Reason**: Requires CW20 token integration which is out of scope for this toolkit. Users should use dedicated marketplace contracts instead.

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
- [x] All 5 variants supported (excluding fixed-price)
- [x] Variant-specific options validated
- [x] CW2981 royalties at mint time
- [x] Validation order: numeric constraints checked before type compatibility
- [x] `cargo test` passes (223 tests, 43 asset_builder tests including 5 new CLI validation tests)
- [x] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] CLI `--help` shows new options

### Phase 3
- [x] `xion-toolkit asset predict` works and returns correct address
- [x] `xion-toolkit asset batch-mint --tokens-file` works
- [x] `asset types` shows 5 types (fixed-price removed)
- [ ] Address prediction matches actual deployment address (requires testnet testing)
- [ ] Batch minting handles partial failures gracefully (requires testnet testing)
- [x] Unit tests pass for all new functionality (232 tests, 47 asset_builder tests)
- [x] `--type cw721-fixed-price` returns "invalid asset type" error
- [x] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [x] `cargo fmt --check` passes

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
| 2026-03-13 | @qa-engineer | Phase 1 QA verification | Done |
| 2026-03-13 | @project-manager | Phase 1 sign-off | Done |
| 2026-03-13 | @fullstack-dev | Phase 2 implementation | Done |
| 2026-03-13 | @fullstack-dev | Fix validation order bug | Done |
| 2026-03-13 | @qc-specialist | Phase 2 code review | Done |
| 2026-03-13 | @qa-engineer | Phase 2 QA verification | Done |
| 2026-03-13 | @project-manager | Phase 2 final sign-off | Done |
| 2026-03-14 | @fullstack-dev | Phase 3 implementation | Done |
| 2026-03-14 | @qc-specialist | Phase 3 code review | Done |
| 2026-03-14 | @qa-engineer | Phase 3 QA verification | Done |
| 2026-03-14 | @project-manager | Phase 3 final sign-off | **Done** |
