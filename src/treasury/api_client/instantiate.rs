//! Treasury instantiation operations.
//!
//! This module provides methods for:
//! - Creating new treasury contracts via instantiate2
//! - Waiting for treasury creation to be indexed
//! - Building treasury instantiation messages

use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, instrument, warn};

use crate::shared::error::{TreasuryError, XionResult};
use crate::treasury::types::CreateTreasuryRequest;

/// Default delay before polling for new treasury (in seconds)
const DEFAULT_POLL_DELAY_SECS: u64 = 2;
/// Default timeout for waiting for treasury creation (in seconds)
const DEFAULT_POLL_TIMEOUT_SECS: u64 = 30;
/// Polling interval (in seconds)
const POLL_INTERVAL_SECS: u64 = 2;

/// Build the treasury instantiation message
///
/// Creates the instantiate message in the format expected by the treasury contract:
/// - `grant_configs`: Array of JSON objects with authorization, description, and optional fields
/// - `fee_config`: JSON object with allowance, description, and optional expiration
/// - All fields use snake_case naming (matching the CosmWasm contract)
pub(crate) fn build_treasury_instantiate_msg(
    request: &CreateTreasuryRequest,
) -> XionResult<serde_json::Value> {
    // Build the metadata JSON string
    let metadata = serde_json::json!({
        "name": request.name.as_deref().unwrap_or(""),
        "archived": false,
        "is_oauth2_app": request.is_oauth2_app,
    });

    // Extract type URLs from grant configs (message type URLs like /cosmos.bank.v1beta1.MsgSend)
    let type_urls: Vec<String> = request
        .grant_configs
        .iter()
        .map(|gc| gc.type_url.clone())
        .collect();

    // Build the grant configs array (without type_url, that's in type_urls)
    // Each grant_config is a JSON object with: authorization, description, optional
    let grant_configs: Vec<serde_json::Value> = request
        .grant_configs
        .iter()
        .map(|gc| {
            // Build the authorization object
            let auth = serde_json::json!({
                "type_url": gc.authorization.type_url,
                "value": gc.authorization.value,
            });

            // Build the grant config object, omitting description if None
            let mut grant_config = serde_json::json!({
                "authorization": auth,
                "optional": gc.optional,
            });

            if let Some(ref desc) = gc.description {
                grant_config["description"] = serde_json::json!(desc);
            }

            grant_config
        })
        .collect();

    // Build fee_config as a JSON object
    // Fields: allowance (JSON object), description (string), expiration (optional string)
    let mut fee_config = serde_json::json!({
        "allowance": {
            "type_url": request.fee_config.allowance.type_url,
            "value": request.fee_config.allowance.value,
        },
        "description": request.fee_config.description,
    });

    // Add expiration if present (omit if None)
    if let Some(ref expiration) = request.fee_config.expiration {
        fee_config["expiration"] = serde_json::json!(expiration);
    }

    // Build params object, omitting display_url if None
    let mut params = serde_json::json!({
        "redirect_url": request.params.redirect_url,
        "icon_url": request.params.icon_url,
        "metadata": metadata.to_string(),
    });
    if let Some(ref display_url) = request.params.display_url {
        params["display_url"] = serde_json::json!(display_url);
    }

    // Build the complete instantiation message
    Ok(serde_json::json!({
        "type_urls": type_urls,
        "grant_configs": grant_configs,
        "fee_config": fee_config,
        "admin": request.admin,
        "params": params,
    }))
}

impl super::TreasuryApiClient {
    /// Create a new treasury contract
    ///
    /// Creates a new treasury contract using CosmWasm instantiate2 for predictable addresses.
    /// The treasury contract is instantiated with the provided configuration.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_code_id` - Treasury contract code ID from network config
    /// * `request` - Treasury creation request with all required parameters
    /// * `salt` - Random salt for instantiate2 (32 bytes)
    ///
    /// # Returns
    /// Create treasury result with the new treasury address and transaction hash
    ///
    /// # Errors
    /// Returns an error if:
    /// - The access token is invalid or expired
    /// - Invalid parameters
    /// - Contract instantiation fails
    /// - Network request fails
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::{TreasuryApiClient, CreateTreasuryRequest, FeeConfigMessage, GrantConfigMessage, TreasuryParamsMessage, TypeUrlValue};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://api.testnet-2.burnt.com".to_string(),
    /// );
    ///
    /// let request = CreateTreasuryRequest {
    ///     admin: "xion1admin...".to_string(),
    ///     fee_config: FeeConfigMessage {
    ///         allowance: TypeUrlValue {
    ///             type_url: "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
    ///             value: "Cg==".to_string(), // Base64-encoded empty BasicAllowance
    ///         },
    ///         description: "Fee grant for users".to_string(),
    ///         expiration: None,
    ///     },
    ///     grant_configs: vec![
    ///         GrantConfigMessage {
    ///             type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
    ///             authorization: TypeUrlValue {
    ///                 type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
    ///                 value: "Cg==".to_string(), // Base64-encoded empty authorization
    ///             },
    ///             description: Some("Allow sending tokens".to_string()),
    ///             optional: false,
    ///         },
    ///     ],
    ///     params: TreasuryParamsMessage {
    ///         redirect_url: "https://example.com/callback".to_string(),
    ///         icon_url: "https://example.com/icon.png".to_string(),
    ///         display_url: None,
    ///         metadata: None,
    ///     },
    ///     name: Some("My Treasury".to_string()),
    ///     is_oauth2_app: false,
    /// };
    ///
    /// let salt = [0u8; 32];
    /// let result = client.create_treasury("access_token_123", 1260, request, &salt).await?;
    /// println!("Treasury created at: {}", result.treasury_address);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token, request, salt))]
    pub async fn create_treasury(
        &self,
        access_token: &str,
        treasury_code_id: u64,
        request: crate::treasury::types::CreateTreasuryRequest,
        salt: &[u8],
    ) -> XionResult<crate::treasury::types::CreateTreasuryResult> {
        debug!("Creating treasury with code ID: {}", treasury_code_id);

        // CRITICAL: Compute predicted instantiate2 address BEFORE broadcasting
        // This allows us to verify the correct treasury is returned
        let code_info = self.get_code_info(treasury_code_id).await?;
        let checksum_bytes = hex::decode(&code_info.checksum).map_err(|e| {
            TreasuryError::OperationFailed(format!("Failed to decode checksum: {}", e))
        })?;

        let predicted_address = crate::shared::instantiate2::compute_address(
            &request.admin,
            treasury_code_id,
            salt,
            &checksum_bytes,
        )
        .map_err(|e| {
            TreasuryError::OperationFailed(format!("Failed to compute predicted address: {}", e))
        })?;

        debug!(
            "Predicted treasury address (instantiate2): {}",
            predicted_address
        );

        // Build the instantiation message
        let instantiate_msg = build_treasury_instantiate_msg(&request)?;

        // Generate label for the treasury
        let label = format!("Treasury-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"));

        // Use the generic broadcast_instantiate_contract2 method
        let tx_hash = self
            .broadcast_instantiate_contract2(
                access_token,
                &request.admin,
                treasury_code_id,
                &instantiate_msg,
                &label,
                salt,
                Some(&request.admin), // admin for contract migrations
                "Create Treasury via Xion Agent Toolkit",
            )
            .await?;

        debug!("Treasury creation transaction broadcast: {}", tx_hash);

        // Wait for the treasury to be indexed and verify it matches the predicted address
        let treasury_address = self
            .wait_for_treasury_creation(access_token, &predicted_address, &tx_hash)
            .await?;

        Ok(crate::treasury::types::CreateTreasuryResult {
            treasury_address,
            tx_hash,
            admin: request.admin,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Wait for treasury creation to be indexed
    ///
    /// Polls the DaoDao Indexer to find the newly created treasury.
    /// Validates that the returned treasury matches the expected instantiate2 address.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `expected_address` - The predicted instantiate2 address to verify
    /// * `tx_hash` - Transaction hash for error reporting
    ///
    /// # Returns
    /// The treasury address if found and verified within timeout
    ///
    /// # Errors
    /// Returns an error with the tx_hash if the treasury is not found within the timeout period
    /// or if the found treasury doesn't match the expected address
    #[instrument(skip(self, access_token))]
    pub(crate) async fn wait_for_treasury_creation(
        &self,
        access_token: &str,
        expected_address: &str,
        tx_hash: &str,
    ) -> XionResult<String> {
        debug!(
            "Waiting for treasury creation to be indexed (expected: {})",
            expected_address
        );

        // Initial delay to allow indexing
        sleep(Duration::from_secs(DEFAULT_POLL_DELAY_SECS)).await;

        let start_time = std::time::Instant::now();
        let timeout = Duration::from_secs(DEFAULT_POLL_TIMEOUT_SECS);

        loop {
            // Check if we've exceeded the timeout
            if start_time.elapsed() >= timeout {
                return Err(TreasuryError::OperationFailed(format!(
                    "Treasury creation timed out after {} seconds. Transaction was broadcast successfully (tx_hash: {}). \
                      The treasury may still be created. Check the transaction status manually or wait a few moments and list your treasuries.",
                    DEFAULT_POLL_TIMEOUT_SECS,
                    tx_hash
                ))
                .into());
            }

            // List treasuries and look for the newly created one
            match self.list_treasuries(access_token).await {
                Ok(treasuries) => {
                    // CRITICAL: Verify the treasury matches our expected instantiate2 address
                    // This prevents race conditions where another treasury might be returned
                    if let Some(treasury) =
                        treasuries.iter().find(|t| t.address == expected_address)
                    {
                        debug!(
                            "Found and verified treasury: {} (matches predicted instantiate2 address)",
                            treasury.address
                        );
                        return Ok(treasury.address.clone());
                    }
                    debug!(
                        "Treasury {} not yet indexed, retrying... ({}/{}s elapsed)",
                        expected_address,
                        start_time.elapsed().as_secs(),
                        DEFAULT_POLL_TIMEOUT_SECS
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to list treasuries while waiting for creation: {}",
                        e
                    );
                }
            }

            // Wait before next poll
            sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
        }
    }
}
