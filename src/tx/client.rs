//! Transaction Client
//!
//! Client for querying transaction status from Xion RPC endpoints.

use anyhow::{Context, Result};
use reqwest::Client;
use tokio::time::{sleep, Duration, Instant};
use tracing::{debug, info, instrument};

use super::types::{RpcTxResponse, TxInfo, TxStatus, TxWaitResult};

/// Client for querying transaction status from RPC
#[derive(Debug, Clone)]
pub struct TxClient {
    /// RPC endpoint URL
    rpc_url: String,
    /// HTTP client for making requests
    http_client: Client,
}

impl TxClient {
    /// Create a new transaction client
    ///
    /// # Arguments
    /// * `rpc_url` - Base URL of the RPC endpoint (e.g., "https://rpc.xion-testnet-2.burnt.com:443")
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::tx::TxClient;
    ///
    /// let client = TxClient::new("https://rpc.xion-testnet-2.burnt.com:443".to_string());
    /// ```
    pub fn new(rpc_url: String) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            rpc_url,
            http_client,
        }
    }

    /// Get transaction status from RPC
    ///
    /// Queries the RPC endpoint for transaction information.
    /// Returns `Ok(None)` if the transaction is not found (pending).
    ///
    /// # Arguments
    /// * `hash` - Transaction hash (hex format, with or without 0x prefix)
    ///
    /// # Returns
    /// * `Ok(Some(TxInfo))` - Transaction found with status information
    /// * `Ok(None)` - Transaction not found (still pending)
    /// * `Err(_)` - RPC query failed
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::tx::TxClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TxClient::new("https://rpc.xion-testnet-2.burnt.com:443".to_string());
    /// match client.get_tx("ABC123...").await? {
    ///     Some(tx_info) => println!("Transaction status: {:?}", tx_info.status),
    ///     None => println!("Transaction not found (pending)"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, hash))]
    pub async fn get_tx(&self, hash: &str) -> Result<Option<TxInfo>> {
        let normalized_hash = normalize_tx_hash(hash);
        let url = format!("{}/tx?hash=0x{}", self.rpc_url, normalized_hash);

        debug!("Querying transaction status from RPC: {}", url);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context("Failed to send transaction query request")?;

        let status = response.status();
        debug!("Transaction query response status: {}", status);

        // Handle 404 or empty response as "not found" (pending)
        if status == reqwest::StatusCode::NOT_FOUND {
            debug!("Transaction not found (404): {}", normalized_hash);
            return Ok(Some(TxInfo::pending(&normalized_hash)));
        }

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Check for specific error patterns that indicate "not found"
            if error_text.contains("not found") || error_text.contains("transaction not found") {
                debug!("Transaction not found: {}", normalized_hash);
                return Ok(Some(TxInfo::pending(&normalized_hash)));
            }

            anyhow::bail!("Transaction query failed: HTTP {} - {}", status, error_text);
        }

        let body = response
            .text()
            .await
            .context("Failed to read transaction response body")?;

        // Check if body is empty
        if body.trim().is_empty() {
            debug!("Transaction response body empty: {}", normalized_hash);
            return Ok(Some(TxInfo::pending(&normalized_hash)));
        }

        debug!("Transaction response body: {}", body);

        // Parse the RPC response
        let rpc_response: RpcTxResponse =
            serde_json::from_str(&body).context("Failed to parse transaction response")?;

        let tx_info = self.parse_tx_response(&normalized_hash, rpc_response)?;

        info!(
            "Transaction {} status: {:?}",
            normalized_hash, tx_info.status
        );

        Ok(Some(tx_info))
    }

    /// Parse RPC response into TxInfo
    fn parse_tx_response(&self, hash: &str, response: RpcTxResponse) -> Result<TxInfo> {
        // If tx_result is missing, transaction is pending
        let tx_result = match response.tx_result {
            Some(result) => result,
            None => {
                debug!("No tx_result in response, treating as pending");
                return Ok(TxInfo::pending(hash));
            }
        };

        // Parse height
        let height = response.height.as_ref().and_then(|h| h.parse::<u64>().ok());

        // Parse gas_used
        let gas_used = tx_result
            .gas_used
            .as_ref()
            .and_then(|g| g.parse::<u64>().ok());

        // Determine status based on code
        let (status, error) = if tx_result.code == 0 {
            (TxStatus::Success, None)
        } else {
            let error_msg = tx_result
                .error
                .clone()
                .or_else(|| {
                    // Try to extract error from log if present
                    tx_result.log.as_ref().and_then(|log| {
                        if log.contains("error") {
                            Some(log.clone())
                        } else {
                            None
                        }
                    })
                })
                .unwrap_or_else(|| format!("Transaction failed with code {}", tx_result.code));

            (TxStatus::Failed, Some(error_msg))
        };

        // Create timestamp from current time (RPC doesn't always provide this)
        let timestamp = Some(chrono::Utc::now().to_rfc3339());

        Ok(TxInfo {
            tx_hash: hash.to_string(),
            status,
            height,
            timestamp,
            gas_used,
            error,
        })
    }

    /// Wait for transaction to be confirmed
    ///
    /// Polls the RPC endpoint until the transaction is found and in a final state,
    /// or until the timeout is reached.
    ///
    /// # Arguments
    /// * `hash` - Transaction hash (hex format, with or without 0x prefix)
    /// * `timeout` - Maximum time to wait in seconds
    /// * `interval` - Polling interval in seconds
    ///
    /// # Returns
    /// `TxWaitResult` containing the final status and timing information
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::tx::TxClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TxClient::new("https://rpc.xion-testnet-2.burnt.com:443".to_string());
    /// let result = client.wait_tx("ABC123...", 60, 2).await?;
    /// println!("Wait result: {:?}", result.status);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, hash))]
    pub async fn wait_tx(&self, hash: &str, timeout: u64, interval: u64) -> Result<TxWaitResult> {
        let normalized_hash = normalize_tx_hash(hash);
        let timeout_duration = Duration::from_secs(timeout);
        let interval_duration = Duration::from_secs(interval);

        info!(
            "Waiting for transaction {} (timeout: {}s, interval: {}s)",
            normalized_hash, timeout, interval
        );

        let start_time = Instant::now();

        loop {
            let elapsed = start_time.elapsed();

            // Check if timeout exceeded
            if elapsed >= timeout_duration {
                info!(
                    "Timeout waiting for transaction {} after {}ms",
                    normalized_hash,
                    elapsed.as_millis()
                );
                return Ok(TxWaitResult::timeout(elapsed.as_millis() as u64));
            }

            // Query transaction status
            match self.get_tx(&normalized_hash).await {
                Ok(Some(tx_info)) => {
                    debug!(
                        "Transaction {} status: {:?}",
                        normalized_hash, tx_info.status
                    );

                    // Check if transaction is in a final state
                    if tx_info.status.is_final() {
                        let wait_time_ms = elapsed.as_millis() as u64;

                        info!(
                            "Transaction {} reached final state: {:?} after {}ms",
                            normalized_hash, tx_info.status, wait_time_ms
                        );

                        return Ok(match tx_info.status {
                            TxStatus::Success => TxWaitResult::success(tx_info, wait_time_ms),
                            TxStatus::Failed => TxWaitResult::failed(tx_info, wait_time_ms),
                            _ => TxWaitResult::timeout(wait_time_ms),
                        });
                    }
                }
                Ok(None) => {
                    debug!("Transaction {} not yet found", normalized_hash);
                }
                Err(e) => {
                    debug!("Error querying transaction {}: {}", normalized_hash, e);
                    // Continue polling on error, don't fail immediately
                }
            }

            // Wait before next poll
            sleep(interval_duration).await;
        }
    }

    /// Get the RPC URL
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }
}

/// Normalize transaction hash by removing 0x prefix and converting to lowercase
fn normalize_tx_hash(hash: &str) -> String {
    let hash = hash.trim();
    let hash = hash.strip_prefix("0x").unwrap_or(hash);
    let hash = hash.strip_prefix("0X").unwrap_or(hash);
    hash.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_tx_hash() {
        assert_eq!(normalize_tx_hash("ABC123"), "abc123");
        assert_eq!(normalize_tx_hash("0xABC123"), "abc123");
        assert_eq!(normalize_tx_hash("0XABC123"), "abc123");
        assert_eq!(normalize_tx_hash("  ABC123  "), "abc123");
    }

    #[test]
    fn test_tx_client_creation() {
        let client = TxClient::new("https://rpc.xion-testnet-2.burnt.com:443".to_string());
        assert_eq!(client.rpc_url(), "https://rpc.xion-testnet-2.burnt.com:443");
    }

    // Note: Integration tests with mock server would require wiremock
    // and would be placed in tests/ directory or as async integration tests
}
