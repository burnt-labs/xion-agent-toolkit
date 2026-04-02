#!/bin/bash
set -e

# Asset Query - Query NFT contract
# Usage: query.sh --contract <ADDRESS> --msg <JSON>

# Parse arguments
CONTRACT=""
MSG=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --contract) CONTRACT="$2"; shift 2 ;;
        --msg) MSG="$2"; shift 2 ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

# Validate required args
if [[ -z "$CONTRACT" ]] || [[ -z "$MSG" ]]; then
    echo '{"success": false, "error": "Missing required args: --contract, --msg", "error_code": "MISSING_ARGS"}' >&2
    exit 1
fi

xion-toolkit --no-interactive asset query --contract "$CONTRACT" --msg "$MSG" --output json
