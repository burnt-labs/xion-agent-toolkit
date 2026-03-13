---
status: Blocked
created_at: 2026-03-13
updated_at: 2026-03-13
---

# MetaAccount Info Command

## ⚠️ BLOCKED

**Status**: Blocked pending API research

**Blocker**: DaoDao Indexer API endpoint verification required

**Reason**: During implementation, we discovered that the GraphQL endpoint documented in Developer Portal's `src/lib/queries.ts` appears to be **unused/legacy code**. The actual DaoDao Indexer at `daodaoindexer.burnt.com` seems to be a **REST-only API**.

**Investigation Findings**:
1. GraphQL queries in `queries.ts` use Apollo Client's `gql` tag but are NOT actively used in the current codebase
2. The `/graphql` endpoint returns 404 on the indexer
3. Developer Portal uses REST endpoints like `/contract/{admin}/xion/account/treasuries` instead

**Resolution Options**:
1. Research the correct REST API endpoint for SmartAccount/MetaAccount queries
2. Contact Xion team to confirm the correct API structure
3. Skip this feature if the API is not publicly available

**Temporary Code**: Implementation files exist in `src/account/` but have syntax errors and are not functional. The code has been committed but needs API research to complete.

---

## Background

Developer Portal queries MetaAccount data from DaoDao Indexer GraphQL. A logged-in OAuth2 user has exactly one MetaAccount associated with their authenticator.

## Goal

Add `xion-toolkit account info` command to query MetaAccount authenticators.

## API

```bash
# Query current user's MetaAccount info
xion-toolkit account info

# Output
{
  "success": true,
  "address": "xion1abc...",
  "authenticators": [
    {
      "id": "auth_123",
      "type": "secp256k1",
      "authenticator": "MFYwEAYHKo...",
      "authenticator_index": 0,
      "version": 1
    }
  ],
  "latest_authenticator_id": "auth_123"
}
```

## Data Source

### DaoDao Indexer GraphQL

| Network | Endpoint |
|---------|----------|
| testnet | `https://indexer.daodao.zone/5.56/xion-testnet-2/graphql` |
| mainnet | `https://indexer.daodao.zone/5.56/xion-mainnet-1/graphql` |

### GraphQL Query

```graphql
fragment SmartAccountFragment on SmartAccountAuthenticator {
  id
  type
  authenticator
  authenticatorIndex
  version
}

query SingleSmartWalletQuery($id: String!) {
  smartAccount(id: $id) {
    id
    latestAuthenticatorId
    authenticators {
      nodes {
        ...SmartAccountFragment
      }
    }
  }
}
```

## Implementation

### Tasks

- [ ] Create `src/account/` module
  - [ ] `mod.rs` - Module exports
  - [ ] `types.rs` - MetaAccount types (SmartAccount, Authenticator)
  - [ ] `client.rs` - GraphQL client for DaoDao Indexer
- [ ] Create `src/cli/account.rs` - Account subcommand handler
- [ ] Update `src/cli/mod.rs` - Add account module
- [ ] Update `src/config/constants.rs` - Add indexer URLs
- [ ] Add tests
- [ ] Update documentation

### Type Definitions

```rust
// src/account/types.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartAccount {
    pub id: String,
    pub latest_authenticator_id: Option<String>,
    pub authenticators: Vec<Authenticator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authenticator {
    pub id: String,
    #[serde(rename = "type")]
    pub auth_type: String,
    pub authenticator: String,
    pub authenticator_index: i32,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfoOutput {
    pub success: bool,
    pub address: String,
    pub authenticators: Vec<Authenticator>,
    pub latest_authenticator_id: Option<String>,
}
```

### CLI Design

```rust
// src/cli/account.rs

#[derive(Subcommand)]
pub enum AccountCommands {
    /// Show current user's MetaAccount info
    Info,
}
```

### Error Handling

| Error Code | Message | Hint |
|------------|---------|------|
| NOT_AUTHENTICATED | Not authenticated | Run `xion-toolkit auth login` first |
| ACCOUNT_NOT_FOUND | MetaAccount not found | No MetaAccount associated with this user |
| INDEXER_ERROR | Failed to query indexer | Check network connection |

## Acceptance Criteria

- [ ] `xion-toolkit account info` returns MetaAccount authenticators
- [ ] Works on both testnet and mainnet
- [ ] Proper error handling when not authenticated
- [ ] JSON output for agent consumption
- [ ] Unit tests for types and client
- [ ] Integration test with real indexer

## Files to Create/Modify

```
src/
├── account/
│   ├── mod.rs          # NEW
│   ├── client.rs       # NEW
│   └── types.rs        # NEW
├── cli/
│   ├── account.rs      # NEW
│   └── mod.rs          # MODIFY
└── config/
    └── constants.rs    # MODIFY
```

## Dependencies

- `reqwest` - HTTP client (already in use)
- `serde` / `serde_json` - JSON handling (already in use)

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-13 | @project-manager | Plan created | InProgress |
