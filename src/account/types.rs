//! Account Types
//!
//! Data structures for MetaAccount (Smart Account) information.
//! These types align with the OAuth2 API `/api/v1/me` response format.

use serde::{Deserialize, Serialize};

use crate::api::UserInfo;

/// CLI output for account info command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfoOutput {
    /// Success flag
    pub success: bool,
    /// MetaAccount address
    pub address: String,
    /// List of authenticators
    pub authenticators: Vec<AuthenticatorOutput>,
    /// Account balances
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balances: Option<BalancesOutput>,
}

/// Authenticator output format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatorOutput {
    /// Unique authenticator ID
    pub id: String,
    /// Authenticator type (e.g., "secp256k1")
    #[serde(rename = "type")]
    pub auth_type: String,
    /// Authenticator index
    pub index: u32,
    /// Authenticator data
    pub data: serde_json::Value,
}

/// Balances output format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesOutput {
    /// Xion balance
    pub xion: BalanceOutput,
    /// USDC balance
    pub usdc: BalanceOutput,
}

/// Balance output format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceOutput {
    /// Human-readable amount
    pub amount: String,
    /// Denomination
    pub denom: String,
    /// Micro amount (smallest unit)
    pub micro_amount: String,
}

impl From<UserInfo> for AccountInfoOutput {
    fn from(user_info: UserInfo) -> Self {
        let authenticators: Vec<AuthenticatorOutput> = user_info
            .authenticators
            .into_iter()
            .map(|a| AuthenticatorOutput {
                id: a.id,
                auth_type: a.auth_type,
                index: a.index,
                data: a.data,
            })
            .collect();

        let balances = user_info.balances.map(|b| BalancesOutput {
            xion: BalanceOutput {
                amount: b.xion.amount,
                denom: b.xion.denom,
                micro_amount: b.xion.micro_amount,
            },
            usdc: BalanceOutput {
                amount: b.usdc.amount,
                denom: b.usdc.denom,
                micro_amount: b.usdc.micro_amount,
            },
        });

        Self {
            success: true,
            address: user_info.id,
            authenticators,
            balances,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{AccountBalances, AuthenticatorInfo, Balance};

    #[test]
    fn test_user_info_to_account_info_output() {
        let user_info = UserInfo {
            id: "xion1abc123".to_string(),
            authenticators: vec![AuthenticatorInfo {
                id: "xion1abc123-0".to_string(),
                auth_type: "secp256k1".to_string(),
                index: 0,
                data: serde_json::json!({}),
            }],
            balances: Some(AccountBalances {
                xion: Balance {
                    amount: "100.5".to_string(),
                    denom: "uxion".to_string(),
                    micro_amount: "100500000".to_string(),
                },
                usdc: Balance {
                    amount: "50.0".to_string(),
                    denom: "uusdc".to_string(),
                    micro_amount: "50000000".to_string(),
                },
            }),
        };

        let output: AccountInfoOutput = user_info.into();
        assert!(output.success);
        assert_eq!(output.address, "xion1abc123");
        assert_eq!(output.authenticators.len(), 1);
        assert!(output.balances.is_some());
        let balances = output.balances.unwrap();
        assert_eq!(balances.xion.amount, "100.5");
        assert_eq!(balances.usdc.amount, "50.0");
    }

    #[test]
    fn test_user_info_minimal_to_account_info_output() {
        let user_info = UserInfo {
            id: "xion1abc123".to_string(),
            authenticators: vec![],
            balances: None,
        };

        let output: AccountInfoOutput = user_info.into();
        assert!(output.success);
        assert_eq!(output.address, "xion1abc123");
        assert!(output.authenticators.is_empty());
        assert!(output.balances.is_none());
    }
}
