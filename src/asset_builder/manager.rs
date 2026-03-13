//! Asset Builder Manager
//!
//! High-level manager for NFT contract deployment and management.
//!
//! ## Features
//!
//! - Create NFT collections (instantiate2 for predictable addresses)
//! - Mint tokens
//! - Query contract state

use anyhow::Result;
use bech32::{decode, encode, Bech32, Hrp};
use cosmwasm_std::{instantiate2_address, CanonicalAddr, HexBinary};
use rand::RngCore;
use tracing::{debug, instrument};

use crate::config::NetworkConfig;
use crate::oauth::OAuthClient;
use crate::treasury::api_client::TreasuryApiClient;

use super::code_ids::get_code_id;
use super::types::{
    AssetBuilderError, AssetType, CreateCollectionInput, CreateCollectionResult,
    Cw721InstantiateMsg, Cw721MintMsg, MintContent, MintTokenInput, MintTokenResult, QueryResult,
};

/// Asset Builder Manager
///
/// High-level manager for NFT operations that integrates:
/// - OAuth2 authentication with automatic token refresh
/// - Treasury API client for transaction broadcasting
/// - Network configuration for code ID resolution
#[derive(Debug)]
pub struct AssetBuilderManager {
    /// OAuth client for authentication
    oauth_client: OAuthClient,
    /// API client for broadcasting transactions
    api_client: TreasuryApiClient,
    /// Network configuration
    config: NetworkConfig,
}

impl AssetBuilderManager {
    /// Create a new Asset Builder manager
    ///
    /// # Arguments
    /// * `oauth_client` - OAuth client for authentication
    /// * `config` - Network configuration
    pub fn new(oauth_client: OAuthClient, config: NetworkConfig) -> Self {
        let api_client = TreasuryApiClient::new(
            config.oauth_api_url.clone(),
            config.indexer_url.clone(),
            config.rpc_url.clone(),
        );

        Self {
            oauth_client,
            api_client,
            config,
        }
    }

    /// Check if user is authenticated
    pub fn is_authenticated(&self) -> Result<bool> {
        self.oauth_client.is_authenticated()
    }

    /// Create a new NFT collection
    ///
    /// Creates a new NFT collection using instantiate2 for predictable addresses.
    ///
    /// # Arguments
    /// * `input` - Collection creation input
    ///
    /// # Returns
    /// Collection creation result with contract address and transaction hash
    #[instrument(skip(self, input))]
    pub async fn create_collection(
        &self,
        input: CreateCollectionInput,
    ) -> Result<CreateCollectionResult, AssetBuilderError> {
        debug!("Creating NFT collection: {} ({})", input.name, input.symbol);

        // Phase 1: Only support cw721-base
        if input.asset_type != AssetType::Cw721Base {
            let supported: Vec<&'static str> =
                AssetType::all().iter().map(|t| t.as_str()).collect();
            return Err(AssetBuilderError::InvalidAssetType(format!(
                "Phase 1 only supports 'cw721-base'. Got: '{}'. Supported types: {}",
                input.asset_type,
                supported.join(", ")
            )));
        }

        // Get user credentials
        let credentials = self
            .oauth_client
            .get_credentials()
            .map_err(|e| AssetBuilderError::ApiError(e.to_string()))?
            .ok_or(AssetBuilderError::NotAuthenticated)?;

        let sender = credentials
            .xion_address
            .ok_or(AssetBuilderError::NotAuthenticated)?;

        // Get code ID
        let code_id = match input.code_id {
            Some(id) => id,
            None => get_code_id(input.asset_type, &self.config)?,
        };

        // Determine minter address (default to sender)
        let minter = input.minter.unwrap_or_else(|| sender.clone());

        // Build instantiate message
        let instantiate_msg = Cw721InstantiateMsg {
            name: input.name.clone(),
            symbol: input.symbol.clone(),
            minter: minter.clone(),
        };

        // Generate salt for instantiate2
        let salt_bytes = if let Some(ref salt_hex) = input.salt {
            // Parse hex-encoded salt (expecting 32 bytes / 64 hex characters)
            hex::decode(salt_hex).map_err(|e| {
                AssetBuilderError::ApiError(format!(
                    "Invalid salt format: {}. Expected 64 hex characters (32 bytes). Error: {}",
                    salt_hex, e
                ))
            })?
        } else {
            // Generate random 32-byte salt
            let mut buf = vec![0u8; 32];
            rand::thread_rng().fill_bytes(&mut buf);
            buf
        };

        // Build label
        let label = format!("{}-{}", input.name, input.symbol);

        // Get valid access token
        let access_token = self
            .oauth_client
            .get_valid_token()
            .await
            .map_err(|e| AssetBuilderError::ApiError(e.to_string()))?;

        // Broadcast instantiate2 transaction
        let tx_hash = self
            .broadcast_instantiate2(
                &access_token,
                &sender,
                code_id,
                &instantiate_msg,
                &label,
                &salt_bytes,
            )
            .await?;

        // Compute predicted contract address using instantiate2 algorithm
        let contract_address = self
            .compute_instantiate2_address(code_id, &sender, &salt_bytes)
            .await?;

        Ok(CreateCollectionResult {
            success: true,
            contract_address,
            tx_hash,
            code_id,
            name: input.name,
            symbol: input.symbol,
            minter,
            salt: hex::encode(&salt_bytes),
        })
    }

    /// Mint a new NFT token
    ///
    /// Mints a new token in an existing NFT collection.
    ///
    /// # Arguments
    /// * `input` - Mint input with contract address, token ID, owner, etc.
    ///
    /// # Returns
    /// Mint result with transaction hash
    #[instrument(skip(self, input))]
    pub async fn mint_token(
        &self,
        input: MintTokenInput,
    ) -> Result<MintTokenResult, AssetBuilderError> {
        debug!(
            "Minting token {} in contract {}",
            input.token_id, input.contract
        );

        // Get user credentials
        let credentials = self
            .oauth_client
            .get_credentials()
            .map_err(|e| AssetBuilderError::ApiError(e.to_string()))?
            .ok_or(AssetBuilderError::NotAuthenticated)?;

        let sender = credentials
            .xion_address
            .ok_or(AssetBuilderError::NotAuthenticated)?;

        // Build mint message
        let mint_msg = Cw721MintMsg {
            mint: MintContent {
                token_id: input.token_id.clone(),
                owner: input.owner.clone(),
                token_uri: input.token_uri.clone(),
                extension: input.extension.clone(),
            },
        };

        // Get valid access token
        let access_token = self
            .oauth_client
            .get_valid_token()
            .await
            .map_err(|e| AssetBuilderError::ApiError(e.to_string()))?;

        // Broadcast execute contract transaction
        let tx_hash = self
            .broadcast_execute(&access_token, &sender, &input.contract, &mint_msg)
            .await?;

        Ok(MintTokenResult {
            success: true,
            contract_address: input.contract,
            token_id: input.token_id,
            owner: input.owner,
            tx_hash,
        })
    }

    /// Query an NFT contract
    ///
    /// Queries an NFT contract using a smart query.
    ///
    /// # Arguments
    /// * `contract` - Contract address
    /// * `msg` - Query message as JSON value
    ///
    /// # Returns
    /// Query result with response data
    #[instrument(skip(self, msg))]
    pub async fn query_contract(
        &self,
        contract: &str,
        msg: &serde_json::Value,
    ) -> Result<QueryResult, AssetBuilderError> {
        debug!("Querying contract: {}", contract);

        // Perform smart query (no auth required for queries)
        let response = self
            .api_client
            .query_contract_smart(contract, msg)
            .await
            .map_err(|e| AssetBuilderError::QueryFailed(e.to_string()))?;

        Ok(QueryResult {
            success: true,
            contract_address: contract.to_string(),
            response,
        })
    }

    // ========================================================================
    // Private Helpers
    // ========================================================================

    /// Broadcast instantiate2 transaction
    ///
    /// Uses the OAuth2 API to broadcast an instantiate2 message.
    async fn broadcast_instantiate2<T: serde::Serialize + std::fmt::Debug>(
        &self,
        access_token: &str,
        sender: &str,
        code_id: u64,
        instantiate_msg: &T,
        label: &str,
        salt: &[u8],
    ) -> Result<String, AssetBuilderError> {
        // Serialize instantiate message to JSON, then convert to number array
        let msg_json = serde_json::to_string(instantiate_msg)?;
        let msg_bytes = msg_json.as_bytes();

        debug!("Instantiate2 message JSON:\n{}", msg_json);

        // Build MsgInstantiateContract2 message
        // Note: codeId is number, msg and salt are number arrays (not base64 strings)
        let msg_value = serde_json::json!({
            "sender": sender,
            "codeId": code_id,  // number, not string
            "label": label,
            "msg": Self::bytes_to_json_array(msg_bytes),  // Number array
            "salt": Self::bytes_to_json_array(salt),       // Number array
            "funds": []
        });

        let broadcast_request = crate::treasury::types::BroadcastRequest {
            messages: vec![crate::treasury::types::TransactionMessage {
                type_url: "/cosmwasm.wasm.v1.MsgInstantiateContract2".to_string(),
                value: msg_value,
            }],
            memo: Some("Create NFT collection via Xion Agent Toolkit".to_string()),
        };

        let response = self
            .api_client
            .broadcast_transaction(access_token, broadcast_request)
            .await
            .map_err(|e| AssetBuilderError::InstantiationFailed(e.to_string()))?;

        Ok(response.tx_hash)
    }

    /// Broadcast execute contract transaction
    ///
    /// Uses the OAuth2 API to broadcast an execute contract message.
    async fn broadcast_execute<T: serde::Serialize + std::fmt::Debug>(
        &self,
        access_token: &str,
        sender: &str,
        contract: &str,
        execute_msg: &T,
    ) -> Result<String, AssetBuilderError> {
        // Serialize execute message to JSON, then convert to number array
        let msg_json = serde_json::to_string(execute_msg)?;
        let msg_bytes = msg_json.as_bytes();

        debug!("Execute message JSON:\n{}", msg_json);

        // Build MsgExecuteContract message
        let msg_value = serde_json::json!({
            "sender": sender,
            "contract": contract,
            "msg": Self::bytes_to_json_array(msg_bytes),  // Number array, not base64 string
            "funds": []
        });

        let broadcast_request = crate::treasury::types::BroadcastRequest {
            messages: vec![crate::treasury::types::TransactionMessage {
                type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
                value: msg_value,
            }],
            memo: Some("Mint NFT token via Xion Agent Toolkit".to_string()),
        };

        let response = self
            .api_client
            .broadcast_transaction(access_token, broadcast_request)
            .await
            .map_err(|e| AssetBuilderError::MintFailed(e.to_string()))?;

        Ok(response.tx_hash)
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

    /// Compute the predicted contract address for instantiate2
    ///
    /// Uses the instantiate2 algorithm to compute the deterministic contract address
    /// before broadcasting the transaction.
    ///
    /// # Arguments
    /// * `code_id` - The code ID being instantiated
    /// * `sender` - The sender's bech32 address (e.g., "xion1...")
    /// * `salt` - The salt bytes used for instantiate2
    ///
    /// # Returns
    /// The predicted contract address as a bech32 string
    async fn compute_instantiate2_address(
        &self,
        code_id: u64,
        sender: &str,
        salt: &[u8],
    ) -> Result<String, AssetBuilderError> {
        // Fetch code info to get the checksum
        let code_info = self.api_client.get_code_info(code_id).await.map_err(|e| {
            AssetBuilderError::ApiError(format!("Failed to fetch code info: {}", e))
        })?;

        debug!(
            "Code info - code_id: {}, checksum: {}",
            code_info.code_id, code_info.checksum
        );

        // Parse checksum from hex string to bytes
        let checksum_bytes = hex::decode(&code_info.checksum)
            .map_err(|e| AssetBuilderError::ApiError(format!("Invalid checksum format: {}", e)))?;

        // Convert sender address to canonical format
        let canonical_sender = decode_bech32_address(sender)
            .map_err(|e| AssetBuilderError::ApiError(format!("Failed to decode address: {}", e)))?;

        // Compute instantiate2 address using cosmwasm_std
        let canonical_addr = instantiate2_address(&checksum_bytes, &canonical_sender, salt)
            .map_err(|e| {
                AssetBuilderError::ApiError(format!(
                    "Failed to compute instantiate2 address: {}",
                    e
                ))
            })?;

        // Convert canonical address back to bech32
        let bech32_prefix = extract_bech32_prefix(sender)
            .map_err(|e| AssetBuilderError::ApiError(e.to_string()))?;
        let predicted_address = encode_canonical_address(&canonical_addr, &bech32_prefix)
            .map_err(|e| AssetBuilderError::ApiError(format!("Failed to encode address: {}", e)))?;

        debug!(
            "Predicted instantiate2 address: {} (checksum: {}, sender: {}, salt: {})",
            predicted_address,
            code_info.checksum,
            sender,
            hex::encode(salt)
        );

        Ok(predicted_address)
    }
}

// ============================================================================
// Bech32 Address Utilities
// ============================================================================

/// Decode a bech32 address to its canonical (binary) form
///
/// # Arguments
/// * `address` - Bech32 encoded address (e.g., "xion1abc...")
///
/// # Returns
/// Canonical address as `CanonicalAddr`
fn decode_bech32_address(address: &str) -> Result<CanonicalAddr> {
    let (_hrp, data) =
        decode(address).map_err(|e| anyhow::anyhow!("Bech32 decode error: {}", e))?;

    Ok(CanonicalAddr::from(HexBinary::from(data)))
}

/// Encode a canonical address to bech32 format
///
/// # Arguments
/// * `canonical` - Canonical address bytes
/// * `prefix` - Bech32 prefix (e.g., "xion")
///
/// # Returns
/// Bech32 encoded address string
fn encode_canonical_address(canonical: &CanonicalAddr, prefix: &str) -> Result<String> {
    let hrp = Hrp::parse(prefix).map_err(|e| anyhow::anyhow!("Invalid bech32 prefix: {}", e))?;

    let encoded = encode::<Bech32>(hrp, canonical.as_slice())
        .map_err(|e| anyhow::anyhow!("Bech32 encode error: {}", e))?;

    Ok(encoded)
}

/// Extract the bech32 prefix from an address
///
/// # Arguments
/// * `address` - Bech32 encoded address (e.g., "xion1abc...")
///
/// # Returns
/// The prefix part (e.g., "xion")
fn extract_bech32_prefix(address: &str) -> Result<String> {
    let separator_pos = address
        .find('1')
        .ok_or_else(|| anyhow::anyhow!("Invalid bech32 address format: missing separator"))?;

    Ok(address[..separator_pos].to_string())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_json_array() {
        let bytes = b"hello";
        let arr = AssetBuilderManager::bytes_to_json_array(bytes);

        assert_eq!(arr, serde_json::json!([104, 101, 108, 108, 111]));
    }

    #[test]
    fn test_bytes_to_json_array_empty() {
        let bytes: &[u8] = &[];
        let arr = AssetBuilderManager::bytes_to_json_array(bytes);

        assert_eq!(arr, serde_json::json!([]));
    }

    #[test]
    fn test_instantiate_msg_serialization() {
        let msg = Cw721InstantiateMsg {
            name: "My Collection".to_string(),
            symbol: "NFT".to_string(),
            minter: "xion1abc123".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"name\":\"My Collection\""));
    }

    #[test]
    fn test_mint_msg_serialization() {
        let msg = Cw721MintMsg {
            mint: MintContent {
                token_id: "1".to_string(),
                owner: "xion1owner".to_string(),
                token_uri: Some("ipfs://hash".to_string()),
                extension: serde_json::json!({}),
            },
        };

        let json = serde_json::to_string(&msg).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["mint"]["token_id"], "1");
        assert_eq!(parsed["mint"]["owner"], "xion1owner");
        assert_eq!(parsed["mint"]["token_uri"], "ipfs://hash");
    }

    #[test]
    fn test_decode_encode_bech32_address() {
        // Test with a known xion address
        let address = "xion1q9lqzpc73fewqva98pwaqvezaf9vqqulw3hmmx";

        // Decode
        let canonical = decode_bech32_address(address).unwrap();
        assert!(!canonical.is_empty());

        // Encode back
        let encoded = encode_canonical_address(&canonical, "xion").unwrap();

        // Should round-trip correctly
        assert_eq!(encoded, address);
    }

    #[test]
    fn test_extract_bech32_prefix() {
        let address = "xion1q9lqzpc73fewqva98pwaqvezaf9vqqulw3hmmx";
        let prefix = extract_bech32_prefix(address).unwrap();
        assert_eq!(prefix, "xion");
    }

    #[test]
    fn test_instantiate2_address_computation() {
        // Test that instantiate2 address computation works with valid inputs
        // Use the cw721-base code ID 522 checksum from testnet

        let checksum_hex = "e13aa30e0d70ea895b294ad1bc809950e60fe081b322b1657f75b67be6021b1c";
        let checksum_bytes = hex::decode(checksum_hex).unwrap();

        // Use a valid xion address
        let sender_address = "xion1q9lqzpc73fewqva98pwaqvezaf9vqqulw3hmmx";
        let canonical_sender = decode_bech32_address(sender_address).unwrap();

        let salt =
            hex::decode("6162636465666768696a6b6c6d6e6f707172737475767778797a30313233343536")
                .unwrap(); // 32-byte salt

        // Compute the predicted address
        let canonical_addr =
            instantiate2_address(&checksum_bytes, &canonical_sender, &salt).unwrap();

        // Encode back to bech32
        let predicted = encode_canonical_address(&canonical_addr, "xion").unwrap();

        // The predicted address should be deterministic and a valid xion address
        assert!(!predicted.is_empty());
        assert!(predicted.starts_with("xion1"));

        // Same inputs should always produce the same address
        let canonical_addr2 =
            instantiate2_address(&checksum_bytes, &canonical_sender, &salt).unwrap();
        let predicted2 = encode_canonical_address(&canonical_addr2, "xion").unwrap();
        assert_eq!(predicted, predicted2);
    }
}
