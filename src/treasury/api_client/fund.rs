//! Fund and withdraw operations for treasury contracts.
//!
//! This module provides methods for:
//! - Funding treasury contracts by sending tokens
//! - Withdrawing tokens from treasury contracts

use tracing::{debug, instrument};

use crate::shared::error::XionResult;
use crate::treasury::types::{BroadcastRequest, BroadcastResponse};

use super::helpers::parse_coin;

impl super::TreasuryApiClient {
    /// Fund a treasury contract
    ///
    /// Sends tokens from the authenticated user's wallet to a treasury contract.
    /// This creates a MsgSend transaction and broadcasts it to the blockchain.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address to fund
    /// * `amount` - Amount to send (e.g., "1000000uxion")
    /// * `from_address` - Sender's MetaAccount address
    ///
    /// # Returns
    /// Transaction broadcast response with tx_hash
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::TreasuryApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://api.testnet-2.burnt.com".to_string(),
    /// );
    /// let result = client.fund_treasury(
    ///     "access_token_123",
    ///     "xion1treasury...",
    ///     "1000000uxion",
    ///     "xion1sender..."
    /// ).await?;
    /// println!("Fund transaction hash: {}", result.tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn fund_treasury(
        &self,
        access_token: &str,
        treasury_address: &str,
        amount: &str,
        from_address: &str,
    ) -> XionResult<BroadcastResponse> {
        debug!(
            "Funding treasury {} with {} from {}",
            treasury_address, amount, from_address
        );

        // Parse amount (e.g., "1000000uxion" -> amount: "1000000", denom: "uxion")
        let (amount_val, denom) = parse_coin(amount)?;

        let request = BroadcastRequest {
            messages: vec![crate::treasury::types::TransactionMessage {
                type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                value: serde_json::json!({
                    "fromAddress": from_address,
                    "toAddress": treasury_address,
                    "amount": [{ "amount": amount_val, "denom": denom }]
                }),
            }],
            memo: Some(format!("Fund treasury {}", treasury_address)),
        };

        self.broadcast_transaction(access_token, request).await
    }

    /// Withdraw from a treasury contract
    ///
    /// Withdraws tokens from a treasury contract to the admin's wallet.
    /// This creates a MsgExecuteContract transaction calling the Withdraw message.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `treasury_address` - Treasury contract address to withdraw from
    /// * `amount` - Amount to withdraw (e.g., "1000000uxion")
    /// * `from_address` - Sender's MetaAccount address (must be the admin)
    ///
    /// # Returns
    /// Transaction broadcast response with tx_hash
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::treasury::TreasuryApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new(
    ///     "https://oauth2.testnet.burnt.com".to_string(),
    ///     "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    ///     "https://api.testnet-2.burnt.com".to_string(),
    /// );
    /// let result = client.withdraw_treasury(
    ///     "access_token_123",
    ///     "xion1treasury...",
    ///     "1000000uxion",
    ///     "xion1admin..."
    /// ).await?;
    /// println!("Withdraw transaction hash: {}", result.tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn withdraw_treasury(
        &self,
        access_token: &str,
        treasury_address: &str,
        amount: &str,
        from_address: &str,
    ) -> XionResult<BroadcastResponse> {
        debug!(
            "Withdrawing {} from treasury {} to {}",
            amount, treasury_address, from_address
        );

        // Parse amount
        let (amount_val, denom) = parse_coin(amount)?;

        // Create the Withdraw execute message
        let withdraw_msg = serde_json::json!({
            "withdraw": {
                "coins": [{ "amount": amount_val, "denom": denom }]
            }
        });

        // Use the unified broadcast_execute_contract method
        let tx_hash = self
            .broadcast_execute_contract(
                access_token,
                from_address,
                treasury_address,
                &withdraw_msg,
                None,
                &format!("Withdraw from treasury {}", treasury_address),
            )
            .await?;

        // Return BroadcastResponse format for backward compatibility
        Ok(BroadcastResponse {
            success: true,
            tx_hash,
            from: from_address.to_string(),
            gas_used: None,
            gas_wanted: None,
        })
    }
}
