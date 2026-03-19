---
status: Done
created_at: 2026-03-19
updated_at: 2026-03-19
---

# Skill Patterns Optimization

## Background

Based on Google Cloud Tech's article on Agent Skills design patterns, we need to optimize our skills following industry best practices. The article identifies 5 key patterns:
- Tool Wrapper: Make agent an instant expert on any library
- Generator: Produce structured documents from reusable template
- Reviewer: Score code against checklist by severity
- Inversion: Agent interviews you before acting
- Pipeline: Enforce strict multi-step workflow with checkpoints

Additionally, users requested:
1. Navigation skills should mention re-installation for updates
2. Auth skills should prioritize refresh over re-login
3. CLI auth login should add `--force` flag for forcing re-authentication

## Goal

1. Apply design patterns to existing skills
2. Add skill update guidance to navigation skills (xion-dev, xion-toolkit-init)
3. Emphasize refresh-first in xion-oauth2 skill
4. Add `--force` flag to `auth login` CLI command

## Approach

### Phase 1: Skill Documentation Updates

1. **xion-dev (Navigation Skill)**
   - Add "Keeping Skills Updated" section
   - Explain re-installation for new features/fixes

2. **xion-toolkit-init (Navigation Skill)**
   - Add similar update guidance

3. **xion-oauth2 (Auth Skill)**
   - Add prominent "Refresh-First" section
   - Emphasize using `auth refresh` before `auth login`
   - Update workflow examples

### Phase 2: CLI Enhancement

1. **auth login command**
   - Add `--force` flag
   - Default behavior: check for refresh token first
   - If refresh token exists and valid, use refresh
   - If `--force` provided, always open browser for new auth

## Tasks

- [x] Update xion-dev SKILL.md with update guidance
- [x] Update xion-toolkit-init SKILL.md with update guidance
- [x] Update xion-oauth2 SKILL.md emphasizing refresh-first (v1.2.0)
- [x] Update auth.rs CLI with --force flag logic
- [x] Update login.json schema with force parameter
- [x] Run tests to verify no regression (48 passed, clippy clean)
- [x] QC Fixes: login.sh --force support (F-001)
- [x] QC Fixes: auth.rs error handling (CRIT-001)
- [x] QC Fixes: JSON consistency refreshed: false (WARN-002)
- [x] QC Fixes: SKILL.md parameter table and output examples

## Acceptance Criteria

- [x] xion-dev has clear update instructions
- [x] xion-toolkit-init has clear update instructions
- [x] xion-oauth2 prominently shows refresh-first approach
- [x] `xion auth login` checks for refresh token before browser auth
- [x] `xion auth login --force` always triggers browser auth
- [x] All existing tests pass (48 passed)
- [x] QC critical issues resolved

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-19 | @qa-engineer | QA Sign-off: All 8 acceptance criteria verified | ✅ Signed-off |
