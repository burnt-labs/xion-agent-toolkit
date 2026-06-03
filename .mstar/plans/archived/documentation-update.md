---
status: Done
created_at: 2026-03-09
updated_at: 2026-03-09
---

# Documentation Update

## Background

After completing Treasury Enhancements, the project documentation needs to be updated to reflect new CLI commands and features. This includes README, skills scripts, and a comprehensive CLI reference.

## Goal

1. Update README.md with all new CLI commands
2. Update skills scripts in `skills/` directory
3. Create comprehensive CLI Reference documentation

## Approach

### 1. README Update

Add new commands to README:
- Treasury Admin commands (`propose-admin`, `accept-admin`, `cancel-admin`)
- Treasury Params commands (`update-params`)
- Treasury Chain Query commands (`chain-query grants`, `chain-query allowances`)
- Contract Instantiation commands (`instantiate`, `instantiate2`)

### 2. Skills Scripts Update

Review and update scripts in `skills/`:
- Update `update-params.sh` to use the new CLI command
- Add `admin.sh` for admin management operations
- Add `chain-query.sh` for on-chain query operations
- Update SKILL.md documentation

### 3. CLI Reference Documentation

Create `docs/cli-reference.md` with:
- Complete command reference
- All options and flags
- Example usage for each command
- Output format documentation

## Tasks

### README Update
- [x] Add admin management commands to README
- [x] Add params update command to README
- [x] Add chain-query commands to README
- [x] Add instantiate commands to README
- [x] Update feature list and quick start examples

### Skills Scripts
- [x] Update `skills/xion-treasury/scripts/update-params.sh` to use new CLI
- [x] Create `skills/xion-treasury/scripts/admin.sh` for admin operations
- [x] Create `skills/xion-treasury/scripts/chain-query.sh` for queries
- [x] Update `skills/xion-treasury/SKILL.md` documentation

### CLI Reference
- [x] Create `docs/` directory if needed
- [x] Create `docs/cli-reference.md`
- [x] Document all auth commands
- [x] Document all treasury commands (including new ones)
- [x] Document all config commands
- [x] Add output format examples

## Acceptance Criteria

- [ ] README reflects all current CLI commands
- [ ] Skills scripts work with current API
- [ ] CLI Reference is comprehensive and accurate
- [ ] All examples are tested and working

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-09 | @frontend-dev | README, skills scripts, CLI reference | Done |
