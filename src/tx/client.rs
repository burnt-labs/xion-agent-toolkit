//! Transaction Client
//!
//! Client for querying transaction status from Xion RPC endpoints.

use anyhow::{Context, Result};
use reqwest::Client;
use tokio::time::{sleep, Duration, Instant};
use tracing::{debug, info, instrument};

use super::types::{CosmosTxResponse, TxInfo, TxStatus, TxWaitResult};

/// Client for querying transaction status from REST API
#[derive(Debug, Clone)]
pub struct TxClient {
    /// REST endpoint URL
    rest_url: String,
    /// HTTP client for making requests
    http_client: Client,
}

impl TxClient {
    /// Create a new transaction client
    ///
    /// # Arguments
    /// * `rest_url` - Base URL of the REST endpoint (e.g., "https://api.xion-testnet-2.burnt.com")
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::tx::TxClient;
    ///
    /// let client = TxClient::new("https://api.xion-testnet-2.burnt.com".to_string());
    /// ```
    pub fn new(rest_url: String) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            rest_url,
            http_client,
        }
    }

    /// Get transaction status from REST API
    ///
    /// Queries the REST endpoint for transaction information.
    /// Returns `Ok(None)` if the transaction is not found (pending).
    ///
    /// # Arguments
    /// * `hash` - Transaction hash (hex format, with or without 0x prefix)
    ///
    /// # Returns
    /// * `Ok(Some(TxInfo))` - Transaction found with status information
    /// * `Ok(None)` - Transaction not found (still pending)
    /// * `Err(_)` - REST query failed
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::tx::TxClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TxClient::new("https://api.xion-testnet-2.burnt.com".to_string());
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
        // Use Cosmos SDK REST API endpoint (not Tendermint RPC)
        // Format: /cosmos/tx/v1beta1/txs/{hash}
        let url = format!(
            "{}/cosmos/tx/v1beta1/txs/{}",
            self.rest_url, normalized_hash
        );

        debug!("Querying transaction status from REST API: {}", url);

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

        // Parse the Cosmos SDK REST API response
        let cosmos_response: CosmosTxResponse =
            serde_json::from_str(&body).context("Failed to parse transaction response")?;

        let tx_info = self.parse_cosmos_tx_response(&normalized_hash, cosmos_response)?;

        info!(
            "Transaction {} status: {:?}",
            normalized_hash, tx_info.status
        );

        Ok(Some(tx_info))
    }

    /// Parse Cosmos SDK REST API response into TxInfo
    fn parse_cosmos_tx_response(&self, hash: &str, response: CosmosTxResponse) -> Result<TxInfo> {
        let tx_response = response.tx_response;

        // Parse height from string to u64
        let height = tx_response.height.parse::<u64>().ok();

        // Parse gas_used from string to u64
        let gas_used = tx_response
            .gas_used
            .as_ref()
            .and_then(|g| g.parse::<u64>().ok());

        // Determine status based on code
        let (status, error) = if tx_response.code == 0 {
            (TxStatus::Success, None)
        } else {
            // Extract error message from raw_log or info
            let error_msg = if !tx_response.raw_log.is_empty() {
                tx_response.raw_log
            } else if !tx_response.info.is_empty() {
                tx_response.info
            } else {
                format!(
                    "Transaction failed with code {} in codespace {}",
                    tx_response.code, tx_response.codespace
                )
            };

            (TxStatus::Failed, Some(error_msg))
        };

        // Use timestamp from response
        let timestamp = tx_response.timestamp;

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
    /// let client = TxClient::new("https://api.xion-testnet-2.burnt.com".to_string());
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

    /// Get the REST URL
    pub fn rest_url(&self) -> &str {
        &self.rest_url
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
        let client = TxClient::new("https://api.xion-testnet-2.burnt.com".to_string());
        assert_eq!(client.rest_url(), "https://api.xion-testnet-2.burnt.com");
    }

    // Note: Integration tests with mock server would require wiremock
    // and would be placed in tests/ directory or as async integration tests
}
