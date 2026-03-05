mod cli;
mod config;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use anyhow::Result;
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

    match cli.command {
        Commands::Auth(auth_cmd) => cli::handle_auth_command(auth_cmd).await?,
        Commands::Treasury(treasury_cmd) => cli::handle_treasury_command(treasury_cmd).await?,
        Commands::Config(config_cmd) => cli::handle_config_command(config_cmd)?,
        Commands::Status => cli::handle_status_command()?,
    }

    Ok(())
}
