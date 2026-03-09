# Xion Agent Toolkit - Development Guidelines

## Project Overview

Xion Agent Toolkit is a CLI-first, agent-oriented toolkit for Xion. It uses Xion MetaAccount + OAuth2 APIs to provide a gasless development experience.

## Core Principles

### 1. Agent-First Design

- Optimize all features for agent invocation.
- Output JSON for machine parsing.
- Return structured errors with codes and remediation hints.

### 2. MetaAccount-Centric

- Use OAuth2, not mnemonic-based auth.
- Sign with Session Keys.
- Support Fee Grant and Authz Grant.

### 3. Modular Architecture

- CLI is the core.
- Skills extend agent capabilities.
- Configuration remains independent and transparent.

## Technology Stack

### Main Tool: Rust

- `clap`, `reqwest`, `tokio`, `serde`, `serde_json`, `directories`, `thiserror`, `anyhow`, `tracing`, `tracing-subscriber`, `keyring`

### Skills: Bash + Node.js

- Follow [Agent Skills](https://agentskills.io/).
- Write JSON to stdout and status messages to stderr.
- Use `set -e`.

## Project Structure

```text
xion-agent-toolkit/
├── AGENTS.md
├── README.md / CONTRIBUTING.md
├── Cargo.toml
├── src/{cli,oauth,api,treasury,config,utils}
├── skills/
├── tests/                 # integration tests and local debug helpers
├── plans/
│   ├── status.json
│   ├── *.md               # actual plans
│   └── knowledge/         # reasoning, comparisons, dev logs
└── logs/                  # dev-time logs (git-ignored)
```

Development progress and roadmap live in `plans/`; use `plans/status.json` as the SSOT.

## Repository Hygiene

- Put ad-hoc test scripts and debug artifacts in `tests/`, not repo root.
- Put non-plan “process / reasoning” documents in `plans/knowledge/`.
- Put development-time logs in `logs/`; keep only `logs/.gitkeep` tracked.

## Code Standards

### Rust Code Standards

- Use `thiserror` for error definitions.
- Use `serde` / `serde_json` for serialization.
- CLI success output goes to stdout as JSON.
- Human-readable errors go to stderr.

### CLI Command Design Principles

- All commands should support JSON output, e.g. `xion-toolkit auth login --output json`.
- Errors should include codes, messages, and actionable suggestions.
- Support both config-driven and flag-driven usage, e.g. `--network` and `--config`.

### Skills Script Standards

- Use `#!/bin/bash` + `set -e`.
- Write machine-readable JSON to stdout.
- Write status / progress logs to stderr.
- On failure, return structured JSON such as `{"success": false, "error": "...", "code": "..."}`.

## Configuration Management

### Configuration File Location

Use a unified directory on all platforms:

```text
~/.xion-toolkit/
├── config.json
├── oauth_endpoints.json
└── credentials/
    ├── testnet.enc
    └── mainnet.enc
```

Credentials are encrypted with AES-256-GCM. Do not delete `~/.xion-toolkit/credentials/*.enc` during testing; they contain long-lived refresh tokens.

### ⚠️ IMPORTANT: Do Not Delete Credentials

- NEVER delete `~/.xion-toolkit/credentials/*.enc` unless explicitly requested.
- NEVER run `auth logout` unless explicitly requested.
- Refresh tokens last 30 days; deleting them forces browser re-login.
- Expired access tokens should refresh automatically.

### Configuration Schema

#### User Config (`~/.xion-toolkit/config.json`)

```json
{"version":"1.0","network":"testnet"}
```

#### Credentials Metadata (`~/.xion-toolkit/credentials/{network}.enc`)

```json
{"access_token":"xion1...:grantId:secret","refresh_token":"xion1...:grantId:refreshSecret","expires_at":"2024-01-01T00:00:00Z","refresh_token_expires_at":"2024-02-01T00:00:00Z","xion_address":"xion1..."}
```

Encryption key source:

1. `XION_CI_ENCRYPTION_KEY` in CI/CD
2. Machine ID via `machine-uid` locally

## Network Configuration

Compiled-in network settings:

| Network | OAuth API | RPC | Chain ID | Treasury Code ID |
| ------- | --------- | --- | -------- | ---------------- |
| testnet | <https://oauth2.testnet.burnt.com> | <https://rpc.xion-testnet-2.burnt.com:443> | `xion-testnet-2` | `1260` |
| mainnet | <https://oauth2.burnt.com> | <https://rpc.xion-mainnet-1.burnt.com:443> | `xion-mainnet-1` | `63` |

## OAuth2 API Service Message Formats

### Supported Transaction Message Formats

`/api/v1/transaction` accepts three `value` formats:

| Format | `value` shape | `msg` encoding |
| ------ | ------------- | -------------- |
| Byte array protobuf | `[1,2,3,...]` | base64-encoded JSON string |
| Base64 protobuf | `"base64:..."` | base64-encoded JSON string |
| Raw JSON object | `{ sender, contract, msg, funds }` | raw JSON object |

### Important: msg Field Encoding Rules

For `MsgExecuteContract` and `MsgInstantiateContract`:

- Protobuf formats: `msg` must be a base64-encoded JSON string.
- Raw JSON format: `msg` must be a raw JSON object, never base64.

**CRITICAL**: When using raw JSON object format, `msg` must remain raw JSON.

### Reference Implementation

- OAuth2 API Service:
  - `~/workspace/xion/oauth2-api-service/src/routes/api/transaction/broadcast.ts`
  - `~/workspace/xion/oauth2-api-service/src/utils/transactions.ts`
- xion-types:
  - `~/workspace/xion/xion-types/ts/types/cosmwasm/wasm/v1/tx.ts`

### Our Implementation Choice

The CLI uses raw JSON object format for simplicity and parity with Developer Portal patterns:

- `MsgExecuteContract.msg` = raw JSON object
- `MsgInstantiateContract2.msg` = raw JSON object

## OAuth2 Authentication Flow

### Pre-configured OAuth Clients

Each network uses a pre-configured OAuth client with the permissions needed for Treasury management, matching the Developer Portal capability set.

### Login Flow

1. Generate PKCE verifier and challenge.
2. Start localhost callback server.
3. Open or display authorization URL.
4. Exchange callback code for access and refresh tokens.
5. Store tokens and return JSON output.

### Callback Server

- Default port: `54321` (configurable)
- Callback path: `/callback`
- Timeout: 5 minutes
- Bind only to localhost

## Treasury Commands

### Available Commands

```bash
xion-toolkit treasury list
xion-toolkit treasury query <address>
xion-toolkit treasury fund <address> --amount <amount>
xion-toolkit treasury withdraw <address> --amount <amount> --to <recipient>
xion-toolkit treasury grant-config <address> [options]
xion-toolkit treasury fee-config <address> [options]
```

### Grant Configuration Options

Typical patterns:

```bash
xion-toolkit treasury grant-config <address> --grant-type-url "/cosmos.bank.v1beta1.MsgSend" --grant-auth-type generic --grant-description "Generic permission"
xion-toolkit treasury grant-config <address> --grant-type-url "/cosmos.bank.v1beta1.MsgSend" --grant-auth-type send --grant-spend-limit "1000000uxion" --grant-description "Allow sending funds"
```

### Fee Configuration Options

Typical patterns:

```bash
xion-toolkit treasury fee-config <address> --fee-allowance-type basic --fee-spend-limit "1000000uxion" --fee-description "Basic fee allowance"
xion-toolkit treasury fee-config <address> --fee-allowance-type periodic --fee-period-seconds 86400 --fee-period-spend-limit "100000uxion" --fee-description "Daily fee allowance"
```

### Protected Treasury Contracts

- For all development, testing, and automation, treat Treasury `xion17vg5l9za4768g0hnxezltgnu4h7eleqdcmwark2uuz2s4z5q4dfsr80vvm` as **write-protected**:
  - Do **not** fund, withdraw from, modify grants/fees for, or otherwise mutate this Treasury via CLI, scripts, or skills.
  - Tests and e2e scripts **must not** touch this Treasury address under any circumstance.

## Git Standards

### Commit Messages

Use conventional, scoped messages such as:

```text
feat(cli): add OAuth2 login command
fix(treasury): handle insufficient balance error
docs(skill): update xion-treasury skill documentation
chore(config): migrate to new config schema
```

### Branch Strategy

- `main`: stable release
- `develop`: development version
- `feature/*`: new features
- `fix/*`: bug fixes

## Testing Standards

### Unit Tests

- Use standard Rust `#[test]`.
- Keep units focused and deterministic.

### Integration Tests

- Use `#[tokio::test]` for async integration tests when needed.

### Running Tests

```bash
cargo test
cargo test test_pkce_challenge
cargo test -- --nocapture
```

Current status: **330 tests passing**

### Test Serialization Rules

Tests that modify environment variables, especially `XION_CI_ENCRYPTION_KEY`, MUST use `#[serial(encryption_key)]`.

Rules:

- `#[serial(encryption_key)]` is the correct shared group.
- Bare `#[serial]` is a different group and is not sufficient.
- Any test in `src/config/encryption.rs` or `src/config/credentials.rs` touching `XION_CI_ENCRYPTION_KEY` must use `#[serial(encryption_key)]`.

### Pre-commit Checklist

Run before every commit:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

CI must pass:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- successful compilation

## Security Standards

1. Token storage
   - Store credentials as AES-256-GCM encrypted `~/.xion-toolkit/credentials/*.enc`.
   - Derive the key from machine ID, or `XION_CI_ENCRYPTION_KEY` in CI.
   - Never store tokens in plain text.
   - Never delete `.enc` files during testing unless explicitly requested.
2. PKCE
   - Use a cryptographically secure RNG.
   - Verifier length must be at least 43 chars.
   - Use SHA-256 for challenge generation.
3. API communication
   - Enforce HTTPS, validate certificates, and apply timeouts.
4. Callback server
   - Bind only to localhost, validate state, use a timeout, and choose a random port if needed.
5. Testing with credentials
   - Never run `auth logout` automatically.
   - Never delete `~/.xion-toolkit/credentials/*.enc`.
   - Let access tokens auto-refresh.
   - Clear credentials only when explicitly requested.

## Documentation Standards

- `README.md`: overview and quick start
- `docs/`: detailed docs such as `cli-reference.md`, `oauth-flow.md`, `treasury-guide.md`
- `examples/`: example code and scripts

## Language Standards

1. Conversation language
   - Match the user's language.
2. Documentation and code language
   - Persistent docs (`README.md`, `docs/`, `plans/`, skill `SKILL.md`, `AGENTS.md`) must be in English.
   - All code comments must be in English.

## Related Resources

### Official Documentation

- [Xion Documentation](https://docs.burnt.com/xion)
- [OAuth2 API Service](https://github.com/burnt-labs/xion/tree/main/oauth2-api-service)
- [Agent Skills Format](https://agentskills.io/)

### Key Reference Implementations

#### 1. Xion-Types (On-chain Message Types)

- Repo: <https://github.com/burnt-labs/xion-types>
- Local path: `~/workspace/xion/xion-types`
- Use for protobuf message structures and encoding examples.

#### 2. OAuth2 App Demo

- Repo: <https://github.com/burnt-labs/xion-oauth2-app-demo>
- Local path: `~/workspace/xion/xion-oauth2-app-demo`
- Use for OAuth2 login, token management, and transaction signing patterns.

#### 3. Developer Portal

- Local path: `~/workspace/xion/xion-developer-portal`
- Live: <https://dev.testnet2.burnt.com>
- This is the primary reference for CLI API behavior.
- CLI APIs, message formats, encoding patterns, and field naming must match the Developer Portal.
- Key areas: `src/components/Treasury/`, `src/lib/`

#### 4. CosmJS (Communication Protocol)

- Repo: <https://github.com/cosmos/cosmjs>
- Use for transaction construction, protobuf encoding, signing, and broadcasting.

### Additional Resources

- [Xion Skills](https://github.com/burnt-labs/xion-skills)

### API Compatibility Note

Treat the CLI as a command-line equivalent of the Developer Portal:

1. Match message formats exactly.
2. Use the same endpoints and parameters.
3. Match encoding patterns exactly.
4. Follow the same field naming conventions.

## OAuth2 API vs Query API

### Important: OAuth2 API is Transaction-Only

The OAuth2 API (`/api/v1/transaction`) is designed for **transaction broadcasting only**, not for querying.

- **OAuth2 API**: Use only for `/api/v1/transaction` endpoint (signing and broadcasting transactions)
- **Query operations**: Use DaoDao Indexer or direct chain queries (RPC)

### Query Data Sources

| Data | Source |
| ---- | ------ |
| Treasury list | DaoDao Indexer (`{indexer_url}/contract/{user_address}/xion/account/treasuries`) |
| Treasury info | DaoDao Indexer (basic) + on-chain query (admin, grants, fee config) |
| Transaction status | RPC (`{rpc_url}/tx?hash=0x...`) |
| Authz grants | Chain query via RPC |

### Why This Matters

1. OAuth2 API is optimized for transaction signing with session keys
2. Query endpoints on OAuth2 API may not exist or return incomplete data
3. Indexer provides faster, aggregated views of treasury data
4. Direct chain queries ensure data accuracy for critical fields

<!-- gitnexus:start -->
## GitNexus — Code Intelligence

This project is indexed by GitNexus as **xion-agent-toolkit** (598 symbols, 1498 relationships, 49 execution flows). Use GitNexus MCP tools to understand code, assess impact, and navigate safely.

> If any GitNexus tool warns the index is stale, run `npx gitnexus analyze` in terminal first.

## Always Do

- Before editing any function, class, or method, run `gitnexus_impact({target: "symbolName", direction: "upstream"})` and report direct callers, affected processes, and risk level.
- Before committing, run `gitnexus_detect_changes()` to confirm the change scope.
- Warn the user before proceeding on HIGH or CRITICAL impact.
- For unfamiliar areas, prefer `gitnexus_query({query: "concept"})` over grep.
- For full symbol context, use `gitnexus_context({name: "symbolName"})`.

## When Debugging

1. `gitnexus_query({query: "<error or symptom>"})`
2. `gitnexus_context({name: "<suspect function>"})`
3. Read `gitnexus://repo/xion-agent-toolkit/process/{processName}`
4. For regressions: `gitnexus_detect_changes({scope: "compare", base_ref: "main"})`

## When Refactoring

- Renaming: use `gitnexus_rename({symbol_name: "old", new_name: "new", dry_run: true})` first; review text-search edits, then run with `dry_run: false`.
- Extracting / splitting: run `gitnexus_context({name: "target"})` and `gitnexus_impact({target: "target", direction: "upstream"})` first.
- After refactors, run `gitnexus_detect_changes({scope: "all"})`.

## Never Do

- Never edit a function, class, or method without `gitnexus_impact`.
- Never ignore HIGH or CRITICAL impact warnings.
- Never rename symbols with find-and-replace; use `gitnexus_rename`.
- Never commit without `gitnexus_detect_changes()`.

## Tools Quick Reference

| Tool | When to use | Command |
| ---- | ----------- | ------- |
| `query` | Find code by concept | `gitnexus_query({query: "auth validation"})` |
| `context` | 360-degree view of one symbol | `gitnexus_context({name: "validateUser"})` |
| `impact` | Blast radius before editing | `gitnexus_impact({target: "X", direction: "upstream"})` |
| `detect_changes` | Pre-commit scope check | `gitnexus_detect_changes({scope: "staged"})` |
| `rename` | Safe multi-file rename | `gitnexus_rename({symbol_name: "old", new_name: "new", dry_run: true})` |
| `cypher` | Custom graph queries | `gitnexus_cypher({query: "MATCH ..."})` |

## Impact Risk Levels

| Depth | Meaning | Action |
| ----- | ------- | ------ |
| d=1 | WILL BREAK — direct callers/importers | MUST update these |
| d=2 | LIKELY AFFECTED — indirect deps | Should test |
| d=3 | MAY NEED TESTING — transitive | Test if critical path |

## Resources

| Resource | Use for |
| -------- | ------- |
| `gitnexus://repo/xion-agent-toolkit/context` | Codebase overview and freshness |
| `gitnexus://repo/xion-agent-toolkit/clusters` | All functional areas |
| `gitnexus://repo/xion-agent-toolkit/processes` | All execution flows |
| `gitnexus://repo/xion-agent-toolkit/process/{name}` | Step-by-step execution trace |

## Self-Check Before Finishing

Before completing code changes, verify:

1. `gitnexus_impact` ran for all modified symbols
2. No HIGH / CRITICAL warnings were ignored
3. `gitnexus_detect_changes()` matches the expected scope
4. All d=1 dependents were updated

## CLI

- Re-index: `npx gitnexus analyze`
- Check freshness: `npx gitnexus status`
- Generate docs: `npx gitnexus wiki`

<!-- gitnexus:end -->
