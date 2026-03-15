use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

use crate::cli::ExecuteContext;

#[derive(Debug, Subcommand)]
pub enum ContractCommands {
    /// Instantiate a new smart contract (v1 - dynamic address)
    Instantiate(InstantiateArgs),

    /// Instantiate a contract with predictable address (v2 - instantiate2)
    Instantiate2(Instantiate2Args),

    /// Execute a message on a deployed smart contract
    Execute(ExecuteArgs),

    /// Query a smart contract (read-only, no authentication required)
    Query(QueryArgs),
}

#[derive(Debug, Args)]
pub struct InstantiateArgs {
    /// Code ID of the contract to instantiate
    #[arg(long)]
    pub code_id: u64,

    /// Label for the contract instance
    #[arg(long)]
    pub label: String,

    /// Path to JSON file containing instantiate message
    #[arg(short, long)]
    pub msg: PathBuf,

    /// Admin address for contract migrations (optional)
    #[arg(long)]
    pub admin: Option<String>,
}

#[derive(Debug, Args)]
pub struct Instantiate2Args {
    /// Code ID of the contract to instantiate
    #[arg(long)]
    pub code_id: u64,

    /// Label for the contract instance
    #[arg(long)]
    pub label: String,

    /// Path to JSON file containing instantiate message
    #[arg(short, long)]
    pub msg: PathBuf,

    /// Salt for predictable address (hex-encoded, optional - auto-generated if not provided)
    #[arg(long)]
    pub salt: Option<String>,

    /// Admin address for contract migrations (optional)
    #[arg(long)]
    pub admin: Option<String>,
}

#[derive(Debug, Args)]
pub struct ExecuteArgs {
    /// Contract address to execute
    #[arg(long)]
    pub contract: String,

    /// Path to JSON file containing execute message
    #[arg(short, long)]
    pub msg: PathBuf,

    /// Optional funds to send (e.g., "1000000uxion")
    #[arg(long)]
    pub funds: Option<String>,
}

#[derive(Debug, Args)]
pub struct QueryArgs {
    /// Contract address to query
    #[arg(long)]
    pub contract: String,

    /// Path to JSON file containing the query message
    #[arg(short, long)]
    pub msg: PathBuf,
}

pub async fn handle_command(cmd: ContractCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        ContractCommands::Instantiate(args) => handle_instantiate(args, ctx).await,
        ContractCommands::Instantiate2(args) => handle_instantiate2(args, ctx).await,
        ContractCommands::Execute(args) => handle_execute(args, ctx).await,
        ContractCommands::Query(args) => handle_query(args, ctx).await,
    }
}

async fn handle_instantiate(args: InstantiateArgs, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Instantiating contract code_id={} with label='{}'...",
        args.code_id, args.label
    ));

    // Load instantiate message from file
    let content = std::fs::read_to_string(&args.msg)
        .map_err(|e| anyhow::anyhow!("Failed to read msg file: {}", e))?;
    let msg: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid JSON in msg file: {}", e))?;

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config);

    // Check auth
    if !manager.is_authenticated()? {
        return print_formatted(&serde_json::json!({
            "success": false,
            "code": "NOT_AUTHENTICATED",
            "error": "Not authenticated",
            "suggestion": "Run 'xion-toolkit auth login' first"
        }), ctx.output_format());
    }

    // Call manager
    match manager
        .instantiate_contract(args.code_id, &msg, &args.label, args.admin.as_deref())
        .await
    {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "tx_hash": result.tx_hash,
                "code_id": result.code_id,
                "label": result.label,
                "admin": result.admin
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let error_msg = e.to_string();
            let (code, suggestion) =
                if error_msg.contains("insufficient") || error_msg.contains("balance") {
                    (
                        "INSUFFICIENT_BALANCE",
                        "Fund your account before instantiating a contract",
                    )
                } else if error_msg.contains("invalid") || error_msg.contains("format") {
                    (
                        "INVALID_INPUT",
                        "Check your instantiate message and parameters",
                    )
                } else if error_msg.contains("unauthorized") {
                    (
                        "UNAUTHORIZED",
                        "You may not have permission to perform this action",
                    )
                } else {
                    ("INSTANTIATE_FAILED", "Check the error message for details")
                };

            let result = serde_json::json!({
                "success": false,
                "code": code,
                "error": e.to_string(),
                "suggestion": suggestion
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_instantiate2(args: Instantiate2Args, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Instantiating contract2 code_id={} with label='{}'...",
        args.code_id, args.label
    ));

    // Load instantiate message from file
    let content = std::fs::read_to_string(&args.msg)
        .map_err(|e| anyhow::anyhow!("Failed to read msg file: {}", e))?;
    let msg: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid JSON in msg file: {}", e))?;

    // Parse salt if provided (hex-encoded)
    let salt_bytes = if let Some(s) = args.salt {
        Some(hex::decode(s).map_err(|e| anyhow::anyhow!("Invalid hex salt: {}", e))?)
    } else {
        None
    };

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config);

    // Check auth
    if !manager.is_authenticated()? {
        return print_formatted(&serde_json::json!({
            "success": false,
            "code": "NOT_AUTHENTICATED",
            "error": "Not authenticated",
            "suggestion": "Run 'xion-toolkit auth login' first"
        }), ctx.output_format());
    }

    // Call manager
    match manager
        .instantiate_contract2(
            args.code_id,
            &msg,
            &args.label,
            salt_bytes.as_deref(),
            args.admin.as_deref(),
        )
        .await
    {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "tx_hash": result.tx_hash,
                "code_id": result.code_id,
                "label": result.label,
                "salt": result.salt,
                "admin": result.admin,
                "predicted_address": result.predicted_address
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let error_msg = e.to_string();
            let (code, suggestion) =
                if error_msg.contains("insufficient") || error_msg.contains("balance") {
                    (
                        "INSUFFICIENT_BALANCE",
                        "Fund your account before instantiating a contract",
                    )
                } else if error_msg.contains("invalid") || error_msg.contains("format") {
                    (
                        "INVALID_INPUT",
                        "Check your instantiate message and parameters",
                    )
                } else if error_msg.contains("unauthorized") {
                    (
                        "UNAUTHORIZED",
                        "You may not have permission to perform this action",
                    )
                } else {
                    ("INSTANTIATE_FAILED", "Check the error message for details")
                };

            let result = serde_json::json!({
                "success": false,
                "code": code,
                "error": e.to_string(),
                "suggestion": suggestion
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_execute(args: ExecuteArgs, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Executing message on contract {}...",
        args.contract
    ));

    // Load execute message from file
    let content = std::fs::read_to_string(&args.msg)
        .map_err(|e| anyhow::anyhow!("Failed to read msg file: {}", e))?;
    let msg: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid JSON in msg file: {}", e))?;

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config);

    // Check auth
    if !manager.is_authenticated()? {
        return print_formatted(&serde_json::json!({
            "success": false,
            "code": "NOT_AUTHENTICATED",
            "error": "Not authenticated",
            "suggestion": "Run 'xion-toolkit auth login' first"
        }), ctx.output_format());
    }

    // Call manager
    match manager
        .execute_contract(&args.contract, &msg, args.funds.as_deref())
        .await
    {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "tx_hash": result.tx_hash,
                "contract": result.contract
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let error_msg = e.to_string();
            let (code, suggestion) =
                if error_msg.contains("insufficient") || error_msg.contains("balance") {
                    (
                        "INSUFFICIENT_BALANCE",
                        "Fund your account before executing a contract",
                    )
                } else if error_msg.contains("invalid") || error_msg.contains("format") {
                    ("INVALID_INPUT", "Check your execute message and parameters")
                } else if error_msg.contains("unauthorized") {
                    (
                        "UNAUTHORIZED",
                        "You may not have permission to perform this action",
                    )
                } else {
                    ("EXECUTE_FAILED", "Check the error message for details")
                };

            let result = serde_json::json!({
                "success": false,
                "code": code,
                "error": e.to_string(),
                "suggestion": suggestion
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_query(args: QueryArgs, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!("Querying contract {}...", args.contract));

    // Load query message from file
    let content = std::fs::read_to_string(&args.msg)
        .map_err(|e| anyhow::anyhow!("Failed to read msg file: {}", e))?;
    let msg: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid JSON in msg file: {}", e))?;

    // Create manager (for config access - no auth required for query)
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config);

    // Note: Query is read-only and doesn't require authentication

    // Call manager to query contract
    match manager.query_contract(&args.contract, &msg).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "contract": args.contract,
                "query": msg,
                "result": result
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let error_msg = e.to_string();
            let (code, suggestion) =
                if error_msg.contains("not found") || error_msg.contains("unknown") {
                    (
                        "CONTRACT_NOT_FOUND",
                        "Check that the contract address is correct",
                    )
                } else if error_msg.contains("invalid") || error_msg.contains("format") {
                    (
                        "INVALID_QUERY",
                        "Check your query message format matches the contract's query schema",
                    )
                } else if error_msg.contains("query failed") {
                    (
                        "QUERY_FAILED",
                        "The contract may have returned an error, check the query message",
                    )
                } else {
                    ("QUERY_ERROR", "Check the error message for details")
                };

            let result = serde_json::json!({
                "success": false,
                "code": code,
                "error": e.to_string(),
                "suggestion": suggestion
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}
