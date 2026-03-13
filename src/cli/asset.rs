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
            } else if error_msg.contains("Phase 1 only supports") {
                "UNSUPPORTED_ASSET_TYPE"
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
