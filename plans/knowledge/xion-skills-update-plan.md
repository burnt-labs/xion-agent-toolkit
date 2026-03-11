# xion-skills Update Plan

> This plan is for updating the [burnt-labs/xion-skills](https://github.com/burnt-labs/xion-skills) repository separately.

## Background

The xion-skills repository provides `xiond` CLI skills that complement xion-agent-toolkit. After integrating with xion-agent-toolkit, we need to:

1. Position xion-skills as the "advanced/specialized" toolset
2. Optimize trigger keywords to avoid confusion with toolkit skills
3. Strengthen chain query capabilities (primary differentiation)

## Target Positioning

```
┌─────────────────────────────────────────────────────────────────┐
│                    Xion Developer Ecosystem                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   MetaAccount Developers (90%)        Contract Developers (10%) │
│   ┌─────────────────────────┐        ┌─────────────────────────┐│
│   │   xion-agent-toolkit    │        │     xion-skills         ││
│   │   (Primary Tool)        │        │   (Advanced Tool)       ││
│   ├─────────────────────────┤        ├─────────────────────────┤│
│   │ • OAuth2 authentication │        │ • Chain queries         ││
│   │ • Treasury management   │        │ • CosmWasm deployment   ││
│   │ • Gasless transactions  │        │ • Contract migration    ││
│   │ • Grant configuration   │        │ • Mnemonic management   ││
│   └─────────────────────────┘        └─────────────────────────┘│
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Tasks

### Phase 1: Update SKILL.md Files

#### xiond-init

Update `skills/xiond-init/SKILL.md`:

**Current description:**
```
Install, upgrade, and verify the `xiond` CLI for Xion blockchain. 
Use this proactively whenever the user mentions xiond installation, 
setup, initialization, configuration, environment setup...
```

**New description:**
```yaml
---
name: xiond-init
description: |
  Install, upgrade, and verify the `xiond` CLI for Xion blockchain.
  Use when user specifically mentions xiond, contract development environment,
  validator setup, or needs the traditional Cosmos SDK CLI.
  
  For most Xion developers, xion-toolkit (MetaAccount) is recommended instead.
  
  Triggers: xiond, xiond install, contract development, validator, cosmos CLI,
  traditional wallet, mnemonic-based.
---
```

**Add section:**
```markdown
## When to Use xiond vs xion-toolkit

| Scenario | Recommended Tool |
|----------|------------------|
| Regular development | xion-toolkit (MetaAccount) |
| Gasless transactions | xion-toolkit |
| Contract deployment | xiond (this skill) |
| Chain queries | xiond (this skill) |
| Validator operations | xiond (this skill) |
```

---

#### xiond-usage

Update `skills/xiond-usage/SKILL.md`:

**Current description:**
```
Day-to-day Xion account operations using `xiond` CLI. 
Use this proactively whenever the user mentions wallet, key, account...
```

**New description:**
```yaml
---
name: xiond-usage
description: |
  Chain queries and traditional wallet operations using `xiond` CLI.
  
  PRIMARY USE CASES:
  - Chain/block queries (primary strength)
  - Transaction status queries
  - Balance queries (any address)
  - Mnemonic-based wallet management
  
  For gasless transactions and Treasury operations, use xion-toolkit instead.
  
  Triggers: 链上查询, chain query, 交易查询, tx status, block query,
  balance query, mnemonic, 传统钱包, cosmos wallet.
---
```

**Add section:**
```markdown
## Query Capabilities

This skill excels at chain queries:

| Query Type | Command |
|------------|---------|
| Block info | `query-chain-info.sh` |
| Transaction | `query-tx.sh <txhash>` |
| Balance | `query-balance.sh <address>` |
| Account list | `list-accounts.sh` |

## When to Use xiond vs xion-toolkit

| Scenario | Recommended Tool |
|----------|------------------|
| Gasless transactions | xion-toolkit |
| Treasury operations | xion-toolkit |
| Chain queries | xiond (this skill) |
| Transaction queries | xiond (this skill) |
| Mnemonic wallet | xiond (this skill) |
```

---

#### xiond-wasm

Update `skills/xiond-wasm/SKILL.md`:

**Current description:**
```
Deploy, interact with, and manage CosmWasm smart contracts...
```

**New description:**
```yaml
---
name: xiond-wasm
description: |
  Deploy and manage CosmWasm smart contracts on Xion using `xiond`.
  
  This is the PRIMARY tool for contract developers. Covers the full lifecycle:
  optimize → upload → instantiate → query/execute → migrate.
  
  Triggers: CosmWasm, 合约部署, contract deployment, wasm, Code ID,
  instantiate, execute contract, migrate contract, 合约迁移, 智能合约.
---
```

**Add section:**
```markdown
## Contract Developer Focus

This skill is specifically designed for CosmWasm contract developers:

1. **Optimize** - Prepare contract bytecode
2. **Upload** - Store code on chain
3. **Instantiate** - Create contract instances
4. **Execute/Query** - Interact with contracts
5. **Migrate** - Upgrade contract logic

## Integration with xion-toolkit

After deploying contracts, you can use xion-toolkit's Treasury to:
- Fund contract operations gaslessly
- Configure authz grants for contract interactions
```

---

### Phase 2: Update README.md

Add section to `README.md`:

```markdown
## Relationship with xion-toolkit

**xion-skills** provides `xiond` CLI skills for advanced scenarios:

| Use xion-skills when... | Use xion-toolkit when... |
|------------------------|--------------------------|
| Deploying CosmWasm contracts | Building apps with MetaAccount |
| Querying chain data | Managing Treasury contracts |
| Validator operations | Gasless transactions |
| Mnemonic-based wallets | OAuth2 authentication |

**For most Xion developers, start with [xion-toolkit](https://github.com/burnt-labs/xion-agent-toolkit).**
```

---

### Phase 3: Add Cross-Reference

Consider adding a lightweight cross-reference file:

`skills/xion-toolkit-bridge/SKILL.md`:
```markdown
---
name: xion-toolkit-bridge
description: |
  MetaAccount development with xion-toolkit. 
  For gasless transactions and Treasury management.
  See: https://github.com/burnt-labs/xion-agent-toolkit
---

# xion-toolkit Bridge

For MetaAccount-based development (gasless, OAuth2, Treasury):

👉 **Use xion-toolkit**: https://github.com/burnt-labs/xion-agent-toolkit

## Quick Comparison

| Feature | xion-toolkit | xiond |
|---------|--------------|-------|
| Auth | OAuth2 + MetaAccount | Mnemonic |
| Gas | Gasless | Paid |
| Treasury | ✓ Full support | Limited |
| Contracts | Execute only | Full lifecycle |
| Queries | Basic | Advanced |
```

---

## Trigger Keywords Summary

### Avoid These (toolkit territory)

- `MetaAccount`, `gasless`, `无 gas`, `OAuth2`, `Treasury`
- `session key`, `fee grant`, `authz grant` (for configuration)

### Emphasize These

- `xiond`, `cosmos CLI`, `命令行`, `合约开发`
- `链上查询`, `区块查询`, `交易查询`, `RPC`
- `CosmWasm`, `wasm`, `合约部署`, `合约迁移`
- `mnemonic`, `传统钱包`, `助记词`

---

## File Changes Summary

| File | Changes |
|------|---------|
| `skills/xiond-init/SKILL.md` | Update description, add comparison section |
| `skills/xiond-usage/SKILL.md` | Update description, emphasize query capabilities |
| `skills/xiond-wasm/SKILL.md` | Update description, add integration notes |
| `README.md` | Add xion-toolkit relationship section |
| `skills/xion-toolkit-bridge/SKILL.md` | (Optional) New bridge skill |

---

## Acceptance Criteria

- [ ] All skills clearly differentiate from xion-toolkit
- [ ] Trigger keywords don't overlap with toolkit skills
- [ ] README explains relationship with xion-toolkit
- [ ] Chain query capabilities are emphasized

---

## Notes

- This plan should be implemented after xion-agent-toolkit integration is complete
- Consider versioning the skill updates (e.g., v2.0.0)
- Update evals to test new trigger conditions
