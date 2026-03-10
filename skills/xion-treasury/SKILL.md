---
name: xion-treasury
description: |
  Treasury management for Xion blockchain gasless transactions. Use this skill whenever the user needs to: create/query/manage Treasury contracts, fund or withdraw from treasuries, configure Authz grants, set up fee allowances, manage gasless transactions, or perform any Treasury-related operations on Xion. Triggers on: treasury, treasuries, gasless, fee grant, authz grant, xion treasury, treasury create, treasury fund, treasury withdraw, metaaccount, burnt labs treasury. Use AFTER xion-oauth2 skill - authentication is required for all Treasury operations.
metadata:
  author: burnt-labs
  version: "1.1.0"
  requires:
    - xion-toolkit-init
    - xion-oauth2
compatibility: Requires xion-toolkit CLI and OAuth2 authentication
---

# xion-treasury

Treasury management skill for Xion blockchain. Enables gasless transactions through Treasury contracts with fee grants and authz grants.

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
| `chain-query.sh` | On-chain queries |

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

- **xion-oauth2** - Authentication (use before this skill)
- **xion-toolkit-init** - CLI installation (use if CLI not found)

## Version

- Skill Version: 1.1.0
- Compatible CLI Version: >=0.1.0
