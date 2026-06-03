---
status: Done
created_at: 2026-03-15
updated_at: 2026-03-15
done_at: 2026-03-15
---

# Batch Treasury Operations

## Background

Managing multiple treasuries individually is tedious. Users need bulk operations for:
- Funding multiple treasuries at once
- Applying consistent grant configurations
- Exporting treasury configs for backup/audit

## Current State

| Component | Status | Notes |
|-----------|--------|-------|
| `batch execute` command | ✅ Exists | Generic batch executor |
| `treasury fund` | ✅ Exists | Single treasury |
| `treasury grant-config` | ✅ Exists | Single treasury |
| Treasury export | ❌ Missing | No export functionality |

## Goal

Add batch treasury management commands:
1. `treasury batch fund` - Fund multiple treasuries
2. `treasury batch grant-config` - Configure grants for multiple treasuries
3. `treasury export` - Export treasury configurations

## Approach

### Phase 1: Batch Fund Command

```bash
xion-toolkit treasury batch fund --config funds.json
```

**Config format:**
```json
{
  "operations": [
    {"address": "xion1...", "amount": "1000000uxion"},
    {"address": "xion1...", "amount": "500000uxion"}
  ]
}
```

### Phase 2: Batch Grant Config Command

```bash
xion-toolkit treasury batch grant-config --config grants.json
```

**Config format:**
```json
{
  "grant_type": "send",
  "treasuries": [
    {"address": "xion1...", "spend_limit": "1000000uxion"},
    {"address": "xion1...", "spend_limit": "500000uxion"}
  ],
  "message_type_url": "/cosmos.bank.v1beta1.MsgSend"
}
```

### Phase 3: Export Command

```bash
xion-toolkit treasury export --output treasuries.json
xion-toolkit treasury export --address xion1... --output single.json
```

**Export format:**
```json
{
  "exported_at": "2026-03-15T00:00:00Z",
  "network": "testnet",
  "treasuries": [
    {
      "address": "xion1...",
      "admin": "xion1...",
      "balance": {"uxion": "1000000"},
      "grants": [...],
      "fee_config": {...}
    }
  ]
}
```

### Phase 4: Batch Report

Add summary report after batch operations:

```json
{
  "success": true,
  "data": {
    "total": 5,
    "successful": 4,
    "failed": 1,
    "results": [
      {"address": "xion1...", "status": "success", "tx_hash": "..."},
      {"address": "xion1...", "status": "failed", "error": "..."}
    ]
  }
}
```

## Tasks

### Phase 1: Batch Fund
- [x] Create `TreasuryBatchFundArgs` in `src/cli/treasury.rs`
- [x] Implement config file parsing
- [x] Add `treasury batch fund` subcommand
- [x] Execute funding in sequence with progress reporting
- [x] Handle partial failures gracefully

### Phase 2: Batch Grant Config
- [x] Create `TreasuryBatchGrantConfigArgs`
- [x] Implement config file parsing
- [x] Add `treasury batch grant-config` subcommand
- [x] Apply config to each treasury
- [x] Report success/failure per treasury

### Phase 3: Export
- [x] Create `TreasuryExportArgs`
- [x] Query treasury data (balance, grants, fee config)
- [x] Format as JSON export
- [x] Support single and bulk export
- [ ] Add `--include-history` flag (optional) - skipped per scope

### Phase 4: Reporting
- [x] Add batch operation summary
- [x] Track success/failure counts
- [x] Include tx hashes for successful operations
- [x] Include errors for failed operations

## Acceptance Criteria

- [x] `treasury batch fund --config funds.json` funds all treasuries
- [x] `treasury batch grant-config --config grants.json` configures all
- [x] `treasury export --output file.json` exports configs
- [x] Partial failures don't stop batch execution
- [x] JSON report shows per-treasury results
- [x] Works with `--output json-compact` for CI/CD

## Example Usage

```bash
# Batch fund treasuries
xion-toolkit treasury batch fund --config funds.json

# Batch configure grants
xion-toolkit treasury batch grant-config --config grants.json

# Export all treasuries
xion-toolkit treasury export --output all-treasuries.json

# Export single treasury
xion-toolkit treasury export --address xion1... --output single.json

# Compact output for CI
xion-toolkit treasury batch fund --config funds.json --output json-compact
```

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-15 | @fullstack-dev-2 | Implementation complete: batch fund, batch grant-config, bulk export | ✅ Done |
| 2026-03-15 | @qa-engineer | Acceptance criteria verified: batch fund, batch grant-config, export command, example configs, all tests pass | ✅ SIGN-OFF |
