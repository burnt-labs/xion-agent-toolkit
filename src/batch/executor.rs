//! Batch Executor
//!
//! Handles execution of batch transactions via the OAuth2 API.

use anyhow::{Context, Result};
use thiserror::Error;
use tracing::{debug, info, instrument};

use super::types::{BatchRequest, BatchResult};
use crate::treasury::types::{BroadcastRequest, TransactionMessage};

/// Errors that can occur during batch execution
#[derive(Debug, Error)]
pub enum BatchExecutorError {
    /// Not authenticated
    #[error("Not authenticated. Please run 'xion auth login' first.")]
    NotAuthenticated,

    /// Batch validation failed
    #[error("Batch validation failed: {0}")]
    ValidationFailed(String),

    /// Transaction broadcast failed
    #[error("Transaction broadcast failed: {0}")]
    BroadcastFailed(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),
}

/// Batch executor for executing multiple messages in a single transaction
///
/// Uses the OAuth2 API's transaction endpoint to broadcast batch transactions.
pub struct BatchExecutor {
    /// Treasury API client for broadcasting transactions
    api_client: crate::treasury::TreasuryApiClient,
    /// OAuth client for authentication
    oauth_client: crate::oauth::OAuthClient,
}

impl BatchExecutor {
    /// Create a new batch executor
    ///
    /// # Arguments
    /// * `oauth_client` - OAuth client for authentication
    /// * `network_config` - Network configuration (used to create API client)
    pub fn new(
        oauth_client: crate::oauth::OAuthClient,
        network_config: crate::config::NetworkConfig,
    ) -> Self {
        let api_client = crate::treasury::TreasuryApiClient::new(
            network_config.oauth_api_url.clone(),
            network_config.indexer_url.clone(),
            network_config.rpc_url.clone(),
        );

        Self {
            api_client,
            oauth_client,
        }
    }

    /// Check if the user is authenticated
    pub fn is_authenticated(&self) -> Result<bool> {
        self.oauth_client.is_authenticated()
    }

    /// Get the user's address from credentials
    fn get_user_address(&self) -> Result<String> {
        let credentials = self
            .oauth_client
            .get_credentials()?
            .ok_or_else(|| anyhow::anyhow!("No credentials found"))?;

        credentials
            .xion_address
            .ok_or_else(|| anyhow::anyhow!("No Xion address in credentials"))
    }

    /// Execute a batch transaction
    ///
    /// # Arguments
    /// * `request` - Batch request containing messages to execute
    ///
    /// # Returns
    /// Batch result with transaction hash and status
    ///
    /// # Errors
    /// Returns an error if:
    /// - Not authenticated
    /// - Batch validation fails
    /// - Transaction broadcast fails
    #[instrument(skip(self, request))]
    pub async fn execute(&self, request: &BatchRequest) -> Result<BatchResult, BatchExecutorError> {
        info!("Executing batch with {} messages", request.messages.len());

        // Validate the batch
        request
            .validate()
            .map_err(|e| BatchExecutorError::ValidationFailed(e.to_string()))?;

        // Check authentication
        if !self.is_authenticated().map_err(|e| {
            BatchExecutorError::NetworkError(format!("Failed to check authentication: {}", e))
        })? {
            return Err(BatchExecutorError::NotAuthenticated);
        }

        // Get valid access token
        let access_token = self
            .oauth_client
            .get_valid_token()
            .await
            .context("Failed to get valid access token")
            .map_err(|e| BatchExecutorError::NetworkError(e.to_string()))?;

        // Get user address
        let from_address = self
            .get_user_address()
            .map_err(|e| BatchExecutorError::NetworkError(e.to_string()))?;

        debug!("Executing batch from: {}", from_address);

        // Convert batch messages to transaction messages
        let messages: Vec<TransactionMessage> = request
            .messages
            .iter()
            .map(|msg| TransactionMessage {
                type_url: msg.type_url.clone(),
                value: msg.value.clone(),
            })
            .collect();

        // Build the broadcast request
        let broadcast_request = BroadcastRequest {
            messages,
            memo: request.memo.clone(),
        };

        // Broadcast the transaction
        match self
            .api_client
            .broadcast_transaction(&access_token, broadcast_request)
            .await
        {
            Ok(response) => {
                info!("Batch executed successfully: {}", response.tx_hash);
                Ok(BatchResult {
                    success: true,
                    tx_hash: Some(response.tx_hash),
                    from: response.from,
                    gas_used: response.gas_used,
                    gas_wanted: response.gas_wanted,
                    message_count: request.messages.len(),
                })
            }
            Err(e) => {
                let error_msg = e.to_string();
                debug!("Batch execution failed: {}", error_msg);

                // Determine error type based on error message
                let batch_error = if error_msg.contains("unauthorized") || error_msg.contains("401")
                {
                    BatchExecutorError::NotAuthenticated
                } else {
                    BatchExecutorError::BroadcastFailed(error_msg)
                };

                Err(batch_error)
            }
        }
    }

    /// Simulate a batch transaction (dry-run)
    ///
    /// This estimates gas without actually executing the transaction.
    ///
    /// # Arguments
    /// * `request` - Batch request to simulate
    ///
    /// # Returns
    /// Estimated gas usage (if supported by API)
    ///
    /// # Note
    /// Currently returns the validation result. Full simulation support
    /// requires the `/api/v1/transaction/simulate` endpoint which may not
    /// be available on all networks.
    #[instrument(skip(self, request))]
    pub async fn simulate(
        &self,
        request: &BatchRequest,
    ) -> Result<BatchSimulationResult, BatchExecutorError> {
        debug!("Simulating batch with {} messages", request.messages.len());

        // Validate the batch
        request
            .validate()
            .map_err(|e| BatchExecutorError::ValidationFailed(e.to_string()))?;

        // Check authentication
        if !self.is_authenticated().map_err(|e| {
            BatchExecutorError::NetworkError(format!("Failed to check authentication: {}", e))
        })? {
            return Err(BatchExecutorError::NotAuthenticated);
        }

        // Get user address for validation
        let from_address = self
            .get_user_address()
            .map_err(|e| BatchExecutorError::NetworkError(e.to_string()))?;

        // Return simulation result
        // Note: Full gas estimation would require calling /api/v1/transaction/simulate
        // which may not be available. For now, return a basic validation result.
        Ok(BatchSimulationResult {
            valid: true,
            message_count: request.messages.len(),
            from: from_address,
            gas_estimate: None,
            message_types: request
                .messages
                .iter()
                .map(|m| m.type_url.clone())
                .collect(),
        })
    }
}

/// Result of batch simulation
#[derive(Debug, Clone, serde::Serialize)]
pub struct BatchSimulationResult {
    /// Whether the batch is valid
    pub valid: bool,

    /// Number of messages in the batch
    pub message_count: usize,

    /// Sender address
    pub from: String,

    /// Estimated gas usage (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_estimate: Option<String>,

    /// List of message type URLs in the batch
    pub message_types: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch::types::BatchMessage;

    fn create_test_batch_request() -> BatchRequest {
        BatchRequest {
            messages: vec![BatchMessage {
                type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                value: serde_json::json!({
                    "toAddress": "xion1abc",
                    "amount": [{ "denom": "uxion", "amount": "1000000" }]
                }),
            }],
            memo: Some("Test batch".to_string()),
        }
    }

    #[test]
    fn test_validate_success() {
        let request = create_test_batch_request();

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_batch() {
        let request = BatchRequest {
            messages: vec![],
            memo: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_simulation_result_serialization() {
        let result = BatchSimulationResult {
            valid: true,
            message_count: 2,
            from: "xion1sender".to_string(),
            gas_estimate: Some("150000".to_string()),
            message_types: vec![
                "/cosmos.bank.v1beta1.MsgSend".to_string(),
                "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
            ],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"valid\":true"));
        assert!(json.contains("\"message_count\":2"));
    }
}
