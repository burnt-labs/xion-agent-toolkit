---
status: Done
created_at: 2026-03-10
updated_at: 2026-03-10
done_at: 2026-03-10
---

# Integration Tests Fix

## Background

Integration tests were disabled due to a mockito/tokio runtime conflict. The mockito library doesn't work well with tokio's async runtime.

## Goal
Replace mockito with wiremock to OAuth2 API integration tests. wiremock is async-friendly and designed for tokio.

## Approach
1. Add wiremock dependency to Cargo.toml (dev-dependencies)
2. Remove mockito dependency (replacement, not addition)
3. Rewrite integration tests using wiremock's async matchers
4. Test token exchange, refresh token, and user info endpoints

## Tasks

- [x] Add wiremock dependency
- [x] Remove mockito dependency
- [x] Rewrite integration tests (partially - has syntax errors)
- [x] Fix syntax errors in oauth2_api.rs (extra `}` at line 777)
- [x] Move orphaned tests inside integration_tests module
- [x] Verify all tests pass

## Issues Found (2026-03-10 QA Review)

| Issue | Severity | Location |
|-------|----------|----------|
| Extra `}` causing syntax error | Critical | src/api/oauth2_api.rs:777 |
| Orphaned tests outside module | High | test_get_user_info_success, test_oauth2_error_response |
| mockito not removed | Medium | Cargo.toml |

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-10 | @qa-engineer | All 328 tests pass including 3 new integration tests | ~~Approved~~ **Invalid** - code doesn't compile |
| 2026-03-10 | @qa-engineer | Fixed syntax errors, migrated to wiremock, all tests pass (331 tests) | **Approved** |
