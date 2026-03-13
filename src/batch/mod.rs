//! Batch Operations Module
//!
//! Provides functionality for executing multiple messages in a single transaction.
//!
//! ## Overview
//!
//! The batch module enables atomic execution of multiple blockchain messages:
//! - Supports all message types accepted by the OAuth2 API
//! - Maximum 50 messages per batch (configurable limit)
//! - Atomic execution (all-or-nothing)
//! - Optional simulation mode for gas estimation
//!
//! ## Example
//!
//! ```no_run
//! use xion_agent_toolkit::batch::{BatchRequest, BatchExecutor};
//! use xion_agent_toolkit::batch::types::BatchMessage;
//! use xion_agent_toolkit::config::NetworkConfig;
//! use xion_agent_toolkit::oauth::OAuthClient;
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let request = BatchRequest {
//!     messages: vec![
//!         BatchMessage {
//!             type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
//!             value: serde_json::json!({
//!                 "toAddress": "xion1...",
//!                 "amount": [{ "denom": "uxion", "amount": "1000000" }]
//!             }),
//!         },
//!     ],
//!     memo: Some("Batch transaction".to_string()),
//! };
//!
//! // Execute with OAuthClient
//! // let result = executor.execute(&request).await?;
//! # Ok(())
//! # }
//! ```

mod executor;
pub mod types;

pub use executor::{BatchExecutor, BatchExecutorError};
pub use types::{BatchRequest, BatchValidationResult};

#[cfg(test)]
mod tests {
    use super::*;
    use types::{BatchMessage, BatchResult};

    #[test]
    fn test_batch_request_deserialization() {
        let json = r#"{
            "messages": [
                {
                    "typeUrl": "/cosmos.bank.v1beta1.MsgSend",
                    "value": {
                        "toAddress": "xion1abc",
                        "amount": [{ "denom": "uxion", "amount": "1000000" }]
                    }
                }
            ],
            "memo": "Test batch"
        }"#;

        let request: BatchRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.memo, Some("Test batch".to_string()));
    }

    #[test]
    fn test_batch_request_minimal() {
        let json = r#"{
            "messages": [
                {
                    "typeUrl": "/cosmos.bank.v1beta1.MsgSend",
                    "value": {}
                }
            ]
        }"#;

        let request: BatchRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.messages.len(), 1);
        assert!(request.memo.is_none());
    }

    #[test]
    fn test_batch_request_empty_messages() {
        let json = r#"{
            "messages": []
        }"#;

        // Empty messages array deserializes successfully but fails validation
        let request: BatchRequest = serde_json::from_str(json).unwrap();
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_batch_request_too_large() {
        let messages: Vec<BatchMessage> = (0..51)
            .map(|_| BatchMessage {
                type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                value: serde_json::json!({}),
            })
            .collect();

        let request = BatchRequest {
            messages,
            memo: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_batch_request_max_size() {
        let messages: Vec<BatchMessage> = (0..50)
            .map(|_| BatchMessage {
                type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                value: serde_json::json!({}),
            })
            .collect();

        let request = BatchRequest {
            messages,
            memo: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_batch_message_type_url_alternate_format() {
        // Support both typeUrl and type_url for flexibility
        let json = r#"{
            "type_url": "/cosmos.bank.v1beta1.MsgSend",
            "value": {}
        }"#;

        let msg: BatchMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.type_url, "/cosmos.bank.v1beta1.MsgSend");
    }

    #[test]
    fn test_batch_result_serialization() {
        let result = BatchResult {
            success: true,
            tx_hash: Some("ABC123".to_string()),
            from: "xion1sender".to_string(),
            gas_used: Some("150000".to_string()),
            gas_wanted: Some("200000".to_string()),
            message_count: 2,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"tx_hash\":\"ABC123\""));
        assert!(json.contains("\"message_count\":2"));
    }

    #[test]
    fn test_batch_result_error() {
        let result = BatchResult {
            success: false,
            tx_hash: None,
            from: "xion1sender".to_string(),
            gas_used: None,
            gas_wanted: None,
            message_count: 0,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"success\":false"));
        assert!(!json.contains("tx_hash"));
    }
}
