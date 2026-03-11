# Xion Agent Toolkit

A CLI-driven, Agent-oriented toolkit for developing on the Xion blockchain.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Test Coverage](https://img.shields.io/badge/tests-369%20passing-green)]()
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

## Overview

Xion Agent Toolkit provides a command-line interface for interacting with Xion's MetaAccount system, enabling gasless transactions and automated Treasury management through OAuth2 authentication.

**Key Features:**
- 🔐 OAuth2 authentication with PKCE security
- 💰 Treasury management (list, query, fund, withdraw, export, import)
- ⚙️ Grant & fee configuration
- 👤 Admin management (propose, accept, cancel)
- 🔧 Treasury parameter updates
- 🚀 Generic contract instantiation
- 🔍 Smart contract queries (read-only)
- 🤖 Agent-friendly JSON output
- 🔒 Encrypted credential storage

## Installation

### For AI Agents

If you're setting up an AI Agent, see [INSTALL-FOR-AGENTS.md](./INSTALL-FOR-AGENTS.md) for complete integration instructions.

Please tell your AI Agent the following instruction:

```text
Follow this guide https://raw.githubusercontent.com/burnt-labs/xion-agent-toolkit/main/INSTALL-FOR-AGENTS.md to install and configure the Xion Agent Toolkit skills for AI agents.
```

### From GitHub Releases (Recommended)

Pre-built binaries are available for Linux, macOS, and Windows:

**macOS / Linux:**
```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh
```

**Windows (PowerShell):**
```powershell
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1 | iex"
```

**Manual Download:**

Download the appropriate archive from the [Releases page](https://github.com/burnt-labs/xion-agent-toolkit/releases):

| Platform | Archive |
|----------|---------|
| Linux x64 | `xion-agent-toolkit-x86_64-unknown-linux-gnu.tar.xz` |
| Linux ARM64 | `xion-agent-toolkit-aarch64-unknown-linux-gnu.tar.xz` |
| macOS Intel | `xion-agent-toolkit-x86_64-apple-darwin.tar.xz` |
| macOS Apple Silicon | `xion-agent-toolkit-aarch64-apple-darwin.tar.xz` |
| Windows x64 | `xion-agent-toolkit-x86_64-pc-windows-msvc.zip` |

Extract the archive and add the binary to your `PATH`.

### From Source

Prerequisites:
- Rust 1.75 or higher
- OpenSSL development libraries

```bash
git clone https://github.com/burnt-labs/xion-agent-toolkit
cd xion-agent-toolkit
cargo install --path .
```

### From Crates.io

```bash
cargo install xion-agent-toolkit
```

## Skills for AI Agents

Xion Agent Toolkit includes pre-built skills that wrap CLI commands for easy AI Agent integration. These skills follow the [Agent Skills format](https://agentskills.io/).

### Available Skills

| Skill | Description |
|-------|-------------|
| `xion-dev` | Unified entry point - routes to correct skill based on user needs |
| `xion-toolkit-init` | Install xion-toolkit CLI automatically |
| `xion-oauth2` | OAuth2 authentication (login, logout, status, refresh) |
| `xion-treasury` | Treasury management (list, query, create, fund, withdraw, grants, fees) |

### When to Use xion-toolkit vs xion-skills

**xion-toolkit** (this repo) is for **MetaAccount development** - the primary way to build on Xion:

| Use xion-toolkit when... | Use xion-skills when... |
|--------------------------|-------------------------|
| Building Xion applications | Deploying CosmWasm contracts |
| Managing Treasury contracts | Querying chain data |
| Gasless transactions | Checking transaction status |
| OAuth2 authentication | Mnemonic wallet management |
| Authz/Fee grant configuration | Validator operations |

**For most Xion developers, xion-toolkit is the recommended tool.**

Install both for complete coverage:
```bash
# Primary: MetaAccount development
npx skills add burnt-labs/xion-agent-toolkit

# Secondary: Advanced chain operations
npx skills add burnt-labs/xion-skills
```

### Installing via skills.sh (Recommended)

Install all skills with a single command using [skills.sh](https://skills.sh):

```bash
# Install xion-agent-toolkit skills (includes xion-toolkit-init, xion-oauth2, xion-treasury)
npx skills add burnt-labs/xion-agent-toolkit

# Optionally, also install xion-skills for xiond CLI operations
npx skills add burnt-labs/xion-skills
```

### Using Skills

After installation via skills.sh, the `xion-toolkit` CLI is available and wrapped by skills for agent use.

- **From a shell (human/operator)**: call the CLI directly:

```bash
# Authenticate with OAuth2
xion-toolkit auth login

# List all treasuries
xion-toolkit treasury list

# Query a treasury
xion-toolkit treasury query xion1abc123...
```

- **From AI Agents**: call the installed skill scripts (for JSON-only output and better tooling). See:
  - [INSTALL-FOR-AGENTS.md](./INSTALL-FOR-AGENTS.md)
  - [docs/skills-guide.md](./docs/skills-guide.md)

## Quick Start

### 1. Check Status

```bash
xion-toolkit status
```

Output:
```json
{
  "success": true,
  "network": "testnet",
  "authenticated": true,
  "xion_address": "xion1abc123...",
  "config_path": "~/.xion-toolkit/config.json"
}
```

### 2. Login

```bash
xion-toolkit auth login
```

This opens your browser for OAuth2 authorization and saves tokens securely.

Output:
```json
{
  "success": true,
  "network": "testnet",
  "authenticated": true,
  "token_type": "Bearer",
  "expires_in": 3600,
  "xion_address": "xion1abc123..."
}
```

### 3. Manage Treasuries

```bash
# List treasuries
xion-toolkit treasury list
```

Output:
```json
{
  "success": true,
  "treasuries": [
    {
      "address": "xion1def456...",
      "balance": "10000000",
      "denom": "uxion",
      "admin": "xion1abc123..."
    }
  ],
  "count": 1
}
```

```bash
# Query treasury details
xion-toolkit treasury query xion1def456...

# Fund a treasury (1 XION = 1,000,000 uxion)
xion-toolkit treasury fund xion1def456... --amount 1000000

# Withdraw from a treasury
xion-toolkit treasury withdraw xion1def456... --amount 500000 --to xion1recipient...
```

### 4. Configure Grants and Fees

```bash
# Configure authz grant for sending funds
xion-toolkit treasury grant-config add xion1def456... \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --grant-description "Allow sending funds"

# Configure fee allowance for gasless transactions
xion-toolkit treasury fee-config set xion1def456... \
  --fee-allowance-type basic \
  --fee-spend-limit "5000000uxion" \
  --fee-description "Basic fee allowance"
```

### 5. Admin Management

```bash
# Propose new admin
xion-toolkit treasury admin propose xion1def456... \
  --new-admin xion1newadmin...

# Accept admin role (called by pending admin)
xion-toolkit treasury admin accept xion1def456...

# Cancel proposed admin
xion-toolkit treasury admin cancel xion1def456...
```

### Error Handling

All errors return structured JSON with actionable hints:

```json
{
  "success": false,
  "error": "Not authenticated",
  "error_code": "NOT_AUTHENTICATED",
  "hint": "Run 'xion-toolkit auth login' to authenticate"
}
```

## Contract Commands

```bash
# Instantiate a contract
xion-toolkit contract instantiate \
  --code-id 1260 \
  --label "my-contract" \
  --msg instantiate-msg.json

# Instantiate with predictable address (instantiate2)
xion-toolkit contract instantiate2 \
  --code-id 1260 \
  --label "my-contract" \
  --msg instantiate-msg.json \
  --salt "01020304"

# Execute a message on a deployed contract
xion-toolkit contract execute \
  --contract xion1abc123... \
  --msg execute-msg.json

# Execute with funds
xion-toolkit contract execute \
  --contract xion1abc123... \
  --msg execute-msg.json \
  --funds "1000000uxion"

# Query a smart contract (read-only, no auth required)
xion-toolkit contract query \
  --contract xion1abc123... \
  --msg query.json
```

## CLI Reference

For detailed documentation, see [docs/cli-reference.md](./docs/cli-reference.md).

### Authentication

```bash
xion-toolkit auth login [--port <PORT>]   # OAuth2 login
xion-toolkit auth logout                  # Clear credentials
xion-toolkit auth status                  # Check auth status
xion-toolkit auth refresh                 # Refresh token
```

### Treasury

```bash
# Basic operations
xion-toolkit treasury list                       # List treasuries
xion-toolkit treasury query <address>            # Query details
xion-toolkit treasury fund <address> --amount N  # Fund treasury
xion-toolkit treasury withdraw <address> --amount N --to <recipient>  # Withdraw

# Export/Import (backup & migration)
xion-toolkit treasury export <address> [--output file.json]  # Export config
xion-toolkit treasury import <address> --from-file config.json [--dry-run]  # Import config

# Grant configuration
xion-toolkit treasury grant-config add <address> [options]     # Add grant
xion-toolkit treasury grant-config remove <address> --type-url <url>  # Remove grant
xion-toolkit treasury grant-config list <address>              # List grants

# Fee configuration
xion-toolkit treasury fee-config set <address> [options]    # Set fee config
xion-toolkit treasury fee-config remove <address> --grantee <address> # Remove fee allowance
xion-toolkit treasury fee-config query <address>            # Query fee config

# Admin management
xion-toolkit treasury admin propose <address> --new-admin <address>   # Propose new admin
xion-toolkit treasury admin accept <address>                   # Accept admin role
xion-toolkit treasury admin cancel <address>                   # Cancel proposed admin

# Parameters
xion-toolkit treasury params update <address> [options]        # Update treasury params
```

### Contract

```bash
# Contract instantiation
xion-toolkit contract instantiate --code-id <id> --label <label> --msg <file> [options]
xion-toolkit contract instantiate2 --code-id <id> --label <label> --msg <file> [options]

# Contract execution
xion-toolkit contract execute --contract <address> --msg <file> [--funds <amount>]

# Contract query (read-only, no auth required)
xion-toolkit contract query --contract <address> --msg <file>
```

### Configuration

```bash
xion-toolkit config show                  # Show config
xion-toolkit config set-network <network> # Switch network
xion-toolkit status                       # Show status
```

### Global Options

```bash
xion-toolkit --network <testnet|mainnet>  # Network override
xion-toolkit --help                        # Show help
xion-toolkit --version                     # Show version
```

## Networks

| Network | OAuth API | Chain ID | Status |
|---------|-----------|----------|--------|
| testnet | https://oauth2.testnet.burnt.com | xion-testnet-2 | Default |
| mainnet | https://oauth2.burnt.com | xion-mainnet-1 | Production |

```bash
# Switch networks
xion-toolkit config set-network testnet
xion-toolkit --network mainnet status
```

## Output Format

All commands output JSON for easy Agent integration:

```json
{
  "success": true,
  "treasuries": [...],
  "count": 1
}
```

Errors include actionable hints:

```json
{
  "success": false,
  "error": "Treasury not found",
  "error_code": "TREASURY_NOT_FOUND",
  "hint": "Verify the address or run 'treasury list' to see available treasuries"
}
```

## Security

- **PKCE (RFC 7636)** - Prevents authorization code interception
- **AES-256-GCM** - Encrypted credential storage
- **Localhost Only** - Callback server only accepts localhost
- **HTTPS Only** - All external communications encrypted

## Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [Developer Portal](https://dev.testnet2.burnt.com)
- [Agent Skills Format](https://agentskills.io/)
- [CLI Reference](./docs/cli-reference.md)
- [Contributing Guide](CONTRIBUTING.md)
- [xion-skills](https://github.com/burnt-labs/xion-skills) - Advanced chain operations (xiond)

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
