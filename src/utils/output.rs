//! Output utilities for CLI responses
//!
//! This module provides consistent output formatting for both
//! machine-readable (JSON) and human-readable outputs.
//!
//! # Output Formats
//!
//! - `json`: Pretty-printed JSON (default for agents)
//! - `json-compact`: Single-line JSON for CI/CD pipelines
//! - `github-actions`: GitHub Actions workflow commands format

use anyhow::Result;
use serde::Serialize;
use std::str::FromStr;

use crate::shared::error::{
    AuthError, ErrorResponse, NetworkError, TreasuryError, XionError, XionErrorCode,
};

/// Output format for CLI responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// JSON output, pretty-printed (default for agents)
    #[default]
    Json,
    /// Compact JSON output (minimal, single-line for CI/CD)
    JsonCompact,
    /// GitHub Actions workflow commands format
    GitHubActions,
    /// Human-readable output (default for interactive use)
    Human,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "json-compact" => Ok(OutputFormat::JsonCompact),
            "github-actions" | "github" | "gha" => Ok(OutputFormat::GitHubActions),
            "human" | "text" => Ok(OutputFormat::Human),
            _ => Err(format!(
                "Invalid output format '{}'. Valid options: json, json-compact, github-actions, human",
                s
            )),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::JsonCompact => write!(f, "json-compact"),
            OutputFormat::GitHubActions => write!(f, "github-actions"),
            OutputFormat::Human => write!(f, "human"),
        }
    }
}

/// Print JSON to stdout (for Agent consumption)
pub fn print_json<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}

/// Print compact JSON to stdout (for CI/CD pipelines)
pub fn print_json_compact<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_string(data)?;
    println!("{}", json);
    Ok(())
}

/// Print GitHub Actions workflow commands to stdout
///
/// GitHub Actions format:
/// - Success: `::notice::{message}`
/// - Warning: `::warning::{message}`
/// - Error: `::error::{message}`
/// - Output: `::set-output name={key}::{value}`
pub fn print_github_actions<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_value(data)?;

    // Check if this is a success or error response
    let is_success = json
        .get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    if is_success {
        // Extract key information for notice
        if let Some(data_obj) = json.get("data") {
            // Output main data
            println!("::notice::{}", serde_json::to_string(data_obj)?);
            // Set outputs for key fields
            if let Some(obj) = data_obj.as_object() {
                for (key, value) in obj {
                    if let Some(str_val) = value.as_str() {
                        println!("::set-output name={}::{}", key, str_val);
                    } else if !value.is_null() {
                        println!("::set-output name={}::{}", key, value);
                    }
                }
            }
        } else {
            // No data wrapper, output as notice
            println!("::notice::{}", serde_json::to_string(&json)?);
        }
    } else {
        // Error response
        if let Some(error) = json.get("error") {
            let code = error
                .get("code")
                .and_then(|v| v.as_str())
                .unwrap_or("UNKNOWN");
            let message = error
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("An error occurred");
            let hint = error.get("hint").and_then(|v| v.as_str()).unwrap_or("");

            println!("::error title={},file=cli::{}", code, message);
            if !hint.is_empty() {
                println!("::warning::Hint: {}", hint);
            }
        }
    }

    Ok(())
}

/// Print output in the specified format
pub fn print_formatted<T: Serialize>(data: &T, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => print_json(data),
        OutputFormat::JsonCompact => print_json_compact(data),
        OutputFormat::GitHubActions => print_github_actions(data),
        OutputFormat::Human => print_json(data), // Human falls back to pretty JSON
    }
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
        OutputFormat::JsonCompact => {
            let response = error.to_response();
            print_json_compact(&response)
        }
        OutputFormat::GitHubActions => {
            let response = error.to_response();
            print_github_actions(&response)
        }
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

    /// Create a compact JSON output handler
    pub fn json_compact() -> Self {
        Self::new(OutputFormat::JsonCompact)
    }

    /// Create a GitHub Actions output handler
    pub fn github_actions() -> Self {
        Self::new(OutputFormat::GitHubActions)
    }

    /// Create a human-readable output handler
    pub fn human() -> Self {
        Self::new(OutputFormat::Human)
    }

    /// Print a success result
    pub fn success<T: Serialize>(&self, data: &T) -> Result<()> {
        let wrapper = serde_json::json!({
            "success": true,
            "data": data
        });
        match self.format {
            OutputFormat::Json => print_json(&wrapper),
            OutputFormat::JsonCompact => print_json_compact(&wrapper),
            OutputFormat::GitHubActions => print_github_actions(&wrapper),
            OutputFormat::Human => print_json(&wrapper),
        }
    }

    /// Print raw data without success wrapper
    pub fn raw<T: Serialize>(&self, data: &T) -> Result<()> {
        match self.format {
            OutputFormat::Json => print_json(data),
            OutputFormat::JsonCompact => print_json_compact(data),
            OutputFormat::GitHubActions => {
                print_github_actions(&serde_json::json!({"success": true, "data": data}))
            }
            OutputFormat::Human => print_json(data),
        }
    }

    /// Print a success message
    pub fn success_message(&self, message: &str) -> Result<()> {
        let wrapper = serde_json::json!({
            "success": true,
            "message": message
        });
        match self.format {
            OutputFormat::Json => print_json(&wrapper),
            OutputFormat::JsonCompact => print_json_compact(&wrapper),
            OutputFormat::GitHubActions => {
                println!("::notice::{}", message);
                Ok(())
            }
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
            OutputFormat::Json | OutputFormat::JsonCompact | OutputFormat::GitHubActions => {
                print_error_response_with_context(code, context)
            }
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

    /// Get the current output format
    pub fn format(&self) -> OutputFormat {
        self.format
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
    fn test_output_format_from_str() {
        assert_eq!(OutputFormat::from_str("json").unwrap(), OutputFormat::Json);
        assert_eq!(
            OutputFormat::from_str("json-compact").unwrap(),
            OutputFormat::JsonCompact
        );
        assert_eq!(
            OutputFormat::from_str("github-actions").unwrap(),
            OutputFormat::GitHubActions
        );
        assert_eq!(
            OutputFormat::from_str("github").unwrap(),
            OutputFormat::GitHubActions
        );
        assert_eq!(
            OutputFormat::from_str("gha").unwrap(),
            OutputFormat::GitHubActions
        );
        assert_eq!(
            OutputFormat::from_str("human").unwrap(),
            OutputFormat::Human
        );
        assert_eq!(OutputFormat::from_str("text").unwrap(), OutputFormat::Human);
    }

    #[test]
    fn test_output_format_from_str_invalid() {
        assert!(OutputFormat::from_str("invalid").is_err());
        assert!(OutputFormat::from_str("xml").is_err());
    }

    #[test]
    fn test_output_format_display() {
        assert_eq!(format!("{}", OutputFormat::Json), "json");
        assert_eq!(format!("{}", OutputFormat::JsonCompact), "json-compact");
        assert_eq!(format!("{}", OutputFormat::GitHubActions), "github-actions");
        assert_eq!(format!("{}", OutputFormat::Human), "human");
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
        assert_eq!(output.format(), OutputFormat::Json);
    }

    #[test]
    fn test_cli_output_json_compact() {
        let output = CliOutput::json_compact();
        assert_eq!(output.format(), OutputFormat::JsonCompact);
    }

    #[test]
    fn test_cli_output_github_actions() {
        let output = CliOutput::github_actions();
        assert_eq!(output.format(), OutputFormat::GitHubActions);
    }

    #[test]
    fn test_cli_output_human() {
        let output = CliOutput::human();
        assert_eq!(output.format(), OutputFormat::Human);
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

    #[test]
    fn test_print_json_compact_output() {
        let data = serde_json::json!({
            "key": "value",
            "nested": {
                "inner": 123
            }
        });
        let json = serde_json::to_string(&data).unwrap();
        // Compact JSON should be single line
        assert!(!json.contains('\n'));
    }

    #[test]
    fn test_print_github_actions_success() {
        let data = serde_json::json!({
            "success": true,
            "data": {
                "address": "xion1test",
                "balance": "1000000"
            }
        });
        // Should not panic
        let result = print_github_actions(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_github_actions_error() {
        let data = serde_json::json!({
            "success": false,
            "error": {
                "code": "EAUTH001",
                "message": "Not authenticated",
                "hint": "Run auth login"
            }
        });
        // Should not panic
        let result = print_github_actions(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_formatted_json() {
        let data = serde_json::json!({"test": "value"});
        let result = print_formatted(&data, OutputFormat::Json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_formatted_compact() {
        let data = serde_json::json!({"test": "value"});
        let result = print_formatted(&data, OutputFormat::JsonCompact);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_formatted_github_actions() {
        let data = serde_json::json!({"success": true, "data": {"key": "value"}});
        let result = print_formatted(&data, OutputFormat::GitHubActions);
        assert!(result.is_ok());
    }
}
