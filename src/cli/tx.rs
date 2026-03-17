//! Transaction CLI Commands
//!
//! Commands for monitoring transaction status and waiting for confirmation.

use anyhow::Result;
use clap::Subcommand;
use tracing::info;

use crate::cli::ExecuteContext;
use crate::config::ConfigManager;
use crate::shared::error::{TxError, XionError};
use crate::shared::exit_codes::exit_code;
use crate::tx::TxClient;
use crate::tx::TxStatus;
use crate::utils::output::{print_formatted, print_info};

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

pub async fn handle_command(cmd: TxCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        TxCommands::Status { hash } => handle_status(hash, ctx).await,
        TxCommands::Wait {
            hash,
            timeout,
            interval,
        } => handle_wait(hash, timeout, interval, ctx).await,
    }
}

async fn handle_status(hash: String, ctx: &ExecuteContext) -> Result<()> {
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
            print_formatted(&tx_info, ctx.output_format())
        }
        Ok(None) => {
            // This shouldn't happen with current implementation, but handle it anyway
            let result = serde_json::json!({
                "tx_hash": hash,
                "status": "pending"
            });
            print_formatted(&result, ctx.output_format())
        }
        Err(e) => {
            info!("Failed to query transaction: {}", e);
            let tx_error = TxError::QueryFailed(e.to_string());
            let xion_error: XionError = tx_error.into();
            Err(xion_error.into())
        }
    }
}

async fn handle_wait(
    hash: String,
    timeout: u64,
    interval: u64,
    ctx: &ExecuteContext,
) -> Result<()> {
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
                    print_formatted(&result, ctx.output_format())
                }
                TxStatus::Failed => {
                    eprintln!("[INFO] Transaction failed!");
                    if let Some(ref tx_info) = result.tx_info {
                        if let Some(ref error) = tx_info.error {
                            eprintln!("[INFO] Error: {}", error);
                        }
                    }
                    print_formatted(&result, ctx.output_format())?;
                    std::process::exit(exit_code::TX_WAIT_FAILED);
                }
                TxStatus::Timeout => {
                    eprintln!("[INFO] Timeout waiting for transaction confirmation");
                    print_formatted(&result, ctx.output_format())?;
                    std::process::exit(exit_code::TX_TIMEOUT);
                }
                _ => print_formatted(&result, ctx.output_format()),
            }
        }
        Err(e) => {
            info!("Failed to wait for transaction: {}", e);
            let tx_error = TxError::WaitFailed(e.to_string());
            let xion_error: XionError = tx_error.into();
            Err(xion_error.into())
        }
    }
}
