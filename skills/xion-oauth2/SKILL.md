---
name: xion-oauth2
description: |
  OAuth2 authentication for Xion MetaAccount. Use this skill whenever the user needs to login to Xion, authenticate with MetaAccount, check authentication status, refresh tokens, or before any Treasury/contract operations that require authentication.
  
  This skill provides GASLESS authentication through MetaAccount - no gas fees required for authentication.
  
  Triggers on: MetaAccount 登录, MetaAccount login, browser login, gasless auth, gasless authentication, session key, OAuth2 登录, xion 认证, xion auth, xion login, access token, refresh token, OAuth2 xion, authenticate xion, login to xion, browser authentication.
  
  Use this skill BEFORE xion-treasury - authentication is required for all Treasury operations. Make sure to use this skill whenever the user mentions logging into Xion, even if they don't explicitly say "OAuth2" or "MetaAccount".
metadata:
  author: burnt-labs
  version: "1.1.0"
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

### Login

```bash
./scripts/login.sh
```

This will:
1. Open your browser to the Xion authorization page
2. Wait for you to approve the authorization
3. Return authentication status as JSON

### Check Status

```bash
./scripts/status.sh
```

### Logout

```bash
./scripts/logout.sh
```

### Refresh Token

```bash
./scripts/refresh.sh
```

## Script Details

### login.sh

Initiates the OAuth2 login flow.

**Usage:**
```bash
./scripts/login.sh [--port PORT] [--network NETWORK]
```

**Options:**
- `--port PORT` - Callback server port (default: 54321)
- `--network NETWORK` - Network to use: local, testnet, mainnet (default: testnet)

**Output (stdout):**
```json
{
  "success": true,
  "network": "testnet",
  "authenticated": true,
  "token_type": "Bearer",
  "expires_in": 3600,
  "scope": "treasury:manage"
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

- Skill Version: 1.1.0
- Compatible CLI Version: >=0.1.0

## License

MIT License - See main project LICENSE file
