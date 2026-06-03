# Architecture Design: Treasury Management System

## System Overview

The Treasury Management System provides a comprehensive interface for managing Xion Treasury contracts through the OAuth2 API Service. It integrates with the existing OAuth2 authentication system to provide secure, gasless treasury operations for Agent-driven workflows.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLI Layer                                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐ │
│  │ treasury list│  │treasury query│  │ treasury create/fund │ │
│  └──────────────┘  └──────────────┘  └──────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Treasury Manager Layer                        │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              TreasuryManager                              │  │
│  │  - list()                                                 │  │
│  │  - query(address)                                         │  │
│  │  - get_balance(address)                                   │  │
│  │  - create(config)                                         │  │
│  │  - fund(address, amount)                                  │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      API Client Layer                            │
│  ┌────────────────────┐         ┌─────────────────────────┐   │
│  │ TreasuryApiClient  │         │   OAuthClient           │   │
│  │ - list_treasuries()│◄────────┤   - get_valid_token()   │   │
│  │ - query_treasury() │         │   - auto_refresh        │   │
│  │ - create_treasury()│         └─────────────────────────┘   │
│  └────────────────────┘                                         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                  External Services                               │
│  ┌────────────────────────────────────────────────────────┐    │
│  │         OAuth2 API Service (mgr-api)                    │    │
│  │  - GET  /mgr-api/treasuries                             │    │
│  │  - GET  /mgr-api/treasury/{address}                     │    │
│  │  - POST /mgr-api/treasury/create (future)               │    │
│  └────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

## Technology Stack

- **Language**: Rust
- **HTTP Client**: reqwest (with tokio async runtime)
- **Serialization**: serde + serde_json
- **Error Handling**: thiserror + anyhow
- **Logging**: tracing
- **Authentication**: OAuth2 Bearer tokens (from existing OAuthClient)

## Module Structure

```
src/
├── api/
│   ├── mod.rs                    # API module exports
│   ├── oauth2_api.rs             # Existing OAuth2 API client
│   └── treasury_api.rs           # NEW: Treasury API client
├── treasury/
│   ├── mod.rs                    # Treasury module exports
│   ├── manager.rs                # NEW: Treasury manager (high-level API)
│   ├── types.rs                  # NEW: Treasury data structures
│   └── cache.rs                  # NEW: Treasury data caching
└── cli/
    ├── mod.rs                    # CLI module exports
    ├── auth.rs                   # Existing auth commands
    └── treasury.rs               # MODIFIED: Treasury commands implementation
```

## Module Responsibilities

### 1. Treasury API Client (`src/api/treasury_api.rs`)

**Responsibility**: Low-level HTTP client for Treasury API endpoints

**Key Methods**:
- `list_treasuries(access_token)` - List all user's treasuries
- `query_treasury(access_token, address, options)` - Query treasury details
- `create_treasury(access_token, config)` - Create new treasury (future)
- `fund_treasury(access_token, address, amount)` - Fund treasury (future)

**Dependencies**:
- `reqwest::Client` for HTTP requests
- `serde` for JSON serialization
- `OAuth2ApiClient` pattern for consistency

### 2. Treasury Data Structures (`src/treasury/types.rs`)

**Responsibility**: Define all Treasury-related data types

**Key Types**:
- `TreasuryInfo` - Complete treasury information
- `TreasuryListItem` - Simplified list item
- `TreasuryParams` - Treasury parameters
- `FeeConfig` - Fee grant configuration
- `GrantConfig` - Authz grant configuration
- `QueryOptions` - Query parameters

### 3. Treasury Manager (`src/treasury/manager.rs`)

**Responsibility**: High-level treasury management, integrates OAuth2 and caching

**Key Methods**:
- `list()` - List treasuries with auto token refresh
- `query(address)` - Query treasury with auto token refresh
- `get_balance(address)` - Get treasury balance
- `create(config)` - Create treasury (future)
- `fund(address, amount)` - Fund treasury (future)

**Dependencies**:
- `OAuthClient` for token management
- `TreasuryApiClient` for API calls
- `TreasuryCache` for data caching

### 4. Treasury Cache (`src/treasury/cache.rs`)

**Responsibility**: Cache treasury data to reduce API calls

**Key Features**:
- In-memory cache with TTL
- Cache invalidation on updates
- Optional persistent cache (future)

### 5. Treasury CLI (`src/cli/treasury.rs`)

**Responsibility**: CLI command handlers for treasury operations

**Commands**:
- `xion-toolkit treasury list` - List all treasuries
- `xion-toolkit treasury query <address>` - Query treasury details
- `xion-toolkit treasury create` - Create treasury (future)
- `xion-toolkit treasury fund <address> --amount <amount>` - Fund treasury (future)
- `xion-toolkit treasury withdraw <address> --amount <amount>` - Withdraw funds (future)

## API Design

### Treasury API Client Interface

```rust
// src/api/treasury_api.rs

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Treasury API Client
pub struct TreasuryApiClient {
    base_url: String,
    http_client: Client,
}

/// Query options for treasury details
#[derive(Debug, Clone)]
pub struct QueryOptions {
    pub grants: bool,
    pub fee: bool,
    pub admin: bool,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            grants: true,
            fee: true,
            admin: true,
        }
    }
}

impl TreasuryApiClient {
    /// Create new Treasury API client
    pub fn new(base_url: String) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { base_url, http_client }
    }
    
    /// List all treasuries for authenticated user
    pub async fn list_treasuries(
        &self,
        access_token: &str,
    ) -> Result<Vec<TreasuryListItem>>;
    
    /// Query specific treasury details
    pub async fn query_treasury(
        &self,
        access_token: &str,
        address: &str,
        options: QueryOptions,
    ) -> Result<TreasuryInfo>;
    
    /// Get treasury balance
    pub async fn get_treasury_balance(
        &self,
        access_token: &str,
        address: &str,
    ) -> Result<String>;
}
```

## Data Models

### Treasury Information

```rust
// src/treasury/types.rs

use serde::{Deserialize, Serialize};

/// Treasury list item (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryListItem {
    /// Treasury contract address
    pub address: String,
    /// Admin address
    pub admin: Option<String>,
    /// Treasury balance
    pub balance: String,
    /// Display name
    #[serde(default)]
    pub name: Option<String>,
    /// Creation timestamp
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Complete treasury information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryInfo {
    /// Treasury contract address
    pub address: String,
    /// Admin address
    pub admin: Option<String>,
    /// Treasury balance in uxion
    pub balance: String,
    /// Treasury parameters
    pub params: TreasuryParams,
    /// Fee grant configuration
    #[serde(default)]
    pub fee_config: Option<FeeConfig>,
    /// Grant configurations
    #[serde(default)]
    pub grant_configs: Option<Vec<GrantConfig>>,
}

/// Treasury parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryParams {
    /// Display URL
    #[serde(default)]
    pub display_url: Option<String>,
    /// Redirect URL
    pub redirect_url: String,
    /// Icon URL
    pub icon_url: String,
    /// Additional metadata
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Fee grant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfig {
    /// Fee grant type (e.g., "basic", "limited")
    #[serde(rename = "type")]
    pub config_type: String,
    /// Maximum spend limit
    pub spend_limit: Option<String>,
    /// Expiration time
    #[serde(default)]
    pub expires_at: Option<String>,
    /// Additional configuration
    #[serde(flatten)]
    pub additional: Option<serde_json::Value>,
}

/// Grant configuration (for Authz)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfig {
    /// Type URL of the message type
    pub type_url: String,
    /// Grant configuration
    pub grant_config: serde_json::Value,
}

/// Treasury creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTreasuryRequest {
    /// Fee grant configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_grant: Option<FeeGrantRequest>,
    /// Grant configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_config: Option<GrantConfigRequest>,
    /// Initial funding amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_fund: Option<String>,
}

/// Fee grant request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeGrantRequest {
    /// Fee grant type
    #[serde(rename = "type")]
    pub grant_type: String,
    /// Spend limit
    pub spend_limit: String,
}

/// Grant config request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigRequest {
    /// Message type URL
    pub type_url: String,
    /// Grant configuration
    pub config: serde_json::Value,
}
```

## Treasury Manager Design

```rust
// src/treasury/manager.rs

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::api::treasury_api::{TreasuryApiClient, QueryOptions};
use crate::oauth::OAuthClient;
use crate::treasury::{TreasuryInfo, TreasuryListItem, TreasuryCache};

/// Treasury Manager
/// 
/// High-level manager for treasury operations with automatic token refresh
/// and caching support.
pub struct TreasuryManager {
    /// OAuth client for token management
    oauth_client: OAuthClient,
    /// Treasury API client
    api_client: TreasuryApiClient,
    /// Optional cache
    cache: Option<Arc<RwLock<TreasuryCache>>>,
}

impl TreasuryManager {
    /// Create new Treasury manager
    pub fn new(oauth_client: OAuthClient, api_base_url: String) -> Self {
        let api_client = TreasuryApiClient::new(api_base_url);
        let cache = Some(Arc::new(RwLock::new(TreasuryCache::new())));
        
        Self {
            oauth_client,
            api_client,
            cache,
        }
    }
    
    /// List all treasuries for the authenticated user
    pub async fn list(&self) -> Result<Vec<TreasuryListItem>> {
        // Check cache first
        if let Some(cache) = &self.cache {
            let cache_read = cache.read().await;
            if let Some(cached) = cache_read.get_treasury_list() {
                return Ok(cached);
            }
        }
        
        // Get valid token (auto-refresh if needed)
        let token = self.oauth_client.get_valid_token().await?;
        
        // Call API
        let treasuries = self.api_client.list_treasuries(&token).await?;
        
        // Update cache
        if let Some(cache) = &self.cache {
            let mut cache_write = cache.write().await;
            cache_write.set_treasury_list(treasuries.clone());
        }
        
        Ok(treasuries)
    }
    
    /// Query specific treasury details
    pub async fn query(&self, address: &str) -> Result<TreasuryInfo> {
        // Check cache first
        if let Some(cache) = &self.cache {
            let cache_read = cache.read().await;
            if let Some(cached) = cache_read.get_treasury(address) {
                return Ok(cached);
            }
        }
        
        // Get valid token
        let token = self.oauth_client.get_valid_token().await?;
        
        // Call API with full options
        let options = QueryOptions::default();
        let treasury = self.api_client.query_treasury(&token, address, options).await?;
        
        // Update cache
        if let Some(cache) = &self.cache {
            let mut cache_write = cache.write().await;
            cache_write.set_treasury(address.to_string(), treasury.clone());
        }
        
        Ok(treasury)
    }
    
    /// Get treasury balance
    pub async fn get_balance(&self, address: &str) -> Result<String> {
        let treasury = self.query(address).await?;
        Ok(treasury.balance)
    }
    
    /// Create new treasury (future implementation)
    pub async fn create(&self, _request: CreateTreasuryRequest) -> Result<TreasuryInfo> {
        anyhow::bail!("Treasury creation not yet implemented");
    }
    
    /// Fund treasury (future implementation)
    pub async fn fund(&self, _address: &str, _amount: &str) -> Result<()> {
        anyhow::bail!("Treasury funding not yet implemented");
    }
}
```

## Cache Implementation

```rust
// src/treasury/cache.rs

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::treasury::{TreasuryInfo, TreasuryListItem};

/// Cache entry with expiration
struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

impl<T> CacheEntry<T> {
    fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            expires_at: Instant::now() + ttl,
        }
    }
    
    fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }
}

/// Treasury data cache
pub struct TreasuryCache {
    /// Treasury list cache
    treasury_list: Option<CacheEntry<Vec<TreasuryListItem>>>,
    /// Individual treasury cache
    treasuries: HashMap<String, CacheEntry<TreasuryInfo>>,
    /// Default TTL (5 minutes)
    default_ttl: Duration,
}

impl TreasuryCache {
    /// Create new cache
    pub fn new() -> Self {
        Self {
            treasury_list: None,
            treasuries: HashMap::new(),
            default_ttl: Duration::from_secs(300), // 5 minutes
        }
    }
    
    /// Get cached treasury list
    pub fn get_treasury_list(&self) -> Option<Vec<TreasuryListItem>> {
        self.treasury_list
            .as_ref()
            .filter(|entry| !entry.is_expired())
            .map(|entry| entry.data.clone())
    }
    
    /// Set treasury list cache
    pub fn set_treasury_list(&mut self, list: Vec<TreasuryListItem>) {
        self.treasury_list = Some(CacheEntry::new(list, self.default_ttl));
    }
    
    /// Get cached treasury
    pub fn get_treasury(&self, address: &str) -> Option<TreasuryInfo> {
        self.treasuries
            .get(address)
            .filter(|entry| !entry.is_expired())
            .map(|entry| entry.data.clone())
    }
    
    /// Set treasury cache
    pub fn set_treasury(&mut self, address: String, treasury: TreasuryInfo) {
        self.treasuries
            .insert(address, CacheEntry::new(treasury, self.default_ttl));
    }
    
    /// Clear all cache
    pub fn clear(&mut self) {
        self.treasury_list = None;
        self.treasuries.clear();
    }
    
    /// Set custom TTL
    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = ttl;
        self
    }
}

impl Default for TreasuryCache {
    fn default() -> Self {
        Self::new()
    }
}
```

## CLI Command Implementation

```rust
// src/cli/treasury.rs (modified)

use clap::Subcommand;
use anyhow::Result;

use crate::config::ConfigManager;
use crate::oauth::OAuthClient;
use crate::treasury::TreasuryManager;
use crate::utils::output::{print_json, print_info, print_success};

#[derive(Subcommand)]
pub enum TreasuryCommands {
    /// List all treasury contracts for the authenticated user
    List {
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Query treasury contract details
    Query {
        /// Treasury contract address
        address: String,
        
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Create a new treasury contract
    Create {
        /// Fee grant configuration (e.g., "basic:1000000uxion")
        #[arg(short, long)]
        fee_grant: Option<String>,

        /// Grant configuration (e.g., "authz:cosmwasm.wasm.v1.MsgExecuteContract")
        #[arg(short, long)]
        grant_config: Option<String>,
        
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Fund a treasury contract
    Fund {
        /// Treasury contract address
        address: String,

        /// Amount to fund (e.g., "1000000uxion")
        #[arg(short, long)]
        amount: String,
        
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Withdraw funds from a treasury contract
    Withdraw {
        /// Treasury contract address
        address: String,

        /// Amount to withdraw (e.g., "1000000uxion")
        #[arg(short, long)]
        amount: String,
        
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },
}

pub async fn handle_command(cmd: TreasuryCommands) -> Result<()> {
    match cmd {
        TreasuryCommands::List { json } => handle_list(json).await,
        TreasuryCommands::Query { address, json } => handle_query(&address, json).await,
        TreasuryCommands::Create { fee_grant, grant_config, json } => {
            handle_create(fee_grant.as_deref(), grant_config.as_deref(), json).await
        }
        TreasuryCommands::Fund { address, amount, json } => handle_fund(&address, &amount, json).await,
        TreasuryCommands::Withdraw { address, amount, json } => handle_withdraw(&address, &amount, json).await,
    }
}

async fn get_treasury_manager() -> Result<TreasuryManager> {
    // Load config
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    
    // Create OAuth client
    let oauth_client = OAuthClient::new(network_config.clone())?;
    
    // Create Treasury manager
    let treasury_manager = TreasuryManager::new(
        oauth_client,
        network_config.oauth_api_url,
    );
    
    Ok(treasury_manager)
}

async fn handle_list(json: bool) -> Result<()> {
    if !json {
        print_info("Listing treasury contracts...");
    }
    
    let manager = get_treasury_manager().await?;
    let treasuries = manager.list().await?;
    
    let result = serde_json::json!({
        "success": true,
        "count": treasuries.len(),
        "treasuries": treasuries,
    });
    
    print_json(&result)?;
    
    if !json {
        print_success(&format!("Found {} treasury contract(s)", treasuries.len()));
    }
    
    Ok(())
}

async fn handle_query(address: &str, json: bool) -> Result<()> {
    if !json {
        print_info(&format!("Querying treasury: {}", address));
    }
    
    let manager = get_treasury_manager().await?;
    let treasury = manager.query(address).await?;
    
    let result = serde_json::json!({
        "success": true,
        "treasury": treasury,
    });
    
    print_json(&result)?;
    
    if !json {
        print_success(&format!(
            "Treasury balance: {} uxion",
            treasury.balance
        ));
    }
    
    Ok(())
}

async fn handle_create(fee_grant: Option<&str>, grant_config: Option<&str>, json: bool) -> Result<()> {
    if !json {
        print_info("Creating treasury contract...");
    }
    
    let manager = get_treasury_manager().await?;
    
    // Parse fee grant configuration
    let fee_grant_req = if let Some(fg) = fee_grant {
        Some(parse_fee_grant(fg)?)
    } else {
        None
    };
    
    // Parse grant configuration
    let grant_config_req = if let Some(gc) = grant_config {
        Some(parse_grant_config(gc)?)
    } else {
        None
    };
    
    let request = CreateTreasuryRequest {
        fee_grant: fee_grant_req,
        grant_config: grant_config_req,
        initial_fund: None,
    };
    
    let treasury = manager.create(request).await?;
    
    let result = serde_json::json!({
        "success": true,
        "treasury": treasury,
    });
    
    print_json(&result)?;
    
    if !json {
        print_success(&format!("Treasury created: {}", treasury.address));
    }
    
    Ok(())
}

async fn handle_fund(address: &str, amount: &str, json: bool) -> Result<()> {
    if !json {
        print_info(&format!("Funding treasury {} with {}", address, amount));
    }
    
    let manager = get_treasury_manager().await?;
    manager.fund(address, amount).await?;
    
    let result = serde_json::json!({
        "success": true,
        "message": format!("Successfully funded treasury {} with {}", address, amount),
        "address": address,
        "amount": amount,
    });
    
    print_json(&result)?;
    
    if !json {
        print_success(&format!("Treasury funded successfully"));
    }
    
    Ok(())
}

async fn handle_withdraw(address: &str, amount: &str, json: bool) -> Result<()> {
    // Similar implementation to fund
    // Will be implemented in future phases
    
    let result = serde_json::json!({
        "success": true,
        "message": "Treasury withdrawal not yet implemented",
        "address": address,
        "amount": amount
    });
    
    print_json(&result)
}

// Helper functions for parsing configurations
fn parse_fee_grant(input: &str) -> Result<FeeGrantRequest> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid fee grant format. Expected: type:amount (e.g., basic:1000000uxion)");
    }
    
    Ok(FeeGrantRequest {
        grant_type: parts[0].to_string(),
        spend_limit: parts[1].to_string(),
    })
}

fn parse_grant_config(input: &str) -> Result<GrantConfigRequest> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid grant config format. Expected: type:message_type (e.g., authz:cosmwasm.wasm.v1.MsgExecuteContract)");
    }
    
    Ok(GrantConfigRequest {
        type_url: parts[1].to_string(),
        config: serde_json::json!({}),
    })
}
```

## Error Handling

```rust
// src/utils/error.rs (extended)

#[derive(Debug, Error)]
pub enum XionError {
    // Existing errors...
    
    #[error("Treasury error: {0}")]
    Treasury(String),
    
    #[error("Treasury not found: {0}")]
    TreasuryNotFound(String),
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: String, available: String },
    
    #[error("Invalid treasury address: {0}")]
    InvalidTreasuryAddress(String),
    
    #[error("Treasury operation failed: {0}")]
    TreasuryOperationFailed(String),
}
```

## File Organization Summary

### New Files to Create

1. **`src/api/treasury_api.rs`**
   - Treasury API client implementation
   - HTTP request handling
   - API error handling

2. **`src/treasury/mod.rs`**
   - Module exports
   - Re-exports of types and manager

3. **`src/treasury/types.rs`**
   - Treasury data structures
   - Serialization/deserialization

4. **`src/treasury/manager.rs`**
   - High-level treasury management
   - OAuth2 integration
   - Cache integration

5. **`src/treasury/cache.rs`**
   - In-memory cache implementation
   - TTL support
   - Cache invalidation

### Files to Modify

1. **`src/api/mod.rs`**
   - Add `pub mod treasury_api;`

2. **`src/treasury.rs` (rename to `src/treasury/mod.rs`)**
   - Keep existing structure
   - Add new modules

3. **`src/cli/treasury.rs`**
   - Implement actual command handlers
   - Integrate with TreasuryManager
   - Add JSON output support

4. **`src/utils/error.rs`**
   - Add treasury-specific errors

5. **`Cargo.toml`**
   - No changes needed (all dependencies already present)

## Module Dependencies

```
┌─────────────────────┐
│  CLI (treasury.rs)  │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ TreasuryManager     │
│  (treasury/         │
│   manager.rs)       │
└──────────┬──────────┘
           │
    ┌──────┴──────┬───────────┐
    │             │           │
    ▼             ▼           ▼
┌─────────┐ ┌──────────┐ ┌──────────┐
│OAuth    │ │Treasury  │ │Treasury  │
│Client   │ │ApiClient │ │Cache     │
└─────────┘ └──────────┘ └──────────┘
                │
                ▼
         ┌─────────────┐
         │ Treasury    │
         │ Types       │
         └─────────────┘
```

## Security Considerations

1. **Token Management**
   - All API calls use OAuth2 access tokens
   - Tokens are automatically refreshed when expired
   - Tokens stored securely in OS keyring

2. **HTTPS Enforcement**
   - All API communication over HTTPS
   - Certificate validation enabled

3. **Input Validation**
   - Validate treasury addresses before API calls
   - Validate amount formats
   - Sanitize user inputs

4. **Error Sanitization**
   - Don't expose sensitive data in error messages
   - Log detailed errors, show user-friendly messages

5. **Cache Security**
   - In-memory cache only (no persistent storage)
   - Cache cleared on logout

## Performance Optimizations

1. **Caching**
   - 5-minute TTL for treasury data
   - Automatic cache invalidation
   - Optional cache bypass via force refresh

2. **Connection Pooling**
   - Reuse HTTP client connections
   - Configure connection timeouts

3. **Async Operations**
   - All API calls are async
   - Non-blocking CLI commands

## Testing Strategy

### Unit Tests

1. **Treasury API Client Tests**
   - Test request construction
   - Test response parsing
   - Test error handling

2. **Treasury Manager Tests**
   - Test token refresh integration
   - Test cache hit/miss scenarios
   - Test error propagation

3. **Cache Tests**
   - Test TTL expiration
   - Test cache invalidation
   - Test concurrent access

### Integration Tests

1. **API Integration**
   - Test against mock server
   - Test against testnet API

2. **CLI Integration**
   - Test command parsing
   - Test output formatting
   - Test error messages

## Future Enhancements

### Phase 3.5: Treasury Creation & Funding
- Implement `treasury create` command
- Implement `treasury fund` command
- Add multi-sig support

### Phase 4: Advanced Features
- Treasury analytics dashboard
- Batch operations
- Transaction history
- Grant management UI

### Phase 5: Extended Features
- Multi-network support
- Treasury templates
- Automated treasury setup
- Integration with deployment workflows

## CLI Usage Examples

### List Treasuries

```bash
# List all treasuries (human-readable)
xion-toolkit treasury list

# List all treasuries (JSON output)
xion-toolkit treasury list --json

# Example output (JSON)
{
  "success": true,
  "count": 2,
  "treasuries": [
    {
      "address": "xion1abc...",
      "admin": "xion1def...",
      "balance": "10000000",
      "name": "My Treasury"
    },
    {
      "address": "xion1xyz...",
      "admin": "xion1def...",
      "balance": "5000000",
      "name": "Another Treasury"
    }
  ]
}
```

### Query Treasury

```bash
# Query treasury details
xion-toolkit treasury query xion1abc...

# Query with JSON output
xion-toolkit treasury query xion1abc... --json

# Example output (JSON)
{
  "success": true,
  "treasury": {
    "address": "xion1abc...",
    "admin": "xion1def...",
    "balance": "10000000",
    "params": {
      "display_url": "https://myapp.com",
      "redirect_url": "https://myapp.com/callback",
      "icon_url": "https://myapp.com/icon.png"
    },
    "fee_config": {
      "type": "basic",
      "spend_limit": "10000000uxion"
    },
    "grant_configs": [
      {
        "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
        "grant_config": {}
      }
    ]
  }
}
```

### Create Treasury (Future)

```bash
# Create treasury with basic fee grant
xion-toolkit treasury create --fee-grant basic:1000000uxion

# Create treasury with authz grant
xion-toolkit treasury create \
  --fee-grant basic:1000000uxion \
  --grant-config authz:cosmwasm.wasm.v1.MsgExecuteContract
```

### Fund Treasury (Future)

```bash
# Fund treasury with 10 XION
xion-toolkit treasury fund xion1abc... --amount 10000000uxion
```

## Conclusion

This architecture provides a solid foundation for Treasury management in the Xion Agent Toolkit:

1. **Modular Design**: Clear separation of concerns with API client, manager, and cache layers
2. **OAuth2 Integration**: Seamless integration with existing authentication system
3. **Performance**: Built-in caching to reduce API calls
4. **Extensibility**: Easy to add new features and commands
5. **Type Safety**: Strong typing with serde for JSON serialization
6. **Error Handling**: Comprehensive error handling with helpful messages
7. **Testing**: Designed for testability with clear interfaces

The implementation should follow the phased approach:
- **Immediate**: Treasury list and query commands
- **Phase 3.5**: Treasury creation and funding
- **Future**: Advanced features and optimizations
