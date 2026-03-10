# CLI Reference

Complete reference for the Xion Agent Toolkit CLI commands.

## Table of Contents

- [Global Options](#global-options)
- [Authentication Commands](#authentication-commands)
- [Treasury Commands](#treasury-commands)
- [Contract Commands](#contract-commands)
- [Configuration Commands](#configuration-commands)
- [Output Format](#output-format)

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

**Output:**
```json
{
  "success": true,
  "message": "Successfully authenticated",
  "address": "xion1abc..."
}
```

**Example:**
```bash
xion-toolkit auth login
xion-toolkit auth login --port 8080
```

---

### `auth logout`

Clears stored credentials and logs out.

**Usage:**
```bash
xion-toolkit auth logout
```

**Output:**
```json
{
  "success": true,
  "message": "Successfully logged out"
}
```

---

### `auth status`

Checks authentication status and token expiration.

**Usage:**
```bash
xion-toolkit auth status
```

**Output:**
```json
{
  "success": true,
  "authenticated": true,
  "address": "xion1abc...",
  "expires_at": "2024-01-01T00:00:00Z"
}
```

---

### `auth refresh`

Refreshes the access token using the stored refresh token.

**Usage:**
```bash
xion-toolkit auth refresh
```

**Output:**
```json
{
  "success": true,
  "message": "Token refreshed successfully",
  "expires_at": "2024-01-01T00:00:00Z"
}
```

---

## Treasury Commands

### `treasury list`

Lists all Treasury contracts owned by the authenticated user.

**Usage:**
```bash
xion-toolkit treasury list
```

**Output:**
```json
{
  "success": true,
  "treasuries": [
    {
      "address": "xion1abc123...",
      "balance": "10000000",
      "denom": "uxion",
      "admin": "xion1admin...",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "count": 1
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

**Output:**
```json
{
  "success": true,
  "treasury": {
    "address": "xion1abc123...",
    "balance": {
      "denom": "uxion",
      "amount": "10000000"
    },
    "admin": "xion1admin...",
    "params": {
      "redirect_url": "https://example.com/callback",
      "icon_url": "https://example.com/icon.png",
      "metadata": {
        "name": "My Treasury",
        "archived": false
      }
    }
  }
}
```

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

**Output:**
```json
{
  "success": true,
  "treasury": {
    "address": "xion1abc123...",
    "admin": "xion1admin...",
    "balance": "0",
    "denom": "uxion",
    "params": {
      "redirect_url": "https://example.com/callback",
      "icon_url": "",
      "metadata": {
        "name": "My Treasury",
        "archived": false
      }
    }
  },
  "tx_hash": "ABC123..."
}
```

**Examples:**
```bash
# Using config file
xion-toolkit treasury create --config treasury-config.json

# Using flags
xion-toolkit treasury create \
  --redirect-url "https://example.com/callback" \
  --icon-url "https://example.com/icon.png" \
  --name "My Treasury" \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --grant-description "Allow sending funds"
```

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

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "amount": "1000000uxion",
  "tx_hash": "ABC123..."
}
```

**Example:**
```bash
xion-toolkit treasury fund xion1abc123... --amount 1000000uxion
```

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

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "amount": "1000000uxion",
  "tx_hash": "ABC123..."
}
```

**Example:**
```bash
xion-toolkit treasury withdraw xion1abc123... --amount 1000000uxion
```

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
- `--preset <TYPE>` - Preset: send, execute, instantiate, delegate, vote

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "operation": "add",
  "type_url": "/cosmos.bank.v1beta1.MsgSend",
  "tx_hash": "ABC123..."
}
```

**Examples:**
```bash
# Add send authorization
xion-toolkit treasury grant-config add xion1abc123... \
  --type-url "/cosmos.bank.v1beta1.MsgSend" \
  --auth-type send \
  --spend-limit "1000000uxion" \
  --description "Allow sending funds"

# Add contract execution authorization
xion-toolkit treasury grant-config add xion1abc123... \
  --type-url "/cosmwasm.wasm.v1.MsgExecuteContract" \
  --auth-type contract-execution \
  --contract xion1contract... \
  --max-calls 100 \
  --max-funds "1000000uxion" \
  --filter-type allow-all \
  --description "Execute contract with limits"

# Using preset
xion-toolkit treasury grant-config add xion1abc123... \
  --preset send \
  --spend-limit "1000000uxion" \
  --description "Allow sending funds"
```

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

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "operation": "remove",
  "tx_hash": "ABC123..."
}
```

**Example:**
```bash
xion-toolkit treasury grant-config remove xion1abc123... \
  --type-url "/cosmos.bank.v1beta1.MsgSend"
```

---

### `treasury grant-config list`

Lists all Authz grant configurations for a Treasury.

**Usage:**
```bash
xion-toolkit treasury grant-config list <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "grant_configs": [
    {
      "type_url": "/cosmos.bank.v1beta1.MsgSend",
      "description": "Allow sending funds",
      "authorization": {
        "type": "SendAuthorization",
        "spend_limit": [{"denom": "uxion", "amount": "1000000"}]
      }
    }
  ],
  "count": 1
}
```

**Example:**
```bash
xion-toolkit treasury grant-config list xion1abc123...
```

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

**Config File Format:**

*Basic Allowance:*
```json
{
  "basic": {
    "spend_limit": "1000000uxion",
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
    "period_spend_limit": "100000uxion",
    "description": "Daily fee allowance"
  }
}
```

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "operation": "set",
  "allowance_type": "BasicAllowance",
  "tx_hash": "ABC123..."
}
```

**Example:**
```bash
xion-toolkit treasury fee-config set xion1abc123... --fee-config fee-config.json
```

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

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "operation": "remove",
  "tx_hash": "ABC123..."
}
```

**Example:**
```bash
xion-toolkit treasury fee-config remove xion1abc123... --grantee xion1grantee...
```

---

### `treasury fee-config query`

Queries fee configuration for a Treasury.

**Usage:**
```bash
xion-toolkit treasury fee-config query <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "fee_config": {
    "allowance_type": "BasicAllowance",
    "spend_limit": [{"denom": "uxion", "amount": "1000000"}]
  }
}
```

**Example:**
```bash
xion-toolkit treasury fee-config query xion1abc123...
```

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

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "operation": "propose_admin",
  "new_admin": "xion1newadmin...",
  "tx_hash": "ABC123..."
}
```

**Example:**
```bash
xion-toolkit treasury admin propose xion1abc123... --new-admin xion1newadmin...
```

---

### `treasury admin accept`

Accepts admin role (must be called by the pending admin).

**Usage:**
```bash
xion-toolkit treasury admin accept <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "operation": "accept_admin",
  "tx_hash": "ABC123..."
}
```

**Example:**
```bash
xion-toolkit treasury admin accept xion1abc123...
```

---

### `treasury admin cancel`

Cancels a proposed admin.

**Usage:**
```bash
xion-toolkit treasury admin cancel <ADDRESS>
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "operation": "cancel_proposed_admin",
  "tx_hash": "ABC123..."
}
```

**Example:**
```bash
xion-toolkit treasury admin cancel xion1abc123...
```

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
- `--metadata <JSON>` - Metadata as JSON string

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "tx_hash": "ABC123..."
}
```

**Examples:**
```bash
# Update redirect URL
xion-toolkit treasury params update xion1abc123... \
  --redirect-url "https://example.com/callback"

# Update metadata
xion-toolkit treasury params update xion1abc123... \
  --metadata '{"name":"Updated Treasury"}'

# Update multiple params
xion-toolkit treasury params update xion1abc123... \
  --redirect-url "https://app.com/callback" \
  --icon-url "https://app.com/icon.png" \
  --metadata '{"name":"My App"}'
```

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
