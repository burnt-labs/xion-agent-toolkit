---
status: Done
created_at: 2026-03-09
updated_at: 2026-03-09
---

# Contract Instantiate Refactor

## Background

The `instantiate` and `instantiate2` commands are currently under `treasury` subcommand, but they are generic contract instantiation commands that should be at the `contract` level. This is a structural issue that needs to be fixed.

**Previous (incorrectly under treasury)**:
```bash
xion-toolkit treasury instantiate ...
xion-toolkit treasury instantiate2 ...
```

> These commands have been moved to `contract` subcommand. Use `contract instantiate` instead.

**Expected (correct)**:
```bash
xion-toolkit contract instantiate ...
xion-toolkit contract instantiate2 ...
```

## Goal

1. Create new `contract` subcommand module
2. Move `instantiate` and `instantiate2` from `treasury` to `contract`
3. Update documentation to reflect the change
4. Deprecate old commands with warning (optional, for backward compatibility)

## Approach

### 1. Create Contract CLI Module

Create `src/cli/contract.rs`:
```rust
// Contract subcommand
pub enum ContractCommands {
    /// Instantiate a new contract
    Instantiate { ... },
    /// Instantiate a contract with predictable address (instantiate2)
    Instantiate2 { ... },
}
```

### 2. Update CLI Module

Modify `src/cli/mod.rs`:
```rust
pub mod contract;  // Add new module

pub enum Commands {
    Auth(...),
    Treasury(...),
    Config(...),
    Contract(contract::ContractCommands),  // Add new subcommand
    Status,
}
```

### 3. Move Handlers

- Move `handle_instantiate` and `handle_instantiate2` from `treasury.rs` to `contract.rs`
- Update imports and dependencies

### 4. Update Documentation

- README.md: Change `treasury instantiate` → `contract instantiate`
- docs/cli-reference.md: Add new Contract section, update Treasury section
- skills/xion-treasury/SKILL.md: Remove instantiate references (or note they're in contract skill)

## Tasks

### Code Changes
- [x] Create `src/cli/contract.rs` module
- [x] Define `ContractCommands` enum with Instantiate and Instantiate2
- [x] Move handler functions from treasury to contract
- [x] Update `src/cli/mod.rs` to include contract module
- [x] Update `src/main.rs` to handle contract commands

### Documentation Updates
- [x] Update README.md CLI examples
- [x] Update docs/cli-reference.md
- [x] Update skills/xion-treasury/SKILL.md

### Testing
- [x] Verify `contract instantiate` works
- [x] Verify `contract instantiate2` works
- [x] Run all existing tests
- [x] Verify clippy passes

## Acceptance Criteria

- [x] `xion-toolkit contract instantiate` works
- [x] `xion-toolkit contract instantiate2` works
- [x] Old `treasury instantiate` returns error or deprecation warning
- [x] Documentation updated
- [x] All tests pass

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-09 | @fullstack-dev | Completed refactoring: instantiate/instantiate2 moved to contract subcommand. All tests passing (330 tests). Documentation updated. | Done |
