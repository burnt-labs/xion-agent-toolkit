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

/// Treasury creation request
///
/// Contains all required parameters to instantiate a new treasury contract.
/// Treasury contracts are created using CosmWasm instantiate2 for predictable addresses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTreasuryRequest {
    /// Admin address (user's MetaAccount address)
    pub admin: String,

    /// Fee grant configuration (required)
    /// Allows the treasury to pay for user transactions
    pub fee_config: FeeConfigMessage,

    /// Grant configurations for Authz (required at least one)
    /// Defines what permissions the treasury can grant to users
    pub grant_configs: Vec<GrantConfigMessage>,

    /// Treasury parameters
    pub params: TreasuryParamsMessage,

    /// Treasury name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Is this an OAuth2 application treasury (optional)
    #[serde(default)]
    pub is_oauth2_app: bool,
}

/// Fee config for treasury instantiation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfigMessage {
    /// Fee allowance type URL
    pub allowance: TypeUrlValue,
    /// Description of the fee grant
    pub description: String,
}

/// Grant config for treasury instantiation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigMessage {
    /// Authorization type URL
    pub authorization: TypeUrlValue,
    /// Description of the grant (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Type URL with base64-encoded value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeUrlValue {
    /// Protobuf type URL
    #[serde(rename = "type_url")]
    pub type_url: String,
    /// Base64-encoded protobuf value
    pub value: String,
}

/// Treasury parameters for instantiation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryParamsMessage {
    /// Redirect URL for OAuth callbacks
    pub redirect_url: String,
    /// Icon URL for display
    pub icon_url: String,
    /// Display URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    /// Additional metadata as JSON object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Treasury creation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTreasuryResult {
    /// New treasury contract address
    pub treasury_address: String,
    /// Transaction hash
    pub tx_hash: String,
    /// Admin address
    pub admin: String,
    /// Creation timestamp
    pub created_at: String,
}

/// Legacy fee grant request (for backward compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeGrantRequest {
    /// Fee grant type
    #[serde(rename = "type")]
    pub grant_type: String,
    /// Spend limit
    pub spend_limit: String,
}

/// Legacy grant config request (for backward compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigRequest {
    /// Message type URL
    pub type_url: String,
    /// Grant configuration
    pub config: serde_json::Value,
}

// ============================================================================
// Transaction Types
// ============================================================================

/// Transaction message for broadcasting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMessage {
    /// Protobuf type URL (e.g., "/cosmos.bank.v1beta1.MsgSend")
    pub type_url: String,
    /// Message value as JSON object
    pub value: serde_json::Value,
}

/// Transaction broadcast request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastRequest {
    /// List of transaction messages
    pub messages: Vec<TransactionMessage>,
    /// Optional memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Transaction broadcast response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastResponse {
    /// Success status
    pub success: bool,
    /// Transaction hash
    pub tx_hash: String,
    /// Sender address
    pub from: String,
    /// Gas used (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_used: Option<String>,
    /// Gas wanted (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_wanted: Option<String>,
}

/// Coin type for transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    /// Amount (as string to handle large numbers)
    pub amount: String,
    /// Denomination (e.g., "uxion")
    pub denom: String,
}

/// Fund treasury result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundResult {
    /// Treasury address that was funded
    pub treasury_address: String,
    /// Amount funded
    pub amount: String,
    /// Transaction hash
    pub tx_hash: String,
}

/// Withdraw from treasury result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResult {
    /// Treasury address withdrawn from
    pub treasury_address: String,
    /// Amount withdrawn
    pub amount: String,
    /// Transaction hash
    pub tx_hash: String,
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
