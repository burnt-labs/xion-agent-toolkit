# Xion Agent Toolkit - AI Agent Installation Guide

A comprehensive, self-contained installation guide for AI Agents to install and configure the Xion Agent Toolkit.

## Overview

### What is Xion Agent Toolkit?

Xion Agent Toolkit is a CLI-first, agent-oriented toolkit for developing on the Xion blockchain. It provides a gasless development experience through OAuth2 authentication and MetaAccount integration.

### What It Does

- **OAuth2 Authentication**: Browser-based login with PKCE security, no mnemonics required
- **Treasury Management**: Create, query, fund, and withdraw from Treasury contracts
- **Grant Configuration**: Set up Authz grants for delegated authorization
- **Fee Configuration**: Configure fee allowances for gasless transactions
- **Admin Management**: Propose, accept, and cancel admin transfers
- **Contract Operations**: Instantiate and execute CosmWasm contracts

### Who Should Use It

- AI Agents (Claude, GPT, etc.) performing blockchain operations
- Automation scripts and CI/CD pipelines
- Developers building agent-powered applications

## Prerequisites

### Supported Operating Systems

| OS | Architecture | Support |
|----|--------------|---------|
| macOS | x86_64 (Intel) | ✅ Full |
| macOS | ARM64 (Apple Silicon) | ✅ Full |
| Linux | x86_64 | ✅ Full |
| Linux | ARM64 | ✅ Full |
| Windows | x86_64 | ✅ PowerShell installer |

### Required Tools

The following tools must be available in the agent's environment:

```bash
# Required
curl        # For downloading installer
bash        # For running shell scripts (macOS/Linux)
powershell  # For Windows installations

# Optional (for verification)
jq          # JSON parsing for verification scripts
which       # Check if commands exist in PATH
```

### Environment Requirements

- **Network**: Internet access to GitHub and Xion OAuth2 APIs
- **Browser**: Required for OAuth2 login flow (headless browser supported)
- **Disk Space**: ~50MB for CLI binary and configuration files

## Installation

### Step 1: Detect Environment

First, detect the operating system and architecture to determine the correct binary:

```bash
# Detect OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
echo "Operating System: $OS"

# Detect architecture
ARCH=$(uname -m)
echo "Architecture: $ARCH"

# Normalize architecture names
case "$ARCH" in
  x86_64) ARCH_NORMALIZED="x86_64" ;;
  arm64|aarch64) ARCH_NORMALIZED="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac
echo "Normalized Architecture: $ARCH_NORMALIZED"

# Detect if running on macOS
if [[ "$OS" == "darwin" ]]; then
    OS_NORMALIZED="apple-darwin"
elif [[ "$OS" == "linux" ]]; then
    OS_NORMALIZED="unknown-linux-gnu"
else
    echo "Unsupported OS: $OS"
    exit 1
fi
echo "OS Target: $OS_NORMALIZED"
```

### Step 2: Install xion-toolkit CLI

#### Option A: Shell Installer (macOS/Linux - Recommended)

The simplest method using the official installer script:

```bash
# Install using shell installer
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh

# Add to PATH if not automatically added
export PATH="$HOME/.cargo/bin:$PATH"
```

#### Option B: Manual Installation (macOS/Linux)

For more control over the installation process:

```bash
#!/bin/bash
set -e

# Configuration
INSTALL_DIR="${HOME}/.local/bin"
REPO="burnt-labs/xion-agent-toolkit"

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Get latest release info
LATEST_RELEASE=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest")
VERSION=$(echo "$LATEST_RELEASE" | grep -o '"tag_name": "v[^"]*"' | cut -d'"' -f4)
echo "Latest version: $VERSION"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Normalize values
case "$ARCH" in
  x86_64) ARCH_NAME="x86_64" ;;
  arm64|aarch64) ARCH_NAME="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case "$OS" in
  darwin) OS_NAME="apple-darwin" ;;
  linux) OS_NAME="unknown-linux-gnu" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Build download URL
FILENAME="xion-agent-toolkit-${ARCH_NAME}-${OS_NAME}.tar.xz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${FILENAME}"
echo "Downloading: $DOWNLOAD_URL"

# Download and extract
TEMP_DIR=$(mktemp -d)
curl -L "$DOWNLOAD_URL" -o "${TEMP_DIR}/${FILENAME}"
tar -xf "${TEMP_DIR}/${FILENAME}" -C "$TEMP_DIR"

# Move binary to install directory
mv "${TEMP_DIR}/xion-toolkit" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/xion-toolkit"

# Cleanup
rm -rf "$TEMP_DIR"

echo "Installation complete!"
echo "Binary location: $INSTALL_DIR/xion-toolkit"

# Add to PATH if not already present
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "Add to your PATH by adding this line to your shell profile:"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi
```

#### Option C: PowerShell Installer (Windows)

```powershell
# Install using PowerShell installer
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1 | iex"
```

#### Option D: Install from Source (All Platforms)

Requires Rust toolchain (rustc 1.75+):

```bash
# Clone repository
git clone https://github.com/burnt-labs/xion-agent-toolkit
cd xion-agent-toolkit

# Install using cargo
cargo install --path .

# Or install from crates.io
cargo install xion-agent-toolkit
```

### Step 3: Verify CLI Installation

Verify that the CLI is properly installed and functional:

```bash
#!/bin/bash
set -e

echo "=== Verifying xion-toolkit Installation ==="

# Check if binary exists
if command -v xion-toolkit &> /dev/null; then
    echo "✓ xion-toolkit found in PATH"
    XION_PATH=$(which xion-toolkit)
    echo "  Location: $XION_PATH"
else
    echo "✗ xion-toolkit not found in PATH"
    exit 1
fi

# Check version
echo ""
echo "Checking version..."
VERSION=$(xion-toolkit --version)
echo "✓ Version: $VERSION"

# Check help command
echo ""
echo "Checking CLI help..."
if xion-toolkit --help &> /dev/null; then
    echo "✓ CLI responds to --help"
else
    echo "✗ CLI failed to respond to --help"
    exit 1
fi

# List available commands
echo ""
echo "Available command groups:"
xion-toolkit --help | grep -E "^  (auth|treasury|contract|config)" | head -10

echo ""
echo "=== Installation Verification Complete ==="
```

Expected output:
```
=== Verifying xion-toolkit Installation ===
✓ xion-toolkit found in PATH
  Location: /Users/username/.local/bin/xion-toolkit

Checking version...
✓ Version: xion-toolkit 0.1.0

Checking CLI help...
✓ CLI responds to --help

Available command groups:
  auth         OAuth2 authentication commands
  treasury     Treasury management commands
  contract     Contract operations
  config       Configuration commands

=== Installation Verification Complete ===
```

### Step 4: Install Skills (Recommended)

Skills provide agent-friendly wrappers around CLI commands with JSON output and error handling. Use [skills.sh](https://skills.sh) for easy installation.

#### Option A: Install All Skills via skills.sh (Recommended)

The simplest approach - install everything in one command:

```bash
# Install xion-agent-toolkit skills (OAuth2 + Treasury)
npx skills add burnt-labs/xion-agent-toolkit

# Optionally, also install xion-skills for xiond CLI operations
npx skills add burnt-labs/xion-skills
```

**What you get:**

| Package | Skills | Purpose |
|---------|--------|---------|
| `burnt-labs/xion-agent-toolkit` | `xion-dev` | Unified entry point (routes to correct skill) |
| | `xion-toolkit-init` | Install xion-toolkit CLI |
| | `xion-oauth2` | OAuth2 authentication |
| | `xion-treasury` | Treasury management |
| `burnt-labs/xion-skills` (optional) | `xiond-init` | Install xiond CLI |
| | `xiond-usage` | Chain queries, account management |
| | `xiond-wasm` | CosmWasm deployment |

**Dependency Graph:**

```
burnt-labs/xion-agent-toolkit
├── xion-dev (entry point - routes to correct skill)
├── xion-toolkit-init (installs xion-toolkit CLI)
├── xion-oauth2 (requires xion-toolkit)
└── xion-treasury (requires xion-oauth2)

burnt-labs/xion-skills (optional, for advanced operations)
├── xiond-init (installs xiond CLI)
├── xiond-usage (chain queries, requires xiond)
└── xiond-wasm (contract deployment, requires xiond)
```

#### Skills Comparison

| Feature | xion-agent-toolkit | xion-skills |
|---------|-------------------|-------------|
| **Installation** | `npx skills add burnt-labs/xion-agent-toolkit` | `npx skills add burnt-labs/xion-skills` |
| **Target CLI** | xion-toolkit | xiond |
| **Auth Method** | OAuth2 (browser-based, gasless) | Mnemonic / Keyring |
| **Use Case** | MetaAccount, Treasury management | Chain queries, CosmWasm |
| **Skills** | xion-dev, xion-toolkit-init, xion-oauth2, xion-treasury | xiond-init, xiond-usage, xiond-wasm |

**Recommendation**: 
- **Primary**: Install `burnt-labs/xion-agent-toolkit` for MetaAccount development
- **Optional**: Install `burnt-labs/xion-skills` for chain queries and CosmWasm deployment

#### When to Use Which Toolkit

| Use xion-agent-toolkit when... | Use xion-skills when... |
|-------------------------------|-------------------------|
| Building Xion applications | Deploying CosmWasm contracts |
| Managing Treasury contracts | Querying chain data (blocks, txs) |
| Gasless transactions | Checking transaction status |
| OAuth2 authentication | Mnemonic wallet management |
| Authz/Fee grant configuration | Validator operations |

#### Using Installed Skills

After installation, skills are automatically available. Example usage:

```bash
# Using xion-toolkit-init skill (if xion-toolkit not installed)
# The skill will be invoked automatically when needed

# Using xion-oauth2 skill
xion-toolkit auth login

# Using xion-treasury skill
xion-toolkit treasury list
```

### Step 5: Authentication Setup

Authentication is required for most Treasury operations.

#### Quick Authentication Flow

```bash
# Check current auth status
xion-toolkit auth status

# Login (opens browser for OAuth2 authorization)
xion-toolkit auth login
```

#### Authentication with Custom Port

If the default port (54321) is in use:

```bash
xion-toolkit auth login --port 54322
```

#### Network Selection

```bash
# Login to testnet (default)
xion-toolkit auth login --network testnet

# Login to mainnet
xion-toolkit auth login --network mainnet

# Set default network
xion-toolkit config set-network testnet
```

#### Expected Authentication Output

Successful login returns JSON:

```json
{
  "success": true,
  "network": "testnet",
  "authenticated": true,
  "xion_address": "xion1...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

## Quick Verification

Run this complete verification script to test the entire installation:

```bash
#!/bin/bash
set -e

echo "=============================================="
echo "  Xion Agent Toolkit - Full Verification"
echo "=============================================="
echo ""

# Color codes (optional, for terminals that support it)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS_COUNT=0
FAIL_COUNT=0

pass() {
    echo -e "${GREEN}✓${NC} $1"
    ((PASS_COUNT++))
}

fail() {
    echo -e "${RED}✗${NC} $1"
    ((FAIL_COUNT++))
}

warn() {
    echo -e "${YELLOW}!${NC} $1"
}

# 1. Check CLI installation
echo "1. Checking CLI Installation..."
if command -v xion-toolkit &> /dev/null; then
    pass "xion-toolkit found in PATH"
    VERSION=$(xion-toolkit --version 2>&1)
    echo "   Version: $VERSION"
else
    fail "xion-toolkit not found in PATH"
fi

# 2. Check CLI functionality
echo ""
echo "2. Checking CLI Functionality..."
if xion-toolkit --help &> /dev/null; then
    pass "CLI --help works"
else
    fail "CLI --help failed"
fi

if xion-toolkit auth --help &> /dev/null; then
    pass "auth commands available"
else
    fail "auth commands not available"
fi

if xion-toolkit treasury --help &> /dev/null; then
    pass "treasury commands available"
else
    fail "treasury commands not available"
fi

# 3. Check configuration directory
echo ""
echo "3. Checking Configuration..."
CONFIG_DIR="$HOME/.xion-toolkit"
if [[ -d "$CONFIG_DIR" ]]; then
    pass "Configuration directory exists: $CONFIG_DIR"
else
    warn "Configuration directory will be created on first use"
fi

# 4. Check authentication status
echo ""
echo "4. Checking Authentication Status..."
AUTH_STATUS=$(xion-toolkit auth status 2>&1)
if echo "$AUTH_STATUS" | grep -q '"authenticated": true'; then
    pass "User is authenticated"
    XION_ADDR=$(echo "$AUTH_STATUS" | grep -o '"xion_address": "[^"]*"' | cut -d'"' -f4)
    echo "   Address: $XION_ADDR"
else
    warn "User not authenticated (run 'xion-toolkit auth login')"
fi

# 5. Check network connectivity
echo ""
echo "5. Checking Network Connectivity..."
if curl -s --head --request GET "https://oauth2.testnet.burnt.com" &> /dev/null; then
    pass "Can reach testnet OAuth2 API"
else
    fail "Cannot reach testnet OAuth2 API"
fi

# Summary
echo ""
echo "=============================================="
echo "  Verification Summary"
echo "=============================================="
echo -e "  Passed: ${GREEN}$PASS_COUNT${NC}"
echo -e "  Failed: ${RED}$FAIL_COUNT${NC}"
echo ""

if [[ $FAIL_COUNT -eq 0 ]]; then
    echo -e "${GREEN}Installation verified successfully!${NC}"
    exit 0
else
    echo -e "${RED}Some checks failed. Review the output above.${NC}"
    exit 1
fi
```

Save this as `~/.xion-toolkit/verify-installation.sh` and run:

```bash
chmod +x ~/.xion-toolkit/verify-installation.sh
~/.xion-toolkit/verify-installation.sh
```

## Troubleshooting

### Common Issues and Solutions

#### Issue 1: "Command not found: xion-toolkit"

**Symptom**: CLI not found after installation

**Solutions**:

```bash
# Check if binary was installed
ls -la ~/.local/bin/xion-toolkit

# Add to PATH manually
export PATH="$HOME/.local/bin:$PATH"

# Add to shell profile for persistence
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc

# Reload shell profile
source ~/.bashrc  # or source ~/.zshrc
```

#### Issue 2: "Port already in use" during login

**Symptom**: Callback server fails to start on port 54321

**Solutions**:

```bash
# Use a different port
xion-toolkit auth login --port 54322

# Find and kill process using the port (macOS/Linux)
lsof -i :54321
kill <PID>
```

#### Issue 3: "Token expired" errors

**Symptom**: API calls fail with authentication errors

**Solutions**:

```bash
# Refresh token manually
xion-toolkit auth refresh

# Or re-login
xion-toolkit auth login
```

The CLI automatically refreshes tokens, but manual refresh may be needed if:
- The refresh token has expired (30 days)
- Network configuration changed

#### Issue 4: "Network connectivity" errors

**Symptom**: Cannot connect to OAuth2 API

**Solutions**:

```bash
# Check internet connection
curl -I https://oauth2.testnet.burnt.com

# Check firewall settings
# Ensure outbound HTTPS (port 443) is allowed

# Try alternative network
xion-toolkit config set-network mainnet
```

#### Issue 5: "Unsupported architecture" error

**Symptom**: Installation fails on ARM64 or other architectures

**Solutions**:

```bash
# Verify architecture
uname -m

# For ARM64 (Apple Silicon), ensure you're downloading the correct binary
# aarch64-apple-darwin for macOS ARM64
# aarch64-unknown-linux-gnu for Linux ARM64

# If pre-built binary not available, install from source
git clone https://github.com/burnt-labs/xion-agent-toolkit
cd xion-agent-toolkit
cargo install --path .
```

#### Issue 6: Credentials not persisting

**Symptom**: Need to login every time

**Solutions**:

```bash
# Check credentials directory
ls -la ~/.xion-toolkit/credentials/

# Verify encryption key is consistent
# In CI/CD, ensure XION_CI_ENCRYPTION_KEY is set
# Locally, credentials are tied to machine ID

# Do NOT delete .enc files during testing
# They contain refresh tokens valid for 30 days
```

### Getting Help

```bash
# Show CLI help
xion-toolkit --help

# Show command-specific help
xion-toolkit auth login --help
xion-toolkit treasury --help

# Check status
xion-toolkit status

# View configuration
xion-toolkit config show
```

### Recovery Steps

If installation is corrupted:

```bash
#!/bin/bash
set -e

echo "=== Recovery: Clean Reinstall ==="

# Remove existing installation
rm -rf ~/.local/bin/xion-toolkit
rm -rf ~/.xion-toolkit

# Clear any cached downloads
rm -rf /tmp/xion-toolkit-*

# Reinstall
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh

echo ""
echo "Reinstall complete. Run verification:"
echo "  xion-toolkit --version"
echo "  xion-toolkit auth login"
```

## Next Steps

### Documentation

- **[QUICK-REFERENCE.md](./docs/QUICK-REFERENCE.md)** - Condensed CLI reference for AI Agents (recommended)
- **[ERROR-CODES.md](./docs/ERROR-CODES.md)** - Complete error code reference
- [CLI Reference](./docs/cli-reference.md) - Complete command documentation
- [Configuration Guide](./docs/configuration.md) - Configuration details
- [Skills Guide](./docs/skills-guide.md) - Skills usage documentation
- [Contributing](./CONTRIBUTING.md) - Development guidelines

### Quick Start Commands

```bash
# 1. Login
xion-toolkit auth login

# 2. List your treasuries
xion-toolkit treasury list

# 3. Create a new treasury
xion-toolkit treasury create --name "My Treasury" \
  --redirect-url "https://example.com/callback"

# 4. Fund the treasury
xion-toolkit treasury fund <ADDRESS> --amount 1000000

# 5. Configure fee allowance
xion-toolkit treasury fee-config set <ADDRESS> \
  --fee-allowance-type basic \
  --fee-spend-limit "5000000uxion"
```

### Agent Integration

After installing skills with `npx skills add burnt-labs/xion-agent-toolkit`, use the CLI directly:

```bash
# Login
xion-toolkit auth login

# List treasuries
xion-toolkit treasury list

# Query a treasury
xion-toolkit treasury query <ADDRESS>
```

All CLI commands support `--output json` for easy parsing:

```json
{
  "success": true,
  "treasuries": [...],
  "count": 1
}
```

### Example Agent Workflow

```bash
#!/bin/bash
# Example: Agent treasury management workflow

# 1. Check authentication
AUTH_STATUS=$(xion-toolkit auth status --output json)
AUTHENTICATED=$(echo "$AUTH_STATUS" | jq -r '.authenticated')

if [[ "$AUTHENTICATED" != "true" ]]; then
    echo "Not authenticated. Logging in..."
    xion-toolkit auth login
fi

# 2. List treasuries
TREASURIES=$(xion-toolkit treasury list --output json)
echo "$TREASURIES" | jq '.'

# 3. Query first treasury
FIRST_ADDR=$(echo "$TREASURIES" | jq -r '.treasuries[0].address')
if [[ "$FIRST_ADDR" != "null" && "$FIRST_ADDR" != "" ]]; then
    echo "Querying treasury: $FIRST_ADDR"
    xion-toolkit treasury query "$FIRST_ADDR" --output json | jq '.'
fi
```

## Resources

| Resource | URL |
|----------|-----|
| GitHub Repository | https://github.com/burnt-labs/xion-agent-toolkit |
| Latest Releases | https://github.com/burnt-labs/xion-agent-toolkit/releases |
| Xion Documentation | https://docs.burnt.com/xion |
| Developer Portal | https://dev.testnet2.burnt.com |
| Agent Skills Format | https://agentskills.io/ |

## Support

For issues and feature requests:
- **GitHub Issues**: https://github.com/burnt-labs/xion-agent-toolkit/issues
- **Discussions**: https://github.com/burnt-labs/xion-agent-toolkit/discussions

---

*Document Version: 1.1.0*
*Last Updated: 2026-03-11*
*Compatible CLI Version: >=0.1.0*
