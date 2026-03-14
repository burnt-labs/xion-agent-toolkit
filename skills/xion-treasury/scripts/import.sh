#!/bin/bash
#
# xion-treasury/import.sh - Import Treasury configuration
#
# Imports configuration (grants, fee config, params) to a Treasury.
#
# Usage:
#   ./import.sh <ADDRESS> --from-file <FILE> [--dry-run] [--network NETWORK]
#
# Arguments:
#   ADDRESS - Treasury contract address (required)
#
# Options:
#   --from-file FILE - Path to JSON configuration file (required)
#   --dry-run        - Preview actions without executing (optional)
#   --network NETWORK - Network: testnet, mainnet (default: testnet)
#

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
DRY_RUN=false
NETWORK="testnet"

# Check for help flag first
if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
    echo "Usage: $0 <ADDRESS> --from-file <FILE> [--dry-run] [--network NETWORK]" >&2
    echo "" >&2
    echo "Arguments:" >&2
    echo "  ADDRESS         Treasury contract address (required)" >&2
    echo "" >&2
    echo "Options:" >&2
    echo "  --from-file FILE Path to JSON configuration file (required)" >&2
    echo "  --dry-run        Preview actions without executing (optional)" >&2
    echo "  --network NETWORK Network: testnet, mainnet (default: testnet)" >&2
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
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 <ADDRESS> --from-file <FILE> [--dry-run] [--network NETWORK]" >&2
            echo "" >&2
            echo "Arguments:" >&2
            echo "  ADDRESS         Treasury contract address (required)" >&2
            echo "" >&2
            echo "Options:" >&2
            echo "  --from-file FILE Path to JSON configuration file (required)" >&2
            echo "  --dry-run        Preview actions without executing (optional)" >&2
            echo "  --network NETWORK Network: testnet, mainnet (default: testnet)" >&2
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

# Build command
CMD="xion-toolkit --network $NETWORK treasury import $ADDRESS --from-file $FROM_FILE --output json"

if [[ "$DRY_RUN" == "true" ]]; then
    CMD="$CMD --dry-run"
fi

log_info "Executing: $CMD"

# Execute command
RESULT=$(eval "$CMD" 2>&1)
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
