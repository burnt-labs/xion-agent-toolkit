//! Faucet Command Module
//!
//! CLI commands for interacting with the Xion testnet faucet contract.
//! Allows users to claim testnet tokens without browser interaction.

use anyhow::{Context, Result};
use clap::Subcommand;
use serde::{Deserialize, Serialize};

use crate::cli::ExecuteContext;
use crate::utils::output::{print_formatted, print_info};

/// Faucet contract address on testnet
const FAUCET_TESTNET_ADDRESS: &str =
    "xion1kv2mz7yjk5azuuq7ptd7hrl7trwphu5enereqv8t66rkre00dxxqac9ywl";

/// Amount per claim in uxion (1 XION)
const FAUCET_AMOUNT: u64 = 1_000_000;

/// Cooldown period in seconds (24 hours)
const FAUCET_COOLDOWN_SECS: u64 = 86_400;

/// Faucet command variants
#[derive(Subcommand)]
pub enum FaucetCommands {
    /// Claim testnet tokens from the faucet
    Claim {
        /// Receiver address for delegated claims (optional)
        /// If provided, uses the Delegate message instead of Faucet
        #[arg(long)]
        receiver: Option<String>,
    },

    /// Check faucet claim status and cooldown
    Status {
        /// Address to check (defaults to current authenticated address)
        #[arg(long)]
        address: Option<String>,
    },

    /// Query faucet configuration
    Info,
}

/// Faucet claim response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaucetClaimResult {
    /// Whether the claim was successful
    pub success: bool,
    /// Transaction hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Amount claimed in uxion
    pub amount: u64,
    /// Receiver address (for delegated claims)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<String>,
}

/// Faucet status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaucetStatusResult {
    /// Address checked
    pub address: String,
    /// Last claim timestamp (Unix seconds, 0 if never claimed)
    pub last_claim_timestamp: u64,
    /// Whether the user can claim now
    pub can_claim: bool,
    /// Remaining cooldown time in seconds (0 if can claim)
    pub remaining_cooldown_secs: u64,
    /// Human-readable remaining time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_cooldown_human: Option<String>,
}

/// Faucet info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaucetInfoResult {
    /// Faucet contract address
    pub faucet_address: String,
    /// Amount per claim in uxion
    pub amount: u64,
    /// Cooldown period in seconds
    pub cooldown_secs: u64,
    /// Denomination
    pub denom: String,
    /// Network
    pub network: String,
}

/// Handle faucet commands
pub async fn handle_command(cmd: FaucetCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        FaucetCommands::Claim { receiver } => handle_claim(receiver.as_deref(), ctx).await,
        FaucetCommands::Status { address } => handle_status(address.as_deref(), ctx).await,
        FaucetCommands::Info => handle_info(ctx).await,
    }
}

/// Handle faucet claim command
async fn handle_claim(receiver: Option<&str>, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::api_client::TreasuryApiClient;

    print_info("Claiming tokens from faucet...");

    // Create manager
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;

    // Check if faucet is available on this network
    if network_config.network_name != "testnet" {
        let result = serde_json::json!({
            "success": false,
            "error": "Faucet is only available on testnet",
            "code": "EFAUCET004",
            "hint": "Use --network testnet to claim testnet tokens"
        });
        return print_formatted(&result, ctx.output_format());
    }

    let oauth_client = OAuthClient::new(network_config.clone())?;

    // Check authentication
    if !oauth_client.is_authenticated()? {
        let result = serde_json::json!({
            "success": false,
            "error": "Not authenticated. Please run 'xion-toolkit auth login' first.",
            "code": "EFAUCET003",
            "hint": "Run 'xion-toolkit auth login' to authenticate"
        });
        return print_formatted(&result, ctx.output_format());
    }

    // Get credentials
    let credentials = oauth_client
        .get_credentials()?
        .context("No credentials found")?;

    let sender = credentials
        .xion_address
        .context("No Xion address found in credentials")?;

    let access_token = oauth_client.get_valid_token().await?.access_token;

    // Create API client
    let api_client = TreasuryApiClient::new(
        network_config.oauth_api_url.clone(),
        network_config.indexer_url.clone(),
        network_config.rest_url.clone(),
    );

    // Build execute message
    let execute_msg = if let Some(recv) = receiver {
        // Validate receiver address format (basic check: must start with xion1 and be reasonable length)
        if !recv.starts_with("xion1") || recv.len() < 10 {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Invalid receiver address format: {}", recv),
                "code": "EFAUCET001",
                "hint": "Receiver address must be a valid Xion address starting with 'xion1'"
            });
            return print_formatted(&result, ctx.output_format());
        }
        serde_json::json!({
            "delegate": {
                "receiver_address": recv
            }
        })
    } else {
        serde_json::json!({
            "faucet": {}
        })
    };

    // Broadcast transaction
    let tx_hash = match api_client
        .broadcast_execute_contract(
            &access_token,
            &sender,
            FAUCET_TESTNET_ADDRESS,
            &execute_msg,
            None,
            "Claim from faucet via Xion Agent Toolkit",
        )
        .await
    {
        Ok(hash) => hash,
        Err(e) => {
            let error_msg = e.to_string();
            let (code, hint) = parse_faucet_error(&error_msg);
            let result = serde_json::json!({
                "success": false,
                "error": format!("Faucet claim failed: {}", error_msg),
                "code": code,
                "hint": hint
            });
            return print_formatted(&result, ctx.output_format());
        }
    };

    let result = FaucetClaimResult {
        success: true,
        tx_hash: Some(tx_hash),
        amount: FAUCET_AMOUNT,
        receiver: receiver.map(|s| s.to_string()),
    };

    print_formatted(&result, ctx.output_format())
}

/// Handle faucet status command
async fn handle_status(address: Option<&str>, ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;
    use crate::treasury::api_client::TreasuryApiClient;

    // Check network first
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;

    // Check if faucet is available on this network
    if network_config.network_name != "testnet" {
        let result = serde_json::json!({
            "success": false,
            "error": "Faucet is only available on testnet",
            "code": "EFAUCET004",
            "hint": "Use --network testnet to query faucet status"
        });
        return print_formatted(&result, ctx.output_format());
    }

    // Determine address to check
    let check_address = if let Some(addr) = address {
        // Validate address format (basic check: must start with xion1 and be reasonable length)
        if !addr.starts_with("xion1") || addr.len() < 10 {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Invalid address format: {}", addr),
                "code": "EFAUCET001",
                "hint": "Address must be a valid Xion address starting with 'xion1'"
            });
            return print_formatted(&result, ctx.output_format());
        }
        addr.to_string()
    } else {
        // Default to current authenticated address
        let oauth_client = OAuthClient::new(network_config.clone())?;

        if !oauth_client.is_authenticated()? {
            let result = serde_json::json!({
                "success": false,
                "error": "Not authenticated. Provide --address or run 'xion-toolkit auth login' first.",
                "code": "EFAUCET003",
                "hint": "Provide --address flag or authenticate with 'xion-toolkit auth login'"
            });
            return print_formatted(&result, ctx.output_format());
        }

        let credentials = oauth_client
            .get_credentials()?
            .context("No credentials found")?;

        credentials
            .xion_address
            .context("No Xion address found in credentials")?
    };

    print_info(&format!("Checking faucet status for: {}", check_address));

    // Create API client for query
    let api_client = TreasuryApiClient::new(
        network_config.oauth_api_url.clone(),
        network_config.indexer_url.clone(),
        network_config.rest_url.clone(),
    );

    // Query last faucet timestamp
    let query_msg = serde_json::json!({
        "get_address_last_faucet_timestamp": {
            "address": check_address
        }
    });

    let query_result = match api_client
        .query_contract_smart(FAUCET_TESTNET_ADDRESS, &query_msg)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to query faucet status: {}", e),
                "code": "EFAUCET002",
                "hint": "Check your network connection and verify the faucet contract is available"
            });
            return print_formatted(&result, ctx.output_format());
        }
    };

    // Parse timestamp from response
    let last_claim_timestamp = query_result
        .get("timestamp")
        .and_then(|t| t.as_u64())
        .unwrap_or(0);

    // Calculate cooldown
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let elapsed = now.saturating_sub(last_claim_timestamp);
    let remaining = FAUCET_COOLDOWN_SECS.saturating_sub(elapsed);
    let can_claim = remaining == 0;

    // Format human-readable remaining time
    let remaining_cooldown_human = if remaining > 0 {
        Some(format_cooldown(remaining))
    } else {
        None
    };

    let result = FaucetStatusResult {
        address: check_address,
        last_claim_timestamp,
        can_claim,
        remaining_cooldown_secs: remaining,
        remaining_cooldown_human,
    };

    print_formatted(&result, ctx.output_format())
}

/// Handle faucet info command
async fn handle_info(ctx: &ExecuteContext) -> Result<()> {
    use crate::config::ConfigManager;
    use crate::treasury::api_client::TreasuryApiClient;

    print_info("Querying faucet configuration...");

    // Create API client for query
    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;

    // Check if faucet is available on this network
    if network_config.network_name != "testnet" {
        let result = serde_json::json!({
            "success": false,
            "error": "Faucet is only available on testnet",
            "code": "EFAUCET004",
            "hint": "Use --network testnet to query faucet configuration"
        });
        return print_formatted(&result, ctx.output_format());
    }

    // Query faucet denom
    let api_client = TreasuryApiClient::new(
        network_config.oauth_api_url.clone(),
        network_config.indexer_url.clone(),
        network_config.rest_url.clone(),
    );

    let query_msg = serde_json::json!({
        "get_denom": {}
    });

    let query_result = match api_client
        .query_contract_smart(FAUCET_TESTNET_ADDRESS, &query_msg)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": format!("Failed to query faucet info: {}", e),
                "code": "EFAUCET002",
                "hint": "Check your network connection and verify the faucet contract is available"
            });
            return print_formatted(&result, ctx.output_format());
        }
    };

    let denom = query_result
        .get("denom")
        .and_then(|d| d.as_str())
        .unwrap_or("uxion")
        .to_string();

    let result = FaucetInfoResult {
        faucet_address: FAUCET_TESTNET_ADDRESS.to_string(),
        amount: FAUCET_AMOUNT,
        cooldown_secs: FAUCET_COOLDOWN_SECS,
        denom,
        network: network_config.network_name.clone(),
    };

    print_formatted(&result, ctx.output_format())
}

/// Parse faucet error and return (code, hint)
fn parse_faucet_error(error_msg: &str) -> (&'static str, &'static str) {
    let error_lower = error_msg.to_lowercase();

    if error_lower.contains("cooldown") || error_lower.contains("not met") {
        (
            "EFAUCET001",
            "Wait for the 24-hour cooldown period to expire before claiming again",
        )
    } else if error_lower.contains("balance gate") || error_lower.contains("too many") {
        (
            "EFAUCET001",
            "Your balance exceeds the faucet threshold. Transfer some tokens out to claim",
        )
    } else if error_lower.contains("insufficient") || error_lower.contains("empty") {
        (
            "EFAUCET001",
            "The faucet is temporarily out of funds. Try again later",
        )
    } else if error_lower.contains("unauthorized") || error_lower.contains("not allowed") {
        (
            "EFAUCET001",
            "You are not authorized to claim from this faucet",
        )
    } else {
        (
            "EFAUCET001",
            "Check the error message for details or try again later",
        )
    }
}

/// Format cooldown seconds to human-readable string
fn format_cooldown(secs: u64) -> String {
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_cooldown() {
        assert_eq!(format_cooldown(0), "0s");
        assert_eq!(format_cooldown(30), "30s");
        assert_eq!(format_cooldown(90), "1m 30s");
        assert_eq!(format_cooldown(3661), "1h 1m 1s");
        assert_eq!(format_cooldown(86400), "24h 0m 0s");
    }

    #[test]
    fn test_parse_faucet_error() {
        let (code, hint) = parse_faucet_error("Cooldown not met");
        assert_eq!(code, "EFAUCET001");
        assert!(hint.contains("cooldown"));

        let (code, hint) = parse_faucet_error("Balance gate failed");
        assert_eq!(code, "EFAUCET001");
        assert!(hint.contains("threshold"));

        let (code, hint) = parse_faucet_error("Insufficient funds");
        assert_eq!(code, "EFAUCET001");
        assert!(hint.contains("out of funds"));
    }

    #[test]
    fn test_faucet_claim_result_serialization() {
        let result = FaucetClaimResult {
            success: true,
            tx_hash: Some("ABC123".to_string()),
            amount: 1_000_000,
            receiver: None,
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"amount\":1000000"));
    }

    #[test]
    fn test_faucet_status_result_serialization() {
        let result = FaucetStatusResult {
            address: "xion1test".to_string(),
            last_claim_timestamp: 12345,
            can_claim: false,
            remaining_cooldown_secs: 3600,
            remaining_cooldown_human: Some("1h 0m 0s".to_string()),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"can_claim\":false"));
        assert!(json.contains("\"remaining_cooldown_secs\":3600"));
    }

    #[test]
    fn test_faucet_info_result_serialization() {
        let result = FaucetInfoResult {
            faucet_address: FAUCET_TESTNET_ADDRESS.to_string(),
            amount: 1_000_000,
            cooldown_secs: 86_400,
            denom: "uxion".to_string(),
            network: "testnet".to_string(),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"amount\":1000000"));
        assert!(json.contains("\"cooldown_secs\":86400"));
    }
}
