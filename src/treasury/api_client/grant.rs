//! Grant config and fee config operations for treasury contracts.
//!
//! This module provides methods for:
//! - Grant configuration management (add, remove, list)
//! - Fee configuration management (set, revoke, query)

use tracing::{debug, instrument};

use crate::shared::error::XionResult;
use crate::treasury::types::QueryOptions;

impl super::TreasuryApiClient {
    // ========================================================================
    // Grant Config Operations
    // ========================================================================

    /// Add a grant configuration to a treasury
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address
    /// * `type_url` - Type URL of the message (e.g., "/cosmwasm.wasm.v1.MsgExecuteContract")
    /// * `grant_config` - Grant configuration input
    /// * `from_address` - Sender's MetaAccount address (must be admin)
    ///
    /// # Returns
    /// Grant config result with transaction hash
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::{TreasuryApiClient, GrantConfigInput};
    /// use xion_agent_toolkit::treasury::types::AuthorizationInput;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://api.testnet-2.burnt.com".to_string(),
    /// );
    ///
    /// let grant_config = GrantConfigInput {
    ///     type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
    ///     description: "Allow token sends".to_string(),
    ///     authorization: AuthorizationInput::Send {
    ///         spend_limit: "1000000".to_string(),
    ///         allow_list: None,
    ///     },
    ///     optional: false,
    /// };
    ///
    /// let result = client
    ///     .add_grant_config(
    ///         "access_token_123",
    ///         "xion1treasury...",
    ///         "/cosmos.bank.v1beta1.MsgSend",
    ///         grant_config,
    ///         "xion1admin..."
    ///     )
    ///     .await?;
    /// println!("Grant config added: {}", result.tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token, grant_config))]
    pub async fn add_grant_config(
        &self,
        access_token: &str,
        treasury_address: &str,
        type_url: &str,
        grant_config: crate::treasury::types::GrantConfigInput,
        from_address: &str,
    ) -> XionResult<crate::treasury::types::GrantConfigResult> {
        debug!(
            "Adding grant config for type_url: {} to treasury: {}",
            type_url, treasury_address
        );

        // Encode the authorization (pass type_url for GenericAuthorization)
        let (auth_type_url, auth_value) = crate::treasury::encoding::encode_authorization_input(
            &grant_config.authorization,
            type_url,
        )?;

        // Build the grant config for chain
        let grant_config_chain = crate::treasury::types::GrantConfigChain {
            description: grant_config.description.clone(),
            authorization: crate::treasury::types::ProtobufAny {
                type_url: auth_type_url,
                value: auth_value, // Already base64 encoded string
            },
            optional: grant_config.optional,
        };

        // Create the update_grant_config message (matches contract's ExecuteMsg)
        let exec_msg = crate::treasury::types::TreasuryExecuteMsg::UpdateGrantConfig {
            msg_type_url: type_url.to_string(),
            grant_config: grant_config_chain,
        };

        // Broadcast using helper function (follows create_treasury format)
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                None,
                &format!("Update grant config for {}", type_url),
            )
            .await?;

        Ok(crate::treasury::types::GrantConfigResult {
            treasury_address: treasury_address.to_string(),
            type_url: type_url.to_string(),
            operation: "update".to_string(),
            tx_hash,
        })
    }

    /// Remove a grant configuration from a treasury
    #[instrument(skip(self, access_token))]
    pub async fn remove_grant_config(
        &self,
        access_token: &str,
        treasury_address: &str,
        type_url: &str,
        from_address: &str,
    ) -> XionResult<crate::treasury::types::GrantConfigResult> {
        debug!(
            "Removing grant config for type_url: {} from treasury: {}",
            type_url, treasury_address
        );

        // Create the remove_grant_config message (matches contract's ExecuteMsg)
        let remove_msg = crate::treasury::types::TreasuryExecuteMsg::RemoveGrantConfig {
            msg_type_url: type_url.to_string(),
        };

        // Broadcast using helper function (follows create_treasury format)
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &remove_msg,
                None,
                &format!("Remove grant config {}", type_url),
            )
            .await?;

        Ok(crate::treasury::types::GrantConfigResult {
            treasury_address: treasury_address.to_string(),
            type_url: type_url.to_string(),
            operation: "remove".to_string(),
            tx_hash,
        })
    }

    /// List grant configurations for a treasury
    #[instrument(skip(self, access_token))]
    pub async fn list_grant_configs(
        &self,
        access_token: &str,
        treasury_address: &str,
    ) -> XionResult<Vec<crate::treasury::types::GrantConfigInfo>> {
        debug!("Listing grant configs for treasury: {}", treasury_address);

        // Query the treasury info which includes grant configs
        let options = QueryOptions {
            grants: true,
            fee: false,
            admin: false,
        };
        let treasury = self
            .query_treasury(access_token, treasury_address, options)
            .await?;

        // Extract grant configs
        let grant_configs = treasury.grant_configs.unwrap_or_default();
        let configs: Vec<crate::treasury::types::GrantConfigInfo> = grant_configs
            .into_iter()
            .map(|gc| crate::treasury::types::GrantConfigInfo {
                type_url: gc.type_url,
                description: gc
                    .grant_config
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                authorization_type_url: gc
                    .grant_config
                    .get("authorization")
                    .and_then(|a| a.get("type_url"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                optional: gc
                    .grant_config
                    .get("optional")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                // authorization_input is not available from on-chain query
                // It will be None for exports, and import will default to Generic
                authorization_input: None,
            })
            .collect();

        Ok(configs)
    }

    // ========================================================================
    // Fee Config Operations
    // ========================================================================

    /// Set fee configuration for a treasury
    #[instrument(skip(self, access_token, fee_config))]
    pub async fn set_fee_config(
        &self,
        access_token: &str,
        treasury_address: &str,
        fee_config: crate::treasury::types::FeeConfigInput,
        from_address: &str,
    ) -> XionResult<crate::treasury::types::FeeConfigResult> {
        debug!("Setting fee config for treasury: {}", treasury_address);

        // Encode the fee allowance
        let (allowance_type_url, allowance_value) =
            crate::treasury::encoding::encode_fee_config_input(&fee_config)?;

        // Build the fee config for chain
        let fee_config_chain = crate::treasury::types::FeeConfigChain {
            description: match &fee_config {
                crate::treasury::types::FeeConfigInput::Basic { description, .. } => {
                    description.clone()
                }
                crate::treasury::types::FeeConfigInput::Periodic { description, .. } => {
                    description.clone()
                }
                crate::treasury::types::FeeConfigInput::AllowedMsg { description, .. } => {
                    description.clone()
                }
            },
            allowance: Some(crate::treasury::types::ProtobufAny {
                type_url: allowance_type_url,
                value: allowance_value, // Already base64 encoded string
            }),
            expiration: None,
        };

        // Create the update_fee_config message (matches contract's ExecuteMsg)
        let exec_msg = crate::treasury::types::TreasuryExecuteMsg::UpdateFeeConfig {
            fee_config: fee_config_chain,
        };

        // Broadcast using helper function (follows create_treasury format)
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                None,
                "Update fee config",
            )
            .await?;

        Ok(crate::treasury::types::FeeConfigResult {
            treasury_address: treasury_address.to_string(),
            operation: "update".to_string(),
            tx_hash,
        })
    }

    /// Revoke fee allowance from a treasury (revokes from grantee)
    #[instrument(skip(self, access_token))]
    pub async fn revoke_allowance(
        &self,
        access_token: &str,
        treasury_address: &str,
        grantee: &str,
        from_address: &str,
    ) -> XionResult<crate::treasury::types::FeeConfigResult> {
        debug!(
            "Revoking allowance from grantee: {} for treasury: {}",
            grantee, treasury_address
        );

        // Create the revoke_allowance message (matches contract's ExecuteMsg)
        let exec_msg = crate::treasury::types::TreasuryExecuteMsg::RevokeAllowance {
            grantee: grantee.to_string(),
        };

        // Broadcast using helper function (follows create_treasury format)
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &exec_msg,
                None,
                "Remove fee config",
            )
            .await?;

        Ok(crate::treasury::types::FeeConfigResult {
            treasury_address: treasury_address.to_string(),
            operation: "remove".to_string(),
            tx_hash,
        })
    }

    /// Query fee configuration for a treasury
    #[instrument(skip(self, access_token))]
    pub async fn query_fee_config(
        &self,
        access_token: &str,
        treasury_address: &str,
    ) -> XionResult<Option<crate::treasury::types::FeeConfigInfo>> {
        debug!("Querying fee config for treasury: {}", treasury_address);
        // Query the treasury info which includes fee config
        let options = QueryOptions {
            grants: false,
            fee: true,
            admin: false,
        };
        let treasury = self
            .query_treasury(access_token, treasury_address, options)
            .await?;

        // Extract fee config
        if let Some(fee_config) = treasury.fee_config {
            let description = fee_config
                .additional
                .as_ref()
                .and_then(|a| a.get("description"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();

            Ok(Some(crate::treasury::types::FeeConfigInfo {
                allowance_type_url: fee_config.config_type,
                description,
                spend_limit: fee_config.spend_limit,
                expiration: fee_config.expires_at,
                // Periodic fields are not available from indexer query
                // They will be populated by export_treasury_state via on-chain query
                period: None,
                period_spend_limit: None,
                can_period_reset: None,
            }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::treasury::types::{
        AuthorizationInput, BroadcastResponse, FeeConfigInput, GrantConfigInput,
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
    // add_grant_config tests
    // =========================================================================

    #[tokio::test]
    async fn test_add_grant_config_success() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";
        let token = create_test_token(admin_address);

        // Mock the broadcast endpoint
        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx123hash".to_string(),
                from: admin_address.to_string(),
                gas_used: Some("100000".to_string()),
                gas_wanted: Some("200000".to_string()),
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let grant_config = GrantConfigInput {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            description: "Allow token sends".to_string(),
            authorization: AuthorizationInput::Send {
                spend_limit: "1000000".to_string(),
                allow_list: None,
            },
            optional: false,
        };

        let result = client
            .add_grant_config(
                &token,
                treasury_address,
                "/cosmos.bank.v1beta1.MsgSend",
                grant_config,
                admin_address,
            )
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.treasury_address, treasury_address);
        assert_eq!(result.type_url, "/cosmos.bank.v1beta1.MsgSend");
        assert_eq!(result.operation, "update");
        assert_eq!(result.tx_hash, "tx123hash");
    }

    #[tokio::test]
    async fn test_add_grant_config_with_optional_flag() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin123";
        let treasury_address = "xion1treasury456";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx456hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let grant_config = GrantConfigInput {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            description: "Optional grant".to_string(),
            authorization: AuthorizationInput::Generic,
            optional: true,
        };

        let result = client
            .add_grant_config(
                &token,
                treasury_address,
                "/cosmos.bank.v1beta1.MsgSend",
                grant_config,
                admin_address,
            )
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.operation, "update");
    }

    // =========================================================================
    // remove_grant_config tests
    // =========================================================================

    #[tokio::test]
    async fn test_remove_grant_config_success() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin789";
        let treasury_address = "xion1treasury999";
        let token = create_test_token(admin_address);
        let type_url = "/cosmos.bank.v1beta1.MsgSend";

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_remove_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client
            .remove_grant_config(&token, treasury_address, type_url, admin_address)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.treasury_address, treasury_address);
        assert_eq!(result.type_url, type_url);
        assert_eq!(result.operation, "remove");
        assert_eq!(result.tx_hash, "tx_remove_hash");
    }

    // =========================================================================
    // list_grant_configs tests
    // =========================================================================

    #[tokio::test]
    async fn test_list_grant_configs_empty_from_indexer() {
        // Note: The indexer does not return grant configs - they require on-chain queries.
        // This test verifies that when indexer returns no grant configs, we get an empty list.
        let mock_server = MockServer::start().await;
        let admin_address = "xion1admin333";
        let treasury_address = "xion1treasury444";
        let token = create_test_token(admin_address);

        // Mock the indexer response - indexer does not return grant configs
        Mock::given(method("GET"))
            .and(path_regex(
                r"/contract/xion1admin333/xion/account/treasuries",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([{
                    "contractAddress": treasury_address,
                    "balances": {"uxion": "0"},
                    "params": {
                        "redirect_url": "https://example.com",
                        "icon_url": "https://example.com/icon.png"
                    }
                }])),
            )
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client.list_grant_configs(&token, treasury_address).await;

        assert!(result.is_ok());
        let configs = result.unwrap();
        // Grant configs are not returned by indexer, so we get empty
        assert!(configs.is_empty());
    }

    // =========================================================================
    // set_fee_config tests
    // =========================================================================

    #[tokio::test]
    async fn test_set_fee_config_basic_success() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1feeadmin";
        let treasury_address = "xion1feetreasury";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_fee_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let fee_config = FeeConfigInput::Basic {
            spend_limit: "1000000000".to_string(),
            description: "Basic fee allowance".to_string(),
        };

        let result = client
            .set_fee_config(&token, treasury_address, fee_config, admin_address)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.treasury_address, treasury_address);
        assert_eq!(result.operation, "update");
        assert_eq!(result.tx_hash, "tx_fee_hash");
    }

    #[tokio::test]
    async fn test_set_fee_config_periodic_success() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1periodic";
        let treasury_address = "xion1periodtreasury";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_periodic_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let fee_config = FeeConfigInput::Periodic {
            basic_spend_limit: Some("100000000".to_string()),
            period_seconds: 3600,
            period_spend_limit: "500000000".to_string(),
            description: "Periodic fee allowance".to_string(),
        };

        let result = client
            .set_fee_config(&token, treasury_address, fee_config, admin_address)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.operation, "update");
    }

    // =========================================================================
    // revoke_allowance tests
    // =========================================================================

    #[tokio::test]
    async fn test_revoke_allowance_success() {
        let mock_server = MockServer::start().await;
        let admin_address = "xion1revokeadmin";
        let treasury_address = "xion1revoketreasury";
        let grantee = "xion1grantee123";
        let token = create_test_token(admin_address);

        Mock::given(method("POST"))
            .and(path("/api/v1/transaction"))
            .respond_with(ResponseTemplate::new(200).set_body_json(BroadcastResponse {
                success: true,
                tx_hash: "tx_revoke_hash".to_string(),
                from: admin_address.to_string(),
                gas_used: None,
                gas_wanted: None,
            }))
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client
            .revoke_allowance(&token, treasury_address, grantee, admin_address)
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.treasury_address, treasury_address);
        assert_eq!(result.operation, "remove");
        assert_eq!(result.tx_hash, "tx_revoke_hash");
    }

    // =========================================================================
    // query_fee_config tests
    // =========================================================================

    #[tokio::test]
    async fn test_query_fee_config_not_found_from_indexer() {
        // Note: The indexer does not return fee config - it requires on-chain queries.
        // This test verifies that when indexer returns no fee config, we get None.
        let mock_server = MockServer::start().await;
        let admin_address = "xion1noadmin";
        let treasury_address = "xion1notreasury";
        let token = create_test_token(admin_address);

        // Mock the indexer response - indexer does not return fee config
        Mock::given(method("GET"))
            .and(path_regex(
                r"/contract/xion1noadmin/xion/account/treasuries",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([{
                    "contractAddress": treasury_address,
                    "balances": {"uxion": "0"},
                    "params": {
                        "redirect_url": "https://example.com",
                        "icon_url": "https://example.com/icon.png"
                    }
                }])),
            )
            .mount(&mock_server)
            .await;

        let client = create_test_client(&mock_server.uri());

        let result = client.query_fee_config(&token, treasury_address).await;

        assert!(result.is_ok());
        let config = result.unwrap();
        // Fee config is not returned by indexer, so we get None
        assert!(config.is_none());
    }
}
