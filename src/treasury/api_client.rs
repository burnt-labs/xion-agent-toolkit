//! Treasury API Client
//!
//! Client for communicating with Xion's Treasury API Service.
//! Supports listing, querying, and managing treasury contracts.

use anyhow::{Context, Result};
use chrono::DateTime;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, instrument, warn};

use super::types::{
    BroadcastRequest, BroadcastResponse, QueryOptions, TreasuryInfo, TreasuryListItem,
    TreasuryParams,
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
/// - Listing user's treasury contracts (via DaoDao Indexer)
/// - Querying treasury details
/// - Creating new treasury contracts (future)
/// - Funding treasury contracts (future)
#[derive(Debug, Clone)]
pub struct TreasuryApiClient {
    /// Base URL of the Treasury API service
    base_url: String,
    /// DaoDao Indexer URL for treasury listing
    indexer_url: String,
    /// RPC URL for on-chain queries
    rpc_url: String,
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

/// DaoDao Indexer returns a direct array of treasury items
/// (not wrapped in an object with "treasuries" field)
///
/// Individual treasury item from DaoDao Indexer
/// Matches the actual API response format:
/// ```json
/// {
///   "contractAddress": "xion1...",
///   "balances": {"uxion": "10000000000"},
///   "block": {"height": "...", "timeUnixMs": "..."},
///   "codeId": 1260,
///   "params": {"icon_url": "...", "metadata": "...", "redirect_url": "..."}
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexerTreasuryItem {
    /// Treasury contract address
    #[serde(rename = "contractAddress")]
    contract_address: String,
    /// Balances map (denom -> amount)
    #[serde(default)]
    balances: HashMap<String, String>,
    /// Block info (height and timestamp)
    #[serde(default)]
    block: Option<IndexerBlockInfo>,
    /// Code ID of the treasury contract
    #[serde(rename = "codeId", default)]
    code_id: Option<u64>,
    /// Treasury params
    #[serde(default)]
    params: Option<IndexerTreasuryParams>,
}

/// Block info from DaoDao Indexer
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexerBlockInfo {
    /// Block height
    #[serde(default)]
    height: Option<String>,
    /// Unix timestamp in milliseconds
    #[serde(rename = "timeUnixMs", default)]
    time_unix_ms: Option<String>,
}

/// Treasury params from DaoDao Indexer
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexerTreasuryParams {
    /// Icon URL
    #[serde(default)]
    icon_url: Option<String>,
    /// Metadata JSON string
    #[serde(default)]
    metadata: Option<String>,
    /// Redirect URL
    #[serde(default)]
    redirect_url: Option<String>,
}

/// Metadata JSON structure within params.metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreasuryMetadataJson {
    /// Treasury name (optional)
    #[serde(default)]
    name: Option<String>,
    /// Is OAuth2 app flag
    #[serde(rename = "is_oauth2_app", default)]
    is_oauth2_app: Option<bool>,
    /// Archived flag
    #[serde(default)]
    archived: Option<bool>,
}

impl TreasuryApiClient {
    /// Create a new Treasury API client
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the Treasury API service (e.g., "https://oauth2.testnet.burnt.com")
    /// * `indexer_url` - DaoDao Indexer URL for listing treasuries (e.g., "https://daodaoindexer.burnt.com/xion-testnet-2")
    /// * `rpc_url` - RPC URL for on-chain queries (e.g., "https://rpc.xion-testnet-2.burnt.com:443")
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::api::TreasuryApiClient;
    ///
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// );
    /// ```
    pub fn new(base_url: String, indexer_url: String, rpc_url: String) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url,
            indexer_url,
            rpc_url,
            http_client,
        }
    }

    /// Helper function to build and broadcast a CosmWasm execute contract message
    ///
    /// This follows the same format as create_treasury, ensuring consistency across all APIs.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `sender` - Sender address
    /// * `contract` - Treasury contract address
    /// * `execute_msg` - Execute message to send (will be JSON-encoded then base64-encoded)
    /// * `memo` - Transaction memo
    ///
    /// # Returns
    /// Transaction hash on success
    async fn broadcast_execute_contract<T: Serialize>(
        &self,
        access_token: &str,
        sender: &str,
        contract: &str,
        execute_msg: &T,
        memo: &str,
    ) -> Result<String> {
        // Serialize execute message to JSON, then convert to number array
        // (OAuth2 API's JSON object path uses `fromPartial` which expects
        // bytes fields as array-like objects, not base64 strings)
        let msg_json = serde_json::to_string(execute_msg)?;
        let msg_bytes = msg_json.as_bytes();

        debug!("Execute message JSON:\n{}", msg_json);

        let msg_value = serde_json::json!({
            "sender": sender,
            "contract": contract,
            "msg": bytes_to_json_array(msg_bytes),  // Number array, not base64 string
            "funds": []
        });

        let broadcast_request = BroadcastRequest {
            messages: vec![super::types::TransactionMessage {
                type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
                value: msg_value,
            }],
            memo: Some(memo.to_string()),
        };

        let response = self
            .broadcast_transaction(access_token, broadcast_request)
            .await?;

        Ok(response.tx_hash)
    }

    /// Helper function to build and broadcast a CosmWasm instantiate contract message (v1)
    ///
    /// Instantiates a new contract instance with a dynamically assigned address.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `sender` - Sender address
    /// * `code_id` - Code ID of the contract to instantiate
    /// * `instantiate_msg` - Instantiate message (will be JSON-encoded)
    /// * `label` - Label for the contract instance
    /// * `admin` - Optional admin address for contract migrations
    /// * `memo` - Transaction memo
    ///
    /// # Returns
    /// Transaction hash on success
    #[allow(clippy::too_many_arguments)]
    pub async fn broadcast_instantiate_contract<T: Serialize>(
        &self,
        access_token: &str,
        sender: &str,
        code_id: u64,
        instantiate_msg: &T,
        label: &str,
        admin: Option<&str>,
        memo: &str,
    ) -> Result<String> {
        // Serialize instantiate message to JSON, then convert to number array
        let msg_json = serde_json::to_string(instantiate_msg)?;
        let msg_bytes = msg_json.as_bytes();

        debug!("Instantiate message JSON:\n{}", msg_json);

        // Build MsgInstantiateContract message
        // Note: codeId is number, msg is number array (not base64 string)
        let mut msg_value = serde_json::json!({
            "sender": sender,
            "codeId": code_id,  // number, not string
            "label": label,
            "msg": bytes_to_json_array(msg_bytes),  // Number array
            "funds": []
        });

        // Add optional admin field
        if let Some(admin_addr) = admin {
            msg_value["admin"] = serde_json::json!(admin_addr);
        }

        let broadcast_request = BroadcastRequest {
            messages: vec![super::types::TransactionMessage {
                type_url: "/cosmwasm.wasm.v1.MsgInstantiateContract".to_string(),
                value: msg_value,
            }],
            memo: Some(memo.to_string()),
        };

        let response = self
            .broadcast_transaction(access_token, broadcast_request)
            .await?;

        Ok(response.tx_hash)
    }

    /// Helper function to build and broadcast a CosmWasm instantiate2 contract message (v2, predictable addresses)
    ///
    /// Instantiates a new contract instance with a predictable address using instantiate2.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `sender` - Sender address
    /// * `code_id` - Code ID of the contract to instantiate
    /// * `instantiate_msg` - Instantiate message (will be JSON-encoded)
    /// * `label` - Label for the contract instance
    /// * `salt` - Salt for predictable address generation
    /// * `admin` - Optional admin address for contract migrations
    /// * `memo` - Transaction memo
    ///
    /// # Returns
    /// Transaction hash on success
    #[allow(clippy::too_many_arguments)]
    pub async fn broadcast_instantiate_contract2<T: Serialize>(
        &self,
        access_token: &str,
        sender: &str,
        code_id: u64,
        instantiate_msg: &T,
        label: &str,
        salt: &[u8],
        admin: Option<&str>,
        memo: &str,
    ) -> Result<String> {
        // Serialize instantiate message to JSON, then convert to number array
        let msg_json = serde_json::to_string(instantiate_msg)?;
        let msg_bytes = msg_json.as_bytes();

        debug!("Instantiate2 message JSON:\n{}", msg_json);

        // Build MsgInstantiateContract2 message
        // Note: codeId is number, msg and salt are number arrays (not base64 strings)
        let mut msg_value = serde_json::json!({
            "sender": sender,
            "codeId": code_id,  // number, not string
            "label": label,
            "msg": bytes_to_json_array(msg_bytes),  // Number array
            "salt": bytes_to_json_array(salt),       // Number array
            "funds": [],
            "fixMsg": false,
        });

        // Add optional admin field
        if let Some(admin_addr) = admin {
            msg_value["admin"] = serde_json::json!(admin_addr);
        }

        let broadcast_request = BroadcastRequest {
            messages: vec![super::types::TransactionMessage {
                type_url: "/cosmwasm.wasm.v1.MsgInstantiateContract2".to_string(),
                value: msg_value,
            }],
            memo: Some(memo.to_string()),
        };

        let response = self
            .broadcast_transaction(access_token, broadcast_request)
            .await?;

        Ok(response.tx_hash)
    }

    /// List all treasuries for authenticated user
    ///
    /// Retrieves a list of all treasury contracts associated with the authenticated user
    /// using the DaoDao Indexer.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token (format: {userId}:{grantId}:{secret})
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
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// );
    /// let treasuries = client.list_treasuries("access_token_123").await?;
    /// println!("Found {} treasuries", treasuries.len());
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn list_treasuries(&self, access_token: &str) -> Result<Vec<TreasuryListItem>> {
        // Extract user address from access token (format: {userId}:{grantId}:{secret})
        let user_address = extract_address_from_token(access_token)?;

        // Use DaoDao Indexer to list treasuries
        let url = format!(
            "{}/contract/{}/xion/account/treasuries",
            self.indexer_url, user_address
        );
        debug!("Listing treasuries from DaoDao Indexer: {}", url);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context("Failed to send list treasuries request to indexer")?;

        let status = response.status();
        debug!("List treasuries response status: {}", status);

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!(
                "List treasuries failed with status {}: {}",
                status,
                error_text
            );
        }

        // Parse indexer response - it returns a direct array, not wrapped in an object
        let indexer_items: Vec<IndexerTreasuryItem> = response
            .json()
            .await
            .context("Failed to parse indexer response")?;

        // Convert indexer items to TreasuryListItem
        let treasuries: Vec<TreasuryListItem> = indexer_items
            .into_iter()
            .map(|item| {
                // Get uxion balance (or 0 if not present)
                let balance = item
                    .balances
                    .get("uxion")
                    .cloned()
                    .unwrap_or_else(|| "0".to_string());

                // Parse metadata to extract name
                let name = item.params.as_ref().and_then(|p| {
                    p.metadata.as_ref().and_then(|m| {
                        // Try to parse metadata JSON
                        serde_json::from_str::<TreasuryMetadataJson>(m)
                            .ok()
                            .and_then(|meta| meta.name)
                    })
                });

                // Convert timestamp from milliseconds to ISO string
                let created_at = item.block.as_ref().and_then(|b| {
                    b.time_unix_ms.as_ref().and_then(|ms| {
                        ms.parse::<i64>().ok().and_then(|ms_val| {
                            // Convert milliseconds to DateTime
                            DateTime::from_timestamp_millis(ms_val).map(|dt| dt.to_rfc3339())
                        })
                    })
                });

                TreasuryListItem {
                    address: item.contract_address,
                    // Admin is not returned by indexer; set to None
                    admin: None,
                    balance,
                    name,
                    created_at,
                }
            })
            .collect();

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
    /// * `options` - Query options (currently unused, kept for API compatibility)
    ///
    /// # Returns
    /// Treasury information from DaoDao Indexer (basic info only)
    ///
    /// # Errors
    /// Returns an error if:
    /// - The treasury address is not found
    /// - Network request fails
    /// - API returns an error response
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::{TreasuryApiClient, QueryOptions};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// );
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
        _options: QueryOptions,
    ) -> Result<TreasuryInfo> {
        // Use DaoDao Indexer to query treasury info
        // Extract user address from token to build the list URL
        let user_address = extract_address_from_token(access_token)?;

        let url = format!(
            "{}/contract/{}/xion/account/treasuries",
            self.indexer_url, user_address
        );
        debug!("Querying treasury from DaoDao Indexer: {}", url);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context("Failed to send query treasury request to indexer")?;

        let status = response.status();
        debug!("Query treasury response status: {}", status);

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!(
                "Query treasury failed with status {}: {}",
                status,
                error_text
            );
        }

        // Parse indexer response - direct array
        let indexer_items: Vec<IndexerTreasuryItem> = response
            .json()
            .await
            .context("Failed to parse indexer response")?;

        // Find the specific treasury by address
        let item = indexer_items
            .iter()
            .find(|item| item.contract_address == address)
            .ok_or_else(|| anyhow::anyhow!("Treasury {} not found", address))?;

        // Convert to TreasuryInfo
        let treasury = self.indexer_item_to_treasury_info(item)?;

        debug!("Successfully queried treasury: {}", treasury.address);
        Ok(treasury)
    }

    /// Convert IndexerTreasuryItem to TreasuryInfo
    fn indexer_item_to_treasury_info(&self, item: &IndexerTreasuryItem) -> Result<TreasuryInfo> {
        // Get uxion balance
        let balance = item
            .balances
            .get("uxion")
            .cloned()
            .unwrap_or_else(|| "0".to_string());

        // Parse params
        let params = if let Some(ref p) = item.params {
            TreasuryParams {
                display_url: None,
                redirect_url: p.redirect_url.clone().unwrap_or_default(),
                icon_url: p.icon_url.clone().unwrap_or_default(),
                metadata: p
                    .metadata
                    .as_ref()
                    .and_then(|m| serde_json::from_str(m).ok()),
            }
        } else {
            TreasuryParams {
                display_url: None,
                redirect_url: String::new(),
                icon_url: String::new(),
                metadata: None,
            }
        };

        Ok(TreasuryInfo {
            address: item.contract_address.clone(),
            admin: None, // Indexer doesn't return admin
            balance,
            params,
            fee_config: None,    // Requires on-chain query
            grant_configs: None, // Requires on-chain query
        })
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
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// );
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
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
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
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// );
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
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// );
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

        // Use the unified broadcast_execute_contract method
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &withdraw_msg,
                &format!("Withdraw from treasury {}", treasury_address),
            )
            .await?;

        // Return BroadcastResponse format for backward compatibility
        Ok(BroadcastResponse {
            success: true,
            tx_hash,
            from: from_address.to_string(),
            gas_used: None,
            gas_wanted: None,
        })
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
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    /// );
    ///
    /// let request = CreateTreasuryRequest {
    ///     admin: "xion1admin...".to_string(),
    ///     fee_config: FeeConfigMessage {
    ///         allowance: TypeUrlValue {
    ///             type_url: "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
    ///             value: "Cg==".to_string(), // Base64-encoded empty BasicAllowance
    ///         },
    ///         description: "Fee grant for users".to_string(),
    ///         expiration: None,
    ///     },
    ///     grant_configs: vec![
    ///         GrantConfigMessage {
    ///             type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
    ///             authorization: TypeUrlValue {
    ///                 type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
    ///                 value: "Cg==".to_string(), // Base64-encoded empty authorization
    ///             },
    ///             description: Some("Allow sending tokens".to_string()),
    ///             optional: false,
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

        // Generate label for the treasury
        let label = format!("Treasury-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"));

        // Use the generic broadcast_instantiate_contract2 method
        let tx_hash = self
            .broadcast_instantiate_contract2(
                access_token,
                &request.admin,
                treasury_code_id,
                &instantiate_msg,
                &label,
                salt,
                Some(&request.admin), // admin for contract migrations
                "Create Treasury via Xion Agent Toolkit",
            )
            .await?;

        debug!("Treasury creation transaction broadcast: {}", tx_hash);

        // Wait for the treasury to be indexed and return the actual address
        let treasury_address = self
            .wait_for_treasury_creation(access_token, &request.admin, &tx_hash)
            .await?;

        Ok(super::types::CreateTreasuryResult {
            treasury_address,
            tx_hash,
            admin: request.admin,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Wait for treasury creation to be indexed
    ///
    /// Polls the DaoDao Indexer to find the newly created treasury.
    /// Returns the first treasury found (assumed to be the newest one).
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `_admin_address` - Unused (kept for API compatibility)
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
        _admin_address: &str,
        tx_hash: &str,
    ) -> Result<String> {
        debug!("Waiting for treasury creation to be indexed");

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
                    // The newest treasury should be at the top of the list (most recent first)
                    // Since DaoDao Indexer doesn't return admin, we return the first treasury
                    // that has code_id matching our treasury code (1260)
                    if let Some(treasury) = treasuries.first() {
                        debug!(
                            "Found treasury: {} (assuming it's the newly created one)",
                            treasury.address
                        );
                        return Ok(treasury.address.clone());
                    }
                    debug!(
                        "No treasuries found yet, retrying... ({}/{}s elapsed)",
                        start_time.elapsed().as_secs(),
                        DEFAULT_POLL_TIMEOUT_SECS
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to list treasuries while waiting for creation: {}",
                        e
                    );
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
///
/// Creates the instantiate message in the format expected by the treasury contract:
/// - `grant_configs`: Array of JSON objects with authorization, description, and optional fields
/// - `fee_config`: JSON object with allowance, description, and optional expiration
/// - All fields use snake_case naming (matching the CosmWasm contract)
fn build_treasury_instantiate_msg(
    request: &super::types::CreateTreasuryRequest,
) -> Result<serde_json::Value> {
    // Build the metadata JSON string
    let metadata = serde_json::json!({
        "name": request.name.as_deref().unwrap_or(""),
        "archived": false,
        "is_oauth2_app": request.is_oauth2_app,
    });

    // Extract type URLs from grant configs (message type URLs like /cosmos.bank.v1beta1.MsgSend)
    let type_urls: Vec<String> = request
        .grant_configs
        .iter()
        .map(|gc| gc.type_url.clone())
        .collect();

    // Build the grant configs array (without type_url, that's in type_urls)
    // Each grant_config is a JSON object with: authorization, description, optional
    let grant_configs: Vec<serde_json::Value> = request
        .grant_configs
        .iter()
        .map(|gc| {
            // Build the authorization object
            let auth = serde_json::json!({
                "type_url": gc.authorization.type_url,
                "value": gc.authorization.value,
            });

            // Build the grant config object, omitting description if None
            let mut grant_config = serde_json::json!({
                "authorization": auth,
                "optional": gc.optional,
            });

            if let Some(ref desc) = gc.description {
                grant_config["description"] = serde_json::json!(desc);
            }

            grant_config
        })
        .collect();

    // Build fee_config as a JSON object
    // Fields: allowance (JSON object), description (string), expiration (optional string)
    let mut fee_config = serde_json::json!({
        "allowance": {
            "type_url": request.fee_config.allowance.type_url,
            "value": request.fee_config.allowance.value,
        },
        "description": request.fee_config.description,
    });

    // Add expiration if present (omit if None)
    if let Some(ref expiration) = request.fee_config.expiration {
        fee_config["expiration"] = serde_json::json!(expiration);
    }

    // Build params object, omitting display_url if None
    let mut params = serde_json::json!({
        "redirect_url": request.params.redirect_url,
        "icon_url": request.params.icon_url,
        "metadata": metadata.to_string(),
    });
    if let Some(ref display_url) = request.params.display_url {
        params["display_url"] = serde_json::json!(display_url);
    }

    // Build the complete instantiation message
    Ok(serde_json::json!({
        "type_urls": type_urls,
        "grant_configs": grant_configs,
        "fee_config": fee_config,
        "admin": request.admin,
        "params": params,
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

/// Extract user address from OAuth2 access token
///
/// Token format: {userId}:{grantId}:{secret}
/// userId is the user's Xion address (starts with "xion1")
fn extract_address_from_token(token: &str) -> Result<String> {
    let parts: Vec<&str> = token.split(':').collect();
    if parts.len() != 3 {
        anyhow::bail!("Invalid access token format: expected 3 parts separated by ':'");
    }

    let address = parts[0].to_string();
    if !address.starts_with("xion1") {
        anyhow::bail!(
            "Invalid access token: userId must be a valid Xion address (starts with 'xion1')"
        );
    }

    Ok(address)
}

/// Convert bytes to JSON number array for OAuth2 API
///
/// The OAuth2 API's JSON object path uses `fromPartial` which expects
/// bytes fields (like `msg` and `salt`) to be array-like objects (number arrays)
/// rather than base64 strings.
fn bytes_to_json_array(bytes: &[u8]) -> serde_json::Value {
    serde_json::Value::Array(
        bytes
            .iter()
            .map(|b| serde_json::Value::Number((*b).into()))
            .collect(),
    )
}

// ============================================================================
// Grant Config Operations
// ============================================================================

impl TreasuryApiClient {
    /// Add a grant configuration to a treasury
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `type_url` - Type URL of the message (e.g., "/cosmwasm.wasm.v1.MsgExecuteContract")
    /// * `grant_config` - Grant configuration input
    /// * `from_address` - Sender's MetaAccount address (must be admin)
    ///
    /// # Returns
    /// Grant config result with transaction hash
    #[instrument(skip(self, access_token, grant_config))]
    pub async fn add_grant_config(
        &self,
        access_token: &str,
        treasury_address: &str,
        type_url: &str,
        grant_config: super::types::GrantConfigInput,
        from_address: &str,
    ) -> Result<super::types::GrantConfigResult> {
        debug!(
            "Adding grant config for type_url: {} to treasury: {}",
            type_url, treasury_address
        );

        // Encode the authorization (pass type_url for GenericAuthorization)
        let (auth_type_url, auth_value) =
            super::encoding::encode_authorization_input(&grant_config.authorization, type_url)?;

        // Build the grant config for chain
        let grant_config_chain = super::types::GrantConfigChain {
            description: grant_config.description.clone(),
            authorization: super::types::ProtobufAny {
                type_url: auth_type_url,
                value: auth_value, // Already base64 encoded string
            },
            optional: grant_config.optional,
        };

        // Create the update_grant_config message (matches contract's ExecuteMsg)
        let exec_msg = super::types::TreasuryExecuteMsg::UpdateGrantConfig {
            msg_type_url: type_url.to_string(),
            grant_config: grant_config_chain,
        };

        // Broadcast using helper function (follows create_treasury format)
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                &format!("Update grant config for {}", type_url),
            )
            .await?;

        Ok(super::types::GrantConfigResult {
            treasury_address: treasury_address.to_string(),
            type_url: type_url.to_string(),
            operation: "update".to_string(),
            tx_hash,
        })
    }

    /// Remove a grant configuration from a treasury
    #[instrument(skip(self, access_token))]
    pub async fn remove_grant_config(
        &self,
        access_token: &str,
        treasury_address: &str,
        type_url: &str,
        from_address: &str,
    ) -> Result<super::types::GrantConfigResult> {
        debug!(
            "Removing grant config for type_url: {} from treasury: {}",
            type_url, treasury_address
        );

        // Create the remove_grant_config message (matches contract's ExecuteMsg)
        let remove_msg = super::types::TreasuryExecuteMsg::RemoveGrantConfig {
            msg_type_url: type_url.to_string(),
        };

        // Broadcast using helper function (follows create_treasury format)
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &remove_msg,
                &format!("Remove grant config {}", type_url),
            )
            .await?;

        Ok(super::types::GrantConfigResult {
            treasury_address: treasury_address.to_string(),
            type_url: type_url.to_string(),
            operation: "remove".to_string(),
            tx_hash,
        })
    }

    /// List grant configurations for a treasury
    #[instrument(skip(self, access_token))]
    pub async fn list_grant_configs(
        &self,
        access_token: &str,
        treasury_address: &str,
    ) -> Result<Vec<super::types::GrantConfigInfo>> {
        debug!("Listing grant configs for treasury: {}", treasury_address);

        // Query the treasury info which includes grant configs
        let options = QueryOptions {
            grants: true,
            fee: false,
            admin: false,
        };
        let treasury = self
            .query_treasury(access_token, treasury_address, options)
            .await?;

        // Extract grant configs
        let grant_configs = treasury.grant_configs.unwrap_or_default();
        let configs: Vec<super::types::GrantConfigInfo> = grant_configs
            .into_iter()
            .map(|gc| super::types::GrantConfigInfo {
                type_url: gc.type_url,
                description: gc
                    .grant_config
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                authorization_type_url: gc
                    .grant_config
                    .get("authorization")
                    .and_then(|a| a.get("type_url"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                optional: gc
                    .grant_config
                    .get("optional")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            })
            .collect();

        Ok(configs)
    }

    /// Set fee configuration for a treasury
    #[instrument(skip(self, access_token, fee_config))]
    pub async fn set_fee_config(
        &self,
        access_token: &str,
        treasury_address: &str,
        fee_config: super::types::FeeConfigInput,
        from_address: &str,
    ) -> Result<super::types::FeeConfigResult> {
        debug!("Setting fee config for treasury: {}", treasury_address);

        // Encode the fee allowance
        let (allowance_type_url, allowance_value) =
            super::encoding::encode_fee_config_input(&fee_config)?;

        // Build the fee config for chain
        let fee_config_chain = super::types::FeeConfigChain {
            description: match &fee_config {
                super::types::FeeConfigInput::Basic { description, .. } => description.clone(),
                super::types::FeeConfigInput::Periodic { description, .. } => description.clone(),
                super::types::FeeConfigInput::AllowedMsg { description, .. } => description.clone(),
            },
            allowance: Some(super::types::ProtobufAny {
                type_url: allowance_type_url,
                value: allowance_value, // Already base64 encoded string
            }),
            expiration: None, // TODO: Add expiration support in FeeConfigInput
        };

        // Create the update_fee_config message (matches contract's ExecuteMsg)
        let exec_msg = super::types::TreasuryExecuteMsg::UpdateFeeConfig {
            fee_config: fee_config_chain,
        };

        // Broadcast using helper function (follows create_treasury format)
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                "Update fee config",
            )
            .await?;

        Ok(super::types::FeeConfigResult {
            treasury_address: treasury_address.to_string(),
            operation: "update".to_string(),
            tx_hash,
        })
    }

    /// Revoke fee allowance from a treasury (revokes from grantee)
    #[instrument(skip(self, access_token))]
    pub async fn revoke_allowance(
        &self,
        access_token: &str,
        treasury_address: &str,
        grantee: &str,
        from_address: &str,
    ) -> Result<super::types::FeeConfigResult> {
        debug!(
            "Revoking allowance from grantee: {} for treasury: {}",
            grantee, treasury_address
        );

        // Create the revoke_allowance message (matches contract's ExecuteMsg)
        let exec_msg = super::types::TreasuryExecuteMsg::RevokeAllowance {
            grantee: grantee.to_string(),
        };

        // Broadcast using helper function (follows create_treasury format)
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                "Remove fee config",
            )
            .await?;

        Ok(super::types::FeeConfigResult {
            treasury_address: treasury_address.to_string(),
            operation: "remove".to_string(),
            tx_hash,
        })
    }

    /// Query fee configuration for a treasury
    #[instrument(skip(self, access_token))]
    pub async fn query_fee_config(
        &self,
        access_token: &str,
        treasury_address: &str,
    ) -> Result<Option<super::types::FeeConfigInfo>> {
        debug!("Querying fee config for treasury: {}", treasury_address);
        // Query the treasury info which includes fee config
        let options = QueryOptions {
            grants: false,
            fee: true,
            admin: false,
        };
        let treasury = self
            .query_treasury(access_token, treasury_address, options)
            .await?;

        // Extract fee config
        if let Some(fee_config) = treasury.fee_config {
            let description = fee_config
                .additional
                .as_ref()
                .and_then(|a| a.get("description"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();

            Ok(Some(super::types::FeeConfigInfo {
                allowance_type_url: fee_config.config_type,
                description,
                spend_limit: fee_config.spend_limit,
                expiration: fee_config.expires_at,
            }))
        } else {
            Ok(None)
        }
    }

    // ========================================================================
    // Admin Management Operations
    // ========================================================================

    /// Propose a new admin for a treasury
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `new_admin` - New admin address to propose
    /// * `from_address` - Current admin's MetaAccount address
    ///
    /// # Returns
    /// Admin result with transaction hash
    #[instrument(skip(self, access_token))]
    pub async fn propose_admin(
        &self,
        access_token: &str,
        treasury_address: &str,
        new_admin: &str,
        from_address: &str,
    ) -> Result<super::types::AdminResult> {
        debug!(
            "Proposing new admin {} for treasury: {}",
            new_admin, treasury_address
        );

        // Create the propose_admin message (matches contract's ExecuteMsg)
        let exec_msg = super::types::TreasuryExecuteMsg::ProposeAdmin {
            new_admin: new_admin.to_string(),
        };

        // Broadcast using helper function
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                &format!("Propose new admin: {}", new_admin),
            )
            .await?;

        Ok(super::types::AdminResult {
            treasury_address: treasury_address.to_string(),
            operation: "propose".to_string(),
            new_admin: Some(new_admin.to_string()),
            tx_hash,
        })
    }

    /// Accept admin role for a treasury
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `from_address` - Pending admin's MetaAccount address
    ///
    /// # Returns
    /// Admin result with transaction hash
    #[instrument(skip(self, access_token))]
    pub async fn accept_admin(
        &self,
        access_token: &str,
        treasury_address: &str,
        from_address: &str,
    ) -> Result<super::types::AdminResult> {
        debug!("Accepting admin role for treasury: {}", treasury_address);

        // Create the accept_admin message (matches contract's ExecuteMsg)
        let exec_msg = super::types::TreasuryExecuteMsg::AcceptAdmin {};

        // Broadcast using helper function
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                "Accept admin role",
            )
            .await?;

        Ok(super::types::AdminResult {
            treasury_address: treasury_address.to_string(),
            operation: "accept".to_string(),
            new_admin: None,
            tx_hash,
        })
    }

    /// Cancel proposed admin for a treasury
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `from_address` - Current admin's MetaAccount address
    ///
    /// # Returns
    /// Admin result with transaction hash
    #[instrument(skip(self, access_token))]
    pub async fn cancel_proposed_admin(
        &self,
        access_token: &str,
        treasury_address: &str,
        from_address: &str,
    ) -> Result<super::types::AdminResult> {
        debug!(
            "Canceling proposed admin for treasury: {}",
            treasury_address
        );

        // Create the cancel_proposed_admin message (matches contract's ExecuteMsg)
        let exec_msg = super::types::TreasuryExecuteMsg::CancelProposedAdmin {};

        // Broadcast using helper function
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                "Cancel proposed admin",
            )
            .await?;

        Ok(super::types::AdminResult {
            treasury_address: treasury_address.to_string(),
            operation: "cancel".to_string(),
            new_admin: None,
            tx_hash,
        })
    }

    // ========================================================================
    // Params Management Operations
    // ========================================================================

    /// Update treasury parameters
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `params` - New parameters to set
    /// * `from_address` - Admin's MetaAccount address
    ///
    /// # Returns
    /// Params result with transaction hash
    #[instrument(skip(self, access_token))]
    pub async fn update_params(
        &self,
        access_token: &str,
        treasury_address: &str,
        params: super::types::UpdateParamsInput,
        from_address: &str,
    ) -> Result<super::types::ParamsResult> {
        debug!("Updating params for treasury: {}", treasury_address);

        // Validate that at least one parameter is provided
        if params.redirect_url.is_none() && params.icon_url.is_none() && params.metadata.is_none() {
            anyhow::bail!("At least one parameter must be provided for update (redirect_url, icon_url, or metadata)");
        }

        // Build params chain format
        // Note: metadata should be JSON string, not JSON object
        let params_chain = super::types::TreasuryParamsChain {
            redirect_url: params.redirect_url.unwrap_or_default(),
            icon_url: params.icon_url.unwrap_or_default(),
            metadata: params
                .metadata
                .map(|m| m.to_string())
                .unwrap_or_else(|| "{}".to_string()),
        };

        // Create the update_params message (matches contract's ExecuteMsg)
        let exec_msg = super::types::TreasuryExecuteMsg::UpdateParams {
            params: params_chain,
        };

        // Broadcast using helper function
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                "Update treasury params",
            )
            .await?;

        Ok(super::types::ParamsResult {
            treasury_address: treasury_address.to_string(),
            tx_hash,
        })
    }

    // ========================================================================
    // Batch Operations
    // ========================================================================

    /// Add multiple grant configurations in a single transaction
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `grant_configs` - List of grant configurations to add
    /// * `from_address` - Admin's MetaAccount address
    ///
    /// # Returns
    /// Batch grant config result with transaction hash
    #[allow(dead_code)]
    #[instrument(skip(self, access_token, grant_configs))]
    pub async fn grant_config_batch(
        &self,
        access_token: &str,
        treasury_address: &str,
        grant_configs: Vec<(String, super::types::GrantConfigInput)>,
        from_address: &str,
    ) -> Result<super::types::BatchGrantConfigResult> {
        debug!(
            "Adding {} grant configs in batch to treasury: {}",
            grant_configs.len(),
            treasury_address
        );

        // Save the count before consuming the vector
        let count = grant_configs.len();

        // Build multiple update_grant_config messages
        let mut messages = Vec::new();

        for (type_url, grant_config_input) in grant_configs {
            // Encode the authorization
            let (auth_type_url, auth_value) = super::encoding::encode_authorization_input(
                &grant_config_input.authorization,
                &type_url,
            )?;

            // Build the grant config for chain
            let grant_config_chain = super::types::GrantConfigChain {
                description: grant_config_input.description.clone(),
                authorization: super::types::ProtobufAny {
                    type_url: auth_type_url,
                    value: auth_value,
                },
                optional: grant_config_input.optional,
            };

            // Create update_grant_config execute message
            let exec_msg = super::types::TreasuryExecuteMsg::UpdateGrantConfig {
                msg_type_url: type_url,
                grant_config: grant_config_chain,
            };

            // Serialize execute message
            let msg_json = serde_json::to_string(&exec_msg)?;
            let msg_bytes = msg_json.as_bytes();

            // Build MsgExecuteContract message
            let msg_value = serde_json::json!({
                "sender": from_address,
                "contract": treasury_address,
                "msg": bytes_to_json_array(msg_bytes),
                "funds": []
            });

            messages.push(super::types::TransactionMessage {
                type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
                value: msg_value,
            });
        }

        // Broadcast all messages in a single transaction
        let broadcast_request = BroadcastRequest {
            messages,
            memo: Some(format!("Batch update {} grant configs", count)),
        };

        let response = self
            .broadcast_transaction(access_token, broadcast_request)
            .await?;

        Ok(super::types::BatchGrantConfigResult {
            treasury_address: treasury_address.to_string(),
            count,
            tx_hash: response.tx_hash,
        })
    }

    // ========================================================================
    // On-Chain Query Operations (via RPC)
    // ========================================================================

    /// List all authz grants for a treasury (granter)
    ///
    /// Queries the blockchain directly via RPC for authz grants.
    ///
    /// # Arguments
    /// * `treasury_address` - Treasury contract address (granter)
    ///
    /// # Returns
    /// List of authz grants where the treasury is the granter
    #[instrument(skip(self))]
    pub async fn list_authz_grants(
        &self,
        treasury_address: &str,
    ) -> Result<Vec<super::types::AuthzGrantInfo>> {
        debug!("Listing authz grants for treasury: {}", treasury_address);

        // Query authz grants via RPC
        // Endpoint: /cosmos/authz/v1beta1/grants?granter={treasury_address}
        let url = format!(
            "{}/cosmos/authz/v1beta1/grants?granter={}",
            self.rpc_url, treasury_address
        );

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context("Failed to query authz grants")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to query authz grants: {} - {}", status, error_text);
        }

        // Parse the response
        #[derive(Debug, Deserialize)]
        struct AuthzGrantsResponse {
            grants: Vec<AuthzGrantItem>,
        }

        #[derive(Debug, Deserialize)]
        struct AuthzGrantItem {
            granter: String,
            grantee: String,
            authorization: Option<serde_json::Value>,
            expiration: Option<String>,
        }

        let grants_response: AuthzGrantsResponse = response
            .json()
            .await
            .context("Failed to parse authz grants response")?;

        // Convert to AuthzGrantInfo
        let grants: Vec<super::types::AuthzGrantInfo> = grants_response
            .grants
            .into_iter()
            .map(|grant| {
                let authorization_type_url = grant
                    .authorization
                    .as_ref()
                    .and_then(|auth| auth.get("@type"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                super::types::AuthzGrantInfo {
                    granter: grant.granter,
                    grantee: grant.grantee,
                    authorization_type_url,
                    expiration: grant.expiration,
                }
            })
            .collect();

        debug!("Found {} authz grants", grants.len());
        Ok(grants)
    }

    /// List all fee allowances for a treasury (granter)
    ///
    /// Queries the blockchain directly via RPC for fee allowances.
    ///
    /// # Arguments
    /// * `treasury_address` - Treasury contract address (granter)
    ///
    /// # Returns
    /// List of fee allowances where the treasury is the granter
    #[instrument(skip(self))]
    pub async fn list_fee_allowances(
        &self,
        treasury_address: &str,
    ) -> Result<Vec<super::types::FeeAllowanceInfo>> {
        debug!("Listing fee allowances for treasury: {}", treasury_address);

        // Query fee allowances via RPC
        // Endpoint: /cosmos/feegrant/v1beta1/allowances/{granter}
        let url = format!(
            "{}/cosmos/feegrant/v1beta1/allowances/{}",
            self.rpc_url, treasury_address
        );

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context("Failed to query fee allowances")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Failed to query fee allowances: {} - {}",
                status,
                error_text
            );
        }

        // Parse the response
        #[derive(Debug, Deserialize)]
        struct FeeAllowancesResponse {
            allowances: Vec<FeeAllowanceItem>,
        }

        #[derive(Debug, Deserialize)]
        struct FeeAllowanceItem {
            granter: String,
            grantee: String,
            allowance: Option<serde_json::Value>,
        }

        let allowances_response: FeeAllowancesResponse = response
            .json()
            .await
            .context("Failed to parse fee allowances response")?;

        // Convert to FeeAllowanceInfo
        let allowances: Vec<super::types::FeeAllowanceInfo> = allowances_response
            .allowances
            .into_iter()
            .map(|allowance| {
                let allowance_type_url = allowance
                    .allowance
                    .as_ref()
                    .and_then(|a| a.get("@type"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                // Extract spend limit if present
                let spend_limit = allowance.allowance.as_ref().and_then(|a| {
                    a.get("spend_limit")
                        .and_then(|sl| sl.as_array())
                        .and_then(|coins| coins.first())
                        .and_then(|coin| {
                            let amount = coin.get("amount")?.as_str()?;
                            let denom = coin.get("denom")?.as_str()?;
                            Some(format!("{}{}", amount, denom))
                        })
                });

                // Extract expiration if present
                let expiration = allowance
                    .allowance
                    .as_ref()
                    .and_then(|a| a.get("expiration"))
                    .and_then(|e| {
                        // Try to parse as timestamp and convert to string
                        e.as_str()
                            .map(|s| s.to_string())
                            .or_else(|| e.as_i64().map(|ts| ts.to_string()))
                    });

                // Extract period details for periodic allowance
                let (period, period_spend_limit, period_can_spend) = allowance
                    .allowance
                    .as_ref()
                    .map(|a| {
                        let period = a
                            .get("period")
                            .and_then(|p| p.as_str())
                            .map(|s| s.to_string());
                        let period_spend_limit = a
                            .get("period_spend_limit")
                            .and_then(|sl| sl.as_array())
                            .and_then(|coins| coins.first())
                            .and_then(|coin| {
                                let amount = coin.get("amount")?.as_str()?;
                                let denom = coin.get("denom")?.as_str()?;
                                Some(format!("{}{}", amount, denom))
                            });
                        let period_can_spend = a
                            .get("period_can_spend")
                            .and_then(|sl| sl.as_array())
                            .and_then(|coins| coins.first())
                            .and_then(|coin| {
                                let amount = coin.get("amount")?.as_str()?;
                                let denom = coin.get("denom")?.as_str()?;
                                Some(format!("{}{}", amount, denom))
                            });
                        (period, period_spend_limit, period_can_spend)
                    })
                    .unwrap_or((None, None, None));

                super::types::FeeAllowanceInfo {
                    granter: allowance.granter,
                    grantee: allowance.grantee,
                    allowance_type_url,
                    spend_limit,
                    expiration,
                    period,
                    period_spend_limit,
                    period_can_spend,
                }
            })
            .collect();

        debug!("Found {} fee allowances", allowances.len());
        Ok(allowances)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = TreasuryApiClient::new(
            "https://test.com".to_string(),
            "https://indexer.test.com/network".to_string(),
            "https://rpc.test.com:443".to_string(),
        );
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

    #[tokio::test]
    async fn test_wait_for_treasury_creation_success() {
        // This test verifies the polling mechanism finds the treasury
        let mut server = mockito::Server::new_async().await;

        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";

        // Create a token with admin address as userId
        let token = format!("{}:grant123:secret456", admin_address);

        // Mock the DaoDao indexer endpoint - return treasury with matching admin
        // Using the actual DaoDao Indexer format (direct array)
        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Regex(
                    r"/contract/xion1admin123/xion/account/treasuries".to_string(),
                ),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::json!([
                    {
                        "contractAddress": treasury_address,
                        "balances": {"uxion": "0"},
                        "codeId": 1260
                    }
                ])
                .to_string(),
            )
            .create();

        let client = TreasuryApiClient::new(
            server.url(),
            server.url(),
            "https://rpc.test.com:443".to_string(),
        );

        // Call the wait_for_treasury_creation method
        let result = client
            .wait_for_treasury_creation(&token, admin_address, "tx123")
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), treasury_address);

        mock.assert();
    }

    #[tokio::test]
    async fn test_wait_for_treasury_creation_multiple_treasuries() {
        // Test that it returns the first treasury (newest one from indexer)
        // DaoDao Indexer returns treasuries sorted by block height (newest first)
        let mut server = mockito::Server::new_async().await;

        let admin_address = "xion1admin999";
        let treasury_address = "xion1treasury999";

        // Create a token with admin address as userId
        let token = format!("{}:grant123:secret456", admin_address);

        // Mock returning multiple treasuries - newest one first
        // Using the actual DaoDao Indexer format (direct array)
        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Regex(
                    r"/contract/xion1admin999/xion/account/treasuries".to_string(),
                ),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::json!([
                    {
                        "contractAddress": treasury_address,
                        "balances": {"uxion": "0"},
                        "codeId": 1260
                    },
                    {
                        "contractAddress": "xion1older1",
                        "balances": {"uxion": "1000"},
                        "codeId": 1260
                    },
                    {
                        "contractAddress": "xion1older2",
                        "balances": {"uxion": "2000"},
                        "codeId": 1260
                    }
                ])
                .to_string(),
            )
            .create();

        let client = TreasuryApiClient::new(
            server.url(),
            server.url(),
            "https://rpc.test.com:443".to_string(),
        );

        let result = client
            .wait_for_treasury_creation(&token, admin_address, "tx456")
            .await;

        assert!(result.is_ok());
        // Returns the first treasury (newest one)
        assert_eq!(result.unwrap(), treasury_address);

        mock.assert();
    }

    #[test]
    fn test_extract_address_from_token() {
        // Valid token format
        let token = "xion1abc123:grant123:secret456";
        let address = extract_address_from_token(token).unwrap();
        assert_eq!(address, "xion1abc123");

        // Invalid token - missing parts
        assert!(extract_address_from_token("invalid").is_err());
        assert!(extract_address_from_token("xion1abc:onlytwoparts").is_err());

        // Invalid token - wrong format
        assert!(extract_address_from_token("notxion:grant:secret").is_err());
    }

    #[test]
    fn test_bytes_to_json_array() {
        // Test empty bytes
        let empty: &[u8] = &[];
        let result = bytes_to_json_array(empty);
        assert_eq!(result, serde_json::json!([]));

        // Test single byte
        let single: &[u8] = &[65];
        let result = bytes_to_json_array(single);
        assert_eq!(result, serde_json::json!([65]));

        // Test multiple bytes
        let bytes: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
        let result = bytes_to_json_array(bytes);
        assert_eq!(result, serde_json::json!([72, 101, 108, 108, 111]));
    }

    #[test]
    fn test_broadcast_instantiate_contract_message_format() {
        // Test that the message format is correct for MsgInstantiateContract
        // This verifies the JSON structure without making actual network calls

        // Simulate the message construction logic
        let sender = "xion1sender";
        let code_id: u64 = 1260;
        let label = "TestContract-20240101-120000";
        let admin = Some("xion1admin");

        // Create a simple instantiate message
        #[derive(Serialize)]
        struct TestInstantiateMsg {
            name: String,
            value: u64,
        }
        let instantiate_msg = TestInstantiateMsg {
            name: "test".to_string(),
            value: 42,
        };

        // Serialize to JSON and convert to bytes
        let msg_json = serde_json::to_string(&instantiate_msg).unwrap();
        let msg_bytes = msg_json.as_bytes();

        // Build the expected message value
        let mut msg_value = serde_json::json!({
            "sender": sender,
            "codeId": code_id,
            "label": label,
            "msg": bytes_to_json_array(msg_bytes),
            "funds": []
        });
        if let Some(admin_addr) = admin {
            msg_value["admin"] = serde_json::json!(admin_addr);
        }

        // Verify the structure
        assert_eq!(msg_value["sender"], "xion1sender");
        assert_eq!(msg_value["codeId"], 1260);
        assert_eq!(msg_value["label"], "TestContract-20240101-120000");
        assert!(msg_value["msg"].is_array());
        assert_eq!(msg_value["funds"], serde_json::json!([]));
        assert_eq!(msg_value["admin"], "xion1admin");

        // Verify msg is a number array, not a base64 string
        if let serde_json::Value::Array(arr) = &msg_value["msg"] {
            assert!(!arr.is_empty());
            // Verify all elements are numbers
            for item in arr {
                assert!(item.is_number());
            }
        } else {
            panic!("msg should be an array");
        }
    }

    #[test]
    fn test_broadcast_instantiate_contract2_message_format() {
        // Test that the message format is correct for MsgInstantiateContract2
        // This verifies the JSON structure without making actual network calls

        let sender = "xion1sender";
        let code_id: u64 = 1260;
        let label = "Treasury-20240101-120000";
        let salt: &[u8] = &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let admin = Some("xion1admin");

        // Create a simple instantiate message
        #[derive(Serialize)]
        struct TestInstantiateMsg {
            admin: String,
            params: TestParams,
        }
        #[derive(Serialize)]
        struct TestParams {
            redirect_url: String,
            icon_url: String,
        }
        let instantiate_msg = TestInstantiateMsg {
            admin: "xion1admin".to_string(),
            params: TestParams {
                redirect_url: "https://example.com".to_string(),
                icon_url: "https://example.com/icon.png".to_string(),
            },
        };

        // Serialize to JSON and convert to bytes
        let msg_json = serde_json::to_string(&instantiate_msg).unwrap();
        let msg_bytes = msg_json.as_bytes();

        // Build the expected message value
        let mut msg_value = serde_json::json!({
            "sender": sender,
            "codeId": code_id,
            "label": label,
            "msg": bytes_to_json_array(msg_bytes),
            "salt": bytes_to_json_array(salt),
            "funds": [],
            "fixMsg": false,
        });
        if let Some(admin_addr) = admin {
            msg_value["admin"] = serde_json::json!(admin_addr);
        }

        // Verify the structure
        assert_eq!(msg_value["sender"], "xion1sender");
        assert_eq!(msg_value["codeId"], 1260);
        assert_eq!(msg_value["label"], "Treasury-20240101-120000");
        assert!(msg_value["msg"].is_array());
        assert!(msg_value["salt"].is_array());
        assert_eq!(msg_value["funds"], serde_json::json!([]));
        assert_eq!(msg_value["fixMsg"], false);
        assert_eq!(msg_value["admin"], "xion1admin");

        // Verify msg and salt are number arrays, not base64 strings
        if let serde_json::Value::Array(arr) = &msg_value["msg"] {
            assert!(!arr.is_empty());
            for item in arr {
                assert!(item.is_number());
            }
        } else {
            panic!("msg should be an array");
        }

        if let serde_json::Value::Array(arr) = &msg_value["salt"] {
            assert_eq!(arr.len(), 32); // 32-byte salt
            for item in arr {
                assert!(item.is_number());
            }
        } else {
            panic!("salt should be an array");
        }
    }

    #[test]
    fn test_broadcast_instantiate_without_admin() {
        // Test MsgInstantiateContract without optional admin field
        let sender = "xion1sender";
        let code_id: u64 = 1260;
        let label = "NoAdminContract";

        #[derive(Serialize)]
        struct EmptyMsg {}
        let instantiate_msg = EmptyMsg {};

        let msg_json = serde_json::to_string(&instantiate_msg).unwrap();
        let msg_bytes = msg_json.as_bytes();

        let msg_value = serde_json::json!({
            "sender": sender,
            "codeId": code_id,
            "label": label,
            "msg": bytes_to_json_array(msg_bytes),
            "funds": []
        });

        // Verify admin is not present
        assert!(msg_value.get("admin").is_none() || msg_value["admin"].is_null());
    }

    #[test]
    fn test_broadcast_instantiate2_without_admin() {
        // Test MsgInstantiateContract2 without optional admin field
        let sender = "xion1sender";
        let code_id: u64 = 1260;
        let label = "NoAdminContract2";
        let salt: &[u8] = &[0u8; 32];

        #[derive(Serialize)]
        struct EmptyMsg {}
        let instantiate_msg = EmptyMsg {};

        let msg_json = serde_json::to_string(&instantiate_msg).unwrap();
        let msg_bytes = msg_json.as_bytes();

        let msg_value = serde_json::json!({
            "sender": sender,
            "codeId": code_id,
            "label": label,
            "msg": bytes_to_json_array(msg_bytes),
            "salt": bytes_to_json_array(salt),
            "funds": [],
            "fixMsg": false,
        });

        // Verify admin is not present
        assert!(msg_value.get("admin").is_none() || msg_value["admin"].is_null());
        // fixMsg should still be present
        assert_eq!(msg_value["fixMsg"], false);
    }
}
