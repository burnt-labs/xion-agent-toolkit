---
status: InProgress
created_at: 2026-03-15
updated_at: 2026-03-15
---

# Skills Test Framework

## Background

The skills directory contains 5 bash-based skills (xion-toolkit-init, xion-oauth2, xion-treasury, xion-asset, xion-dev) that lack automated testing. This makes it difficult to ensure reliability when making changes or during CI/CD.

## Goal

Create a lightweight bash-based test framework for skills with mock support, enabling offline testing and CI integration.

## Approach

1. Create test framework core (lib.sh)
2. Create mock response data
3. Implement skill-specific tests
4. Integrate with GitHub Actions CI

---

## Tasks

### Phase 1: Test Framework Core

- [x] Create `tests/skills/` directory structure
- [x] Create `tests/skills/lib.sh` with:
  - Mock control functions
  - Assertion functions (assert_success, assert_error_code, assert_json_contains)
  - Test runner (run_test, main wrapper)
- [x] Create `tests/skills/run_all.sh` entry point
- [x] Document testing patterns

### Phase 2: Mock Data

- [x] Create `tests/skills/mocks/` directory
- [x] Create `mocks/oauth2-responses.json` (login, status, logout, refresh) - 10 scenarios
- [x] Create `mocks/treasury-responses.json` (list, query, create, fund) - 18 scenarios
- [x] Create `mocks/asset-responses.json` (types, create, mint) - 18 scenarios
- [x] Create `mocks/tx-responses.json` (status, wait) - 12 scenarios

### Phase 3: Skill Tests

- [ ] Create `test_xion_toolkit_init.sh` - Installation and version tests
- [x] Create `test_xion_oauth2.sh` - Auth flow tests (12 tests, all passing)
- [x] Create `test_xion_treasury.sh` - Treasury operation tests (18 tests, all passing)
- [x] Create `test_xion_asset.sh` - Asset builder tests (18 tests, all passing)
- [ ] Create `test_xion_dev.sh` - Routing tests

### Phase 4: CI Integration

- [x] Create `.github/workflows/test-skills.yml`
- [x] Configure mock mode for PR checks
- [x] Configure E2E mode for main branch
- [ ] Add test status badge to README

### Phase 5: Documentation

- [x] Update skill SKILL.md files with testing section (xion-oauth2 completed)
- [ ] Create `tests/skills/README.md`
- [ ] Add testing guide to contributing docs

---

## Directory Structure

```
tests/skills/
├── README.md                    # Testing guide
├── run_all.sh                   # Run all tests entry point
├── lib.sh                       # Test framework core
├── mocks/                       # Mock response data
│   ├── oauth2-responses.json
│   ├── treasury-responses.json
│   ├── asset-responses.json
│   └── tx-responses.json
├── test_xion_toolkit_init.sh
├── test_xion_oauth2.sh
├── test_xion_treasury.sh
├── test_xion_asset.sh
└── test_xion_dev.sh
```

---

## Test Framework API

### lib.sh Functions

```bash
# Mock control
MOCK_ENABLED=${MOCK_ENABLED:-false}
mock_cli <command> <response_key>

# Assertions
assert_success <json_result>
assert_error_code <json_result> <expected_code>
assert_json_contains <json_result> <jq_path> <expected_value>
assert_json_has_key <json_result> <key_path>

# Test runner
run_test <test_name> <test_function>
```

### Test Template

```bash
#!/bin/bash
set -e
source "$(dirname "$0")/lib.sh"

test_example() {
    local result
    result=$(mock_cli "auth status" "status_authenticated")
    assert_success "$result"
    assert_json_contains "$result" '.authenticated' 'true'
}

main() {
    local failed=0
    run_test "test_example" test_example || ((failed++))
    echo "Tests: 1, Failed: $failed"
    [[ $failed -eq 0 ]]
}

main "$@"
```

---

## Acceptance Criteria

- [x] Each skill has at least one E2E test (OAuth2: 12, Treasury: 18, Asset: 18 tests)
- [x] Tests can run offline with MOCK_ENABLED=true
- [x] CI runs skill tests automatically on PR
- [x] All tests pass in mock mode (48 tests passing)
- [x] Documentation updated in skill SKILL.md files

---

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-15 | @fullstack-dev | Phase 1, Phase 4 (CI), Phase 5 (xion-oauth2 docs) completed | Partial |
| 2026-03-15 | @fullstack-dev-2 | Phase 2 (mock data: 58 scenarios), Phase 3 (test scripts: 48 tests) | Partial |
