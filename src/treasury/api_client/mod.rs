//! Treasury API Client
//!
//! Client for communicating with Xion's Treasury API Service.
//! Supports listing, querying, and managing treasury contracts.

use reqwest::Client;
use serde::Serialize;
use tracing::{debug, instrument};

use crate::shared::error::{NetworkError, XionResult};
use crate::treasury::types::{BroadcastRequest, BroadcastResponse, Coin};

// Sub-modules for organized code
mod admin;
mod fund;
mod grant;
mod helpers;
mod instantiate;
mod query;
pub(crate) mod types;

// Re-export public types
pub use types::CodeInfo;

/// Treasury API Client for Xion
///
/// Handles communication with the Treasury API service for:
/// - Listing user's treasury contracts (via DaoDao Indexer)
/// - Querying treasury details
/// - Creating new treasury contracts
/// - Funding treasury contracts
#[derive(Debug, Clone)]
pub struct TreasuryApiClient {
    /// Base URL of the Treasury API service
    pub(crate) base_url: String,
    /// DaoDao Indexer URL for treasury listing
    pub(crate) indexer_url: String,
    /// REST URL for on-chain queries
    pub(crate) rest_url: String,
    /// HTTP client for making requests
    pub(crate) http_client: Client,
}

impl TreasuryApiClient {
    /// Create a new Treasury API client
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the Treasury API service (e.g., "https://oauth2.testnet.burnt.com")
    /// * `indexer_url` - DaoDao Indexer URL for listing treasuries (e.g., "https://daodaoindexer.burnt.com/xion-testnet-2")
    /// * `rest_url` - REST URL for on-chain queries (e.g., "https://api.xion-testnet-2.burnt.com")
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::api::TreasuryApiClient;
    ///
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://api.testnet-2.burnt.com".to_string(),
    /// );
    /// ```
    pub fn new(base_url: String, indexer_url: String, rest_url: String) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url,
            indexer_url,
            rest_url,
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
    /// * `funds` - Optional funds to send with the execution
    /// * `memo` - Transaction memo
    ///
    /// # Returns
    /// Transaction hash on success
    pub async fn broadcast_execute_contract<T: Serialize>(
        &self,
        access_token: &str,
        sender: &str,
        contract: &str,
        execute_msg: &T,
        funds: Option<&[Coin]>,
        memo: &str,
    ) -> XionResult<String> {
        // Serialize execute message to JSON, then convert to number array
        // (OAuth2 API's JSON object path uses `fromPartial` which expects
        // bytes fields as array-like objects, not base64 strings)
        let msg_json = serde_json::to_string(execute_msg).map_err(|e| {
            crate::shared::error::TreasuryError::OperationFailed(format!(
                "Failed to serialize execute message: {}",
                e
            ))
        })?;
        let msg_bytes = msg_json.as_bytes();

        debug!("Execute message JSON:\n{}", msg_json);

        // Build funds array if provided
        let funds_value = if let Some(f) = funds {
            f.iter()
                .map(|coin| {
                    serde_json::json!({
                        "denom": coin.denom,
                        "amount": coin.amount
                    })
                })
                .collect()
        } else {
            vec![]
        };

        let msg_value = serde_json::json!({
            "sender": sender,
            "contract": contract,
            "msg": helpers::bytes_to_json_array(msg_bytes),  // Number array, not base64 string
            "funds": funds_value
        });

        let broadcast_request = BroadcastRequest {
            messages: vec![crate::treasury::types::TransactionMessage {
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
    ) -> XionResult<String> {
        // Serialize instantiate message to JSON, then convert to number array
        let msg_json = serde_json::to_string(instantiate_msg).map_err(|e| {
            crate::shared::error::TreasuryError::OperationFailed(format!(
                "Failed to serialize instantiate message: {}",
                e
            ))
        })?;
        let msg_bytes = msg_json.as_bytes();

        debug!("Instantiate message JSON:\n{}", msg_json);

        // Build MsgInstantiateContract message
        // Note: codeId is number, msg is number array (not base64 string)
        let mut msg_value = serde_json::json!({
            "sender": sender,
            "codeId": code_id,  // number, not string
            "label": label,
            "msg": helpers::bytes_to_json_array(msg_bytes),  // Number array
            "funds": []
        });

        // Add optional admin field
        if let Some(admin_addr) = admin {
            msg_value["admin"] = serde_json::json!(admin_addr);
        }

        let broadcast_request = BroadcastRequest {
            messages: vec![crate::treasury::types::TransactionMessage {
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
    ) -> XionResult<String> {
        // Serialize instantiate message to JSON, then convert to number array
        let msg_json = serde_json::to_string(instantiate_msg).map_err(|e| {
            crate::shared::error::TreasuryError::OperationFailed(format!(
                "Failed to serialize instantiate2 message: {}",
                e
            ))
        })?;
        let msg_bytes = msg_json.as_bytes();

        debug!("Instantiate2 message JSON:\n{}", msg_json);

        // Build MsgInstantiateContract2 message
        // Note: codeId is number, msg and salt are number arrays (not base64 strings)
        let mut msg_value = serde_json::json!({
            "sender": sender,
            "codeId": code_id,  // number, not string
            "label": label,
            "msg": helpers::bytes_to_json_array(msg_bytes),  // Number array
            "salt": helpers::bytes_to_json_array(salt),       // Number array
            "funds": [],
            "fixMsg": false,
        });

        // Add optional admin field
        if let Some(admin_addr) = admin {
            msg_value["admin"] = serde_json::json!(admin_addr);
        }

        let broadcast_request = BroadcastRequest {
            messages: vec![crate::treasury::types::TransactionMessage {
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
    ///     "https://api.testnet-2.burnt.com".to_string(),
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
    ) -> XionResult<BroadcastResponse> {
        let url = format!("{}/api/v1/transaction", self.base_url);
        debug!("Broadcasting transaction to: {}", url);

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                NetworkError::RequestFailed(format!(
                    "Failed to send broadcast transaction request: {}",
                    e
                ))
            })?;

        let status = response.status();
        debug!("Broadcast transaction response status: {}", status);

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(NetworkError::InvalidResponse(format!(
                "Broadcast transaction failed with status {}: {}",
                status, error_text
            ))
            .into());
        }

        let result: BroadcastResponse = response.json().await.map_err(|e| {
            NetworkError::InvalidResponse(format!(
                "Failed to parse broadcast transaction response: {}",
                e
            ))
        })?;

        debug!("Successfully broadcast transaction: {}", result.tx_hash);
        Ok(result)
    }

    /// Get the base URL
    #[allow(dead_code)]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::helpers::{
        base64_decode, base64_encode, bytes_to_json_array, extract_address_from_token, parse_coin,
    };
    use super::types::{TreasuryListResponse, TreasuryQueryResponse};
    use super::*;
    use crate::treasury::types::QueryOptions;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_client_creation() {
        let client = TreasuryApiClient::new(
            "https://test.com".to_string(),
            "https://indexer.test.com/network".to_string(),
            "https://api.test.com".to_string(),
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
        // This test verifies the polling mechanism finds and verifies the treasury
        let mock_server = MockServer::start().await;

        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";

        // Create a token with admin address as userId
        let token = format!("{}:grant123:secret456", admin_address);

        // Mock the DaoDao indexer endpoint - return treasury with matching address
        // Using the actual DaoDao Indexer format (direct array)
        Mock::given(method("GET"))
            .and(path_regex(
                r"/contract/xion1admin123/xion/account/treasuries",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "contractAddress": treasury_address,
                    "balances": {"uxion": "0"},
                    "codeId": 1260
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = TreasuryApiClient::new(
            mock_server.uri(),
            mock_server.uri(),
            "https://api.test.com".to_string(),
        );

        // Call the wait_for_treasury_creation method with expected address
        let result = client
            .wait_for_treasury_creation(&token, treasury_address, "tx123")
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), treasury_address);
    }

    #[tokio::test]
    async fn test_wait_for_treasury_creation_multiple_treasuries() {
        // Test that it finds and verifies the correct treasury by matching expected address
        // This ensures we don't return the wrong treasury in a race condition
        let mock_server = MockServer::start().await;

        let admin_address = "xion1admin999";
        let expected_treasury_address = "xion1treasury999";

        // Create a token with admin address as userId
        let token = format!("{}:grant123:secret456", admin_address);

        // Mock returning multiple treasuries - with expected one NOT first
        // This tests that we find the correct treasury by address, not just first
        Mock::given(method("GET"))
            .and(path_regex(
                r"/contract/xion1admin999/xion/account/treasuries",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "contractAddress": "xion1older1",
                    "balances": {"uxion": "1000"},
                    "codeId": 1260
                },
                {
                    "contractAddress": expected_treasury_address,
                    "balances": {"uxion": "0"},
                    "codeId": 1260
                },
                {
                    "contractAddress": "xion1older2",
                    "balances": {"uxion": "2000"},
                    "codeId": 1260
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = TreasuryApiClient::new(
            mock_server.uri(),
            mock_server.uri(),
            "https://api.test.com".to_string(),
        );

        // Call with the expected address - should find it even though it's not first
        let result = client
            .wait_for_treasury_creation(&token, expected_treasury_address, "tx456")
            .await;

        assert!(result.is_ok());
        // Returns the treasury matching expected address
        assert_eq!(result.unwrap(), expected_treasury_address);
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

    // ========================================================================
    // Contract Query Tests
    // ========================================================================

    #[test]
    fn test_base64_encode() {
        // Test simple string
        let input = "hello";
        let encoded = base64_encode(input);
        assert_eq!(encoded, "aGVsbG8=");

        // Test JSON string
        let json_input = r#"{"balance":{}}"#;
        let encoded_json = base64_encode(json_input);
        assert_eq!(encoded_json, "eyJiYWxhbmNlIjp7fX0=");

        // Test empty string
        let empty = "";
        let encoded_empty = base64_encode(empty);
        assert_eq!(encoded_empty, "");
    }

    #[test]
    fn test_base64_decode() {
        // Test simple string
        let encoded = "aGVsbG8=";
        let decoded = base64_decode(encoded).unwrap();
        assert_eq!(decoded, "hello");

        // Test JSON string
        let encoded_json = "eyJiYWxhbmNlIjp7fX0=";
        let decoded_json = base64_decode(encoded_json).unwrap();
        assert_eq!(decoded_json, r#"{"balance":{}}"#);

        // Test empty string
        let decoded_empty = base64_decode("").unwrap();
        assert_eq!(decoded_empty, "");
    }

    #[test]
    fn test_base64_decode_invalid() {
        // Invalid base64 should fail
        let result = base64_decode("not-valid-base64!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_base64_roundtrip() {
        // Test that encode -> decode returns original
        let original = r#"{"balance":{"address":"xion1abc123"}}"#;
        let encoded = base64_encode(original);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[tokio::test]
    async fn test_query_contract_smart_url_construction() {
        // Test that the URL is correctly constructed
        // This verifies the base64 encoding and URL building logic

        let query_msg = serde_json::json!({"balance": {}});

        // Verify the query message serializes correctly
        let query_json = serde_json::to_string(&query_msg).unwrap();
        assert_eq!(query_json, r#"{"balance":{}}"#);

        // Verify base64 encoding
        let query_base64 = base64_encode(&query_json);
        assert_eq!(query_base64, "eyJiYWxhbmNlIjp7fX0=");

        // The expected URL format:
        // {rpc_url}/cosmwasm/wasm/v1/contract/{address}/smart/{base64}
        // We can verify the components are correct
        assert!(query_base64.contains("eyJiYWxhbmNl"));
    }

    #[tokio::test]
    async fn test_query_contract_smart_double_encoded_response() {
        // Test parsing of double-encoded response format
        // REST returns: { "data": "base64_encoded_result" }
        // The base64 decodes to the actual JSON result

        // Simulate a contract query response
        let inner_result = serde_json::json!({
            "balance": "1000000"
        });
        let inner_json = serde_json::to_string(&inner_result).unwrap();
        let encoded_data = base64_encode(&inner_json);

        // This is what the REST API returns
        let response_body = serde_json::json!({
            "data": encoded_data
        });

        // Parse the response
        #[derive(Debug, serde::Deserialize)]
        struct QueryResponse {
            data: String,
        }
        let query_response: QueryResponse =
            serde_json::from_value(response_body).expect("Failed to parse response");

        // Decode the data field
        let decoded = base64_decode(&query_response.data).expect("Failed to decode base64");

        // Parse as JSON
        let result: serde_json::Value =
            serde_json::from_str(&decoded).expect("Failed to parse JSON");

        // Verify the result
        assert_eq!(result["balance"], "1000000");
    }

    #[tokio::test]
    async fn test_query_contract_smart_with_mock_server() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Prepare the response
        let inner_result = serde_json::json!({
            "balance": "5000000uxion"
        });
        let inner_json = serde_json::to_string(&inner_result).unwrap();
        let encoded_data = base64_encode(&inner_json);

        // Mock the REST endpoint
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({ "data": encoded_data })),
            )
            .mount(&mock_server)
            .await;

        let client =
            TreasuryApiClient::new(mock_server.uri(), mock_server.uri(), mock_server.uri());

        // Query the contract
        let query_msg = serde_json::json!({ "balance": {} });
        let result = client.query_contract_smart("xion1test", &query_msg).await;

        assert!(result.is_ok());
        let result_value = result.unwrap();
        assert_eq!(result_value["balance"], "5000000uxion");
    }
}
