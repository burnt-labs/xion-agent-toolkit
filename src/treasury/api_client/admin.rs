//! Admin management, params, batch, on-chain query, and export operations.
//!
//! This module provides methods for:
//! - Admin management (propose, accept, cancel)
//! - Treasury parameters management (update)
//! - Batch grant config operations
//! - On-chain query operations (authz grants, fee allowances)
//! - Treasury state export for backup/migration

use serde::Deserialize;
use tracing::{debug, instrument, warn};

use crate::shared::error::{NetworkError, XionResult};
use crate::treasury::types::{BroadcastRequest, QueryOptions};

use super::helpers::bytes_to_json_array;

impl super::TreasuryApiClient {
    // ========================================================================
    // Admin Management Operations
    // ========================================================================

    /// Propose a new admin for a treasury
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `new_admin` - New admin address to propose
    /// * `from_address` - Current admin's MetaAccount address
    ///
    /// # Returns
    /// Admin result with transaction hash
    #[instrument(skip(self, access_token))]
    pub async fn propose_admin(
        &self,
        access_token: &str,
        treasury_address: &str,
        new_admin: &str,
        from_address: &str,
    ) -> XionResult<crate::treasury::types::AdminResult> {
        debug!(
            "Proposing new admin {} for treasury: {}",
            new_admin, treasury_address
        );

        // Create the propose_admin message (matches contract's ExecuteMsg)
        let exec_msg = crate::treasury::types::TreasuryExecuteMsg::ProposeAdmin {
            new_admin: new_admin.to_string(),
        };

        // Broadcast using helper function
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                None,
                &format!("Propose new admin: {}", new_admin),
            )
            .await?;

        Ok(crate::treasury::types::AdminResult {
            treasury_address: treasury_address.to_string(),
            operation: "propose".to_string(),
            new_admin: Some(new_admin.to_string()),
            tx_hash,
        })
    }

    /// Accept admin role for a treasury
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `from_address` - Pending admin's MetaAccount address
    ///
    /// # Returns
    /// Admin result with transaction hash
    #[instrument(skip(self, access_token))]
    pub async fn accept_admin(
        &self,
        access_token: &str,
        treasury_address: &str,
        from_address: &str,
    ) -> XionResult<crate::treasury::types::AdminResult> {
        debug!("Accepting admin role for treasury: {}", treasury_address);

        // Create the accept_admin message (matches contract's ExecuteMsg)
        let exec_msg = crate::treasury::types::TreasuryExecuteMsg::AcceptAdmin {};

        // Broadcast using helper function
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                None,
                "Accept admin role",
            )
            .await?;

        Ok(crate::treasury::types::AdminResult {
            treasury_address: treasury_address.to_string(),
            operation: "accept".to_string(),
            new_admin: None,
            tx_hash,
        })
    }

    /// Cancel proposed admin for a treasury
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `from_address` - Current admin's MetaAccount address
    ///
    /// # Returns
    /// Admin result with transaction hash
    #[instrument(skip(self, access_token))]
    pub async fn cancel_proposed_admin(
        &self,
        access_token: &str,
        treasury_address: &str,
        from_address: &str,
    ) -> XionResult<crate::treasury::types::AdminResult> {
        debug!(
            "Canceling proposed admin for treasury: {}",
            treasury_address
        );

        // Create the cancel_proposed_admin message (matches contract's ExecuteMsg)
        let exec_msg = crate::treasury::types::TreasuryExecuteMsg::CancelProposedAdmin {};

        // Broadcast using helper function
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                None,
                "Cancel proposed admin",
            )
            .await?;

        Ok(crate::treasury::types::AdminResult {
            treasury_address: treasury_address.to_string(),
            operation: "cancel".to_string(),
            new_admin: None,
            tx_hash,
        })
    }

    // ========================================================================
    // Params Management Operations
    // ========================================================================

    /// Update treasury parameters
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `params` - New parameters to set
    /// * `from_address` - Admin's MetaAccount address
    ///
    /// # Returns
    /// Params result with transaction hash
    #[instrument(skip(self, access_token))]
    pub async fn update_params(
        &self,
        access_token: &str,
        treasury_address: &str,
        params: crate::treasury::types::UpdateParamsInput,
        from_address: &str,
    ) -> XionResult<crate::treasury::types::ParamsResult> {
        debug!("Updating params for treasury: {}", treasury_address);

        // Validate that at least one parameter is provided
        if params.redirect_url.is_none()
            && params.icon_url.is_none()
            && params.name.is_none()
            && params.is_oauth2_app.is_none()
            && params.metadata.is_none()
        {
            return Err(crate::shared::error::TreasuryError::OperationFailed(
                "At least one parameter must be provided for update (redirect_url, icon_url, name, is_oauth2_app, or metadata)".to_string()
            ).into());
        }

        // Build metadata by merging provided fields
        // Priority: explicit name/is_oauth2_app > metadata object values > defaults
        let mut metadata_obj = match params.metadata.clone() {
            Some(v) if v.is_object() => v,
            Some(_) => {
                return Err(crate::shared::error::TreasuryError::OperationFailed(
                    "--metadata must be a JSON object (e.g., '{\"key\": \"value\"}'), not a primitive or array".to_string()
                ).into());
            }
            None => serde_json::json!({}),
        };

        // Merge name into metadata (if provided)
        if let Some(name) = &params.name {
            if let Some(obj) = metadata_obj.as_object_mut() {
                obj.insert("name".to_string(), serde_json::json!(name));
            }
        }

        // Merge is_oauth2_app into metadata (if provided)
        if let Some(is_oauth2_app) = params.is_oauth2_app {
            if let Some(obj) = metadata_obj.as_object_mut() {
                obj.insert(
                    "is_oauth2_app".to_string(),
                    serde_json::json!(is_oauth2_app),
                );
            }
        }

        // Build params chain format
        // Note: metadata should be JSON string, not JSON object
        let params_chain = crate::treasury::types::TreasuryParamsChain {
            redirect_url: params.redirect_url.unwrap_or_default(),
            icon_url: params.icon_url.unwrap_or_default(),
            metadata: metadata_obj.to_string(),
        };

        // Create the update_params message (matches contract's ExecuteMsg)
        let exec_msg = crate::treasury::types::TreasuryExecuteMsg::UpdateParams {
            params: params_chain,
        };

        // Broadcast using helper function
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                None,
                "Update treasury params",
            )
            .await?;

        Ok(crate::treasury::types::ParamsResult {
            treasury_address: treasury_address.to_string(),
            tx_hash,
        })
    }

    // ========================================================================
    // Batch Operations
    // ========================================================================

    /// Add multiple grant configurations in a single transaction
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `grant_configs` - List of grant configurations to add
    /// * `from_address` - Admin's MetaAccount address
    ///
    /// # Returns
    /// Batch grant config result with transaction hash
    #[allow(dead_code)]
    #[instrument(skip(self, access_token, grant_configs))]
    pub async fn grant_config_batch(
        &self,
        access_token: &str,
        treasury_address: &str,
        grant_configs: Vec<(String, crate::treasury::types::GrantConfigInput)>,
        from_address: &str,
    ) -> XionResult<crate::treasury::types::BatchGrantConfigResult> {
        debug!(
            "Adding {} grant configs in batch to treasury: {}",
            grant_configs.len(),
            treasury_address
        );

        // Save the count before consuming the vector
        let count = grant_configs.len();

        // Build multiple update_grant_config messages
        let mut messages = Vec::new();

        for (type_url, grant_config_input) in grant_configs {
            // Encode the authorization
            let (auth_type_url, auth_value) =
                crate::treasury::encoding::encode_authorization_input(
                    &grant_config_input.authorization,
                    &type_url,
                )?;

            // Build the grant config for chain
            let grant_config_chain = crate::treasury::types::GrantConfigChain {
                description: grant_config_input.description.clone(),
                authorization: crate::treasury::types::ProtobufAny {
                    type_url: auth_type_url,
                    value: auth_value,
                },
                optional: grant_config_input.optional,
            };

            // Create update_grant_config execute message
            let exec_msg = crate::treasury::types::TreasuryExecuteMsg::UpdateGrantConfig {
                msg_type_url: type_url,
                grant_config: grant_config_chain,
            };

            // Serialize execute message
            let msg_json = serde_json::to_string(&exec_msg)?;
            let msg_bytes = msg_json.as_bytes();

            // Build MsgExecuteContract message
            let msg_value = serde_json::json!({
                "sender": from_address,
                "contract": treasury_address,
                "msg": bytes_to_json_array(msg_bytes),
                "funds": []
            });

            messages.push(crate::treasury::types::TransactionMessage {
                type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
                value: msg_value,
            });
        }

        // Broadcast all messages in a single transaction
        let broadcast_request = BroadcastRequest {
            messages,
            memo: Some(format!("Batch update {} grant configs", count)),
        };

        let response = self
            .broadcast_transaction(access_token, broadcast_request)
            .await?;

        Ok(crate::treasury::types::BatchGrantConfigResult {
            treasury_address: treasury_address.to_string(),
            count,
            tx_hash: response.tx_hash,
        })
    }

    // ========================================================================
    // On-Chain Query Operations (via RPC)
    // ========================================================================

    /// List all authz grants for a treasury (granter)
    ///
    /// Queries the blockchain directly via RPC for authz grants.
    ///
    /// # Arguments
    /// * `treasury_address` - Treasury contract address (granter)
    ///
    /// # Returns
    /// List of authz grants where the treasury is the granter
    #[instrument(skip(self))]
    pub async fn list_authz_grants(
        &self,
        treasury_address: &str,
    ) -> XionResult<Vec<crate::treasury::types::AuthzGrantInfo>> {
        debug!("Listing authz grants for treasury: {}", treasury_address);

        // Query authz grants via RPC
        // Endpoint: /cosmos/authz/v1beta1/grants?granter={treasury_address}
        let url = format!(
            "{}/cosmos/authz/v1beta1/grants?granter={}",
            self.rest_url, treasury_address
        );

        let response = self.http_client.get(&url).send().await.map_err(|e| {
            NetworkError::RequestFailed(format!("Failed to query authz grants: {}", e))
        })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(NetworkError::InvalidResponse(format!(
                "Failed to query authz grants: {} - {}",
                status, error_text
            ))
            .into());
        }

        // Parse the response
        #[derive(Debug, Deserialize)]
        struct AuthzGrantsResponse {
            grants: Vec<AuthzGrantItem>,
        }

        #[derive(Debug, Deserialize)]
        struct AuthzGrantItem {
            granter: String,
            grantee: String,
            authorization: Option<serde_json::Value>,
            expiration: Option<String>,
        }

        let grants_response: AuthzGrantsResponse = response.json().await.map_err(|e| {
            NetworkError::InvalidResponse(format!("Failed to parse authz grants response: {}", e))
        })?;

        // Convert to AuthzGrantInfo
        let grants: Vec<crate::treasury::types::AuthzGrantInfo> = grants_response
            .grants
            .into_iter()
            .map(|grant| {
                let authorization_type_url = grant
                    .authorization
                    .as_ref()
                    .and_then(|auth| auth.get("@type"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                crate::treasury::types::AuthzGrantInfo {
                    granter: grant.granter,
                    grantee: grant.grantee,
                    authorization_type_url,
                    expiration: grant.expiration,
                }
            })
            .collect();

        debug!("Found {} authz grants", grants.len());
        Ok(grants)
    }

    /// List all fee allowances for a treasury (granter)
    ///
    /// Queries the blockchain directly via RPC for fee allowances.
    ///
    /// # Arguments
    /// * `treasury_address` - Treasury contract address (granter)
    ///
    /// # Returns
    /// List of fee allowances where the treasury is the granter
    #[instrument(skip(self))]
    pub async fn list_fee_allowances(
        &self,
        treasury_address: &str,
    ) -> XionResult<Vec<crate::treasury::types::FeeAllowanceInfo>> {
        debug!("Listing fee allowances for treasury: {}", treasury_address);

        // Query fee allowances via RPC
        // Endpoint: /cosmos/feegrant/v1beta1/allowances/{granter}
        let url = format!(
            "{}/cosmos/feegrant/v1beta1/allowances/{}",
            self.rest_url, treasury_address
        );

        let response = self.http_client.get(&url).send().await.map_err(|e| {
            NetworkError::RequestFailed(format!("Failed to query fee allowances: {}", e))
        })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(NetworkError::InvalidResponse(format!(
                "Failed to query fee allowances: {} - {}",
                status, error_text
            ))
            .into());
        }

        // Parse the response
        #[derive(Debug, Deserialize)]
        struct FeeAllowancesResponse {
            allowances: Vec<FeeAllowanceItem>,
        }

        #[derive(Debug, Deserialize)]
        struct FeeAllowanceItem {
            granter: String,
            grantee: String,
            allowance: Option<serde_json::Value>,
        }

        let allowances_response: FeeAllowancesResponse = response.json().await.map_err(|e| {
            NetworkError::InvalidResponse(format!("Failed to parse fee allowances response: {}", e))
        })?;

        // Convert to FeeAllowanceInfo
        let allowances: Vec<crate::treasury::types::FeeAllowanceInfo> = allowances_response
            .allowances
            .into_iter()
            .map(|allowance| {
                let allowance_type_url = allowance
                    .allowance
                    .as_ref()
                    .and_then(|a| a.get("@type"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                // Extract spend limit if present
                let spend_limit = allowance.allowance.as_ref().and_then(|a| {
                    a.get("spend_limit")
                        .and_then(|sl| sl.as_array())
                        .and_then(|coins| coins.first())
                        .and_then(|coin| {
                            let amount = coin.get("amount")?.as_str()?;
                            let denom = coin.get("denom")?.as_str()?;
                            Some(format!("{}{}", amount, denom))
                        })
                });

                // Extract expiration if present
                let expiration = allowance
                    .allowance
                    .as_ref()
                    .and_then(|a| a.get("expiration"))
                    .and_then(|e| {
                        // Try to parse as timestamp and convert to string
                        e.as_str()
                            .map(|s| s.to_string())
                            .or_else(|| e.as_i64().map(|ts| ts.to_string()))
                    });

                // Extract period details for periodic allowance
                let (period, period_spend_limit, period_can_spend) = allowance
                    .allowance
                    .as_ref()
                    .map(|a| {
                        let period = a
                            .get("period")
                            .and_then(|p| p.as_str())
                            .map(|s| s.to_string());
                        let period_spend_limit = a
                            .get("period_spend_limit")
                            .and_then(|sl| sl.as_array())
                            .and_then(|coins| coins.first())
                            .and_then(|coin| {
                                let amount = coin.get("amount")?.as_str()?;
                                let denom = coin.get("denom")?.as_str()?;
                                Some(format!("{}{}", amount, denom))
                            });
                        let period_can_spend = a
                            .get("period_can_spend")
                            .and_then(|sl| sl.as_array())
                            .and_then(|coins| coins.first())
                            .and_then(|coin| {
                                let amount = coin.get("amount")?.as_str()?;
                                let denom = coin.get("denom")?.as_str()?;
                                Some(format!("{}{}", amount, denom))
                            });
                        (period, period_spend_limit, period_can_spend)
                    })
                    .unwrap_or((None, None, None));

                crate::treasury::types::FeeAllowanceInfo {
                    granter: allowance.granter,
                    grantee: allowance.grantee,
                    allowance_type_url,
                    spend_limit,
                    expiration,
                    period,
                    period_spend_limit,
                    period_can_spend,
                }
            })
            .collect();

        debug!("Found {} fee allowances", allowances.len());
        Ok(allowances)
    }

    // ========================================================================
    // Export Operations
    // ========================================================================

    /// Export treasury configuration for backup/migration
    ///
    /// This is a client-side operation that aggregates treasury data from
    /// multiple sources (indexer + RPC queries). Authentication is required
    /// for indexer queries but not for on-chain queries.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token (for indexer queries)
    /// * `treasury_address` - Treasury contract address
    ///
    /// # Returns
    /// Treasury export data containing all configuration
    #[instrument(skip(self, access_token))]
    pub async fn export_treasury_state(
        &self,
        access_token: &str,
        treasury_address: &str,
    ) -> XionResult<crate::treasury::types::TreasuryExportData> {
        debug!("Exporting treasury state for: {}", treasury_address);

        // Query basic treasury info from indexer
        let options = QueryOptions::default();
        let treasury_info = self
            .query_treasury(access_token, treasury_address, options)
            .await?;

        // Query fee config from indexer
        let mut fee_config = self
            .query_fee_config(access_token, treasury_address)
            .await?;

        // Enhance fee config with periodic allowance data from on-chain query
        if let Some(ref mut fc) = fee_config {
            // Query on-chain fee allowances to get periodic details
            match self.list_fee_allowances(treasury_address).await {
                Ok(allowances) => {
                    // Find the first allowance (treasury typically has one grantee for fee)
                    if let Some(allowance) = allowances.first() {
                        // Add periodic fields if present
                        if allowance.period.is_some() || allowance.period_spend_limit.is_some() {
                            fc.period = allowance.period.clone();
                            fc.period_spend_limit = allowance.period_spend_limit.clone();
                            fc.can_period_reset = Some(true); // Default to true for periodic
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to query on-chain fee allowances: {}", e);
                    // Continue without periodic data
                }
            }
        }

        // Query grant configs
        let grant_configs = self
            .list_grant_configs(access_token, treasury_address)
            .await?;

        // Build export data
        let export_data = crate::treasury::types::TreasuryExportData {
            address: treasury_address.to_string(),
            admin: treasury_info.admin,
            fee_config,
            grant_configs,
            params: Some(treasury_info.params),
            exported_at: chrono::Utc::now().to_rfc3339(),
        };

        debug!(
            "Successfully exported treasury state for: {}",
            treasury_address
        );
        Ok(export_data)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::treasury::types::{
        AuthorizationInput, BroadcastResponse, GrantConfigInput, UpdateParamsInput,
    };
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // Helper to create a test client
    fn create_test_client(mock_server_uri: &str) -> TreasuryApiClient {
        TreasuryApiClient::new(
            mock_server_uri.to_string(),
            mock_server_uri.to_string(),
            mock_server_uri.to_string(),
        )
    }

    // Helper to create a valid access token
    fn create_test_token(address: &str) -> String {
        format!("{}:grant123:secret456", address)
    }

    // =========================================================================
    // propose_admin tests
    // =========================================================================

    #[tokio::test]
    async fn test_propose_admin_success() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1currentadmin";
        let treasury_address = "xion1treasury123";
        let new_admin = "xion1newadmin456";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_propose_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client
            .propose_admin(&token, treasury_address, new_admin, admin_address)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.treasury_address, treasury_address);
        assert_eq!(result.operation, "propose");
        assert_eq!(result.new_admin, Some(new_admin.to_string()));
        assert_eq!(result.tx_hash, "tx_propose_hash");
    }

    // =========================================================================
    // accept_admin tests
    // =========================================================================

    #[tokio::test]
    async fn test_accept_admin_success() {
        let mock_server = MockServer::start().await;
        let pending_admin = "xion1pendingadmin";
        let treasury_address = "xion1treasury789";
        let token = create_test_token(pending_admin);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_accept_hash".to_string(),
                from: pending_admin.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client
            .accept_admin(&token, treasury_address, pending_admin)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.treasury_address, treasury_address);
        assert_eq!(result.operation, "accept");
        assert_eq!(result.new_admin, None);
        assert_eq!(result.tx_hash, "tx_accept_hash");
    }

    // =========================================================================
    // cancel_proposed_admin tests
    // =========================================================================

    #[tokio::test]
    async fn test_cancel_proposed_admin_success() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1currentadmin";
        let treasury_address = "xion1treasuryabc";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_cancel_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client
            .cancel_proposed_admin(&token, treasury_address, admin_address)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.treasury_address, treasury_address);
        assert_eq!(result.operation, "cancel");
        assert_eq!(result.new_admin, None);
        assert_eq!(result.tx_hash, "tx_cancel_hash");
    }

    // =========================================================================
    // update_params tests
    // =========================================================================

    #[tokio::test]
    async fn test_update_params_with_redirect_url() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_update_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let params = UpdateParamsInput {
            redirect_url: Some("https://new-callback.com".to_string()),
            icon_url: None,
            name: None,
            is_oauth2_app: None,
            metadata: None,
        };

        let result = client
            .update_params(&token, treasury_address, params, admin_address)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.treasury_address, treasury_address);
        assert_eq!(result.tx_hash, "tx_update_hash");
    }

    #[tokio::test]
    async fn test_update_params_with_name() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_update_name_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let params = UpdateParamsInput {
            redirect_url: None,
            icon_url: None,
            name: Some("My Updated Treasury".to_string()),
            is_oauth2_app: None,
            metadata: None,
        };

        let result = client
            .update_params(&token, treasury_address, params, admin_address)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_params_with_is_oauth2_app() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_update_oauth_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let params = UpdateParamsInput {
            redirect_url: None,
            icon_url: None,
            name: None,
            is_oauth2_app: Some(true),
            metadata: None,
        };

        let result = client
            .update_params(&token, treasury_address, params, admin_address)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_params_with_metadata() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_update_meta_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let params = UpdateParamsInput {
            redirect_url: None,
            icon_url: None,
            name: None,
            is_oauth2_app: None,
            metadata: Some(serde_json::json!({"custom_field": "custom_value"})),
        };

        let result = client
            .update_params(&token, treasury_address, params, admin_address)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_params_error_no_params() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";
        let token = create_test_token(admin_address);

        // No mock needed since the function should fail before making any request

        let client = create_test_client(&mock_server.uri());

        let params = UpdateParamsInput {
            redirect_url: None,
            icon_url: None,
            name: None,
            is_oauth2_app: None,
            metadata: None,
        };

        let result = client
            .update_params(&token, treasury_address, params, admin_address)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_params_error_invalid_metadata() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";
        let token = create_test_token(admin_address);

        let client = create_test_client(&mock_server.uri());

        // Metadata must be an object, not a string
        let params = UpdateParamsInput {
            redirect_url: None,
            icon_url: None,
            name: None,
            is_oauth2_app: None,
            metadata: Some(serde_json::json!("not an object")),
        };

        let result = client
            .update_params(&token, treasury_address, params, admin_address)
            .await;

        assert!(result.is_err());
    }

    // =========================================================================
    // grant_config_batch tests
    // =========================================================================

    #[tokio::test]
    async fn test_grant_config_batch_success() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1batchadmin";
        let treasury_address = "xion1batchtreasury";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_batch_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let grant_configs = vec![
            (
                "/cosmos.bank.v1beta1.MsgSend".to_string(),
                GrantConfigInput {
                    type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                    description: "Allow sends".to_string(),
                    authorization: AuthorizationInput::Generic,
                    optional: false,
                },
            ),
            (
                "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
                GrantConfigInput {
                    type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
                    description: "Allow staking".to_string(),
                    authorization: AuthorizationInput::Generic,
                    optional: true,
                },
            ),
        ];

        let result = client
            .grant_config_batch(&token, treasury_address, grant_configs, admin_address)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.treasury_address, treasury_address);
        assert_eq!(result.count, 2);
        assert_eq!(result.tx_hash, "tx_batch_hash");
    }

    #[tokio::test]
    async fn test_grant_config_batch_single_item() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1singleadmin";
        let treasury_address = "xion1singletreasury";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_single_batch_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let grant_configs = vec![(
            "/cosmos.bank.v1beta1.MsgSend".to_string(),
            GrantConfigInput {
                type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                description: "Single grant".to_string(),
                authorization: AuthorizationInput::Generic,
                optional: false,
            },
        )];

        let result = client
            .grant_config_batch(&token, treasury_address, grant_configs, admin_address)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.count, 1);
    }

    // =========================================================================
    // list_authz_grants tests
    // =========================================================================

    #[tokio::test]
    async fn test_list_authz_grants_success() {
        let mock_server = MockServer::start().await;
        let treasury_address = "xion1treasurygrants";

        // Use wiremock's query_param matcher for query parameters
        use wiremock::matchers::query_param;

        Mock::given(method("GET"))
            .and(path("/cosmos/authz/v1beta1/grants"))
            .and(query_param("granter", treasury_address))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "grants": [
                    {
                        "granter": treasury_address,
                        "grantee": "xion1grantee1",
                        "authorization": {"@type": "/cosmos.bank.v1beta1.SendAuthorization"},
                        "expiration": "2025-12-31T23:59:59Z"
                    },
                    {
                        "granter": treasury_address,
                        "grantee": "xion1grantee2",
                        "authorization": {"@type": "/cosmos.staking.v1beta1.StakeAuthorization"},
                        "expiration": "2025-06-30T23:59:59Z"
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client.list_authz_grants(treasury_address).await;

        assert!(result.is_ok());
        let grants = result.unwrap();
        assert_eq!(grants.len(), 2);
        assert_eq!(grants[0].granter, treasury_address);
        assert_eq!(grants[0].grantee, "xion1grantee1");
        assert_eq!(
            grants[0].authorization_type_url,
            "/cosmos.bank.v1beta1.SendAuthorization"
        );
        assert_eq!(grants[1].grantee, "xion1grantee2");
    }

    #[tokio::test]
    async fn test_list_authz_grants_empty() {
        let mock_server = MockServer::start().await;
        let treasury_address = "xion1emptytreasury";

        use wiremock::matchers::query_param;

        Mock::given(method("GET"))
            .and(path("/cosmos/authz/v1beta1/grants"))
            .and(query_param("granter", treasury_address))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "grants": []
            })))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client.list_authz_grants(treasury_address).await;

        assert!(result.is_ok());
        let grants = result.unwrap();
        assert!(grants.is_empty());
    }

    #[tokio::test]
    async fn test_list_authz_grants_http_error() {
        let mock_server = MockServer::start().await;
        let treasury_address = "xion1errortreasury";

        Mock::given(method("GET"))
            .and(path_regex(r"/cosmos/authz/v1beta1/grants"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client.list_authz_grants(treasury_address).await;

        assert!(result.is_err());
    }

    // =========================================================================
    // list_fee_allowances tests
    // =========================================================================

    #[tokio::test]
    async fn test_list_fee_allowances_success() {
        let mock_server = MockServer::start().await;
        let treasury_address = "xion1feeallowance";

        Mock::given(method("GET"))
            .and(path_regex(
                r"/cosmos/feegrant/v1beta1/allowances/xion1feeallowance",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "allowances": [
                    {
                        "granter": treasury_address,
                        "grantee": "xion1feegrantee1",
                        "allowance": {
                            "@type": "/cosmos.feegrant.v1beta1.BasicAllowance",
                            "spend_limit": [{"denom": "uxion", "amount": "1000000"}],
                            "expiration": "2025-12-31T23:59:59Z"
                        }
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client.list_fee_allowances(treasury_address).await;

        assert!(result.is_ok());
        let allowances = result.unwrap();
        assert_eq!(allowances.len(), 1);
        assert_eq!(allowances[0].granter, treasury_address);
        assert_eq!(allowances[0].grantee, "xion1feegrantee1");
        assert_eq!(
            allowances[0].allowance_type_url,
            "/cosmos.feegrant.v1beta1.BasicAllowance"
        );
        assert_eq!(allowances[0].spend_limit, Some("1000000uxion".to_string()));
    }

    #[tokio::test]
    async fn test_list_fee_allowances_with_periodic() {
        let mock_server = MockServer::start().await;
        let treasury_address = "xion1periodicfee";

        Mock::given(method("GET"))
            .and(path_regex(
                r"/cosmos/feegrant/v1beta1/allowances/xion1periodicfee",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "allowances": [
                    {
                        "granter": treasury_address,
                        "grantee": "xion1periodicgrantee",
                        "allowance": {
                            "@type": "/cosmos.feegrant.v1beta1.PeriodicAllowance",
                            "period": "3600",
                            "period_spend_limit": [{"denom": "uxion", "amount": "500000"}],
                            "period_can_spend": [{"denom": "uxion", "amount": "250000"}],
                            "expiration": "2025-12-31T23:59:59Z"
                        }
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client.list_fee_allowances(treasury_address).await;

        assert!(result.is_ok());
        let allowances = result.unwrap();
        assert_eq!(allowances.len(), 1);
        assert_eq!(allowances[0].period, Some("3600".to_string()));
        assert_eq!(
            allowances[0].period_spend_limit,
            Some("500000uxion".to_string())
        );
        assert_eq!(
            allowances[0].period_can_spend,
            Some("250000uxion".to_string())
        );
    }

    #[tokio::test]
    async fn test_list_fee_allowances_empty() {
        let mock_server = MockServer::start().await;
        let treasury_address = "xion1noallowance";

        Mock::given(method("GET"))
            .and(path_regex(
                r"/cosmos/feegrant/v1beta1/allowances/xion1noallowance",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "allowances": []
            })))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client.list_fee_allowances(treasury_address).await;

        assert!(result.is_ok());
        let allowances = result.unwrap();
        assert!(allowances.is_empty());
    }

    // =========================================================================
    // export_treasury_state tests
    // =========================================================================

    #[tokio::test]
    async fn test_export_treasury_state_success() {
        // Note: export_treasury_state aggregates data from indexer and on-chain queries.
        // The indexer does not return grant configs, so grant_configs will be empty.
        let mock_server = MockServer::start().await;
        let admin_address = "xion1exportadmin";
        let treasury_address = "xion1exporttreasury";
        let token = create_test_token(admin_address);

        // Mock the indexer - all indexer calls use the same URL pattern
        // Wiremock will use sequential matching
        Mock::given(method("GET"))
            .and(path_regex(
                r"/contract/xion1exportadmin/xion/account/treasuries",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([{
                    "contractAddress": treasury_address,
                    "balances": {"uxion": "1000000"},
                    "params": {
                        "redirect_url": "https://example.com",
                        "icon_url": "https://example.com/icon.png",
                        "metadata": "{\"name\": \"Export Treasury\"}"
                    }
                }])),
            )
            .mount(&mock_server)
            .await;

        // Second call returns same treasury info
        Mock::given(method("GET"))
            .and(path_regex(
                r"/contract/xion1exportadmin/xion/account/treasuries",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([{
                    "contractAddress": treasury_address,
                    "balances": {"uxion": "1000000"},
                    "params": {
                        "redirect_url": "https://example.com",
                        "icon_url": "https://example.com/icon.png",
                        "metadata": "{}"
                    }
                }])),
            )
            .mount(&mock_server)
            .await;

        // Third call returns same treasury info (indexer doesn't return grant configs)
        Mock::given(method("GET"))
            .and(path_regex(
                r"/contract/xion1exportadmin/xion/account/treasuries",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([{
                    "contractAddress": treasury_address,
                    "balances": {"uxion": "1000000"},
                    "params": {
                        "redirect_url": "https://example.com",
                        "icon_url": "https://example.com/icon.png",
                        "metadata": "{}"
                    }
                }])),
            )
            .mount(&mock_server)
            .await;

        // Mock for list_fee_allowances (on-chain RPC query)
        Mock::given(method("GET"))
            .and(path_regex(
                r"/cosmos/feegrant/v1beta1/allowances/xion1exporttreasury",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "allowances": []
            })))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client.export_treasury_state(&token, treasury_address).await;

        assert!(result.is_ok());
        let export = result.unwrap();
        assert_eq!(export.address, treasury_address);
        assert!(export.params.is_some());
        // Note: grant_configs are empty because indexer doesn't return them
        assert!(export.grant_configs.is_empty());
    }
}
