#!/bin/bash
#
# tests/skills/test_xion_treasury.sh - Treasury Skill Tests
#
# Tests for xion-treasury skill including list, query, create, fund, and withdraw.
#
# Usage:
#   MOCK_ENABLED=true ./test_xion_treasury.sh
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

# Test: List treasuries with results
test_treasury_list_success() {
    local result
    result=$(mock_cli "treasury" "treasury list" "list_success")

    assert_success "$result" || return 1
    assert_json_has_key "$result" ".treasuries" || return 1
    assert_json_array_length "$result" ".treasuries" "2" || return 1
    assert_json_contains "$result" ".count" "2" || return 1

    # Verify treasury address format
    local address
    address=$(json_get "$result" ".treasuries[0].address")
    if [[ ! "$address" =~ ^xion1[a-z0-9]+$ ]]; then
        log_error "Invalid treasury address format: $address"
        return 1
    fi

    # Verify balance structure
    assert_json_has_key "$result" ".treasuries[0].balance" || return 1
    assert_json_has_key "$result" ".treasuries[0].balance[0].denom" || return 1
}

# Test: List treasuries - empty result
test_treasury_list_empty() {
    local result
    result=$(mock_cli "treasury" "treasury list" "list_empty")

    assert_success "$result" || return 1
    assert_json_array_length "$result" ".treasuries" "0" || return 1
    assert_json_contains "$result" ".count" "0" || return 1
}

# Test: List treasuries - not authenticated
test_treasury_list_not_authenticated() {
    local result
    result=$(mock_cli "treasury" "treasury list" "list_not_authenticated")

    assert_error "$result" || return 1
    assert_error_code "$result" "EAUTH001" || return 1
}

# Test: List treasuries - network error
test_treasury_list_network_error() {
    local result
    result=$(mock_cli "treasury" "treasury list" "list_network_error")

    assert_error "$result" || return 1
    assert_error_code "$result" "ENETWORK001" || return 1
    assert_json_contains "$result" ".error.retryable" "true" || return 1
}

# Test: Query treasury - success with grants
test_treasury_query_success() {
    local result
    result=$(mock_cli "treasury" "treasury query" "query_success")

    assert_success "$result" || return 1
    assert_json_has_key "$result" ".address" || return 1
    assert_json_has_key "$result" ".name" || return 1
    assert_json_has_key "$result" ".admin" || return 1
    assert_json_has_key "$result" ".balance" || return 1
    assert_json_has_key "$result" ".grants" || return 1

    # Verify grants structure
    assert_json_has_key "$result" ".grants.authz" || return 1
    assert_json_has_key "$result" ".grants.fee" || return 1
}

# Test: Query treasury - success without grants
test_treasury_query_success_no_grants() {
    local result
    result=$(mock_cli "treasury" "treasury query" "query_success_no_grants")

    assert_success "$result" || return 1
    assert_json_has_key "$result" ".address" || return 1
    assert_json_contains "$result" ".name" "Test Treasury 2" || return 1
}

# Test: Query treasury - not found
test_treasury_query_not_found() {
    local result
    result=$(mock_cli "treasury" "treasury query" "query_not_found")

    assert_error "$result" || return 1
    assert_error_code "$result" "ETREASURY001" || return 1
}

# Test: Query treasury - invalid address
test_treasury_query_invalid_address() {
    local result
    result=$(mock_cli "treasury" "treasury query" "query_invalid_address")

    assert_error "$result" || return 1
    assert_error_code "$result" "ETREASURY003" || return 1
}

# Test: Query treasury - not authenticated
test_treasury_query_not_authenticated() {
    local result
    result=$(mock_cli "treasury" "treasury query" "query_not_authenticated")

    assert_error "$result" || return 1
    assert_error_code "$result" "EAUTH001" || return 1
}

# Test: Create treasury - success
test_treasury_create_success() {
    local result
    result=$(mock_cli "treasury" "treasury create" "create_success")

    assert_success "$result" || return 1
    assert_json_has_key "$result" ".address" || return 1
    assert_json_has_key "$result" ".transaction_hash" || return 1
    assert_json_contains "$result" ".name" "New Treasury" || return 1
    assert_json_contains "$result" ".network" "testnet" || return 1
}

# Test: Create treasury - success with config
test_treasury_create_success_with_config() {
    local result
    result=$(mock_cli "treasury" "treasury create" "create_success_with_config")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".fee_configured" "true" || return 1
    assert_json_contains "$result" ".grant_configured" "true" || return 1
}

# Test: Create treasury - already exists
test_treasury_create_already_exists() {
    local result
    result=$(mock_cli "treasury" "treasury create" "create_already_exists")

    assert_error "$result" || return 1
    assert_error_code "$result" "ETREASURY009" || return 1
}

# Test: Create treasury - failed
test_treasury_create_failed() {
    local result
    result=$(mock_cli "treasury" "treasury create" "create_failed")

    assert_error "$result" || return 1
    assert_error_code "$result" "ETREASURY004" || return 1
}

# Test: Fund treasury - success
test_treasury_fund_success() {
    local result
    result=$(mock_cli "treasury" "treasury fund" "fund_success")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".message" "Treasury funded successfully" || return 1
    assert_json_contains "$result" ".amount" "1000000uxion" || return 1
    assert_json_has_key "$result" ".transaction_hash" || return 1
    assert_json_has_key "$result" ".new_balance" || return 1
}

# Test: Withdraw from treasury - success
test_treasury_withdraw_success() {
    local result
    result=$(mock_cli "treasury" "treasury withdraw" "withdraw_success")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".message" "Withdrawal successful" || return 1
    assert_json_contains "$result" ".amount" "500000uxion" || return 1
    assert_json_has_key "$result" ".transaction_hash" || return 1
}

# Test: Fund treasury - insufficient balance
test_treasury_insufficient_balance() {
    local result
    result=$(mock_cli "treasury" "treasury fund" "insufficient_balance")

    assert_error "$result" || return 1
    assert_error_code "$result" "ETREASURY002" || return 1
}

# Test: Withdraw treasury - insufficient balance
test_treasury_withdraw_insufficient_balance() {
    local result
    result=$(mock_cli "treasury" "treasury withdraw" "withdraw_insufficient_balance")

    assert_error "$result" || return 1
    assert_error_code "$result" "ETREASURY002" || return 1
}

# Test: Treasury operation - not authorized
test_treasury_not_authorized() {
    local result
    result=$(mock_cli "treasury" "treasury fund" "not_authorized")

    assert_error "$result" || return 1
    assert_error_code "$result" "ETREASURY008" || return 1
}

# ==============================================================================
# Main
# ==============================================================================

main() {
    log_info "Running Treasury Skill Tests"
    log_info "Mock mode: ${MOCK_ENABLED}"
    log_info "Mock dir: ${MOCK_DIR}"

    # Check if in mock mode
    if [[ "$MOCK_ENABLED" != "true" ]]; then
        log_warn "Not in mock mode. Some tests may fail or require real authentication."
        log_warn "Run with: MOCK_ENABLED=true $0"
    fi

    # Run all test cases
    run_test "test_treasury_list_success" test_treasury_list_success
    run_test "test_treasury_list_empty" test_treasury_list_empty
    run_test "test_treasury_list_not_authenticated" test_treasury_list_not_authenticated
    run_test "test_treasury_list_network_error" test_treasury_list_network_error
    run_test "test_treasury_query_success" test_treasury_query_success
    run_test "test_treasury_query_success_no_grants" test_treasury_query_success_no_grants
    run_test "test_treasury_query_not_found" test_treasury_query_not_found
    run_test "test_treasury_query_invalid_address" test_treasury_query_invalid_address
    run_test "test_treasury_query_not_authenticated" test_treasury_query_not_authenticated
    run_test "test_treasury_create_success" test_treasury_create_success
    run_test "test_treasury_create_success_with_config" test_treasury_create_success_with_config
    run_test "test_treasury_create_already_exists" test_treasury_create_already_exists
    run_test "test_treasury_create_failed" test_treasury_create_failed
    run_test "test_treasury_fund_success" test_treasury_fund_success
    run_test "test_treasury_withdraw_success" test_treasury_withdraw_success
    run_test "test_treasury_insufficient_balance" test_treasury_insufficient_balance
    run_test "test_treasury_withdraw_insufficient_balance" test_treasury_withdraw_insufficient_balance
    run_test "test_treasury_not_authorized" test_treasury_not_authorized

    # Print summary and exit
    test_exit
}

# Run main function
main "$@"
