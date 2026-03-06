//! Treasury management module
//!
//! This module provides treasury management functionality including:
//! - Treasury data structures
//! - Treasury API client
//! - Treasury manager (high-level API)
//! - Cache for treasury data
//! - Encoding utilities for fee grants and authz grants

pub mod api_client;
pub mod cache;
pub mod encoding;
pub mod manager;
pub mod types;

// Re-export commonly used types and manager
pub use api_client::TreasuryApiClient;
pub use cache::TreasuryCache;
pub use encoding::{
    parse_coin_string, parse_single_denom, encode_to_base64, Coin, IbcAllocation, ContractGrant,
    EncodingError, encode_basic_allowance, encode_periodic_allowance, encode_allowed_msg_allowance,
    encode_generic_authorization, encode_send_authorization, encode_stake_authorization,
    encode_ibc_transfer_authorization, encode_contract_execution_authorization,
};
pub use manager::TreasuryManager;
pub use types::{
    BroadcastRequest, BroadcastResponse, CreateTreasuryRequest, FeeConfig, FeeConfigMessage,
    FeeGrantRequest, FundResult, GrantConfig, GrantConfigMessage, GrantConfigRequest,
    QueryOptions, TransactionMessage, TreasuryInfo, TreasuryListItem, TreasuryParams,
    TreasuryParamsMessage, TypeUrlValue, WithdrawResult,
};
