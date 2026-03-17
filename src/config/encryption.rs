//! AES-GCM encryption for credential storage.
//!
//! This module provides encrypted file-based credential storage that works
//! across all platforms without requiring OS keychain interaction.
//!
//! # Key Derivation Strategy
//!
//! The encryption key is derived from (in order of priority):
//! 1. `XION_CI_ENCRYPTION_KEY` environment variable (for CI/CD only)
//! 2. Machine ID via `machine-uid` crate (for local development, default)
//!
//! **Note**: Local development does NOT need `XION_CI_ENCRYPTION_KEY`.
//! The machine ID derivation is the default and recommended for local use.
//!
//! # Security Model
//!
//! - Uses AES-256-GCM for authenticated encryption
//! - Random 96-bit nonce for each encryption operation
//! - Key is never stored on disk
//!
//! # CI/CD Usage
//!
//! For automated testing in CI/CD environments, set `XION_CI_ENCRYPTION_KEY` to a fixed
//! 32-byte hex string. This is only needed in CI/CD where machine ID may be unstable.
//!
//! ```bash
//! export XION_CI_ENCRYPTION_KEY=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
//! cargo test
//! ```

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use std::env;

/// Environment variable name for CI/CD encryption key
pub const ENV_KEY_NAME: &str = "XION_CI_ENCRYPTION_KEY";

/// Key length in bytes (AES-256)
const KEY_LEN: usize = 32;

/// Nonce length in bytes (96-bit for GCM)
const NONCE_LEN: usize = 12;

/// Get or derive the encryption key.
///
/// Priority:
/// 1. `XION_CI_ENCRYPTION_KEY` environment variable (hex-encoded 32 bytes)
/// 2. Machine ID derivation
pub fn get_encryption_key() -> Result<[u8; KEY_LEN]> {
    // Try environment variable first (for CI/CD)
    if let Ok(key_hex) = env::var(ENV_KEY_NAME) {
        let key_bytes = hex::decode(&key_hex)
            .with_context(|| format!("Failed to decode {} as hex", ENV_KEY_NAME))?;

        if key_bytes.len() != KEY_LEN {
            return Err(anyhow!(
                "{} must be exactly {} bytes ({} hex characters), got {} bytes",
                ENV_KEY_NAME,
                KEY_LEN,
                KEY_LEN * 2,
                key_bytes.len()
            ));
        }

        let mut key = [0u8; KEY_LEN];
        key.copy_from_slice(&key_bytes);
        return Ok(key);
    }

    // Fallback to machine ID derivation
    derive_key_from_machine_id()
}

/// Derive encryption key from machine ID.
///
/// Uses a simple hash-based derivation. The machine ID is combined with
/// a domain separator and hashed to produce a 32-byte key.
fn derive_key_from_machine_id() -> Result<[u8; KEY_LEN]> {
    let machine_id = machine_uid::get().map_err(|e| {
        anyhow::anyhow!(
            "Failed to get machine ID: {}. Set XION_CI_ENCRYPTION_KEY environment variable for CI/CD",
            e
        )
    })?;

    // Use SHA-256 to derive key from machine ID + domain separator
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"xion-toolkit-credentials-v1:");
    hasher.update(machine_id.as_bytes());
    let hash = hasher.finalize();

    let mut key = [0u8; KEY_LEN];
    key.copy_from_slice(&hash);
    Ok(key)
}

/// Encrypt data using AES-256-GCM.
///
/// Returns base64-encoded ciphertext with prepended nonce.
/// Format: base64(nonce || ciphertext || tag)
pub fn encrypt(plaintext: &[u8]) -> Result<String> {
    let key = get_encryption_key()?;
    let cipher = Aes256Gcm::new_from_slice(&key).context("Failed to create cipher from key")?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| anyhow!("Encryption failed: {}", e))?;

    // Prepend nonce to ciphertext
    let mut result = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(&result))
}

/// Decrypt data encrypted with `encrypt`.
///
/// Expects base64-encoded ciphertext with prepended nonce.
pub fn decrypt(ciphertext_b64: &str) -> Result<Vec<u8>> {
    let key = get_encryption_key()?;
    let cipher = Aes256Gcm::new_from_slice(&key).context("Failed to create cipher from key")?;

    // Decode base64
    let data = BASE64
        .decode(ciphertext_b64)
        .context("Failed to decode base64 ciphertext")?;

    if data.len() < NONCE_LEN {
        return Err(anyhow!("Ciphertext too short"));
    }

    // Extract nonce and ciphertext
    let nonce = Nonce::from_slice(&data[..NONCE_LEN]);
    let ciphertext = &data[NONCE_LEN..];

    // Decrypt
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("Decryption failed: {}", e))?;

    Ok(plaintext)
}

/// Generate a random key for testing purposes.
///
/// Returns a 32-byte hex-encoded string suitable for XION_CI_ENCRYPTION_KEY.
#[allow(dead_code)]
pub fn generate_test_key() -> String {
    let mut key = [0u8; KEY_LEN];
    rand::thread_rng().fill_bytes(&mut key);
    hex::encode(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    /// Helper to set up encryption key for tests
    fn setup_test_key() -> Option<String> {
        let original = env::var(ENV_KEY_NAME).ok();
        let test_key = generate_test_key();
        env::set_var(ENV_KEY_NAME, &test_key);
        original
    }

    /// Helper to restore original key
    fn restore_key(original: Option<String>) {
        if let Some(key) = original {
            env::set_var(ENV_KEY_NAME, key);
        } else {
            env::remove_var(ENV_KEY_NAME);
        }
    }

    #[test]
    #[serial(encryption_key)]
    fn test_encrypt_decrypt_roundtrip() {
        let original = setup_test_key();

        let plaintext = b"hello, world!";
        let encrypted = encrypt(plaintext).expect("Encryption failed");
        let decrypted = decrypt(&encrypted).expect("Decryption failed");
        assert_eq!(plaintext.as_slice(), decrypted.as_slice());

        restore_key(original);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_encrypt_produces_different_ciphertext() {
        let original = setup_test_key();

        let plaintext = b"hello, world!";
        let encrypted1 = encrypt(plaintext).expect("Encryption failed");
        let encrypted2 = encrypt(plaintext).expect("Encryption failed");
        // Different due to random nonce
        assert_ne!(encrypted1, encrypted2);

        restore_key(original);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_decrypt_wrong_key_fails() {
        let original_key = env::var(ENV_KEY_NAME).ok();

        env::set_var(
            ENV_KEY_NAME,
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        );
        let plaintext = b"secret data";
        let encrypted = encrypt(plaintext).expect("Encryption failed");

        // Change key
        env::set_var(
            ENV_KEY_NAME,
            "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
        );
        let result = decrypt(&encrypted);
        assert!(result.is_err(), "Decryption with wrong key should fail");

        restore_key(original_key);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_env_key_validation() {
        let original_key = env::var(ENV_KEY_NAME).ok();

        // Too short
        env::set_var(ENV_KEY_NAME, "0123456789abcdef");
        assert!(get_encryption_key().is_err());

        // Invalid hex
        env::set_var(
            ENV_KEY_NAME,
            "not-valid-hex-string-!!!!!-not-valid-hex-string!!",
        );
        assert!(get_encryption_key().is_err());

        // Correct length
        env::set_var(
            ENV_KEY_NAME,
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        );
        assert!(get_encryption_key().is_ok());

        // Restore
        restore_key(original_key);
    }

    #[test]
    fn test_generate_test_key() {
        let key1 = generate_test_key();
        let key2 = generate_test_key();
        assert_eq!(key1.len(), 64); // 32 bytes = 64 hex chars
        assert_ne!(key1, key2); // Random keys should differ
    }

    #[test]
    #[serial(encryption_key)]
    fn test_decrypt_malformed_data() {
        let original = setup_test_key();

        // Invalid base64
        let result = decrypt("not-valid-base64!!!");
        assert!(result.is_err());

        // Too short (less than nonce length)
        let short_data = BASE64.encode(b"short");
        let result = decrypt(&short_data);
        assert!(result.is_err());

        restore_key(original);
    }
}
