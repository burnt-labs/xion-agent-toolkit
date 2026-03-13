---
status: InProgress
created_at: 2026-03-13
updated_at: 2026-03-13
---

# Feature Roadmap: Next Steps for xion-agent-toolkit

This document outlines planned features based on comparison with Developer Portal and user feedback.

## Overview

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| MetaAccount Info | P1 | Low | Pending |
| Extended Grant Types | P2 | Medium | Pending |
| Batch Operations | P3 | Medium | Pending |
| Asset Builder | P4 | High | Pending |
| Transaction Wait (xion-skills) | P3 | Low | Deferred |

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

- [ ] `xion-toolkit account info` returns MetaAccount authenticators
- [ ] Works on both testnet and mainnet
- [ ] Proper error handling when not authenticated
- [ ] JSON output for agent consumption

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

- [ ] All 24 type URLs supported via CLI
- [ ] Proper presets for common use cases
- [ ] Documentation updated
- [ ] Tests for new presets

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

- [ ] OAuth2 API batch capability documented
- [ ] If supported: batch execute command works
- [ ] Proper error handling for partial failures
- [ ] Transaction hash returned for all messages

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

- [ ] Deploy cw721-base contract
- [ ] Mint tokens to recipient
- [ ] Query contract state
- [ ] Support CW2981 royalties
- [ ] Predictable addresses via instantiate2

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

## Next Steps

1. Start with **MetaAccount Info** implementation
2. Create detailed plan document: `plans/metaaccount-info.md`
3. Implement in phases with proper testing
