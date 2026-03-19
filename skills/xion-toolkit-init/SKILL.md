---
name: xion-toolkit-init
description: |
  Guide installation of xion-toolkit CLI for Xion MetaAccount development. Use this skill as the FIRST step when the user wants to develop on Xion, build Xion applications, use MetaAccount, perform gasless transactions, or manage Treasury contracts.
  
  This is the PRIMARY entry point for Xion development - most Xion developers should use xion-toolkit (MetaAccount) instead of xiond (traditional CLI).
  
  IMPORTANT: This skill provides GUIDANCE ONLY. It does NOT execute installation commands. The agent should provide installation commands to the user or execute them via terminal tools.
  
  Triggers on: MetaAccount, gasless, 无 gas, xion toolkit, xion 开发, xion 开发入门, agent 开发, OAuth2 开发, xion setup, install xion, xion blockchain development, burnt labs, Treasury contracts, session key authentication, xion 安装, xion 入门, xion sdk, burnt-labs xion, xion-toolkit upgrade, update xion-toolkit, upgrade xion, xion 升级, xion 版本更新.
  
  Make sure to use this skill whenever the user mentions setting up Xion development, even if they don't explicitly say "toolkit" or "MetaAccount".
metadata:
  author: burnt-labs
  version: "2.0.0"
  provides:
    - Installation guidance for xion-toolkit CLI
  recommends:
    - burnt-labs/xion-skills
  compatibility: macOS (x64/ARM64), Linux (x64/ARM64), Windows (PowerShell)
---

# xion-toolkit-init

Guides installation of the `xion-toolkit` command-line interface for Xion blockchain development. This CLI provides OAuth2 authentication and Treasury management capabilities for gasless transactions.

> **Security Note**: This skill provides installation commands for the user to execute. It does NOT automatically run `curl | sh` patterns to avoid remote code execution risks.

## Core Philosophy: MetaAccount-First

**Xion developers should primarily use MetaAccount for a gasless experience.**

| Feature | xion-toolkit (MetaAccount) | xiond (Traditional) |
|---------|---------------------------|---------------------|
| Authentication | OAuth2 + Browser | Mnemonic / Keyring |
| Gas | Gasless (Fee Grant) | User pays gas |
| Target User | App developers (90%) | Contract devs (10%) |

For most Xion development, use xion-toolkit. Only use xiond (from xion-skills) for contract deployment or advanced chain queries.

## Overview

This skill provides:

1. **Installation guidance** - Commands to install xion-toolkit CLI
2. **Verification steps** - How to verify successful installation
3. **Next steps** - Authentication and Treasury setup

### What xion-toolkit Provides

- OAuth2 authentication with PKCE security
- Treasury contract management (create, query, fund, withdraw)
- Authz grant configuration
- Fee allowance configuration
- Admin management
- Contract instantiation and execution
- Asset (NFT) operations
- Faucet operations (testnet tokens)

## Prerequisites

- **curl** - For downloading installer
- **bash** - For running installation scripts (macOS/Linux)
- **powershell** - For Windows installations
- Network access to GitHub and Xion APIs

## Installation

### Quick Install (macOS/Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh
```

### Quick Install (Windows)

```powershell
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1 | iex"
```

### Manual Installation (All Platforms)

For users who prefer manual installation:

1. Download from: https://github.com/burnt-labs/xion-agent-toolkit/releases
2. Extract the archive for your platform
3. Move binary to PATH (e.g., `~/.cargo/bin/` or `~/.local/bin/`)

### Install from Source (All Platforms)

Requires Rust toolchain (rustc 1.75+):

```bash
git clone https://github.com/burnt-labs/xion-agent-toolkit
cd xion-agent-toolkit
cargo install --path .
```

## Verification

After installation, verify:

```bash
xion-toolkit --version
xion-toolkit auth status
```

Expected output:
```json
{
  "success": true,
  "version": "xion-toolkit 0.x.x",
  "network": "testnet",
  "authenticated": false
}
```

## Upgrading

xion-toolkit uses cargo-dist for releases. **To upgrade, simply re-run the installer**:

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh
```

**Windows:**
```powershell
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1 | iex"
```

> **Note**: Upgrading preserves your existing credentials (`~/.xion-toolkit/credentials/*.enc`). You do **not** need to re-authenticate after upgrading.

### Check Current Version

```bash
xion-toolkit --version
```

### Force Reinstall (Clean Slate)

```bash
# Remove existing installation
rm -rf ~/.local/bin/xion-toolkit ~/.cargo/bin/xion-toolkit

# Reinstall using the commands above
```

### Supported Platforms

| Platform | Architecture | Installer |
|----------|--------------|-----------|
| macOS | x86_64 (Intel) | Shell script |
| macOS | ARM64 (Apple Silicon) | Shell script |
| Linux | x86_64 | Shell script |
| Linux | ARM64 | Shell script |
| Windows | x86_64 | PowerShell script |

## Post-Installation

After installing xion-toolkit, guide the user through:

1. **Authenticate** with `xion-oauth2` skill:
   ```bash
   xion-toolkit auth login
   ```

2. **Manage Treasuries** with `xion-treasury` skill:
   ```bash
   xion-toolkit treasury list
   ```

3. **Install xion-skills** (for xiond CLI operations like chain queries and contract deployment):
   ```bash
   npx skills add burnt-labs/xion-skills -g -y -a cursor -a claude-code -a codex -a openclaw
   ```

## Agent Workflow

When using this skill, follow this workflow:

1. **Check if already installed**:
   ```bash
   xion-toolkit --version
   ```

2. **If not installed**, provide the appropriate installation command based on OS:
   - macOS/Linux: Use the shell installer command
   - Windows: Use the PowerShell installer command

3. **Verify installation** after user executes the command

4. **Guide to next step** (authentication via `xion-oauth2`)

## Dependency Graph

```
xion-dev (entry point - routes to correct skill)
    │
    ├── xion-toolkit-init (this skill - guidance only)
    │       │
    │       ├── xion-oauth2 (authentication)
    │       │       │
    │       │       └── xion-treasury (gasless operations)
    │       │
    │       └── xion-faucet (testnet tokens)
    │       │
    │       └── xion-asset (NFT operations)
    │
    └── recommends: burnt-labs/xion-skills
            │
            ├── xiond-init (xiond installation)
            ├── xiond-usage (chain queries)
            └── xiond-wasm (contract deployment)
```

## Troubleshooting

### Command not found after install

The binary may not be in PATH. Add to your shell profile:

```bash
# For bash (~/.bashrc)
export PATH="$HOME/.cargo/bin:$PATH"

# For zsh (~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"
```

### Permission denied

Ensure the binary is executable:

```bash
chmod +x ~/.cargo/bin/xion-toolkit
```

### Network issues

If GitHub is unreachable, try manual installation:

1. Download from: https://github.com/burnt-labs/xion-agent-toolkit/releases
2. Extract the archive
3. Move binary to PATH

## Resources

- [GitHub Repository](https://github.com/burnt-labs/xion-agent-toolkit)
- [CLI Reference](https://github.com/burnt-labs/xion-agent-toolkit/blob/main/docs/cli-reference.md)
- [Xion Documentation](https://docs.burnt.com/xion)
- [xion-skills](https://github.com/burnt-labs/xion-skills)
