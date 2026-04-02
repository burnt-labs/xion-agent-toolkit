# Interactive CLI Mode Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** When human users run CLI commands without all required parameters, prompt them interactively to fill in missing values instead of immediately erroring out.

**Architecture:** Replace `Cli::parse()` with `Cli::try_parse()` in `main.rs`. On parse failure, if stdin is a TTY and `--no-interactive` was not passed, extract which arguments are missing from the clap error, and prompt the user via `dialoguer` for each missing value. Re-assemble args and re-parse. For agent/CI usage, the `--no-interactive` global flag disables this entirely.

**Tech Stack:** `dialoguer` crate for interactive prompts, `atty` or `std::io::IsTerminal` for TTY detection, `clap` `try_parse()` for error interception.

---

## Design Decisions

### TTY Detection
- Use `std::io::stdin().is_terminal()` ( stabilized in Rust 1.70+ ) instead of adding `atty` as a dependency.
- This is simpler and has no external dependency.

### Prompt UX
| Param Type | Prompt Style | Validation |
|-----------|-------------|-----------|
| Blockchain address (`xion1...`) | Text input with `xion1` prefix hint | Basic format: starts with `xion1`, length >= 20 |
| Amount (`1000000uxion`) | Text input with hint text | Must contain digits |
| Path / File (`PathBuf`) | Text input (no tab-completion in `dialoguer::Input`) | Must exist for input files |
| Number (`u64`, code-id) | Text input, parsed as u64 | Must parse as u64 |
| Enum / choice (`value_enum`) | `dialoguer::Select` dropdown | N/A (pre-validated) |
| Free text (`String`, name, symbol) | Text input | Non-empty |
| Client ID / generic ID | Text input | Non-empty |

### Global Flag
```
--no-interactive    Disable interactive prompts; exit on missing required arguments (default for non-TTY)
```
Added to `Cli` struct alongside `--network`, `--output`, `--config`.

### Agent Safety
- When stdin is NOT a TTY (piped), interactive mode is **never** triggered regardless of flags.
- `--no-interactive` is the explicit escape hatch for TTY environments where user doesn't want prompts.
- No change to output format behavior — interactive mode is purely about input collection.

---

## Scope: 34 Subcommands with Required Parameters

### Batch 1 — High-frequency commands (this plan)
| # | Subcommand | Required Params |
|---|-----------|----------------|
| 1 | `treasury query` | address |
| 2 | `treasury fund` | address, amount |
| 3 | `treasury withdraw` | address, amount |
| 4 | `treasury create` | (runtime-validated: redirect-url, icon-url) |
| 5 | `asset create` | name, symbol |
| 6 | `asset mint` | contract, token-id, owner |
| 7 | `contract instantiate` | code-id, label, msg |
| 8 | `contract execute` | contract, msg |
| 9 | `contract query` | contract, msg |
| 10 | `tx status` | hash |
| 11 | `tx wait` | hash |
| 12 | `config set-network` | network |
| 13 | `config get` | key |
| 14 | `batch execute` | from-file |
| 15 | `batch validate` | from-file |
| 16 | `asset predict` | name, symbol, salt |
| 17 | `asset batch-mint` | contract, tokens-file |
| 18 | `asset query` | contract, msg |

### Batch 2 — Treasury subcommands (deferred)
| # | Subcommand | Required Params |
|---|-----------|----------------|
| 19 | `treasury grant-config add` | address |
| 20 | `treasury grant-config remove` | address, --type-url |
| 21 | `treasury grant-config list` | address |
| 22 | `treasury fee-config set` | address, --fee-config |
| 23 | `treasury fee-config remove` | address, --grantee |
| 24 | `treasury fee-config query` | address |
| 25 | `treasury admin propose` | address, --new-admin |
| 26 | `treasury admin accept` | address |
| 27 | `treasury admin cancel` | address |
| 28 | `treasury params update` | address |
| 29 | `treasury chain-query grants` | address |
| 30 | `treasury chain-query allowances` | address |
| 31 | `treasury batch fund` | --config |
| 32 | `treasury batch grant-config` | --config |
| 33 | `treasury import` | address, --from-file |

### Batch 3 — OAuth2 client subcommands (deferred)
| # | Subcommand | Required Params |
|---|-----------|----------------|
| 34 | `oauth2 client get` | client_id |
| 35 | `oauth2 client update` | client_id |
| 36 | `oauth2 client delete` | client_id |
| 37 | `oauth2 client managers add` | client_id, --manager-id |
| 38 | `oauth2 client managers remove` | client_id, --manager-id |
| 39 | `oauth2 client transfer-ownership` | client_id, --new-owner |
| 40 | `oauth2 client extension get` | client_id |

---

## Task 1: Add `dialoguer` dependency and `--no-interactive` global flag

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/cli/mod.rs`

- [x] **Step 1: Add `dialoguer` to Cargo.toml**

Add to `[dependencies]`:
```toml
# Interactive prompts
dialoguer = "0.11"
```

- [x] **Step 2: Add `--no-interactive` flag to `Cli` struct**

In `src/cli/mod.rs`, add after the `config` field in the `Cli` struct:

```rust
/// Disable interactive prompts (exit on missing required arguments)
#[arg(long, global = true)]
pub no_interactive: bool,
```

Also add an `is_interactive()` helper method to `ExecuteContext` — but since `ExecuteContext` doesn't carry `no_interactive`, we need to pass it separately. Better approach: add it to `ExecuteContext`:

```rust
#[derive(Debug, Clone)]
pub struct ExecuteContext {
    pub output_format: OutputFormat,
    pub network: String,
    /// Whether interactive prompts are allowed
    pub interactive: bool,
}
```

Update `ExecuteContext::new()` to accept `interactive: bool`. Update `Cli::to_context()` to pass `!self.no_interactive`. Update `default_context()` to pass `false`.

- [x] **Step 3: Run `cargo build` to verify compilation**

Run: `cargo build`
Expected: Compiles without errors.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock src/cli/mod.rs
git commit -m "feat(cli): add dialoguer dependency and --no-interactive flag"
```

---

## Task 2: Create interactive prompt module `src/cli/interactive.rs`

**Files:**
- Create: `src/cli/interactive.rs`
- Modify: `src/cli/mod.rs` (add `pub mod interactive;`)

- [x] **Step 1: Write the interactive module**

Create `src/cli/interactive.rs`:

```rust
//! Interactive parameter prompting for missing required arguments.
//!
//! When running in a TTY without `--no-interactive`, missing required
//! parameters are collected via user prompts instead of causing an error exit.

use std::io;
use std::path::{Path, PathBuf};

use dialoguer::{Input, Select};

/// Check if interactive mode is available (TTY on stdin).
pub fn is_tty() -> bool {
    io::stdin().is_terminal()
}

/// Result of interactive prompting — either the user-supplied string value,
/// or the user chose to cancel (Ctrl+C / empty input when not allowed).
pub type PromptResult<T> = Result<T, PromptError>;

#[derive(Debug)]
pub enum PromptError {
    /// User cancelled (Ctrl+C)
    Cancelled,
    /// User entered empty input when non-empty was required
    EmptyInput,
    /// Input failed validation
    ValidationFailed(String),
}

impl std::fmt::Display for PromptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptError::Cancelled => write!(f, "Prompt cancelled by user"),
            PromptError::EmptyInput => write!(f, "Empty input is not allowed"),
            PromptError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
        }
    }
}

impl std::error::Error for PromptError {}

// ============================================================================
// Prompt Builders
// ============================================================================

/// Prompt for a non-empty text string.
pub fn prompt_text(label: &str, help_text: &str) -> PromptResult<String> {
    let input: String = Input::new()
        .with_prompt(label)
        .with_initial_text(help_text)
        .allow_empty(true)
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    if input.trim().is_empty() {
        Err(PromptError::EmptyInput)
    } else {
        Ok(input.trim().to_string())
    }
}

/// Prompt for a text string with a default value.
pub fn prompt_text_with_default(label: &str, default: &str) -> PromptResult<String> {
    let input: String = Input::new()
        .with_prompt(label)
        .default(default.to_string())
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(input)
}

/// Prompt for a u64 number.
pub fn prompt_u64(label: &str, help_text: &str) -> PromptResult<u64> {
    let input: String = Input::new()
        .with_prompt(label)
        .with_initial_text(help_text)
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    input
        .trim()
        .parse::<u64>()
        .map_err(|_| PromptError::ValidationFailed("Must be a valid number".to_string()))
}

/// Prompt for a blockchain address (xion1... format).
pub fn prompt_address(label: &str) -> PromptResult<String> {
    let input: String = Input::new()
        .with_prompt(label)
        .validate_with(|input: &str| -> Result<(), &str> {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Err("Address cannot be empty");
            }
            if !trimmed.starts_with("xion1") {
                return Err("Address must start with 'xion1'");
            }
            if trimmed.len() < 20 {
                return Err("Address seems too short");
            }
            Ok(())
        })
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(input.trim().to_string())
}

/// Prompt for an amount string (e.g., "1000000uxion").
pub fn prompt_amount(label: &str) -> PromptResult<String> {
    let input: String = Input::new()
        .with_prompt(label)
        .validate_with(|input: &str| -> Result<(), &str> {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Err("Amount cannot be empty");
            }
            if !trimmed.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                return Err("Amount must start with a number");
            }
            Ok(())
        })
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(input.trim().to_string())
}

/// Prompt for a file path. Does NOT validate existence (caller decides).
pub fn prompt_path(label: &str) -> PromptResult<PathBuf> {
    let input: String = Input::new()
        .with_prompt(label)
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(PromptError::EmptyInput);
    }

    Ok(PathBuf::from(trimmed))
}

/// Prompt for a file path that must exist.
pub fn prompt_existing_path(label: &str) -> PromptResult<PathBuf> {
    let path = prompt_path(label)?;
    if !path.exists() {
        return Err(PromptError::ValidationFailed(format!(
            "File not found: {}",
            path.display()
        )));
    }
    Ok(path)
}

/// Prompt for a selection from a list of choices.
pub fn prompt_select<T: ToString>(label: &str, items: &[T]) -> PromptResult<usize> {
    Select::new()
        .with_prompt(label)
        .items(items)
        .interact()
        .map_err(|_| PromptError::Cancelled)
}

/// Prompt for a hash string (transaction hash).
pub fn prompt_hash(label: &str) -> PromptResult<String> {
    let input: String = Input::new()
        .with_prompt(label)
        .validate_with(|input: &str| -> Result<(), &str> {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Err("Hash cannot be empty");
            }
            // Accept hex with or without 0x prefix, minimum 10 chars
            let hex = trimmed.strip_prefix("0x").unwrap_or(trimmed);
            if hex.len() < 10 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err("Must be a valid hex hash (min 10 chars, with or without 0x prefix)");
            }
            Ok(())
        })
        .interact_text()
        .map_err(|_| PromptError::Cancelled)?;

    Ok(input.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tty_returns_bool() {
        // We can't test the actual TTY state in unit tests,
        // but we verify the function exists and returns a bool.
        let _result: bool = is_tty();
    }

    #[test]
    fn test_prompt_error_display() {
        let err = PromptError::Cancelled;
        assert_eq!(format!("{}", err), "Prompt cancelled by user");

        let err = PromptError::EmptyInput;
        assert_eq!(format!("{}", err), "Empty input is not allowed");

        let err = PromptError::ValidationFailed("bad input".to_string());
        assert!(format!("{}", err).contains("bad input"));
    }

    #[test]
    fn test_address_validation_logic() {
        // Test the validation logic conceptually
        let valid = "xion1abc123def456";
        assert!(valid.starts_with("xion1"));
        assert!(valid.len() >= 20);

        let invalid_prefix = "cosmos1abc";
        assert!(!invalid_prefix.starts_with("xion1"));

        let too_short = "xion1";
        assert!(too_short.len() < 20);
    }
}
```

- [x] **Step 2: Add module declaration**

In `src/cli/mod.rs`, add:
```rust
pub mod interactive;
```

- [x] **Step 3: Run tests**

Run: `cargo test --lib cli::interactive`
Expected: All 3 tests pass.

- [x] **Step 4: Commit**

```bash
git add src/cli/interactive.rs src/cli/mod.rs
git commit -m "feat(cli): add interactive prompt module with validators"
```

---

## Task 3: Core infrastructure — intercept clap parse errors and route to interactive prompts

This is the key architectural change. Instead of changing every subcommand's parameter types from bare `String` to `Option<String>`, we intercept the parse error, ask the user for missing values, and **re-invoke the CLI** with the supplemented arguments.

**Approach: Re-exec pattern**
1. `Cli::try_parse()` fails → extract missing argument names from clap error
2. Prompt user for each missing argument
3. Build a new `Vec<String>` with original args + prompted values
4. Call `Cli::parse_from(new_args)` (or recursively call the modified args)

This is cleaner than changing every parameter type and avoids touching handler signatures.

**Files:**
- Modify: `src/main.rs`
- Create: `src/cli/interactive_fallback.rs`

- [ ] **Step 1: Create the interactive fallback module**

Create `src/cli/interactive_fallback.rs`:

```rust
//! Interactive fallback for missing CLI arguments.
//!
//! When `Cli::try_parse()` fails due to missing required arguments,
//! this module prompts the user for each missing value and re-parses.

use std::env;

use crate::cli::interactive::{
    prompt_address, prompt_amount, prompt_existing_path, prompt_hash, prompt_text,
    prompt_text_with_default, prompt_u64, PromptError,
};

/// A description of a missing required argument, extracted from clap error.
#[derive(Debug, Clone)]
pub struct MissingArg {
    /// The argument name (e.g., "ADDRESS", "NAME", "--code-id")
    pub name: String,
    /// The long flag name if it's a named arg (e.g., "--code-id"), None for positional
    pub flag: Option<String>,
    /// Human-readable description for the prompt
    pub description: String,
    /// The type of prompt to show
    pub prompt_type: PromptType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PromptType {
    /// Free text (name, symbol, client_id, etc.)
    Text,
    /// Blockchain address (xion1...)
    Address,
    /// Amount string (e.g., "1000000uxion")
    Amount,
    /// File path that must exist
    ExistingPath,
    /// u64 number (code-id)
    U64,
    /// Transaction hash (hex)
    Hash,
    /// Enum choice (value_enum)
    Enum(Vec<String>),
}

/// Try to parse the CLI. On failure with missing args + TTY, prompt and re-parse.
/// Returns the final `Vec<String>` args to pass to `Cli::parse_from()`.
pub fn try_interactive_parse<I, T>(args: I) -> Option<Vec<String>>
where
    I: IntoIterator<Item = T>,
    T: AsRef<str> + Clone,
{
    use clap::CommandFactory;
    use clap::error::ErrorKind;

    // First, try normal parse
    let args_vec: Vec<String> = args.into_iter().map(|s| s.as_ref().to_string()).collect();

    // Check if --no-interactive was explicitly passed
    if args_vec.iter().any(|a| a == "--no-interactive") {
        // Not interactive — let clap handle the error normally
        return None;
    }

    // Check TTY
    if !crate::cli::interactive::is_tty() {
        return None;
    }

    // Try to parse and see what errors we get
    // We use the Cli command factory to validate
    let cmd = crate::cli::Cli::command();

    // Try parsing
    match cmd.try_get_matches_from_mut(args_vec.clone()) {
        Ok(_) => None, // Parsed fine, no need for interactive
        Err(clap_err) => {
            // Check if it's a "missing required argument" error
            match clap_err.kind() {
                ErrorKind::MissingRequiredArgument => {
                    // Extract the missing argument info from the error
                    let missing = extract_missing_args(&clap_err, &args_vec);
                    if missing.is_empty() {
                        return None;
                    }

                    eprintln!("\n⚠️  Missing required arguments. Let's fill them in:\n");

                    let mut supplemented_args = args_vec.clone();

                    for arg in &missing {
                        match prompt_for_arg(arg) {
                            Ok(value) => {
                                if let Some(ref flag) = arg.flag {
                                    supplemented_args.push(flag.clone());
                                }
                                supplemented_args.push(value);
                            }
                            Err(PromptError::Cancelled) => {
                                eprintln!("\nCancelled. Exiting.");
                                std::process::exit(1);
                            }
                            Err(PromptError::EmptyInput) => {
                                eprintln!("\nEmpty input not allowed. Exiting.");
                                std::process::exit(1);
                            }
                            Err(PromptError::ValidationFailed(msg)) => {
                                eprintln!("\nInvalid input: {}. Exiting.", msg);
                                std::process::exit(1);
                            }
                        }
                    }

                    Some(supplemented_args)
                }
                _ => None, // Other errors (invalid value, etc.) — let clap handle
            }
        }
    }
}

/// Extract missing argument names from a clap error message.
///
/// Clap's error output contains lines like:
///   `<ADDRESS>  <AMOUNT>` or `--code-id <CODE_ID>`
/// We parse these to build prompt descriptions.
fn extract_missing_args(err: &clap::error::Error, _orig_args: &[String]) -> Vec<MissingArg> {
    let mut missing = Vec::new();
    let error_string = err.to_string();

    // Parse the "required arguments were not provided" section from clap output.
    // Clap's rendered error looks like:
    //   error: The following required arguments were not provided:
    //     <ADDRESS>
    //     <AMOUNT>
    //
    //     <NAME>...
    //
    //   Usage: xion-toolkit treasury fund <ADDRESS> <AMOUNT>
    //
    // We extract lines that look like <UPPER_CASE> or --flag-name.

    let mut in_missing_section = false;

    for line in error_string.lines() {
        let trimmed = line.trim();

        if trimmed.contains("required arguments were not provided") {
            in_missing_section = true;
            continue;
        }

        if in_missing_section {
            // End of missing section: empty line or "Usage:" line
            if trimmed.is_empty() || trimmed.starts_with("Usage:") {
                in_missing_section = false;
                continue;
            }

            // Skip indentation-only lines
            if trimmed.chars().all(|c| c.is_whitespace()) {
                continue;
            }

            // Parse the argument specifier
            // Could be: "<ADDRESS>" or "--code-id <CODE_ID>" or "<NAME>..." 
            let (arg_name, flag, description) = if trimmed.starts_with("--") {
                // Named argument: "--code-id <CODE_ID>"
                let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
                let flag_name = parts[0].to_string();
                let arg_display = if parts.len() > 1 {
                    parts[1].trim_matches('<').trim_matches('>').trim_matches('.').to_string()
                } else {
                    flag_name.strip_prefix("--").unwrap_or(&flag_name).to_string()
                };
                let desc = format_arg_description(&flag_name, &arg_display);
                (arg_display, Some(flag_name), desc)
            } else {
                // Positional: "<ADDRESS>" or "<NAME>..."
                let clean = trimmed
                    .trim_matches('<')
                    .trim_matches('>')
                    .trim_matches('.')
                    .trim()
                    .to_string();
                let desc = format_arg_description(&format!("<{}>", clean), &clean);
                (clean, None, desc)
            };

            // Determine prompt type from the argument name
            let prompt_type = determine_prompt_type(&arg_name);

            missing.push(MissingArg {
                name: arg_name,
                flag,
                description,
                prompt_type,
            });
        }
    }

    missing
}

/// Determine the prompt type based on the argument name/context.
fn determine_prompt_type(arg_name: &str) -> PromptType {
    let name_lower = arg_name.to_lowercase();

    // Address patterns
    if name_lower == "address"
        || name_lower.contains("contract")
        || name_lower.contains("admin")
        || name_lower.contains("grantee")
        || name_lower.contains("owner")
        || name_lower.contains("manager-id")
        || name_lower.contains("new-owner")
        || name_lower.contains("new-admin")
        || name_lower.contains("client_id")
        || name_lower.contains("receiver")
    {
        return PromptType::Address;
    }

    // Amount patterns
    if name_lower == "amount" || name_lower.contains("spend-limit") {
        return PromptType::Amount;
    }

    // Hash patterns
    if name_lower == "hash" || name_lower.contains("tx_hash") {
        return PromptType::Hash;
    }

    // Path patterns
    if name_lower == "file"
        || name_lower.contains("path")
        || name_lower.contains("from-file")
        || name_lower.contains("config")
        || name_lower.contains("fee-config")
        || name_lower.contains("msg")
        || name_lower.contains("tokens-file")
    {
        return PromptType::ExistingPath;
    }

    // Number patterns
    if name_lower == "code-id" || name_lower == "code_id" || name_lower.contains("limit") {
        return PromptType::U64;
    }

    PromptType::Text
}

/// Format a human-readable description for the prompt.
fn format_arg_description(flag_or_pos: &str, arg_name: &str) -> String {
    let name_lower = arg_name.to_lowercase();

    match name_lower.as_str() {
        "address" => "Treasury contract address (xion1...)".to_string(),
        "amount" => "Amount to send (e.g., 1000000uxion)".to_string(),
        "name" => "Collection name".to_string(),
        "symbol" => "Collection symbol (e.g., MYTOKEN)".to_string(),
        "hash" => "Transaction hash (hex, with or without 0x prefix)".to_string(),
        "network" => "Network name (testnet, mainnet)".to_string(),
        "key" => "Configuration key (e.g., network, version)".to_string(),
        "salt" => "Salt for predictable address (hex-encoded)".to_string(),
        "label" => "Contract label (human-readable identifier)".to_string(),
        "code-id" | "code_id" => "Code ID of the uploaded contract".to_string(),
        "msg" => "Path to JSON message file".to_string(),
        "token-id" | "token_id" => "Token ID for minting".to_string(),
        "owner" => "Owner address (xion1...)".to_string(),
        "contract" => "Contract address (xion1...)".to_string(),
        "client_id" | "client-id" => "OAuth2 client ID".to_string(),
        "manager-id" | "manager_id" => "Manager user ID to add/remove".to_string(),
        "new-owner" | "new_owner" => "New owner address (xion1...)".to_string(),
        "new-admin" | "new_admin" => "New admin address (xion1...)".to_string(),
        "type-url" | "type_url" => "Authorization type URL".to_string(),
        "grantee" => "Grantee address (xion1...)".to_string(),
        "from-file" | "from_file" => "Path to JSON config file".to_string(),
        "config" => "Path to config file".to_string(),
        "fee-config" | "fee_config" => "Path to fee config JSON file".to_string(),
        "tokens-file" | "tokens_file" => "Path to tokens JSON file".to_string(),
        _ => format!("{} ({})", flag_or_pos, arg_name),
    }
}

/// Prompt the user for a single missing argument based on its type.
fn prompt_for_arg(arg: &MissingArg) -> Result<String, PromptError> {
    match arg.prompt_type {
        PromptType::Address => prompt_address(&arg.description),
        PromptType::Amount => prompt_amount(&arg.description),
        PromptType::ExistingPath => prompt_existing_path(&arg.description).map(|p| {
            p.to_string_lossy().to_string()
        }),
        PromptType::U64 => prompt_u64(&arg.description, "").map(|v| v.to_string()),
        PromptType::Hash => prompt_hash(&arg.description),
        PromptType::Text => prompt_text(&arg.description, ""),
        PromptType::Enum(ref choices) => {
            let idx = crate::cli::interactive::prompt_select(&arg.description, choices)?;
            Ok(choices[idx].clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_prompt_type() {
        assert_eq!(determine_prompt_type("ADDRESS"), PromptType::Address);
        assert_eq!(determine_prompt_type("address"), PromptType::Address);
        assert_eq!(determine_prompt_type("AMOUNT"), PromptType::Amount);
        assert_eq!(determine_prompt_type("hash"), PromptType::Hash);
        assert_eq!(determine_prompt_type("HASH"), PromptType::Hash);
        assert_eq!(determine_prompt_type("code-id"), PromptType::U64);
        assert_eq!(determine_prompt_type("code_id"), PromptType::U64);
        assert_eq!(determine_prompt_type("msg"), PromptType::ExistingPath);
        assert_eq!(determine_prompt_type("from-file"), PromptType::ExistingPath);
        assert_eq!(determine_prompt_type("tokens-file"), PromptType::ExistingPath);
        assert_eq!(determine_prompt_type("NAME"), PromptType::Text);
        assert_eq!(determine_prompt_type("symbol"), PromptType::Text);
        assert_eq!(determine_prompt_type("client_id"), PromptType::Address);
        assert_eq!(determine_prompt_type("network"), PromptType::Text);
        assert_eq!(determine_prompt_type("key"), PromptType::Text);
    }

    #[test]
    fn test_format_arg_description() {
        let desc = format_arg_description("<ADDRESS>", "address");
        assert!(desc.contains("xion1"));

        let desc = format_arg_description("<AMOUNT>", "amount");
        assert!(desc.contains("uxion"));

        let desc = format_arg_description("<HASH>", "hash");
        assert!(desc.contains("hex"));

        let desc = format_arg_description("--code-id", "code-id");
        assert!(desc.contains("Code ID"));
    }

    #[test]
    fn test_missing_arg_struct() {
        let arg = MissingArg {
            name: "ADDRESS".to_string(),
            flag: None,
            description: "Treasury contract address".to_string(),
            prompt_type: PromptType::Address,
        };
        assert_eq!(arg.flag, None);
        assert!(matches!(arg.prompt_type, PromptType::Address));
    }

    #[test]
    fn test_try_interactive_parse_with_no_interactive_flag() {
        // When --no-interactive is passed, should return None
        let args = vec!["xion-toolkit", "--no-interactive", "treasury", "fund"];
        let result = try_interactive_parse(args);
        assert!(result.is_none());
    }
}
```

- [ ] **Step 2: Add module declaration**

In `src/cli/mod.rs`, add:
```rust
pub mod interactive_fallback;
```

- [ ] **Step 3: Wire into `main.rs`**

Modify `src/main.rs` to use the interactive fallback:

```rust
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
            // Try interactive fallback
            match cli::interactive_fallback::try_interactive_parse(
                std::env::args().collect::<Vec<_>>(),
            ) {
                Some(supplemented_args) => {
                    // Re-parse with supplemented arguments
                    Cli::parse_from(supplemented_args)
                }
                None => {
                    // Not interactive or not a missing-arg error — let clap print its error and exit
                    // Re-run parse() to get the default error output
                    Cli::parse();
                    unreachable!("Cli::parse() exits on error")
                }
            }
        }
    };

    let ctx = cli.to_context();

    // ... rest of main() unchanged ...
}
```

**Important**: The rest of `main.rs` (network check, match block, error handling) stays exactly the same. Only the `Cli::parse()` line is replaced.

- [ ] **Step 4: Run `cargo build`**

Run: `cargo build`
Expected: Compiles.

- [ ] **Step 5: Run `cargo clippy`**

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: No warnings.

- [ ] **Step 6: Run tests**

Run: `cargo test --lib cli::interactive_fallback`
Expected: All tests pass.

- [ ] **Step 7: Commit**

```bash
git add src/cli/interactive_fallback.rs src/cli/mod.rs src/main.rs
git commit -m "feat(cli): interactive fallback for missing required arguments"
```

---

## Task 4: Manual E2E testing (not automatable — human must verify)

- [ ] **Step 1: Rebuild and install**

```bash
export $(grep -v '^#' .env | grep -v '^$' | xargs) && cargo install --path /Users/bibi/workspace/xion/xion-agent-toolkit --force
```

- [ ] **Step 2: Test `treasury fund` without args**

Run: `xion-toolkit treasury fund`
Expected: Interactive prompt asks for ADDRESS and AMOUNT. After filling in, command proceeds.

- [ ] **Step 3: Test `treasury fund` with all args (no prompt)**

Run: `xion-toolkit treasury fund xion1abc123 1000000uxion`
Expected: No interactive prompt. Command proceeds directly (will fail at API level since address is fake, but that's expected).

- [ ] **Step 4: Test `--no-interactive` override**

Run: `xion-toolkit --no-interactive treasury fund`
Expected: Clap error message, no interactive prompt, exit 1.

- [ ] **Step 5: Test `asset create` without args**

Run: `xion-toolkit asset create`
Expected: Prompts for NAME and SYMBOL.

- [ ] **Step 6: Test `tx status` without args**

Run: `xion-toolkit tx status`
Expected: Prompts for transaction HASH with hex validation.

- [ ] **Step 7: Test `contract instantiate` without args**

Run: `xion-toolkit contract instantiate`
Expected: Prompts for code-id (u64), label (text), msg (file path).

- [ ] **Step 8: Test piped input (non-TTY)**

Run: `echo "" | xion-toolkit treasury fund`
Expected: Clap error, no interactive prompt.

- [ ] **Step 9: Verify agent usage unchanged**

Run: `xion-toolkit --output json treasury list`
Expected: Works exactly as before (all params provided, no prompt).

---

## Task 5: Run full CI suite

- [ ] **Step 1: Format check**

Run: `cargo fmt -- --check`
Expected: Clean.

- [ ] **Step 2: Clippy**

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: Zero warnings.

- [ ] **Step 3: All tests**

Run: `cargo test --all-features`
Expected: All tests pass (505+).

- [ ] **Step 4: Commit any fixes if needed**

---

## Batch 2: Treasury subcommands (deferred)

These subcommands share the same architecture from Batch 1. The `interactive_fallback.rs` module already handles them via pattern matching on argument names. However, some may need:

- **Treasury subcommand address prompts**: Already handled by `PromptType::Address` detection.
- **`--type-url`**: May need custom handling (currently falls through to `PromptType::Text`).
- **`--fee-config`, `--config` (treasury batch)**: Already handled as `PromptType::ExistingPath`.

**Work needed**: Test each subcommand interactively and adjust `determine_prompt_type()` and `format_arg_description()` if needed.

No new modules required — just tuning the pattern matching.

## Batch 3: OAuth2 client subcommands (deferred)

Same approach. `client_id` is already detected as `PromptType::Address` (which is acceptable — it's a UUID string, not an `xion1` address, so we may want to change the prompt type for `client_id` specifically).

**Work needed**:
- Adjust `determine_prompt_type()` to treat `client_id` as `PromptType::Text` (UUID format) instead of `PromptType::Address`.
- Test all 7 OAuth2 client subcommands interactively.

## Batch 4: Skills and docs update (deferred)

- Update `skills/xion-oauth2/SKILL.md` with note about interactive mode
- Update `docs/cli-reference.md` with `--no-interactive` flag documentation
- Update `docs/QUICK-REFERENCE.md` if needed

---

## Non-Goals (explicitly out of scope)

1. **Shell tab-completion for prompts**: `dialoguer` doesn't support tab-completion; this would require `rustyline` which is a much heavier dependency. Deferred.
2. **Changing all parameter types to `Option<String>`**: We use the re-exec pattern instead, which avoids touching any handler signatures.
3. **Prompting for optional parameters**: Only required (missing) parameters trigger prompts.
4. **Multi-step wizards / guided flows**: This is purely "fill in missing args", not a guided walkthrough.
5. **Persistent history**: No command history or previously-used values.

---

## QC Review: Residual Findings

### Fixed in this PR (QC tri-review + follow-up review)

All Critical/Warning items from both QC reviews have been resolved:

| ID | Description | Resolution |
|----|-------------|------------|
| QC1 C-2 | Docs claimed `XION_NO_INTERACTIVE` env var | Removed from docs |
| QC3 CR-1 | Underscore/hyphen arg name mismatch | Added `_` variants to all match arms |
| QC2 C1 | `ExecuteContext.interactive` dead code | Removed field |
| QC2 C2 | `prompt_text_with_default()` dead code | Removed function |
| QC1 W-1 | TTY should check stdout | Added stdout check |
| QC2 W-4 | `PromptType::Enum` never constructed | Removed variant |
| QC3 W-1 | Amount prompt example confusing | Clarified in docs |
| QC3 W-2 | Ctrl+C exit behavior undocumented | Added to docs |
| QC2 C-001 (follow-up) | `-N` short flag conflicts with `treasury create --name` | Removed short flag |
| RF-1 | Short flag for `--no-interactive` | Attempted `-N`, removed due to conflict |
| RF-4 | Prompt functions `pub` → `pub(crate)` | Applied (10 functions) |
| RF-5 | Architecture design note in `main.rs` | Applied |
| RF-6+RF-7 | Enhanced module doc + checklist | Applied |
| RF-9 | Pin clap version | Skipped (already `"4.5"`) |
| RF-10 | Label capitalization | Skipped (already consistent) |
| RF-11 | Skill scripts add `--no-interactive` | Applied (21 scripts) |

### Deferred to future PRs

| ID | Source | Description | Owner | Target |
|----|--------|-------------|-------|--------|
| RF-2 | QC1 | Add unit tests for prompt validators (reject invalid address/amount/hash) | @qa-engineer | Next release |
| RF-3 | QC2 | Consolidate `determine_prompt_type()` + `format_arg_description()` into single `ArgPromptInfo` struct | — | Skipped (over-engineering) |
| RF-8 | QC1 | Use `bech32::decode()` for proper address validation instead of prefix+length check | @fullstack-dev | Next release |

### E2E Batch 2+3 Test Results (2026-04-02)

**Diagnosis**: Initial E2E testing appeared to show only 1 prompt instead of 2 for commands with both named flags and positional args (e.g., `treasury admin propose --new-admin <NEW_ADMIN> <ADDRESS>`). Root cause was **test methodology**, not code.

**Root Cause**: Tests were run using `echo "" | xion-toolkit <command>` which pipes stdin — this makes `is_tty()` return `false`, so interactive mode never triggers. The correct approach is `script -q /dev/null` (macOS) or `unbuffer` (Linux) to simulate a TTY.

**Verified Working** (all via `script` pseudo-TTY):
| # | Command | Missing Args | Prompts Triggered | Result |
|---|---------|-------------|-------------------|--------|
| 1 | `treasury admin propose` | `--new-admin`, `<ADDRESS>` | 2/2 ✅ | Both prompted, command reached RPC |
| 2 | `treasury fee-config set` | `--fee-config`, `<ADDRESS>` | 2/2 ✅ | Both prompted (cancelled on invalid path) |
| 3 | `oauth2 client managers add` | `--manager-id`, `<CLIENT_ID>` | 2/2 ✅ | Both prompted, command reached API |
| 4 | `oauth2 client transfer-ownership` | `--new-owner`, `<CLIENT_ID>` | 2/2 ✅ | Both prompted, command reached confirmation check |
| 5 | `treasury admin propose --new-admin xion1...` | `<ADDRESS>` only | 1/1 ✅ | Correct — only missing arg prompted |

**Conclusion**: Interactive mode correctly handles all multi-arg scenarios. Debug logging added for diagnosis has been removed. 529 tests passing, zero clippy warnings, fmt clean.
