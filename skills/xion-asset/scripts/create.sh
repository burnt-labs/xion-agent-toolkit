#!/bin/bash
set -e

# Asset Create - Create NFT collection
# Usage: create.sh --type <TYPE> --name <NAME> --symbol <SYMBOL> [options]

# Parse arguments
TYPE=""
NAME=""
SYMBOL=""
MINTER=""
SALT=""
NETWORK=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --type) TYPE="$2"; shift 2 ;;
        --name) NAME="$2"; shift 2 ;;
        --symbol) SYMBOL="$2"; shift 2 ;;
        --minter) MINTER="$2"; shift 2 ;;
        --salt) SALT="$2"; shift 2 ;;
        --network) NETWORK="$2"; shift 2 ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

# Validate required args
if [[ -z "$TYPE" ]] || [[ -z "$NAME" ]] || [[ -z "$SYMBOL" ]]; then
    echo '{"success": false, "error": "Missing required args: --type, --name, --symbol", "error_code": "MISSING_ARGS"}'
    exit 1
fi

# Build command
CMD="xion-toolkit asset create --type $TYPE --name \"$NAME\" --symbol \"$SYMBOL\" --output json"

if [[ -n "$MINTER" ]]; then
    CMD="$CMD --minter $MINTER"
fi

if [[ -n "$SALT" ]]; then
    CMD="$CMD --salt \"$SALT\""
fi

if [[ -n "$NETWORK" ]]; then
    CMD="$CMD --network $NETWORK"
fi

# Execute command and capture output
OUTPUT=$(eval $CMD 2>&1)
EXIT_CODE=$?

# Output result and propagate exit code
if [[ $EXIT_CODE -ne 0 ]]; then
    echo "$OUTPUT"
    exit $EXIT_CODE
fi
echo "$OUTPUT"
