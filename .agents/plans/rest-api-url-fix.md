---
status: Done
created_at: 2026-03-18
updated_at: 2026-03-18
done_at: 2026-03-18
---

# REST API URL Fix - Separate Chain Query Endpoint

## Background

The xion-toolkit currently uses the RPC URL (`rpc.xion-testnet-2.burnt.com:443`) for REST-style chain queries like `/cosmwasm/wasm/v1/...` and `/tx?hash=...`. However, the RPC endpoint doesn't support these REST paths, resulting in 404 errors.

**Evidence**:
- ❌ `https://rpc.xion-testnet-2.burnt.com:443/cosmwasm/wasm/v1/code/1260` → 404
- ✅ `https://api.xion-testnet-2.burnt.com/cosmwasm/wasm/v1/code/1260` → 200 OK

**Impact**:
- `faucet create` command fails with 404 when querying code info
- `treasury` commands using contract queries fail
- `tx wait` command may fail

## Goal

Add a dedicated `rest_url` field to `NetworkConfig` and update all chain query code to use it instead of `rpc_url`.

## Root Cause Analysis

1. **NetworkConfig** (`build.rs` lines 26-43) only has `rpc_url`, no `rest_url` or `lcd_url`
2. **TreasuryApiClient** (`src/treasury/api_client.rs`) uses `rpc_url` for:
   - `query_contract_smart()` → `/cosmwasm/wasm/v1/contract/{addr}/smart/{query}`
   - `get_code_info()` → `/cosmwasm/wasm/v1/code/{code_id}`
3. **TxClient** (`src/tx/client.rs`) uses `rpc_url` for:
   - `get_tx()` → `/tx?hash=0x{hash}`

## Approach

### Phase 1: Add `rest_url` to NetworkConfig

1. Modify `build.rs`:
   - Add `rest_url: String` to `NetworkConfig` struct
   - Add testnet value: `https://api.xion-testnet-2.burnt.com`
   - Add mainnet value: `https://api.xion-mainnet-1.burnt.com`

### Phase 2: Update TreasuryApiClient

1. Modify `TreasuryApiClient`:
   - Rename `rpc_url` field to `rest_url` (for chain queries)
   - Update `new()` signature to accept `rest_url`
   - Update all doc examples
   - Update `query_contract_smart()` to use `rest_url`
   - Update `get_code_info()` to use `rest_url`

### Phase 3: Update TxClient

1. Modify `TxClient`:
   - Rename `rpc_url` field to `rest_url`
   - Update `new()` signature
   - Update `get_tx()` to use `rest_url`
   - Update doc examples

### Phase 4: Update All Callers

Update all 21 `TreasuryApiClient::new()` call sites:
- `src/cli/faucet.rs` (3 sites)
- `src/cli/treasury.rs` (1 site)
- `src/treasury/manager.rs` (2 sites)
- `src/batch/executor.rs` (1 site)
- `src/asset_builder/manager.rs` (1 site)
- `src/treasury/api_client.rs` (tests, 13 sites)

Update `TxClient::new()` call sites.

## Tasks

- [x] T1: Add `rest_url` field to `NetworkConfig` in `build.rs`
- [x] T2: Update `TreasuryApiClient` to use `rest_url`
- [x] T3: Update `TxClient` to use `rest_url`
- [x] T4: Update all caller sites to pass `config.rest_url`
- [x] T5: Run tests and fix any failures
- [x] T6: Manual E2E verification

## Acceptance Criteria

- [x] `xion-toolkit faucet info` no 404 error - **VERIFIED** (HTTP 200)
- [x] `xion-toolkit faucet status` no 404 error - **VERIFIED** (HTTP 200)
- [x] `xion-toolkit treasury list` works correctly - **VERIFIED** (17 treasuries returned)
- [x] All 375 tests pass
- [x] No clippy warnings
- [x] Code is formatted with `cargo fmt`

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-18 | @qa-engineer | E2E verification passed. 404 resolved. URL correctly points to api.xion-*.burnt.com | ✅ APPROVED |