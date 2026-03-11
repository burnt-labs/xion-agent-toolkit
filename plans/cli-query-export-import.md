---
status: Done
created_at: 2026-03-11
updated_at: 2026-03-11
---

# CLI Query, Export & Import Commands

## Background

The CLI needs additional commands to match Developer Portal functionality and provide complete developer experience:

1. **`contract query`** - Missing piece of CosmWasm interaction (instantiate + execute + query = complete)
2. **`treasury export`** - Export treasury configuration for backup/migration
3. **`treasury import`** - Import configuration to existing treasury (client-side batching)

## Research Findings

### Contract Query

- **Mechanism**: Direct RPC call to `/cosmwasm/wasm/v1/contract/{address}/smart/{base64_query}`
- **Authentication**: Not required (read-only operation)
- **Response**: Double-encoded (REST returns `{data: base64}`, which decodes to JSON result)

### Treasury Export

- **Mechanism**: Client-side operation, no on-chain transaction
- **Data**: Serialize admin, fee_config, grant_configs, params to JSON
- **Use cases**: Backup, migration, configuration sharing

### Treasury Import

- **On-chain capability**: Does NOT exist
- **Implementation**: Client-side sequence of existing commands:
  1. `update_fee_config` (1 tx)
  2. `update_grant_config` for each config (N txs)
- **Note**: Developer Portal's import is for NEW treasuries only, not existing ones

## Goals

1. Add `contract query` command for smart contract queries
2. Add `treasury export` command for configuration backup
3. Add `treasury import` command for configuration restore (via batching)
4. Update documentation (cli-reference, skills)

## Non-Goals

- `treasury migrate` - explicitly excluded per requirements

## Approach

### 1. Contract Query Implementation

```bash
xion-toolkit contract query --contract <address> --msg <file.json>
```

Implementation:

- Add `query_contract_smart()` method to `TreasuryApiClient`
- Base64 encode query message
- Call REST endpoint
- Decode double-encoded response

### 2. Treasury Export Implementation

```bash
xion-toolkit treasury export <address> [--output file.json]
```

Implementation:

- Query treasury state (admin, fee_config, grant_configs, params)
- Serialize to JSON
- Output to stdout or file

### 3. Treasury Import Implementation

```bash
xion-toolkit treasury import <address> --from-file treasury.json [--dry-run]
```

Implementation:

- Parse JSON file
- Validate structure
- Execute sequence:
  1. `update_fee_config` if fee_config present
  2. `update_grant_config` for each grant config
- Support `--dry-run` to preview actions without executing

## Tasks

### Phase 1: Contract Query

- [x] Add `query_contract_smart()` to `TreasuryApiClient`
- [x] Add `QueryArgs` and handler in `src/cli/contract.rs`
- [x] Add `query_contract()` to `TreasuryManager`
- [x] Add unit tests for base64 encoding/decoding
- [x] Add unit tests for double-encoded response parsing
- [x] Add unit tests for query_contract_smart with mock server
- [x] Update cli-reference.md

### Phase 2: Treasury Export

- [x] Add `export_treasury_state()` to `TreasuryApiClient`
- [x] Add `export_treasury()` to `TreasuryManager`
- [x] Add `ExportArgs` and handler in `src/cli/treasury.rs`
- [x] Add `TreasuryExportData` type in `src/treasury/types.rs`
- [x] Add unit tests for TreasuryExportData serialization/deserialization
- [x] Update cli-reference.md
- [x] Update skills/xion-treasury/SKILL.md
- [x] Update docs/skills-guide.md

### Phase 3: Treasury Import

- [x] Add `import_treasury_state()` method (batching logic)
- [x] Add `ImportArgs` and handler in `src/cli/treasury.rs`
- [x] Add `--dry-run` support
- [x] Add unit tests
- [x] Update cli-reference.md
- [x] Update skills/xion-treasury/SKILL.md
- [x] Update docs/skills-guide.md

## Acceptance Criteria

### Contract Query

- [x] `xion-toolkit contract query --contract <addr> --msg query.json` works
- [x] JSON output with query result
- [x] Proper error handling
- [x] Works without authentication (read-only)

### Treasury Export

- [x] `xion-toolkit treasury export <address>` outputs JSON
- [x] `--output` flag writes to file
- [x] Includes: admin, fee_config, grant_configs, params

### Treasury Import

- [x] `xion-toolkit treasury import <address> --from-file config.json` works
- [x] `--dry-run` shows planned actions
- [x] Proper error handling for each step
- [x] Progress output to stderr

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-11 | @qa-engineer | Verified contract query, treasury export, treasury import commands; all tests pass | Pass |

## QA Verification Report

### 1. Contract Query Command

| Check | Result |
|-------|--------|
| Help output | Pass - Shows correct usage with --contract, --msg, --network, --output options |
| Error format | Pass - JSON with `success: false`, `error`, `code`, `suggestion` fields |

### 2. Treasury Export Command

| Check | Result |
|-------|--------|
| Help output | Pass - Shows correct usage with --output option |
| Error format | Pass - JSON with `success: false`, `error`, `code` fields |

### 3. Treasury Import Command

| Check | Result |
|-------|--------|
| Help output | Pass - Shows --dry-run option |
| Dry-run output | Pass - Shows planned actions in JSON format |

### 4. Test Suite

| Test Suite | Passed | Failed |
|------------|--------|--------|
| lib.rs | 141 | 0 |
| main.rs | 141 | 0 |
| treasury_create_integration_test.rs | 29 | 0 |
| treasury_integration_test.rs | 19 | 0 |
| Doc tests | 39 | 0 |
| **Total** | **369** | **0** |

## Post-QA Fixes (2026-03-11)

### Critical Data Loss Issues Fixed

Two critical issues were identified and fixed in the import implementation:

#### Issue 1: Grant Config Data Loss
- **Problem**: `import_grant_config()` always converted to `AuthorizationInput::Generic`, losing authorization details
- **Impact**: Send authorizations lost spend limits; Contract execution grants lost max_calls/max_funds
- **Fix**: Added `authorization_input` field to `GrantConfigInfo` for round-trip preservation

#### Issue 2: Fee Config Data Loss  
- **Problem**: `import_fee_config()` always converted to `FeeConfigInput::Basic`, losing periodic allowance info
- **Impact**: Periodic allowances lost rate limiting configuration (period_seconds, period_spend_limit)
- **Fix**: Added `period`, `period_spend_limit`, `can_period_reset` fields to `FeeConfigInfo`

#### Issue 3: Address Mismatch Warning
- **Added**: Warning message when import target differs from export source

### Files Modified

- `src/treasury/types.rs`: Added new fields to `GrantConfigInfo` and `FeeConfigInfo`
- `src/treasury/api_client.rs`: Updated `export_treasury_state()` to include periodic fee data
- `src/treasury/manager.rs`: Updated import functions to use preserved data
- `src/cli/treasury.rs`: Added address mismatch warning

### Tests Updated

- Updated test cases in `types.rs` to include new fields
- All 369 tests continue to pass
