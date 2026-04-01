//! Interactive parameter prompting for missing required arguments.
//!
//! When running in a TTY without `--no-interactive`, missing required
//! parameters are collected via user prompts instead of causing an error exit.

use std::io::{self, IsTerminal};
use std::path::PathBuf;

use dialoguer::{Input, Select};

/// Check if interactive mode is available (TTY on stdin).
pub fn is_tty() -> bool {
    io::stdin().is_terminal()
}

/// Result of interactive prompting.
pub type PromptResult<T> = Result<T, PromptError>;

#[derive(Debug)]
pub enum PromptError {
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
pub fn prompt_text(label: &str) -> PromptResult<String> {
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

/// Prompt for a text string with a default value.
pub fn prompt_text_with_default(label: &str, default: &str) -> PromptResult<String> {
    let input: String = Input::new()
        .with_prompt(label)
        .default(default.to_string())
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(input)
}

/// Prompt for a u64 number. Re-prompts on invalid input.
pub fn prompt_u64(label: &str) -> PromptResult<u64> {
    let value: u64 = Input::<u64>::new()
        .with_prompt(label)
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(value)
}

/// Prompt for a blockchain address (xion1... format).
pub fn prompt_address(label: &str) -> PromptResult<String> {
    let input: String = Input::<String>::new()
        .with_prompt(label)
        .validate_with(|input: &String| -> Result<(), &str> {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Err("Address cannot be empty");
            }
            if !trimmed.starts_with("xion1") {
                return Err("Address must start with 'xion1'");
            }
            if trimmed.len() < 20 {
                return Err("Address seems too short");
            }
            Ok(())
        })
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(input.trim().to_string())
}

/// Prompt for an amount string (e.g., "1000000uxion" or "1000000").
/// If the user enters a plain number without a denomination, "uxion" is appended automatically.
pub fn prompt_amount(label: &str) -> PromptResult<String> {
    let input: String = Input::<String>::new()
        .with_prompt(label)
        .validate_with(|input: &String| -> Result<(), &str> {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Err("Amount cannot be empty");
            }
            if !trimmed
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
            {
                return Err("Amount must start with a number");
            }
            // Accept: "1000000uxion" (digits + denom) or "1000000" (pure digits)
            if !trimmed.ends_with("uxion") {
                if !trimmed.chars().all(|c| c.is_ascii_digit()) {
                    return Err("Amount must be a number (e.g., 1000000) or with denom (e.g., 1000000uxion)");
                }
            } else {
                let prefix = &trimmed[..trimmed.len() - 5];
                if prefix.is_empty() || !prefix.chars().all(|c| c.is_ascii_digit()) {
                    return Err("Amount must be a number (e.g., 1000000) or with denom (e.g., 1000000uxion)");
                }
            }
            Ok(())
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
pub fn prompt_path(label: &str) -> PromptResult<PathBuf> {
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
pub fn prompt_existing_path(label: &str) -> PromptResult<PathBuf> {
    let path = prompt_path(label)?;
    if !path.exists() {
        return Err(PromptError::ValidationFailed(format!(
            "File not found: {}",
            path.display()
        )));
    }
    Ok(path)
}

/// Prompt for a selection from a list of choices.
pub fn prompt_select<T: ToString>(label: &str, items: &[T]) -> PromptResult<usize> {
    Select::new()
        .with_prompt(label)
        .items(items)
        .interact()
        .map_err(|_| PromptError::Cancelled)
}

/// Prompt for a hash string (transaction hash).
pub fn prompt_hash(label: &str) -> PromptResult<String> {
    let input: String = Input::<String>::new()
        .with_prompt(label)
        .validate_with(|input: &String| -> Result<(), &str> {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Err("Hash cannot be empty");
            }
            let hex = trimmed.strip_prefix("0x").unwrap_or(trimmed);
            if hex.len() < 10 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err("Must be a valid hex hash (min 10 chars, with or without 0x prefix)");
            }
            Ok(())
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

    #[test]
    fn test_address_validation_logic() {
        let valid = "xion1abc123def456ghij";
        assert!(valid.starts_with("xion1"));
        assert!(valid.len() >= 20);

        let invalid_prefix = "cosmos1abc";
        assert!(!invalid_prefix.starts_with("xion1"));

        let too_short = "xion1";
        assert!(too_short.len() < 20);
    }
}
