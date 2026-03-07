use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use super::credentials::CredentialsManager;
use super::schema::Config;

pub struct ConfigManager {
    config_dir: PathBuf,
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        // Use unified ~/.xion-toolkit/ directory for all platforms
        let home_dir = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .context("Failed to determine home directory")?;

        let config_dir = PathBuf::from(home_dir).join(".xion-toolkit");

        // Ensure config directory exists
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;

        let config = Self::load_or_create_config(&config_dir)?;

        Ok(Self { config_dir, config })
    }

    fn config_file_path(&self) -> PathBuf {
        self.config_dir.join("config.json")
    }

    fn load_or_create_config(config_dir: &Path) -> Result<Config> {
        let config_path = config_dir.join("config.json");

        if config_path.exists() {
            let config_str =
                fs::read_to_string(&config_path).context("Failed to read config file")?;
            let config: Config =
                serde_json::from_str(&config_str).context("Failed to parse config file")?;
            Ok(config)
        } else {
            let config = Config::default();
            let config_str = serde_json::to_string_pretty(&config)
                .context("Failed to serialize default config")?;
            fs::write(&config_path, config_str).context("Failed to write default config file")?;
            Ok(config)
        }
    }

    pub fn load_config(&self) -> Result<&Config> {
        Ok(&self.config)
    }

    pub fn save_config(&mut self) -> Result<()> {
        let config_path = self.config_file_path();
        let config_str =
            serde_json::to_string_pretty(&self.config).context("Failed to serialize config")?;
        fs::write(&config_path, config_str).context("Failed to write config file")?;
        Ok(())
    }

    pub fn get_current_network(&self) -> &str {
        // Check for CLI override via environment variable
        if let Ok(network_override) = std::env::var("XION_NETWORK_OVERRIDE") {
            // Return the override value (leak to get 'static lifetime)
            // This is safe because the environment variable lives for the process lifetime
            Box::leak(network_override.into_boxed_str())
        } else {
            &self.config.network
        }
    }

    pub fn set_network(&mut self, network: &str) -> Result<()> {
        if !["local", "testnet", "mainnet"].contains(&network) {
            anyhow::bail!(
                "Invalid network: {}. Must be local, testnet, or mainnet",
                network
            );
        }
        self.config.network = network.to_string();
        self.save_config()
    }

    pub fn get_status(&self) -> Result<serde_json::Value> {
        use super::constants::{get_local_config, get_mainnet_config, get_testnet_config};

        let current_network = self.get_current_network();
        let network_config = match current_network {
            "local" => get_local_config(),
            "testnet" => get_testnet_config(),
            "mainnet" => get_mainnet_config(),
            _ => anyhow::bail!("Unknown network: {}", current_network),
        };

        // Check if user has credentials for this network
        let credentials_manager = CredentialsManager::new(current_network)?;
        let has_credentials = credentials_manager.has_credentials()?;

        Ok(serde_json::json!({
            "network": current_network,
            "chain_id": network_config.chain_id,
            "oauth_api_url": network_config.oauth_api_url,
            "authenticated": has_credentials,
            "callback_port": network_config.callback_port
        }))
    }

    pub fn get_network_config(&self) -> Result<super::constants::NetworkConfig> {
        use super::constants::{get_local_config, get_mainnet_config, get_testnet_config};

        let current_network = self.get_current_network();
        match current_network {
            "local" => Ok(get_local_config()),
            "testnet" => Ok(get_testnet_config()),
            "mainnet" => Ok(get_mainnet_config()),
            _ => anyhow::bail!("Unknown network: {}", current_network),
        }
    }

    pub fn reset_config(&mut self) -> Result<()> {
        self.config = Config::default();
        self.save_config()
    }
}
