---
status: Done
created_at: 2026-03-09
updated_at: 2026-03-09
done_at: 2026-03-09
---
# Release-please + Cargo-dist Automated Release

## Background

Currently, releases require manual version bumps and CHANGELOG updates. We want to automate the entire release process using:
- **release-please**: Automates version bumps and CHANGELOG generation based on conventional commits
- **cargo-dist**: Builds cross-platform binaries and creates GitHub releases

## Goal

Implement a fully automated release pipeline:
1. Developers commit with conventional commit messages
2. On merge to main, release-please creates/updates a Release PR
3. Merging the Release PR triggers cargo-dist to build and publish

## Approach

### Workflow Overview

```
Developer commits (feat/fix/docs/etc)
              ↓
         PR to main
              ↓
       Merge to main
              ↓
release-please-action runs
              ↓
   Creates/Updates Release PR
   (Cargo.toml + CHANGELOG.md)
              ↓
     Merge Release PR
              ↓
   release-please creates tag
              ↓
   cargo-dist workflow triggered
              ↓
    GitHub Release published
```

### Conventional Commits

Required commit format:
```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: New feature → minor version bump
- `fix`: Bug fix → patch version bump
- `docs`: Documentation only
- `style`: Code style changes
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes
- `breaking!`: Breaking change → major version bump

### Configuration

1. **release-please-config.json**: Main configuration
2. **.release-please-manifest.json**: Version tracking
3. **.github/workflows/release-please.yml**: GitHub Actions workflow

## Tasks

- [x] Create release-please-config.json
- [x] Create .release-please-manifest.json
- [x] Create .github/workflows/release-please.yml
- [x] Update CONTRIBUTING.md with commit conventions
- [x] Update README.md with installation instructions
- [x] Update docs/release.md with new workflow

## Acceptance Criteria

- [x] release-please workflow triggers on merge to main
- [x] Release PR is created with version bump and CHANGELOG
- [x] Merging Release PR creates tag and triggers cargo-dist
- [x] Documentation updated with commit conventions
- [x] README includes installation from releases

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-09 | @project-manager | Full automation complete | ✅ Done |
