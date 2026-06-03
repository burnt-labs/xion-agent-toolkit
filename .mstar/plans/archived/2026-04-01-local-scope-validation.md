# Local Scope Validation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Add local scope validation so that MGR API calls fail fast with a clear error message when the user's token lacks the required scopes, instead of waiting for a server round-trip.

**Architecture:** Add a `scope` field to `TokenResponse` and `UserCredentials` to persist scopes from the OAuth2 server. Introduce a `has_scope()` helper on `UserCredentials`. Add a pre-flight scope check in `prepare_api_client()` (the single chokepoint for all `oauth2 client` subcommands) that validates the token has the required scopes before making any HTTP request. Return `OAuthClientError::InsufficientScope` locally if scopes are missing, with a hint to re-login with `--dev-mode`.

**Tech Stack:** Rust, serde, anyhow, existing error types (`XionError`, `OAuthClientError`)

---

## File Structure

| File | Responsibility |
|------|---------------|
| `src/api/oauth2_api.rs` | Add `scope` field to `TokenResponse` |
| `src/config/schema.rs` | Add `scope` field to `UserCredentials` |
| `src/oauth/client.rs` | Propagate `scope` in TokenResponse → UserCredentials conversion during login |
| `src/oauth/token_manager.rs` | Propagate `scope` during token refresh; return `UserCredentials` instead of bare `String` from `get_valid_token()` |
| `src/oauth/mod.rs` | Update `TokenManager::get_valid_token()` return type re-export if needed |
| `src/cli/oauth2_client.rs` | Add scope validation in `prepare_api_client()` |
| `src/shared/error.rs` | Enhance `InsufficientScope` hint to include `--dev-mode` guidance |

---

### Task 1: Add `scope` field to `TokenResponse`

**Files:**
- Modify: `src/api/oauth2_api.rs:49-72`

- [x] **Step 1: Add scope field to TokenResponse**

In `src/api/oauth2_api.rs`, add a `scope` field to the `TokenResponse` struct. Use `#[serde(default)]` so existing server responses that omit `scope` don't break deserialization.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub refresh_token_expires_in: Option<i64>,
    #[serde(default)]
    pub refresh_token_expires_at: Option<String>,
    pub token_type: String,
    #[serde(default)]
    pub xion_address: Option<String>,
    /// Space-separated OAuth2 scopes granted by the authorization server.
    /// Example: "xion:identity:read xion:blockchain:read xion:transactions:submit xion:mgr:read xion:mgr:write"
    #[serde(default)]
    pub scope: Option<String>,
}
```

- [x] **Step 2: Verify compilation**

Run: `cargo build`
Expected: Compiles without errors

- [x] **Step 3: Commit**

```bash
git add src/api/oauth2_api.rs
git commit -m "feat(auth): add scope field to TokenResponse"
```

---

### Task 2: Add `scope` field and helper to `UserCredentials`

**Files:**
- Modify: `src/config/schema.rs:24-42`

- [x] **Step 1: Add scope field and has_scope helper**

In `src/config/schema.rs`, add the `scope` field and a `has_scope()` method:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: String,
    #[serde(default)]
    pub refresh_token_expires_at: Option<String>,
    pub xion_address: Option<String>,
    /// Space-separated OAuth2 scopes granted by the authorization server.
    /// None if credentials were created before scope tracking was added.
    #[serde(default)]
    pub scope: Option<String>,
}

impl UserCredentials {
    /// Check if the credentials contain a specific OAuth2 scope.
    /// Scopes are stored as a space-separated string (e.g., "xion:identity:read xion:blockchain:read").
    /// Returns false if scope field is None (legacy credentials) or scope is not found.
    pub fn has_scope(&self, required_scope: &str) -> bool {
        self.scope
            .as_ref()
            .map(|s| s.split_whitespace().any(|scope| scope == required_scope))
            .unwrap_or(false)
    }

    /// Check if the credentials contain all specified OAuth2 scopes.
    pub fn has_all_scopes(&self, required_scopes: &[&str]) -> bool {
        required_scopes.iter().all(|s| self.has_scope(s))
    }
}
```

- [x] **Step 2: Verify compilation**

Run: `cargo build`
Expected: Compiles without errors

- [x] **Step 3: Commit**

```bash
git add src/config/schema.rs
git commit -m "feat(auth): add scope field and has_scope helper to UserCredentials"
```

---

### Task 3: Propagate scope in login and refresh flows

**Files:**
- Modify: `src/oauth/client.rs` (login flow, around line 343)
- Modify: `src/oauth/token_manager.rs` (refresh flow, around line 327)

- [x] **Step 1: Propagate scope in login flow**

In `src/oauth/client.rs`, find the `UserCredentials` construction during login (search for `UserCredentials {`). Add the `scope` field:

```rust
let credentials = UserCredentials {
    access_token: token_response.access_token,
    refresh_token: token_response.refresh_token,
    expires_at: token_response.expires_at.clone(),
    refresh_token_expires_at: token_response.refresh_token_expires_at,
    xion_address: token_response.xion_address,
    scope: token_response.scope,
};
```

There may be two locations — one for the login flow and one for the test helper. Update all occurrences.

- [x] **Step 2: Propagate scope in refresh flow**

In `src/oauth/token_manager.rs`, find the `UserCredentials` construction during `refresh_access_token()`. Add the `scope` field:

```rust
let credentials = UserCredentials {
    access_token: new_token.access_token,
    refresh_token: new_token.refresh_token,
    expires_at: new_token.expires_at.clone(),
    refresh_token_expires_at: new_token.refresh_token_expires_at,
    xion_address: new_token.xion_address,
    scope: new_token.scope,
};
```

- [x] **Step 3: Run tests to verify no breakage**

Run: `cargo test --all-features`
Expected: All tests pass

- [x] **Step 4: Commit**

```bash
git add src/oauth/client.rs src/oauth/token_manager.rs
git commit -m "feat(auth): propagate scope from TokenResponse to UserCredentials"
```

---

### Task 4: Return `UserCredentials` from `get_valid_token()`

**Files:**
- Modify: `src/oauth/token_manager.rs` (method signature + both return sites)
- Modify: `src/cli/oauth2_client.rs` (caller)

- [x] **Step 1: Change `get_valid_token()` return type**

In `src/oauth/token_manager.rs`, change `get_valid_token()` to return `UserCredentials` instead of bare `String`. This preserves scope metadata for callers.

Change signature from:
```rust
pub async fn get_valid_token(&self) -> Result<String>
```
to:
```rust
pub async fn get_valid_token(&self) -> Result<UserCredentials>
```

Update all return sites in the method (there are 3 `Ok(...)` return points):
- Change `Ok(credentials.access_token)` to `Ok(credentials)`
- Change `Ok(fresh_credentials.access_token)` to `Ok(fresh_credentials)`
- Change `Ok(new_credentials.access_token)` to `Ok(new_credentials)`

- [x] **Step 2: Update callers of `get_valid_token()`**

Search the codebase for all calls to `get_valid_token()`. Update each call site to extract `.access_token` from the `UserCredentials` result.

The key call site is `src/cli/oauth2_client.rs:prepare_api_client()` (line ~276):
```rust
// Before:
let access_token = oauth_client.get_valid_token().await?;

// After:
let credentials = oauth_client.get_valid_token().await?;
let access_token = credentials.access_token.clone();
```

Search for any other callers and update them similarly.

- [x] **Step 3: Run tests**

Run: `cargo test --all-features`
Expected: All tests pass

- [x] **Step 4: Commit**

```bash
git add src/oauth/token_manager.rs src/cli/oauth2_client.rs
# Also add any other files that call get_valid_token() if found
git commit -m "refactor(auth): return UserCredentials from get_valid_token for scope access"
```

---

### Task 5: Add scope validation constants and pre-flight check

**Files:**
- Modify: `src/cli/oauth2_client.rs` (scope validation in `prepare_api_client()`)
- Modify: `src/shared/error.rs` (enhance InsufficientScope hint)

- [x] **Step 1: Add MGR scope constants**

In `src/cli/oauth2_client.rs`, add constants at the module level:

```rust
/// OAuth2 scopes required for Manager API operations.
const MGR_REQUIRED_SCOPES: &[&str] = &["xion:mgr:read", "xion:mgr:write"];
```

- [x] **Step 2: Add pre-flight scope check in `prepare_api_client()`**

After obtaining credentials but before returning, validate scopes:

```rust
async fn prepare_api_client(_ctx: &ExecuteContext) -> Result<(String, MgrApiClient)> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::shared::error::{OAuthClientError, XionError};

    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;

    let oauth_client = OAuthClient::new(network_config.clone())?;

    // Check authentication
    if !oauth_client.is_authenticated()? {
        anyhow::bail!("Not authenticated. Please run 'xion-toolkit auth login' first.");
    }

    let credentials = oauth_client.get_valid_token().await?;

    // Pre-flight scope validation: fail fast if token lacks required scopes
    if !credentials.has_all_scopes(MGR_REQUIRED_SCOPES) {
        let missing: Vec<&str> = MGR_REQUIRED_SCOPES
            .iter()
            .filter(|s| !credentials.has_scope(s))
            .copied()
            .collect();

        return Err(XionError::from(OAuthClientError::InsufficientScope {
            message: format!(
                "Missing required scopes: {}. Re-login with --dev-mode to obtain manager API permissions.",
                missing.join(", ")
            ),
        })
        .into());
    }

    let access_token = credentials.access_token.clone();

    // Never log the access token
    debug!("Obtained valid access token for MGR API call");

    let mgr_client = MgrApiClient::new(network_config.oauth_api_url)?;

    Ok((access_token, mgr_client))
}
```

- [x] **Step 3: Enhance InsufficientScope hint in error.rs**

In `src/shared/error.rs`, find the hint for `XionErrorCode::EOAUTHCLIENT010` and update it to include `--dev-mode`:

```rust
XionErrorCode::EOAUTHCLIENT010 => {
    "Re-authorize with xion:mgr:read or xion:mgr:write scope. Use --dev-mode flag: xion-toolkit auth login --dev-mode"
}
```

- [x] **Step 4: Run tests**

Run: `cargo test --all-features`
Expected: All tests pass

- [x] **Step 5: Commit**

```bash
git add src/cli/oauth2_client.rs src/shared/error.rs
git commit -m "feat(auth): add pre-flight scope validation for MGR API calls"
```

---

### Task 6: Add tests and verify

**Files:**
- Modify: `src/config/schema.rs` (tests for has_scope)
- Modify: `src/cli/oauth2_client.rs` or a new test module (integration test for scope check)

- [x] **Step 1: Add unit tests for UserCredentials::has_scope**

Add tests to the bottom of `src/config/schema.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_scope_with_valid_scope() {
        let creds = UserCredentials {
            access_token: "test".to_string(),
            refresh_token: "test".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
            refresh_token_expires_at: None,
            xion_address: None,
            scope: Some("xion:identity:read xion:blockchain:read xion:transactions:submit".to_string()),
        };
        assert!(creds.has_scope("xion:identity:read"));
        assert!(creds.has_scope("xion:blockchain:read"));
        assert!(creds.has_scope("xion:transactions:submit"));
        assert!(!creds.has_scope("xion:mgr:read"));
    }

    #[test]
    fn test_has_scope_with_dev_mode_scopes() {
        let creds = UserCredentials {
            access_token: "test".to_string(),
            refresh_token: "test".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
            refresh_token_expires_at: None,
            xion_address: None,
            scope: Some("xion:identity:read xion:blockchain:read xion:transactions:submit xion:mgr:read xion:mgr:write".to_string()),
        };
        assert!(creds.has_all_scopes(&["xion:mgr:read", "xion:mgr:write"]));
    }

    #[test]
    fn test_has_scope_with_none_scope() {
        let creds = UserCredentials {
            access_token: "test".to_string(),
            refresh_token: "test".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
            refresh_token_expires_at: None,
            xion_address: None,
            scope: None,
        };
        assert!(!creds.has_scope("xion:identity:read"));
        assert!(!creds.has_all_scopes(&["xion:mgr:read"]));
    }

    #[test]
    fn test_has_all_scopes_partial_match() {
        let creds = UserCredentials {
            access_token: "test".to_string(),
            refresh_token: "test".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
            refresh_token_expires_at: None,
            xion_address: None,
            scope: Some("xion:identity:read xion:mgr:read".to_string()),
        };
        // Has mgr:read but not mgr:write
        assert!(!creds.has_all_scopes(&["xion:mgr:read", "xion:mgr:write"]));
        // Has both
        assert!(creds.has_all_scopes(&["xion:identity:read", "xion:mgr:read"]));
    }
}
```

- [x] **Step 2: Run full test suite**

Run: `cargo test --all-features`
Expected: All tests pass (including new tests)

- [x] **Step 3: Run clippy**

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: Zero warnings

- [x] **Step 4: Run format check**

Run: `cargo fmt -- --check`
Expected: No diffs

- [x] **Step 5: Commit**

```bash
git add src/config/schema.rs
git commit -m "test(auth): add unit tests for UserCredentials::has_scope"
```

---

### Task 7: Update documentation

**Files:**
- Modify: `docs/cli-reference.md`
- Modify: `docs/QUICK-REFERENCE.md`
- Modify: `skills/xion-oauth2/SKILL.md` (if agent skills exist at this path)

- [x] **Step 1: Update cli-reference.md**

In the `auth login` section, under the `--dev-mode` description or Notes, add information about local scope validation:

```markdown
**Notes:**
- Opens browser automatically for authentication
- Stores encrypted credentials in `~/.xion-toolkit/credentials/`
- Refresh tokens valid for 30 days
- OAuth2 scopes are persisted with credentials and validated locally
- Manager API commands (`oauth2 client *`) require `xion:mgr:read` and `xion:mgr:write` scopes
  — use `--dev-mode` to obtain these scopes during login
  — if scopes are missing, the CLI will fail fast with a clear error message
```

- [x] **Step 2: Update QUICK-REFERENCE.md**

Add a note in the auth section about scope validation behavior.

- [x] **Step 3: Commit**

```bash
git add docs/cli-reference.md docs/QUICK-REFERENCE.md
git commit -m "docs(auth): document local scope validation behavior"
```
