# Release Process

This document describes the automated release workflow for the Xion Agent Toolkit.

## Overview

We use a fully automated release pipeline combining:

| Tool | Purpose |
|------|---------|
| **[release-please](https://github.com/google-github-actions/release-please-action)** | Generates Release PRs with version bumps and CHANGELOG |
| **[cargo-dist](https://axodotdev.github.io/cargo-dist/)** | Builds cross-platform binaries and publishes releases |

## Release Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  Developer commits with conventional commits (feat/fix/etc)     │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  PR merged to main                                              │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  release-please-action runs                                     │
│  • Analyzes commits since last release                          │
│  • Creates/updates Release PR with:                             │
│    - Version bump in Cargo.toml                                 │
│    - Updated CHANGELOG.md                                       │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  Maintainer reviews Release PR                                  │
│  • Check CHANGELOG accuracy                                     │
│  • Verify version bump is correct                               │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  Merge Release PR                                               │
│  • release-please creates tag (v0.X.X)                          │
│  • release-please creates GitHub Release (draft)                │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  cargo-dist workflow triggered by tag                           │
│  • Builds binaries for all platforms                            │
│  • Generates installers and checksums                           │
│  • Uploads artifacts to GitHub Release                          │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  ✅ Release Published                                           │
└─────────────────────────────────────────────────────────────────┘
```

## Supported Platforms

Each release includes pre-built binaries for:

| Platform | Target | Archive Format |
|----------|--------|----------------|
| Linux x64 (GNU) | `x86_64-unknown-linux-gnu` | `.tar.xz` |
| Linux x64 (musl) | `x86_64-unknown-linux-musl` | `.tar.xz` |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | `.tar.xz` |
| macOS Intel | `x86_64-apple-darwin` | `.tar.xz` |
| macOS Apple Silicon | `aarch64-apple-darwin` | `.tar.xz` |
| Windows x64 | `x86_64-pc-windows-msvc` | `.zip` |

## Commit Requirements

**All commits** must follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>
```

### Version Bump Rules

| Commit Type | Version Impact | Example |
|-------------|----------------|---------|
| `feat:` | Minor bump (0.2.0 → 0.3.0) | New features |
| `fix:` | Patch bump (0.2.0 → 0.2.1) | Bug fixes |
| `feat!:` or `BREAKING CHANGE:` | Major bump (0.2.0 → 1.0.0) | Breaking changes |
| `docs:`, `chore:`, `test:`, etc. | No bump | Non-functional changes |

### Examples

```bash
# Minor version bump (new feature)
feat(treasury): add batch withdrawal support

# Patch version bump (bug fix)
fix(auth): handle token refresh edge case

# Major version bump (breaking change)
feat(api)!: redesign callback server interface

BREAKING CHANGE: The callback server now requires explicit port.
```

## For Maintainers

### Releasing a New Version

1. **Wait for Release PR** - release-please automatically creates a Release PR when features/fixes are merged

2. **Review the Release PR**:
   - Check CHANGELOG entries are accurate
   - Verify version bump is correct
   - Edit the PR description if needed

3. **Merge the Release PR**:
   - This creates the version tag
   - This triggers cargo-dist to build and publish

4. **Done!** Monitor the [Actions tab](https://github.com/burnt-labs/xion-agent-toolkit/actions) for build status

### Release PR Example

```markdown
## 0.3.0 (2026-03-10)

### Features

* **treasury:** add batch withdrawal support (#123)
* **auth:** add token caching (#120)

### Bug Fixes

* **oauth:** handle PKCE edge case (#125)

### Documentation

* update installation instructions (#122)
```

## Installing from a Release

### Using the Shell Installer (macOS/Linux)

```bash
# Latest version
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh

# Specific version
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/download/v0.3.0/xion-agent-toolkit-installer.sh | sh
```

### Using the PowerShell Installer (Windows)

```powershell
# Latest version
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1 | iex"

# Specific version
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/download/v0.3.0/xion-agent-toolkit-installer.ps1 | iex"
```

### Manual Installation

Download the appropriate archive from the [Releases page](https://github.com/burnt-labs/xion-agent-toolkit/releases) and extract to a directory in your `PATH`.

## Release Artifacts

Each release includes:

| Artifact | Description |
|----------|-------------|
| `xion-agent-toolkit-*.tar.xz` / `.zip` | Platform-specific binaries |
| `xion-agent-toolkit-installer.sh` | Shell installer (macOS/Linux) |
| `xion-agent-toolkit-installer.ps1` | PowerShell installer (Windows) |
| `*.sha256` | SHA256 checksums |
| `source.tar.gz` | Source code archive |

## Troubleshooting

### Release PR Not Created

- Ensure commits follow conventional format
- Check that commits have `feat:` or `fix:` type
- Verify release-please workflow ran successfully

### Build Failed

1. Check the GitHub Actions logs
2. Common issues:
   - Missing dependencies for cross-compilation
   - Network timeouts
   - Platform-specific code errors

### Re-running a Failed Release

1. Delete the GitHub release (if created)
2. Delete the tag:
   ```bash
   git tag -d v0.X.X
   git push --delete origin v0.X.X
   ```
3. Re-run the release-please workflow or create a new commit

## Manual Release (Emergency Only)

If automation fails, you can release manually:

```bash
# 1. Update version
# Edit Cargo.toml version field

# 2. Update CHANGELOG.md
# Add new version section

# 3. Update manifest
# Edit .release-please-manifest.json

# 4. Commit, tag, and push
git add Cargo.toml CHANGELOG.md .release-please-manifest.json
git commit -m "chore(release): v0.X.X"
git tag -a v0.X.X -m "Release v0.X.X"
git push origin main --tags
```

## OAuth Client ID Configuration

The release binaries embed pre-configured OAuth client IDs for both testnet and mainnet. These values are provided at build time via GitHub Actions variables and consumed by `build.rs`.

### Required GitHub Actions variables

In the GitHub repository, configure the following repository **Variables** (not secrets) under:

`Settings → Secrets and variables → Actions → Variables`

- `XION_TESTNET_OAUTH_CLIENT_ID` – OAuth client ID for testnet
- `XION_MAINNET_OAUTH_CLIENT_ID` – OAuth client ID for mainnet

These map directly to the environment variables read in `build.rs`:

- `XION_TESTNET_OAUTH_CLIENT_ID`
- `XION_MAINNET_OAUTH_CLIENT_ID`

The `cargo-dist` workflow (`.github/workflows/release.yml`) uses `github-build-setup` (configured in `dist-workspace.toml`) to inject these variables into the build environment before `dist build` runs. If either variable is missing, the binaries will fall back to placeholder client IDs and OAuth flows will not work correctly in production.

## Configuration Files

| File | Purpose |
|------|---------|
| `release-please-config.json` | release-please configuration |
| `.release-please-manifest.json` | Current version tracking |
| `dist-workspace.toml` | cargo-dist configuration (including GitHub build setup) |
| `.github/workflows/release-please.yml` | Release PR automation |
| `.github/workflows/release.yml` | Binary build automation |

## See Also

- [CONTRIBUTING.md](../CONTRIBUTING.md) - Commit conventions and PR process
- [cargo-dist documentation](https://axodotdev.github.io/cargo-dist/)
- [release-please documentation](https://github.com/google-github-actions/release-please-action)
