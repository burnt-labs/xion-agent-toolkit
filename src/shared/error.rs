//! Structured Error Types for Xion Agent Toolkit
//!
//! This module provides structured error handling with:
//! - Unique error codes for each error type
//! - Actionable remediation hints
//! - Retry classification for transient failures
//!
//! # Error Code Schema
//!
//! Format: `E{MODULE}{NUMBER}`
//!
//! Modules:
//! - AUTH: Authentication (EAUTH001-EAUTH099)
//! - TREASURY: Treasury operations (ETREASURY001-ETREASURY099)
//! - ASSET: Asset builder (EASSET001-EASSET099)
//! - BATCH: Batch operations (EBATCH001-EBATCH099)
//! - CONFIG: Configuration (ECONFIG001-ECONFIG099)
//! - NETWORK: Network/API (ENETWORK001-ENETWORK099)

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Error code enumeration with structured hints
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum XionErrorCode {
    // ========================================================================
    // Authentication Errors (EAUTH001-EAUTH099)
    // ========================================================================
    /// Not authenticated
    EAUTH001,
    /// Token expired
    EAUTH002,
    /// Refresh token expired
    EAUTH003,
    /// Invalid credentials
    EAUTH004,
    /// OAuth2 callback failed
    EAUTH005,
    /// PKCE verification failed
    EAUTH006,
    /// Authentication timeout
    EAUTH007,

    // ========================================================================
    // Treasury Errors (ETREASURY001-ETREASURY099)
    // ========================================================================
    /// Treasury not found
    ETREASURY001,
    /// Insufficient balance
    ETREASURY002,
    /// Invalid treasury address
    ETREASURY003,
    /// Treasury creation failed
    ETREASURY004,
    /// Treasury operation failed
    ETREASURY005,
    /// Grant config not found
    ETREASURY006,
    /// Fee config not found
    ETREASURY007,
    /// Not authorized for treasury
    ETREASURY008,
    /// Treasury already exists
    ETREASURY009,
    /// Missing authorization input
    ETREASURY010,

    // ========================================================================
    // Asset Builder Errors (EASSET001-EASSET099)
    // ========================================================================
    /// Invalid metadata
    EASSET001,
    /// Asset creation failed
    EASSET002,
    /// Invalid asset configuration
    EASSET003,
    /// Code ID not found
    EASSET004,
    /// Invalid schema
    EASSET005,

    // ========================================================================
    // Batch Errors (EBATCH001-EBATCH099)
    // ========================================================================
    /// Batch too large
    EBATCH001,
    /// Batch execution failed
    EBATCH002,
    /// Partial batch failure
    EBATCH003,
    /// Invalid batch item
    EBATCH004,

    // ========================================================================
    // Configuration Errors (ECONFIG001-ECONFIG099)
    // ========================================================================
    /// Configuration not found
    ECONFIG001,
    /// Invalid configuration
    ECONFIG002,
    /// Encryption failed
    ECONFIG003,
    /// Decryption failed
    ECONFIG004,
    /// Network not found
    ECONFIG005,

    // ========================================================================
    // Network Errors (ENETWORK001-ENETWORK099)
    // ========================================================================
    /// Connection timeout
    ENETWORK001,
    /// Rate limited
    ENETWORK002,
    /// Service unavailable
    ENETWORK003,
    /// Invalid response
    ENETWORK004,
    /// Request failed
    ENETWORK005,
    /// Connection refused
    ENETWORK006,
    /// DNS resolution failed
    ENETWORK007,
    /// TLS error
    ENETWORK008,

    // ========================================================================
    // Transaction Errors (ETX001-ETX099)
    // ========================================================================
    /// Transaction query failed
    ETX001,
    /// Transaction wait failed
    ETX002,
    /// Transaction timeout
    ETX003,

    // ========================================================================
    // Faucet Errors (EFAUCET001-EFAUCET099)
    // ========================================================================
    /// Faucet claim failed
    EFAUCET001,
    /// Faucet query failed
    EFAUCET002,
    /// Not authenticated for faucet
    EFAUCET003,
    /// Faucet not available
    EFAUCET004,

    // ========================================================================
    // OAuth Client Errors (EOAUTHCLIENT001-EOAUTHCLIENT099)
    // ========================================================================
    /// Bad request
    EOAUTHCLIENT001,
    /// Client ID required
    EOAUTHCLIENT002,
    /// Redirect URIs required
    EOAUTHCLIENT003,
    /// Binded treasury required
    EOAUTHCLIENT004,
    /// Owner required
    EOAUTHCLIENT005,
    /// Invalid grant type
    EOAUTHCLIENT006,
    /// Manager user ID required
    EOAUTHCLIENT007,
    /// Authentication required
    EOAUTHCLIENT008,
    /// User not found
    EOAUTHCLIENT009,
    /// Insufficient scope
    EOAUTHCLIENT010,
    /// Only owner allowed
    EOAUTHCLIENT011,
    /// Client not found
    EOAUTHCLIENT012,
    /// Client extension not found
    EOAUTHCLIENT013,
    /// Treasury not found (MGR API)
    EOAUTHCLIENT014,
    /// Internal server error (MGR API)
    EOAUTHCLIENT015,
    /// Treasury fetch error
    EOAUTHCLIENT016,
    /// Treasury query error
    EOAUTHCLIENT017,
    /// Unknown network
    EOAUTHCLIENT018,
    /// Re-run the command with --force to confirm
    EOAUTHCLIENT019,
}

impl XionErrorCode {
    /// Get the error message for this code
    pub fn message(&self) -> &'static str {
        match self {
            // Authentication
            XionErrorCode::EAUTH001 => "Not authenticated",
            XionErrorCode::EAUTH002 => "Token expired",
            XionErrorCode::EAUTH003 => "Refresh token expired",
            XionErrorCode::EAUTH004 => "Invalid credentials",
            XionErrorCode::EAUTH005 => "OAuth2 callback failed",
            XionErrorCode::EAUTH006 => "PKCE verification failed",
            XionErrorCode::EAUTH007 => "Authentication timeout",

            // Treasury
            XionErrorCode::ETREASURY001 => "Treasury not found",
            XionErrorCode::ETREASURY002 => "Insufficient balance",
            XionErrorCode::ETREASURY003 => "Invalid treasury address",
            XionErrorCode::ETREASURY004 => "Treasury creation failed",
            XionErrorCode::ETREASURY005 => "Treasury operation failed",
            XionErrorCode::ETREASURY006 => "Grant config not found",
            XionErrorCode::ETREASURY007 => "Fee config not found",
            XionErrorCode::ETREASURY008 => "Not authorized for treasury operation",
            XionErrorCode::ETREASURY009 => "Treasury already exists",
            XionErrorCode::ETREASURY010 => "Missing authorization input for grant config",

            // Asset Builder
            XionErrorCode::EASSET001 => "Invalid metadata",
            XionErrorCode::EASSET002 => "Asset creation failed",
            XionErrorCode::EASSET003 => "Invalid asset configuration",
            XionErrorCode::EASSET004 => "Code ID not found",
            XionErrorCode::EASSET005 => "Invalid schema",

            // Batch
            XionErrorCode::EBATCH001 => "Batch too large",
            XionErrorCode::EBATCH002 => "Batch execution failed",
            XionErrorCode::EBATCH003 => "Partial batch failure",
            XionErrorCode::EBATCH004 => "Invalid batch item",

            // Configuration
            XionErrorCode::ECONFIG001 => "Configuration not found",
            XionErrorCode::ECONFIG002 => "Invalid configuration",
            XionErrorCode::ECONFIG003 => "Encryption failed",
            XionErrorCode::ECONFIG004 => "Decryption failed",
            XionErrorCode::ECONFIG005 => "Network not found in configuration",

            // Network
            XionErrorCode::ENETWORK001 => "Connection timeout",
            XionErrorCode::ENETWORK002 => "Rate limited",
            XionErrorCode::ENETWORK003 => "Service unavailable",
            XionErrorCode::ENETWORK004 => "Invalid response from server",
            XionErrorCode::ENETWORK005 => "Request failed",
            XionErrorCode::ENETWORK006 => "Connection refused",
            XionErrorCode::ENETWORK007 => "DNS resolution failed",
            XionErrorCode::ENETWORK008 => "TLS error",

            // Transaction
            XionErrorCode::ETX001 => "Transaction query failed",
            XionErrorCode::ETX002 => "Transaction wait failed",
            XionErrorCode::ETX003 => "Transaction timeout",

            // Faucet
            XionErrorCode::EFAUCET001 => "Faucet claim failed",
            XionErrorCode::EFAUCET002 => "Faucet query failed",
            XionErrorCode::EFAUCET003 => "Not authenticated for faucet operation",
            XionErrorCode::EFAUCET004 => "Faucet not available on this network",

            // OAuth Client Management
            XionErrorCode::EOAUTHCLIENT001 => "Bad request",
            XionErrorCode::EOAUTHCLIENT002 => "Client ID is required",
            XionErrorCode::EOAUTHCLIENT003 => "Redirect URIs are required",
            XionErrorCode::EOAUTHCLIENT004 => "Binded treasury is required",
            XionErrorCode::EOAUTHCLIENT005 => "Owner is required",
            XionErrorCode::EOAUTHCLIENT006 => "Invalid grant type",
            XionErrorCode::EOAUTHCLIENT007 => "Manager user ID is required",
            XionErrorCode::EOAUTHCLIENT008 => "Authentication required",
            XionErrorCode::EOAUTHCLIENT009 => "User not found",
            XionErrorCode::EOAUTHCLIENT010 => "Insufficient scope",
            XionErrorCode::EOAUTHCLIENT011 => "Only owner allowed",
            XionErrorCode::EOAUTHCLIENT012 => "Client not found",
            XionErrorCode::EOAUTHCLIENT013 => "Client extension not found",
            XionErrorCode::EOAUTHCLIENT014 => "Treasury not found",
            XionErrorCode::EOAUTHCLIENT015 => "Internal server error",
            XionErrorCode::EOAUTHCLIENT016 => "Treasury fetch error",
            XionErrorCode::EOAUTHCLIENT017 => "Treasury query error",
            XionErrorCode::EOAUTHCLIENT018 => "Unknown network",
            XionErrorCode::EOAUTHCLIENT019 => "Confirmation required",
        }
    }

    /// Get the remediation hint for this error code
    pub fn hint(&self) -> &'static str {
        match self {
            // Authentication
            XionErrorCode::EAUTH001 => "Run 'xion-toolkit auth login' first",
            XionErrorCode::EAUTH002 => "Token refreshed automatically, please retry",
            XionErrorCode::EAUTH003 => "Re-login required: 'xion-toolkit auth login'",
            XionErrorCode::EAUTH004 => "Check your credentials and try again",
            XionErrorCode::EAUTH005 => "Ensure callback URL is accessible and try again",
            XionErrorCode::EAUTH006 => "PKCE verification mismatch, restart login flow",
            XionErrorCode::EAUTH007 => "Authentication took too long, please try again",

            // Treasury
            XionErrorCode::ETREASURY001 => {
                "Run 'xion-toolkit treasury list' to see available treasuries"
            }
            XionErrorCode::ETREASURY002 => "Fund treasury with 'xion-toolkit treasury fund'",
            XionErrorCode::ETREASURY003 => "Verify the treasury address is a valid bech32 address",
            XionErrorCode::ETREASURY004 => "Check parameters and try again",
            XionErrorCode::ETREASURY005 => "Check treasury state and try again",
            XionErrorCode::ETREASURY006 => {
                "Run 'xion-toolkit treasury grant-config list' to see available grants"
            }
            XionErrorCode::ETREASURY007 => {
                "Run 'xion-toolkit treasury fee-config query' to check fee config"
            }
            XionErrorCode::ETREASURY008 => "Ensure you are the admin of this treasury",
            XionErrorCode::ETREASURY009 => "Use a different salt or address for the new treasury",
            XionErrorCode::ETREASURY010 => {
                "Ensure grant config has authorization_input when importing"
            }

            // Asset Builder
            XionErrorCode::EASSET001 => "Check JSON structure against schema",
            XionErrorCode::EASSET002 => "Check asset configuration and try again",
            XionErrorCode::EASSET003 => "Verify all required fields are present",
            XionErrorCode::EASSET004 => {
                "Check available code IDs with 'xion-toolkit asset code-ids'"
            }
            XionErrorCode::EASSET005 => "Validate your schema against the expected format",

            // Batch
            XionErrorCode::EBATCH001 => "Maximum 50 messages per batch",
            XionErrorCode::EBATCH002 => "Check individual message errors and retry",
            XionErrorCode::EBATCH003 => "Some operations succeeded, check results for details",
            XionErrorCode::EBATCH004 => "Verify batch item format and content",

            // Configuration
            XionErrorCode::ECONFIG001 => "Run 'xion-toolkit config init' to create configuration",
            XionErrorCode::ECONFIG002 => "Check configuration file format and values",
            XionErrorCode::ECONFIG003 => "Check encryption key availability",
            XionErrorCode::ECONFIG004 => "Check encryption key matches the one used for encryption",
            XionErrorCode::ECONFIG005 => "Specify network with '--network' flag or update config",

            // Network
            XionErrorCode::ENETWORK001 => "Check network connectivity, will retry",
            XionErrorCode::ENETWORK002 => "Wait and retry, or reduce request frequency",
            XionErrorCode::ENETWORK003 => "Service is temporarily unavailable, retry later",
            XionErrorCode::ENETWORK004 => "Server returned unexpected data, check API version",
            XionErrorCode::ENETWORK005 => "Check network settings and API endpoint",
            XionErrorCode::ENETWORK006 => "Server is not accepting connections, check endpoint",
            XionErrorCode::ENETWORK007 => "Check DNS settings and network connectivity",
            XionErrorCode::ENETWORK008 => "Check TLS certificates and HTTPS configuration",

            // Transaction
            XionErrorCode::ETX001 => "Check network connection and transaction hash",
            XionErrorCode::ETX002 => "Check network connection and wait parameters",
            XionErrorCode::ETX003 => "Transaction took too long to confirm, check chain status",

            // Faucet
            XionErrorCode::EFAUCET001 => "Wait for cooldown or check error details",
            XionErrorCode::EFAUCET002 => {
                "Check network connection and faucet contract availability"
            }
            XionErrorCode::EFAUCET003 => "Run 'xion-toolkit auth login' first",
            XionErrorCode::EFAUCET004 => "Use --network testnet to claim testnet tokens",

            // OAuth Client Management
            XionErrorCode::EOAUTHCLIENT001 => "Check request parameters and try again",
            XionErrorCode::EOAUTHCLIENT002 => "Provide a client ID",
            XionErrorCode::EOAUTHCLIENT003 => "Provide at least one redirect URI",
            XionErrorCode::EOAUTHCLIENT004 => "Provide a treasury address with --treasury",
            XionErrorCode::EOAUTHCLIENT005 => "Provide an owner user ID",
            XionErrorCode::EOAUTHCLIENT006 => "Use a valid grant type (authorization_code, etc.)",
            XionErrorCode::EOAUTHCLIENT007 => "Provide a manager user ID",
            XionErrorCode::EOAUTHCLIENT008 => "Run 'xion-toolkit auth login' first",
            XionErrorCode::EOAUTHCLIENT009 => "Run 'xion-toolkit auth login' first",
            XionErrorCode::EOAUTHCLIENT010 => {
                "Re-authorize with --dev-mode: xion-toolkit auth login --dev-mode"
            }
            XionErrorCode::EOAUTHCLIENT011 => "Only the client owner can perform this action",
            XionErrorCode::EOAUTHCLIENT012 => "Check the client ID and try again",
            XionErrorCode::EOAUTHCLIENT013 => "Check the client ID; extension may not exist",
            XionErrorCode::EOAUTHCLIENT014 => "Verify the treasury address is correct",
            XionErrorCode::EOAUTHCLIENT015 => {
                "The server encountered an error. Please try again later."
            }
            XionErrorCode::EOAUTHCLIENT016 => "Failed to fetch treasury data. Try again later.",
            XionErrorCode::EOAUTHCLIENT017 => "Failed to query treasury data. Try again later.",
            XionErrorCode::EOAUTHCLIENT018 => "Verify network configuration and try again",
            XionErrorCode::EOAUTHCLIENT019 => "Re-run the command with --force to confirm",
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            // Network errors are generally retryable
            XionErrorCode::ENETWORK001
            | XionErrorCode::ENETWORK002
            | XionErrorCode::ENETWORK003
            | XionErrorCode::ENETWORK006
            | XionErrorCode::ENETWORK007
            // Token expired can be retried after refresh
            | XionErrorCode::EAUTH002
            // Faucet errors may be retryable (cooldown, temporary issues)
            | XionErrorCode::EFAUCET001
            | XionErrorCode::EFAUCET002
        )
    }

    /// Get the module name for this error code
    pub fn module(&self) -> &'static str {
        match self {
            XionErrorCode::EAUTH001
            | XionErrorCode::EAUTH002
            | XionErrorCode::EAUTH003
            | XionErrorCode::EAUTH004
            | XionErrorCode::EAUTH005
            | XionErrorCode::EAUTH006
            | XionErrorCode::EAUTH007 => "AUTH",
            XionErrorCode::ETREASURY001
            | XionErrorCode::ETREASURY002
            | XionErrorCode::ETREASURY003
            | XionErrorCode::ETREASURY004
            | XionErrorCode::ETREASURY005
            | XionErrorCode::ETREASURY006
            | XionErrorCode::ETREASURY007
            | XionErrorCode::ETREASURY008
            | XionErrorCode::ETREASURY009
            | XionErrorCode::ETREASURY010 => "TREASURY",
            XionErrorCode::EASSET001
            | XionErrorCode::EASSET002
            | XionErrorCode::EASSET003
            | XionErrorCode::EASSET004
            | XionErrorCode::EASSET005 => "ASSET",
            XionErrorCode::EBATCH001
            | XionErrorCode::EBATCH002
            | XionErrorCode::EBATCH003
            | XionErrorCode::EBATCH004 => "BATCH",
            XionErrorCode::ECONFIG001
            | XionErrorCode::ECONFIG002
            | XionErrorCode::ECONFIG003
            | XionErrorCode::ECONFIG004
            | XionErrorCode::ECONFIG005 => "CONFIG",
            XionErrorCode::ENETWORK001
            | XionErrorCode::ENETWORK002
            | XionErrorCode::ENETWORK003
            | XionErrorCode::ENETWORK004
            | XionErrorCode::ENETWORK005
            | XionErrorCode::ENETWORK006
            | XionErrorCode::ENETWORK007
            | XionErrorCode::ENETWORK008 => "NETWORK",
            XionErrorCode::ETX001 | XionErrorCode::ETX002 | XionErrorCode::ETX003 => "TX",
            XionErrorCode::EFAUCET001
            | XionErrorCode::EFAUCET002
            | XionErrorCode::EFAUCET003
            | XionErrorCode::EFAUCET004 => "FAUCET",
            XionErrorCode::EOAUTHCLIENT001
            | XionErrorCode::EOAUTHCLIENT002
            | XionErrorCode::EOAUTHCLIENT003
            | XionErrorCode::EOAUTHCLIENT004
            | XionErrorCode::EOAUTHCLIENT005
            | XionErrorCode::EOAUTHCLIENT006
            | XionErrorCode::EOAUTHCLIENT007
            | XionErrorCode::EOAUTHCLIENT008
            | XionErrorCode::EOAUTHCLIENT009
            | XionErrorCode::EOAUTHCLIENT010
            | XionErrorCode::EOAUTHCLIENT011
            | XionErrorCode::EOAUTHCLIENT012
            | XionErrorCode::EOAUTHCLIENT013
            | XionErrorCode::EOAUTHCLIENT014
            | XionErrorCode::EOAUTHCLIENT015
            | XionErrorCode::EOAUTHCLIENT016
            | XionErrorCode::EOAUTHCLIENT017
            | XionErrorCode::EOAUTHCLIENT018 => "OAUTH_CLIENT",
            XionErrorCode::EOAUTHCLIENT019 => "OAUTH_CLIENT",
        }
    }
}

impl fmt::Display for XionErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Detailed error information for JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    /// Error code (e.g., "ETREASURY001")
    pub code: XionErrorCode,
    /// Human-readable error message
    pub message: String,
    /// Actionable remediation hint
    pub hint: String,
    /// Whether this error can be retried
    pub retryable: bool,
    /// Optional source error for debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl ErrorDetail {
    /// Create a new error detail with the given code
    pub fn new(code: XionErrorCode) -> Self {
        Self {
            code,
            message: code.message().to_string(),
            hint: code.hint().to_string(),
            retryable: code.is_retryable(),
            source: None,
        }
    }

    /// Create a new error detail with additional context
    pub fn with_context(code: XionErrorCode, context: impl Into<String>) -> Self {
        Self {
            code,
            message: format!("{}: {}", code.message(), context.into()),
            hint: code.hint().to_string(),
            retryable: code.is_retryable(),
            source: None,
        }
    }

    /// Create a new error detail with source information
    pub fn with_source(code: XionErrorCode, source: impl Into<String>) -> Self {
        Self {
            code,
            message: code.message().to_string(),
            hint: code.hint().to_string(),
            retryable: code.is_retryable(),
            source: Some(source.into()),
        }
    }

    /// Add source information to the error
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

/// Structured error output for JSON responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Always false for errors
    pub success: bool,
    /// Error details
    pub error: ErrorDetail,
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(code: XionErrorCode) -> Self {
        Self {
            success: false,
            error: ErrorDetail::new(code),
        }
    }

    /// Create a new error response with context
    pub fn with_context(code: XionErrorCode, context: impl Into<String>) -> Self {
        Self {
            success: false,
            error: ErrorDetail::with_context(code, context),
        }
    }

    /// Create a new error response with source
    pub fn with_source(code: XionErrorCode, source: impl Into<String>) -> Self {
        Self {
            success: false,
            error: ErrorDetail::with_source(code, source),
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error [{}]: {}\n\nHint: {}",
            self.error.code, self.error.message, self.error.hint
        )
    }
}

/// Main error type for Xion Agent Toolkit
#[derive(Debug, Error)]
pub enum XionError {
    /// Authentication error
    #[error("{0}")]
    Auth(#[source] AuthError),

    /// Treasury operation error
    #[error("{0}")]
    Treasury(#[source] TreasuryError),

    /// Asset builder error
    #[error("{0}")]
    Asset(#[source] AssetError),

    /// Batch operation error
    #[error("{0}")]
    Batch(#[source] BatchError),

    /// Configuration error
    #[error("{0}")]
    Config(#[source] ConfigError),

    /// Network/API error
    #[error("{0}")]
    Network(#[source] NetworkError),

    /// Transaction error
    #[error("{0}")]
    Tx(#[source] TxError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// OAuth client management error
    #[error("{0}")]
    OAuthClient(#[source] OAuthClientError),

    /// Generic error with code
    #[error("{message}")]
    Generic {
        code: XionErrorCode,
        message: String,
        hint: String,
    },
}

impl XionError {
    /// Get the error code for this error
    pub fn code(&self) -> XionErrorCode {
        match self {
            XionError::Auth(e) => e.code(),
            XionError::Treasury(e) => e.code(),
            XionError::Asset(e) => e.code(),
            XionError::Batch(e) => e.code(),
            XionError::Config(e) => e.code(),
            XionError::Network(e) => e.code(),
            XionError::Tx(e) => e.code(),
            XionError::OAuthClient(e) => e.code(),
            XionError::Io(_) => XionErrorCode::ECONFIG002,
            XionError::Serialization(_) => XionErrorCode::ECONFIG002,
            XionError::Generic { code, .. } => *code,
        }
    }

    /// Get the hint for this error
    pub fn hint(&self) -> String {
        match self {
            XionError::Auth(e) => e.hint(),
            XionError::Treasury(e) => e.hint(),
            XionError::Asset(e) => e.hint(),
            XionError::Batch(e) => e.hint(),
            XionError::Config(e) => e.hint(),
            XionError::Network(e) => e.hint(),
            XionError::Tx(e) => e.hint(),
            XionError::OAuthClient(e) => e.hint(),
            XionError::Io(_) => "Check file permissions and disk space".to_string(),
            XionError::Serialization(_) => "Check JSON format and structure".to_string(),
            XionError::Generic { hint, .. } => hint.clone(),
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        self.code().is_retryable()
    }

    /// Convert to error response for JSON output
    pub fn to_response(&self) -> ErrorResponse {
        ErrorResponse {
            success: false,
            error: ErrorDetail {
                code: self.code(),
                message: self.to_string(),
                hint: self.hint(),
                retryable: self.is_retryable(),
                source: std::error::Error::source(self).map(|s| s.to_string()),
            },
        }
    }
}

/// Authentication errors
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Not authenticated: {0}")]
    NotAuthenticated(String),

    #[error("Token expired: {0}")]
    TokenExpired(String),

    #[error("Refresh token expired: {0}")]
    RefreshTokenExpired(String),

    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),

    #[error("OAuth2 callback failed: {0}")]
    CallbackFailed(String),

    #[error("PKCE verification failed: {0}")]
    PkceFailed(String),

    #[error("Authentication timeout: {0}")]
    Timeout(String),
}

impl AuthError {
    pub fn code(&self) -> XionErrorCode {
        match self {
            AuthError::NotAuthenticated(_) => XionErrorCode::EAUTH001,
            AuthError::TokenExpired(_) => XionErrorCode::EAUTH002,
            AuthError::RefreshTokenExpired(_) => XionErrorCode::EAUTH003,
            AuthError::InvalidCredentials(_) => XionErrorCode::EAUTH004,
            AuthError::CallbackFailed(_) => XionErrorCode::EAUTH005,
            AuthError::PkceFailed(_) => XionErrorCode::EAUTH006,
            AuthError::Timeout(_) => XionErrorCode::EAUTH007,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Treasury operation errors
#[derive(Debug, Error)]
pub enum TreasuryError {
    #[error("Treasury not found: {0}")]
    NotFound(String),

    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),

    #[error("Invalid treasury address: {0}")]
    InvalidAddress(String),

    #[error("Treasury creation failed: {0}")]
    CreationFailed(String),

    #[error("Treasury operation failed: {0}")]
    OperationFailed(String),

    #[error("Grant config not found: {0}")]
    GrantConfigNotFound(String),

    #[error("Fee config not found: {0}")]
    FeeConfigNotFound(String),

    #[error("Not authorized for treasury: {0}")]
    NotAuthorized(String),

    #[error("Treasury already exists: {0}")]
    AlreadyExists(String),

    #[error("Missing authorization input for grant config: {0}")]
    MissingAuthorizationInput(String),
}

impl TreasuryError {
    pub fn code(&self) -> XionErrorCode {
        match self {
            TreasuryError::NotFound(_) => XionErrorCode::ETREASURY001,
            TreasuryError::InsufficientBalance(_) => XionErrorCode::ETREASURY002,
            TreasuryError::InvalidAddress(_) => XionErrorCode::ETREASURY003,
            TreasuryError::CreationFailed(_) => XionErrorCode::ETREASURY004,
            TreasuryError::OperationFailed(_) => XionErrorCode::ETREASURY005,
            TreasuryError::GrantConfigNotFound(_) => XionErrorCode::ETREASURY006,
            TreasuryError::FeeConfigNotFound(_) => XionErrorCode::ETREASURY007,
            TreasuryError::NotAuthorized(_) => XionErrorCode::ETREASURY008,
            TreasuryError::AlreadyExists(_) => XionErrorCode::ETREASURY009,
            TreasuryError::MissingAuthorizationInput(_) => XionErrorCode::ETREASURY010,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Asset builder errors
#[derive(Debug, Error)]
pub enum AssetError {
    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),

    #[error("Asset creation failed: {0}")]
    CreationFailed(String),

    #[error("Invalid asset configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Code ID not found: {0}")]
    CodeIdNotFound(String),

    #[error("Invalid schema: {0}")]
    InvalidSchema(String),
}

impl AssetError {
    pub fn code(&self) -> XionErrorCode {
        match self {
            AssetError::InvalidMetadata(_) => XionErrorCode::EASSET001,
            AssetError::CreationFailed(_) => XionErrorCode::EASSET002,
            AssetError::InvalidConfiguration(_) => XionErrorCode::EASSET003,
            AssetError::CodeIdNotFound(_) => XionErrorCode::EASSET004,
            AssetError::InvalidSchema(_) => XionErrorCode::EASSET005,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Batch operation errors
#[derive(Debug, Error)]
pub enum BatchError {
    #[error("Batch too large: {0} messages, maximum is 50")]
    TooLarge(usize),

    #[error("Batch execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Partial batch failure: {0} succeeded, {1} failed")]
    PartialFailure(usize, usize),

    #[error("Invalid batch item at index {0}: {1}")]
    InvalidItem(usize, String),
}

impl BatchError {
    pub fn code(&self) -> XionErrorCode {
        match self {
            BatchError::TooLarge(_) => XionErrorCode::EBATCH001,
            BatchError::ExecutionFailed(_) => XionErrorCode::EBATCH002,
            BatchError::PartialFailure(_, _) => XionErrorCode::EBATCH003,
            BatchError::InvalidItem(_, _) => XionErrorCode::EBATCH004,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Configuration errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration not found: {0}")]
    NotFound(String),

    #[error("Invalid configuration: {0}")]
    Invalid(String),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Network not found: {0}")]
    NetworkNotFound(String),
}

impl ConfigError {
    pub fn code(&self) -> XionErrorCode {
        match self {
            ConfigError::NotFound(_) => XionErrorCode::ECONFIG001,
            ConfigError::Invalid(_) => XionErrorCode::ECONFIG002,
            ConfigError::EncryptionFailed(_) => XionErrorCode::ECONFIG003,
            ConfigError::DecryptionFailed(_) => XionErrorCode::ECONFIG004,
            ConfigError::NetworkNotFound(_) => XionErrorCode::ECONFIG005,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Network/API errors
#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Connection timeout: {0}")]
    Timeout(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Connection refused: {0}")]
    ConnectionRefused(String),

    #[error("DNS resolution failed: {0}")]
    DnsFailed(String),

    #[error("TLS error: {0}")]
    TlsError(String),
}

impl NetworkError {
    pub fn code(&self) -> XionErrorCode {
        match self {
            NetworkError::Timeout(_) => XionErrorCode::ENETWORK001,
            NetworkError::RateLimited(_) => XionErrorCode::ENETWORK002,
            NetworkError::ServiceUnavailable(_) => XionErrorCode::ENETWORK003,
            NetworkError::InvalidResponse(_) => XionErrorCode::ENETWORK004,
            NetworkError::RequestFailed(_) => XionErrorCode::ENETWORK005,
            NetworkError::ConnectionRefused(_) => XionErrorCode::ENETWORK006,
            NetworkError::DnsFailed(_) => XionErrorCode::ENETWORK007,
            NetworkError::TlsError(_) => XionErrorCode::ENETWORK008,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Transaction operation errors
#[derive(Debug, Error)]
pub enum TxError {
    #[error("Transaction query failed: {0}")]
    QueryFailed(String),

    #[error("Transaction wait failed: {0}")]
    WaitFailed(String),

    #[error("Transaction timeout: {0}")]
    Timeout(String),
}

impl TxError {
    pub fn code(&self) -> XionErrorCode {
        match self {
            TxError::QueryFailed(_) => XionErrorCode::ETX001,
            TxError::WaitFailed(_) => XionErrorCode::ETX002,
            TxError::Timeout(_) => XionErrorCode::ETX003,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// OAuth client management errors
#[derive(Debug, Error)]
pub enum OAuthClientError {
    #[error("Bad request: {code} - {message}")]
    BadRequest { code: String, message: String },

    #[error("Authentication required: {message}")]
    AuthenticationRequired { message: String },

    #[error("Insufficient scope: {message}")]
    InsufficientScope { message: String },

    #[error("Only owner allowed: {message}")]
    OnlyOwnerAllowed { message: String },

    #[error("Client not found: {client_id}")]
    ClientNotFound { client_id: String },

    #[error("Client extension not found: {client_id}")]
    ClientExtensionNotFound { client_id: String },

    #[error("Treasury not found: {address}")]
    TreasuryNotFound { address: String },

    #[error("User not found: {message}")]
    UserNotFound { message: String },

    #[error("Server error: {code} - {message}")]
    ServerError { code: String, message: String },

    #[error("Network error: {message}")]
    NetworkError { message: String },

    #[error("Invalid response: {message}")]
    InvalidResponse { message: String },

    #[error("Confirmation required: {message}")]
    ConfirmationRequired { message: String },
}

impl OAuthClientError {
    pub fn code(&self) -> XionErrorCode {
        match self {
            OAuthClientError::BadRequest { .. } => XionErrorCode::EOAUTHCLIENT001,
            OAuthClientError::AuthenticationRequired { .. } => XionErrorCode::EOAUTHCLIENT008,
            OAuthClientError::InsufficientScope { .. } => XionErrorCode::EOAUTHCLIENT010,
            OAuthClientError::OnlyOwnerAllowed { .. } => XionErrorCode::EOAUTHCLIENT011,
            OAuthClientError::ClientNotFound { .. } => XionErrorCode::EOAUTHCLIENT012,
            OAuthClientError::ClientExtensionNotFound { .. } => XionErrorCode::EOAUTHCLIENT013,
            OAuthClientError::TreasuryNotFound { .. } => XionErrorCode::EOAUTHCLIENT014,
            OAuthClientError::UserNotFound { .. } => XionErrorCode::EOAUTHCLIENT009,
            OAuthClientError::ServerError { .. } => XionErrorCode::EOAUTHCLIENT015,
            OAuthClientError::NetworkError { .. } => XionErrorCode::ENETWORK005,
            OAuthClientError::InvalidResponse { .. } => XionErrorCode::ENETWORK004,
            OAuthClientError::ConfirmationRequired { .. } => XionErrorCode::EOAUTHCLIENT019,
        }
    }

    pub fn hint(&self) -> String {
        match self {
            OAuthClientError::BadRequest { message, .. } => {
                format!("Check request parameters: {}", message)
            }
            OAuthClientError::AuthenticationRequired { .. } => {
                "Run 'xion-toolkit auth login' first".to_string()
            }
            OAuthClientError::InsufficientScope { .. } => {
                "Re-authorize with --dev-mode: xion-toolkit auth login --dev-mode".to_string()
            }
            OAuthClientError::OnlyOwnerAllowed { .. } => {
                "Only the client owner can perform this action".to_string()
            }
            OAuthClientError::ClientNotFound { .. } => {
                "Check the client ID and try again".to_string()
            }
            OAuthClientError::ClientExtensionNotFound { .. } => {
                "Check the client ID; extension may not exist".to_string()
            }
            OAuthClientError::TreasuryNotFound { .. } => {
                "Verify the treasury address is correct".to_string()
            }
            OAuthClientError::UserNotFound { .. } => {
                "Run 'xion-toolkit auth login' first".to_string()
            }
            OAuthClientError::ServerError { .. } => {
                "The server encountered an error. Please try again later.".to_string()
            }
            OAuthClientError::NetworkError { .. } => {
                "Check network connectivity and try again".to_string()
            }
            OAuthClientError::InvalidResponse { .. } => {
                "Server returned unexpected data. Check API version.".to_string()
            }
            OAuthClientError::ConfirmationRequired { .. } => {
                "Re-run the command with --force to confirm the destructive operation".to_string()
            }
        }
    }
}

// Implement From traits for easy conversion

impl From<AuthError> for XionError {
    fn from(e: AuthError) -> Self {
        XionError::Auth(e)
    }
}

impl From<TreasuryError> for XionError {
    fn from(e: TreasuryError) -> Self {
        XionError::Treasury(e)
    }
}

impl From<AssetError> for XionError {
    fn from(e: AssetError) -> Self {
        XionError::Asset(e)
    }
}

impl From<BatchError> for XionError {
    fn from(e: BatchError) -> Self {
        XionError::Batch(e)
    }
}

impl From<ConfigError> for XionError {
    fn from(e: ConfigError) -> Self {
        XionError::Config(e)
    }
}

impl From<NetworkError> for XionError {
    fn from(e: NetworkError) -> Self {
        XionError::Network(e)
    }
}

impl From<TxError> for XionError {
    fn from(e: TxError) -> Self {
        XionError::Tx(e)
    }
}

impl From<OAuthClientError> for XionError {
    fn from(e: OAuthClientError) -> Self {
        XionError::OAuthClient(e)
    }
}

// Implement From for crate::treasury::encoding::EncodingError
// This is in a separate impl block to handle the cross-module import
impl From<crate::treasury::encoding::EncodingError> for XionError {
    fn from(e: crate::treasury::encoding::EncodingError) -> Self {
        XionError::Treasury(TreasuryError::OperationFailed(e.to_string()))
    }
}

// Implement From<anyhow::Error> for backward compatibility with modules not yet migrated
impl From<anyhow::Error> for XionError {
    fn from(e: anyhow::Error) -> Self {
        // Try to extract a more specific error type from the message
        let err_str = e.to_string();

        // Check for common auth errors
        if err_str.contains("Not authenticated") || err_str.contains("Please login") {
            return XionError::Auth(AuthError::NotAuthenticated(err_str));
        }
        if err_str.contains("Token expired") || err_str.contains("refresh") {
            return XionError::Auth(AuthError::TokenExpired(err_str));
        }
        if err_str.contains("Refresh token expired") {
            return XionError::Auth(AuthError::RefreshTokenExpired(err_str));
        }

        // Default to a generic error
        XionError::Generic {
            code: XionErrorCode::ECONFIG002,
            message: err_str,
            hint: "Check the error message for details".to_string(),
        }
    }
}

/// Result type alias for XionError
pub type XionResult<T> = std::result::Result<T, XionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_message() {
        assert_eq!(XionErrorCode::EAUTH001.message(), "Not authenticated");
        assert_eq!(XionErrorCode::ETREASURY001.message(), "Treasury not found");
        assert_eq!(XionErrorCode::ENETWORK001.message(), "Connection timeout");
    }

    #[test]
    fn test_error_code_hint() {
        assert_eq!(
            XionErrorCode::EAUTH001.hint(),
            "Run 'xion-toolkit auth login' first"
        );
        assert_eq!(
            XionErrorCode::ETREASURY001.hint(),
            "Run 'xion-toolkit treasury list' to see available treasuries"
        );
    }

    #[test]
    fn test_error_code_retryable() {
        // Network errors are retryable
        assert!(XionErrorCode::ENETWORK001.is_retryable());
        assert!(XionErrorCode::ENETWORK002.is_retryable());
        assert!(XionErrorCode::ENETWORK003.is_retryable());

        // Token expired is retryable (after refresh)
        assert!(XionErrorCode::EAUTH002.is_retryable());

        // Most other errors are not retryable
        assert!(!XionErrorCode::EAUTH001.is_retryable());
        assert!(!XionErrorCode::ETREASURY001.is_retryable());
    }

    #[test]
    fn test_error_detail_new() {
        let detail = ErrorDetail::new(XionErrorCode::ETREASURY001);
        assert_eq!(detail.code, XionErrorCode::ETREASURY001);
        assert_eq!(detail.message, "Treasury not found");
        assert!(!detail.retryable);
        assert!(detail.source.is_none());
    }

    #[test]
    fn test_error_detail_with_context() {
        let detail = ErrorDetail::with_context(XionErrorCode::ETREASURY001, "xion1abc123");
        assert_eq!(detail.message, "Treasury not found: xion1abc123");
        assert_eq!(
            detail.hint,
            "Run 'xion-toolkit treasury list' to see available treasuries"
        );
    }

    #[test]
    fn test_error_response() {
        let response =
            ErrorResponse::with_context(XionErrorCode::ETREASURY002, "Required: 1000000uxion");
        assert!(!response.success);
        assert_eq!(response.error.code, XionErrorCode::ETREASURY002);
        assert!(response.error.message.contains("Insufficient balance"));
    }

    #[test]
    fn test_error_response_display() {
        let response = ErrorResponse::new(XionErrorCode::EAUTH001);
        let display = format!("{}", response);
        assert!(display.contains("Error [EAUTH001]"));
        assert!(display.contains("Not authenticated"));
        assert!(display.contains("Hint:"));
    }

    #[test]
    fn test_xion_error_from_auth_error() {
        let auth_err = AuthError::NotAuthenticated("Please login".to_string());
        let xion_err: XionError = auth_err.into();
        assert_eq!(xion_err.code(), XionErrorCode::EAUTH001);
    }

    #[test]
    fn test_xion_error_from_network_error() {
        let net_err = NetworkError::Timeout("Request timed out".to_string());
        let xion_err: XionError = net_err.into();
        assert_eq!(xion_err.code(), XionErrorCode::ENETWORK001);
        assert!(xion_err.is_retryable());
    }

    #[test]
    fn test_xion_error_to_response() {
        let err = XionError::from(AuthError::TokenExpired("Access token expired".to_string()));
        let response = err.to_response();
        assert!(!response.success);
        assert_eq!(response.error.code, XionErrorCode::EAUTH002);
        assert!(response.error.retryable);
    }

    #[test]
    fn test_error_code_serialization() {
        let code = XionErrorCode::ETREASURY001;
        let json = serde_json::to_string(&code).unwrap();
        assert_eq!(json, "\"ETREASURY001\"");

        let decoded: XionErrorCode = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, code);
    }

    #[test]
    fn test_error_response_serialization() {
        let response = ErrorResponse::new(XionErrorCode::EAUTH001);
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":false"));
        assert!(json.contains("\"code\":\"EAUTH001\""));
        assert!(json.contains("\"retryable\":false"));
    }
}
