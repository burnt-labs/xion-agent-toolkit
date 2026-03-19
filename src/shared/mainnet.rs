//! Mainnet Disable Switch
//!
//! This module provides functionality to disable mainnet mode in the CLI.
//! When mainnet is disabled, users attempting to use `--network mainnet`
//! receive a clear error message.
//!
//! # Configuration
//!
//! The mainnet disable switch is controlled by the `XION_MAINNET_DISABLED`
//! environment variable:
//!
//! - `true` or `1`: Mainnet is disabled (default)
//! - `false` or `0`: Mainnet is enabled
//!
//! # Examples
//!
//! ```bash
//! # Default behavior - mainnet disabled
//! xion-toolkit --network mainnet auth status
//! # Error: Mainnet mode is currently disabled.
//!
//! # Enable mainnet
//! XION_MAINNET_DISABLED=false xion-toolkit --network mainnet auth status
//! ```

/// Check if mainnet mode is disabled.
///
/// # Returns
///
/// - `true` if mainnet is disabled (default behavior)
/// - `false` if mainnet is explicitly enabled
///
/// # Environment Variable
///
/// Reads `XION_MAINNET_DISABLED` environment variable:
/// - `"true"` or `"1"` -> disabled (returns true)
/// - `"false"` or `"0"` -> enabled (returns false)
/// - Not set or invalid value -> disabled by default (returns true)
///
/// # Examples
///
/// ```rust,ignore
/// use xion_agent_toolkit::shared::mainnet::is_mainnet_disabled;
///
/// // Default: mainnet is disabled
/// assert!(is_mainnet_disabled());
///
/// // When XION_MAINNET_DISABLED=false
/// // is_mainnet_disabled() returns false
/// ```
pub fn is_mainnet_disabled() -> bool {
    match std::env::var("XION_MAINNET_DISABLED") {
        Ok(v) => {
            let v_lower = v.to_lowercase();
            // Only "false" or "0" explicitly enables mainnet
            // All other values (including invalid ones) default to disabled
            !(v_lower == "false" || v == "0")
        }
        Err(_) => true, // Default: mainnet is disabled
    }
}

/// Print the mainnet disabled error message to stderr.
pub fn print_mainnet_disabled_error() {
    eprintln!("Error: Mainnet mode is currently disabled.");
    eprintln!("  The xion-toolkit CLI is currently only available for testnet.");
    eprintln!("  Please use --network testnet or omit the flag (testnet is default).");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial(mainnet_env)]
    fn test_mainnet_disabled_by_default() {
        // Remove the env var to test default behavior
        std::env::remove_var("XION_MAINNET_DISABLED");
        assert!(is_mainnet_disabled());
    }

    #[test]
    #[serial(mainnet_env)]
    fn test_mainnet_explicitly_disabled() {
        std::env::set_var("XION_MAINNET_DISABLED", "true");
        assert!(is_mainnet_disabled());
        std::env::remove_var("XION_MAINNET_DISABLED");

        std::env::set_var("XION_MAINNET_DISABLED", "TRUE");
        assert!(is_mainnet_disabled());
        std::env::remove_var("XION_MAINNET_DISABLED");

        std::env::set_var("XION_MAINNET_DISABLED", "1");
        assert!(is_mainnet_disabled());
        std::env::remove_var("XION_MAINNET_DISABLED");
    }

    #[test]
    #[serial(mainnet_env)]
    fn test_mainnet_explicitly_enabled() {
        std::env::set_var("XION_MAINNET_DISABLED", "false");
        assert!(!is_mainnet_disabled());
        std::env::remove_var("XION_MAINNET_DISABLED");

        std::env::set_var("XION_MAINNET_DISABLED", "FALSE");
        assert!(!is_mainnet_disabled());
        std::env::remove_var("XION_MAINNET_DISABLED");

        std::env::set_var("XION_MAINNET_DISABLED", "0");
        assert!(!is_mainnet_disabled());
        std::env::remove_var("XION_MAINNET_DISABLED");
    }

    #[test]
    #[serial(mainnet_env)]
    fn test_mainnet_invalid_value_defaults_to_disabled() {
        // Invalid values should default to disabled (only "false" or "0" enables mainnet)
        std::env::set_var("XION_MAINNET_DISABLED", "invalid");
        assert!(
            is_mainnet_disabled(),
            "'invalid' should default to disabled"
        );
        std::env::remove_var("XION_MAINNET_DISABLED");

        std::env::set_var("XION_MAINNET_DISABLED", "yes");
        assert!(is_mainnet_disabled(), "'yes' should default to disabled");
        std::env::remove_var("XION_MAINNET_DISABLED");

        std::env::set_var("XION_MAINNET_DISABLED", "no");
        assert!(is_mainnet_disabled(), "'no' should default to disabled");
        std::env::remove_var("XION_MAINNET_DISABLED");

        std::env::set_var("XION_MAINNET_DISABLED", "true");
        assert!(is_mainnet_disabled(), "'true' should be disabled");
        std::env::remove_var("XION_MAINNET_DISABLED");

        std::env::set_var("XION_MAINNET_DISABLED", "1");
        assert!(is_mainnet_disabled(), "'1' should be disabled");
        std::env::remove_var("XION_MAINNET_DISABLED");
    }
}
