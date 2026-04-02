---
status: InReview
created_at: 2026-03-15
updated_at: 2026-03-15
---

# Unified CI Workflow

## Background

Currently we have three separate GitHub Actions workflows that run in parallel:
- `ci.yml` - Test, Lint, Build, Security Audit
- `test-skills.yml` - Skills mock tests
- `e2e-tests.yml` - End-to-end tests

Problem: `test-skills.yml` and `e2e-tests.yml` download artifacts from `ci.yml`, but there's no guaranteed execution order. They may run before `ci.yml` completes, causing artifact download failures.

## Goal

Merge all workflows into a single unified `ci.yml` with proper job dependencies using `needs` to ensure execution order:

```
test ─────┐
lint ─────┼──► build ──► skills-tests ──► e2e-tests
audit ────┘
```

## Approach

### New Job Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                      ci.yml (unified)                        │
├─────────────────────────────────────────────────────────────┤
│  test ──────┐                                               │
│  lint ──────┼──► build ──► skills-mock-tests ──► ci-status  │
│  audit ─────┘            skills-lint ──────────►            │
│                          e2e-tests ─────────────►           │
└─────────────────────────────────────────────────────────────┘
```

### Job Structure

1. **Base Jobs** (parallel):
   - `test` - Rust unit tests
   - `lint` - fmt + clippy
   - `security-audit` - cargo audit

2. **Build Job** (depends on base jobs):
   - `build` - Build release binary, upload artifact

3. **Skills Tests** (depends on build):
   - `skills-mock-tests` - Run mock tests with downloaded binary
   - `skills-lint` - Shellcheck for skills scripts (can run in parallel with mock-tests)

4. **E2E Tests** (depends on build):
   - `e2e-tests` - Run E2E tests (conditional on secrets)

5. **Status Jobs** (aggregators):
   - `skills-tests-status` - Aggregate skills test results
   - `e2e-status` - Aggregate E2E test results
   - `ci-status` - Final CI status for branch protection

## Tasks

- [x] Rewrite `ci.yml` with unified job structure
- [x] Remove `test-skills.yml`
- [x] Remove `e2e-tests.yml`
- [x] Update `.agents/plans/status.json`

## Acceptance Criteria

- [x] All jobs run in correct order (base → build → downstream tests)
- [x] Build artifact is available to skills-tests and e2e-tests
- [x] No `continue-on-error` needed for artifact download
- [x] Branch protection can use single `ci-status` job
- [x] `workflow_dispatch` available for manual triggering
- [x] Existing test/lint/build/audit functionality preserved

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-15 | @ops-engineer | Unified CI workflow created, old workflows removed | Pending Review |
| 2026-03-15 | @qa-engineer | Validated: job dependencies, YAML syntax, E2E config, artifact handling, workflow_dispatch | **PASS** |
