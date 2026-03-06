# Xion Agent Toolkit - Development Guidelines

## Project Overview

Xion Agent Toolkit is a CLI-driven, Agent-oriented toolkit designed to facilitate development on the Xion blockchain. Built on Xion's MetaAccount system, it leverages OAuth2 API to provide a gasless development experience.

## Core Principles

### 1. Agent-First Design
- All features prioritize Agent invocation scenarios
- CLI output formatted as JSON for easy Agent parsing
- Structured error messages with error codes and remediation suggestions

### 2. MetaAccount-Centric
- Based on OAuth2 authentication flow, not traditional mnemonic
- Uses Session Key for transaction signing
- Supports Fee Grant and Authz Grant

### 3. Modular Architecture
- CLI tool as the core component
- Skills as Agent extensions
- Independent and transparent configuration management

## Technology Stack

### Main Tool: Rust
- **CLI Framework**: clap (v4.x) - Powerful command-line argument parsing
- **HTTP Client**: reqwest + tokio - Asynchronous HTTP requests
- **Serialization**: serde + serde_json - JSON processing
- **Configuration Management**: directories - Cross-platform configuration directories
- **Error Handling**: thiserror + anyhow - Structured errors
- **Logging**: tracing + tracing-subscriber - Structured logging
- **Credential Storage**: keyring - OS-native credential storage

### Skills: Bash + Node.js
- Follows [Agent Skills](https://agentskills.io/) format
- Outputs JSON to stdout, status messages to stderr
- Scripts use `set -e` for fail-fast behavior

## Project Structure

```
xion-agent-toolkit/
├── AGENTS.md                    # This file - Development Guidelines
├── Cargo.toml                   # Rust project configuration
├── src/
│   ├── main.rs                  # CLI entry point
│   ├── cli/                     # CLI command definitions
│   │   ├── mod.rs
│   │   ├── auth.rs              # OAuth2 authentication commands
│   │   ├── treasury.rs          # Treasury management commands
│   │   └── config.rs            # Configuration management commands
│   ├── oauth/                   # OAuth2 client implementation
│   │   ├── mod.rs
│   │   ├── client.rs            # OAuth2 client
│   │   ├── token.rs             # Token management
│   │   └── pkce.rs              # PKCE implementation
│   ├── api/                     # API clients
│   │   ├── mod.rs
│   │   ├── oauth2_api.rs        # OAuth2 API Service client
│   │   ├── treasury.rs          # Treasury API
│   │   └── xiond.rs             # xiond query client
│   ├── config/                  # Configuration management
│   │   ├── mod.rs
│   │   ├── manager.rs           # Configuration manager
│   │   └── schema.rs            # Configuration Schema
│   └── utils/                   # Utility functions
│       ├── mod.rs
│       ├── output.rs            # Output formatting
│       └── error.rs             # Error definitions
├── skills/                      # Agent Skills
│   ├── xion-oauth2/             # OAuth2 setup
│   │   ├── SKILL.md
│   │   └── scripts/
│   ├── xion-treasury/           # Treasury management
│   │   ├── SKILL.md
│   │   └── scripts/
│   └── xion-deploy/             # Smart contract deployment (future)
│       ├── SKILL.md
│       └── scripts/
└── plans/                       # Development plan documents
    └── treasury-automation.md
```

## Development Phases

### Phase 1: Foundation (Days 1-7)
1. CLI framework setup
2. Configuration management system
3. OAuth2 client basic functionality

### Phase 2: Core Features (Days 8-21)
1. OAuth2 authentication flow (PKCE)
2. Token management and auto-refresh
3. Treasury query and creation
4. Grant configuration (Fee + Authz)

### Phase 3: Skills Development (Days 22-28)
1. xion-oauth2 skill
2. xion-treasury skill
3. Documentation and examples

## Code Standards

### Rust Code Standards

```rust
// 1. Use thiserror for error definitions
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    #[error("Failed to exchange code: {0}")]
    CodeExchange(String),
    
    #[error("Token expired")]
    TokenExpired,
}

// 2. Use serde for serialization
#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryInfo {
    pub address: String,
    pub admin: String,
    pub balance: String,
}

// 3. CLI output must be JSON
pub fn output_json<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}

// 4. Errors to stderr
pub fn output_error(error: &Error) {
    eprintln!("{}", error);
    process::exit(1);
}
```

### CLI Command Design Principles

```bash
# 1. All commands support JSON output
xion auth login --output json
xion treasury list --output json

# 2. Errors include error codes
xion treasury create --fee 1000
# Error: INSUFFICIENT_BALANCE
# Message: Treasury requires at least 1000000 uxion
# Suggestion: Fund your account with 'xion treasury fund'

# 3. Support config file and command-line arguments
xion --network testnet treasury list
xion --config ~/.xion-toolkit/config.json treasury list
```

### Skills Script Standards

```bash
#!/bin/bash
set -e  # Fail fast

# 1. Output JSON to stdout
output_json() {
    echo "$1"
}

# 2. Status messages to stderr
log_info() {
    echo "[INFO] $1" >&2
}

# 3. Error handling
handle_error() {
    output_json "{\"success\": false, \"error\": \"$1\", \"code\": \"$2\"}"
    exit 1
}

# Main logic
main() {
    log_info "Starting treasury creation..."
    
    # Call CLI tool
    result=$(xion treasury create --output json 2>&1)
    
    if [ $? -eq 0 ]; then
        output_json "$result"
    else
        handle_error "$result" "TREASURY_CREATE_FAILED"
    fi
}

main
```

## Configuration Management

### Configuration File Location

All configuration files are stored in a unified location for easy access across all platforms:

```
~/.xion-toolkit/
├── config.json              # Main configuration (network preference)
└── credentials/             # Per-network credential metadata
    ├── local.json           # Local network credentials metadata
    ├── testnet.json         # Testnet credentials metadata
    └── mainnet.json         # Mainnet credentials metadata
```

**Note**: Sensitive tokens (access_token, refresh_token) are stored securely in the OS keyring, not in plain text files. The `credentials/*.json` files only contain non-sensitive metadata (expiration time, address).

**Platform Support**: The toolkit uses the unified `~/.xion-toolkit/` directory on all platforms (macOS, Linux, Windows) for consistency and ease of access.

### Configuration Schema

#### User Config (`~/.xion-toolkit/config.json`)

```json
{
  "version": "1.0",
  "network": "testnet"
}
```

#### Network Configuration (Compiled into Binary)

Network configurations are embedded at compile time via environment variables:

| Network | OAuth API | RPC | Chain ID | Treasury Code ID |
|---------|-----------|-----|----------|------------------|
| local | http://localhost:8787 | http://localhost:26657 | xion-local | - |
| testnet | https://oauth2.testnet.burnt.com | https://rpc.xion-testnet-2.burnt.com:443 | xion-testnet-2 | 1260 |
| mainnet | https://oauth2.burnt.com | https://rpc.xion-mainnet-1.burnt.com:443 | xion-mainnet-1 | 63 |

#### Credentials Metadata (`~/.xion-toolkit/credentials/{network}.json`)

```json
{
  "expires_at": "2024-01-01T00:00:00Z",
  "xion_address": "xion1..."
}
```

**Note**: Access tokens and refresh tokens are stored securely in the OS keyring, not in files.

## Network Configuration

The toolkit supports three network environments:

### Local Development
- **OAuth API**: http://localhost:8787
- **RPC**: http://localhost:26657
- **Chain ID**: xion-local
- **Usage**: For local development and testing

### Testnet
- **OAuth API**: https://oauth2.testnet.burnt.com
- **RPC**: https://rpc.xion-testnet-2.burnt.com:443
- **Chain ID**: xion-testnet-2
- **Treasury Code ID**: 1260
- **Treasury Config**: xion175qd54keur7gkuwtctfupgtucvlvkrxhv0pgq753sfh5xueputvsms6nll

### Mainnet
- **OAuth API**: Coming soon
- **RPC**: https://rpc.xion-mainnet-1.burnt.com:443
- **Chain ID**: xion-mainnet-1
- **Treasury Code ID**: 63
- **Treasury Config**: xion1dlsvvgey26ernlj0sq2afjluh3qd4ap0k9eerekfkw5algqrwqkshmn3uq

## OAuth2 Authentication Flow

### Pre-configured OAuth Clients

The toolkit uses pre-configured OAuth clients for each network. These clients are already set up with the necessary permissions to manage Treasury contracts, providing the same capabilities as the Developer Portal.

### Login Flow

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant Callback Server
    participant Browser
    participant OAuth2 API

    CLI->>CLI: Generate PKCE verifier & challenge
    CLI->>Callback Server: Start localhost callback server
    CLI->>User: Display authorization URL
    User->>Browser: Open authorization URL
    Browser->>OAuth2 API: Authorization request (with PKCE)
    OAuth2 API->>Browser: Display authorization page
    User->>Browser: Approve authorization
    OAuth2 API->>Callback Server: Callback with auth code
    Callback Server->>OAuth2 API: Exchange code for tokens
    OAuth2 API->>Callback Server: Return access & refresh tokens
    Callback Server->>CLI: Store tokens
    CLI->>User: Output JSON (token info)
```

### Callback Server

The CLI implements a localhost callback server to handle OAuth2 redirects:
- **Default Port**: 8080 (configurable)
- **Callback Path**: /callback
- **Timeout**: 5 minutes
- **Security**: Only accepts localhost connections

## Git Standards

### Commit Messages
```
feat(cli): add OAuth2 login command
fix(treasury): handle insufficient balance error
docs(skill): update xion-treasury skill documentation
chore(config): migrate to new config schema
```

### Branch Strategy
- `main` - Stable release
- `develop` - Development version
- `feature/*` - Feature branches
- `fix/*` - Bug fix branches

## Testing Standards

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pkce_challenge() {
        let verifier = generate_pkce_verifier();
        let challenge = generate_pkce_challenge(&verifier);
        assert!(verify_pkce(&verifier, &challenge));
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_oauth_login() {
    let client = OAuthClient::new("http://localhost:8787");
    let result = client.login().await;
    assert!(result.is_ok());
}
```

## Security Standards

1. **Token Storage**
   - Use OS-native keyring for encrypted storage
   - Never store tokens in plain text
   - Separate storage for different networks

2. **PKCE Implementation**
   - Use cryptographically secure random number generator
   - Verifier length at least 43 characters
   - Use SHA-256 for challenge generation

3. **API Communication**
   - Enforce HTTPS for all external communications
   - Validate server certificates
   - Implement request timeout

4. **Callback Server**
   - Only bind to localhost
   - Validate state parameter
   - Implement timeout mechanism
   - Use random port if default is occupied

## Documentation Standards

1. **README.md** - Project overview and quick start
2. **docs/** - Detailed documentation
   - `cli-reference.md` - CLI command reference
   - `oauth-flow.md` - OAuth2 flow explanation
   - `treasury-guide.md` - Treasury usage guide
3. **examples/** - Example code and scripts

## Language Standards

1. **Conversation Language**
   - Interactive conversations with developers should use the same language as the incoming question (for example, Chinese questions are answered in Chinese, English questions are answered in English).

2. **Documentation & Code Language**
   - All persistent documentation (including `README.md`, files in `docs/`, `plans/`, skill `SKILL.md` files, and `AGENTS.md`) MUST be written in English.
   - All comments written in code MUST be written in English.

## Related Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [OAuth2 API Service](https://github.com/burnt-labs/xion/tree/main/oauth2-api-service)
- [Developer Portal](https://dev.testnet2.burnt.com)
- [Agent Skills Format](https://agentskills.io/)
- [Xion Skills](https://github.com/burnt-labs/xion-skills)

<!-- gitnexus:start -->
# GitNexus — Code Intelligence

This project is indexed by GitNexus as **xion-agent-toolkit** (598 symbols, 1498 relationships, 49 execution flows). Use the GitNexus MCP tools to understand code, assess impact, and navigate safely.

> If any GitNexus tool warns the index is stale, run `npx gitnexus analyze` in terminal first.

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run `gitnexus_impact({target: "symbolName", direction: "upstream"})` and report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run `gitnexus_detect_changes()` before committing** to verify your changes only affect expected symbols and execution flows.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use `gitnexus_query({query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use `gitnexus_context({name: "symbolName"})`.

## When Debugging

1. `gitnexus_query({query: "<error or symptom>"})` — find execution flows related to the issue
2. `gitnexus_context({name: "<suspect function>"})` — see all callers, callees, and process participation
3. `READ gitnexus://repo/xion-agent-toolkit/process/{processName}` — trace the full execution flow step by step
4. For regressions: `gitnexus_detect_changes({scope: "compare", base_ref: "main"})` — see what your branch changed

## When Refactoring

- **Renaming**: MUST use `gitnexus_rename({symbol_name: "old", new_name: "new", dry_run: true})` first. Review the preview — graph edits are safe, text_search edits need manual review. Then run with `dry_run: false`.
- **Extracting/Splitting**: MUST run `gitnexus_context({name: "target"})` to see all incoming/outgoing refs, then `gitnexus_impact({target: "target", direction: "upstream"})` to find all external callers before moving code.
- After any refactor: run `gitnexus_detect_changes({scope: "all"})` to verify only expected files changed.

## Never Do

- NEVER edit a function, class, or method without first running `gitnexus_impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use `gitnexus_rename` which understands the call graph.
- NEVER commit changes without running `gitnexus_detect_changes()` to check affected scope.

## Tools Quick Reference

| Tool | When to use | Command |
|------|-------------|---------|
| `query` | Find code by concept | `gitnexus_query({query: "auth validation"})` |
| `context` | 360-degree view of one symbol | `gitnexus_context({name: "validateUser"})` |
| `impact` | Blast radius before editing | `gitnexus_impact({target: "X", direction: "upstream"})` |
| `detect_changes` | Pre-commit scope check | `gitnexus_detect_changes({scope: "staged"})` |
| `rename` | Safe multi-file rename | `gitnexus_rename({symbol_name: "old", new_name: "new", dry_run: true})` |
| `cypher` | Custom graph queries | `gitnexus_cypher({query: "MATCH ..."})` |

## Impact Risk Levels

| Depth | Meaning | Action |
|-------|---------|--------|
| d=1 | WILL BREAK — direct callers/importers | MUST update these |
| d=2 | LIKELY AFFECTED — indirect deps | Should test |
| d=3 | MAY NEED TESTING — transitive | Test if critical path |

## Resources

| Resource | Use for |
|----------|---------|
| `gitnexus://repo/xion-agent-toolkit/context` | Codebase overview, check index freshness |
| `gitnexus://repo/xion-agent-toolkit/clusters` | All functional areas |
| `gitnexus://repo/xion-agent-toolkit/processes` | All execution flows |
| `gitnexus://repo/xion-agent-toolkit/process/{name}` | Step-by-step execution trace |

## Self-Check Before Finishing

Before completing any code modification task, verify:
1. `gitnexus_impact` was run for all modified symbols
2. No HIGH/CRITICAL risk warnings were ignored
3. `gitnexus_detect_changes()` confirms changes match expected scope
4. All d=1 (WILL BREAK) dependents were updated

## CLI

- Re-index: `npx gitnexus analyze`
- Check freshness: `npx gitnexus status`
- Generate docs: `npx gitnexus wiki`

<!-- gitnexus:end -->
