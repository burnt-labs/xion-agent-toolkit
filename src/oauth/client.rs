//! OAuth2 Client
//!
//! High-level OAuth2 client that orchestrates the complete authentication flow.
//! Integrates PKCE, callback server, token management, and credential storage.

use anyhow::{Context, Result};
use tracing::{debug, info, instrument};

use crate::api::oauth2_api::OAuth2ApiClient;
use crate::config::{CredentialsManager, NetworkConfig, UserCredentials};
use crate::oauth::{CallbackServer, PKCEChallenge, TokenManager};

/// OAuth2 Client
///
/// High-level client that manages the complete OAuth2 authentication lifecycle:
/// - Login flow (authorization code with PKCE)
/// - Token management (auto-refresh)
/// - Credential persistence
///
/// # Example
/// ```no_run
/// use xion_agent_toolkit::config::NetworkConfig;
/// use xion_agent_toolkit::oauth::OAuthClient;
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let config = NetworkConfig {
///     network_name: "testnet".to_string(),
///     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
///     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
///     chain_id: "xion-testnet-2".to_string(),
///     oauth_client_id: "your-client-id".to_string(),
///     treasury_code_id: 1260,
///     callback_port: 54321,
///     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
/// };
///
/// let client = OAuthClient::new(config)?;
///
/// // Login
/// let credentials = client.login().await?;
/// println!("Logged in as: {:?}", credentials.xion_address);
///
/// // Get valid token (auto-refresh if needed)
/// let token = client.get_valid_token().await?;
/// println!("Access token: {}", token);
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct OAuthClient {
    /// Network configuration
    network_config: NetworkConfig,
    /// OAuth2 API client for token operations
    api_client: OAuth2ApiClient,
    /// Token manager for lifecycle management
    token_manager: TokenManager,
    /// Credentials manager for persistence
    credentials_manager: CredentialsManager,
}

impl OAuthClient {
    /// Create a new OAuth2 client
    ///
    /// # Arguments
    /// * `network_config` - Network configuration containing OAuth2 endpoints
    ///
    /// # Returns
    /// A new OAuthClient instance ready for authentication
    ///
    /// # Errors
    /// Returns an error if credential manager initialization fails
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::config::NetworkConfig;
    /// use xion_agent_toolkit::oauth::OAuthClient;
    ///
    /// let config = NetworkConfig {
    ///     network_name: "testnet".to_string(),
    ///     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    ///     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    ///     chain_id: "xion-testnet-2".to_string(),
    ///     oauth_client_id: "client-id".to_string(),
    ///     treasury_code_id: 1260,
    ///     callback_port: 54321,
    ///     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    /// };
    ///
    /// let client = OAuthClient::new(config)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn new(network_config: NetworkConfig) -> Result<Self> {
        debug!(
            "Creating OAuth2 client for network: {}",
            network_config.chain_id
        );

        // Create OAuth2 API client
        let api_client = OAuth2ApiClient::new(network_config.oauth_api_url.clone());

        // Create credentials manager
        let credentials_manager = CredentialsManager::new(&network_config.network_name)
            .context("Failed to create credentials manager")?;

        // Create token manager
        let token_manager = TokenManager::new(
            credentials_manager.clone(),
            api_client.clone(),
            network_config.oauth_client_id.clone(),
        );

        Ok(Self {
            network_config,
            api_client,
            token_manager,
            credentials_manager,
        })
    }

    /// Create a new OAuth2 client with a custom credentials manager (for testing).
    ///
    /// This allows tests to use isolated credential storage without affecting real credentials.
    #[cfg(test)]
    pub fn with_credentials_manager(
        network_config: NetworkConfig,
        credentials_manager: CredentialsManager,
    ) -> Result<Self> {
        debug!(
            "Creating OAuth2 client for network: {}",
            network_config.chain_id
        );

        // Create OAuth2 API client
        let api_client = OAuth2ApiClient::new(network_config.oauth_api_url.clone());

        // Create token manager with the provided credentials manager
        let token_manager = TokenManager::new(
            credentials_manager.clone(),
            api_client.clone(),
            network_config.oauth_client_id.clone(),
        );

        Ok(Self {
            network_config,
            api_client,
            token_manager,
            credentials_manager,
        })
    }

    /// Execute full OAuth2 login flow
    ///
    /// This method orchestrates the complete OAuth2 authorization code flow with PKCE:
    /// 1. Generate PKCE challenge (verifier + challenge + state)
    /// 2. Build authorization URL
    /// 3. Start localhost callback server
    /// 4. Open browser for user authorization
    /// 5. Wait for callback (with timeout)
    /// 6. Exchange authorization code for tokens
    /// 7. Save credentials to secure storage
    ///
    /// # Returns
    /// User credentials including access token, refresh token, and Xion address
    ///
    /// # Errors
    /// Returns an error if:
    /// - PKCE generation fails
    /// - Callback server fails to start
    /// - Browser fails to open
    /// - User denies authorization or timeout
    /// - Token exchange fails
    /// - Credential storage fails
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::OAuthClient;
    /// # use xion_agent_toolkit::config::NetworkConfig;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     network_name: "testnet".to_string(),
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: 1260,
    /// #     callback_port: 54321,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    /// # };
    /// let client = OAuthClient::new(config)?;
    /// let credentials = client.login().await?;
    /// println!("Logged in successfully!");
    /// println!("Xion address: {:?}", credentials.xion_address);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn login(&self) -> Result<UserCredentials> {
        info!("Starting OAuth2 login flow");

        // Step 0: Fetch OAuth2 endpoints from discovery document
        info!("Fetching OAuth2 endpoints from discovery endpoint");
        let oauth_endpoints = crate::config::get_oauth2_endpoints(
            &self.network_config.network_name,
            &self.network_config.oauth_api_url,
        )
        .await
        .context("Failed to fetch OAuth2 endpoints")?;

        info!(
            "Using authorization endpoint: {}",
            oauth_endpoints.authorization_endpoint
        );
        info!("Using token endpoint: {}", oauth_endpoints.token_endpoint);

        // Step 1: Generate PKCE challenge
        debug!("Generating PKCE challenge");
        let pkce = PKCEChallenge::generate().context("Failed to generate PKCE challenge")?;

        // Step 2: Build redirect URI (using 127.0.0.1 instead of localhost for consistency)
        let redirect_uri = format!(
            "http://127.0.0.1:{}/callback",
            self.network_config.callback_port
        );
        debug!("Redirect URI: {}", redirect_uri);

        // Step 3: Build authorization URL
        let auth_url = self.build_authorization_url(
            &pkce,
            &redirect_uri,
            &oauth_endpoints.authorization_endpoint,
        );
        info!("Authorization URL: {}", auth_url);

        // Step 4: Start callback server
        debug!(
            "Starting callback server on port {}",
            self.network_config.callback_port
        );
        let callback_server = CallbackServer::new(self.network_config.callback_port);

        // Step 5: Open browser
        info!("Opening browser for authorization...");
        self.open_browser(&auth_url)
            .context("Failed to open browser")?;

        // Step 6: Wait for callback (5 minute timeout)
        debug!("Waiting for OAuth callback...");
        let code = callback_server
            .wait_for_code(&pkce.state, 300)
            .await
            .context("Failed to receive OAuth callback")?;

        info!("Received authorization code");

        // Step 7: Exchange code for tokens
        debug!("Exchanging authorization code for tokens");
        let token_response = self
            .api_client
            .exchange_code_with_endpoint(
                &code,
                &pkce.verifier,
                &redirect_uri,
                &self.network_config.oauth_client_id,
                &oauth_endpoints.token_endpoint,
            )
            .await
            .context("Failed to exchange authorization code for tokens")?;

        info!("Successfully obtained tokens");

        // Step 8: Get user info to obtain xion_address
        debug!("Fetching user info to get MetaAccount address");
        let user_info = self
            .api_client
            .get_user_info(&token_response.access_token)
            .await
            .context("Failed to get user info")?;

        info!("Retrieved MetaAccount address: {}", user_info.id);

        // Step 9: Save credentials with xion_address
        let expires_at = match &token_response.expires_at {
            Some(expires_at) => expires_at.clone(),
            None => token_response.calculate_expires_at(),
        };

        // Calculate refresh token expiration (default 30 days if not provided by server)
        let refresh_token_expires_at = match &token_response.refresh_token_expires_at {
            Some(expires_at) => Some(expires_at.clone()),
            None => Some(token_response.calculate_refresh_token_expires_at()),
        };

        let credentials = UserCredentials {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_at,
            refresh_token_expires_at,
            xion_address: Some(user_info.id),
        };

        self.credentials_manager
            .save_credentials(&credentials)
            .context("Failed to save credentials")?;

        info!("Login completed successfully");
        Ok(credentials)
    }

    /// Logout (clear credentials)
    ///
    /// Removes all stored credentials for the current network.
    ///
    /// # Returns
    /// `Ok(())` if credentials were successfully removed
    ///
    /// # Errors
    /// Returns an error if credential deletion fails
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::OAuthClient;
    /// # use xion_agent_toolkit::config::NetworkConfig;
    /// # fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     network_name: "testnet".to_string(),
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: 1260,
    /// #     callback_port: 54321,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    /// # };
    /// let client = OAuthClient::new(config)?;
    /// client.logout()?;
    /// println!("Logged out successfully");
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub fn logout(&self) -> Result<()> {
        info!("Logging out");
        self.credentials_manager
            .clear_credentials()
            .context("Failed to clear credentials")?;
        info!("Logged out successfully");
        Ok(())
    }

    /// Check if authenticated
    ///
    /// Verifies whether valid credentials exist for the current network.
    ///
    /// # Returns
    /// `true` if credentials exist, `false` otherwise
    ///
    /// # Errors
    /// Returns an error if credential check fails
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::OAuthClient;
    /// # use xion_agent_toolkit::config::NetworkConfig;
    /// # fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     network_name: "testnet".to_string(),
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: 1260,
    /// #     callback_port: 54321,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    /// # };
    /// let client = OAuthClient::new(config)?;
    /// if client.is_authenticated()? {
    ///     println!("User is authenticated");
    /// } else {
    ///     println!("User is not authenticated");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub fn is_authenticated(&self) -> Result<bool> {
        debug!("Checking authentication status");
        self.credentials_manager
            .has_credentials()
            .context("Failed to check authentication status")
    }

    /// Get current credentials
    ///
    /// Loads the stored credentials without validation.
    ///
    /// # Returns
    /// `Some(credentials)` if credentials exist, `None` otherwise
    ///
    /// # Errors
    /// Returns an error if credential loading fails
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::OAuthClient;
    /// # use xion_agent_toolkit::config::NetworkConfig;
    /// # fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     network_name: "testnet".to_string(),
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: 1260,
    /// #     callback_port: 54321,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    /// # };
    /// let client = OAuthClient::new(config)?;
    /// if let Some(creds) = client.get_credentials()? {
    ///     println!("Xion address: {:?}", creds.xion_address);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub fn get_credentials(&self) -> Result<Option<UserCredentials>> {
        debug!("Getting current credentials");
        match self.credentials_manager.load_credentials() {
            Ok(creds) => Ok(Some(creds)),
            Err(_) => Ok(None),
        }
    }

    /// Get valid access token (auto-refresh if needed)
    ///
    /// Returns a valid access token, automatically refreshing it if expired.
    /// This is the primary method for obtaining tokens for API calls.
    ///
    /// # Returns
    /// Valid access token ready for use
    ///
    /// # Errors
    /// Returns an error if:
    /// - No credentials found
    /// - Token refresh fails
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::OAuthClient;
    /// # use xion_agent_toolkit::config::NetworkConfig;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     network_name: "testnet".to_string(),
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: 1260,
    /// #     callback_port: 54321,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    /// # };
    /// let client = OAuthClient::new(config)?;
    /// let token = client.get_valid_token().await?;
    /// println!("Access token: {}", token);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_valid_token(&self) -> Result<String> {
        debug!("Getting valid access token");
        self.token_manager
            .get_valid_token()
            .await
            .context("Failed to get valid access token")
    }

    /// Force refresh token
    ///
    /// Forces a token refresh even if the current token is still valid.
    /// Useful when you suspect the token may have been revoked.
    ///
    /// # Returns
    /// New credentials with fresh access token
    ///
    /// # Errors
    /// Returns an error if:
    /// - No credentials found
    /// - Refresh token is invalid or revoked
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::OAuthClient;
    /// # use xion_agent_toolkit::config::NetworkConfig;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     network_name: "testnet".to_string(),
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: 1260,
    /// #     callback_port: 54321,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    /// # };
    /// let client = OAuthClient::new(config)?;
    /// let new_creds = client.refresh_token().await?;
    /// println!("New token expires at: {}", new_creds.expires_at);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn refresh_token(&self) -> Result<UserCredentials> {
        info!("Forcing token refresh");
        self.token_manager
            .refresh_access_token()
            .await
            .context("Failed to refresh token")
    }

    /// Build authorization URL
    ///
    /// Constructs the OAuth2 authorization URL with PKCE parameters.
    ///
    /// # Arguments
    /// * `pkce` - PKCE challenge containing verifier, challenge, and state
    /// * `redirect_uri` - Callback URL for receiving the authorization code
    /// * `authorization_endpoint` - OAuth2 authorization endpoint URL
    ///
    /// # Returns
    /// Fully constructed authorization URL
    fn build_authorization_url(
        &self,
        pkce: &PKCEChallenge,
        redirect_uri: &str,
        authorization_endpoint: &str,
    ) -> String {
        format!(
            "{}?\
             response_type=code&\
             client_id={}&\
             redirect_uri={}&\
             code_challenge={}&\
             code_challenge_method=S256&\
             state={}",
            authorization_endpoint,
            urlencoding::encode(&self.network_config.oauth_client_id),
            urlencoding::encode(redirect_uri),
            pkce.challenge,
            pkce.state
        )
    }

    /// Open browser for authorization
    ///
    /// Opens the system's default browser to the authorization URL.
    ///
    /// # Arguments
    /// * `url` - Authorization URL to open
    ///
    /// # Errors
    /// Returns an error if the browser fails to open
    fn open_browser(&self, url: &str) -> Result<()> {
        debug!("Opening browser to: {}", url);
        open::that(url).context("Failed to open browser")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_config() -> NetworkConfig {
        NetworkConfig {
            network_name: "testnet".to_string(),
            oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
            rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
            chain_id: "xion-testnet-2".to_string(),
            oauth_client_id: "test-client-id".to_string(),
            treasury_code_id: 1260,
            callback_port: 54321,
            indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
        }
    }

    /// Create isolated test environment with temp directory
    fn create_isolated_test_env() -> (TempDir, String) {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let test_key =
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string();
        (temp_dir, test_key)
    }

    #[test]
    fn test_is_authenticated_without_credentials() {
        let (temp_dir, test_key) = create_isolated_test_env();
        let original_key = std::env::var(crate::config::encryption::ENV_KEY_NAME).ok();
        std::env::set_var(crate::config::encryption::ENV_KEY_NAME, &test_key);

        let config = create_test_config();
        // Use isolated credentials manager
        let creds_manager = crate::config::CredentialsManager::with_config_dir(
            "testnet",
            temp_dir.path().to_path_buf(),
        )
        .expect("Failed to create credentials manager");
        let client = OAuthClient::with_credentials_manager(config, creds_manager).unwrap();

        // Should not be authenticated initially (no credentials in isolated dir)
        let is_auth = client.is_authenticated().unwrap();
        assert!(!is_auth);

        // Restore env
        if let Some(key) = original_key {
            std::env::set_var(crate::config::encryption::ENV_KEY_NAME, key);
        } else {
            std::env::remove_var(crate::config::encryption::ENV_KEY_NAME);
        }
    }

    #[test]
    fn test_get_credentials_without_credentials() {
        let (temp_dir, test_key) = create_isolated_test_env();
        let original_key = std::env::var(crate::config::encryption::ENV_KEY_NAME).ok();
        std::env::set_var(crate::config::encryption::ENV_KEY_NAME, &test_key);

        let config = create_test_config();
        // Use isolated credentials manager
        let creds_manager = crate::config::CredentialsManager::with_config_dir(
            "testnet",
            temp_dir.path().to_path_buf(),
        )
        .expect("Failed to create credentials manager");
        let client = OAuthClient::with_credentials_manager(config, creds_manager).unwrap();

        // Should return None when no credentials (isolated dir)
        let creds = client.get_credentials().unwrap();
        assert!(creds.is_none());

        // Restore env
        if let Some(key) = original_key {
            std::env::set_var(crate::config::encryption::ENV_KEY_NAME, key);
        } else {
            std::env::remove_var(crate::config::encryption::ENV_KEY_NAME);
        }
    }

    #[test]
    fn test_logout_without_credentials() {
        let (temp_dir, test_key) = create_isolated_test_env();
        let original_key = std::env::var(crate::config::encryption::ENV_KEY_NAME).ok();
        std::env::set_var(crate::config::encryption::ENV_KEY_NAME, &test_key);

        let config = create_test_config();
        // Use isolated credentials manager
        let creds_manager = crate::config::CredentialsManager::with_config_dir(
            "testnet",
            temp_dir.path().to_path_buf(),
        )
        .expect("Failed to create credentials manager");
        let client = OAuthClient::with_credentials_manager(config, creds_manager).unwrap();

        // Should succeed even without credentials (isolated dir - won't affect real credentials)
        let result = client.logout();
        assert!(result.is_ok());

        // Restore env
        if let Some(key) = original_key {
            std::env::set_var(crate::config::encryption::ENV_KEY_NAME, key);
        } else {
            std::env::remove_var(crate::config::encryption::ENV_KEY_NAME);
        }
    }

    #[test]
    fn test_client_debug() {
        let config = create_test_config();
        let client = OAuthClient::new(config).unwrap();

        // Should implement Debug
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("OAuthClient"));
    }

    #[test]
    fn test_authorization_url_construction() {
        let config = create_test_config();
        let client = OAuthClient::new(config).unwrap();

        let pkce = PKCEChallenge::generate().unwrap();
        let redirect_uri = "http://127.0.0.1:54321/callback";
        let auth_endpoint = "https://oauth2.testnet.burnt.com/oauth/authorize";
        let auth_url = client.build_authorization_url(&pkce, redirect_uri, auth_endpoint);

        // Verify URL structure
        assert!(auth_url.starts_with("https://oauth2.testnet.burnt.com/oauth/authorize"));
        assert!(auth_url.contains("response_type=code"));
        assert!(auth_url.contains("client_id=test-client-id"));
        assert!(auth_url.contains("redirect_uri=http%3A%2F%2F127.0.0.1%3A54321%2Fcallback"));
        assert!(auth_url.contains(&format!("code_challenge={}", pkce.challenge)));
        assert!(auth_url.contains("code_challenge_method=S256"));
        assert!(auth_url.contains(&format!("state={}", pkce.state)));
    }

    #[test]
    fn test_authorization_url_encoding() {
        let config = create_test_config();
        let client = OAuthClient::new(config).unwrap();

        let pkce = PKCEChallenge::generate().unwrap();
        let redirect_uri = "http://localhost:54321/callback";
        let auth_endpoint = "https://oauth2.testnet.burnt.com/oauth/authorize";
        let auth_url = client.build_authorization_url(&pkce, redirect_uri, auth_endpoint);

        // Verify URL encoding
        assert!(auth_url.contains("redirect_uri=http%3A%2F%2Flocalhost%3A54321%2Fcallback"));
    }
}
