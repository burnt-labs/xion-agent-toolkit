use clap::Subcommand;
use anyhow::Result;

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Login using OAuth2 flow
    Login {
        /// Callback port (default: 54321)
        #[arg(short, long, default_value = "54321")]
        port: Option<u16>,
    },

    /// Logout and clear stored credentials
    Logout,

    /// Show authentication status
    Status,

    /// Refresh access token
    Refresh,
}

pub async fn handle_command(cmd: AuthCommands) -> Result<()> {
    match cmd {
        AuthCommands::Login { port } => handle_login(port).await?,
        AuthCommands::Logout => handle_logout()?,
        AuthCommands::Status => handle_status()?,
        AuthCommands::Refresh => handle_refresh().await?,
    }
    Ok(())
}

async fn handle_login(port: Option<u16>) -> Result<()> {
    use crate::utils::output::{print_json, print_info};
    use crate::config::ConfigManager;

    print_info("Starting OAuth2 login flow...");

    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    
    let callback_port = port.unwrap_or(network_config.callback_port);

    print_info(&format!("Network: {}", config_manager.get_current_network()));
    print_info(&format!("OAuth API: {}", network_config.oauth_api_url));
    print_info(&format!("Client ID: {}", network_config.oauth_client_id));
    print_info(&format!("Callback port: {}", callback_port));

    // TODO: Implement OAuth2 login with callback server
    // For now, return a placeholder response
    let result = serde_json::json!({
        "success": false,
        "message": "OAuth2 login not yet implemented",
        "network": config_manager.get_current_network(),
        "oauth_api_url": network_config.oauth_api_url,
        "client_id": network_config.oauth_client_id,
        "callback_port": callback_port
    });

    print_json(&result)
}

fn handle_logout() -> Result<()> {
    use crate::utils::output::{print_json, print_info};
    use crate::config::{ConfigManager, CredentialsManager};

    print_info("Logging out...");

    let config_manager = ConfigManager::new()?;
    let network = config_manager.get_current_network();
    
    let credentials_manager = CredentialsManager::new(network)?;
    credentials_manager.clear_credentials()?;

    let result = serde_json::json!({
        "success": true,
        "message": "Logged out successfully",
        "network": network
    });

    print_json(&result)
}

fn handle_status() -> Result<()> {
    use crate::utils::output::print_json;
    use crate::config::{ConfigManager, CredentialsManager};

    let config_manager = ConfigManager::new()?;
    let network = config_manager.get_current_network();
    let network_config = config_manager.get_network_config()?;

    let credentials_manager = CredentialsManager::new(network)?;
    let has_credentials = credentials_manager.has_credentials()?;

    let mut result = serde_json::json!({
        "network": network,
        "chain_id": network_config.chain_id,
        "oauth_api_url": network_config.oauth_api_url,
        "authenticated": has_credentials,
    });

    if has_credentials {
        match credentials_manager.load_credentials() {
            Ok(creds) => {
                result["xion_address"] = serde_json::json!(creds.xion_address);
                result["expires_at"] = serde_json::json!(creds.expires_at);
            }
            Err(e) => {
                result["error"] = serde_json::json!(format!("Failed to load credentials: {}", e));
            }
        }
    } else {
        result["message"] = serde_json::json!("Not authenticated. Please run 'xion auth login' first.");
    }

    print_json(&result)
}

async fn handle_refresh() -> Result<()> {
    use crate::utils::output::{print_json, print_info};
    use crate::config::{ConfigManager, CredentialsManager};

    print_info("Refreshing access token...");

    let config_manager = ConfigManager::new()?;
    let network = config_manager.get_current_network();
    
    let credentials_manager = CredentialsManager::new(network)?;
    
    if !credentials_manager.has_credentials()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first."
        });
        return print_json(&result);
    }

    // TODO: Implement token refresh
    let result = serde_json::json!({
        "success": false,
        "message": "Token refresh not yet implemented",
        "network": network
    });

    print_json(&result)
}

