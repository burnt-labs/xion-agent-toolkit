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
- 🤖 Agent-friendly JSON output
- 🔒 Encrypted credential storage

## Installation

### Prerequisites

- Rust 1.75 or higher
- OpenSSL development libraries

### From Source

```bash
git clone https://github.com/burnt-labs/xion-agent-toolkit
cd xion-agent-toolkit
cargo install --path .
```

## Quick Start

### 1. Configure OAuth Client IDs

```bash
cp .env.example .env
# Edit .env and add your OAuth Client IDs
```

Required variables:
- `XION_LOCAL_OAUTH_CLIENT_ID`
- `XION_TESTNET_OAUTH_CLIENT_ID`
- `XION_MAINNET_OAUTH_CLIENT_ID`

### 2. Check Status

```bash
xion-toolkit status
```

### 3. Login

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
xion-toolkit treasury list                       # List treasuries
xion-toolkit treasury query <address>            # Query details
xion-toolkit treasury fund <address> --amount N  # Fund treasury
xion-toolkit treasury withdraw <address> --amount N  # Withdraw
xion-toolkit treasury grant-config <address> [options]  # Config grants
xion-toolkit treasury fee-config <address> [options]    # Config fees
```

### Configuration

```bash
xion-toolkit config show                  # Show config
xion-toolkit config set-network <network> # Switch network
xion-toolkit status                       # Show status
```

### Global Options

```bash
xion-toolkit --network <local|testnet|mainnet>  # Network override
xion-toolkit --help                             # Show help
xion-toolkit --version                          # Show version
```

## Networks

| Network | OAuth API | Chain ID | Status |
|---------|-----------|----------|--------|
| local | http://localhost:8787 | xion-local | Development |
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
