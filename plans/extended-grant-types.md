---
status: Todo
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

- [ ] Extend `GrantPreset` enum in `src/treasury/types.rs`
- [ ] Add new preset variants:
  - [ ] `GovDeposit`
  - [ ] `GovSubmitProposal`
  - [ ] `AuthzExec`
  - [ ] `AuthzRevoke`
  - [ ] `FeegrantGrant`
  - [ ] `FeegrantRevoke`
  - [ ] `Unjail`
  - [ ] `CrisisVerifyInvariant`
  - [ ] `CrisisSubmitEvidence`
  - [ ] `VestingCreateAccount`
  - [ ] `TokenFactoryMint`
  - [ ] `TokenFactoryBurn`
- [ ] Update CLI help text with new presets
- [ ] Add unit tests for new presets
- [ ] Update documentation

### Code Changes

```rust
// src/treasury/types.rs

pub enum GrantPreset {
    // Existing
    Send,
    Execute,
    Instantiate,
    Instantiate2,
    Delegate,
    Undelegate,
    Redelegate,
    WithdrawRewards,
    Vote,
    IbcTransfer,
    
    // NEW - Governance
    GovDeposit,
    GovSubmitProposal,
    
    // NEW - Authz
    AuthzExec,
    AuthzRevoke,
    
    // NEW - Feegrant
    FeegrantGrant,
    FeegrantRevoke,
    
    // NEW - Slashing
    Unjail,
    
    // NEW - Crisis
    CrisisVerifyInvariant,
    CrisisSubmitEvidence,
    
    // NEW - Vesting
    VestingCreateAccount,
    
    // NEW - TokenFactory
    TokenFactoryMint,
    TokenFactoryBurn,
}

impl GrantPreset {
    pub fn type_url(&self) -> &'static str {
        match self {
            // Existing...
            Self::GovDeposit => "/cosmos.gov.v1beta1.MsgDeposit",
            Self::GovSubmitProposal => "/cosmos.gov.v1beta1.MsgSubmitProposal",
            Self::AuthzExec => "/cosmos.authz.v1beta1.MsgExec",
            Self::AuthzRevoke => "/cosmos.authz.v1beta1.MsgRevoke",
            Self::FeegrantGrant => "/cosmos.feegrant.v1beta1.MsgGrantAllowance",
            Self::FeegrantRevoke => "/cosmos.feegrant.v1beta1.MsgRevokeAllowance",
            Self::Unjail => "/cosmos.slashing.v1beta1.MsgUnjail",
            Self::CrisisVerifyInvariant => "/cosmos.crisis.v1beta1.MsgVerifyInvariant",
            Self::CrisisSubmitEvidence => "/cosmos.evidence.v1beta1.MsgSubmitEvidence",
            Self::VestingCreateAccount => "/cosmos.vesting.v1beta1.MsgCreateVestingAccount",
            Self::TokenFactoryMint => "/osmosis.tokenfactory.v1beta1.MsgMint",
            Self::TokenFactoryBurn => "/osmosis.tokenfactory.v1beta1.MsgBurn",
        }
    }
    
    pub fn default_auth_type(&self) -> AuthorizationType {
        AuthorizationType::Generic // Most new types use GenericAuthorization
    }
}
```

## Files to Modify

```
src/
├── cli/
│   └── treasury.rs     # MODIFY - add preset options
├── treasury/
│   └── types.rs        # MODIFY - add preset variants
└── config/
    └── constants.rs    # MODIFY - add type URL constants (optional)
```

## Acceptance Criteria

- [ ] All 24 type URLs supported via `--grant-type-url` flag
- [ ] New presets for common governance operations
- [ ] Documentation updated with new presets
- [ ] Unit tests for new presets
- [ ] Manual testing on testnet

## Dependencies

- No new dependencies required
- Encoding module already supports GenericAuthorization for all types

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-13 | @project-manager | Plan created | Todo |
