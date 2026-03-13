mod account;
mod api;
mod asset_builder;
mod batch;
mod cli;
mod config;
mod oauth;
mod treasury;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    // Set environment variable for network override (used by commands)
    std::env::set_var("XION_NETWORK_OVERRIDE", &cli.network);

    match cli.command {
        Commands::Auth(auth_cmd) => cli::handle_auth_command(auth_cmd).await?,
        Commands::Treasury(treasury_cmd) => cli::handle_treasury_command(*treasury_cmd).await?,
        Commands::Contract(contract_cmd) => cli::handle_contract_command(contract_cmd).await?,
        Commands::Config(config_cmd) => cli::handle_config_command(config_cmd)?,
        Commands::Status => cli::handle_status_command()?,
        Commands::Account(account_cmd) => cli::handle_account_command(account_cmd).await?,
        Commands::Batch(batch_cmd) => cli::handle_batch_command(batch_cmd).await?,
        Commands::Asset(asset_cmd) => cli::handle_asset_command(asset_cmd).await?,
    }

    Ok(())
}
