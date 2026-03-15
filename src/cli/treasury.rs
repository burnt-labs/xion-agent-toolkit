use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use std::fs;
use std::path::PathBuf;

use crate::cli::ExecuteContext;
use crate::treasury::types::{
    AuthorizationInput, FeeConfigInput, GrantConfigInput, TreasuryCreateRequest,
    TreasuryParamsInput,
};

#[allow(clippy::large_enum_variant)]
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

    /// Manage treasury admin
    #[command(subcommand)]
    Admin(AdminCommands),

    /// Manage treasury parameters
    #[command(subcommand)]
    Params(ParamsCommands),

    /// Batch operations for multiple treasuries
    #[command(subcommand)]
    Batch(BatchCommands),

    /// Query on-chain data (authz grants, fee allowances)
    #[command(subcommand)]
    ChainQuery(ChainQueryCommands),

    /// Export treasury configuration for backup/migration
    Export(ExportArgs),

    /// Import configuration to an existing treasury
    Import(ImportArgs),
}

/// Batch operations subcommands
#[derive(Subcommand)]
pub enum BatchCommands {
    /// Fund multiple treasuries from a config file
    Fund {
        /// Path to JSON config file containing funding operations
        #[arg(long, value_name = "FILE")]
        config: PathBuf,
    },

    /// Configure grants for multiple treasuries from a config file
    GrantConfig {
        /// Path to JSON config file containing grant configurations
        #[arg(long, value_name = "FILE")]
        config: PathBuf,
    },
}

/// Export treasury configuration arguments
#[derive(Debug, Args)]
pub struct ExportArgs {
    /// Treasury contract address (optional, exports all if not provided)
    pub address: Option<String>,

    /// Output file path (optional, defaults to stdout)
    #[arg(long)]
    pub output: Option<String>,
}

/// Import treasury configuration arguments
#[derive(Debug, Args)]
pub struct ImportArgs {
    /// Treasury contract address to import configuration to
    pub address: String,

    /// Path to JSON file containing the configuration to import
    #[arg(long)]
    pub from_file: String,

    /// Preview actions without executing transactions
    #[arg(long)]
    pub dry_run: bool,
}

/// Admin management subcommands
#[derive(Subcommand)]
pub enum AdminCommands {
    /// Propose a new admin for the treasury
    Propose {
        /// Treasury contract address
        address: String,
        /// New admin address
        #[arg(long)]
        new_admin: String,
    },

    /// Accept admin role (called by pending admin)
    Accept {
        /// Treasury contract address
        address: String,
    },

    /// Cancel proposed admin
    Cancel {
        /// Treasury contract address
        address: String,
    },
}

/// Params management subcommands
#[derive(Subcommand)]
pub enum ParamsCommands {
    /// Update treasury parameters
    Update {
        /// Treasury contract address
        address: String,

        /// Redirect URL
        #[arg(long)]
        redirect_url: Option<String>,

        /// Icon URL
        #[arg(long)]
        icon_url: Option<String>,

        /// Treasury name
        #[arg(long)]
        name: Option<String>,

        /// Mark as OAuth2 application
        #[arg(long)]
        is_oauth2_app: bool,

        /// Metadata as JSON string
        #[arg(long)]
        metadata: Option<String>,
    },
}

/// Query subcommands for on-chain data
#[derive(Subcommand)]
pub enum ChainQueryCommands {
    /// List authz grants for a treasury
    Grants {
        /// Treasury contract address
        address: String,
    },

    /// List fee allowances for a treasury
    Allowances {
        /// Treasury contract address
        address: String,
    },
}

/// Grant configuration subcommands
#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum GrantConfigCommands {
    /// Add a grant configuration
    Add {
        /// Treasury contract address
        address: String,

        /// Path to JSON config file (alternative to flags)
        #[arg(long, value_name = "FILE")]
        grant_config: Option<PathBuf>,

        /// Message type URL (e.g., /cosmos.bank.v1beta1.MsgSend)
        #[arg(long, value_name = "URL")]
        type_url: Option<String>,

        /// Authorization type: generic, send, contract-execution, stake, ibc-transfer
        #[arg(long, value_name = "TYPE")]
        auth_type: Option<String>,

        /// Grant description
        #[arg(long, value_name = "TEXT")]
        description: Option<String>,

        /// Spend limit for send authorization (e.g., "1000000uxion")
        #[arg(long, value_name = "AMOUNT")]
        spend_limit: Option<String>,

        /// Allowed recipients for send authorization (comma-separated)
        #[arg(long, value_name = "ADDRS")]
        allow_list: Option<String>,

        /// Contract address for contract-execution (can be repeated)
        #[arg(long = "contract", value_name = "ADDRESS")]
        contracts: Vec<String>,

        /// Max calls for contract-execution (can be repeated)
        #[arg(long = "max-calls", value_name = "NUM")]
        max_calls: Vec<u64>,

        /// Max funds for contract-execution (can be repeated)
        #[arg(long = "max-funds", value_name = "AMOUNT")]
        max_funds: Vec<String>,

        /// Filter type for contract-execution: allow-all, accepted-keys
        #[arg(long, value_name = "TYPE", default_value = "allow-all")]
        filter_type: String,

        /// Accepted message keys for accepted-keys filter (comma-separated)
        #[arg(long, value_name = "KEYS")]
        keys: Option<String>,

        /// Preset shortcut: send, execute, instantiate, delegate, vote
        #[arg(long, value_name = "TYPE")]
        preset: Option<String>,
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
        #[arg(long, value_name = "FILE")]
        fee_config: PathBuf,
    },

    /// Revoke fee allowance from a specific grantee
    Remove {
        /// Treasury contract address
        address: String,

        /// Grantee address to revoke allowance from
        #[arg(long)]
        grantee: String,
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

    /// Predict address without deploying (dry-run mode)
    /// When set, returns the predicted treasury address without creating it.
    #[arg(long)]
    pub predict: bool,

    /// Salt for deterministic address computation (required with --predict)
    /// Can be a UTF-8 string or hex-encoded bytes.
    /// Example: "my-treasury-v1" or "6d792d74726561737572792d7631"
    #[arg(long, requires = "predict")]
    pub salt: Option<String>,

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

pub async fn handle_command(cmd: TreasuryCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        TreasuryCommands::List => handle_list(ctx).await,
        TreasuryCommands::Query { address } => handle_query(&address, ctx).await,
        TreasuryCommands::Create(args) => handle_create(*args, ctx).await,
        TreasuryCommands::Fund { address, amount } => handle_fund(&address, &amount, ctx).await,
        TreasuryCommands::Withdraw { address, amount } => {
            handle_withdraw(&address, &amount, ctx).await
        }
        TreasuryCommands::GrantConfig(sub) => handle_grant_config(sub, ctx).await,
        TreasuryCommands::FeeConfig(sub) => handle_fee_config(sub, ctx).await,
        TreasuryCommands::Admin(sub) => handle_admin(sub, ctx).await,
        TreasuryCommands::Params(sub) => handle_params(sub, ctx).await,
        TreasuryCommands::ChainQuery(sub) => handle_chain_query(sub, ctx).await,
        TreasuryCommands::Export(args) => handle_export(args, ctx).await,
        TreasuryCommands::Import(args) => handle_import(args, ctx).await,
        TreasuryCommands::Batch(sub) => handle_batch(sub, ctx).await,
    }
}

async fn handle_list(ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

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
        return print_formatted(&result, ctx.output_format());
    }

    // List treasuries
    match manager.list().await {
        Ok(treasuries) => {
            let result = serde_json::json!({
                "success": true,
                "treasuries": treasuries,
                "count": treasuries.len()
            });
            print_formatted(&result, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to list treasuries: {}", e),
                "code": "TREASURY_LIST_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_query(address: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

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
        return print_formatted(&result, ctx.output_format());
    }

    // Query treasury
    match manager.query(address).await {
        Ok(treasury) => {
            let result = serde_json::json!({
                "success": true,
                "treasury": treasury
            });
            print_formatted(&result, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to query treasury: {}", e),
                "code": "TREASURY_QUERY_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_create(args: CreateArgs, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    // Handle prediction mode separately
    if args.predict {
        return handle_predict_address(&args, ctx).await;
    }

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
        return print_formatted(&result, ctx.output_format());
    }

    // Call manager to create treasury
    match manager.create(request).await {
        Ok(result) => print_formatted(&result, ctx.output_format()),
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
            print_formatted(&result, ctx.output_format())
        }
    }
}

/// Handle treasury address prediction (dry-run mode)
async fn handle_predict_address(args: &CreateArgs, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::shared::instantiate2::{predict_treasury_address, validate_salt};
    use crate::treasury::api_client::TreasuryApiClient;
    use crate::utils::output::{print_formatted, print_info};

    print_info("Predicting treasury address...");

    // Validate salt is provided
    let salt_str = args
        .salt
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--salt is required when using --predict"))?;

    // Validate salt format
    let encoding = crate::shared::instantiate2::detect_salt_encoding(salt_str);
    validate_salt(salt_str, encoding)?;

    // Create manager to get authentication and network config
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;

    // Check authentication to get creator address
    if !oauth_client.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_formatted(&result, ctx.output_format());
    }

    // Get user credentials for creator address
    let credentials = oauth_client
        .get_credentials()?
        .ok_or_else(|| anyhow::anyhow!("No credentials found. Please run 'xion auth login'."))?;

    let creator = credentials
        .xion_address
        .ok_or_else(|| anyhow::anyhow!("No Xion address found in credentials."))?;

    // Create API client to fetch code info
    let api_client = TreasuryApiClient::new(
        network_config.oauth_api_url.clone(),
        network_config.indexer_url.clone(),
        network_config.rpc_url.clone(),
    );

    // Get code info for the treasury code ID
    let code_id = network_config.treasury_code_id;
    let code_info = api_client
        .get_code_info(code_id)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch code info: {}", e))?;

    // Parse checksum
    let checksum_bytes = hex::decode(&code_info.checksum)
        .map_err(|e| anyhow::anyhow!("Invalid checksum format: {}", e))?;

    // Compute predicted address
    let predicted = predict_treasury_address(&creator, code_id, salt_str, &checksum_bytes)?;

    // Output result
    let result = serde_json::json!({
        "success": true,
        "data": {
            "predicted_address": predicted.address,
            "salt": salt_str,
            "salt_hex": predicted.salt_hex,
            "code_id": predicted.code_id,
            "checksum": predicted.checksum,
            "creator": predicted.creator,
            "network": network_config.network_name
        }
    });

    print_formatted(&result, ctx.output_format())
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

async fn handle_fund(address: &str, amount: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

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
        return print_formatted(&result, ctx.output_format());
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
            print_formatted(&response, ctx.output_format())
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
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_withdraw(address: &str, amount: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

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
        return print_formatted(&result, ctx.output_format());
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
            print_formatted(&response, ctx.output_format())
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
            print_formatted(&result, ctx.output_format())
        }
    }
}

// ============================================================================
// Grant Config Handlers
// ============================================================================

async fn handle_grant_config(cmd: GrantConfigCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        GrantConfigCommands::Add {
            address,
            grant_config: config_path,
            type_url,
            auth_type,
            description,
            spend_limit,
            allow_list,
            contracts,
            max_calls,
            max_funds,
            filter_type,
            keys,
            preset,
        } => {
            handle_grant_config_add(
                &address,
                config_path.as_ref(),
                type_url.as_deref(),
                auth_type.as_deref(),
                description.as_deref(),
                spend_limit.as_deref(),
                allow_list.as_deref(),
                &contracts,
                &max_calls,
                &max_funds,
                &filter_type,
                keys.as_deref(),
                preset.as_deref(),
                ctx,
            )
            .await
        }
        GrantConfigCommands::Remove { address, type_url } => {
            handle_grant_config_remove(&address, &type_url, ctx).await
        }
        GrantConfigCommands::List { address } => handle_grant_config_list(&address, ctx).await,
    }
}

#[allow(clippy::too_many_arguments)]
async fn handle_grant_config_add(
    address: &str,
    config_path: Option<&PathBuf>,
    type_url: Option<&str>,
    auth_type: Option<&str>,
    description: Option<&str>,
    spend_limit: Option<&str>,
    allow_list: Option<&str>,
    contracts: &[String],
    max_calls: &[u64],
    max_funds: &[String],
    filter_type: &str,
    keys: Option<&str>,
    preset: Option<&str>,
    ctx: &ExecuteContext,
) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!("Adding grant config to treasury {}...", address));

    // Load config from file or build from flags
    let grant_config = if let Some(path) = config_path {
        // Load from config file
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Invalid config file format: {}", e))?
    } else {
        // Build from flags
        build_grant_config_from_flags(
            type_url,
            auth_type,
            description,
            spend_limit,
            allow_list,
            contracts,
            max_calls,
            max_funds,
            filter_type,
            keys,
            preset,
        )?
    };

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
        return print_formatted(&result, ctx.output_format());
    }

    // Add grant config
    match manager.add_grant_config(address, grant_config).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "operation": result.operation,
                "type_url": result.type_url,
                "tx_hash": result.tx_hash
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to add grant config: {}", e),
                "code": "GRANT_CONFIG_ADD_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

/// Message type presets for convenience
/// Note: MsgExecuteContract only supports contract-execution authorization (generic is too risky)
const PRESET_TYPES: &[(&str, &str, &str)] = &[
    // Banking
    ("send", "/cosmos.bank.v1beta1.MsgSend", "send"),
    // CosmWasm
    (
        "execute",
        "/cosmwasm.wasm.v1.MsgExecuteContract",
        "contract-execution",
    ),
    (
        "instantiate",
        "/cosmwasm.wasm.v1.MsgInstantiateContract",
        "generic",
    ),
    (
        "instantiate2",
        "/cosmwasm.wasm.v1.MsgInstantiateContract2",
        "generic",
    ),
    // Staking
    ("delegate", "/cosmos.staking.v1beta1.MsgDelegate", "generic"),
    (
        "undelegate",
        "/cosmos.staking.v1beta1.MsgUndelegate",
        "generic",
    ),
    (
        "redelegate",
        "/cosmos.staking.v1beta1.MsgBeginRedelegate",
        "generic",
    ),
    (
        "withdraw-rewards",
        "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward",
        "generic",
    ),
    // Governance
    ("vote", "/cosmos.gov.v1beta1.MsgVote", "generic"),
    ("gov-deposit", "/cosmos.gov.v1beta1.MsgDeposit", "generic"),
    (
        "gov-submit-proposal",
        "/cosmos.gov.v1beta1.MsgSubmitProposal",
        "generic",
    ),
    // IBC
    (
        "ibc-transfer",
        "/ibc.applications.transfer.v1.MsgTransfer",
        "ibc_transfer",
    ),
    // Authz
    ("authz-exec", "/cosmos.authz.v1beta1.MsgExec", "generic"),
    ("authz-revoke", "/cosmos.authz.v1beta1.MsgRevoke", "generic"),
    // Feegrant
    (
        "feegrant-grant",
        "/cosmos.feegrant.v1beta1.MsgGrantAllowance",
        "generic",
    ),
    (
        "feegrant-revoke",
        "/cosmos.feegrant.v1beta1.MsgRevokeAllowance",
        "generic",
    ),
    // Slashing
    ("unjail", "/cosmos.slashing.v1beta1.MsgUnjail", "generic"),
    // Crisis
    (
        "crisis-verify",
        "/cosmos.crisis.v1beta1.MsgVerifyInvariant",
        "generic",
    ),
    (
        "evidence-submit",
        "/cosmos.evidence.v1beta1.MsgSubmitEvidence",
        "generic",
    ),
    // Vesting
    (
        "vesting-create",
        "/cosmos.vesting.v1beta1.MsgCreateVestingAccount",
        "generic",
    ),
    // TokenFactory (Osmosis)
    (
        "tokenfactory-mint",
        "/osmosis.tokenfactory.v1beta1.MsgMint",
        "generic",
    ),
    (
        "tokenfactory-burn",
        "/osmosis.tokenfactory.v1beta1.MsgBurn",
        "generic",
    ),
];

/// Build GrantConfigInput from CLI flags
#[allow(clippy::too_many_arguments)]
fn build_grant_config_from_flags(
    type_url: Option<&str>,
    auth_type: Option<&str>,
    description: Option<&str>,
    spend_limit: Option<&str>,
    allow_list: Option<&str>,
    contracts: &[String],
    max_calls: &[u64],
    max_funds: &[String],
    filter_type: &str,
    keys: Option<&str>,
    preset: Option<&str>,
) -> Result<crate::treasury::types::GrantConfigInput> {
    use crate::treasury::types::{AuthorizationInput, ContractGrantInput, GrantConfigInput};

    // Resolve preset if provided
    let (resolved_type_url, resolved_auth_type) = if let Some(preset_name) = preset {
        let found = PRESET_TYPES
            .iter()
            .find(|(name, _, _)| *name == preset_name)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Unknown preset: {}. Available presets: send, execute, instantiate, \
                     instantiate2, delegate, undelegate, redelegate, withdraw-rewards, vote, \
                     gov-deposit, gov-submit-proposal, ibc-transfer, authz-exec, authz-revoke, \
                     feegrant-grant, feegrant-revoke, unjail, crisis-verify, evidence-submit, \
                     vesting-create, tokenfactory-mint, tokenfactory-burn",
                    preset_name
                )
            })?;
        (Some(found.1.to_string()), Some(found.2.to_string()))
    } else {
        (
            type_url.map(|s| s.to_string()),
            auth_type.map(|s| s.to_string()),
        )
    };

    // Validate required fields
    let type_url =
        resolved_type_url.ok_or_else(|| anyhow::anyhow!("--type-url or --preset is required"))?;
    let description = description
        .ok_or_else(|| anyhow::anyhow!("--description is required"))?
        .to_string();

    // Determine auth type (default to generic if not specified)
    let auth_type_str = resolved_auth_type.unwrap_or_else(|| "generic".to_string());

    // Security check: MsgExecuteContract must use contract-execution authorization
    // Generic authorization is too risky as it allows unlimited contract execution
    if type_url.contains("MsgExecuteContract") && auth_type_str == "generic" {
        anyhow::bail!(
            "MsgExecuteContract requires --auth-type contract-execution for security. \
             Generic authorization is not allowed as it permits unlimited contract execution. \
             Use --contract, --max-calls/--max-funds, and --filter-type to configure limits."
        );
    }

    // Build authorization based on type
    let authorization = match auth_type_str.as_str() {
        "generic" => AuthorizationInput::Generic,
        "send" => {
            let spend = spend_limit
                .ok_or_else(|| anyhow::anyhow!("--spend-limit is required for send authorization"))?
                .to_string();
            let allow = allow_list.map(|s| s.split(',').map(|s| s.trim().to_string()).collect());
            AuthorizationInput::Send {
                spend_limit: spend,
                allow_list: allow,
            }
        }
        "contract-execution" => {
            if contracts.is_empty() {
                anyhow::bail!("--contract is required for contract-execution authorization");
            }

            let mut grants = Vec::new();
            for (i, contract) in contracts.iter().enumerate() {
                let max_call = max_calls.get(i).copied();
                let max_fund = max_funds.get(i).map(|s| s.to_string());

                if max_call.is_none() && max_fund.is_none() {
                    anyhow::bail!(
                        "At least one of --max-calls or --max-funds is required for each contract"
                    );
                }

                let filter = if filter_type == "accepted-keys" {
                    let keys_str = keys.ok_or_else(|| {
                        anyhow::anyhow!("--keys is required when --filter-type=accepted-keys")
                    })?;
                    Some(keys_str.split(',').map(|s| s.trim().to_string()).collect())
                } else {
                    None
                };

                // Convert kebab-case to snake_case for encoding layer
                let normalized_filter_type = filter_type.replace('-', "_");

                grants.push(ContractGrantInput {
                    address: contract.clone(),
                    max_calls: max_call,
                    max_funds: max_fund,
                    filter_type: normalized_filter_type,
                    keys: filter,
                });
            }
            AuthorizationInput::ContractExecution { grants }
        }
        "stake" => {
            anyhow::bail!("Stake authorization requires --config file for complex configuration");
        }
        "ibc_transfer" => {
            anyhow::bail!(
                "IBC transfer authorization requires --config file for complex configuration"
            );
        }
        _ => {
            anyhow::bail!("Unknown auth-type: {}. Valid options: generic, send, contract-execution, stake, ibc_transfer", auth_type_str);
        }
    };

    Ok(GrantConfigInput {
        type_url,
        description,
        authorization,
        optional: false,
    })
}

async fn handle_grant_config_remove(
    address: &str,
    type_url: &str,
    ctx: &ExecuteContext,
) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

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
        return print_formatted(&result, ctx.output_format());
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
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to remove grant config: {}", e),
                "code": "GRANT_CONFIG_REMOVE_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_grant_config_list(address: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

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
        return print_formatted(&result, ctx.output_format());
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
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to list grant configs: {}", e),
                "code": "GRANT_CONFIG_LIST_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

// ============================================================================
// Fee Config Handlers
// ============================================================================

async fn handle_fee_config(cmd: FeeConfigCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        FeeConfigCommands::Set {
            address,
            fee_config,
        } => handle_fee_config_set(&address, &fee_config, ctx).await,
        FeeConfigCommands::Remove { address, grantee } => {
            handle_fee_config_remove(&address, &grantee, ctx).await
        }
        FeeConfigCommands::Query { address } => handle_fee_config_query(&address, ctx).await,
    }
}

async fn handle_fee_config_set(
    address: &str,
    config_path: &PathBuf,
    ctx: &ExecuteContext,
) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

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
        return print_formatted(&result, ctx.output_format());
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
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to set fee config: {}", e),
                "code": "FEE_CONFIG_SET_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_fee_config_remove(
    address: &str,
    grantee: &str,
    ctx: &ExecuteContext,
) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Revoking fee allowance from grantee {} for treasury {}...",
        grantee, address
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
        return print_formatted(&result, ctx.output_format());
    }

    // Revoke allowance
    match manager.remove_fee_config(address, grantee).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "operation": result.operation,
                "tx_hash": result.tx_hash
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to revoke allowance: {}", e),
                "code": "REVOKE_ALLOWANCE_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_fee_config_query(address: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

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
        return print_formatted(&result, ctx.output_format());
    }

    // Query fee config
    match manager.query_fee_config(address).await {
        Ok(Some(config)) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": address,
                "fee_config": config
            });
            print_formatted(&response, ctx.output_format())
        }
        Ok(None) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": address,
                "fee_config": null,
                "message": "No fee config set"
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to query fee config: {}", e),
                "code": "FEE_CONFIG_QUERY_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

// ============================================================================
// Admin Management Handlers
// ============================================================================

async fn handle_admin(cmd: AdminCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        AdminCommands::Propose { address, new_admin } => {
            handle_admin_propose(&address, &new_admin, ctx).await
        }
        AdminCommands::Accept { address } => handle_admin_accept(&address, ctx).await,
        AdminCommands::Cancel { address } => handle_admin_cancel(&address, ctx).await,
    }
}

async fn handle_admin_propose(address: &str, new_admin: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Proposing new admin {} for treasury {}...",
        new_admin, address
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
        return print_formatted(&result, ctx.output_format());
    }

    // Propose admin
    match manager.propose_admin(address, new_admin).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "operation": result.operation,
                "new_admin": result.new_admin,
                "tx_hash": result.tx_hash
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to propose admin: {}", e),
                "code": "PROPOSE_ADMIN_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_admin_accept(address: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!("Accepting admin role for treasury {}...", address));

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
        return print_formatted(&result, ctx.output_format());
    }

    // Accept admin
    match manager.accept_admin(address).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "operation": result.operation,
                "tx_hash": result.tx_hash
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to accept admin: {}", e),
                "code": "ACCEPT_ADMIN_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_admin_cancel(address: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Canceling proposed admin for treasury {}...",
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
        return print_formatted(&result, ctx.output_format());
    }

    // Cancel proposed admin
    match manager.cancel_proposed_admin(address).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "operation": result.operation,
                "tx_hash": result.tx_hash
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to cancel proposed admin: {}", e),
                "code": "CANCEL_ADMIN_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

// ============================================================================
// Params Management Handlers
// ============================================================================

async fn handle_params(cmd: ParamsCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        ParamsCommands::Update {
            address,
            redirect_url,
            icon_url,
            name,
            is_oauth2_app,
            metadata,
        } => {
            handle_params_update(
                &address,
                redirect_url,
                icon_url,
                name,
                is_oauth2_app,
                metadata,
                ctx,
            )
            .await
        }
    }
}

async fn handle_params_update(
    address: &str,
    redirect_url: Option<String>,
    icon_url: Option<String>,
    name: Option<String>,
    is_oauth2_app: bool,
    metadata: Option<String>,
    ctx: &ExecuteContext,
) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!("Updating params for treasury {}...", address));

    // Build params input
    let metadata_value = match metadata {
        Some(m) => Some(serde_json::from_str(&m).context("Invalid JSON in --metadata argument")?),
        None => None,
    };
    let params = crate::treasury::types::UpdateParamsInput {
        redirect_url,
        icon_url,
        name,
        is_oauth2_app: if is_oauth2_app { Some(true) } else { None },
        metadata: metadata_value,
    };

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
        return print_formatted(&result, ctx.output_format());
    }

    // Update params
    match manager.update_params(address, params).await {
        Ok(result) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": result.treasury_address,
                "tx_hash": result.tx_hash
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to update params: {}", e),
                "code": "UPDATE_PARAMS_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

// ============================================================================
// Query Handlers (On-Chain)
// ============================================================================

async fn handle_chain_query(cmd: ChainQueryCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        ChainQueryCommands::Grants { address } => handle_query_grants(&address, ctx).await,
        ChainQueryCommands::Allowances { address } => handle_query_allowances(&address, ctx).await,
    }
}

async fn handle_query_grants(address: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Querying authz grants for treasury {}...",
        address
    ));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Query authz grants (no auth required for on-chain query)
    match manager.list_authz_grants(address).await {
        Ok(grants) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": address,
                "grants": grants,
                "count": grants.len()
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to query authz grants: {}", e),
                "code": "QUERY_GRANTS_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

async fn handle_query_allowances(address: &str, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Querying fee allowances for treasury {}...",
        address
    ));

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Query fee allowances (no auth required for on-chain query)
    match manager.list_fee_allowances(address).await {
        Ok(allowances) => {
            let response = serde_json::json!({
                "success": true,
                "treasury_address": address,
                "allowances": allowances,
                "count": allowances.len()
            });
            print_formatted(&response, ctx.output_format())
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to query fee allowances: {}", e),
                "code": "QUERY_ALLOWANCES_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

// ============================================================================
// Export Handlers
// ============================================================================

async fn handle_export(args: ExportArgs, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::types::TreasuryExportConfig;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    // Determine if we're exporting single or all treasuries
    if let Some(address) = args.address {
        // Single treasury export
        print_info(&format!(
            "Exporting treasury configuration for {}...",
            address
        ));

        // Create manager
        let config_manager = ConfigManager::new()?;
        let network_config = config_manager.get_network_config()?;
        let oauth_client = OAuthClient::new(network_config.clone())?;
        let manager = TreasuryManager::new(oauth_client, network_config.clone());

        // Check authentication (needed for indexer queries)
        if !manager.is_authenticated()? {
            let result = serde_json::json!({
                "success": false,
                "error": "Not authenticated. Please run 'xion auth login' first.",
                "code": "NOT_AUTHENTICATED"
            });
            return print_formatted(&result, ctx.output_format());
        }

        // Export treasury state
        match manager.export_treasury(&address).await {
            Ok(export_data) => {
                // Build the output JSON
                let output = serde_json::json!({
                    "success": true,
                    "data": export_data
                });

                // Output to file or stdout
                if let Some(ref output_path) = args.output {
                    // Write to file
                    let json_string = serde_json::to_string_pretty(&output)?;
                    fs::write(output_path, json_string)?;
                    eprintln!("[INFO] Exported treasury config to: {}", output_path);
                    Ok(())
                } else {
                    // Output to stdout
                    print_formatted(&output, ctx.output_format())
                }
            }
            Err(e) => {
                let result = serde_json::json!({
                    "success": false,
                    "error": format!("Failed to export treasury: {}", e),
                    "code": "EXPORT_FAILED"
                });
                print_formatted(&result, ctx.output_format())
            }
        }
    } else {
        // Bulk export - list all treasuries and export each
        print_info("Exporting all treasury configurations...");

        // Create manager
        let config_manager = ConfigManager::new()?;
        let network_config = config_manager.get_network_config()?;
        let oauth_client = OAuthClient::new(network_config.clone())?;
        let manager = TreasuryManager::new(oauth_client, network_config.clone());

        // Check authentication (needed for indexer queries)
        if !manager.is_authenticated()? {
            let result = serde_json::json!({
                "success": false,
                "error": "Not authenticated. Please run 'xion auth login' first.",
                "code": "NOT_AUTHENTICATED"
            });
            return print_formatted(&result, ctx.output_format());
        }

        // List all treasuries
        let treasuries = match manager.list().await {
            Ok(t) => t,
            Err(e) => {
                let result = serde_json::json!({
                    "success": false,
                    "error": format!("Failed to list treasuries: {}", e),
                    "code": "EXPORT_LIST_FAILED"
                });
                return print_formatted(&result, ctx.output_format());
            }
        };

        if treasuries.is_empty() {
            let result = serde_json::json!({
                "success": true,
                "data": {
                    "exported_at": chrono::Utc::now().to_rfc3339(),
                    "network": ctx.network,
                    "treasuries": [],
                    "count": 0
                }
            });
            return print_formatted(&result, ctx.output_format());
        }

        // Export each treasury
        let mut export_results = Vec::new();
        let mut failed_exports = Vec::new();

        for (i, treasury) in treasuries.iter().enumerate() {
            eprintln!(
                "[INFO] Exporting treasury {}/{}: {}",
                i + 1,
                treasuries.len(),
                treasury.address
            );

            match manager.export_treasury(&treasury.address).await {
                Ok(export_data) => {
                    export_results.push(export_data);
                }
                Err(e) => {
                    eprintln!(
                        "[WARN] Failed to export treasury {}: {}",
                        treasury.address, e
                    );
                    failed_exports.push((treasury.address.clone(), e.to_string()));
                }
            }
        }

        // Build the bulk export config
        let export_config = TreasuryExportConfig {
            exported_at: chrono::Utc::now().to_rfc3339(),
            network: ctx.network.clone(),
            treasuries: export_results,
        };

        // Build the output
        let output = serde_json::json!({
            "success": true,
            "data": export_config,
            "failed_count": failed_exports.len(),
            "failed": if failed_exports.is_empty() { None } else { Some(failed_exports) }
        });

        // Output to file or stdout
        if let Some(ref output_path) = args.output {
            // Write to file
            let json_string = serde_json::to_string_pretty(&output)?;
            fs::write(output_path, json_string)?;
            eprintln!(
                "[INFO] Exported {} treasury configs to: {}",
                export_config.treasuries.len(),
                output_path
            );
            Ok(())
        } else {
            // Output to stdout
            print_formatted(&output, ctx.output_format())
        }
    }
}

// ============================================================================
// Import Handlers
// ============================================================================

async fn handle_import(args: ImportArgs, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::types::TreasuryExportData;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    if args.dry_run {
        print_info(&format!(
            "Previewing import to treasury {} (dry-run)...",
            args.address
        ));
    } else {
        print_info(&format!(
            "Importing configuration to treasury {}...",
            args.address
        ));
    }

    // Load import data from file
    let import_content = fs::read_to_string(&args.from_file)
        .map_err(|e| anyhow::anyhow!("Failed to read import file: {}", e))?;

    // Parse the import data
    let import_json: serde_json::Value = serde_json::from_str(&import_content)
        .map_err(|e| anyhow::anyhow!("Invalid JSON in import file: {}", e))?;

    // Extract TreasuryExportData - handle both direct format and wrapped format
    let export_data: TreasuryExportData = if import_json.is_object() {
        if let Some(data) = import_json.get("data") {
            // Wrapped format: {"success": true, "data": {...}}
            serde_json::from_value(data.clone())
                .map_err(|e| anyhow::anyhow!("Invalid import data format: {}", e))?
        } else {
            // Direct format: just the TreasuryExportData
            serde_json::from_value(import_json)
                .map_err(|e| anyhow::anyhow!("Invalid import data format: {}", e))?
        }
    } else {
        return Err(anyhow::anyhow!("Import file must contain a JSON object"));
    };

    // Validate the import data
    if export_data.address.is_empty() {
        return Err(anyhow::anyhow!(
            "Import data must contain a valid treasury address"
        ));
    }

    // Warn if import target differs from export source
    if !export_data.address.is_empty() && export_data.address != args.address {
        eprintln!(
            "[WARN] Import data is from treasury {} but applying to {}",
            export_data.address, args.address
        );
    }

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.clone());

    // Check authentication (required for transactions)
    if !manager.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion auth login' first.",
            "code": "NOT_AUTHENTICATED"
        });
        return print_formatted(&result, ctx.output_format());
    }

    // Import treasury configuration
    match manager
        .import_treasury(&args.address, &export_data, args.dry_run)
        .await
    {
        Ok(result) => print_formatted(&result, ctx.output_format()),
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to import treasury: {}", e),
                "code": "IMPORT_FAILED"
            });
            print_formatted(&result, ctx.output_format())
        }
    }
}

// ============================================================================
// Batch Operation Handlers
// ============================================================================

async fn handle_batch(cmd: BatchCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        BatchCommands::Fund { config } => handle_batch_fund(&config, ctx).await,
        BatchCommands::GrantConfig { config } => handle_batch_grant_config(&config, ctx).await,
    }
}

async fn handle_batch_fund(config_path: &PathBuf, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::types::BatchFundConfig;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Loading batch fund config from: {}",
        config_path.display()
    ));

    // Load config file
    let config_content = fs::read_to_string(config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
    let config: BatchFundConfig = serde_json::from_str(&config_content)
        .map_err(|e| anyhow::anyhow!("Invalid config file format: {}", e))?;

    // Validate config
    if config.operations.is_empty() {
        return Err(anyhow::anyhow!("Config file contains no operations"));
    }

    print_info(&format!(
        "Executing batch fund for {} treasuries...",
        config.operations.len()
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
        return print_formatted(&result, ctx.output_format());
    }

    // Execute operations sequentially (to avoid nonce issues)
    let mut results = Vec::new();
    let mut successful = 0;
    let mut failed = 0;

    for (i, operation) in config.operations.iter().enumerate() {
        eprintln!(
            "[INFO] Funding treasury {}/{}: {} with {}",
            i + 1,
            config.operations.len(),
            operation.address,
            operation.amount
        );

        match manager.fund(&operation.address, &operation.amount).await {
            Ok(result) => {
                eprintln!(
                    "[INFO] Successfully funded {}: tx_hash={}",
                    operation.address, result.tx_hash
                );
                results.push(serde_json::json!({
                    "address": operation.address,
                    "status": "success",
                    "amount": operation.amount,
                    "tx_hash": result.tx_hash
                }));
                successful += 1;
            }
            Err(e) => {
                let error_msg = format!("Failed to fund: {}", e);
                eprintln!("[ERROR] {}: {}", operation.address, error_msg);
                results.push(serde_json::json!({
                    "address": operation.address,
                    "status": "failed",
                    "amount": operation.amount,
                    "error": error_msg
                }));
                failed += 1;
            }
        }
    }

    // Build summary report
    let summary = serde_json::json!({
        "success": true,
        "data": {
            "total": config.operations.len(),
            "successful": successful,
            "failed": failed,
            "results": results
        }
    });

    eprintln!(
        "[INFO] Batch fund complete: {}/{} successful",
        successful,
        config.operations.len()
    );

    print_formatted(&summary, ctx.output_format())
}

async fn handle_batch_grant_config(config_path: &PathBuf, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::types::{AuthorizationInput, BatchGrantConfig, GrantConfigInput};
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_formatted, print_info};

    print_info(&format!(
        "Loading batch grant config from: {}",
        config_path.display()
    ));

    // Load config file
    let config_content = fs::read_to_string(config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
    let config: BatchGrantConfig = serde_json::from_str(&config_content)
        .map_err(|e| anyhow::anyhow!("Invalid config file format: {}", e))?;

    // Validate config
    if config.treasuries.is_empty() {
        return Err(anyhow::anyhow!("Config file contains no treasuries"));
    }

    // Validate required fields
    if config.message_type_url.is_empty() {
        return Err(anyhow::anyhow!("message_type_url is required in config"));
    }

    print_info(&format!(
        "Executing batch grant-config for {} treasuries...",
        config.treasuries.len()
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
        return print_formatted(&result, ctx.output_format());
    }

    // Build authorization based on grant type
    let authorization = match config.grant_type.as_str() {
        "send" => {
            let spend_limit = config
                .spend_limit
                .clone()
                .ok_or_else(|| anyhow::anyhow!("spend_limit is required for 'send' grant type"))?;
            AuthorizationInput::Send {
                spend_limit,
                allow_list: None,
            }
        }
        "generic" => AuthorizationInput::Generic,
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported grant_type: {}. Supported: send, generic",
                config.grant_type
            ));
        }
    };

    // Execute operations sequentially (to avoid nonce issues)
    let mut results = Vec::new();
    let mut successful = 0;
    let mut failed = 0;

    for (i, treasury) in config.treasuries.iter().enumerate() {
        eprintln!(
            "[INFO] Configuring grant for treasury {}/{}: {}",
            i + 1,
            config.treasuries.len(),
            treasury.address
        );

        // Use treasury-specific spend limit if provided, otherwise use global
        let treasury_authorization = if let Some(ref spend_limit) = treasury.spend_limit {
            match config.grant_type.as_str() {
                "send" => AuthorizationInput::Send {
                    spend_limit: spend_limit.clone(),
                    allow_list: None,
                },
                _ => authorization.clone(),
            }
        } else {
            authorization.clone()
        };

        let grant_config = GrantConfigInput {
            type_url: config.message_type_url.clone(),
            description: config
                .description
                .clone()
                .unwrap_or_else(|| format!("Batch grant config for {}", config.message_type_url)),
            authorization: treasury_authorization,
            optional: false,
        };

        match manager
            .add_grant_config(&treasury.address, grant_config)
            .await
        {
            Ok(result) => {
                eprintln!(
                    "[INFO] Successfully configured grant for {}: tx_hash={}",
                    treasury.address, result.tx_hash
                );
                results.push(serde_json::json!({
                    "address": treasury.address,
                    "status": "success",
                    "grant_type": config.grant_type,
                    "tx_hash": result.tx_hash
                }));
                successful += 1;
            }
            Err(e) => {
                let error_msg = format!("Failed to configure grant: {}", e);
                eprintln!("[ERROR] {}: {}", treasury.address, error_msg);
                results.push(serde_json::json!({
                    "address": treasury.address,
                    "status": "failed",
                    "grant_type": config.grant_type,
                    "error": error_msg
                }));
                failed += 1;
            }
        }
    }

    // Build summary report
    let summary = serde_json::json!({
        "success": true,
        "data": {
            "total": config.treasuries.len(),
            "successful": successful,
            "failed": failed,
            "results": results
        }
    });

    eprintln!(
        "[INFO] Batch grant-config complete: {}/{} successful",
        successful,
        config.treasuries.len()
    );

    print_formatted(&summary, ctx.output_format())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that all existing presets are correctly defined
    #[test]
    fn test_preset_types_count() {
        // We should have 22 presets (10 original + 12 new)
        assert_eq!(PRESET_TYPES.len(), 22);
    }

    /// Test that all preset names are unique
    #[test]
    fn test_preset_names_unique() {
        let mut names = std::collections::HashSet::new();
        for (name, _, _) in PRESET_TYPES {
            assert!(names.insert(*name), "Duplicate preset name: {}", name);
        }
    }

    /// Test that all type URLs are unique
    #[test]
    fn test_preset_type_urls_unique() {
        let mut type_urls = std::collections::HashSet::new();
        for (_, type_url, _) in PRESET_TYPES {
            assert!(
                type_urls.insert(*type_url),
                "Duplicate type URL: {}",
                type_url
            );
        }
    }

    /// Test preset resolution for all presets
    #[test]
    fn test_preset_resolution_all_presets() {
        // Test each preset can be found
        let preset_names: Vec<&str> = PRESET_TYPES.iter().map(|(name, _, _)| *name).collect();

        for name in preset_names {
            let found = PRESET_TYPES.iter().find(|(n, _, _)| *n == name);
            assert!(found.is_some(), "Preset '{}' should be found", name);
        }
    }

    /// Test specific preset resolutions (original presets)
    #[test]
    fn test_preset_resolution_original() {
        let test_cases = [
            ("send", "/cosmos.bank.v1beta1.MsgSend", "send"),
            (
                "execute",
                "/cosmwasm.wasm.v1.MsgExecuteContract",
                "contract-execution",
            ),
            (
                "instantiate",
                "/cosmwasm.wasm.v1.MsgInstantiateContract",
                "generic",
            ),
            (
                "instantiate2",
                "/cosmwasm.wasm.v1.MsgInstantiateContract2",
                "generic",
            ),
            ("delegate", "/cosmos.staking.v1beta1.MsgDelegate", "generic"),
            (
                "undelegate",
                "/cosmos.staking.v1beta1.MsgUndelegate",
                "generic",
            ),
            (
                "redelegate",
                "/cosmos.staking.v1beta1.MsgBeginRedelegate",
                "generic",
            ),
            (
                "withdraw-rewards",
                "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward",
                "generic",
            ),
            ("vote", "/cosmos.gov.v1beta1.MsgVote", "generic"),
            (
                "ibc-transfer",
                "/ibc.applications.transfer.v1.MsgTransfer",
                "ibc_transfer",
            ),
        ];

        for (name, expected_type_url, expected_auth_type) in test_cases {
            let found = PRESET_TYPES.iter().find(|(n, _, _)| *n == name);
            assert!(found.is_some(), "Preset '{}' should be found", name);
            let (_, type_url, auth_type) = found.unwrap();
            assert_eq!(
                *type_url, expected_type_url,
                "Preset '{}' has wrong type_url",
                name
            );
            assert_eq!(
                *auth_type, expected_auth_type,
                "Preset '{}' has wrong auth_type",
                name
            );
        }
    }

    /// Test specific preset resolutions (new governance presets)
    #[test]
    fn test_preset_resolution_governance() {
        let test_cases = [
            ("gov-deposit", "/cosmos.gov.v1beta1.MsgDeposit", "generic"),
            (
                "gov-submit-proposal",
                "/cosmos.gov.v1beta1.MsgSubmitProposal",
                "generic",
            ),
        ];

        for (name, expected_type_url, expected_auth_type) in test_cases {
            let found = PRESET_TYPES.iter().find(|(n, _, _)| *n == name);
            assert!(found.is_some(), "Preset '{}' should be found", name);
            let (_, type_url, auth_type) = found.unwrap();
            assert_eq!(
                *type_url, expected_type_url,
                "Preset '{}' has wrong type_url",
                name
            );
            assert_eq!(
                *auth_type, expected_auth_type,
                "Preset '{}' has wrong auth_type",
                name
            );
        }
    }

    /// Test specific preset resolutions (new authz presets)
    #[test]
    fn test_preset_resolution_authz() {
        let test_cases = [
            ("authz-exec", "/cosmos.authz.v1beta1.MsgExec", "generic"),
            ("authz-revoke", "/cosmos.authz.v1beta1.MsgRevoke", "generic"),
        ];

        for (name, expected_type_url, expected_auth_type) in test_cases {
            let found = PRESET_TYPES.iter().find(|(n, _, _)| *n == name);
            assert!(found.is_some(), "Preset '{}' should be found", name);
            let (_, type_url, auth_type) = found.unwrap();
            assert_eq!(
                *type_url, expected_type_url,
                "Preset '{}' has wrong type_url",
                name
            );
            assert_eq!(
                *auth_type, expected_auth_type,
                "Preset '{}' has wrong auth_type",
                name
            );
        }
    }

    /// Test specific preset resolutions (new feegrant presets)
    #[test]
    fn test_preset_resolution_feegrant() {
        let test_cases = [
            (
                "feegrant-grant",
                "/cosmos.feegrant.v1beta1.MsgGrantAllowance",
                "generic",
            ),
            (
                "feegrant-revoke",
                "/cosmos.feegrant.v1beta1.MsgRevokeAllowance",
                "generic",
            ),
        ];

        for (name, expected_type_url, expected_auth_type) in test_cases {
            let found = PRESET_TYPES.iter().find(|(n, _, _)| *n == name);
            assert!(found.is_some(), "Preset '{}' should be found", name);
            let (_, type_url, auth_type) = found.unwrap();
            assert_eq!(
                *type_url, expected_type_url,
                "Preset '{}' has wrong type_url",
                name
            );
            assert_eq!(
                *auth_type, expected_auth_type,
                "Preset '{}' has wrong auth_type",
                name
            );
        }
    }

    /// Test specific preset resolutions (new misc presets)
    #[test]
    fn test_preset_resolution_misc() {
        let test_cases = [
            ("unjail", "/cosmos.slashing.v1beta1.MsgUnjail", "generic"),
            (
                "crisis-verify",
                "/cosmos.crisis.v1beta1.MsgVerifyInvariant",
                "generic",
            ),
            (
                "evidence-submit",
                "/cosmos.evidence.v1beta1.MsgSubmitEvidence",
                "generic",
            ),
            (
                "vesting-create",
                "/cosmos.vesting.v1beta1.MsgCreateVestingAccount",
                "generic",
            ),
            (
                "tokenfactory-mint",
                "/osmosis.tokenfactory.v1beta1.MsgMint",
                "generic",
            ),
            (
                "tokenfactory-burn",
                "/osmosis.tokenfactory.v1beta1.MsgBurn",
                "generic",
            ),
        ];

        for (name, expected_type_url, expected_auth_type) in test_cases {
            let found = PRESET_TYPES.iter().find(|(n, _, _)| *n == name);
            assert!(found.is_some(), "Preset '{}' should be found", name);
            let (_, type_url, auth_type) = found.unwrap();
            assert_eq!(
                *type_url, expected_type_url,
                "Preset '{}' has wrong type_url",
                name
            );
            assert_eq!(
                *auth_type, expected_auth_type,
                "Preset '{}' has wrong auth_type",
                name
            );
        }
    }

    /// Test that new presets all use generic auth type (except special cases)
    #[test]
    fn test_new_presets_use_generic_auth_type() {
        let non_generic_presets = ["send", "execute", "ibc-transfer"];

        for (name, _, auth_type) in PRESET_TYPES {
            if non_generic_presets.contains(name) {
                // These have special auth types
                continue;
            }
            assert_eq!(
                *auth_type, "generic",
                "Preset '{}' should use 'generic' auth_type",
                name
            );
        }
    }

    /// Test that invalid preset name returns proper error
    #[test]
    fn test_invalid_preset_error() {
        let invalid_name = "invalid-preset";
        let found = PRESET_TYPES
            .iter()
            .find(|(name, _, _)| *name == invalid_name);
        assert!(found.is_none(), "Invalid preset should not be found");
    }
}
