//! Interactive fallback for missing CLI arguments.
//!
//! When `Cli::try_parse()` fails due to missing required arguments,
//! this module prompts the user for each missing value and re-parses
//! using the supplemented argument vector.

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
    /// One of a fixed set of values.
    Enum(Vec<String>),
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
/// Clap renders errors with lines like:
///   `  --address <ADDRESS>` or
///   `  <CODE_ID>`
/// We parse the rendered string to pull out these entries.
fn extract_missing_args(err: &clap::error::Error) -> Vec<MissingArg> {
    let rendered = err.to_string();

    let mut args = Vec::new();
    for line in rendered.lines() {
        let trimmed = line.trim();

        // Try to match `  --flag-name <ARG_NAME>` pattern
        if let Some(caps) = parse_flag_arg(trimmed) {
            args.push(caps);
            continue;
        }

        // Try to match `<ARG_NAME>` or `ARG_NAME` positional pattern
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
        "address" | "contract" | "admin" | "grantee" | "owner" | "manager-id" | "new-owner"
        | "new-admin" | "receiver" => PromptType::Address,

        // Amounts
        "amount" | "spend-limit" => PromptType::Amount,

        // Hash strings
        "hash" => PromptType::Hash,

        // File paths that must exist
        "file" | "path" | "from-file" | "config" | "fee-config" | "msg" | "tokens-file" => {
            PromptType::ExistingPath
        }

        // Unsigned integers
        "code-id" | "code_id" | "limit" => PromptType::U64,

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
        "manager-id" => "Manager ID address (xion1...)".to_string(),
        "new-owner" => "New owner address (xion1...)".to_string(),
        "new-admin" => "New admin address (xion1...)".to_string(),
        "receiver" => "Receiver address (xion1...)".to_string(),

        // Amounts
        "amount" => "Amount (e.g., 1000000uxion)".to_string(),
        "spend-limit" => "Spend limit (e.g., 1000000uxion)".to_string(),

        // Hash
        "hash" => "Transaction hash".to_string(),

        // Paths
        "file" | "path" => "File path".to_string(),
        "from-file" => "Input file path".to_string(),
        "config" => "Config file path".to_string(),
        "fee-config" => "Fee config file path".to_string(),
        "msg" => "Message file path".to_string(),
        "tokens-file" => "Tokens file path".to_string(),

        // Integers
        "code-id" | "code_id" => "Code ID (unsigned integer)".to_string(),
        "limit" => "Limit (unsigned integer)".to_string(),

        // Fallback
        _ => format!("{} value", arg_name),
    }
}

/// Prompt the user for a single missing argument value.
fn prompt_for_arg(arg: &MissingArg) -> Result<String, PromptError> {
    let label = format!("  {}", arg.description);

    match arg.prompt_type {
        PromptType::Text => prompt_text(&label, &format!("Enter {}", arg.name)),
        PromptType::Address => prompt_address(&label),
        PromptType::Amount => prompt_amount(&label),
        PromptType::ExistingPath => {
            prompt_existing_path(&label).map(|v| v.to_string_lossy().to_string())
        }
        PromptType::U64 => prompt_u64(&label, "Enter a number").map(|v| v.to_string()),
        PromptType::Hash => prompt_hash(&label),
        PromptType::Enum(ref choices) => {
            use crate::cli::interactive::prompt_select;
            let idx = prompt_select(&label, choices)?;
            Ok(choices[idx].clone())
        }
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
        assert_eq!(determine_prompt_type("manager-id"), PromptType::Address);
        assert_eq!(determine_prompt_type("new-owner"), PromptType::Address);
        assert_eq!(determine_prompt_type("new-admin"), PromptType::Address);
        assert_eq!(determine_prompt_type("receiver"), PromptType::Address);
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
}
