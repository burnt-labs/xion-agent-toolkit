//! Shared Module
//!
//! This module provides shared functionality used across the toolkit:
//!
//! - **error**: Structured error types with codes and hints
//! - **retry**: Automatic retry with exponential backoff
//! - **exit_codes**: Standardized exit codes for CLI
//! - **instantiate2**: Deterministic address computation for CosmWasm instantiate2
//! - **mainnet**: Mainnet disable switch functionality
//!
//! # Error Handling
//!
//! All errors in the toolkit use structured error codes for easy
//! identification and remediation. Each error includes:
//!
//! - A unique error code (e.g., `ETREASURY001`)
//! - A human-readable message
//! - An actionable hint for resolution
//! - A retryable flag for transient failures
//!
//! ## Example
//!
//! ```rust,ignore
//! use xion_agent_toolkit::shared::error::{XionError, XionErrorCode, ErrorResponse};
//!
//! // Create an error with context
//! let error = XionError::from(AuthError::NotAuthenticated(
//!     "Please login first".to_string()
//! ));
//!
//! // Get the error code
//! assert_eq!(error.code(), XionErrorCode::EAUTH001);
//!
//! // Convert to JSON response
//! let response = error.to_response();
//! println!("{}", serde_json::to_string_pretty(&response)?);
//! ```
//!
//! # Retry Logic
//!
//! Transient network errors are automatically retried with exponential backoff:
//!
//! ```rust,ignore
//! use xion_agent_toolkit::shared::retry::{with_retry, RetryConfig};
//!
//! let config = RetryConfig::default();
//! let result = with_retry(&config, || async_operation(), |err| err.is_retryable()).await;
//! ```
//!
//! # Exit Codes
//!
//! The CLI returns standardized exit codes for CI/CD integration:
//!
//! ```rust,ignore
//! use xion_agent_toolkit::shared::exit_codes::exit_code;
//!
//! // Success
//! assert_eq!(exit_code::SUCCESS, 0);
//!
//! // Auth required
//! assert_eq!(exit_code::AUTH_REQUIRED, 2);
//! ```
//!
//! # Instantiate2 Address Prediction
//!
//! Predict contract addresses before deployment:
//!
//! ```rust,ignore
//! use xion_agent_toolkit::shared::instantiate2::{predict_treasury_address, SaltEncoding};
//!
//! // Auto-detect salt encoding and predict address
//! let predicted = predict_treasury_address(
//!     "xion1...",  // creator
//!     1260,        // code ID
//!     "my-salt",   // salt (auto-detected as UTF-8 or hex)
//!     &checksum,   // from get_code_info
//! )?;
//! ```

pub mod error;
pub mod exit_codes;
pub mod instantiate2;
pub mod mainnet;
pub mod retry;

// Re-export commonly used types
pub use error::{
    AssetError, AuthError, BatchError, ConfigError, ErrorDetail, ErrorResponse, NetworkError,
    TreasuryError, TxError, XionError, XionErrorCode, XionResult,
};
pub use exit_codes::{exit_code, exit_code_name};
pub use instantiate2::{
    compute_address, detect_salt_encoding, predict_treasury_address, validate_salt,
    PredictedAddress, SaltEncoding,
};
pub use mainnet::{is_mainnet_disabled, print_mainnet_disabled_error};
pub use retry::{
    is_retryable_reqwest_error, is_retryable_status, reqwest_to_xion_error, with_retry,
    with_retry_metadata, RetryConfig, RetryResult,
};
