#!/bin/bash
#
# xion-treasury/export.sh - Export Treasury configuration
#
# Exports Treasury configuration (grants, fee config, params) to a JSON file.
#
# Usage:
#   ./export.sh <ADDRESS> [--output FILE] [--network NETWORK]
#
# Arguments:
#   ADDRESS - Treasury contract address (required)
#
# Options:
#   --output FILE   - Output file path (optional, defaults to stdout)
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
OUTPUT_FILE=""
NETWORK="testnet"

# Check for help flag first
if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
    echo "Usage: $0 <ADDRESS> [--output FILE] [--network NETWORK]" >&2
    echo "" >&2
    echo "Arguments:" >&2
    echo "  ADDRESS         Treasury contract address (required)" >&2
    echo "" >&2
    echo "Options:" >&2
    echo "  --output FILE   Output file path (optional, defaults to stdout)" >&2
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
        --output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 <ADDRESS> [--output FILE] [--network NETWORK]" >&2
            echo "" >&2
            echo "Arguments:" >&2
            echo "  ADDRESS         Treasury contract address (required)" >&2
            echo "" >&2
            echo "Options:" >&2
            echo "  --output FILE   Output file path (optional, defaults to stdout)" >&2
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

log_info "Exporting Treasury configuration for: $ADDRESS"

# Build command
CMD="xion-toolkit --network $NETWORK treasury export $ADDRESS --output json"

if [[ -n "$OUTPUT_FILE" ]]; then
    CMD="$CMD --output $OUTPUT_FILE"
fi

log_info "Executing: $CMD"

# Execute command
if [[ -n "$OUTPUT_FILE" ]]; then
    # Output to file
    RESULT=$(eval "$CMD" 2>&1)
    EXIT_CODE=$?
    
    if [[ $EXIT_CODE -eq 0 ]]; then
        log_info "Configuration exported to: $OUTPUT_FILE"
        output_json "{
            \"success\": true,
            \"message\": \"Configuration exported successfully\",
            \"output_file\": \"$OUTPUT_FILE\"
        }"
    else
        log_error "Export failed: $RESULT"
        output_json "{
            \"success\": false,
            \"error\": \"Export failed: $RESULT\",
            \"error_code\": \"EXPORT_FAILED\"
        }"
        exit 1
    fi
else
    # Output to stdout
    RESULT=$(eval "$CMD" 2>&1)
    EXIT_CODE=$?
    
    if [[ $EXIT_CODE -eq 0 ]]; then
        output_json "$RESULT"
    else
        log_error "Export failed: $RESULT"
        output_json "{
            \"success\": false,
            \"error\": \"Export failed: $RESULT\",
            \"error_code\": \"EXPORT_FAILED\"
        }"
        exit 1
    fi
fi
