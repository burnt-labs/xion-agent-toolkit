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
            expiration: None, // TODO: Add expiration support in FeeConfigInput
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
