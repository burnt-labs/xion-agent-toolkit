//! Asset Builder Module
//!
//! CW721 NFT contract deployment and management functionality.
//!
//! ## Features
//!
//! - Create NFT collections (instantiate2 for predictable addresses)
//! - Mint tokens in collections
//! - Query contract state
//!
//! ## Supported Asset Types
//!
//! | Type | Code ID (Testnet) | Description |
//! |------|-------------------|-------------|
//! | cw721-base | 522 | Standard NFT |
//! | cw721-metadata-onchain | 525 | On-chain metadata |
//! | cw721-expiration | 523 | Time-based expiry |
//! | cw721-fixed-price | 524 | Requires CW20 (deferred) |
//! | cw721-non-transferable | 526 | Soulbound |
//! | cw2981-royalties | 528 | Royalties at mint time |
//!
//! ## Usage
//!
//! ```no_run
//! use xion_agent_toolkit::asset_builder::{
//!     AssetBuilderManager, AssetType, CreateCollectionInput, MintTokenInput,
//! };
//! use xion_agent_toolkit::oauth::OAuthClient;
//! use xion_agent_toolkit::config::NetworkConfig;
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! # let config = NetworkConfig {
//! #     network_name: "testnet".to_string(),
//! #     oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
//! #     rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
//! #     chain_id: "xion-testnet-2".to_string(),
//! #     oauth_client_id: "client-id".to_string(),
//! #     treasury_code_id: 1260,
//! #     callback_port: 54321,
//! #     indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
//! #     cw721_base_code_id: 522,
//! #     cw721_metadata_onchain_code_id: 525,
//! #     cw721_expiration_code_id: 523,
//! #     cw721_fixed_price_code_id: 524,
//! #     cw721_non_transferable_code_id: 526,
//! #     cw2981_royalties_code_id: 528,
//! # };
//! let oauth_client = OAuthClient::new(config.clone())?;
//! let manager = AssetBuilderManager::new(oauth_client, config);
//!
//! // Create collection
//! let input = CreateCollectionInput {
//!     asset_type: AssetType::Cw721Base,
//!     name: "My NFT".to_string(),
//!     symbol: "NFT".to_string(),
//!     minter: None,
//!     code_id: None,
//!     salt: None,
//! };
//! let result = manager.create_collection(input).await?;
//! println!("Created collection: {}", result.contract_address);
//!
//! # Ok(())
//! # }
//! ```

pub mod code_ids;
pub mod manager;
pub mod types;

// Re-export commonly used types (public API)
pub use manager::AssetBuilderManager;
pub use types::{AssetType, CreateCollectionInput, MintTokenInput};
