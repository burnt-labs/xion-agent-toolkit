# Contributing to Xion Agent Toolkit

Thank you for your interest in contributing to Xion Agent Toolkit! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples to demonstrate the steps**
- **Describe the behavior you observed and expected**
- **Include screenshots if helpful**
- **Include your environment details** (OS, Rust version, etc.)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Use a clear and descriptive title**
- **Provide a detailed description of the suggested enhancement**
- **Explain why this enhancement would be useful**
- **List some other applications where this enhancement exists**

### Pull Requests

1. **Fork the repo** and create your branch from `main`
2. **Make your changes** following our code standards
3. **Add tests** for any new functionality
4. **Update documentation** if needed
5. **Ensure all tests pass**: `cargo test`
6. **Format your code**: `cargo fmt`
7. **Run clippy**: `cargo clippy`
8. **Submit a pull request**

## Development Setup

### Prerequisites

- Rust 1.75 or higher
- OpenSSL development libraries
- Git

### Building

```bash
git clone https://github.com/burnt-labs/xion-agent-cli
cd xion-agent-cli
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_pkce_verifier_length

# Run tests with output
cargo test -- --nocapture
```

### Code Style

We follow standard Rust conventions:

1. **Use `cargo fmt`** to format your code
2. **Use `cargo clippy`** to catch common mistakes
3. **Follow Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools

**Examples:**
```
feat(oauth): add PKCE challenge generation
fix(treasury): handle API timeout correctly
docs(readme): update installation instructions
test(pkce): add unit tests for verifier generation
```

## Project Structure

```
xion-agent-cli/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── cli/                 # CLI command handlers
│   ├── oauth/               # OAuth2 implementation
│   ├── api/                 # API clients
│   ├── treasury/            # Treasury management
│   ├── config/              # Configuration management
│   └── utils/               # Utilities
├── plans/                   # Development plans
├── docs/                    # Documentation
└── tests/                   # Integration tests
```

## Module Guidelines

### Adding a New Command

1. Create command handler in `src/cli/`
2. Add command enum variant in `src/cli/mod.rs`
3. Update module exports
4. Add tests
5. Update documentation

### Adding a New Module

1. Create module directory in `src/`
2. Add `mod.rs` with public API
3. Update `src/lib.rs` or parent module
4. Add tests in module directory
5. Update documentation

### Error Handling

Use `thiserror` for custom errors:

```rust
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Failed to process: {0}")]
    ProcessingFailed(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
```

Use `anyhow` for error propagation:

```rust
use anyhow::{Context, Result};

pub fn my_function() -> Result<()> {
    let data = load_data()
        .context("Failed to load data")?;
    
    process_data(&data)
        .context("Failed to process data")?;
    
    Ok(())
}
```

### Testing Standards

Write comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_success() {
        // Test successful case
        let result = my_function("input");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_function_failure() {
        // Test failure case
        let result = my_function("invalid");
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_async_function() {
        // Test async function
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Documentation Standards

- **Document all public APIs** with rustdoc comments
- **Include examples** in documentation
- **Update README** for user-facing changes
- **Update CHANGELOG** for all changes

```rust
/// Generates a PKCE challenge for OAuth2 security.
///
/// # Arguments
///
/// * `verifier` - A string slice containing the PKCE verifier
///
/// # Returns
///
/// A `Result` containing the Base64URL-encoded challenge string
///
/// # Example
///
/// ```
/// use xion_agent_cli::oauth::generate_pkce_challenge;
///
/// let verifier = "my_verifier_string";
/// let challenge = generate_pkce_challenge(verifier)?;
/// println!("Challenge: {}", challenge);
/// ```
pub fn generate_pkce_challenge(verifier: &str) -> Result<String> {
    // Implementation
}
```

## Security Considerations

When contributing, please consider:

- **Never log sensitive data** (tokens, credentials, etc.)
- **Use secure storage** for sensitive information
- **Validate all inputs** from external sources
- **Use HTTPS** for all external communications
- **Follow OWASP guidelines** for security

## Getting Help

- **GitHub Issues**: For bug reports and feature requests
- **Code Review**: All PRs are reviewed by maintainers
- **Documentation**: Check `plans/` directory for architecture details

## License

By contributing, you agree that your contributions will be licensed under the MIT License or Apache License 2.0, at the option of the project maintainers.

---

Thank you for contributing to Xion Agent Toolkit! 🎉
