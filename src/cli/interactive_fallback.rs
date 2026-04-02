//! Interactive fallback for missing CLI arguments.
//!
//! When `Cli::try_parse()` fails due to missing required arguments,
//! this module prompts the user for each missing value and re-parses
//! using the supplemented argument vector.
//!
//! # How it works
//!
//! 1. `main.rs` calls `Cli::try_parse()` instead of `Cli::parse()`
//! 2. On `MissingRequiredArgument` error, we check for TTY + no `--no-interactive`
//! 3. We parse clap's rendered error to extract which args are missing
//! 4. For each missing arg, we prompt the user with a type-appropriate dialog
//! 5. We build a supplemented `Vec<String>` and call `Cli::parse_from()`
//!
//! # When adding new CLI arguments
//!
//! 1. Add the arg name to `determine_prompt_type()` if it needs a non-text prompt
//! 2. Add a description to `format_arg_description()` if the default is unclear
//! 3. Add underscore AND hyphen variants (clap uses underscores in errors)
//! 4. Add test cases for both `determine_prompt_type` and `format_arg_description`

use clap::error::ErrorKind;

use crate::cli::interactive::{
    prompt_address, prompt_amount, prompt_existing_path, prompt_hash, prompt_text, prompt_u64,
    PromptError,
};

/// A description of a missing required argument extracted from a clap error.
#[derive(Debug, Clone)]
pub struct MissingArg {
    /// The name of the argument (e.g., "ADDRESS" or "code-id").
    pub name: String,
    /// The long flag associated with this argument, if any (e.g., "--address").
    pub flag: Option<String>,
    /// A human-readable description for the prompt.
    pub description: String,
    /// The type of interactive prompt to use.
    pub prompt_type: PromptType,
}

/// The kind of interactive prompt to use for a missing argument.
#[derive(Debug, Clone, PartialEq)]
pub enum PromptType {
    /// Free-form text.
    Text,
    /// Blockchain address (xion1...).
    Address,
    /// Token amount string.
    Amount,
    /// Path to a file that must exist.
    ExistingPath,
    /// Unsigned 64-bit integer.
    U64,
    /// Hex hash string.
    Hash,
}

/// Try to parse the CLI. On failure with missing args + TTY, prompt and return supplemented args.
///
/// Returns `Some(supplemented_args)` if interactive prompts filled in values,
/// or `None` if not interactive, not a missing-arg error, or no args could be extracted.
pub fn try_interactive_parse(args: &[String]) -> Option<Vec<String>> {
    use clap::CommandFactory;

    // Check if --no-interactive was explicitly passed
    if args.iter().any(|a| a == "--no-interactive") {
        return None;
    }

    // Check TTY
    if !crate::cli::interactive::is_tty() {
        return None;
    }

    // Try parsing
    let mut cmd = crate::cli::Cli::command();
    match cmd.try_get_matches_from_mut(args.iter().map(String::as_str)) {
        Ok(_) => None,
        Err(clap_err) => {
            if clap_err.kind() != ErrorKind::MissingRequiredArgument {
                return None;
            }

            let missing = extract_missing_args(&clap_err);
            if missing.is_empty() {
                return None;
            }

            eprintln!("\n\u{26a0}\u{fe0f}  Missing required arguments. Let's fill them in:\n");

            let mut supplemented_args: Vec<String> = args.to_vec();
            for arg in &missing {
                match prompt_for_arg(arg) {
                    Ok(value) => {
                        if let Some(ref flag) = arg.flag {
                            supplemented_args.push(flag.clone());
                        }
                        supplemented_args.push(value);
                    }
                    Err(_) => {
                        eprintln!("\nCancelled. Exiting.");
                        std::process::exit(1);
                    }
                }
            }

            Some(supplemented_args)
        }
    }
}

/// Extract missing required argument names from a clap error.
///
/// Clap renders errors with a section listing missing args between
/// "required arguments were not provided" and the next empty/Usage line.
/// We only parse that section to avoid picking up args from the usage template.
fn extract_missing_args(err: &clap::error::Error) -> Vec<MissingArg> {
    let rendered = err.to_string();

    let mut args = Vec::new();
    let mut in_missing_section = false;

    for line in rendered.lines() {
        let trimmed = line.trim();

        // Detect the start of the "missing required arguments" section
        if trimmed.contains("required arguments were not provided") {
            in_missing_section = true;
            continue;
        }

        if !in_missing_section {
            continue;
        }

        // End of missing section: empty line or "Usage:" line
        if trimmed.is_empty() || trimmed.starts_with("Usage:") {
            in_missing_section = false;
            continue;
        }

        // Try to match `  --flag-name <ARG_NAME>` pattern
        if let Some(caps) = parse_flag_arg(trimmed) {
            args.push(caps);
            continue;
        }

        // Try to match `<ARG_NAME>` positional pattern
        if let Some(caps) = parse_positional_arg(trimmed) {
            args.push(caps);
        }
    }

    args
}

/// Parse a line like `  --address <ADDRESS>` into a `MissingArg`.
fn parse_flag_arg(line: &str) -> Option<MissingArg> {
    // Pattern: optional whitespace, --flag-name <ARG_NAME>
    let line = line.trim();

    // Skip if it doesn't look like a flag entry
    if !line.starts_with("--") {
        return None;
    }

    // Extract the flag and argument name: --flag-name <ARG_NAME>
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() < 2 {
        return None;
    }

    let flag = parts[0].trim();
    let arg_part = parts[1].trim();

    // Extract <ARG_NAME> from the remainder
    let arg_name = extract_angle_bracket_name(arg_part)?;

    let name = arg_name.to_lowercase();
    let description = format_arg_description(&name);
    let prompt_type = determine_prompt_type(&name);

    Some(MissingArg {
        name,
        flag: Some(flag.to_string()),
        description,
        prompt_type,
    })
}

/// Parse a line like `  <CODE_ID>` into a `MissingArg`.
fn parse_positional_arg(line: &str) -> Option<MissingArg> {
    let line = line.trim();

    // Skip lines that start with -- (handled by parse_flag_arg)
    if line.starts_with("--") {
        return None;
    }

    // Try to extract <ARG_NAME>
    let arg_name = extract_angle_bracket_name(line)?;

    let name = arg_name.to_lowercase();
    let description = format_arg_description(&name);
    let prompt_type = determine_prompt_type(&name);

    Some(MissingArg {
        name,
        flag: None,
        description,
        prompt_type,
    })
}

/// Extract the name inside angle brackets from a string.
/// e.g., "<ADDRESS>" -> "ADDRESS", "--flag <NAME>" -> "NAME"
fn extract_angle_bracket_name(s: &str) -> Option<&str> {
    let start = s.find('<')?;
    let end = s.find('>')?;
    if end <= start + 1 {
        return None;
    }
    Some(&s[start + 1..end])
}

/// Determine the appropriate prompt type based on the argument name.
pub fn determine_prompt_type(arg_name: &str) -> PromptType {
    match arg_name {
        // Blockchain addresses
        "address" | "contract" | "admin" | "grantee" | "owner" | "new-owner" | "new_admin"
        | "new-admin" | "new_owner" | "receiver" => PromptType::Address,

        // Amounts
        "amount" | "spend-limit" | "spend_limit" => PromptType::Amount,

        // Hash strings
        "hash" => PromptType::Hash,

        // File paths that must exist
        "file" | "path" | "from-file" | "from_file" | "config" | "fee-config" | "fee_config"
        | "grant-config" | "grant_config" | "msg" | "tokens-file" | "tokens_file" | "preset" => {
            PromptType::ExistingPath
        }

        // Unsigned integers
        "code-id" | "code_id" | "limit" => PromptType::U64,

        // UUID-style identifiers (not blockchain addresses)
        "client_id" | "client-id" | "manager-id" | "manager_id" => PromptType::Text,

        // Everything else is free-form text
        _ => PromptType::Text,
    }
}

/// Generate a human-readable description for a missing argument prompt.
pub fn format_arg_description(arg_name: &str) -> String {
    match arg_name {
        // Addresses
        "address" | "contract" => "Contract address (xion1...)".to_string(),
        "admin" => "Admin address (xion1...)".to_string(),
        "grantee" => "Grantee address (xion1...)".to_string(),
        "owner" => "Owner address (xion1...)".to_string(),
        "new-owner" | "new_owner" => "New owner address (xion1...)".to_string(),
        "new-admin" | "new_admin" => "New admin address (xion1...)".to_string(),
        "receiver" => "Receiver address (xion1...)".to_string(),

        // UUID-style identifiers
        "client_id" | "client-id" => "OAuth2 client ID".to_string(),
        "manager-id" | "manager_id" => "Manager user ID".to_string(),

        // Amounts
        "amount" => "Amount (e.g., 1000000uxion)".to_string(),
        "spend-limit" | "spend_limit" => "Spend limit (e.g., 1000000uxion)".to_string(),

        // Hash
        "hash" => "Transaction hash".to_string(),

        // Paths
        "file" | "path" => "File path".to_string(),
        "from-file" | "from_file" => "Input file path".to_string(),
        "config" => "Config file path".to_string(),
        "fee-config" | "fee_config" => "Fee config file path".to_string(),
        "grant-config" | "grant_config" => "Grant config file path".to_string(),
        "msg" => "Message file path".to_string(),
        "tokens-file" | "tokens_file" => "Tokens file path".to_string(),
        "preset" => "Preset name".to_string(),

        // Integers
        "code-id" | "code_id" => "Code ID (unsigned integer)".to_string(),
        "limit" => "Limit (unsigned integer)".to_string(),

        // Token / contract identifiers
        "token-id" | "token_id" => "Token ID".to_string(),
        "label" => "Contract label".to_string(),
        "salt" => "Salt for predictable address (hex-encoded)".to_string(),

        // OAuth2 URLs
        "type-url" | "type_url" => "Authorization type URL".to_string(),
        "redirect-url" | "redirect_url" => "Redirect URL".to_string(),
        "icon-url" | "icon_url" => "Icon URL".to_string(),

        // Descriptions / config
        "description" => "Description".to_string(),
        "key" => "Configuration key (e.g., network, version)".to_string(),
        "network" => "Network name (testnet or mainnet)".to_string(),

        // Fallback
        _ => format!("{} value", arg_name),
    }
}

/// Prompt the user for a single missing argument value.
fn prompt_for_arg(arg: &MissingArg) -> Result<String, PromptError> {
    let label = format!("  {}", arg.description);

    match arg.prompt_type {
        PromptType::Text => prompt_text(&label),
        PromptType::Address => prompt_address(&label),
        PromptType::Amount => prompt_amount(&label),
        PromptType::ExistingPath => {
            prompt_existing_path(&label).map(|v| v.to_string_lossy().to_string())
        }
        PromptType::U64 => prompt_u64(&label).map(|v| v.to_string()),
        PromptType::Hash => prompt_hash(&label),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_prompt_type_address() {
        assert_eq!(determine_prompt_type("address"), PromptType::Address);
        assert_eq!(determine_prompt_type("contract"), PromptType::Address);
        assert_eq!(determine_prompt_type("admin"), PromptType::Address);
        assert_eq!(determine_prompt_type("grantee"), PromptType::Address);
        assert_eq!(determine_prompt_type("owner"), PromptType::Address);
        assert_eq!(determine_prompt_type("new-owner"), PromptType::Address);
        assert_eq!(determine_prompt_type("new-admin"), PromptType::Address);
        assert_eq!(determine_prompt_type("receiver"), PromptType::Address);
    }

    #[test]
    fn test_determine_prompt_type_client_id_is_text() {
        assert_eq!(determine_prompt_type("client_id"), PromptType::Text);
        assert_eq!(determine_prompt_type("client-id"), PromptType::Text);
        assert_eq!(determine_prompt_type("manager-id"), PromptType::Text);
        assert_eq!(determine_prompt_type("manager_id"), PromptType::Text);
    }

    #[test]
    fn test_determine_prompt_type_amount() {
        assert_eq!(determine_prompt_type("amount"), PromptType::Amount);
        assert_eq!(determine_prompt_type("spend-limit"), PromptType::Amount);
    }

    #[test]
    fn test_determine_prompt_type_hash() {
        assert_eq!(determine_prompt_type("hash"), PromptType::Hash);
    }

    #[test]
    fn test_determine_prompt_type_existing_path() {
        assert_eq!(determine_prompt_type("file"), PromptType::ExistingPath);
        assert_eq!(determine_prompt_type("path"), PromptType::ExistingPath);
        assert_eq!(determine_prompt_type("from-file"), PromptType::ExistingPath);
        assert_eq!(determine_prompt_type("config"), PromptType::ExistingPath);
        assert_eq!(
            determine_prompt_type("fee-config"),
            PromptType::ExistingPath
        );
        assert_eq!(determine_prompt_type("msg"), PromptType::ExistingPath);
        assert_eq!(
            determine_prompt_type("tokens-file"),
            PromptType::ExistingPath
        );
    }

    #[test]
    fn test_determine_prompt_type_u64() {
        assert_eq!(determine_prompt_type("code-id"), PromptType::U64);
        assert_eq!(determine_prompt_type("code_id"), PromptType::U64);
        assert_eq!(determine_prompt_type("limit"), PromptType::U64);
    }

    #[test]
    fn test_determine_prompt_type_text_fallback() {
        assert_eq!(determine_prompt_type("name"), PromptType::Text);
        assert_eq!(determine_prompt_type("description"), PromptType::Text);
        assert_eq!(determine_prompt_type("unknown-arg"), PromptType::Text);
    }

    #[test]
    fn test_determine_prompt_type_underscore_variants() {
        // Clap uses underscores in error messages
        assert_eq!(
            determine_prompt_type("grant_config"),
            PromptType::ExistingPath
        );
        assert_eq!(
            determine_prompt_type("fee_config"),
            PromptType::ExistingPath
        );
        assert_eq!(determine_prompt_type("from_file"), PromptType::ExistingPath);
        assert_eq!(
            determine_prompt_type("tokens_file"),
            PromptType::ExistingPath
        );
        assert_eq!(determine_prompt_type("type_url"), PromptType::Text);
        assert_eq!(determine_prompt_type("redirect_url"), PromptType::Text);
        assert_eq!(determine_prompt_type("icon_url"), PromptType::Text);
        assert_eq!(determine_prompt_type("token_id"), PromptType::Text);
        assert_eq!(determine_prompt_type("new_admin"), PromptType::Address);
        assert_eq!(determine_prompt_type("new_owner"), PromptType::Address);
        assert_eq!(determine_prompt_type("code_id"), PromptType::U64);
        assert_eq!(determine_prompt_type("spend_limit"), PromptType::Amount);
        assert_eq!(determine_prompt_type("client_id"), PromptType::Text);
        assert_eq!(determine_prompt_type("manager_id"), PromptType::Text);
    }

    #[test]
    fn test_format_arg_description_address() {
        let desc = format_arg_description("address");
        assert!(desc.contains("address"));
        assert!(desc.contains("xion1"));

        let desc = format_arg_description("admin");
        assert!(desc.contains("Admin"));
        assert!(desc.contains("xion1"));
    }

    #[test]
    fn test_format_arg_description_amount() {
        let desc = format_arg_description("amount");
        assert!(desc.contains("Amount"));
        assert!(desc.contains("uxion"));
    }

    #[test]
    fn test_format_arg_description_hash() {
        let desc = format_arg_description("hash");
        assert!(desc.contains("hash"));
        assert!(desc.contains("Transaction"));
    }

    #[test]
    fn test_format_arg_description_path() {
        let desc = format_arg_description("file");
        assert!(desc.contains("File path"));

        let desc = format_arg_description("config");
        assert!(desc.contains("Config file path"));
    }

    #[test]
    fn test_format_arg_description_u64() {
        let desc = format_arg_description("code-id");
        assert!(desc.contains("Code ID"));

        let desc = format_arg_description("limit");
        assert!(desc.contains("Limit"));
    }

    #[test]
    fn test_format_arg_description_fallback() {
        let desc = format_arg_description("some-unknown-arg");
        assert_eq!(desc, "some-unknown-arg value");
    }

    #[test]
    fn test_format_arg_description_client_id() {
        let desc = format_arg_description("client_id");
        assert!(desc.contains("client ID") || desc.contains("Client ID"));

        let desc = format_arg_description("manager-id");
        assert!(desc.contains("Manager"));
    }

    #[test]
    fn test_format_arg_description_oauth2_and_treasury() {
        // OAuth2 client ID (both forms)
        assert!(format_arg_description("client_id").contains("OAuth2"));
        assert!(format_arg_description("client-id").contains("OAuth2"));

        // Manager ID
        assert!(format_arg_description("manager-id").contains("Manager"));

        // URLs
        assert!(format_arg_description("type-url").contains("Authorization"));
        assert!(format_arg_description("redirect-url").contains("Redirect"));
        assert!(format_arg_description("icon-url").contains("Icon"));

        // Token and contract identifiers
        assert!(format_arg_description("token-id").contains("Token ID"));
        assert!(format_arg_description("label").contains("Contract label"));
        assert!(format_arg_description("salt").contains("hex"));

        // Config values
        assert!(format_arg_description("key").contains("Configuration key"));
        assert!(format_arg_description("network").contains("testnet"));
        assert!(format_arg_description("description").contains("Description"));
    }

    #[test]
    fn test_try_interactive_parse_returns_none_with_no_interactive_flag() {
        let args = vec![
            "xion-toolkit".to_string(),
            "--no-interactive".to_string(),
            "treasury".to_string(),
            "create".to_string(),
        ];
        assert!(try_interactive_parse(&args).is_none());
    }

    #[test]
    fn test_extract_angle_bracket_name() {
        assert_eq!(extract_angle_bracket_name("<ADDRESS>"), Some("ADDRESS"));
        assert_eq!(extract_angle_bracket_name("--flag <NAME>"), Some("NAME"));
        assert_eq!(extract_angle_bracket_name("no brackets here"), None);
        assert_eq!(extract_angle_bracket_name("<>"), None);
        assert_eq!(extract_angle_bracket_name(">bad<"), None);
    }

    #[test]
    fn test_parse_flag_arg() {
        let result = parse_flag_arg("  --address <ADDRESS>");
        assert!(result.is_some());
        let arg = result.unwrap();
        assert_eq!(arg.name, "address");
        assert_eq!(arg.flag, Some("--address".to_string()));
        assert_eq!(arg.prompt_type, PromptType::Address);

        // Non-flag line returns None
        assert!(parse_flag_arg("<ADDRESS>").is_none());
        assert!(parse_flag_arg("").is_none());
    }

    #[test]
    fn test_parse_positional_arg() {
        let result = parse_positional_arg("  <CODE_ID>");
        assert!(result.is_some());
        let arg = result.unwrap();
        assert_eq!(arg.name, "code_id");
        assert!(arg.flag.is_none());
        assert_eq!(arg.prompt_type, PromptType::U64);

        // Flag line returns None
        assert!(parse_positional_arg("--address <ADDRESS>").is_none());
    }

    #[test]
    fn test_extract_missing_args_no_duplicates_from_usage_line() {
        // Simulate a clap error output for `treasury fund` missing ADDRESS and AMOUNT.
        // The Usage line also contains <ADDRESS> <AMOUNT> — we must NOT pick those up.
        use clap::error::ErrorKind;
        use clap::CommandFactory;

        let mut cmd = crate::cli::Cli::command();
        let result = cmd.try_get_matches_from_mut(["xion-toolkit", "treasury", "fund"]);
        match result {
            Ok(_) => panic!("expected error"),
            Err(err) => {
                assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
                let missing = extract_missing_args(&err);
                // Should be exactly 2: ADDRESS and AMOUNT (not 4 from the Usage line)
                assert_eq!(
                    missing.len(),
                    2,
                    "Expected 2 missing args, got {}: {:?}",
                    missing.len(),
                    missing.iter().map(|a| a.name.as_str()).collect::<Vec<_>>()
                );
                let names: Vec<&str> = missing.iter().map(|a| a.name.as_str()).collect();
                assert!(
                    names.contains(&"address"),
                    "Expected 'address' in {:?}",
                    names
                );
                assert!(
                    names.contains(&"amount"),
                    "Expected 'amount' in {:?}",
                    names
                );
            }
        }
    }
}
