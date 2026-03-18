//! Transaction Monitoring Module
//!
//! Provides transaction status tracking and waiting capabilities for Xion blockchain.
//!
//! ## Usage
//!
//! ```no_run
//! use xion_agent_toolkit::tx::{TxClient, TxInfo, TxStatus};
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let client = TxClient::new("https://api.xion-testnet-2.burnt.com".to_string());
//!
//! // Check transaction status
//! match client.get_tx("ABC123...").await? {
//!     Some(tx_info) => println!("Status: {:?}", tx_info.status),
//!     None => println!("Transaction pending"),
//! }
//!
//! // Wait for confirmation
//! let result = client.wait_tx("ABC123...", 60, 2).await?;
//! println!("Final status: {:?}", result.status);
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod types;

pub use client::TxClient;
#[allow(unused_imports)]
pub use types::{TxInfo, TxStatus, TxWaitResult};
