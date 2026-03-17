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
├── src/
│   └── {cli,oauth,api,treasury,config,utils,asset_builder}
├── skills/
├── tests/                 # all test scripts and integration tests
│   ├── e2e_treasury_lifecycle.sh      # full lifecycle E2E test
│   ├── e2e_treasury_grant_fee.sh      # grant/fee config E2E test
│   └── archived/                      # archived/legacy test scripts
├── scripts/               # build and utility scripts (not tests)
├── plans/
│   ├── status.json
│   ├── *.md               # actual plans
│   └── knowledge/         # reasoning, comparisons, dev logs
└── logs/                  # dev-time logs (git-ignored)
```

### Directory Purposes

- **tests/**: All test scripts (bash E2E tests, Rust integration tests). E2E test scripts should be named `e2e_*.sh`.
- **scripts/**: Build scripts, deployment scripts, and utility scripts. NOT for test scripts.
- **plans/**: Project planning documents and knowledge base.

Development progress and roadmap live in `plans/`; use `plans/status.json` as the SSOT.

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

### Skills Parameter Validation Framework

When creating or modifying skills, each CLI command **must** have a corresponding parameter schema. This enables AI Agents to validate and collect parameters before command execution.

#### Schema File Structure

Each skill stores schemas in `skills/<skill-name>/schemas/<command>.json`:

```text
skills/xion-treasury/schemas/
├── grant-config-add.json
├── grant-config-remove.json
├── fee-config-set.json
├── create.json
└── ...
```

#### Schema Format

```json
{
  "command": "grant-config add",
  "description": "Add an authz grant to a Treasury",
  "parameters": [
    {
      "name": "address",
      "type": "string",
      "required": true,
      "format": "xion-address",
      "description": "Treasury contract address"
    },
    {
      "name": "preset",
      "type": "enum",
      "required": false,
      "enum": ["send", "execute", "delegate"],
      "description": "Shortcut for common grant types",
      "conflicts_with": ["type-url", "auth-type"]
    },
    {
      "name": "spend-limit",
      "type": "string",
      "required": false,
      "format": "coin",
      "description": "Spend limit (e.g., 1000000uxion)",
      "depends_on": {
        "parameter": "auth-type",
        "values": ["send"]
      }
    }
  ],
  "presets": {
    "send": {
      "type-url": "/cosmos.bank.v1beta1.MsgSend",
      "auth-type": "send",
      "requires": ["spend-limit"]
    }
  },
  "examples": [
    {
      "description": "Add send authorization",
      "params": {
        "address": "xion1abc...",
        "preset": "send",
        "spend-limit": "1000000uxion"
      }
    }
  ]
}
```

#### Parameter Properties

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `name` | string | ✅ | Parameter name (kebab-case) |
| `type` | enum | ✅ | `string`, `integer`, `number`, `boolean`, `enum`, `file` |
| `required` | boolean | ✅ | Whether parameter is required |
| `description` | string | ✅ | Human-readable description |
| `format` | string | ❌ | Format hint: `xion-address`, `coin`, `url`, `iso8601-datetime` |
| `enum` | array | ❌ | Allowed values (for type=enum) |
| `default` | any | ❌ | Default value |
| `depends_on` | object | ❌ | Conditional requirement |
| `conflicts_with` | array | ❌ | Mutually exclusive parameters |
| `notes` | array | ❌ | Helpful notes for AI agents (e.g., unit conversions, important caveats) |
| `validation_rules` | array | ❌ | Array of strings documenting complex validation logic that cannot be expressed in schema |

#### Dependency Rules

**`depends_on`** - Parameter required when another parameter has specific values:

```json
{
  "name": "spend-limit",
  "depends_on": {
    "parameter": "auth-type",
    "values": ["send"]
  }
}
```

For "any value triggers dependency", omit `values`:

```json
{
  "name": "fee-spend-limit",
  "depends_on": {
    "parameter": "fee-allowance-type"
  }
}
```

**`conflicts_with`** - Parameters that cannot be used together:

```json
{
  "name": "preset",
  "conflicts_with": ["type-url", "auth-type"]
}
```

#### Validation Script

Use `skills/scripts/validate-params.sh` for pre-flight validation:

```bash
./skills/scripts/validate-params.sh <skill> <command> '<json-params>'

# Example
./skills/scripts/validate-params.sh xion-treasury grant-config-add '{"address": "xion1test"}'

# Output (missing params)
{"valid": false, "missing": ["description"], "errors": [...]}

# Output (valid)
{"valid": true, "missing": [], "errors": []}
```

Exit codes: `0` = valid, `1` = invalid, `2` = usage error.

#### Adding a New Command

1. Create schema file: `skills/<skill>/schemas/<command>.json`
2. Define all parameters with required dependencies
3. Add presets for common use cases
4. Include examples
5. Update SKILL.md with parameter table reference

#### Parameter Collection Workflow

Document in SKILL.md for AI Agents:

```markdown
## Parameter Collection Workflow

Before executing any command, ensure all required parameters are collected.

### Step 1: Identify Operation
Determine which operation the user wants to perform.

### Step 2: Check Parameter Schema
Refer to the `schemas/` directory for detailed parameter definitions.

### Step 3: Collect Missing Parameters
Collect ALL missing required parameters in a SINGLE interaction.

### Step 4: Confirm Before Execution
```
Will execute: grant-config add
├─ Address: xion1abc...
├─ Type: send
└─ Spend Limit: 1000000uxion
Confirm? [y/n]
```
```

#### Known Limitations

The validation framework validates **structure** (required params, dependencies, conflicts) but has limitations on **content** validation:

- **Enum validation**: Validates that provided values match enum constraints defined in schema
- **Format validation**: Informational only (`xion-address`, `coin`, `url`, `iso8601-datetime` formats are hints)
- **Type validation**: String vs number distinction is minimal; relies on schema definitions
- **Numeric ranges**: `min`/`max` constraints are informational hints
- **File existence**: Not validated (files are checked at CLI execution time)
- **Mutual requirement**: Cannot enforce "at least one of X or Y required" patterns (documented in `validation_rules`)

For full validation, rely on the CLI's own validation after parameter collection.

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

### Protected Treasury Contracts

For all development, testing, and automation, treat Treasury `xion17vg5l9za4768g0hnxezltgnu4h7eleqdcmwark2uuz2s4z5q4dfsr80vvm` as **write-protected**:
- Do **not** fund, withdraw from, modify grants/fees for, or otherwise mutate this Treasury via CLI, scripts, or skills.
- Tests and e2e scripts **must not** touch this Treasury address under any circumstance.

### Grant & Fee Configuration Patterns

When implementing grant/fee configuration in code:

```bash
# Grant configuration patterns
xion-toolkit treasury grant-config <address> --grant-type-url "/cosmos.bank.v1beta1.MsgSend" --grant-auth-type send --grant-spend-limit "1000000uxion"

# Fee configuration patterns
xion-toolkit treasury fee-config <address> --fee-allowance-type periodic --fee-period-seconds 86400 --fee-period-spend-limit "100000uxion"
```

> For complete command reference, see [docs/cli-reference.md](./docs/cli-reference.md).

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

Current status: **232 tests passing**

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
- `docs/QUICK-REFERENCE.md`: condensed CLI reference for AI Agents (~200 lines)
- `docs/ERROR-CODES.md`: complete error code reference for error handling
- `docs/cli-reference.md`: detailed CLI command documentation
- `examples/`: example JSON config files for CLI commands

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
