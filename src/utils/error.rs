//! Error utilities (compatibility layer)
//!
//! This module re-exports error types from the shared module for backward compatibility.
//! New code should use `crate::shared::error` directly.

// Re-export from shared module for backward compatibility
pub use crate::shared::error::{
    AssetError, AuthError, BatchError, ConfigError, ErrorDetail, ErrorResponse, NetworkError,
    TreasuryError, XionError, XionErrorCode, XionResult,
};
