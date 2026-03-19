#!/bin/bash
#
# xion-treasury/create.sh - Create a new Treasury contract
#
# Creates a Treasury contract with optional fee grant and authz grant configuration.
# Supports CLI flags for simple configurations and config file for complex setups.
#
# Usage:
#   ./create.sh [OPTIONS]
#
# Options:
#   --network NETWORK           Network: testnet, mainnet (default: testnet)
#   --name NAME                 Treasury name (required if not using --config)
#   --redirect-url URL          OAuth redirect URL
#   --icon-url URL              Treasury icon URL
#   --config FILE               JSON config file with all settings
#
# Fee Grant Options:
#   --fee-allowance-type TYPE   Fee allowance: basic, periodic, allowed-msg
#   --fee-spend-limit AMOUNT    Spend limit (e.g., "1000000uxion")
#   --fee-description TEXT      Fee grant description
#   --fee-period-seconds N      Period duration (for periodic allowance)
#   --fee-period-spend-limit N  Period spend limit (for periodic allowance)
#
# Authz Grant Options:
#   --grant-type-url URL        Message type URL (e.g., /cosmos.bank.v1beta1.MsgSend)
#   --grant-auth-type TYPE      Authorization: generic, send, stake, ibc-transfer, contract-execution
#   --grant-spend-limit AMOUNT  Spend limit (for send authorization)
#   --grant-description TEXT    Grant description
#
# Output:
#   JSON to stdout with creation result
#
# Security: This script uses array-based command execution instead of eval
# to prevent command injection vulnerabilities.

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

check_cli() {
    if ! command -v xion-toolkit &> /dev/null; then
        output_json '{
            "success": false,
            "error": "xion-toolkit CLI not found in PATH",
            "error_code": "CLI_NOT_FOUND",
            "suggestion": "Install xion-toolkit CLI or ensure it is in your PATH"
        }'
        exit 1
    fi
}

# ==============================================================================
# Argument Parsing
# ==============================================================================

NETWORK=""
NAME=""
REDIRECT_URL=""
ICON_URL=""
CONFIG_FILE=""
FEE_ALLOWANCE_TYPE=""
FEE_SPEND_LIMIT=""
FEE_DESCRIPTION=""
FEE_PERIOD_SECONDS=""
FEE_PERIOD_SPEND_LIMIT=""
GRANT_TYPE_URL=""
GRANT_AUTH_TYPE=""
GRANT_SPEND_LIMIT=""
GRANT_DESCRIPTION=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --name)
            NAME="$2"
            shift 2
            ;;
        --redirect-url)
            REDIRECT_URL="$2"
            shift 2
            ;;
        --icon-url)
            ICON_URL="$2"
            shift 2
            ;;
        --config)
            CONFIG_FILE="$2"
            shift 2
            ;;
        --fee-allowance-type)
            FEE_ALLOWANCE_TYPE="$2"
            shift 2
            ;;
        --fee-spend-limit)
            FEE_SPEND_LIMIT="$2"
            shift 2
            ;;
        --fee-description)
            FEE_DESCRIPTION="$2"
            shift 2
            ;;
        --fee-period-seconds)
            FEE_PERIOD_SECONDS="$2"
            shift 2
            ;;
        --fee-period-spend-limit)
            FEE_PERIOD_SPEND_LIMIT="$2"
            shift 2
            ;;
        --grant-type-url)
            GRANT_TYPE_URL="$2"
            shift 2
            ;;
        --grant-auth-type)
            GRANT_AUTH_TYPE="$2"
            shift 2
            ;;
        --grant-spend-limit)
            GRANT_SPEND_LIMIT="$2"
            shift 2
            ;;
        --grant-description)
            GRANT_DESCRIPTION="$2"
            shift 2
            ;;
        *)
            log_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# ==============================================================================
# Validation
# ==============================================================================

if [ -z "$CONFIG_FILE" ] && [ -z "$NAME" ]; then
    output_json '{
        "success": false,
        "error": "Either --name or --config is required",
        "error_code": "MISSING_REQUIRED_ARG"
    }'
    exit 1
fi

# ==============================================================================
# Main Logic
# ==============================================================================

check_cli

log_info "Creating Treasury contract..."

# Build CLI command as array (safe from injection)
CMD=(xion-toolkit treasury create --output json)

# Add network if specified
if [ -n "$NETWORK" ]; then
    CMD+=(--network "$NETWORK")
fi

# If using config file, use it directly
if [ -n "$CONFIG_FILE" ]; then
    if [ ! -f "$CONFIG_FILE" ]; then
        output_json "{
            \"success\": false,
            \"error\": \"Config file not found: $CONFIG_FILE\",
            \"error_code\": \"FILE_NOT_FOUND\"
        }"
        exit 1
    fi
    CMD+=(--config "$CONFIG_FILE")
    log_info "Using config file: $CONFIG_FILE"
else
    # Build from individual flags
    if [ -n "$NAME" ]; then
        CMD+=(--name "$NAME")
    fi
    if [ -n "$REDIRECT_URL" ]; then
        CMD+=(--redirect-url "$REDIRECT_URL")
    fi
    if [ -n "$ICON_URL" ]; then
        CMD+=(--icon-url "$ICON_URL")
    fi
    
    # Fee grant configuration
    if [ -n "$FEE_ALLOWANCE_TYPE" ]; then
        CMD+=(--fee-allowance-type "$FEE_ALLOWANCE_TYPE")
    fi
    if [ -n "$FEE_SPEND_LIMIT" ]; then
        CMD+=(--fee-spend-limit "$FEE_SPEND_LIMIT")
    fi
    if [ -n "$FEE_DESCRIPTION" ]; then
        CMD+=(--fee-description "$FEE_DESCRIPTION")
    fi
    if [ -n "$FEE_PERIOD_SECONDS" ]; then
        CMD+=(--fee-period-seconds "$FEE_PERIOD_SECONDS")
    fi
    if [ -n "$FEE_PERIOD_SPEND_LIMIT" ]; then
        CMD+=(--fee-period-spend-limit "$FEE_PERIOD_SPEND_LIMIT")
    fi
    
    # Authz grant configuration
    if [ -n "$GRANT_TYPE_URL" ]; then
        CMD+=(--grant-type-url "$GRANT_TYPE_URL")
    fi
    if [ -n "$GRANT_AUTH_TYPE" ]; then
        CMD+=(--grant-auth-type "$GRANT_AUTH_TYPE")
    fi
    if [ -n "$GRANT_SPEND_LIMIT" ]; then
        CMD+=(--grant-spend-limit "$GRANT_SPEND_LIMIT")
    fi
    if [ -n "$GRANT_DESCRIPTION" ]; then
        CMD+=(--grant-description "$GRANT_DESCRIPTION")
    fi
fi

# Execute CLI command safely using array expansion
log_info "Executing: ${CMD[*]}"
RESULT=$("${CMD[@]}" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    # Success - output the JSON from CLI
    output_json "$RESULT"
    log_info "Treasury created successfully"
else
    # Error from CLI
    output_json "{
        \"success\": false,
        \"error\": \"Failed to create treasury\",
        \"error_code\": \"TREASURY_CREATE_FAILED\",
        \"details\": $(echo "$RESULT" | jq -R .)
    }"
    exit 1
fi
