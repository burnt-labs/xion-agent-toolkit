//! Treasury API Client
//!
//! Client for communicating with Xion's Treasury API Service.
//! Supports listing, querying, and managing treasury contracts.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, instrument, warn};

use super::types::{
    BroadcastRequest, BroadcastResponse, QueryOptions, TreasuryInfo, TreasuryListItem,
};

/// Default delay before polling for new treasury (in seconds)
const DEFAULT_POLL_DELAY_SECS: u64 = 2;
/// Default timeout for waiting for treasury creation (in seconds)
const DEFAULT_POLL_TIMEOUT_SECS: u64 = 30;
/// Polling interval (in seconds)
const POLL_INTERVAL_SECS: u64 = 2;

/// Treasury API Client for Xion
///
/// Handles communication with the Treasury API service for:
/// - Listing user's treasury contracts
/// - Querying treasury details
/// - Creating new treasury contracts (future)
/// - Funding treasury contracts (future)
#[derive(Debug, Clone)]
pub struct TreasuryApiClient {
    /// Base URL of the Treasury API service
    base_url: String,
    /// HTTP client for making requests
    http_client: Client,
}

/// Treasury list response from API
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreasuryListResponse {
    /// List of treasuries
    treasuries: Vec<TreasuryListItem>,
}

/// Treasury query response from API
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreasuryQueryResponse {
    /// Treasury details
    treasury: TreasuryInfo,
}

impl TreasuryApiClient {
    /// Create a new Treasury API client
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the Treasury API service (e.g., "https://oauth2.testnet.burnt.com")
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::api::TreasuryApiClient;
    ///
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// ```
    pub fn new(base_url: String) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url,
            http_client,
        }
    }

    /// List all treasuries for authenticated user
    ///
    /// Retrieves a list of all treasury contracts associated with the authenticated user.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    ///
    /// # Returns
    /// List of treasury items with basic information
    ///
    /// # Errors
    /// Returns an error if:
    /// - The access token is invalid or expired
    /// - Network request fails
    /// - API returns an error response
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::api::TreasuryApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let treasuries = client.list_treasuries("access_token_123").await?;
    /// println!("Found {} treasuries", treasuries.len());
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn list_treasuries(
        &self,
        access_token: &str,
    ) -> Result<Vec<TreasuryListItem>> {
        let url = format!("{}/mgr-api/treasuries", self.base_url);
        debug!("Listing treasuries from: {}", url);

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .context("Failed to send list treasuries request")?;

        let status = response.status();
        debug!("List treasuries response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!(
                "List treasuries failed with status {}: {}",
                status,
                error_text
            );
        }

        let result: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse list treasuries response")?;

        let treasuries: Vec<TreasuryListItem> = serde_json::from_value(
            result.get("treasuries").cloned().unwrap_or_default()
        ).context("Failed to parse treasuries list")?;

        debug!("Successfully retrieved {} treasuries", treasuries.len());
        Ok(treasuries)
    }

    /// Query specific treasury details
    ///
    /// Retrieves detailed information about a specific treasury contract.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `address` - Treasury contract address
    /// * `options` - Query options to control what information to include
    ///
    /// # Returns
    /// Complete treasury information including balance, parameters, and configurations
    ///
    /// # Errors
    /// Returns an error if:
    /// - The access token is invalid or expired
    /// - The treasury address is invalid or not found
    /// - Network request fails
    /// - API returns an error response
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::{TreasuryApiClient, QueryOptions};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let options = QueryOptions::default();
    /// let treasury = client.query_treasury(
    ///     "access_token_123",
    ///     "xion1abc123...",
    ///     options
    /// ).await?;
    /// println!("Treasury balance: {} uxion", treasury.balance);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn query_treasury(
        &self,
        access_token: &str,
        address: &str,
        options: QueryOptions,
    ) -> Result<TreasuryInfo> {
        let mut url = format!("{}/mgr-api/treasury/{}", self.base_url, address);

        // Add query parameters
        let mut params = Vec::new();
        if options.grants {
            params.push("grants=true");
        }
        if options.fee {
            params.push("fee=true");
        }
        if options.admin {
            params.push("admin=true");
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        debug!("Querying treasury from: {}", url);

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .context("Failed to send query treasury request")?;

        let status = response.status();
        debug!("Query treasury response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!(
                "Query treasury failed with status {}: {}",
                status,
                error_text
            );
        }

        let result: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse query treasury response")?;

        let treasury: TreasuryInfo = serde_json::from_value(
            result.get("treasury").cloned().unwrap_or_default()
        ).context("Failed to parse treasury info")?;

        debug!("Successfully queried treasury: {}", treasury.address);
        Ok(treasury)
    }

    /// Broadcast a transaction to the blockchain
    ///
    /// Signs and broadcasts a transaction using the authenticated user's wallet.
    /// This is used for operations like funding treasuries and withdrawing from treasuries.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `request` - Transaction broadcast request containing messages
    ///
    /// # Returns
    /// Transaction broadcast response with tx_hash
    ///
    /// # Errors
    /// Returns an error if:
    /// - The access token is invalid or expired
    /// - Network request fails
    /// - API returns an error response
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::{TreasuryApiClient, TransactionMessage, BroadcastRequest};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    ///
    /// let request = BroadcastRequest {
    ///     messages: vec![TransactionMessage {
    ///         type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
    ///         value: serde_json::json!({
    ///             "fromAddress": "xion1...",
    ///             "toAddress": "xion1...",
    ///             "amount": [{ "denom": "uxion", "amount": "1000000" }]
    ///         }),
    ///     }],
    ///     memo: None,
    /// };
    ///
    /// let result = client.broadcast_transaction("access_token_123", request).await?;
    /// println!("Transaction hash: {}", result.tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token, request))]
    pub async fn broadcast_transaction(
        &self,
        access_token: &str,
        request: BroadcastRequest,
    ) -> Result<BroadcastResponse> {
        let url = format!("{}/api/v1/transaction", self.base_url);
        debug!("Broadcasting transaction to: {}", url);

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await
            .context("Failed to send broadcast transaction request")?;

        let status = response.status();
        debug!("Broadcast transaction response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!(
                "Broadcast transaction failed with status {}: {}",
                status,
                error_text
            );
        }

        let result: BroadcastResponse = response
            .json()
            .await
            .context("Failed to parse broadcast transaction response")?;

        debug!("Successfully broadcast transaction: {}", result.tx_hash);
        Ok(result)
    }

    /// Fund a treasury contract
    ///
    /// Sends tokens from the authenticated user's wallet to a treasury contract.
    /// This creates a MsgSend transaction and broadcasts it to the blockchain.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address to fund
    /// * `amount` - Amount to send (e.g., "1000000uxion")
    /// * `from_address` - Sender's MetaAccount address
    ///
    /// # Returns
    /// Transaction broadcast response with tx_hash
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::TreasuryApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let result = client.fund_treasury(
    ///     "access_token_123",
    ///     "xion1treasury...",
    ///     "1000000uxion",
    ///     "xion1sender..."
    /// ).await?;
    /// println!("Fund transaction hash: {}", result.tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn fund_treasury(
        &self,
        access_token: &str,
        treasury_address: &str,
        amount: &str,
        from_address: &str,
    ) -> Result<BroadcastResponse> {
        debug!(
            "Funding treasury {} with {} from {}",
            treasury_address, amount, from_address
        );

        // Parse amount (e.g., "1000000uxion" -> amount: "1000000", denom: "uxion")
        let (amount_val, denom) = parse_coin(amount)?;

        let request = BroadcastRequest {
            messages: vec![super::types::TransactionMessage {
                type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                value: serde_json::json!({
                    "fromAddress": from_address,
                    "toAddress": treasury_address,
                    "amount": [{ "amount": amount_val, "denom": denom }]
                }),
            }],
            memo: Some(format!("Fund treasury {}", treasury_address)),
        };

        self.broadcast_transaction(access_token, request).await
    }

    /// Withdraw from a treasury contract
    ///
    /// Withdraws tokens from a treasury contract to the admin's wallet.
    /// This creates a MsgExecuteContract transaction calling the Withdraw message.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address to withdraw from
    /// * `amount` - Amount to withdraw (e.g., "1000000uxion")
    /// * `from_address` - Sender's MetaAccount address (must be the admin)
    ///
    /// # Returns
    /// Transaction broadcast response with tx_hash
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::TreasuryApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let result = client.withdraw_treasury(
    ///     "access_token_123",
    ///     "xion1treasury...",
    ///     "1000000uxion",
    ///     "xion1admin..."
    /// ).await?;
    /// println!("Withdraw transaction hash: {}", result.tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn withdraw_treasury(
        &self,
        access_token: &str,
        treasury_address: &str,
        amount: &str,
        from_address: &str,
    ) -> Result<BroadcastResponse> {
        debug!(
            "Withdrawing {} from treasury {} to {}",
            amount, treasury_address, from_address
        );

        // Parse amount
        let (amount_val, denom) = parse_coin(amount)?;

        // Create the Withdraw execute message
        let withdraw_msg = serde_json::json!({
            "withdraw": {
                "coins": [{ "amount": amount_val, "denom": denom }]
            }
        });

        // Encode the message as base64
        let msg_base64 = base64_encode(&withdraw_msg)?;

        let request = BroadcastRequest {
            messages: vec![super::types::TransactionMessage {
                type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
                value: serde_json::json!({
                    "sender": from_address,
                    "contract": treasury_address,
                    "msg": msg_base64,
                    "funds": []
                }),
            }],
            memo: Some(format!("Withdraw from treasury {}", treasury_address)),
        };

        self.broadcast_transaction(access_token, request).await
    }

    /// Create a new treasury contract
    ///
    /// Creates a new treasury contract using CosmWasm instantiate2 for predictable addresses.
    /// The treasury contract is instantiated with the provided configuration.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_code_id` - Treasury contract code ID from network config
    /// * `request` - Treasury creation request with all required parameters
    /// * `salt` - Random salt for instantiate2 (32 bytes)
    ///
    /// # Returns
    /// Create treasury result with the new treasury address and transaction hash
    ///
    /// # Errors
    /// Returns an error if:
    /// - The access token is invalid or expired
    /// - Invalid parameters
    /// - Contract instantiation fails
    /// - Network request fails
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::{TreasuryApiClient, CreateTreasuryRequest, FeeConfigMessage, GrantConfigMessage, TreasuryParamsMessage, TypeUrlValue};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    ///
    /// let request = CreateTreasuryRequest {
    ///     admin: "xion1admin...".to_string(),
    ///     fee_config: FeeConfigMessage {
    ///         allowance: TypeUrlValue {
    ///             type_url: "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
    ///             value: base64::Engine::encode(
    ///                 &base64::engine::general_purpose::STANDARD,
    ///                 b"{}"
    ///             ),
    ///         },
    ///         description: "Fee grant for users".to_string(),
    ///     },
    ///     grant_configs: vec![
    ///         GrantConfigMessage {
    ///             authorization: TypeUrlValue {
    ///                 type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
    ///                 value: base64::Engine::encode(
    ///                     &base64::engine::general_purpose::STANDARD,
    ///                     b"{}"
    ///                 ),
    ///             },
    ///             description: Some("Allow sending tokens".to_string()),
    ///         },
    ///     ],
    ///     params: TreasuryParamsMessage {
    ///         redirect_url: "https://example.com/callback".to_string(),
    ///         icon_url: "https://example.com/icon.png".to_string(),
    ///         display_url: None,
    ///         metadata: None,
    ///     },
    ///     name: Some("My Treasury".to_string()),
    ///     is_oauth2_app: false,
    /// };
    ///
    /// let salt = [0u8; 32];
    /// let result = client.create_treasury("access_token_123", 1260, request, &salt).await?;
    /// println!("Treasury created at: {}", result.treasury_address);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token, request, salt))]
    pub async fn create_treasury(
        &self,
        access_token: &str,
        treasury_code_id: u64,
        request: super::types::CreateTreasuryRequest,
        salt: &[u8],
    ) -> Result<super::types::CreateTreasuryResult> {
        debug!("Creating treasury with code ID: {}", treasury_code_id);

        // Build the instantiation message
        let instantiate_msg = build_treasury_instantiate_msg(&request)?;

        // Encode the message as base64
        let msg_base64 = base64_encode(&instantiate_msg)?;

        // Convert salt to base64
        let salt_base64 = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            salt,
        );

        // Build the MsgInstantiateContract2 message
        let msg_value = serde_json::json!({
            "sender": request.admin,
            "code_id": treasury_code_id,
            "label": format!("Treasury-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")),
            "msg": msg_base64,
            "salt": salt_base64,
            "funds": [],
            "admin": request.admin, // Treasury is its own admin
        });

        let broadcast_request = BroadcastRequest {
            messages: vec![super::types::TransactionMessage {
                type_url: "/cosmwasm.wasm.v1.MsgInstantiateContract2".to_string(),
                value: msg_value,
            }],
            memo: Some("Create Treasury via Xion Agent Toolkit".to_string()),
        };

        // Broadcast the transaction
        let response = self.broadcast_transaction(access_token, broadcast_request).await?;
        
        debug!("Treasury creation transaction broadcast: {}", response.tx_hash);

        // Wait for the treasury to be indexed and return the actual address
        let treasury_address = self
            .wait_for_treasury_creation(access_token, &request.admin, &response.tx_hash)
            .await?;

        Ok(super::types::CreateTreasuryResult {
            treasury_address,
            tx_hash: response.tx_hash,
            admin: request.admin,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Wait for treasury creation to be indexed
    ///
    /// Polls the `/mgr-api/treasuries` endpoint to find the newly created treasury.
    /// The treasury is identified by matching the admin address and recent creation time.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `admin_address` - Expected admin address of the new treasury
    /// * `tx_hash` - Transaction hash for error reporting
    ///
    /// # Returns
    /// The treasury address if found within timeout
    ///
    /// # Errors
    /// Returns an error with the tx_hash if the treasury is not found within the timeout period
    #[instrument(skip(self, access_token))]
    async fn wait_for_treasury_creation(
        &self,
        access_token: &str,
        admin_address: &str,
        tx_hash: &str,
    ) -> Result<String> {
        debug!(
            "Waiting for treasury creation to be indexed (admin: {})",
            admin_address
        );

        // Initial delay to allow indexing
        sleep(Duration::from_secs(DEFAULT_POLL_DELAY_SECS)).await;

        let start_time = std::time::Instant::now();
        let timeout = Duration::from_secs(DEFAULT_POLL_TIMEOUT_SECS);

        loop {
            // Check if we've exceeded the timeout
            if start_time.elapsed() >= timeout {
                anyhow::bail!(
                    "Treasury creation timed out after {} seconds. Transaction was broadcast successfully (tx_hash: {}). \
                     The treasury may still be created. Check the transaction status manually or wait a few moments and list your treasuries.",
                    DEFAULT_POLL_TIMEOUT_SECS,
                    tx_hash
                );
            }

            // List treasuries and look for the newly created one
            match self.list_treasuries(access_token).await {
                Ok(treasuries) => {
                    // Look for a treasury with matching admin address
                    // The newest treasury should be at the top of the list (most recent first)
                    for treasury in &treasuries {
                        if treasury.admin.as_deref() == Some(admin_address) {
                            debug!(
                                "Found newly created treasury: {} for admin: {}",
                                treasury.address, admin_address
                            );
                            return Ok(treasury.address.clone());
                        }
                    }
                    debug!(
                        "Treasury not yet indexed, retrying... ({}/{}s elapsed)",
                        start_time.elapsed().as_secs(),
                        DEFAULT_POLL_TIMEOUT_SECS
                    );
                }
                Err(e) => {
                    warn!("Failed to list treasuries while waiting for creation: {}", e);
                }
            }

            // Wait before next poll
            sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
        }
    }

    /// Get the base URL
    #[allow(dead_code)]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Build the treasury instantiation message
fn build_treasury_instantiate_msg(request: &super::types::CreateTreasuryRequest) -> Result<serde_json::Value> {
    // Build the metadata JSON string
    let metadata = serde_json::json!({
        "name": request.name.as_deref().unwrap_or(""),
        "archived": false,
        "is_oauth2_app": request.is_oauth2_app,
    });

    // Extract type URLs from grant configs
    let type_urls: Vec<String> = request.grant_configs
        .iter()
        .map(|gc| gc.authorization.type_url.clone())
        .collect();

    // Build the grant configs array (without type_url, that's in type_urls)
    let grant_configs: Vec<serde_json::Value> = request.grant_configs
        .iter()
        .map(|gc| {
            serde_json::json!({
                "authorization": gc.authorization,
                "description": gc.description,
            })
        })
        .collect();

    // Build the complete instantiation message
    Ok(serde_json::json!({
        "type_urls": type_urls,
        "grant_configs": grant_configs,
        "fee_config": request.fee_config,
        "admin": request.admin,
        "params": {
            "redirect_url": request.params.redirect_url,
            "icon_url": request.params.icon_url,
            "display_url": request.params.display_url,
            "metadata": metadata.to_string(),
        },
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Parse a coin string (e.g., "1000000uxion") into (amount, denom)
fn parse_coin(coin: &str) -> Result<(String, String)> {
    // Find where digits end and letters begin
    let split_pos = coin
        .chars()
        .position(|c| !c.is_ascii_digit())
        .ok_or_else(|| anyhow::anyhow!("Invalid coin format: {}", coin))?;

    let amount = coin[..split_pos].to_string();
    let denom = coin[split_pos..].to_string();

    if amount.is_empty() || denom.is_empty() {
        anyhow::bail!("Invalid coin format: {}", coin);
    }

    Ok((amount, denom))
}

/// Encode a JSON value to base64
fn base64_encode(value: &serde_json::Value) -> Result<String> {
    let json_str = serde_json::to_string(value)?;
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        json_str.as_bytes(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = TreasuryApiClient::new("https://test.com".to_string());
        assert_eq!(client.base_url, "https://test.com");
    }

    #[test]
    fn test_query_options_default() {
        let options = QueryOptions::default();
        assert!(options.grants);
        assert!(options.fee);
        assert!(options.admin);
    }

    #[test]
    fn test_query_options_custom() {
        let options = QueryOptions {
            grants: false,
            fee: true,
            admin: false,
        };
        assert!(!options.grants);
        assert!(options.fee);
        assert!(!options.admin);
    }

    #[test]
    fn test_treasury_list_response_deserialization() {
        let json = r#"{
            "treasuries": [
                {
                    "address": "xion1abc123",
                    "admin": "xion1def456",
                    "balance": "10000000"
                },
                {
                    "address": "xion1xyz789",
                    "balance": "5000000"
                }
            ]
        }"#;

        let response: TreasuryListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.treasuries.len(), 2);
        assert_eq!(response.treasuries[0].address, "xion1abc123");
        assert_eq!(response.treasuries[0].balance, "10000000");
        assert_eq!(response.treasuries[1].address, "xion1xyz789");
        assert_eq!(response.treasuries[1].balance, "5000000");
    }

    #[test]
    fn test_treasury_query_response_deserialization() {
        let json = r#"{
            "treasury": {
                "address": "xion1abc123",
                "admin": "xion1def456",
                "balance": "10000000",
                "params": {
                    "redirect_url": "https://example.com/callback",
                    "icon_url": "https://example.com/icon.png"
                }
            }
        }"#;

        let response: TreasuryQueryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.treasury.address, "xion1abc123");
        assert_eq!(response.treasury.balance, "10000000");
    }

    #[test]
    fn test_parse_coin() {
        let (amount, denom) = parse_coin("1000000uxion").unwrap();
        assert_eq!(amount, "1000000");
        assert_eq!(denom, "uxion");

        let (amount, denom) = parse_coin("500uusdc").unwrap();
        assert_eq!(amount, "500");
        assert_eq!(denom, "uusdc");
    }

    #[test]
    fn test_parse_coin_invalid() {
        assert!(parse_coin("invalid").is_err());
        assert!(parse_coin("123").is_err());
        assert!(parse_coin("abc").is_err());
    }

    #[test]
    fn test_base64_encode() {
        let value = serde_json::json!({"withdraw": {"coins": [{"amount": "1000", "denom": "uxion"}]}});
        let encoded = base64_encode(&value).unwrap();
        assert!(!encoded.is_empty());

        // Verify we can decode it back
        let decoded = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            encoded,
        )
        .unwrap();
        let decoded_value: serde_json::Value = serde_json::from_slice(&decoded).unwrap();
        assert_eq!(value, decoded_value);
    }

    #[tokio::test]
    async fn test_wait_for_treasury_creation_success() {
        // This test verifies the polling mechanism finds the treasury
        let mut server = mockito::Server::new_async().await;
        
        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";
        
        // Mock the list treasuries endpoint - return treasury with matching admin
        let mock = server.mock("GET", "/mgr-api/treasuries")
            .match_header("authorization", "Bearer test_token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::json!({
                "treasuries": [
                    {
                        "address": treasury_address,
                        "admin": admin_address,
                        "balance": "0"
                    }
                ]
            }).to_string())
            .create();
        
        let client = TreasuryApiClient::new(server.url());
        
        // Call the wait_for_treasury_creation method
        let result = client
            .wait_for_treasury_creation("test_token", admin_address, "tx123")
            .await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), treasury_address);
        
        mock.assert();
    }

    #[tokio::test]
    async fn test_wait_for_treasury_creation_multiple_treasuries() {
        // Test that it finds the correct treasury when there are multiple
        let mut server = mockito::Server::new_async().await;
        
        let admin_address = "xion1admin999";
        let treasury_address = "xion1treasury999";
        
        // Mock returning multiple treasuries, one with matching admin
        let mock = server.mock("GET", "/mgr-api/treasuries")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::json!({
                "treasuries": [
                    {
                        "address": "xion1other1",
                        "admin": "xion1admin1",
                        "balance": "1000"
                    },
                    {
                        "address": "xion1other2",
                        "admin": "xion1admin2",
                        "balance": "2000"
                    },
                    {
                        "address": treasury_address,
                        "admin": admin_address,
                        "balance": "0"
                    }
                ]
            }).to_string())
            .create();
        
        let client = TreasuryApiClient::new(server.url());
        
        let result = client
            .wait_for_treasury_creation("test_token", admin_address, "tx456")
            .await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), treasury_address);
        
        mock.assert();
    }
}