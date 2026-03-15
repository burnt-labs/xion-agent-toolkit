//! Transaction CLI Commands
//!
//! Commands for monitoring transaction status and waiting for confirmation.

use anyhow::Result;
use clap::Subcommand;
use tracing::info;

use crate::config::ConfigManager;
use crate::shared::error::{TxError, XionError};
use crate::tx::TxClient;
use crate::tx::TxStatus;
use crate::utils::output::{print_error_response, print_info, print_json};

#[derive(Subcommand)]
pub enum TxCommands {
    /// Check transaction status
    Status {
        /// Transaction hash (hex format, with or without 0x prefix)
        hash: String,
    },
    /// Wait for transaction confirmation
    Wait {
        /// Transaction hash (hex format, with or without 0x prefix)
        hash: String,
        /// Maximum time to wait in seconds
        #[arg(long, default_value = "60")]
        timeout: u64,
        /// Polling interval in seconds
        #[arg(long, default_value = "2")]
        interval: u64,
    },
}

pub async fn handle_command(cmd: TxCommands) -> Result<()> {
    match cmd {
        TxCommands::Status { hash } => handle_status(hash).await,
        TxCommands::Wait {
            hash,
            timeout,
            interval,
        } => handle_wait(hash, timeout, interval).await,
    }
}

async fn handle_status(hash: String) -> Result<()> {
    print_info(&format!("Querying transaction status for: {}", hash));

    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;

    info!(
        "Using RPC endpoint: {} for network: {}",
        network_config.rpc_url,
        config_manager.get_current_network()
    );

    let tx_client = TxClient::new(network_config.rpc_url);

    match tx_client.get_tx(&hash).await {
        Ok(Some(tx_info)) => {
            info!(
                "Transaction status retrieved: {:?} at height {:?}",
                tx_info.status, tx_info.height
            );
            print_json(&tx_info)?;
            Ok(())
        }
        Ok(None) => {
            // This shouldn't happen with current implementation, but handle it anyway
            let result = serde_json::json!({
                "tx_hash": hash,
                "status": "pending"
            });
            print_json(&result)?;
            Ok(())
        }
        Err(e) => {
            info!("Failed to query transaction: {}", e);
            let tx_error = TxError::QueryFailed(e.to_string());
            let xion_error: XionError = tx_error.into();
            print_error_response(&xion_error)?;
            std::process::exit(1);
        }
    }
}

async fn handle_wait(hash: String, timeout: u64, interval: u64) -> Result<()> {
    print_info(&format!(
        "Waiting for transaction: {} (timeout: {}s, interval: {}s)",
        hash, timeout, interval
    ));

    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;

    info!(
        "Using RPC endpoint: {} for network: {}",
        network_config.rpc_url,
        config_manager.get_current_network()
    );

    let tx_client = TxClient::new(network_config.rpc_url);

    // Print progress indicator to stderr
    eprintln!("[INFO] Polling for transaction confirmation...");

    match tx_client.wait_tx(&hash, timeout, interval).await {
        Ok(result) => {
            info!(
                "Wait completed with status: {:?} after {}ms",
                result.status, result.wait_time_ms
            );

            // Print summary to stderr for human consumption
            match result.status {
                TxStatus::Success => {
                    eprintln!("[INFO] Transaction confirmed successfully!");
                    if let Some(ref tx_info) = result.tx_info {
                        eprintln!("[INFO] Block height: {}", tx_info.height.unwrap_or(0));
                        eprintln!("[INFO] Gas used: {}", tx_info.gas_used.unwrap_or(0));
                    }
                    print_json(&result)?;
                    Ok(())
                }
                TxStatus::Failed => {
                    eprintln!("[INFO] Transaction failed!");
                    if let Some(ref tx_info) = result.tx_info {
                        if let Some(ref error) = tx_info.error {
                            eprintln!("[INFO] Error: {}", error);
                        }
                    }
                    print_json(&result)?;
                    std::process::exit(1);
                }
                TxStatus::Timeout => {
                    eprintln!("[INFO] Timeout waiting for transaction confirmation");
                    print_json(&result)?;
                    std::process::exit(1);
                }
                _ => {
                    print_json(&result)?;
                    Ok(())
                }
            }
        }
        Err(e) => {
            info!("Failed to wait for transaction: {}", e);
            let tx_error = TxError::WaitFailed(e.to_string());
            let xion_error: XionError = tx_error.into();
            print_error_response(&xion_error)?;
            std::process::exit(1);
        }
    }
}
