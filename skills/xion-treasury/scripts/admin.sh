#!/bin/bash
#
# xion-treasury/admin.sh - Admin management operations
#
# This script manages Treasury admin operations: propose, accept, cancel.
#
# Usage:
#   ./admin.sh <address> propose --new-admin <address> [--network NETWORK]
#   ./admin.sh <address> accept [--network NETWORK]
#   ./admin.sh <address> cancel [--network NETWORK]
#
# Arguments:
#   ADDRESS  - Treasury contract address (required)
#   ACTION   - Action to perform: propose, accept, cancel (required)
#
# Options:
#   --new-admin <ADDRESS>   - New admin address (required for propose)
#   --network <NETWORK>     - Network: local, testnet, mainnet (default: testnet)
#
# Output:
#   JSON to stdout with success status and transaction details
#
# Examples:
#   ./admin.sh xion1abc... propose --new-admin xion1newadmin...
#   ./admin.sh xion1abc... accept
#   ./admin.sh xion1abc... cancel --network mainnet

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

show_usage() {
    cat << 'EOF' >&2
Usage: admin.sh <ADDRESS> <ACTION> [options]

Actions:
  propose  Propose a new admin for the treasury
  accept   Accept admin role (called by pending admin)
  cancel   Cancel proposed admin

Options:
  --new-admin <ADDRESS>  New admin address (required for propose)
  --network <NETWORK>    Network: local, testnet, mainnet (default: testnet)

Examples:
  # Propose new admin
  admin.sh xion1abc... propose --new-admin xion1newadmin...

  # Accept admin role (must be called by pending admin)
  admin.sh xion1abc... accept

  # Cancel proposed admin
  admin.sh xion1abc... cancel

  # Specify network
  admin.sh xion1abc... propose --new-admin xion1newadmin... --network mainnet
EOF
}

# ==============================================================================
# Argument Parsing
# ==============================================================================

ADDRESS=""
ACTION=""
NEW_ADMIN=""
NETWORK="testnet"

while [[ $# -gt 0 ]]; do
    case $1 in
        propose|accept|cancel)
            ACTION="$1"
            shift
            ;;
        --new-admin)
            NEW_ADMIN="$2"
            shift 2
            ;;
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --help|-h)
            show_usage
            exit 0
            ;;
        -*)
            output_json "{
  \"success\": false,
  \"error\": \"Unknown option: $1\",
  \"error_code\": \"INVALID_ARGUMENT\"
}"
            exit 1
            ;;
        *)
            if [[ -z "$ADDRESS" ]]; then
                ADDRESS="$1"
            else
                output_json "{
  \"success\": false,
  \"error\": \"Too many arguments\",
  \"error_code\": \"INVALID_ARGUMENT\"
}"
                exit 1
            fi
            shift
            ;;
    esac
done

# ==============================================================================
# Validation
# ==============================================================================

if [[ -z "$ADDRESS" ]]; then
    output_json '{
  "success": false,
  "error": "Treasury address is required",
  "error_code": "MISSING_ADDRESS",
  "usage": "./admin.sh <ADDRESS> <ACTION> [options]"
}'
    exit 1
fi

if [[ -z "$ACTION" ]]; then
    output_json '{
  "success": false,
  "error": "Action is required (propose, accept, cancel)",
  "error_code": "MISSING_ACTION",
  "usage": "./admin.sh <ADDRESS> <ACTION> [options]"
}'
    exit 1
fi

if [[ "$ACTION" == "propose" && -z "$NEW_ADMIN" ]]; then
    output_json '{
  "success": false,
  "error": "--new-admin is required for propose action",
  "error_code": "MISSING_NEW_ADMIN",
  "usage": "./admin.sh <ADDRESS> propose --new-admin <ADDRESS>"
}'
    exit 1
fi

# ==============================================================================
# Check CLI
# ==============================================================================

if ! command -v xion-toolkit &> /dev/null; then
    CLI_CMD=(cargo run --quiet --)
else
    CLI_CMD=(xion-toolkit)
fi

# ==============================================================================
# Main Logic
# ==============================================================================

log_info "Executing admin action '$ACTION' on treasury $ADDRESS..."

# Build and execute command based on action (using array for safety)
case "$ACTION" in
    propose)
        log_info "Proposing new admin: $NEW_ADMIN"
        RESULT=$("${CLI_CMD[@]}" treasury admin propose "$ADDRESS" --new-admin "$NEW_ADMIN" --network "$NETWORK" --output json 2>&1)
        ;;
    accept)
        log_info "Accepting admin role for treasury"
        RESULT=$("${CLI_CMD[@]}" treasury admin accept "$ADDRESS" --network "$NETWORK" --output json 2>&1)
        ;;
    cancel)
        log_info "Canceling proposed admin"
        RESULT=$("${CLI_CMD[@]}" treasury admin cancel "$ADDRESS" --network "$NETWORK" --output json 2>&1)
        ;;
esac

EXIT_CODE=$?

if [[ $EXIT_CODE -eq 0 ]]; then
    log_info "Admin action '$ACTION' completed successfully"
    output_json "$RESULT"
else
    log_error "Admin action '$ACTION' failed"
    if echo "$RESULT" | jq -e . > /dev/null 2>&1; then
        output_json "$RESULT"
    else
        output_json "{
  \"success\": false,
  \"error\": $(echo "$RESULT" | jq -Rs .),
  \"error_code\": \"ADMIN_ACTION_FAILED\"
}"
    fi
    exit 1
fi
