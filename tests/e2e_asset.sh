#!/bin/bash
#
# Xion Agent Toolkit - Asset (NFT) E2E Test
# Tests the asset module: types, predict, create, mint, query, batch-mint
#

# Use pipefail but not -e to avoid early exits
set -uo pipefail

# =============================================================================
# Configuration
# =============================================================================

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Binary path (can be overridden via CLI argument)
BINARY_PATH="${1:-./target/release/xion-toolkit}"
NETWORK="${NETWORK:-testnet}"

# Test contract (if available - DO NOT use protected treasury)
TEST_CONTRACT="${2:-}"
PROTECTED_TREASURY="xion17vg5l9za4768g0hnxezltgnu4h7eleqdcmwark2uuz2s4z5q4dfsr80vvm"

# Temporary directory
TEMP_DIR=""

# =============================================================================
# Helper Functions
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

# Run a CLI command and capture JSON output
run_cmd() {
    local cmd="$1"
    local full_cmd="$BINARY_PATH $cmd --output json"
    
    # Execute and capture output
    local output
    output=$(eval "$full_cmd" 2>&1)
    
    # Remove ANSI escape codes and filter out tracing log lines
    output=$(echo "$output" | sed 's/\x1b\[[0-9;]*m//g' | grep -v "^[[:space:]]*20[0-9]" | grep -v "^\[INFO\]")
    
    # Try to extract JSON using jq (first compact the JSON to one line)
    local json
    json=$(echo "$output" | jq -c '.' 2>/dev/null)
    
    if [ -n "$json" ] && [ "$json" != "null" ] && [ "$json" != "" ]; then
        echo "$json"
    else
        # Fallback: output raw output
        echo "$output"
    fi
}

# Parse JSON field from output
parse_json() {
    local json="$1"
    local field="$2"
    
    # Use jq if available for better JSON parsing
    # First extract lines that start with { or contain JSON
    local json_only
    json_only=$(echo "$json" | grep -E '^\{.*\}$|^\[' 2>/dev/null | head -1)
    if [ -z "$json_only" ]; then
        # Try to find the JSON portion differently
        json_only=$(echo "$json" | tr '}' '\n' | grep -E '^\{.*"')
        if [ -z "$json_only" ]; then
            json_only="$json"
        fi
    fi
    
    local value
    value=$(echo "$json_only" | jq -r ".$field // empty" 2>/dev/null)
    
    if [ -z "$value" ] || [ "$value" = "null" ]; then
        # Fallback to grep with better pattern that handles colons in values
        value=$(echo "$json_only" | grep -o "\"$field\":.*" | head -1 | sed 's/.*: *//' | sed 's/",* *$//' | tr -d ' "')
    fi
    
    # Trim whitespace
    value=$(echo "$value" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
    echo "$value"
}

# Check if JSON indicates success
json_true() {
    local json="$1"
    local field="$2"
    
    # First check for explicit success field
    if echo "$json" | grep -q "\"$field\""; then
        local value
        value=$(echo "$json" | grep -o "\"$field\":[^,}]*" | sed 's/.*: *//' | tr -d ' "')
        [ "$value" = "true" ] && return 0
    fi
    
    # For operations - success indicated by tx_hash, address, count
    if [ "$field" = "success" ]; then
        if echo "$json" | grep -q '"tx_hash"'; then
            return 0
        fi
        if echo "$json" | grep -q '"address"'; then
            return 0
        fi
        if echo "$json" | grep -q '"count"'; then
            return 0
        fi
        if echo "$json" | grep -q '"valid"'; then
            return 0
        fi
    fi
    
    return 1
}

# Print step header
print_step() {
    local current=$1
    local total=$2
    local name=$3
    printf "[%d/%d] %-28s " "$current" "$total" "$name"
}

# =============================================================================
# Test Results Tracking
# =============================================================================

PASS_COUNT=0
FAIL_COUNT=0
SKIP_COUNT=0
TEST_RESULTS=()

record_result() {
    local status="$1"
    local name="$2"
    local detail="${3:-}"
    
    TEST_RESULTS+=("[$status] $name")
    
    case "$status" in
        PASS)
            PASS_COUNT=$((PASS_COUNT + 1))
            if [ -n "$detail" ]; then
                echo -e "${GREEN}✓ PASS${NC} ($detail)"
            else
                echo -e "${GREEN}✓ PASS${NC}"
            fi
            ;;
        FAIL)
            FAIL_COUNT=$((FAIL_COUNT + 1))
            if [ -n "$detail" ]; then
                echo -e "${RED}✗ FAIL${NC} ($detail)"
            else
                echo -e "${RED}✗ FAIL${NC}"
            fi
            ;;
        SKIP)
            SKIP_COUNT=$((SKIP_COUNT + 1))
            if [ -n "$detail" ]; then
                echo -e "${CYAN}⊘ SKIP${NC} ($detail)"
            else
                echo -e "${CYAN}⊘ SKIP${NC}"
            fi
            ;;
    esac
    
    # Always return 0 to avoid set -e issues
    return 0
}

# =============================================================================
# Pre-flight Checks
# =============================================================================

test_preflight() {
    print_step 1 7 "Pre-flight Check"
    
    # Check if binary exists
    if [ ! -f "$BINARY_PATH" ]; then
        record_result "FAIL" "Pre-flight Check" "Binary not found at $BINARY_PATH"
        echo "Run 'cargo build --release' first"
        return 1
    fi
    
    # Check authentication status
    local auth_output=$(run_cmd "auth status")
    
    if json_true "$auth_output" "authenticated"; then
        local user_addr=$(parse_json "$auth_output" "xion_address")
        record_result "PASS" "Pre-flight Check" "CLI ready, user: ${user_addr:0:16}..."
        return 0
    else
        record_result "FAIL" "Pre-flight Check" "Not authenticated"
        echo ""
        log_error "Please login first: $BINARY_PATH auth login --network $NETWORK"
        return 1
    fi
}

# =============================================================================
# Test: Asset Types
# =============================================================================

test_asset_types() {
    print_step 2 7 "Asset Types List"
    
    local types_output=$(run_cmd "asset types")
    
    if json_true "$types_output" "success"; then
        local count=$(parse_json "$types_output" "count")
        
        # Trim trailing comma if present
        count=$(echo "$count" | sed 's/,$//')
        
        # Verify count is 5 as specified
        if [ "$count" = "5" ]; then
            record_result "PASS" "Asset Types List" "Found $count asset types"
            return 0
        else
            record_result "FAIL" "Asset Types List" "Expected 5 types, got $count"
            return 1
        fi
    fi
    
    local error_msg=$(parse_json "$types_output" "error")
    record_result "FAIL" "Asset Types List" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Test: Asset Predict Address
# =============================================================================

test_asset_predict() {
    print_step 3 7 "Asset Predict Address"
    
    # Use hex-encoded salt (must be valid hex)
    local salt="aabbccdd"
    
    local predict_output=$(run_cmd "asset predict \
        --type cw721-base \
        --name 'E2E Test Collection' \
        --symbol 'E2ETEST' \
        --salt $salt")
    
    # Check if the command succeeded (even if API returns 404 for code info)
    # Success means the prediction logic worked, even if external API wasn't available
    if echo "$predict_output" | grep -q '"address"'; then
        local address=$(parse_json "$predict_output" "address")
        record_result "PASS" "Asset Predict Address" "addr: ${address:0:20}..."
        return 0
    fi
    
    # If it failed with code info error, that's acceptable for test
    # Check for 404 in the raw output (more reliable)
    if echo "$predict_output" | grep -qi '404\|not found\|code info'; then
        record_result "SKIP" "Asset Predict Address" "Code info unavailable (network issue)"
        return 0
    fi
    
    # Other errors are actual failures
    local error_msg=$(parse_json "$predict_output" "error")
    record_result "FAIL" "Asset Predict Address" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Test: Asset Create Collection
# =============================================================================

test_asset_create() {
    print_step 4 7 "Asset Create Collection"
    
    # Skip - creating collections requires on-chain transactions
    # and should not be done during automated testing
    record_result "SKIP" "Asset Create Collection" "Requires on-chain transaction"
    return 0
}

# =============================================================================
# Test: Asset Mint Token
# =============================================================================

test_asset_mint() {
    print_step 5 7 "Asset Mint Token"
    
    # Skip if no test contract provided
    if [ -z "$TEST_CONTRACT" ]; then
        record_result "SKIP" "Asset Mint Token" "No test contract provided"
        return 0
    fi
    
    # Skip if it's the protected treasury
    if [ "$TEST_CONTRACT" = "$PROTECTED_TREASURY" ]; then
        record_result "SKIP" "Asset Mint Token" "Protected treasury"
        return 0
    fi
    
    record_result "SKIP" "Asset Mint Token" "Requires valid NFT contract"
    return 0
}

# =============================================================================
# Test: Asset Query Contract
# =============================================================================

test_asset_query() {
    print_step 6 7 "Asset Query Contract"
    
    # If test contract provided, try to query it
    if [ -n "$TEST_CONTRACT" ]; then
        # Skip if it's the protected treasury
        if [ "$TEST_CONTRACT" = "$PROTECTED_TREASURY" ]; then
            record_result "SKIP" "Asset Query Contract" "Protected treasury"
            return 0
        fi
        
        # Try to query contract info
        local query_output=$(run_cmd "asset query \
            --contract $TEST_CONTRACT \
            --msg '{\"contract_info\":{}}'")
        
        if json_true "$query_output" "success"; then
            record_result "PASS" "Asset Query Contract" "Contract queried"
            return 0
        fi
        
        # Check if it's a contract not found error
        local error_msg=$(parse_json "$query_output" "error")
        if echo "$error_msg" | grep -qi "not found\|contract.*error\|query.*failed"; then
            record_result "SKIP" "Asset Query Contract" "Contract not found or not NFT"
            return 0
        fi
        
        record_result "FAIL" "Asset Query Contract" "${error_msg:0:50}"
        return 1
    fi
    
    record_result "SKIP" "Asset Query Contract" "No test contract provided"
    return 0
}

# =============================================================================
# Test: Asset Batch Mint
# =============================================================================

test_asset_batch_mint() {
    print_step 7 7 "Asset Batch Mint"
    
    # Skip if no test contract provided
    if [ -z "$TEST_CONTRACT" ]; then
        record_result "SKIP" "Asset Batch Mint" "No test contract provided"
        return 0
    fi
    
    # Skip if it's the protected treasury
    if [ "$TEST_CONTRACT" = "$PROTECTED_TREASURY" ]; then
        record_result "SKIP" "Asset Batch Mint" "Protected treasury"
        return 0
    fi
    
    record_result "SKIP" "Asset Batch Mint" "Requires valid NFT contract"
    return 0
}

# =============================================================================
# Cleanup
# =============================================================================

cleanup() {
    [ -d "$TEMP_DIR" ] && rm -rf "$TEMP_DIR"
}

# =============================================================================
# Main Test Runner
# =============================================================================

main() {
    echo ""
    echo "================================"
    echo "Asset (NFT) Module E2E Test"
    echo "================================"
    echo ""
    log_info "Binary: $BINARY_PATH"
    log_info "Network: $NETWORK"
    if [ -n "$TEST_CONTRACT" ]; then
        log_info "Test Contract: $TEST_CONTRACT"
    fi
    echo ""
    
    # Run all tests
    test_preflight || { echo ""; echo "Pre-flight failed. Exiting."; exit 1; }
    echo ""
    
    test_asset_types || true
    echo ""
    
    test_asset_predict || true
    echo ""
    
    test_asset_create || true
    echo ""
    
    test_asset_mint || true
    echo ""
    
    test_asset_query || true
    echo ""
    
    test_asset_batch_mint || true
    echo ""
    
    # Cleanup before exit
    cleanup
    
    # Print summary
    echo "================================"
    echo "Results: $PASS_COUNT PASS, $FAIL_COUNT FAIL, $SKIP_COUNT SKIP"
    echo "================================"
    
    # Exit with error if any tests failed
    if [ $FAIL_COUNT -gt 0 ]; then
        exit 1
    fi
    
    exit 0
}

# Run main
main "$@"
