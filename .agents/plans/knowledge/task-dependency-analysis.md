# Task Dependency Analysis — Phase 4 Maintenance

> Analysis of task dependencies and parallel execution feasibility.

## Task Dependency Graph

```
┌─────────────────────────────────────────────────────────┐
│  Session 1 (Parallel Start)                             │
│  ┌──────────────┐        ┌──────────────┐              │
│  │  Task 1      │        │  Task 2      │              │
│  │  Expiration  │        │  Refactoring │              │
│  │  Research    │        │  (api_client)│              │
│  └──────────────┘        └──────────────┘              │
└─────────────────────────────────────────────────────────┘
         │                         │
         │ Decision                 │ New structure
         │ (implement/remove)       │ (5-6 modules)
         ↓                         ↓
┌─────────────────────────────────────────────────────────┐
│  Session 2 (Sequential)                                 │
│  ┌──────────────┐                                       │
│  │  Task 3      │ ← Must wait for Task 2               │
│  │  Security    │   (avoid duplicate work)             │
│  │  Audit       │                                       │
│  └──────────────┘                                       │
│         │ Fixed locations                               │
│         ↓                                               │
│  ┌──────────────┐        ┌──────────────┐              │
│  │  Task 4      │        │  Task 5      │              │
│  │  TODO Cleanup│ ← wait │  Test        │ ← wait       │
│  │  (if remove) │ Task 1 │  Coverage    │ Task 3       │
│  └──────────────┘        └──────────────┘              │
│                                                         │
│  Task 4 also has:                                       │
│  - predicted_address TODO (independent)                 │
│  - checksum TODO (independent)                          │
│  - unused code (independent)                            │
│  → Can partially run after Task 1                      │
└─────────────────────────────────────────────────────────┘
```

---

## Dependency Details

### Task 1 → Task 4 (Decision Dependency)

**Task 1**: Expiration feature research
- Outputs: Decision (implement or remove TODO)
- Location: `treasury/manager.rs:1939`, `treasury/api_client.rs:1659`

**Task 4**: TODO cleanup
- If Task 1 decides **implement**: Task 4 must wait for implementation (new task)
- If Task 1 decides **remove**: Task 4 can immediately remove the TODO

**Impact**: Task 4 partially blocked by Task 1 decision.

---

### Task 2 → Task 3 (Structure Dependency) ⚠️ CRITICAL

**Task 2**: Refactor `treasury/api_client.rs` (2,967 lines)
- Current structure: Single large file
- Target structure: 5-6 modules (fund.rs, grant.rs, withdraw.rs, etc.)

**Task 3**: Security audit (unwrap/expect review)
- Scope: 363 occurrences across codebase
- Includes: `treasury/api_client.rs` (highest priority)

**Dependency Reason**:
- If Task 3 runs **before** Task 2:
  - Unwrap locations identified in original `api_client.rs`
  - Task 2 splits the file → locations move to new modules
  - Task 3 needs to **re-audit** the new structure
  - **Duplicate work + potential conflicts**

- If Task 3 runs **after** Task 2:
  - Audit on final structure → one-time work
  - No duplicate effort

**Recommendation**: Task 3 **must wait for Task 2 completion**.

---

### Task 3 → Task 5 (Testing Dependency)

**Task 3**: Security audit
- Outputs: Locations of fixed unwrap/expect
- Example: Replace `unwrap()` with `?` in `api_client/fund.rs`

**Task 5**: Test coverage enhancement
- Needs: Error path tests for fixed locations
- Example: Add test for new error handling in `fund.rs`

**Dependency Reason**: Task 5 tests the **locations fixed in Task 3**.
- Running Task 5 before Task 3 = tests don't know what to cover
- Running after = targeted tests for specific error paths

**Recommendation**: Task 5 **must wait for Task 3 completion**.

---

### Task 2 → Task 5 (Test Structure Dependency)

**Task 2**: Code refactoring
- Changes module structure
- Example: `api_client.rs` → `api_client/mod.rs` + sub-modules

**Task 5**: Test coverage
- Needs to adapt test imports to new structure
- Example: `use crate::treasury::api_client::*` → multiple imports

**Impact**: Minor (test refactoring is straightforward), but still a dependency.

---

## Parallel Execution Analysis

### ✅ Can Run in Parallel (Session 1)

| Task | Reason | Risk |
|------|--------|------|
| **Task 1** | Independent research (external repos) | None |
| **Task 2** | Independent refactoring (no external dependencies) | None |

**Parallel Benefit**: Reduces total time by ~50% for first phase.

---

### ⚠️ Cannot Run in Parallel (Must Sequential)

| Task | Must Wait For | Reason |
|------|---------------|--------|
| **Task 3** | Task 2 | Avoid duplicate audit work (structure dependency) |
| **Task 4** | Task 1 (partial) | Decision dependency for expiration TODO |
| **Task 5** | Task 3 | Needs fixed locations to test |

**Critical Path**: Task 2 → Task 3 → Task 5 (hard dependency chain)

---

## Execution Strategies

### Strategy A: Full Sequential (Safe, Slow)

```
Session 1: Task 1 (Expiration research)
Session 2: Task 2 (Code refactoring)
Session 3: Task 3 (Security audit)
Session 4: Task 4 (TODO cleanup) + Task 5 (Test coverage)
```

**Pros**: 
- No conflicts guaranteed
- Clear progression
- Each task benefits from previous results

**Cons**: 
- Slowest timeline (4 sessions)
- No parallelization benefit

---

### Strategy B: Partial Parallel (Recommended) ✅

```
Session 1 (Parallel):
  - Task 1 (Expiration research) → @architect
  - Task 2 (Code refactoring) → @fullstack-dev

Session 2:
  - Task 3 (Security audit) → @qc-specialist → @fullstack-dev
  - Task 4 partial (clean independent TODOs) → @fullstack-dev

Session 3:
  - Task 4 final (handle expiration TODO based on Task 1) → @fullstack-dev
  - Task 5 (Test coverage) → @qa-engineer
```

**Pros**:
- Faster timeline (3 sessions vs 4)
- Task 1 + Task 2 no conflicts (different scopes)
- Task 3 runs after Task 2 (avoids duplicate work)

**Cons**:
- Need to coordinate Session 1 parallel tasks
- Task 4 split across 2 sessions (minor complexity)

**Estimated Timeline**:
- Session 1: 1–2 hours (parallel)
- Session 2: 1–2 hours
- Session 3: 1 hour
- **Total**: ~4–5 hours (vs 6–8 hours sequential)

---

### Strategy C: Aggressive Parallel (Risky)

```
Session 1 (All Parallel):
  - Task 1 + Task 2 + Task 3 + Task 4 (independent parts) + Task 5

Session 2: Integration & cleanup
```

**Pros**: Fastest (2 sessions)

**Cons**:
- Task 2 + Task 3 conflicts (critical)
- Task 5 tests wrong structure
- High risk of duplicate work and conflicts

**Recommendation**: ❌ **Do NOT use** — violates dependency constraints.

---

## Session-Level Execution Plan (Strategy B)

### Session 1 — Parallel Start

**Agent Assignments**:
```
@architect + @explore (Task 1):
  - Explore ~/workspace/xion/contracts/contracts/treasury
  - Explore ~/workspace/xion/xion-developer-portal
  - Analyze expiration necessity
  - Output: Decision report (implement/remove)

@fullstack-dev (Task 2):
  - Split treasury/api_client.rs
  - Create 5-6 modules (fund/grant/withdraw/query/instantiate)
  - Ensure all tests pass
  - Output: Refactored structure
```

**No Conflicts**:
- Task 1 reads external repos (no toolkit code changes)
- Task 2 modifies toolkit code (different scope)
- Both can run simultaneously

---

### Session 2 — Sequential Core

**Agent Assignments**:
```
@qc-specialist → @fullstack-dev (Task 3):
  - Audit unwrap/expect in refactored structure (from Task 2)
  - Replace unsafe occurrences
  - Output: Fixed code + audit report

@fullstack-dev (Task 4 partial):
  - Clean predicted_address TODO (independent)
  - Clean checksum TODO (independent)
  - Clean unused code (independent)
  - Output: Cleaner codebase
  - Note: Expiration TODO waits for Task 1 decision
```

---

### Session 3 — Final Polish

**Agent Assignments**:
```
@fullstack-dev (Task 4 final):
  - If Task 1 decision = remove: delete expiration TODOs
  - If Task 1 decision = implement: create new task (out of scope)
  - Output: All TODOs resolved

@qa-engineer (Task 5):
  - Add tests for Task 3 fixed locations
  - Add boundary tests for validators
  - Output: 10–20% more tests
```

---

## Parallelization Summary

| Can Parallel | Cannot Parallel | Critical Path |
|--------------|-----------------|---------------|
| Task 1 + Task 2 | Task 2 → Task 3 | Task 2 → Task 3 → Task 5 |
| (Session 1) | Task 3 → Task 5 | (Hard dependency chain) |
| | Task 1 → Task 4 (partial) | |

**Best Strategy**: Strategy B (Partial Parallel)
- **Sessions**: 3 sessions (parallel + sequential + final)
- **Timeline**: ~4–5 hours total
- **Agents**: 4 subagents (architect, fullstack-dev, qc-specialist, qa-engineer)
- **Risk**: Low (respects all dependencies)

---

## Current Status (2026-04-03)

### Session-1 Execution

**Architecture**: Combined Task 1 + Task 2 in single session (same branch)

**Branch**: `feature/maintenance-optimization`

**Parallel Strategy**: Same-branch parallel execution (no sub-branch isolation)
- Task 1 (Expiration Research): Research phase (read-only, no code modification)
- Task 2 (Code Refactoring): Active refactoring (splitting api_client.rs)
- PM controls commit ordering to avoid conflicts

**Progress**:
- [x] Task 1 + Task 2 launched in session-1 (parallel)
- [x] Task 2 progress: `fund.rs` (160 lines) + `query.rs` (459 lines) created
- [ ] Task 1 decision locked (pending)
- [ ] Task 2 all modules split (pending: grant/withdraw/instantiate/mod)

**Monitoring**:
- PM checks `git status` periodically to monitor Task 2 progress
- Untracked files visible: `src/treasury/api_client/` directory created

### Task 3 Start Decision

**User Decision**: Wait for Task 2 complete completion (not partial start)

**Rationale**:
- User preference: Complete audit after all modules split
- Safer scope definition: Audit 5-6 modules together
- No incremental audit complexity

**Rejected Alternative**: Partial audit (fund.rs + query.rs early)
- Would require phased audit (audit new modules, then audit remaining)
- User chose simpler complete-then-audit approach

### Prepared Actions (Waiting Phase)

PM prepared while waiting for session-1 completion:
1. [x] Update knowledge document (this file)
2. [ ] Prepare Assignment templates for Task 3, Task 4, Task 5
3. [ ] Monitor Task 2 progress (periodic git status checks)

---

## Notes

- "Session" = one focused agent working session (not calendar time)
- Task 4 can be split: independent parts (Session 2) + expiration-dependent (Session 3)
- Task 5 strictly depends on Task 3 (needs fixed locations)
- Task 2 → Task 3 dependency is **critical** (avoid duplicate audit)
- Current execution: Same-branch parallel (Task 1 + Task 2 in session-1)
- Task 3 start: After Task 2 complete completion (user decision 2026-04-03)