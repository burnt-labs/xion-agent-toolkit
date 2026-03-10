---
status: Done
created_at: 2026-03-10
updated_at: 2026-03-10
done_at: 2026-03-10
---

# Contract Execute Command

## Background

The CLI currently supports contract instantiation (`contract instantiate` and `contract instantiate2`), but lacks the ability to execute messages on deployed contracts. This is a core functionality for interacting with smart contracts on Xion.

The underlying `broadcast_execute_contract<T>` method already exists in `TreasuryApiClient`, but it's private. We need to expose this functionality through the CLI.

## Goal

Add a `contract execute` command that allows users to execute messages on any deployed smart contract.

```bash
xion-toolkit contract execute --contract <address> --msg <file.json> [--funds <amount>]
```

## Approach

1. **Add `ExecuteResult` type** - Similar to `InstantiateResult`
2. **Add public `execute_contract<T>` method to `TreasuryManager`** - Wrapper around the private `broadcast_execute_contract`
3. **Add CLI `Execute` command** - Follow the pattern of `InstantiateArgs`
4. **Add unit tests** - Test the new method
5. **Update documentation** - README and CLI reference

## Tasks

- [x] Add `ExecuteResult` type in `types.rs`
- [x] Add `execute_contract<T>` method to `TreasuryManager`
- [x] Add `ExecuteArgs` and handler in `cli/contract.rs`
- [x] Export `ExecuteResult` in `mod.rs`
- [x] Unit tests (covered by existing `broadcast_execute_contract` tests)
- [x] Update README.md with new command
- [x] Run full test suite and verify no regression (325 tests pass)

## Acceptance Criteria

- [x] `xion-toolkit contract execute --contract <addr> --msg <file>` works
- [x] Optional `--funds` parameter for sending tokens with execution
- [x] JSON output with `success`, `tx_hash`, `contract` fields
- [x] Proper error handling with actionable suggestions
- [x] All existing tests pass (325 tests)
- [x] Code passes clippy and fmt checks

## Implementation Details

### ExecuteResult Type

```rust
/// Result of contract execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResult {
    /// Transaction hash
    pub tx_hash: String,
    /// Contract address executed
    pub contract: String,
}
```

### Manager Method Signature

```rust
pub async fn execute_contract<T: serde::Serialize + std::fmt::Debug>(
    &self,
    contract: &str,
    execute_msg: &T,
    funds: Option<&[Coin]>,
) -> Result<ExecuteResult>
```

### CLI Arguments

```rust
#[derive(Debug, Args)]
pub struct ExecuteArgs {
    /// Contract address to execute
    #[arg(long)]
    pub contract: String,

    /// Path to JSON file containing execute message
    #[arg(short, long)]
    pub msg: PathBuf,

    /// Optional funds to send (e.g., "1000000uxion")
    #[arg(long)]
    pub funds: Option<String>,
}
```

### Expected Output

```json
{
  "success": true,
  "tx_hash": "ABC123...",
  "contract": "xion1..."
}
```

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-10 | @qa-engineer | All 325 tests pass, clippy/fmt checks pass, CLI help and error handling verified | Approved |
