#!/bin/bash
#
# xion-oauth2-client/rotate-secret.sh - Rotate OAuth2 client secret
#
# This script rotates the client secret for a confidential OAuth2 client.
# The new secret is returned ONLY ONCE and must be stored securely.
#
# Requirements:
#   - xion-toolkit CLI installed
#   - Authenticated with --dev-mode (xion:mgr:write scope)
#   - Must be the client owner
#   - Client must be confidential (auth_method: client_secret_basic/post)
#
# Usage:
#   ./rotate-secret.sh --client-id <CLIENT_ID>
#
# Output:
#   JSON to stdout with the new client secret
#   Warning to stderr about saving the secret

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

log_warn() {
    echo "[WARN] $1" >&2
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

CLIENT_ID=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --client-id)
            CLIENT_ID="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 --client-id <CLIENT_ID>" >&2
            echo "" >&2
            echo "Rotate the client secret for a confidential OAuth2 client." >&2
            echo "" >&2
            echo "Arguments:" >&2
            echo "  --client-id ID    OAuth2 client ID (required)" >&2
            echo "" >&2
            echo "Notes:" >&2
            echo "  - Requires authentication with --dev-mode" >&2
            echo "  - Only the client owner can rotate the secret" >&2
            echo "  - Only confidential clients support secrets" >&2
            echo "  - The new secret is returned ONLY ONCE" >&2
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            handle_error "Unknown option: $1" "INVALID_ARGUMENT"
            ;;
    esac
done

# ==============================================================================
# Validation
# ==============================================================================

if [[ -z "$CLIENT_ID" ]]; then
    handle_error "Missing required argument: --client-id" "MISSING_CLIENT_ID"
fi

# ==============================================================================
# Main Logic
# ==============================================================================

log_info "Rotating OAuth2 client secret..."
log_info "Client ID: $CLIENT_ID"

# Check if xion-toolkit CLI is available
if ! command -v xion-toolkit &> /dev/null; then
    handle_error "xion-toolkit CLI not found in PATH. Please install xion-agent-toolkit first." "CLI_NOT_FOUND"
fi

# Build command arguments
CMD_ARGS=("oauth2" "client" "rotate-secret" "$CLIENT_ID" "--output" "json")

# Convert array to string for logging
CMD_STR="${CMD_ARGS[*]}"
log_info "Running: xion-toolkit --no-interactive $CMD_STR"

# Execute xion-toolkit rotate-secret command
# Capture both stdout and stderr, but keep them separate
RESULT=$(xion-toolkit --no-interactive "${CMD_ARGS[@]}" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    # Success - output the JSON result from CLI
    log_info "Client secret rotated successfully"
    
    # Output the result (contains the new clientSecret)
    output_json "$RESULT"
    
    # Print warning to stderr about saving the secret
    log_warn ""
    log_warn "=========================================="
    log_warn "  SAVE THE CLIENT SECRET ABOVE IMMEDIATELY"
    log_warn "  IT WILL NOT BE SHOWN AGAIN"
    log_warn "=========================================="
    log_warn ""
else
    # Error occurred
    log_error "Failed to rotate client secret: $RESULT"
    
    # Parse error type and return appropriate error code
    if echo "$RESULT" | grep -qi "not authenticated"; then
        handle_error "Not authenticated. Run 'xion-toolkit auth login --dev-mode' first." "EOAUTHCLIENT008"
    elif echo "$RESULT" | grep -qi "insufficient scope\|xion:mgr:write"; then
        handle_error "Insufficient scope. Re-login with --dev-mode for xion:mgr:write scope." "EOAUTHCLIENT010"
    elif echo "$RESULT" | grep -qi "only owner\|not owner"; then
        handle_error "Only the client owner can rotate the secret." "EOAUTHCLIENT011"
    elif echo "$RESULT" | grep -qi "not found\|client not found"; then
        handle_error "Client not found: $CLIENT_ID" "EOAUTHCLIENT012"
    elif echo "$RESULT" | grep -qi "not confidential\|public client"; then
        handle_error "Cannot rotate secret for public client. Only confidential clients support secrets." "CLIENT_NOT_CONFIDENTIAL"
    else
        handle_error "Failed to rotate client secret: $RESULT" "ROTATE_SECRET_FAILED"
    fi
fi