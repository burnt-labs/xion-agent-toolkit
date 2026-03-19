---
status: Done
created_at: 2026-03-10
updated_at: 2026-03-10
done_at: 2026-03-10
---
# Documentation Improvement Plan

## Background

The xion-agent-toolkit CLI is designed primarily for AI Agents, not human users. Current documentation lacks:
1. A comprehensive guide for AI Agents to self-install and configure the toolkit
2. Clear usage examples in CLI reference
3. Skills usage documentation

## Goal

Improve documentation to make it easy for AI Agents to:
1. Understand what xion-agent-toolkit does
2. Install the CLI and Skills with a single guide reference
3. Use the toolkit effectively with clear examples

## Approach

### Phase 1: AI Agent Installation Guide (High Priority) ✅

Create `INSTALL-FOR-AGENTS.md` - a single document that an AI Agent can read and execute to:
- Detect the current environment (OS, architecture)
- Install `xion-toolkit` CLI from GitHub releases
- Set up Skills for agent use
- Verify the installation

Key design principles:
- Self-contained: All instructions in one file
- Executable: Agent can follow steps programmatically
- Idempotent: Safe to run multiple times
- Error-recoverable: Clear error messages and recovery steps

### Phase 2: README Enhancement (High Priority) ✅

Update `README.md`:
- Add "Skills for AI Agents" section
- Improve Quick Start with more realistic examples
- Add link to INSTALL-FOR-AGENTS.md
- Reorganize for better flow

### Phase 3: CLI Reference Examples (Medium Priority) ✅

Update `docs/cli-reference.md`:
- Add practical usage examples for each command
- Include JSON output examples
- Add common workflow examples (Complete Treasury Lifecycle, Authentication Flow, Grant and Fee Setup, Contract Deployment)

### Phase 4: Skills Guide (Medium Priority) ✅

Create `docs/skills-guide.md`:
- How to use Skills with AI Agents
- Integration with Claude Code and other frameworks
- Script reference

### Phase 5: Configuration Guide (Low Priority) ✅

Create `docs/configuration.md`:
- `~/.xion-toolkit/` directory structure
- Credential encryption details
- Network configuration

### Phase 6: Skills Publishing (New - High Priority) ✅

Publish skills to skills.sh ecosystem:
- Fix SKILL.md format with YAML frontmatter
- Create `xion-toolkit-init` skill for CLI installation
- Declare dependencies between skills
- Update documentation to recommend skills.sh

## Tasks

- [x] Create INSTALL-FOR-AGENTS.md
- [x] Update README.md with Skills section
- [x] Add usage examples to docs/cli-reference.md
- [x] Create docs/skills-guide.md
- [x] Create docs/configuration.md
- [x] Fix xion-oauth2/SKILL.md format (add YAML frontmatter)
- [x] Fix xion-treasury/SKILL.md format (add YAML frontmatter)
- [x] Create xion-toolkit-init skill with install script
- [x] Update INSTALL-FOR-AGENTS.md to recommend skills.sh
- [x] Update docs/skills-guide.md

## Acceptance Criteria

- [x] AI Agent can install toolkit by reading INSTALL-FOR-AGENTS.md
- [x] Skills are properly documented and accessible
- [x] All CLI commands have usage examples
- [x] Documentation is clear and actionable
- [x] Skills can be installed via `npx skills add burnt-labs/xion-agent-toolkit`

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-10 | @project-manager | Created INSTALL-FOR-AGENTS.md, docs/skills-guide.md | ✅ |
| 2026-03-10 | @project-manager | Created xion-toolkit-init skill | ✅ |
| 2026-03-10 | @project-manager | Fixed SKILL.md formats, updated README | ✅ |
| 2026-03-10 | @project-manager | Added workflow examples to cli-reference.md | ✅ |
| 2026-03-10 | @project-manager | Created docs/configuration.md | ✅ |
| 2026-03-11 | @project-manager | Optimized skill descriptions for better triggering | ✅ |
| 2026-03-11 | @project-manager | Refactored xion-treasury: 1002 lines → 220 lines + references/ | ✅ |
| 2026-03-10 | @project-manager | Optimized skill descriptions (pushy triggers) | ✅ |
| 2026-03-10 | @project-manager | Refactored xion-treasury to references/ pattern | ✅ |
