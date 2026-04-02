#!/bin/bash
#
# xion-treasury/list.sh - List all Treasury contracts
#
# This script lists all Treasury contracts owned by the authenticated user.
#
# Usage:
#   ./list.sh [--network NETWORK] [--no-cache]
#
# Output:
#   JSON to stdout with treasury list
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

NETWORK=""
NO_CACHE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --no-cache)
            NO_CACHE=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [--network NETWORK] [--no-cache]" >&2
            echo "" >&2
            echo "Options:" >&2
            echo "  --network NETWORK Network to use: local, testnet, mainnet (default: testnet)" >&2
            echo "  --no-cache        Bypass cache and fetch fresh data" >&2
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            handle_error "Unknown option: $1" "INVALID_ARGUMENT"
            ;;
    esac
done

# ==============================================================================
# Main Logic
# ==============================================================================

log_info "Listing Treasury contracts..."

# Check if xion-toolkit CLI is available
if ! command -v xion-toolkit &> /dev/null; then
    handle_error "xion-toolkit CLI not found in PATH. Please install xion-agent-toolkit first." "CLI_NOT_FOUND"
fi

# Build command as array (safe from injection)
CMD=(xion-toolkit --no-interactive treasury list --output json)

if [[ -n "$NETWORK" ]]; then
    CMD+=(--network "$NETWORK")
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
    log_info "List successful"
    output_json "$RESULT"
else
    # Error occurred
    log_error "List failed: $RESULT"
    
    if echo "$RESULT" | grep -qi "not authenticated\|no credentials\|not logged in"; then
        handle_error "Not authenticated. Please use 'xion-oauth2/login.sh' first." "NOT_AUTHENTICATED"
    elif echo "$RESULT" | grep -qi "network\|connection\|timeout"; then
        handle_error "Network error. Check your connection and try again." "NETWORK_ERROR"
    else
        handle_error "Failed to list treasuries: $RESULT" "LIST_FAILED"
    fi
fi
