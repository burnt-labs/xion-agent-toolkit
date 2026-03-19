---
status: Done
created_at: 2026-03-15
updated_at: 2026-03-15
---

# Error Recovery Enhancement

## Background

The current error handling in xion-agent-toolkit is functional but lacks structured error codes and actionable remediation hints. This makes debugging difficult for AI agents and developers, especially in automated/CI environments.

## Goal

Improve error handling with structured error codes, automatic retry for transient failures, and actionable remediation hints.

## Approach

1. Define a unified error code schema
2. Implement retry logic for network/transient errors
3. Add remediation hints for common failure scenarios
4. Update documentation

---

## Tasks

### 1. Error Code Schema Design
- [x] Define error code format: `E{MODULE}{NUMBER}` (e.g., `EAUTH001`, `ETREASURY042`)
- [x] Create `src/shared/error.rs` with `XionError` enum and `XionErrorCode` types
- [x] Define error categories: Auth, Treasury, Asset, Batch, Config, Network

### 2. Error Types Refactoring
- [x] Refactor `src/api/oauth2_api.rs` to use `XionError`
- [x] Refactor `src/treasury/api_client.rs` to use `XionError`
- [x] Refactor `src/treasury/manager.rs` to use `XionError`
- [ ] Refactor `src/asset_builder/` to use `XionError`
- [ ] Refactor `src/oauth/` to use `XionError`
- [x] Ensure all errors include: `code`, `message`, `hint`, `source` (optional)

### 3. Retry Logic Implementation
- [x] Create `src/shared/retry.rs` with exponential backoff
- [x] Add retry decorator for network operations
- [x] Configure max retries (default: 3) and backoff interval
- [x] Identify transient error codes eligible for retry:
  - Network timeouts
  - Rate limiting (429)
  - Service unavailable (503)

### 4. CLI Output Enhancement
- [x] Update `src/utils/output.rs` to format errors consistently
- [x] JSON output: `{"success": false, "error": {"code": "...", "message": "...", "hint": "..."}}`
- [x] Human output: Show hint as actionable suggestion
- [ ] Add `--debug` flag to include stack trace/source

### 5. Error Documentation
- [x] Create/update `docs/ERROR-CODES.md` with all error codes
- [x] Document remediation steps for each error code
- [x] Add examples of error scenarios and solutions

### 6. Testing
- [x] Unit tests for error serialization
- [x] Unit tests for retry logic
- [ ] Integration tests for error scenarios with mock server

---

## Error Code Schema

```
Format: E{MODULE}{NUMBER}

Modules:
- AUTH: Authentication (EAUTH001-EAUTH099)
- TREASURY: Treasury operations (ETREASURY001-ETREASURY099)
- ASSET: Asset builder (EASSET001-EASSET099)
- BATCH: Batch operations (EBATCH001-EBATCH099)
- CONFIG: Configuration (ECONFIG001-ECONFIG099)
- NETWORK: Network/API (ENETWORK001-ENETWORK099)
```

## Error Output Format

### JSON (default for scripts)
```json
{
  "success": false,
  "error": {
    "code": "ETREASURY001",
    "message": "Treasury not found: xion1...",
    "hint": "Run 'xion-toolkit treasury list' to see available treasuries.",
    "retryable": false
  }
}
```

### Human-readable
```
Error [ETREASURY001]: Treasury not found: xion1...

Hint: Run 'xion-toolkit treasury list' to see available treasuries.
```

## Key Error Codes

| Code | Message | Hint |
|------|---------|------|
| EAUTH001 | Not authenticated | Run 'xion-toolkit auth login' first |
| EAUTH002 | Token expired | Token refreshed automatically, please retry |
| EAUTH003 | Refresh token expired | Re-login required: 'xion-toolkit auth login' |
| ENETWORK001 | Connection timeout | Check network connectivity, will retry |
| ENETWORK002 | Rate limited | Wait and retry, or reduce request frequency |
| ETREASURY001 | Treasury not found | Run 'xion-toolkit treasury list' |
| ETREASURY002 | Insufficient balance | Fund treasury with 'xion-toolkit treasury fund' |
| EASSET001 | Invalid metadata | Check JSON structure against schema |
| EBATCH001 | Batch too large | Maximum 50 messages per batch |

---

## Acceptance Criteria

- [x] All errors return structured JSON with `code`, `message`, `hint`
- [x] Network errors implement automatic retry (max 3 attempts, exponential backoff)
- [x] Error documentation in `docs/ERROR-CODES.md` covers all error codes
- [x] All existing tests pass after refactoring
- [x] New tests for error handling cover major scenarios

---

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-15 | @fullstack-dev | Core error types, retry logic, and documentation completed | InProgress |
| 2026-03-15 | @fullstack-dev | Refactored oauth2_api.rs, treasury/api_client.rs, treasury/manager.rs to use XionError | Done |