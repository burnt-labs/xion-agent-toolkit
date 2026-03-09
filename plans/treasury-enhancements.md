---
status: Todo
created_at: 2026-03-09
updated_at: 2026-03-09
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
- [ ] Add `grant_config_batch` method to `TreasuryApiClient`
- [ ] Add `treasury grant-config-batch` CLI command

### Admin Management
- [ ] Add `propose_admin` method to `TreasuryApiClient`
- [ ] Add `accept_admin` method to `TreasuryApiClient`
- [ ] Add `cancel_proposed_admin` method to `TreasuryApiClient`
- [ ] Add `treasury propose-admin` CLI command
- [ ] Add `treasury accept-admin` CLI command
- [ ] Add `treasury cancel-admin` CLI command

### Params Configuration
- [ ] Add `update_params` method to `TreasuryApiClient`
- [ ] Add `treasury update-params` CLI command

### Grant Config Management
- [ ] Add `update_grant_config` method to `TreasuryApiClient`
- [ ] Add `remove_grant_config` method to `TreasuryApiClient`
- [ ] Add `treasury update-grant-config` CLI command
- [ ] Add `treasury remove-grant-config` CLI command

### Fee Config Management
- [ ] Add `update_fee_config` method to `TreasuryApiClient`
- [ ] Add `treasury update-fee-config` CLI command

### Query Enhancements
- [ ] Add `list_grants` method using on-chain query
- [ ] Add `list_allowances` method using on-chain query
- [ ] Add `treasury list-grants` CLI command
- [ ] Add `treasury list-allowances` CLI command

### Testing
- [ ] Add unit tests for batch operations
- [ ] Add unit tests for admin management
- [ ] Add unit tests for params methods
- [ ] Add unit tests for grant config methods
- [ ] Add unit tests for fee config methods
- [ ] Add unit tests for query methods
- [ ] Add E2E tests for new CLI commands

## Acceptance Criteria

- [ ] Batch grant-config works with single transaction
- [ ] Admin management flow works (propose → accept / cancel)
- [ ] Params update modifies Treasury metadata correctly
- [ ] Grant config update/remove works correctly
- [ ] Fee config update works correctly
- [ ] Query commands return structured JSON output
- [ ] All tests pass

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
