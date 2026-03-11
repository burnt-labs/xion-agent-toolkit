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
    ExecuteResult, FundResult, Instantiate2Result, InstantiateResult, QueryOptions, TreasuryInfo,
    TreasuryListItem, TreasuryParams, WithdrawResult,
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
/// #     treasury_code_id: 1260,
/// #     callback_port: 54321,
/// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
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
    /// #     treasury_code_id: 1260,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    /// #     callback_port: 54321,
    /// # };
    /// let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.clone());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(oauth_client: OAuthClient, config: NetworkConfig) -> Self {
        let api_client = TreasuryApiClient::new(
            config.oauth_api_url.clone(),
            config.indexer_url.clone(),
            config.rpc_url.clone(),
        );
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
        let api_client = TreasuryApiClient::new(
            config.oauth_api_url.clone(),
            config.indexer_url.clone(),
            config.rpc_url.clone(),
        );

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
    /// #     treasury_code_id: 1260,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
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
    /// #     treasury_code_id: 1260,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
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
    /// #     treasury_code_id: 1260,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
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
    /// #     treasury_code_id: 1260,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
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
    /// #     treasury_code_id: 1260,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
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
        let code_id = self.config.treasury_code_id;

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
                        .and_then(|f| f.allowance.as_ref().map(|a| a.type_url.clone()))
                        .unwrap_or_default(),
                    value: instantiate_msg
                        .fee_config
                        .as_ref()
                        .and_then(|f| f.allowance.as_ref().map(|a| a.value.clone()))
                        .unwrap_or_default(),
                },
                description: instantiate_msg
                    .fee_config
                    .as_ref()
                    .map(|f| f.description.clone())
                    .unwrap_or_default(),
                expiration: instantiate_msg
                    .fee_config
                    .as_ref()
                    .and_then(|f| f.expiration.clone()),
            },
            grant_configs: instantiate_msg
                .type_urls
                .iter()
                .zip(instantiate_msg.grant_configs.iter())
                .map(|(type_url, gc)| super::types::GrantConfigMessage {
                    type_url: type_url.clone(),
                    authorization: super::types::TypeUrlValue {
                        type_url: gc.authorization.type_url.clone(),
                        value: gc.authorization.value.clone(),
                    },
                    description: Some(gc.description.clone()),
                    optional: gc.optional,
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

    /// Instantiate a generic contract (v1 - dynamic address)
    ///
    /// Instantiates a new contract instance from a code ID with a dynamically assigned address.
    ///
    /// Use this when you don't need a predictable contract address.
    /// For predictable addresses, use `instantiate_contract2` instead.
    ///
    /// # Arguments
    /// * `code_id` - Code ID of the contract to instantiate
    /// * `instantiate_msg` - Instantiate message (any serializable type)
    /// * `label` - Label for the contract instance
    /// * `admin` - Optional admin address for contract migrations
    ///
    /// # Returns
    /// Instantiate result with transaction hash
    #[instrument(skip(self, instantiate_msg))]
    pub async fn instantiate_contract<T: serde::Serialize + std::fmt::Debug>(
        &self,
        code_id: u64,
        instantiate_msg: &T,
        label: &str,
        admin: Option<&str>,
    ) -> Result<InstantiateResult> {
        debug!(
            "Instantiating contract code_id={} with label={}",
            code_id, label
        );

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let sender = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to broadcast instantiate
        let tx_hash = self
            .api_client
            .broadcast_instantiate_contract(
                &access_token,
                &sender,
                code_id,
                instantiate_msg,
                label,
                admin,
                "Instantiate contract via Xion Agent Toolkit",
            )
            .await?;

        Ok(InstantiateResult {
            tx_hash,
            code_id,
            label: label.to_string(),
            admin: admin.map(|s| s.to_string()),
        })
    }

    /// Instantiate a generic contract (v2 - predictable address)
    ///
    /// Instantiates a new contract instance with a predictable address using instantiate2.
    ///
    /// # Arguments
    /// * `code_id` - Code ID of the contract to instantiate
    /// * `instantiate_msg` - Instantiate message (any serializable type)
    /// * `label` - Label for the contract instance
    /// * `salt` - Optional salt for predictable address. If None, a cryptographically
    ///   random 32-byte salt is generated. Provide your own salt if you need address
    ///   predictability or reproducibility across deployments.
    /// * `admin` - Optional admin address for contract migrations
    ///
    /// # Returns
    /// Instantiate2 result with transaction hash and salt
    #[instrument(skip(self, instantiate_msg))]
    pub async fn instantiate_contract2<T: serde::Serialize + std::fmt::Debug>(
        &self,
        code_id: u64,
        instantiate_msg: &T,
        label: &str,
        salt: Option<&[u8]>,
        admin: Option<&str>,
    ) -> Result<Instantiate2Result> {
        debug!(
            "Instantiating contract2 code_id={} with label={}",
            code_id, label
        );

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let sender = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Generate salt if not provided (32 bytes)
        let salt_bytes = salt.map(|s| s.to_vec()).unwrap_or_else(|| {
            use rand::RngCore;
            let mut buf = vec![0u8; 32];
            rand::thread_rng().fill_bytes(&mut buf);
            buf
        });

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to broadcast instantiate2
        let tx_hash = self
            .api_client
            .broadcast_instantiate_contract2(
                &access_token,
                &sender,
                code_id,
                instantiate_msg,
                label,
                &salt_bytes,
                admin,
                "Instantiate contract2 via Xion Agent Toolkit",
            )
            .await?;

        Ok(Instantiate2Result {
            tx_hash,
            code_id,
            label: label.to_string(),
            salt: hex::encode(&salt_bytes),
            admin: admin.map(|s| s.to_string()),
            predicted_address: None, // TODO: compute if needed
        })
    }

    /// Execute a message on a smart contract
    ///
    /// Executes a message on a deployed smart contract.
    ///
    /// # Arguments
    /// * `contract` - Contract address to execute
    /// * `execute_msg` - Execute message (any serializable type)
    /// * `funds` - Optional funds to send with the execution (e.g., "1000000uxion")
    ///
    /// # Returns
    /// Execute result with transaction hash
    #[instrument(skip(self, execute_msg))]
    pub async fn execute_contract<T: serde::Serialize + std::fmt::Debug>(
        &self,
        contract: &str,
        execute_msg: &T,
        funds: Option<&str>,
    ) -> Result<ExecuteResult> {
        debug!("Executing message on contract: {}", contract);

        // Get user credentials to obtain xion_address
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("Not authenticated. Please login first."))?;

        let sender = credentials.xion_address.ok_or_else(|| {
            anyhow::anyhow!("User address not found in credentials. Please login again.")
        })?;

        // Parse funds if provided
        let funds_coins = if let Some(f) = funds {
            let encoding_coins = super::encoding::parse_coin_string(f)?;
            Some(
                encoding_coins
                    .into_iter()
                    .map(|c| super::types::Coin {
                        amount: c.amount,
                        denom: c.denom,
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        };

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to broadcast execute
        let tx_hash = self
            .api_client
            .broadcast_execute_contract(
                &access_token,
                &sender,
                contract,
                execute_msg,
                funds_coins.as_deref(),
                "Execute contract via Xion Agent Toolkit",
            )
            .await?;

        Ok(ExecuteResult {
            tx_hash,
            contract: contract.to_string(),
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

    /// Remove fee configuration from a treasury by revoking the allowance
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    /// * `grantee` - Address of the grantee to revoke allowance from
    ///
    /// # Returns
    /// Fee config result with transaction hash
    #[instrument(skip(self))]
    pub async fn remove_fee_config(
        &self,
        address: &str,
        grantee: &str,
    ) -> Result<super::types::FeeConfigResult> {
        debug!(
            "Removing fee allowance for grantee {} from treasury {}",
            grantee, address
        );

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Extract user address from OAuth2 access token
        let from_address = Self::extract_address_from_token(&access_token)?;

        // Call API client to revoke allowance
        self.api_client
            .revoke_allowance(&access_token, address, grantee, &from_address)
            .await
    }

    /// Extract address from OAuth2 access token
    fn extract_address_from_token(token: &str) -> Result<String> {
        // Token format: {userId}:{grantId}:{secret}
        let parts: Vec<&str> = token.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Invalid token format"));
        }
        Ok(parts[0].to_string())
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

    // ========================================================================
    // Admin Management Operations
    // ========================================================================

    /// Propose a new admin for a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    /// * `new_admin` - New admin address to propose
    ///
    /// # Returns
    /// Admin result with transaction hash
    #[instrument(skip(self))]
    pub async fn propose_admin(
        &self,
        address: &str,
        new_admin: &str,
    ) -> Result<super::types::AdminResult> {
        debug!("Proposing new admin {} for treasury {}", new_admin, address);

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

        // Call API client to propose admin
        self.api_client
            .propose_admin(&access_token, address, new_admin, &from_address)
            .await
    }

    /// Accept admin role for a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    ///
    /// # Returns
    /// Admin result with transaction hash
    #[instrument(skip(self))]
    pub async fn accept_admin(&self, address: &str) -> Result<super::types::AdminResult> {
        debug!("Accepting admin role for treasury {}", address);

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

        // Call API client to accept admin
        self.api_client
            .accept_admin(&access_token, address, &from_address)
            .await
    }

    /// Cancel proposed admin for a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    ///
    /// # Returns
    /// Admin result with transaction hash
    #[instrument(skip(self))]
    pub async fn cancel_proposed_admin(&self, address: &str) -> Result<super::types::AdminResult> {
        debug!("Canceling proposed admin for treasury {}", address);

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

        // Call API client to cancel proposed admin
        self.api_client
            .cancel_proposed_admin(&access_token, address, &from_address)
            .await
    }

    // ========================================================================
    // Params Management Operations
    // ========================================================================

    /// Update treasury parameters
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    /// * `params` - New parameters to set
    ///
    /// # Returns
    /// Params result with transaction hash
    #[instrument(skip(self))]
    pub async fn update_params(
        &self,
        address: &str,
        params: super::types::UpdateParamsInput,
    ) -> Result<super::types::ParamsResult> {
        debug!("Updating params for treasury {}", address);

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

        // Call API client to update params
        self.api_client
            .update_params(&access_token, address, params, &from_address)
            .await
    }

    // ========================================================================
    // Batch Operations
    // ========================================================================

    /// Add multiple grant configurations in a single transaction
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    /// * `grant_configs` - List of grant configurations to add
    ///
    /// # Returns
    /// Batch grant config result with transaction hash
    #[allow(dead_code)]
    #[instrument(skip(self, grant_configs))]
    pub async fn grant_config_batch(
        &self,
        address: &str,
        grant_configs: Vec<(String, super::types::GrantConfigInput)>,
    ) -> Result<super::types::BatchGrantConfigResult> {
        debug!(
            "Adding {} grant configs in batch to treasury {}",
            grant_configs.len(),
            address
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

        // Call API client to add grant configs in batch
        self.api_client
            .grant_config_batch(&access_token, address, grant_configs, &from_address)
            .await
    }

    // ========================================================================
    // On-Chain Query Operations
    // ========================================================================

    /// List all authz grants for a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address (granter)
    ///
    /// # Returns
    /// List of authz grants
    #[instrument(skip(self))]
    pub async fn list_authz_grants(
        &self,
        address: &str,
    ) -> Result<Vec<super::types::AuthzGrantInfo>> {
        debug!("Listing authz grants for treasury {}", address);

        // Call API client to list authz grants (no auth required for query)
        self.api_client.list_authz_grants(address).await
    }

    /// List all fee allowances for a treasury
    ///
    /// # Arguments
    /// * `address` - Treasury contract address (granter)
    ///
    /// # Returns
    /// List of fee allowances
    #[instrument(skip(self))]
    pub async fn list_fee_allowances(
        &self,
        address: &str,
    ) -> Result<Vec<super::types::FeeAllowanceInfo>> {
        debug!("Listing fee allowances for treasury {}", address);

        // Call API client to list fee allowances (no auth required for query)
        self.api_client.list_fee_allowances(address).await
    }

    // ========================================================================
    // Export Operations
    // ========================================================================

    /// Export treasury configuration for backup/migration
    ///
    /// Exports all treasury configuration data including admin, fee config,
    /// grant configs, and params. This is a read-only operation.
    ///
    /// # Arguments
    /// * `address` - Treasury contract address
    ///
    /// # Returns
    /// Treasury export data with all configuration
    #[instrument(skip(self))]
    pub async fn export_treasury(&self, address: &str) -> Result<super::types::TreasuryExportData> {
        debug!("Exporting treasury configuration for: {}", address);

        // Get valid access token
        let access_token = self.oauth_client.get_valid_token().await?;

        // Call API client to export treasury state
        self.api_client
            .export_treasury_state(&access_token, address)
            .await
    }

    /// Import treasury configuration to an existing treasury
    ///
    /// Imports configuration by executing a sequence of transactions:
    /// 1. Update fee config (if present)
    /// 2. Update grant configs (for each config)
    ///
    /// This is a client-side batching operation, NOT an on-chain batch.
    ///
    /// # Arguments
    /// * `treasury_address` - Target treasury contract address
    /// * `import_data` - Configuration to import
    /// * `dry_run` - If true, only preview actions without executing
    ///
    /// # Returns
    /// Import result with actions performed
    #[instrument(skip(self, import_data))]
    pub async fn import_treasury(
        &self,
        treasury_address: &str,
        import_data: &super::types::TreasuryExportData,
        dry_run: bool,
    ) -> Result<super::types::ImportResult> {
        debug!(
            "Importing configuration to treasury: {} (dry_run={})",
            treasury_address, dry_run
        );

        let mut actions = Vec::new();
        let mut errors = Vec::new();

        // Step 1: Update fee config if present
        if let Some(ref fee_config) = import_data.fee_config {
            eprintln!("[INFO] Planning fee config update...");

            let action = if dry_run {
                // Dry-run: just record the planned action
                super::types::ImportAction {
                    action_type: "update_fee_config".to_string(),
                    index: None,
                    success: true,
                    tx_hash: None,
                    error: None,
                    config: Some(serde_json::to_value(fee_config)?),
                }
            } else {
                // Execute: convert FeeConfigInfo to FeeConfigInput and update
                eprintln!("[INFO] Updating fee config...");
                match self.import_fee_config(treasury_address, fee_config).await {
                    Ok(tx_hash) => {
                        eprintln!("[INFO] Fee config updated: tx_hash={}", tx_hash);
                        super::types::ImportAction {
                            action_type: "update_fee_config".to_string(),
                            index: None,
                            success: true,
                            tx_hash: Some(tx_hash),
                            error: None,
                            config: None,
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to update fee config: {}", e);
                        eprintln!("[ERROR] {}", error_msg);
                        errors.push(error_msg.clone());
                        super::types::ImportAction {
                            action_type: "update_fee_config".to_string(),
                            index: None,
                            success: false,
                            tx_hash: None,
                            error: Some(error_msg),
                            config: None,
                        }
                    }
                }
            };
            actions.push(action);
        }

        // Step 2: Update grant configs
        for (i, grant_config) in import_data.grant_configs.iter().enumerate() {
            eprintln!(
                "[INFO] Planning grant config update {}/{}...",
                i + 1,
                import_data.grant_configs.len()
            );

            let action = if dry_run {
                // Dry-run: just record the planned action
                super::types::ImportAction {
                    action_type: "update_grant_config".to_string(),
                    index: Some(i),
                    success: true,
                    tx_hash: None,
                    error: None,
                    config: Some(serde_json::to_value(grant_config)?),
                }
            } else {
                // Execute: convert GrantConfigInfo to GrantConfigInput and update
                eprintln!(
                    "[INFO] Updating grant config {}/{}...",
                    i + 1,
                    import_data.grant_configs.len()
                );
                match self
                    .import_grant_config(treasury_address, grant_config)
                    .await
                {
                    Ok(tx_hash) => {
                        eprintln!("[INFO] Grant config updated: tx_hash={}", tx_hash);
                        super::types::ImportAction {
                            action_type: "update_grant_config".to_string(),
                            index: Some(i),
                            success: true,
                            tx_hash: Some(tx_hash),
                            error: None,
                            config: None,
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to update grant config {}: {}", i, e);
                        eprintln!("[ERROR] {}", error_msg);
                        errors.push(error_msg.clone());
                        super::types::ImportAction {
                            action_type: "update_grant_config".to_string(),
                            index: Some(i),
                            success: false,
                            tx_hash: None,
                            error: Some(error_msg),
                            config: None,
                        }
                    }
                }
            };
            actions.push(action);
        }

        // Calculate total successful transactions
        let total_transactions = actions
            .iter()
            .filter(|a| a.success && a.tx_hash.is_some())
            .count();

        if dry_run {
            eprintln!("[INFO] Dry-run complete: {} actions planned", actions.len());
        } else {
            eprintln!(
                "[INFO] Import complete: {} transactions executed",
                total_transactions
            );
        }

        Ok(super::types::ImportResult {
            success: errors.is_empty(),
            treasury_address: treasury_address.to_string(),
            dry_run,
            actions,
            total_transactions,
            errors,
        })
    }

    /// Import fee config to treasury
    ///
    /// Converts FeeConfigInfo to FeeConfigInput and calls set_fee_config.
    /// Preserves periodic allowance configuration if present in the export data.
    async fn import_fee_config(
        &self,
        treasury_address: &str,
        fee_config: &super::types::FeeConfigInfo,
    ) -> Result<String> {
        // Determine the fee config type based on available fields
        let fee_input = if let (Some(period), Some(period_spend_limit)) =
            (&fee_config.period, &fee_config.period_spend_limit)
        {
            // Periodic allowance - parse period duration
            let period_seconds = parse_duration_string(period)?;
            super::types::FeeConfigInput::Periodic {
                basic_spend_limit: fee_config.spend_limit.clone(),
                period_seconds,
                period_spend_limit: period_spend_limit.clone(),
                description: fee_config.description.clone(),
            }
        } else {
            // Basic allowance
            super::types::FeeConfigInput::Basic {
                spend_limit: fee_config
                    .spend_limit
                    .clone()
                    .unwrap_or_else(|| "0uxion".to_string()),
                description: fee_config.description.clone(),
            }
        };

        // Call set_fee_config
        let result = self.set_fee_config(treasury_address, fee_input).await?;
        Ok(result.tx_hash)
    }

    /// Import grant config to treasury
    ///
    /// Converts GrantConfigInfo to GrantConfigInput and calls add_grant_config.
    /// Uses preserved authorization input if available, otherwise defaults to Generic.
    async fn import_grant_config(
        &self,
        treasury_address: &str,
        grant_config: &super::types::GrantConfigInfo,
    ) -> Result<String> {
        // Use preserved authorization input if available, otherwise default to Generic
        let authorization = grant_config
            .authorization_input
            .clone()
            .unwrap_or(super::types::AuthorizationInput::Generic);

        let grant_input = super::types::GrantConfigInput {
            type_url: grant_config.type_url.clone(),
            description: grant_config.description.clone(),
            authorization,
            optional: grant_config.optional,
        };

        // Call add_grant_config
        let result = self.add_grant_config(treasury_address, grant_input).await?;
        Ok(result.tx_hash)
    }

    // ========================================================================
    // Contract Query Operations
    // ========================================================================

    /// Query a CosmWasm smart contract
    ///
    /// Performs a read-only query on a CosmWasm smart contract.
    /// This is a direct RPC call and does not require authentication.
    ///
    /// # Arguments
    /// * `contract_address` - Contract address to query
    /// * `query_msg` - Query message as JSON value
    ///
    /// # Returns
    /// Query result as JSON value
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::TreasuryManager;
    /// use xion_agent_toolkit::oauth::OAuthClient;
    /// use xion_agent_toolkit::config::NetworkConfig;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let config = NetworkConfig {
    /// #     network_name: "testnet".to_string(),
    /// #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
    /// #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// #     chain_id: "xion-testnet-2".to_string(),
    /// #     oauth_client_id: "client-id".to_string(),
    /// #     treasury_code_id: 1260,
    /// #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    /// #     callback_port: 54321,
    /// # };
    /// # let oauth_client = OAuthClient::new(config.clone())?;
    /// let manager = TreasuryManager::new(oauth_client, config.clone());
    /// let query = serde_json::json!({ "balance": {} });
    /// let result = manager.query_contract("xion1contract...", &query).await?;
    /// println!("Query result: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, query_msg))]
    pub async fn query_contract(
        &self,
        contract_address: &str,
        query_msg: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        debug!("Querying contract: {}", contract_address);

        // Call API client to query contract (no auth required for query)
        self.api_client
            .query_contract_smart(contract_address, query_msg)
            .await
    }
}

// ============================================================================
// Helper Functions for Encoding
// ============================================================================

/// Parse a duration string (e.g., "86400s", "24h", "3600") into seconds
///
/// Supports:
/// - Protobuf Duration format: "86400s"
/// - Simple number: "86400"
/// - Human readable: "24h", "1d", "1h30m"
fn parse_duration_string(s: &str) -> Result<u64> {
    let s = s.trim();

    // Handle empty string
    if s.is_empty() {
        anyhow::bail!("Empty duration string");
    }

    // Handle protobuf duration format (e.g., "86400s")
    if s.ends_with('s') && !s.contains('h') && !s.contains('m') && !s.contains('d') {
        let num_str = &s[..s.len() - 1];
        return num_str
            .parse::<u64>()
            .map_err(|_| anyhow::anyhow!("Invalid duration: {}", s));
    }

    // Handle simple number (assume seconds)
    if s.chars().all(|c| c.is_ascii_digit()) {
        return s
            .parse::<u64>()
            .map_err(|_| anyhow::anyhow!("Invalid duration: {}", s));
    }

    // Handle human-readable format (e.g., "24h", "1d", "1h30m")
    let mut total_seconds: u64 = 0;
    let mut current_num = String::new();

    for c in s.chars() {
        if c.is_ascii_digit() {
            current_num.push(c);
        } else {
            let num: u64 = current_num
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid duration: {}", s))?;
            current_num.clear();

            match c {
                'd' => total_seconds += num * 86400,
                'h' => total_seconds += num * 3600,
                'm' => total_seconds += num * 60,
                's' => total_seconds += num,
                _ => anyhow::bail!("Unknown duration unit: {}", c),
            }
        }
    }

    // Handle trailing number without unit (assume seconds)
    if !current_num.is_empty() {
        let num: u64 = current_num
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid duration: {}", s))?;
        total_seconds += num;
    }

    if total_seconds == 0 {
        anyhow::bail!("Invalid duration: {}", s);
    }

    Ok(total_seconds)
}

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
            let nested_allowance = nested
                .allowance
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Nested allowance is required for AllowedMsg"))?;
            let encoded = encode_allowed_msg_allowance(
                allowed_messages.clone(),
                &nested_allowance.type_url,
                &nested_allowance.value,
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
        allowance: Some(super::types::ProtobufAny {
            type_url: allowance_type_url,
            value: allowance_value, // Already base64 encoded
        }),
        expiration: None, // TODO: Add expiration support
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
                value: auth_value, // Already base64 encoded, no conversion needed
            },
            optional: input.optional,
        },
    ))
}
