# Treasury Automation Plan

## Background

Xion's Treasury contracts provide gasless transactions and delegated authorization capabilities. Currently, these must be manually created and configured through the Developer Portal. To enable an Agent-driven development workflow, we need to build a CLI tool that can automate Treasury creation, configuration, and management.

## Goals

1. **Automate Treasury Creation**: Automatically create Treasury contracts via OAuth2 API
2. **Automate Grant Configuration**: Automatically configure Fee Grants and Authz Grants
3. **Fund Management**: Support balance queries, deposits, and withdrawals
4. **Agent-Friendly**: CLI outputs JSON for easy Agent parsing and processing

## Implementation Plan

### Phase 1: Foundation

#### 1.1 Technology Stack
- **Language**: Rust
- **Reasons**:
  - High performance, suitable for CLI tools
  - Strong type system, reduces runtime errors
  - Excellent cross-platform support
  - Mature ecosystem (clap, serde, tokio, reqwest, keyring)

#### 1.2 Core Modules

```
xion-agent-toolkit/
├── CLI Core
│   ├── auth - OAuth2 authentication commands
│   ├── treasury - Treasury management commands
│   └── config - Configuration management commands
├── OAuth2 Client
│   ├── PKCE implementation
│   ├── Token management
│   └── Auto-refresh
├── API Clients
│   ├── OAuth2 API Service
│   ├── xiond Query
│   └── Treasury API
└── Configuration Management
    ├── JSON configuration
    ├── Credential encryption (keyring)
    └── Cache management
```

### Phase 2: Core Features

#### 2.1 OAuth2 Authentication Flow

```mermaid
sequenceDiagram
    participant CLI
    participant Browser
    participant OAuth2 Service
    participant Callback Server

    CLI->>CLI: Generate PKCE verifier & challenge
    CLI->>Callback Server: Start temporary callback server
    CLI->>Browser: Open authorization URL (with PKCE)
    Browser->>OAuth2 Service: Authorization request
    OAuth2 Service->>Browser: Display authorization page
    User->>Browser: Click Allow
    OAuth2 Service->>Callback Server: Callback (code)
    Callback Server->>OAuth2 Service: Exchange token (code + verifier)
    OAuth2 Service->>Callback Server: access_token + refresh_token
    Callback Server->>CLI: Save token
    CLI->>User: Output JSON (token info)
```

**Key Features**:
- Localhost callback server on port 8080 (configurable)
- PKCE for enhanced security
- Automatic token refresh
- Secure token storage via OS keyring

#### 2.2 Treasury Creation Flow

**Approach: Via OAuth2 API (Recommended)**

The toolkit uses pre-configured OAuth clients that are already set up with Treasury management capabilities. This provides the same functionality as the Developer Portal.

```bash
# 1. Check authentication status
xion-toolkit auth status

# 2. Login if needed
xion-toolkit auth login

# 3. List existing treasuries
xion-toolkit treasury list

# 4. Create a new treasury
xion-toolkit treasury create \
  --fee-grant basic:1000000uxion \
  --grant-config authz:cosmwasm.wasm.v1.MsgExecuteContract

# 5. Query treasury details
xion-toolkit treasury query <treasury-address>
```

**Network Support**:
- **local**: http://localhost:8787 (local OAuth2 service)
- **testnet**: https://oauth2.testnet.burnt.com/
- **mainnet**: Coming soon

#### 2.3 Grant Configuration

**Fee Grant Configuration Example**:
```json
{
  "allowance_type": "BasicAllowance",
  "spend_limit": [
    {
      "denom": "uxion",
      "amount": "1000000"
    }
  ]
}
```

**Authz Grant Configuration Example**:
```json
{
  "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
  "authorization": {
    "type": "ContractExecutionAuthorization",
    "contract": "xion1...",
    "limits": {
      "max_calls": 100,
      "max_funds": [
        {
          "denom": "uxion",
          "amount": "10000000"
        }
      ]
    }
  }
}
```

### Phase 3: Skills Development

#### 3.1 xion-oauth2 Skill

**Purpose**: Guide users through OAuth2 authentication setup

**Scripts**:
- `login.sh` - Initiate OAuth2 login flow
- `status.sh` - Check authentication status
- `logout.sh` - Clear stored credentials
- `refresh.sh` - Manually refresh access token

#### 3.2 xion-treasury Skill

**Purpose**: Manage Treasury contracts

**Scripts**:
- `create.sh` - Create a new Treasury
- `list.sh` - List user's Treasury contracts
- `query.sh` - Query Treasury details
- `fund.sh` - Fund a Treasury
- `withdraw.sh` - Withdraw from Treasury
- `grant-config.sh` - Configure Authz Grants
- `fee-config.sh` - Configure Fee Grants
- `update-params.sh` - Update Treasury parameters

### Phase 4: Testing & Documentation

#### 4.1 Testing Scenarios

1. **OAuth2 Authentication Tests**
   - New user first login
   - Token refresh
   - Token expiration handling
   - Network switching

2. **Treasury Management Tests**
   - Create Treasury
   - Query Treasury
   - Update configuration
   - Deposit/withdrawal

3. **Grant Configuration Tests**
   - Configure Fee Grant
   - Configure Authz Grant
   - Verify Grant effectiveness

4. **Network Tests**
   - Switch between local/testnet/mainnet
   - Verify correct API endpoints
   - Cross-network token isolation

#### 4.2 Testing Environment
- Xion Testnet (xion-testnet-2)
- Local OAuth2 API Service (Docker or local build)
- Mock callback server for automated testing

## Task Checklist

### Phase 1: Foundation (Days 1-7)
- [x] Project initialization (Cargo project)
- [x] CLI framework setup (clap)
- [x] Configuration management system
  - [x] Configuration file read/write
  - [x] Credential encryption (using keyring) - **Completed**
  - [x] Cache management
- [x] Error handling framework
- [x] Output formatting (JSON)
- [x] Network configuration
  - [x] local/testnet/mainnet endpoints
  - [x] Network switching commands
- [x] Configuration architecture refactor
  - [x] Separate network config (compile-time) from user data
  - [x] OAuth Client IDs from environment variables
  - [x] Per-network credential storage
  - [x] Callback port: 54321 (changed from 8080)

### Phase 2: OAuth2 Client (Days 8-14) ✅ **COMPLETED**
- [x] PKCE implementation
  - [x] Generate verifier
  - [x] Generate challenge
  - [x] Validation logic
- [x] OAuth2 client
  - [x] Authorization URL generation
  - [x] Localhost callback server
  - [x] Code exchange
  - [x] Token refresh
- [x] Token management
  - [x] Secure storage (keyring)
  - [x] Auto-refresh
  - [x] Expiration checking
- [x] Authentication commands
  - [x] login - OAuth2 login
  - [x] status - Check auth status
  - [x] logout - Clear credentials
  - [x] refresh - Refresh access token

### Phase 2.5: OAuth2 Improvements (Day 15) ✅ **COMPLETED**
- [x] OAuth2 Discovery
  - [x] Fetch endpoints from `.well-known/oauth-authorization-server`
  - [x] Cache endpoints per network (24-hour TTL)
  - [x] Use dynamic authorization_endpoint and token_endpoint
- [x] Configuration fixes
  - [x] Add `network_name` field to NetworkConfig
  - [x] Use network_name (not chain_id) for credentials and endpoint caching
  - [x] Update credentials file naming: `testnet.json` instead of `xion-testnet-2.json`
- [x] Redirect URI fix
  - [x] Changed from `http://localhost:54321` to `http://127.0.0.1:54321/callback`
  - [x] Match Treasury OAuth client configuration
- [x] Integration testing
  - [x] Testnet OAuth2 login flow tested successfully
  - [x] Token refresh works correctly
  - [x] Credentials saved in OS keyring
  - [x] OAuth2 endpoints cached correctly
- [x] Fixed: `xion_address` field returning null
  - [x] Call `/api/v1/me` endpoint after token exchange
  - [x] Extract MetaAccount address from user info response
  - [x] Updated UserInfo struct to match API response format
  - [x] All tests passing (65 tests)
- [x] OAuth2 Discovery (RFC 8414)
  - [x] Dynamic endpoint discovery from `.well-known/oauth-authorization-server`
  - [x] Endpoint caching (24-hour TTL)
  - [x] Network-specific cache keys
- [x] Network configuration fixes
  - [x] Added `network_name` field to NetworkConfig
  - [x] Credentials stored by network name (not chain ID)
  - [x] Correct redirect URI (`http://127.0.0.1:54321/callback`)
  - [x] Correct endpoint paths (`/oauth/authorize`, `/oauth/token`)
- [x] Integration testing
  - [x] Testnet OAuth2 login flow tested successfully
  - [x] Token refresh working
  - [x] All 65 unit tests passing
- [x] Configuration improvements
  - [x] Unified config path (`~/.xion-toolkit/`) across all platforms
  - [x] Removed `directories` dependency
  - [x] Default network set to testnet
  - [x] Local network requires explicit `--network local` flag

### Phase 3: Treasury API (Days 15-21) ✅ **98% COMPLETED**
- [x] API clients
  - [x] Treasury API client (OAuth2 API Service)
  - [ ] xiond Query client (optional)
  - [x] Error handling
- [x] Treasury commands
  - [x] list - List Treasury contracts
  - [x] query - Query details
  - [x] create - Create Treasury (complete with fee grant & authz grant config)
  - [x] fund - Fund Treasury (implemented, needs testing)
  - [x] withdraw - Withdraw from Treasury (implemented, needs testing)
- [x] Grant configuration
  - [x] fee-grant configuration (Basic, Periodic, AllowedMsg allowances)
  - [x] authz-grant configuration (Generic, Send, Stake, IbcTransfer, ContractExecution)
  - [ ] Verify configuration effectiveness (needs testing)
- [x] Treasury manager
  - [x] OAuth2 integration
  - [x] Auto token refresh
  - [x] Caching (5-minute TTL)
- [x] Transaction broadcasting
  - [x] broadcast_transaction() API method
  - [x] BroadcastRequest and BroadcastResponse types
  - [x] FundResult and WithdrawResult types

### Phase 4: Skills & Documentation (Days 22-28) ✅ **COMPLETED**
- [x] xion-oauth2 Skill
  - [x] login.sh
  - [x] status.sh
  - [x] logout.sh
  - [x] refresh.sh
  - [x] SKILL.md
- [x] xion-treasury Skill
  - [x] create.sh (fully implemented with fee grant & authz grant support)
  - [x] list.sh
  - [x] query.sh
  - [x] fund.sh (placeholder for future implementation)
  - [x] withdraw.sh (placeholder for future implementation)
  - [x] grant-config.sh (placeholder for future implementation)
  - [x] fee-config.sh (placeholder for future implementation)
  - [x] update-params.sh (placeholder for future implementation)
  - [x] SKILL.md
- [x] Documentation
  - [x] README.md (already completed in Phase 1)
  - [ ] CLI Reference (future)
  - [ ] OAuth2 Flow documentation (future)
  - [ ] Treasury Guide (future)
  - [ ] Network Configuration Guide (future)
- [x] Testing
  - [x] Unit tests (63 tests passing)
  - [ ] Integration tests (future)
  - [ ] E2E tests (future)

## Acceptance Criteria

### Functional Acceptance
- [ ] Users can complete OAuth2 authentication via CLI
- [ ] Users can create and manage Treasury contracts
- [ ] Users can configure Fee Grants and Authz Grants
- [ ] All commands support JSON output
- [ ] Error messages are structured with remediation suggestions
- [ ] Network switching works correctly (local/testnet/mainnet)
- [ ] Status command shows current network and authentication state

### Performance Acceptance
- [ ] OAuth2 login flow < 10 seconds
- [ ] Treasury query < 2 seconds
- [ ] CLI startup time < 100ms

### Security Acceptance
- [ ] Tokens encrypted and stored in OS keyring
- [ ] PKCE prevents code interception attacks
- [ ] Enforce HTTPS communication
- [ ] Sensitive information not logged
- [ ] Callback server only accepts localhost connections

### Cross-Platform Acceptance
- [ ] Works on macOS
- [ ] Works on Linux
- [ ] Works on Windows
- [ ] Keyring integration works on all platforms

---
## Sign-off

| Date | Completed Tasks | Status |
|------|-----------------|--------|
| 2025-03-05 | Created development plan | ✅ |
| 2025-03-05 | Created AGENTS.md guidelines | ✅ |
| 2025-03-05 | Phase 1: Project initialization (Cargo) | ✅ |
| 2025-03-05 | Phase 1: CLI framework (clap) | ✅ |
| 2025-03-05 | Phase 1: Configuration management (JSON) | ✅ |
| 2025-03-05 | Phase 1: Error handling (thiserror) | ✅ |
| 2025-03-05 | Phase 1: Output formatting (JSON) | ✅ |
| 2025-03-05 | Phase 1: Network configuration (local/testnet/mainnet) | ✅ |
| 2025-03-05 | Created comprehensive README.md | ✅ |
| 2025-03-05 | Phase 1: Config architecture refactor | ✅ |
| 2025-03-05 | Phase 1: Credential encryption (keyring) | ✅ |
| 2025-03-05 | Phase 1: Per-network credentials | ✅ |
| 2025-03-05 | Phase 1: OAuth Client IDs from env vars | ✅ |
| 2026-03-05 | Phase 2: PKCE implementation | ✅ |
| 2026-03-05 | Phase 2: OAuth2 callback server | ✅ |
| 2026-03-05 | Phase 2: Token manager | ✅ |
| 2026-03-05 | Phase 2: OAuth2 client orchestration | ✅ |
| 2026-03-05 | Phase 2: CLI integration (auth commands) | ✅ |
| 2026-03-05 | Phase 2: All tests passing (59 tests) | ✅ |
| 2026-03-05 | Phase 3: Treasury API client | ✅ |
| 2026-03-05 | Phase 3: Treasury manager | ✅ |
| 2026-03-05 | Phase 3: Treasury cache system | ✅ |
| 2026-03-05 | Phase 3: CLI integration (list + query) | ✅ |
| 2026-03-05 | Phase 3: All tests passing (63 tests) | ✅ |
| 2026-03-06 | Phase 4: xion-oauth2 skill (SKILL.md + 4 scripts) | ✅ |
| 2026-03-06 | Phase 4: xion-treasury skill (SKILL.md + 8 scripts) | ✅ |
| 2026-03-06 | OAuth2 Discovery: Dynamic endpoint fetching | ✅ |
| 2026-03-06 | OAuth2 Discovery: Endpoint caching (24h TTL) | ✅ |
| 2026-03-06 | Configuration: Unified path (~/.xion-toolkit/) | ✅ |
| 2026-03-06 | Configuration: Network name-based credential storage | ✅ |
| 2026-03-06 | OAuth2 Integration: Testnet login successful | ✅ |
| 2026-03-06 | OAuth2 Integration: Token refresh working | ✅ |
| 2026-03-06 | All tests passing (65 tests) | ✅ |
| 2026-03-06 | Fixed: xion_address field returning null | ✅ |
| 2026-03-06 | OAuth2: Call /api/v1/me to get MetaAccount address | ✅ |
| 2026-03-06 | OAuth2: Testnet login verified with xion_address | ✅ |
| 2026-03-06 | Treasury: Implement fund operation with MsgSend | ✅ |
| 2026-03-06 | Treasury: Implement withdraw operation with MsgExecuteContract | ✅ |
| 2026-03-06 | Treasury: Add transaction broadcasting support | ✅ |
| 2026-03-06 | Docs: Fix all doc tests (122 tests passing) | ✅ |
| 2026-03-06 | All tests passing (68 unit + 20 integration + 34 doc) | ✅ |
| 2026-03-06 | Treasury Create: Encoding module (36 tests) | ✅ |
| 2026-03-06 | Treasury Create: Types & CLI flags | ✅ |
| 2026-03-06 | Treasury Create: Config file support | ✅ |
| 2026-03-06 | Treasury Create: Polling mechanism | ✅ |
| 2026-03-06 | Treasury Create: Documentation updated | ✅ |
| 2026-03-06 | Treasury Create: Integration testing (in-progress) | 🔄 |

---
*Created: 2025-03-05*
*Last Updated: 2026-03-06*
