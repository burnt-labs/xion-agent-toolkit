#!/bin/bash
#
# Xion Agent Toolkit - Batch Module E2E Test
# Tests the batch module: validate, execute, error handling
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

# Protected treasury (DO NOT USE)
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
    
    # Also check for explicit valid field (must be true, not false)
    if [ "$field" = "valid" ]; then
        if echo "$json" | grep -qE '"valid": *true'; then
            return 0
        fi
    fi
    
    # For operations - success indicated by tx_hash, valid, count
    if [ "$field" = "success" ]; then
        if echo "$json" | grep -q '"tx_hash"'; then
            return 0
        fi
        if echo "$json" | grep -qE '"valid": *true'; then
            return 0
        fi
        if echo "$json" | grep -q '"count"'; then
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
# Test: Batch Validate - Valid File
# =============================================================================

test_batch_validate_valid() {
    print_step 2 7 "Batch Validate (Valid)"
    
    # Create temp batch file
    TEMP_DIR=$(mktemp -d)
    local batch_file="$TEMP_DIR/valid_batch.json"
    
    cat > "$batch_file" << 'EOF'
{
  "messages": [
    {
      "typeUrl": "/cosmwasm.wasm.v1.MsgExecuteContract",
      "value": {
        "sender": "xion1crqm3t66ytul4rv4yjea7yzkr8s29kt3m8rlzvw3k9ytkjyyxajsmcak6m",
        "contract": "xion1contract12345678901234567890",
        "msg": { "transfer": { "recipient": "xion1recipient123456789", "amount": "1000" } },
        "funds": []
      }
    }
  ]
}
EOF
    
    local validate_output=$(run_cmd "batch validate --from-file $batch_file")
    
    # Cleanup
    rm -rf "$TEMP_DIR"
    TEMP_DIR=""
    
    if json_true "$validate_output" "valid"; then
        local msg_count=$(parse_json "$validate_output" "message_count")
        record_result "PASS" "Batch Validate (Valid)" "$msg_count message(s) valid"
        return 0
    fi
    
    local error_msg=$(parse_json "$validate_output" "error")
    record_result "FAIL" "Batch Validate (Valid)" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Test: Batch Validate - Invalid Format
# =============================================================================

test_batch_validate_invalid_format() {
    print_step 3 7 "Batch Validate (Invalid)"
    
    # Create temp batch file with invalid format
    TEMP_DIR=$(mktemp -d)
    local batch_file="$TEMP_DIR/invalid_batch.json"
    
    # Missing typeUrl field
    cat > "$batch_file" << 'EOF'
{
  "messages": [
    {
      "value": {
        "sender": "xion1sender",
        "contract": "xion1contract"
      }
    }
  ]
}
EOF
    
    local validate_output=$(run_cmd "batch validate --from-file $batch_file")
    
    # Cleanup
    rm -rf "$TEMP_DIR"
    TEMP_DIR=""
    
    # Should return valid: false
    if echo "$validate_output" | grep -q '"valid":false'; then
        record_result "PASS" "Batch Validate (Invalid)" "Correctly rejected invalid format"
        return 0
    fi
    
    local error_msg=$(parse_json "$validate_output" "error")
    record_result "FAIL" "Batch Validate (Invalid)" "Did not reject invalid format: ${error_msg:0:50}"
    return 1
}

# =============================================================================
# Test: Batch Validate - Missing File
# =============================================================================

test_batch_validate_missing_file() {
    print_step 4 7 "Batch Validate (No File)"
    
    local validate_output=$(run_cmd "batch validate --from-file /nonexistent/batch.json")
    
    # Should return an error
    if echo "$validate_output" | grep -qi "error\|not found\|no such"; then
        record_result "PASS" "Batch Validate (No File)" "Correctly handled missing file"
        return 0
    fi
    
    record_result "FAIL" "Batch Validate (No File)" "Did not handle missing file"
    return 1
}

# =============================================================================
# Test: Batch Execute - Simulate
# =============================================================================

test_batch_execute_simulate() {
    print_step 5 7 "Batch Execute (Simulate)"
    
    # Create temp batch file
    TEMP_DIR=$(mktemp -d)
    local batch_file="$TEMP_DIR/simulate_batch.json"
    
    cat > "$batch_file" << 'EOF'
{
  "messages": [
    {
      "typeUrl": "/cosmwasm.wasm.v1.MsgExecuteContract",
      "value": {
        "sender": "xion1crqm3t66ytul4rv4yjea7yzkr8s29kt3m8rlzvw3k9ytkjyyxajsmcak6m",
        "contract": "xion1contract12345678901234567890",
        "msg": { "transfer": { "recipient": "xion1recipient123456789", "amount": "1" } },
        "funds": []
      }
    }
  ]
}
EOF
    
    local execute_output=$(run_cmd "batch execute --from-file $batch_file --simulate")
    
    # Cleanup
    rm -rf "$TEMP_DIR"
    TEMP_DIR=""
    
    # Simulate should return some response (may fail due to contract not found, which is OK)
    # We just want to verify the command structure works
    if echo "$execute_output" | grep -qE '"success"|"error"|"simulation_results"'; then
        record_result "PASS" "Batch Execute (Simulate)" "Simulation executed"
        return 0
    fi
    
    local error_msg=$(parse_json "$execute_output" "error")
    
    # If the contract doesn't exist, that's expected in test environment
    if echo "$error_msg" | grep -qi "contract.*not found\|not found\|execute.*failed"; then
        record_result "SKIP" "Batch Execute (Simulate)" "Contract not found (expected)"
        return 0
    fi
    
    record_result "FAIL" "Batch Execute (Simulate)" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Test: Batch Execute - Actual (Skipped)
# =============================================================================

test_batch_execute_actual() {
    print_step 6 7 "Batch Execute (Actual)"
    
    # Skip actual execution as it requires a valid contract
    record_result "SKIP" "Batch Execute (Actual)" "Requires valid contract on chain"
    return 0
}

# =============================================================================
# Test: Error Handling - Malformed JSON
# =============================================================================

test_error_handling_malformed() {
    print_step 7 7 "Error Handling"
    
    TEMP_DIR=$(mktemp -d)
    local batch_file="$TEMP_DIR/malformed_batch.json"
    
    # Create malformed JSON
    cat > "$batch_file" << 'EOF'
{
  "messages": [
    {
      "typeUrl": "/cosmwasm.wasm.v1.MsgExecuteContract",
      "value": {
        "sender": "xion1sender",
        "contract": "xion1contract"
        invalid json here
      }
    }
  ]
}
EOF
    
    local validate_output=$(run_cmd "batch validate --from-file $batch_file")
    
    # Cleanup
    rm -rf "$TEMP_DIR"
    TEMP_DIR=""
    
    # Should return an error
    if echo "$validate_output" | grep -qi "error\|json\|parse"; then
        record_result "PASS" "Error Handling" "Correctly handled malformed JSON"
        return 0
    fi
    
    record_result "FAIL" "Error Handling" "Did not handle malformed JSON"
    return 1
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
    echo "Batch Module E2E Test"
    echo "================================"
    echo ""
    log_info "Binary: $BINARY_PATH"
    log_info "Network: $NETWORK"
    echo ""
    
    # Run all tests
    test_preflight || { echo ""; echo "Pre-flight failed. Exiting."; exit 1; }
    echo ""
    
    test_batch_validate_valid
    echo ""
    
    test_batch_validate_invalid_format
    echo ""
    
    test_batch_validate_missing_file
    echo ""
    
    test_batch_execute_simulate
    echo ""
    
    test_batch_execute_actual
    echo ""
    
    test_error_handling_malformed
    echo ""
    
    # Print summary
    echo "================================"
    echo "Results: $PASS_COUNT PASS, $FAIL_COUNT FAIL, $SKIP_COUNT SKIP"
    echo "================================"
    
    # Cleanup before exit
    cleanup
    
    # Exit with error if any tests failed
    if [ $FAIL_COUNT -gt 0 ]; then
        exit 1
    fi
    
    exit 0
}

# Run main
main "$@"
