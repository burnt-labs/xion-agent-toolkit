use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

use super::schema::Config;

pub struct ConfigManager {
    config_dir: PathBuf,
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "burnt", "xion-toolkit")
            .context("Failed to determine config directory")?;

        let config_dir = project_dirs.config_dir().to_path_buf();

        // Ensure config directory exists
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;

        let config = Self::load_or_create_config(&config_dir)?;

        Ok(Self { config_dir, config })
    }

    fn config_file_path(&self) -> PathBuf {
        self.config_dir.join("config.json")
    }

    fn load_or_create_config(config_dir: &PathBuf) -> Result<Config> {
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

    pub fn get_value(&self, key: &str) -> Result<String> {
        let config_str =
            serde_json::to_string(&self.config).context("Failed to serialize config")?;
        let value: Value =
            serde_json::from_str(&config_str).context("Failed to parse config as JSON")?;

        let parts: Vec<&str> = key.split('.').collect();
        let mut current = &value;

        for part in &parts {
            current = current
                .get(part)
                .with_context(|| format!("Key '{}' not found in config", key))?;
        }

        Ok(current.to_string())
    }

    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        // Simple implementation for common keys
        match key {
            "network" => {
                self.config.network = value.to_string();
            }
            _ => {
                anyhow::bail!("Setting key '{}' is not supported yet", key);
            }
        }
        Ok(())
    }

    pub fn reset_config(&mut self) -> Result<()> {
        self.config = Config::default();
        self.save_config()
    }

    pub fn get_status(&self) -> Result<serde_json::Value> {
        let current_network = &self.config.network;
        let network_config = match current_network.as_str() {
            "local" => &self.config.networks.local,
            "testnet" => &self.config.networks.testnet,
            "mainnet" => &self.config.networks.mainnet,
            _ => anyhow::bail!("Unknown network: {}", current_network),
        };

        let authenticated = self.config.oauth.is_some();

        Ok(serde_json::json!({
            "network": current_network,
            "chain_id": network_config.chain_id,
            "oauth_api_url": network_config.oauth_api_url,
            "authenticated": authenticated,
            "treasury_configured": self.config.treasury.is_some()
        }))
    }

    pub fn get_network_config(&self) -> Result<&super::schema::NetworkConfig> {
        match self.config.network.as_str() {
            "local" => Ok(&self.config.networks.local),
            "testnet" => Ok(&self.config.networks.testnet),
            "mainnet" => Ok(&self.config.networks.mainnet),
            _ => anyhow::bail!("Unknown network: {}", self.config.network),
        }
    }
}
