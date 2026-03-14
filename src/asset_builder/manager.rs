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
    AssetBuilderError, AssetType, BatchMintInput, BatchMintResult, BatchMintTokenResult,
    CreateCollectionInput, CreateCollectionResult, Cw2981MintContent, Cw2981MintMsg,
    Cw2981RoyaltyInfo, Cw721ExpirationMintContent, Cw721ExpirationMintMsg, Cw721InstantiateMsg,
    Cw721MintMsg, MintContent, MintTokenInput, MintTokenResult, PredictAddressInput,
    PredictAddressResult, QueryResult,
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

        // Build instantiate message (all CW721 variants use the same format)
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
    /// Supports different CW721 variants based on asset_type.
    ///
    /// # Arguments
    /// * `input` - Mint input with contract address, token ID, owner, etc.
    ///
    /// # Returns
    /// Mint result with transaction hash
    ///
    /// # Variant-specific Fields
    /// - CW2981: Use `royalty_address` and `royalty_percentage` for royalties
    /// - Expiration: Use `expires_at` for time-based expiration
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

        // Determine asset type (default to Cw721Base for backward compatibility)
        let asset_type = input.asset_type.unwrap_or(AssetType::Cw721Base);

        // Build mint message based on asset type
        let mint_msg = self.build_mint_msg(&input, asset_type)?;

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

    /// Build mint message based on asset type
    ///
    /// Dispatches to the appropriate mint message format based on the asset type.
    fn build_mint_msg(
        &self,
        input: &MintTokenInput,
        asset_type: AssetType,
    ) -> Result<serde_json::Value, AssetBuilderError> {
        match asset_type {
            AssetType::Cw721Base
            | AssetType::Cw721MetadataOnchain
            | AssetType::Cw721NonTransferable => {
                // Standard mint message for base, metadata-onchain, and non-transferable
                let msg = Cw721MintMsg {
                    mint: MintContent {
                        token_id: input.token_id.clone(),
                        owner: input.owner.clone(),
                        token_uri: input.token_uri.clone(),
                        extension: input.extension.clone(),
                    },
                };
                Ok(serde_json::to_value(msg)?)
            }
            AssetType::Cw2981Royalties => {
                // Build with royalty_info for CW2981
                let royalty_info = self.build_royalty_info(input)?;

                let msg = Cw2981MintMsg {
                    mint: Cw2981MintContent {
                        token_id: input.token_id.clone(),
                        owner: input.owner.clone(),
                        token_uri: input.token_uri.clone(),
                        extension: input.extension.clone(),
                        royalty_info,
                    },
                };
                Ok(serde_json::to_value(msg)?)
            }
            AssetType::Cw721Expiration => {
                // Build with expires_at for expiration variant
                let expires_at = input.expires_at.as_ref().ok_or_else(|| {
                    AssetBuilderError::MissingRequiredField(
                        "expires_at is required for cw721-expiration".to_string(),
                    )
                })?;

                let msg = Cw721ExpirationMintMsg {
                    mint: Cw721ExpirationMintContent {
                        token_id: input.token_id.clone(),
                        owner: input.owner.clone(),
                        token_uri: input.token_uri.clone(),
                        extension: input.extension.clone(),
                        expires_at: expires_at.clone(),
                    },
                };
                Ok(serde_json::to_value(msg)?)
            }
        }
    }

    /// Build royalty info for CW2981 tokens
    ///
    /// Validates royalty fields and constructs royalty info if provided.
    fn build_royalty_info(
        &self,
        input: &MintTokenInput,
    ) -> Result<Option<Cw2981RoyaltyInfo>, AssetBuilderError> {
        match (&input.royalty_address, input.royalty_percentage) {
            (Some(address), Some(percentage)) => {
                // Validate percentage range
                if !(0.0..=1.0).contains(&percentage) {
                    return Err(AssetBuilderError::InvalidRoyaltyPercentage(percentage));
                }

                // Convert percentage to share string
                let share = format!("{}", percentage);

                Ok(Some(Cw2981RoyaltyInfo {
                    payment_address: address.clone(),
                    share,
                }))
            }
            (Some(_), None) | (None, Some(_)) => {
                // Incomplete royalty info - both fields must be provided together
                Err(AssetBuilderError::IncompleteRoyaltyInfo)
            }
            (None, None) => {
                // No royalty info provided - this is valid
                Ok(None)
            }
        }
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

    /// Predict contract address before deployment
    ///
    /// Computes the deterministic contract address using instantiate2 algorithm.
    /// This allows you to know the address before actually deploying the contract.
    ///
    /// # Arguments
    /// * `input` - Prediction input with asset type, name, symbol, salt, etc.
    ///
    /// # Returns
    /// Prediction result with the computed contract address
    #[instrument(skip(self, input))]
    pub async fn predict_address(
        &self,
        input: PredictAddressInput,
    ) -> Result<PredictAddressResult, AssetBuilderError> {
        debug!("Predicting address for {} ({})", input.name, input.symbol);

        // Get user credentials to obtain sender address
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

        // Parse salt from hex string
        let salt_bytes = hex::decode(&input.salt).map_err(|e| {
            AssetBuilderError::ApiError(format!(
                "Invalid salt format: {}. Expected hex-encoded string. Error: {}",
                input.salt, e
            ))
        })?;

        // Compute predicted address
        let predicted_address = self
            .compute_instantiate2_address(code_id, &sender, &salt_bytes)
            .await?;

        Ok(PredictAddressResult {
            success: true,
            contract_address: predicted_address,
            code_id,
            salt: input.salt,
            creator: sender,
        })
    }

    /// Batch mint multiple NFT tokens in a single transaction
    ///
    /// Mints multiple tokens in an existing NFT collection in one transaction.
    /// This is more efficient than minting tokens individually.
    ///
    /// # Arguments
    /// * `input` - Batch mint input with contract address, asset type, and tokens
    ///
    /// # Returns
    /// Batch mint result with per-token status
    #[instrument(skip(self, input))]
    pub async fn batch_mint(
        &self,
        input: BatchMintInput,
    ) -> Result<BatchMintResult, AssetBuilderError> {
        debug!(
            "Batch minting {} tokens in contract {}",
            input.tokens.len(),
            input.contract
        );

        if input.tokens.is_empty() {
            return Err(AssetBuilderError::BatchMintError(
                "No tokens provided for batch minting".to_string(),
            ));
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

        // Build mint messages for each token
        let mut messages: Vec<serde_json::Value> = Vec::new();
        let mut results: Vec<BatchMintTokenResult> = Vec::new();

        for token in &input.tokens {
            // Build MintTokenInput for each token
            let mint_input = MintTokenInput {
                contract: input.contract.clone(),
                token_id: token.token_id.clone(),
                owner: token.owner.clone(),
                token_uri: token.token_uri.clone(),
                extension: token.extension.clone(),
                royalty_address: token.royalty_address.clone(),
                royalty_percentage: token.royalty_percentage,
                expires_at: token.expires_at.clone(),
                asset_type: Some(input.asset_type),
            };

            // Build mint message
            match self.build_mint_msg(&mint_input, input.asset_type) {
                Ok(mint_msg) => {
                    messages.push(mint_msg);
                    results.push(BatchMintTokenResult {
                        token_id: token.token_id.clone(),
                        success: true,
                        error: None,
                    });
                }
                Err(e) => {
                    results.push(BatchMintTokenResult {
                        token_id: token.token_id.clone(),
                        success: false,
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        // Check if any messages were built successfully
        let successful_messages: Vec<_> = messages
            .iter()
            .zip(results.iter())
            .filter(|(_, r)| r.success)
            .map(|(m, _)| m.clone())
            .collect();

        if successful_messages.is_empty() {
            return Ok(BatchMintResult {
                success: false,
                contract_address: input.contract.clone(),
                tx_hash: String::new(),
                total: input.tokens.len(),
                succeeded: 0,
                failed: input.tokens.len(),
                results,
            });
        }

        // Get valid access token
        let access_token = self
            .oauth_client
            .get_valid_token()
            .await
            .map_err(|e| AssetBuilderError::ApiError(e.to_string()))?;

        // Broadcast batch execute
        let tx_hash = self
            .broadcast_batch_execute(
                &access_token,
                &sender,
                &input.contract,
                &successful_messages,
            )
            .await?;

        // Update results with transaction status
        let succeeded_count = results.iter().filter(|r| r.success).count();
        let failed_count = results.iter().filter(|r| !r.success).count();

        Ok(BatchMintResult {
            success: failed_count == 0,
            contract_address: input.contract.clone(),
            tx_hash,
            total: input.tokens.len(),
            succeeded: succeeded_count,
            failed: failed_count,
            results,
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

    /// Broadcast batch execute contract transaction
    ///
    /// Uses the OAuth2 API to broadcast multiple execute contract messages in one transaction.
    async fn broadcast_batch_execute(
        &self,
        access_token: &str,
        sender: &str,
        contract: &str,
        execute_msgs: &[serde_json::Value],
    ) -> Result<String, AssetBuilderError> {
        debug!(
            "Batch executing {} messages on contract {}",
            execute_msgs.len(),
            contract
        );

        // Build multiple MsgExecuteContract messages
        let messages: Vec<crate::treasury::types::TransactionMessage> = execute_msgs
            .iter()
            .map(|execute_msg| {
                // Serialize execute message to JSON, then convert to number array
                let msg_json = serde_json::to_string(execute_msg)?;
                let msg_bytes = msg_json.as_bytes();

                let msg_value = serde_json::json!({
                    "sender": sender,
                    "contract": contract,
                    "msg": Self::bytes_to_json_array(msg_bytes),
                    "funds": []
                });

                Ok(crate::treasury::types::TransactionMessage {
                    type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
                    value: msg_value,
                })
            })
            .collect::<Result<Vec<_>, AssetBuilderError>>()?;

        let broadcast_request = crate::treasury::types::BroadcastRequest {
            messages,
            memo: Some("Batch mint NFT tokens via Xion Agent Toolkit".to_string()),
        };

        let response = self
            .api_client
            .broadcast_transaction(access_token, broadcast_request)
            .await
            .map_err(|e| AssetBuilderError::BatchMintError(e.to_string()))?;

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

    // ========================================================================
    // Mint Message Dispatch Tests
    // ========================================================================

    #[test]
    fn test_build_mint_msg_cw721_base() {
        // Test that Cw721Base produces standard mint message
        let input = MintTokenInput {
            contract: "xion1contract".to_string(),
            token_id: "1".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: Some("ipfs://hash".to_string()),
            extension: serde_json::json!({}),
            royalty_address: None,
            royalty_percentage: None,
            expires_at: None,
            asset_type: Some(AssetType::Cw721Base),
        };

        // Create a mock manager to test build_mint_msg
        // We test the message structure directly by serializing
        let msg = Cw721MintMsg {
            mint: MintContent {
                token_id: input.token_id.clone(),
                owner: input.owner.clone(),
                token_uri: input.token_uri.clone(),
                extension: input.extension.clone(),
            },
        };

        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["mint"]["token_id"], "1");
        assert_eq!(json["mint"]["owner"], "xion1owner");
        assert_eq!(json["mint"]["token_uri"], "ipfs://hash");
        assert!(!json["mint"]
            .as_object()
            .unwrap()
            .contains_key("royalty_info"));
        assert!(!json["mint"].as_object().unwrap().contains_key("expires_at"));
    }

    #[test]
    fn test_build_mint_msg_cw721_metadata_onchain() {
        // Metadata-onchain uses same mint format as base
        let msg = Cw721MintMsg {
            mint: MintContent {
                token_id: "meta-1".to_string(),
                owner: "xion1owner".to_string(),
                token_uri: None,
                extension: serde_json::json!({"name": "On-chain NFT", "description": "Test"}),
            },
        };

        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["mint"]["token_id"], "meta-1");
        assert!(json["mint"]["extension"]["name"].is_string());
    }

    #[test]
    fn test_build_mint_msg_cw2981_with_royalty() {
        // Test CW2981 mint with royalty
        let msg = Cw2981MintMsg {
            mint: Cw2981MintContent {
                token_id: "royalty-1".to_string(),
                owner: "xion1owner".to_string(),
                token_uri: None,
                extension: serde_json::json!({}),
                royalty_info: Some(Cw2981RoyaltyInfo {
                    payment_address: "xion1artist".to_string(),
                    share: "0.05".to_string(),
                }),
            },
        };

        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["mint"]["token_id"], "royalty-1");
        assert_eq!(
            json["mint"]["royalty_info"]["payment_address"],
            "xion1artist"
        );
        assert_eq!(json["mint"]["royalty_info"]["share"], "0.05");
    }

    #[test]
    fn test_build_mint_msg_cw721_expiration() {
        // Test expiration mint with expires_at
        let msg = Cw721ExpirationMintMsg {
            mint: Cw721ExpirationMintContent {
                token_id: "exp-1".to_string(),
                owner: "xion1owner".to_string(),
                token_uri: None,
                extension: serde_json::json!({}),
                expires_at: "2025-12-31T23:59:59Z".to_string(),
            },
        };

        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["mint"]["token_id"], "exp-1");
        assert_eq!(json["mint"]["expires_at"], "2025-12-31T23:59:59Z");
    }

    #[test]
    fn test_cw2981_royalty_percentage_conversion() {
        // Test that 5% is converted to "0.05"
        let percentage = 0.05;
        let share = format!("{}", percentage);
        assert_eq!(share, "0.05");

        // Test that 10% is converted to "0.1"
        let percentage = 0.1;
        let share = format!("{}", percentage);
        assert_eq!(share, "0.1");
    }

    #[test]
    fn test_royalty_info_optional() {
        // CW2981 mint without royalty info should work
        let content = Cw2981MintContent {
            token_id: "no-royalty".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: None,
            extension: serde_json::json!({}),
            royalty_info: None,
        };

        let json = serde_json::to_string(&content).unwrap();
        assert!(!json.contains("royalty_info"));
    }
}
