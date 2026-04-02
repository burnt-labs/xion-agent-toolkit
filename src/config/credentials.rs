//! Encrypted file-based credential storage.
//!
//! This module provides secure credential storage using AES-256-GCM encryption.
//! All credentials are stored in encrypted JSON files under `~/.xion-toolkit/credentials/`.
//!
//! # Key Derivation
//!
//! The encryption key is derived from (in order of priority):
//! 1. `XION_CI_ENCRYPTION_KEY` environment variable (for CI/CD only)
//! 2. Machine ID via `machine-uid` crate (for local development, default)
//!
//! **Note**: Local development does NOT need `XION_CI_ENCRYPTION_KEY`.
//! It is only needed in CI/CD environments where machine ID may be unstable.
//!
//! # File Format
//!
//! Credentials are stored as base64-encoded encrypted JSON:
//! ```text
//! ~/.xion-toolkit/credentials/{network}.enc
//! ```
//!
//! # CI/CD Setup
//!
//! For automated testing in CI environments, set a fixed encryption key:
//! ```bash
//! export XION_CI_ENCRYPTION_KEY=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
//! ```
//!
//! This enables headless testing without OS keychain interaction.

use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

use super::encryption::{decrypt, encrypt};
use super::schema::UserCredentials;

#[derive(Debug, Clone)]
pub struct CredentialsManager {
    network: String,
    config_dir: PathBuf,
}

impl CredentialsManager {
    /// Create a new credentials manager for the specified network.
    ///
    /// # Arguments
    ///
    /// * `network` - Network name (e.g., "testnet", "mainnet")
    ///
    /// # Returns
    ///
    /// A new `CredentialsManager` instance
    pub fn new(network: &str) -> Result<Self> {
        // Use unified ~/.xion-toolkit/ directory for all platforms
        let home_dir = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .context("Failed to determine home directory")?;

        let config_dir = PathBuf::from(home_dir).join(".xion-toolkit");

        // Ensure credentials directory exists
        let creds_dir = config_dir.join("credentials");
        fs::create_dir_all(&creds_dir).context("Failed to create credentials directory")?;

        Ok(Self {
            network: network.to_string(),
            config_dir,
        })
    }

    /// Create a credentials manager with a custom config directory (for testing).
    #[cfg(test)]
    pub fn with_config_dir(network: &str, config_dir: PathBuf) -> Result<Self> {
        let creds_dir = config_dir.join("credentials");
        fs::create_dir_all(&creds_dir).context("Failed to create credentials directory")?;

        Ok(Self {
            network: network.to_string(),
            config_dir,
        })
    }

    /// Get the path to the encrypted credentials file.
    fn credentials_file_path(&self) -> PathBuf {
        self.config_dir
            .join("credentials")
            .join(format!("{}.enc", self.network))
    }

    /// Save user credentials to encrypted file.
    ///
    /// # Arguments
    ///
    /// * `credentials` - User credentials to save
    ///
    /// # Security
    ///
    /// Credentials are encrypted with AES-256-GCM before being written to disk.
    /// The encryption key is derived from environment variable or machine ID.
    pub fn save_credentials(&self, credentials: &UserCredentials) -> Result<()> {
        // Serialize credentials to JSON
        let json = serde_json::to_string(credentials)
            .context("Failed to serialize credentials to JSON")?;

        // Encrypt the JSON
        let encrypted = encrypt(json.as_bytes()).context("Failed to encrypt credentials")?;

        // Write to file
        fs::write(self.credentials_file_path(), encrypted)
            .context("Failed to write encrypted credentials file")?;

        Ok(())
    }

    /// Load user credentials from encrypted file.
    ///
    /// # Returns
    ///
    /// The decrypted user credentials
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Credentials file doesn't exist
    /// - Decryption fails (wrong key or corrupted data)
    /// - JSON deserialization fails
    pub fn load_credentials(&self) -> Result<UserCredentials> {
        let path = self.credentials_file_path();

        // Check if file exists
        if !path.exists() {
            return Err(anyhow::anyhow!(
                "No credentials found for network '{}'. Run 'xion auth login' first.",
                self.network
            ));
        }

        // Read encrypted data
        let encrypted =
            fs::read_to_string(&path).context("Failed to read encrypted credentials file")?;

        // Decrypt
        let decrypted = decrypt(&encrypted).context("Failed to decrypt credentials")?;

        // Deserialize JSON
        let json_str =
            String::from_utf8(decrypted).context("Decrypted credentials are not valid UTF-8")?;

        let credentials: UserCredentials =
            serde_json::from_str(&json_str).context("Failed to parse credentials JSON")?;

        Ok(credentials)
    }

    /// Check if credentials exist for this network.
    ///
    /// # Returns
    ///
    /// `true` if encrypted credentials file exists, `false` otherwise
    pub fn has_credentials(&self) -> Result<bool> {
        Ok(self.credentials_file_path().exists())
    }

    /// Clear all credentials for this network.
    ///
    /// Removes the encrypted credentials file if it exists.
    pub fn clear_credentials(&self) -> Result<()> {
        let path = self.credentials_file_path();
        if path.exists() {
            fs::remove_file(&path).context("Failed to remove credentials file")?;
        }
        Ok(())
    }

    /// Update only the access token (e.g., after refresh).
    ///
    /// This loads the existing credentials, updates the access token and expiration,
    /// then saves them back.
    #[allow(dead_code)]
    pub fn update_access_token(&self, access_token: &str, expires_at: &str) -> Result<()> {
        // Load existing credentials
        let mut credentials = self
            .load_credentials()
            .context("Failed to load existing credentials for update")?;

        // Update fields
        credentials.access_token = access_token.to_string();
        credentials.expires_at = expires_at.to_string();

        // Save back
        self.save_credentials(&credentials)
            .context("Failed to save updated credentials")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::encryption;
    use base64::Engine;
    use serial_test::serial;
    use tempfile::tempdir;

    /// Helper to set up test environment with isolated directory and encryption key
    fn setup_isolated_test_env() -> (tempfile::TempDir, String) {
        // Create temp directory
        let temp_dir = tempdir().expect("Failed to create temp dir");

        // Generate a unique test key
        let test_key = encryption::generate_test_key();

        (temp_dir, test_key)
    }

    /// Helper to set up encryption key
    #[allow(dead_code)]
    fn setup_test_key() -> Option<String> {
        let original = env::var(encryption::ENV_KEY_NAME).ok();
        let test_key = encryption::generate_test_key();
        env::set_var(encryption::ENV_KEY_NAME, &test_key);
        original
    }

    /// Helper to restore original key
    fn restore_key(original: Option<String>) {
        if let Some(key) = original {
            env::set_var(encryption::ENV_KEY_NAME, key);
        } else {
            env::remove_var(encryption::ENV_KEY_NAME);
        }
    }

    #[test]
    #[serial(encryption_key)]
    fn test_save_and_load_credentials() {
        let (temp_dir, test_key) = setup_isolated_test_env();
        let original_key = env::var(encryption::ENV_KEY_NAME).ok();
        env::set_var(encryption::ENV_KEY_NAME, &test_key);

        let manager = CredentialsManager::with_config_dir("testnet", temp_dir.path().to_path_buf())
            .expect("Failed to create manager");

        let credentials = UserCredentials {
            access_token: "test_access_token".to_string(),
            refresh_token: "test_refresh_token".to_string(),
            expires_at: "2024-12-31T23:59:59Z".to_string(),
            refresh_token_expires_at: Some("2025-01-31T23:59:59Z".to_string()),
            xion_address: Some("xion1test".to_string()),
            scope: None,
        };

        // Save
        manager
            .save_credentials(&credentials)
            .expect("Failed to save");

        // Load
        let loaded = manager.load_credentials().expect("Failed to load");

        assert_eq!(credentials.access_token, loaded.access_token);
        assert_eq!(credentials.refresh_token, loaded.refresh_token);
        assert_eq!(credentials.expires_at, loaded.expires_at);
        assert_eq!(credentials.xion_address, loaded.xion_address);

        restore_key(original_key);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_has_credentials() {
        let (temp_dir, test_key) = setup_isolated_test_env();
        let original_key = env::var(encryption::ENV_KEY_NAME).ok();
        env::set_var(encryption::ENV_KEY_NAME, &test_key);

        let manager = CredentialsManager::with_config_dir("testnet", temp_dir.path().to_path_buf())
            .expect("Failed to create manager");

        // No credentials initially
        assert!(!manager.has_credentials().expect("has_credentials failed"));

        // Save credentials
        let credentials = UserCredentials {
            access_token: "token".to_string(),
            refresh_token: "refresh".to_string(),
            expires_at: "2024-12-31T23:59:59Z".to_string(),
            refresh_token_expires_at: None,
            xion_address: None,
            scope: None,
        };
        manager
            .save_credentials(&credentials)
            .expect("Failed to save");

        // Now has credentials
        assert!(manager.has_credentials().expect("has_credentials failed"));

        restore_key(original_key);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_clear_credentials() {
        let (temp_dir, test_key) = setup_isolated_test_env();
        let original_key = env::var(encryption::ENV_KEY_NAME).ok();
        env::set_var(encryption::ENV_KEY_NAME, &test_key);

        let manager = CredentialsManager::with_config_dir("testnet", temp_dir.path().to_path_buf())
            .expect("Failed to create manager");

        // Save credentials
        let credentials = UserCredentials {
            access_token: "token".to_string(),
            refresh_token: "refresh".to_string(),
            expires_at: "2024-12-31T23:59:59Z".to_string(),
            refresh_token_expires_at: None,
            xion_address: None,
            scope: None,
        };
        manager
            .save_credentials(&credentials)
            .expect("Failed to save");
        assert!(manager.has_credentials().expect("has_credentials failed"));

        // Clear
        manager.clear_credentials().expect("Failed to clear");
        assert!(!manager.has_credentials().expect("has_credentials failed"));

        restore_key(original_key);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_update_access_token() {
        let (temp_dir, test_key) = setup_isolated_test_env();
        let original_key = env::var(encryption::ENV_KEY_NAME).ok();
        env::set_var(encryption::ENV_KEY_NAME, &test_key);

        let manager = CredentialsManager::with_config_dir("testnet", temp_dir.path().to_path_buf())
            .expect("Failed to create manager");

        // Save initial credentials
        let credentials = UserCredentials {
            access_token: "old_token".to_string(),
            refresh_token: "refresh".to_string(),
            expires_at: "2024-01-01T00:00:00Z".to_string(),
            refresh_token_expires_at: Some("2025-01-01T00:00:00Z".to_string()),
            xion_address: Some("xion1test".to_string()),
            scope: None,
        };
        manager
            .save_credentials(&credentials)
            .expect("Failed to save");

        // Update access token
        manager
            .update_access_token("new_token", "2024-12-31T23:59:59Z")
            .expect("Failed to update");

        // Load and verify
        let loaded = manager.load_credentials().expect("Failed to load");
        assert_eq!(loaded.access_token, "new_token");
        assert_eq!(loaded.expires_at, "2024-12-31T23:59:59Z");
        assert_eq!(loaded.refresh_token, "refresh"); // Unchanged
        assert_eq!(loaded.xion_address, Some("xion1test".to_string())); // Unchanged

        restore_key(original_key);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_different_networks_isolated() {
        let (temp_dir, test_key) = setup_isolated_test_env();
        let original_key = env::var(encryption::ENV_KEY_NAME).ok();
        env::set_var(encryption::ENV_KEY_NAME, &test_key);

        let manager_testnet =
            CredentialsManager::with_config_dir("testnet", temp_dir.path().to_path_buf())
                .expect("Failed to create manager");
        let manager_mainnet =
            CredentialsManager::with_config_dir("mainnet", temp_dir.path().to_path_buf())
                .expect("Failed to create manager");

        let testnet_creds = UserCredentials {
            access_token: "testnet_token".to_string(),
            refresh_token: "testnet_refresh".to_string(),
            expires_at: "2024-01-01T00:00:00Z".to_string(),
            refresh_token_expires_at: Some("2025-01-01T00:00:00Z".to_string()),
            xion_address: Some("xion1testnet".to_string()),
            scope: None,
        };

        let mainnet_creds = UserCredentials {
            access_token: "mainnet_token".to_string(),
            refresh_token: "mainnet_refresh".to_string(),
            expires_at: "2024-01-01T00:00:00Z".to_string(),
            refresh_token_expires_at: Some("2025-01-01T00:00:00Z".to_string()),
            xion_address: Some("xion1mainnet".to_string()),
            scope: None,
        };

        manager_testnet
            .save_credentials(&testnet_creds)
            .expect("Failed to save testnet");
        manager_mainnet
            .save_credentials(&mainnet_creds)
            .expect("Failed to save mainnet");

        let loaded_testnet = manager_testnet
            .load_credentials()
            .expect("Failed to load testnet");
        let loaded_mainnet = manager_mainnet
            .load_credentials()
            .expect("Failed to load mainnet");

        assert_eq!(loaded_testnet.access_token, "testnet_token");
        assert_eq!(loaded_mainnet.access_token, "mainnet_token");

        restore_key(original_key);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_load_nonexistent_credentials() {
        let (temp_dir, test_key) = setup_isolated_test_env();
        let original_key = env::var(encryption::ENV_KEY_NAME).ok();
        env::set_var(encryption::ENV_KEY_NAME, &test_key);

        let manager =
            CredentialsManager::with_config_dir("nonexistent", temp_dir.path().to_path_buf())
                .expect("Failed to create manager");

        let result = manager.load_credentials();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No credentials found"));

        restore_key(original_key);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_credentials_encrypted_on_disk() {
        let (temp_dir, test_key) = setup_isolated_test_env();
        let original_key = env::var(encryption::ENV_KEY_NAME).ok();
        env::set_var(encryption::ENV_KEY_NAME, &test_key);

        let manager = CredentialsManager::with_config_dir("testnet", temp_dir.path().to_path_buf())
            .expect("Failed to create manager");

        let credentials = UserCredentials {
            access_token: "secret_token_12345".to_string(),
            refresh_token: "secret_refresh_67890".to_string(),
            expires_at: "2024-12-31T23:59:59Z".to_string(),
            refresh_token_expires_at: Some("2025-01-31T23:59:59Z".to_string()),
            xion_address: Some("xion1secret".to_string()),
            scope: None,
        };

        manager
            .save_credentials(&credentials)
            .expect("Failed to save");

        // Read the raw file content
        let file_path = temp_dir.path().join("credentials").join("testnet.enc");
        let raw_content = fs::read_to_string(&file_path).expect("Failed to read file");

        // Verify the content is encrypted (not plain text)
        assert!(!raw_content.contains("secret_token_12345"));
        assert!(!raw_content.contains("secret_refresh_67890"));
        assert!(!raw_content.contains("xion1secret"));

        // Verify it's valid base64
        assert!(base64::engine::general_purpose::STANDARD
            .decode(&raw_content)
            .is_ok());

        restore_key(original_key);
    }
}
