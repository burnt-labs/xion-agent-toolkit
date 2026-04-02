#!/bin/bash
#
# xion-oauth2/logout.sh - Clear stored credentials
#
# This script clears the stored credentials for a specific network.
#
# Usage:
#   ./logout.sh [--network NETWORK]
#
# Output:
#   JSON to stdout with logout result
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
            echo "  --network NETWORK Network to logout from: local, testnet, mainnet (default: testnet)" >&2
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

log_info "Logging out..."

# Check if xion-toolkit CLI is available
if ! command -v xion-toolkit &> /dev/null; then
    handle_error "xion-toolkit CLI not found in PATH. Please install xion-agent-toolkit first." "CLI_NOT_FOUND"
fi

# Build command as array (safe from injection)
CMD_ARGS=(auth logout --output json)

if [[ -n "$NETWORK" ]]; then
    CMD_ARGS+=(--network "$NETWORK")
fi

log_info "Running: xion-toolkit ${CMD_ARGS[*]}"

# Execute command safely using array expansion
RESULT=$(xion-toolkit --no-interactive "${CMD_ARGS[@]}" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    # Success - output the JSON result
    log_info "Logout successful"
    output_json "$RESULT"
else
    # Error occurred
    log_error "Logout failed: $RESULT"
    
    # Even if there's an error, if it's because user wasn't logged in, that's still success
    if echo "$RESULT" | grep -qi "not authenticated\|no credentials\|not logged in"; then
        output_json "{\"success\": true, \"message\": \"Already logged out\"}"
    else
        handle_error "Failed to logout: $RESULT" "LOGOUT_FAILED"
    fi
fi
