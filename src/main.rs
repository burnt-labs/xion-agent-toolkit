use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use xion_agent_toolkit::cli;
use xion_agent_toolkit::cli::{Cli, Commands};
use xion_agent_toolkit::shared::exit_codes::exit_code;
use xion_agent_toolkit::shared::mainnet::{is_mainnet_disabled, print_mainnet_disabled_error};
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

    // Try normal parse first; if missing required args and TTY, prompt interactively
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(_) => {
            match cli::interactive_fallback::try_interactive_parse(
                &std::env::args().collect::<Vec<_>>(),
            ) {
                Some(supplemented_args) => Cli::parse_from(supplemented_args),
                None => {
                    // Not interactive or not a missing-arg error — let clap print its error and exit
                    Cli::parse();
                    unreachable!("Cli::parse() exits on error")
                }
            }
        }
    };
    let ctx = cli.to_context();

    // Set environment variable for network override (used by commands)
    std::env::set_var("XION_NETWORK_OVERRIDE", &ctx.network);

    // Check if mainnet is disabled before processing any commands
    if ctx.network == "mainnet" && is_mainnet_disabled() {
        print_mainnet_disabled_error();
        std::process::exit(exit_code::MAINNET_DISABLED);
    }

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
        Commands::Faucet(faucet_cmd) => cli::handle_faucet_command(faucet_cmd, &ctx).await,
        Commands::OAuth2(oauth2_cmd) => cli::handle_oauth2_command(oauth2_cmd, &ctx).await,
        Commands::Completions { shell, install } => {
            cli::handle_completions_command(shell, install, &ctx)
        }
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
