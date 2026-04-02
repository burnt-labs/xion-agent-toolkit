#!/bin/bash
set -e

# Asset Predict - Predict contract address
# Usage: predict.sh --type <TYPE> --name <NAME> --symbol <SYMBOL> --salt <SALT> [options]
#
# Security: This script uses array-based command execution instead of eval
# to prevent command injection vulnerabilities.

# Parse arguments
TYPE=""
NAME=""
SYMBOL=""
SALT=""
MINTER=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --type) TYPE="$2"; shift 2 ;;
        --name) NAME="$2"; shift 2 ;;
        --symbol) SYMBOL="$2"; shift 2 ;;
        --salt) SALT="$2"; shift 2 ;;
        --minter) MINTER="$2"; shift 2 ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

# Validate required args
if [[ -z "$TYPE" ]] || [[ -z "$NAME" ]] || [[ -z "$SYMBOL" ]] || [[ -z "$SALT" ]]; then
    echo '{"success": false, "error": "Missing required args: --type, --name, --symbol, --salt", "error_code": "MISSING_ARGS"}' >&2
    exit 1
fi

# Build command as array (safe from injection)
CMD=(xion-toolkit --no-interactive asset predict --type "$TYPE" --name "$NAME" --symbol "$SYMBOL" --salt "$SALT" --output json)

if [[ -n "$MINTER" ]]; then
    CMD+=(--minter "$MINTER")
fi

# Execute command safely using array expansion
"${CMD[@]}"
