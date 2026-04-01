---
status: Planned
created_at: 2026-04-01
updated_at: 2026-04-01
---

# OAuth2 Client Advanced Lifecycle Plan

**Goal:** Deliver advanced lifecycle operations for OAuth2 clients in `xion-agent-toolkit`, focused on secret rotation and client revocation/deactivation workflows.

**Scope anchor:** Linear `ENG-1576` (Phase 4 - Advanced Lifecycle).

**Working branch:** `feature/oauth2-client-advanced-lifecycle` (from `main` after PR #63 merge)

## Progress Tracking

### Phase Gate Checklist

| Gate | Status |
|------|--------|
| `specify` | ✅ Done |
| `clarify` | ⏳ In Progress |
| `plan` | ✅ Done |
| `tasks` | ⏳ Pending |
| `implement` | ⏳ Pending |

### Current Status

- The base OAuth2 client management feature is implemented in `feature/oauth2-client-mgmt`.
- This plan only tracks follow-on lifecycle hardening and operator-safe destructive flows.

## Objectives

1. Add an explicit rotation command flow for client secrets with operator confirmation.
2. Add revoke/deactivate/delete flow aligned with available MGR API capabilities.
3. Introduce guardrails that reduce accidental destructive actions.
4. Cover new flows with tests and documentation updates.

## Non-Goals

- Rebuilding existing CRUD/manager/ownership flows from Phase 1-3.
- Changing OAuth2 authentication model or scope definitions in this phase.
- Introducing backend-only features not exposed by current MGR API endpoints.

## Implementation Phases

### Phase 1: Capability Clarification

- Confirm exact backend behavior for:
  - Secret rotation endpoint/response semantics.
  - Revoke/deactivate/delete semantics and recoverability.
  - Required scopes and permission boundaries.
- Record contract notes in this plan before coding.

### Phase 2: CLI UX and Safety Guardrails

- Add/extend commands under `oauth2 client` for advanced lifecycle operations.
- Require explicit confirmation for destructive actions (or a deliberate override flag).
- Keep output machine-readable and consistent with existing command patterns.

### Phase 3: Adapter and Error Mapping Updates

- Update `src/api/mgr_api.rs` if new endpoints or payloads are required.
- Extend `OAuthClientError` mapping only where needed.
- Preserve secret redaction defaults for all outputs and logs.

### Phase 4: Tests and Documentation

- Add/extend unit and integration tests for:
  - Happy path for rotate/revoke/deactivate.
  - Permission and scope failures.
  - Confirmation guardrails and non-interactive behavior.
- Update:
  - `docs/cli-reference.md`
  - `docs/QUICK-REFERENCE.md`
  - relevant skill docs/schemas if command surface changes.

## Acceptance Criteria

- [ ] Rotation flow is implemented and validated end-to-end.
- [ ] Revoke/deactivate/delete flow is implemented according to confirmed backend contract.
- [ ] Destructive operations require explicit operator intent.
- [ ] No plaintext client secret leakage in normal output/log paths.
- [ ] Tests and docs are updated and pass CI quality gates.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Backend contract ambiguity for advanced lifecycle | Wrong CLI behavior | Lock capability matrix before implementation |
| Destructive command misuse | High | Confirmation flow + safe defaults |
| Secret exposure via debug or output paths | High | Keep default redaction and targeted regression tests |

## Linear Mapping

- Primary issue: `ENG-1576`
- Follow-up issues may be split after capability clarification if needed.
