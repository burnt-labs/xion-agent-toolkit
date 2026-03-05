//! Treasury Data Types
//!
//! Data structures for Treasury contract information and operations.

use serde::{Deserialize, Serialize};

/// Treasury list item (simplified view)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryListItem {
    /// Treasury contract address
    pub address: String,
    /// Admin address
    pub admin: Option<String>,
    /// Treasury balance in uxion
    pub balance: String,
    /// Display name
    #[serde(default)]
    pub name: Option<String>,
    /// Creation timestamp
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Complete treasury information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryInfo {
    /// Treasury contract address
    pub address: String,
    /// Admin address
    pub admin: Option<String>,
    /// Treasury balance in uxion
    pub balance: String,
    /// Treasury parameters
    pub params: TreasuryParams,
    /// Fee grant configuration
    #[serde(default)]
    pub fee_config: Option<FeeConfig>,
    /// Grant configurations
    #[serde(default)]
    pub grant_configs: Option<Vec<GrantConfig>>,
}

/// Treasury parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryParams {
    /// Display URL
    #[serde(default)]
    pub display_url: Option<String>,
    /// Redirect URL
    pub redirect_url: String,
    /// Icon URL
    pub icon_url: String,
    /// Additional metadata
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Fee grant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfig {
    /// Fee grant type (e.g., "basic", "limited")
    #[serde(rename = "type")]
    pub config_type: String,
    /// Maximum spend limit
    pub spend_limit: Option<String>,
    /// Expiration time
    #[serde(default)]
    pub expires_at: Option<String>,
    /// Additional configuration
    #[serde(flatten)]
    pub additional: Option<serde_json::Value>,
}

/// Grant configuration (for Authz)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfig {
    /// Type URL of the message type
    pub type_url: String,
    /// Grant configuration
    pub grant_config: serde_json::Value,
}

/// Query options for treasury details
#[derive(Debug, Clone)]
pub struct QueryOptions {
    /// Include grant configurations
    pub grants: bool,
    /// Include fee configuration
    pub fee: bool,
    /// Include admin information
    pub admin: bool,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            grants: true,
            fee: true,
            admin: true,
        }
    }
}

/// Treasury creation request (for future implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTreasuryRequest {
    /// Fee grant configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_grant: Option<FeeGrantRequest>,
    /// Grant configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_config: Option<GrantConfigRequest>,
    /// Initial funding amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_fund: Option<String>,
}

/// Fee grant request (for treasury creation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeGrantRequest {
    /// Fee grant type
    #[serde(rename = "type")]
    pub grant_type: String,
    /// Spend limit
    pub spend_limit: String,
}

/// Grant config request (for treasury creation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigRequest {
    /// Message type URL
    pub type_url: String,
    /// Grant configuration
    pub config: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treasury_list_item_deserialization() {
        let json = r#"{
            "address": "xion1abc123",
            "admin": "xion1def456",
            "balance": "10000000",
            "name": "My Treasury",
            "created_at": "2024-01-01T00:00:00Z"
        }"#;

        let item: TreasuryListItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.address, "xion1abc123");
        assert_eq!(item.admin, Some("xion1def456".to_string()));
        assert_eq!(item.balance, "10000000");
        assert_eq!(item.name, Some("My Treasury".to_string()));
    }

    #[test]
    fn test_treasury_list_item_minimal() {
        let json = r#"{
            "address": "xion1abc123",
            "balance": "5000000"
        }"#;

        let item: TreasuryListItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.address, "xion1abc123");
        assert_eq!(item.balance, "5000000");
        assert_eq!(item.admin, None);
        assert_eq!(item.name, None);
    }

    #[test]
    fn test_treasury_info_deserialization() {
        let json = r#"{
            "address": "xion1abc123",
            "admin": "xion1def456",
            "balance": "10000000",
            "params": {
                "display_url": "https://myapp.com",
                "redirect_url": "https://myapp.com/callback",
                "icon_url": "https://myapp.com/icon.png"
            },
            "fee_config": {
                "type": "basic",
                "spend_limit": "10000000uxion"
            },
            "grant_configs": [
                {
                    "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
                    "grant_config": {}
                }
            ]
        }"#;

        let info: TreasuryInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.address, "xion1abc123");
        assert_eq!(info.balance, "10000000");
        assert!(info.fee_config.is_some());
        assert!(info.grant_configs.is_some());
    }

    #[test]
    fn test_query_options_default() {
        let options = QueryOptions::default();
        assert!(options.grants);
        assert!(options.fee);
        assert!(options.admin);
    }

    #[test]
    fn test_fee_config_serialization() {
        let config = FeeConfig {
            config_type: "basic".to_string(),
            spend_limit: Some("1000000uxion".to_string()),
            expires_at: None,
            additional: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"type\":\"basic\""));
        assert!(json.contains("\"spend_limit\":\"1000000uxion\""));
    }
}
