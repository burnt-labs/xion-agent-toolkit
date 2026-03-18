//! Exit Codes for CLI
//!
//! This module provides standardized exit codes for the CLI, enabling
//! reliable error handling in CI/CD pipelines and shell scripts.
//!
//! # Exit Code Ranges
//!
//! | Range | Category |
//! |-------|----------|
//! | 0 | Success |
//! | 1 | General/Unknown error |
//! | 2-19 | Authentication errors |
//! | 20-39 | Configuration errors |
//! | 40-59 | Network errors |
//! | 60-79 | Transaction errors |
//! | 80-99 | Treasury errors |
//! | 100-119 | Asset errors |
//! | 120-139 | Batch errors |
//! | 140-159 | Faucet errors |
//!
//! # Usage in Scripts
//!
//! ```bash
//! xion-toolkit auth login
//! exit_code=$?
//! if [ $exit_code -eq 0 ]; then
//!     echo "Success"
//! elif [ $exit_code -eq 2 ]; then
//!     echo "Authentication required"
//! elif [ $exit_code -eq 3 ]; then
//!     echo "Token expired, please re-login"
//! fi
//! ```

use crate::shared::error::XionErrorCode;

/// Exit codes for CLI operations
pub mod exit_code {
    // ========================================================================
    // Success
    // ========================================================================
    /// Operation completed successfully
    pub const SUCCESS: i32 = 0;

    // ========================================================================
    // General Errors (1)
    // ========================================================================
    /// General/unknown error
    pub const GENERAL_ERROR: i32 = 1;

    // ========================================================================
    // Authentication Errors (2-19)
    // ========================================================================
    /// Not authenticated - login required
    pub const AUTH_REQUIRED: i32 = 2;
    /// Token has expired
    pub const TOKEN_EXPIRED: i32 = 3;
    /// Refresh token has expired - re-login required
    pub const REFRESH_TOKEN_EXPIRED: i32 = 4;
    /// Invalid credentials
    pub const INVALID_CREDENTIALS: i32 = 5;
    /// OAuth2 callback failed
    pub const OAUTH_CALLBACK_FAILED: i32 = 6;
    /// PKCE verification failed
    pub const PKCE_FAILED: i32 = 7;
    /// Authentication timeout
    pub const AUTH_TIMEOUT: i32 = 8;

    // ========================================================================
    // Configuration Errors (20-39)
    // ========================================================================
    /// Configuration not found
    pub const CONFIG_NOT_FOUND: i32 = 20;
    /// Invalid configuration
    pub const INVALID_CONFIG: i32 = 21;
    /// Encryption failed
    pub const ENCRYPTION_FAILED: i32 = 22;
    /// Decryption failed
    pub const DECRYPTION_FAILED: i32 = 23;
    /// Network not found in configuration
    pub const NETWORK_NOT_FOUND: i32 = 24;

    // ========================================================================
    // Network Errors (40-59)
    // ========================================================================
    /// Connection timeout
    pub const NETWORK_TIMEOUT: i32 = 40;
    /// Rate limited
    pub const RATE_LIMITED: i32 = 41;
    /// Service unavailable
    pub const SERVICE_UNAVAILABLE: i32 = 42;
    /// Invalid response from server
    pub const INVALID_RESPONSE: i32 = 43;
    /// Request failed
    pub const REQUEST_FAILED: i32 = 44;
    /// Connection refused
    pub const CONNECTION_REFUSED: i32 = 45;
    /// DNS resolution failed
    pub const DNS_FAILED: i32 = 46;
    /// TLS error
    pub const TLS_ERROR: i32 = 47;

    // ========================================================================
    // Transaction Errors (60-79)
    // ========================================================================
    /// Transaction query failed
    pub const TX_QUERY_FAILED: i32 = 60;
    /// Transaction wait failed
    pub const TX_WAIT_FAILED: i32 = 61;
    /// Transaction timeout
    pub const TX_TIMEOUT: i32 = 62;

    // ========================================================================
    // Treasury Errors (80-99)
    // ========================================================================
    /// Treasury not found
    pub const TREASURY_NOT_FOUND: i32 = 80;
    /// Insufficient balance
    pub const INSUFFICIENT_BALANCE: i32 = 81;
    /// Invalid treasury address
    pub const INVALID_TREASURY_ADDRESS: i32 = 82;
    /// Treasury creation failed
    pub const TREASURY_CREATION_FAILED: i32 = 83;
    /// Treasury operation failed
    pub const TREASURY_OPERATION_FAILED: i32 = 84;
    /// Grant config not found
    pub const GRANT_CONFIG_NOT_FOUND: i32 = 85;
    /// Fee config not found
    pub const FEE_CONFIG_NOT_FOUND: i32 = 86;
    /// Not authorized for treasury operation
    pub const NOT_AUTHORIZED: i32 = 87;
    /// Treasury already exists
    pub const TREASURY_ALREADY_EXISTS: i32 = 88;
    /// Missing authorization input for grant config
    pub const MISSING_AUTHORIZATION_INPUT: i32 = 89;

    // ========================================================================
    // Asset Errors (100-119)
    // ========================================================================
    /// Invalid metadata
    pub const INVALID_METADATA: i32 = 100;
    /// Asset creation failed
    pub const ASSET_CREATION_FAILED: i32 = 101;
    /// Invalid asset configuration
    pub const INVALID_ASSET_CONFIG: i32 = 102;
    /// Code ID not found
    pub const CODE_ID_NOT_FOUND: i32 = 103;
    /// Invalid schema
    pub const INVALID_SCHEMA: i32 = 104;

    // ========================================================================
    // Batch Errors (120-139)
    // ========================================================================
    /// Batch too large
    pub const BATCH_TOO_LARGE: i32 = 120;
    /// Batch execution failed
    pub const BATCH_EXECUTION_FAILED: i32 = 121;
    /// Partial batch failure
    pub const BATCH_PARTIAL_FAILURE: i32 = 122;
    /// Invalid batch item
    pub const INVALID_BATCH_ITEM: i32 = 123;

    // ========================================================================
    // Faucet Errors (140-159)
    // ========================================================================
    /// Faucet claim failed
    pub const FAUCET_CLAIM_FAILED: i32 = 140;
    /// Faucet query failed
    pub const FAUCET_QUERY_FAILED: i32 = 141;
    /// Not authenticated for faucet
    pub const FAUCET_AUTH_REQUIRED: i32 = 142;
    /// Faucet not available on this network
    pub const FAUCET_NOT_AVAILABLE: i32 = 143;
}

impl XionErrorCode {
    /// Map error code to exit code for CLI
    pub fn exit_code(&self) -> i32 {
        use exit_code::*;

        match self {
            // Authentication errors
            XionErrorCode::EAUTH001 => AUTH_REQUIRED,
            XionErrorCode::EAUTH002 => TOKEN_EXPIRED,
            XionErrorCode::EAUTH003 => REFRESH_TOKEN_EXPIRED,
            XionErrorCode::EAUTH004 => INVALID_CREDENTIALS,
            XionErrorCode::EAUTH005 => OAUTH_CALLBACK_FAILED,
            XionErrorCode::EAUTH006 => PKCE_FAILED,
            XionErrorCode::EAUTH007 => AUTH_TIMEOUT,

            // Treasury errors
            XionErrorCode::ETREASURY001 => TREASURY_NOT_FOUND,
            XionErrorCode::ETREASURY002 => INSUFFICIENT_BALANCE,
            XionErrorCode::ETREASURY003 => INVALID_TREASURY_ADDRESS,
            XionErrorCode::ETREASURY004 => TREASURY_CREATION_FAILED,
            XionErrorCode::ETREASURY005 => TREASURY_OPERATION_FAILED,
            XionErrorCode::ETREASURY006 => GRANT_CONFIG_NOT_FOUND,
            XionErrorCode::ETREASURY007 => FEE_CONFIG_NOT_FOUND,
            XionErrorCode::ETREASURY008 => NOT_AUTHORIZED,
            XionErrorCode::ETREASURY009 => TREASURY_ALREADY_EXISTS,
            XionErrorCode::ETREASURY010 => MISSING_AUTHORIZATION_INPUT,

            // Asset errors
            XionErrorCode::EASSET001 => INVALID_METADATA,
            XionErrorCode::EASSET002 => ASSET_CREATION_FAILED,
            XionErrorCode::EASSET003 => INVALID_ASSET_CONFIG,
            XionErrorCode::EASSET004 => CODE_ID_NOT_FOUND,
            XionErrorCode::EASSET005 => INVALID_SCHEMA,

            // Batch errors
            XionErrorCode::EBATCH001 => BATCH_TOO_LARGE,
            XionErrorCode::EBATCH002 => BATCH_EXECUTION_FAILED,
            XionErrorCode::EBATCH003 => BATCH_PARTIAL_FAILURE,
            XionErrorCode::EBATCH004 => INVALID_BATCH_ITEM,

            // Configuration errors
            XionErrorCode::ECONFIG001 => CONFIG_NOT_FOUND,
            XionErrorCode::ECONFIG002 => INVALID_CONFIG,
            XionErrorCode::ECONFIG003 => ENCRYPTION_FAILED,
            XionErrorCode::ECONFIG004 => DECRYPTION_FAILED,
            XionErrorCode::ECONFIG005 => NETWORK_NOT_FOUND,

            // Network errors
            XionErrorCode::ENETWORK001 => NETWORK_TIMEOUT,
            XionErrorCode::ENETWORK002 => RATE_LIMITED,
            XionErrorCode::ENETWORK003 => SERVICE_UNAVAILABLE,
            XionErrorCode::ENETWORK004 => INVALID_RESPONSE,
            XionErrorCode::ENETWORK005 => REQUEST_FAILED,
            XionErrorCode::ENETWORK006 => CONNECTION_REFUSED,
            XionErrorCode::ENETWORK007 => DNS_FAILED,
            XionErrorCode::ENETWORK008 => TLS_ERROR,

            // Transaction errors
            XionErrorCode::ETX001 => TX_QUERY_FAILED,
            XionErrorCode::ETX002 => TX_WAIT_FAILED,
            XionErrorCode::ETX003 => TX_TIMEOUT,

            // Faucet errors
            XionErrorCode::EFAUCET001 => FAUCET_CLAIM_FAILED,
            XionErrorCode::EFAUCET002 => FAUCET_QUERY_FAILED,
            XionErrorCode::EFAUCET003 => FAUCET_AUTH_REQUIRED,
            XionErrorCode::EFAUCET004 => FAUCET_NOT_AVAILABLE,
        }
    }
}

/// Get exit code name for display
pub fn exit_code_name(code: i32) -> &'static str {
    use exit_code::*;

    match code {
        SUCCESS => "SUCCESS",
        GENERAL_ERROR => "GENERAL_ERROR",

        // Authentication
        AUTH_REQUIRED => "AUTH_REQUIRED",
        TOKEN_EXPIRED => "TOKEN_EXPIRED",
        REFRESH_TOKEN_EXPIRED => "REFRESH_TOKEN_EXPIRED",
        INVALID_CREDENTIALS => "INVALID_CREDENTIALS",
        OAUTH_CALLBACK_FAILED => "OAUTH_CALLBACK_FAILED",
        PKCE_FAILED => "PKCE_FAILED",
        AUTH_TIMEOUT => "AUTH_TIMEOUT",

        // Configuration
        CONFIG_NOT_FOUND => "CONFIG_NOT_FOUND",
        INVALID_CONFIG => "INVALID_CONFIG",
        ENCRYPTION_FAILED => "ENCRYPTION_FAILED",
        DECRYPTION_FAILED => "DECRYPTION_FAILED",
        NETWORK_NOT_FOUND => "NETWORK_NOT_FOUND",

        // Network
        NETWORK_TIMEOUT => "NETWORK_TIMEOUT",
        RATE_LIMITED => "RATE_LIMITED",
        SERVICE_UNAVAILABLE => "SERVICE_UNAVAILABLE",
        INVALID_RESPONSE => "INVALID_RESPONSE",
        REQUEST_FAILED => "REQUEST_FAILED",
        CONNECTION_REFUSED => "CONNECTION_REFUSED",
        DNS_FAILED => "DNS_FAILED",
        TLS_ERROR => "TLS_ERROR",

        // Transaction
        TX_QUERY_FAILED => "TX_QUERY_FAILED",
        TX_WAIT_FAILED => "TX_WAIT_FAILED",
        TX_TIMEOUT => "TX_TIMEOUT",

        // Treasury
        TREASURY_NOT_FOUND => "TREASURY_NOT_FOUND",
        INSUFFICIENT_BALANCE => "INSUFFICIENT_BALANCE",
        INVALID_TREASURY_ADDRESS => "INVALID_TREASURY_ADDRESS",
        TREASURY_CREATION_FAILED => "TREASURY_CREATION_FAILED",
        TREASURY_OPERATION_FAILED => "TREASURY_OPERATION_FAILED",
        GRANT_CONFIG_NOT_FOUND => "GRANT_CONFIG_NOT_FOUND",
        FEE_CONFIG_NOT_FOUND => "FEE_CONFIG_NOT_FOUND",
        NOT_AUTHORIZED => "NOT_AUTHORIZED",
        TREASURY_ALREADY_EXISTS => "TREASURY_ALREADY_EXISTS",
        MISSING_AUTHORIZATION_INPUT => "MISSING_AUTHORIZATION_INPUT",

        // Asset
        INVALID_METADATA => "INVALID_METADATA",
        ASSET_CREATION_FAILED => "ASSET_CREATION_FAILED",
        INVALID_ASSET_CONFIG => "INVALID_ASSET_CONFIG",
        CODE_ID_NOT_FOUND => "CODE_ID_NOT_FOUND",
        INVALID_SCHEMA => "INVALID_SCHEMA",

        // Batch
        BATCH_TOO_LARGE => "BATCH_TOO_LARGE",
        BATCH_EXECUTION_FAILED => "BATCH_EXECUTION_FAILED",
        BATCH_PARTIAL_FAILURE => "BATCH_PARTIAL_FAILURE",
        INVALID_BATCH_ITEM => "INVALID_BATCH_ITEM",

        // Faucet
        FAUCET_CLAIM_FAILED => "FAUCET_CLAIM_FAILED",
        FAUCET_QUERY_FAILED => "FAUCET_QUERY_FAILED",
        FAUCET_AUTH_REQUIRED => "FAUCET_AUTH_REQUIRED",
        FAUCET_NOT_AVAILABLE => "FAUCET_NOT_AVAILABLE",

        _ => "UNKNOWN",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::error::*;

    #[test]
    fn test_success_exit_code() {
        assert_eq!(exit_code::SUCCESS, 0);
    }

    #[test]
    fn test_auth_exit_codes() {
        assert_eq!(
            XionErrorCode::EAUTH001.exit_code(),
            exit_code::AUTH_REQUIRED
        );
        assert_eq!(
            XionErrorCode::EAUTH002.exit_code(),
            exit_code::TOKEN_EXPIRED
        );
        assert_eq!(
            XionErrorCode::EAUTH003.exit_code(),
            exit_code::REFRESH_TOKEN_EXPIRED
        );
    }

    #[test]
    fn test_treasury_exit_codes() {
        assert_eq!(
            XionErrorCode::ETREASURY001.exit_code(),
            exit_code::TREASURY_NOT_FOUND
        );
        assert_eq!(
            XionErrorCode::ETREASURY002.exit_code(),
            exit_code::INSUFFICIENT_BALANCE
        );
    }

    #[test]
    fn test_network_exit_codes() {
        assert_eq!(
            XionErrorCode::ENETWORK001.exit_code(),
            exit_code::NETWORK_TIMEOUT
        );
        assert_eq!(
            XionErrorCode::ENETWORK002.exit_code(),
            exit_code::RATE_LIMITED
        );
    }

    #[test]
    fn test_config_exit_codes() {
        assert_eq!(
            XionErrorCode::ECONFIG001.exit_code(),
            exit_code::CONFIG_NOT_FOUND
        );
        assert_eq!(
            XionErrorCode::ECONFIG002.exit_code(),
            exit_code::INVALID_CONFIG
        );
    }

    #[test]
    fn test_exit_code_name() {
        assert_eq!(exit_code_name(0), "SUCCESS");
        assert_eq!(exit_code_name(2), "AUTH_REQUIRED");
        assert_eq!(exit_code_name(80), "TREASURY_NOT_FOUND");
        assert_eq!(exit_code_name(999), "UNKNOWN");
    }

    #[test]
    fn test_exit_code_ranges() {
        // Authentication: 2-19
        assert!((2..=19).contains(&XionErrorCode::EAUTH001.exit_code()));

        // Configuration: 20-39
        assert!((20..=39).contains(&XionErrorCode::ECONFIG001.exit_code()));

        // Network: 40-59
        assert!((40..=59).contains(&XionErrorCode::ENETWORK001.exit_code()));

        // Transaction: 60-79
        assert!((60..=79).contains(&XionErrorCode::ETX001.exit_code()));

        // Treasury: 80-99
        assert!((80..=99).contains(&XionErrorCode::ETREASURY001.exit_code()));

        // Asset: 100-119
        assert!((100..=119).contains(&XionErrorCode::EASSET001.exit_code()));

        // Batch: 120-139
        assert!((120..=139).contains(&XionErrorCode::EBATCH001.exit_code()));

        // Faucet: 140-159
        assert!((140..=159).contains(&XionErrorCode::EFAUCET001.exit_code()));
    }

    #[test]
    fn test_xion_error_exit_code() {
        let err = XionError::from(AuthError::NotAuthenticated("test".to_string()));
        assert_eq!(err.code().exit_code(), exit_code::AUTH_REQUIRED);
    }
}
