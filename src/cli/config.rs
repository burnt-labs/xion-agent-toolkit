use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
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
        ConfigCommands::Show => handle_show(),
        ConfigCommands::Set { key, value } => handle_set(&key, &value),
        ConfigCommands::Get { key } => handle_get(&key),
        ConfigCommands::Reset => handle_reset(),
    }
}

fn handle_show() -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_json;

    let config_manager = ConfigManager::new()?;
    let config = config_manager.load_config()?;

    print_json(&config)
}

fn handle_set(key: &str, value: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Setting {} = {}", key, value));

    let mut config_manager = ConfigManager::new()?;
    config_manager.set_value(key, value)?;
    config_manager.save_config()?;

    let result = serde_json::json!({
        "success": true,
        "message": "Configuration updated",
        "key": key,
        "value": value
    });

    print_json(&result)
}

fn handle_get(key: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_json;

    let config_manager = ConfigManager::new()?;
    let value = config_manager.get_value(key)?;

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

    let config_manager = ConfigManager::new()?;
    config_manager.reset_config()?;

    let result = serde_json::json!({
        "success": true,
        "message": "Configuration reset to defaults"
    });

    print_json(&result)
}
