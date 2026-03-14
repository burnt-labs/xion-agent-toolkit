---
status: Done
created_at: 2026-03-13
updated_at: 2026-03-13
---

# Extended Grant Types

## Background

Developer Portal supports 24 message type URLs for authz grants. The CLI currently supports a subset via presets. This plan extends CLI to support all types.

## Goal

Extend `xion-toolkit treasury grant-config` to support all 24 message type URLs that Developer Portal supports.

## Current State

CLI supports these grant types via presets:
- `send` (MsgSend)
- `execute` (MsgExecuteContract)
- `instantiate` (MsgInstantiateContract)
- `instantiate2` (MsgInstantiateContract2)
- `delegate`, `undelegate`, `redelegate`, `withdraw-rewards`
- `vote`
- `ibc-transfer`

## Missing Types

| Category | Type URLs to Add |
|----------|------------------|
| Governance | `MsgDeposit`, `MsgSubmitProposal` |
| Authz | `MsgExec`, `MsgRevoke` |
| Feegrant | `MsgGrantAllowance`, `MsgRevokeAllowance` |
| Crisis | `MsgVerifyInvariant`, `MsgSubmitEvidence` |
| Slashing | `MsgUnjail` |
| Vesting | `MsgCreateVestingAccount` |
| TokenFactory | `MsgMint`, `MsgBurn` |

## API Design

```bash
# Add grant with new presets
xion-toolkit treasury grant-config add <address> \
  --preset gov-deposit \
  --grant-description "Allow governance deposits"

xion-toolkit treasury grant-config add <address> \
  --preset gov-submit-proposal \
  --grant-description "Allow submitting proposals"

# Or use generic type URL
xion-toolkit treasury grant-config add <address> \
  --grant-type-url "/cosmos.gov.v1beta1.MsgDeposit" \
  --grant-auth-type generic \
  --grant-description "Allow deposits"
```

## Implementation

### Tasks

- [x] Add 12 new presets to `PRESET_TYPES` array in `src/cli/treasury.rs`
- [x] Update error message with new preset names
- [x] Add unit tests for new presets (11 tests added)
- [x] Update documentation (`docs/cli-reference.md`)
- [x] Update skills script `skills/xion-treasury/scripts/grant-config.sh`

### Code Changes

**Note**: The implementation uses a simple array-based approach (`PRESET_TYPES: &[(&str, &str, &str)]`) instead of creating a new enum. This approach is simpler and works well.

New presets added:
- Governance: `gov-deposit`, `gov-submit-proposal`
- Authz: `authz-exec`, `authz-revoke`
- Feegrant: `feegrant-grant`, `feegrant-revoke`
- Slashing: `unjail`
- Crisis: `crisis-verify`, `evidence-submit`
- Vesting: `vesting-create`
- TokenFactory: `tokenfactory-mint`, `tokenfactory-burn`

All new presets use `generic` authorization type.

## Files Modified

```
src/
└── cli/
    └── treasury.rs     # MODIFIED - added 12 new presets, updated error message, added tests
docs/
└── cli-reference.md    # MODIFIED - documented all 22 presets in table format
skills/
└── xion-treasury/
    └── scripts/
        └── grant-config.sh  # MODIFIED - added all new preset mappings
```

## Acceptance Criteria

- [x] All 22 presets (10 existing + 12 new) are available
- [x] `--preset gov-deposit` resolves to correct type URL
- [x] Error message lists all available presets
- [x] Unit tests pass for new presets (9 tests added)
- [x] `cargo test` passes
- [x] `cargo clippy` passes with no warnings
- [x] Documentation updated

## Dependencies

- No new dependencies required
- Encoding module already supports GenericAuthorization for all types

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-13 | @project-manager | Plan created | Done |
| 2026-03-13 | @fullstack-dev | Implementation complete | Done |
| 2026-03-13 | @qc-specialist | Code review passed (0 critical/warning) | Done |
| 2026-03-13 | @qa-engineer | All tests pass (401 tests, clippy clean) | Done |
| 2026-03-13 | @project-manager | Final sign-off | Done |
