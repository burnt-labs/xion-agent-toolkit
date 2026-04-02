#!/bin/bash
set -e

# Asset Batch Mint - Batch mint from JSON file
# Usage: batch-mint.sh --contract <ADDRESS> --tokens-file <FILE> [--asset-type <TYPE>]

# Parse arguments
CONTRACT=""
TOKENS_FILE=""
ASSET_TYPE="cw721-base"

while [[ $# -gt 0 ]]; do
    case $1 in
        --contract) CONTRACT="$2"; shift 2 ;;
        --tokens-file) TOKENS_FILE="$2"; shift 2 ;;
        --asset-type) ASSET_TYPE="$2"; shift 2 ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

# Validate required args
if [[ -z "$CONTRACT" ]] || [[ -z "$TOKENS_FILE" ]]; then
    echo '{"success": false, "error": "Missing required args: --contract, --tokens-file", "error_code": "MISSING_ARGS"}'
    exit 1
fi

# Check file exists
if [[ ! -f "$TOKENS_FILE" ]]; then
    echo "{\"success\": false, \"error\": \"Tokens file not found: $TOKENS_FILE\", \"error_code\": \"FILE_NOT_FOUND\"}"
    exit 1
fi

# Execute command and capture output
OUTPUT=$(xion-toolkit --no-interactive asset batch-mint --contract "$CONTRACT" --tokens-file "$TOKENS_FILE" --asset-type "$ASSET_TYPE" --output json 2>&1)
EXIT_CODE=$?

# Output result and propagate exit code
if [[ $EXIT_CODE -ne 0 ]]; then
    echo "$OUTPUT"
    exit $EXIT_CODE
fi
echo "$OUTPUT"
