# xion-oauth2 Testing

This skill includes comprehensive test coverage using the Skills Test Framework.

## Test File

- `tests/skills/test_oauth2.sh` - OAuth2 skill test suite

## Running Tests

### Mock Mode (Fast, No Network Required)

```bash
# Run OAuth2 tests with mock responses
MOCK_ENABLED=true ./tests/skills/run_all.sh oauth2

# Or run all skill tests
MOCK_ENABLED=true ./tests/skills/run_all.sh
```

### E2E Mode (Requires Network and Credentials)

```bash
# Run with real CLI (requires xion-toolkit installed and configured)
./tests/skills/run_all.sh oauth2

# Requires valid credentials for testnet
# Credentials should be in ~/.xion-toolkit/credentials/testnet.enc
```

## Test Coverage

| Function | Mock Test | E2E Test |
|----------|-----------|----------|
| `login.sh` | ✅ | ⚠️ (requires browser) |
| `status.sh` (authenticated) | ✅ | ✅ |
| `status.sh` (not authenticated) | ✅ | ✅ |
| `logout.sh` | ✅ | ✅ |
| `refresh.sh` | ✅ | ✅ |
| Error handling | ✅ | ✅ |

## Mock Responses

Mock responses are defined in `tests/skills/mocks/oauth2-responses.json`:

```json
{
  "status_authenticated": {
    "success": true,
    "authenticated": true,
    "network": "testnet",
    "token_info": {
      "expires_at": "2024-01-15T10:30:00Z",
      "expires_in_seconds": 3600,
      "needs_refresh": false
    }
  },
  "status_not_authenticated": {
    "success": true,
    "authenticated": false,
    "network": "testnet",
    "message": "No credentials found"
  },
  "logout_success": {
    "success": true,
    "message": "Successfully logged out from testnet"
  },
  "refresh_success": {
    "success": true,
    "message": "Token refreshed successfully",
    "expires_at": "2024-01-15T11:30:00Z",
    "expires_in_seconds": 3600
  }
}
```

## Writing New Tests

Tests use the framework from `tests/skills/lib.sh`:

```bash
#!/bin/bash
set -euo pipefail

# Source test framework
source "$(dirname "$0")/lib.sh"

# Test: Status returns valid JSON when authenticated
test_status_authenticated() {
    local output
    output=$(mock_cli "oauth2" "auth status" "status_authenticated")
    
    assert_success "$output"
    assert_json_contains "$output" ".authenticated" "true"
    assert_json_has_key "$output" ".token_info.expires_at"
}

# Test: Status returns not authenticated when no credentials
test_status_not_authenticated() {
    local output
    output=$(mock_cli "oauth2" "auth status" "status_not_authenticated")
    
    assert_success "$output"
    assert_json_contains "$output" ".authenticated" "false"
}

# Run tests if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    run_test "test_status_authenticated" test_status_authenticated
    run_test "test_status_not_authenticated" test_status_not_authenticated
    test_exit
fi
```

## CI Integration

Tests run automatically in GitHub Actions:

- **Mock Tests**: Run on every PR and push to main/develop
- **E2E Tests**: Run on main branch with secrets (testnet credentials)
- **Lint**: Shell scripts are checked with shellcheck

See `.github/workflows/test-skills.yml` for configuration.

## Debugging Failed Tests

```bash
# Run with verbose output
bash -x tests/skills/test_oauth2.sh

# Check JSON output manually
MOCK_ENABLED=true bash -c 'source tests/skills/lib.sh; mock_cli "oauth2" "auth status" "status_authenticated"'

# Validate mock response file
jq '.' tests/skills/mocks/oauth2-responses.json
```
