---
status: Active
created_at: 2026-03-13
updated_at: 2026-04-02
---

# Feature Roadmap: xion-agent-toolkit

This document outlines the feature roadmap for the Xion Agent Toolkit.

## Current Status

**All phases complete as of 2026-04-02.** See `.agents/plans/status.json` for deferred items and the full archive at `.agents/plans/archived/README.md`.

**Phase 1 Completed: 2026-03-14**
**Phase 2 Completed: 2026-03-15**
**Phase 3 Completed: 2026-03-15**
**OAuth2 Client Management Completed: 2026-03-31 ~ 04-01**
**Interactive CLI Mode Completed: 2026-04-02**

---

## Phase 2: Ecosystem Polish

Phase 2 focuses on improving developer experience, code quality, and production readiness.

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| Error Recovery Enhancement | P1 | Medium | ✅ Done (2026-03-15) |
| Transaction Monitoring | P1 | Low | ✅ Done (2026-03-15) |
| `shared/` Module Implementation | P2 | Low | ✅ Done (2026-03-15) |
| Skills Test Framework | P2 | Medium | ✅ Done (2026-03-15) |
| CI/CD Integration Output | P2 | Low | ✅ Done (2026-03-15) |

### 2.1 Error Recovery Enhancement (P1) ✅

**Completed: 2026-03-15**

- 40+ structured error codes (EAUTH, ETREASURY, EASSET, EBATCH, ECONFIG, ENETWORK, ETX)
- Exponential backoff retry logic
- Refactored oauth2_api.rs, treasury/api_client.rs, treasury/manager.rs
- Documentation in `docs/ERROR-CODES.md`

### 2.2 Transaction Monitoring (P1) ✅

**Completed: 2026-03-15**

```bash
xion-toolkit tx status <tx_hash> --output json
xion-toolkit tx wait <tx_hash> --timeout 60 --interval 2 --output json
```

- Query transaction status from RPC
- Wait for confirmation with configurable timeout
- Proper exit codes (0=success, 1=failure/timeout)

### 2.3 `shared/` Module Implementation (P2) ✅

**Completed: 2026-03-15** (as part of Error Recovery)

- `src/shared/error.rs` - Structured error types
- `src/shared/retry.rs` - Retry logic with exponential backoff
- Re-exports for backward compatibility

### 2.4 Skills Test Framework (P2) ✅

**Completed: 2026-03-15**

- Created `tests/skills/` directory with skill-specific E2E tests
- Added mock/stub support for testing without network access
- 48 tests, 58 mock scenarios
- Documented testing patterns in skill SKILL.md files
- CI integration completed

### 2.5 CI/CD Integration Output (P2) ✅

**Completed: 2026-03-15**

- Extended `OutputFormat` enum with `JsonCompact` and `GitHubActions` variants
- Created `ExecuteContext` for passing output format to command handlers
- Implemented GitHub Actions workflow commands (`::notice::`, `::error::`, etc.)
- Created `src/shared/exit_codes.rs` with 41 mapped exit codes
- Created `docs/EXIT-CODES.md` documentation

**Usage:**
```bash
xion-toolkit auth status --output json           # Pretty-printed (default)
xion-toolkit auth status --output json-compact   # Single-line JSON
xion-toolkit auth status --output github-actions # GitHub Actions format
```

**Acceptance Criteria:**
- [x] GitHub Actions format outputs workflow commands
- [x] Exit codes documented and consistent
- [x] JSON output is parseable by standard tools (jq)

### ~~2.6 Treasury Analytics (P3)~~ — Removed

**Removed: 2026-03-15** — On-chain queries are too heavy for this toolkit. Analytics features should be handled by external indexing services.

---

## Phase 3: Advanced Features

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| Predicted Address Computation | P1 | Low | ✅ Done (2026-03-15) |
| Batch Treasury Operations | P2 | Medium | ✅ Done (2026-03-15) |

### 3.1 Predicted Address Computation (P1) ✅

**Completed: 2026-03-15**

- Created `src/shared/instantiate2.rs` with address computation logic
- Added `--predict` and `--salt` flags to `treasury create`
- Auto-detects salt encoding (UTF-8 or hex)
- 17 new unit tests

**Usage:**
```bash
# Predict address without deploying
xion-toolkit treasury create --predict --salt "my-salt"
# Output: {"success":true,"data":{"predicted_address":"xion1...","salt":"my-salt","code_id":1260}}

# Deploy with same salt to get same address
xion-toolkit treasury create --salt "my-salt"
```

**Acceptance Criteria:**
- [x] `treasury create --predict` returns predicted address
- [x] Predicted address matches actual deployment
- [x] Checksum validation implemented

### 3.2 Batch Treasury Operations (P2) ✅

**Completed: 2026-03-15**

- Added `treasury batch fund` subcommand
- Added `treasury batch grant-config` subcommand
- Enhanced `treasury export` for bulk export
- Partial failure handling with detailed reports

**Usage:**
```bash
# Batch fund treasuries
xion-toolkit treasury batch fund --config funds.json

# Batch configure grants
xion-toolkit treasury batch grant-config --config grants.json

# Export all treasuries
xion-toolkit treasury export --output treasuries.json
```

**Acceptance Criteria:**
- [x] Batch fund from JSON config
- [x] Apply same grant config to multiple treasuries
- [x] Export all treasury configs

### ~~3.3 IBC Transfer Enhancement~~ — Removed

**Removed: 2026-03-15** — Not needed for current roadmap.

### ~~3.4 Multi-sig Treasury Support~~ — Removed

**Removed: 2026-03-15** — Not needed for current roadmap.

---

## Deferred Features

These features are acknowledged but deferred to external projects or future consideration.

| Feature | Reason | Alternative |
|---------|--------|-------------|
| Transaction Wait (standalone) | Better suited for xion-skills | Use `bash wait-tx.sh` from xion-skills |

---

## Completed Features

### Phase 1 (Completed 2026-03-14)

| Feature | Priority | Complexity | Completed |
|---------|----------|------------|-----------|
| MetaAccount Info | P1 | Low | 2026-03-13 |
| Extended Grant Types | P2 | Medium | 2026-03-13 |
| Batch Operations | P3 | Medium | 2026-03-13 |
| Asset Builder (CW721) | P4 | High | 2026-03-14 |

#### MetaAccount Info (P1)
- Added `xion-toolkit account info` command
- Uses OAuth2 API `/api/v1/me` endpoint
- Returns MetaAccount authenticators data

#### Extended Grant Types (P2)
- Added 12 new presets to support all 24 message type URLs
- Added 11 unit tests
- Updated error messages

#### Batch Operations (P3)
- Added `src/batch/` module with executor and types
- Added `xion-toolkit batch execute` command
- 23 unit tests passing

#### Asset Builder (P4)
- Phase 1: Basic CW721 deployment and minting
- Phase 2: All 5 variants supported (cw721-base, metadata-onchain, expiration, non-transferable, cw2981-royalties)
- Phase 3: Address prediction and batch minting
- Skills integration: `skills/xion-asset/`
- 232 tests passing

---

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-04-02 | @project-manager | Linear ENG-1576 marked done; revoke/deactivate flow requirement removed from scope | ✅ Done |
| 2026-04-02 | @project-manager | Interactive CLI Mode completed (dialoguer, --no-interactive, 529 tests) | ✅ Done |
| 2026-04-01 | @project-manager | OAuth2 Client Management + Scope Validation completed | ✅ Done |
| 2026-03-15 | @project-manager | Phase 3 completed (Predicted Address ✅, Batch Treasury Ops ✅, IBC/Multi-sig removed) | ✅ Done |
| 2026-03-15 | @project-manager | Phase 2 P2 completed (Skills Test Framework ✅, CI/CD Integration Output ✅, Treasury Analytics removed) | ✅ Done |
| 2026-03-15 | @project-manager | Phase 2 P1 features completed (Error Recovery, Transaction Monitoring, shared module) | ✅ Done |
| 2026-03-14 | @project-manager | All Phase 1 features completed | ✅ Done |
