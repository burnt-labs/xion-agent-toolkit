#!/bin/bash
#
# xion-oauth2/status.sh - Check authentication status
#
# This script checks the current authentication status for a network.
#
# Usage:
#   ./status.sh [--network NETWORK]
#
# Output:
#   JSON to stdout with authentication status
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
            echo "  --network NETWORK Network to check: local, testnet, mainnet (default: testnet)" >&2
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

log_info "Checking authentication status..."

# Check if xion-toolkit CLI is available
if ! command -v xion-toolkit &> /dev/null; then
    handle_error "xion-toolkit CLI not found in PATH. Please install xion-agent-toolkit first." "CLI_NOT_FOUND"
fi

# Build command as array (safe from injection)
CMD_ARGS=(auth status --output json)

if [[ -n "$NETWORK" ]]; then
    CMD_ARGS+=(--network "$NETWORK")
fi

log_info "Running: xion-toolkit ${CMD_ARGS[*]}"

# Execute command safely using array expansion
RESULT=$(xion-toolkit --no-interactive "${CMD_ARGS[@]}" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    # Success - output the JSON result
    log_info "Status check successful"
    output_json "$RESULT"
else
    # Error occurred
    log_error "Status check failed: $RESULT"
    
    if echo "$RESULT" | grep -qi "not authenticated\|no credentials\|not logged in"; then
        output_json "{\"success\": true, \"authenticated\": false, \"message\": \"Not authenticated. Use 'login.sh' to authenticate.\"}"
    elif echo "$RESULT" | grep -qi "expired"; then
        output_json "{\"success\": true, \"authenticated\": false, \"message\": \"Token expired. Use 'refresh.sh' or 'login.sh'.\", \"token_expired\": true}"
    else
        handle_error "Failed to check status: $RESULT" "STATUS_CHECK_FAILED"
    fi
fi
