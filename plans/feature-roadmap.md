---
status: Active
created_at: 2026-03-13
updated_at: 2026-03-15
---

# Feature Roadmap: xion-agent-toolkit

This document outlines the feature roadmap for the Xion Agent Toolkit.

## Current Status

**Phase 1 Completed: 2026-03-14**

All planned Phase 1 features have been implemented and tested.

**Phase 2 P1 Completed: 2026-03-15**

Error Recovery Enhancement and Transaction Monitoring implemented.

---

## Phase 2: Ecosystem Polish

Phase 2 focuses on improving developer experience, code quality, and production readiness.

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| Error Recovery Enhancement | P1 | Medium | ✅ Done (2026-03-15) |
| Transaction Monitoring | P1 | Low | ✅ Done (2026-03-15) |
| `shared/` Module Implementation | P2 | Low | ✅ Done (2026-03-15) |
| Skills Test Framework | P2 | Medium | Proposed |
| CI/CD Integration Output | P2 | Low | Proposed |
| Treasury Analytics | P3 | Medium | Proposed |

### 2.1 Error Recovery Enhancement (P1) ✅

**Completed: 2026-03-15**

- 40+ structured error codes (EAUTH, ETREASURY, EASSET, EBATCH, ECONFIG, ENETWORK, ETX)
- Exponential backoff retry logic
- Refactored oauth2_api.rs, treasury/api_client.rs, treasury/manager.rs
- Documentation in `docs/ERROR-CODES.md`

### 2.2 Transaction Monitoring (P1) ✅

**Completed: 2026-03-15**

```bash
xion-toolkit tx status <tx_hash> --output json
xion-toolkit tx wait <tx_hash> --timeout 60 --interval 2 --output json
```

- Query transaction status from RPC
- Wait for confirmation with configurable timeout
- Proper exit codes (0=success, 1=failure/timeout)

### 2.3 `shared/` Module Implementation (P2) ✅

**Completed: 2026-03-15** (as part of Error Recovery)

- `src/shared/error.rs` - Structured error types
- `src/shared/retry.rs` - Retry logic with exponential backoff
- Re-exports for backward compatibility

### 2.4 Skills Test Framework (P2)

Add dedicated test scripts for skills validation.

**Goals:**
- Create `tests/skills/` directory with skill-specific E2E tests
- Add mock/stub support for testing without network access
- Document testing patterns in skill SKILL.md files

**Acceptance Criteria:**
- [ ] Each skill has at least one E2E test
- [ ] Tests can run offline with mocks
- [ ] CI runs skill tests automatically

### 2.5 CI/CD Integration Output (P2)

Improve output formats for CI/CD pipeline integration.

**Goals:**
- Add `--format github-actions` for GitHub Actions annotations
- Add `--format json-compact` for minimal output
- Add exit codes documentation for scripting

**Acceptance Criteria:**
- [ ] GitHub Actions format outputs workflow commands
- [ ] Exit codes documented and consistent
- [ ] JSON output is parseable by standard tools (jq)

### 2.6 Treasury Analytics (P3)

Add basic analytics and reporting for treasury usage.

```bash
xion-toolkit treasury analytics <address> --period 7d --output json
```

**Goals:**
- Track balance changes over time
- Report grant/fee usage statistics
- Export data in CSV/JSON format

**Acceptance Criteria:**
- [ ] Balance history tracking
- [ ] Usage statistics by grant type
- [ ] Export functionality

---

## Phase 3: Advanced Features

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| Multi-sig Treasury Support | P2 | High | Proposed |
| IBC Transfer Enhancement | P3 | Medium | Proposed |
| Batch Treasury Operations | P3 | Medium | Proposed |
| Predicted Address Computation | P3 | Low | Proposed |

### 3.1 Multi-sig Treasury Support (P2)

Support for multi-signature treasury management.

**Goals:**
- Support threshold signature schemes
- Proposal creation and voting workflow
- Multi-party approval process

**Acceptance Criteria:**
- [ ] Create multi-sig treasury with configurable threshold
- [ ] Submit and vote on proposals
- [ ] Execute approved transactions

### 3.2 IBC Transfer Enhancement (P3)

Improved cross-chain transfer experience.

**Goals:**
- Auto-discover IBC channels
- Validate destination chain compatibility
- Track IBC transfer status

**Acceptance Criteria:**
- [ ] Auto-detect transfer channels
- [ ] Support common IBC destinations (Cosmos Hub, Osmosis)
- [ ] Transfer status tracking

### 3.3 Batch Treasury Operations (P3)

Bulk management operations for treasuries.

**Goals:**
- Batch funding multiple treasuries
- Batch grant/fee configuration
- Import/export multiple treasury configs

**Acceptance Criteria:**
- [ ] Batch fund from JSON config
- [ ] Apply same grant config to multiple treasuries
- [ ] Export all treasury configs

### 3.4 Predicted Address Computation (P3)

Complete predicted address feature for treasury creation.

**Goals:**
- Implement instantiate2 address prediction
- Support all treasury creation scenarios
- Add checksum verification

**Acceptance Criteria:**
- [ ] `treasury create --predict` returns predicted address
- [ ] Predicted address matches actual deployment
- [ ] Checksum validation implemented

---

## Deferred Features

These features are acknowledged but deferred to external projects or future consideration.

| Feature | Reason | Alternative |
|---------|--------|-------------|
| Transaction Wait (standalone) | Better suited for xion-skills | Use `bash wait-tx.sh` from xion-skills |

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
| 2026-03-15 | @project-manager | Phase 2 P1 features completed (Error Recovery, Transaction Monitoring, shared module) | ✅ Done |
| 2026-03-14 | @project-manager | All Phase 1 features completed | ✅ Done |
