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
use std::str::FromStr;

use crate::utils::output::OutputFormat;

/// Execution context passed to command handlers
#[derive(Debug, Clone)]
pub struct ExecuteContext {
    /// Output format for responses
    pub output_format: OutputFormat,
    /// Network to use (testnet, mainnet)
    pub network: String,
}

impl ExecuteContext {
    /// Create a new execution context
    pub fn new(output_format: OutputFormat, network: String) -> Self {
        Self {
            output_format,
            network,
        }
    }

    /// Create a default context (JSON output, testnet)
    pub fn default_context() -> Self {
        Self::new(OutputFormat::default(), "testnet".to_string())
    }

    /// Get the output format
    pub fn output_format(&self) -> OutputFormat {
        self.output_format
    }

    /// Check if output format is JSON (pretty or compact)
    pub fn is_json(&self) -> bool {
        matches!(
            self.output_format,
            OutputFormat::Json | OutputFormat::JsonCompact
        )
    }

    /// Check if output format is GitHub Actions
    pub fn is_github_actions(&self) -> bool {
        matches!(self.output_format, OutputFormat::GitHubActions)
    }
}

#[derive(Parser)]
#[command(name = "xion-toolkit")]
#[command(about = "CLI-driven, Agent-oriented toolkit for Xion blockchain", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Network to use (testnet, mainnet)
    #[arg(short, long, global = true, default_value = "testnet")]
    pub network: String,

    /// Output format (json, json-compact, github-actions, human)
    #[arg(short, long, global = true, default_value = "json", value_parser = parse_output_format)]
    pub output: OutputFormat,

    /// Path to config file
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

/// Parse output format from string
fn parse_output_format(s: &str) -> Result<OutputFormat, String> {
    OutputFormat::from_str(s)
}

impl Cli {
    /// Create an execution context from CLI options
    pub fn to_context(&self) -> ExecuteContext {
        ExecuteContext::new(self.output, self.network.clone())
    }
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

pub async fn handle_auth_command(cmd: auth::AuthCommands, ctx: &ExecuteContext) -> Result<()> {
    auth::handle_command(cmd, ctx).await
}

pub async fn handle_treasury_command(cmd: treasury::TreasuryCommands, ctx: &ExecuteContext) -> Result<()> {
    treasury::handle_command(cmd, ctx).await
}

pub fn handle_config_command(cmd: config::ConfigCommands, ctx: &ExecuteContext) -> Result<()> {
    config::handle_command(cmd, ctx)
}

pub async fn handle_contract_command(cmd: contract::ContractCommands, ctx: &ExecuteContext) -> Result<()> {
    contract::handle_command(cmd, ctx).await
}

pub async fn handle_account_command(cmd: account::AccountCommands, ctx: &ExecuteContext) -> Result<()> {
    account::handle_command(cmd, ctx).await
}

pub async fn handle_batch_command(cmd: batch::BatchCommands, ctx: &ExecuteContext) -> Result<()> {
    batch::handle_command(cmd, ctx).await
}

pub async fn handle_asset_command(cmd: asset::AssetCommands, ctx: &ExecuteContext) -> Result<()> {
    asset::handle_command(cmd, ctx).await
}

pub async fn handle_tx_command(cmd: tx::TxCommands, ctx: &ExecuteContext) -> Result<()> {
    tx::handle_command(cmd, ctx).await
}

pub fn handle_status_command(ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_formatted;

    let config_manager = ConfigManager::new()?;
    let status = config_manager.get_status()?;

    print_formatted(&status, ctx.output_format())
}
