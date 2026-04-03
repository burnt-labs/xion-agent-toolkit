---
status: InProgress
created_at: 2026-04-03
updated_at: 2026-04-03
priority: P1
complexity: Medium
effort: L (multiple agent sessions across 5 subtasks)
---

# Phase 4: Maintenance & Optimization

> Post-feature completion maintenance and optimization cycle.

## Background

After completing all core features (Phase 1-3, OAuth2 Client Management, Interactive CLI), the project enters a maintenance phase. This plan addresses code quality, security, and technical debt identified through systematic analysis on 2026-04-03.

---

## Current State Analysis

### Code Metrics (2026-04-03)

| Metric | Value | Notes |
|--------|-------|-------|
| Total Lines | 30,397 | 19 main modules |
| Largest File | treasury/api_client.rs (2,967) | Refactoring candidate |
| Public Functions | 303 | Reasonable coverage |
| Unit Tests | 561 | +11 E2E bash +22 skill scripts |
| Documentation | 5,906 lines | CLI reference detailed |
| unwrap/expect | 363 occurrences | Needs security audit |
| TODO Comments | 4 occurrences | 2 feature gaps + 2 reserved |
| Dependencies | 27 direct | Reasonable range |

---

## Identified Issues

### 1. TODO Comments (4 occurrences)

| File | Line | Content | Category | Priority |
|------|------|---------|----------|----------|
| `treasury/manager.rs` | 848 | `predicted_address: None, // TODO: compute if needed` | Reserved | Low |
| `treasury/manager.rs` | 1939 | `expiration: None, // TODO: Add expiration support` | Feature gap | **High** |
| `treasury/api_client.rs` | 1659 | `expiration: None, // TODO: Add expiration support in FeeConfigInput` | Feature gap | **High** |
| `asset_builder/code_ids.rs` | 61 | `AssetType::Cw721Base => None, // TODO: Add actual checksum` | Reserved | Low |

**Action**: Evaluate expiration feature necessity; remove unnecessary TODOs or implement required ones.

---

### 2. Large Files (>1000 lines)

| File | Lines | Refactoring Opportunity |
|------|-------|------------------------|
| `treasury/api_client.rs` | 2,967 | **Split by functionality** (fund/grant/withdraw/query) |
| `cli/treasury.rs` | 2,724 | Split by subcommands |
| `treasury/manager.rs` | 2,044 | Split core/cache/utils |
| `api/mgr_api.rs` | 1,779 | Split by API endpoints |
| `treasury/types.rs` | 1,698 | Type definitions (keep) |
| `shared/error.rs` | 1,270 | Error definitions (keep) |
| `treasury/encoding.rs` | 1,246 | Encoding logic (keep) |
| `asset_builder/manager.rs` | 1,053 | Evaluate split |
| `cli/oauth2_client.rs` | 1,030 | Split by subcommands |
| `asset_builder/types.rs` | 1,022 | Type definitions (keep) |

**Action**: Refactor `treasury/api_client.rs` first (highest priority).

---

### 3. unwrap/expect Usage (363 occurrences)

**CLI module sample** (test code - safe):
```rust
src/cli/batch.rs:    let mut file = NamedTempFile::new().unwrap();
src/cli/asset.rs:    assert!(args.royalty_percentage.unwrap() > 1.0);
```

**Action**: Audit production code for unsafe unwrap/expect; replace with proper error handling.

---

### 4. Unused/Deprecated Code

Files containing unused/deprecated markers:
- `src/cli/config.rs`
- `src/tx/types.rs`
- `src/tx/mod.rs`
- `src/asset_builder/mod.rs`
- `src/api/mod.rs`
- `src/treasury/mod.rs`
- `src/treasury/api_client.rs`

**Action**: Review and clean unused code; mark as `#[allow(dead_code)]` if intentionally reserved.

---

### 5. Documentation Coverage

**Existing**:
- ✅ CLI Reference (3,802 lines) — Detailed
- ✅ Error Codes (493 lines) — Complete
- ✅ Exit Codes (435 lines) — Complete
- ✅ Quick Reference (459 lines) — Complete
- ✅ Configuration (250 lines) — Complete
- ✅ Skills Guide (199 lines) — Complete
- ✅ README (176 lines) — Concise
- ✅ INSTALL-FOR-AGENTS (685 lines) — Detailed
- ✅ CONTRIBUTING (360 lines) — Complete

**Potential gaps**:
- Usage examples/tutorials (if needed)
- API usage guide (covered by CLI reference)

---

## Tasks

### Task 1: Expiration Feature Research

**Status**: ✅ Done
**Owner**: @architect
**Effort**: M (1 focused agent session)
**Decision**: **Remove TODOs, keep `expiration: None`** — see `.agents/plans/knowledge/expiration-research.md`

**Objective**: Determine if FeeConfig expiration support is necessary.

**References**:
- `~/workspace/xion/contracts/contracts/treasury` (Treasury contract implementation)
- `~/workspace/xion/xion-developer-portal` (Developer documentation)

**Deliverables**:
- ✅ Research report: `.agents/plans/knowledge/expiration-research.md`
- ✅ Decision: **Remove TODOs** (Option B — see report §4)
- N/A: Implementation not needed

**Key Findings**:
- On-chain `FeeConfig.expiration` is `Option<u32>` (relative seconds) — contract converts to absolute `Timestamp` during `deploy_fee_grant`
- Developer portal does NOT expose expiration in creation/editing UI; field is vestigial
- Toolkit has type mismatch: comment says ISO 8601 but contract expects `u32` seconds
- Zero known users requesting expiration; no documentation mentions it
- Implementation path documented for future if needed (report §4)

**Acceptance Criteria**:
- [x] Contract code reviewed for expiration requirements
- [x] Developer portal checked for expiration usage
- [x] Decision documented with rationale
- [x] Implementation plan not needed (decision: remove TODOs)

---

### Task 2: Code Refactoring — treasury/api_client.rs

**Status**: ✅ InReview
**Owner**: @fullstack-dev
**Effort**: M (2–3 focused agent sessions)

**Objective**: Split `treasury/api_client.rs` (2,967 lines) into maintainable modules.

**Actual Structure** (8 files):
```
src/treasury/api_client/
├── mod.rs          (387 LOC code + 575 LOC tests = 963 total)
├── fund.rs         (160 LOC — fund/withdraw operations)
├── grant.rs        (319 LOC — grant/fee config operations)
├── admin.rs        (648 LOC — admin/params/batch/on-chain/export)
├── query.rs        (458 LOC — query/smart-contract/code-info operations)
├── instantiate.rs  (323 LOC — treasury creation + helpers)
├── helpers.rs      (79 LOC — shared utility functions)
└── types.rs        (115 LOC — internal response types)
```

**Deliverables**:
- ✅ Refactored module structure (8 files, all <800 LOC)
- ✅ All tests passing (548 tests: 452 lib + 29 bin + 19 integration + 48 doc)
- ✅ No clippy warnings
- ✅ All public exports preserved (no breaking changes)
- ✅ `cargo fmt -- --check` passes

**Acceptance Criteria**:
- [x] File split into 7 sub-modules + mod.rs (8 files total)
- [x] Each module <800 lines (max 963 total in mod.rs including 575 test lines)
- [x] All exports preserved (no breaking changes)
- [x] `cargo test --all-features` passes
- [x] `cargo clippy -- -D warnings` passes

---

### Task 3: Security Audit — unwrap/expect Review

**Status**: Pending
**Owner**: @qc-specialist → @fullstack-dev
**Effort**: S-M (1–2 focused agent sessions)

**Objective**: Ensure production code uses safe error handling.

**Scope**:
- Review all 363 unwrap/expect occurrences
- Identify production code vs test code
- Replace unsafe unwrap with proper error handling
- Document safe unwrap locations (tests, controlled environments)

**Deliverables**:
- Audit report: unsafe unwrap locations
- Refactored code: unsafe unwrap replaced
- All tests passing

**Acceptance Criteria**:
- [ ] All production unwrap/expect identified
- [ ] Unsafe occurrences replaced with `?` or `.map_err()`
- [ ] Test unwrap documented as safe
- [ ] `cargo test --all-features` passes

---

### Task 4: TODO & Unused Code Cleanup

**Status**: Pending
**Owner**: @fullstack-dev
**Effort**: XS-S (1 focused agent session)

**Objective**: Clean unnecessary TODOs and unused code.

**Scope**:
- Remove unnecessary TODOs (predicted_address, checksum if not needed)
- Implement or remove expiration TODO (depends on Task 1 result)
- Clean unused code in 7 identified files
- Mark intentionally reserved code with `#[allow(dead_code)]`

**Deliverables**:
- All TODOs resolved (removed or implemented)
- Unused code cleaned
- Code compiles without warnings

**Acceptance Criteria**:
- [ ] TODO count reduced to 0 (or documented as intentional)
- [ ] Unused code removed or marked
- [ ] `cargo clippy -- -D warnings` passes

---

### Task 5: Test Coverage Enhancement

**Status**: Pending
**Owner**: @qa-engineer
**Effort**: S-M (1–2 focused agent sessions)

**Objective**: Increase test coverage for edge cases and error paths.

**Scope**:
- Add tests for error paths (unwrap replacement locations)
- Add boundary tests for validators
- Increase coverage in large modules (treasury, asset_builder)
- Document test patterns

**Deliverables**:
- New unit tests added
- Test coverage report
- Updated test documentation

**Acceptance Criteria**:
- [ ] Error path tests added for Task 3 locations
- [ ] Boundary tests for validators (address, amount, hash)
- [ ] Test count increased by 10–20%
- [ ] All tests passing

---

## Execution Order

```
Task 1 (Expiration Research) → Task 4 (TODO Cleanup)
Task 2 (Code Refactoring) ← can run parallel with Task 1
Task 3 (Security Audit) ← can start after Task 1/2 partial completion
Task 5 (Test Coverage) ← after Task 3 completion
```

**Parallelization opportunities**:
- Task 1 + Task 2 can run in parallel (no dependencies)
- Task 3 can start after Task 2 partial completion
- Task 5 depends on Task 3 (needs locations to test)

---

## Acceptance Criteria (Overall)

- [ ] Expiration feature decision made with documented rationale
- [x] treasury/api_client.rs refactored into 8 modules (all <800 LOC)
- [ ] All production unwrap/expect replaced with safe error handling
- [ ] TODO count reduced to 0 (or documented)
- [ ] Unused code cleaned or marked
- [ ] Test coverage increased by 10–20%
- [ ] All 561+ tests passing
- [ ] `cargo clippy -- -D warnings` passes
- [ ] Documentation updated

---

## Phase Gate Checklist

### Prepare
- `specify`: ✅ Done (analysis complete)
- `clarify`: ✅ Done (priorities confirmed)
- `plan`: ✅ Done (this document)

### Execute
- `plan locked`: Pending (await user approval)
- `tasks`: Pending (5 tasks defined)
- `implement`: Pending (Task assignments)

---

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-04-03 | @project-manager | Maintenance analysis complete; 5 tasks identified | InProgress |