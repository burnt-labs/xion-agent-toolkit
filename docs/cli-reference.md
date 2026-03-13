# CLI Reference

> **For AI Agents**: This document is comprehensive. For a condensed quick reference, see [QUICK-REFERENCE.md](./QUICK-REFERENCE.md).

Complete reference for the Xion Agent Toolkit CLI commands.

## Table of Contents

- [Usage Examples](#usage-examples)
- [Global Options](#global-options)
- [Authentication Commands](#authentication-commands)
- [Treasury Commands](#treasury-commands)
  - [`treasury export`](#treasury-export)
  - [`treasury import`](#treasury-import)
- [Contract Commands](#contract-commands)
  - [`contract query`](#contract-query)
- [Configuration Commands](#configuration-commands)
- [Output Format](#output-format)

---

## Usage Examples

Quick reference to common tasks and workflows.

### Quick Start

```bash
# 1. Login to your account
xion-toolkit auth login

# 2. Check authentication status
xion-toolkit auth status

# 3. List your treasuries
xion-toolkit treasury list

# 4. Create a new treasury
xion-toolkit treasury create \
  --redirect-url "https://example.com/callback" \
  --icon-url "https://example.com/icon.png" \
  --name "My Treasury"
```

### Common Workflows

| Workflow | Commands |
|----------|----------|
| [Complete Treasury Lifecycle](#complete-treasury-lifecycle) | create → fund → grant-config → fee-config → use |
| [Authentication Flow](#authentication-flow) | login → status → refresh |
| [Grant and Fee Setup](#grant-and-fee-setup-workflow) | grant-config add → fee-config set → verify |
| [Contract Deployment](#contract-deployment-workflow) | instantiate → execute → query |
| [Treasury Backup & Migration](#treasury-backup--migration-workflow) | export → (edit if needed) → import |

### Example Outputs

All commands return JSON output for easy parsing:

**Success:**
```json
{
  "success": true,
  "data": { ... }
}
```

**Error:**
```json
{
  "success": false,
  "error": "Error message",
  "code": "ERROR_CODE",
  "suggestion": "How to fix it"
}
```

### Complete Treasury Lifecycle

End-to-end example of creating and configuring a Treasury:

```bash
# Step 1: Authenticate
xion-toolkit auth login

# Step 2: Create a new Treasury
xion-toolkit treasury create \
  --redirect-url "https://myapp.example.com/callback" \
  --name "My App Treasury"
# Output: {"success": true, "address": "xion1treasury...", ...}

# Step 3: Fund the Treasury (1 XION = 1,000,000 uxion)
xion-toolkit treasury fund xion1treasury... --amount 10000000

# Step 4: Configure Authz Grant (allow MsgSend)
xion-toolkit treasury grant-config add xion1treasury... \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "5000000uxion" \
  --grant-description "Allow sending funds"

# Step 5: Configure Fee Allowance (gasless transactions)
xion-toolkit treasury fee-config set xion1treasury... \
  --fee-allowance-type basic \
  --fee-spend-limit "1000000uxion" \
  --fee-description "Basic fee allowance"

# Step 6: Verify configuration
xion-toolkit treasury query xion1treasury... --include-grants

# Step 7: Use the Treasury (withdraw funds)
xion-toolkit treasury withdraw xion1treasury... \
  --amount 1000000 \
  --to xion1recipient...
```

### Authentication Flow

Complete authentication management:

```bash
# Login (opens browser)
xion-toolkit auth login

# Check status
xion-toolkit auth status
# Output: {"success": true, "authenticated": true, ...}

# If token expired, refresh it
xion-toolkit auth refresh

# Logout when done (clears credentials)
xion-toolkit auth logout
```

### Grant and Fee Setup Workflow

Setting up permissions for Treasury operations:

```bash
# List existing grants
xion-toolkit treasury grant-config list xion1treasury...

# Add a new grant
xion-toolkit treasury grant-config add xion1treasury... \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --grant-description "Daily spending limit"

# Query on-chain grants
xion-toolkit treasury chain-query grants xion1treasury...

# Set fee allowance
xion-toolkit treasury fee-config set xion1treasury... \
  --fee-allowance-type periodic \
  --fee-period-seconds 86400 \
  --fee-period-spend-limit "100000uxion" \
  --fee-description "Daily fee allowance"

# Query fee configuration
xion-toolkit treasury fee-config query xion1treasury...

# Remove grant when no longer needed
xion-toolkit treasury grant-config remove xion1treasury... \
  --type-url "/cosmos.bank.v1beta1.MsgSend"
```

### Contract Deployment Workflow

Deploying and interacting with CosmWasm contracts:

```bash
# Instantiate a contract
xion-toolkit contract instantiate \
  --code-id 1260 \
  --label "my-contract-001" \
  --msg '{"admin": "xion1abc..."}'
# Output: {"success": true, "address": "xion1contract...", ...}

# Instantiate with predictable address (instantiate2)
xion-toolkit contract instantiate2 \
  --code-id 1260 \
  --label "my-contract-002" \
  --msg '{"admin": "xion1abc..."}' \
  --salt "0102030405060708"
# Output: {"success": true, "address": "xion1predictable...", ...}

# Execute a message on the contract
xion-toolkit contract execute \
  --contract xion1contract... \
  --msg '{"increment": {}}'

# Execute with funds
xion-toolkit contract execute \
  --contract xion1contract... \
  --msg '{"buy_item": {"id": 1}}' \
  --funds "1000000uxion"
```

### Treasury Backup & Migration Workflow

Export treasury configuration for backup or migration:

```bash
# Step 1: Export treasury configuration
xion-toolkit treasury export xion1treasury... --output treasury-backup.json

# Step 2: (Optional) Edit configuration file
# Modify grant_configs, fee_config, or params as needed

# Step 3: Preview import (dry run)
xion-toolkit treasury import xion1newtreasury... \
  --from-file treasury-backup.json \
  --dry-run

# Step 4: Execute import
xion-toolkit treasury import xion1newtreasury... \
  --from-file treasury-backup.json
```

---

## Global Options

These options can be used with any command:

```bash
xion-toolkit --network <NETWORK> <command>  # Network override (testnet, mainnet)
xion-toolkit --help                          # Show help
xion-toolkit --version                       # Show version
```

---

## Authentication Commands

### `auth login`

Initiates OAuth2 authentication flow with PKCE security.

**Usage:**
```bash
xion-toolkit auth login [--port <PORT>]
```

**Options:**
- `--port <PORT>` - Callback server port (default: 54321)

**Examples:**

Basic usage:
```bash
xion-toolkit auth login
```

Output (success):
```json
{
  "success": true,
  "message": "Successfully authenticated",
  "address": "xion1abc123def456..."
}
```

With custom port:
```bash
xion-toolkit auth login --port 8080
```

Output (error - port in use):
```json
{
  "success": false,
  "error": "Port 8080 is already in use",
  "code": "PORT_IN_USE",
  "suggestion": "Try using a different port with --port flag"
}
```

**Notes:**
- Opens browser automatically for authentication
- Stores encrypted credentials in `~/.xion-toolkit/credentials/`
- Refresh tokens valid for 30 days

---

### `auth logout`

Clears stored credentials and logs out.

**Usage:**
```bash
xion-toolkit auth logout
```

**Examples:**

Basic usage:
```bash
xion-toolkit auth logout
```

Output (success):
```json
{
  "success": true,
  "message": "Successfully logged out"
}
```

Output (error - not authenticated):
```json
{
  "success": false,
  "error": "No credentials found",
  "code": "NOT_AUTHENTICATED",
  "suggestion": "Run 'xion-toolkit auth login' to authenticate first"
}
```

**Notes:**
- ⚠️ Deletes encrypted credential files
- ⚠️ Requires browser re-login on next use
- Refresh tokens are long-lived; only logout when necessary

---

### `auth status`

Checks authentication status and token expiration.

**Usage:**
```bash
xion-toolkit auth status
```

**Examples:**

Basic usage:
```bash
xion-toolkit auth status
```

Output (authenticated):
```json
{
  "success": true,
  "authenticated": true,
  "address": "xion1abc123def456...",
  "expires_at": "2024-01-15T12:00:00Z",
  "refresh_token_expires_at": "2024-02-15T12:00:00Z"
}
```

Output (not authenticated):
```json
{
  "success": true,
  "authenticated": false,
  "message": "No valid credentials found"
}
```

Output (token expired):
```json
{
  "success": true,
  "authenticated": false,
  "message": "Access token expired",
  "expires_at": "2024-01-01T00:00:00Z",
  "suggestion": "Run 'xion-toolkit auth refresh' to refresh your token"
}
```

---

### `auth refresh`

Refreshes the access token using the stored refresh token.

**Usage:**
```bash
xion-toolkit auth refresh
```

**Examples:**

Basic usage:
```bash
xion-toolkit auth refresh
```

Output (success):
```json
{
  "success": true,
  "message": "Token refreshed successfully",
  "expires_at": "2024-01-16T12:00:00Z"
}
```

Output (error - refresh token expired):
```json
{
  "success": false,
  "error": "Refresh token has expired",
  "code": "REFRESH_TOKEN_EXPIRED",
  "suggestion": "Run 'xion-toolkit auth login' to re-authenticate"
}
```

**Notes:**
- Access tokens expire after ~1 hour
- Refresh tokens valid for 30 days
- Auto-refresh happens automatically on expired access token

---

## Treasury Commands

### `treasury list`

Lists all Treasury contracts owned by the authenticated user.

**Usage:**
```bash
xion-toolkit treasury list
```

**Options:**
- `--network <NETWORK>` - Network override (testnet, mainnet)
- `--output <FORMAT>` - Output format (json, text). Default: json

**Examples:**

Basic usage:
```bash
xion-toolkit treasury list
```

Output (success):
```json
{
  "success": true,
  "treasuries": [
    {
      "address": "xion1abc123def456...",
      "balance": "10000000",
      "denom": "uxion",
      "admin": "xion1admin789...",
      "created_at": "2024-01-01T00:00:00Z"
    },
    {
      "address": "xion1xyz789ghi012...",
      "balance": "5000000",
      "denom": "uxion",
      "admin": "xion1admin789...",
      "created_at": "2024-01-02T00:00:00Z"
    }
  ],
  "count": 2
}
```

With network override:
```bash
xion-toolkit treasury list --network mainnet
```

Output (no treasuries):
```json
{
  "success": true,
  "treasuries": [],
  "count": 0
}
```

Output (error - not authenticated):
```json
{
  "success": false,
  "error": "No valid credentials found",
  "code": "NOT_AUTHENTICATED",
  "suggestion": "Run 'xion-toolkit auth login' to authenticate first"
}
```

---

### `treasury query`

Queries detailed information about a specific Treasury contract.

**Usage:**
```bash
xion-toolkit treasury query <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic usage:
```bash
xion-toolkit treasury query xion1abc123def456...
```

Output (success):
```json
{
  "success": true,
  "treasury": {
    "address": "xion1abc123def456...",
    "balance": {
      "denom": "uxion",
      "amount": "10000000"
    },
    "admin": "xion1admin789...",
    "params": {
      "redirect_url": "https://example.com/callback",
      "icon_url": "https://example.com/icon.png",
      "metadata": {
        "name": "My Treasury",
        "archived": false
      }
    },
    "grants": [
      {
        "type_url": "/cosmos.bank.v1beta1.MsgSend",
        "description": "Allow sending funds"
      }
    ]
  }
}
```

Output (error - treasury not found):
```json
{
  "success": false,
  "error": "Treasury not found: xion1invalid...",
  "code": "TREASURY_NOT_FOUND",
  "suggestion": "Check the address and ensure you have access to this treasury"
}
```

Output (error - invalid address):
```json
{
  "success": false,
  "error": "Invalid address format",
  "code": "INVALID_ADDRESS",
  "suggestion": "Address should start with 'xion1' and be a valid bech32 address"
}
```

**Notes:**
- Returns combined data from indexer and on-chain queries
- Includes current grants and fee allowances

---

### `treasury create`

Creates a new Treasury contract with configurable parameters.

**Usage:**
```bash
xion-toolkit treasury create [options]
```

**Options:**
- `--config <FILE>` - JSON configuration file
- `--redirect-url <URL>` - OAuth redirect URL (required if not using --config)
- `--icon-url <URL>` - Treasury icon URL (required if not using --config)
- `--name <NAME>` - Treasury display name
- `--is-oauth2-app` - Mark as OAuth2 application

**Fee Grant Options:**
- `--fee-allowance-type <TYPE>` - Fee allowance type: basic, periodic
- `--fee-spend-limit <AMOUNT>` - Spend limit (e.g., "1000000uxion")
- `--fee-description <TEXT>` - Fee grant description
- `--fee-period-seconds <SECONDS>` - Period duration (for periodic allowance)
- `--fee-period-spend-limit <AMOUNT>` - Period spend limit (for periodic allowance)

**Authz Grant Options:**
- `--grant-type-url <URL>` - Message type URL
- `--grant-auth-type <TYPE>` - Authorization type: generic, send
- `--grant-description <TEXT>` - Grant description
- `--grant-spend-limit <AMOUNT>` - Spend limit (for send authorization)

**Examples:**

Using config file:
```bash
# treasury-config.json
{
  "redirect_url": "https://example.com/callback",
  "icon_url": "https://example.com/icon.png",
  "name": "My Treasury",
  "fee_allowance": {
    "type": "basic",
    "spend_limit": "10000000uxion"
  }
}

xion-toolkit treasury create --config treasury-config.json
```

Output (success):
```json
{
  "success": true,
  "treasury": {
    "address": "xion1newtreasury123...",
    "admin": "xion1admin789...",
    "balance": "0",
    "denom": "uxion",
    "params": {
      "redirect_url": "https://example.com/callback",
      "icon_url": "https://example.com/icon.png",
      "metadata": {
        "name": "My Treasury",
        "archived": false
      }
    }
  },
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Using flags (minimal):
```bash
xion-toolkit treasury create \
  --redirect-url "https://example.com/callback" \
  --icon-url "https://example.com/icon.png"
```

Using flags (with grants):
```bash
xion-toolkit treasury create \
  --redirect-url "https://example.com/callback" \
  --icon-url "https://example.com/icon.png" \
  --name "My Treasury" \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --grant-description "Allow sending funds" \
  --fee-allowance-type basic \
  --fee-spend-limit "10000000uxion" \
  --fee-description "Basic fee allowance"
```

With OAuth2 app flag:
```bash
xion-toolkit treasury create \
  --redirect-url "https://app.example.com/oauth" \
  --icon-url "https://app.example.com/icon.png" \
  --name "My OAuth2 App" \
  --is-oauth2-app
```

Output (error - missing required fields):
```json
{
  "success": false,
  "error": "Missing required fields: redirect-url, icon-url",
  "code": "INVALID_INPUT",
  "suggestion": "Provide --redirect-url and --icon-url, or use --config with a complete config file"
}
```

Output (error - network error):
```json
{
  "success": false,
  "error": "Failed to broadcast transaction: connection timeout",
  "code": "NETWORK_ERROR",
  "suggestion": "Check your network connection and try again"
}
```

**Notes:**
- Treasury code ID: 1260 (testnet), 63 (mainnet)
- Creator becomes the initial admin
- Consider setting up grants and fee allowances during creation

---

### `treasury fund`

Funds a Treasury contract with tokens.

**Usage:**
```bash
xion-toolkit treasury fund <ADDRESS> --amount <AMOUNT>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--amount <AMOUNT>` - Amount to fund (e.g., "1000000uxion", required)
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic usage:
```bash
xion-toolkit treasury fund xion1abc123def456... --amount 1000000uxion
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "amount": "1000000uxion",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

With larger amount:
```bash
xion-toolkit treasury fund xion1abc123def456... --amount 100000000uxion
```

Output (error - insufficient balance):
```json
{
  "success": false,
  "error": "Insufficient balance: have 500000uxion, need 1000000uxion",
  "code": "INSUFFICIENT_BALANCE",
  "suggestion": "Fund your account first or reduce the amount"
}
```

Output (error - treasury not found):
```json
{
  "success": false,
  "error": "Treasury not found: xion1invalid...",
  "code": "TREASURY_NOT_FOUND",
  "suggestion": "Check the address and ensure the treasury exists"
}
```

**Notes:**
- Transfers tokens from your account to the treasury
- Treasury must exist before funding

---

### `treasury withdraw`

Withdraws funds from a Treasury to the admin account.

**Usage:**
```bash
xion-toolkit treasury withdraw <ADDRESS> --amount <AMOUNT>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--amount <AMOUNT>` - Amount to withdraw (e.g., "1000000uxion", required)
- `--to <ADDRESS>` - Recipient address (default: admin address)
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic usage (to admin):
```bash
xion-toolkit treasury withdraw xion1abc123def456... --amount 500000uxion
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "amount": "500000uxion",
  "recipient": "xion1admin789...",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

To specific recipient:
```bash
xion-toolkit treasury withdraw xion1abc123def456... \
  --amount 500000uxion \
  --to xion1recipient123...
```

Output (error - insufficient treasury balance):
```json
{
  "success": false,
  "error": "Treasury has insufficient balance: have 300000uxion, need 500000uxion",
  "code": "INSUFFICIENT_BALANCE",
  "suggestion": "Check treasury balance with 'xion-toolkit treasury query'"
}
```

Output (error - not admin):
```json
{
  "success": false,
  "error": "Only admin can withdraw funds from this treasury",
  "code": "UNAUTHORIZED",
  "suggestion": "Only the treasury admin can perform withdrawals"
}
```

**Notes:**
- Only admin can withdraw funds
- Funds go to admin address by default
- Use `--to` to send to a different address

---

### `treasury grant-config add`

Adds an Authz grant configuration to a Treasury.

**Usage:**
```bash
xion-toolkit treasury grant-config add <ADDRESS> [options]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--grant-config <FILE>` - JSON config file (alternative to flags)
- `--type-url <URL>` - Message type URL
- `--auth-type <TYPE>` - Authorization type: generic, send, contract-execution
- `--description <TEXT>` - Grant description
- `--spend-limit <AMOUNT>` - Spend limit for send authorization
- `--allow-list <ADDRS>` - Comma-separated list of allowed recipients
- `--contract <ADDRESS>` - Contract address (can be repeated, for contract-execution)
- `--max-calls <NUM>` - Maximum number of calls (can be repeated)
- `--max-funds <AMOUNT>` - Maximum funds per contract (can be repeated)
- `--filter-type <TYPE>` - Filter type: allow-all, accepted-keys
- `--keys <KEYS>` - Comma-separated list of accepted message keys
- `--preset <TYPE>` - Preset shortcut (see Available Presets below)

**Available Presets:**

| Preset | Type URL | Auth Type |
|--------|----------|-----------|
| `send` | /cosmos.bank.v1beta1.MsgSend | send |
| `execute` | /cosmwasm.wasm.v1.MsgExecuteContract | contract-execution |
| `instantiate` | /cosmwasm.wasm.v1.MsgInstantiateContract | generic |
| `instantiate2` | /cosmwasm.wasm.v1.MsgInstantiateContract2 | generic |
| `delegate` | /cosmos.staking.v1beta1.MsgDelegate | generic |
| `undelegate` | /cosmos.staking.v1beta1.MsgUndelegate | generic |
| `redelegate` | /cosmos.staking.v1beta1.MsgBeginRedelegate | generic |
| `withdraw-rewards` | /cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward | generic |
| `vote` | /cosmos.gov.v1beta1.MsgVote | generic |
| `gov-deposit` | /cosmos.gov.v1beta1.MsgDeposit | generic |
| `gov-submit-proposal` | /cosmos.gov.v1beta1.MsgSubmitProposal | generic |
| `ibc-transfer` | /ibc.applications.transfer.v1.MsgTransfer | ibc_transfer |
| `authz-exec` | /cosmos.authz.v1beta1.MsgExec | generic |
| `authz-revoke` | /cosmos.authz.v1beta1.MsgRevoke | generic |
| `feegrant-grant` | /cosmos.feegrant.v1beta1.MsgGrantAllowance | generic |
| `feegrant-revoke` | /cosmos.feegrant.v1beta1.MsgRevokeAllowance | generic |
| `unjail` | /cosmos.slashing.v1beta1.MsgUnjail | generic |
| `crisis-verify` | /cosmos.crisis.v1beta1.MsgVerifyInvariant | generic |
| `evidence-submit` | /cosmos.evidence.v1beta1.MsgSubmitEvidence | generic |
| `vesting-create` | /cosmos.vesting.v1beta1.MsgCreateVestingAccount | generic |
| `tokenfactory-mint` | /osmosis.tokenfactory.v1beta1.MsgMint | generic |
| `tokenfactory-burn` | /osmosis.tokenfactory.v1beta1.MsgBurn | generic |

**Examples:**

Add send authorization:
```bash
xion-toolkit treasury grant-config add xion1abc123def456... \
  --type-url "/cosmos.bank.v1beta1.MsgSend" \
  --auth-type send \
  --spend-limit "1000000uxion" \
  --description "Allow sending funds"
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operation": "add",
  "type_url": "/cosmos.bank.v1beta1.MsgSend",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Add contract execution authorization:
```bash
xion-toolkit treasury grant-config add xion1abc123def456... \
  --type-url "/cosmwasm.wasm.v1.MsgExecuteContract" \
  --auth-type contract-execution \
  --contract xion1contract789... \
  --max-calls 100 \
  --max-funds "1000000uxion" \
  --filter-type allow-all \
  --description "Execute contract with limits"
```

Using preset (quick setup):
```bash
# Send preset
xion-toolkit treasury grant-config add xion1abc123def456... \
  --preset send \
  --spend-limit "1000000uxion" \
  --description "Allow sending funds"

# Execute preset
xion-toolkit treasury grant-config add xion1abc123def456... \
  --preset execute \
  --contract xion1contract789... \
  --description "Allow contract execution"

# Instantiate preset
xion-toolkit treasury grant-config add xion1abc123def456... \
  --preset instantiate \
  --description "Allow contract instantiation"
```

Using config file:
```bash
# grant-config.json
{
  "grants": [
    {
      "type_url": "/cosmos.bank.v1beta1.MsgSend",
      "auth_type": "send",
      "spend_limit": "1000000uxion",
      "description": "Allow sending funds"
    }
  ]
}

xion-toolkit treasury grant-config add xion1abc123def456... --grant-config grant-config.json
```

Output (error - duplicate grant):
```json
{
  "success": false,
  "error": "Grant already exists for type: /cosmos.bank.v1beta1.MsgSend",
  "code": "DUPLICATE_GRANT",
  "suggestion": "Use 'grant-config remove' to remove existing grant first"
}
```

**Notes:**
- Multiple grants can be added to a treasury
- Use presets for common authorization patterns
- Contract execution grants support fine-grained controls

---

### `treasury grant-config remove`

Removes an Authz grant configuration from a Treasury.

**Usage:**
```bash
xion-toolkit treasury grant-config remove <ADDRESS> --type-url <URL>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--type-url <URL>` - Type URL of the grant to remove (required)
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic usage:
```bash
xion-toolkit treasury grant-config remove xion1abc123def456... \
  --type-url "/cosmos.bank.v1beta1.MsgSend"
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operation": "remove",
  "type_url": "/cosmos.bank.v1beta1.MsgSend",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Output (error - grant not found):
```json
{
  "success": false,
  "error": "No grant found for type: /cosmos.bank.v1beta1.MsgSend",
  "code": "GRANT_NOT_FOUND",
  "suggestion": "Use 'grant-config list' to see available grants"
}
```

**Notes:**
- Removes the grant from the treasury
- Does not affect already-used authorizations

---

### `treasury grant-config list`

Lists all Authz grant configurations for a Treasury.

**Usage:**
```bash
xion-toolkit treasury grant-config list <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--network <NETWORK>` - Network override (testnet, mainnet)
- `--output <FORMAT>` - Output format (json, text). Default: json

**Examples:**

Basic usage:
```bash
xion-toolkit treasury grant-config list xion1abc123def456...
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "grant_configs": [
    {
      "type_url": "/cosmos.bank.v1beta1.MsgSend",
      "description": "Allow sending funds",
      "authorization": {
        "type": "SendAuthorization",
        "spend_limit": [{"denom": "uxion", "amount": "1000000"}]
      }
    },
    {
      "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
      "description": "Execute contract with limits",
      "authorization": {
        "type": "ContractExecutionAuthorization",
        "allowed_addresses": ["xion1contract789..."],
        "max_calls": 100
      }
    }
  ],
  "count": 2
}
```

Output (no grants):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "grant_configs": [],
  "count": 0
}
```

**Notes:**
- Shows configured grants, not necessarily active on-chain grants
- Use `chain-query grants` for on-chain state

---

### `treasury fee-config set`

Sets fee grant configuration for a Treasury.

**Usage:**
```bash
xion-toolkit treasury fee-config set <ADDRESS> --fee-config <FILE>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--fee-config <FILE>` - JSON config file with fee configuration (required)
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Config File Format:**

*Basic Allowance:*
```json
{
  "basic": {
    "spend_limit": "10000000uxion",
    "description": "Basic fee allowance"
  }
}
```

*Periodic Allowance:*
```json
{
  "periodic": {
    "basic_spend_limit": "10000000uxion",
    "period_seconds": 86400,
    "period_spend_limit": "1000000uxion",
    "description": "Daily fee allowance"
  }
}
```

**Examples:**

Set basic fee allowance:
```bash
xion-toolkit treasury fee-config set xion1abc123def456... \
  --fee-config fee-config-basic.json
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operation": "set",
  "allowance_type": "BasicAllowance",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Set periodic fee allowance:
```bash
xion-toolkit treasury fee-config set xion1abc123def456... \
  --fee-config fee-config-periodic.json
```

Output (success - periodic):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operation": "set",
  "allowance_type": "PeriodicAllowance",
  "period_seconds": 86400,
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Output (error - invalid config):
```json
{
  "success": false,
  "error": "Invalid fee config: missing required field 'spend_limit'",
  "code": "INVALID_INPUT",
  "suggestion": "Check config file format in documentation"
}
```

**Notes:**
- Fee allowance lets others pay gas on behalf of the treasury
- Periodic allowance resets spend limit after each period
- Only one fee allowance can be active at a time per grantee

---

### `treasury fee-config remove`

Revokes fee allowance from a specific grantee.

**Usage:**
```bash
xion-toolkit treasury fee-config remove <ADDRESS> --grantee <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--grantee <ADDRESS>` - Grantee address to revoke allowance from (required)
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic usage:
```bash
xion-toolkit treasury fee-config remove xion1abc123def456... \
  --grantee xion1grantee789...
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operation": "remove",
  "grantee": "xion1grantee789...",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Output (error - no allowance found):
```json
{
  "success": false,
  "error": "No fee allowance found for grantee: xion1grantee789...",
  "code": "ALLOWANCE_NOT_FOUND",
  "suggestion": "Use 'fee-config query' to check existing allowances"
}
```

**Notes:**
- Removes fee allowance for specific grantee
- Grantee will need new allowance to pay gas on behalf of treasury

---

### `treasury fee-config query`

Queries fee configuration for a Treasury.

**Usage:**
```bash
xion-toolkit treasury fee-config query <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic usage:
```bash
xion-toolkit treasury fee-config query xion1abc123def456...
```

Output (success - basic allowance):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "fee_config": {
    "allowance_type": "BasicAllowance",
    "spend_limit": [{"denom": "uxion", "amount": "10000000"}],
    "description": "Basic fee allowance"
  }
}
```

Output (success - periodic allowance):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "fee_config": {
    "allowance_type": "PeriodicAllowance",
    "basic_spend_limit": [{"denom": "uxion", "amount": "10000000"}],
    "period_seconds": 86400,
    "period_spend_limit": [{"denom": "uxion", "amount": "1000000"}],
    "description": "Daily fee allowance"
  }
}
```

Output (no fee config):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "fee_config": null,
  "message": "No fee allowance configured"
}
```

**Notes:**
- Shows current fee allowance configuration
- Use `chain-query allowances` for on-chain state

---

### `treasury admin propose`

Proposes a new admin for the Treasury.

**Usage:**
```bash
xion-toolkit treasury admin propose <ADDRESS> --new-admin <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--new-admin <ADDRESS>` - New admin address (required)
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic usage:
```bash
xion-toolkit treasury admin propose xion1abc123def456... \
  --new-admin xion1newadmin789...
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operation": "propose_admin",
  "new_admin": "xion1newadmin789...",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Output (error - invalid address):
```json
{
  "success": false,
  "error": "Invalid admin address format",
  "code": "INVALID_ADDRESS",
  "suggestion": "Admin address must be a valid xion bech32 address"
}
```

Output (error - already pending):
```json
{
  "success": false,
  "error": "Admin proposal already pending for this treasury",
  "code": "PENDING_PROPOSAL",
  "suggestion": "Cancel existing proposal with 'admin cancel' first"
}
```

**Notes:**
- Creates a pending admin proposal
- New admin must call `admin accept` to complete transfer
- Current admin can cancel with `admin cancel`

---

### `treasury admin accept`

Accepts admin role (must be called by the pending admin).

**Usage:**
```bash
xion-toolkit treasury admin accept <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic usage:
```bash
xion-toolkit treasury admin accept xion1abc123def456...
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operation": "accept_admin",
  "new_admin": "xion1newadmin789...",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Output (error - not pending admin):
```json
{
  "success": false,
  "error": "No pending admin proposal for this address",
  "code": "NO_PENDING_PROPOSAL",
  "suggestion": "Ask current admin to run 'admin propose' first"
}
```

Output (error - wrong caller):
```json
{
  "success": false,
  "error": "Only the pending admin can accept the proposal",
  "code": "UNAUTHORIZED",
  "suggestion": "The proposed admin must call this command"
}
```

**Notes:**
- Must be called by the pending admin (not current admin)
- Completes the admin transfer
- Previous admin loses admin privileges

---

### `treasury admin cancel`

Cancels a proposed admin.

**Usage:**
```bash
xion-toolkit treasury admin cancel <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic usage:
```bash
xion-toolkit treasury admin cancel xion1abc123def456...
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operation": "cancel_proposed_admin",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Output (error - no pending proposal):
```json
{
  "success": false,
  "error": "No pending admin proposal to cancel",
  "code": "NO_PENDING_PROPOSAL",
  "suggestion": "Use 'admin propose' to create a new proposal first"
}
```

**Notes:**
- Can only be called by current admin
- Removes pending admin proposal
- Proposed admin will no longer be able to accept

---

### `treasury params update`

Updates Treasury parameters.

**Usage:**
```bash
xion-toolkit treasury params update <ADDRESS> [options]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--redirect-url <URL>` - OAuth redirect URL
- `--icon-url <URL>` - Treasury icon URL
- `--name <NAME>` - Display name (stored in metadata.name)
- `--is-oauth2-app` - Mark as OAuth2 application (stored in metadata.is_oauth2_app)
- `--metadata <JSON>` - Additional metadata as JSON object
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Update redirect URL:
```bash
xion-toolkit treasury params update xion1abc123def456... \
  --redirect-url "https://newurl.com/callback"
```

Output (success):
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operation": "update_params",
  "tx_hash": "A1B2C3D4E5F6..."
}
```

Update name and OAuth2 app flag:
```bash
xion-toolkit treasury params update xion1abc123def456... \
  --name "My Updated Treasury" \
  --is-oauth2-app
```

Update metadata:
```bash
xion-toolkit treasury params update xion1abc123def456... \
  --metadata '{"description":"Updated description","custom_field":"value"}'
```

Update multiple params:
```bash
xion-toolkit treasury params update xion1abc123def456... \
  --redirect-url "https://app.com/callback" \
  --icon-url "https://app.com/icon.png" \
  --name "My App Treasury" \
  --is-oauth2-app
```

Output (error - invalid JSON):
```json
{
  "success": false,
  "error": "Invalid JSON in metadata: unexpected token",
  "code": "INVALID_INPUT",
  "suggestion": "Ensure metadata is valid JSON string"
}
```

Output (error - metadata not object):
```json
{
  "success": false,
  "error": "--metadata must be a JSON object (e.g., '{\"key\": \"value\"}'), not a primitive or array",
  "code": "INVALID_INPUT",
  "suggestion": "Provide metadata as a JSON object"
}
```

**Notes:**
- Only admin can update parameters
- Partial updates supported (only provide fields to change)
- Metadata must be a valid JSON object
- `name` and `is_oauth2_app` are stored in metadata on-chain but exposed as CLI flags for convenience

---

### `treasury chain-query grants`

Queries on-chain Authz grants for a Treasury.

**Usage:**
```bash
xion-toolkit treasury chain-query grants <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "grants": [
    {
      "grantee": "xion1grantee...",
      "authorization": {
        "type": "cosmos.bank.v1beta1.SendAuthorization",
        "value": { "spend_limit": [{"denom": "uxion", "amount": "1000000"}] }
      },
      "expiration": "2025-01-01T00:00:00Z"
    }
  ],
  "count": 1
}
```

**Example:**
```bash
xion-toolkit treasury chain-query grants xion1abc123...
```

---

### `treasury chain-query allowances`

Queries on-chain fee allowances for a Treasury.

**Usage:**
```bash
xion-toolkit treasury chain-query allowances <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "allowances": [
    {
      "grantee": "xion1grantee...",
      "allowance": {
        "type": "cosmos.feegrant.v1beta1.BasicAllowance",
        "value": { "spend_limit": [{"denom": "uxion", "amount": "5000000"}] }
      }
    }
  ],
  "count": 1
}
```

**Example:**
```bash
xion-toolkit treasury chain-query allowances xion1abc123...
```

---

### `treasury export`

Export treasury configuration for backup or migration.

**Usage:**
```bash
xion-toolkit treasury export <ADDRESS> [--output <FILE>]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--output <FILE>` - Output file path (optional, default: stdout)
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Export to stdout:
```bash
xion-toolkit treasury export xion1abc123def456...
```

Output:
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "export": {
    "admin": "xion1admin789...",
    "params": {
      "redirect_url": "https://example.com/callback",
      "icon_url": "https://example.com/icon.png",
      "metadata": {
        "name": "My Treasury",
        "archived": false
      }
    },
    "fee_config": {
      "allowance_type": "BasicAllowance",
      "spend_limit": [{"denom": "uxion", "amount": "10000000"}],
      "description": "Basic fee allowance"
    },
    "grant_configs": [
      {
        "type_url": "/cosmos.bank.v1beta1.MsgSend",
        "description": "Allow sending funds",
        "authorization": {
          "type": "SendAuthorization",
          "spend_limit": [{"denom": "uxion", "amount": "1000000"}]
        }
      }
    ]
  }
}
```

Export to file:
```bash
xion-toolkit treasury export xion1abc123def456... --output treasury-backup.json
```

Output:
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "file": "treasury-backup.json",
  "message": "Configuration exported successfully"
}
```

**Notes:**
- Exports admin, params, fee_config, and grant_configs
- Use for backup, migration, or configuration sharing
- Output is compatible with `treasury import`

---

### `treasury import`

Import configuration to an existing treasury.

**Usage:**
```bash
xion-toolkit treasury import <ADDRESS> --from-file <FILE> [--dry-run]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--from-file <FILE>` - Path to JSON file containing configuration (required)
- `--dry-run` - Preview actions without executing (optional)
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Preview import (no changes):
```bash
xion-toolkit treasury import xion1abc123def456... \
  --from-file treasury-backup.json \
  --dry-run
```

Output:
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "dry_run": true,
  "planned_actions": [
    {
      "action": "update_fee_config",
      "allowance_type": "BasicAllowance",
      "spend_limit": "10000000uxion"
    },
    {
      "action": "add_grant_config",
      "type_url": "/cosmos.bank.v1beta1.MsgSend",
      "description": "Allow sending funds"
    }
  ],
  "estimated_transactions": 2
}
```

Execute import:
```bash
xion-toolkit treasury import xion1abc123def456... \
  --from-file treasury-backup.json
```

Output:
```json
{
  "success": true,
  "treasury_address": "xion1abc123def456...",
  "operations": [
    {
      "action": "update_fee_config",
      "tx_hash": "A1B2C3D4E5F6...",
      "status": "success"
    },
    {
      "action": "add_grant_config",
      "type_url": "/cosmos.bank.v1beta1.MsgSend",
      "tx_hash": "B2C3D4E5F6G7...",
      "status": "success"
    }
  ],
  "completed": 2,
  "failed": 0
}
```

**Notes:**
- Imports fee_config and grant_configs (admin and params not modified)
- Uses client-side batching of existing commands
- Supports `--dry-run` to preview actions
- Progress messages written to stderr

---

## Contract Commands

### `contract instantiate`

Instantiates a generic smart contract (v1 - dynamic address).

**Usage:**
```bash
xion-toolkit contract instantiate --code-id <ID> --label <LABEL> --msg <FILE> [options]
```

**Options:**
- `--code-id <ID>` - Code ID of the contract to instantiate (required)
- `--label <LABEL>` - Label for the contract instance (required)
- `--msg <FILE>` - Path to JSON file containing instantiate message (required)
- `--admin <ADDRESS>` - Admin address for contract migrations (optional)

**Output:**
```json
{
  "success": true,
  "tx_hash": "ABC123...",
  "code_id": 1260,
  "label": "my-contract",
  "admin": "xion1admin..."
}
```

**Example:**
```bash
xion-toolkit contract instantiate \
  --code-id 1260 \
  --label "my-contract" \
  --msg instantiate-msg.json \
  --admin xion1admin...
```

---

### `contract instantiate2`

Instantiates a smart contract with predictable address (v2 - using salt).

**Usage:**
```bash
xion-toolkit contract instantiate2 --code-id <ID> --label <LABEL> --msg <FILE> [options]
```

**Options:**
- `--code-id <ID>` - Code ID of the contract to instantiate (required)
- `--label <LABEL>` - Label for the contract instance (required)
- `--msg <FILE>` - Path to JSON file containing instantiate message (required)
- `--salt <HEX>` - Salt for predictable address (hex-encoded, auto-generated if not provided)
- `--admin <ADDRESS>` - Admin address for contract migrations (optional)

**Output:**
```json
{
  "success": true,
  "tx_hash": "ABC123...",
  "code_id": 1260,
  "label": "my-contract",
  "salt": "abc123...",
  "admin": "xion1admin...",
  "predicted_address": "xion1predicted..."
}
```

**Example:**
```bash
xion-toolkit contract instantiate2 \
  --code-id 1260 \
  --label "my-contract" \
  --msg instantiate-msg.json \
  --salt "01020304" \
  --admin xion1admin...
```

---

### `contract execute`

Executes a message on a deployed smart contract.

**Usage:**
```bash
xion-toolkit contract execute --contract <ADDRESS> --msg <FILE> [options]
```

**Options:**
- `--contract <ADDRESS>` - Contract address to execute (required)
- `--msg <FILE>` - Path to JSON file containing execute message (required)
- `--funds <AMOUNT>` - Optional funds to send (e.g., "1000000uxion")

**Output:**
```json
{
  "success": true,
  "tx_hash": "ABC123...",
  "contract": "xion1abc..."
}
```

**Examples:**
```bash
# Basic execution
xion-toolkit contract execute \
  --contract xion1abc... \
  --msg execute-msg.json

# Execution with funds
xion-toolkit contract execute \
  --contract xion1abc... \
  --msg execute-msg.json \
  --funds "1000000uxion"
```

---

### `contract query`

Query a CosmWasm smart contract (read-only operation, no authentication required).

**Usage:**
```bash
xion-toolkit contract query --contract <ADDRESS> --msg <QUERY_FILE>
```

**Options:**
- `--contract <ADDRESS>` - Contract address to query (required)
- `--msg <FILE>` - Path to JSON file containing query message (required)
- `--network <NETWORK>` - Network override (testnet, mainnet)

**Examples:**

Basic query:
```bash
# Create query file
echo '{"get_config": {}}' > query.json

# Query contract
xion-toolkit contract query --contract xion1contract... --msg query.json
```

Output:
```json
{
  "success": true,
  "contract": "xion1contract...",
  "query": {"get_config": {}},
  "result": {
    "config": {
      "admin": "xion1admin...",
      "threshold": 3
    }
  }
}
```

Query with parameters:
```bash
echo '{"get_balance": {"address": "xion1user..."}}' > balance-query.json
xion-toolkit contract query --contract xion1contract... --msg balance-query.json
```

Output:
```json
{
  "success": true,
  "contract": "xion1contract...",
  "query": {"get_balance": {"address": "xion1user..."}},
  "result": {
    "balance": "1000000",
    "denom": "uxion"
  }
}
```

Output (error - contract not found):
```json
{
  "success": false,
  "error": "Contract not found: xion1invalid...",
  "code": "CONTRACT_NOT_FOUND",
  "suggestion": "Check the contract address and ensure the contract exists on this network"
}
```

Output (error - invalid query):
```json
{
  "success": false,
  "error": "Query failed: unknown variant",
  "code": "QUERY_FAILED",
  "suggestion": "Check the query message format matches the contract's expected schema"
}
```

**Notes:**
- Read-only operation - no authentication required
- Uses direct RPC call to `/cosmwasm/wasm/v1/contract/{address}/smart/{query}`
- Query message is base64-encoded before sending

---

## Configuration Commands

### `config show`

Shows current configuration.

**Usage:**
```bash
xion-toolkit config show
```

**Output:**
```json
{
  "success": true,
  "config": {
    "version": "1.0",
    "network": "testnet"
  }
}
```

---

### `config set-network`

Switches the active network.

**Usage:**
```bash
xion-toolkit config set-network <NETWORK>
```

**Arguments:**
- `NETWORK` - Network to use: testnet, mainnet (required)

**Output:**
```json
{
  "success": true,
  "message": "Network set to testnet",
  "network": "testnet"
}
```

**Example:**
```bash
xion-toolkit config set-network testnet
xion-toolkit config set-network mainnet
```

---

### `status`

Shows toolkit status including version and configuration.

**Usage:**
```bash
xion-toolkit status
```

**Output:**
```json
{
  "success": true,
  "version": "0.1.0",
  "network": "testnet",
  "authenticated": true,
  "address": "xion1abc..."
}
```

---

## Output Format

All commands output JSON to stdout for easy Agent integration. The output always includes a `success` field:

**Success Response:**
```json
{
  "success": true,
  "data": { ... }
}
```

**Error Response:**
```json
{
  "success": false,
  "error": "Error message",
  "code": "ERROR_CODE",
  "suggestion": "Optional suggestion for fixing the error"
}
```

**Common Error Codes:**
- `NOT_AUTHENTICATED` - User not authenticated
- `TREASURY_NOT_FOUND` - Treasury address not found
- `INVALID_ADDRESS` - Invalid address format
- `INSUFFICIENT_BALANCE` - Not enough balance for operation
- `NETWORK_ERROR` - Failed to connect to API
- `INVALID_INPUT` - Invalid input parameters

---

## Networks

| Network | OAuth API | RPC | Chain ID | Treasury Code ID |
|---------|-----------|-----|----------|------------------|
| testnet | https://oauth2.testnet.burnt.com | https://rpc.xion-testnet-2.burnt.com:443 | xion-testnet-2 | 1260 |
| mainnet | https://oauth2.burnt.com | https://rpc.xion-mainnet-1.burnt.com:443 | xion-mainnet-1 | 63 |

---

## See Also

- [README.md](../README.md) - Project overview and quick start
- [SKILL.md](../skills/xion-treasury/SKILL.md) - Skill documentation for Agent integration
- [Contributing Guide](../CONTRIBUTING.md) - Contribution guidelines
