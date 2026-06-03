# Xion Agent Toolkit — Agent Guide

CLI-first, agent-oriented Rust toolkit for Xion blockchain. Uses OAuth2/MetaAccount for gasless transactions.

## Agent documentation layers

| Layer | File | Scope |
|-------|------|--------|
| Project (this file) | `AGENTS.md` | Rust CLI, build/test, coding style, security |
| Harness | `.mstar/AGENTS.md` | `.mstar/status.json`, plans, knowledge, QC reports |
| Skills | `.agents/AGENTS.md` | Bundled `skills/`, install paths, skill conventions |

`CLAUDE.md` mirrors this file for Claude Code. Subtree `AGENTS.md` files are self-contained; do not duplicate harness rules here or in `.agents/AGENTS.md`.

## Build / Lint / Test Commands

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Format (run before every commit)
cargo fmt

# Check formatting (CI uses this)
cargo fmt -- --check

# Lint — MUST pass with zero warnings
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests (unit + integration)
cargo test
cargo test --all-features

# Run a SINGLE test by name
cargo test test_pkce_challenge
cargo test test_pkce_verifier_length

# Run tests in a specific module
cargo test --lib config::encryption
cargo test --lib shared::error

# Run tests with stdout visible
cargo test -- --nocapture

# Run one test with output
cargo test test_pkce_challenge -- --nocapture

# Run only ignored tests
cargo test -- --ignored

# Security audit
cargo audit
```

## Project Structure

```
src/
├── main.rs, lib.rs
├── cli/          # CLI command definitions (clap Subcommand enums) & handlers
├── api/          # HTTP clients (OAuth2 API, Manager/Indexer API)
├── oauth/        # OAuth2 flow, PKCE, token management, callback server
├── config/       # Config manager, credential encryption, network constants
├── treasury/     # Treasury contract operations
├── asset_builder/# CW721 NFT minting
├── batch/        # Batch transaction execution
├── tx/           # Transaction monitoring
├── account/      # MetaAccount queries
├── shared/       # Error types, retry logic, exit codes, instantiate2
└── utils/        # Output formatting (JSON/human/GHA)
skills/           # Agent skills (bash scripts + JSON schemas) that consume the CLI.
                  # Each skill has its own SKILL.md with conventions.
                  # Read the relevant SKILL.md if modifying a skill.
.mstar/           # Harness: status.json, plans/, knowledge/ — see .mstar/AGENTS.md
.agents/          # Codex/Cursor skill context anchor — see .agents/AGENTS.md
tests/            # E2E bash tests (e2e_*.sh) and skills mock tests.
scripts/          # Build/utility scripts (NOT test scripts).
```

## Rust Code Style

### Imports

- Group: (1) std, (2) external crates, (3) crate-internal. No blank lines between groups in practice; separate with a single blank line after `use` block.
- Prefer `use crate::module::Type;` over glob imports.
- Local `use` statements inside function bodies are acceptable for handler functions (e.g., `handle_login`).

### Error Handling

- **Custom domain errors**: `thiserror` derive macros in `src/shared/error.rs`. Error codes follow `E{MODULE}{NUMBER}` schema (e.g., `EAUTH001`, `ETREASURY002`).
- **Error propagation in handlers**: `anyhow::Result` with `.context()` for adding context.
- CLI success → `stdout` as JSON. CLI errors → `stderr` with structured `ErrorResponse` (code, message, hint).
- Use `crate::utils::output::print_formatted(&data, ctx.output_format())` for all output.

### Structs & Types

- Derive `Debug` on all public types. Add `Clone` where needed for async handlers.
- Serialize with `serde`: use `#[serde(rename_all = "snake_case")]` for JSON field naming.
- Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields.
- clap CLI types use `#[derive(Parser)]` / `#[derive(Subcommand)]`.

### Naming Conventions

- **Modules**: `snake_case` files and directories (`oauth2_api.rs`, `asset_builder/`).
- **Types**: `PascalCase` (`OAuth2ApiClient`, `TokenResponse`, `ExecuteContext`).
- **Constants**: `SCREAMING_SNAKE_CASE` (`ENV_KEY_NAME`, `KEY_LEN`).
- **CLI args**: `kebab-case` flags (`--output json`, `--fee-allowance-type`).
- **Error codes**: `EMODULE` + 3 digits (`EAUTH001`, `ETREASURY009`).
- Tests: `test_` prefix, descriptive snake_case (`test_pkce_verifier_length`).

### Async Patterns

- Use `#[tokio::main]` for the binary entry. All CLI command handlers are `async fn` returning `anyhow::Result<()>`.
- Use `#[instrument]` (tracing) on API client methods for observability.
- Use `tracing::{info, debug, warn, error}` — never `println!` for logging in library code.

### Documentation

- Module-level: `//!` doc comments with description and usage examples.
- Public items: `///` doc comments with `# Arguments`, `# Returns`, `# Example` sections.
- All comments in English.

## Testing Rules

- **Single test**: `cargo test <exact_test_name>`
- **Env var mutation** (especially `XION_CI_ENCRYPTION_KEY`): MUST use `#[serial(encryption_key)]` from `serial_test`. Bare `#[serial]` is a different group and will NOT serialize correctly.
- **Async integration tests**: `#[tokio::test]`
- Current test count: **500 tests passing**

## Git / CI

- **Commit messages**: Conventional commits: `feat(cli): ...`, `fix(treasury): ...`, `docs(skill): ...`
- **CI gate** (all must pass):
  - `cargo fmt -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --all-features`
  - `cargo audit`
- **No** `rustfmt.toml` or `.clippy.toml` — uses default toolchain settings.

## Critical Rules

- NEVER delete `~/.xion-toolkit/credentials/*.enc` — they hold long-lived refresh tokens.
- NEVER run `auth logout` unless explicitly requested.
- Treasury `xion17vg5l9za4768g0hnxezltgnu4h7eleqdcmwark2uuz2s4z5q4dfsr80vvm` is write-protected in all tests/scripts.
- OAuth2 API is transaction-only; queries go to DaoDao Indexer or RPC.
- CLI uses raw JSON object format for `MsgExecuteContract` / `MsgInstantiateContract2` (never base64).

## Key Dependencies

`clap` (CLI), `reqwest` (HTTP), `tokio` (async), `serde`/`serde_json` (serialization), `thiserror`+`anyhow` (errors), `aes-gcm` (credential encryption), `tracing` (logging), `prost` (protobuf), `cosmwasm-std` (CosmWasm types), `axum` (callback server).
