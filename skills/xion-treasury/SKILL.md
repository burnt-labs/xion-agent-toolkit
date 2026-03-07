# xion-treasury

Treasury management skill for Xion blockchain development. This skill enables AI agents to create, query, and manage Treasury contracts for gasless transactions.

## Overview

This skill wraps the `xion-toolkit` CLI tool to provide Agent-friendly Treasury management capabilities:

- **list.sh** - List all Treasury contracts owned by the authenticated user
- **query.sh** - Query detailed information about a specific Treasury
- **create.sh** - Create a new Treasury contract with fee grant and authz grant configuration
- **fund.sh** - Fund a Treasury contract
- **withdraw.sh** - Withdraw funds from a Treasury
- **grant-config.sh** - Configure Authz Grants
- **fee-config.sh** - Configure Fee Grants
- **update-params.sh** - Update Treasury parameters (coming soon)

## Prerequisites

- `xion-toolkit` CLI tool installed and in PATH
- Authenticated with `xion-oauth2` skill (required for most operations)
- Network connectivity to Xion OAuth2 API

## Quick Start

### List Your Treasuries

```bash
./scripts/list.sh
```

### Query a Treasury

```bash
./scripts/query.sh xion1abc123...
```

### Create a Treasury

```bash
# Basic creation
./scripts/create.sh --name "My Treasury" --redirect-url "https://example.com/callback"

# With fee grant configuration
./scripts/create.sh --name "My Treasury" \
  --redirect-url "https://example.com/callback" \
  --fee-allowance basic \
  --fee-spend-limit "1000000uxion"

# With authz grant configuration
./scripts/create.sh --name "My Treasury" \
  --redirect-url "https://example.com/callback" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion"

# Using config file
./scripts/create.sh --config treasury-config.json
```

### Check Status

```bash
./scripts/status.sh
```

### Create a Treasury

```bash
# Basic creation
./scripts/create.sh --name "My Treasury"

# With fee grant
./scripts/create.sh \
  --name "My Treasury" \
  --fee-allowance basic \
  --fee-spend-limit "1000000uxion"

# With config file
./scripts/create.sh --config treasury-config.json
```

### Check Status

```bash
./scripts/status.sh
```

## Script Details

### list.sh

Lists all Treasury contracts owned by the authenticated user.

**Usage:**
```bash
./scripts/list.sh [--network NETWORK] [--no-cache]
```

**Options:**
- `--network NETWORK` - Network to use: local, testnet, mainnet (default: testnet)
- `--no-cache` - Bypass cache and fetch fresh data

**Output (stdout):**
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
  "count": 1,
  "cached": false
}
```

### query.sh

Queries detailed information about a specific Treasury contract.

**Usage:**
```bash
./scripts/query.sh <ADDRESS> [--network NETWORK] [--include-grants] [--include-fee]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--network NETWORK` - Network to use (default: testnet)
- `--include-grants` - Include Authz grant information
- `--include-fee` - Include Fee grant information

**Output (stdout):**
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
    "grants": [
      {
        "granter": "xion1granter...",
        "grantee": "xion1grantee...",
        "authorization_type": "ContractExecution"
      }
    ],
    "fee_grant": {
      "allowance_type": "BasicAllowance",
      "spend_limit": [
        {
          "denom": "uxion",
          "amount": "1000000"
        }
      ]
    }
  }
}
```

### create.sh

Creates a new Treasury contract with full configuration support including fee grants and authz grants.

**Usage:**
```bash
./scripts/create.sh [OPTIONS]
```

**Options:**
- `--network NETWORK` - Network to use: local, testnet, mainnet (default: testnet)
- `--name NAME` - Treasury name (required)
- `--redirect-url URL` - OAuth redirect URL
- `--icon-url URL` - Treasury icon URL
- `--config FILE` - JSON config file with all settings

**Fee Grant Options:**
- `--fee-allowance TYPE` - Fee allowance type: basic, periodic, allowed-msg
- `--fee-spend-limit AMOUNT` - Spend limit (e.g., "1000000uxion")
- `--fee-description TEXT` - Fee grant description
- `--fee-period-seconds SECONDS` - Period duration (for periodic allowance)
- `--fee-period-spend-limit AMOUNT` - Period spend limit (for periodic allowance)

**Authz Grant Options:**
- `--grant-auth-type TYPE` - Authorization type: generic, send, stake, ibc-transfer, contract-execution
- `--grant-spend-limit AMOUNT` - Spend limit for send authorization
- `--grant-description TEXT` - Grant description

**Output (stdout):**
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
        "archived": false,
        "is_oauth2_app": false
      }
    }
  },
  "tx_hash": "ABC123..."
}
```

**Config File Format (treasury-config.json):**
```json
{
  "params": {
    "redirect_url": "https://example.com/callback",
    "icon_url": "https://example.com/icon.png",
    "metadata": {
      "name": "My Treasury",
      "is_oauth2_app": true
    }
  },
  "fee_config": {
    "description": "Basic fee allowance",
    "allowance_type": "basic",
    "spend_limit": "1000000uxion"
  },
  "grant_configs": [
    {
      "type_url": "/cosmos.bank.v1beta1.MsgSend",
      "description": "Allow sending funds",
      "authorization_type": "send",
      "spend_limit": "1000000uxion"
    }
  ]
}
```

**Examples:**

```bash
# Minimal creation
./scripts/create.sh --name "My Treasury"

# With redirect URL
./scripts/create.sh \
  --name "My Treasury" \
  --redirect-url "https://example.com/callback"

# With basic fee grant
./scripts/create.sh \
  --name "My Treasury" \
  --fee-allowance basic \
  --fee-spend-limit "1000000uxion" \
  --fee-description "Basic fee allowance"

# With periodic fee grant (daily limit)
./scripts/create.sh \
  --name "My Treasury" \
  --fee-allowance periodic \
  --fee-period-seconds 86400 \
  --fee-period-spend-limit "100000uxion" \
  --fee-description "Daily fee allowance"

# With authz grant for sending
./scripts/create.sh \
  --name "My Treasury" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --grant-description "Allow sending funds"

# Using config file (recommended for complex configurations)
./scripts/create.sh --config treasury-config.json

# Specify network
./scripts/create.sh --network testnet --name "Test Treasury"
```

**Note:** The CLI will poll for treasury indexing (up to 30 seconds) to return the complete treasury information.

### fund.sh

Funds a Treasury contract with tokens from the authenticated account.

**Usage:**
```bash
./scripts/fund.sh <ADDRESS> <AMOUNT> [--network NETWORK]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)
- `AMOUNT` - Amount to fund with denomination (required, e.g., "1000000uxion")

**Options:**
- `--network NETWORK` - Network to use (default: testnet)

**Output (stdout):**
```json
{
  "success": true,
  "treasury": "xion1abc123...",
  "amount": "1000000uxion",
  "tx_hash": "ABC123...",
  "new_balance": "2000000uxion"
}
```

**Examples:**
```bash
# Fund treasury with 1 XION
./scripts/fund.sh xion1abc123... 1000000uxion

# Fund with 5 XION on testnet
./scripts/fund.sh xion1abc123... 5000000uxion --network testnet
```

### withdraw.sh

Withdraws funds from a Treasury to the admin account.

**Usage:**
```bash
./scripts/withdraw.sh <ADDRESS> <AMOUNT> [--network NETWORK]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)
- `AMOUNT` - Amount to withdraw with denomination (required, e.g., "1000000uxion")

**Options:**
- `--network NETWORK` - Network to use (default: testnet)

**Output (stdout):**
```json
{
  "success": true,
  "treasury": "xion1abc123...",
  "amount": "1000000uxion",
  "recipient": "xion1admin...",
  "tx_hash": "ABC123...",
  "remaining_balance": "500000uxion"
}
```

**Examples:**
```bash
# Withdraw 1 XION from treasury
./scripts/withdraw.sh xion1abc123... 1000000uxion

# Withdraw 5 XION on testnet
./scripts/withdraw.sh xion1abc123... 5000000uxion --network testnet
```

### grant-config.sh

Manages Authz Grants for a Treasury contract.

**Usage:**
```bash
# Add a grant configuration
./scripts/grant-config.sh <ADDRESS> --action add --config <CONFIG_FILE> [--network NETWORK]

# Remove a grant configuration
./scripts/grant-config.sh <ADDRESS> --action remove --type-url <TYPE_URL> [--network NETWORK]

# List all grant configurations
./scripts/grant-config.sh <ADDRESS> --action list [--network NETWORK]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--action <ACTION>` - Action to perform: add, remove, list (required)
- `--config <FILE>` - JSON config file (required for add)
- `--type-url <URL>` - Type URL to remove (required for remove)
- `--network NETWORK` - Network to use (default: testnet)

**Config File Format (grant-config.json):**
```json
{
  "type_url": "/cosmos.bank.v1beta1.MsgSend",
  "description": "Allow sending funds",
  "authorization": {
    "auth_type": "send",
    "spend_limit": "1000000uxion",
    "allow_list": ["xion1recipient..."]
  },
  "optional": false
}
```

**Authorization Types:**

1. **Generic Authorization:**
   
   Note: Generic authorization is NOT allowed for `MsgExecuteContract` for security reasons.
   Use `contract_execution` authorization instead.
   
   ```json
   {
     "type_url": "/cosmos.staking.v1beta1.MsgDelegate",
     "description": "Allow delegation",
     "authorization": {
       "auth_type": "generic"
     }
   }
   ```

2. **Send Authorization:**
```json
{
  "type_url": "/cosmos.bank.v1beta1.MsgSend",
  "description": "Allow sending funds",
  "authorization": {
    "auth_type": "send",
    "spend_limit": "1000000uxion",
    "allow_list": ["xion1recipient..."]
  }
}
```

3. **Stake Authorization:**
```json
{
  "type_url": "/cosmos.staking.v1beta1.MsgDelegate",
  "description": "Allow staking operations",
  "authorization": {
    "auth_type": "stake",
    "max_tokens": "1000000uxion",
    "validators": ["xionvaloper1..."],
    "authorization_type": 1
  }
}
```

4. **IBC Transfer Authorization:**
```json
{
  "type_url": "/ibc.applications.transfer.v1.MsgTransfer",
  "description": "Allow IBC transfers",
  "authorization": {
    "auth_type": "ibc_transfer",
    "allocations": [{
      "source_port": "transfer",
      "source_channel": "channel-0",
      "spend_limit": "1000000uxion"
    }]
  }
}
```

5. **Contract Execution Authorization:**
```json
{
  "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
  "description": "Allow contract execution",
  "authorization": {
    "auth_type": "contract_execution",
    "grants": [{
      "address": "xion1contract...",
      "max_calls": 100,
      "max_funds": "1000000uxion",
      "filter_type": "allow_all"
    }]
  }
}
```

**Output (stdout):**
```json
{
  "success": true,
  "treasury": "xion1abc123...",
  "action": "add",
  "type_url": "/cosmos.bank.v1beta1.MsgSend",
  "tx_hash": "ABC123..."
}
```

### fee-config.sh

Manages Fee Grants for a Treasury contract.

**Usage:**
```bash
# Set fee configuration
./scripts/fee-config.sh <ADDRESS> --action set --config <CONFIG_FILE> [--network NETWORK]

# Remove fee configuration
./scripts/fee-config.sh <ADDRESS> --action remove [--network NETWORK]

# Query fee configuration
./scripts/fee-config.sh <ADDRESS> --action query [--network NETWORK]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--action <ACTION>` - Action to perform: set, remove, query (required)
- `--config <FILE>` - JSON config file (required for set)
- `--network NETWORK` - Network to use (default: testnet)

**Config File Formats:**

1. **Basic Allowance:**
```json
{
  "basic": {
    "spend_limit": "1000000uxion",
    "description": "Basic fee allowance for gasless transactions"
  }
}
```

2. **Periodic Allowance:**
```json
{
  "periodic": {
    "basic_spend_limit": "10000000uxion",
    "period_seconds": 86400,
    "period_spend_limit": "100000uxion",
    "description": "Daily fee allowance with 10 XION total cap"
  }
}
```

3. **Allowed Message Allowance:**
```json
{
  "allowed_msg": {
    "allowed_messages": [
      "/cosmos.bank.v1beta1.MsgSend",
      "/cosmwasm.wasm.v1.MsgExecuteContract"
    ],
    "nested_allowance": {
      "basic": {
        "spend_limit": "1000000uxion",
        "description": "Nested basic allowance"
      }
    },
    "description": "Fee allowance for specific message types"
  }
}
```

**Output (stdout):**
```json
{
  "success": true,
  "treasury": "xion1abc123...",
  "action": "set",
  "allowance_type": "BasicAllowance",
  "tx_hash": "ABC123..."
}
```

### update-params.sh (Coming Soon)

Updates Treasury parameters.

**Usage:**
```bash
./scripts/update-params.sh <ADDRESS> --params <PARAMS_FILE> [--network NETWORK]
```

**Status:**
```json
{
  "success": false,
  "error": "Parameter update is not yet implemented",
  "error_code": "FEATURE_NOT_AVAILABLE"
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
- `NOT_AUTHENTICATED` - User not authenticated, run `xion-oauth2/login.sh` first
- `TREASURY_NOT_FOUND` - Treasury address not found
- `INVALID_ADDRESS` - Invalid Treasury address format
- `NETWORK_ERROR` - Failed to connect to API
- `FEATURE_NOT_AVAILABLE` - Feature not yet implemented

## Treasury Concepts

### What is a Treasury?

A Treasury is a smart contract that provides:
- **Gasless Transactions** - Fee grants allow agents to execute transactions without holding tokens
- **Delegated Authorization** - Authz grants allow agents to perform specific actions on behalf of the treasury admin
- **Fund Management** - Deposit and withdraw tokens from the treasury

### Treasury Balance

Treasury balance is denominated in `uxion` (micro XION):
- 1 XION = 1,000,000 uxion
- Minimum recommended balance: 1,000,000 uxion (1 XION)

### Fee Grants

Fee grants allow the Treasury to pay transaction fees for authorized agents:

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

### Authz Grants

Authz grants authorize agents to execute specific message types:

```json
{
  "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
  "authorization": {
    "type": "ContractExecutionAuthorization",
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

## Integration Examples

### Using with Claude Code

```javascript
// In your Claude Code skill
{
  "tools": [
    {
      "name": "xion_treasury_list",
      "description": "List all Treasury contracts",
      "command": "./skills/xion-treasury/scripts/list.sh"
    },
    {
      "name": "xion_treasury_query",
      "description": "Query Treasury details",
      "command": "./skills/xion-treasury/scripts/query.sh ${args.address}"
    }
  ]
}
```

### Programmatic Usage

```python
import subprocess
import json

# List treasuries
result = subprocess.run(
    ['./skills/xion-treasury/scripts/list.sh'],
    capture_output=True,
    text=True
)

if result.returncode == 0:
    data = json.loads(result.stdout)
    if data['success']:
        for treasury in data['treasuries']:
            print(f"Treasury: {treasury['address']}")
            print(f"  Balance: {treasury['balance']} {treasury['denom']}")
    else:
        print(f"Error: {data['error']}")
else:
    print(f"Script failed: {result.stderr}")
```

### Checking Authentication First

```bash
# Always check authentication before treasury operations
./skills/xion-oauth2/scripts/status.sh | jq -r '.authenticated'

# If not authenticated, login first
if [ "$(./skills/xion-oauth2/scripts/status.sh | jq -r '.authenticated')" != "true" ]; then
    ./skills/xion-oauth2/scripts/login.sh
fi

# Now list treasuries
./skills/xion-treasury/scripts/list.sh
```

## Caching

The skill implements caching to reduce API calls:

- **Cache Duration**: 5 minutes
- **Cache Scope**: Per network
- **Bypass Cache**: Use `--no-cache` flag

Cache is automatically invalidated when:
- Token is refreshed
- User logs out
- Cache TTL expires

## Network Configuration

The skill supports three networks:

| Network | OAuth2 API URL | Chain ID | Treasury Code ID |
|---------|----------------|----------|------------------|
| local | http://localhost:8787 | xion-local | - |
| testnet | https://oauth2.testnet.burnt.com | xion-testnet-2 | 1260 |
| mainnet | https://oauth2.burnt.com | xion-mainnet-1 | 63 |

## Workflow Examples

### Basic Workflow

```bash
# 1. Authenticate
./skills/xion-oauth2/scripts/login.sh

# 2. List treasuries
./skills/xion-treasury/scripts/list.sh

# 3. Query specific treasury
./skills/xion-treasury/scripts/query.sh xion1abc123... --include-grants
```

### Check Balance Workflow

```bash
# Query treasury and extract balance
BALANCE=$(./skills/xion-treasury/scripts/query.sh xion1abc123... | jq -r '.treasury.balance.amount')
echo "Treasury balance: $BALANCE uxion"

# Check if balance is low
if [ "$BALANCE" -lt 1000000 ]; then
    echo "Warning: Treasury balance is low!"
    echo "Consider funding the treasury via Developer Portal"
fi
```

## Troubleshooting

### Not Authenticated

```
Error: NOT_AUTHENTICATED - Not authenticated
```

**Solution:**
```bash
./skills/xion-oauth2/scripts/login.sh
```

### Treasury Not Found

```
Error: TREASURY_NOT_FOUND - Treasury xion1abc... not found
```

**Solution:**
- Verify the address is correct
- Check you're on the right network
- Use `list.sh` to see your treasuries

### Cache Issues

If you see stale data:
```bash
./scripts/list.sh --no-cache
```

## Related Skills

- **xion-oauth2** - Authentication (required before using this skill)
- **xion-deploy** - Smart contract deployment (future)

## Version

- Skill Version: 1.0.0
- Compatible CLI Version: >=0.1.0

## License

MIT License - See main project LICENSE file
