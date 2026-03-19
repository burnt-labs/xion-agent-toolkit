#!/bin/bash
#
# xion-treasury/update-params.sh - Update Treasury parameters
#
# This script updates Treasury contract parameters (redirect URL, icon URL, metadata).
#
# Usage:
#   ./update-params.sh <ADDRESS> [options]
#
# Arguments:
#   ADDRESS  - Treasury contract address (required)
#
# Options:
#   --redirect-url <URL>    - OAuth redirect URL
#   --icon-url <URL>        - Treasury icon URL
#   --metadata <JSON>       - Metadata as JSON string
#   --network <NETWORK>     - Network: local, testnet, mainnet (default: testnet)
#
# Output:
#   JSON to stdout with success status and transaction details
#
# Examples:
#   ./update-params.sh xion1abc... --redirect-url "https://example.com/callback"
#   ./update-params.sh xion1abc... --metadata '{"name":"My Treasury"}'
#   ./update-params.sh xion1abc... --redirect-url "https://app.com" --icon-url "https://app.com/icon.png"
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

# ==============================================================================
# Argument Parsing
# ==============================================================================

ADDRESS=""
REDIRECT_URL=""
ICON_URL=""
METADATA=""
NETWORK="testnet"

while [[ $# -gt 0 ]]; do
    case $1 in
        --redirect-url)
            REDIRECT_URL="$2"
            shift 2
            ;;
        --icon-url)
            ICON_URL="$2"
            shift 2
            ;;
        --metadata)
            METADATA="$2"
            shift 2
            ;;
        --network)
            NETWORK="$2"
            shift 2
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
  "usage": "./update-params.sh <ADDRESS> [options]"
}'
    exit 1
fi

if [[ -z "$REDIRECT_URL" && -z "$ICON_URL" && -z "$METADATA" ]]; then
    output_json '{
  "success": false,
  "error": "At least one parameter to update is required (--redirect-url, --icon-url, or --metadata)",
  "error_code": "MISSING_PARAMS"
}'
    exit 1
fi

# ==============================================================================
# Build CLI Command (using array for safety)
# ==============================================================================

# Build command as array (safe from injection)
if command -v xion-toolkit &> /dev/null; then
    CMD=(xion-toolkit treasury params update "$ADDRESS")
else
    CMD=(cargo run --quiet -- treasury params update "$ADDRESS")
fi

# Add optional parameters
[[ -n "$REDIRECT_URL" ]] && CMD+=(--redirect-url "$REDIRECT_URL")
[[ -n "$ICON_URL" ]] && CMD+=(--icon-url "$ICON_URL")
[[ -n "$METADATA" ]] && CMD+=(--metadata "$METADATA")
CMD+=(--network "$NETWORK" --output json)

# ==============================================================================
# Main Logic
# ==============================================================================

log_info "Updating treasury $ADDRESS parameters on $NETWORK..."

# Execute the command safely using array expansion
RESULT=$("${CMD[@]}" 2>&1)
EXIT_CODE=$?

if [[ $EXIT_CODE -eq 0 ]]; then
    log_info "Treasury parameters updated successfully"
    output_json "$RESULT"
else
    log_error "Failed to update treasury parameters"
    if echo "$RESULT" | jq -e . > /dev/null 2>&1; then
        output_json "$RESULT"
    else
        output_json "{
  \"success\": false,
  \"error\": $(echo "$RESULT" | jq -Rs .),
  \"error_code\": \"UPDATE_PARAMS_FAILED\"
}"
    fi
    exit 1
fi
