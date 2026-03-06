//! API clients for Xion services
//!
//! This module provides client implementations for various Xion APIs:
//! - OAuth2 API Service for authentication
//! - Xion daemon (xiond) for blockchain queries
//!
//! Note: Treasury API client is located in the `treasury` module.

pub mod oauth2_api;

#[allow(unused_imports)]
pub use oauth2_api::OAuth2ApiClient;

// Re-export TreasuryApiClient for convenience in doc examples
#[allow(unused_imports)]
pub use crate::treasury::TreasuryApiClient;