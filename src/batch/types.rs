//! Batch Types
//!
//! Data structures for batch operations.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Maximum number of messages allowed in a single batch
pub const MAX_BATCH_SIZE: usize = 50;

/// Batch request containing multiple messages
///
/// # Example
///
/// ```json
/// {
///     "messages": [
///         {
///             "typeUrl": "/cosmos.bank.v1beta1.MsgSend",
///             "value": {
///                 "toAddress": "xion1...",
///                 "amount": [{ "denom": "uxion", "amount": "1000000" }]
///             }
///         }
///     ],
///     "memo": "Optional memo"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequest {
    /// List of messages to execute in the batch
    pub messages: Vec<BatchMessage>,

    /// Optional transaction memo
    #[serde(default)]
    pub memo: Option<String>,
}

impl BatchRequest {
    /// Validate the batch request
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The batch contains no messages
    /// - The batch exceeds the maximum size (50 messages)
    /// - Any message has an empty type_url
    pub fn validate(&self) -> Result<(), BatchValidationError> {
        // Check for empty messages
        if self.messages.is_empty() {
            return Err(BatchValidationError::EmptyBatch);
        }

        // Check for batch size limit
        if self.messages.len() > MAX_BATCH_SIZE {
            return Err(BatchValidationError::BatchTooLarge {
                count: self.messages.len(),
                max: MAX_BATCH_SIZE,
            });
        }

        // Validate each message
        for (index, msg) in self.messages.iter().enumerate() {
            if msg.type_url.is_empty() {
                return Err(BatchValidationError::InvalidMessage {
                    index,
                    reason: "type_url cannot be empty".to_string(),
                });
            }
            if !msg.type_url.starts_with('/') {
                return Err(BatchValidationError::InvalidMessage {
                    index,
                    reason: format!("type_url must start with '/': {}", msg.type_url),
                });
            }
        }

        Ok(())
    }

    /// Load batch request from a JSON file
    ///
    /// # Arguments
    /// * `path` - Path to the JSON file
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read batch file: {}", e))?;
        let request: BatchRequest = serde_json::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Invalid batch file format: {}", e))?;
        Ok(request)
    }
}

/// Single message in a batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMessage {
    /// Protobuf type URL (e.g., "/cosmos.bank.v1beta1.MsgSend")
    #[serde(rename = "typeUrl", alias = "type_url")]
    pub type_url: String,

    /// Message value as JSON object
    pub value: serde_json::Value,
}

/// Result of a batch execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    /// Success status
    pub success: bool,

    /// Transaction hash (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,

    /// Sender address
    pub from: String,

    /// Gas used (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_used: Option<String>,

    /// Gas wanted (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_wanted: Option<String>,

    /// Number of messages in the batch
    pub message_count: usize,
}

/// Validation error for batch requests
#[derive(Debug, Error)]
pub enum BatchValidationError {
    /// The batch contains no messages
    #[error("Batch must contain at least one message")]
    EmptyBatch,

    /// The batch exceeds the maximum size
    #[error("Batch too large: {count} messages (max {max})")]
    BatchTooLarge {
        /// Actual number of messages
        count: usize,
        /// Maximum allowed messages
        max: usize,
    },

    /// A message in the batch is invalid
    #[error("Invalid message at index {index}: {reason}")]
    InvalidMessage {
        /// Index of the invalid message
        index: usize,
        /// Reason for the validation failure
        reason: String,
    },
}

/// Validation result for batch files
#[derive(Debug, Clone, Serialize)]
pub struct BatchValidationResult {
    /// Whether validation passed
    pub valid: bool,

    /// Number of messages in the batch
    pub message_count: usize,

    /// List of validation errors (if any)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,

    /// List of message type URLs in the batch
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub message_types: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_request_validate_empty() {
        let request = BatchRequest {
            messages: vec![],
            memo: None,
        };
        assert!(matches!(
            request.validate(),
            Err(BatchValidationError::EmptyBatch)
        ));
    }

    #[test]
    fn test_batch_request_validate_too_large() {
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

        match request.validate() {
            Err(BatchValidationError::BatchTooLarge { count, max }) => {
                assert_eq!(count, 51);
                assert_eq!(max, 50);
            }
            _ => panic!("Expected BatchTooLarge error"),
        }
    }

    #[test]
    fn test_batch_request_validate_empty_type_url() {
        let request = BatchRequest {
            messages: vec![BatchMessage {
                type_url: "".to_string(),
                value: serde_json::json!({}),
            }],
            memo: None,
        };

        match request.validate() {
            Err(BatchValidationError::InvalidMessage { index, reason }) => {
                assert_eq!(index, 0);
                assert!(reason.contains("type_url"));
            }
            _ => panic!("Expected InvalidMessage error"),
        }
    }

    #[test]
    fn test_batch_request_validate_type_url_without_slash() {
        let request = BatchRequest {
            messages: vec![BatchMessage {
                type_url: "cosmos.bank.v1beta1.MsgSend".to_string(),
                value: serde_json::json!({}),
            }],
            memo: None,
        };

        match request.validate() {
            Err(BatchValidationError::InvalidMessage { index, reason }) => {
                assert_eq!(index, 0);
                assert!(reason.contains("must start with '/'"));
            }
            _ => panic!("Expected InvalidMessage error for type_url without leading slash"),
        }
    }

    #[test]
    fn test_batch_request_validate_success() {
        let request = BatchRequest {
            messages: vec![
                BatchMessage {
                    type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                    value: serde_json::json!({}),
                },
                BatchMessage {
                    type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
                    value: serde_json::json!({}),
                },
            ],
            memo: Some("Test batch".to_string()),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_batch_message_alias() {
        // Test both typeUrl and type_url formats
        let json1 = r#"{"typeUrl": "/cosmos.bank.v1beta1.MsgSend", "value": {}}"#;
        let msg1: BatchMessage = serde_json::from_str(json1).unwrap();
        assert_eq!(msg1.type_url, "/cosmos.bank.v1beta1.MsgSend");

        let json2 = r#"{"type_url": "/cosmos.bank.v1beta1.MsgSend", "value": {}}"#;
        let msg2: BatchMessage = serde_json::from_str(json2).unwrap();
        assert_eq!(msg2.type_url, "/cosmos.bank.v1beta1.MsgSend");
    }

    #[test]
    fn test_batch_validation_result_serialization() {
        let result = BatchValidationResult {
            valid: true,
            message_count: 2,
            errors: vec![],
            message_types: vec![
                "/cosmos.bank.v1beta1.MsgSend".to_string(),
                "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
            ],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"valid\":true"));
        assert!(json.contains("\"message_count\":2"));
    }

    #[test]
    fn test_batch_validation_result_with_errors() {
        let result = BatchValidationResult {
            valid: false,
            message_count: 0,
            errors: vec!["Batch must contain at least one message".to_string()],
            message_types: vec![],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"valid\":false"));
        assert!(json.contains("\"errors\""));
    }
}
