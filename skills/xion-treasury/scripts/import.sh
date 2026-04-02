#!/bin/bash
#
# xion-treasury/import.sh - Import Treasury configuration
#
# Imports Treasury configuration (grants, fee config, params) from a JSON file.
#
# Usage:
#   ./import.sh <ADDRESS> --from-file <FILE> [--network NETWORK] [--dry-run]
#
# Arguments:
#   ADDRESS - Treasury contract address (required)
#
# Options:
#   --from-file FILE   Configuration file to import (required)
#   --network NETWORK  Network: testnet, mainnet (default: testnet)
#   --dry-run          Preview changes without executing
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
FROM_FILE=""
NETWORK="testnet"
DRY_RUN="false"

# Check for help flag first
if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
    echo "Usage: $0 <ADDRESS> --from-file <FILE> [--network NETWORK] [--dry-run]" >&2
    echo "" >&2
    echo "Arguments:" >&2
    echo "  ADDRESS            Treasury contract address (required)" >&2
    echo "" >&2
    echo "Options:" >&2
    echo "  --from-file FILE   Configuration file to import (required)" >&2
    echo "  --network NETWORK  Network: testnet, mainnet (default: testnet)" >&2
    echo "  --dry-run          Preview changes without executing" >&2
    exit 0
fi

# First argument is address
if [[ $# -gt 0 ]]; then
    ADDRESS="$1"
    shift
fi

while [[ $# -gt 0 ]]; do
    case $1 in
        --from-file)
            FROM_FILE="$2"
            shift 2
            ;;
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --dry-run)
            DRY_RUN="true"
            shift
            ;;
        --help|-h)
            echo "Usage: $0 <ADDRESS> --from-file <FILE> [--network NETWORK] [--dry-run]" >&2
            echo "" >&2
            echo "Arguments:" >&2
            echo "  ADDRESS            Treasury contract address (required)" >&2
            echo "" >&2
            echo "Options:" >&2
            echo "  --from-file FILE   Configuration file to import (required)" >&2
            echo "  --network NETWORK  Network: testnet, mainnet (default: testnet)" >&2
            echo "  --dry-run          Preview changes without executing" >&2
            exit 0
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

if [[ -z "$ADDRESS" ]]; then
    output_json '{
        "success": false,
        "error": "Missing required argument: ADDRESS",
        "error_code": "MISSING_ARGS"
    }'
    exit 1
fi

if [[ -z "$FROM_FILE" ]]; then
    output_json '{
        "success": false,
        "error": "Missing required argument: --from-file",
        "error_code": "MISSING_ARGS"
    }'
    exit 1
fi

if [[ ! -f "$FROM_FILE" ]]; then
    output_json "{
        \"success\": false,
        \"error\": \"Configuration file not found: $FROM_FILE\",
        \"error_code\": \"FILE_NOT_FOUND\"
    }"
    exit 1
fi

if ! command -v xion-toolkit &> /dev/null; then
    output_json '{
        "success": false,
        "error": "xion-toolkit CLI not found in PATH",
        "error_code": "CLI_NOT_FOUND"
    }'
    exit 1
fi

# ==============================================================================
# Main Logic
# ==============================================================================

log_info "Importing configuration to Treasury: $ADDRESS"
log_info "Configuration file: $FROM_FILE"

if [[ "$DRY_RUN" == "true" ]]; then
    log_info "DRY RUN MODE - No transactions will be executed"
fi

# Build command as array (safe from injection)
CMD=(xion-toolkit --no-interactive --network "$NETWORK" treasury import "$ADDRESS" --from-file "$FROM_FILE" --output json)

if [[ "$DRY_RUN" == "true" ]]; then
    CMD+=(--dry-run)
fi

log_info "Executing: ${CMD[*]}"

# Execute command safely using array expansion
RESULT=$("${CMD[@]}" 2>&1)
EXIT_CODE=$?

if [[ $EXIT_CODE -eq 0 ]]; then
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "Dry run completed successfully"
    else
        log_info "Configuration imported successfully"
    fi
    output_json "$RESULT"
else
    log_error "Import failed: $RESULT"
    output_json "{
        \"success\": false,
        \"error\": \"Import failed: $RESULT\",
        \"error_code\": \"IMPORT_FAILED\"
    }"
    exit 1
fi
