# Treasury Scripts Reference

Detailed documentation for all xion-treasury scripts.

## Table of Contents

- [list.sh](#listsh)
- [query.sh](#querysh)
- [create.sh](#createsh)
- [fund.sh](#fundsh)
- [withdraw.sh](#withdrawsh)
- [grant-config.sh](#grant-configsh)
- [fee-config.sh](#fee-configsh)
- [admin.sh](#adminsh)
- [update-params.sh](#update-paramssh)
- [export.sh](#exportsh)
- [import.sh](#importsh)

---

## list.sh

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

---

## query.sh

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
    "grants": [...],
    "fee_grant": {...}
  }
}
```

---

## create.sh

Creates a new Treasury contract with full configuration support.

**Usage:**
```bash
./scripts/create.sh [OPTIONS]
```

**Core Options:**
- `--network NETWORK` - Network to use (default: testnet)
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
    ...
  },
  "tx_hash": "ABC123..."
}
```

**Examples:**
```bash
# Minimal creation
./scripts/create.sh --name "My Treasury"

# With basic fee grant
./scripts/create.sh \
  --name "My Treasury" \
  --fee-allowance basic \
  --fee-spend-limit "1000000uxion"

# With periodic fee grant (daily limit)
./scripts/create.sh \
  --name "My Treasury" \
  --fee-allowance periodic \
  --fee-period-seconds 86400 \
  --fee-period-spend-limit "100000uxion"

# Using config file
./scripts/create.sh --config treasury-config.json
```

---

## fund.sh

Funds a Treasury contract with tokens from the authenticated account.

**Usage:**
```bash
./scripts/fund.sh <ADDRESS> <AMOUNT> [--network NETWORK]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)
- `AMOUNT` - Amount to fund with denomination (required, e.g., "1000000uxion")

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

---

## withdraw.sh

Withdraws funds from a Treasury to the admin account.

**Usage:**
```bash
./scripts/withdraw.sh <ADDRESS> <AMOUNT> [--network NETWORK]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)
- `AMOUNT` - Amount to withdraw with denomination (required)

---

## grant-config.sh

Manages Authz Grants for a Treasury contract. See `grant-config-examples.md` for detailed authorization types.

**Usage:**
```bash
# Add a grant configuration
./scripts/grant-config.sh <ADDRESS> --action add --config <CONFIG_FILE>

# Remove a grant configuration
./scripts/grant-config.sh <ADDRESS> --action remove --type-url <TYPE_URL>

# List all grant configurations
./scripts/grant-config.sh <ADDRESS> --action list
```

---

## fee-config.sh

Manages Fee Grants for a Treasury contract. See `fee-config-examples.md` for allowance types.

**Usage:**
```bash
# Set fee configuration
./scripts/fee-config.sh <ADDRESS> --action set --config <CONFIG_FILE>

# Remove fee configuration
./scripts/fee-config.sh <ADDRESS> --action remove

# Query fee configuration
./scripts/fee-config.sh <ADDRESS> --action query
```

---

## admin.sh

Manages Treasury admin operations.

**Usage:**
```bash
# Propose a new admin
./scripts/admin.sh <ADDRESS> propose --new-admin <NEW_ADMIN_ADDRESS>

# Accept admin role (called by pending admin)
./scripts/admin.sh <ADDRESS> accept

# Cancel proposed admin
./scripts/admin.sh <ADDRESS> cancel
```

---

## update-params.sh

Updates Treasury contract parameters.

**Usage:**
```bash
./scripts/update-params.sh <ADDRESS> [options]
```

**Options:**
- `--redirect-url <URL>` - OAuth redirect URL
- `--icon-url <URL>` - Treasury icon URL
- `--metadata <JSON>` - Metadata as JSON string

---

## export.sh

Exports Treasury configuration (grants, fee config, params) to a JSON file for backup or migration.

**Usage:**
```bash
./scripts/export.sh <ADDRESS> [--output FILE] [--network NETWORK]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--output FILE` - Output file path (optional, defaults to stdout)
- `--network NETWORK` - Network: testnet, mainnet (default: testnet)

**Output (stdout):**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "config": {
    "grants": [...],
    "fee_config": {...},
    "params": {...}
  }
}
```

**Examples:**
```bash
# Export to stdout
./scripts/export.sh xion1treasury...

# Export to file
./scripts/export.sh xion1treasury... --output treasury-backup.json

# Export from mainnet
./scripts/export.sh xion1treasury... --network mainnet --output backup.json
```

---

## import.sh

Imports configuration (grants, fee config, params) to an existing Treasury.

**Usage:**
```bash
./scripts/import.sh <ADDRESS> --from-file <FILE> [--dry-run] [--network NETWORK]
```

**Arguments:**
- `ADDRESS` - Treasury contract address (required)

**Options:**
- `--from-file FILE` - Path to JSON configuration file (required)
- `--dry-run` - Preview actions without executing transactions (optional)
- `--network NETWORK` - Network: testnet, mainnet (default: testnet)

**Output (stdout):**
```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "actions_executed": [...],
  "tx_hashes": ["ABC123...", "DEF456..."]
}
```

**Examples:**
```bash
# Preview import (dry run)
./scripts/import.sh xion1treasury... --from-file treasury-backup.json --dry-run

# Execute import
./scripts/import.sh xion1treasury... --from-file treasury-backup.json

# Import to mainnet treasury
./scripts/import.sh xion1treasury... --from-file backup.json --network mainnet
```

**Warning**: Import will execute transactions to configure the Treasury. Always use `--dry-run` first to preview changes.

---

## Chain Queries

> **Note**: For chain-level queries, use xiond from [xion-skills](https://github.com/burnt-labs/xion-skills).

| Query Type | Command |
|------------|---------|
| Transaction status | `xiond query tx <hash>` |
| Block info | `xiond query block` |
| Balance (any address) | `xiond query bank balances <address>` |

See `xiond-usage` skill in xion-skills for more details.
