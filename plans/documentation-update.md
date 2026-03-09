---
status: Todo
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
- [ ] Add admin management commands to README
- [ ] Add params update command to README
- [ ] Add chain-query commands to README
- [ ] Add instantiate commands to README
- [ ] Update feature list and quick start examples

### Skills Scripts
- [ ] Update `skills/xion-treasury/scripts/update-params.sh` to use new CLI
- [ ] Create `skills/xion-treasury/scripts/admin.sh` for admin operations
- [ ] Create `skills/xion-treasury/scripts/chain-query.sh` for queries
- [ ] Update `skills/xion-treasury/SKILL.md` documentation

### CLI Reference
- [ ] Create `docs/` directory if needed
- [ ] Create `docs/cli-reference.md`
- [ ] Document all auth commands
- [ ] Document all treasury commands (including new ones)
- [ ] Document all config commands
- [ ] Add output format examples

## Acceptance Criteria

- [ ] README reflects all current CLI commands
- [ ] Skills scripts work with current API
- [ ] CLI Reference is comprehensive and accurate
- [ ] All examples are tested and working

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
