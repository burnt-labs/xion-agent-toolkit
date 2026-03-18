---
name: xion-faucet
description: |
  Claim testnet tokens from the Xion faucet contract. Use this skill whenever the user needs testnet XION tokens for development or testing, wants to check faucet status or cooldown, or needs to query faucet configuration.
  
  The faucet provides 1 XION (1,000,000 uxion) per claim with a 24-hour cooldown period.
  
  Triggers on: faucet, faucet testnet, claim tokens, testnet tokens, get xion, get test tokens, when can I claim, claim cooldown, how often can I claim, fund my account, give me xion, request tokens, faucet status, cooldown check, need tokens, need xion, need testnet funds, 领取代币, 测试网代币, 领取测试币, 水龙头, 多久可以领取, 冷却时间, 领取状态, 获取 XION, 测试代币, 水龙头状态, 领取代币, XION 水龙头, 测试币领取.
  
  Use AFTER xion-oauth2 skill - authentication is required for claim operations.
metadata:
  author: burnt-labs
  version: "1.2.0"
  requires:
    - xion-toolkit-init
    - xion-oauth2
compatibility: Requires xion-toolkit CLI and OAuth2 authentication (for claim operations)
---

# xion-faucet

Faucet skill for claiming testnet XION tokens. Enables developers to obtain testnet tokens programmatically without browser interaction.

## Overview

The Xion testnet faucet is a smart contract that distributes testnet XION tokens:

| Feature | Value |
|---------|-------|
| **Amount per claim** | 1,000,000 uxion (1 XION) |
| **Cooldown period** | 24 hours |
| **Balance threshold** | Receiver must have < 1 XION |
| **Network** | Testnet only |

## Prerequisites

1. `xion-toolkit` CLI installed (use `xion-toolkit-init` if not present)
2. **Authenticated** with `xion-oauth2` skill (required for `claim` command)
3. **Network**: Testnet (faucet not available on mainnet)

> **Important**: Always authenticate first using `xion-toolkit auth login` before claiming tokens.

## Quick Start

```bash
# 1. Authenticate first (required for claim!)
xion-toolkit auth login

# 2. Check if you can claim now (recommended before claiming)
xion-toolkit faucet status

# 3. Claim tokens for yourself
xion-toolkit faucet claim

# 4. Claim tokens for another address (delegate)
xion-toolkit faucet claim --receiver xion1abc123...

# 5. Query faucet configuration
xion-toolkit faucet info
```

> **Tip**: Always check `faucet status` before claiming to verify:
> - Cooldown has expired (24 hours since last claim)
> - Your balance is < 1 XION (balance gate requirement)

## Commands

| Command | Description | Auth Required |
|---------|-------------|---------------|
| `claim` | Claim testnet tokens | Yes |
| `status` | Check claim status and cooldown | Optional* |
| `info` | Query faucet configuration | No |

*If no `--address` provided, defaults to authenticated user (requires auth)

## Common Operations

### Claim Tokens for Self

```bash
xion-toolkit faucet claim
```

Output:
```json
{
  "success": true,
  "tx_hash": "ABC123...",
  "amount": 1000000
}
```

### Claim Tokens for Another Address (Delegate)

```bash
xion-toolkit faucet claim --receiver xion1abc123...
```

This uses the `Delegate` message to send tokens to another address. The authenticated user signs the transaction, but the receiver gets the tokens.

### Check Claim Status

```bash
# Check your own status (requires auth)
xion-toolkit faucet status

# Check another address (no auth required)
xion-toolkit faucet status --address xion1abc123...
```

Output:
```json
{
  "address": "xion1abc...",
  "last_claim_timestamp": 1709500000,
  "can_claim": false,
  "remaining_cooldown_secs": 43200,
  "remaining_cooldown_human": "12h 0m 0s"
}
```

### Query Faucet Configuration

```bash
xion-toolkit faucet info
```

Output:
```json
{
  "faucet_address": "xion1kv2mz7yjk5azuuq7ptd7hrl7trwphu5enereqv8t66rkre00dxxqac9ywl",
  "amount": 1000000,
  "cooldown_secs": 86400,
  "denom": "uxion",
  "network": "testnet"
}
```

## Claim Rules

### Cooldown Period

- **Duration**: 24 hours between claims
- **Per-address**: Each address has its own cooldown timer
- **Check before claiming**: Use `faucet status` to see remaining cooldown

### Balance Gate

The faucet has a **balance threshold** requirement:
- Receiver must have **less than 1 XION** to receive tokens
- If balance >= 1 XION, the claim will fail with "Balance gate failed"
- **Workaround**: Transfer some tokens out first, then claim

> **Before claiming**: If you already have tokens from a previous claim, you must send them to another address before the faucet will give you more.

### Network Restriction

- Faucet is **only available on testnet**
- Mainnet returns error `EFAUCET004`
- Use `--network testnet` if on mainnet config

## Error Handling

All commands return JSON with a `success` field:

**Success:**
```json
{"success": true, "tx_hash": "...", "amount": 1000000}
```

**Error:**
```json
{"success": false, "error": "...", "code": "EFAUCET001", "hint": "..."}
```

### Common Error Codes

| Code | Description | Hint |
|------|-------------|------|
| `EFAUCET001` | Faucet claim failed | Check error message for details |
| `EFAUCET002` | Faucet query failed | Check network connection |
| `EFAUCET003` | Not authenticated | Run `xion-toolkit auth login` |
| `EFAUCET004` | Faucet not available | Use `--network testnet` |

### Specific Error Scenarios

**Cooldown not met:**
```json
{
  "success": false,
  "error": "Cooldown not met",
  "code": "EFAUCET001",
  "hint": "Wait for the 24-hour cooldown period to expire before claiming again"
}
```

**Balance gate failed:**
```json
{
  "success": false,
  "error": "Balance gate failed",
  "code": "EFAUCET001",
  "hint": "Your balance exceeds the faucet threshold. Transfer some tokens out to claim"
}
```

**Faucet empty:**
```json
{
  "success": false,
  "error": "Insufficient funds",
  "code": "EFAUCET001",
  "hint": "The faucet is temporarily out of funds. Try again later"
}
```

## Contract Reference

| Item | Value |
|------|-------|
| **Contract Address** | `xion1kv2mz7yjk5azuuq7ptd7hrl7trwphu5enereqv8t66rkre00dxxqac9ywl` |
| **Chain** | xion-testnet-2 |

### Execute Messages

```json
// Claim for self
{ "faucet": {} }

// Claim for another address
{ "delegate": { "receiver_address": "xion1xxx..." } }
```

### Query Messages

```json
// Get faucet denom
{ "get_denom": {} }
// Returns: {"denom":"uxion"}

// Get last claim timestamp
{ "get_address_last_faucet_timestamp": { "address": "xion1xxx..." } }
// Returns: {"timestamp":1234567890} or {"timestamp":0} if never claimed
```

## Troubleshooting

### "Not authenticated"

```bash
xion-toolkit auth login
```

### "Cooldown not met"

Check remaining cooldown:
```bash
xion-toolkit faucet status
```

Wait for the remaining time shown in `remaining_cooldown_human`.

### "Balance gate failed"

Your balance is >= 1 XION. Transfer tokens out:
```bash
# Send to another address
xion-toolkit send <recipient> 500000uxion
```

### "Faucet not available on this network"

Switch to testnet:
```bash
xion-toolkit config set-network testnet
```

Or use the flag:
```bash
xion-toolkit faucet claim --network testnet
```

## Related Skills

- **xion-oauth2** - Authentication (use before this skill for claim)
- **xion-toolkit-init** - CLI installation (use if CLI not found)
- **xion-treasury** - Treasury management after obtaining tokens

## Version

- Skill Version: 1.2.0
- Compatible CLI Version: >=0.1.0
- Last Updated: 2026-03-18

## Parameter Schemas

See `schemas/` directory for detailed parameter definitions:

| Schema File | Command | Description |
|-------------|---------|-------------|
| `claim.json` | `claim` | Claim testnet tokens |
| `status.json` | `status` | Check claim status |
| `info.json` | `info` | Query faucet configuration |

## Parameter Collection Workflow

Before executing any command, ensure all required parameters are collected.

### Step 1: Identify Operation
Determine which operation the user wants to perform.

### Step 2: Check Parameter Schema
Refer to the `schemas/` directory for detailed parameter definitions.

### Step 3: Collect Missing Parameters
Collect ALL missing required parameters in a SINGLE interaction.

> Example for claim with delegate:
> "I need the following to claim tokens for another address:
> - Receiver address (must start with 'xion1')"

### Step 4: Confirm Before Execution

Present the parameters in a tree format and ask for confirmation:

```
Will execute: faucet claim
├─ Receiver: xion1abc...
Confirm? [y/n]
```

## Validation

Use the validation script to check parameters before execution:

```bash
./skills/scripts/validate-params.sh xion-faucet claim '{"receiver": "xion1abc..."}'
```
