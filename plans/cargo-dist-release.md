---
status: Done
created_at: 2026-03-09
updated_at: 2026-03-09
---
# Cargo-dist Release Automation

## Background

The project currently lacks automated release tooling. We need to:
1. Standardize the release process with `cargo-dist`
2. Automate cross-platform binary builds
3. Create GitHub releases with checksums
4. Document the CHANGELOG update workflow

## Goal

Establish a fully automated release pipeline that:
- Builds binaries for Linux (x64/ARM64), macOS (x64/ARM64), and Windows (x64)
- Creates GitHub releases with installers and checksums
- Integrates seamlessly with our existing CI/CD
- Documents the release and CHANGELOG workflow

## Approach

### 1. Cargo-dist Configuration

Add `[workspace.metadata.dist]` to `Cargo.toml` with:
- Target platforms: Linux (gnu/musl), macOS, Windows
- CI: GitHub Actions integration
- Installers: shell script, powershell, homebrew (optional)
- Checksums: SHA256

### 2. GitHub Actions Release Workflow

Create `.github/workflows/release.yml` that:
- Triggers on version tags (`v*`)
- Uses cargo-dist for building and publishing
- Creates GitHub release with release notes

### 3. CHANGELOG Workflow

Document the release process:
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` following Keep a Changelog format
3. Commit changes
4. Create and push tag: `git tag v0.X.X && git push --tags`
5. CI handles the rest

### 4. Version Tagging Strategy

- Follow semver: `vMAJOR.MINOR.PATCH`
- Tag format: `v0.2.0`, `v0.3.0`, etc.
- Protected tags: only push from `main` branch

## Tasks

- [x] Initialize cargo-dist config (`cargo dist init`)
- [x] Configure target platforms in `Cargo.toml`
- [x] Generate release workflow (`.github/workflows/release.yml`)
- [x] Add release documentation to `CONTRIBUTING.md` or `docs/release.md`
- [x] Test dry-run of release process

## Acceptance Criteria

- [x] `cargo dist plan` runs successfully
- [x] `cargo dist build` produces expected artifacts locally
- [x] Release workflow triggers on tag push
- [x] GitHub release created with binaries and checksums
- [x] Release process documented

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-09 | @ops | cargo-dist integration complete | ✅ Complete |
