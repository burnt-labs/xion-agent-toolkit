//! Asset Builder Types
//!
//! Data structures for CW721 NFT contract deployment and management.
//!
//! ## Asset Types Supported
//!
//! | Type | Code ID (Testnet) | Description |
//! |------|-------------------|-------------|
//! | cw721-base | 522 | Standard NFT |
//! | cw721-metadata-onchain | 525 | On-chain metadata |
//! | cw721-expiration | 523 | Time-based expiry |
//! | cw721-non-transferable | 526 | Soulbound NFT |
//! | cw2981-royalties | 528 | Royalties at mint time |

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Asset type variants for NFT contracts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AssetType {
    /// Standard CW721 NFT
    Cw721Base,
    /// NFT with on-chain metadata
    Cw721MetadataOnchain,
    /// NFT with time-based expiration
    Cw721Expiration,
    /// Non-transferable (soulbound) NFT
    Cw721NonTransferable,
    /// NFT with CW2981 royalties
    Cw2981Royalties,
}

impl AssetType {
    /// Get the string identifier for this asset type
    pub fn as_str(&self) -> &'static str {
        match self {
            AssetType::Cw721Base => "cw721-base",
            AssetType::Cw721MetadataOnchain => "cw721-metadata-onchain",
            AssetType::Cw721Expiration => "cw721-expiration",
            AssetType::Cw721NonTransferable => "cw721-non-transferable",
            AssetType::Cw2981Royalties => "cw2981-royalties",
        }
    }

    /// Get display name for this asset type
    pub fn display_name(&self) -> &'static str {
        match self {
            AssetType::Cw721Base => "Standard NFT (CW721-Base)",
            AssetType::Cw721MetadataOnchain => "NFT with On-Chain Metadata",
            AssetType::Cw721Expiration => "NFT with Expiration",
            AssetType::Cw721NonTransferable => "Soulbound NFT",
            AssetType::Cw2981Royalties => "NFT with Royalties",
        }
    }

    /// Parse from string
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "cw721-base" => Some(AssetType::Cw721Base),
            "cw721-metadata-onchain" => Some(AssetType::Cw721MetadataOnchain),
            "cw721-expiration" => Some(AssetType::Cw721Expiration),
            "cw721-non-transferable" => Some(AssetType::Cw721NonTransferable),
            "cw2981-royalties" => Some(AssetType::Cw2981Royalties),
            _ => None,
        }
    }

    /// Get all available asset types
    pub fn all() -> &'static [AssetType] {
        &[
            AssetType::Cw721Base,
            AssetType::Cw721MetadataOnchain,
            AssetType::Cw721Expiration,
            AssetType::Cw721NonTransferable,
            AssetType::Cw2981Royalties,
        ]
    }
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Instantiate Messages
// ============================================================================

/// Instantiate message for cw721-base contract
///
/// This is the standard CW721 instantiation message.
/// Other variants may have additional fields (Phase 2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cw721InstantiateMsg {
    /// Collection name
    pub name: String,
    /// Collection symbol (e.g., "NFT")
    pub symbol: String,
    /// Minter address (can mint new tokens)
    pub minter: String,
}

// ============================================================================
// Mint Messages
// ============================================================================

/// Mint message for cw721-base contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cw721MintMsg {
    /// Mint operation wrapper
    pub mint: MintContent,
}

/// Content of the mint message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintContent {
    /// Unique token ID
    pub token_id: String,
    /// Owner address
    pub owner: String,
    /// Optional token URI (e.g., IPFS URI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_uri: Option<String>,
    /// Extension data (empty object for standard NFTs)
    #[serde(default)]
    pub extension: serde_json::Value,
}

// ============================================================================
// CW2981 Royalty Types
// ============================================================================

/// Royalty info for CW2981 tokens
///
/// Royalties are set at mint time for CW2981 compliant NFTs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cw2981RoyaltyInfo {
    /// Address to receive royalties
    pub payment_address: String,
    /// Royalty share as decimal string (e.g., "0.05" for 5%)
    pub share: String,
}

/// Mint content for CW2981 with royalties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cw2981MintContent {
    /// Unique token ID
    pub token_id: String,
    /// Owner address
    pub owner: String,
    /// Optional token URI (e.g., IPFS URI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_uri: Option<String>,
    /// Extension data
    #[serde(default)]
    pub extension: serde_json::Value,
    /// Optional royalty information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty_info: Option<Cw2981RoyaltyInfo>,
}

/// Mint message for CW2981 contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cw2981MintMsg {
    /// Mint operation wrapper
    pub mint: Cw2981MintContent,
}

// ============================================================================
// CW721 Expiration Types
// ============================================================================

/// Mint content for cw721-expiration
///
/// Supports time-based token expiration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cw721ExpirationMintContent {
    /// Unique token ID
    pub token_id: String,
    /// Owner address
    pub owner: String,
    /// Optional token URI (e.g., IPFS URI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_uri: Option<String>,
    /// Extension data
    #[serde(default)]
    pub extension: serde_json::Value,
    /// Expiration timestamp (Unix seconds or ISO 8601)
    pub expires_at: String,
}

/// Mint message for cw721-expiration contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cw721ExpirationMintMsg {
    /// Mint operation wrapper
    pub mint: Cw721ExpirationMintContent,
}

// ============================================================================
// Input Types
// ============================================================================

/// Input for creating a new NFT collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCollectionInput {
    /// Asset type to create
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    /// Collection name
    pub name: String,
    /// Collection symbol
    pub symbol: String,
    /// Optional minter address (defaults to sender)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minter: Option<String>,
    /// Optional custom code ID (overrides network default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_id: Option<u64>,
    /// Optional salt for predictable address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}

/// Input for minting an NFT token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintTokenInput {
    /// Contract address
    pub contract: String,
    /// Token ID
    pub token_id: String,
    /// Owner address
    pub owner: String,
    /// Optional token URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_uri: Option<String>,
    /// Extension data
    #[serde(default)]
    pub extension: serde_json::Value,
    /// Royalty payment address (CW2981 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty_address: Option<String>,
    /// Royalty percentage as decimal (CW2981 only, e.g., 0.05 for 5%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty_percentage: Option<f64>,
    /// Expiration timestamp (cw721-expiration only)
    /// Can be Unix timestamp or ISO 8601 format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Asset type for dispatch (optional, defaults to Cw721Base)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
}

// ============================================================================
// Result Types
// ============================================================================

/// Result of creating an NFT collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCollectionResult {
    /// Success status
    pub success: bool,
    /// Contract address
    pub contract_address: String,
    /// Transaction hash
    pub tx_hash: String,
    /// Code ID used
    pub code_id: u64,
    /// Collection name
    pub name: String,
    /// Collection symbol
    pub symbol: String,
    /// Minter address
    pub minter: String,
    /// Salt used for instantiate2 (hex-encoded)
    pub salt: String,
}

/// Result of minting an NFT token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintTokenResult {
    /// Success status
    pub success: bool,
    /// Contract address
    pub contract_address: String,
    /// Token ID
    pub token_id: String,
    /// Owner address
    pub owner: String,
    /// Transaction hash
    pub tx_hash: String,
}

/// Result of querying an NFT contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Success status
    pub success: bool,
    /// Contract address
    pub contract_address: String,
    /// Query response
    pub response: serde_json::Value,
}

// ============================================================================
// Address Prediction Types
// ============================================================================

/// Input for predicting contract address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictAddressInput {
    /// Asset type to create
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    /// Collection name
    pub name: String,
    /// Collection symbol
    pub symbol: String,
    /// Optional minter address (defaults to sender)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minter: Option<String>,
    /// Optional custom code ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_id: Option<u64>,
    /// Salt for predictable address (required for prediction)
    pub salt: String,
}

/// Result of address prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictAddressResult {
    /// Success status
    pub success: bool,
    /// Predicted contract address
    pub contract_address: String,
    /// Code ID used
    pub code_id: u64,
    /// Salt used (hex-encoded)
    pub salt: String,
    /// Creator address (sender)
    pub creator: String,
}

// ============================================================================
// Batch Minting Types
// ============================================================================

/// Single token in a batch mint operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMintToken {
    /// Token ID
    pub token_id: String,
    /// Owner address
    pub owner: String,
    /// Optional token URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_uri: Option<String>,
    /// Extension data
    #[serde(default)]
    pub extension: serde_json::Value,
    /// Royalty address (CW2981 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty_address: Option<String>,
    /// Royalty percentage (CW2981 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty_percentage: Option<f64>,
    /// Expiration timestamp (cw721-expiration only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Input for batch minting tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMintInput {
    /// Contract address
    pub contract: String,
    /// Asset type (determines mint message format)
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    /// Tokens to mint
    pub tokens: Vec<BatchMintToken>,
}

/// Result of a single token mint in batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMintTokenResult {
    /// Token ID
    pub token_id: String,
    /// Success status
    pub success: bool,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Result of batch minting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMintResult {
    /// Success status (true if all succeeded)
    pub success: bool,
    /// Contract address
    pub contract_address: String,
    /// Transaction hash
    pub tx_hash: String,
    /// Total tokens attempted
    pub total: usize,
    /// Successful mints
    pub succeeded: usize,
    /// Failed mints
    pub failed: usize,
    /// Per-token results
    pub results: Vec<BatchMintTokenResult>,
}

/// Asset type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetTypeInfo {
    /// Asset type identifier
    #[serde(rename = "type")]
    pub asset_type: String,
    /// Display name
    pub display_name: String,
    /// Testnet code ID
    pub testnet_code_id: u64,
    /// Mainnet code ID (0 if not deployed)
    pub mainnet_code_id: u64,
    /// Description
    pub description: String,
}

// ============================================================================
// Error Types
// ============================================================================

/// Asset builder error types
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AssetBuilderError {
    /// Invalid asset type
    #[error("Invalid asset type: {0}")]
    InvalidAssetType(String),

    /// Code ID not found for network
    #[error("Code ID not configured for {0} on {1}")]
    CodeIdNotFound(String, String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingRequiredField(String),

    /// Invalid royalty percentage
    #[error("Invalid royalty percentage: {0}. Must be between 0.0 and 1.0")]
    InvalidRoyaltyPercentage(f64),

    /// Royalty info missing required fields
    #[error("Royalty info incomplete: both royalty_address and royalty_percentage are required")]
    IncompleteRoyaltyInfo,

    /// Contract instantiation failed
    #[error("Contract instantiation failed: {0}")]
    InstantiationFailed(String),

    /// Token minting failed
    #[error("Token minting failed: {0}")]
    MintFailed(String),

    /// Contract query failed
    #[error("Contract query failed: {0}")]
    QueryFailed(String),

    /// Not authenticated
    #[error("Not authenticated. Please run 'xion auth login' first.")]
    NotAuthenticated,

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// API error
    #[error("API error: {0}")]
    ApiError(String),

    /// Batch minting error
    #[error("Batch minting failed: {0}")]
    BatchMintError(String),
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_type_from_str() {
        assert_eq!(AssetType::parse("cw721-base"), Some(AssetType::Cw721Base));
        assert_eq!(
            AssetType::parse("cw721-metadata-onchain"),
            Some(AssetType::Cw721MetadataOnchain)
        );
        assert_eq!(
            AssetType::parse("cw2981-royalties"),
            Some(AssetType::Cw2981Royalties)
        );
        assert_eq!(AssetType::parse("invalid"), None);
    }

    #[test]
    fn test_asset_type_as_str() {
        assert_eq!(AssetType::Cw721Base.as_str(), "cw721-base");
        assert_eq!(
            AssetType::Cw721NonTransferable.as_str(),
            "cw721-non-transferable"
        );
    }

    #[test]
    fn test_cw721_instantiate_msg_serialization() {
        let msg = Cw721InstantiateMsg {
            name: "My Collection".to_string(),
            symbol: "NFT".to_string(),
            minter: "xion1abc123".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"name\":\"My Collection\""));
        assert!(json.contains("\"symbol\":\"NFT\""));
        assert!(json.contains("\"minter\":\"xion1abc123\""));
    }

    #[test]
    fn test_cw721_mint_msg_serialization() {
        let msg = Cw721MintMsg {
            mint: MintContent {
                token_id: "1".to_string(),
                owner: "xion1abc123".to_string(),
                token_uri: Some("ipfs://QmHash".to_string()),
                extension: serde_json::json!({}),
            },
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"mint\":"));
        assert!(json.contains("\"token_id\":\"1\""));
        assert!(json.contains("\"token_uri\":\"ipfs://QmHash\""));
    }

    #[test]
    fn test_cw721_mint_msg_without_token_uri() {
        let msg = Cw721MintMsg {
            mint: MintContent {
                token_id: "2".to_string(),
                owner: "xion1def456".to_string(),
                token_uri: None,
                extension: serde_json::json!({}),
            },
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"token_id\":\"2\""));
        assert!(!json.contains("token_uri")); // Should not include null field
    }

    #[test]
    fn test_create_collection_input_deserialization() {
        let json = r#"{
            "type": "cw721-base",
            "name": "My NFT",
            "symbol": "NFT",
            "minter": "xion1abc123"
        }"#;

        let input: CreateCollectionInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.asset_type, AssetType::Cw721Base);
        assert_eq!(input.name, "My NFT");
        assert_eq!(input.symbol, "NFT");
        assert_eq!(input.minter, Some("xion1abc123".to_string()));
    }

    #[test]
    fn test_mint_token_input_deserialization() {
        let json = r#"{
            "contract": "xion1contract",
            "token_id": "42",
            "owner": "xion1owner",
            "token_uri": "ipfs://QmHash"
        }"#;

        let input: MintTokenInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.contract, "xion1contract");
        assert_eq!(input.token_id, "42");
        assert_eq!(input.owner, "xion1owner");
        assert_eq!(input.token_uri, Some("ipfs://QmHash".to_string()));
    }

    #[test]
    fn test_asset_type_all() {
        let all = AssetType::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&AssetType::Cw721Base));
        assert!(all.contains(&AssetType::Cw2981Royalties));
    }

    // ========================================================================
    // CW2981 Royalty Tests
    // ========================================================================

    #[test]
    fn test_cw2981_royalty_info_serialization() {
        let royalty = Cw2981RoyaltyInfo {
            payment_address: "xion1abc123".to_string(),
            share: "0.05".to_string(),
        };

        let json = serde_json::to_string(&royalty).unwrap();
        assert!(json.contains("\"payment_address\":\"xion1abc123\""));
        assert!(json.contains("\"share\":\"0.05\""));
    }

    #[test]
    fn test_cw2981_mint_content_with_royalty() {
        let content = Cw2981MintContent {
            token_id: "1".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: Some("ipfs://QmHash".to_string()),
            extension: serde_json::json!({}),
            royalty_info: Some(Cw2981RoyaltyInfo {
                payment_address: "xion1royalty".to_string(),
                share: "0.05".to_string(),
            }),
        };

        let json = serde_json::to_string(&content).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["token_id"], "1");
        assert_eq!(parsed["owner"], "xion1owner");
        assert_eq!(parsed["royalty_info"]["payment_address"], "xion1royalty");
        assert_eq!(parsed["royalty_info"]["share"], "0.05");
    }

    #[test]
    fn test_cw2981_mint_content_without_royalty() {
        let content = Cw2981MintContent {
            token_id: "2".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: None,
            extension: serde_json::json!({}),
            royalty_info: None,
        };

        let json = serde_json::to_string(&content).unwrap();
        assert!(!json.contains("royalty_info")); // Should be skipped
    }

    #[test]
    fn test_cw2981_mint_msg_serialization() {
        let msg = Cw2981MintMsg {
            mint: Cw2981MintContent {
                token_id: "royalty-1".to_string(),
                owner: "xion1owner".to_string(),
                token_uri: None,
                extension: serde_json::json!({"name": "NFT with royalties"}),
                royalty_info: Some(Cw2981RoyaltyInfo {
                    payment_address: "xion1artist".to_string(),
                    share: "0.10".to_string(),
                }),
            },
        };

        let json = serde_json::to_string(&msg).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["mint"]["token_id"], "royalty-1");
        assert_eq!(parsed["mint"]["royalty_info"]["share"], "0.10");
    }

    // ========================================================================
    // CW721 Expiration Tests
    // ========================================================================

    #[test]
    fn test_cw721_expiration_mint_content() {
        let content = Cw721ExpirationMintContent {
            token_id: "exp-1".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: Some("ipfs://QmHash".to_string()),
            extension: serde_json::json!({}),
            expires_at: "2025-12-31T23:59:59Z".to_string(),
        };

        let json = serde_json::to_string(&content).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["token_id"], "exp-1");
        assert_eq!(parsed["expires_at"], "2025-12-31T23:59:59Z");
    }

    #[test]
    fn test_cw721_expiration_mint_msg_serialization() {
        let msg = Cw721ExpirationMintMsg {
            mint: Cw721ExpirationMintContent {
                token_id: "exp-2".to_string(),
                owner: "xion1owner".to_string(),
                token_uri: None,
                extension: serde_json::json!({}),
                expires_at: "1704067200".to_string(), // Unix timestamp
            },
        };

        let json = serde_json::to_string(&msg).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["mint"]["token_id"], "exp-2");
        assert_eq!(parsed["mint"]["expires_at"], "1704067200");
    }

    // ========================================================================
    // MintTokenInput Tests
    // ========================================================================

    #[test]
    fn test_mint_token_input_with_royalty() {
        let input = MintTokenInput {
            contract: "xion1contract".to_string(),
            token_id: "1".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: Some("ipfs://hash".to_string()),
            extension: serde_json::json!({}),
            royalty_address: Some("xion1artist".to_string()),
            royalty_percentage: Some(0.05),
            expires_at: None,
            asset_type: Some(AssetType::Cw2981Royalties),
        };

        let json = serde_json::to_string(&input).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["royalty_address"], "xion1artist");
        assert_eq!(parsed["royalty_percentage"], 0.05);
        assert!(!json.contains("expires_at")); // Should be skipped
    }

    #[test]
    fn test_mint_token_input_with_expiration() {
        let input = MintTokenInput {
            contract: "xion1contract".to_string(),
            token_id: "2".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: None,
            extension: serde_json::json!({}),
            royalty_address: None,
            royalty_percentage: None,
            expires_at: Some("2025-12-31T23:59:59Z".to_string()),
            asset_type: Some(AssetType::Cw721Expiration),
        };

        let json = serde_json::to_string(&input).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["expires_at"], "2025-12-31T23:59:59Z");
        assert!(!json.contains("royalty_address")); // Should be skipped
        assert!(!json.contains("royalty_percentage")); // Should be skipped
    }

    #[test]
    fn test_mint_token_input_backward_compatible() {
        // Test that old-style input without new fields still works
        let json = r#"{
            "contract": "xion1contract",
            "token_id": "3",
            "owner": "xion1owner"
        }"#;

        let input: MintTokenInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.contract, "xion1contract");
        assert_eq!(input.token_id, "3");
        assert_eq!(input.owner, "xion1owner");
        assert_eq!(input.token_uri, None);
        assert_eq!(input.royalty_address, None);
        assert_eq!(input.royalty_percentage, None);
        assert_eq!(input.expires_at, None);
        assert_eq!(input.asset_type, None); // Defaults to None (will use Cw721Base)
    }

    #[test]
    fn test_mint_token_input_with_asset_type() {
        let json = r#"{
            "contract": "xion1contract",
            "token_id": "4",
            "owner": "xion1owner",
            "asset_type": "cw2981-royalties",
            "royalty_address": "xion1artist",
            "royalty_percentage": 0.05
        }"#;

        let input: MintTokenInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.asset_type, Some(AssetType::Cw2981Royalties));
        assert_eq!(input.royalty_address, Some("xion1artist".to_string()));
        assert_eq!(input.royalty_percentage, Some(0.05));
    }

    // ========================================================================
    // Address Prediction Tests
    // ========================================================================

    #[test]
    fn test_predict_address_input_serialization() {
        let input = PredictAddressInput {
            asset_type: AssetType::Cw721Base,
            name: "My Collection".to_string(),
            symbol: "NFT".to_string(),
            minter: Some("xion1abc123".to_string()),
            code_id: Some(522),
            salt: "6162636465666768696a6b6c6d6e6f707172737475767778797a303132333435".to_string(),
        };

        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"type\":\"cw721-base\""));
        assert!(json.contains("\"name\":\"My Collection\""));
        assert!(json.contains("\"symbol\":\"NFT\""));
        assert!(json.contains("\"salt\":\"616263"));
    }

    #[test]
    fn test_predict_address_result_serialization() {
        let result = PredictAddressResult {
            success: true,
            contract_address: "xion1predicted".to_string(),
            code_id: 522,
            salt: "616263".to_string(),
            creator: "xion1creator".to_string(),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"contract_address\":\"xion1predicted\""));
        assert!(json.contains("\"code_id\":522"));
    }

    // ========================================================================
    // Batch Minting Tests
    // ========================================================================

    #[test]
    fn test_batch_mint_token_serialization() {
        let token = BatchMintToken {
            token_id: "1".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: Some("ipfs://hash".to_string()),
            extension: serde_json::json!({"name": "NFT #1"}),
            royalty_address: None,
            royalty_percentage: None,
            expires_at: None,
        };

        let json = serde_json::to_string(&token).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["token_id"], "1");
        assert_eq!(parsed["owner"], "xion1owner");
        assert_eq!(parsed["token_uri"], "ipfs://hash");
    }

    #[test]
    fn test_batch_mint_token_with_royalty() {
        let token = BatchMintToken {
            token_id: "royalty-1".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: None,
            extension: serde_json::json!({}),
            royalty_address: Some("xion1artist".to_string()),
            royalty_percentage: Some(0.05),
            expires_at: None,
        };

        let json = serde_json::to_string(&token).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["royalty_address"], "xion1artist");
        assert_eq!(parsed["royalty_percentage"], 0.05);
    }

    #[test]
    fn test_batch_mint_token_with_expiration() {
        let token = BatchMintToken {
            token_id: "exp-1".to_string(),
            owner: "xion1owner".to_string(),
            token_uri: None,
            extension: serde_json::json!({}),
            royalty_address: None,
            royalty_percentage: None,
            expires_at: Some("2025-12-31T23:59:59Z".to_string()),
        };

        let json = serde_json::to_string(&token).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["expires_at"], "2025-12-31T23:59:59Z");
    }

    #[test]
    fn test_batch_mint_input_serialization() {
        let input = BatchMintInput {
            contract: "xion1contract".to_string(),
            asset_type: AssetType::Cw721Base,
            tokens: vec![
                BatchMintToken {
                    token_id: "1".to_string(),
                    owner: "xion1owner1".to_string(),
                    token_uri: Some("ipfs://hash1".to_string()),
                    extension: serde_json::json!({}),
                    royalty_address: None,
                    royalty_percentage: None,
                    expires_at: None,
                },
                BatchMintToken {
                    token_id: "2".to_string(),
                    owner: "xion1owner2".to_string(),
                    token_uri: Some("ipfs://hash2".to_string()),
                    extension: serde_json::json!({}),
                    royalty_address: None,
                    royalty_percentage: None,
                    expires_at: None,
                },
            ],
        };

        let json = serde_json::to_string(&input).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["contract"], "xion1contract");
        assert_eq!(parsed["type"], "cw721-base");
        assert_eq!(parsed["tokens"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_batch_mint_result_serialization() {
        let result = BatchMintResult {
            success: true,
            contract_address: "xion1contract".to_string(),
            tx_hash: "ABC123".to_string(),
            total: 2,
            succeeded: 2,
            failed: 0,
            results: vec![
                BatchMintTokenResult {
                    token_id: "1".to_string(),
                    success: true,
                    error: None,
                },
                BatchMintTokenResult {
                    token_id: "2".to_string(),
                    success: true,
                    error: None,
                },
            ],
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["success"], true);
        assert_eq!(parsed["total"], 2);
        assert_eq!(parsed["succeeded"], 2);
        assert_eq!(parsed["failed"], 0);
    }

    #[test]
    fn test_batch_mint_result_with_failures() {
        let result = BatchMintResult {
            success: false,
            contract_address: "xion1contract".to_string(),
            tx_hash: "ABC123".to_string(),
            total: 2,
            succeeded: 1,
            failed: 1,
            results: vec![
                BatchMintTokenResult {
                    token_id: "1".to_string(),
                    success: true,
                    error: None,
                },
                BatchMintTokenResult {
                    token_id: "2".to_string(),
                    success: false,
                    error: Some("Duplicate token ID".to_string()),
                },
            ],
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["success"], false);
        assert_eq!(parsed["failed"], 1);
        assert_eq!(parsed["results"][1]["error"], "Duplicate token ID");
    }

    // ========================================================================
    // Asset Type Cleanup Tests
    // ========================================================================

    #[test]
    fn test_asset_type_fixed_price_removed() {
        // Verify that cw721-fixed-price is no longer a valid type
        assert_eq!(AssetType::parse("cw721-fixed-price"), None);

        // Verify all() only returns 5 types
        let all = AssetType::all();
        assert_eq!(all.len(), 5);

        // Verify none of them is FixedPrice
        for t in all {
            assert_ne!(t.as_str(), "cw721-fixed-price");
        }
    }
}
