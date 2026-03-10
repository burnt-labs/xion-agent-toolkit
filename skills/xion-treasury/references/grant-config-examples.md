# Grant Configuration Examples

This reference provides detailed examples for configuring Authz grants with the `grant-config.sh` script.

## Authorization Types Overview

| Type | Use Case | Security Level |
|------|----------|----------------|
| Generic | Delegate any message type | High - use carefully |
| Send | Allow token transfers | Medium |
| Stake | Allow staking operations | Medium |
| IBC Transfer | Allow cross-chain transfers | Medium |
| Contract Execution | Allow smart contract calls | Low - most specific |

---

## 1. Generic Authorization

Allows the grantee to execute any message of the specified type.

**Important**: Generic authorization is NOT allowed for `MsgExecuteContract` for security reasons. Use `contract_execution` instead.

```json
{
  "type_url": "/cosmos.staking.v1beta1.MsgDelegate",
  "description": "Allow delegation to any validator",
  "authorization": {
    "auth_type": "generic"
  },
  "optional": false
}
```

---

## 2. Send Authorization

Allows token transfers with spend limits and optional allowlist.

```json
{
  "type_url": "/cosmos.bank.v1beta1.MsgSend",
  "description": "Allow sending funds with limits",
  "authorization": {
    "auth_type": "send",
    "spend_limit": "1000000uxion",
    "allow_list": ["xion1recipient1...", "xion1recipient2..."]
  },
  "optional": false
}
```

**Fields:**
- `spend_limit` - Maximum amount that can be sent (e.g., "1000000uxion")
- `allow_list` - (Optional) Array of addresses that can receive funds

---

## 3. Stake Authorization

Allows staking operations with validator restrictions.

```json
{
  "type_url": "/cosmos.staking.v1beta1.MsgDelegate",
  "description": "Allow staking to specific validators",
  "authorization": {
    "auth_type": "stake",
    "max_tokens": "1000000uxion",
    "validators": ["xionvaloper1...", "xionvaloper2..."],
    "authorization_type": 1
  },
  "optional": false
}
```

**Fields:**
- `max_tokens` - Maximum tokens that can be delegated
- `validators` - List of allowed validator addresses
- `authorization_type` - 1 = Delegate, 2 = Undelegate, 3 = Redelegate

---

## 4. IBC Transfer Authorization

Allows cross-chain transfers with channel-specific limits.

```json
{
  "type_url": "/ibc.applications.transfer.v1.MsgTransfer",
  "description": "Allow IBC transfers on specific channels",
  "authorization": {
    "auth_type": "ibc_transfer",
    "allocations": [
      {
        "source_port": "transfer",
        "source_channel": "channel-0",
        "spend_limit": "1000000uxion"
      }
    ]
  },
  "optional": false
}
```

**Fields:**
- `allocations` - Array of channel allocations
  - `source_port` - IBC port (usually "transfer")
  - `source_channel` - IBC channel ID
  - `spend_limit` - Maximum amount per channel

---

## 5. Contract Execution Authorization

Allows calling smart contracts with specific limits. This is the recommended authorization type for `MsgExecuteContract`.

```json
{
  "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
  "description": "Allow contract execution with limits",
  "authorization": {
    "auth_type": "contract_execution",
    "grants": [
      {
        "address": "xion1contract...",
        "max_calls": 100,
        "max_funds": "1000000uxion",
        "filter_type": "allow_all"
      }
    ]
  },
  "optional": false
}
```

**Fields:**
- `grants` - Array of contract grants
  - `address` - Contract address
  - `max_calls` - Maximum number of calls allowed
  - `max_funds` - Maximum funds that can be attached to calls
  - `filter_type` - "allow_all", "allow_messages", or "deny_messages"

---

## Common Patterns

### Pattern: Allow All MsgSend

```json
{
  "type_url": "/cosmos.bank.v1beta1.MsgSend",
  "description": "Allow all sends up to limit",
  "authorization": {
    "auth_type": "send",
    "spend_limit": "10000000uxion"
  }
}
```

### Pattern: Contract Execution Only

```json
{
  "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
  "description": "Execute specific contract",
  "authorization": {
    "auth_type": "contract_execution",
    "grants": [
      {
        "address": "xion1mycontract...",
        "max_calls": 1000,
        "max_funds": "5000000uxion",
        "filter_type": "allow_all"
      }
    ]
  }
}
```

### Pattern: Multiple Grants

```bash
# Create multiple grants by running grant-config.sh multiple times
./scripts/grant-config.sh xion1treasury... --action add --config grant1.json
./scripts/grant-config.sh xion1treasury... --action add --config grant2.json
```
