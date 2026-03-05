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
    use crate::oauth::OAuthClient;
    use tracing::info;

    let config_manager = ConfigManager::new()?;
    let mut network_config = config_manager.get_network_config()?;
    
    // Override callback port if specified
    if let Some(p) = port {
        network_config.callback_port = p;
    }

    print_info(&format!("Starting OAuth2 login on network: {}...", config_manager.get_current_network()));
    print_info(&format!("OAuth API: {}", network_config.oauth_api_url));
    print_info(&format!("Callback port: {}", network_config.callback_port));

    info!("Creating OAuth2 client");
    
    // Create OAuth client
    let oauth_client = OAuthClient::new(network_config.clone())?;

    // Execute login flow
    info!("Starting OAuth2 login flow");
    match oauth_client.login().await {
        Ok(credentials) => {
            info!("Login successful");
            let result = serde_json::json!({
                "success": true,
                "network": config_manager.get_current_network(),
                "xion_address": credentials.xion_address,
                "expires_at": credentials.expires_at,
            });
            print_json(&result)
        }
        Err(e) => {
            info!("Login failed: {}", e);
            let result = serde_json::json!({
                "success": false,
                "error": format!("Login failed: {}", e),
                "code": "AUTH_LOGIN_FAILED",
                "suggestion": "Please try again or check your browser for authorization"
            });
            print_json(&result)
        }
    }
}

fn handle_logout() -> Result<()> {
    use crate::utils::output::{print_json, print_info};
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use tracing::info;

    print_info("Logging out...");

    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config)?;

    info!("Logging out from network: {}", config_manager.get_current_network());
    
    match oauth_client.logout() {
        Ok(()) => {
            info!("Logout successful");
            let result = serde_json::json!({
                "success": true,
                "message": "Logged out successfully",
                "network": config_manager.get_current_network()
            });
            print_json(&result)
        }
        Err(e) => {
            info!("Logout failed: {}", e);
            let result = serde_json::json!({
                "success": false,
                "error": format!("Logout failed: {}", e),
                "code": "AUTH_LOGOUT_FAILED"
            });
            print_json(&result)
        }
    }
}

fn handle_status() -> Result<()> {
    use crate::utils::output::print_json;
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use tracing::info;

    let config_manager = ConfigManager::new()?;
    let network = config_manager.get_current_network();
    let network_config = config_manager.get_network_config()?;

    info!("Checking authentication status for network: {}", network);

    // Create OAuth client
    let oauth_client = OAuthClient::new(network_config.clone())?;

    // Check if authenticated using OAuthClient
    let is_authenticated = oauth_client.is_authenticated()?;

    let mut result = serde_json::json!({
        "network": network,
        "chain_id": network_config.chain_id,
        "oauth_api_url": network_config.oauth_api_url,
        "authenticated": is_authenticated,
    });

    if is_authenticated {
        // Load credentials to show details
        match oauth_client.get_credentials() {
            Ok(Some(creds)) => {
                result["xion_address"] = serde_json::json!(creds.xion_address);
                result["expires_at"] = serde_json::json!(creds.expires_at);
                info!("User is authenticated: {:?}", creds.xion_address);
            }
            Ok(None) => {
                result["error"] = serde_json::json!("Credentials not found");
                info!("Credentials not found despite has_credentials returning true");
            }
            Err(e) => {
                result["error"] = serde_json::json!(format!("Failed to load credentials: {}", e));
                info!("Failed to load credentials: {}", e);
            }
        }
    } else {
        result["message"] = serde_json::json!("Not authenticated. Please run 'xion auth login' first.");
        info!("User is not authenticated");
    }

    print_json(&result)
}

async fn handle_refresh() -> Result<()> {
    use crate::utils::output::{print_json, print_info};
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use tracing::info;

    print_info("Refreshing access token...");

    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config)?;

    info!("Attempting to refresh token");
    
    // Refresh token
    match oauth_client.refresh_token().await {
        Ok(credentials) => {
            info!("Token refreshed successfully");
            let result = serde_json::json!({
                "success": true,
                "message": "Token refreshed successfully",
                "network": config_manager.get_current_network(),
                "expires_at": credentials.expires_at
            });
            print_json(&result)
        }
        Err(e) => {
            info!("Token refresh failed: {}", e);
            let result = serde_json::json!({
                "success": false,
                "error": format!("Token refresh failed: {}", e),
                "code": "AUTH_REFRESH_FAILED",
                "suggestion": "Your session may have expired. Please run 'xion auth login' to re-authenticate."
            });
            print_json(&result)
        }
    }
}

