# OAuth2 Client Architecture Design

## System Overview

The OAuth2 client in the Xion Agent Toolkit implements the OAuth2 Authorization Code Flow with PKCE. It supports browser-based login, a local callback server, token management, and multiple network environments.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Layer (auth.rs)                     │
│  - Handle login/logout/status/refresh commands              │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                   OAuth Client (client.rs)                   │
│  - Orchestrate OAuth2 flow                                  │
│  - Coordinate PKCE, callback server, token manager          │
└────────┬─────────────────────────────────────────────────────┘
         │                    │                    │
         ▼                    ▼                    ▼
┌──────────────┐   ┌──────────────────┐   ┌─────────────────┐
│   PKCE       │   │ Callback Server  │   │ Token Manager   │
│  (pkce.rs)   │   │(callback_server) │   │(token_manager)  │
│              │   │                  │   │                 │
│- Verifier    │   │- Axum server     │   │- Refresh token  │
│- Challenge   │   │- Code receiver   │   │- Expiry check   │
│- State       │   │- State validate  │   │- Auto-refresh   │
└──────────────┘   └──────────────────┘   └─────────────────┘
         │                    │                    │
         └────────────────────┴────────────────────┘
                              │
                              ▼
              ┌───────────────────────────────┐
              │   OAuth2 API Client           │
              │   (api/oauth2_api.rs)         │
              │                               │
              │   - Exchange code for tokens  │
              │   - Refresh token             │
              │   - Get user info             │
              └───────────────────────────────┘
                              │
                              ▼
              ┌───────────────────────────────┐
              │  Credentials Manager          │
              │  (config/credentials.rs)      │
              │                               │
              │  - Keyring storage            │
              │  - File metadata              │
              └───────────────────────────────┘
```

## Module Breakdown

### 1. OAuth2 Core Module (`src/oauth/`)

#### 1.1 `mod.rs`
**Responsibility**: Module exports and public API

```rust
pub mod pkce;
pub mod client;
pub mod callback_server;
pub mod token_manager;

pub use client::OAuthClient;
pub use pkce::PKCEChallenge;
pub use token_manager::TokenManager;
```

#### 1.2 `pkce.rs`
**Responsibility**: PKCE implementation (RFC 7636)

**Key struct**:
```rust
pub struct PKCEChallenge {
    pub verifier: String,
    pub challenge: String,
    pub state: String,
}

impl PKCEChallenge {
    /// Generate a new PKCE challenge
    /// - Verifier: 43 characters, cryptographically random
    /// - Challenge: SHA-256(verifier), Base64URL encoded
    /// - State: 32 bytes random, hex encoded
    pub fn generate() -> Result<Self>;
    
    /// Verify if a state matches
    pub fn verify_state(&self, state: &str) -> bool;
}
```

#### 1.3 `client.rs`
**Responsibility**: Main OAuth2 client logic, orchestrating the full login flow

**Key struct**:
```rust
pub struct OAuthClient {
    config: NetworkConfig,
    http_client: reqwest::Client,
    credentials_manager: CredentialsManager,
}

impl OAuthClient {
    pub fn new(config: NetworkConfig) -> Result<Self>;
    
    /// Execute full OAuth2 login flow
    pub async fn login(&self, port: u16) -> Result<UserCredentials>;
    
    /// Refresh access token
    pub async fn refresh_token(&self) -> Result<UserCredentials>;
    
    /// Logout (clear credentials)
    pub fn logout(&self) -> Result<()>;
    
    /// Check if authenticated
    pub fn is_authenticated(&self) -> Result<bool>;
}
```

#### 1.4 `callback_server.rs`
**Responsibility**: Local callback server to receive OAuth2 redirects

**Key struct**:
```rust
pub struct CallbackServer {
    port: u16,
}

impl CallbackServer {
    pub fn new(port: u16) -> Self;
    
    /// Start server and wait for OAuth2 callback
    pub async fn wait_for_code(self, expected_state: &str) -> Result<String>;
}
```

#### 1.5 `token_manager.rs`
**Responsibility**: Token lifecycle management (refresh, expiry checks)

**Key struct**:
```rust
pub struct TokenManager {
    credentials_manager: CredentialsManager,
    http_client: reqwest::Client,
    oauth_api_url: String,
}

impl TokenManager {
    pub fn new(credentials_manager: CredentialsManager, oauth_api_url: String) -> Self;
    
    /// Get valid access token (refresh if needed)
    pub async fn get_valid_token(&self) -> Result<String>;
    
    /// Refresh access token
    pub async fn refresh_access_token(&self) -> Result<UserCredentials>;
}
```

### 2. OAuth2 API Client Module (`src/api/oauth2_api.rs`)

**Responsibility**: Communicate with the OAuth2 API Service

**Key struct**:
```rust
pub struct OAuth2ApiClient {
    base_url: String,
    http_client: reqwest::Client,
}

impl OAuth2ApiClient {
    pub fn new(base_url: String) -> Self;
    
    /// Exchange authorization code for tokens
    pub async fn exchange_code(
        &self,
        code: &str,
        code_verifier: &str,
        redirect_uri: &str,
        client_id: &str,
    ) -> Result<TokenResponse>;
    
    /// Refresh access token
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
        client_id: &str,
    ) -> Result<TokenResponse>;
}
```

## File Creation Checklist

New files to create:

1. `src/oauth/mod.rs` - Module exports
2. `src/oauth/pkce.rs` - PKCE implementation
3. `src/oauth/client.rs` - Main OAuth2 client logic
4. `src/oauth/callback_server.rs` - Axum-based callback server
5. `src/oauth/token_manager.rs` - Token manager
6. `src/api/mod.rs` - API module exports
7. `src/api/oauth2_api.rs` - OAuth2 API client

Files to modify:

1. `src/main.rs` - Add `oauth` and `api` modules
2. `src/cli/auth.rs` - Implement `handle_login` and `handle_refresh`
3. `src/utils/error.rs` - Extend error types

## Development Plan

### Phase 2.1: Infrastructure (Priority: P0)
- [ ] Create module structure (`oauth`, `api`)
- [ ] Implement PKCE (`pkce.rs`)
- [ ] Implement OAuth2 API client (`oauth2_api.rs`)
- [ ] Extend error handling (`error.rs`)

### Phase 2.2: Core Flow (Priority: P0)
- [ ] Implement callback server (`callback_server.rs`)
- [ ] Implement OAuth2 client orchestration (`client.rs`)
- [ ] Implement token manager (`token_manager.rs`)

### Phase 2.3: Integration (Priority: P0)
- [ ] Integrate CLI commands (`auth.rs`)
- [ ] Testing and validation

---
*Created by: @architect*
*Date: 2025-03-05*
