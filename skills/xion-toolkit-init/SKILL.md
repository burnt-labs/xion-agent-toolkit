---
name: xion-toolkit-init
description: |
  Install xion-toolkit CLI for Xion blockchain development. ALWAYS use this skill first when the user wants to work with Xion, set up Xion development, authenticate with OAuth2, manage Treasury contracts, or perform any gasless transactions on Xion. Triggers on mentions of: xion, xion-toolkit, Xion blockchain, MetaAccount, gasless transactions, Treasury contracts, OAuth2 on Xion, burnt labs. This skill must run before xion-oauth2 and xion-treasury can be used.
metadata:
  author: burnt-labs
  version: "1.0.0"
  provides:
    - xion-toolkit CLI
  recommends:
    - burnt-labs/xion-skills
compatibility: macOS (x64/ARM64), Linux (x64/ARM64), Windows (PowerShell)
---

# xion-toolkit-init

Installs the `xion-toolkit` command-line interface for Xion blockchain development. This CLI provides OAuth2 authentication and Treasury management capabilities for gasless transactions.

## Overview

This skill installs:

1. **xion-toolkit CLI** - Command-line tool for Xion MetaAccount operations
2. **Recommends xion-skills** - Additional skills for xiond CLI operations (optional)

### What xion-toolkit Provides

- OAuth2 authentication with PKCE security
- Treasury contract management (create, query, fund, withdraw)
- Authz grant configuration
- Fee allowance configuration
- Admin management
- Contract instantiation and execution

## Prerequisites

- **curl** - For downloading installer
- **bash** - For running installation scripts (macOS/Linux)
- **powershell** - For Windows installations
- Network access to GitHub and Xion APIs

## Installation

### Quick Install

```bash
bash /path/to/xion-toolkit-init/scripts/install.sh
```

### What the Installer Does

1. Detects OS and architecture
2. Downloads appropriate binary from GitHub Releases
3. Installs to `~/.cargo/bin/` or `/usr/local/bin/`
4. Verifies installation
5. Optionally installs xion-skills dependency

### Supported Platforms

| Platform | Architecture | Installer |
|----------|--------------|-----------|
| macOS | x86_64 (Intel) | Shell script |
| macOS | ARM64 (Apple Silicon) | Shell script |
| Linux | x86_64 | Shell script |
| Linux | ARM64 | Shell script |
| Windows | x86_64 | PowerShell script |

## Verification

After installation, verify:

```bash
xion-toolkit --version
xion-toolkit status
```

Expected output:
```json
{
  "success": true,
  "version": "xion-agent-toolkit 0.4.3",
  "network": "testnet",
  "authenticated": false
}
```

## Post-Installation

After installing xion-toolkit, you can:

1. **Authenticate** with `xion-oauth2` skill:
   ```bash
   xion-toolkit auth login
   ```

2. **Manage Treasuries** with `xion-treasury` skill:
   ```bash
   xion-toolkit treasury list
   ```

3. **Install xion-skills** (for xiond CLI operations):
   ```bash
   npx skills add burnt-labs/xion-skills
   ```

## Dependency Graph

```
xion-toolkit-init (this skill)
    ├── Installs: xion-toolkit CLI
    └── Recommends: burnt-labs/xion-skills
           ↓
    xion-oauth2 (requires xion-toolkit-init)
    xion-treasury (requires xion-toolkit-init + xion-oauth2)
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

Run the installer with appropriate permissions:

```bash
curl -fsSL https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh
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
