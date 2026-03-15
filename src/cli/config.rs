use anyhow::Result;
use clap::Subcommand;

use crate::cli::ExecuteContext;

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

pub fn handle_command(cmd: ConfigCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        ConfigCommands::Show => handle_show(ctx)?,
        ConfigCommands::SetNetwork { network } => handle_set_network(&network, ctx)?,
        ConfigCommands::Get { key } => handle_get(&key, ctx)?,
        ConfigCommands::Reset => handle_reset(ctx)?,
    }
    Ok(())
}

fn handle_show(ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_formatted;

    let config_manager = ConfigManager::new()?;
    let config = config_manager.load_config()?;
    let network_config = config_manager.get_network_config()?;

    let result = serde_json::json!({
        "config": {
            "version": config.version,
            "network": config.network,
        },
        "network_config": {
            "network_name": network_config.network_name,
            "chain_id": network_config.chain_id,
            "oauth_api_url": network_config.oauth_api_url,
            "rpc_url": network_config.rpc_url,
            "oauth_client_id": network_config.oauth_client_id,
            "treasury_code_id": network_config.treasury_code_id,
            "callback_port": network_config.callback_port,
            "indexer_url": network_config.indexer_url,
        }
    });

    print_formatted(&result, ctx.output_format())
}

fn handle_set_network(network: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::{print_formatted, print_info};

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

    print_formatted(&result, ctx.output_format())
}

fn handle_get(key: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_formatted;

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
            return print_formatted(&result, ctx.output_format());
        }
    };

    let result = serde_json::json!({
        "key": key,
        "value": value
    });

    print_formatted(&result, ctx.output_format())
}

fn handle_reset(ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info("Resetting configuration to defaults...");

    let mut config_manager = ConfigManager::new()?;
    config_manager.reset_config()?;

    let result = serde_json::json!({
        "success": true,
        "message": "Configuration reset to defaults",
        "network": "testnet"
    });

    print_formatted(&result, ctx.output_format())
}
