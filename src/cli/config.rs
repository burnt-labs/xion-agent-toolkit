use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,

    /// Set the active network
    SetNetwork {
        /// Network to use (local, testnet, mainnet)
        network: String,
    },

    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },

    /// Reset configuration to defaults
    Reset,
}

pub fn handle_command(cmd: ConfigCommands) -> Result<()> {
    match cmd {
        ConfigCommands::Show => handle_show()?,
        ConfigCommands::SetNetwork { network } => handle_set_network(&network)?,
        ConfigCommands::Get { key } => handle_get(&key)?,
        ConfigCommands::Reset => handle_reset()?,
    }
    Ok(())
}

fn handle_show() -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_json;

    let config_manager = ConfigManager::new()?;
    let config = config_manager.load_config()?;
    let network_config = config_manager.get_network_config()?;

    let result = serde_json::json!({
        "config": {
            "version": config.version,
            "network": config.network,
        },
        "network_config": {
            "chain_id": network_config.chain_id,
            "oauth_api_url": network_config.oauth_api_url,
            "rpc_url": network_config.rpc_url,
            "oauth_client_id": network_config.oauth_client_id,
            "treasury_code_id": network_config.treasury_code_id,
            "treasury_config": network_config.treasury_config,
            "callback_port": network_config.callback_port,
        }
    });

    print_json(&result)
}

fn handle_set_network(network: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Switching to network: {}", network));

    let mut config_manager = ConfigManager::new()?;
    config_manager.set_network(network)?;

    let network_config = config_manager.get_network_config()?;

    let result = serde_json::json!({
        "success": true,
        "message": "Network updated",
        "network": network,
        "chain_id": network_config.chain_id,
        "oauth_api_url": network_config.oauth_api_url
    });

    print_json(&result)
}

fn handle_get(key: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_json;

    let config_manager = ConfigManager::new()?;
    let config = config_manager.load_config()?;

    let value = match key {
        "network" => serde_json::json!(config.network),
        "version" => serde_json::json!(config.version),
        _ => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Unknown configuration key: {}", key)
            });
            return print_json(&result);
        }
    };

    let result = serde_json::json!({
        "key": key,
        "value": value
    });

    print_json(&result)
}

fn handle_reset() -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::{print_info, print_json};

    print_info("Resetting configuration to defaults...");

    let mut config_manager = ConfigManager::new()?;
    config_manager.reset_config()?;

    let result = serde_json::json!({
        "success": true,
        "message": "Configuration reset to defaults",
        "network": "testnet"
    });

    print_json(&result)
}
