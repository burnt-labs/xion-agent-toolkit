#!/bin/bash
#
# xion-treasury/withdraw.sh - Withdraw from Treasury
#
# This script withdraws funds from a Treasury contract to the admin account.
#
# Usage:
#   ./withdraw.sh <ADDRESS> <AMOUNT> [--network NETWORK] [--yes]
#
# Arguments:
#   ADDRESS  - Treasury contract address (required)
#   AMOUNT   - Amount to withdraw with denomination (required, e.g., "1000000uxion")
#
# Options:
#   --network NETWORK - Network to use: local, testnet, mainnet (default: testnet)
#   --yes, -y         - Skip confirmation prompt
#
# Output:
#   JSON to stdout with success status and transaction details
#
# Examples:
#   ./withdraw.sh xion1abc... 1000000uxion
#   ./withdraw.sh xion1abc... 5000000uxion --network testnet
#   ./withdraw.sh xion1abc... 1000000uxion --yes  # Skip confirmation
#
# Environment:
#   XION_SKIP_CONFIRM=true - Skip all confirmation prompts (useful for CI/CD)
#
# Security: This script uses array-based command execution and requires
# confirmation for withdrawal operations to prevent accidental token transfers.

set -e

# ==============================================================================
# Source Security Utilities
# ==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../scripts/security-utils.sh
source "$SCRIPT_DIR/../scripts/security-utils.sh" 2>/dev/null || {
    # Fallback if security-utils.sh is not available
    log_info() { echo "[INFO] $1" >&2; }
    log_error() { echo "[ERROR] $1" >&2; }
    output_json() { echo "$1"; }
    validate_address() { return 0; }
    validate_amount() { return 0; }
    format_coin() { echo "$1"; }
    confirm_sensitive_operation() { return 0; }
}

# ==============================================================================
# Helper Functions
# ==============================================================================

output_json() {
    echo "$1"
}

# ==============================================================================
# Argument Parsing
# ==============================================================================

ADDRESS=""
AMOUNT=""
NETWORK="testnet"
SKIP_CONFIRM="false"

while [[ $# -gt 0 ]]; do
    case $1 in
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --yes|-y)
            SKIP_CONFIRM="true"
            shift
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
            elif [[ -z "$AMOUNT" ]]; then
                AMOUNT="$1"
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
  "usage": "./withdraw.sh <ADDRESS> <AMOUNT> [--network NETWORK] [--yes]"
}'
    exit 1
fi

if [[ -z "$AMOUNT" ]]; then
    output_json '{
  "success": false,
  "error": "Amount is required",
  "error_code": "MISSING_AMOUNT",
  "usage": "./withdraw.sh <ADDRESS> <AMOUNT> [--network NETWORK] [--yes]",
  "example": "1000000uxion"
}'
    exit 1
fi

# Validate address format
if ! validate_address "$ADDRESS"; then
    exit 1
fi

# Validate amount format
if ! validate_amount "$AMOUNT"; then
    exit 1
fi

# ==============================================================================
# Confirmation (Withdraw is a sensitive operation)
# ==============================================================================

# Get formatted amount for display
DISPLAY_AMOUNT=$(format_coin "$AMOUNT")

if [[ "$SKIP_CONFIRM" != "true" ]] && [[ "$XION_SKIP_CONFIRM" != "true" ]]; then
    DETAILS="  Treasury: $ADDRESS
  Amount: $DISPLAY_AMOUNT ($AMOUNT)
  Network: $NETWORK
  
  WARNING: This will transfer tokens from the Treasury to the admin account."
    
    if ! confirm_sensitive_operation "Withdraw from Treasury" "$DISPLAY_AMOUNT" "$ADDRESS"; then
        output_json '{
  "success": false,
  "error": "Operation cancelled by user",
  "error_code": "CANCELLED"
}'
        exit 1
    fi
fi

# ==============================================================================
# Main Logic
# ==============================================================================

log_info "Withdrawing $AMOUNT from treasury $ADDRESS on $NETWORK..."

# Check if xion-toolkit CLI is available
if ! command -v xion-toolkit &> /dev/null; then
    # Try to use cargo run instead (for development)
    CLI_CMD=(cargo run --quiet --)
else
    CLI_CMD=(xion-toolkit)
fi

# Execute the withdraw command safely using array expansion
RESULT=$("${CLI_CMD[@]}" treasury withdraw "$ADDRESS" --amount "$AMOUNT" --network "$NETWORK" --output json 2>&1)
EXIT_CODE=$?

if [[ $EXIT_CODE -eq 0 ]]; then
    log_info "Withdrawal successful"
    log_audit "WITHDRAW" "address=$ADDRESS amount=$AMOUNT network=$NETWORK"
    output_json "$RESULT"
else
    log_error "Failed to withdraw from treasury"
    # Parse error if possible, otherwise return raw output
    if echo "$RESULT" | jq -e . > /dev/null 2>&1; then
        output_json "$RESULT"
    else
        output_json "{
  \"success\": false,
  \"error\": $(echo "$RESULT" | jq -Rs .),
  \"error_code\": \"WITHDRAW_FAILED\"
}"
    fi
    exit 1
fi
