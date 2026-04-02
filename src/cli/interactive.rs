//! Interactive parameter prompting for missing required arguments.
//!
//! When running in a TTY without `--no-interactive`, missing required
//! parameters are collected via user prompts instead of causing an error exit.

use std::io::{self, IsTerminal};
use std::path::PathBuf;

use dialoguer::Input;

/// Check if interactive mode is available (TTY on both stdin and stdout).
pub(crate) fn is_tty() -> bool {
    io::stdin().is_terminal() && io::stdout().is_terminal()
}

/// Result of interactive prompting.
pub(crate) type PromptResult<T> = Result<T, PromptError>;

#[derive(Debug)]
pub(crate) enum PromptError {
    /// User cancelled (Ctrl+C)
    Cancelled,
    /// User entered empty input when non-empty was required
    EmptyInput,
    /// Input failed validation
    ValidationFailed(String),
}

impl std::fmt::Display for PromptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptError::Cancelled => write!(f, "Prompt cancelled by user"),
            PromptError::EmptyInput => write!(f, "Empty input is not allowed"),
            PromptError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
        }
    }
}

impl std::error::Error for PromptError {}

// ============================================================================
// Prompt Builders
// ============================================================================

/// Prompt for a non-empty text string.
pub(crate) fn prompt_text(label: &str) -> PromptResult<String> {
    let input: String = Input::<String>::new()
        .with_prompt(label)
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("Input cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(input.trim().to_string())
}

/// Prompt for a u64 number. Re-prompts on invalid input.
pub(crate) fn prompt_u64(label: &str) -> PromptResult<u64> {
    let value: u64 = Input::<u64>::new()
        .with_prompt(label)
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(value)
}

/// Validates an Xion address string using bech32 decoding.
///
/// Returns `Ok(())` if the address is a valid xion1... bech32 address,
/// or an error message describing the validation failure.
pub(crate) fn validate_address(value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("Address cannot be empty".to_string());
    }
    bech32::decode(trimmed)
        .map_err(|_| "Invalid Xion address (bech32 decode failed)".to_string())
        .and_then(|(hrp, _data)| {
            if hrp.as_str() == "xion" {
                Ok(())
            } else {
                Err(format!(
                    "Invalid Xion address (expected prefix 'xion', got '{}')",
                    hrp.as_str()
                ))
            }
        })
}

/// Prompt for a blockchain address (xion1... format).
///
/// Validates the input using `bech32::decode()` to ensure the checksum
/// is correct and the human-readable prefix is `"xion"`.
pub(crate) fn prompt_address(label: &str) -> PromptResult<String> {
    let input: String = Input::<String>::new()
        .with_prompt(label)
        .validate_with(|input: &String| -> Result<(), &str> {
            match validate_address(input) {
                Ok(()) => Ok(()),
                Err(ref e) if e.contains("bech32 decode failed") => {
                    Err("Invalid Xion address (bech32 decode failed)")
                }
                Err(ref e) if e.contains("expected prefix") => {
                    Err("Invalid Xion address (expected prefix 'xion', got '...')")
                }
                Err(_) => Err("Invalid Xion address"),
            }
        })
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(input.trim().to_string())
}

/// Validates an amount string (e.g., "1000000uxion" or "1000000").
///
/// Returns `Ok(())` if the amount is valid (digits only or digits + "uxion" suffix),
/// or an error message describing the validation failure.
pub(crate) fn validate_amount(value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("Amount cannot be empty".to_string());
    }
    if !trimmed
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        return Err("Amount must start with a number".to_string());
    }
    // Accept: "1000000uxion" (digits + denom) or "1000000" (pure digits)
    if !trimmed.ends_with("uxion") {
        if !trimmed.chars().all(|c| c.is_ascii_digit()) {
            return Err(
                "Amount must be a number (e.g., 1000000) or with denom (e.g., 1000000uxion)"
                    .to_string(),
            );
        }
    } else {
        let prefix = &trimmed[..trimmed.len() - 5];
        if prefix.is_empty() || !prefix.chars().all(|c| c.is_ascii_digit()) {
            return Err(
                "Amount must be a number (e.g., 1000000) or with denom (e.g., 1000000uxion)"
                    .to_string(),
            );
        }
    }
    Ok(())
}

/// Prompt for an amount string (e.g., "1000000uxion" or "1000000").
/// If the user enters a plain number without a denomination, "uxion" is appended automatically.
pub(crate) fn prompt_amount(label: &str) -> PromptResult<String> {
    let input: String = Input::<String>::new()
        .with_prompt(label)
        .validate_with(|input: &String| -> Result<(), &str> {
            if validate_amount(input).is_err() {
                Err("Amount must be a number (e.g., 1000000) or with denom (e.g., 1000000uxion)")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    let trimmed = input.trim().to_string();
    // Auto-append "uxion" denomination if user entered a plain number
    if !trimmed.ends_with("uxion") {
        Ok(format!("{}uxion", trimmed))
    } else {
        Ok(trimmed)
    }
}

/// Prompt for a file path. Does NOT validate existence (caller decides).
pub(crate) fn prompt_path(label: &str) -> PromptResult<PathBuf> {
    let input: String = Input::new()
        .with_prompt(label)
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(PromptError::EmptyInput);
    }

    Ok(PathBuf::from(trimmed))
}

/// Prompt for a file path that must exist.
pub(crate) fn prompt_existing_path(label: &str) -> PromptResult<PathBuf> {
    let path = prompt_path(label)?;
    if !path.exists() {
        return Err(PromptError::ValidationFailed(format!(
            "File not found: {}",
            path.display()
        )));
    }
    Ok(path)
}

/// Validates a hex hash string (e.g., "0xabc...def" or "abc...def").
///
/// Returns `Ok(())` if the hash is valid (at least 10 hex characters),
/// or an error message describing the validation failure.
pub(crate) fn validate_hash(value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("Hash cannot be empty".to_string());
    }
    let hex = trimmed.strip_prefix("0x").unwrap_or(trimmed);
    if hex.len() < 10 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(
            "Must be a valid hex hash (min 10 chars, with or without 0x prefix)".to_string(),
        );
    }
    Ok(())
}

/// Prompt for a hash string (transaction hash).
pub(crate) fn prompt_hash(label: &str) -> PromptResult<String> {
    let input: String = Input::<String>::new()
        .with_prompt(label)
        .validate_with(|input: &String| -> Result<(), &str> {
            if validate_hash(input).is_err() {
                Err("Must be a valid hex hash (min 10 chars, with or without 0x prefix)")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(input.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tty_returns_bool() {
        let _result: bool = is_tty();
    }

    #[test]
    fn test_prompt_error_display() {
        let err = PromptError::Cancelled;
        assert_eq!(format!("{}", err), "Prompt cancelled by user");

        let err = PromptError::EmptyInput;
        assert_eq!(format!("{}", err), "Empty input is not allowed");

        let err = PromptError::ValidationFailed("bad input".to_string());
        assert!(format!("{}", err).contains("bad input"));
    }

    // =========================================================================
    // validate_address tests
    // =========================================================================

    #[test]
    fn test_validate_address_valid_xion_address() {
        // A real xion1 address encoded with bech32
        let hrp = bech32::Hrp::parse("xion").unwrap();
        let valid = bech32::encode::<bech32::Bech32>(hrp, &[0u8; 32]).unwrap();
        assert!(valid.starts_with("xion1"));

        let result = validate_address(&valid);
        assert!(
            result.is_ok(),
            "Valid xion address should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_address_empty_string() {
        let result = validate_address("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_address_whitespace_only() {
        let result = validate_address("   ");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_address_wrong_prefix() {
        // cosmos1 prefix is invalid for Xion
        let hrp_cosmos = bech32::Hrp::parse("cosmos").unwrap();
        let cosmos_addr = bech32::encode::<bech32::Bech32>(hrp_cosmos, &[0u8; 32]).unwrap();

        let result = validate_address(&cosmos_addr);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("expected prefix 'xion'"), "Got: {}", err);
    }

    #[test]
    fn test_validate_address_invalid_bech32() {
        // Not a valid bech32 string at all
        let result = validate_address("not-a-valid-address");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("bech32 decode failed") || err.contains("Invalid Xion address"),
            "Got: {}",
            err
        );
    }

    #[test]
    fn test_validate_address_invalid_checksum() {
        // Valid bech32 format but invalid checksum
        let bad_checksum = "xion1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqn7t2qz";
        assert!(bech32::decode(bad_checksum).is_err());

        let result = validate_address(bad_checksum);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("bech32 decode failed") || err.contains("Invalid Xion address"),
            "Got: {}",
            err
        );
    }

    // =========================================================================
    // validate_amount tests
    // =========================================================================

    #[test]
    fn test_validate_amount_valid_plain_number() {
        let result = validate_amount("1000000");
        assert!(result.is_ok(), "Plain number should pass: {:?}", result);
    }

    #[test]
    fn test_validate_amount_valid_with_uxion_suffix() {
        let result = validate_amount("1000000uxion");
        assert!(
            result.is_ok(),
            "Amount with uxion suffix should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_amount_empty_string() {
        let result = validate_amount("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_amount_whitespace_only() {
        let result = validate_amount("   ");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_amount_non_numeric_string() {
        let result = validate_amount("abc123");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("must start with a number") || err.contains("must be a number"),
            "Got: {}",
            err
        );
    }

    #[test]
    fn test_validate_amount_starts_with_letter() {
        let result = validate_amount("uxion1000000");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("must start with a number") || err.contains("must be a number"),
            "Got: {}",
            err
        );
    }

    // =========================================================================
    // validate_hash tests
    // =========================================================================

    #[test]
    fn test_validate_hash_valid_hex_without_prefix() {
        let result = validate_hash("abcdef0123456789");
        assert!(
            result.is_ok(),
            "Valid hex without prefix should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_hash_valid_hex_with_prefix() {
        let result = validate_hash("0xabcdef0123456789");
        assert!(
            result.is_ok(),
            "Valid hex with 0x prefix should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_hash_empty_string() {
        let result = validate_hash("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_hash_whitespace_only() {
        let result = validate_hash("   ");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_hash_too_short() {
        // Less than 10 characters after stripping 0x
        let result = validate_hash("0xabcdef");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("min 10 chars") || err.contains("valid hex hash"),
            "Got: {}",
            err
        );
    }

    #[test]
    fn test_validate_hash_non_hex_characters() {
        // Contains 'g' which is not a hex character
        let result = validate_hash("0xabcdefghij");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("valid hex hash") || err.contains("hex"),
            "Got: {}",
            err
        );
    }
}
