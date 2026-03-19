#!/bin/bash
#
# xion-treasury/query.sh - Query Treasury details
#
# This script queries detailed information about a specific Treasury contract.
#
# Usage:
#   ./query.sh <ADDRESS> [--network NETWORK] [--include-grants] [--include-fee] [--no-cache]
#
# Output:
#   JSON to stdout with treasury details
#   Status messages to stderr

set -e

# ==============================================================================
# Helper Functions
# ==============================================================================

output_json() {
    echo "$1"
}

log_info() {
    echo "[INFO] $1" >&2
}

log_error() {
    echo "[ERROR] $1" >&2
}

handle_error() {
    local message="$1"
    local code="${2:-UNKNOWN_ERROR}"
    output_json "{\"success\": false, \"error\": \"$message\", \"error_code\": \"$code\"}"
    exit 1
}

# ==============================================================================
# Argument Parsing
# ==============================================================================

ADDRESS=""
NETWORK=""
INCLUDE_GRANTS=false
INCLUDE_FEE=false
NO_CACHE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --include-grants)
            INCLUDE_GRANTS=true
            shift
            ;;
        --include-fee)
            INCLUDE_FEE=true
            shift
            ;;
        --no-cache)
            NO_CACHE=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 <ADDRESS> [--network NETWORK] [--include-grants] [--include-fee] [--no-cache]" >&2
            echo "" >&2
            echo "Arguments:" >&2
            echo "  ADDRESS           Treasury contract address (required)" >&2
            echo "" >&2
            echo "Options:" >&2
            echo "  --network NETWORK Network to use: local, testnet, mainnet (default: testnet)" >&2
            echo "  --include-grants  Include Authz grant information" >&2
            echo "  --include-fee     Include Fee grant information" >&2
            echo "  --no-cache        Bypass cache and fetch fresh data" >&2
            exit 0
            ;;
        -*)
            log_error "Unknown option: $1"
            handle_error "Unknown option: $1" "INVALID_ARGUMENT"
            ;;
        *)
            if [ -z "$ADDRESS" ]; then
                ADDRESS="$1"
            fi
            shift
            ;;
    esac
done

# Validate address
if [ -z "$ADDRESS" ]; then
    handle_error "Treasury address is required. Usage: $0 <ADDRESS>" "MISSING_ADDRESS"
fi

# Validate address format (basic check)
if ! echo "$ADDRESS" | grep -qE "^xion1[a-zA-Z0-9]{38,}"; then
    handle_error "Invalid Treasury address format. Expected: xion1..." "INVALID_ADDRESS"
fi

# ==============================================================================
# Main Logic
# ==============================================================================

log_info "Querying Treasury: $ADDRESS"

# Check if xion-toolkit CLI is available
if ! command -v xion-toolkit &> /dev/null; then
    handle_error "xion-toolkit CLI not found in PATH. Please install xion-agent-toolkit first." "CLI_NOT_FOUND"
fi

# Build command as array (safe from injection)
CMD=(xion-toolkit treasury query "$ADDRESS" --output json)

if [[ -n "$NETWORK" ]]; then
    CMD+=(--network "$NETWORK")
fi

if [[ "$INCLUDE_GRANTS" == true ]]; then
    CMD+=(--include-grants)
fi

if [[ "$INCLUDE_FEE" == true ]]; then
    CMD+=(--include-fee)
fi

if [[ "$NO_CACHE" == true ]]; then
    CMD+=(--no-cache)
fi

log_info "Running: ${CMD[*]}"

# Execute command safely using array expansion
RESULT=$("${CMD[@]}" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    # Success - output the JSON result
    log_info "Query successful"
    output_json "$RESULT"
else
    # Error occurred
    log_error "Query failed: $RESULT"
    
    if echo "$RESULT" | grep -qi "not authenticated\|no credentials\|not logged in"; then
        handle_error "Not authenticated. Please use 'xion-oauth2/login.sh' first." "NOT_AUTHENTICATED"
    elif echo "$RESULT" | grep -qi "not found\|does not exist"; then
        handle_error "Treasury not found: $ADDRESS" "TREASURY_NOT_FOUND"
    elif echo "$RESULT" | grep -qi "invalid.*address"; then
        handle_error "Invalid Treasury address: $ADDRESS" "INVALID_ADDRESS"
    elif echo "$RESULT" | grep -qi "network\|connection\|timeout"; then
        handle_error "Network error. Check your connection and try again." "NETWORK_ERROR"
    else
        handle_error "Failed to query treasury: $RESULT" "QUERY_FAILED"
    fi
fi
