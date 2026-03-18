# Xion Agent Toolkit - AI Agent Installation Guide

> **Purpose**: This document enables AI Agents to autonomously install, configure, and use the Xion Agent Toolkit for gasless blockchain operations.

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
- **Asset Builder**: Deploy and mint CW721 NFT collections
- **Faucet Operations**: Claim testnet XION tokens programmatically

### Who Should Use It

- AI Agents (Claude, GPT, etc.) performing blockchain operations
- Automation scripts and CI/CD pipelines
- Developers building agent-powered applications

---

## Prerequisites

### Supported Operating Systems

| OS | Architecture | Installer |
|----|--------------|-----------|
| macOS | x86_64 (Intel) | Shell script |
| macOS | ARM64 (Apple Silicon) | Shell script |
| Linux | x86_64 | Shell script |
| Linux | ARM64 | Shell script |
| Windows | x86_64 | PowerShell script |

### Required Tools

```bash
# Required
curl        # For downloading installer
bash        # For running shell scripts (macOS/Linux)
powershell  # For Windows installations

# Optional (for verification)
jq          # JSON parsing
which       # Check if commands exist in PATH
```

### Environment Requirements

- **Network**: Internet access to GitHub and Xion OAuth2 APIs
- **Browser**: Required for OAuth2 login flow (headless browser supported)
- **Disk Space**: ~50MB for CLI binary and configuration files

---

## Installation

### Step 1: Detect Environment

Before installation, detect the operating system and architecture:

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

```bash
# Install using shell installer
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh

# Add to PATH if not automatically added
export PATH="$HOME/.cargo/bin:$PATH"

# For persistence, add to shell profile
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
```

#### Option B: Manual Installation (macOS/Linux)

```bash
#!/bin/bash
set -e

# Configuration
INSTALL_DIR="${HOME}/.local/bin"
REPO="burnt-labs/xion-agent-toolkit"

# Create install directory
mkdir -p "$INSTALL_DIR"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

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
    echo "Add to your PATH: export PATH=\"$INSTALL_DIR:\$PATH\""
fi
```

#### Option C: PowerShell Installer (Windows)

```powershell
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
```

### Step 3: Verify CLI Installation

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
xion-toolkit --help | grep -E "^  (auth|treasury|contract|asset|batch|account|config)" | head -10

echo ""
echo "=== Installation Verification Complete ==="
```

**Expected output:**
```
=== Verifying xion-toolkit Installation ===
✓ xion-toolkit found in PATH
  Location: /Users/username/.local/bin/xion-toolkit

Checking version...
✓ Version: xion-toolkit 0.6.0

Checking CLI help...
✓ CLI responds to --help

Available command groups:
  auth         OAuth2 authentication commands
  treasury     Treasury management commands
  contract     Contract operations
  asset        Asset (NFT) operations
  batch        Batch transaction operations
  account      Account information commands
  config       Configuration commands

=== Installation Verification Complete ===
```

### Step 4: Install Skills (Recommended)

Skills provide agent-friendly wrappers around CLI commands with JSON output and error handling.

```bash
# Install xion-agent-toolkit skills (global, all common agents: Cursor, Claude Code, Codex, OpenClaw)
npx skills add burnt-labs/xion-agent-toolkit -g -y -a cursor -a claude-code -a codex -a openclaw

# Optional: Install xion-skills for xiond CLI operations
npx skills add burnt-labs/xion-skills -g -y -a cursor -a claude-code -a codex -a openclaw
```

- **`-g`**: Install to global directory (`~/.agents/skills/`) so skills are available in all projects.
- **`-y`**: Non-interactive; skip confirmation prompts (suitable for scripts and agents).
- **`-a cursor -a claude-code -a codex -a openclaw`**: Create symlinks for these agents so each tool (Cursor, Claude Code, Codex, OpenClaw) can load the skills from its own directory. You can omit agents you do not use.

**Available Skills:**

| Package | Skill | Purpose |
|---------|-------|---------|
| `burnt-labs/xion-agent-toolkit` | `xion-dev` | Unified entry point - routes to correct skill |
| | `xion-toolkit-init` | Install xion-toolkit CLI |
| | `xion-oauth2` | OAuth2 authentication |
| | `xion-treasury` | Treasury management |
| | `xion-asset` | NFT operations |
| | `xion-faucet` | Claim testnet tokens |
| `burnt-labs/xion-skills` | `xiond-init` | Install xiond CLI |
| (optional) | `xiond-usage` | Chain queries, account management |
| | `xiond-wasm` | CosmWasm deployment |

**When to Use Which:**

| Use xion-agent-toolkit when... | Use xion-skills when... |
|-------------------------------|-------------------------|
| Building Xion applications | Deploying CosmWasm contracts |
| Managing Treasury contracts | Querying chain data (blocks, txs) |
| Gasless transactions | Checking transaction status |
| OAuth2 authentication | Mnemonic wallet operations |
| Authz/Fee grant configuration | Validator operations |
| Claiming testnet tokens | - |

---

## Authentication

### OAuth2 Flow Overview

Xion Agent Toolkit uses OAuth2 with PKCE for secure, gasless authentication:

1. Agent runs: `xion-toolkit auth login`
2. CLI starts localhost callback server (default port 54321)
3. CLI opens browser for user authorization
4. User authorizes in browser
5. Browser redirects to localhost with auth code
6. CLI exchanges code for access + refresh tokens
7. Tokens stored encrypted in `~/.xion-toolkit/credentials/`

### Authentication Commands

```bash
# Login (opens browser for user authorization)
xion-toolkit auth login

# Check authentication status
xion-toolkit auth status

# Refresh access token
xion-toolkit auth refresh

# Logout (clears stored credentials)
xion-toolkit auth logout
```

### Expected Outputs

**Login Success:**
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

**Status (Authenticated):**
```json
{
  "success": true,
  "authenticated": true,
  "xion_address": "xion1...",
  "network": "testnet",
  "expires_at": "2024-01-15T12:00:00Z"
}
```

**Status (Not Authenticated):**
```json
{
  "success": true,
  "authenticated": false
}
```

### Token Lifecycle

| Token Type | Validity | Storage |
|------------|----------|---------|
| Access Token | ~1 hour | `~/.xion-toolkit/credentials/{network}.enc` |
| Refresh Token | 30 days | `~/.xion-toolkit/credentials/{network}.enc` |

**Important:**
- Access tokens auto-refresh when expired
- Do NOT delete `.enc` files during testing
- Only run `auth logout` when explicitly requested

### Custom Port (If Default In Use)

```bash
xion-toolkit auth login --port 54322
```

---

## Network Configuration

### Available Networks

| Network | OAuth API | RPC | Chain ID |
|---------|-----------|-----|----------|
| testnet | `https://oauth2.testnet.burnt.com` | `https://rpc.xion-testnet-2.burnt.com:443` | `xion-testnet-2` |
| mainnet | `https://oauth2.burnt.com` | `https://rpc.xion-mainnet-1.burnt.com:443` | `xion-mainnet-1` |

### Network Commands

```bash
# Set default network
xion-toolkit config set-network testnet
xion-toolkit config set-network mainnet

# Override for single command
xion-toolkit --network mainnet auth status

# Show current config
xion-toolkit config show
```

---

## Quick Reference

### Authentication

| Command | Purpose |
|---------|---------|
| `xion-toolkit auth login` | OAuth2 login (opens browser) |
| `xion-toolkit auth status` | Check authentication status |
| `xion-toolkit auth refresh` | Refresh access token |
| `xion-toolkit auth logout` | Clear stored credentials |

### Treasury

| Command | Purpose |
|---------|---------|
| `xion-toolkit treasury list` | List all treasuries |
| `xion-toolkit treasury query <ADDR>` | Query treasury details |
| `xion-toolkit treasury create --name "..." --redirect-url "..."` | Create treasury |
| `xion-toolkit treasury fund <ADDR> --amount 1000000uxion` | Fund treasury |
| `xion-toolkit treasury withdraw <ADDR> --amount 500000uxion` | Withdraw funds |
| `xion-toolkit treasury grant-config add <ADDR> ...` | Add grant config |
| `xion-toolkit treasury fee-config set <ADDR> ...` | Set fee config |
| `xion-toolkit treasury export <ADDR>` | Export configuration |
| `xion-toolkit treasury import <ADDR> --from-file config.json` | Import configuration |

### Asset (NFT)

| Command | Purpose |
|---------|---------|
| `xion-toolkit asset types` | List available NFT types |
| `xion-toolkit asset create --type cw721-base --name "..." --symbol "..."` | Create collection |
| `xion-toolkit asset mint --contract <ADDR> --token-id "1" --owner <ADDR>` | Mint NFT |
| `xion-toolkit asset predict --type cw721-base --name "..." --symbol "..." --salt "..."` | Predict address |

### Faucet

| Command | Purpose |
|---------|---------|
| `xion-toolkit faucet claim` | Claim testnet tokens for yourself |
| `xion-toolkit faucet claim --receiver xion1...` | Claim tokens for another address |
| `xion-toolkit faucet status` | Check claim cooldown status |
| `xion-toolkit faucet info` | Query faucet configuration |

**Note:** 1 XION per claim, 24-hour cooldown, testnet only.

### Batch

| Command | Purpose |
|---------|---------|
| `xion-toolkit batch execute --from-file batch.json` | Execute batch operations |

---

## Output Format

All commands return JSON:

**Success:**
```json
{"success": true, ...data}
```

**Error:**
```json
{"success": false, "error": "...", "code": "...", "suggestion": "..."}
```

---

## Error Handling

### Common Error Codes

| Code | Meaning | Action |
|------|---------|--------|
| `NOT_AUTHENTICATED` | Not logged in | Run `auth login` |
| `TOKEN_EXPIRED` | Token expired | Run `auth refresh` |
| `AUTH_LOGIN_FAILED` | Login failed | Retry, check browser |
| `TREASURY_NOT_FOUND` | Invalid address | Check address, verify network |
| `INSUFFICIENT_BALANCE` | Not enough funds | Fund the account |
| `INVALID_ADDRESS` | Bad address format | Use valid bech32 address (xion1...) |
| `PORT_IN_USE` | Callback port busy | Use `--port` with different port |
| `CLI_NOT_FOUND` | CLI not installed | Run installer |
| `NETWORK_ERROR` | Connection failed | Check internet, retry |
| `INVALID_INPUT` | Invalid parameters | Check command syntax |

> See [docs/ERROR-CODES.md](./docs/ERROR-CODES.md) for complete error reference.

### Error Handling Strategy for Agents

```python
# Pseudocode for agent error handling
def run_command(cmd):
    result = execute(cmd)
    data = parse_json(result.stdout)
    
    if data.get("success"):
        return data
    
    error_code = data.get("code")
    
    if error_code == "NOT_AUTHENTICATED":
        execute("xion-toolkit auth login")
        return run_command(cmd)  # Retry
    
    elif error_code == "TOKEN_EXPIRED":
        execute("xion-toolkit auth refresh")
        return run_command(cmd)  # Retry
    
    elif error_code == "PORT_IN_USE":
        return run_command(cmd + " --port " + random_port())
    
    elif error_code == "NETWORK_ERROR":
        sleep(5)
        return run_command(cmd)  # Retry with delay
    
    else:
        report_error(data.get("error"), data.get("suggestion"))
```

---

## Troubleshooting

### CLI Not Found After Install

```bash
# Check if binary was installed
ls -la ~/.local/bin/xion-toolkit

# Add to current session
export PATH="$HOME/.cargo/bin:$PATH"

# Add to shell profile for persistence
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc
```

### Port Already in Use

```bash
# Use different port
xion-toolkit auth login --port 54322

# Or find and kill process using the port (macOS/Linux)
lsof -i :54321
kill <PID>
```

### Token Expired

```bash
# Refresh token
xion-toolkit auth refresh

# If refresh fails, re-login
xion-toolkit auth login
```

### Credentials Not Persisting

1. Check credentials directory: `ls -la ~/.xion-toolkit/credentials/`
2. In CI/CD, ensure `XION_CI_ENCRYPTION_KEY` environment variable is set
3. **Do NOT delete `.enc` files** - they contain 30-day refresh tokens

### Wrong Network

```bash
# Check current network
xion-toolkit config show

# Switch network
xion-toolkit config set-network testnet
```

### Recovery (Clean Reinstall)

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

---

## Example Agent Workflow

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

# 4. Create NFT collection
xion-toolkit asset create --type cw721-base --name "My Collection" --symbol "NFT"

# 5. Mint NFT
xion-toolkit asset mint --contract <CONTRACT_ADDR> --token-id "1" --owner <OWNER_ADDR>
```

---

## Documentation Reference

| Document | Purpose | Lines | Audience |
|----------|---------|-------|----------|
| [QUICK-REFERENCE.md](./docs/QUICK-REFERENCE.md) | Condensed command reference | ~260 | AI Agents |
| [ERROR-CODES.md](./docs/ERROR-CODES.md) | Error code reference | ~185 | AI Agents |
| [cli-reference.md](./docs/cli-reference.md) | Complete CLI documentation | ~2300 | Humans |
| [configuration.md](./docs/configuration.md) | Configuration details | ~250 | Humans |

**Recommended Loading for AI Agents:**
1. This document (INSTALL-FOR-AGENTS.md)
2. [QUICK-REFERENCE.md](./docs/QUICK-REFERENCE.md)
3. [ERROR-CODES.md](./docs/ERROR-CODES.md)

---

## Resources

| Resource | URL |
|----------|-----|
| GitHub Repository | https://github.com/burnt-labs/xion-agent-toolkit |
| Releases | https://github.com/burnt-labs/xion-agent-toolkit/releases |
| Xion Documentation | https://docs.burnt.com/xion |
| Developer Portal | https://dev.testnet2.burnt.com |
| Agent Skills Format | https://agentskills.io/ |

## Support

For issues and feature requests:
- **GitHub Issues**: https://github.com/burnt-labs/xion-agent-toolkit/issues
- **Discussions**: https://github.com/burnt-labs/xion-agent-toolkit/discussions

---

*Document Version: 2.0.0*
*Last Updated: 2026-03-18*
*Compatible CLI Version: >=0.1.0*
