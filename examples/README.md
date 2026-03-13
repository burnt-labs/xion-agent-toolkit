# Xion Agent Toolkit - Example Files

This directory contains practical, well-documented JSON example files for the `xion-toolkit` CLI commands.

## Quick Reference

| File | Command | Description |
|------|---------|-------------|
| `treasury-create-config.json` | `treasury create --config` | Full treasury creation config |
| `treasury-export-sample.json` | `treasury export` | Example export output |
| `grant-config-send.json` | `treasury grant-config add --grant-config` | Send authorization grant |
| `grant-config-contract-exec.json` | `treasury grant-config add --grant-config` | Contract execution grant |
| `fee-config-basic.json` | `treasury fee-config set --fee-config` | Basic fee allowance |
| `fee-config-periodic.json` | `treasury fee-config set --fee-config` | Periodic fee allowance |
| `contract-instantiate-msg.json` | `contract instantiate --msg` | Contract instantiation message |
| `contract-query-msg.json` | `contract query --msg` | Contract query message |
| `contract-execute-msg.json` | `contract execute --msg` | Contract execute message |
| `treasury-params-update.json` | `treasury params update --metadata` | Treasury metadata update |

---

## Treasury Creation

### `treasury-create-config.json`

Complete configuration for creating a new Treasury contract.

**Command:**
```bash
xion-toolkit treasury create --config examples/treasury-create-config.json
```

**Schema:**
- `params` - Treasury parameters (required)
  - `redirect_url` - OAuth callback URL
  - `icon_url` - Treasury icon URL
  - `name` - Display name (optional)
  - `is_oauth2_app` - Mark as OAuth2 application (optional)
- `fee_config` - Fee grant configuration (optional)
  - `allowance_type` - Type: `basic`, `periodic`, or `allowed_msg`
  - `spend_limit` - Maximum spend limit (e.g., "1000000uxion")
  - `description` - Human-readable description
- `grant_configs` - Array of authorization grants (at least one required)
  - `type_url` - Message type URL (e.g., "/cosmos.bank.v1beta1.MsgSend")
  - `description` - Grant description
  - `authorization` - Authorization config (see grant config examples)
  - `optional` - Whether the grant is optional

---

## Treasury Export

### `treasury-export-sample.json`

Example output from the `treasury export` command. Use this for backup, migration, or configuration sharing.

**Command:**
```bash
xion-toolkit treasury export xion1abc123... --output treasury-backup.json
```

**Output fields:**
- `success` - Operation success status
- `treasury_address` - Treasury contract address
- `export` - Exported configuration
  - `address` - Treasury address
  - `admin` - Current admin address
  - `params` - Treasury parameters
  - `fee_config` - Fee configuration (if any)
  - `grant_configs` - Grant configurations array
  - `exported_at` - Export timestamp (ISO 8601)

---

## Treasury Parameters

### `treasury-params-update.json`

Update treasury parameters including `redirect_url`, `icon_url`, `name`, and `is_oauth2_app`:

```bash
# Update individual parameters
xion-toolkit treasury params update xion1treasury... \
  --redirect-url "https://newapp.com/callback" \
  --icon-url "https://newapp.com/icon.png"

# Update name
xion-toolkit treasury params update xion1treasury... \
  --name "My Updated Treasury"

# Mark as OAuth2 application
xion-toolkit treasury params update xion1treasury... \
  --is-oauth2-app

# Update with additional metadata
xion-toolkit treasury params update xion1treasury... \
  --metadata @examples/treasury-params-update.json
```

**Available flags:**
- `--redirect-url <URL>` - OAuth callback URL
- `--icon-url <URL>` - Treasury icon URL
- `--name <NAME>` - Display name (stored in metadata)
- `--is-oauth2-app` - Mark as OAuth2 application (stored in metadata.is_oauth2_app)
- `--metadata <JSON>` - Additional metadata as JSON string

**Important**: `is_oauth2_app` and `name` are stored in the `metadata` JSON field on-chain, but exposed as first-class CLI parameters for convenience.

**Metadata schema** (for `--metadata` flag):
- `name` - Display name for the treasury
- `description` - Human-readable description
- `archived` - Whether the treasury is archived
- Custom fields - Any additional JSON data

---

## Grant Configuration

### `grant-config-send.json`

Authorization for `MsgSend` - allows the treasury to send funds on your behalf.

**Command:**
```bash
xion-toolkit treasury grant-config add xion1treasury... --grant-config examples/grant-config-send.json
```

**Authorization schema:**
- `auth_type` - Must be `send` for MsgSend
- `spend_limit` - Maximum amount that can be sent (e.g., "1000000uxion")
- `allow_list` - Optional array of allowed recipient addresses (empty = all addresses)

### `grant-config-contract-exec.json`

Authorization for `MsgExecuteContract` - allows the treasury to execute smart contracts.

**Command:**
```bash
xion-toolkit treasury grant-config add xion1treasury... --grant-config examples/grant-config-contract-exec.json
```

**Authorization schema:**
- `auth_type` - Must be `contract_execution`
- `grants` - Array of per-contract grants
  - `address` - Contract address
  - `max_calls` - Maximum number of calls allowed
  - `max_funds` - Maximum funds that can be sent (e.g., "1000000uxion")
  - `filter_type` - `allow_all` or `accepted_keys`
  - `keys` - Array of accepted message keys (only for `accepted_keys` filter)

---

## Fee Configuration

### `fee-config-basic.json`

Basic fee allowance - a one-time spend limit for gasless transactions.

**Command:**
```bash
xion-toolkit treasury fee-config set xion1treasury... --fee-config examples/fee-config-basic.json
```

**Schema:**
- `allowance_type` - Must be `basic`
- `spend_limit` - Total amount available for fees (e.g., "10000000uxion")
- `description` - Human-readable description

### `fee-config-periodic.json`

Periodic fee allowance - spend limit that resets after each period.

**Command:**
```bash
xion-toolkit treasury fee-config set xion1treasury... --fee-config examples/fee-config-periodic.json
```

**Schema:**
- `allowance_type` - Must be `periodic`
- `basic_spend_limit` - Total lifetime spend limit (optional)
- `period_seconds` - Period duration in seconds (86400 = 1 day)
- `period_spend_limit` - Maximum spend per period (e.g., "1000000uxion")
- `description` - Human-readable description

---

## Contract Operations

### `contract-instantiate-msg.json`

Instantiate message for deploying a new smart contract. The schema depends on the specific contract being deployed.

**Command:**
```bash
xion-toolkit contract instantiate \
  --code-id 1260 \
  --label "my-contract" \
  --msg examples/contract-instantiate-msg.json
```

### `contract-query-msg.json`

Query message for reading contract state. No authentication required.

**Command:**
```bash
xion-toolkit contract query \
  --contract xion1contract... \
  --msg examples/contract-query-msg.json
```

### `contract-execute-msg.json`

Execute message for mutating contract state. Requires authentication.

**Command:**
```bash
xion-toolkit contract execute \
  --contract xion1contract... \
  --msg examples/contract-execute-msg.json
```

---

## Common Patterns

### Complete Treasury Lifecycle

```bash
# 1. Create treasury with config
xion-toolkit treasury create --config examples/treasury-create-config.json

# 2. Fund the treasury
xion-toolkit treasury fund xion1treasury... --amount 10000000uxion

# 3. Add additional grant
xion-toolkit treasury grant-config add xion1treasury... --grant-config examples/grant-config-contract-exec.json

# 4. Set fee config
xion-toolkit treasury fee-config set xion1treasury... --fee-config examples/fee-config-basic.json

# 5. Export for backup
xion-toolkit treasury export xion1treasury... --output backup.json
```

### Contract Deployment

```bash
# 1. Instantiate contract
xion-toolkit contract instantiate \
  --code-id 1260 \
  --label "my-contract-001" \
  --msg examples/contract-instantiate-msg.json

# 2. Query contract state
xion-toolkit contract query \
  --contract xion1contract... \
  --msg examples/contract-query-msg.json

# 3. Execute contract message
xion-toolkit contract execute \
  --contract xion1contract... \
  --msg examples/contract-execute-msg.json \
  --funds "1000000uxion"
```

---

## Notes

- All amounts use the `uxion` denomination (micro-XION)
- 1 XION = 1,000,000 uxion
- Addresses use the `xion1...` prefix (bech32 format)
- Placeholder addresses in examples (e.g., `xion1abc...`) should be replaced with real addresses
- JSON files must be valid JSON (no comments, trailing commas, etc.)

For complete CLI documentation, see [docs/cli-reference.md](../docs/cli-reference.md).