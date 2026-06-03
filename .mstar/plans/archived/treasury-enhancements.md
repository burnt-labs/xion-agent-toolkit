---
status: Done
created_at: 2026-03-09
updated_at: 2026-03-09
done_at: 2026-03-09
---

# Treasury Enhancements

## Background

Current Treasury functionality covers basic operations (create, fund, withdraw, grant-config, fee-config). There are opportunities to enhance usability with batch operations, Treasury attribute management, and improved query capabilities.

Reference: Developer Portal (`~/workspace/xion/xion-developer-portal/src/core/client.ts`)

## Goal

1. Add batch operations for grant-config
2. Add Treasury attribute management (admin, params, grant-config, fee-config)
3. Enhance query capabilities with more on-chain data

## Approach

### 1. Batch Operations

- `treasury grant-config-batch` - Configure multiple grants in one transaction

### 2. Admin Management

- `treasury propose-admin <address> --new-admin <address>` - Propose a new admin
- `treasury accept-admin <address>` - Accept admin role (called by pending admin)
- `treasury cancel-admin <address>` - Cancel pending admin proposal

### 3. Params Configuration

- `treasury update-params <address> --redirect-url <url> --icon-url <url> --metadata <json>` - Update Treasury metadata

### 4. Grant Config Management

- `treasury update-grant-config <address> --type-url <msg_type_url> [options]` - Update existing grant config
- `treasury remove-grant-config <address> --type-url <msg_type_url>` - Remove grant config

### 5. Fee Config Management

- `treasury update-fee-config <address> [options]` - Update fee allowance configuration

### 6. Query Enhancements

- `treasury list-grants <address>` - List all authz grants for a treasury
- `treasury list-allowances <address>` - List all fee allowances for a treasury

## Tasks

### Batch Operations
- [x] Add `grant_config_batch` method to `TreasuryApiClient`
- [ ] Add `treasury grant-config-batch` CLI command (deferred to future iteration)

### Admin Management
- [x] Add `propose_admin` method to `TreasuryApiClient`
- [x] Add `accept_admin` method to `TreasuryApiClient`
- [x] Add `cancel_proposed_admin` method to `TreasuryApiClient`
- [x] Add `treasury propose-admin` CLI command
- [x] Add `treasury accept-admin` CLI command
- [x] Add `treasury cancel-admin` CLI command

### Params Configuration
- [x] Add `update_params` method to `TreasuryApiClient`
- [x] Add `treasury update-params` CLI command

### Grant Config Management
- [x] Add `update_grant_config` method to `TreasuryApiClient` (already existed)
- [x] Add `remove_grant_config` method to `TreasuryApiClient` (already existed)
- [x] Add `treasury update-grant-config` CLI command (already existed via grant-config add)
- [x] Add `treasury remove-grant-config` CLI command (already existed)

### Fee Config Management
- [x] Add `update_fee_config` method to `TreasuryApiClient` (already existed via set_fee_config)
- [x] Add `treasury update-fee-config` CLI command (already existed via fee-config set)

### Query Enhancements
- [x] Add `list_authz_grants` method using on-chain query
- [x] Add `list_fee_allowances` method using on-chain query
- [x] Add `treasury list-grants` CLI command (via chain-query grants)
- [x] Add `treasury list-allowances` CLI command (via chain-query allowances)

### Testing
- [x] All existing tests pass (120 tests passing)
- [x] clippy passes with no warnings
- [x] fmt passes

## Acceptance Criteria

- [x] Batch grant-config API implemented (CLI deferred)
- [x] Admin management flow implemented (propose → accept / cancel)
- [x] Params update modifies Treasury metadata correctly
- [x] Grant config update/remove works correctly
- [x] Fee config update works correctly
- [x] Query commands return structured JSON output
- [x] All tests pass

## Implementation Notes

1. **RPC URL Addition**: Added `rpc_url` parameter to `TreasuryApiClient::new()` to support on-chain queries
2. **Chain Query Commands**: Created new `ChainQuery` subcommand to avoid naming conflict with existing `Query` command
3. **Batch Operations**: API methods implemented but CLI command deferred to future iteration
4. **Grant Config**: `update-grant-config` uses the same `grant-config add` command (UpdateGrantConfig message)
5. **Fee Config**: `update-fee-config` uses the existing `fee-config set` command

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-09 | @fullstack-dev | Implemented admin management, params update, and on-chain query capabilities. Batch CLI deferred. | Done |
| 2026-03-09 | @qc-specialist | Code review: 2 critical issues fixed (metadata JSON parsing, empty params validation) | Pass |
| 2026-03-09 | @qa-engineer | All 325 tests pass, clippy passes, CLI commands verified | ✅ Approved |
