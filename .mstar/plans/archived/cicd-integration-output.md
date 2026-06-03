---
status: Done
created_at: 2026-03-15
updated_at: 2026-03-15
---

# CI/CD Integration Output

## Background

The CLI currently outputs JSON but has unused `--output` flag and `OutputFormat` enum. For CI/CD integration, we need:
- GitHub Actions workflow command format
- Compact JSON output for minimal logging
- Documented, consistent exit codes

## Current State Analysis

| Component | Status | Notes |
|-----------|--------|-------|
| `--output json` flag | Defined but **unused** | `src/cli/mod.rs` |
| `OutputFormat` enum | Defined but **unused** | `src/utils/output.rs` |
| `print_json()` | Used everywhere | Always pretty-prints |
| Exit codes | Implicit only | 0=success, 1=failure via anyhow |
| `XionErrorCode` | Defined | `src/shared/error.rs`, not mapped to exit codes |

## Goal

Make the CLI first-class citizen in CI/CD pipelines by:
1. Supporting GitHub Actions workflow commands
2. Providing compact JSON output option
3. Documenting and standardizing exit codes

## Approach

### Phase 1: Extend OutputFormat Enum

Extend `src/utils/output.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// JSON output, pretty-printed (default for agents)
    #[default]
    Json,
    /// Compact JSON output (minimal, single-line)
    JsonCompact,
    /// GitHub Actions workflow commands format
    GitHubActions,
}
```

### Phase 2: Wire Up --output Flag

Modify `src/cli/mod.rs`:
- Parse `--output` value into `OutputFormat`
- Pass format to command handlers via `ExecuteContext`

Create `ExecuteContext`:
```rust
pub struct ExecuteContext {
    pub output_format: OutputFormat,
    pub network: Network,
}
```

### Phase 3: Implement Output Handlers

Add to `src/utils/output.rs`:

```rust
impl OutputFormat {
    pub fn output<T: Serialize>(&self, data: &T) -> Result<()> {
        match self {
            Self::Json => print_json_pretty(data),
            Self::JsonCompact => print_json_compact(data),
            Self::GitHubActions => print_github_actions(data),
        }
    }
}

fn print_json_compact<T: Serialize>(data: &T) -> Result<()>;

fn print_github_actions<T: Serialize>(data: &T) -> Result<()> {
    // Check for success/error in data
    // Output appropriate workflow commands:
    // ::notice::{message}
    // ::warning::{message}
    // ::error::{message}
    // ::set-output name={key}::{value}
}
```

### Phase 4: Exit Codes Standardization

Create `src/shared/exit_codes.rs`:

```rust
/// Exit codes for CLI
pub mod exit_code {
    pub const SUCCESS: i32 = 0;
    pub const GENERAL_ERROR: i32 = 1;
    
    // Authentication errors (2-19)
    pub const AUTH_REQUIRED: i32 = 2;
    pub const TOKEN_EXPIRED: i32 = 3;
    
    // Configuration errors (20-39)
    pub const CONFIG_NOT_FOUND: i32 = 20;
    pub const INVALID_CONFIG: i32 = 21;
    
    // Network errors (40-59)
    pub const NETWORK_ERROR: i32 = 40;
    pub const TIMEOUT: i32 = 41;
    
    // Transaction errors (60-79)
    pub const TX_FAILED: i32 = 60;
    pub const TX_TIMEOUT: i32 = 61;
    
    // Treasury errors (80-99)
    pub const TREASURY_NOT_FOUND: i32 = 80;
}

impl XionErrorCode {
    pub fn exit_code(&self) -> i32 { ... }
}
```

Update `main.rs` to use explicit exit codes.

### Phase 5: Documentation

Create `docs/EXIT-CODES.md`:

| Code | Name | Description |
|------|------|-------------|
| 0 | SUCCESS | Operation completed successfully |
| 1 | GENERAL_ERROR | Unspecified error |
| 2 | AUTH_REQUIRED | Authentication required |
| 3 | TOKEN_EXPIRED | Token has expired |
| ... | ... | ... |

## Tasks

### Phase 1: OutputFormat Extension
- [x] Extend `OutputFormat` enum in `src/utils/output.rs`
- [x] Add `FromStr` implementation for CLI parsing
- [x] Add unit tests for format parsing

### Phase 2: CLI Integration
- [x] Create `ExecuteContext` struct
- [x] Wire `--output` flag to `OutputFormat`
- [x] Pass context to all command handlers
- [x] Update `main.rs` to use context

### Phase 3: Output Handlers
- [x] Implement `print_json_compact()`
- [x] Implement `print_github_actions()` with workflow commands
- [x] Create `GithubActionsOutput` struct for structured output
- [x] Add unit tests for each format

### Phase 4: Exit Codes
- [x] Create `src/shared/exit_codes.rs`
- [x] Map `XionErrorCode` to exit codes
- [x] Update `main.rs` to use explicit exit codes
- [x] Add integration tests for exit codes

### Phase 5: Documentation
- [x] Create `docs/EXIT-CODES.md`
- [ ] Update `docs/cli-reference.md` with format options (deferred - minor enhancement)
- [ ] Update `docs/QUICK-REFERENCE.md` (deferred - minor enhancement)
- [ ] Add CI/CD usage examples to README (deferred - minor enhancement)

## Acceptance Criteria

- [x] `--output json` (default): pretty-printed JSON (current behavior)
- [x] `--output json-compact`: single-line JSON, parseable by `jq`
- [x] `--output github-actions`: workflow commands for annotations
- [x] Exit codes documented in `docs/EXIT-CODES.md`
- [x] All exit codes are consistent and numeric
- [x] GitHub Actions format produces valid workflow commands
- [x] All tests pass

## Example Usage

```bash
# Default (JSON pretty-printed)
xion-toolkit auth login
# Output: { "success": true, "data": { ... } }

# Compact JSON for CI
xion-toolkit auth login --output json-compact
# Output: {"success":true,"data":{...}}

# GitHub Actions format
xion-toolkit auth login --output github-actions
# Output:
# ::notice::{Successfully logged in as xion1...}
# ::set-output name=xion_address::xion1...

# Exit codes in scripts
xion-toolkit treasury fund $ADDR 1000uxion
if [ $? -eq 0 ]; then
  echo "Success"
elif [ $? -eq 3 ]; then
  echo "Token expired, please re-login"
fi
```

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-15 | @fullstack-dev | Implementation complete: OutputFormat extended, ExecuteContext created, exit codes module added, docs created. Pending @qc-specialist review. | Pending Review |
| 2026-03-15 | @fullstack-dev | Fixed ExecuteContext propagation: All command handlers now receive `&ExecuteContext` and use `print_formatted()` for output. `cargo test` passes (298 tests), `cargo clippy` passes. | Done |
| 2026-03-15 | @qc-specialist | Code review approved: All handler signatures correct, context propagation verified, no warnings. | Approved |
| 2026-03-15 | @qa-engineer | All acceptance criteria verified: json/json-compact/github-actions formats working, 298 tests pass, no clippy warnings. **SIGN-OFF** | ✅ Done |

## PUA & Failure Log

### [PUA-REPORT] 2026-03-15: ExecuteContext Propagation Fix

**Background**: The task "Pass context to all command handlers" was marked complete but NOT actually implemented. `ExecuteContext` was only passed to `handle_status_command`, not to other command handlers.

**Issue Identified By**: @qc-specialist and @qa-engineer during code review

**Root Cause**: The initial implementation only partially completed the task - only the `handle_status_command` received the context.

**Fix Applied**:
1. Updated `src/cli/mod.rs` - all handler function signatures to accept `&ExecuteContext`
2. Updated `src/main.rs` - pass context to all handlers
3. Updated all command files (`auth.rs`, `treasury.rs`, `config.rs`, `account.rs`, `contract.rs`, `batch.rs`, `asset.rs`, `tx.rs`) to:
   - Accept `ctx: &ExecuteContext` parameter
   - Replace `print_json()` with `print_formatted(ctx.output_format(), ...)`

**Validation**:
- `cargo test` passes (298 tests)
- `cargo clippy` passes (no warnings)
- `xion-toolkit config show --output json-compact` outputs compact JSON
- `xion-toolkit config show --output github-actions` outputs GitHub Actions format
- `xion-toolkit auth status --output json-compact` outputs compact JSON
- `xion-toolkit treasury list --output json-compact` outputs compact JSON
