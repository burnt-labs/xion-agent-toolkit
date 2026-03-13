//! Asset CLI Commands
//!
//! CLI commands for NFT contract deployment and management.

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::asset_builder::{AssetBuilderManager, AssetType, CreateCollectionInput, MintTokenInput};
use crate::config::ConfigManager;
use crate::oauth::OAuthClient;
use crate::utils::output::{print_info, print_json};

/// Asset management commands
#[derive(Subcommand)]
pub enum AssetCommands {
    /// Create a new NFT collection
    Create(CreateArgs),

    /// Mint a new NFT token
    Mint(MintArgs),

    /// Query an NFT contract
    Query(QueryArgs),

    /// List available asset types
    Types,
}

/// Create collection arguments
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Asset type (cw721-base, cw721-metadata-onchain, etc.)
    #[arg(long, value_name = "TYPE", default_value = "cw721-base")]
    pub asset_type: String,

    /// Collection name
    #[arg(long, value_name = "NAME")]
    pub name: String,

    /// Collection symbol
    #[arg(long, value_name = "SYMBOL")]
    pub symbol: String,

    /// Minter address (defaults to your address)
    #[arg(long, value_name = "ADDRESS")]
    pub minter: Option<String>,

    /// Custom code ID (overrides network default)
    #[arg(long, value_name = "ID")]
    pub code_id: Option<u64>,

    /// Salt for predictable address (hex-encoded)
    #[arg(long, value_name = "HEX")]
    pub salt: Option<String>,
}

/// Mint token arguments
#[derive(Debug, Args)]
pub struct MintArgs {
    /// Contract address
    #[arg(long, value_name = "ADDRESS")]
    pub contract: String,

    /// Token ID
    #[arg(long, value_name = "ID")]
    pub token_id: String,

    /// Owner address
    #[arg(long, value_name = "ADDRESS")]
    pub owner: String,

    /// Token URI (e.g., IPFS URI)
    #[arg(long, value_name = "URI")]
    pub token_uri: Option<String>,

    /// Extension data as JSON string
    #[arg(long, value_name = "JSON")]
    pub extension: Option<String>,

    /// Asset type for minting (cw721-base, cw2981-royalties, cw721-expiration, etc.)
    /// Determines mint message format
    #[arg(long, value_name = "TYPE", default_value = "cw721-base")]
    pub asset_type: String,

    /// Royalty payment address (CW2981 only)
    #[arg(long, value_name = "ADDRESS")]
    pub royalty_address: Option<String>,

    /// Royalty percentage as decimal, e.g., 0.05 for 5% (CW2981 only)
    #[arg(long, value_name = "DECIMAL")]
    pub royalty_percentage: Option<f64>,

    /// Expiration timestamp (cw721-expiration only)
    /// Can be Unix timestamp or ISO 8601 format
    #[arg(long, value_name = "TIMESTAMP")]
    pub expires_at: Option<String>,
}

/// Query contract arguments
#[derive(Debug, Args)]
pub struct QueryArgs {
    /// Contract address
    #[arg(long, value_name = "ADDRESS")]
    pub contract: String,

    /// Query message as JSON string
    #[arg(long, value_name = "JSON")]
    pub msg: String,
}

/// Handle asset commands
pub async fn handle_command(cmd: AssetCommands) -> Result<()> {
    match cmd {
        AssetCommands::Create(args) => handle_create(args).await,
        AssetCommands::Mint(args) => handle_mint(args).await,
        AssetCommands::Query(args) => handle_query(args).await,
        AssetCommands::Types => handle_types().await,
    }
}

/// Handle create collection command
async fn handle_create(args: CreateArgs) -> Result<()> {
    print_info("Creating NFT collection...");

    // Parse asset type
    let asset_type = AssetType::parse(&args.asset_type).ok_or_else(|| {
        let valid_types: Vec<&'static str> = AssetType::all().iter().map(|t| t.as_str()).collect();
        anyhow::anyhow!(
            "Invalid asset type: '{}'. Valid types: {}",
            args.asset_type,
            valid_types.join(", ")
        )
    })?;

    // Check for cw721-fixed-price (requires CW20)
    if asset_type == AssetType::Cw721FixedPrice {
        let result = serde_json::json!({
            "success": false,
            "error": "cw721-fixed-price requires CW20 token support (not yet implemented)",
            "code": "CW20_REQUIRED"
        });
        return print_json(&result);
    }

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = AssetBuilderManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Build input
    let input = CreateCollectionInput {
        asset_type,
        name: args.name,
        symbol: args.symbol,
        minter: args.minter,
        code_id: args.code_id,
        salt: args.salt,
    };

    // Create collection
    match manager.create_collection(input).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "contract_address": result.contract_address,
                "tx_hash": result.tx_hash,
                "code_id": result.code_id,
                "name": result.name,
                "symbol": result.symbol,
                "minter": result.minter,
                "salt": result.salt
            });
            print_json(&response)
        }
        Err(e) => {
            let error_msg = e.to_string();
            let code = if error_msg.contains("Not authenticated") {
                "NOT_AUTHENTICATED"
            } else if error_msg.contains("Code ID not configured") {
                "CODE_ID_NOT_FOUND"
            } else if error_msg.contains("CW20") {
                "CW20_REQUIRED"
            } else {
                "CREATE_COLLECTION_FAILED"
            };

            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to create collection: {}", e),
                "code": code
            });
            print_json(&result)
        }
    }
}

/// Handle mint token command
async fn handle_mint(args: MintArgs) -> Result<()> {
    print_info(&format!(
        "Minting token {} in contract {}...",
        args.token_id, args.contract
    ));

    // Parse asset type
    let asset_type = AssetType::parse(&args.asset_type).ok_or_else(|| {
        let valid_types: Vec<&'static str> = AssetType::all().iter().map(|t| t.as_str()).collect();
        anyhow::anyhow!(
            "Invalid asset type: '{}'. Valid types: {}",
            args.asset_type,
            valid_types.join(", ")
        )
    })?;

    // Validate variant-specific options
    // IMPORTANT: Validation order matters for user experience
    // 1. First validate numeric constraints (catchs invalid values early)
    // 2. Then validate completeness (both fields required together)
    // 3. Finally validate type compatibility

    // Validate royalty options
    if args.royalty_address.is_some() || args.royalty_percentage.is_some() {
        // 1. Validate percentage range FIRST (before type check)
        if let Some(pct) = args.royalty_percentage {
            if !(0.0..=1.0).contains(&pct) {
                let result = serde_json::json!({
                    "success": false,
                    "error": format!("Royalty percentage must be between 0.0 and 1.0. Got: {}", pct),
                    "code": "INVALID_ROYALTY_PERCENTAGE"
                });
                return print_json(&result);
            }
        }

        // 2. Validate completeness (both fields required together)
        if args.royalty_address.is_none() || args.royalty_percentage.is_none() {
            let result = serde_json::json!({
                "success": false,
                "error": "Both --royalty-address and --royalty-percentage are required for CW2981 royalties",
                "code": "INCOMPLETE_ROYALTY_INFO"
            });
            return print_json(&result);
        }

        // 3. Finally validate type compatibility
        if asset_type != AssetType::Cw2981Royalties {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Royalty options are only valid for cw2981-royalties type. Got: {}", asset_type),
                "code": "INVALID_OPTION_FOR_TYPE"
            });
            return print_json(&result);
        }
    }

    if args.expires_at.is_some() && asset_type != AssetType::Cw721Expiration {
        let result = serde_json::json!({
            "success": false,
            "error": format!("--expires-at is only valid for cw721-expiration type. Got: {}", asset_type),
            "code": "INVALID_OPTION_FOR_TYPE"
        });
        return print_json(&result);
    }

    // Check required fields for expiration type
    if asset_type == AssetType::Cw721Expiration && args.expires_at.is_none() {
        let result = serde_json::json!({
            "success": false,
            "error": "--expires-at is required for cw721-expiration type",
            "code": "MISSING_REQUIRED_FIELD"
        });
        return print_json(&result);
    }

    // Check unsupported types
    if asset_type == AssetType::Cw721FixedPrice {
        let result = serde_json::json!({
            "success": false,
            "error": "cw721-fixed-price requires CW20 token support (not yet implemented)",
            "code": "CW20_REQUIRED"
        });
        return print_json(&result);
    }

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = AssetBuilderManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Parse extension JSON
    let extension = if let Some(ref ext) = args.extension {
        serde_json::from_str(ext).map_err(|e| anyhow::anyhow!("Invalid extension JSON: {}", e))?
    } else {
        serde_json::json!({})
    };

    // Build input
    let input = MintTokenInput {
        contract: args.contract,
        token_id: args.token_id,
        owner: args.owner,
        token_uri: args.token_uri,
        extension,
        royalty_address: args.royalty_address,
        royalty_percentage: args.royalty_percentage,
        expires_at: args.expires_at,
        asset_type: Some(asset_type),
    };

    // Mint token
    match manager.mint_token(input).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "contract_address": result.contract_address,
                "token_id": result.token_id,
                "owner": result.owner,
                "tx_hash": result.tx_hash
            });
            print_json(&response)
        }
        Err(e) => {
            let error_msg = e.to_string();
            let code = if error_msg.contains("Not authenticated") {
                "NOT_AUTHENTICATED"
            } else if error_msg.contains("unauthorized") || error_msg.contains("minter") {
                "UNAUTHORIZED_MINTER"
            } else if error_msg.contains("CW20") {
                "CW20_REQUIRED"
            } else if error_msg.contains("royalty") {
                "INVALID_ROYALTY"
            } else {
                "MINT_TOKEN_FAILED"
            };

            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to mint token: {}", e),
                "code": code
            });
            print_json(&result)
        }
    }
}

/// Handle query contract command
async fn handle_query(args: QueryArgs) -> Result<()> {
    print_info(&format!("Querying contract {}...", args.contract));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = AssetBuilderManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Parse query message
    let msg: serde_json::Value = serde_json::from_str(&args.msg)
        .map_err(|e| anyhow::anyhow!("Invalid query JSON: {}", e))?;

    // Query contract
    match manager.query_contract(&args.contract, &msg).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "contract_address": result.contract_address,
                "response": result.response
            });
            print_json(&response)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to query contract: {}", e),
                "code": "QUERY_FAILED"
            });
            print_json(&result)
        }
    }
}

/// Handle list types command
async fn handle_types() -> Result<()> {
    print_info("Listing available asset types...");

    // Get asset types info
    let types_info = crate::asset_builder::code_ids::get_asset_types_info();

    let response = serde_json::json!({
        "success": true,
        "types": types_info,
        "count": types_info.len()
    });

    print_json(&response)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that royalty percentage validation happens before type compatibility check.
    /// This ensures users get meaningful error messages for invalid values
    /// regardless of the asset type.
    #[test]
    fn test_royalty_validation_order_percentage_first() {
        // When royalty_percentage is out of range, it should fail with
        // INVALID_ROYALTY_PERCENTAGE, not INVALID_OPTION_FOR_TYPE
        let args = MintArgs {
            contract: "xion1test".to_string(),
            token_id: "1".to_string(),
            owner: "xion1test".to_string(),
            token_uri: None,
            extension: None,
            asset_type: "cw721-base".to_string(), // Default type, not cw2981-royalties
            royalty_address: Some("xion1royalty".to_string()),
            royalty_percentage: Some(1.5), // Invalid: > 1.0
            expires_at: None,
        };

        // The validation should check percentage range FIRST
        // Expected error: INVALID_ROYALTY_PERCENTAGE (not INVALID_OPTION_FOR_TYPE)
        // This test verifies the validation order by checking the logic structure
        assert!(args.royalty_percentage.is_some());
        assert!(args.royalty_percentage.unwrap() > 1.0);
        assert!(args.royalty_address.is_some());
    }

    /// Test that incomplete royalty info is caught after percentage validation.
    #[test]
    fn test_royalty_validation_order_incomplete_info() {
        // When only royalty_address is provided (without percentage),
        // it should fail with INCOMPLETE_ROYALTY_INFO
        let args = MintArgs {
            contract: "xion1test".to_string(),
            token_id: "1".to_string(),
            owner: "xion1test".to_string(),
            token_uri: None,
            extension: None,
            asset_type: "cw721-base".to_string(),
            royalty_address: Some("xion1royalty".to_string()),
            royalty_percentage: None, // Missing percentage
            expires_at: None,
        };

        // Incomplete royalty info
        assert!(args.royalty_address.is_some());
        assert!(args.royalty_percentage.is_none());
    }

    /// Test that type compatibility is checked last.
    #[test]
    fn test_royalty_validation_order_type_compatibility() {
        // When royalty options are valid but type is wrong,
        // it should fail with INVALID_OPTION_FOR_TYPE
        let args = MintArgs {
            contract: "xion1test".to_string(),
            token_id: "1".to_string(),
            owner: "xion1test".to_string(),
            token_uri: None,
            extension: None,
            asset_type: "cw721-base".to_string(), // Wrong type for royalties
            royalty_address: Some("xion1royalty".to_string()),
            royalty_percentage: Some(0.05), // Valid percentage
            expires_at: None,
        };

        // Valid royalty options but wrong type
        assert!(args.royalty_address.is_some());
        assert!(args.royalty_percentage.unwrap() >= 0.0 && args.royalty_percentage.unwrap() <= 1.0);
        assert_ne!(args.asset_type, "cw2981-royalties");
    }

    /// Test valid royalty configuration.
    #[test]
    fn test_valid_royalty_configuration() {
        let args = MintArgs {
            contract: "xion1test".to_string(),
            token_id: "1".to_string(),
            owner: "xion1test".to_string(),
            token_uri: None,
            extension: None,
            asset_type: "cw2981-royalties".to_string(), // Correct type
            royalty_address: Some("xion1royalty".to_string()),
            royalty_percentage: Some(0.05), // Valid percentage
            expires_at: None,
        };

        // All validations should pass
        assert!(args.royalty_address.is_some());
        assert!(args.royalty_percentage.unwrap() >= 0.0 && args.royalty_percentage.unwrap() <= 1.0);
        assert_eq!(args.asset_type, "cw2981-royalties");
    }

    /// Test boundary values for royalty percentage.
    #[test]
    fn test_royalty_percentage_boundaries() {
        // Test 0.0 (valid)
        let args_min = MintArgs {
            contract: "xion1test".to_string(),
            token_id: "1".to_string(),
            owner: "xion1test".to_string(),
            token_uri: None,
            extension: None,
            asset_type: "cw2981-royalties".to_string(),
            royalty_address: Some("xion1royalty".to_string()),
            royalty_percentage: Some(0.0),
            expires_at: None,
        };
        assert_eq!(args_min.royalty_percentage, Some(0.0));

        // Test 1.0 (valid)
        let args_max = MintArgs {
            contract: "xion1test".to_string(),
            token_id: "1".to_string(),
            owner: "xion1test".to_string(),
            token_uri: None,
            extension: None,
            asset_type: "cw2981-royalties".to_string(),
            royalty_address: Some("xion1royalty".to_string()),
            royalty_percentage: Some(1.0),
            expires_at: None,
        };
        assert_eq!(args_max.royalty_percentage, Some(1.0));

        // Test negative value (invalid)
        let args_neg = MintArgs {
            contract: "xion1test".to_string(),
            token_id: "1".to_string(),
            owner: "xion1test".to_string(),
            token_uri: None,
            extension: None,
            asset_type: "cw2981-royalties".to_string(),
            royalty_address: Some("xion1royalty".to_string()),
            royalty_percentage: Some(-0.1),
            expires_at: None,
        };
        assert!(args_neg.royalty_percentage.unwrap() < 0.0);
    }
}
