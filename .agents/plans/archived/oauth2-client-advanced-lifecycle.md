---
status: Done
created_at: 2026-04-01
updated_at: 2026-04-01
done_at: 2026-04-01
---

# OAuth2 Client Advanced Lifecycle Plan

**Goal:** Add safety guardrails for destructive OAuth2 client operations and improve test coverage for CLI handlers.

**Scope anchor:** Linear `ENG-1576` (Phase 4 - Advanced Lifecycle, revised after capability clarification).

**Working branch:** `feature/oauth2-client-mgmt` (continuing on existing branch).

## Progress Tracking

### Phase Gate Checklist

| Gate | Status |
|------|--------|
| `specify` | ✅ Done |
| `clarify` | ✅ Done |
| `plan` | ✅ Done (revised) |
| `tasks` | ✅ Done |
| `implement` | ✅ Done (residual fixes + docs/skills update) |
| `review` | ✅ Done (QC tri-review: Approve with residuals, all fixed) |
| `done` | ✅ Done |

### Current Status

- Capability clarification complete — see "Backend Contract" section below.
- Scope revised based on backend API audit: secret rotation and revoke/deactivate do NOT exist in the backend.
- Focus narrowed to safety guardrails + test coverage.

## Backend Contract (Clarification Results)

Audit of `oauth2-api-service` (Cloudflare Workers + Hono + `@cloudflare/workers-oauth-provider`):

| Capability | Backend Status |
|---|---|
| Secret Rotation | ❌ Does NOT exist. Secret returned only on creation, never again. No regenerate endpoint. |
| Client Revoke/Deactivate | ❌ Does NOT exist. Only hard delete via `DELETE /mgr-api/clients/{id}`. No soft delete, no status concept. |
| Client Status (active/disabled) | ❌ Does NOT exist. Client is either present or absent. |
| Delete (hard) | ✅ `DELETE /mgr-api/clients/{id}`, owner-only, no confirmation mechanism. |
| Transfer Ownership | ✅ `POST /mgr-api/clients/{id}/transfer-ownership`, owner-only, no confirmation mechanism. |

**Decision:** Secret rotation and revoke/deactivate deferred — requires backend API changes. See "Deferred Items" below.

## Objectives (Revised)

1. Add `--force` confirmation guard to `oauth2 client delete` — refuse execution without explicit `--force` flag.
2. Add `--force` confirmation guard to `oauth2 client transfer-ownership` — refuse execution without explicit `--force` flag.
3. Add CLI handler integration tests for `delete` and `transfer-ownership` commands (existing tests are arg-parsing only).
4. Add CLI handler integration tests for other key commands where missing.

## Non-Goals

- Secret rotation (deferred — backend API does not exist).
- Client revoke/deactivate/status (deferred — backend API does not exist).
- Rebuilding existing CRUD/manager/ownership flows from Phase 1-3.
- Changing OAuth2 authentication model or scope definitions.

## Deferred Items

| Item | Reason | Tracking |
|---|---|---|
| Secret rotation | Backend has no rotation API. Compound delete+recreate workflow rejected due to non-atomic data loss risk. | Backend follow-up issue (post ENG-1576) |
| Client revoke/deactivate | Backend has no soft-delete or status concept. | Backend follow-up issue (post ENG-1576) |

## Implementation Tasks

### Task 1: `--force` Guard for `delete` Command

**File:** `src/cli/oauth2_client.rs`
- Add `--force` flag to the `Delete` subcommand struct.
- In `handle_delete()`: if `--force` is not set, print a warning message to stderr and return an error (exit code) instructing the user to re-run with `--force`.
- When `--force` is set, proceed with existing delete logic unchanged.
- Add a new error variant `ConfirmationRequired` to `OAuthClientError` with code `EOAUTHCLIENT019`.
- Map to exit code `178` in `src/shared/exit_codes.rs`.

### Task 2: `--force` Guard for `transfer-ownership` Command

**File:** `src/cli/oauth2_client.rs`
- Add `--force` flag to the `TransferOwnership` subcommand struct.
- Same guard pattern as Task 1: refuse without `--force`, warn on stderr.
- Reuse `EOAUTHCLIENT019` / exit code `178` (same semantic — confirmation required for destructive operation).

### Task 3: CLI Handler Integration Tests

**File:** `src/cli/oauth2_client.rs` (tests module)
- Add wiremock-based handler tests for:
  - `delete` with `--force` → succeeds (API call made)
  - `delete` without `--force` → returns error
  - `transfer-ownership` with `--force` → succeeds
  - `transfer-ownership` without `--force` → returns error
- Extend arg-parsing tests to verify `--force` flag is accepted.

### Task 4: Error Code and Exit Code Registration

**Files:** `src/shared/error.rs`, `src/shared/exit_codes.rs`
- Add `ConfirmationRequired { message }` variant to `OAuthClientError` with code `EOAUTHCLIENT019`.
- Add exit code `178` = `OAUTH_CLIENT_CONFIRMATION_REQUIRED` in exit codes.
- Update error mapping in `map_error()` if needed (backend may return specific codes for this, but currently the guard is CLI-side only).

## Acceptance Criteria

- [x] `oauth2 client delete CLIENT_ID` refuses without `--force`, prints warning to stderr.
- [x] `oauth2 client delete CLIENT_ID --force` executes normally.
- [x] `oauth2 client transfer-ownership CLIENT_ID --new-owner UID` refuses without `--force`, prints warning to stderr.
- [x] `oauth2 client transfer-ownership CLIENT_ID --new-owner UID --force` executes normally.
- [x] Error code `EOAUTHCLIENT019` and exit code `178` are registered and documented.
- [x] All new paths have integration test coverage.
- [x] `cargo fmt`, `cargo clippy`, `cargo test` all pass.
- [x] Skills (SKILL.md + JSON schemas) updated with `--force` / `force` parameter.
- [x] CLI docs (cli-reference.md, QUICK-REFERENCE.md) updated with `--force`.
- [x] No stale references to delete/transfer-ownership without `--force` in skills/docs.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| `--force` flag breaks existing scripts/agents that call delete/transfer-ownership | Medium | Clear error message tells user to add `--force`; update any internal scripts |
| Existing E2E/skill tests may fail if they call delete without `--force` | Low | Update test invocations to include `--force` |

## Linear Mapping

- Primary issue: `ENG-1576` (revised scope)
- Deferred: secret rotation, revoke/deactivate → backend follow-up issues
