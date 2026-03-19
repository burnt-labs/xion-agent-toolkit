#!/bin/bash
#
# xion-oauth2/refresh.sh - Refresh access token
#
# This script manually refreshes the access token using the refresh token.
#
# Usage:
#   ./refresh.sh [--network NETWORK]
#
# Output:
#   JSON to stdout with refresh result
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

while [[ $# -gt 0 ]]; do
    case $1 in
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [--network NETWORK]" >&2
            echo "" >&2
            echo "Options:" >&2
            echo "  --network NETWORK Network to refresh token for: local, testnet, mainnet (default: testnet)" >&2
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

log_info "Refreshing access token..."

# Check if xion-toolkit CLI is available
if ! command -v xion-toolkit &> /dev/null; then
    handle_error "xion-toolkit CLI not found in PATH. Please install xion-agent-toolkit first." "CLI_NOT_FOUND"
fi

# Build command as array (safe from injection)
CMD_ARGS=(auth refresh --output json)

if [[ -n "$NETWORK" ]]; then
    CMD_ARGS+=(--network "$NETWORK")
fi

log_info "Running: xion-toolkit ${CMD_ARGS[*]}"

# Execute command safely using array expansion
RESULT=$(xion-toolkit "${CMD_ARGS[@]}" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    # Success - output the JSON result
    log_info "Token refresh successful"
    output_json "$RESULT"
else
    # Error occurred
    log_error "Token refresh failed: $RESULT"
    
    if echo "$RESULT" | grep -qi "not authenticated\|no credentials\|not logged in"; then
        handle_error "Not authenticated. Please use 'login.sh' first." "NOT_AUTHENTICATED"
    elif echo "$RESULT" | grep -qi "refresh token.*expired\|invalid.*refresh"; then
        handle_error "Refresh token expired or invalid. Please use 'login.sh' to re-authenticate." "REFRESH_TOKEN_EXPIRED"
    elif echo "$RESULT" | grep -qi "network"; then
        handle_error "Network error during token refresh. Check your connection." "NETWORK_ERROR"
    else
        handle_error "Failed to refresh token: $RESULT" "REFRESH_FAILED"
    fi
fi
