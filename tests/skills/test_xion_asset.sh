#!/bin/bash
#
# tests/skills/test_xion_asset.sh - Asset Skill Tests
#
# Tests for xion-asset skill including types, create, mint, and query operations.
#
# Usage:
#   MOCK_ENABLED=true ./test_xion_asset.sh
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

# Test: List asset types
test_asset_types_list() {
    local result
    result=$(mock_cli "asset" "asset types" "types_list")

    assert_success "$result" || return 1
    assert_json_has_key "$result" ".types" || return 1
    assert_json_array_length "$result" ".types" "4" || return 1

    # Verify type structure
    assert_json_has_key "$result" ".types[0].id" || return 1
    assert_json_has_key "$result" ".types[0].name" || return 1
    assert_json_has_key "$result" ".types[0].code_id" || return 1

    # Verify code_id has network keys
    assert_json_has_key "$result" ".types[0].code_id.testnet" || return 1
    assert_json_has_key "$result" ".types[0].code_id.mainnet" || return 1
}

# Test: List asset types - empty
test_asset_types_empty() {
    local result
    result=$(mock_cli "asset" "asset types" "types_empty")

    assert_success "$result" || return 1
    assert_json_array_length "$result" ".types" "0" || return 1
}

# Test: List asset types - network error
test_asset_types_network_error() {
    local result
    result=$(mock_cli "asset" "asset types" "types_network_error")

    assert_error "$result" || return 1
    assert_error_code "$result" "ENETWORK001" || return 1
    assert_json_contains "$result" ".error.retryable" "true" || return 1
}

# Test: Create NFT collection - success
test_asset_create_success_cw721() {
    local result
    result=$(mock_cli "asset" "asset create" "create_success_cw721")

    assert_success "$result" || return 1
    assert_json_has_key "$result" ".contract_address" || return 1
    assert_json_has_key "$result" ".transaction_hash" || return 1
    assert_json_contains "$result" ".name" "My NFT Collection" || return 1
    assert_json_contains "$result" ".symbol" "MYNFT" || return 1
    assert_json_contains "$result" ".asset_type" "cw721-base" || return 1

    # Verify contract address format
    local address
    address=$(json_get "$result" ".contract_address")
    if [[ ! "$address" =~ ^xion1[a-z0-9]+$ ]]; then
        log_error "Invalid contract address format: $address"
        return 1
    fi
}

# Test: Create NFT collection - success with custom minter
test_asset_create_success_with_minter() {
    local result
    result=$(mock_cli "asset" "asset create" "create_success_with_minter")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".minter" "xion1treasury1abcdefghijklmnopqrstuvwxyz1234" || return 1
    assert_json_contains "$result" ".symbol" "CMC" || return 1
}

# Test: Create NFT collection - invalid type
test_asset_create_failed_invalid_type() {
    local result
    result=$(mock_cli "asset" "asset create" "create_failed_invalid_type")

    assert_error "$result" || return 1
    assert_error_code "$result" "EASSET004" || return 1
}

# Test: Create NFT collection - invalid metadata
test_asset_create_failed_invalid_metadata() {
    local result
    result=$(mock_cli "asset" "asset create" "create_failed_invalid_metadata")

    assert_error "$result" || return 1
    assert_error_code "$result" "EASSET001" || return 1
}

# Test: Create NFT collection - missing arguments
test_asset_create_failed_missing_args() {
    local result
    result=$(mock_cli "asset" "asset create" "create_failed_missing_args")

    assert_error "$result" || return 1
    assert_json_contains "$result" ".error_code" "MISSING_ARGS" || return 1
}

# Test: Mint NFT - success
test_asset_mint_success() {
    local result
    result=$(mock_cli "asset" "asset mint" "mint_success")

    assert_success "$result" || return 1
    assert_json_has_key "$result" ".transaction_hash" || return 1
    assert_json_contains "$result" ".token_id" "token-001" || return 1
    assert_json_has_key "$result" ".contract_address" || return 1
    assert_json_has_key "$result" ".token_uri" || return 1

    # Verify owner address format
    local owner
    owner=$(json_get "$result" ".owner")
    if [[ ! "$owner" =~ ^xion1[a-z0-9]+$ ]]; then
        log_error "Invalid owner address format: $owner"
        return 1
    fi
}

# Test: Mint NFT - success with royalties
test_asset_mint_success_with_royalties() {
    local result
    result=$(mock_cli "asset" "asset mint" "mint_success_with_royalties")

    assert_success "$result" || return 1
    assert_json_has_key "$result" ".royalty_info" || return 1
    assert_json_contains "$result" ".royalty_info.percentage" "5.0" || return 1
}

# Test: Mint NFT - not minter
test_asset_mint_failed_not_minter() {
    local result
    result=$(mock_cli "asset" "asset mint" "mint_failed_not_minter")

    assert_error "$result" || return 1
    assert_error_code "$result" "ETREASURY008" || return 1
}

# Test: Mint NFT - invalid metadata
test_asset_mint_failed_invalid_metadata() {
    local result
    result=$(mock_cli "asset" "asset mint" "mint_failed_invalid_metadata")

    assert_error "$result" || return 1
    assert_error_code "$result" "EASSET001" || return 1
}

# Test: Mint NFT - token already exists
test_asset_mint_failed_token_exists() {
    local result
    result=$(mock_cli "asset" "asset mint" "mint_failed_token_exists")

    assert_error "$result" || return 1
    assert_error_code "$result" "EASSET002" || return 1
}

# Test: Query collection
test_asset_query_collection() {
    local result
    result=$(mock_cli "asset" "asset query" "query_collection")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".name" "My NFT Collection" || return 1
    assert_json_contains "$result" ".symbol" "MYNFT" || return 1
    assert_json_contains "$result" ".total_supply" "100" || return 1
    assert_json_contains "$result" ".num_tokens" "10" || return 1
    assert_json_has_key "$result" ".minter" || return 1
}

# Test: Query token
test_asset_query_token() {
    local result
    result=$(mock_cli "asset" "asset query" "query_token")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".token_id" "token-001" || return 1
    assert_json_has_key "$result" ".token_uri" || return 1
    assert_json_has_key "$result" ".extension" || return 1
}

# Test: Query token - not found
test_asset_query_token_not_found() {
    local result
    result=$(mock_cli "asset" "asset query" "query_token_not_found")

    assert_error "$result" || return 1
    assert_error_code "$result" "ETREASURY001" || return 1
}

# Test: Batch mint - success
test_asset_batch_mint_success() {
    local result
    result=$(mock_cli "asset" "asset batch-mint" "batch_mint_success")

    assert_success "$result" || return 1
    assert_json_contains "$result" ".minted_count" "5" || return 1
    assert_json_array_length "$result" ".token_ids" "5" || return 1
}

# Test: Batch mint - partial failure
test_asset_batch_mint_partial_failure() {
    local result
    result=$(mock_cli "asset" "asset batch-mint" "batch_mint_partial_failure")

    assert_error "$result" || return 1
    assert_error_code "$result" "EBATCH003" || return 1
    assert_json_has_key "$result" ".results" || return 1
}

# ==============================================================================
# Main
# ==============================================================================

main() {
    log_info "Running Asset Skill Tests"
    log_info "Mock mode: ${MOCK_ENABLED}"
    log_info "Mock dir: ${MOCK_DIR}"

    # Check if in mock mode
    if [[ "$MOCK_ENABLED" != "true" ]]; then
        log_warn "Not in mock mode. Some tests may fail or require real authentication."
        log_warn "Run with: MOCK_ENABLED=true $0"
    fi

    # Run all test cases
    run_test "test_asset_types_list" test_asset_types_list
    run_test "test_asset_types_empty" test_asset_types_empty
    run_test "test_asset_types_network_error" test_asset_types_network_error
    run_test "test_asset_create_success_cw721" test_asset_create_success_cw721
    run_test "test_asset_create_success_with_minter" test_asset_create_success_with_minter
    run_test "test_asset_create_failed_invalid_type" test_asset_create_failed_invalid_type
    run_test "test_asset_create_failed_invalid_metadata" test_asset_create_failed_invalid_metadata
    run_test "test_asset_create_failed_missing_args" test_asset_create_failed_missing_args
    run_test "test_asset_mint_success" test_asset_mint_success
    run_test "test_asset_mint_success_with_royalties" test_asset_mint_success_with_royalties
    run_test "test_asset_mint_failed_not_minter" test_asset_mint_failed_not_minter
    run_test "test_asset_mint_failed_invalid_metadata" test_asset_mint_failed_invalid_metadata
    run_test "test_asset_mint_failed_token_exists" test_asset_mint_failed_token_exists
    run_test "test_asset_query_collection" test_asset_query_collection
    run_test "test_asset_query_token" test_asset_query_token
    run_test "test_asset_query_token_not_found" test_asset_query_token_not_found
    run_test "test_asset_batch_mint_success" test_asset_batch_mint_success
    run_test "test_asset_batch_mint_partial_failure" test_asset_batch_mint_partial_failure

    # Print summary and exit
    test_exit
}

# Run main function
main "$@"
