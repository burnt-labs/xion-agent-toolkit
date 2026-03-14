---
status: Active
created_at: 2026-03-13
updated_at: 2026-03-14
---

# Feature Roadmap: xion-agent-toolkit

This document outlines the feature roadmap for the Xion Agent Toolkit.

## Current Status

**Phase 1 Completed: 2026-03-14**

All planned Phase 1 features have been implemented and tested.

---

## Future Considerations

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| CW20 Token Support | P2 | Medium | Proposed |
| Transaction Wait (xion-skills) | P3 | Low | Deferred |
| Multi-sig Wallet Support | P3 | High | Proposed |
| IBC Transfer Enhancement | P3 | Medium | Proposed |

---

## Proposed Features

### 1. CW20 Token Support (P2)

Extend Asset Builder to support fungible tokens (CW20).

```bash
xion-toolkit asset create --type cw20-base --name "My Token" --symbol "TKN" --decimals 6
xion-toolkit asset mint --contract xion1... --recipient xion1... --amount 1000000
```

### 2. Transaction Wait (Deferred to xion-skills)

Add transaction wait functionality to xion-skills for polling until confirmation.

```bash
# This will be in xion-skills, not xion-agent-toolkit
bash wait-tx.sh <txhash> [--timeout 60] [--interval 2]
```

### 3. Multi-sig Wallet Support (P3)

Support for multi-signature treasury management.

### 4. IBC Transfer Enhancement (P3)

Improved cross-chain transfer experience with channel auto-discovery.

---

## Completed Features

### Phase 1 (Completed 2026-03-14)

| Feature | Priority | Complexity | Completed |
|---------|----------|------------|-----------|
| MetaAccount Info | P1 | Low | 2026-03-13 |
| Extended Grant Types | P2 | Medium | 2026-03-13 |
| Batch Operations | P3 | Medium | 2026-03-13 |
| Asset Builder (CW721) | P4 | High | 2026-03-14 |

#### MetaAccount Info (P1)
- Added `xion-toolkit account info` command
- Uses OAuth2 API `/api/v1/me` endpoint
- Returns MetaAccount authenticators data

#### Extended Grant Types (P2)
- Added 12 new presets to support all 24 message type URLs
- Added 11 unit tests
- Updated error messages

#### Batch Operations (P3)
- Added `src/batch/` module with executor and types
- Added `xion-toolkit batch execute` command
- 23 unit tests passing

#### Asset Builder (P4)
- Phase 1: Basic CW721 deployment and minting
- Phase 2: All 5 variants supported (cw721-base, metadata-onchain, expiration, non-transferable, cw2981-royalties)
- Phase 3: Address prediction and batch minting
- Skills integration: `skills/xion-asset/`
- 232 tests passing

---

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-14 | @project-manager | All Phase 1 features completed | ✅ Done |
