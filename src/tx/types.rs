//! Transaction Types
//!
//! Types for transaction monitoring and status tracking.

use serde::{Deserialize, Serialize};

/// Transaction status variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TxStatus {
    /// Transaction is pending (not yet found on chain)
    Pending,
    /// Transaction was successful
    Success,
    /// Transaction failed
    Failed,
    /// Wait operation timed out
    Timeout,
}

impl TxStatus {
    /// Check if the transaction is in a final state (success, failed, or timeout)
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            TxStatus::Success | TxStatus::Failed | TxStatus::Timeout
        )
    }

    /// Check if the transaction was successful
    pub fn is_success(&self) -> bool {
        matches!(self, TxStatus::Success)
    }

    /// Check if the transaction failed
    pub fn is_failed(&self) -> bool {
        matches!(self, TxStatus::Failed)
    }

    /// Check if the transaction is still pending
    pub fn is_pending(&self) -> bool {
        matches!(self, TxStatus::Pending)
    }
}

impl std::fmt::Display for TxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxStatus::Pending => write!(f, "pending"),
            TxStatus::Success => write!(f, "success"),
            TxStatus::Failed => write!(f, "failed"),
            TxStatus::Timeout => write!(f, "timeout"),
        }
    }
}

/// Transaction information from chain query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInfo {
    /// Transaction hash (hex format)
    pub tx_hash: String,
    /// Current status of the transaction
    pub status: TxStatus,
    /// Block height where transaction was included (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// Timestamp when transaction was included (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Gas used by the transaction (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_used: Option<u64>,
    /// Error message if transaction failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl TxInfo {
    /// Create a new pending transaction info
    pub fn pending(tx_hash: impl Into<String>) -> Self {
        Self {
            tx_hash: tx_hash.into(),
            status: TxStatus::Pending,
            height: None,
            timestamp: None,
            gas_used: None,
            error: None,
        }
    }

    /// Create a new successful transaction info
    pub fn success(
        tx_hash: impl Into<String>,
        height: u64,
        timestamp: impl Into<String>,
        gas_used: u64,
    ) -> Self {
        Self {
            tx_hash: tx_hash.into(),
            status: TxStatus::Success,
            height: Some(height),
            timestamp: Some(timestamp.into()),
            gas_used: Some(gas_used),
            error: None,
        }
    }

    /// Create a new failed transaction info
    pub fn failed(tx_hash: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            tx_hash: tx_hash.into(),
            status: TxStatus::Failed,
            height: None,
            timestamp: None,
            gas_used: None,
            error: Some(error.into()),
        }
    }
}

/// Result of a wait operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxWaitResult {
    /// Final status after waiting
    pub status: TxStatus,
    /// Transaction information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_info: Option<TxInfo>,
    /// Total wait time in milliseconds
    pub wait_time_ms: u64,
}

impl TxWaitResult {
    /// Create a new successful wait result
    pub fn success(tx_info: TxInfo, wait_time_ms: u64) -> Self {
        Self {
            status: TxStatus::Success,
            tx_info: Some(tx_info),
            wait_time_ms,
        }
    }

    /// Create a new timeout result
    pub fn timeout(wait_time_ms: u64) -> Self {
        Self {
            status: TxStatus::Timeout,
            tx_info: None,
            wait_time_ms,
        }
    }

    /// Create a new failed wait result
    pub fn failed(tx_info: TxInfo, wait_time_ms: u64) -> Self {
        Self {
            status: TxStatus::Failed,
            tx_info: Some(tx_info),
            wait_time_ms,
        }
    }
}

/// RPC response structure for transaction query (Tendermint RPC format - deprecated)
#[derive(Debug, Clone, Deserialize)]
pub struct RpcTxResponse {
    /// Transaction result information
    #[serde(rename = "tx_result")]
    pub tx_result: Option<RpcTxResult>,
    /// Transaction hash
    pub hash: Option<String>,
    /// Block height
    pub height: Option<String>,
}

/// RPC transaction result structure (Tendermint RPC format - deprecated)
#[derive(Debug, Clone, Deserialize)]
pub struct RpcTxResult {
    /// Transaction execution code (0 = success)
    pub code: i32,
    /// Log messages
    pub log: Option<String>,
    /// Gas used
    #[serde(rename = "gas_used")]
    pub gas_used: Option<String>,
    /// Gas wanted
    #[serde(rename = "gas_wanted")]
    pub gas_wanted: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
    /// Events emitted during transaction execution
    pub events: Option<Vec<serde_json::Value>>,
}

/// Cosmos SDK REST API response for transaction query
/// Endpoint: /cosmos/tx/v1beta1/txs/{hash}
#[derive(Debug, Clone, Deserialize)]
pub struct CosmosTxResponse {
    /// Transaction response data
    pub tx_response: CosmosTxResponseData,
}

/// Transaction response data from Cosmos SDK REST API
#[derive(Debug, Clone, Deserialize)]
pub struct CosmosTxResponseData {
    /// Block height where transaction was included
    pub height: String,
    /// Transaction hash
    pub txhash: String,
    /// Transaction execution code (0 = success)
    pub code: u32,
    /// Codespace for error
    #[serde(default)]
    pub codespace: String,
    /// Gas requested
    #[serde(rename = "gas_wanted")]
    pub gas_wanted: Option<String>,
    /// Gas used
    #[serde(rename = "gas_used")]
    pub gas_used: Option<String>,
    /// Timestamp when transaction was included (ISO 8601)
    pub timestamp: Option<String>,
    /// Raw log (error message if failed)
    #[serde(rename = "raw_log", default)]
    pub raw_log: String,
    /// Additional info
    #[serde(default)]
    pub info: String,
    /// Events emitted during transaction execution
    #[serde(default)]
    pub events: Vec<serde_json::Value>,
}

/// RPC error response
#[derive(Debug, Clone, Deserialize)]
pub struct RpcErrorResponse {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Error data (optional)
    pub data: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tx_status_is_final() {
        assert!(!TxStatus::Pending.is_final());
        assert!(TxStatus::Success.is_final());
        assert!(TxStatus::Failed.is_final());
        assert!(TxStatus::Timeout.is_final());
    }

    #[test]
    fn test_tx_status_checks() {
        assert!(TxStatus::Success.is_success());
        assert!(!TxStatus::Failed.is_success());
        assert!(!TxStatus::Pending.is_success());

        assert!(TxStatus::Failed.is_failed());
        assert!(!TxStatus::Success.is_failed());
        assert!(!TxStatus::Pending.is_failed());

        assert!(TxStatus::Pending.is_pending());
        assert!(!TxStatus::Success.is_pending());
        assert!(!TxStatus::Failed.is_pending());
    }

    #[test]
    fn test_tx_status_display() {
        assert_eq!(TxStatus::Pending.to_string(), "pending");
        assert_eq!(TxStatus::Success.to_string(), "success");
        assert_eq!(TxStatus::Failed.to_string(), "failed");
        assert_eq!(TxStatus::Timeout.to_string(), "timeout");
    }

    #[test]
    fn test_tx_status_serialization() {
        let pending = TxStatus::Pending;
        let json = serde_json::to_string(&pending).unwrap();
        assert_eq!(json, "\"pending\"");

        let success = TxStatus::Success;
        let json = serde_json::to_string(&success).unwrap();
        assert_eq!(json, "\"success\"");
    }

    #[test]
    fn test_tx_status_deserialization() {
        let pending: TxStatus = serde_json::from_str("\"pending\"").unwrap();
        assert_eq!(pending, TxStatus::Pending);

        let success: TxStatus = serde_json::from_str("\"success\"").unwrap();
        assert_eq!(success, TxStatus::Success);
    }

    #[test]
    fn test_tx_info_pending() {
        let info = TxInfo::pending("ABC123");
        assert_eq!(info.tx_hash, "ABC123");
        assert_eq!(info.status, TxStatus::Pending);
        assert!(info.height.is_none());
        assert!(info.timestamp.is_none());
        assert!(info.gas_used.is_none());
        assert!(info.error.is_none());
    }

    #[test]
    fn test_tx_info_success() {
        let info = TxInfo::success("ABC123", 12345, "2026-03-15T10:00:00Z", 150000);
        assert_eq!(info.tx_hash, "ABC123");
        assert_eq!(info.status, TxStatus::Success);
        assert_eq!(info.height, Some(12345));
        assert_eq!(info.timestamp, Some("2026-03-15T10:00:00Z".to_string()));
        assert_eq!(info.gas_used, Some(150000));
        assert!(info.error.is_none());
    }

    #[test]
    fn test_tx_info_failed() {
        let info = TxInfo::failed("ABC123", "insufficient funds");
        assert_eq!(info.tx_hash, "ABC123");
        assert_eq!(info.status, TxStatus::Failed);
        assert_eq!(info.error, Some("insufficient funds".to_string()));
    }

    #[test]
    fn test_tx_info_serialization() {
        let info = TxInfo::success("ABC123", 12345, "2026-03-15T10:00:00Z", 150000);
        let json = serde_json::to_string_pretty(&info).unwrap();
        assert!(json.contains("\"tx_hash\": \"ABC123\""));
        assert!(json.contains("\"status\": \"success\""));
        assert!(json.contains("\"height\": 12345"));
        assert!(json.contains("\"gas_used\": 150000"));
    }

    #[test]
    fn test_tx_wait_result_success() {
        let tx_info = TxInfo::success("ABC123", 12345, "2026-03-15T10:00:00Z", 150000);
        let result = TxWaitResult::success(tx_info.clone(), 4500);
        assert_eq!(result.status, TxStatus::Success);
        assert_eq!(result.wait_time_ms, 4500);
        assert!(result.tx_info.is_some());
        assert_eq!(result.tx_info.unwrap().tx_hash, "ABC123");
    }

    #[test]
    fn test_tx_wait_result_timeout() {
        let result = TxWaitResult::timeout(60000);
        assert_eq!(result.status, TxStatus::Timeout);
        assert_eq!(result.wait_time_ms, 60000);
        assert!(result.tx_info.is_none());
    }

    #[test]
    fn test_rpc_tx_response_deserialization() {
        let json = r#"{
            "tx_result": {
                "code": 0,
                "log": "",
                "gas_used": "150000",
                "gas_wanted": "200000"
            },
            "hash": "ABC123",
            "height": "12345"
        }"#;

        let response: RpcTxResponse = serde_json::from_str(json).unwrap();
        assert!(response.tx_result.is_some());
        let tx_result = response.tx_result.unwrap();
        assert_eq!(tx_result.code, 0);
        assert_eq!(tx_result.gas_used, Some("150000".to_string()));
    }

    #[test]
    fn test_rpc_tx_response_failed() {
        let json = r#"{
            "tx_result": {
                "code": 5,
                "log": "",
                "error": "insufficient funds"
            },
            "hash": "ABC123",
            "height": "12345"
        }"#;

        let response: RpcTxResponse = serde_json::from_str(json).unwrap();
        let tx_result = response.tx_result.unwrap();
        assert_eq!(tx_result.code, 5);
        assert_eq!(tx_result.error, Some("insufficient funds".to_string()));
    }

    #[test]
    fn test_cosmos_tx_response_deserialization() {
        let json = r#"{
            "tx_response": {
                "height": "13761384",
                "txhash": "F726C1540C8EB9479A59039329AC92417B961FC0BE750314D162F2AE578BC143",
                "code": 0,
                "codespace": "",
                "gas_wanted": "228641",
                "gas_used": "191178",
                "timestamp": "2026-03-18T14:47:00Z",
                "raw_log": "",
                "info": "",
                "events": []
            }
        }"#;

        let response: CosmosTxResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.tx_response.height, "13761384");
        assert_eq!(
            response.tx_response.txhash,
            "F726C1540C8EB9479A59039329AC92417B961FC0BE750314D162F2AE578BC143"
        );
        assert_eq!(response.tx_response.code, 0);
        assert_eq!(response.tx_response.gas_used, Some("191178".to_string()));
        assert_eq!(
            response.tx_response.timestamp,
            Some("2026-03-18T14:47:00Z".to_string())
        );
    }

    #[test]
    fn test_cosmos_tx_response_failed() {
        let json = r#"{
            "tx_response": {
                "height": "12345678",
                "txhash": "ABC123DEF456",
                "code": 5,
                "codespace": "bank",
                "gas_wanted": "200000",
                "gas_used": "50000",
                "timestamp": "2026-03-18T15:00:00Z",
                "raw_log": "insufficient funds",
                "info": "",
                "events": []
            }
        }"#;

        let response: CosmosTxResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.tx_response.code, 5);
        assert_eq!(response.tx_response.codespace, "bank");
        assert_eq!(response.tx_response.raw_log, "insufficient funds");
    }
}
