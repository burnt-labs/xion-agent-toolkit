---
name: xion-oauth2
description: |
  OAuth2 authentication for Xion MetaAccount with REFRESH-FIRST approach.
  
  **IMPORTANT: Always prefer `auth refresh` over `auth login` when credentials exist.**
  
  Use this skill whenever the user mentions:
  - Xion login, authentication, MetaAccount login, OAuth2 xion
  - Token expired, refresh token, access token, session key
  - xion 认证, xion 登录, MetaAccount 登录, OAuth2 登录
  - "login to xion", "authenticate xion", "gasless auth"
  - Token issues, authentication problems, credential errors
  - Before any Treasury/contract operations that require authentication
  
  This skill provides GASLESS authentication through MetaAccount - no gas fees required.
  
  Key commands:
  - `auth status` - Check current authentication state
  - `auth refresh` - Refresh token (PREFERRED when credentials exist)
  - `auth login` - New browser auth (only if no credentials or refresh failed)
  - `auth login --force` - Force fresh browser auth (skip refresh check)
  
  Make sure to use this skill whenever authentication is mentioned, even if the user doesn't explicitly say "OAuth2" or "MetaAccount".
metadata:
  author: burnt-labs
  version: "1.2.1"
  requires:
    - xion-toolkit-init
  compatibility: Requires xion-toolkit CLI and browser for OAuth2 flow
---

# xion-oauth2

OAuth2 authentication skill for Xion blockchain development. This skill enables AI agents to authenticate with Xion's **MetaAccount** system using browser-based OAuth2 flow with PKCE security.

## Why MetaAccount?

MetaAccount authentication provides:
- **Gasless transactions** - No need to hold XION for gas fees
- **Browser-based login** - Familiar OAuth2 flow, no mnemonic management
- **Session keys** - Secure, revocable access tokens
- **30-day refresh tokens** - Long-lived sessions for automation

## ⚠️ Refresh-First Authentication

**ALWAYS prefer `auth refresh` over `auth login` when you have existing credentials.**

Refresh tokens last 30 days. Using refresh is:
- **Faster** - No browser interaction required
- **More reliable** - No callback server needed
- **Preserves session** - Same MetaAccount, same grants

### When to Use Each Command

| Command | Use When |
|---------|----------|
| `auth refresh` | ✅ You have existing credentials (default choice) |
| `auth login` | ⚠️ First-time setup OR refresh failed |
| `auth login --force` | 🔧 Force new browser auth (rarely needed) |

### Recommended Workflow

```bash
# Step 1: Check if already authenticated
xion-toolkit auth status

# Step 2: If authenticated but token may be stale, refresh first
xion-toolkit auth refresh

# Step 3: Only use login if refresh fails or no credentials exist
xion-toolkit auth login
```

### Automatic Refresh

The CLI automatically refreshes expired access tokens when you run any command that requires authentication. You rarely need to manually call `auth refresh` - just run your intended command and the CLI will handle token refresh automatically.

### When Refresh Fails

If `auth refresh` fails with `TOKEN_EXPIRED` or `AUTH_FAILED`:

1. Check `auth status` to confirm token state
2. Use `auth login --force` to force fresh browser authentication
3. Common reasons refresh fails:
   - Refresh token expired (>30 days since last login)
   - Credentials were manually deleted from keyring
   - OAuth2 client permissions changed

### When to Use --force

Use `auth login --force` (skip refresh, open browser immediately) when:
- User explicitly requests fresh authentication
- Refresh failed due to expired refresh token
- You suspect credential corruption
- Switching to a different MetaAccount

## Overview

This skill wraps the `xion-toolkit` CLI tool to provide Agent-friendly OAuth2 authentication capabilities:

- **login.sh** - Initiate OAuth2 login flow via browser
- **status.sh** - Check current authentication status
- **logout.sh** - Clear stored credentials
- **refresh.sh** - Manually refresh access token

## Prerequisites

- `xion-toolkit` CLI tool installed and in PATH
- Browser available for OAuth2 authorization
- Network connectivity to Xion OAuth2 API

> **Note**: If `xion-toolkit` is not installed, use the `xion-toolkit-init` skill first.

## Quick Start

### Check Authentication Status

```bash
./scripts/status.sh
```

### Refresh Token (Recommended First Step)

If you have existing credentials, refresh them first:

```bash
./scripts/refresh.sh
```

### Login (Only If No Credentials)

```bash
./scripts/login.sh
```

This will:
1. Open your browser to the Xion authorization page
2. Wait for you to approve the authorization
3. Return authentication status as JSON

### Logout

```bash
./scripts/logout.sh
```

## Script Details

### login.sh

Initiates the OAuth2 login flow.

**Usage:**
```bash
./scripts/login.sh [--port PORT] [--network NETWORK] [--force]
```

**Options:**
- `--port PORT` - Callback server port (default: 54321)
- `--network NETWORK` - Network to use: local, testnet, mainnet (default: testnet)
- `--force` - Force new browser authentication (skip refresh check)

**Output (stdout):**
```json
// Login success (browser auth):
{
  "success": true,
  "network": "testnet",
  "xion_address": "xion1...",
  "expires_at": "2024-01-15T10:30:00Z",
  "refreshed": false
}

// Refresh success (no browser):
{
  "success": true,
  "network": "testnet",
  "xion_address": "xion1...",
  "expires_at": "2024-01-15T10:30:00Z",
  "refreshed": true,
  "message": "Token refreshed successfully. No browser auth needed."
}
```

**Error Output (stderr):**
```
[ERROR] Failed to start callback server: Port 54321 already in use
```

### status.sh

Checks the current authentication status.

**Usage:**
```bash
./scripts/status.sh [--network NETWORK]
```

**Output (stdout):**
```json
{
  "success": true,
  "authenticated": true,
  "network": "testnet",
  "token_info": {
    "expires_at": "2024-01-15T10:30:00Z",
    "expires_in_seconds": 3600,
    "needs_refresh": false
  }
}
```

### logout.sh

Clears stored credentials for a specific network.

**Usage:**
```bash
./scripts/logout.sh [--network NETWORK]
```

**Output (stdout):**
```json
{
  "success": true,
  "message": "Successfully logged out from testnet"
}
```

### refresh.sh

Manually refreshes the access token.

**Usage:**
```bash
./scripts/refresh.sh [--network NETWORK]
```

**Output (stdout):**
```json
{
  "success": true,
  "message": "Token refreshed successfully",
  "expires_at": "2024-01-15T11:30:00Z",
  "expires_in_seconds": 3600
}
```

## Error Handling

All scripts output JSON to stdout with a `success` field:

**Success Response:**
```json
{
  "success": true,
  ...
}
```

**Error Response:**
```json
{
  "success": false,
  "error": "Error message",
  "error_code": "ERROR_CODE"
}
```

**Common Error Codes:**
- `CLI_NOT_FOUND` - xion-toolkit CLI not found in PATH
- `AUTH_FAILED` - Authentication failed
- `TOKEN_EXPIRED` - Token has expired and refresh failed
- `NETWORK_ERROR` - Failed to connect to OAuth2 API
- `INVALID_NETWORK` - Invalid network specified

## Network Configuration

The skill supports three networks:

| Network | OAuth2 API URL | Chain ID |
|---------|----------------|----------|
| local | http://localhost:8787 | xion-local |
| testnet | https://oauth2.testnet.burnt.com | xion-testnet-2 |
| mainnet | https://oauth2.burnt.com | xion-mainnet-1 |

## Integration Examples

### Using with Claude Code

```javascript
// In your Claude Code skill
{
  "tools": [
    {
      "name": "xion_login",
      "description": "Authenticate with Xion blockchain",
      "command": "./skills/xion-oauth2/scripts/login.sh"
    }
  ]
}
```

### Programmatic Usage

```python
import subprocess
import json

# Login
result = subprocess.run(
    ['./skills/xion-oauth2/scripts/login.sh'],
    capture_output=True,
    text=True
)

if result.returncode == 0:
    data = json.loads(result.stdout)
    if data['success']:
        print(f"Authenticated on {data['network']}")
    else:
        print(f"Error: {data['error']}")
else:
    print(f"Script failed: {result.stderr}")
```

## Security Considerations

1. **PKCE Protection** - All authorization requests use PKCE (Proof Key for Code Exchange)
2. **Localhost Callback** - Callback server only accepts localhost connections
3. **Encrypted Storage** - Tokens are stored in OS-native keyring
4. **Network Isolation** - Credentials are isolated per network

## Troubleshooting

### Port Already in Use

If port 54321 is already in use:
```bash
./scripts/login.sh --port 54322
```

### Token Expired

If you get `TOKEN_EXPIRED` error:
```bash
./scripts/refresh.sh
# or
./scripts/login.sh
```

### CLI Not Found

Ensure `xion-toolkit` CLI is in PATH:
```bash
which xion-toolkit
# If not found, add to PATH or create alias
```

## Related Skills

- **xion-dev** - Unified entry point for Xion development
- **xion-treasury** - Treasury management (requires authentication)
- **xion-toolkit-init** - CLI installation (use if CLI not found)
- **xiond-usage** (xion-skills) - Chain-level queries

## Version

- Skill Version: 1.2.0
- Compatible CLI Version: >=0.1.0

## Parameter Collection Workflow

Before executing any command, ensure all required parameters are collected.

### Step 1: Identify Operation
Determine which operation the user wants to perform (login, status, logout, refresh).

### Step 2: Check Parameter Schema
Refer to the `schemas/` directory for detailed parameter definitions.

### Step 3: Collect Missing Parameters
Most OAuth2 commands have minimal or no required parameters:

> Example for login:
> "I'll initiate the OAuth2 login flow. A browser will open for authorization. Ready? [y/n]"

### Step 4: Confirm Before Execution

Present the parameters in a tree format and ask for confirmation:

```
Will execute: login
├─ Network: testnet
├─ Port: 54321 (default)
└─ Opens browser for authorization
Confirm? [y/n]
```

### Quality Checklist

Before executing auth commands, verify:
- [ ] **Checked `auth status` first** (unless user explicitly requested login)
- [ ] **Using correct network** (testnet vs mainnet)
- [ ] **Prefer refresh over login** when user has existing credentials
- [ ] **Confirmed with user** if browser will open (not headless-safe)
- [ ] **Using `--force` only when necessary** (refresh failed or user requested fresh auth)

## Parameter Schemas

See `schemas/` directory for detailed parameter definitions:

| Schema File | Command | Description |
|-------------|---------|-------------|
| `login.json` | `login` | OAuth2 login |
| `status.json` | `status` | Check auth status |
| `logout.json` | `logout` | Clear credentials |
| `refresh.json` | `refresh` | Refresh token |

### Quick Parameter Reference

#### login
| Parameter | Required | Description |
|-----------|----------|-------------|
| `port` | No | Callback server port (default: 54321) |
| `network` | No | Network (default: testnet) |
| `force` | No | Force new browser auth (default: false) |

> **Note**: See `schemas/login.json` for complete parameter list including conditional parameters.

#### status
| Parameter | Required | Description |
|-----------|----------|-------------|
| `network` | No | Network to check (default: testnet) |

> **Note**: See `schemas/status.json` for complete parameter list including conditional parameters.

#### logout
| Parameter | Required | Description |
|-----------|----------|-------------|
| `network` | No | Network to logout from (default: testnet) |

> **Note**: See `schemas/logout.json` for complete parameter list including conditional parameters.

#### refresh
| Parameter | Required | Description |
|-----------|----------|-------------|
| `network` | No | Network to refresh (default: testnet) |

> **Note**: See `schemas/refresh.json` for complete parameter list including conditional parameters.

## Validation

Use the validation script to check parameters before execution:

```bash
./skills/scripts/validate-params.sh xion-oauth2 login '{}'
```

## License

MIT License - See main project LICENSE file

## Testing

This skill includes comprehensive test coverage using the Skills Test Framework.

### Test File

- `tests/skills/test_oauth2.sh` - OAuth2 skill test suite

### Running Tests

#### Mock Mode (Fast, No Network Required)

```bash
# Run OAuth2 tests with mock responses
MOCK_ENABLED=true ./tests/skills/run_all.sh oauth2

# Or run all skill tests
MOCK_ENABLED=true ./tests/skills/run_all.sh
```

#### E2E Mode (Requires Network and Credentials)

```bash
# Run with real CLI (requires xion-toolkit installed and configured)
./tests/skills/run_all.sh oauth2

# Requires valid credentials for testnet
# Credentials should be in ~/.xion-toolkit/credentials/testnet.enc
```

### Test Coverage

| Function | Mock Test | E2E Test |
|----------|-----------|----------|
| `login.sh` | ✅ | ⚠️ (requires browser) |
| `status.sh` (authenticated) | ✅ | ✅ |
| `status.sh` (not authenticated) | ✅ | ✅ |
| `logout.sh` | ✅ | ✅ |
| `refresh.sh` | ✅ | ✅ |
| Error handling | ✅ | ✅ |

### Mock Responses

Mock responses are defined in `tests/skills/mocks/oauth2-responses.json`:

```json
{
  "status_authenticated": {
    "success": true,
    "authenticated": true,
    "network": "testnet",
    "token_info": {
      "expires_at": "2024-01-15T10:30:00Z",
      "expires_in_seconds": 3600,
      "needs_refresh": false
    }
  },
  "status_not_authenticated": {
    "success": true,
    "authenticated": false,
    "network": "testnet",
    "message": "No credentials found"
  },
  "logout_success": {
    "success": true,
    "message": "Successfully logged out from testnet"
  },
  "refresh_success": {
    "success": true,
    "message": "Token refreshed successfully",
    "expires_at": "2024-01-15T11:30:00Z",
    "expires_in_seconds": 3600
  }
}
```

### Writing New Tests

Tests use the framework from `tests/skills/lib.sh`:

```bash
#!/bin/bash
set -euo pipefail

# Source test framework
source "$(dirname "$0")/lib.sh"

# Test: Status returns valid JSON when authenticated
test_status_authenticated() {
    local output
    output=$(mock_cli "oauth2" "auth status" "status_authenticated")
    
    assert_success "$output"
    assert_json_contains "$output" ".authenticated" "true"
    assert_json_has_key "$output" ".token_info.expires_at"
}

# Test: Status returns not authenticated when no credentials
test_status_not_authenticated() {
    local output
    output=$(mock_cli "oauth2" "auth status" "status_not_authenticated")
    
    assert_success "$output"
    assert_json_contains "$output" ".authenticated" "false"
}

# Run tests if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    run_test "test_status_authenticated" test_status_authenticated
    run_test "test_status_not_authenticated" test_status_not_authenticated
    test_exit
fi
```

### CI Integration

Tests run automatically in GitHub Actions:

- **Mock Tests**: Run on every PR and push to main/develop
- **E2E Tests**: Run on main branch with secrets (testnet credentials)
- **Lint**: Shell scripts are checked with shellcheck

See `.github/workflows/test-skills.yml` for configuration.

### Debugging Failed Tests

```bash
# Run with verbose output
bash -x tests/skills/test_oauth2.sh

# Check JSON output manually
MOCK_ENABLED=true bash -c 'source tests/skills/lib.sh; mock_cli "oauth2" "auth status" "status_authenticated"'

# Validate mock response file
jq '.' tests/skills/mocks/oauth2-responses.json
```
