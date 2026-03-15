#!/bin/bash
#
# Xion Agent Toolkit - Skills Test Framework Library
# Provides mock control, assertions, and test runner utilities
#
# Usage: source tests/skills/lib.sh
#

set -euo pipefail

# =============================================================================
# Environment Configuration
# =============================================================================

# Mock control
MOCK_ENABLED="${MOCK_ENABLED:-false}"
MOCK_DIR="${MOCK_DIR:-$(dirname "${BASH_SOURCE[0]}")/mocks}"

# Test state
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0
CURRENT_TEST=""

# Colors for output (disabled in CI)
if [[ -z "${CI:-}" ]]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    CYAN='\033[0;36m'
    NC='\033[0m' # No Color
else
    GREEN=''
    RED=''
    YELLOW=''
    BLUE=''
    CYAN=''
    NC=''
fi

# =============================================================================
# Logging Functions
# =============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" >&2
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1" >&2
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1" >&2
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" >&2
}

log_skip() {
    echo -e "${CYAN}[SKIP]${NC} $1" >&2
}

# =============================================================================
# Mock System
# =============================================================================

# mock_cli - Returns mock response if enabled, else runs real CLI
#
# Usage: mock_cli <skill> <cli_command> <mock_key> [extra_args...]
#
# Arguments:
#   skill      - Skill name (e.g., "oauth2", "treasury")
#   command    - CLI command to run (e.g., "auth status", "treasury list")
#   mock_key   - Key in mock JSON file (e.g., "status_authenticated")
#   extra_args - Additional arguments to pass to CLI
#
# Examples:
#   mock_cli "oauth2" "auth status" "status_authenticated"
#   mock_cli "treasury" "treasury list" "list_empty" "--network testnet"
#
# Environment:
#   MOCK_ENABLED=true - Enable mock mode
#   MOCK_DIR          - Directory containing mock JSON files
#
mock_cli() {
    local skill="$1"
    local command="$2"
    local mock_key="$3"
    shift 3
    local extra_args=("$@")

    if [[ "$MOCK_ENABLED" == "true" ]]; then
        local mock_file="$MOCK_DIR/${skill}-responses.json"
        
        if [[ ! -f "$mock_file" ]]; then
            echo "{\"success\": false, \"error\": \"Mock file not found: $mock_file\", \"error_code\": \"MOCK_FILE_NOT_FOUND\"}"
            return 1
        fi
        
        local mock_response
        mock_response=$(jq -r ".$mock_key // empty" "$mock_file" 2>/dev/null)
        
        if [[ -z "$mock_response" || "$mock_response" == "null" ]]; then
            echo "{\"success\": false, \"error\": \"Mock key not found: $mock_key\", \"error_code\": \"MOCK_KEY_NOT_FOUND\"}"
            return 1
        fi
        
        echo "$mock_response"
        return 0
    fi

    # Run actual CLI command
    local full_cmd="xion-toolkit $command --output json ${extra_args[*]}"
    eval "$full_cmd" 2>&1
}

# mock_response - Get a specific mock response value
#
# Usage: mock_response <skill> <mock_key>
#
mock_response() {
    local skill="$1"
    local mock_key="$2"
    local mock_file="$MOCK_DIR/${skill}-responses.json"
    
    if [[ ! -f "$mock_file" ]]; then
        echo ""
        return 1
    fi
    
    jq -r ".$mock_key // empty" "$mock_file" 2>/dev/null
}

# =============================================================================
# JSON Parsing Utilities
# =============================================================================

# Extract a JSON field value using jq
#
# Usage: json_get <json_string> <field_path>
#
# Examples:
#   json_get "$output" ".success"
#   json_get "$output" ".error.code"
#   json_get "$output" ".data.treasuries[0].address"
#
json_get() {
    local json="$1"
    local field="$2"
    
    echo "$json" | jq -r "$field // empty" 2>/dev/null
}

# Check if JSON has a specific key
#
# Usage: json_has_key <json_string> <key_path>
#
json_has_key() {
    local json="$1"
    local key="$2"
    
    local value
    value=$(echo "$json" | jq -r "$key // empty" 2>/dev/null)
    
    [[ -n "$value" && "$value" != "null" ]]
}

# Check if JSON contains expected value at path
#
# Usage: json_equals <json_string> <key_path> <expected_value>
#
json_equals() {
    local json="$1"
    local key="$2"
    local expected="$3"
    
    local actual
    actual=$(json_get "$json" "$key")
    
    [[ "$actual" == "$expected" ]]
}

# =============================================================================
# Assertion Functions
# =============================================================================

# assert_success - Check that response indicates success
#
# Usage: assert_success <json_response>
#
# Checks:
#   - JSON is valid
#   - .success == true
#
assert_success() {
    local json="$1"
    
    # Validate JSON
    if ! echo "$json" | jq -e '.' >/dev/null 2>&1; then
        log_error "Invalid JSON response"
        echo "Response: $json" >&2
        return 1
    fi
    
    # Check success field
    local success
    success=$(json_get "$json" ".success")
    
    if [[ "$success" != "true" ]]; then
        log_error "Expected success=true, got: $success"
        local error_msg
        error_msg=$(json_get "$json" ".error")
        if [[ -n "$error_msg" ]]; then
            echo "Error: $error_msg" >&2
        fi
        return 1
    fi
    
    return 0
}

# assert_error - Check that response indicates an error
#
# Usage: assert_error <json_response>
#
# Checks:
#   - JSON is valid
#   - .success == false
#
assert_error() {
    local json="$1"
    
    # Validate JSON
    if ! echo "$json" | jq -e '.' >/dev/null 2>&1; then
        log_error "Invalid JSON response"
        echo "Response: $json" >&2
        return 1
    fi
    
    # Check success field
    local success
    success=$(json_get "$json" ".success")
    
    if [[ "$success" == "true" ]]; then
        log_error "Expected failure, but got success"
        return 1
    fi
    
    return 0
}

# assert_error_code - Check that error has specific code
#
# Usage: assert_error_code <json_response> <expected_code>
#
# Checks:
#   - .success == false
#   - .error_code == expected_code OR .error.code == expected_code
#
assert_error_code() {
    local json="$1"
    local expected_code="$2"
    
    # First verify it's an error
    if ! assert_error "$json"; then
        return 1
    fi
    
    # Check error_code field (both formats supported)
    local actual_code
    actual_code=$(json_get "$json" ".error_code")
    
    if [[ -z "$actual_code" || "$actual_code" == "null" ]]; then
        actual_code=$(json_get "$json" ".error.code")
    fi
    
    if [[ "$actual_code" != "$expected_code" ]]; then
        log_error "Expected error_code=$expected_code, got: $actual_code"
        return 1
    fi
    
    return 0
}

# assert_json_contains - Check that JSON path equals expected value
#
# Usage: assert_json_contains <json_response> <jq_path> <expected_value>
#
# Examples:
#   assert_json_contains "$output" ".authenticated" "true"
#   assert_json_contains "$output" ".network" "testnet"
#   assert_json_contains "$output" ".data.items | length" "5"
#
assert_json_contains() {
    local json="$1"
    local path="$2"
    local expected="$3"
    
    # Validate JSON
    if ! echo "$json" | jq -e '.' >/dev/null 2>&1; then
        log_error "Invalid JSON response"
        echo "Response: $json" >&2
        return 1
    fi
    
    local actual
    actual=$(json_get "$json" "$path")
    
    if [[ "$actual" != "$expected" ]]; then
        log_error "Expected $path='$expected', got: '$actual'"
        return 1
    fi
    
    return 0
}

# assert_json_has_key - Check that JSON has a specific key
#
# Usage: assert_json_has_key <json_response> <key_path>
#
# Examples:
#   assert_json_has_key "$output" ".authenticated"
#   assert_json_has_key "$output" ".token_info.expires_at"
#
assert_json_has_key() {
    local json="$1"
    local key="$2"
    
    # Validate JSON
    if ! echo "$json" | jq -e '.' >/dev/null 2>&1; then
        log_error "Invalid JSON response"
        echo "Response: $json" >&2
        return 1
    fi
    
    if ! json_has_key "$json" "$key"; then
        log_error "Expected key not found: $key"
        return 1
    fi
    
    return 0
}

# assert_json_array_length - Check that JSON array has specific length
#
# Usage: assert_json_array_length <json_response> <array_path> <expected_length>
#
# Examples:
#   assert_json_array_length "$output" ".treasuries" "5"
#
assert_json_array_length() {
    local json="$1"
    local path="$2"
    local expected="$3"
    
    local actual
    actual=$(json_get "$json" "$path | length")
    
    if [[ "$actual" != "$expected" ]]; then
        log_error "Expected array $path length=$expected, got: $actual"
        return 1
    fi
    
    return 0
}

# assert_json_matches - Check that JSON matches a pattern
#
# Usage: assert_json_matches <json_response> <jq_filter> <expected_pattern>
#
# Examples:
#   assert_json_matches "$output" ".address" "^xion1[a-z0-9]+$"
#
assert_json_matches() {
    local json="$1"
    local path="$2"
    local pattern="$3"
    
    local actual
    actual=$(json_get "$json" "$path")
    
    if ! echo "$actual" | grep -qE "$pattern"; then
        log_error "Expected $path to match pattern '$pattern', got: '$actual'"
        return 1
    fi
    
    return 0
}

# =============================================================================
# Test Runner Functions
# =============================================================================

# run_test - Run a single test with GitHub Actions grouping
#
# Usage: run_test <test_name> <test_function>
#
# Features:
#   - GitHub Actions grouping (::group:: / ::endgroup::)
#   - Test state tracking
#   - Pass/fail reporting
#
run_test() {
    local name="$1"
    local func="$2"
    
    CURRENT_TEST="$name"
    ((TESTS_RUN++)) || true
    
    # GitHub Actions grouping
    echo "::group::$name"
    
    # Run test function
    if $func; then
        log_success "$name PASSED"
        ((TESTS_PASSED++)) || true
        echo "::endgroup::"
        return 0
    else
        log_error "$name FAILED"
        ((TESTS_FAILED++)) || true
        echo "::endgroup::"
        return 1
    fi
}

# run_test_suite - Run all test functions in a file
#
# Usage: run_test_suite <test_file>
#
# Discovers and runs all functions starting with "test_"
#
run_test_suite() {
    local test_file="$1"
    
    # Source the test file
    # shellcheck source=/dev/null
    source "$test_file"
    
    # Find all test functions
    local test_functions
    test_functions=$(grep -E "^test_[a-zA-Z0-9_]+\(\)" "$test_file" | sed 's/().*//' | sort)
    
    if [[ -z "$test_functions" ]]; then
        log_warn "No test functions found in $test_file"
        return 0
    fi
    
    log_info "Running tests from: $test_file"
    
    local failed=0
    for func in $test_functions; do
        if ! run_test "$func" "$func"; then
            failed=1
        fi
    done
    
    return $failed
}

# =============================================================================
# Test Summary and Exit
# =============================================================================

# print_summary - Print test results summary
#
print_summary() {
    echo ""
    echo "========================================"
    echo "Test Summary"
    echo "========================================"
    echo -e "Total:  ${TESTS_RUN}"
    echo -e "Passed: ${GREEN}${TESTS_PASSED}${NC}"
    echo -e "Failed: ${RED}${TESTS_FAILED}${NC}"
    echo "========================================"
    
    if [[ $TESTS_FAILED -gt 0 ]]; then
        return 1
    fi
    return 0
}

# test_exit - Exit with appropriate code based on test results
#
test_exit() {
    print_summary
    
    if [[ $TESTS_FAILED -gt 0 ]]; then
        exit 1
    fi
    exit 0
}

# =============================================================================
# Setup and Teardown Hooks
# =============================================================================

# These can be overridden in test files

setup() {
    : # Default: no-op
}

teardown() {
    : # Default: no-op
}

# run_with_hooks - Run a test with setup/teardown
#
# Usage: run_with_hooks <test_function>
#
run_with_hooks() {
    local func="$1"
    
    setup
    local result=0
    if ! $func; then
        result=1
    fi
    teardown
    
    return $result
}

# =============================================================================
# CLI Command Helpers
# =============================================================================

# run_cli - Run CLI command and capture output
#
# Usage: run_cli <command_args...>
#
# Returns: JSON output to stdout, logs to stderr
#
run_cli() {
    local cmd="xion-toolkit $* --output json"
    
    local output
    output=$(eval "$cmd" 2>&1) || {
        echo "{\"success\": false, \"error\": \"Command failed\", \"output\": $(echo "$output" | jq -Rs '.')}"
        return 1
    }
    
    # Extract JSON from output (handle mixed stdout/stderr)
    local json
    json=$(echo "$output" | grep -E '^\{.*\}$' | tail -1)
    
    if [[ -z "$json" ]]; then
        # Try to parse entire output as JSON
        if echo "$output" | jq -e '.' >/dev/null 2>&1; then
            json="$output"
        else
            echo "{\"success\": false, \"error\": \"No valid JSON in output\", \"raw\": $(echo "$output" | jq -Rs '.')}"
            return 1
        fi
    fi
    
    echo "$json"
}

# =============================================================================
# Utility Functions
# =============================================================================

# skip_test - Mark a test as skipped
#
# Usage: skip_test <reason>
#
skip_test() {
    local reason="$1"
    log_skip "$CURRENT_TEST: $reason"
    return 0
}

# require_env - Check required environment variable
#
# Usage: require_env <var_name> [error_message]
#
require_env() {
    local var_name="$1"
    local error_msg="${2:-Missing required environment variable: $var_name}"
    
    if [[ -z "${!var_name:-}" ]]; then
        log_error "$error_msg"
        return 1
    fi
    return 0
}

# require_command - Check required command is available
#
# Usage: require_command <command_name>
#
require_command() {
    local cmd="$1"
    
    if ! command -v "$cmd" >/dev/null 2>&1; then
        log_error "Required command not found: $cmd"
        return 1
    fi
    return 0
}

# require_mock_mode - Skip test if not in mock mode
#
# Usage: require_mock_mode
#
require_mock_mode() {
    if [[ "$MOCK_ENABLED" != "true" ]]; then
        skip_test "Test requires mock mode (MOCK_ENABLED=true)"
        return 0
    fi
    return 0
}

# require_e2e_mode - Skip test if in mock mode
#
# Usage: require_e2e_mode
#
require_e2e_mode() {
    if [[ "$MOCK_ENABLED" == "true" ]]; then
        skip_test "Test requires E2E mode (MOCK_ENABLED=false or unset)"
        return 0
    fi
    return 0
}

# =============================================================================
# Initialization Check
# =============================================================================

# Verify dependencies
if ! command -v jq >/dev/null 2>&1; then
    echo "[ERROR] jq is required but not installed" >&2
    exit 1
fi

# Verify mock directory exists in mock mode
if [[ "$MOCK_ENABLED" == "true" && ! -d "$MOCK_DIR" ]]; then
    log_warn "MOCK_DIR does not exist: $MOCK_DIR"
fi