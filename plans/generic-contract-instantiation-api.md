---
status: Done
created_at: 2026-03-09
updated_at: 2026-03-09
---

# Generic Contract Instantiation Public API

## Background

The `broadcast_instantiate_contract` and `broadcast_instantiate_contract2` methods are currently private. Users cannot instantiate arbitrary CosmWasm contracts - they can only create Treasury contracts.

## Goal

1. Expose generic contract instantiation as public API in `TreasuryManager`
2. Add CLI commands for `xion-toolkit treasury instantiate` and `xion-toolkit treasury instantiate2`
3. Allow users to instantiate any CosmWasm contract, not just Treasury

## Approach

1. Add public methods to `TreasuryManager`:
   - `instantiate_contract<T: Serialize>()` - generic v1 instantiation
   - `instantiate_contract2<T: Serialize>()` - generic v2 instantiation with predictable address

2. Add CLI commands:
   - `xion-toolkit treasury instantiate --code-id <ID> --msg <JSON_FILE> --label <LABEL>`
   - `xion-toolkit treasury instantiate2 --code-id <ID> --msg <JSON_FILE> --label <LABEL> --salt <HEX>`

3. Support file input for `--msg` (JSON file containing instantiate message)

## Tasks

- [x] Make `broadcast_instantiate_contract` public in `TreasuryApiClient`
- [x] Make `broadcast_instantiate_contract2` public in `TreasuryApiClient`
- [x] Add `InstantiateResult` type to `types.rs`
- [x] Add `Instantiate2Result` type to `types.rs`
- [x] Add `instantiate_contract` method to `TreasuryManager`
- [x] Add `instantiate_contract2` method to `TreasuryManager`
- [x] Add `Instantiate` CLI command to `treasury.rs`
- [x] Add `Instantiate2` CLI command to `treasury.rs`
- [x] All tests pass (120 tests)

## Acceptance Criteria

- [x] API methods are public
- [x] Manager has `instantiate_contract` and `instantiate_contract2` methods
- [x] CLI has `instantiate` and `instantiate2` commands
- [x] Commands read JSON msg from file
- [x] Output is structured JSON with tx_hash
- [x] All tests pass
- [x] cargo clippy passes

## Implementation Details

### Files Modified

1. `src/treasury/api_client.rs`
   - Made `broadcast_instantiate_contract` public
   - Made `broadcast_instantiate_contract2` public

2. `src/treasury/types.rs`
   - Added `InstantiateResult` struct
   - Added `Instantiate2Result` struct

3. `src/treasury/manager.rs`
   - Added `instantiate_contract()` method
   - Added `instantiate_contract2()` method

4. `src/cli/treasury.rs`
   - Added `Instantiate` variant to `TreasuryCommands`
   - Added `Instantiate2` variant to `TreasuryCommands`
   - Added `handle_instantiate()` handler
   - Added `handle_instantiate2()` handler

### CLI Usage

```bash
# Instantiate contract (v1 - dynamic address)
xion-toolkit treasury instantiate \
  --code-id 123 \
  --label "my-contract" \
  --msg ./instantiate_msg.json \
  --admin xion1... # optional

# Instantiate contract (v2 - predictable address)
xion-toolkit treasury instantiate2 \
  --code-id 123 \
  --label "my-contract" \
  --msg ./instantiate_msg.json \
  --salt 0123456789abcdef... # optional, auto-generated if not provided \
  --admin xion1... # optional
```

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-09 | @backend-engineer | Implemented all tasks, all tests pass | Done |