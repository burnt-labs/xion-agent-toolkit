---
status: Done
created_at: 2026-03-13
updated_at: 2026-03-14
done_at: 2026-03-14
---

# Feature Roadmap: xion-agent-toolkit

This document outlines completed and planned features for the toolkit.

## Completed Features (Phase 1)

| Feature | Priority | Complexity | Status | Completed |
|---------|----------|------------|--------|-----------|
| MetaAccount Info | P1 | Low | ✅ Done | 2026-03-13 |
| Extended Grant Types | P2 | Medium | ✅ Done | 2026-03-13 |
| Batch Operations | P3 | Medium | ✅ Done | 2026-03-13 |
| Asset Builder (CW721) | P4 | High | ✅ Done | 2026-03-14 |

## Future Considerations

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| Transaction Wait (xion-skills) | P3 | Low | Deferred |
| CW20 Token Support | P2 | Medium | Proposed |
| Multi-sig Wallet Support | P3 | High | Proposed |
| IBC Transfer Enhancement | P3 | Medium | Proposed |

---

## 1. MetaAccount Info Command (P1)

### Goal

Add a command to query MetaAccount authenticators for the logged-in user.

### Background

- Developer Portal uses DaoDao Indexer GraphQL to query MetaAccount data
- A logged-in user has exactly **one MetaAccount** (no list needed)
- Authenticators include session keys, their indices, and versions

### API

```bash
# Query current user's MetaAccount info
xion-toolkit account info

# Output
{
  "success": true,
  "address": "xion1abc...",
  "authenticators": [
    {
      "id": "...",
      "type": "secp256k1",
      "authenticator": "...",
      "authenticatorIndex": 0,
      "version": 1
    }
  ],
  "latest_authenticator_id": "..."
}
```

### Implementation

1. **Add new module** `src/account/`
   - `mod.rs` - Module exports
   - `client.rs` - DaoDao Indexer GraphQL client
   - `types.rs` - MetaAccount types

2. **Add CLI command**
   - `src/cli/account.rs` - Account subcommands
   - Integrate with main CLI in `src/cli/mod.rs`

3. **Data Source**
   - DaoDao Indexer GraphQL endpoint
   - Query: `SingleSmartWalletQuery` by address

### Network Configuration

| Network | Indexer URL |
|---------|-------------|
| testnet | `https://indexer.daodao.zone/5.56/xion-testnet-2` |
| mainnet | `https://indexer.daodao.zone/5.56/xion-mainnet-1` |

### Files to Create/Modify

```
src/
├── account/
│   ├── mod.rs          # NEW
│   ├── client.rs       # NEW - GraphQL client
│   └── types.rs        # NEW - MetaAccount types
├── cli/
│   ├── account.rs      # NEW - account subcommand
│   └── mod.rs          # MODIFY - add account module
└── config/
    └── constants.rs    # MODIFY - add indexer URLs
```

### Acceptance Criteria

- [x] `xion-toolkit account info` returns MetaAccount authenticators
- [x] Works on both testnet and mainnet
- [x] Proper error handling when not authenticated
- [x] JSON output for agent consumption

### Completion Notes

**Completed: 2026-03-13**

- Rewrote to use OAuth2 API `/api/v1/me` endpoint instead of DaoDao Indexer
- Removed DaoDao Indexer dependency for cleaner architecture
- See `plans/metaaccount-info.md` for details

---

## 2. Extended Grant Types (P2)

### Goal

Extend CLI to support all 24 message type URLs that Developer Portal supports.

### Current State

CLI supports these grant types via presets:
- `send` (MsgSend)
- `execute` (MsgExecuteContract)
- `instantiate` (MsgInstantiateContract)
- `instantiate2` (MsgInstantiateContract2)
- `delegate`, `undelegate`, `redelegate`, `withdraw-rewards`
- `vote`
- `ibc-transfer`

### Missing Types

| Category | Type URLs to Add |
|----------|------------------|
| Governance | `MsgDeposit`, `MsgSubmitProposal` |
| Authz | `MsgExec`, `MsgRevoke` |
| Feegrant | `MsgGrantAllowance`, `MsgRevokeAllowance` |
| Crisis | `MsgVerifyInvariant`, `MsgSubmitEvidence` |
| Slashing | `MsgUnjail` |
| Vesting | `MsgCreateVestingAccount` |
| TokenFactory | `MsgMint`, `MsgBurn` |

### Implementation

1. **Extend encoding module** (`src/treasury/encoding.rs`)
   - Already supports all authorization types
   - Need to add presets for new message types

2. **Extend CLI presets** (`src/cli/treasury.rs`)
   - Add new preset options to `grant-config add`
   - Example: `--preset gov-deposit`, `--preset gov-submit-proposal`

3. **Update grant-config presets**

```rust
pub enum GrantPreset {
    // Existing
    Send,
    Execute,
    Instantiate,
    Instantiate2,
    Delegate,
    Undelegate,
    Redelegate,
    WithdrawRewards,
    Vote,
    IbcTransfer,
    
    // NEW
    GovDeposit,
    GovSubmitProposal,
    AuthzExec,
    AuthzRevoke,
    FeegrantGrant,
    FeegrantRevoke,
    Unjail,
    // etc.
}
```

### Files to Modify

```
src/
├── cli/
│   └── treasury.rs     # MODIFY - add presets
├── treasury/
│   └── types.rs        # MODIFY - add preset variants
└── config/
    └── constants.rs    # MODIFY - add type URL constants
```

### Acceptance Criteria

- [x] All 24 type URLs supported via CLI
- [x] Proper presets for common use cases
- [x] Documentation updated
- [x] Tests for new presets

### Completion Notes

**Completed: 2026-03-13**

- Added 12 new presets to `PRESET_TYPES` array
- Updated error messages for better UX
- Added 11 unit tests for new presets
- See `plans/extended-grant-types.md` for details

---

## 3. Batch Operations (P3)

### Goal

Support executing multiple messages in a single transaction.

### Background

Developer Portal supports batch transactions for:
- Asset deployment (multiple instantiate messages)
- Treasury configuration (grant + fee config in one tx)

### API Design

```bash
# Execute batch from JSON file
xion-toolkit batch execute --from-file batch.json

# batch.json format
{
  "messages": [
    {
      "type": "execute",
      "contract": "xion1...",
      "msg": { "transfer": { ... } }
    },
    {
      "type": "instantiate",
      "code_id": 1260,
      "label": "...",
      "msg": { ... }
    }
  ]
}
```

### OAuth2 API Compatibility

**Research needed**: Check if OAuth2 API `/api/v1/transaction` supports multiple messages.

Reference: `~/workspace/xion/oauth2-api-service/src/routes/api/transaction/broadcast.ts`

### Implementation Plan

1. **Research Phase**
   - Verify OAuth2 API batch support
   - Check message size limits
   - Test with Developer Portal patterns

2. **If API supports batches**:
   - Add `src/batch/` module
   - Add `batch execute` CLI command
   - Support JSON input format

3. **If API doesn't support batches**:
   - Document limitation
   - Consider sequential execution with rollback

### Files to Create/Modify

```
src/
├── batch/
│   ├── mod.rs          # NEW
│   ├── executor.rs     # NEW
│   └── types.rs        # NEW
├── cli/
│   ├── batch.rs        # NEW
│   └── mod.rs          # MODIFY
```

### Acceptance Criteria

- [x] OAuth2 API batch capability documented
- [x] Batch execute command works
- [x] Proper error handling for partial failures
- [x] Transaction hash returned for all messages

### Completion Notes

**Completed: 2026-03-13**

- Confirmed OAuth2 API supports batch transactions
- Added `src/batch/` module with executor and types
- Added `xion-toolkit batch execute` command
- 23 unit tests passing
- See `plans/batch-operations.md` for details

---

## 4. Asset Builder (P4)

### Goal

Support CW721 NFT contract deployment and minting.

### Contract Variants

| Asset Type | Code ID (Testnet) | Available | Features |
|------------|-------------------|-----------|----------|
| cw721-base | 522 | Yes | Standard NFT |
| cw721-metadata-onchain | 525 | Yes | On-chain metadata |
| cw721-expiration | 523 | Yes | Time-based expiry |
| cw721-fixed-price | 524 | Yes | Fixed price (CW20 only) |
| cw721-non-transferable | 526 | Yes | Soulbound |
| cw2981-royalties | 528 | Yes | Royalty support |

### API Design

```bash
# Deploy NFT collection
xion-toolkit asset deploy \
  --type cw721-base \
  --name "My Collection" \
  --symbol "NFT" \
  --description "..." \
  [--max-supply 10000] \
  [--base-uri "ipfs://..."]

# Mint NFT (to self or recipient)
xion-toolkit asset mint \
  --contract xion1... \
  --token-id "1" \
  --owner xion1... \
  [--token-uri "ipfs://..."] \
  [--royalty-percentage 5] \
  [--royalty-address xion1...]

# Query NFT contract
xion-toolkit asset query \
  --contract xion1... \
  --msg '{"nft_info": {"token_id": "1"}}'
```

### Implementation Plan

1. **Phase 1: Basic Deployment**
   - Add `src/asset/` module
   - Support cw721-base deployment
   - Use instantiate2 for predictable addresses

2. **Phase 2: Minting**
   - Implement mint command
   - Support variant-specific mint messages
   - Handle CW2981 royalty fields

3. **Phase 3: All Variants**
   - Add support for all 6 available variants
   - Variant-specific instantiation messages

### Contract Configuration

Store code IDs and checksums in `src/config/constants.rs`:

```rust
pub const ASSET_CODE_IDS: &[(Network, &str, u64)] = &[
    (Network::Testnet, "cw721-base", 522),
    (Network::Testnet, "cw721-metadata-onchain", 525),
    // ...
];
```

### Files to Create/Modify

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

### Acceptance Criteria

- [x] Deploy cw721-base contract
- [x] Mint tokens to recipient
- [x] Query contract state
- [x] Support CW2981 royalties
- [x] Predictable addresses via instantiate2

### Completion Notes

**Completed: 2026-03-14**

**Phase 1**: Basic CW721 deployment and minting
**Phase 2**: All 5 variants supported (cw721-base, metadata-onchain, expiration, non-transferable, cw2981-royalties)
**Phase 3**: Address prediction and batch minting

- Added `src/asset/` module with deploy, mint, query, and predict-address commands
- 5 CW721 variants supported (fixed-price removed as CW20-only)
- Batch minting support
- Predictable contract addresses via instantiate2
- Skills integration: `skills/xion-asset.sh`
- See `plans/asset-builder.md` for details

---

## 5. Transaction Wait (xion-skills) - Deferred

### Goal

Add transaction wait functionality to xion-skills.

### Background

xion-skills already has `query-tx.sh` for status queries. Wait functionality would poll until confirmation.

### Implementation (in xion-skills repo)

```bash
# Wait for transaction confirmation
bash wait-tx.sh <txhash> [--timeout 60] [--interval 2]
```

### Notes

- This belongs in xion-skills, not xion-agent-toolkit
- xion-agent-toolkit can recommend using xion-skills for this
- No CLI changes needed in this repo

---

## Priority Order

1. **MetaAccount Info** (P1) - Simple, high value
2. **Extended Grant Types** (P2) - Feature parity
3. **Batch Operations** (P3) - Research first
4. **Asset Builder** (P4) - Complex, later phase

---

---

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-14 | @project-manager | All Phase 1 features completed | ✅ Done |

---

## Future Directions

### CW20 Token Support (Proposed)

Extend Asset Builder to support fungible tokens (CW20).

```bash
xion-toolkit asset deploy --type cw20-base --name "My Token" --symbol "TKN" --decimals 6
xion-toolkit asset mint --contract xion1... --recipient xion1... --amount 1000000
```

### Multi-sig Wallet Support (Proposed)

Support for multi-signature treasury management.

### IBC Transfer Enhancement (Proposed)

Improved cross-chain transfer experience with channel auto-discovery.

---

## Next Steps

1. ~~Start with **MetaAccount Info** implementation~~ ✅
2. ~~Create detailed plan document: `plans/metaaccount-info.md`~~ ✅
3. ~~Implement in phases with proper testing~~ ✅
4. **Current**: Skills documentation improvement
5. **Current**: E2E test coverage for new modules
