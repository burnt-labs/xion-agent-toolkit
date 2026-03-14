//! Account CLI Commands
//!
//! Commands for querying MetaAccount (Smart Account) information.

use anyhow::Result;
use clap::Subcommand;
use tracing::info;

use crate::account::AccountInfoOutput;
use crate::api::OAuth2ApiClient;
use crate::config::ConfigManager;
use crate::oauth::OAuthClient;

#[derive(Subcommand)]
pub enum AccountCommands {
    /// Show current user's MetaAccount info (address, authenticators, balances)
    Info,
}

pub async fn handle_command(cmd: AccountCommands) -> Result<()> {
    match cmd {
        AccountCommands::Info => handle_info().await,
    }
}

async fn handle_info() -> Result<()> {
    use crate::utils::output::{print_info, print_json};

    print_info("Querying MetaAccount info...");

    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;

    if !oauth_client.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated",
            "code": "NOT_AUTHENTICATED",
            "hint": "Run 'xion-toolkit auth login' first"
        });
        return print_json(&result);
    }

    let credentials = match oauth_client.get_credentials()? {
        Some(creds) => creds,
        None => {
            let result = serde_json::json!({
                "success": false,
                "error": "Credentials not found",
                "code": "CREDENTIALS_NOT_FOUND",
                "hint": "Run 'xion-toolkit auth login' to authenticate"
            });
            return print_json(&result);
        }
    };

    let address = credentials
        .xion_address
        .clone()
        .unwrap_or_else(|| "unknown".to_string());
    info!("Querying MetaAccount info for: {}", address);

    let api_client = OAuth2ApiClient::new(network_config.oauth_api_url.clone());

    match api_client.get_user_info(&credentials.access_token).await {
        Ok(user_info) => {
            info!("Successfully retrieved MetaAccount info");
            print_info(&format!("MetaAccount address: {}", user_info.id));
            let output: AccountInfoOutput = user_info.into();
            print_json(&output)
        }
        Err(e) => {
            info!("Failed to query MetaAccount: {}", e);
            let result = serde_json::json!({
                "success": false,
                "error": e.to_string(),
                "code": "ACCOUNT_QUERY_FAILED",
                "hint": "Failed to query MetaAccount via OAuth2 API. Check network connection or try re-authenticating."
            });
            print_json(&result)
        }
    }
}
