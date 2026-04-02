#!/bin/bash
#
# xion-oauth2/login.sh - Initiate OAuth2 login flow
#
# This script initiates the OAuth2 login flow via the xion CLI.
# It opens a browser for user authorization and returns JSON status.
#
# Usage:
#   ./login.sh [--port PORT] [--network NETWORK] [--dev-mode]
#
# Output:
#   JSON to stdout with authentication result
#   Status messages to stderr

set -e

# ==============================================================================
# Helper Functions
# ==============================================================================

# Output JSON to stdout
output_json() {
    echo "$1"
}

# Log message to stderr
log_info() {
    echo "[INFO] $1" >&2
}

log_error() {
    echo "[ERROR] $1" >&2
}

# Handle errors and output JSON error response
handle_error() {
    local message="$1"
    local code="${2:-UNKNOWN_ERROR}"
    output_json "{\"success\": false, \"error\": \"$message\", \"error_code\": \"$code\"}"
    exit 1
}

# ==============================================================================
# Argument Parsing
# ==============================================================================

PORT=""
NETWORK=""
FORCE=""
DEV_MODE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --port)
            PORT="$2"
            shift 2
            ;;
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --force)
            FORCE="--force"
            shift
            ;;
        --dev-mode)
            DEV_MODE="--dev-mode"
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]" >&2
            echo "" >&2
            echo "Options:" >&2
            echo "  --port PORT       Callback server port (default: 54321)" >&2
            echo "  --network NET     Network: local, testnet, mainnet (default: testnet)" >&2
            echo "  --force           Force new browser authentication (skip refresh check)" >&2
            echo "  --dev-mode        Request Manager API scopes (xion:mgr:read, xion:mgr:write)" >&2
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

log_info "Starting Xion OAuth2 login..."

# Check if xion-toolkit CLI is available
if ! command -v xion-toolkit &> /dev/null; then
    handle_error "xion-toolkit CLI not found in PATH. Please install xion-agent-toolkit first." "CLI_NOT_FOUND"
fi

# Build command arguments
CMD_ARGS=()
CMD_ARGS+=("auth" "login" "--output" "json")

if [[ -n "$PORT" ]]; then
    CMD_ARGS+=("--port" "$PORT")
fi

if [[ -n "$NETWORK" ]]; then
    CMD_ARGS+=("--network" "$NETWORK")
fi

if [[ -n "$FORCE" ]]; then
    CMD_ARGS+=("--force")
fi

if [[ -n "$DEV_MODE" ]]; then
    CMD_ARGS+=("--dev-mode")
fi

# Convert array to string for logging
CMD_STR="${CMD_ARGS[*]}"
log_info "Running: xion-toolkit $CMD_STR"

# Execute xion-toolkit login command
# Capture both stdout and stderr, but keep them separate
RESULT=$(xion-toolkit --no-interactive "${CMD_ARGS[@]}" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    # Success - output the JSON result from CLI
    log_info "Login successful"
    output_json "$RESULT"
else
    # Error occurred
    log_error "Login failed: $RESULT"
    
    # Parse error type and return appropriate error code
    if echo "$RESULT" | grep -qi "already logged in"; then
        output_json "{\"success\": true, \"message\": \"Already logged in\", \"note\": \"Use 'logout' first to re-authenticate\"}"
    elif echo "$RESULT" | grep -qi "timeout"; then
        handle_error "Login timed out. Please try again." "TIMEOUT"
    elif echo "$RESULT" | grep -qi "port.*in use"; then
        handle_error "Port is already in use. Try a different port with --port option." "PORT_IN_USE"
    elif echo "$RESULT" | grep -qi "network"; then
        handle_error "Network error. Check your connection and try again." "NETWORK_ERROR"
    else
        handle_error "Login failed: $RESULT" "AUTH_FAILED"
    fi
fi
