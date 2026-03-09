# Contributing to Xion Agent Toolkit

Thank you for your interest in contributing! This guide will help you get started.

## Quick Links

- [Development Setup](#development-setup)
- [Code Standards](#code-standards)
- [Pull Request Process](#pull-request-process)
- [Testing Guidelines](#testing-guidelines)
- [Release Process](#release-process)

## Development Setup

### Prerequisites

- Rust 1.75 or higher
- OpenSSL development libraries
- Git

### Getting Started

```bash
# Clone the repository
git clone https://github.com/burnt-labs/xion-agent-toolkit
cd xion-agent-toolkit

# Build
cargo build

# Run tests
cargo test

# Configure OAuth (required for integration testing)
cp .env.example .env
# Edit .env with your OAuth Client IDs (XION_TESTNET_OAUTH_CLIENT_ID, XION_MAINNET_OAUTH_CLIENT_ID)
```

## Code Standards

### Formatting & Linting

```bash
# Format code (REQUIRED before commits)
cargo fmt

# Run clippy (REQUIRED before commits)
cargo clippy --all-targets --all-features -- -D warnings
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/). This is **required** for automated releases.

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types and Version Impact:**

| Type | Version Bump | Description |
|------|--------------|-------------|
| `feat` | Minor (0.2.0 → 0.3.0) | New feature |
| `fix` | Patch (0.2.0 → 0.2.1) | Bug fix |
| `feat!` or `fix!` | Major (0.2.0 → 1.0.0) | Breaking change |
| `docs` | None | Documentation only |
| `style` | None | Code style (formatting, semicolons) |
| `refactor` | None | Code refactoring |
| `perf` | Patch | Performance improvement |
| `test` | None | Adding/updating tests |
| `chore` | None | Maintenance tasks |
| `ci` | None | CI/CD changes |

**Scope (optional):** `oauth`, `treasury`, `cli`, `config`, `api`, etc.

**Examples:**
```
feat(treasury): add batch withdrawal support
fix(auth): handle token refresh edge case
docs(readme): update installation instructions
test(pkce): add unit tests for verifier generation
chore(deps): update reqwest to 0.12
ci(release): add cargo-dist configuration
```

**Breaking Changes:**
```
feat(api)!: change callback server port signature

BREAKING CHANGE: The callback server now requires explicit port configuration.
```

or use `!` after the type:
```
feat!: redesign CLI output format
```

### Error Handling

Use `thiserror` for custom errors:

```rust
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Failed to process: {0}")]
    ProcessingFailed(String),
}
```

Use `anyhow` for error propagation:

```rust
use anyhow::{Context, Result};

pub fn my_function() -> Result<()> {
    let data = load_data().context("Failed to load data")?;
    process_data(&data).context("Failed to process data")?;
    Ok(())
}
```

### CLI Output

All commands must output JSON:

```rust
pub fn output_json<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}
```

## Testing Guidelines

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_pkce_verifier_length

# Run with output
cargo test -- --nocapture
```

### Test Serialization Rules

**CRITICAL**: Tests that modify `XION_CI_ENCRYPTION_KEY` environment variable 
MUST use `#[serial(encryption_key)]` to prevent race conditions in CI.

```rust
// CORRECT
#[test]
#[serial(encryption_key)]
fn test_something() {
    let original = env::var(ENV_KEY_NAME).ok();
    env::set_var(ENV_KEY_NAME, "test_key");
    // ... test code ...
    restore_key(original);
}

// WRONG - Different serial group, allows parallel execution
#[test]
#[serial]  // NOT the same as #[serial(encryption_key)]!
fn test_something_bad() { ... }
```

### CI Environment

In CI, `XION_CI_ENCRYPTION_KEY` is pre-configured. Local development uses 
machine ID for key derivation automatically.

## Pull Request Process

1. **Fork** the repository
2. **Create a branch** from `main`
3. **Make changes** following code standards
4. **Add tests** for new functionality
5. **Run pre-commit checks**:
   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   ```
6. **Update documentation** if needed
7. **Submit pull request**

### PR Requirements

- All tests pass
- No clippy warnings
- Code is formatted
- Documentation updated for user-facing changes
- Commit messages follow conventions

## Project Structure

```
xion-agent-toolkit/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── cli/             # CLI commands
│   ├── oauth/           # OAuth2 implementation
│   ├── api/             # API clients
│   ├── treasury/        # Treasury management
│   ├── config/          # Configuration
│   └── utils/           # Utilities
├── skills/              # Agent Skills
├── plans/               # Development plans & progress
└── tests/               # Integration tests
```

## Adding New Features

### New CLI Command

1. Add command handler in `src/cli/`
2. Add enum variant in `src/cli/mod.rs`
3. Implement logic in appropriate module
4. Add tests
5. Update documentation

### New Module

1. Create module directory in `src/`
2. Add `mod.rs` with public API
3. Update `src/lib.rs` or parent module
4. Add tests
5. Update `AGENTS.md` if needed

## Documentation Standards

- **Document all public APIs** with rustdoc comments
- **Include examples** in documentation
- **Update README.md** for user-facing changes
- **Update AGENTS.md** for development guidelines

```rust
/// Generates a PKCE challenge for OAuth2 security.
///
/// # Arguments
/// * `verifier` - PKCE verifier string
///
/// # Returns
/// Base64URL-encoded challenge string
///
/// # Example
/// ```
/// let challenge = generate_pkce_challenge("verifier")?;
/// ```
pub fn generate_pkce_challenge(verifier: &str) -> Result<String> {
    // Implementation
}
```

## Security Guidelines

- Never log sensitive data (tokens, credentials)
- Use secure storage for sensitive information
- Validate all external inputs
- Use HTTPS for all external communications

## Release Process

We use an **automated release pipeline** combining:
- **[release-please](https://github.com/google-github-actions/release-please-action)**: Generates Release PRs with version bumps and CHANGELOG
- **[cargo-dist](https://axodotdev.github.io/cargo-dist/)**: Builds cross-platform binaries and publishes GitHub Releases

See the full [Release Process Documentation](docs/release.md) for details.

### Automated Release Flow

```
1. Developer commits (conventional commits required)
2. PR merged to main
3. release-please creates/updates Release PR
4. Maintainer reviews and merges Release PR
5. Tag created automatically (v0.X.X)
6. cargo-dist builds and publishes release
```

### For Maintainers: Releasing

1. **Review the Release PR** that release-please creates automatically
2. **Check the CHANGELOG** is accurate
3. **Merge the Release PR** when ready
4. **Done!** The tag triggers cargo-dist to build and publish

### Commit Requirements for Releases

- **All commits** must follow conventional commits format
- `feat:` commits → included in minor version bump
- `fix:` commits → included in patch version bump
- `feat!:` or `BREAKING CHANGE:` → major version bump
- `chore:`, `docs:`, `test:`, etc. → no version bump

### Manual Release (Emergency Only)

If you need to release manually:

```bash
# 1. Update version
# Edit Cargo.toml version

# 2. Update CHANGELOG
# Add new version section

# 3. Commit and tag
git add Cargo.toml CHANGELOG.md
git commit -m "chore(release): prepare for v0.X.X"
git tag -a v0.X.X -m "Release v0.X.X"
git push origin main --tags
```

## Getting Help

- **GitHub Issues**: Bug reports and feature requests
- **Code Review**: All PRs reviewed by maintainers
- **Documentation**: See `plans/` for architecture details
- **Release Process**: See `docs/release.md`

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.

---

Thank you for contributing! 🎉
