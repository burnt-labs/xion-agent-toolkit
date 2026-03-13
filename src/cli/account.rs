//! Account CLI Commands
//!
//! Commands for querying MetaAccount (Smart Account) information.

use anyhow::Result;
use clap::Subcommand;
use tracing::info;

use crate::account::{AccountClient, AccountInfoOutput};
use crate::config::ConfigManager;
use crate::oauth::OAuthClient;
use crate::utils::output::print_json;

#[derive(Subcommand)]
pub enum AccountCommands {
    /// Show current user's MetaAccount info ( authenticators)
    Info,
}

pub async fn handle_command(cmd: AccountCommands) -> Result<()> {
    match cmd {
        AccountCommands::Info => handle_info().await?,
    }
}

async fn handle_info() -> Result<()> {
    // Check authentication first
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;

    if !oauth_client.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated",
            "error_code": "NOT_AUTHENTICATED",
            "hint": "Run 'xion-toolkit auth login' first"
        });
        print_json(&result);
        return Ok(());
    }

    // Get user's address from credentials
    let credentials = match oauth_client.get_credentials()? {
        Some(creds) => creds,
        None => {
            let result = serde_json::json!({
                "success": false,
                "error": "Credentials not found",
                "error_code": "CREDENTIALS_NOT_FOUND",
                "hint": "Run 'xion-toolkit auth login' to authenticate"
            });
            print_json(&result);
            return Ok(());
        }
    };

    let address = credentials.xion_address;
    info!("Querying MetaAccount info for: {}", address);

    // Create account client and query
    let account_client = AccountClient::new(&network_config);

    match account_client.get_smart_account(&address).await {
        Ok(account) => {
            info!("Successfully retrieved MetaAccount info");
            let output: AccountInfoOutput = account.into();
            print_json(&output);
        }
        Err(e) => {
            info!("Failed to query MetaAccount: {}", e);
            let result = serde_json::json!({
                "success": false,
                "error": e.to_string(),
                "error_code": "ACCOUNT_QUERY_FAILED",
                "hint": "Failed to query MetaAccount from indexer. The account may not exist yet, or the indexer may be unavailable."
            });
            print_json(&result);
        }
    }

    Ok(())
}
