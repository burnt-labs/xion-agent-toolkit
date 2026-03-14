//! Batch CLI Commands
//!
//! Command-line interface for batch operations.

use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

use crate::batch::{BatchExecutor, BatchRequest, BatchValidationResult};

#[derive(Subcommand)]
pub enum BatchCommands {
    /// Execute a batch transaction from a JSON file
    Execute(ExecuteArgs),

    /// Validate a batch file without executing it
    Validate(ValidateArgs),
}

/// Arguments for batch execute command
#[derive(Debug, Args)]
pub struct ExecuteArgs {
    /// Path to JSON file containing the batch request
    #[arg(short, long, value_name = "FILE")]
    pub from_file: PathBuf,

    /// Simulate the batch without executing (dry-run)
    #[arg(long)]
    pub simulate: bool,

    /// Transaction memo
    #[arg(short, long)]
    pub memo: Option<String>,
}

/// Arguments for batch validate command
#[derive(Debug, Args)]
pub struct ValidateArgs {
    /// Path to JSON file containing the batch request
    #[arg(short, long, value_name = "FILE")]
    pub from_file: PathBuf,
}

pub async fn handle_command(cmd: BatchCommands) -> Result<()> {
    match cmd {
        BatchCommands::Execute(args) => handle_execute(args).await,
        BatchCommands::Validate(args) => handle_validate(args).await,
    }
}

async fn handle_execute(args: ExecuteArgs) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Loading batch from: {:?}", args.from_file));

    // Load batch request from file
    let mut request = BatchRequest::from_file(&args.from_file)?;

    // Override memo if provided
    if let Some(ref memo) = args.memo {
        request.memo = Some(memo.clone());
    }

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let executor = BatchExecutor::new(oauth_client, network_config.clone());

    // Check authentication
    if !executor.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Handle simulation mode
    if args.simulate {
        print_info("Simulating batch transaction...");

        match executor.simulate(&request).await {
            Ok(sim_result) => {
                let result = serde_json::json!({
                    "success": true,
                    "simulation": true,
                    "valid": sim_result.valid,
                    "message_count": sim_result.message_count,
                    "from": sim_result.from,
                    "gas_estimate": sim_result.gas_estimate,
                    "message_types": sim_result.message_types
                });
                return print_json(&result);
            }
            Err(e) => {
                let result = serde_json::json!({
                    "success": false,
                    "error": format!("Simulation failed: {}", e),
                    "code": "SIMULATION_FAILED"
                });
                return print_json(&result);
            }
        }
    }

    // Execute the batch
    print_info(&format!(
        "Executing batch with {} messages...",
        request.messages.len()
    ));

    match executor.execute(&request).await {
        Ok(batch_result) => {
            let result = serde_json::json!({
                "success": batch_result.success,
                "tx_hash": batch_result.tx_hash,
                "from": batch_result.from,
                "gas_used": batch_result.gas_used,
                "gas_wanted": batch_result.gas_wanted,
                "message_count": batch_result.message_count
            });
            print_json(&result)
        }
        Err(e) => {
            let (code, suggestion) = match &e {
                crate::batch::BatchExecutorError::NotAuthenticated => {
                    ("NOT_AUTHENTICATED", "Please run 'xion auth login' first.")
                }
                crate::batch::BatchExecutorError::ValidationFailed(_) => {
                    ("BATCH_VALIDATION_FAILED", "Check your batch file format.")
                }
                crate::batch::BatchExecutorError::BroadcastFailed(_) => {
                    ("TX_FAILED", "Check the error message for details.")
                }
                crate::batch::BatchExecutorError::NetworkError(_) => {
                    ("NETWORK_ERROR", "Check your network connection.")
                }
            };

            let result = serde_json::json!({
                "success": false,
                "error": format!("Batch execution failed: {}", e),
                "code": code,
                "suggestion": suggestion
            });
            print_json(&result)
        }
    }
}

async fn handle_validate(args: ValidateArgs) -> Result<()> {
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Validating batch from: {:?}", args.from_file));

    // Load batch request from file
    let request: BatchRequest = match BatchRequest::from_file(&args.from_file) {
        Ok(req) => req,
        Err(e) => {
            let result = serde_json::json!({
                "valid": false,
                "error": format!("Failed to load batch file: {}", e),
                "code": "FILE_LOAD_ERROR"
            });
            return print_json(&result);
        }
    };

    // Validate the batch request directly (no OAuth or network config needed)
    let result = match request.validate() {
        Ok(()) => BatchValidationResult {
            valid: true,
            message_count: request.messages.len(),
            errors: Vec::new(),
            message_types: request
                .messages
                .iter()
                .map(|m| m.type_url.clone())
                .collect(),
        },
        Err(e) => BatchValidationResult {
            valid: false,
            message_count: request.messages.len(),
            errors: vec![e.to_string()],
            message_types: request
                .messages
                .iter()
                .map(|m| m.type_url.clone())
                .collect(),
        },
    };

    print_json(&result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_batch_file() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        let content = r#"{
            "messages": [
                {
                    "typeUrl": "/cosmos.bank.v1beta1.MsgSend",
                    "value": {
                        "toAddress": "xion1abc",
                        "amount": [{ "denom": "uxion", "amount": "1000000" }]
                    }
                }
            ],
            "memo": "Test batch"
        }"#;
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_execute_args_parsing() {
        let file = create_test_batch_file();

        let args = ExecuteArgs {
            from_file: file.path().to_path_buf(),
            simulate: false,
            memo: Some("Custom memo".to_string()),
        };

        assert!(!args.simulate);
        assert_eq!(args.memo, Some("Custom memo".to_string()));
    }

    #[test]
    fn test_validate_args_parsing() {
        let file = create_test_batch_file();

        let args = ValidateArgs {
            from_file: file.path().to_path_buf(),
        };

        assert!(args.from_file.exists());
    }

    #[test]
    fn test_batch_request_from_file() {
        let file = create_test_batch_file();
        let request = BatchRequest::from_file(file.path()).unwrap();

        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.memo, Some("Test batch".to_string()));
    }

    #[test]
    fn test_batch_request_invalid_file() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"not valid json").unwrap();

        let result = BatchRequest::from_file(file.path());
        assert!(result.is_err());
    }
}
