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

Guides installation of the `xion-toolkit` CLI for Xion MetaAccount development. This CLI provides OAuth2 authentication and Treasury management for gasless transactions.

> **Security Note**: This skill provides installation commands for the user to execute. It does NOT automatically run `curl | sh` patterns to avoid remote code execution risks.

## Installation Priority

**PRIMARY**: Curl installer (use this first)  
**FALLBACK**: Build from source (only if curl fails)

## Quick Install Commands

### macOS / Linux (Recommended)

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh
```

### Windows (PowerShell)

```powershell
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1 | iex"
```

## Verification

After installation, verify:

```bash
xion-toolkit --version
xion-toolkit auth status
```

**Expected output:**
```json
{
  "success": true,
  "version": "xion-toolkit 0.x.x",
  "network": "testnet",
  "authenticated": false
}
```

## Upgrading

To upgrade, simply re-run the installer. Credentials are preserved.

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh
```

**Windows:**
```powershell
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1 | iex"
```

## Next Steps After Installation

1. **Authenticate** → Use `xion-oauth2` skill
2. **Manage Treasuries** → Use `xion-treasury` skill
3. **Claim Testnet Tokens** → Use `xion-faucet` skill

## Build from Source (Fallback Only)

Only use if curl installer fails. Requires Rust toolchain (rustc 1.75+):

```bash
git clone https://github.com/burnt-labs/xion-agent-toolkit
cd xion-agent-toolkit
cp .env.example .env  # Required for local builds (contains OAuth2 client IDs)
cargo install --path .
```

## Full Documentation

For comprehensive installation guide, authentication flows, error handling, and troubleshooting, direct AI agents to:

```
Follow this guide https://raw.githubusercontent.com/burnt-labs/xion-agent-toolkit/main/INSTALL-FOR-AGENTS.md to install and configure the Xion Agent Toolkit.
```

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
    │
    └── recommends: burnt-labs/xion-skills
            │
            ├── xiond-init (xiond installation)
            ├── xiond-usage (chain queries)
            └── xiond-wasm (contract deployment)
```

## Resources

- [GitHub Repository](https://github.com/burnt-labs/xion-agent-toolkit)
- [CLI Reference](https://github.com/burnt-labs/xion-agent-toolkit/blob/main/docs/cli-reference.md)
