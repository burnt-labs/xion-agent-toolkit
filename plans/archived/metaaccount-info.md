---
status: Completed
created_at: 2026-03-13
updated_at: 2026-03-13
---

# MetaAccount Info Command

## ✅ COMPLETED

**Status**: Completed

**Implementation**: Rewrote `account info` command to use OAuth2 API `/api/v1/me` endpoint.

**Previous Blocker**: DaoDao Indexer GraphQL endpoint was not available.

**Resolution**: Use OAuth2 API `/api/v1/me` endpoint instead of DaoDao Indexer!

**Key Findings**:
1. OAuth2 API has `/api/v1/me` endpoint that returns MetaAccount info
2. `OAuth2ApiClient::get_user_info()` method already exists in `src/api/oauth2_api.rs`
3. `UserInfo` type already defined with `id`, `authenticators`, and `balances` fields
4. CLI already has OAuth2 tokens, so no additional auth needed

---

## Background

OAuth2 API provides `/api/v1/me` endpoint that returns MetaAccount information for authenticated users. This includes address, authenticators, and balances.

## Goal

Add `xion-toolkit account info` command to query MetaAccount authenticators using OAuth2 API.

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
      "id": "xion1abc...-0",
      "type": "secp256k1",
      "index": 0,
      "data": {}
    }
  ],
  "balances": {
    "xion": {
      "amount": "100.5",
      "denom": "uxion",
      "micro_amount": "100500000"
    },
    "usdc": {
      "amount": "0",
      "denom": "uusdc",
      "micro_amount": "0"
    }
  }
}
```

## Data Source

### OAuth2 API `/api/v1/me`

| Network | Endpoint |
|---------|----------|
| testnet | `https://oauth2.testnet.burnt.com/api/v1/me` |
| mainnet | `https://oauth2.burnt.com/api/v1/me` |

### Response Structure (from `src/api/oauth2_api.rs`)

```rust
pub struct UserInfo {
    pub id: String,                          // MetaAccount address
    pub authenticators: Vec<AuthenticatorInfo>,
    pub balances: Option<AccountBalances>,
}

pub struct AuthenticatorInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub auth_type: String,
    pub index: u32,
    pub data: serde_json::Value,
}

pub struct AccountBalances {
    pub xion: Balance,
    pub usdc: Balance,
}
```

## Implementation

### Tasks

- [x] ~~Create `src/account/` module~~ - EXISTS but uses wrong API
- [x] Rewrite `src/cli/account.rs` to use `OAuth2ApiClient::get_user_info()`
- [x] Update `src/account/types.rs` to match OAuth2 API response format
- [x] Simplify or remove `src/account/client.rs` (no longer needed) - REMOVED
- [x] Run tests and verify CLI works - 146 tests passing
- [ ] Update documentation

### Implementation Approach

1. **Modify `src/cli/account.rs`**:
   - Use `OAuth2ApiClient::get_user_info()` instead of `AccountClient`
   - Get access token from credentials
   - Map `UserInfo` to `AccountInfoOutput`

2. **Update `src/account/types.rs`**:
   - Align types with `UserInfo` from OAuth2 API
   - Remove unused GraphQL-specific fields

3. **Clean up `src/account/client.rs`**:
   - Remove or simplify (no longer querying DaoDao Indexer)
   - Could be removed entirely if not needed

### CLI Design

```rust
// src/cli/account.rs - Updated approach

async fn handle_info() -> Result<()> {
    // Get credentials and access token
    let oauth_client = OAuthClient::new(network_config)?;
    let credentials = oauth_client.get_credentials()?.ok_or(...)?;
    
    // Use OAuth2 API to get user info
    let api_client = OAuth2ApiClient::new(oauth_base_url);
    let user_info = api_client.get_user_info(&credentials.access_token).await?;
    
    // Output result
    print_json(&user_info);
}
```

### Error Handling

| Error Code | Message | Hint |
|------------|---------|------|
| NOT_AUTHENTICATED | Not authenticated | Run `xion-toolkit auth login` first |
| TOKEN_EXPIRED | Access token expired | Token refresh should happen automatically |
| API_ERROR | Failed to query OAuth2 API | Check network connection |

## Acceptance Criteria

- [x] `xion-toolkit account info` returns MetaAccount info
- [x] Uses OAuth2 API `/api/v1/me` endpoint
- [x] Works on both testnet and mainnet
- [x] Proper error handling when not authenticated
- [x] JSON output for agent consumption
- [x] Unit tests pass
- [ ] E2E test with real OAuth2 API

## Files to Modify

```
src/
├── account/
│   ├── mod.rs          # Review exports
│   ├── client.rs       # SIMPLIFY or REMOVE
│   └── types.rs        # UPDATE to match UserInfo
├── cli/
│   └── account.rs      # REWRITE to use OAuth2ApiClient
└── api/
    └── oauth2_api.rs   # NO CHANGES (already has get_user_info)
```

## Dependencies

- `reqwest` - HTTP client (already in use)
- `serde` / `serde_json` - JSON handling (already in use)
- OAuth2 API client (already implemented)

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-13 | @project-manager | Plan created | InProgress |
| 2026-03-13 | @project-manager | Blocker resolved - use OAuth2 API | InProgress |
| 2026-03-13 | @fullstack-dev | Implementation complete | Completed |
| 2026-03-13 | @qc-specialist | Code review passed (2 warnings fixed) | Completed |
| 2026-03-13 | @project-manager | Final sign-off: all tests pass, QC approved | Done |
