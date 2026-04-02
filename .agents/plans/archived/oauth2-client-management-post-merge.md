---
status: Done
created_at: 2026-03-31
updated_at: 2026-03-31
---

# OAuth2 Client Management — Toolkit Implementation Plan

**Goal:** Deliver CLI-first OAuth2 client lifecycle management in `xion-agent-toolkit`, consuming the merged dual-bearer MGR API from `oauth2-api-service`.

**Architecture:** A new `src/api/mgr_api.rs` HTTP adapter wraps the MGR REST endpoints. A new `src/cli/oauth2_client.rs` command group exposes CRUD + manager + ownership commands. Authentication reuses the existing `OAuthClient::get_valid_token()` pipeline. Errors map to a new `OAuthClientError` variant range (`EOAUTHCLIENT001`–`EOAUTHCLIENT020`) in `shared/error.rs`.

**Tech Stack:** Rust, `clap`, `reqwest`, `serde`/`serde_json`, `thiserror`, existing `OAuthClient` + `TokenManager` for auth.

**Working branch:** `feature/oauth2-client-mgmt` (from `main`)

## Progress Tracking

### Phase Gate Checklist

| Gate | Status |
|------|--------|
| `specify` | ✅ Done |
| `clarify` | ✅ Done — scope blocker resolved via local dev mode (ADR-008) |
| `plan` | ✅ Done |
| `tasks` | ✅ Done |
| `implement` | ✅ Done |

### Phase Status

| Phase | Status | Files | Tests |
|-------|--------|-------|-------|
| Local Dev Mode | ✅ Done | `build.rs`, `.env.example` | — |
| Phase 1: Foundation | ✅ Done | `src/shared/error.rs`, `src/api/mod.rs`, `src/api/mgr_api.rs` (skeleton) | ✅ 21 pass |
| Phase 2: API Client | ✅ Done | `src/api/mgr_api.rs` (full impl) | ✅ 25 pass |
| Phase 3: CLI Commands | ✅ Done | `src/cli/oauth2_client.rs`, `src/cli/mod.rs`, `src/main.rs` | ✅ All pass |
| Phase 4: Secret Redaction | ✅ Done | `src/cli/oauth2_client.rs` | ✅ Implemented |
| Phase 5: Skill Schemas | ✅ Done | 10 JSON files + docs/ | — |
| Phase 6: Final Tests + CI | ✅ Done | — | ✅ 444 pass |
| QC Tri-Review | ✅ Done | — | 3 reviewers, consolidated decision: Approve |
| QA Verification | ✅ Done | — | 8/8 criteria PASS, 444 tests, 0 warnings |

### Known Issue — Phase 3 Clap Bug (RESOLVED)

~~`Get`, `Delete`, `TransferOwnership` variants use inline struct syntax which clap treats as `#[command(subcommand)]`.~~ **Fixed** by converting to `#[derive(Args)]` structs (`GetArgs`, `DeleteArgs`, `TransferOwnershipArgs`). All tests pass.

### Post-Implementation: SKILL.md Creation

After QA sign-off, a `SKILL.md` was created following the `xion-treasury` skill pattern:

| File | Action | Description |
|------|--------|-------------|
| `skills/xion-oauth2-client/SKILL.md` | CREATE | 403 lines — full skill document with 35+ triggers, all 10 commands, secret redaction, error handling, parameter collection workflow, schema references |

---

## A. API Contract — MGR API Endpoints

All endpoints are under the `oauth2-api-service` base URL (e.g., `https://oauth2.testnet.burnt.com`). Authentication uses `Authorization: Bearer <access_token>`. Scope enforcement is OAuth-mode-only; Session JWT mode is permissive in Beta.

### Common Response Shape (Error)

```json
{
  "error": "Error message",
  "code": "ERROR_CODE",
  "statusCode": 400
}
```

### A1. GET `/mgr-api/me` — Get Current User

| Field | Value |
|-------|-------|
| **Auth** | Required (Bearer token) |
| **Scope (OAuth mode)** | `xion:mgr:read` |
| **Handler** | `clients/me.ts` |

**Response 200:**
```json
{
  "success": true,
  "userId": "xion1abc123...",
  "user": {
    "userId": "xion1abc123...",
    "xionAddress": "xion1abc123...",
    "authenticatorType": "EthWallet",
    "authenticatorIndex": 0,
    "network": "testnet"
  }
}
```

Notes: `authenticatorType` and `authenticatorIndex` are `undefined` in OAuth mode.

### A2. GET `/mgr-api/clients` — List OAuth Clients

| Field | Value |
|-------|-------|
| **Auth** | Required |
| **Scope** | `xion:mgr:read` |
| **Handler** | `clients/list.ts` |

**Query params:** `limit` (string, optional), `cursor` (string, optional)

**Response 200:**
```json
{
  "success": true,
  "items": [ClientInfo],
  "cursor": "next_page_cursor",
  "count": 10
}
```

**ClientInfo shape:**
```json
{
  "clientId": "client_abc123",
  "redirectUris": ["https://example.com/callback"],
  "clientName": "My App",
  "clientUri": "https://myapp.com",
  "logoUri": "https://myapp.com/logo.png",
  "policyUri": "https://myapp.com/privacy",
  "tosUri": "https://myapp.com/terms",
  "jwksUri": "https://myapp.com/.well-known/jwks.json",
  "contacts": ["admin@myapp.com"],
  "grantTypes": ["authorization_code"],
  "responseTypes": ["code"],
  "tokenEndpointAuthMethod": "none",
  "extension": {
    "bindedTreasury": "xion1abc123...",
    "owner": "user_123",
    "managers": ["user_456", "user_789"]
  },
  "role": "owner"
}
```

### A3. POST `/mgr-api/clients` — Create OAuth Client

| Field | Value |
|-------|-------|
| **Auth** | Required |
| **Scope** | `xion:mgr:write` |
| **Handler** | `clients/create.ts` |

**Request body:**
```json
{
  "redirectUris": ["https://example.com/callback"],
  "clientName": "My App",
  "clientUri": "https://myapp.com",
  "logoUri": "https://myapp.com/logo.png",
  "policyUri": "https://myapp.com/privacy",
  "tosUri": "https://myapp.com/terms",
  "jwksUri": "https://myapp.com/.well-known/jwks.json",
  "contacts": ["admin@myapp.com"],
  "tokenEndpointAuthMethod": "none",
  "bindedTreasury": "xion1abc123...",
  "owner": "user_123",
  "managers": []
}
```

Required: `redirectUris` (min 1, must be valid URLs), `bindedTreasury`.
Optional: `clientName`, `clientUri`, `logoUri`, `policyUri`, `tosUri`, `jwksUri`, `contacts` (must be valid emails), `tokenEndpointAuthMethod` (enum: `none`, `client_secret_basic`, `client_secret_post`, default `none`), `owner` (defaults to authenticated userId), `managers` (default `[]`).

**Response 201:**
```json
{
  "success": true,
  "client": ClientInfo,
  "clientSecret": "secret_xyz789"
}
```

⚠️ **Security:** `clientSecret` is returned ONLY on creation. Must be redacted in toolkit JSON output by default.

### A4. GET `/mgr-api/clients/{clientId}` — Get Client

| Field | Value |
|-------|-------|
| **Auth** | Required (middleware) |
| **Scope** | `xion:mgr:read` |
| **Handler** | `clients/get.ts` |

Handler does NOT call `getUserId()` — lookup only. Auth is enforced at middleware level.

**Response 200:**
```json
{
  "success": true,
  "client": ClientInfo
}
```

**Response 404:**
```json
{ "error": "Client not found: ...", "code": "CLIENT_NOT_FOUND", "statusCode": 404 }
```

### A5. PUT `/mgr-api/clients/{clientId}` — Update Client

| Field | Value |
|-------|-------|
| **Auth** | Required |
| **Scope** | `xion:mgr:write` |
| **Handler** | `clients/update.ts` |

**Request body (all optional):**
```json
{
  "redirectUris": ["https://example.com/callback"],
  "clientName": "My App",
  "clientUri": "https://myapp.com",
  "logoUri": "https://myapp.com/logo.png",
  "policyUri": "https://myapp.com/privacy",
  "tosUri": "https://myapp.com/terms",
  "jwksUri": "https://myapp.com/.well-known/jwks.json",
  "contacts": ["admin@myapp.com"]
}
```

URI fields accept empty string `""` to clear (delete) the value. `bindedTreasury` cannot be modified.

**Response 200:**
```json
{
  "success": true,
  "client": ClientInfo
}
```

### A6. DELETE `/mgr-api/clients/{clientId}` — Delete Client

| Field | Value |
|-------|-------|
| **Auth** | Required |
| **Scope** | `xion:mgr:write` |
| **Handler** | `clients/delete.ts` |

Only the owner can delete. **No request body.**

**Response 200:**
```json
{ "success": true, "message": "Client deleted successfully" }
```

**Response 403:** `{ "error": "...", "code": "ONLY_OWNER_ALLOWED", "statusCode": 403 }`

### A7. GET `/mgr-api/clients/{clientId}/extension` — Get Extension

| Field | Value |
|-------|-------|
| **Auth** | Required (middleware) |
| **Scope** | `xion:mgr:read` |
| **Handler** | `clients/extension.ts` |

**Response 200:**
```json
{
  "success": true,
  "extension": {
    "bindedTreasury": "xion1abc123...",
    "owner": "user_123",
    "managers": ["user_456", "user_789"]
  }
}
```

### A8. PATCH `/mgr-api/clients/{clientId}/extension` — Update Extension

| Field | Value |
|-------|-------|
| **Auth** | Required |
| **Scope** | `xion:mgr:write` |
| **Handler** | `clients/extension.ts` |

**Request body:**
```json
{
  "managers": ["user_456"]
}
```

Only `managers` is mutable. `bindedTreasury` cannot be changed.

**Response 200:**
```json
{
  "success": true,
  "client": ClientInfo
}
```

### A9. POST `/mgr-api/clients/{clientId}/managers` — Add Manager

| Field | Value |
|-------|-------|
| **Auth** | Required |
| **Scope** | `xion:mgr:write` |
| **Handler** | `clients/managers.ts` |

**Request body:**
```json
{ "managerUserId": "user_456" }
```

**Response 200:**
```json
{ "success": true, "message": "Manager added successfully" }
```

### A10. DELETE `/mgr-api/clients/{clientId}/managers/{managerUserId}` — Remove Manager

| Field | Value |
|-------|-------|
| **Auth** | Required |
| **Scope** | `xion:mgr:write` |
| **Handler** | `clients/managers.ts` |

No request body. Both `clientId` and `managerUserId` are path params.

**Response 200:**
```json
{ "success": true, "message": "Manager removed successfully" }
```

### A11. POST `/mgr-api/clients/{clientId}/transfer-ownership` — Transfer Ownership

| Field | Value |
|-------|-------|
| **Auth** | Required |
| **Scope** | `xion:mgr:write` |
| **Handler** | `clients/transfer-ownership.ts` |

**Request body:**
```json
{ "newOwner": "user_789" }
```

**Response 200:**
```json
{ "success": true, "message": "Ownership transferred successfully" }
```

### A12. GET `/mgr-api/treasuries` — List User Treasuries

| Field | Value |
|-------|-------|
| **Auth** | Required |
| **Scope** | `xion:mgr:read` |
| **Handler** | `treasury/treasuries.ts` |

Fetches from DaoDao indexer by authenticated user address.

**Response 200:**
```json
{
  "success": true,
  "treasuries": [
    {
      "address": "xion1abc123...",
      "admin": "xion1admin...",
      "params": {
        "display_url": "https://...",
        "redirect_url": "https://...",
        "icon_url": "https://..."
      }
    }
  ],
  "count": 5
}
```

### A13. GET `/mgr-api/utils/treasury/{address}` — Query Treasury

| Field | Value |
|-------|-------|
| **Auth** | Required (middleware) |
| **Scope** | `xion:mgr:read` |
| **Handler** | `treasury/index.ts` |

No user context needed beyond middleware. Queries on-chain data.

**Query params:** `grants` (string, `"true"`), `fee` (string, `"true"`), `admin` (string, `"true"`)

**Response 200:**
```json
{
  "success": true,
  "treasury": {
    "address": "xion1abc123...",
    "params": { "display_url": "...", "redirect_url": "...", "icon_url": "..." },
    "balance": "1000000",
    "admin": "xion1admin...",
    "pendingAdmin": "xion1pending...",
    "feeConfig": {},
    "grantConfigs": [{ "typeUrl": "...", "grantConfig": {} }]
  }
}
```

### Backend Error Code Reference

| HTTP | Backend Code | Toolkit Mapping |
|------|-------------|-----------------|
| 400 | `BAD_REQUEST` | `EOAUTHCLIENT001` |
| 400 | `CLIENT_ID_REQUIRED` | `EOAUTHCLIENT002` |
| 400 | `REDIRECT_URIS_REQUIRED` | `EOAUTHCLIENT003` |
| 400 | `BINDED_TREASURY_REQUIRED` | `EOAUTHCLIENT004` |
| 400 | `OWNER_REQUIRED` | `EOAUTHCLIENT005` |
| 400 | `INVALID_GRANT_TYPE` | `EOAUTHCLIENT006` |
| 400 | `MANAGER_USER_ID_REQUIRED` | `EOAUTHCLIENT007` |
| 401 | `AUTHENTICATION_REQUIRED` | `EOAUTHCLIENT008` |
| 401 | `USER_NOT_FOUND` | `EOAUTHCLIENT009` |
| 403 | `INSUFFICIENT_SCOPE` | `EOAUTHCLIENT010` |
| 403 | `ONLY_OWNER_ALLOWED` | `EOAUTHCLIENT011` |
| 404 | `CLIENT_NOT_FOUND` | `EOAUTHCLIENT012` |
| 404 | `CLIENT_EXTENSION_NOT_FOUND` | `EOAUTHCLIENT013` |
| 404 | `TREASURY_NOT_FOUND` | `EOAUTHCLIENT014` |
| 500 | `INTERNAL_SERVER_ERROR` | `EOAUTHCLIENT015` |
| 500 | `TREASURY_FETCH_ERROR` | `EOAUTHCLIENT016` |
| 500 | `TREASURY_QUERY_ERROR` | `EOAUTHCLIENT017` |
| 500 | `UNKNOWN_NETWORK` | `EOAUTHCLIENT018` |

---

## B. Key Question: Can Toolkit Access Tokens Use `/mgr-api/*`?

### Analysis

The toolkit's `auth login` flow obtains an OAuth access token from the `@cloudflare/workers-oauth-provider` via the authorization code grant. The token format is **opaque** (`userId:grantId:secretPart`), validated by KV lookup + AES-GCM decryption in `oauth-token-validator.ts`.

The composite `mgrAuthMiddleware` (line 124 of `mgr-auth.ts`) has this flow:

1. Check `executionCtx.props.xionAddress` test bypass → skip if set
2. If `MGR_DUAL_BEARER_ENABLED=false` → session-only auth
3. Try Session JWT validation via `validateSessionAuth()`
4. Try OAuth access token validation via `validateOAuthAccessToken()`
5. For OAuth mode: **enforce scopes** — check if `grantedScopes` contains `xion:mgr:read` or `xion:mgr:write`

### Critical Finding

The current `auth login` flow requests scopes defined in the OAuth consent screen. The existing `SCOPE_NAMES` constants are:
- `xion:identity:read`
- `xion:blockchain:read`
- `xion:transactions:submit`

The **new** `MGR_SCOPE_NAMES` are:
- `xion:mgr:read`
- `xion:mgr:write`

**Conclusion: The toolkit's current `auth login` access token does NOT include `xion:mgr:read` / `xion:mgr:write` scopes.** Attempting to call `/mgr-api/*` with the existing token will receive `403 INSUFFICIENT_SCOPE`.

### Resolution — Resolved via Local Dev Mode (ADR-008)

**Decision:** Use local `oauth2-api-service` instance for development. The toolkit already supports pointing at a local server via the `XION_TESTNET_OAUTH_API_URL` build-time environment variable (added in `build.rs`). The `oauth_discovery.rs` module already allows `http://localhost` URLs.

**Implementation:**
1. Start local oauth2-api-service: `pnpm wrangler dev --port 8787`
2. Set in `.env`: `XION_TESTNET_OAUTH_API_URL=http://localhost:8787`
3. Use ClientID `upAWDXvkmtCU5RVB` (same as testnet)
4. `cargo build` — toolkit now points at localhost
5. `xion-toolkit auth login` — obtains token with all scopes including `xion:mgr:read/write`

**Production path:** Coordinate with backend team to add `xion:mgr:read xion:mgr:write` to the toolkit's pre-configured OAuth client's default scope set (one-line config change).

---

## C. Toolkit Architecture

### C1. New Modules

```
src/
├── api/
│   ├── oauth2_api.rs          # EXISTING — OAuth token exchange
│   └── mgr_api.rs             # NEW — MGR API HTTP adapter
├── cli/
│   ├── mod.rs                 # MODIFY — add OAuth2ClientCommands variant
│   └── oauth2_client.rs       # NEW — CLI command definitions + handlers
└── shared/
    └── error.rs               # MODIFY — add OAuthClientError + error codes
```

### C2. `src/api/mgr_api.rs` — MGR API Client

```rust
pub struct MgrApiClient {
    base_url: String,
    http_client: reqwest::Client,
}

impl MgrApiClient {
    pub fn new(base_url: String) -> Self;

    // Core CRUD
    pub async fn list_clients(&self, access_token: &str, limit: Option<u32>, cursor: Option<&str>) -> XionResult<ClientListResponse>;
    pub async fn create_client(&self, access_token: &str, request: CreateClientRequest) -> XionResult<CreateClientResponse>;
    pub async fn get_client(&self, access_token: &str, client_id: &str) -> XionResult<ClientResponse>;
    pub async fn update_client(&self, access_token: &str, client_id: &str, request: UpdateClientRequest) -> XionResult<ClientResponse>;
    pub async fn delete_client(&self, access_token: &str, client_id: &str) -> XionResult<DeleteResponse>;

    // Extension
    pub async fn get_extension(&self, access_token: &str, client_id: &str) -> XionResult<ExtensionResponse>;
    pub async fn update_extension(&self, access_token: &str, client_id: &str, request: UpdateExtensionRequest) -> XionResult<ClientResponse>;

    // Managers
    pub async fn add_manager(&self, access_token: &str, client_id: &str, manager_user_id: &str) -> XionResult<MessageResponse>;
    pub async fn remove_manager(&self, access_token: &str, client_id: &str, manager_user_id: &str) -> XionResult<MessageResponse>;

    // Ownership
    pub async fn transfer_ownership(&self, access_token: &str, client_id: &str, new_owner: &str) -> XionResult<MessageResponse>;

    // User info
    pub async fn get_me(&self, access_token: &str) -> XionResult<MeResponse>;
}
```

**Authentication:** All methods accept `access_token: &str` as parameter. The caller (CLI handler) is responsible for obtaining a valid token via `OAuthClient::get_valid_token()`.

**Retry strategy:** Reuse `shared::retry::with_retry()` with `RetryConfig::default()`. All methods should wrap HTTP calls in retry for transient errors (5xx, 429).

**Error mapping:** HTTP responses are parsed and mapped to `XionError::OAuthClient(OAuthClientError::...)`. The `BackendError` JSON shape (`{ error, code, statusCode }`) is parsed to extract the backend `code` for precise mapping.

### C3. Type Definitions

All types are defined in `mgr_api.rs` with `Serialize`/`Deserialize`:

```rust
// Request types
pub struct CreateClientRequest { redirect_uris: Vec<String>, client_name: Option<String>, ..., binded_treasury: String, owner: Option<String>, managers: Option<Vec<String>> }
pub struct UpdateClientRequest { redirect_uris: Option<Vec<String>>, client_name: Option<String>, ... }
pub struct UpdateExtensionRequest { managers: Option<Vec<String>> }

// Response types
pub struct ClientListResponse { success: bool, items: Vec<ClientInfo>, cursor: Option<String>, count: usize }
pub struct ClientResponse { success: bool, client: ClientInfo }
pub struct CreateClientResponse { success: bool, client: ClientInfo, client_secret: Option<String> }
pub struct ExtensionResponse { success: bool, extension: ClientExtension }
pub struct MessageResponse { success: bool, message: String }
pub struct MeResponse { success: bool, user_id: String, user: MeUserInfo }

// Shared types
pub struct ClientInfo { client_id: String, redirect_uris: Vec<String>, ..., extension: Option<ClientExtension>, role: Option<String> }
pub struct ClientExtension { binded_treasury: String, owner: String, managers: Vec<String> }
```

### C4. Reused Existing Capabilities

| Capability | Source | Usage |
|-----------|--------|-------|
| Token acquisition | `OAuthClient::get_valid_token()` | Every MGR API call passes a valid token |
| HTTP client | `reqwest::Client` (same pattern as `OAuth2ApiClient`) | `MgrApiClient` builds its own client |
| Retry | `shared::retry::with_retry()` + `RetryConfig` | Transient error recovery |
| Error framework | `XionError` + `XionErrorCode` | New `OAuthClientError` variant + codes `EOAUTHCLIENT001`–`EOAUTHCLIENT018` |
| Output formatting | `utils::output::print_formatted()` | CLI handler JSON output |
| Network config | `NetworkConfig.oauth_api_url` | Base URL for MGR API |
| CLI context | `ExecuteContext` | Output format + network |

### C5. Error Mapping Pipeline

```
Backend HTTP Response
  → Parse JSON body { error, code, statusCode }
  → Match backend code string to OAuthClientError variant
  → Wrap in XionError::OAuthClient(...)
  → CLI handler formats as { success: false, error: { code, message, remediation } }
```

---

## D. CLI Command Design

### D1. Command Group

```
xion-toolkit oauth2 client <subcommand>
```

This nests under a new `OAuth2` top-level command (parallel to existing `Treasury`, `Auth`, etc.):

```
xion-toolkit oauth2 <subcommand>
```

Where `oauth2` has a single subcommand group `client`.

### D2. Subcommands

| Command | Flags | Description |
|---------|-------|-------------|
| `oauth2 client list` | `--limit <N>`, `--cursor <str>`, `--output json` | List OAuth clients |
| `oauth2 client create` | `--redirect-uris <urls>` (required, comma-separated), `--client-name <str>`, `--client-uri <url>`, `--logo-uri <url>`, `--policy-uri <url>`, `--tos-uri <url>`, `--jwks-uri <url>`, `--contacts <emails>` (comma-separated), `--auth-method <none|client_secret_basic|client_secret_post>`, `--treasury <addr>` (required), `--owner <str>`, `--managers <ids>` (comma-separated), `--json-input <file>` | Create OAuth client |
| `oauth2 client get` | `<client-id>` (positional) | Get client details |
| `oauth2 client update` | `<client-id>` (positional), `--redirect-uris`, `--client-name`, `--client-uri`, `--logo-uri`, `--policy-uri`, `--tos-uri`, `--jwks-uri`, `--contacts`, `--json-input <file>` | Update client |
| `oauth2 client delete` | `<client-id>` (positional) | Delete client |
| `oauth2 client extension` | `<client-id>` (positional) | Get extension data |
| `oauth2 client extension update` | `<client-id>` (positional), `--managers <ids>` | Update extension |
| `oauth2 client managers add` | `<client-id>` (positional), `--manager-id <str>` (required) | Add manager |
| `oauth2 client managers remove` | `<client-id>` (positional), `--manager-id <str>` (required) | Remove manager |
| `oauth2 client transfer-ownership` | `<client-id>` (positional), `--new-owner <str>` (required) | Transfer ownership |

### D3. JSON Output Shapes

**`oauth2 client list` (stdout):**
```json
{
  "success": true,
  "items": [ClientInfo],
  "cursor": "...",
  "count": 5
}
```

**`oauth2 client create` (stdout):**
```json
{
  "success": true,
  "client": ClientInfo,
  "clientSecret": "********"
}
```

Note: `clientSecret` is redacted by default. Use `--show-secret` flag to reveal.

**`oauth2 client delete` (stdout):**
```json
{ "success": true, "message": "Client deleted successfully" }
```

**Error output (stderr for human, stdout for `--output json`):**
```json
{
  "success": false,
  "error": {
    "code": "EOAUTHCLIENT012",
    "message": "Client not found: client_abc123",
    "remediation": "Check the client ID and try again"
  }
}
```

### D4. `--json-input` Flag

For complex request bodies (create, update), support `--json-input <file>` to read the request body from a JSON file. This is consistent with existing `treasury` commands using `--config`. When `--json-input` is provided, individual flags are ignored.

---

## E. Skill Schemas

All schemas live under `skills/xion-oauth2-client/schemas/`:

| Schema File | Command | Key Parameters |
|-------------|---------|----------------|
| `list.json` | `oauth2 client list` | `limit` (optional integer), `cursor` (optional string) |
| `create.json` | `oauth2 client create` | `redirect-uris` (required, array), `treasury` (required, xion-address), `client-name`, `owner`, `managers`, `auth-method` (enum), `contacts`, URIs... |
| `get.json` | `oauth2 client get` | `client-id` (required, string) |
| `update.json` | `oauth2 client update` | `client-id` (required), optional fields same as create minus treasury |
| `delete.json` | `oauth2 client delete` | `client-id` (required, string) |
| `extension.json` | `oauth2 client extension` | `client-id` (required, string) |
| `extension-update.json` | `oauth2 client extension update` | `client-id` (required), `managers` (optional array) |
| `managers-add.json` | `oauth2 client managers add` | `client-id` (required), `manager-id` (required) |
| `managers-remove.json` | `oauth2 client managers remove` | `client-id` (required), `manager-id` (required) |
| `transfer-ownership.json` | `oauth2 client transfer-ownership` | `client-id` (required), `new-owner` (required) |

### E1. `create.json` Example

```json
{
  "command": "oauth2 client create",
  "description": "Create a new OAuth2 client bound to a treasury",
  "parameters": [
    {
      "name": "redirect-uris",
      "type": "array",
      "required": true,
      "description": "OAuth redirect URIs (comma-separated or JSON array in --json-input)",
      "notes": "Must be valid HTTPS URLs"
    },
    {
      "name": "treasury",
      "type": "string",
      "required": true,
      "format": "xion-address",
      "description": "Treasury contract address to bind this client to"
    },
    {
      "name": "client-name",
      "type": "string",
      "required": false,
      "description": "Human-readable client name"
    },
    {
      "name": "owner",
      "type": "string",
      "required": false,
      "description": "Client owner user ID (defaults to authenticated user)"
    },
    {
      "name": "managers",
      "type": "array",
      "required": false,
      "default": [],
      "description": "Manager user IDs"
    },
    {
      "name": "auth-method",
      "type": "enum",
      "required": false,
      "enum": ["none", "client_secret_basic", "client_secret_post"],
      "default": "none",
      "description": "Token endpoint authentication method"
    },
    {
      "name": "contacts",
      "type": "array",
      "required": false,
      "description": "Contact email addresses"
    },
    {
      "name": "client-uri",
      "type": "string",
      "required": false,
      "format": "url",
      "description": "Client homepage URL"
    },
    {
      "name": "logo-uri",
      "type": "string",
      "required": false,
      "format": "url",
      "description": "Client logo URL"
    },
    {
      "name": "policy-uri",
      "type": "string",
      "required": false,
      "format": "url",
      "description": "Privacy policy URL"
    },
    {
      "name": "tos-uri",
      "type": "string",
      "required": false,
      "format": "url",
      "description": "Terms of service URL"
    },
    {
      "name": "jwks-uri",
      "type": "string",
      "required": false,
      "format": "url",
      "description": "JWKS endpoint URL"
    },
    {
      "name": "json-input",
      "type": "file",
      "required": false,
      "description": "JSON file with full request body",
      "conflicts_with": ["redirect-uris", "treasury", "client-name", "owner", "managers", "auth-method", "contacts", "client-uri", "logo-uri", "policy-uri", "tos-uri", "jwks-uri"]
    },
    {
      "name": "show-secret",
      "type": "boolean",
      "required": false,
      "default": false,
      "description": "Show client secret in output (default: redacted)"
    }
  ],
  "validation_rules": [
    "redirect-uris must contain at least one valid HTTPS URL",
    "treasury must be a valid Xion address",
    "contacts must be valid email addresses",
    "If --json-input is provided, all other flags are ignored",
    "IMPORTANT: Schema validation cannot enforce array minimum length - AI agents must ensure redirect-uris has at least 1 entry"
  ],
  "errors": [
    { "code": "EOAUTHCLIENT008", "description": "Not authenticated", "remediation": "Run 'xion-toolkit auth login' first" },
    { "code": "EOAUTHCLIENT010", "description": "Insufficient scope for MGR API", "remediation": "Re-authorize with xion:mgr:write scope" },
    { "code": "EOAUTHCLIENT004", "description": "Treasury address required", "remediation": "Provide --treasury with a valid Xion address" },
    { "code": "EOAUTHCLIENT011", "description": "Not the treasury admin", "remediation": "Only the treasury admin can create clients" }
  ],
  "examples": [
    {
      "description": "Create client with minimum params",
      "params": {
        "redirect-uris": "https://example.com/callback",
        "treasury": "xion1abc123..."
      }
    },
    {
      "description": "Create client with all options",
      "params": {
        "redirect-uris": "https://example.com/callback",
        "treasury": "xion1abc123...",
        "client-name": "My DApp",
        "auth-method": "none",
        "managers": "user_456,user_789",
        "show-secret": true
      }
    }
  ]
}
```

---

## F. Implementation Phases

### Phase 1: Foundation — Types + Error Codes + API Client Skeleton ✅ DONE

**Owner:** @fullstack-dev | **Completed:** 2026-03-31

**File changes:**

| File | Action | Description |
|------|--------|-------------|
| `src/shared/error.rs` | MODIFY | ✅ Added `OAuthClientError` enum (11 variants) + `EOAUTHCLIENT001`–`EOAUTHCLIENT018` to `XionErrorCode` |
| `src/shared/exit_codes.rs` | MODIFY | ✅ Added 18 exit code constants (160-177) |
| `src/api/mod.rs` | MODIFY | ✅ Added `pub mod mgr_api;` + `pub use mgr_api::MgrApiClient;` |

**Tests:** 21 pass (map_error coverage for all 18 backend codes + serialization)

### Phase 2: API Client — Full Method Implementation ✅ DONE

**Owner:** @fullstack-dev | **Completed:** 2026-03-31

**File changes:**

| File | Action | Description |
|------|--------|-------------|
| `src/api/mgr_api.rs` | CREATE | ✅ 1751 lines — MgrApiClient with 13 public methods, all request/response types, wiremock integration tests |

**All 13 endpoints implemented:**
- [x] `GET /mgr-api/me`
- [x] `GET /mgr-api/clients`
- [x] `POST /mgr-api/clients`
- [x] `GET /mgr-api/clients/{clientId}`
- [x] `PUT /mgr-api/clients/{clientId}`
- [x] `DELETE /mgr-api/clients/{clientId}`
- [x] `GET /mgr-api/clients/{clientId}/extension`
- [x] `PATCH /mgr-api/clients/{clientId}/extension`
- [x] `POST /mgr-api/clients/{clientId}/managers`
- [x] `DELETE /mgr-api/clients/{clientId}/managers/{managerUserId}`
- [x] `POST /mgr-api/clients/{clientId}/transfer-ownership`
- [x] `GET /mgr-api/treasuries`
- [x] `GET /mgr-api/utils/treasury/{address}`

**Tests:** 25 pass (wiremock integration tests for success + error cases)

### Phase 3: CLI Commands ✅ DONE

**Owner:** @fullstack-dev | **Completed:** 2026-03-31

**File changes:**

| File | Action | Description |
|------|--------|-------------|
| `src/cli/mod.rs` | MODIFY | ✅ Added `OAuth2(OAuth2Commands)` variant + dispatch |
| `src/cli/oauth2_client.rs` | CREATE | ✅ 846 lines — 10 command handlers, secret redaction |
| `src/main.rs` | MODIFY | ✅ Updated main to handle new command |

**Clap bug fixed:** `Get`, `Delete`, `TransferOwnership` converted from inline struct syntax to `#[derive(Args)]` structs (`GetArgs`, `DeleteArgs`, `TransferOwnershipArgs`). All tests pass.

### Phase 4: Secret Redaction ✅ DONE

**Owner:** @fullstack-dev | **Completed:** 2026-03-31

**Status:** Implemented in `src/cli/oauth2_client.rs`. `--show-secret` flag for opt-in; default `"********"` redaction.

### Phase 5: Skill Schemas + Documentation ✅ DONE

**Owner:** @fullstack-dev, @prompt-engineer | **Completed:** 2026-03-31

**File changes:**

| File | Action | Description |
|------|--------|-------------|
| `skills/xion-oauth2-client/schemas/list.json` | CREATE | Schema for `list` command |
| `skills/xion-oauth2-client/schemas/create.json` | CREATE | Schema for `create` command |
| `skills/xion-oauth2-client/schemas/get.json` | CREATE | Schema for `get` command |
| `skills/xion-oauth2-client/schemas/update.json` | CREATE | Schema for `update` command |
| `skills/xion-oauth2-client/schemas/delete.json` | CREATE | Schema for `delete` command |
| `skills/xion-oauth2-client/schemas/extension.json` | CREATE | Schema for `extension get` |
| `skills/xion-oauth2-client/schemas/extension-update.json` | CREATE | Schema for `extension update` |
| `skills/xion-oauth2-client/schemas/managers-add.json` | CREATE | Schema for `managers add` |
| `skills/xion-oauth2-client/schemas/managers-remove.json` | CREATE | Schema for `managers remove` |
| `skills/xion-oauth2-client/schemas/transfer-ownership.json` | CREATE | Schema for `transfer-ownership` |
| `skills/xion-oauth2-client/SKILL.md` | CREATE | 403-line skill document (triggers, workflows, commands, error handling) |
| `docs/cli-reference.md` | MODIFY | Add OAuth2 Client section |
| `docs/QUICK-REFERENCE.md` | MODIFY | Add OAuth2 Client quick reference |
| `docs/ERROR-CODES.md` | MODIFY | Add `EOAUTHCLIENT001`–`EOAUTHCLIENT018` |

### Phase 6: Final Tests + CI ✅ DONE

**Owner:** @fullstack-dev | **Completed:** 2026-03-31

**Results:**
- `cargo fmt` ✅ clean
- `cargo clippy --all-targets --all-features -- -D warnings` ✅ 0 warnings
- `cargo test` ✅ 444 tests passed
- Shell completions verified: bash/zsh/fish all include full `oauth2 client` subcommand tree

---

## G. Technical Decision Records

### G1. Authentication Mode — OAuth Bearer Token (ADR-001)

**Decision:** Use the same OAuth access token obtained via `auth login` for MGR API calls.

**Rationale:**
- The backend's composite `mgrAuthMiddleware` already supports OAuth tokens (merged in `oauth2-api-service`).
- No new authentication flow needed in the toolkit.
- Token lifecycle (refresh, expiry) is handled by existing `TokenManager`.

**Constraint:** The token must include `xion:mgr:read`/`xion:mgr:write` scopes. This requires coordination with the backend team to add these scopes to the toolkit's OAuth client configuration.

**Fallback:** Graceful `403 INSUFFICIENT_SCOPE` handling with remediation message.

### G2. Scope Strategy — Read/Write Dichotomy (ADR-002)

**Decision:** Map GET methods to `xion:mgr:read` scope, POST/PUT/PATCH/DELETE to `xion:mgr:write` scope.

**Rationale:**
- Matches backend implementation (`getRequiredScope()` in `mgr-auth.ts`).
- Read/write is sufficient for Beta. A future `xion:mgr:admin` scope for ownership transfer can be added.
- Data-driven scope map makes adding new scopes low-cost.

### G3. Secret Handling — Default Redaction (ADR-003)

**Decision:** `clientSecret` is redacted by default (`"********"`). Raw secret only available via explicit `--show-secret` flag.

**Rationale:**
- Client secrets are high-value credentials. Leaking them in JSON output or logs is a security risk.
- Redaction-by-default with opt-in reveal follows security best practices.
- The `--show-secret` flag satisfies the legitimate use case where users need to copy the secret during initial setup.

**Implementation:**
- `redact_client_secret()` replaces the value before serialization
- `tracing::debug!` never logs secret values
- `#[serde(skip_serializing)]` is NOT used — the field exists in the type but the value is replaced before output

### G4. HTTP Client — Separate `MgrApiClient` Instance (ADR-004)

**Decision:** Create a new `MgrApiClient` struct in `src/api/mgr_api.rs` rather than extending `OAuth2ApiClient`.

**Rationale:**
- `OAuth2ApiClient` is specific to token exchange (`/oauth/token`, `/api/v1/me`). MGR API endpoints are a different domain.
- Separate client keeps concerns isolated. `OAuth2ApiClient` can evolve independently for token management.
- Same `reqwest::Client` builder pattern for consistency.
- Base URL is the same (`oauth_api_url`), but the path prefix is different (`/mgr-api/` vs `/oauth/` and `/api/v1/`).

### G5. Error Code Range — `EOAUTHCLIENT001`–`EOAUTHCLIENT018` (ADR-005)

**Decision:** Add 18 new error codes under the "OAuth Client" domain in `XionErrorCode`.

**Rationale:**
- Follows existing convention (EAUTH, ETREASURY, EASSET, EBATCH, ECONFIG, ENETWORK, ETX, EFAUCET).
- Each backend error code maps to exactly one toolkit error code.
- Sequential numbering within the range allows future expansion.

### G6. CLI Nesting — `oauth2 client` (ADR-006)

**Decision:** Create a new `OAuth2` top-level command group with `client` subcommand group.

**Rationale:**
- Avoids polluting the top-level namespace.
- Future expansion: `oauth2 token`, `oauth2 scope`, etc.
- Consistent with `treasury` having subgroups (`grant-config`, `fee-config`, `admin`).
- The `oauth2` prefix distinguishes from the existing `auth` command (which handles login/refresh).

### G7. `--json-input` for Complex Bodies (ADR-007)

**Decision:** Support `--json-input <file>` for create and update commands.

**Rationale:**
- Create request has 12+ optional fields. Specifying all via flags is unwieldy.
- Consistent with `treasury create --config <file>` pattern.
- When `--json-input` is provided, all individual flags are ignored (prevents ambiguity).
- The JSON file must match the `CreateClientRequest` / `UpdateClientRequest` schema exactly.

### G8. Local Dev Mode — Compile-Time API URL Override (ADR-008)

**Decision:** Support `XION_TESTNET_OAUTH_API_URL` environment variable in `build.rs` to point toolkit at a local `oauth2-api-service` instance.

**Rationale:**
- Follows existing pattern (`XION_TESTNET_OAUTH_CLIENT_ID` env var in build.rs).
- `oauth_discovery.rs` already allows `http://localhost` URLs (security exception for development).
- Minimal change: 3 lines in `build.rs` + documentation in `.env.example`.
- Enables local development and testing without waiting for backend config changes.

**Usage:**
```env
# .env
XION_TESTNET_OAUTH_API_URL=http://localhost:8787
```
```bash
cargo build  # toolkit now uses local oauth2-api-service
```

---

## Acceptance Criteria

- [x] All 13 MGR API endpoints have precise contracts verified against handler code
- [x] Toolkit access token compatibility with `/mgr-api/*` is analyzed and resolution path defined
- [x] CLI commands follow existing patterns (arg naming, output format, error structure)
- [x] Phase plan is precise to file and function level
- [x] Secret redaction policy is defined and enforced
- [x] Error codes cover all backend error responses
- [x] Skill schemas created for all 10 commands
- [x] `cargo fmt`, `cargo clippy`, `cargo test` pass

## Risks and Mitigations

| # | Risk | Severity | Mitigation |
|---|------|----------|------------|
| R1 | Access token lacks `xion:mgr:*` scopes | HIGH | Coordinate with backend team to add scopes to default grant (Section B). Implement graceful 403 handling as interim. |
| R2 | Backend contract drift between repos | MEDIUM | Lock contract in Phase 1. Add contract snapshot test that validates type shapes against backend Zod schemas. |
| R3 | Secret leakage in debug paths | HIGH | Redaction policy + explicit audit of all `debug!()`/`info!()` logging. Add unit test that asserts no secret in output. |
| R4 | URI field empty-string handling | LOW | Backend uses `""` to mean "clear this field". Toolkit must pass empty strings through without stripping. Document in CLI help text. |
| R5 | `managers` field type mismatch | LOW | Backend expects `string[]`. Toolkit `--managers` flag uses comma-separated string parsing. Ensure consistent serialization. |

## Linear Execution Mapping (Reuse)

Reopen existing canceled issues rather than creating duplicates:

- **ENG-1574**: API contract definition → Phase 1
- **ENG-1573**: toolkit service adapter → Phase 2
- **ENG-1568**: CLI commands (create/update/list/get) → Phase 3
- **ENG-1570**: secret handling and output redaction → Phase 4
- **ENG-1571**: parameter schemas for skill/agent → Phase 5
- **ENG-1575**: unit + integration tests → Phase 6
- **ENG-1569**: CI quality gates → Phase 6
- **ENG-1572**: docs updates → Phase 5
- **ENG-1576**: rotation + revoke/deactivate flow → Deferred (Phase 4+)

Closeout after merge:

- Mark ENG-1577~ENG-1581 as `Done` (service-side tasks completed by merged PR).
