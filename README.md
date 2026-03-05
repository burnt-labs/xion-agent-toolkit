# Xion Agent Toolkit

A CLI-driven, Agent-oriented toolkit for developing on the Xion blockchain.

## Overview

Xion Agent Toolkit provides a command-line interface for interacting with Xion's MetaAccount system, enabling gasless transactions and automated Treasury management through OAuth2 authentication.

## Features

- **OAuth2 Authentication**: Browser-based login flow with PKCE security
- **Treasury Management**: Create, query, and manage Treasury contracts
- **Grant Configuration**: Automated Fee Grant and Authz Grant setup
- **Network Support**: Local, testnet, and mainnet environments
- **Agent-Friendly**: JSON output format for easy integration with AI agents

## Installation

### From Source

```bash
git clone https://github.com/burnt-labs/xion-agent-cli
cd xion-agent-cli
cargo install --path .
```

### Prerequisites

- Rust 1.75 or higher
- OpenSSL development libraries

## Quick Start

### 1. Check Status

```bash
xion status
```

### 2. Login

```bash
xion auth login
```

This will:
- Generate a PKCE challenge
- Start a local callback server on port 8080
- Open your browser for OAuth2 authorization
- Save tokens securely

### 3. List Treasuries

```bash
xion treasury list
```

### 4. Create a Treasury

```bash
xion treasury create \
  --fee-grant basic:1000000uxion \
  --grant-config authz:cosmwasm.wasm.v1.MsgExecuteContract
```

### 5. Query Treasury

```bash
xion treasury query xion1abc123...
```

## Network Configuration

Switch between networks using the `--network` flag:

```bash
# Local development
xion --network local status

# Testnet (default)
xion --network testnet status

# Mainnet
xion --network mainnet status
```

## Configuration

Configuration is stored in `~/.xion-toolkit/config.json`:

```json
{
  "version": "1.0",
  "network": "testnet",
  "oauth": {
    "client_id": "your-client-id",
    "access_token": "encrypted-token",
    "refresh_token": "encrypted-refresh-token",
    "expires_at": "2024-01-01T00:00:00Z"
  },
  "treasury": {
    "default_address": "xion1..."
  },
  "networks": {
    "local": {
      "oauth_api_url": "http://localhost:8787",
      "rpc_url": "http://localhost:26657",
      "chain_id": "xion-local"
    },
    "testnet": {
      "oauth_api_url": "https://oauth2.testnet.burnt.com",
      "rpc_url": "https://rpc.xion-testnet-2.burnt.com:443",
      "chain_id": "xion-testnet-2",
      "treasury_code_id": 1260
    },
    "mainnet": {
      "oauth_api_url": "https://oauth2.burnt.com",
      "rpc_url": "https://rpc.xion-mainnet-1.burnt.com:443",
      "chain_id": "xion-mainnet-1",
      "treasury_code_id": 63
    }
  }
}
```

## CLI Commands

### Authentication

```bash
xion auth login [--port 8080]   # OAuth2 login
xion auth logout                # Clear credentials
xion auth status                # Check auth status
xion auth refresh               # Refresh access token
```

### Treasury Management

```bash
xion treasury list              # List all treasuries
xion treasury query <address>   # Query treasury details
xion treasury create [options]  # Create new treasury
xion treasury fund <address> --amount <amount>  # Fund treasury
xion treasury withdraw <address> --amount <amount>  # Withdraw from treasury
```

### Configuration

```bash
xion config show                # Show current config
xion config get <key>           # Get config value
xion config set <key> <value>   # Set config value
xion config reset               # Reset to defaults
```

### Status

```bash
xion status                     # Show current network and auth status
```

## Output Format

All commands support JSON output (default) for Agent consumption:

```bash
xion --output json treasury list
```

Example output:

```json
{
  "success": true,
  "treasuries": [
    {
      "address": "xion1abc123...",
      "admin": "xion1admin...",
      "balance": "1000000"
    }
  ],
  "count": 1
}
```

## Skills

Xion Agent Toolkit includes pre-built Agent Skills:

### xion-oauth2

Scripts for OAuth2 authentication:
- `login.sh` - Initiate OAuth2 login
- `status.sh` - Check authentication status
- `logout.sh` - Clear credentials
- `refresh.sh` - Refresh access token

### xion-treasury

Scripts for Treasury management:
- `create.sh` - Create a new Treasury
- `list.sh` - List user's Treasury contracts
- `query.sh` - Query Treasury details
- `fund.sh` - Fund a Treasury
- `withdraw.sh` - Withdraw from Treasury
- `grant-config.sh` - Configure Authz Grants
- `fee-config.sh` - Configure Fee Grants

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running

```bash
cargo run -- [commands]
```

## Architecture

```
xion-agent-cli/
├── src/
│   ├── cli/          # CLI command handlers
│   ├── oauth/        # OAuth2 client implementation
│   ├── api/          # API clients (OAuth2, Treasury, xiond)
│   ├── config/       # Configuration management
│   └── utils/        # Utilities (output, errors)
├── skills/           # Agent Skills
│   ├── xion-oauth2/
│   └── xion-treasury/
└── plans/            # Development plans
```

## Security

- **Token Storage**: Tokens are encrypted and stored in the OS keyring
- **PKCE**: OAuth2 authorization uses PKCE for enhanced security
- **HTTPS**: All API communication uses HTTPS
- **Localhost Only**: Callback server only accepts localhost connections

## Network Endpoints

### Local
- OAuth API: http://localhost:8787
- RPC: http://localhost:26657
- Chain ID: xion-local

### Testnet
- OAuth API: https://oauth2.testnet.burnt.com
- RPC: https://rpc.xion-testnet-2.burnt.com:443
- Chain ID: xion-testnet-2
- Treasury Code ID: 1260

### Mainnet
- OAuth API: https://oauth2.burnt.com
- RPC: https://rpc.xion-mainnet-1.burnt.com:443
- Chain ID: xion-mainnet-1
- Treasury Code ID: 63

## Documentation

- [CLI Reference](docs/cli-reference.md) (Coming soon)
- [OAuth2 Flow](docs/oauth-flow.md) (Coming soon)
- [Treasury Guide](docs/treasury-guide.md) (Coming soon)

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

## License

MIT OR Apache-2.0

## Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [OAuth2 API Service](https://github.com/burnt-labs/xion/tree/main/oauth2-api-service)
- [Developer Portal](https://dev.testnet2.burnt.com)
- [Agent Skills Format](https://agentskills.io/)

## Support

- Telegram: [Developer Group]
- Discord: [Dev Chat]
- GitHub Issues: [Project Issues Page]
