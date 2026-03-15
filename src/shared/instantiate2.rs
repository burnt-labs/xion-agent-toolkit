//! Instantiate2 Address Computation
//!
//! This module provides shared functionality for computing deterministic contract
//! addresses using the CosmWasm instantiate2 pattern.
//!
//! ## Overview
//!
//! The instantiate2 pattern allows you to predict a contract's address before
//! deployment. This is useful for:
//!
//! - Pre-configuring grants and permissions before deployment
//! - Setting up monitoring before deployment
//! - Integration with other contracts that reference the treasury
//!
//! ## Usage
//!
//! ```rust,ignore
//! use xion_agent_toolkit::shared::instantiate2::{compute_address, validate_salt, SaltEncoding};
//!
//! // Validate and parse salt
//! let salt_bytes = validate_salt("my-treasury-v1", SaltEncoding::Utf8)?;
//!
//! // Compute predicted address (requires checksum from chain)
//! let predicted = compute_address(
//!     "xion1...",      // creator address
//!     1260,            // code ID
//!     &salt_bytes,
//!     &checksum_bytes, // from get_code_info
//! )?;
//! ```

use anyhow::{anyhow, Result};
use bech32::{decode, encode, Bech32, Hrp};
use cosmwasm_std::{instantiate2_address, CanonicalAddr, HexBinary};

/// Salt encoding format for parsing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaltEncoding {
    /// UTF-8 string encoding (e.g., "my-treasury-v1")
    Utf8,
    /// Hexadecimal encoding (e.g., "6d792d73616c74")
    Hex,
}

/// Result of address prediction
#[derive(Debug, Clone)]
pub struct PredictedAddress {
    /// The predicted contract address
    pub address: String,
    /// The code ID used for prediction
    pub code_id: u64,
    /// The salt used (hex-encoded)
    pub salt_hex: String,
    /// The checksum of the code (hex-encoded)
    pub checksum: String,
    /// The creator address
    pub creator: String,
}

/// Validate and parse a salt string
///
/// # Arguments
/// * `salt` - The salt string to validate
/// * `encoding` - The encoding format (Utf8 or Hex)
///
/// # Returns
/// The parsed salt bytes
///
/// # Errors
/// - If the salt is empty
/// - If hex decoding fails (for Hex encoding)
/// - If the resulting salt exceeds 64 bytes (CosmWasm limit)
pub fn validate_salt(salt: &str, encoding: SaltEncoding) -> Result<Vec<u8>> {
    if salt.is_empty() {
        return Err(anyhow!("Salt cannot be empty"));
    }

    let salt_bytes = match encoding {
        SaltEncoding::Utf8 => salt.as_bytes().to_vec(),
        SaltEncoding::Hex => hex::decode(salt).map_err(|e| {
            anyhow!(
                "Invalid hex-encoded salt: {}. Expected valid hex string. Error: {}",
                salt,
                e
            )
        })?,
    };

    // CosmWasm instantiate2 has a 64-byte limit for salt
    if salt_bytes.len() > 64 {
        return Err(anyhow!(
            "Salt too long: {} bytes. Maximum is 64 bytes. Consider using a shorter string or hash your salt.",
            salt_bytes.len()
        ));
    }

    Ok(salt_bytes)
}

/// Detect the encoding of a salt string
///
/// If the string looks like valid hex (even length, all hex chars), returns Hex encoding.
/// Otherwise, returns Utf8 encoding.
pub fn detect_salt_encoding(salt: &str) -> SaltEncoding {
    // Check if it looks like hex: even length, all hex characters
    if salt.len().is_multiple_of(2)
        && salt.chars().all(|c| c.is_ascii_hexdigit())
        && salt.len() >= 2
    {
        SaltEncoding::Hex
    } else {
        SaltEncoding::Utf8
    }
}

/// Compute the deterministic contract address for instantiate2
///
/// Uses the CosmWasm instantiate2 algorithm to compute the deterministic
/// contract address before broadcasting the transaction.
///
/// # Arguments
/// * `creator` - The creator's bech32 address (e.g., "xion1...")
/// * `_code_id` - The code ID being instantiated (for reference/debugging)
/// * `salt` - The salt bytes used for instantiate2
/// * `checksum` - The checksum bytes of the code (from get_code_info)
///
/// # Returns
/// The predicted contract address as a bech32 string
///
/// # Errors
/// - If the creator address is invalid
/// - If the checksum is invalid
/// - If the instantiate2 computation fails
pub fn compute_address(
    creator: &str,
    _code_id: u64,
    salt: &[u8],
    checksum: &[u8],
) -> Result<String> {
    // Validate creator address
    if !creator.starts_with("xion1") && !creator.starts_with("cosmos1") {
        return Err(anyhow!(
            "Invalid creator address format: {}. Expected bech32 address starting with 'xion1' or 'cosmos1'",
            creator
        ));
    }

    // Convert creator address to canonical format
    let canonical_creator = decode_bech32_address(creator)?;

    // Compute instantiate2 address using cosmwasm_std
    let canonical_addr = instantiate2_address(checksum, &canonical_creator, salt)
        .map_err(|e| anyhow!("Failed to compute instantiate2 address: {}", e))?;

    // Convert canonical address back to bech32
    let bech32_prefix = extract_bech32_prefix(creator)?;
    let predicted_address = encode_canonical_address(&canonical_addr, &bech32_prefix)?;

    Ok(predicted_address)
}

/// Compute a predicted treasury address
///
/// Convenience function that combines salt parsing and address computation.
/// Auto-detects salt encoding (UTF-8 or hex).
///
/// # Arguments
/// * `creator` - The creator's bech32 address
/// * `code_id` - The treasury code ID (1260 for testnet, 63 for mainnet)
/// * `salt_str` - The salt string (auto-detected as UTF-8 or hex)
/// * `checksum` - The checksum bytes of the treasury code
///
/// # Returns
/// A `PredictedAddress` struct with all prediction details
pub fn predict_treasury_address(
    creator: &str,
    code_id: u64,
    salt_str: &str,
    checksum: &[u8],
) -> Result<PredictedAddress> {
    // Auto-detect encoding and validate salt
    let encoding = detect_salt_encoding(salt_str);
    let salt_bytes = validate_salt(salt_str, encoding)?;

    // Compute address
    let address = compute_address(creator, code_id, &salt_bytes, checksum)?;

    Ok(PredictedAddress {
        address,
        code_id,
        salt_hex: hex::encode(&salt_bytes),
        checksum: hex::encode(checksum),
        creator: creator.to_string(),
    })
}

// ============================================================================
// Bech32 Address Utilities
// ============================================================================

/// Decode a bech32 address to its canonical (binary) form
///
/// # Arguments
/// * `address` - Bech32 encoded address (e.g., "xion1abc...")
///
/// # Returns
/// Canonical address as `CanonicalAddr`
fn decode_bech32_address(address: &str) -> Result<CanonicalAddr> {
    let (_hrp, data) =
        decode(address).map_err(|e| anyhow!("Bech32 decode error for '{}': {}", address, e))?;

    Ok(CanonicalAddr::from(HexBinary::from(data)))
}

/// Encode a canonical address to bech32 format
///
/// # Arguments
/// * `canonical` - Canonical address bytes
/// * `prefix` - Bech32 prefix (e.g., "xion")
///
/// # Returns
/// Bech32 encoded address string
fn encode_canonical_address(canonical: &CanonicalAddr, prefix: &str) -> Result<String> {
    let hrp =
        Hrp::parse(prefix).map_err(|e| anyhow!("Invalid bech32 prefix '{}': {}", prefix, e))?;

    let encoded = encode::<Bech32>(hrp, canonical.as_slice())
        .map_err(|e| anyhow!("Bech32 encode error: {}", e))?;

    Ok(encoded)
}

/// Extract the bech32 prefix from an address
///
/// # Arguments
/// * `address` - Bech32 encoded address (e.g., "xion1abc...")
///
/// # Returns
/// The prefix part (e.g., "xion")
fn extract_bech32_prefix(address: &str) -> Result<String> {
    let separator_pos = address.find('1').ok_or_else(|| {
        anyhow!(
            "Invalid bech32 address format: missing separator in '{}'",
            address
        )
    })?;

    Ok(address[..separator_pos].to_string())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_salt_utf8() {
        let result = validate_salt("my-treasury-v1", SaltEncoding::Utf8);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"my-treasury-v1");
    }

    #[test]
    fn test_validate_salt_hex() {
        let result = validate_salt("6d792d73616c74", SaltEncoding::Hex);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"my-salt");
    }

    #[test]
    fn test_validate_salt_empty() {
        let result = validate_salt("", SaltEncoding::Utf8);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_validate_salt_too_long() {
        // Create a 65-byte salt (exceeds 64-byte limit)
        let long_salt = "a".repeat(65);
        let result = validate_salt(&long_salt, SaltEncoding::Utf8);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_validate_salt_invalid_hex() {
        let result = validate_salt("not-valid-hex!", SaltEncoding::Hex);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid hex"));
    }

    #[test]
    fn test_detect_salt_encoding_hex() {
        assert_eq!(detect_salt_encoding("6d792d73616c74"), SaltEncoding::Hex);
        assert_eq!(detect_salt_encoding("abcd"), SaltEncoding::Hex);
        assert_eq!(detect_salt_encoding("1234"), SaltEncoding::Hex);
    }

    #[test]
    fn test_detect_salt_encoding_utf8() {
        assert_eq!(detect_salt_encoding("my-treasury-v1"), SaltEncoding::Utf8);
        assert_eq!(detect_salt_encoding("hello world"), SaltEncoding::Utf8);
        // Odd length can't be hex
        assert_eq!(detect_salt_encoding("abc"), SaltEncoding::Utf8);
    }

    #[test]
    fn test_compute_address() {
        // Use known values for deterministic test
        let checksum_hex = "e13aa30e0d70ea895b294ad1bc809950e60fe081b322b1657f75b67be6021b1c";
        let checksum_bytes = hex::decode(checksum_hex).unwrap();

        let sender_address = "xion1q9lqzpc73fewqva98pwaqvezaf9vqqulw3hmmx";
        let salt = b"test-salt-32-bytes-aaaaaaaaaa"; // 28 bytes

        let result = compute_address(sender_address, 522, salt, &checksum_bytes);
        assert!(result.is_ok());

        let predicted = result.unwrap();
        assert!(predicted.starts_with("xion1"));
        // Same inputs should produce same output
        let result2 = compute_address(sender_address, 522, salt, &checksum_bytes);
        assert_eq!(predicted, result2.unwrap());
    }

    #[test]
    fn test_compute_address_invalid_creator() {
        let checksum_bytes = vec![0u8; 32];
        let result = compute_address("invalid-address", 1260, b"salt", &checksum_bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid creator address"));
    }

    #[test]
    fn test_decode_encode_bech32_address() {
        let address = "xion1q9lqzpc73fewqva98pwaqvezaf9vqqulw3hmmx";

        let canonical = decode_bech32_address(address).unwrap();
        assert!(!canonical.is_empty());

        let encoded = encode_canonical_address(&canonical, "xion").unwrap();
        assert_eq!(encoded, address);
    }

    #[test]
    fn test_extract_bech32_prefix() {
        assert_eq!(
            extract_bech32_prefix("xion1q9lqzpc73fewqva98pwaqvezaf9vqqulw3hmmx").unwrap(),
            "xion"
        );
        assert_eq!(extract_bech32_prefix("cosmos1abc123").unwrap(), "cosmos");
    }

    #[test]
    fn test_extract_bech32_prefix_invalid() {
        let result = extract_bech32_prefix("invalidaddress");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing separator"));
    }

    #[test]
    fn test_predict_treasury_address() {
        let checksum_hex = "e13aa30e0d70ea895b294ad1bc809950e60fe081b322b1657f75b67be6021b1c";
        let checksum_bytes = hex::decode(checksum_hex).unwrap();

        let creator = "xion1q9lqzpc73fewqva98pwaqvezaf9vqqulw3hmmx";
        let salt_str = "my-treasury-test";

        let result = predict_treasury_address(creator, 1260, salt_str, &checksum_bytes);
        assert!(result.is_ok());

        let predicted = result.unwrap();
        assert!(predicted.address.starts_with("xion1"));
        assert_eq!(predicted.code_id, 1260);
        assert_eq!(predicted.creator, creator);
        assert_eq!(predicted.checksum, checksum_hex);
    }

    #[test]
    fn test_predict_treasury_address_hex_salt() {
        let checksum_bytes = vec![0u8; 32];
        let creator = "xion1q9lqzpc73fewqva98pwaqvezaf9vqqulw3hmmx";

        // Use hex-encoded salt
        let salt_hex = "6d792d73616c74"; // "my-salt" in hex
        let result = predict_treasury_address(creator, 1260, salt_hex, &checksum_bytes);
        assert!(result.is_ok());

        let predicted = result.unwrap();
        // Should detect hex encoding and use hex::decode
        assert_eq!(predicted.salt_hex, salt_hex);
    }

    #[test]
    fn test_instantiate2_address_deterministic() {
        // Test that same inputs always produce the same address
        let checksum_hex = "e13aa30e0d70ea895b294ad1bc809950e60fe081b322b1657f75b67be6021b1c";
        let checksum_bytes = hex::decode(checksum_hex).unwrap();

        let sender = "xion1q9lqzpc73fewqva98pwaqvezaf9vqqulw3hmmx";
        let salt = b"deterministic-test-salt";

        let addr1 = compute_address(sender, 522, salt, &checksum_bytes).unwrap();
        let addr2 = compute_address(sender, 522, salt, &checksum_bytes).unwrap();

        assert_eq!(addr1, addr2, "Same inputs should produce same address");
    }
}
