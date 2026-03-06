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

// Re-export commonly used types and manager (public API)
#[allow(unused_imports)]
pub use api_client::TreasuryApiClient;
pub use manager::TreasuryManager;
#[allow(unused_imports)]
pub use types::{
    BroadcastRequest, BroadcastResponse, CreateTreasuryRequest, FeeConfig, FeeConfigInput,
    FeeConfigMessage, FeeGrantRequest, FundResult, GrantConfig, GrantConfigInput,
    GrantConfigMessage, GrantConfigRequest, QueryOptions, TransactionMessage,
    TreasuryCreateRequest, TreasuryInfo, TreasuryListItem, TreasuryParams, TreasuryParamsInput,
    TreasuryParamsMessage, TypeUrlValue, WithdrawResult,
};
