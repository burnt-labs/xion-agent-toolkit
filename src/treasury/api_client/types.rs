//! Internal types for treasury API client responses.
//!
//! These types are used for deserializing responses from the DaoDao Indexer
//! and RPC endpoints. They are not part of the public API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Treasury list response from API
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TreasuryListResponse {
    /// List of treasuries
    pub treasuries: Vec<crate::treasury::types::TreasuryListItem>,
}

/// Treasury query response from API
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TreasuryQueryResponse {
    /// Treasury details
    pub treasury: crate::treasury::types::TreasuryInfo,
}

/// DaoDao Indexer returns a direct array of treasury items
/// (not wrapped in an object with "treasuries" field)
///
/// Individual treasury item from DaoDao Indexer
/// Matches the actual API response format:
/// ```json
/// {
///   "contractAddress": "xion1...",
///   "balances": {"uxion": "10000000000"},
///   "block": {"height": "...", "timeUnixMs": "..."},
///   "codeId": 1260,
///   "params": {"icon_url": "...", "metadata": "...", "redirect_url": "..."}
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IndexerTreasuryItem {
    /// Treasury contract address
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    /// Balances map (denom -> amount)
    #[serde(default)]
    pub balances: HashMap<String, String>,
    /// Block info (height and timestamp)
    #[serde(default)]
    pub block: Option<IndexerBlockInfo>,
    /// Code ID of the treasury contract
    #[serde(rename = "codeId", default)]
    pub code_id: Option<u64>,
    /// Treasury params
    #[serde(default)]
    pub params: Option<IndexerTreasuryParams>,
}

/// Block info from DaoDao Indexer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IndexerBlockInfo {
    /// Block height
    #[serde(default)]
    pub height: Option<String>,
    /// Block timestamp in milliseconds (Unix epoch)
    #[serde(rename = "timeUnixMs", default)]
    pub time_unix_ms: Option<String>,
}

/// Treasury params from DaoDao Indexer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IndexerTreasuryParams {
    /// Redirect URL for OAuth2 callbacks
    #[serde(default)]
    pub redirect_url: Option<String>,
    /// Icon URL for treasury display
    #[serde(default)]
    pub icon_url: Option<String>,
    /// Metadata JSON string
    #[serde(default)]
    pub metadata: Option<String>,
}

/// Treasury metadata JSON structure
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TreasuryMetadataJson {
    /// Treasury display name
    pub name: Option<String>,
}

/// Code info from the chain
#[derive(Debug, Clone)]
pub struct CodeInfo {
    /// Code ID
    pub code_id: u64,
    /// Creator address
    #[allow(dead_code)]
    pub creator: String,
    /// Checksum (SHA-256 hash of wasm bytecode, lowercase hex)
    pub checksum: String,
}

/// Response from /cosmwasm/wasm/v1/code/{code_id}
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CodeInfoResponse {
    pub code_info: CodeInfoData,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CodeInfoData {
    #[allow(dead_code)]
    pub code_id: String,
    #[allow(dead_code)]
    pub creator: String,
    pub data_hash: String,
}
