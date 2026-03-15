#!/bin/bash
#
# tests/skills/test_xion_oauth2.sh - OAuth2 Skill Tests
#
# Tests for xion-oauth2 skill including login, status, logout, and refresh operations.
#
# Usage:
#   MOCK_ENABLED=true ./test_xion_oauth2.sh
#
# Environment:
#   MOCK_ENABLED - Enable mock mode for offline testing (default: false)
#   MOCK_DIR     - Directory containing mock JSON files

set -e

# ==============================================================================
# Setup
# ==============================================================================

# Source test library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=./lib.sh
source "$SCRIPT_DIR/lib.sh"

# ==============================================================================
# Test Cases
# ==============================================================================

# Test: OAuth2 status when authenticated
test_oauth2_status_authenticated() {
    local result
    result=$(mock_cli "oauth2" "auth status" "status_authenticated")

    # CLI status doesn't have success field, check authenticated directly
    assert_json_contains "$result" ".authenticated" "true" || return 1
    assert_json_contains "$result" ".network" "testnet" || return 1
    assert_json_has_key "$result" ".xion_address" || return 1
    assert_json_contains "$result" ".chain_id" "xion-testnet-2" || return 1
}

# Test: OAuth2 status when not authenticated
test_oauth2_status_not_authenticated() {
    local result
    result=$(mock_cli "oauth2" "auth status" "status_not_authenticated")

    # Response should have authenticated=false (boolean) with message
    # Note: jq -r converts false to empty string, so we check the raw value
    local auth_val
    auth_val=$(echo "$result" | jq -r '.authenticated')
    if [[ "$auth_val" != "false" ]]; then
        log_error "Expected authenticated=false, got: '$auth_val'"
        return 1
    fi
    assert_json_has_key "$result" ".message" || return 1
    assert_json_contains "$result" ".network" "testnet" || return 1
}

# Test: OAuth2 login success
test_oauth2_login_success() {
    local result
    result=$(mock_cli "oauth2" "auth login" "login_success")

    assert_success "$result" || return 1
    assert_json_has_key "$result" ".xion_address" || return 1
    assert_json_contains "$result" ".network" "testnet" || return 1
    assert_json_has_key "$result" ".expires_at" || return 1

    # Verify xion address format
    local address
    address=$(json_get "$result" ".xion_address")
    if [[ ! "$address" =~ ^xion1[a-z0-9]+$ ]]; then
        log_error "Invalid xion address format: $address"
        return 1
    fi
}

# Test: OAuth2 login failure
test_oauth2_login_failed() {
    local result
    result=$(mock_cli "oauth2" "auth login" "login_failed")

    assert_error "$result" || return 1
    assert_json_contains "$result" ".code" "AUTH_LOGIN_FAILED" || return 1
    assert_json_has_key "$result" ".suggestion" || return 1
}

# Test: OAuth2 login timeout
test_oauth2_login_timeout() {
    local result
    result=$(mock_cli "oauth2" "auth login" "login_timeout")

    assert_error "$result" || return 1
    assert_json_contains "$result" ".code" "AUTH_LOGIN_FAILED" || return 1
    # Check that error contains "timeout" (full message: "Login failed: Authorization timeout")
    local error_msg
    error_msg=$(json_get "$result" ".error")
    if [[ "$error_msg" != *"timeout"* && "$error_msg" != *"Timeout"* ]]; then
        log_error "Expected error to contain 'timeout', got: '$error_msg'"
        return 1
    fi
}

# Test: OAuth2 logout success
test_oauth2_logout_success() {
    local result
    result=$(mock_cli "oauth2" "auth logout" "logout_success")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".message" "Logged out successfully" || return 1
    assert_json_contains "$result" ".network" "testnet" || return 1
}

# Test: OAuth2 logout when already logged out
test_oauth2_logout_already_logged_out() {
    local result
    result=$(mock_cli "oauth2" "auth logout" "logout_already_logged_out")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".message" "Already logged out" || return 1
}

# Test: OAuth2 token refresh success
test_oauth2_refresh_success() {
    local result
    result=$(mock_cli "oauth2" "auth refresh" "refresh_success")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".message" "Token refreshed successfully" || return 1
    assert_json_has_key "$result" ".expires_at" || return 1
    assert_json_contains "$result" ".network" "testnet" || return 1
}

# Test: OAuth2 token refresh failure
test_oauth2_refresh_failed() {
    local result
    result=$(mock_cli "oauth2" "auth refresh" "refresh_failed")

    assert_error "$result" || return 1
    assert_json_contains "$result" ".code" "AUTH_REFRESH_FAILED" || return 1
    assert_json_has_key "$result" ".suggestion" || return 1
}

# Test: OAuth2 refresh when not authenticated
test_oauth2_refresh_not_authenticated() {
    local result
    result=$(mock_cli "oauth2" "auth refresh" "refresh_not_authenticated")

    assert_error "$result" || return 1
    assert_json_contains "$result" ".code" "AUTH_REFRESH_FAILED" || return 1
    # Check that error contains "credentials" (full message: "Token refresh failed: No credentials found")
    local error_msg
    error_msg=$(json_get "$result" ".error")
    if [[ "$error_msg" != *"credentials"* ]]; then
        log_error "Expected error to contain 'credentials', got: '$error_msg'"
        return 1
    fi
}

# Test: OAuth2 status for mainnet
test_oauth2_status_authenticated_mainnet() {
    local result
    result=$(mock_cli "oauth2" "auth status" "status_authenticated_mainnet")

    assert_json_contains "$result" ".authenticated" "true" || return 1
    assert_json_contains "$result" ".network" "mainnet" || return 1
    assert_json_contains "$result" ".chain_id" "xion-mainnet-1" || return 1
}

# Test: CLI not found error
test_oauth2_cli_not_found() {
    local result
    result=$(mock_cli "oauth2" "auth status" "cli_not_found")

    assert_error "$result" || return 1
    assert_json_contains "$result" ".error.code" "ECONFIG002" || return 1
    # Check retryable boolean (jq -r converts false to empty, so use raw jq)
    local retryable
    retryable=$(echo "$result" | jq '.error.retryable')
    if [[ "$retryable" != "false" ]]; then
        log_error "Expected error.retryable=false, got: '$retryable'"
        return 1
    fi
}

# ==============================================================================
# Main
# ==============================================================================

main() {
    log_info "Running OAuth2 Skill Tests"
    log_info "Mock mode: ${MOCK_ENABLED}"
    log_info "Mock dir: ${MOCK_DIR}"

    # Check if in mock mode
    if [[ "$MOCK_ENABLED" != "true" ]]; then
        log_warn "Not in mock mode. Some tests may fail or require real authentication."
        log_warn "Run with: MOCK_ENABLED=true $0"
    fi

    # Run all test cases
    run_test "test_oauth2_status_authenticated" test_oauth2_status_authenticated
    run_test "test_oauth2_status_not_authenticated" test_oauth2_status_not_authenticated
    run_test "test_oauth2_login_success" test_oauth2_login_success
    run_test "test_oauth2_login_failed" test_oauth2_login_failed
    run_test "test_oauth2_login_timeout" test_oauth2_login_timeout
    run_test "test_oauth2_logout_success" test_oauth2_logout_success
    run_test "test_oauth2_logout_already_logged_out" test_oauth2_logout_already_logged_out
    run_test "test_oauth2_refresh_success" test_oauth2_refresh_success
    run_test "test_oauth2_refresh_failed" test_oauth2_refresh_failed
    run_test "test_oauth2_refresh_not_authenticated" test_oauth2_refresh_not_authenticated
    run_test "test_oauth2_status_authenticated_mainnet" test_oauth2_status_authenticated_mainnet
    run_test "test_oauth2_cli_not_found" test_oauth2_cli_not_found

    # Print summary and exit
    test_exit
}

# Run main function
main "$@"
