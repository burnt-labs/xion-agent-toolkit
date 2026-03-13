# Xion Agent Toolkit - Quick Reference

> For AI Agents: Essential commands and patterns in one file (~100 lines).

## Authentication

```bash
xion-toolkit auth login          # OAuth2 login (opens browser)
xion-toolkit auth status         # Check auth status
xion-toolkit auth refresh        # Refresh access token
xion-toolkit auth logout         # Clear credentials
```

### Auth Output Examples

**Success:**
```json
{"success": true, "network": "testnet", "xion_address": "xion1...", "expires_at": "2024-01-01T00:00:00Z"}
```

**Error:**
```json
{"success": false, "error": "Login failed", "code": "AUTH_LOGIN_FAILED", "suggestion": "Please try again"}
```

---

## Treasury Operations

```bash
# List treasuries
xion-toolkit treasury list

# Create treasury
xion-toolkit treasury create \
  --redirect-url "https://..." \
  --icon-url "https://.../icon.png" \
  --name "My Treasury" \
  --is-oauth2-app

# Query treasury details
xion-toolkit treasury query <ADDRESS>

# Fund treasury
xion-toolkit treasury fund <ADDRESS> --amount 1000000uxion

# Withdraw
xion-toolkit treasury withdraw <ADDRESS> --amount 500000uxion

# Update parameters (redirect_url, icon_url, name, is_oauth2_app)
xion-toolkit treasury params update <ADDRESS> \
  --redirect-url "https://new.example.com/callback" \
  --icon-url "https://new.example.com/icon.png" \
  --name "Updated Name" \
  --is-oauth2-app
```

### Treasury Output Examples

**List:**
```json
{"success": true, "treasuries": [{"address": "xion1...", "admin": "xion1...", "balance": "1000000", "name": "My Treasury"}]}
```

**Create:**
```json
{"success": true, "address": "xion1...", "tx_hash": "0x...", "admin": "xion1...", "created_at": "2024-01-01T00:00:00Z"}
```

---

## Grant & Fee Configuration

```bash
# Add grant (authz)
xion-toolkit treasury grant-config add <ADDRESS> \
  --type-url "/cosmos.bank.v1beta1.MsgSend" \
  --auth-type send \
  --spend-limit "1000000uxion" \
  --description "Allow sending funds"

# List grants
xion-toolkit treasury grant-config list <ADDRESS>

# Remove grant
xion-toolkit treasury grant-config remove <ADDRESS> <TYPE_URL>

# Set fee allowance
xion-toolkit treasury fee-config set <ADDRESS> \
  --fee-allowance-type basic \
  --spend-limit "1000000uxion"

# Set periodic fee allowance
xion-toolkit treasury fee-config set <ADDRESS> \
  --fee-allowance-type periodic \
  --period-seconds 86400 \
  --period-spend-limit "100000uxion"

# Query fee config
xion-toolkit treasury fee-config query <ADDRESS>

# Export/Import
xion-toolkit treasury export <ADDRESS> --output backup.json
xion-toolkit treasury import <ADDRESS> --from-file backup.json
```

---

## Contract Operations

```bash
# Instantiate contract
xion-toolkit contract instantiate \
  --code-id 1260 \
  --label "my-contract" \
  --msg msg.json

# Execute contract
xion-toolkit contract execute \
  --contract <ADDRESS> \
  --msg exec.json \
  --funds 1000000uxion

# Query contract (no auth needed)
xion-toolkit contract query \
  --contract <ADDRESS> \
  --msg query.json
```

### Contract Output Examples

**Execute:**
```json
{"success": true, "tx_hash": "0x...", "gas_used": "150000", "gas_wanted": "200000"}
```

**Query:**
```json
{"success": true, "data": {"balance": "1000000", "owner": "xion1..."}}
```

---

## Configuration

```bash
# Set network
xion-toolkit config set-network testnet
xion-toolkit config set-network mainnet

# Show config
xion-toolkit config show
```

### Networks

| Network | RPC | OAuth API | Chain ID | Treasury Code ID |
|---------|-----|-----------|----------|------------------|
| testnet | https://rpc.xion-testnet-2.burnt.com:443 | https://oauth2.testnet.burnt.com | xion-testnet-2 | 1260 |
| mainnet | https://rpc.xion-mainnet-1.burnt.com:443 | https://oauth2.burnt.com | xion-mainnet-1 | 63 |

---

## Output Format

All commands return JSON:

**Success:**
```json
{"success": true, ...data}
```

**Error:**
```json
{"success": false, "error": "...", "code": "...", "suggestion": "..."}
```

---

## Common Error Codes

| Code | Meaning | Fix |
|------|---------|-----|
| NOT_AUTHENTICATED | Not logged in | `auth login` |
| TOKEN_EXPIRED | Token expired | `auth refresh` |
| AUTH_LOGIN_FAILED | Login failed | Retry or check browser |
| AUTH_REFRESH_FAILED | Refresh failed | Re-login required |
| TREASURY_NOT_FOUND | Invalid address | Check address, network |
| INSUFFICIENT_BALANCE | Not enough funds | Fund the account |
| INVALID_INPUT | Invalid input | Check syntax |
| INVALID_AMOUNT | Invalid amount | Use format `amountdenom` |
| NETWORK_ERROR | Connection failed | Check internet |
| TIMEOUT | Request timed out | Retry |

---

## Quick Workflows

### First-Time Setup
```bash
xion-toolkit auth login
xion-toolkit treasury create --redirect-url "https://..." --name "My Treasury"
xion-toolkit treasury fund <ADDRESS> --amount 10000000uxion
xion-toolkit treasury grant-config add <ADDRESS> --type-url "/cosmos.bank.v1beta1.MsgSend" --auth-type send --spend-limit "5000000uxion"
xion-toolkit treasury fee-config set <ADDRESS> --fee-allowance-type basic --spend-limit "1000000uxion"
```

### Daily Use
```bash
xion-toolkit auth status                    # Check auth
xion-toolkit treasury list                  # List treasuries
xion-toolkit treasury query <ADDRESS>       # Check details
xion-toolkit contract execute --contract <ADDRESS> --msg msg.json
```

### Troubleshooting
```bash
xion-toolkit auth status                    # Check if logged in
xion-toolkit auth refresh                   # Refresh if expired
xion-toolkit treasury chain-query grants <ADDRESS>     # Check authz grants
xion-toolkit treasury chain-query allowances <ADDRESS> # Check fee allowances
```
