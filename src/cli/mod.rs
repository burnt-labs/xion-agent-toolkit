pub mod account;
pub mod asset;
pub mod auth;
pub mod batch;
pub mod config;
pub mod contract;
pub mod faucet;
pub mod interactive;
pub mod interactive_fallback;
pub mod oauth2_client;
pub mod treasury;
pub mod tx;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;

use crate::utils::output::OutputFormat;

/// Execution context passed to command handlers
#[derive(Debug, Clone)]
pub struct ExecuteContext {
    /// Output format for responses
    pub output_format: OutputFormat,
    /// Network to use (testnet, mainnet)
    pub network: String,
    /// Whether interactive prompts are allowed
    pub interactive: bool,
}

impl ExecuteContext {
    /// Create a new execution context
    pub fn new(output_format: OutputFormat, network: String, interactive: bool) -> Self {
        Self {
            output_format,
            network,
            interactive,
        }
    }

    /// Create a default context (JSON output, testnet)
    pub fn default_context() -> Self {
        Self::new(OutputFormat::default(), "testnet".to_string(), false)
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

    /// Disable interactive prompts (exit on missing required arguments)
    #[arg(long, global = true)]
    pub no_interactive: bool,

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
        ExecuteContext::new(self.output, self.network.clone(), !self.no_interactive)
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

    /// OAuth2 client management commands
    #[command(subcommand, name = "oauth2")]
    OAuth2(oauth2_client::OAuth2Commands),

    /// Faucet commands for testnet tokens
    #[command(subcommand)]
    Faucet(faucet::FaucetCommands),

    /// Generate shell completion scripts
    Completions {
        /// Shell type to generate completions for
        /// If not specified, auto-detects from $SHELL environment variable
        #[arg(value_enum)]
        shell: Option<clap_complete::Shell>,

        /// Install completions to shell profile (instead of printing to stdout)
        #[arg(short, long)]
        install: bool,
    },

    /// Show current status (network, auth, etc.)
    Status,
}

pub async fn handle_auth_command(cmd: auth::AuthCommands, ctx: &ExecuteContext) -> Result<()> {
    auth::handle_command(cmd, ctx).await
}

pub async fn handle_treasury_command(
    cmd: treasury::TreasuryCommands,
    ctx: &ExecuteContext,
) -> Result<()> {
    treasury::handle_command(cmd, ctx).await
}

pub fn handle_config_command(cmd: config::ConfigCommands, ctx: &ExecuteContext) -> Result<()> {
    config::handle_command(cmd, ctx)
}

pub async fn handle_contract_command(
    cmd: contract::ContractCommands,
    ctx: &ExecuteContext,
) -> Result<()> {
    contract::handle_command(cmd, ctx).await
}

pub async fn handle_account_command(
    cmd: account::AccountCommands,
    ctx: &ExecuteContext,
) -> Result<()> {
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

pub async fn handle_faucet_command(
    cmd: faucet::FaucetCommands,
    ctx: &ExecuteContext,
) -> Result<()> {
    faucet::handle_command(cmd, ctx).await
}

pub async fn handle_oauth2_command(
    cmd: oauth2_client::OAuth2Commands,
    ctx: &ExecuteContext,
) -> Result<()> {
    oauth2_client::handle_command(cmd, ctx).await
}

pub fn handle_status_command(ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::utils::output::print_formatted;

    let config_manager = ConfigManager::new()?;
    let status = config_manager.get_status()?;

    print_formatted(&status, ctx.output_format())
}

/// Generate shell completion scripts
pub fn handle_completions_command(
    shell: Option<clap_complete::Shell>,
    install: bool,
    ctx: &ExecuteContext,
) -> Result<()> {
    use clap::CommandFactory;
    use serde::Serialize;

    /// Shell detection and installation info
    #[derive(Serialize)]
    struct InstallResult {
        success: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        shell: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        completion_file: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        profile_file: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        already_installed: Option<bool>,
        message: String,
    }

    /// Detect shell from $SHELL environment variable
    fn detect_shell() -> Result<clap_complete::Shell> {
        let shell_env = std::env::var("SHELL").map_err(|_| {
            anyhow::anyhow!(
                "Could not detect shell from $SHELL. Please specify: bash, zsh, fish, or powershell"
            )
        })?;

        // Extract the shell name from the path
        let shell_name = shell_env
            .rsplit('/')
            .next()
            .unwrap_or(&shell_env)
            .to_lowercase();

        match shell_name.as_str() {
            "bash" => Ok(clap_complete::Shell::Bash),
            "zsh" => Ok(clap_complete::Shell::Zsh),
            "fish" => Ok(clap_complete::Shell::Fish),
            "pwsh" | "powershell" => Ok(clap_complete::Shell::PowerShell),
            _ => Err(anyhow::anyhow!(
                "Could not detect shell from $SHELL (got '{}'). Please specify: bash, zsh, fish, or powershell",
                shell_name
            )),
        }
    }

    /// Get completion file path for the given shell
    fn get_completion_file_path(shell: clap_complete::Shell) -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| {
            anyhow::anyhow!(
                "Could not determine home directory. Please set HOME environment variable."
            )
        })?;
        Ok(match shell {
            clap_complete::Shell::Bash => home.join(".local/share/xion-toolkit/completions.bash"),
            clap_complete::Shell::Zsh => home.join(".local/share/xion-toolkit/completions.zsh"),
            clap_complete::Shell::Fish => home.join(".config/fish/completions/xion-toolkit.fish"),
            clap_complete::Shell::PowerShell => {
                home.join(".local/share/xion-toolkit/completions.ps1")
            }
            _ => home.join(".local/share/xion-toolkit/completions"),
        })
    }

    /// Get profile file path for the given shell
    fn get_profile_path(shell: clap_complete::Shell) -> Result<Option<PathBuf>> {
        let home = dirs::home_dir().ok_or_else(|| {
            anyhow::anyhow!(
                "Could not determine home directory. Please set HOME environment variable."
            )
        })?;
        Ok(match shell {
            clap_complete::Shell::Bash => Some(home.join(".bashrc")),
            clap_complete::Shell::Zsh => Some(home.join(".zshrc")),
            clap_complete::Shell::Fish => None, // Fish auto-loads
            clap_complete::Shell::PowerShell => {
                // PowerShell $PROFILE location varies by platform
                // On macOS/Linux it's typically ~/.config/powershell/Microsoft.PowerShell_profile.ps1
                let config_dir = home.join(".config/powershell");
                Some(config_dir.join("Microsoft.PowerShell_profile.ps1"))
            }
            _ => None,
        })
    }

    /// Get the shell name as string
    fn get_shell_name(shell: clap_complete::Shell) -> &'static str {
        match shell {
            clap_complete::Shell::Bash => "bash",
            clap_complete::Shell::Zsh => "zsh",
            clap_complete::Shell::Fish => "fish",
            clap_complete::Shell::PowerShell => "powershell",
            _ => "unknown",
        }
    }

    /// Check if completions are already installed in the profile
    fn is_already_installed(profile_path: &PathBuf) -> bool {
        if !profile_path.exists() {
            return false;
        }
        let content = std::fs::read_to_string(profile_path).unwrap_or_default();
        content.contains("# BEGIN xion-toolkit completions")
    }

    /// Add source line to profile
    fn add_to_profile(
        profile_path: &std::path::Path,
        completion_path: &std::path::Path,
        shell: clap_complete::Shell,
    ) -> Result<()> {
        let profile_content = if profile_path.exists() {
            std::fs::read_to_string(profile_path)?
        } else {
            String::new()
        };

        let completion_path_str = completion_path.to_string_lossy();

        let source_line = match shell {
            clap_complete::Shell::Bash => format!(
                "[ -f {} ] && source {}",
                completion_path_str, completion_path_str
            ),
            clap_complete::Shell::Zsh => format!(
                "[ -f {} ] && source {}",
                completion_path_str, completion_path_str
            ),
            clap_complete::Shell::PowerShell => format!(". {}", completion_path_str),
            _ => format!("source {}", completion_path_str),
        };

        let block = format!(
            "\n# BEGIN xion-toolkit completions\n{}\n# END xion-toolkit completions\n",
            source_line
        );

        let new_content = profile_content + &block;

        // Ensure parent directory exists
        if let Some(parent) = profile_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(profile_path, new_content)?;
        Ok(())
    }

    /// Generate completion script for the given shell
    fn generate_completion(shell: clap_complete::Shell) -> String {
        let mut cmd = Cli::command();
        let bin_name = "xion-toolkit";
        let mut output = Vec::new();
        clap_complete::generate(shell, &mut cmd, bin_name, &mut output);
        String::from_utf8(output).expect("Generated completion should be valid UTF-8")
    }

    // Determine which shell to use
    let detected_shell = match shell {
        Some(s) => s,
        None => detect_shell()?,
    };

    // If not installing, just print to stdout
    if !install {
        let mut cmd = Cli::command();
        let bin_name = "xion-toolkit";
        clap_complete::generate(detected_shell, &mut cmd, bin_name, &mut std::io::stdout());
        return Ok(());
    }

    // Install mode
    let completion_path = get_completion_file_path(detected_shell)?;
    let profile_path = get_profile_path(detected_shell)?;
    let shell_name = get_shell_name(detected_shell);

    // Generate the completion script
    let completion_script = generate_completion(detected_shell);

    // Ensure completion directory exists
    if let Some(parent) = completion_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Write completion file
    std::fs::write(&completion_path, completion_script)?;

    // Handle profile modification (not needed for fish)
    if detected_shell == clap_complete::Shell::Fish {
        let result = InstallResult {
            success: true,
            shell: Some(shell_name.to_string()),
            completion_file: Some(completion_path.to_string_lossy().to_string()),
            profile_file: None,
            already_installed: None,
            message: format!(
                "Completions installed to {} (fish auto-loads this directory)",
                completion_path.to_string_lossy()
            ),
        };
        crate::utils::output::print_formatted(&result, ctx.output_format())?;
        return Ok(());
    }

    // For other shells, check if already installed and update profile
    let profile_path = match profile_path {
        Some(p) => p,
        None => {
            let result = InstallResult {
                success: true,
                shell: Some(shell_name.to_string()),
                completion_file: Some(completion_path.to_string_lossy().to_string()),
                profile_file: None,
                already_installed: None,
                message: format!(
                    "Completions installed to {}. Could not determine profile location.",
                    completion_path.to_string_lossy()
                ),
            };
            crate::utils::output::print_formatted(&result, ctx.output_format())?;
            return Ok(());
        }
    };

    // Check if already installed
    if is_already_installed(&profile_path) {
        let result = InstallResult {
            success: true,
            shell: Some(shell_name.to_string()),
            completion_file: Some(completion_path.to_string_lossy().to_string()),
            profile_file: Some(profile_path.to_string_lossy().to_string()),
            already_installed: Some(true),
            message: "Completions already installed. To reinstall, remove the 'BEGIN xion-toolkit completions' block from your profile first.".to_string(),
        };
        crate::utils::output::print_formatted(&result, ctx.output_format())?;
        return Ok(());
    }

    // Add to profile
    add_to_profile(&profile_path, &completion_path, detected_shell)?;

    let profile_name = profile_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let result = InstallResult {
        success: true,
        shell: Some(shell_name.to_string()),
        completion_file: Some(completion_path.to_string_lossy().to_string()),
        profile_file: Some(profile_path.to_string_lossy().to_string()),
        already_installed: None,
        message: format!(
            "Completions installed. Restart your shell or run: source {}",
            profile_name
        ),
    };
    crate::utils::output::print_formatted(&result, ctx.output_format())?;
    Ok(())
}
