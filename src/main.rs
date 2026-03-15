use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use xion_agent_toolkit::cli;
use xion_agent_toolkit::cli::{Cli, Commands};
use xion_agent_toolkit::shared::exit_codes::exit_code;
use xion_agent_toolkit::utils::output::{anyhow_to_xion_error, print_formatted};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    let ctx = cli.to_context();

    // Set environment variable for network override (used by commands)
    std::env::set_var("XION_NETWORK_OVERRIDE", &ctx.network);

    let result = match cli.command {
        Commands::Auth(auth_cmd) => cli::handle_auth_command(auth_cmd, &ctx).await,
        Commands::Treasury(treasury_cmd) => cli::handle_treasury_command(*treasury_cmd, &ctx).await,
        Commands::Contract(contract_cmd) => cli::handle_contract_command(contract_cmd, &ctx).await,
        Commands::Config(config_cmd) => cli::handle_config_command(config_cmd, &ctx),
        Commands::Status => cli::handle_status_command(&ctx),
        Commands::Account(account_cmd) => cli::handle_account_command(account_cmd, &ctx).await,
        Commands::Batch(batch_cmd) => cli::handle_batch_command(batch_cmd, &ctx).await,
        Commands::Asset(asset_cmd) => cli::handle_asset_command(asset_cmd, &ctx).await,
        Commands::Tx(tx_cmd) => cli::handle_tx_command(tx_cmd, &ctx).await,
    };

    match result {
        Ok(()) => {
            std::process::exit(exit_code::SUCCESS);
        }
        Err(e) => {
            // Convert anyhow error to XionError for proper exit code
            let xion_error = anyhow_to_xion_error(&e);
            let exit_code_value = xion_error.code().exit_code();

            // Print error in appropriate format
            let response = xion_error.to_response();
            if let Err(print_err) = print_formatted(&response, ctx.output_format) {
                eprintln!("Failed to print error: {}", print_err);
            }

            std::process::exit(exit_code_value);
        }
    }
}
