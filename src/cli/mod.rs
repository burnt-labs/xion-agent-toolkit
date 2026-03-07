pub mod auth;
pub mod config;
pub mod treasury;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xion-toolkit")]
#[command(about = "CLI-driven, Agent-oriented toolkit for Xion blockchain", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Network to use (local, testnet, mainnet)
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

pub fn handle_status_command() -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_json;

    let config_manager = ConfigManager::new()?;
    let status = config_manager.get_status()?;

    print_json(&status)
}
