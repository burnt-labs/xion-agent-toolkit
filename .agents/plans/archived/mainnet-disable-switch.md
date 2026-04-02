# Mainnet Disable Switch

## Summary

Add a configurable switch to disable mainnet mode in xion-toolkit CLI. When mainnet is disabled, users attempting to use `--network mainnet` receive a clear error message explaining that mainnet is currently unavailable.

## Motivation

- Current mainnet mode may not work correctly or is not ready for production use
- Users should get a clear, actionable error message instead of cryptic failures
- The switch should be reversible - can be enabled later when mainnet is ready

## Scope

### In

- Add environment variable `XION_MAINNET_DISABLED` to control the switch (default: disabled/true)
- Add compile-time feature flag `mainnet` as alternative control (optional)
- Clear error message when mainnet is disabled
- Update documentation to reflect mainnet availability

### Out

- Actually fixing mainnet functionality (that's a separate effort)
- Changes to network configuration values

## Implementation Plan

### 1. Add Mainnet Disabled Check in `main.rs`

**File**: `src/main.rs`

Add check immediately after network is determined, before command dispatch:

```rust
// After line 22 (after setting XION_NETWORK_OVERRIDE)
if ctx.network == "mainnet" && is_mainnet_disabled() {
    eprintln!("Error: Mainnet mode is currently disabled.");
    eprintln!("  The xion-toolkit CLI is currently only available for testnet.");
    eprintln!("  Please use --network testnet or omit the flag (testnet is default).");
    std::process::exit(exit_codes::MAINNET_DISABLED);
}
```

### 2. Create Helper Function

**File**: `src/config/manager.rs` or `src/shared/mainnet.rs`

```rust
/// Check if mainnet is disabled
/// Priority: 
/// 1. XION_MAINNET_DISABLED env var (true/false, 1/0)
/// 2. Default: true (mainnet disabled by default)
pub fn is_mainnet_disabled() -> bool {
    std::env::var("XION_MAINNET_DISABLED")
        .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
        .unwrap_or(true) // Default: mainnet is disabled
}
```

### 3. Add Exit Code

**File**: `src/shared/exit_codes.rs`

```rust
/// Mainnet mode is disabled
pub const MAINNET_DISABLED: i32 = 10;
```

### 4. Update Error Codes Documentation

**File**: `docs/ERROR-CODES.md`

Add entry for exit code 10.

### 5. Update CLI Reference

**File**: `docs/cli-reference.md` or `docs/QUICK-REFERENCE.md`

Document the mainnet availability status.

## Configuration Options

### Environment Variable (Primary)

```bash
# Disable mainnet (default behavior)
export XION_MAINNET_DISABLED=true

# Enable mainnet (when ready)
export XION_MAINNET_DISABLED=false
```

### Compile-Time Feature Flag (Optional)

```toml
# Cargo.toml
[features]
default = []
mainnet = []  # Enable mainnet support at compile time
```

## Error Message Example

```
$ xion-toolkit --network mainnet auth login

Error: Mainnet mode is currently disabled.
  The xion-toolkit CLI is currently only available for testnet.
  Please use --network testnet or omit the flag (testnet is default).

For more information, see: https://docs.burnt.com/xion/toolkit
```

## Acceptance Criteria

- [x] When `--network mainnet` is used, clear error message is displayed
- [x] Default behavior: mainnet is disabled
- [x] `XION_MAINNET_DISABLED=false` enables mainnet mode
- [x] Exit code is specific and documented (code 10)
- [x] Test cases cover enabled/disabled states
- [x] Documentation updated

## Testing

```bash
# Test 1: Default - mainnet disabled
cargo run -- --network mainnet auth login
# Expected: Error message, exit code 10

# Test 2: Explicitly disabled
XION_MAINNET_DISABLED=true cargo run -- --network mainnet auth login
# Expected: Error message, exit code 10

# Test 3: Explicitly enabled
XION_MAINNET_DISABLED=false cargo run -- --network mainnet auth login
# Expected: Proceeds to auth login (may fail for other reasons, but not mainnet disabled)

# Test 4: Testnet always works
cargo run -- --network testnet auth status
# Expected: Normal operation
```

## Files Modified

1. `src/main.rs` - Added mainnet disabled check before command dispatch
2. `src/shared/mainnet.rs` - New file with `is_mainnet_disabled()` function
3. `src/shared/mod.rs` - Exported mainnet module
4. `src/shared/exit_codes.rs` - Added `MAINNET_DISABLED` exit code (code 10)
5. `docs/ERROR-CODES.md` - Documented exit code 10

## Status

- Status: Done
- Progress: 100%
- Owner: @fullstack-dev
- Completed: 2026-03-19
- QA Sign-off: @qa-engineer (2026-03-19)

### Sign-off Summary

| Criterion | Status |
|-----------|--------|
| When `--network mainnet` is used, clear error message is displayed | ✅ PASS |
| Default behavior: mainnet is disabled | ✅ PASS |
| `XION_MAINNET_DISABLED=false` enables mainnet mode | ✅ PASS |
| `XION_MAINNET_DISABLED=FALSE` enables mainnet mode | ✅ PASS |
| `XION_MAINNET_DISABLED=TRUE` keeps mainnet disabled | ✅ PASS |
| Exit code 10 is returned when disabled | ✅ PASS |
| Testnet never affected by the switch | ✅ PASS |

### Validation Evidence

- 334 unit tests passing
- 29 treasury integration tests passing
- 19 treasury_create integration tests passing
- 48 doc-tests passing
- Clippy: 0 warnings
- Format check: passed
- E2E Tests: 6/6 passed (Tests 1-6 all pass with correct exit codes)
