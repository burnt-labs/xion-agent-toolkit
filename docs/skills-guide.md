# Skills Guide

> **For AI Agents**: Skills wrap `xion-toolkit` CLI for structured JSON output. For command reference, see [QUICK-REFERENCE.md](./QUICK-REFERENCE.md).

## Overview

Skills are bash scripts that provide AI Agents with structured, JSON-output capabilities:

- Output JSON to stdout (machine-readable)
- Progress messages to stderr (non-blocking)
- Consistent error codes with remediation hints
- Follow [Agent Skills](https://agentskills.io/) format

## Installation

```bash
# Install all xion-agent-toolkit skills
npx skills add burnt-labs/xion-agent-toolkit

# Optional: xion-skills for xiond CLI operations
npx skills add burnt-labs/xion-skills
```

## Available Skills

### xion-dev (Entry Point)

Unified entry point for Xion development. Routes to correct skill based on user needs.

**Decision Matrix:**

| User Needs | Skill | Why |
|------------|-------|-----|
| Login / Authentication | `xion-oauth2` | MetaAccount, gasless |
| Create / Manage Treasury | `xion-treasury` | Core functionality |
| Fund / Withdraw | `xion-treasury` | Gasless transactions |
| Authz / Fee Grant | `xion-treasury` | Specialized feature |
| Query chain data | `xiond-usage` | More powerful queries |
| Deploy CosmWasm | `xiond-wasm` | Contract developer tool |

### xion-toolkit-init

Install xion-toolkit CLI when not present.

```bash
bash /path/to/xion-toolkit-init/scripts/install.sh
```

### xion-oauth2

OAuth2 authentication commands.

| Script | Command |
|--------|---------|
| login | `xion-toolkit auth login` |
| status | `xion-toolkit auth status` |
| logout | `xion-toolkit auth logout` |
| refresh | `xion-toolkit auth refresh` |

### xion-treasury

Treasury management commands.

| Script | Command |
|--------|---------|
| list | `xion-toolkit treasury list` |
| query | `xion-toolkit treasury query <ADDR>` |
| create | `xion-toolkit treasury create --name "..." --redirect-url "..."` |
| fund | `xion-toolkit treasury fund <ADDR> --amount 1000000uxion` |
| withdraw | `xion-toolkit treasury withdraw <ADDR> --amount 500000uxion` |
| grant-config | `xion-toolkit treasury grant-config add/remove/list` |
| fee-config | `xion-toolkit treasury fee-config set/remove/query` |
| admin | `xion-toolkit treasury admin propose/accept/cancel` |
| export | `xion-toolkit treasury export <ADDR> --output backup.json` |
| import | `xion-toolkit treasury import <ADDR> --from-file backup.json` |

### xion-asset

NFT operations.

| Script | Command |
|--------|---------|
| types | `xion-toolkit asset types` |
| create | `xion-toolkit asset create --type cw721-base --name "..." --symbol "..."` |
| mint | `xion-toolkit asset mint --contract <ADDR> --token-id "1" --owner <ADDR>` |
| predict | `xion-toolkit asset predict --type cw721-base --name "..." --symbol "..." --salt "..."` |
| batch-mint | `xion-toolkit asset batch-mint --contract <ADDR> --tokens-file tokens.json` |
| query | `xion-toolkit asset query --contract <ADDR> --msg '{"...": {}}'` |

## Output Format

**Success:**
```json
{"success": true, "data": "...", "tx_hash": "..."}
```

**Error:**
```json
{"success": false, "error": "...", "error_code": "...", "hint": "..."}
```

## Common Error Codes

| Code | Fix |
|------|-----|
| `CLI_NOT_FOUND` | Install CLI: `curl ... | sh` |
| `NOT_AUTHENTICATED` | Run `xion-toolkit auth login` |
| `TOKEN_EXPIRED` | Run `xion-toolkit auth refresh` |
| `TREASURY_NOT_FOUND` | Verify address and network |
| `INSUFFICIENT_BALANCE` | Fund the treasury/account |
| `PORT_IN_USE` | Use `--port` flag |

> Full error reference: [ERROR-CODES.md](./ERROR-CODES.md)

## Best Practices

### 1. Check Authentication First
```bash
AUTH_STATUS=$(xion-toolkit auth status --output json)
if [[ $(echo "$AUTH_STATUS" | jq -r '.authenticated') != "true" ]]; then
    xion-toolkit auth login
fi
```

### 2. Parse JSON Output
```bash
RESULT=$(xion-toolkit treasury list --output json)
if [[ $(echo "$RESULT" | jq -r '.success') == "true" ]]; then
    echo "$RESULT" | jq '.treasuries'
fi
```

### 3. Use Network Flag
```bash
xion-toolkit treasury list --network mainnet
```

## xion-skills vs xion-agent-toolkit

| Feature | xion-agent-toolkit | xion-skills |
|---------|-------------------|-------------|
| **Target CLI** | xion-toolkit | xiond |
| **Auth Method** | OAuth2 (gasless) | Mnemonic/Keyring |
| **Use Case** | MetaAccount, Treasury | Chain queries, CosmWasm |

**Recommendation**: Use `xion-agent-toolkit` for most Xion development. Use `xion-skills` for advanced chain operations.

## Resources

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/burnt-labs/xion-agent-toolkit |
| Agent Skills | https://agentskills.io/ |
| Xion Docs | https://docs.burnt.com/xion |
| Developer Portal | https://dev.testnet2.burnt.com |

---

*Document Version: 2.0.0*
*Last Updated: 2026-03-14*
