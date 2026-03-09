# Changelog

All notable changes to the Xion Agent Toolkit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Treasury E2E lifecycle testing (planned)

## [0.2.0] - 2026-03-09

### Added - Treasury Enhancements

#### Admin Management
- `treasury admin propose` - Propose a new admin for treasury
- `treasury admin accept` - Accept admin role (called by pending admin)
- `treasury admin cancel` - Cancel pending admin proposal

#### Params Configuration
- `treasury params update` - Update treasury metadata (redirect_url, icon_url, metadata)

#### Chain Query
- `treasury chain-query grants` - List authz grants via on-chain RPC
- `treasury chain-query allowances` - List fee allowances via on-chain RPC

#### Batch Operations
- `grant_config_batch` API method for batch grant configuration

### Added - Contract Commands

#### New Contract Subcommand
- `contract instantiate` - Instantiate a new smart contract (moved from treasury)
- `contract instantiate2` - Instantiate with predictable address (moved from treasury)

### Added - Documentation
- `docs/cli-reference.md` - Comprehensive CLI command reference
- Updated README.md with all new commands
- Updated skills scripts with new CLI commands
- `skills/xion-treasury/scripts/admin.sh` - Admin management operations
- `skills/xion-treasury/scripts/chain-query.sh` - On-chain query operations
- `skills/xion-treasury/scripts/update-params.sh` - Updated to use actual CLI

### Fixed

#### Transaction Format
- Fixed `msg` and `salt` fields to use number arrays instead of base64 strings
- Fixed Coin protobuf field order: denom=field1, amount=field2
- Fixed `ProtobufAny.value` encoding from Binary to String

#### OAuth2 API Format
- Fixed `MsgExecuteContract.msg` to use raw JSON object format
- Standardized transaction message formats for treasury operations
- Fixed metadata JSON parsing to return clear error messages
- Fixed empty params validation in `update_params`

### Changed

#### CLI Restructure
- **BREAKING**: Moved `instantiate` and `instantiate2` from `treasury` to `contract` subcommand
  - Old: `xion-toolkit treasury instantiate`
  - New: `xion-toolkit contract instantiate`

#### API Changes
- Added `rpc_url` parameter to `TreasuryApiClient::new()` for on-chain queries
- Made `broadcast_instantiate_contract` and `broadcast_instantiate_contract2` public
- Added `InstantiateResult` and `Instantiate2Result` types

### Testing
- 330+ unit tests passing
- Integration tests for treasury operations
- E2E tests verified: grant-config, fee-config, withdraw, fund

## [0.1.0] - 2026-03-05

### Added - Phase 3: Treasury Management (Core)

#### Treasury API Client
- `TreasuryApiClient` - HTTP client for Treasury API endpoints
- `list_treasuries()` - List all treasuries for authenticated user
- `query_treasury()` - Query specific treasury with options
- `get_treasury_balance()` - Get treasury balance
- Bearer token authentication
- Comprehensive error handling

#### Treasury Data Structures
- `TreasuryInfo` - Complete treasury information
- `TreasuryListItem` - Simplified list item
- `TreasuryParams` - Treasury parameters
- `GrantConfig` - Authorization grant configuration
- `FeeConfig` - Fee grant configuration
- `QueryOptions` - Query parameters

#### Treasury Manager
- `TreasuryManager` - High-level treasury management
- Integrates OAuth2 client for automatic token refresh
- Smart caching with 5-minute TTL
- `list()` - List user's treasuries
- `query(address)` - Query treasury details
- `get_balance(address)` - Get treasury balance
- `is_authenticated()` - Check auth status
- `clear_cache()` - Clear cached data

#### Treasury Cache
- In-memory cache with TTL expiration
- Thread-safe async access using `Arc<RwLock<T>>`
- Cache statistics and cleanup methods
- Comprehensive unit tests

#### CLI Commands
- `xion-toolkit treasury list` - List all treasuries (✅ implemented)
- `xion-toolkit treasury query <address>` - Query treasury details (✅ implemented)
- `xion-toolkit treasury create` - Create treasury (🚧 placeholder)
- `xion-toolkit treasury fund` - Fund treasury (🚧 placeholder)
- `xion-toolkit treasury withdraw` - Withdraw from treasury (🚧 placeholder)

### Added - Phase 2: OAuth2 Authentication

#### OAuth2 PKCE Module
- `PKCEChallenge` - PKCE challenge generation
- `generate_pkce_verifier()` - 43-character cryptographically secure verifier
- `generate_pkce_challenge()` - SHA-256 + Base64URL encoding
- `generate_state()` - 32-byte random state parameter
- `verify_state()` - State parameter validation
- Complete RFC 7636 compliance

#### OAuth2 Client
- `OAuthClient` - OAuth2 client implementation
- Browser-based login flow with auto-open
- Localhost callback server (port 54321)
- State parameter CSRF protection
- Code exchange with PKCE verifier
- Token storage in OS keyring
- Automatic token refresh

#### Token Manager
- `TokenManager` - Token lifecycle management
- `get_valid_token()` - Get valid token (auto-refresh)
- `refresh_access_token()` - Refresh access token
- `is_token_expired()` - Check token expiration
- `will_expire_soon()` - Check if token expires soon (5-minute buffer)
- `validate_token()` - Validate token with API

#### Callback Server
- `CallbackServer` - Localhost OAuth2 callback handler
- Axum-based HTTP server
- State parameter validation
- 5-minute timeout
- Single-use server (closes after receiving code)
- Error handling for OAuth2 errors

#### CLI Commands
- `xion-toolkit auth login [--port]` - OAuth2 login
- `xion-toolkit auth logout` - Clear credentials
- `xion-toolkit auth status` - Check auth status
- `xion-toolkit auth refresh` - Refresh access token

### Added - Phase 1: Foundation

#### Project Structure
- Rust project with Cargo
- Modular architecture (cli, oauth, api, treasury, config, utils)
- Library + binary structure

#### CLI Framework
- Clap-based command-line interface
- Hierarchical commands (auth, treasury, config)
- Global flags (--network, --output, --config)
- Help system with examples

#### Configuration System
- `ConfigManager` - Configuration management
- Layered configuration:
  1. Network config (compile-time, auto-generated)
  2. User config (~/.xion-toolkit/config.json)
  3. Credentials (encrypted OS keyring)
- Multi-network support (local, testnet, mainnet)
- Network switching commands

#### Network Configuration
- Pre-configured OAuth Client IDs (environment variables)
- Treasury Code IDs per network
- API endpoints per network
- Compile-time generation via `build.rs`

#### Credential Management
- `CredentialsManager` - Secure credential storage
- OS keyring integration (macOS Keychain, Linux Secret Service, Windows Credential Manager)
- Per-network credential storage
- Encrypted token storage
- Automatic credential isolation

#### Error Handling
- `thiserror` for error definitions
- `anyhow` for error propagation
- Structured error messages
- Error codes for programmatic handling
- Remediation suggestions

#### Output Formatting
- JSON output (default) for Agent consumption
- Structured logging with `tracing`
- Status messages to stderr
- JSON data to stdout

### Security Features
- PKCE (RFC 7636) for OAuth2 security
- State parameter for CSRF protection
- OS-native keyring for token encryption
- HTTPS-only communication
- Localhost-only callback server
- Input validation
- No sensitive data logging

### Testing
- 63 unit tests passing
- Integration test framework
- Mock test utilities
- Test coverage for all modules

### Documentation
- Comprehensive README with examples
- Architecture design documents
- Development plans
- Code documentation (rustdoc)
- Inline code comments

### Dependencies
- clap 4.5 - CLI framework
- tokio 1.37 - Async runtime
- reqwest 0.12 - HTTP client
- serde / serde_json - Serialization
- thiserror / anyhow - Error handling
- sha2 / base64 / rand - PKCE crypto
- axum / tower - Callback server
- chrono - Time handling
- open - Browser launching
- keyring - Secure storage
- hex - Hex encoding
- urlencoding - URL encoding
- tracing - Structured logging
- directories - Cross-platform directories

## [0.0.1] - 2025-03-05

### Added
- Initial project structure
- Basic Cargo.toml setup
- Project planning documents

---

## Version History

- **0.2.0** (2026-03-09) - Treasury Enhancements, Contract Commands, Documentation
- **0.1.0** (2026-03-05) - Phase 1, 2, 3 core complete
- **0.0.1** (2025-03-05) - Initial project setup

## Next Milestones

- **0.3.0** - Phase 5: Advanced Features
  - Treasury E2E lifecycle testing
  - Batch CLI commands
  - Performance optimizations
  
- **1.0.0** - Production release
  - Complete feature set
  - Comprehensive testing
  - Full documentation
  - Stable API
