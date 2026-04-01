//! OAuth2 Client Management CLI Commands
//!
//! CLI commands for managing OAuth2 clients via the Xion MGR API.
//! Covers CRUD operations, extension management, manager management,
//! and ownership transfer.

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tracing::debug;

/// OAuth2 scopes required for Manager API operations.
const MGR_REQUIRED_SCOPES: &[&str] = &["xion:mgr:read", "xion:mgr:write"];

use crate::api::mgr_api::{
    CreateClientRequest, MgrApiClient, UpdateClientRequest, UpdateExtensionRequest,
};
use crate::cli::ExecuteContext;
use crate::utils::output::{print_formatted, print_warning};

// ============================================================================
// Command Definitions
// ============================================================================

/// OAuth2 command group
#[derive(Subcommand)]
pub enum OAuth2Commands {
    /// Manage OAuth2 clients
    #[command(subcommand)]
    Client(OAuth2ClientCommands),
}

/// OAuth2 client subcommands
#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum OAuth2ClientCommands {
    /// List OAuth clients for the authenticated user
    List(ListArgs),

    /// Create a new OAuth client
    Create(CreateClientArgs),

    /// Get a specific OAuth client by ID
    Get(GetArgs),

    /// Update an existing OAuth client
    Update(UpdateClientArgs),

    /// Delete an OAuth client
    Delete(DeleteArgs),

    /// Get or update client extension data
    #[command(subcommand)]
    Extension(ExtensionCommands),

    /// Add or remove managers
    #[command(subcommand)]
    Managers(ManagersCommands),

    /// Transfer client ownership to a new user
    TransferOwnership(TransferOwnershipArgs),
}

/// Arguments for listing OAuth clients
#[derive(Args, Debug)]
pub struct ListArgs {
    /// Maximum number of items to return
    #[arg(long)]
    pub limit: Option<u32>,
    /// Pagination cursor for next page
    #[arg(long)]
    pub cursor: Option<String>,
}

/// Arguments for getting a specific OAuth client
#[derive(Args, Debug)]
pub struct GetArgs {
    /// Client ID to query
    pub client_id: String,
}

/// Arguments for deleting an OAuth client
#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// Client ID to delete
    pub client_id: String,
    /// Skip confirmation prompt and force deletion
    #[arg(long)]
    pub force: bool,
}

/// Arguments for transferring client ownership
#[derive(Args, Debug)]
pub struct TransferOwnershipArgs {
    /// Client ID
    pub client_id: String,
    /// User ID of the new owner
    #[arg(long)]
    pub new_owner: String,
    /// Skip confirmation prompt and force ownership transfer
    #[arg(long)]
    pub force: bool,
}

/// Arguments for creating an OAuth client
#[derive(Args, Debug)]
pub struct CreateClientArgs {
    /// OAuth redirect URIs (required, comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub redirect_uris: Vec<String>,
    /// Treasury contract address to bind (required unless --json-input is provided)
    #[arg(long)]
    pub treasury: Option<String>,
    /// Human-readable client name
    #[arg(long)]
    pub client_name: Option<String>,
    /// Client owner user ID (defaults to authenticated user)
    #[arg(long)]
    pub owner: Option<String>,
    /// Manager user IDs (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub managers: Option<Vec<String>>,
    /// Token endpoint auth method (none, client_secret_basic, client_secret_post)
    #[arg(long)]
    pub auth_method: Option<String>,
    /// Contact email addresses (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub contacts: Option<Vec<String>>,
    /// Client homepage URL
    #[arg(long)]
    pub client_uri: Option<String>,
    /// Client logo URL
    #[arg(long)]
    pub logo_uri: Option<String>,
    /// Privacy policy URL
    #[arg(long)]
    pub policy_uri: Option<String>,
    /// Terms of service URL
    #[arg(long)]
    pub tos_uri: Option<String>,
    /// JWKS endpoint URL
    #[arg(long)]
    pub jwks_uri: Option<String>,
    /// Path to JSON file with full request body
    #[arg(long, value_name = "FILE")]
    pub json_input: Option<PathBuf>,
    /// Show client secret in output (default: redacted)
    #[arg(long)]
    pub show_secret: bool,
}

/// Extension subcommands
#[derive(Subcommand)]
pub enum ExtensionCommands {
    /// Get the extension data for a client
    Get {
        /// Client ID
        client_id: String,
    },

    /// Update the extension data for a client
    Update {
        /// Client ID
        client_id: String,
        /// Manager user IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        managers: Option<Vec<String>>,
    },
}

/// Arguments for updating an OAuth client
#[derive(Args, Debug)]
pub struct UpdateClientArgs {
    /// Client ID to update
    pub client_id: String,
    /// OAuth redirect URIs (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub redirect_uris: Option<Vec<String>>,
    /// Human-readable client name
    #[arg(long)]
    pub client_name: Option<String>,
    /// Client homepage URL
    #[arg(long)]
    pub client_uri: Option<String>,
    /// Client logo URL
    #[arg(long)]
    pub logo_uri: Option<String>,
    /// Privacy policy URL
    #[arg(long)]
    pub policy_uri: Option<String>,
    /// Terms of service URL
    #[arg(long)]
    pub tos_uri: Option<String>,
    /// JWKS endpoint URL
    #[arg(long)]
    pub jwks_uri: Option<String>,
    /// Contact email addresses (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub contacts: Option<Vec<String>>,
    /// Path to JSON file with full request body
    #[arg(long, value_name = "FILE")]
    pub json_input: Option<PathBuf>,
}

/// Manager subcommands
#[derive(Subcommand)]
pub enum ManagersCommands {
    /// Add a manager to a client
    Add {
        /// Client ID
        client_id: String,
        /// User ID of the manager to add
        #[arg(long)]
        manager_id: String,
    },

    /// Remove a manager from a client
    Remove {
        /// Client ID
        client_id: String,
        /// User ID of the manager to remove
        #[arg(long)]
        manager_id: String,
    },
}

// ============================================================================
// Handler Entry Point
// ============================================================================

/// Handle OAuth2 commands
pub async fn handle_command(cmd: OAuth2Commands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        OAuth2Commands::Client(client_cmd) => handle_client_command(client_cmd, ctx).await,
    }
}

/// Handle OAuth2 client subcommands
async fn handle_client_command(cmd: OAuth2ClientCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        OAuth2ClientCommands::List(args) => handle_list(args, ctx).await,
        OAuth2ClientCommands::Create(args) => handle_create(args, ctx).await,
        OAuth2ClientCommands::Get(args) => handle_get(&args.client_id, ctx).await,
        OAuth2ClientCommands::Update(args) => handle_update(args, ctx).await,
        OAuth2ClientCommands::Delete(args) => handle_delete(&args.client_id, args.force, ctx).await,
        OAuth2ClientCommands::Extension(ext_cmd) => handle_extension(ext_cmd, ctx).await,
        OAuth2ClientCommands::Managers(mgr_cmd) => handle_managers(mgr_cmd, ctx).await,
        OAuth2ClientCommands::TransferOwnership(args) => {
            handle_transfer_ownership(&args.client_id, &args.new_owner, args.force, ctx).await
        }
    }
}

// ============================================================================
// Shared Setup Helper
// ============================================================================

/// Shared setup: create OAuthClient, get valid token, create MgrApiClient
///
/// Returns (access_token, mgr_client)
async fn prepare_api_client(_ctx: &ExecuteContext) -> Result<(String, MgrApiClient)> {
    use crate::config::ConfigManager;
    use crate::oauth::OAuthClient;

    let config_manager = ConfigManager::new()?;
    let network_config = config_manager.get_network_config()?;

    let oauth_client = OAuthClient::new(network_config.clone())?;

    // Check authentication
    if !oauth_client.is_authenticated()? {
        anyhow::bail!("Not authenticated. Please run 'xion-toolkit auth login' first.");
    }

    let credentials = oauth_client.get_valid_token().await?;

    // Pre-flight scope validation: fail fast if token lacks required scopes
    if !credentials.has_all_scopes(MGR_REQUIRED_SCOPES) {
        let missing: Vec<&str> = MGR_REQUIRED_SCOPES
            .iter()
            .filter(|s| !credentials.has_scope(s))
            .copied()
            .collect();
        anyhow::bail!(
            "Insufficient scope: missing {}. Re-login with --dev-mode: xion-toolkit auth login --dev-mode",
            missing.join(", ")
        );
    }

    let access_token = credentials.access_token;

    // Never log the access token
    debug!("Obtained valid access token for MGR API call");

    let mgr_client = MgrApiClient::new(network_config.oauth_api_url)?;

    Ok((access_token, mgr_client))
}

/// Format an XionError into a structured error JSON output for print_formatted
#[derive(Serialize)]
struct CliErrorResponse {
    success: bool,
    error: CliErrorDetail,
}

#[derive(Serialize)]
struct CliErrorDetail {
    code: String,
    message: String,
    remediation: String,
}

impl CliErrorResponse {
    fn from_xion_error(err: &crate::shared::error::XionError) -> Self {
        Self {
            success: false,
            error: CliErrorDetail {
                code: format!("{:?}", err.code()),
                message: err.to_string(),
                remediation: err.hint(),
            },
        }
    }
}

// ============================================================================
// Command Handlers
// ============================================================================

/// Handle `oauth2 client list`
async fn handle_list(args: ListArgs, ctx: &ExecuteContext) -> Result<()> {
    let (access_token, mgr_client) = prepare_api_client(ctx).await?;

    let result = mgr_client
        .list_clients(&access_token, args.limit, args.cursor.as_deref())
        .await
        .map_err(|e| {
            let resp = CliErrorResponse::from_xion_error(&e);
            anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
        })?;

    print_formatted(&result, ctx.output_format())
}

/// Handle `oauth2 client create`
async fn handle_create(args: CreateClientArgs, ctx: &ExecuteContext) -> Result<()> {
    let (access_token, mgr_client) = prepare_api_client(ctx).await?;

    // Build request from flags or JSON input file
    let request = if let Some(json_path) = &args.json_input {
        let content = fs::read_to_string(json_path)
            .with_context(|| format!("Failed to read JSON input file: {}", json_path.display()))?;
        serde_json::from_str::<CreateClientRequest>(&content)
            .with_context(|| format!("Failed to parse JSON input file: {}", json_path.display()))?
    } else {
        if args.redirect_uris.is_empty() {
            anyhow::bail!("--redirect-uris is required (at least one URI)");
        }
        let treasury = args
            .treasury
            .ok_or_else(|| anyhow::anyhow!("--treasury is required when not using --json-input"))?;

        CreateClientRequest {
            redirect_uris: args.redirect_uris,
            client_name: args.client_name,
            client_uri: args.client_uri,
            logo_uri: args.logo_uri,
            policy_uri: args.policy_uri,
            tos_uri: args.tos_uri,
            jwks_uri: args.jwks_uri,
            contacts: args.contacts,
            token_endpoint_auth_method: args.auth_method,
            binded_treasury: treasury,
            owner: args.owner,
            managers: args.managers,
        }
    };

    let result = mgr_client
        .create_client(&access_token, request)
        .await
        .map_err(|e| {
            let resp = CliErrorResponse::from_xion_error(&e);
            anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
        })?;

    // Apply secret redaction
    let output = if args.show_secret {
        // Warn on stderr
        print_warning("Client secret is visible in output. Store securely.");
        serde_json::to_value(&result)?
    } else {
        // Redact the secret
        let mut value = serde_json::to_value(&result)?;
        if let Some(secret) = value.get_mut("clientSecret") {
            *secret = serde_json::json!("********");
        }
        value
    };

    print_formatted(&output, ctx.output_format())
}

/// Handle `oauth2 client get`
async fn handle_get(client_id: &str, ctx: &ExecuteContext) -> Result<()> {
    let (access_token, mgr_client) = prepare_api_client(ctx).await?;

    let result = mgr_client
        .get_client(&access_token, client_id)
        .await
        .map_err(|e| {
            let resp = CliErrorResponse::from_xion_error(&e);
            anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
        })?;

    print_formatted(&result, ctx.output_format())
}

/// Handle `oauth2 client update`
async fn handle_update(args: UpdateClientArgs, ctx: &ExecuteContext) -> Result<()> {
    let (access_token, mgr_client) = prepare_api_client(ctx).await?;

    let request = if let Some(json_path) = &args.json_input {
        let content = fs::read_to_string(json_path)
            .with_context(|| format!("Failed to read JSON input file: {}", json_path.display()))?;
        serde_json::from_str::<UpdateClientRequest>(&content)
            .with_context(|| format!("Failed to parse JSON input file: {}", json_path.display()))?
    } else {
        UpdateClientRequest {
            redirect_uris: args.redirect_uris,
            client_name: args.client_name,
            client_uri: args.client_uri,
            logo_uri: args.logo_uri,
            policy_uri: args.policy_uri,
            tos_uri: args.tos_uri,
            jwks_uri: args.jwks_uri,
            contacts: args.contacts,
        }
    };

    let result = mgr_client
        .update_client(&access_token, &args.client_id, request)
        .await
        .map_err(|e| {
            let resp = CliErrorResponse::from_xion_error(&e);
            anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
        })?;

    print_formatted(&result, ctx.output_format())
}

/// Handle `oauth2 client delete`
async fn handle_delete(client_id: &str, force: bool, ctx: &ExecuteContext) -> Result<()> {
    if !force {
        print_warning("This will permanently delete the client and cannot be undone.");
        eprintln!(
            "Use --force to confirm: oauth2 client delete {} --force",
            client_id
        );
        return Err(
            crate::shared::error::OAuthClientError::ConfirmationRequired {
                message: format!(
                    "Destructive operation cancelled. Re-run with --force to confirm deletion of client '{}'.",
                    client_id
                ),
            }
            .into(),
        );
    }

    let (access_token, mgr_client) = prepare_api_client(ctx).await?;

    let result = mgr_client
        .delete_client(&access_token, client_id)
        .await
        .map_err(|e| {
            let resp = CliErrorResponse::from_xion_error(&e);
            anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
        })?;

    print_formatted(&result, ctx.output_format())
}

/// Handle `oauth2 client extension` subcommands
async fn handle_extension(cmd: ExtensionCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        ExtensionCommands::Get { client_id } => {
            let (access_token, mgr_client) = prepare_api_client(ctx).await?;

            let result = mgr_client
                .get_extension(&access_token, &client_id)
                .await
                .map_err(|e| {
                    let resp = CliErrorResponse::from_xion_error(&e);
                    anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
                })?;

            print_formatted(&result, ctx.output_format())
        }
        ExtensionCommands::Update {
            client_id,
            managers,
        } => {
            let (access_token, mgr_client) = prepare_api_client(ctx).await?;

            let request = UpdateExtensionRequest {
                managers: managers.unwrap_or_default(),
            };

            let result = mgr_client
                .update_extension(&access_token, &client_id, request)
                .await
                .map_err(|e| {
                    let resp = CliErrorResponse::from_xion_error(&e);
                    anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
                })?;

            print_formatted(&result, ctx.output_format())
        }
    }
}

/// Handle `oauth2 client managers` subcommands
async fn handle_managers(cmd: ManagersCommands, ctx: &ExecuteContext) -> Result<()> {
    match cmd {
        ManagersCommands::Add {
            client_id,
            manager_id,
        } => {
            let (access_token, mgr_client) = prepare_api_client(ctx).await?;

            let result = mgr_client
                .add_manager(&access_token, &client_id, &manager_id)
                .await
                .map_err(|e| {
                    let resp = CliErrorResponse::from_xion_error(&e);
                    anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
                })?;

            print_formatted(&result, ctx.output_format())
        }
        ManagersCommands::Remove {
            client_id,
            manager_id,
        } => {
            let (access_token, mgr_client) = prepare_api_client(ctx).await?;

            let result = mgr_client
                .remove_manager(&access_token, &client_id, &manager_id)
                .await
                .map_err(|e| {
                    let resp = CliErrorResponse::from_xion_error(&e);
                    anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
                })?;

            print_formatted(&result, ctx.output_format())
        }
    }
}

/// Handle `oauth2 client transfer-ownership`
async fn handle_transfer_ownership(
    client_id: &str,
    new_owner: &str,
    force: bool,
    ctx: &ExecuteContext,
) -> Result<()> {
    if !force {
        print_warning("This will permanently transfer ownership and cannot be undone.");
        eprintln!(
            "Use --force to confirm: oauth2 client transfer-ownership {} --new-owner {} --force",
            client_id, new_owner
        );
        return Err(
            crate::shared::error::OAuthClientError::ConfirmationRequired {
                message: format!(
                    "Destructive operation cancelled. Re-run with --force to confirm ownership transfer of client '{}' to '{}'.",
                    client_id, new_owner
                ),
            }
            .into(),
        );
    }

    let (access_token, mgr_client) = prepare_api_client(ctx).await?;

    let result = mgr_client
        .transfer_ownership(&access_token, client_id, new_owner)
        .await
        .map_err(|e| {
            let resp = CliErrorResponse::from_xion_error(&e);
            anyhow::anyhow!(serde_json::to_string(&resp).unwrap_or_else(|_| e.to_string()))
        })?;

    print_formatted(&result, ctx.output_format())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    /// Helper to parse OAuth2 client subcommands from args.
    ///
    /// Prepends a fake binary name ("xion-toolkit") so that `parse_from`
    /// does not consume the first real argument as the program name.
    fn parse_oauth2_client(args: &[&str]) -> OAuth2ClientCommands {
        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            command: OAuth2ClientCommands,
        }
        let cli = TestCli::parse_from(
            std::iter::once("xion-toolkit" as &str).chain(args.iter().copied()),
        );
        cli.command
    }

    #[test]
    fn test_debug_get_parsing() {
        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            command: OAuth2ClientCommands,
        }
        let result = TestCli::try_parse_from(["xion-toolkit", "get", "client_abc123"]);
        match result {
            Ok(cli) => match cli.command {
                OAuth2ClientCommands::Get(args) => {
                    assert_eq!(args.client_id, "client_abc123");
                }
                _ => panic!("Wrong variant"),
            },
            Err(e) => {
                panic!("Parse failed: {}", e);
            }
        }
    }

    #[test]
    fn test_list_default_args() {
        let cmd = parse_oauth2_client(&["list"]);
        match cmd {
            OAuth2ClientCommands::List(args) => {
                assert!(args.limit.is_none());
                assert!(args.cursor.is_none());
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_list_with_flags() {
        let cmd = parse_oauth2_client(&["list", "--limit", "10", "--cursor", "abc"]);
        match cmd {
            OAuth2ClientCommands::List(args) => {
                assert_eq!(args.limit, Some(10));
                assert_eq!(args.cursor.as_deref(), Some("abc"));
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_create_required_args() {
        let cmd = parse_oauth2_client(&[
            "create",
            "--redirect-uris",
            "https://example.com/callback",
            "--treasury",
            "xion1abc123",
        ]);
        match cmd {
            OAuth2ClientCommands::Create(args) => {
                assert_eq!(args.redirect_uris, vec!["https://example.com/callback"]);
                assert_eq!(args.treasury.as_deref(), Some("xion1abc123"));
                assert!(!args.show_secret);
                assert!(args.client_name.is_none());
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_create_all_args() {
        let cmd = parse_oauth2_client(&[
            "create",
            "--redirect-uris",
            "https://a.com/cb,https://b.com/cb",
            "--treasury",
            "xion1abc123",
            "--client-name",
            "My App",
            "--owner",
            "user_123",
            "--managers",
            "user_456,user_789",
            "--auth-method",
            "client_secret_basic",
            "--contacts",
            "admin@example.com",
            "--client-uri",
            "https://myapp.com",
            "--logo-uri",
            "https://myapp.com/logo.png",
            "--policy-uri",
            "https://myapp.com/privacy",
            "--tos-uri",
            "https://myapp.com/terms",
            "--jwks-uri",
            "https://myapp.com/.well-known/jwks.json",
            "--show-secret",
        ]);
        match cmd {
            OAuth2ClientCommands::Create(args) => {
                assert_eq!(args.redirect_uris.len(), 2);
                assert_eq!(args.treasury.as_deref(), Some("xion1abc123"));
                assert_eq!(args.client_name.as_deref(), Some("My App"));
                assert_eq!(args.owner.as_deref(), Some("user_123"));
                assert_eq!(args.managers.as_deref().map(|m| m.len()), Some(2));
                assert_eq!(args.auth_method.as_deref(), Some("client_secret_basic"));
                assert!(args.show_secret);
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_create_with_json_input() {
        let cmd = parse_oauth2_client(&["create", "--json-input", "request.json"]);
        match cmd {
            OAuth2ClientCommands::Create(args) => {
                assert!(args.json_input.is_some());
                assert_eq!(
                    args.json_input.as_ref().unwrap().to_str(),
                    Some("request.json")
                );
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_get_positional() {
        let cmd = parse_oauth2_client(&["get", "client_abc123"]);
        match cmd {
            OAuth2ClientCommands::Get(args) => {
                assert_eq!(args.client_id, "client_abc123");
            }
            _ => panic!("Expected Get command"),
        }
    }

    #[test]
    fn test_update_positional_and_flags() {
        let cmd = parse_oauth2_client(&[
            "update",
            "client_abc123",
            "--client-name",
            "Updated App",
            "--contacts",
            "new@example.com",
        ]);
        match cmd {
            OAuth2ClientCommands::Update(args) => {
                assert_eq!(args.client_id, "client_abc123");
                assert_eq!(args.client_name.as_deref(), Some("Updated App"));
                assert_eq!(args.contacts.as_deref().map(|c| c.len()), Some(1));
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_delete_positional() {
        let cmd = parse_oauth2_client(&["delete", "client_abc123"]);
        match cmd {
            OAuth2ClientCommands::Delete(args) => {
                assert_eq!(args.client_id, "client_abc123");
                assert!(!args.force);
            }
            _ => panic!("Expected Delete command"),
        }
    }

    #[test]
    fn test_delete_with_force() {
        let cmd = parse_oauth2_client(&["delete", "client_abc123", "--force"]);
        match cmd {
            OAuth2ClientCommands::Delete(args) => {
                assert_eq!(args.client_id, "client_abc123");
                assert!(args.force);
            }
            _ => panic!("Expected Delete command"),
        }
    }

    #[test]
    fn test_extension_get() {
        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            command: OAuth2ClientCommands,
        }
        let cli = TestCli::parse_from(["xion-toolkit", "extension", "get", "client_abc123"]);
        match cli.command {
            OAuth2ClientCommands::Extension(ExtensionCommands::Get { client_id }) => {
                assert_eq!(client_id, "client_abc123");
            }
            _ => panic!("Expected Extension Get command"),
        }
    }

    #[test]
    fn test_extension_update() {
        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            command: OAuth2ClientCommands,
        }
        let cli = TestCli::parse_from([
            "xion-toolkit",
            "extension",
            "update",
            "client_abc123",
            "--managers",
            "user_a,user_b",
        ]);
        match cli.command {
            OAuth2ClientCommands::Extension(ExtensionCommands::Update {
                client_id,
                managers,
            }) => {
                assert_eq!(client_id, "client_abc123");
                assert_eq!(managers.as_deref().map(|m| m.len()), Some(2));
            }
            _ => panic!("Expected Extension Update command"),
        }
    }

    #[test]
    fn test_managers_add() {
        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            command: OAuth2ClientCommands,
        }
        let cli = TestCli::parse_from([
            "xion-toolkit",
            "managers",
            "add",
            "client_abc123",
            "--manager-id",
            "user_456",
        ]);
        match cli.command {
            OAuth2ClientCommands::Managers(ManagersCommands::Add {
                client_id,
                manager_id,
            }) => {
                assert_eq!(client_id, "client_abc123");
                assert_eq!(manager_id, "user_456");
            }
            _ => panic!("Expected Managers Add command"),
        }
    }

    #[test]
    fn test_managers_remove() {
        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            command: OAuth2ClientCommands,
        }
        let cli = TestCli::parse_from([
            "xion-toolkit",
            "managers",
            "remove",
            "client_abc123",
            "--manager-id",
            "user_456",
        ]);
        match cli.command {
            OAuth2ClientCommands::Managers(ManagersCommands::Remove {
                client_id,
                manager_id,
            }) => {
                assert_eq!(client_id, "client_abc123");
                assert_eq!(manager_id, "user_456");
            }
            _ => panic!("Expected Managers Remove command"),
        }
    }

    #[test]
    fn test_transfer_ownership() {
        let cmd = parse_oauth2_client(&[
            "transfer-ownership",
            "client_abc123",
            "--new-owner",
            "user_789",
        ]);
        match cmd {
            OAuth2ClientCommands::TransferOwnership(args) => {
                assert_eq!(args.client_id, "client_abc123");
                assert_eq!(args.new_owner, "user_789");
                assert!(!args.force);
            }
            _ => panic!("Expected TransferOwnership command"),
        }
    }

    #[test]
    fn test_transfer_ownership_with_force() {
        let cmd = parse_oauth2_client(&[
            "transfer-ownership",
            "client_abc123",
            "--new-owner",
            "user_789",
            "--force",
        ]);
        match cmd {
            OAuth2ClientCommands::TransferOwnership(args) => {
                assert_eq!(args.client_id, "client_abc123");
                assert_eq!(args.new_owner, "user_789");
                assert!(args.force);
            }
            _ => panic!("Expected TransferOwnership command"),
        }
    }

    // ========================================================================
    // ConfirmationRequired guard tests
    // ========================================================================

    #[test]
    fn test_delete_without_force_returns_confirmation_error() {
        let err = crate::shared::error::OAuthClientError::ConfirmationRequired {
            message: "Destructive operation cancelled.".to_string(),
        };
        let xion_err: crate::shared::error::XionError = err.into();
        assert_eq!(
            xion_err.code(),
            crate::shared::error::XionErrorCode::EOAUTHCLIENT019
        );
        let display = format!("{}", xion_err);
        assert!(display.contains("Destructive operation cancelled"));
    }

    #[test]
    fn test_transfer_ownership_without_force_returns_confirmation_error() {
        let err = crate::shared::error::OAuthClientError::ConfirmationRequired {
            message: "Re-run with --force to confirm ownership transfer of client 'c1' to 'u1'."
                .to_string(),
        };
        let xion_err: crate::shared::error::XionError = err.into();
        assert_eq!(
            xion_err.code(),
            crate::shared::error::XionErrorCode::EOAUTHCLIENT019
        );
        let hint = xion_err.hint();
        assert!(hint.contains("--force"));
    }

    #[test]
    fn test_confirmation_required_exit_code() {
        let err = crate::shared::error::OAuthClientError::ConfirmationRequired {
            message: "test".to_string(),
        };
        let xion_err: crate::shared::error::XionError = err.into();
        assert_eq!(xion_err.code().exit_code(), 178);
    }
}
