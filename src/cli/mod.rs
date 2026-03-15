pub mod account;
pub mod asset;
pub mod auth;
pub mod batch;
pub mod config;
pub mod contract;
pub mod treasury;
pub mod tx;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xion-toolkit")]
#[command(about = "CLI-driven, Agent-oriented toolkit for Xion blockchain", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Network to use (testnet, mainnet)
    #[arg(short, long, global = true, default_value = "testnet")]
    pub network: String,

    /// Output format (json, text)
    #[arg(short, long, global = true, default_value = "json")]
    pub output: String,

    /// Path to config file
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Login using OAuth2 flow
    #[command(subcommand)]
    Auth(auth::AuthCommands),

    /// Treasury management commands
    #[command(subcommand)]
    Treasury(Box<treasury::TreasuryCommands>),

    /// Configuration management commands
    #[command(subcommand)]
    Config(config::ConfigCommands),

    /// Contract instantiation commands
    #[command(subcommand)]
    Contract(contract::ContractCommands),

    /// Account (MetaAccount) queries
    #[command(subcommand)]
    Account(account::AccountCommands),

    /// Batch operations for multiple messages
    #[command(subcommand)]
    Batch(batch::BatchCommands),

    /// Asset (NFT) management commands
    #[command(subcommand)]
    Asset(asset::AssetCommands),

    /// Transaction monitoring commands
    #[command(subcommand)]
    Tx(tx::TxCommands),

    /// Show current status (network, auth, etc.)
    Status,
}

pub async fn handle_auth_command(cmd: auth::AuthCommands) -> Result<()> {
    auth::handle_command(cmd).await
}

pub async fn handle_treasury_command(cmd: treasury::TreasuryCommands) -> Result<()> {
    treasury::handle_command(cmd).await
}

pub fn handle_config_command(cmd: config::ConfigCommands) -> Result<()> {
    config::handle_command(cmd)
}

pub async fn handle_contract_command(cmd: contract::ContractCommands) -> Result<()> {
    contract::handle_command(cmd).await
}

pub async fn handle_account_command(cmd: account::AccountCommands) -> Result<()> {
    account::handle_command(cmd).await
}

pub async fn handle_batch_command(cmd: batch::BatchCommands) -> Result<()> {
    batch::handle_command(cmd).await
}

pub async fn handle_asset_command(cmd: asset::AssetCommands) -> Result<()> {
    asset::handle_command(cmd).await
}

pub async fn handle_tx_command(cmd: tx::TxCommands) -> Result<()> {
    tx::handle_command(cmd).await
}

pub fn handle_status_command() -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_json;

    let config_manager = ConfigManager::new()?;
    let status = config_manager.get_status()?;

    print_json(&status)
}
