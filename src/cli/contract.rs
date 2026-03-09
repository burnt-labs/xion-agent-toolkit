use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum ContractCommands {
    /// Instantiate a new smart contract (v1 - dynamic address)
    Instantiate(InstantiateArgs),

    /// Instantiate a contract with predictable address (v2 - instantiate2)
    Instantiate2(Instantiate2Args),
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

pub async fn handle_command(cmd: ContractCommands) -> Result<()> {
    match cmd {
        ContractCommands::Instantiate(args) => handle_instantiate(args).await,
        ContractCommands::Instantiate2(args) => handle_instantiate2(args).await,
    }
}

async fn handle_instantiate(args: InstantiateArgs) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

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
        return print_json(&serde_json::json!({
            "success": false,
            "code": "NOT_AUTHENTICATED",
            "error": "Not authenticated",
            "suggestion": "Run 'xion-toolkit auth login' first"
        }));
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
            print_json(&response)
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
            print_json(&result)
        }
    }
}

async fn handle_instantiate2(args: Instantiate2Args) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

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
        return print_json(&serde_json::json!({
            "success": false,
            "code": "NOT_AUTHENTICATED",
            "error": "Not authenticated",
            "suggestion": "Run 'xion-toolkit auth login' first"
        }));
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
            print_json(&response)
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
            print_json(&result)
        }
    }
}
