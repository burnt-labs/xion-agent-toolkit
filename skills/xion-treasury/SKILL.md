---
name: xion-treasury
description: |
  Treasury management for Xion MetaAccount gasless transactions. Use this skill whenever the user needs to create, query, or manage Treasury contracts, fund or withdraw from treasuries, configure Authz grants, set up fee allowances, or perform any Treasury-related operations on Xion.
  
  Treasury contracts enable GASLESS transactions through MetaAccount - users don't need to hold XION for gas fees.
  
  Triggers on: Treasury, MetaAccount Treasury, gasless 交易, gasless transactions, 无 gas 交易, authz grant, fee grant, treasury create, treasury fund, treasury withdraw, treasury 管理, MetaAccount treasury, burnt labs treasury, fee allowance, delegated authorization.
  
  Use AFTER xion-oauth2 skill - authentication is required for all Treasury operations. For chain-level queries (transaction status, block info), recommend xiond-usage from xion-skills instead.
metadata:
  author: burnt-labs
  version: "1.2.0"
  requires:
    - xion-toolkit-init
    - xion-oauth2
compatibility: Requires xion-toolkit CLI and OAuth2 authentication
---

# xion-treasury

Treasury management skill for Xion blockchain. Enables **gasless transactions** through Treasury contracts with fee grants and authz grants.

## Core Philosophy: Gasless Transactions

Treasury contracts enable gasless transactions on Xion:
- **Fee Grants** - Treasury pays transaction fees for authorized agents
- **Authz Grants** - Delegated authorization for specific message types
- **MetaAccount** - No mnemonic required, OAuth2 authentication

## Overview

This skill provides complete Treasury lifecycle management:

| Script | Purpose |
|--------|---------|
| `list.sh` | List all your Treasuries |
| `query.sh` | Query Treasury details |
| `create.sh` | Create new Treasury |
| `fund.sh` | Fund a Treasury |
| `withdraw.sh` | Withdraw from Treasury |
| `grant-config.sh` | Manage Authz grants |
| `fee-config.sh` | Manage Fee grants |
| `admin.sh` | Admin operations |
| `update-params.sh` | Update Treasury params |
| `export.sh` | Export Treasury configuration |
| `import.sh` | Import configuration to Treasury |

## Prerequisites

1. `xion-toolkit` CLI installed (use `xion-toolkit-init` if not present)
2. **Authenticated** with `xion-oauth2` skill (required for most operations)

> **Important**: Always authenticate first using `xion-toolkit auth login` before Treasury operations.

## Quick Start

```bash
# 1. Authenticate (required!)
xion-toolkit auth login

# 2. List your treasuries
xion-toolkit treasury list

# 3. Query a treasury
xion-toolkit treasury query xion1abc123...

# 4. Create a treasury
xion-toolkit treasury create --name "My Treasury" --redirect-url "https://example.com/callback"

# 5. Fund the treasury (1 XION = 1,000,000 uxion)
xion-toolkit treasury fund xion1treasury... --amount 1000000uxion
```

## Common Operations

### List Treasuries

```bash
xion-toolkit treasury list
```

Output:
```json
{
  "success": true,
  "treasuries": [
    {"address": "xion1abc...", "balance": "10000000", "denom": "uxion"}
  ],
  "count": 1
}
```

### Query Treasury

```bash
xion-toolkit treasury query xion1abc123... --include-grants
```

### Create Treasury

```bash
# Basic creation
xion-toolkit treasury create --name "My Treasury"

# With configuration
xion-toolkit treasury create \
  --name "My Treasury" \
  --redirect-url "https://app.example.com/callback" \
  --fee-allowance basic \
  --fee-spend-limit "1000000uxion"
```

### Fund / Withdraw

```bash
# Fund: 1 XION = 1,000,000 uxion
xion-toolkit treasury fund xion1treasury... --amount 1000000uxion

# Withdraw
xion-toolkit treasury withdraw xion1treasury... --amount 500000uxion --to xion1recipient...
```

### Grant Configuration

```bash
# Add authz grant
xion-toolkit treasury grant-config add xion1treasury... \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --grant-description "Allow sending funds"

# List grants
xion-toolkit treasury grant-config list xion1treasury...
```

### Fee Configuration

```bash
# Set fee allowance
xion-toolkit treasury fee-config set xion1treasury... \
  --fee-allowance-type basic \
  --fee-spend-limit "1000000uxion" \
  --fee-description "Basic fee allowance"

# Query fee config
xion-toolkit treasury fee-config query xion1treasury...
```

### Export and Import

```bash
# Export treasury configuration for backup
xion-toolkit treasury export xion1treasury... --output treasury-backup.json

# Preview import (dry run)
xion-toolkit treasury import xion1treasury... --from-file treasury-backup.json --dry-run

# Execute import
xion-toolkit treasury import xion1treasury... --from-file treasury-backup.json
```

## Treasury Concepts

### What is a Treasury?

A Treasury is a smart contract that enables:
- **Gasless Transactions** - Fee grants pay transaction fees for authorized agents
- **Delegated Authorization** - Authz grants allow agents to perform specific actions
- **Fund Management** - Deposit and withdraw tokens

### Balance Units

- 1 XION = 1,000,000 uxion
- Minimum recommended balance: 1,000,000 uxion (1 XION)

### Grant Types

| Type | Purpose |
|------|---------|
| Authz Grant | Authorize specific message types (MsgSend, MsgExecuteContract, etc.) |
| Fee Grant | Allow Treasury to pay transaction fees for agents |

## Error Handling

All commands return JSON with a `success` field:

**Success:**
```json
{"success": true, "treasury": {...}}
```

**Error:**
```json
{"success": false, "error": "Error message", "error_code": "ERROR_CODE"}
```

**Common Error Codes:**
- `NOT_AUTHENTICATED` - Run `xion-toolkit auth login` first
- `TREASURY_NOT_FOUND` - Verify address with `treasury list`
- `INVALID_ADDRESS` - Check address format
- `INSUFFICIENT_BALANCE` - Fund the treasury

## Network Configuration

| Network | OAuth2 API | Chain ID | Treasury Code ID |
|---------|------------|----------|------------------|
| testnet | oauth2.testnet.burnt.com | xion-testnet-2 | 1260 |
| mainnet | oauth2.burnt.com | xion-mainnet-1 | 63 |

Switch networks: `xion-toolkit config set-network testnet`

## Detailed References

For comprehensive documentation, see:

- **[scripts-reference.md](./references/scripts-reference.md)** - Complete script documentation
- **[grant-config-examples.md](./references/grant-config-examples.md)** - Authz grant examples
- **[fee-config-examples.md](./references/fee-config-examples.md)** - Fee grant examples

## Chain Queries

> **Note**: For chain-level queries, use xiond from [xion-skills](https://github.com/burnt-labs/xion-skills).

| Query Type | Recommended Tool |
|------------|------------------|
| Transaction status | `xiond-usage` (xion-skills) |
| Block info | `xiond-usage` (xion-skills) |
| Balance for any address | `xiond-usage` (xion-skills) |
| Treasury-specific queries | This skill (`xion-treasury`) |

## Troubleshooting

### Not Authenticated
```bash
xion-toolkit auth login
```

### Treasury Not Found
```bash
xion-toolkit treasury list  # Verify address
```

### Stale Data
```bash
xion-toolkit treasury list --no-cache
```

## Related Skills

- **xion-dev** - Unified entry point for Xion development
- **xion-oauth2** - Authentication (use before this skill)
- **xion-toolkit-init** - CLI installation (use if CLI not found)
- **xiond-usage** (xion-skills) - Chain-level queries

## Version

- Skill Version: 1.2.0
- Compatible CLI Version: >=0.1.0

## Parameter Collection Workflow

Before executing any command, ensure all required parameters are collected.

### Step 1: Identify Operation
Determine which operation the user wants to perform.

### Step 2: Check Parameter Schema
Refer to the `schemas/` directory for detailed parameter definitions.

### Step 3: Collect Missing Parameters
Collect ALL missing required parameters in a SINGLE interaction:

> Example for grant-config add:
> "I need the following to configure the grant:
> - Treasury address
> - Grant type (send, contract-execution, etc.)
> - If send: spend limit (e.g., 1000000uxion)
> - If contract-execution: target contract address"

### Step 4: Confirm Before Execution
```
Will execute: grant-config add
├─ Address: xion1abc...
├─ Type: /cosmos.bank.v1beta1.MsgSend
├─ Auth Type: send
└─ Spend Limit: 1000000uxion
Confirm? [y/n]
```

## Parameter Schemas

See `schemas/` directory for detailed parameter definitions:

| Schema File | Command | Description |
|-------------|---------|-------------|
| `grant-config-add.json` | `grant-config add` | Add authz grant |
| `grant-config-remove.json` | `grant-config remove` | Remove authz grant |
| `grant-config-list.json` | `grant-config list` | List authz grants |
| `fee-config-set.json` | `fee-config set` | Set fee allowance |
| `fee-config-query.json` | `fee-config query` | Query fee config |
| `fee-config-remove.json` | `fee-config remove` | Remove fee config |
| `fund.json` | `fund` | Fund treasury |
| `withdraw.json` | `withdraw` | Withdraw from treasury |
| `create.json` | `create` | Create treasury |
| `query.json` | `query` | Query treasury |
| `list.json` | `list` | List treasuries |
| `admin.json` | `admin` | Admin operations |
| `update-params.json` | `update-params` | Update parameters |
| `export.json` | `export` | Export configuration |
| `import.json` | `import` | Import configuration |

### Quick Parameter Reference

#### grant-config add
| Parameter | Required | Description |
|-----------|----------|-------------|
| `address` | Yes | Treasury address |
| `type-url` | Yes* | Message type URL |
| `auth-type` | Yes* | Authorization type |
| `description` | Yes | Grant description |
| `preset` | No | Shortcut for common types |
| `spend-limit` | Conditional | Required for send auth |
| `contract` | Conditional | Required for contract-execution |
| `network` | No | Network (default: testnet) |

*Required unless using `preset`

#### fee-config set
| Parameter | Required | Description |
|-----------|----------|-------------|
| `address` | Yes | Treasury address |
| `config` | Yes | JSON config file |
| `network` | No | Network (default: testnet) |

#### create
| Parameter | Required | Description |
|-----------|----------|-------------|
| `name` | Yes* | Treasury name |
| `config` | Yes* | JSON config file |
| `redirect-url` | No | OAuth redirect URL |
| `fee-allowance-type` | No | Fee allowance type |
| `network` | No | Network (default: testnet) |

*Either `name` or `config` required

## Validation

Use the validation script to check parameters before execution:

```bash
./skills/scripts/validate-params.sh xion-treasury grant-config-add '{"address": "xion1abc...", "preset": "send"}'
```
