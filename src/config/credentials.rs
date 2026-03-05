use anyhow::{Context, Result};
use directories::ProjectDirs;
use keyring::Entry;
use std::fs;
use std::path::PathBuf;

use super::schema::UserCredentials;

#[derive(Debug)]
pub struct CredentialsManager {
    network: String,
    config_dir: PathBuf,
}

impl CredentialsManager {
    pub fn new(network: &str) -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "burnt", "xion-toolkit")
            .context("Failed to determine config directory")?;

        let config_dir = project_dirs.config_dir().to_path_buf();

        // Ensure credentials directory exists
        let creds_dir = config_dir.join("credentials");
        fs::create_dir_all(&creds_dir).context("Failed to create credentials directory")?;

        Ok(Self {
            network: network.to_string(),
            config_dir,
        })
    }

    fn credentials_file_path(&self) -> PathBuf {
        self.config_dir
            .join("credentials")
            .join(format!("{}.json", self.network))
    }

    fn keyring_service_name(&self) -> String {
        format!("xion-toolkit-{}", self.network)
    }

    /// Save user credentials (tokens stored in OS keyring, metadata in file)
    pub fn save_credentials(&self, credentials: &UserCredentials) -> Result<()> {
        // Store sensitive tokens in OS keyring
        let entry = Entry::new(&self.keyring_service_name(), "access_token")
            .context("Failed to create keyring entry for access token")?;
        entry
            .set_password(&credentials.access_token)
            .context("Failed to store access token in keyring")?;

        let entry = Entry::new(&self.keyring_service_name(), "refresh_token")
            .context("Failed to create keyring entry for refresh token")?;
        entry
            .set_password(&credentials.refresh_token)
            .context("Failed to store refresh token in keyring")?;

        // Store non-sensitive metadata in file
        let metadata = serde_json::json!({
            "expires_at": credentials.expires_at,
            "xion_address": credentials.xion_address,
        });

        let metadata_str = serde_json::to_string_pretty(&metadata)
            .context("Failed to serialize credentials metadata")?;

        fs::write(self.credentials_file_path(), metadata_str)
            .context("Failed to write credentials metadata file")?;

        Ok(())
    }

    /// Load user credentials (tokens from OS keyring, metadata from file)
    pub fn load_credentials(&self) -> Result<UserCredentials> {
        // Load tokens from OS keyring
        let entry = Entry::new(&self.keyring_service_name(), "access_token")
            .context("Failed to create keyring entry for access token")?;
        let access_token = entry
            .get_password()
            .context("Failed to retrieve access token from keyring")?;

        let entry = Entry::new(&self.keyring_service_name(), "refresh_token")
            .context("Failed to create keyring entry for refresh token")?;
        let refresh_token = entry
            .get_password()
            .context("Failed to retrieve refresh token from keyring")?;

        // Load metadata from file
        let metadata_str = fs::read_to_string(self.credentials_file_path())
            .context("Failed to read credentials metadata file")?;
        let metadata: serde_json::Value =
            serde_json::from_str(&metadata_str).context("Failed to parse credentials metadata")?;

        Ok(UserCredentials {
            access_token,
            refresh_token,
            expires_at: metadata["expires_at"]
                .as_str()
                .context("Missing expires_at in credentials")?
                .to_string(),
            xion_address: metadata["xion_address"].as_str().map(|s| s.to_string()),
        })
    }

    /// Check if credentials exist for this network
    pub fn has_credentials(&self) -> Result<bool> {
        let access_token_entry = Entry::new(&self.keyring_service_name(), "access_token")
            .context("Failed to create keyring entry for access token")?;

        match access_token_entry.get_password() {
            Ok(_) => Ok(true),
            Err(keyring::Error::NoEntry) => Ok(false),
            Err(e) => Err(anyhow::anyhow!("Failed to check credentials: {}", e)),
        }
    }

    /// Clear all credentials for this network
    pub fn clear_credentials(&self) -> Result<()> {
        // Clear from keyring
        if let Ok(entry) = Entry::new(&self.keyring_service_name(), "access_token") {
            let _ = entry.delete_password();
        }

        if let Ok(entry) = Entry::new(&self.keyring_service_name(), "refresh_token") {
            let _ = entry.delete_password();
        }

        // Remove metadata file
        if self.credentials_file_path().exists() {
            fs::remove_file(self.credentials_file_path())
                .context("Failed to remove credentials metadata file")?;
        }

        Ok(())
    }

    /// Update only the access token (e.g., after refresh)
    pub fn update_access_token(&self, access_token: &str, expires_at: &str) -> Result<()> {
        // Update access token in keyring
        let entry = Entry::new(&self.keyring_service_name(), "access_token")
            .context("Failed to create keyring entry for access token")?;
        entry
            .set_password(access_token)
            .context("Failed to update access token in keyring")?;

        // Update expiration in metadata file
        if self.credentials_file_path().exists() {
            let metadata_str = fs::read_to_string(self.credentials_file_path())
                .context("Failed to read credentials metadata file")?;
            let mut metadata: serde_json::Value = serde_json::from_str(&metadata_str)
                .context("Failed to parse credentials metadata")?;

            metadata["expires_at"] = serde_json::json!(expires_at);

            let updated_str = serde_json::to_string_pretty(&metadata)
                .context("Failed to serialize updated credentials metadata")?;

            fs::write(self.credentials_file_path(), updated_str)
                .context("Failed to write updated credentials metadata file")?;
        }

        Ok(())
    }
}
