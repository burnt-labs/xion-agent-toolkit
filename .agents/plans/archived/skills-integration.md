---
status: Done
created_at: 2026-03-11
updated_at: 2026-03-11
done_at: 2026-03-11
---

# Skills Integration Plan

## Background

We have two complementary skills repositories:

| Repository | Target Users | Core Tool | Key Features |
|------------|--------------|-----------|--------------|
| **xion-agent-toolkit** | MetaAccount developers (90%) | `xion-toolkit` (Rust) | Gasless, OAuth2, Treasury |
| **xion-skills** | Contract developers (10%) | `xiond` (Go) | Chain queries, CosmWasm |

**Core Philosophy**: Xion developers should primarily use **MetaAccount** for gasless experience. The `xiond` CLI is reserved for advanced scenarios (contract deployment, chain queries).

## Goals

1. Create unified entry skill (`xion-dev`) for intelligent routing
2. Update toolkit skills with MetaAccount-first messaging and optimized triggers
3. Remove incomplete `chain-query.sh` - delegate all chain queries to xion-skills
4. Establish clear skill boundaries between the two repositories

## Tasks

### Phase 1: New Entry Skill

- [x] Create `skills/xion-dev/SKILL.md` with:
  - Decision matrix for routing to correct skill
  - MetaAccount-first philosophy
  - Quick start guide
  - Links to all related skills

### Phase 2: Update Existing Skills

- [x] Update `skills/xion-toolkit-init/SKILL.md`:
  - Add "Core Philosophy" section emphasizing MetaAccount-first
  - Update trigger keywords (see below)
  - Add reference to xion-skills for query scenarios

- [x] Update `skills/xion-oauth2/SKILL.md`:
  - Update trigger keywords (see below)
  - Emphasize gasless authentication

- [x] Update `skills/xion-treasury/SKILL.md`:
  - Update trigger keywords (see below)
  - Remove `chain-query.sh` from documentation
  - Add "Chain Queries" section pointing to xion-skills

### Phase 3: Code Cleanup

- [x] Remove `skills/xion-treasury/scripts/chain-query.sh`
- [x] Check if CLI has `treasury chain-query` command - consider deprecation
- [x] Update `references/scripts-reference.md` to remove chain-query

### Phase 4: Documentation

- [x] Update `README.md` to mention skill integration with xion-skills
- [x] Create or update section explaining when to use which toolset

---

## Trigger Keywords Optimization

### xion-agent-toolkit Skills (MetaAccount-focused)

| Skill | New Trigger Keywords |
|-------|---------------------|
| `xion-toolkit-init` | `MetaAccount`, `gasless`, `无 gas`, `xion toolkit`, `agent 开发`, `xion 开发入门`, `OAuth2 开发` |
| `xion-oauth2` | `MetaAccount 登录`, `browser login`, `gasless auth`, `session key`, `OAuth2 登录`, `xion 认证` |
| `xion-treasury` | `Treasury`, `MetaAccount Treasury`, `gasless 交易`, `authz grant`, `fee grant`, `无 gas 交易` |

### xion-skills (xiond-focused) - For Reference

| Skill | New Trigger Keywords |
|-------|---------------------|
| `xiond-init` | `xiond`, `合约开发环境`, `validator`, `命令行工具`, `cosmos CLI` |
| `xiond-usage` | `链上查询`, `区块查询`, `交易查询`, `交易状态`, `mnemonic`, `传统钱包`, `余额查询` |
| `xiond-wasm` | `CosmWasm`, `合约部署`, `合约上传`, `合约迁移`, `Code ID`, `instantiate`, `wasm` |

---

## Decision Matrix (for xion-dev)

| User Needs | Recommended Tool | Why |
|------------|------------------|-----|
| Login/Auth | `xion-oauth2` | MetaAccount, gasless |
| Create/Manage Treasury | `xion-treasury` | Core functionality |
| Fund/Withdraw | `xion-treasury` | Gasless transactions |
| Authz/Fee Grant | `xion-treasury` | Specialized feature |
| Query chain data | `xiond-usage` | More powerful queries |
| Query tx status | `xiond-usage` | Direct RPC access |
| Deploy CosmWasm | `xiond-wasm` | Contract developer tool |
| Recover wallet | `xiond-usage` | Mnemonic management |

---

## File Changes Summary

### New Files

| File | Purpose |
|------|---------|
| `skills/xion-dev/SKILL.md` | Unified entry skill |

### Modified Files

| File | Changes |
|------|---------|
| `skills/xion-toolkit-init/SKILL.md` | Add philosophy, update triggers |
| `skills/xion-oauth2/SKILL.md` | Update triggers |
| `skills/xion-treasury/SKILL.md` | Update triggers, remove chain-query, add xion-skills reference |
| `README.md` | Add skill integration section |

### Deleted Files

| File | Reason |
|------|--------|
| `skills/xion-treasury/scripts/chain-query.sh` | Incomplete, delegate to xion-skills |

---

## Acceptance Criteria

- [x] `xion-dev` skill correctly routes to appropriate skill
- [x] All toolkit skills emphasize MetaAccount-first philosophy
- [x] No overlap/confusion in trigger keywords between toolkit and xion-skills
- [x] Chain query documentation points to xion-skills
- [x] `chain-query.sh` removed from repository

---

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
