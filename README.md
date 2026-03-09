# Xion Agent Toolkit

A CLI-driven, Agent-oriented toolkit for developing on the Xion blockchain.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Test Coverage](https://img.shields.io/badge/tests-330%20passing-green)]()
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

## Overview

Xion Agent Toolkit provides a command-line interface for interacting with Xion's MetaAccount system, enabling gasless transactions and automated Treasury management through OAuth2 authentication.

**Key Features:**
- 🔐 OAuth2 authentication with PKCE security
- 💰 Treasury management (list, query, fund, withdraw)
- ⚙️ Grant & fee configuration
- 👤 Admin management (propose, accept, cancel)
- 🔧 Treasury parameter updates
- 🔍 On-chain queries (grants, allowances)
- 🚀 Generic contract instantiation
- 🤖 Agent-friendly JSON output
- 🔒 Encrypted credential storage

## Installation

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

## Quick Start

### 1. Check Status

```bash
xion-toolkit status
```

### 2. Login

```bash
xion-toolkit auth login
```

This opens your browser for OAuth2 authorization and saves tokens securely.

### 4. Manage Treasuries

```bash
# List treasuries
xion-toolkit treasury list

# Query treasury details
xion-toolkit treasury query xion1abc123...

# Fund a treasury
xion-toolkit treasury fund xion1abc123... --amount 1000000

# Withdraw from a treasury
xion-toolkit treasury withdraw xion1abc123... --amount 500000

# Configure grants
xion-toolkit treasury grant-config xion1abc123... \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion"

# Configure fee allowance
xion-toolkit treasury fee-config xion1abc123... \
  --fee-allowance-type basic \
  --fee-spend-limit "5000000uxion"

# Propose new admin
xion-toolkit treasury admin propose xion1abc123... \
  --new-admin xion1newadmin...

# Accept admin role
xion-toolkit treasury admin accept xion1abc123...

# Update treasury parameters
xion-toolkit treasury params update xion1abc123... \
  --redirect-url "https://example.com/callback" \
  --metadata '{"name":"Updated Treasury"}'

# Query on-chain grants
xion-toolkit treasury chain-query grants xion1abc123...
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
```

## CLI Reference

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
xion-toolkit treasury withdraw <address> --amount N  # Withdraw

# Grant configuration
xion-toolkit treasury grant-config add <address> [options]     # Add grant
xion-toolkit treasury grant-config remove <address> --type-url <url>  # Remove grant
xion-toolkit treasury grant-config list <address>              # List grants

# Fee configuration
xion-toolkit treasury fee-config set <address> --fee-config <file>    # Set fee config
xion-toolkit treasury fee-config remove <address> --grantee <address> # Remove fee allowance
xion-toolkit treasury fee-config query <address>               # Query fee config

# Admin management
xion-toolkit treasury admin propose <address> --new-admin <address>   # Propose new admin
xion-toolkit treasury admin accept <address>                   # Accept admin role
xion-toolkit treasury admin cancel <address>                   # Cancel proposed admin

# Parameters
xion-toolkit treasury params update <address> [options]        # Update treasury params

# On-chain queries
xion-toolkit treasury chain-query grants <address>             # Query authz grants
xion-toolkit treasury chain-query allowances <address>         # Query fee allowances
```

### Contract

```bash
# Contract instantiation
xion-toolkit contract instantiate --code-id <id> --label <label> --msg <file> [options]
xion-toolkit contract instantiate2 --code-id <id> --label <label> --msg <file> [options]
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

## Security

- **PKCE (RFC 7636)** - Prevents authorization code interception
- **AES-256-GCM** - Encrypted credential storage
- **Localhost Only** - Callback server only accepts localhost
- **HTTPS Only** - All external communications encrypted

## Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [Developer Portal](https://dev.testnet2.burnt.com)
- [Agent Skills Format](https://agentskills.io/)
- [Contributing Guide](CONTRIBUTING.md)

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
