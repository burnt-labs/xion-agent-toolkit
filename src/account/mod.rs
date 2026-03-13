//! Account Module
//!
//! MetaAccount (Smart Account) queries via DaoDao Indexer.

pub mod client;
pub mod types;

pub use client::AccountClient;
pub use types::{AccountInfoOutput, Authenticator, AuthenticatorDisplay, SmartAccount};
