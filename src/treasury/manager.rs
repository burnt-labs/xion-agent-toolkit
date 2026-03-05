//! Treasury Manager
//!
//! High-level manager for treasury operations with automatic token refresh
//! and caching support.

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, instrument};

use crate::oauth::OAuthClient;

use super::api_client::TreasuryApiClient;
use super::cache::TreasuryCache;
use super::types::{CreateTreasuryRequest, QueryOptions, TreasuryInfo, TreasuryListItem};

/// Treasury Manager
///
/// High-level manager for treasury operations that integrates:
/// - OAuth2 authentication with automatic token refresh
/// - Treasury API client for making requests
/// - In-memory caching to reduce API calls
///
/// # Example
/// ```no_run
/// use xion_agent_cli::config::NetworkConfig;
/// use xion_agent_cli::oauth::OAuthClient;
/// use xion_agent_cli::treasury::TreasuryManager;
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// # let config = NetworkConfig {
/// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
/// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
/// #     chain_id: "xion-testnet-2".to_string(),
/// #     oauth_client_id: "client-id".to_string(),
/// #     treasury_code_id: Some(1260),
/// #     treasury_config: Some("xion1...".to_string()),
/// #     callback_port: 54321,
/// # };
/// let oauth_client = OAuthClient::new(config.clone())?;
/// let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);
///
/// // List treasuries
/// let treasuries = manager.list().await?;
/// println!("Found {} treasuries", treasuries.len());
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct TreasuryManager {
    /// OAuth client for token management
    oauth_client: OAuthClient,
    /// Treasury API client
    api_client: TreasuryApiClient,
    /// Optional cache (wrapped in Arc<RwLock> for thread-safe async access)
    cache: Option<Arc<RwLock<TreasuryCache>>>,
}

impl TreasuryManager {
    /// Create new Treasury manager
    ///
    /// # Arguments
    /// * `oauth_client` - OAuth client for authentication
    /// * `api_base_url` - Base URL of the Treasury API service
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_cli::oauth::OAuthClient;
    /// # use xion_agent_cli::config::NetworkConfig;
    /// # use xion_agent_cli::treasury::TreasuryManager;
    /// # fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(oauth_client: OAuthClient, api_base_url: String) -> Self {
        let api_client = TreasuryApiClient::new(api_base_url);
        let cache = Some(Arc::new(RwLock::new(TreasuryCache::new())));

        Self {
            oauth_client,
            api_client,
            cache,
        }
    }

    /// Create Treasury manager without caching
    ///
    /// Disables caching for all operations. Useful when you always need fresh data.
    pub fn without_cache(oauth_client: OAuthClient, api_base_url: String) -> Self {
        let api_client = TreasuryApiClient::new(api_base_url);

        Self {
            oauth_client,
            api_client,
            cache: None,
        }
    }

    /// List all treasuries for the authenticated user
    ///
    /// Retrieves a list of all treasury contracts owned by the authenticated user.
    /// Uses caching to reduce API calls if cache is enabled.
    ///
    /// # Returns
    /// Vector of treasury list items with basic information
    ///
    /// # Errors
    /// Returns an error if:
    /// - Not authenticated
    /// - Token refresh fails
    /// - API request fails
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_cli::treasury::TreasuryManager;
    /// # use xion_agent_cli::oauth::OAuthClient;
    /// # use xion_agent_cli::config::NetworkConfig;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);
    /// let treasuries = manager.list().await?;
    /// for treasury in treasuries {
    ///     println!("Treasury: {} - Balance: {}", treasury.address, treasury.balance);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn list(&self) -> Result<Vec<TreasuryListItem>> {
        debug!("Listing treasuries");

        // Check cache first
        if let Some(cache) = &self.cache {
            let cache_read = cache.read().await;
            if let Some(cached) = cache_read.get_treasury_list() {
                debug!("Returning cached treasury list");
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
            debug!("Cached treasury list");
        }

        debug!("Retrieved {} treasuries", treasuries.len());
        Ok(treasuries)
    }

    /// Query specific treasury details
    ///
    /// Retrieves detailed information about a specific treasury contract.
    /// Uses caching to reduce API calls if cache is enabled.
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    ///
    /// # Returns
    /// Complete treasury information including balance, parameters, and configurations
    ///
    /// # Errors
    /// Returns an error if:
    /// - Not authenticated
    /// - Token refresh fails
    /// - Treasury not found
    /// - API request fails
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_cli::treasury::TreasuryManager;
    /// # use xion_agent_cli::oauth::OAuthClient;
    /// # use xion_agent_cli::config::NetworkConfig;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);
    /// let treasury = manager.query("xion1abc...").await?;
    /// println!("Balance: {} uxion", treasury.balance);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn query(&self, address: &str) -> Result<TreasuryInfo> {
        debug!("Querying treasury: {}", address);

        // Check cache first
        if let Some(cache) = &self.cache {
            let cache_read = cache.read().await;
            if let Some(cached) = cache_read.get_treasury(address) {
                debug!("Returning cached treasury: {}", address);
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
            debug!("Cached treasury: {}", address);
        }

        debug!("Successfully queried treasury: {}", address);
        Ok(treasury)
    }

    /// Get treasury balance
    ///
    /// Convenience method to retrieve just the balance of a treasury.
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    ///
    /// # Returns
    /// Treasury balance in uxion as a string
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_cli::treasury::TreasuryManager;
    /// # use xion_agent_cli::oauth::OAuthClient;
    /// # use xion_agent_cli::config::NetworkConfig;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);
    /// let balance = manager.get_balance("xion1abc...").await?;
    /// println!("Balance: {} uxion", balance);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_balance(&self, address: &str) -> Result<String> {
        debug!("Getting balance for treasury: {}", address);

        let treasury = self.query(address).await?;
        Ok(treasury.balance)
    }

    /// Check if user is authenticated
    ///
    /// Verifies whether valid credentials exist for the current network.
    ///
    /// # Returns
    /// `true` if credentials exist, `false` otherwise
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_cli::treasury::TreasuryManager;
    /// # use xion_agent_cli::oauth::OAuthClient;
    /// # use xion_agent_cli::config::NetworkConfig;
    /// # fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);
    /// if manager.is_authenticated()? {
    ///     println!("User is authenticated");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_authenticated(&self) -> Result<bool> {
        self.oauth_client.is_authenticated()
    }

    /// Clear cache
    ///
    /// Clears all cached treasury data. Useful when you need fresh data
    /// or when the user logs out.
    pub async fn clear_cache(&self) {
        if let Some(cache) = &self.cache {
            let mut cache_write = cache.write().await;
            cache_write.clear();
            debug!("Cleared treasury cache");
        }
    }

    /// Create new treasury (future implementation)
    ///
    /// This method is a placeholder for future treasury creation functionality.
    #[instrument(skip(self))]
    pub async fn create(&self, _request: CreateTreasuryRequest) -> Result<TreasuryInfo> {
        anyhow::bail!("Treasury creation not yet implemented. Please use the Developer Portal to create Treasury contracts.");
    }

    /// Fund treasury (future implementation)
    ///
    /// This method is a placeholder for future treasury funding functionality.
    #[instrument(skip(self))]
    pub async fn fund(&self, _address: &str, _amount: &str) -> Result<()> {
        anyhow::bail!("Treasury funding not yet implemented. Please use xiond CLI to fund Treasury contracts.");
    }

    /// Withdraw from treasury (future implementation)
    ///
    /// This method is a placeholder for future treasury withdrawal functionality.
    #[instrument(skip(self))]
    pub async fn withdraw(&self, _address: &str, _amount: &str) -> Result<()> {
        anyhow::bail!("Treasury withdrawal not yet implemented. Please use xiond CLI to withdraw from Treasury contracts.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::NetworkConfig;

    fn create_test_config() -> NetworkConfig {
        NetworkConfig {
            oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
            rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
            chain_id: "xion-testnet-2".to_string(),
            oauth_client_id: "test-client-id".to_string(),
            treasury_code_id: Some(1260),
            treasury_config: Some("xion1test".to_string()),
            callback_port: 54321,
        }
    }

    #[test]
    fn test_manager_creation() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);
        assert!(manager.cache.is_some());
    }

    #[test]
    fn test_manager_without_cache() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::without_cache(oauth_client, config.oauth_api_url);
        assert!(manager.cache.is_none());
    }

    #[test]
    fn test_is_authenticated_without_credentials() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);

        // Should not be authenticated initially
        let is_auth = manager.is_authenticated().unwrap();
        assert!(!is_auth);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);

        // Should not panic when clearing cache
        manager.clear_cache().await;
    }

    #[tokio::test]
    async fn test_create_not_implemented() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);

        let request = CreateTreasuryRequest {
            fee_grant: None,
            grant_config: None,
            initial_fund: None,
        };

        let result = manager.create(request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not yet implemented"));
    }

    #[tokio::test]
    async fn test_fund_not_implemented() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);

        let result = manager.fund("xion1abc", "1000uxion").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not yet implemented"));
    }

    #[tokio::test]
    async fn test_withdraw_not_implemented() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.oauth_api_url);

        let result = manager.withdraw("xion1abc", "1000uxion").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not yet implemented"));
    }
}
