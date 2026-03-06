//! Treasury management module
//!
//! This module provides treasury management functionality including:
//! - Treasury data structures
//! - Treasury API client
//! - Treasury manager (high-level API)
//! - Cache for treasury data

pub mod api_client;
pub mod cache;
pub mod manager;
pub mod types;

// Re-export commonly used types and manager
pub use api_client::TreasuryApiClient;
pub use cache::TreasuryCache;
pub use manager::TreasuryManager;
pub use types::{
    BroadcastRequest, BroadcastResponse, CreateTreasuryRequest, FeeConfig, FeeGrantRequest,
    FundResult, GrantConfig, GrantConfigRequest, QueryOptions, TransactionMessage,
    TreasuryInfo, TreasuryListItem, TreasuryParams, WithdrawResult,
};
