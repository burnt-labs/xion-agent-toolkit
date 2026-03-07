//! Treasury Manager
//!
//! High-level manager for treasury operations with automatic token refresh
//! and caching support.

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, instrument};

use crate::config::NetworkConfig;
use crate::oauth::OAuthClient;

use super::api_client::TreasuryApiClient;
use super::cache::TreasuryCache;
use super::types::{
    FundResult, QueryOptions, TreasuryInfo, TreasuryListItem, TreasuryParams, WithdrawResult,
};

/// Treasury Manager
///
/// High-level manager for treasury operations that integrates:
/// - OAuth2 authentication with automatic token refresh
/// - Treasury API client for making requests
/// - In-memory caching to reduce API calls
///
/// # Example
/// ```no_run
/// use xion_agent_toolkit::config::NetworkConfig;
/// use xion_agent_toolkit::oauth::OAuthClient;
/// use xion_agent_toolkit::treasury::TreasuryManager;
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// # let config = NetworkConfig {
/// #     network_name: "testnet".to_string(),
/// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
/// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
/// #     chain_id: "xion-testnet-2".to_string(),
/// #     oauth_client_id: "client-id".to_string(),
/// #     treasury_code_id: Some(1260),
/// #     treasury_config: Some("xion1...".to_string()),
/// #     callback_port: 54321,
/// # };
/// let oauth_client = OAuthClient::new(config.clone())?;
/// let manager = TreasuryManager::new(oauth_client, config.clone());
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
    /// Network configuration
    config: NetworkConfig,
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
    /// # use xion_agent_toolkit::oauth::OAuthClient;
    /// # use xion_agent_toolkit::config::NetworkConfig;
    /// # use xion_agent_toolkit::treasury::TreasuryManager;
    /// # fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     network_name: "testnet".to_string(),
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.clone());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(oauth_client: OAuthClient, config: NetworkConfig) -> Self {
        let api_client = TreasuryApiClient::new(config.oauth_api_url.clone());
        let cache = Some(Arc::new(RwLock::new(TreasuryCache::new())));

        Self {
            oauth_client,
            api_client,
            config,
            cache,
        }
    }

    /// Create Treasury manager without caching
    ///
    /// Disables caching for all operations. Useful when you always need fresh data.
    #[allow(dead_code)]
    pub fn without_cache(oauth_client: OAuthClient, config: NetworkConfig) -> Self {
        let api_client = TreasuryApiClient::new(config.oauth_api_url.clone());

        Self {
            oauth_client,
            api_client,
            config,
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
    /// # use xion_agent_toolkit::treasury::TreasuryManager;
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
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.clone());
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
    /// # use xion_agent_toolkit::treasury::TreasuryManager;
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
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.clone());
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
        let treasury = self
            .api_client
            .query_treasury(&token, address, options)
            .await?;

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
    /// # use xion_agent_toolkit::treasury::TreasuryManager;
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
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.clone());
    /// let balance = manager.get_balance("xion1abc...").await?;
    /// println!("Balance: {} uxion", balance);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(dead_code)]
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
    /// # use xion_agent_toolkit::treasury::TreasuryManager;
    /// # use xion_agent_toolkit::oauth::OAuthClient;
    /// # use xion_agent_toolkit::config::NetworkConfig;
    /// # fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     network_name: "testnet".to_string(),
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.clone());
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
    #[allow(dead_code)]
    pub async fn clear_cache(&self) {
        if let Some(cache) = &self.cache {
            let mut cache_write = cache.write().await;
            cache_write.clear();
            debug!("Cleared treasury cache");
        }
    }

    /// Create new treasury
    ///
    /// Creates a new treasury contract with the specified parameters.
    /// This method encodes the user input into the proper chain format
    /// and submits a transaction to instantiate the treasury contract.
    ///
    /// # Arguments
    /// * `request` - Treasury creation request with parameters, fee config, and grant configs
    ///
    /// # Returns
    /// Treasury information for the newly created treasury
    ///
    /// # Errors
    /// Returns an error if:
    /// - Not authenticated
    /// - Token refresh fails
    /// - Encoding fails
    /// - API request fails
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::treasury::TreasuryManager;
    /// # use xion_agent_toolkit::treasury::types::{TreasuryCreateRequest, TreasuryParamsInput, FeeConfigInput};
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
    /// #     treasury_code_id: Some(1260),
    /// #     treasury_config: Some("xion1...".to_string()),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.clone());
    ///
    /// let request = TreasuryCreateRequest {
    ///     params: TreasuryParamsInput {
    ///         redirect_url: "https://myapp.com/callback".to_string(),
    ///         icon_url: "https://myapp.com/icon.png".to_string(),
    ///         name: Some("My Treasury".to_string()),
    ///         is_oauth2_app: Some(true),
    ///     },
    ///     fee_config: Some(FeeConfigInput::Basic {
    ///         spend_limit: "1000000uxion".to_string(),
    ///         description: "Basic fee allowance".to_string(),
    ///     }),
    ///     grant_configs: vec![],
    /// };
    ///
    /// let treasury = manager.create(request).await?;
    /// println!("Created treasury: {}", treasury.address);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, request))]
    pub async fn create(
        &self,
        request: super::types::TreasuryCreateRequest,
    ) -> Result<TreasuryInfo> {
        debug!("Creating treasury");

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let admin_address = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Step 1: Encode params metadata
        let metadata = serde_json::json!({
            "name": request.params.name.clone().unwrap_or_default(),
            "archived": false,
            "is_oauth2_app": request.params.is_oauth2_app.unwrap_or(false),
        });

        // Step 2: Encode fee config (if provided)
        let fee_config = if let Some(fee_input) = &request.fee_config {
            Some(encode_fee_config_input(fee_input)?)
        } else {
            None
        };

        // Step 3: Encode grant configs
        let (type_urls, grant_configs): (Vec<_>, Vec<_>) = request
            .grant_configs
            .iter()
            .map(encode_grant_config_input)
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .unzip();

        // Step 4: Build the instantiate message
        let instantiate_msg = super::types::TreasuryInstantiateMsg {
            admin: admin_address.clone(),
            params: super::types::TreasuryParamsChain {
                redirect_url: request.params.redirect_url,
                icon_url: request.params.icon_url,
                metadata: metadata.to_string(),
            },
            fee_config,
            grant_configs,
            type_urls,
        };

        // Step 5: Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Step 6: Get treasury code ID from config
        let code_id = self
            .config
            .treasury_code_id
            .ok_or_else(|| anyhow::anyhow!("Treasury code ID not configured for this network"))?;

        // Step 7: Generate random salt for instantiate2
        let salt: [u8; 32] = {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            rng.gen()
        };

        // Step 8: Call API to create treasury
        let create_request = super::types::CreateTreasuryRequest {
            admin: admin_address.clone(),
            fee_config: super::types::FeeConfigMessage {
                allowance: super::types::TypeUrlValue {
                    type_url: instantiate_msg
                        .fee_config
                        .as_ref()
                        .map(|f| f.allowance.type_url.clone())
                        .unwrap_or_default(),
                    value: instantiate_msg
                        .fee_config
                        .as_ref()
                        .map(|f| f.allowance.value.clone())
                        .unwrap_or_default(),
                },
                description: instantiate_msg
                    .fee_config
                    .as_ref()
                    .map(|f| f.description.clone())
                    .unwrap_or_default(),
            },
            grant_configs: instantiate_msg
                .grant_configs
                .iter()
                .map(|gc| super::types::GrantConfigMessage {
                    authorization: super::types::TypeUrlValue {
                        type_url: gc.authorization.type_url.clone(),
                        value: gc.authorization.value.clone(),
                    },
                    description: Some(gc.description.clone()),
                })
                .collect(),
            params: super::types::TreasuryParamsMessage {
                redirect_url: instantiate_msg.params.redirect_url.clone(),
                icon_url: instantiate_msg.params.icon_url.clone(),
                display_url: None,
                metadata: Some(
                    serde_json::from_str(&instantiate_msg.params.metadata).unwrap_or_default(),
                ),
            },
            name: request.params.name.clone(),
            is_oauth2_app: request.params.is_oauth2_app.unwrap_or(false),
        };

        let result = self
            .api_client
            .create_treasury(&access_token, code_id, create_request, &salt)
            .await?;

        // Step 9: Return treasury info
        Ok(TreasuryInfo {
            address: result.treasury_address,
            admin: Some(result.admin),
            balance: "0".to_string(),
            params: TreasuryParams {
                display_url: None,
                redirect_url: instantiate_msg.params.redirect_url,
                icon_url: instantiate_msg.params.icon_url,
                metadata: Some(
                    serde_json::from_str(&instantiate_msg.params.metadata).unwrap_or_default(),
                ),
            },
            fee_config: None,
            grant_configs: None,
        })
    }

    /// Fund treasury
    ///
    /// Funds a treasury contract by sending tokens to it.
    /// This creates a MsgSend transaction from the user's MetaAccount to the treasury.
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    /// * `amount` - Amount to fund (e.g., "1000000uxion")
    ///
    /// # Returns
    /// Fund result with transaction hash
    #[instrument(skip(self))]
    pub async fn fund(&self, address: &str, amount: &str) -> Result<FundResult> {
        debug!("Funding treasury {} with {}", address, amount);

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let from_address = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to fund treasury
        let broadcast_response = self
            .api_client
            .fund_treasury(&access_token, address, amount, &from_address)
            .await?;

        // Convert BroadcastResponse to FundResult
        Ok(FundResult {
            treasury_address: address.to_string(),
            amount: amount.to_string(),
            tx_hash: broadcast_response.tx_hash,
        })
    }

    /// Withdraw from treasury
    ///
    /// Withdraws tokens from a treasury contract to the admin's wallet.
    /// This creates a MsgExecuteContract transaction calling the Withdraw message.
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    /// * `amount` - Amount to withdraw (e.g., "1000000uxion")
    ///
    /// # Returns
    /// Withdraw result with transaction hash
    #[instrument(skip(self))]
    pub async fn withdraw(&self, address: &str, amount: &str) -> Result<WithdrawResult> {
        debug!("Withdrawing {} from treasury {}", amount, address);

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let from_address = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to withdraw from treasury
        let broadcast_response = self
            .api_client
            .withdraw_treasury(&access_token, address, amount, &from_address)
            .await?;

        // Convert BroadcastResponse to WithdrawResult
        Ok(WithdrawResult {
            treasury_address: address.to_string(),
            amount: amount.to_string(),
            tx_hash: broadcast_response.tx_hash,
        })
    }

    // ========================================================================
    // Grant Config Operations
    // ========================================================================

    /// Add a grant configuration to a treasury
    ///
    /// Adds or updates a grant configuration for the specified message type.
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    /// * `grant_config` - Grant configuration input
    ///
    /// # Returns
    /// Grant config result with transaction hash
    #[instrument(skip(self))]
    pub async fn add_grant_config(
        &self,
        address: &str,
        grant_config: super::types::GrantConfigInput,
    ) -> Result<super::types::GrantConfigResult> {
        debug!("Adding grant config to treasury {}", address);

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let from_address = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Clone type_url before moving grant_config
        let type_url = grant_config.type_url.clone();

        // Call API client to add grant config
        self.api_client
            .add_grant_config(
                &access_token,
                address,
                &type_url,
                grant_config,
                &from_address,
            )
            .await
    }

    /// Remove a grant configuration from a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    /// * `type_url` - Type URL of the grant to remove
    ///
    /// # Returns
    /// Grant config result with transaction hash
    #[instrument(skip(self))]
    pub async fn remove_grant_config(
        &self,
        address: &str,
        type_url: &str,
    ) -> Result<super::types::GrantConfigResult> {
        debug!(
            "Removing grant config {} from treasury {}",
            type_url, address
        );

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let from_address = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to remove grant config
        self.api_client
            .remove_grant_config(&access_token, address, type_url, &from_address)
            .await
    }

    /// List all grant configurations for a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    ///
    /// # Returns
    /// List of grant configurations
    #[instrument(skip(self))]
    pub async fn list_grant_configs(
        &self,
        address: &str,
    ) -> Result<Vec<super::types::GrantConfigInfo>> {
        debug!("Listing grant configs for treasury {}", address);

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to list grant configs
        self.api_client
            .list_grant_configs(&access_token, address)
            .await
    }

    // ========================================================================
    // Fee Config Operations
    // ========================================================================

    /// Set fee configuration for a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    /// * `fee_config` - Fee configuration input
    ///
    /// # Returns
    /// Fee config result with transaction hash
    #[instrument(skip(self))]
    pub async fn set_fee_config(
        &self,
        address: &str,
        fee_config: super::types::FeeConfigInput,
    ) -> Result<super::types::FeeConfigResult> {
        debug!("Setting fee config for treasury {}", address);

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let from_address = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to set fee config
        self.api_client
            .set_fee_config(&access_token, address, fee_config, &from_address)
            .await
    }

    /// Remove fee configuration from a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    ///
    /// # Returns
    /// Fee config result with transaction hash
    #[instrument(skip(self))]
    pub async fn remove_fee_config(&self, address: &str) -> Result<super::types::FeeConfigResult> {
        debug!("Removing fee config from treasury {}", address);

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let from_address = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to remove fee config
        self.api_client
            .remove_fee_config(&access_token, address, &from_address)
            .await
    }

    /// Query fee configuration for a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    ///
    /// # Returns
    /// Fee config info if set, None otherwise
    #[instrument(skip(self))]
    pub async fn query_fee_config(
        &self,
        address: &str,
    ) -> Result<Option<super::types::FeeConfigInfo>> {
        debug!("Querying fee config for treasury {}", address);

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to query fee config
        self.api_client
            .query_fee_config(&access_token, address)
            .await
    }
}

// ============================================================================
// Helper Functions for Encoding
// ============================================================================

/// Encode fee config input to chain format
fn encode_fee_config_input(
    input: &super::types::FeeConfigInput,
) -> Result<super::types::FeeConfigChain> {
    use super::encoding::{
        encode_allowed_msg_allowance, encode_basic_allowance, encode_periodic_allowance,
        parse_coin_string,
    };

    let (allowance_type_url, allowance_value) = match input {
        super::types::FeeConfigInput::Basic {
            spend_limit,
            description: _,
        } => {
            let coins = parse_coin_string(spend_limit)?;
            let encoded = encode_basic_allowance(coins)?;
            (
                "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
                encoded,
            )
        }
        super::types::FeeConfigInput::Periodic {
            basic_spend_limit,
            period_seconds,
            period_spend_limit,
            description: _,
        } => {
            let basic = if let Some(limit) = basic_spend_limit {
                Some(parse_coin_string(limit)?)
            } else {
                None
            };
            let period_limit = parse_coin_string(period_spend_limit)?;
            let encoded = encode_periodic_allowance(basic, *period_seconds, period_limit)?;
            (
                "/cosmos.feegrant.v1beta1.PeriodicAllowance".to_string(),
                encoded,
            )
        }
        super::types::FeeConfigInput::AllowedMsg {
            allowed_messages,
            nested_allowance,
            description: _,
        } => {
            // Recursive encoding
            let nested = encode_fee_config_input(nested_allowance)?;
            let encoded = encode_allowed_msg_allowance(
                allowed_messages.clone(),
                &nested.allowance.type_url,
                &nested.allowance.value,
            )?;
            (
                "/cosmos.feegrant.v1beta1.AllowedMsgAllowance".to_string(),
                encoded,
            )
        }
    };

    let description = match input {
        super::types::FeeConfigInput::Basic { description, .. } => description.clone(),
        super::types::FeeConfigInput::Periodic { description, .. } => description.clone(),
        super::types::FeeConfigInput::AllowedMsg { description, .. } => description.clone(),
    };

    Ok(super::types::FeeConfigChain {
        description,
        allowance: super::types::ProtobufAny {
            type_url: allowance_type_url,
            value: allowance_value,
        },
    })
}

/// Encode grant config input to chain format
fn encode_grant_config_input(
    input: &super::types::GrantConfigInput,
) -> Result<(String, super::types::GrantConfigChain)> {
    use super::encoding::{
        encode_contract_execution_authorization, encode_generic_authorization,
        encode_ibc_transfer_authorization, encode_send_authorization, encode_stake_authorization,
        parse_coin_string, parse_single_denom, ContractGrant, IbcAllocation,
    };

    let (auth_type_url, auth_value) = match &input.authorization {
        super::types::AuthorizationInput::Generic => {
            let encoded = encode_generic_authorization(&input.type_url)?;
            (
                "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                encoded,
            )
        }
        super::types::AuthorizationInput::Send {
            spend_limit,
            allow_list,
        } => {
            let coins = parse_coin_string(spend_limit)?;
            let encoded = encode_send_authorization(coins, allow_list.clone())?;
            (
                "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                encoded,
            )
        }
        super::types::AuthorizationInput::Stake {
            max_tokens,
            validators,
            deny_validators,
            authorization_type,
        } => {
            let coin = parse_single_denom(max_tokens)?;
            let encoded = encode_stake_authorization(
                coin,
                validators.clone(),
                deny_validators.clone(),
                *authorization_type,
            )?;
            (
                "/cosmos.staking.v1beta1.StakeAuthorization".to_string(),
                encoded,
            )
        }
        super::types::AuthorizationInput::IbcTransfer { allocations } => {
            let ibc_allocations: Vec<IbcAllocation> = allocations
                .iter()
                .map(|a| {
                    Ok(IbcAllocation {
                        source_port: a.source_port.clone(),
                        source_channel: a.source_channel.clone(),
                        spend_limit: parse_coin_string(&a.spend_limit)?,
                        allow_list: a.allow_list.clone(),
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let encoded = encode_ibc_transfer_authorization(ibc_allocations)?;
            (
                "/ibc.applications.transfer.v1.TransferAuthorization".to_string(),
                encoded,
            )
        }
        super::types::AuthorizationInput::ContractExecution { grants } => {
            let contract_grants: Vec<ContractGrant> = grants
                .iter()
                .map(|g| {
                    Ok(ContractGrant {
                        address: g.address.clone(),
                        max_calls: g.max_calls,
                        max_funds: if let Some(funds) = &g.max_funds {
                            Some(parse_coin_string(funds)?)
                        } else {
                            None
                        },
                        filter_type: g.filter_type.clone(),
                        keys: g.keys.clone(),
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let encoded = encode_contract_execution_authorization(contract_grants)?;
            (
                "/cosmwasm.wasm.v1.ContractExecutionAuthorization".to_string(),
                encoded,
            )
        }
    };

    Ok((
        input.type_url.clone(),
        super::types::GrantConfigChain {
            description: input.description.clone(),
            authorization: super::types::ProtobufAny {
                type_url: auth_type_url,
                value: auth_value,
            },
            optional: input.optional,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::NetworkConfig;

    fn create_test_config() -> NetworkConfig {
        NetworkConfig {
            network_name: "testnet".to_string(),
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
        let manager = TreasuryManager::new(oauth_client, config.clone());
        assert!(manager.cache.is_some());
    }

    #[test]
    fn test_manager_without_cache() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::without_cache(oauth_client, config.clone());
        assert!(manager.cache.is_none());
    }

    #[test]
    fn test_is_authenticated_without_credentials() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.clone());

        // Should not be authenticated initially
        let is_auth = manager.is_authenticated().unwrap();
        assert!(!is_auth);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.clone());

        // Should not panic when clearing cache
        manager.clear_cache().await;
    }

    #[tokio::test]
    async fn test_create_requires_auth() {
        use crate::treasury::types::{FeeConfigInput, TreasuryCreateRequest, TreasuryParamsInput};

        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.clone());

        let request = TreasuryCreateRequest {
            params: TreasuryParamsInput {
                redirect_url: "https://example.com/callback".to_string(),
                icon_url: "https://example.com/icon.png".to_string(),
                name: None,
                is_oauth2_app: None,
            },
            fee_config: Some(FeeConfigInput::Basic {
                spend_limit: "1000000uxion".to_string(),
                description: "Test fee config".to_string(),
            }),
            grant_configs: vec![],
        };

        let result = manager.create(request).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Not authenticated"));
    }

    #[tokio::test]
    async fn test_fund_requires_auth() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.clone());

        let result = manager.fund("xion1abc", "1000uxion").await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Not authenticated"));
    }

    #[tokio::test]
    async fn test_withdraw_requires_auth() {
        let config = create_test_config();
        let oauth_client = OAuthClient::new(config.clone()).unwrap();
        let manager = TreasuryManager::new(oauth_client, config.clone());

        let result = manager.withdraw("xion1abc", "1000uxion").await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Not authenticated"));
    }
}
