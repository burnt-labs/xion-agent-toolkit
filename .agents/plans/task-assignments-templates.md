# Task Assignment Templates — Phase 4 Maintenance

> Pre-prepared Assignment templates for Task 3, Task 4, Task 5.
> Ready to dispatch after session-1 (Task 1 + Task 2) completion.

**Created**: 2026-04-03
**Plan**: `.agents/plans/2026-04-03-maintenance-optimization.md`

---

## Task 3: Security Audit — unwrap/expect Review

### Assignment Template

```markdown
## Assignment

**Primary**: Bug 修复（安全审计）
**Task category**: `deep`（全代码库审计与修复）
**Phase Gate Checklist**:
- Prepare: `specify` ✅ done, `clarify` ✅ done, `plan` ✅ done
- Execute: `plan locked` ✅ done, `tasks` ✅ done, `implement` **this assignment**
- Gate decision: `go` (Task 2 completed)
**Working branch**: `feature/maintenance-optimization`（承接 Task 2 的分支）
**QA note**: full @qa-engineer verification after audit fixes
**QC note**: QC三审并行（@qc-specialist/@qc-specialist-2/@qc-specialist-3）after fix implementation
**Owner Agent**: @qc-specialist → @fullstack-dev
**Delegation**: forbidden（QC 审计 → PM 分派给 @fullstack-dev 修复）
**Why this agent**: QC specialist 专注代码审查与安全审计；发现问题后由 fullstack-dev 执行修复
**Task**: 审计所有 unwrap/expect 使用，识别生产代码中的 unsafe locations，生成修复方案
**Scope**:
- In:
  - 审计所有 363 个 unwrap/expect occurrences（优先 refactored api_client/ 模块）
  - 区分 production code vs test code
  - 生成 audit report：unsafe locations list + fix recommendations
  - **PM 分派**：将 unsafe locations 修复任务分配给 @fullstack-dev
- Out:
  - QC 不直接修改代码（仅审计 + 建议）
  - 不涉及 TODO cleanup（Task 4 负责）
  - 不新增测试（Task 5 负责）
**Inputs**:
- Plan document: `.agents/plans/2026-04-03-maintenance-optimization.md`
- Task 2 result: `src/treasury/api_client/*.rs` (5-6 modules)
- Project style: `AGENTS.md`（error handling 规范）
**Deliverables**:
- Audit report: unsafe unwrap/expect locations with fix recommendations
- Categorized findings: production unsafe / test safe / controlled environment
- Fix recommendations: replace with `?` or `.map_err()` with proper error types
**Acceptance Criteria**:
- [ ] All 363 unwrap/expect occurrences categorized (production/test/environment)
- [ ] Unsafe production locations identified with file paths and line numbers
- [ ] Fix recommendations provided for each unsafe location
- [ ] Audit report structured (severity, location, recommendation)
**Evidence Required**:
- [ ] Audit report with structured findings (markdown or JSON)
- [ ] Unsafe locations count (production code only)
- [ ] Sample fix recommendations for top 5 high-risk locations
**Constraints**:
- Read-only audit (QC does not modify code)
- Use `rg` or grep tools to locate occurrences
- Categorize by context (test code is safe to unwrap)
- **Effort (agent-oriented)**: S-M (1–2 focused agent sessions)
**Plan Path**: `.agents/plans/2026-04-03-maintenance-optimization.md` (Task 3 section)
**Report Format**: Completion Report v2 (include Audit report summary)
**Superpowers**: `systematic-debugging`（系统化审计流程），`verification-before-completion`（报告须有结构化证据）
```

### Fix Implementation Assignment (after QC audit)

```markdown
## Assignment

**Primary**: Bug 修复
**Task category**: `deep`（安全修复）
**Phase Gate Checklist**:
- Prepare: `specify` ✅ done, `clarify` ✅ done, `plan` ✅ done
- Execute: `plan locked` ✅ done, `tasks` ✅ done, `implement` **this assignment**
- Gate decision: `go` (QC audit complete)
**Working branch**: `feature/maintenance-optimization`
**QA note**: full @qa-engineer verification after fixes
**QC note**: QC三审并行（@qc-specialist/@qc-specialist-2/@qc-specialist-3）review fix quality
**Owner Agent**: @fullstack-dev
**Delegation**: forbidden
**Why this agent**: 修复需要修改业务代码，fullstack-dev 负责实现
**Task**: 替换 QC 审计发现的所有 unsafe unwrap/expect 为 proper error handling
**Scope**:
- In:
  - Replace all unsafe unwrap/expect in production code
  - Use `?` or `.map_err()` with proper error types
  - Ensure error messages are meaningful (use `thiserror` codes)
  - Document safe unwrap locations (tests, controlled env)
- Out:
  - Do not change test unwrap (document as safe)
  - Do not add new tests (Task 5 responsibility)
  - Do not modify TODO comments (Task 4 responsibility)
**Inputs**:
- QC audit report: unsafe locations list
- Error types: `src/shared/error.rs` (thiserror definitions)
**Deliverables**:
- All unsafe unwrap replaced with proper error handling
- All tests passing (cargo test --all-features)
- No clippy warnings (cargo clippy -- -D warnings)
**Acceptance Criteria**:
- [ ] All unsafe unwrap from QC report replaced
- [ ] Error handling uses existing error types (E{MODULE}{NUMBER})
- [ ] cargo test --all-features passes
- [ ] cargo clippy -- -D warnings passes
**Evidence Required**:
- [ ] cargo test output (tests passed count)
- [ ] cargo clippy output (zero warnings)
- [ ] Diff showing replaced locations (sample 5 files)
**Constraints**:
- Use `crate::shared::error` types
- Add `.context()` for clarity with `anyhow::Result`
- Preserve test unwrap (document as safe in comments)
- **Effort (agent-oriented)**: S-M (1–2 focused agent sessions)
**Plan Path**: `.agents/plans/2026-04-03-maintenance-optimization.md` (Task 3 implementation)
**Report Format**: Completion Report v2
**Superpowers**: `verification-before-completion`（tests/clippy output required），`systematic-debugging`（fix verification）
```

---

## Task 4: TODO & Unused Code Cleanup

### Assignment Template (depends on Task 1 decision)

**Case A: Task 1 decision = "Remove expiration TODO"**

```markdown
## Assignment

**Primary**: 小功能/改进
**Task category**: `quick`（清理 TODO 与 unused code）
**Phase Gate Checklist**:
- Prepare: `specify` ✅ done, `clarify` ✅ done, `plan` ✅ done
- Execute: `plan locked` ✅ done, `tasks` ✅ done, `implement` **this assignment**
- Gate decision: `go` (Task 1 decision = remove)
**Working branch**: `feature/maintenance-optimization`
**QA note**: QA: self-check only — quick cleanup, no complex logic
**QC note**: QC: skipped — quick cleanup (optional single review by PM)
**Owner Agent**: @fullstack-dev
**Delegation**: forbidden
**Why this agent**: 清理工作简单直接，fullstack-dev 可快速完成
**Task**: 移除不必要的 TODO 注释，清理 unused code
**Scope**:
- In:
  - Remove expiration TODOs (`treasury/manager.rs:1939`, `treasury/api_client.rs:1659`)
  - Remove predicted_address TODO (`treasury/manager.rs:848`)
  - Remove checksum TODO (`asset_builder/code_ids.rs:61`)
  - Clean unused code in 7 files (plan list)
  - Mark intentionally reserved code with `#[allow(dead_code)]`
- Out:
  - No feature implementation (Task 1 decision = remove)
  - No test changes
**Inputs**:
- Task 1 decision: Remove expiration support (rationale documented)
- Plan: `.agents/plans/2026-04-03-maintenance-optimization.md` (Task 4 section)
**Deliverables**:
- All TODOs removed (0 remaining or documented as intentional)
- Unused code cleaned or marked
- Code compiles without warnings
**Acceptance Criteria**:
- [ ] TODO count reduced to 0 (grep confirms)
- [ ] Unused code removed or `#[allow(dead_code)]` added
- [ ] cargo clippy -- -D warnings passes
**Evidence Required**:
- [ ] grep TODO result (0 matches or intentional list)
- [ ] cargo clippy output (zero warnings)
- [ ] Commit message: "chore: remove unnecessary TODOs and clean unused code"
**Constraints**:
- Quick cleanup (XS-S effort)
- **Effort (agent-oriented)**: XS-S (1 focused agent session)
**Plan Path**: `.agents/plans/2026-04-03-maintenance-optimization.md` (Task 4)
**Report Format**: Completion Report v2
**Superpowers**: `verification-before-completion`（grep/clippy evidence）
```

**Case B: Task 1 decision = "Implement expiration feature"**

```markdown
## Assignment

**Primary**: 中型功能（新特性实现）
**Task category**: `deep`（实现 expiration support）
**Phase Gate Checklist**:
- Prepare: `specify` ✅ done, `clarify` ✅ done, `plan` **blocked** (needs spec)
- Execute: `plan locked` **blocked**, `tasks` **blocked**, `implement` **blocked**
- Gate decision: `blocked` (Task 1 decision = implement → new plan required)
**Owner Agent**: @architect (design spec) → @fullstack-dev (implementation)
**Delegation**: forbidden
**Why this agent**: 需要先由 architect 设计 expiration 实现方案，再由 fullstack-dev 实现
**Task**: 设计并实现 FeeConfig expiration support
**Scope**:
- In:
  - Architect: Design spec for expiration field in FeeConfig
  - Fullstack-dev: Implement expiration support in treasury/types.rs + api_client
  - Update CLI to support expiration input
  - Add tests for expiration logic
- Out:
  - No impact on existing grant logic (backward compatible)
**Inputs**:
- Task 1 decision: Implement expiration support (rationale documented)
- Treasury contract implementation (reference)
**Deliverables**:
- Design spec: expiration field structure, validation, storage
- Implementation: FeeConfig with expiration field
- Tests: expiration validation tests
**Acceptance Criteria**:
- [ ] Design spec approved by PM
- [ ] FeeConfig struct updated with expiration field
- [ ] CLI supports expiration input (--expiration flag)
- [ ] Tests added for expiration validation
- [ ] cargo test passes, cargo clippy passes
**Evidence Required**:
- [ ] Design spec document
- [ ] Implementation diff (types.rs + api_client changes)
- [ ] Test output (new tests passing)
**Constraints**:
- **Effort (agent-oriented)**: M (2–3 focused agent sessions for full implementation)
**Plan Path**: New plan required (out of Phase 4 scope → new feature plan)
**Report Format**: Completion Report v2
**Superpowers**: `brainstorming`（设计阶段），`writing-plans`（新 feature plan），`test-driven-development`（实现前写测试）
```

---

## Task 5: Test Coverage Enhancement

### Assignment Template

```markdown
## Assignment

**Primary**: QA 专项
**Task category**: `deep`（测试覆盖率提升）
**Phase Gate Checklist**:
- Prepare: `specify` ✅ done, `clarify` ✅ done, `plan` ✅ done
- Execute: `plan locked` ✅ done, `tasks` ✅ done, `implement` **this assignment**
- Gate decision: `go` (Task 3 fixes complete)
**Working branch**: `feature/maintenance-optimization`
**QA note**: QA mode: self-check only (test addition, no business code changes)
**QC note**: QC: skipped — test-only changes (optional review by PM)
**Owner Agent**: @qa-engineer
**Delegation**: forbidden
**Why this agent**: QA 专注测试质量，qa-engineer 负责测试覆盖率提升
**Task**: 为 Task 3 修复的 error paths 添加测试，增加 boundary tests
**Scope**:
- In:
  - Add tests for Task 3 fixed locations (error paths)
  - Add boundary tests for validators (address, amount, hash)
  - Increase test coverage in large modules (treasury, asset_builder)
  - Document test patterns (best practices)
- Out:
  - No business code changes
  - No test changes for passing tests (only add new)
**Inputs**:
- Task 3 audit report: fixed locations list
- Task 3 implementation: error handling changes
- Current test count: 561 tests
**Deliverables**:
- New unit tests added (10–20% increase = 56–112 new tests)
- Test coverage report (summary of coverage increase)
- Test pattern documentation
**Acceptance Criteria**:
- [ ] Error path tests added for Task 3 locations
- [ ] Boundary tests for validators (address, amount, hash)
- [ ] Test count increased by 10–20% (561 → 617+ tests)
- [ ] All tests passing (cargo test --all-features)
- [ ] Test patterns documented
**Evidence Required**:
- [ ] cargo test output (new test count, all passing)
- [ ] Coverage increase summary (before/after)
- [ ] New test files list
**Constraints**:
- Use `#[test]` for unit tests, `#[tokio::test]` for async
- Use `#[serial(encryption_key)]` for env var mutation tests
- Follow existing test patterns in project
- **Effort (agent-oriented)**: S-M (1–2 focused agent sessions)
**Plan Path**: `.agents/plans/2026-04-03-maintenance-optimization.md` (Task 5 section)
**Report Format**: Completion Report v2
**Superpowers**: `test-driven-development`（test-first mindset），`verification-before-completion`（test count evidence）
```

---

## Usage Instructions

### When to Use These Templates

1. **Task 3**: After Task 2 completion (all modules split)
   - First dispatch QC audit Assignment
   - After QC report → dispatch Fix Implementation Assignment

2. **Task 4**: After Task 1 decision locked
   - If decision = remove → use Case A template
   - If decision = implement → use Case B template (requires new plan)

3. **Task 5**: After Task 3 fixes complete
   - Dispatch QA Assignment with Task 3 fixed locations list

### How to Dispatch

1. Copy template content
2. Fill in missing fields (e.g., current git status, Task 1 decision result)
3. Update `Phase Gate Checklist` markers based on current state
4. Add specific file paths and line numbers from prior task results
5. Dispatch via `Task` tool to appropriate subagent

### Integration with Plan

All assignments reference:
- Plan document: `.agents/plans/2026-04-03-maintenance-optimization.md`
- Status updates should be recorded in plan file
- Completion should update `status.json`

---

## QC三审 + QA Verification (after all tasks)

### QC三审并行 Assignment

```markdown
## Assignment (QC三审组)

**Primary**: 代码审查
**Task category**: `docs`（审查报告）
**Phase Gate Checklist**:
- Prepare: `specify` ✅, `clarify` ✅, `plan` ✅
- Execute: `plan locked` ✅, `tasks` ✅, `implement` **review**
- Gate decision: `go` (all tasks implemented)
**Working branch**: `feature/maintenance-optimization` (review only, no write)
**QA note**: QA: skipped — QC review only
**QC note**: QC三审并行（@qc-specialist/@qc-specialist-2/@qc-specialist-3）
**Owner Agents**: @qc-specialist, @qc-specialist-2, @qc-specialist-3 (parallel review)
**Delegation**: forbidden
**Why this agent**: QC 三审组负责最终质量门禁，并行审查加快收敛
**Task**: 审查 Phase 4 所有实现（Task 2 refactoring + Task 3 fixes + Task 4 cleanup + Task 5 tests）
**Scope**:
- In:
  - Review Task 2: api_client/ refactoring quality (module boundaries, API preservation)
  - Review Task 3: unwrap replacement correctness (error handling, messages)
  - Review Task 4: TODO cleanup completeness (no missing TODOs)
  - Review Task 5: test coverage adequacy (error paths covered, boundary tests)
- Out:
  - No code modification (QC read-only)
  - No new tests (QA responsibility)
**Inputs**:
- All implementation diffs from Task 2, 3, 4, 5
- Plan: `.agents/plans/2026-04-03-maintenance-optimization.md`
- Review harness: `~/.config/opencode/docs/agents/review-harness.md`
**Deliverables**:
- Three independent QC reports (structured findings)
- PM consolidates findings → single gate decision
**Acceptance Criteria**:
- [ ] Each QC reviews all 4 task implementations
- [ ] Findings categorized (Critical/Warning/Suggestion)
- [ ] No critical findings remaining (all must be fixed)
- [ ] PM consolidated decision: Approve | Request Changes | Needs Discussion
**Evidence Required**:
- [ ] Three QC reports (markdown files)
- [ ] PM consolidated decision summary
- [ ] If Request Changes: fix assignments to appropriate dev
**Constraints**:
- Use review-harness.md checklist
- Focus on: API preservation, error handling, test adequacy
- Parallel execution (3 QC agents work independently)
- **Effort (agent-oriented)**: S (1 focused agent session per QC)
**Plan Path**: `.agents/plans/2026-04-03-maintenance-optimization.md`
**Report Format**: QC Report (per review-harness.md template)
**Superpowers**: `verification-before-completion`（evidence required for conclusions）
```

### QA Verification Assignment

```markdown
## Assignment

**Primary**: QA 验证
**Task category**: `deep`（功能验证）
**Phase Gate Checklist**:
- Prepare: `specify` ✅, `clarify` ✅, `plan` ✅
- Execute: `plan locked` ✅, `tasks` ✅, `implement` **verification**
- Gate decision: `go` (QC三审 passed)
**Working branch**: `feature/maintenance-optimization`
**QA note**: full @qa-engineer verification
**QC note**: QC: skipped — QA verification
**Owner Agent**: @qa-engineer
**Delegation**: forbidden
**Why this agent**: QA 负责最终验收验证，确保所有功能正常
**Task**: 验证 Phase 4 所有实现：运行完整测试套件，确认无 regression
**Scope**:
- In:
  - Run cargo test --all-features (561+ tests)
  - Run cargo clippy --all-targets --all-features -- -D warnings
  - Verify E2E bash tests (11 scripts)
  - Verify skill scripts (22 scripts)
  - Check for regressions (compare baseline behavior)
- Out:
  - No code modification
  - No new tests (verification only)
**Inputs**:
- All implementation from Task 2, 3, 4, 5
- QC consolidated decision (Approve)
**Deliverables**:
- Test verification report (all tests passing)
- No regression evidence (behavior preserved)
- Sign-off recommendation (ready for merge)
**Acceptance Criteria**:
- [ ] cargo test --all-features passes (561+ tests)
- [ ] cargo clippy passes (zero warnings)
- [ ] E2E tests pass (11 scripts)
- [ ] Skill tests pass (22 scripts)
- [ ] No regressions detected (manual/automated checks)
**Evidence Required**:
- [ ] cargo test output (tests passed count)
- [ ] cargo clippy output (zero warnings)
- [ ] E2E test execution summary
- [ ] Skill test execution summary
- [ ] Regression check results
**Constraints**:
- Full verification (no shortcuts)
- Compare with baseline (pre-Phase 4 behavior)
- Document any behavior changes (intentional vs regression)
- **Effort (agent-oriented)**: S (1 focused agent session)
**Plan Path**: `.agents/plans/2026-04-03-maintenance-optimization.md`
**Report Format**: Completion Report v2 (include all evidence)
**Superpowers**: `verification-before-completion`（all verification evidence required）
```

---

## Summary

All Assignment templates are pre-prepared and ready for:
- **Task 3**: QC audit + fullstack-dev fix implementation
- **Task 4**: Case A (remove) or Case B (implement) based on Task 1 decision
- **Task 5**: QA test coverage enhancement
- **QC三审**: Parallel review of all implementations
- **QA Verification**: Final acceptance testing

Templates minimize dispatch overhead once session-1 completes Task 1 + Task 2.