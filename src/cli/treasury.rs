use clap::Subcommand;
use anyhow::Result;

#[derive(Subcommand)]
pub enum TreasuryCommands {
    /// List all treasury contracts for the authenticated user
    List,

    /// Query treasury contract details
    Query {
        /// Treasury contract address
        address: String,
    },

    /// Create a new treasury contract
    Create {
        /// Fee grant configuration (e.g., "basic:1000000uxion")
        #[arg(short, long)]
        fee_grant: Option<String>,

        /// Grant configuration (e.g., "authz:cosmwasm.wasm.v1.MsgExecuteContract")
        #[arg(short, long)]
        grant_config: Option<String>,
    },

    /// Fund a treasury contract
    Fund {
        /// Treasury contract address
        address: String,

        /// Amount to fund (e.g., "1000000uxion")
        #[arg(short, long)]
        amount: String,
    },

    /// Withdraw funds from a treasury contract
    Withdraw {
        /// Treasury contract address
        address: String,

        /// Amount to withdraw (e.g., "1000000uxion")
        #[arg(short, long)]
        amount: String,
    },
}

pub async fn handle_command(cmd: TreasuryCommands) -> Result<()> {
    match cmd {
        TreasuryCommands::List => handle_list().await,
        TreasuryCommands::Query { address } => handle_query(&address).await,
        TreasuryCommands::Create { fee_grant, grant_config } => {
            handle_create(fee_grant.as_deref(), grant_config.as_deref()).await
        }
        TreasuryCommands::Fund { address, amount } => handle_fund(&address, &amount).await,
        TreasuryCommands::Withdraw { address, amount } => handle_withdraw(&address, &amount).await,
    }
}

async fn handle_list() -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::TreasuryManager;
    use crate::utils::output::{print_json, print_info};
    
    print_info("Listing treasury contracts...");
    
    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.oauth_api_url);
    
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
    use crate::utils::output::{print_json, print_info};
    
    print_info(&format!("Querying treasury: {}", address));
    
    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;
    let oauth_client = OAuthClient::new(network_config.clone())?;
    let manager = TreasuryManager::new(oauth_client, network_config.oauth_api_url);
    
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

async fn handle_create(_fee_grant: Option<&str>, _grant_config: Option<&str>) -> Result<()> {
    use crate::utils::output::{print_json, print_info};
    
    print_info("Creating treasury contract...");
    
    let result = serde_json::json!({
        "success": false,
        "message": "Treasury creation not yet implemented",
        "suggestion": "Use the Developer Portal to create Treasury contracts"
    });
    
    print_json(&result)
}

async fn handle_fund(_address: &str, _amount: &str) -> Result<()> {
    use crate::utils::output::{print_json, print_info};
    
    print_info("Funding treasury...");
    
    let result = serde_json::json!({
        "success": false,
        "message": "Treasury funding not yet implemented",
        "suggestion": "Use xiond CLI to fund Treasury contracts"
    });
    
    print_json(&result)
}

async fn handle_withdraw(_address: &str, _amount: &str) -> Result<()> {
    use crate::utils::output::{print_json, print_info};
    
    print_info("Withdrawing from treasury...");
    
    let result = serde_json::json!({
        "success": false,
        "message": "Treasury withdrawal not yet implemented",
        "suggestion": "Use xiond CLI to withdraw from Treasury contracts"
    });
    
    print_json(&result)
}
