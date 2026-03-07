use anyhow::Result;
use clap::{Args, Subcommand};
use std::fs;
use std::path::PathBuf;

use crate::treasury::types::{
    AuthorizationInput, FeeConfigInput, GrantConfigInput, TreasuryCreateRequest,
    TreasuryParamsInput,
};

#[derive(Subcommand)]
pub enum TreasuryCommands {
    /// List all treasury contracts for the authenticated user
    List,

    /// Query treasury contract details
    Query {
        /// Treasury contract address
        address: String,
    },

    /// Fund a treasury contract
    Fund {
        /// Treasury contract address
        address: String,
        /// Amount to fund (e.g., "1000000uxion")
        amount: String,
    },

    /// Withdraw from a treasury contract to your meta account
    Withdraw {
        /// Treasury contract address
        address: String,
        /// Amount to withdraw (e.g., "1000000uxion")
        amount: String,
    },

    /// Create a new treasury contract
    Create(Box<CreateArgs>),

    /// Manage grant configurations
    #[command(subcommand)]
    GrantConfig(GrantConfigCommands),

    /// Manage fee configurations
    #[command(subcommand)]
    FeeConfig(FeeConfigCommands),
}

/// Grant configuration subcommands
#[derive(Subcommand)]
pub enum GrantConfigCommands {
    /// Add a grant configuration
    Add {
        /// Treasury contract address
        address: String,

        /// Path to JSON config file
        #[arg(short, long)]
        config: PathBuf,
    },

    /// Remove a grant configuration
    Remove {
        /// Treasury contract address
        address: String,

        /// Type URL of the grant to remove
        #[arg(long)]
        type_url: String,
    },

    /// List all grant configurations
    List {
        /// Treasury contract address
        address: String,
    },
}

/// Fee configuration subcommands
#[derive(Subcommand)]
pub enum FeeConfigCommands {
    /// Set fee configuration
    Set {
        /// Treasury contract address
        address: String,

        /// Path to JSON config file
        #[arg(short, long)]
        config: PathBuf,
    },

    /// Remove fee configuration
    Remove {
        /// Treasury contract address
        address: String,
    },

    /// Query fee configuration
    Query {
        /// Treasury contract address
        address: String,
    },
}

#[derive(Args, Clone)]
pub struct CreateArgs {
    /// Path to JSON config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// OAuth callback URL (required unless using --config)
    #[arg(short = 'r', long)]
    pub redirect_url: Option<String>,

    /// Treasury icon URL (required unless using --config)
    #[arg(short = 'i', long)]
    pub icon_url: Option<String>,

    /// Treasury display name
    #[arg(short = 'N', long)]
    pub name: Option<String>,

    /// Mark as OAuth2 application
    #[arg(long)]
    pub is_oauth2_app: bool,

    // Fee grant flags
    /// Fee allowance type: basic, periodic, or allowed-msg
    #[arg(long)]
    pub fee_allowance_type: Option<String>,

    /// Fee spend limit (e.g., "1000000uxion")
    #[arg(long)]
    pub fee_spend_limit: Option<String>,

    /// Fee period seconds (for periodic allowance)
    #[arg(long)]
    pub fee_period_seconds: Option<u64>,

    /// Fee period spend limit (for periodic allowance)
    #[arg(long)]
    pub fee_period_spend_limit: Option<String>,

    /// Fee grant description
    #[arg(long)]
    pub fee_description: Option<String>,

    // Grant flags (simplified)
    /// Grant permission type URL
    #[arg(long)]
    pub grant_type_url: Option<String>,

    /// Grant authorization type (generic, send, stake)
    #[arg(long)]
    pub grant_auth_type: Option<String>,

    /// Grant description
    #[arg(long)]
    pub grant_description: Option<String>,

    /// Grant spend limit (for send authorization)
    #[arg(long)]
    pub grant_spend_limit: Option<String>,
}

pub async fn handle_command(cmd: TreasuryCommands) -> Result<()> {
    match cmd {
        TreasuryCommands::List => handle_list().await,
        TreasuryCommands::Query { address } => handle_query(&address).await,
        TreasuryCommands::Create(args) => handle_create(*args).await,
        TreasuryCommands::Fund { address, amount } => handle_fund(&address, &amount).await,
        TreasuryCommands::Withdraw { address, amount } => handle_withdraw(&address, &amount).await,
        TreasuryCommands::GrantConfig(sub) => handle_grant_config(sub).await,
        TreasuryCommands::FeeConfig(sub) => handle_fee_config(sub).await,
    }
}

async fn handle_list() -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info("Listing treasury contracts...");

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // List treasuries
    match manager.list().await {
        Ok(treasuries) => {
            let result = serde_json::json!({
                "success": true,
                "treasuries": treasuries,
                "count": treasuries.len()
            });
            print_json(&result)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to list treasuries: {}", e),
                "code": "TREASURY_LIST_FAILED"
            });
            print_json(&result)
        }
    }
}

async fn handle_query(address: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Querying treasury: {}", address));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Query treasury
    match manager.query(address).await {
        Ok(treasury) => {
            let result = serde_json::json!({
                "success": true,
                "treasury": treasury
            });
            print_json(&result)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to query treasury: {}", e),
                "code": "TREASURY_QUERY_FAILED"
            });
            print_json(&result)
        }
    }
}

async fn handle_create(args: CreateArgs) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info("Creating treasury contract...");

    // Load from config file or build from flags
    let request = if let Some(config_path) = args.config {
        load_treasury_config(&config_path)?
    } else {
        build_request_from_flags(&args)?
    };

    // Validate request
    validate_create_request(&request)?;

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Call manager to create treasury
    match manager.create(request).await {
        Ok(result) => print_json(&result),
        Err(e) => {
            let error_msg = e.to_string();
            let (code, suggestion) =
                if error_msg.contains("insufficient") || error_msg.contains("balance") {
                    (
                        "INSUFFICIENT_BALANCE",
                        "Fund your account before creating a treasury",
                    )
                } else if error_msg.contains("invalid") || error_msg.contains("format") {
                    ("INVALID_INPUT", "Check your input parameters")
                } else if error_msg.contains("unauthorized") {
                    (
                        "UNAUTHORIZED",
                        "You may not have permission to perform this action",
                    )
                } else {
                    (
                        "TREASURY_CREATE_FAILED",
                        "Check the error message for details",
                    )
                };

            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to create treasury: {}", e),
                "code": code,
                "suggestion": suggestion
            });
            print_json(&result)
        }
    }
}

/// Load treasury configuration from JSON file
fn load_treasury_config(path: &PathBuf) -> Result<TreasuryCreateRequest> {
    let content = fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
    let config: TreasuryCreateRequest = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid config file format: {}", e))?;
    Ok(config)
}

/// Build treasury creation request from CLI flags
pub fn build_request_from_flags(args: &CreateArgs) -> Result<TreasuryCreateRequest> {
    // Validate required fields
    let redirect_url = args
        .redirect_url
        .clone()
        .ok_or_else(|| anyhow::anyhow!("--redirect-url is required"))?;
    let icon_url = args
        .icon_url
        .clone()
        .ok_or_else(|| anyhow::anyhow!("--icon-url is required"))?;

    // Build params
    let params = TreasuryParamsInput {
        redirect_url,
        icon_url,
        name: args.name.clone(),
        is_oauth2_app: if args.is_oauth2_app { Some(true) } else { None },
    };

    // Build fee config
    let fee_config = if let Some(ref allowance_type) = args.fee_allowance_type {
        let description = args.fee_description.clone().ok_or_else(|| {
            anyhow::anyhow!("--fee-description is required when fee allowance is specified")
        })?;

        Some(match allowance_type.as_str() {
            "basic" => {
                let spend_limit = args.fee_spend_limit.clone().ok_or_else(|| {
                    anyhow::anyhow!("--fee-spend-limit is required for basic allowance")
                })?;
                FeeConfigInput::Basic {
                    spend_limit,
                    description,
                }
            }
            "periodic" => {
                let period_seconds = args.fee_period_seconds.ok_or_else(|| {
                    anyhow::anyhow!("--fee-period-seconds is required for periodic allowance")
                })?;
                let period_spend_limit = args.fee_period_spend_limit.clone().ok_or_else(|| {
                    anyhow::anyhow!("--fee-period-spend-limit is required for periodic allowance")
                })?;
                FeeConfigInput::Periodic {
                    basic_spend_limit: args.fee_spend_limit.clone(),
                    period_seconds,
                    period_spend_limit,
                    description,
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid fee allowance type: {}. Supported: basic, periodic",
                    allowance_type
                ))
            }
        })
    } else {
        None
    };

    // Build grant configs (simplified for now - just one grant)
    let grant_configs = if let (Some(type_url), Some(auth_type), Some(description)) = (
        &args.grant_type_url,
        &args.grant_auth_type,
        &args.grant_description,
    ) {
        let authorization = match auth_type.as_str() {
            "generic" => AuthorizationInput::Generic,
            "send" => {
                let spend_limit = args.grant_spend_limit.clone().ok_or_else(|| {
                    anyhow::anyhow!("--grant-spend-limit is required for send authorization")
                })?;
                AuthorizationInput::Send {
                    spend_limit,
                    allow_list: None,
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid grant auth type: {}. Supported: generic, send",
                    auth_type
                ))
            }
        };

        vec![GrantConfigInput {
            type_url: type_url.clone(),
            description: description.clone(),
            authorization,
            optional: false,
        }]
    } else {
        // For now, we'll create an empty vec and let the user know they need grants
        vec![]
    };

    if grant_configs.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one grant configuration is required. Use --grant-type-url, --grant-auth-type, and --grant-description flags, or provide a config file with --config"
        ));
    }

    Ok(TreasuryCreateRequest {
        params,
        fee_config,
        grant_configs,
    })
}

/// Validate treasury creation request
pub fn validate_create_request(request: &TreasuryCreateRequest) -> Result<()> {
    // Validate URLs
    if !request.params.redirect_url.starts_with("http") {
        return Err(anyhow::anyhow!(
            "redirect_url must be a valid URL starting with http:// or https://"
        ));
    }
    if !request.params.icon_url.starts_with("http") {
        return Err(anyhow::anyhow!(
            "icon_url must be a valid URL starting with http:// or https://"
        ));
    }

    // Validate at least one grant config exists
    if request.grant_configs.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one grant configuration is required"
        ));
    }

    // Validate each grant config
    for grant in &request.grant_configs {
        if grant.type_url.is_empty() {
            return Err(anyhow::anyhow!("Grant type_url cannot be empty"));
        }
        if grant.description.is_empty() {
            return Err(anyhow::anyhow!("Grant description cannot be empty"));
        }
    }

    Ok(())
}

async fn handle_fund(address: &str, amount: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Funding treasury {} with {}...", address, amount));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Fund treasury
    match manager.fund(address, amount).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "amount": result.amount,
                "tx_hash": result.tx_hash
            });
            print_json(&response)
        }
        Err(e) => {
            let error_msg = e.to_string();
            let (code, suggestion) =
                if error_msg.contains("insufficient") || error_msg.contains("balance") {
                    (
                        "INSUFFICIENT_BALANCE",
                        "Check your wallet balance and try with a smaller amount",
                    )
                } else if error_msg.contains("invalid") || error_msg.contains("format") {
                    (
                        "INVALID_AMOUNT",
                        "Amount should be in format like '1000000uxion'",
                    )
                } else if error_msg.contains("not found") {
                    (
                        "TREASURY_NOT_FOUND",
                        "Verify the treasury address is correct",
                    )
                } else {
                    ("FUND_FAILED", "Check the error message for details")
                };

            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to fund treasury: {}", e),
                "code": code,
                "suggestion": suggestion
            });
            print_json(&result)
        }
    }
}

async fn handle_withdraw(address: &str, amount: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!(
        "Withdrawing {} from treasury {} to your meta account...",
        amount, address
    ));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Withdraw from treasury
    match manager.withdraw(address, amount).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "amount": result.amount,
                "tx_hash": result.tx_hash
            });
            print_json(&response)
        }
        Err(e) => {
            let error_msg = e.to_string();
            let (code, suggestion) =
                if error_msg.contains("unauthorized") || error_msg.contains("admin") {
                    ("UNAUTHORIZED", "Only the treasury admin can withdraw funds")
                } else if error_msg.contains("insufficient") || error_msg.contains("balance") {
                    (
                        "INSUFFICIENT_BALANCE",
                        "The treasury doesn't have enough balance for this withdrawal",
                    )
                } else if error_msg.contains("invalid") || error_msg.contains("format") {
                    (
                        "INVALID_AMOUNT",
                        "Amount should be in format like '1000000uxion'",
                    )
                } else if error_msg.contains("not found") {
                    (
                        "TREASURY_NOT_FOUND",
                        "Verify the treasury address is correct",
                    )
                } else {
                    ("WITHDRAW_FAILED", "Check the error message for details")
                };

            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to withdraw from treasury: {}", e),
                "code": code,
                "suggestion": suggestion
            });
            print_json(&result)
        }
    }
}

// ============================================================================
// Grant Config Handlers
// ============================================================================

async fn handle_grant_config(cmd: GrantConfigCommands) -> Result<()> {
    match cmd {
        GrantConfigCommands::Add { address, config } => {
            handle_grant_config_add(&address, &config).await
        }
        GrantConfigCommands::Remove { address, type_url } => {
            handle_grant_config_remove(&address, &type_url).await
        }
        GrantConfigCommands::List { address } => handle_grant_config_list(&address).await,
    }
}

async fn handle_grant_config_add(address: &str, config_path: &PathBuf) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Adding grant config to treasury {}...", address));

    // Load config from file
    let content = fs::read_to_string(config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
    let grant_config: crate::treasury::types::GrantConfigInput = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid config file format: {}", e))?;

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Add grant config
    match manager.add_grant_config(address, grant_config).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "operation": result.operation,
                "tx_hash": result.tx_hash
            });
            print_json(&response)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to add grant config: {}", e),
                "code": "GRANT_CONFIG_ADD_FAILED"
            });
            print_json(&result)
        }
    }
}

async fn handle_grant_config_remove(address: &str, type_url: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!(
        "Removing grant config {} from treasury {}...",
        type_url, address
    ));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Remove grant config
    match manager.remove_grant_config(address, type_url).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "operation": result.operation,
                "tx_hash": result.tx_hash
            });
            print_json(&response)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to remove grant config: {}", e),
                "code": "GRANT_CONFIG_REMOVE_FAILED"
            });
            print_json(&result)
        }
    }
}

async fn handle_grant_config_list(address: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!(
        "Listing grant configs for treasury {}...",
        address
    ));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // List grant configs
    match manager.list_grant_configs(address).await {
        Ok(configs) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": address,
                "grant_configs": configs,
                "count": configs.len()
            });
            print_json(&response)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to list grant configs: {}", e),
                "code": "GRANT_CONFIG_LIST_FAILED"
            });
            print_json(&result)
        }
    }
}

// ============================================================================
// Fee Config Handlers
// ============================================================================

async fn handle_fee_config(cmd: FeeConfigCommands) -> Result<()> {
    match cmd {
        FeeConfigCommands::Set { address, config } => {
            handle_fee_config_set(&address, &config).await
        }
        FeeConfigCommands::Remove { address } => handle_fee_config_remove(&address).await,
        FeeConfigCommands::Query { address } => handle_fee_config_query(&address).await,
    }
}

async fn handle_fee_config_set(address: &str, config_path: &PathBuf) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Setting fee config for treasury {}...", address));

    // Load config from file
    let content = fs::read_to_string(config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
    let fee_config: crate::treasury::types::FeeConfigInput = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid config file format: {}", e))?;

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Set fee config
    match manager.set_fee_config(address, fee_config).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "operation": result.operation,
                "tx_hash": result.tx_hash
            });
            print_json(&response)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to set fee config: {}", e),
                "code": "FEE_CONFIG_SET_FAILED"
            });
            print_json(&result)
        }
    }
}

async fn handle_fee_config_remove(address: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Removing fee config from treasury {}...", address));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Remove fee config
    match manager.remove_fee_config(address).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "operation": result.operation,
                "tx_hash": result.tx_hash
            });
            print_json(&response)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to remove fee config: {}", e),
                "code": "FEE_CONFIG_REMOVE_FAILED"
            });
            print_json(&result)
        }
    }
}

async fn handle_fee_config_query(address: &str) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_info, print_json};

    print_info(&format!("Querying fee config for treasury {}...", address));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_json(&result);
    }

    // Query fee config
    match manager.query_fee_config(address).await {
        Ok(Some(config)) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": address,
                "fee_config": config
            });
            print_json(&response)
        }
        Ok(None) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": address,
                "fee_config": null,
                "message": "No fee config set"
            });
            print_json(&response)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to query fee config: {}", e),
                "code": "FEE_CONFIG_QUERY_FAILED"
            });
            print_json(&result)
        }
    }
}
