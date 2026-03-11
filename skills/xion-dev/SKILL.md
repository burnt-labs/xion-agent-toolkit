---
name: xion-dev
description: |
  The primary entry point for ALL Xion blockchain development. Use this skill whenever the user mentions anything related to Xion, Xion development, building on Xion, MetaAccount, gasless transactions, Treasury contracts, OAuth2 authentication on Xion, or any Xion-related operations.
  
  This skill helps route users to the correct tool based on their needs:
  - MetaAccount/gasless operations → xion-toolkit skills (this repo)
  - Chain queries/contract deployment → xiond skills (xion-skills repo)
  
  Triggers on: xion, xion blockchain, xion 开发, MetaAccount, gasless, 无 gas, Treasury, OAuth2 xion, xion 认证, xion login, xion toolkit, burnt labs, building on xion.
  
  Make sure to use this skill for ANY Xion-related question, even if the user doesn't explicitly ask for "xion-dev" or "toolkit".
metadata:
  author: burnt-labs
  version: "1.0.0"
  recommends:
    - xion-toolkit-init
    - xion-oauth2
    - xion-treasury
    - burnt-labs/xion-skills
---

# xion-dev

Unified entry point for Xion blockchain development. This skill helps you choose the right tool for the job.

## Core Philosophy

**Xion developers should primarily use MetaAccount for a gasless experience.**

- Most developers (90%) use MetaAccount + OAuth2 for gasless transactions
- Traditional xiond CLI is reserved for advanced scenarios (contract deployment, chain queries)

## Decision Matrix

When a user mentions Xion-related needs, use this matrix to recommend the correct tool:

| User Needs | Recommended Skill | Tool | Why |
|------------|-------------------|------|-----|
| **Login / Authentication** | `xion-oauth2` | xion-toolkit | MetaAccount, gasless |
| **Create Treasury** | `xion-treasury` | xion-toolkit | Core functionality |
| **Query Treasury** | `xion-treasury` | xion-toolkit | Direct API access |
| **Fund / Withdraw** | `xion-treasury` | xion-toolkit | Gasless transactions |
| **Authz Grant Config** | `xion-treasury` | xion-toolkit | Specialized feature |
| **Fee Grant Config** | `xion-treasury` | xion-toolkit | Specialized feature |
| **Query chain data** | `xiond-usage` | xiond | More powerful queries |
| **Query tx status** | `xiond-usage` | xiond | Direct RPC access |
| **Query block info** | `xiond-usage` | xiond | Chain-level queries |
| **Deploy CosmWasm** | `xiond-wasm` | xiond | Contract developer tool |
| **Migrate contract** | `xiond-wasm` | xiond | Advanced contract ops |
| **Recover wallet (mnemonic)** | `xiond-usage` | xiond | Mnemonic management |

## Quick Start

### For Most Developers (MetaAccount Path)

```bash
# 1. Install xion-toolkit CLI
# Use: xion-toolkit-init skill

# 2. Authenticate with MetaAccount
xion-toolkit auth login
# Or use: xion-oauth2 skill

# 3. Manage Treasuries
xion-toolkit treasury list
xion-toolkit treasury create --name "My Treasury"
xion-toolkit treasury fund <address> --amount 1000000uxion
# Or use: xion-treasury skill
```

### For Contract Developers (xiond Path)

```bash
# 1. Install xiond CLI
# Use: xiond-init skill from burnt-labs/xion-skills

# 2. Create/import wallet
xiond keys add my-wallet
# Or use: xiond-usage skill

# 3. Deploy contracts
xiond tx wasm store contract.wasm --from my-wallet
# Or use: xiond-wasm skill
```

## Tool Comparison

| Feature | xion-toolkit (MetaAccount) | xiond (Traditional) |
|---------|---------------------------|---------------------|
| **Authentication** | OAuth2 + Browser | Mnemonic / Keyring |
| **Gas** | Gasless (Fee Grant) | User pays gas |
| **Treasury** | Full support | Limited |
| **Contract Deploy** | Execute only | Full lifecycle |
| **Chain Queries** | Basic | Advanced |
| **Target User** | App developers | Contract devs / Validators |

## When to Recommend xion-skills

Point users to [burnt-labs/xion-skills](https://github.com/burnt-labs/xion-skills) when they need:

1. **Chain Queries** - Block info, transaction status, balance queries for any address
2. **Contract Deployment** - Upload, instantiate, migrate CosmWasm contracts
3. **Mnemonic Wallets** - Traditional key management with seed phrases
4. **Validator Operations** - Advanced node and validator management

## Related Skills

### In This Repository (xion-agent-toolkit)

| Skill | Purpose |
|-------|---------|
| `xion-toolkit-init` | Install xion-toolkit CLI |
| `xion-oauth2` | MetaAccount authentication |
| `xion-treasury` | Treasury lifecycle management |

### In xion-skills Repository

| Skill | Purpose |
|-------|---------|
| `xiond-init` | Install xiond CLI |
| `xiond-usage` | Chain queries, wallet management |
| `xiond-wasm` | CosmWasm contract operations |

## Installation

```bash
# Install toolkit skills
npx skills add burnt-labs/xion-agent-toolkit

# Install xiond skills (for advanced scenarios)
npx skills add burnt-labs/xion-skills
```

## Network Configuration

| Network | OAuth2 API | RPC | Chain ID |
|---------|------------|-----|----------|
| testnet | oauth2.testnet.burnt.com | rpc.xion-testnet-2.burnt.com:443 | xion-testnet-2 |
| mainnet | oauth2.burnt.com | rpc.xion-mainnet-1.burnt.com:443 | xion-mainnet-1 |

## Troubleshooting

### User asks about "gas" or "fees"
→ Recommend xion-toolkit (MetaAccount) for gasless transactions

### User mentions "mnemonic" or "seed phrase"
→ Recommend xiond-usage from xion-skills

### User wants to "deploy a contract"
→ Recommend xiond-wasm from xion-skills

### User wants to "query transaction"
→ Recommend xiond-usage from xion-skills

## Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [xion-agent-toolkit](https://github.com/burnt-labs/xion-agent-toolkit)
- [xion-skills](https://github.com/burnt-labs/xion-skills)
- [Agent Skills Format](https://agentskills.io/)
