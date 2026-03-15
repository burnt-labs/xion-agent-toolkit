//! Output utilities for CLI responses
//!
//! This module provides consistent output formatting for both
//! machine-readable (JSON) and human-readable outputs.

use anyhow::Result;
use serde::Serialize;

use crate::shared::error::{
    AuthError, ErrorResponse, NetworkError, TreasuryError, XionError, XionErrorCode,
};

/// Output format for CLI responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// JSON output (default for scripts/agents)
    #[default]
    Json,
    /// Human-readable output (default for interactive use)
    Human,
}

/// Print JSON to stdout (for Agent consumption)
pub fn print_json<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}

/// Print info message to stderr (for human consumption)
pub fn print_info(message: &str) {
    eprintln!("[INFO] {}", message);
}

/// Print error message to stderr (for human consumption)
#[allow(dead_code)]
pub fn print_error_message(message: &str) {
    eprintln!("[ERROR] {}", message);
}

/// Print warning message to stderr (for human consumption)
#[allow(dead_code)]
pub fn print_warning(message: &str) {
    eprintln!("[WARNING] {}", message);
}

/// Print a structured error response to stdout (JSON)
pub fn print_error_response(error: &XionError) -> Result<()> {
    let response = error.to_response();
    print_json(&response)
}

/// Print a structured error response with custom message
pub fn print_error_response_with_context(code: XionErrorCode, context: &str) -> Result<()> {
    let response = ErrorResponse::with_context(code, context);
    print_json(&response)
}

/// Print a success response to stdout (JSON)
pub fn print_success<T: Serialize>(data: &T) -> Result<()> {
    let success_wrapper = serde_json::json!({
        "success": true,
        "data": data
    });
    print_json(&success_wrapper)
}

/// Print a simple success message
pub fn print_success_message(message: &str) -> Result<()> {
    let success_wrapper = serde_json::json!({
        "success": true,
        "message": message
    });
    print_json(&success_wrapper)
}

/// Print an error for human consumption (to stderr)
pub fn print_error_human(error: &XionError) {
    let response = error.to_response();
    eprintln!("{}", response);
}

/// Print an error for JSON consumption (to stdout)
pub fn print_error_json(error: &XionError) -> Result<()> {
    print_error_response(error)
}

/// Print error based on output format
pub fn print_error(error: &XionError, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => print_error_json(error),
        OutputFormat::Human => {
            print_error_human(error);
            Ok(())
        }
    }
}

/// Wrapper for CLI output that handles both success and error cases
pub struct CliOutput {
    format: OutputFormat,
}

impl CliOutput {
    /// Create a new CLI output handler
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    /// Create a JSON output handler
    pub fn json() -> Self {
        Self::new(OutputFormat::Json)
    }

    /// Create a human-readable output handler
    pub fn human() -> Self {
        Self::new(OutputFormat::Human)
    }

    /// Print a success result
    pub fn success<T: Serialize>(&self, data: &T) -> Result<()> {
        match self.format {
            OutputFormat::Json => print_success(data),
            OutputFormat::Human => {
                print_json(data)?;
                Ok(())
            }
        }
    }

    /// Print a success message
    pub fn success_message(&self, message: &str) -> Result<()> {
        match self.format {
            OutputFormat::Json => print_success_message(message),
            OutputFormat::Human => {
                println!("{}", message);
                Ok(())
            }
        }
    }

    /// Print an error
    pub fn error(&self, error: &XionError) -> Result<()> {
        print_error(error, self.format)
    }

    /// Print an error with custom code and context
    pub fn error_with_context(&self, code: XionErrorCode, context: &str) -> Result<()> {
        match self.format {
            OutputFormat::Json => print_error_response_with_context(code, context),
            OutputFormat::Human => {
                let response = ErrorResponse::with_context(code, context);
                eprintln!("{}", response);
                Ok(())
            }
        }
    }

    /// Print info message (always to stderr)
    pub fn info(&self, message: &str) {
        print_info(message);
    }

    /// Print warning message (always to stderr)
    pub fn warning(&self, message: &str) {
        print_warning(message);
    }
}

/// Convert an anyhow error to a Xion error for output
pub fn anyhow_to_xion_error(err: &anyhow::Error) -> XionError {
    // Check for common error patterns in the message
    let err_str = err.to_string().to_lowercase();

    if err_str.contains("not authenticated") || err_str.contains("no credentials") {
        return XionError::from(AuthError::NotAuthenticated(err.to_string()));
    }

    if err_str.contains("token expired") {
        return XionError::from(AuthError::TokenExpired(err.to_string()));
    }

    if err_str.contains("timeout") {
        return XionError::from(NetworkError::Timeout(err.to_string()));
    }

    if err_str.contains("connection") {
        return XionError::from(NetworkError::ConnectionRefused(err.to_string()));
    }

    if err_str.contains("treasury not found") {
        return XionError::from(TreasuryError::NotFound(err.to_string()));
    }

    // Default to a generic error
    XionError::Generic {
        code: XionErrorCode::ECONFIG002,
        message: err.to_string(),
        hint: "Check the error message for details".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::error::AuthError;

    #[test]
    fn test_output_format_default() {
        assert_eq!(OutputFormat::default(), OutputFormat::Json);
    }

    #[test]
    fn test_error_response_serialization() {
        let error = XionError::from(AuthError::NotAuthenticated("test".to_string()));
        let response = error.to_response();
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":false"));
        assert!(json.contains("\"code\":\"EAUTH001\""));
    }

    #[test]
    fn test_cli_output_json() {
        let output = CliOutput::json();
        assert_eq!(output.format, OutputFormat::Json);
    }

    #[test]
    fn test_cli_output_human() {
        let output = CliOutput::human();
        assert_eq!(output.format, OutputFormat::Human);
    }

    #[test]
    fn test_anyhow_to_xion_error_auth() {
        let err = anyhow::anyhow!("No credentials found for network 'testnet'");
        let xion_err = anyhow_to_xion_error(&err);
        assert_eq!(xion_err.code(), XionErrorCode::EAUTH001);
    }

    #[test]
    fn test_anyhow_to_xion_error_timeout() {
        let err = anyhow::anyhow!("Connection timeout after 30s");
        let xion_err = anyhow_to_xion_error(&err);
        assert_eq!(xion_err.code(), XionErrorCode::ENETWORK001);
        assert!(xion_err.is_retryable());
    }

    #[test]
    fn test_anyhow_to_xion_error_treasury() {
        let err = anyhow::anyhow!("Treasury not found: xion1abc123");
        let xion_err = anyhow_to_xion_error(&err);
        assert_eq!(xion_err.code(), XionErrorCode::ETREASURY001);
    }

    #[test]
    fn test_success_wrapper() {
        let data = serde_json::json!({
            "address": "xion1test",
            "balance": "1000000"
        });
        let wrapper = serde_json::json!({
            "success": true,
            "data": data
        });
        let json = serde_json::to_string(&wrapper).unwrap();
        assert!(json.contains("\"success\":true"));
    }
}
