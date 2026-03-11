# Fee Configuration Examples

This reference provides detailed examples for configuring Fee grants with the `fee-config.sh` script.

## Allowance Types Overview

| Type | Use Case | Complexity |
|------|----------|------------|
| Basic | Simple spend limit | Low |
| Periodic | Daily/weekly limits | Medium |
| Allowed Messages | Limit by message type | High |

---

## 1. Basic Allowance

Simple allowance with a total spend limit.

**When to use**: Simple cases where you want to set a total fee budget.

```json
{
  "basic": {
    "spend_limit": "1000000uxion",
    "description": "Basic fee allowance for gasless transactions"
  }
}
```

**Example command:**
```bash
echo '{"basic":{"spend_limit":"1000000uxion","description":"Basic fee allowance"}}' > fee-config.json
./scripts/fee-config.sh xion1treasury... --action set --config fee-config.json
```

---

## 2. Periodic Allowance

Allowance that resets on a schedule (daily, weekly, etc.).

**When to use**: Rate limiting for ongoing operations.

### Daily Allowance

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

### Weekly Allowance

```json
{
  "periodic": {
    "basic_spend_limit": "50000000uxion",
    "period_seconds": 604800,
    "period_spend_limit": "500000uxion",
    "description": "Weekly fee allowance with 50 XION total cap"
  }
}
```

### Hourly Allowance

```json
{
  "periodic": {
    "basic_spend_limit": "1000000uxion",
    "period_seconds": 3600,
    "period_spend_limit": "10000uxion",
    "description": "Hourly fee allowance for high-frequency operations"
  }
}
```

---

## 3. Allowed Message Allowance

Allowance restricted to specific message types.

**When to use**: Fine-grained control over which operations can use fee grants.

### Single Message Type

```json
{
  "allowed_msg": {
    "allowed_messages": [
      "/cosmos.bank.v1beta1.MsgSend"
    ],
    "nested_allowance": {
      "basic": {
        "spend_limit": "1000000uxion",
        "description": "Fee allowance only for MsgSend"
      }
    },
    "description": "Fee allowance for bank sends only"
  }
}
```

### Multiple Message Types

```json
{
  "allowed_msg": {
    "allowed_messages": [
      "/cosmos.bank.v1beta1.MsgSend",
      "/cosmwasm.wasm.v1.MsgExecuteContract"
    ],
    "nested_allowance": {
      "basic": {
        "spend_limit": "5000000uxion",
        "description": "Fee allowance for sends and contract calls"
      }
    },
    "description": "Fee allowance for bank sends and contract execution"
  }
}
```

### With Periodic Nested Allowance

```json
{
  "allowed_msg": {
    "allowed_messages": [
      "/cosmwasm.wasm.v1.MsgExecuteContract"
    ],
    "nested_allowance": {
      "periodic": {
        "basic_spend_limit": "5000000uxion",
        "period_seconds": 86400,
        "period_spend_limit": "100000uxion",
        "description": "Daily limit for contract calls"
      }
    },
    "description": "Daily fee allowance for contract execution only"
  }
}
```

---

## Common Patterns

### Pattern: Simple Testing

```json
{
  "basic": {
    "spend_limit": "100000uxion",
    "description": "Small allowance for testing"
  }
}
```

### Pattern: Production with Rate Limiting

```json
{
  "periodic": {
    "basic_spend_limit": "100000000uxion",
    "period_seconds": 86400,
    "period_spend_limit": "1000000uxion",
    "description": "Production: 100 XION total, 1 XION daily"
  }
}
```

### Pattern: Restricted to Contract Calls

```json
{
  "allowed_msg": {
    "allowed_messages": [
      "/cosmwasm.wasm.v1.MsgExecuteContract"
    ],
    "nested_allowance": {
      "periodic": {
        "basic_spend_limit": "10000000uxion",
        "period_seconds": 3600,
        "period_spend_limit": "100000uxion",
        "description": "Contract calls with hourly rate limit"
      }
    },
    "description": "Gasless contract execution with rate limiting"
  }
}
```

---

## Removing Fee Configuration

```bash
./scripts/fee-config.sh xion1treasury... --action remove
```

---

## Querying Fee Configuration

```bash
./scripts/fee-config.sh xion1treasury... --action query
```

Output:
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "allowance": {
    "type": "BasicAllowance",
    "spend_limit": [
      {"denom": "uxion", "amount": "1000000"}
    ]
  }
}
```
