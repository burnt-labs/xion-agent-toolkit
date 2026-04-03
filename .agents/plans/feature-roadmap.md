---
status: Active
created_at: 2026-03-13
updated_at: 2026-04-03
---

# Feature Roadmap: xion-agent-toolkit

This document outlines the feature roadmap for the Xion Agent Toolkit.

## Current Status

**Phase 4 (Maintenance & Optimization) completed on 2026-04-03.** See `.agents/plans/2026-04-03-maintenance-optimization.md` for details.

**Phase 1 Completed: 2026-03-14**
**Phase 2 Completed: 2026-03-15**
**Phase 3 Completed: 2026-03-15**
**OAuth2 Client Management Completed: 2026-03-31 ~ 04-01**
**Interactive CLI Mode Completed: 2026-04-02**
**Phase 4 Maintenance & Optimization Completed: 2026-04-03**

---

## Phase 4: Maintenance & Optimization (2026-04-03) ✅

Phase 4 focuses on code quality, security, and technical debt resolution after feature completion.

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| Expiration Feature Research | P1 | Medium | ✅ Done |
| Code Refactoring (api_client.rs) | P1 | Medium | ✅ Done |
| Security Audit (unwrap/expect) | P1 | Medium | ✅ Done |
| TODO & Unused Code Cleanup | P2 | Low | ✅ Done |
| Test Coverage Enhancement | P2 | Medium | ✅ Done |

**Plan Document**: `.agents/plans/2026-04-03-maintenance-optimization.md`
**Dependency Analysis**: `.agents/plans/knowledge/task-dependency-analysis.md`

### 4.1 Expiration Feature Research (P1) ✅

**Status**: ✅ Done
**Decision**: Remove TODOs, keep `expiration: None`
**Report**: `.agents/plans/knowledge/expiration-research.md`

**Key Findings**:
- Developer Portal does NOT expose expiration field
- Zero user demand for this feature
- Current toolkit has incorrect comment (ISO 8601 vs u32)
- Removed 2 expiration TODOs

### 4.2 Code Refactoring — treasury/api_client.rs (P1) ✅

**Status**: ✅ Done
**Commit**: 76f8ca2
**QC Review**: Approved (3 reviewers)

**Structure**: Split 2,967-line file into 8 modules (all <800 LOC)
- `mod.rs` (387 LOC + 575 test LOC)
- `admin.rs` (648 LOC)
- `query.rs` (458 LOC)
- `instantiate.rs` (323 LOC)
- `grant.rs` (319 LOC)
- `fund.rs` (160 LOC)
- `types.rs` (115 LOC)
- `helpers.rs` (79 LOC)

### 4.3 Security Audit — unwrap/expect Review (P1) ✅

**Status**: ✅ Done

**Result**: Zero unsafe production unwrap found
- Total occurrences: 409 (351 `.unwrap()` + 58 `.expect()`)
- Production unsafe: 0
- Test code: 406 (safe)

### 4.4 TODO & Unused Code Cleanup (P2) ✅

**Status**: ✅ Done
**Commit**: daad043

**Results**:
- All 4 TODOs removed
- Doc comments corrected
- 0 TODO/FIXME remaining

### 4.5 Test Coverage Enhancement (P2) ✅

**Status**: ✅ Done
**Commit**: 8e23153

**Results**:
- Test count: 548 → 612 (+11.7%)
- New tests: 64 (grant: 11, admin: 12, query: 12, instantiate: 7, encoding: 22)
- All 612 tests passing
- Bug fixes: 3 (base64 typo, 2 test fixes)

---
- Implementation plan if needed

### 4.2 Code Refactoring — treasury/api_client.rs (P1)

**Status**: Pending
**Objective**: Split 2,967-line file into maintainable modules.

**Proposed Structure**:
```
src/treasury/api_client/
├── mod.rs          (core client)
├── fund.rs         (fund operations)
├── grant.rs        (grant config)
├── withdraw.rs     (withdraw ops)
├── query.rs        (query ops)
└── instantiate.rs  (instantiate)
```

**Acceptance Criteria**:
- [ ] File split into 5–6 modules
- [ ] Each module <800 lines
- [ ] No breaking changes
- [ ] All tests passing

### 4.3 Security Audit — unwrap/expect Review (P1)

**Status**: Pending
**Objective**: Replace unsafe unwrap/expect with proper error handling.
**Scope**: 363 occurrences across codebase

**Acceptance Criteria**:
- [ ] Production unsafe unwrap identified
- [ ] Replaced with `?` or `.map_err()`
- [ ] Test unwrap documented as safe
- [ ] All tests passing

### 4.4 TODO & Unused Code Cleanup (P2)

**Status**: Pending
**Objective**: Clean unnecessary TODOs and unused code.

**TODO Count**: 4 (2 feature gaps + 2 reserved)
**Unused Code Files**: 7 files identified

**Acceptance Criteria**:
- [ ] TODO count reduced to 0 (or documented)
- [ ] Unused code cleaned or marked
- [ ] No clippy warnings

### 4.5 Test Coverage Enhancement (P2)

**Status**: Pending
**Objective**: Increase coverage for edge cases and error paths.
**Current**: 561 unit tests

**Acceptance Criteria**:
- [ ] Error path tests added
- [ ] Boundary tests for validators
- [ ] Test count increased by 10–20%

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
| 2026-04-03 | @project-manager | Phase 4 started (Maintenance & Optimization) | 🔵 InProgress |
| 2026-04-03 | @project-manager | verify-address removed from deferred (limited value) | ✅ Done |
| 2026-04-02 | @project-manager | Linear ENG-1576 marked done; revoke/deactivate flow requirement removed from scope | ✅ Done |
| 2026-04-02 | @project-manager | Interactive CLI Mode completed (dialoguer, --no-interactive, 529 tests) | ✅ Done |
| 2026-04-01 | @project-manager | OAuth2 Client Management + Scope Validation completed | ✅ Done |
| 2026-03-15 | @project-manager | Phase 3 completed (Predicted Address ✅, Batch Treasury Ops ✅, IBC/Multi-sig removed) | ✅ Done |
| 2026-03-15 | @project-manager | Phase 2 P2 completed (Skills Test Framework ✅, CI/CD Integration Output ✅, Treasury Analytics removed) | ✅ Done |
| 2026-03-15 | @project-manager | Phase 2 P1 features completed (Error Recovery, Transaction Monitoring, shared module) | ✅ Done |
| 2026-03-14 | @project-manager | All Phase 1 features completed | ✅ Done |
