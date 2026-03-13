//! Account Types
//!
//! Data structures for MetaAccount (Smart Account) queries.

use serde::{Deserialize, Serialize};

/// Smart Account (MetaAccount) information from DaoDao Indexer
///
/// Note: We indexer at da daodaoindexer.burnt.com may return either:
/// - REST API: /contract/{address}/xion/account/smart/{account_id}
/// - GraphQL: /graphql (but only REST is supported)
///
/// For simplicity, we use REST API first.

/// Smart Account information from indexer REST API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartAccount {
    /// MetaAccount address (bech32)
    pub id: String,
    /// Latest authenticator ID
    #[serde(rename = "latestAuthenticatorId")]
    pub latest_authenticator_id: Option<String>,
    /// Associated authenticators
    pub authenticators: AuthenticatorsConnection,
}

/// Authenticators connection (pagination wrapper)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatorsConnection {
    /// List of authenticator nodes
    pub nodes: Vec<Authenticator>,
}
/// Single authenticator entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authenticator {
    /// Unique authenticator ID
    pub id: String,
    /// Authenticator type (e.g., "secp256k1")
    #[serde(rename = "type")]
    pub auth_type: String,
    /// Authenticator public key or identifier
    pub authenticator: String,
    /// Index of this authenticator
    #[serde(rename = "authenticatorIndex")]
    pub authenticator_index: i32,
    /// Authenticator version
    pub version: i32,
}

/// CLI output for account info command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfoOutput {
    /// Success flag
    pub success: bool,
    /// MetaAccount address
    pub address: String,
    /// List of authenticators (display-friendly)
    pub authenticators: Vec<AuthenticatorDisplay>,
    /// Latest authenticator ID
    pub latest_authenticator_id: Option<String>,
}
/// Display-friendly authenticator info (with truncated authenticator string)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatorDisplay {
    /// Unique authenticator ID
    pub id: String,
    /// Authenticator type
    #[serde(rename = "type")]
    pub auth_type: String,
    /// Authenticator public key (truncated for display)
    pub authenticator: String,
    /// Index of this authenticator
    #[serde(rename = "authenticatorIndex")]
    pub authenticator_index: i32,
    /// Authenticator version
    pub version: i32,
}

/// Maximum display length for authenticator strings
const AUTHENTICATOR_DISPLAY_MAX_LEN: usize = 29;

impl From<SmartAccount> for AccountInfoOutput {
    fn from(account: SmartAccount) -> Self {
        let authenticators: Vec<AuthenticatorDisplay> = account
            .authenticators
            .nodes
            .into_iter()
            .map(|a| AuthenticatorDisplay {
                id: a.id,
                auth_type: a.auth_type,
                authenticator: if a.authenticator.len() > AUTHENTICATOR_DISPLAY_MAX_LEN {
                    format!("{}...", &a.authenticator[..AUTHENTICATOR_DISPLAY_MAX_LEN])
                } else {
                    a.authenticator
                },
                authenticator_index: a.authenticator_index,
                version: a.version,
            })
            .collect();

        Self {
            success: true,
            address: account.id,
            authenticators,
            latest_authenticator_id: account.latest_authenticator_id,
        }
    }
}

/// GraphQL response for SingleSmartWalletQuery (only used as fallback)
#[derive(Debug, Clone, Deserialize)]
pub struct SmartAccountQueryResponse {
    /// The smart account data
    #[serde(rename = "smartAccount")]
    pub smart_account: Option<SmartAccount>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_account_deserialization() {
        let json = r#"{
            "id": "xion1abc123",
            "latestAuthenticatorId": "auth_001",
            "authenticators": {
                "nodes": [
                    {
                        "id": "auth_001",
                        "type": "secp256k1",
                        "authenticator": "MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAE",
                        "authenticatorIndex": 0,
                        "version": 1
                    }
                ]
            }
        }"#;

        let account: SmartAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.id, "xion1abc123");
        assert_eq!(account.authenticators.nodes.len(), 1);
        assert_eq!(account.authenticators.nodes[0].auth_type, "secp256k1");
    }

    #[test]
    fn test_account_info_output_from_smart_account() {
        let account = SmartAccount {
            id: "xion1abc".to_string(),
            latest_authenticator_id: Some("auth_001".to_string()),
            authenticators: AuthenticatorsConnection {
                nodes: vec![Authenticator {
                    id: "auth_001".to_string(),
                    auth_type: "secp256k1".to_string(),
                    authenticator: "short_key".to_string(),
                    authenticator_index: 0,
                    version: 1,
                }],
            },
        };

        let output: AccountInfoOutput = account.into();
        assert!(output.success);
        assert_eq!(output.address, "xion1abc");
        assert_eq!(output.authenticators.len(), 1);
    }

    #[test]
    fn test_authenticator_truncation() {
        let account = SmartAccount {
            id: "xion1abc".to_string(),
            latest_authenticator_id: None,
            authenticators: AuthenticatorsConnection {
                nodes: vec![Authenticator {
                    id: "auth_001".to_string(),
                    auth_type: "secp256k1".to_string(),
                    authenticator: "a".repeat(100),
                    authenticator_index: 0,
                    version: 1,
                }],
            },
        };

        let output: AccountInfoOutput = account.into();
        // Should be truncated to 29 chars + "..."
        assert_eq!(output.authenticators[0].authenticator.len(), 32);
        assert!(output.authenticators[0].authenticator.ends_with("..."));
    }
}
