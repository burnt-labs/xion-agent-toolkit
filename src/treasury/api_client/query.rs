//! Query operations for treasury contracts.
//!
//! This module provides methods for:
//! - Listing user's treasury contracts via DaoDao Indexer
//! - Querying specific treasury details
//! - Querying smart contracts via RPC
//! - Getting code info from the chain

use chrono::DateTime;
use serde::Deserialize;
use tracing::{debug, instrument};

use crate::shared::error::{NetworkError, TreasuryError, XionResult};
use crate::treasury::types::{QueryOptions, TreasuryInfo, TreasuryListItem, TreasuryParams};

use super::helpers::{base64_decode, base64_encode, extract_address_from_token};
use super::types::{CodeInfo, CodeInfoResponse, IndexerTreasuryItem, TreasuryMetadataJson};

impl super::TreasuryApiClient {
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
    ///     "https://api.testnet-2.burnt.com".to_string(),
    /// );
    /// let treasuries = client.list_treasuries("access_token_123").await?;
    /// println!("Found {} treasuries", treasuries.len());
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn list_treasuries(&self, access_token: &str) -> XionResult<Vec<TreasuryListItem>> {
        // Extract user address from access token (format: {userId}:{grantId}:{secret})
        let user_address = extract_address_from_token(access_token)?;

        // Use DaoDao Indexer to list treasuries
        let url = format!(
            "{}/contract/{}/xion/account/treasuries",
            self.indexer_url, user_address
        );
        debug!("Listing treasuries from DaoDao Indexer: {}", url);

        let response = self.http_client.get(&url).send().await.map_err(|e| {
            NetworkError::RequestFailed(format!(
                "Failed to send list treasuries request to indexer: {}",
                e
            ))
        })?;

        let status = response.status();
        debug!("List treasuries response status: {}", status);

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(NetworkError::InvalidResponse(format!(
                "List treasuries failed with status {}: {}",
                status, error_text
            ))
            .into());
        }

        // Parse indexer response - it returns a direct array, not wrapped in an object
        let indexer_items: Vec<IndexerTreasuryItem> = response.json().await.map_err(|e| {
            NetworkError::InvalidResponse(format!("Failed to parse indexer response: {}", e))
        })?;

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
    ///     "https://api.testnet-2.burnt.com".to_string(),
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
    ) -> XionResult<TreasuryInfo> {
        // Use DaoDao Indexer to query treasury info
        // Extract user address from token to build the list URL
        let user_address = extract_address_from_token(access_token)?;

        let url = format!(
            "{}/contract/{}/xion/account/treasuries",
            self.indexer_url, user_address
        );
        debug!("Querying treasury from DaoDao Indexer: {}", url);

        let response = self.http_client.get(&url).send().await.map_err(|e| {
            NetworkError::RequestFailed(format!(
                "Failed to send query treasury request to indexer: {}",
                e
            ))
        })?;

        let status = response.status();
        debug!("Query treasury response status: {}", status);

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(NetworkError::InvalidResponse(format!(
                "Query treasury failed with status {}: {}",
                status, error_text
            ))
            .into());
        }

        // Parse indexer response - direct array
        let indexer_items: Vec<IndexerTreasuryItem> = response.json().await.map_err(|e| {
            NetworkError::InvalidResponse(format!("Failed to parse indexer response: {}", e))
        })?;

        // Find the specific treasury by address
        let item = indexer_items
            .iter()
            .find(|item| item.contract_address == address)
            .ok_or_else(|| TreasuryError::NotFound(address.to_string()))?;

        // Convert to TreasuryInfo
        let treasury = self.indexer_item_to_treasury_info(item)?;

        debug!("Successfully queried treasury: {}", treasury.address);
        Ok(treasury)
    }

    /// Convert IndexerTreasuryItem to TreasuryInfo
    fn indexer_item_to_treasury_info(
        &self,
        item: &IndexerTreasuryItem,
    ) -> XionResult<TreasuryInfo> {
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
    /// # Implementation Details
    /// - Endpoint: GET {rpc_url}/cosmwasm/wasm/v1/contract/{address}/smart/{base64_query}
    /// - Response is double-encoded:
    ///   - REST returns `{ "data": "base64_encoded_result" }`
    ///   - The base64 decodes to the actual JSON result
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
    ///     "https://api.testnet-2.burnt.com".to_string(),
    /// );
    /// let query = serde_json::json!({ "balance": {} });
    /// let result = client.query_contract_smart("xion1contract...", &query).await?;
    /// println!("Query result: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, query_msg))]
    pub async fn query_contract_smart(
        &self,
        contract_address: &str,
        query_msg: &serde_json::Value,
    ) -> XionResult<serde_json::Value> {
        debug!("Querying contract: {}", contract_address);

        // Step 1: Serialize query message to JSON string
        let query_json = serde_json::to_string(query_msg).map_err(|e| {
            TreasuryError::OperationFailed(format!("Failed to serialize query message: {}", e))
        })?;
        debug!("Query message JSON: {}", query_json);

        // Step 2: Base64 encode the JSON string
        let query_base64 = base64_encode(&query_json);

        // Step 3: Build URL
        let url = format!(
            "{}/cosmwasm/wasm/v1/contract/{}/smart/{}",
            self.rest_url, contract_address, query_base64
        );
        debug!("Query URL: {}", url);

        // Step 4: Make GET request
        let response = self.http_client.get(&url).send().await.map_err(|e| {
            NetworkError::RequestFailed(format!("Failed to send contract query request: {}", e))
        })?;

        let status = response.status();
        debug!("Query response status: {}", status);

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(NetworkError::InvalidResponse(format!(
                "Contract query failed with status {}: {}",
                status, error_text
            ))
            .into());
        }

        // Step 5: Parse response - two possible formats:
        // 1. Double-encoded: { "data": "base64_encoded_result" }
        // 2. Direct JSON: { "data": { ... actual result ... } }
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        enum QueryResponse {
            Base64 { data: String },
            Direct { data: serde_json::Value },
        }

        let query_response: QueryResponse = response.json().await.map_err(|e| {
            NetworkError::InvalidResponse(format!("Failed to parse query response: {}", e))
        })?;

        // Step 6: Decode or extract the result
        let result: serde_json::Value = match query_response {
            QueryResponse::Base64 { data } => {
                // Base64 decode the data field, then parse as JSON
                let decoded = base64_decode(&data).map_err(|e| {
                    TreasuryError::OperationFailed(format!(
                        "Failed to decode base64 query result: {}",
                        e
                    ))
                })?;
                serde_json::from_str(&decoded).map_err(|e| {
                    TreasuryError::OperationFailed(format!(
                        "Failed to parse decoded query result as JSON: {}",
                        e
                    ))
                })?
            }
            QueryResponse::Direct { data } => data,
        };

        debug!("Query result: {:?}", result);
        Ok(result)
    }

    /// Get code info including checksum from the chain
    ///
    /// Fetches code details from the RPC endpoint to get the checksum
    /// needed for instantiate2 address prediction.
    ///
    /// # Arguments
    /// * `code_id` - The code ID to fetch info for
    ///
    /// # Returns
    /// Code info including checksum (as lowercase hex string)
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
    ///     "https://api.testnet-2.burnt.com".to_string(),
    /// );
    /// let code_info = client.get_code_info(522).await?;
    /// println!("Checksum: {}", code_info.checksum);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_code_info(&self, code_id: u64) -> XionResult<CodeInfo> {
        let url = format!("{}/cosmwasm/wasm/v1/code/{}", self.rest_url, code_id);
        debug!("Fetching code info from: {}", url);

        let response = self.http_client.get(&url).send().await.map_err(|e| {
            NetworkError::RequestFailed(format!("Failed to send code info request: {}", e))
        })?;

        let status = response.status();
        debug!("Code info response status: {}", status);

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(NetworkError::InvalidResponse(format!(
                "Code info query failed with status {}: {}",
                status, error_text
            ))
            .into());
        }

        // Parse response
        let code_response: CodeInfoResponse = response.json().await.map_err(|e| {
            NetworkError::InvalidResponse(format!("Failed to parse code info response: {}", e))
        })?;

        // Convert checksum to lowercase hex
        let checksum = code_response.code_info.data_hash.to_lowercase();

        Ok(CodeInfo {
            code_id,
            creator: code_response.code_info.creator,
            checksum,
        })
    }
}
