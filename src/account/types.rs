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

}
