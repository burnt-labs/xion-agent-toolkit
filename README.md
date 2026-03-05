# Xion Agent Toolkit

A CLI-driven, Agent-oriented toolkit for developing on the Xion blockchain.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Test Coverage](https://img.shields.io/badge/tests-63%20passing-green)]()
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()

## Overview

Xion Agent Toolkit provides a command-line interface for interacting with Xion's MetaAccount system, enabling gasless transactions and automated Treasury management through OAuth2 authentication.

## Current Status

### ✅ Implemented Features

#### Authentication (Phase 2 - Complete)
- ✅ **OAuth2 Login Flow** - Browser-based authentication with PKCE security
- ✅ **Token Management** - Auto-refresh, secure storage (OS keyring)
- ✅ **Multi-Network Support** - Local, testnet, and mainnet
- ✅ **Session Persistence** - Per-network credential storage

#### Treasury Management (Phase 3 - Core Complete)
- ✅ **List Treasuries** - View all Treasury contracts for authenticated user
- ✅ **Query Treasury** - Get detailed Treasury information (balance, grants, configs)
- ✅ **Caching** - Smart 5-minute TTL cache for performance
- ✅ **JSON Output** - Agent-friendly structured output

#### Infrastructure (Phase 1 - Complete)
- ✅ **CLI Framework** - Built with clap for powerful command-line interface
- ✅ **Configuration System** - Layered config (compile-time + user + credentials)
- ✅ **Error Handling** - Structured errors with helpful suggestions
- ✅ **Network Management** - Easy switching between environments

### 🚧 Planned Features

- ⏳ **Treasury Creation** - Create new Treasury contracts (Phase 3.5)
- ⏳ **Treasury Operations** - Fund and withdraw operations (Phase 3.5)
- ⏳ **Grant Management** - Configure Fee Grants and Authz Grants (Phase 3.5)
- ⏳ **Agent Skills** - Pre-built skills for common operations (Phase 4)

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

Output:
```json
{
  "authenticated": false,
  "callback_port": 54321,
  "chain_id": "xion-testnet-2",
  "network": "testnet",
  "oauth_api_url": "https://oauth2.testnet.burnt.com"
}
```

### 2. Configure OAuth Client IDs

Create `.env` file in the project root:

```bash
cp .env.example .env
# Edit .env and add your OAuth Client IDs
```

Required variables:
- `XION_LOCAL_OAUTH_CLIENT_ID`
- `XION_TESTNET_OAUTH_CLIENT_ID`
- `XION_MAINNET_OAUTH_CLIENT_ID`

### 3. Login

```bash
xion auth login
```

This will:
- Generate a PKCE challenge for security
- Start a localhost callback server (port 54321)
- Open your browser for OAuth2 authorization
- Save tokens securely in OS keyring
- Return JSON with authentication details

Output:
```json
{
  "success": true,
  "network": "testnet",
  "xion_address": "xion1...",
  "expires_at": "2024-01-01T01:00:00Z"
}
```

### 4. List Treasuries

```bash
xion treasury list
```

Output:
```json
{
  "success": true,
  "treasuries": [
    {
      "address": "xion1abc123...",
      "admin": "xion1admin...",
      "params": {
        "display_url": "https://myapp.com",
        "redirect_url": "https://myapp.com/callback",
        "icon_url": "https://myapp.com/icon.png"
      }
    }
  ],
  "count": 1
}
```

### 5. Query Treasury Details

```bash
xion treasury query xion1abc123...
```

Output:
```json
{
  "success": true,
  "treasury": {
    "address": "xion1abc123...",
    "admin": "xion1admin...",
    "balance": "1000000",
    "params": {
      "display_url": "https://myapp.com",
      "redirect_url": "https://myapp.com/callback",
      "icon_url": "https://myapp.com/icon.png"
    },
    "fee_config": { ... },
    "grant_configs": [ ... ]
  }
}
```

### 6. Check Authentication Status

```bash
xion auth status
```

Output (authenticated):
```json
{
  "authenticated": true,
  "chain_id": "xion-testnet-2",
  "network": "testnet",
  "oauth_api_url": "https://oauth2.testnet.burnt.com",
  "xion_address": "xion1...",
  "expires_at": "2024-01-01T01:00:00Z"
}
```

### 7. Logout

```bash
xion auth logout
```

Output:
```json
{
  "success": true,
  "message": "Logged out successfully",
  "network": "testnet"
}
```

## CLI Commands

### Authentication Commands

```bash
xion auth login [--port <PORT>]   # OAuth2 login (default port: 54321)
xion auth logout                  # Clear credentials
xion auth status                  # Check authentication status
xion auth refresh                 # Refresh access token
```

### Treasury Commands

```bash
xion treasury list                # List all treasuries
xion treasury query <address>     # Query treasury details

# Future commands (not yet implemented)
xion treasury create [options]    # Create new treasury
xion treasury fund <address>      # Fund treasury
xion treasury withdraw <address>  # Withdraw from treasury
```

### Configuration Commands

```bash
xion config show                  # Show current config
xion config set-network <network> # Switch network (local/testnet/mainnet)
xion config get <key>             # Get config value
xion config reset                 # Reset to defaults
```

### Utility Commands

```bash
xion status                       # Show current network and auth status
xion --help                       # Show help
xion --version                    # Show version
```

## Network Configuration

### Supported Networks

| Network | OAuth API | RPC | Chain ID | Status |
|---------|-----------|-----|----------|--------|
| local | http://localhost:8787 | http://localhost:26657 | xion-local | ✅ Active |
| testnet | https://oauth2.testnet.burnt.com | https://rpc.xion-testnet-2.burnt.com:443 | xion-testnet-2 | ✅ Active |
| mainnet | https://oauth2.burnt.com | https://rpc.xion-mainnet-1.burnt.com:443 | xion-mainnet-1 | 🚧 Coming Soon |

### Switch Networks

```bash
# Switch to local development
xion config set-network local

# Switch to testnet (default)
xion config set-network testnet

# Switch to mainnet
xion config set-network mainnet

# Or use global flag
xion --network local status
xion --network testnet auth login
xion --network mainnet treasury list
```

## Configuration Architecture

### Layered Configuration System

```
┌─────────────────────────────────────────┐
│  1. Network Config (Compile-time)       │
│  - OAuth Client IDs                     │
│  - Treasury Code IDs                    │
│  - API endpoints                        │
│  - Generated by build.rs                │
└─────────────────────────────────────────┘
                ↓
┌─────────────────────────────────────────┐
│  2. User Config (~/.xion-toolkit/)      │
│  - config.json (active network)         │
│  - Per-user settings                    │
└─────────────────────────────────────────┘
                ↓
┌─────────────────────────────────────────┐
│  3. Credentials (Encrypted)             │
│  - OS Keyring (tokens)                  │
│  - Per-network storage                  │
│  - credentials/{network}.json (metadata)│
└─────────────────────────────────────────┘
```

### Configuration Files

**User Config** (`~/.xion-toolkit/config.json`):
```json
{
  "version": "1.0",
  "network": "testnet"
}
```

**Credentials Metadata** (`~/.xion-toolkit/credentials/testnet.json`):
```json
{
  "expires_at": "2024-01-01T01:00:00Z",
  "xion_address": "xion1..."
}
```

**Sensitive Tokens** (OS Keyring):
- Service: `xion-toolkit-testnet`
- Username: `access_token`, `refresh_token`
- Encrypted by OS (Keychain on macOS, Secret Service on Linux, Credential Manager on Windows)

## Security Features

### Authentication Security
- ✅ **PKCE (RFC 7636)** - Prevents authorization code interception
- ✅ **State Parameter** - CSRF protection
- ✅ **Localhost Only** - Callback server only accepts localhost connections
- ✅ **Secure Storage** - Tokens encrypted in OS keyring
- ✅ **Auto Token Refresh** - Seamless session management

### Treasury Security
- ✅ **Bearer Token Auth** - All Treasury API calls authenticated
- ✅ **Per-Network Credentials** - Isolated credentials per network
- ✅ **Smart Caching** - Reduces API calls with TTL expiration

### Communication Security
- ✅ **HTTPS Only** - All external communications over HTTPS
- ✅ **Certificate Validation** - Server certificates validated
- ✅ **Request Timeout** - Prevents hanging requests
- ✅ **No Sensitive Logging** - Tokens and secrets never logged

## Project Architecture

```
xion-agent-cli/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── cli/                 # CLI command handlers
│   │   ├── auth.rs          # Authentication commands
│   │   ├── treasury.rs      # Treasury commands
│   │   └── config.rs        # Configuration commands
│   ├── oauth/               # OAuth2 implementation
│   │   ├── pkce.rs          # PKCE implementation
│   │   ├── client.rs        # OAuth2 client
│   │   ├── callback_server.rs  # Localhost callback
│   │   └── token_manager.rs   # Token lifecycle
│   ├── api/                 # API clients
│   │   ├── oauth2_api.rs    # OAuth2 API client
│   │   └── treasury_api.rs  # Treasury API client
│   ├── treasury/            # Treasury management
│   │   ├── types.rs         # Data structures
│   │   ├── manager.rs       # High-level manager
│   │   └── cache.rs         # Caching system
│   ├── config/              # Configuration
│   │   ├── constants.rs     # Network config (auto-generated)
│   │   ├── credentials.rs   # Credential management
│   │   └── manager.rs       # Config manager
│   └── utils/               # Utilities
│       ├── error.rs         # Error definitions
│       └── output.rs        # Output formatting
├── plans/                   # Development plans
│   ├── treasury-automation.md
│   ├── oauth2-client-architecture.md
│   └── treasury-api-architecture.md
├── .env.example             # Environment variables template
├── build.rs                 # Compile-time config generation
└── Cargo.toml               # Rust dependencies
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

Current test status: ✅ **63 tests passing**

### Running in Development

```bash
cargo run -- [commands]
```

### Code Quality

```bash
cargo clippy        # Linting
cargo fmt           # Formatting
cargo test          # Run tests
```

## Dependencies

### Core Dependencies
- `clap` - CLI framework
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde` / `serde_json` - Serialization
- `thiserror` / `anyhow` - Error handling

### Security Dependencies
- `keyring` - OS-native credential storage
- `sha2` / `base64` / `rand` - PKCE implementation
- `hex` - Hex encoding

### Web Dependencies
- `axum` / `tower` - Callback server
- `open` - Browser launching
- `urlencoding` - URL encoding

### Utilities
- `chrono` - Date/time handling
- `tracing` - Structured logging
- `directories` - Cross-platform directories

## Roadmap

### ✅ Phase 1: Foundation (Complete)
- Project structure
- CLI framework
- Configuration system
- Error handling

### ✅ Phase 2: OAuth2 Authentication (Complete)
- PKCE implementation
- OAuth2 client
- Token management
- Callback server
- CLI integration

### ✅ Phase 3: Treasury Management - Core (Complete)
- Treasury API client
- Treasury manager
- List and query commands
- Caching system

### 🚧 Phase 3.5: Treasury Advanced (Planned)
- Treasury creation
- Fund and withdraw operations
- Grant configuration management

### 🚧 Phase 4: Agent Skills (Planned)
- xion-oauth2 skill
- xion-treasury skill
- Documentation and examples

## Contributing

We welcome contributions! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Submit a pull request

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [OAuth2 API Service](https://github.com/burnt-labs/xion/tree/main/oauth2-api-service)
- [Developer Portal](https://dev.testnet2.burnt.com)
- [Agent Skills Format](https://agentskills.io/)

## Support

- GitHub Issues: [Project Issues Page](https://github.com/burnt-labs/xion-agent-cli/issues)
- Documentation: See `plans/` directory for detailed architecture and implementation docs

---

**Built with ❤️ for the Xion ecosystem**
