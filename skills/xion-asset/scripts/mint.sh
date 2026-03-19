#!/bin/bash
set -e

# Asset Mint - Mint NFT token
# Usage: mint.sh --contract <ADDRESS> --token-id <ID> --owner <ADDRESS> [options]
#
# Security: This script uses array-based command execution instead of eval
# to prevent command injection vulnerabilities.

# Parse arguments
CONTRACT=""
TOKEN_ID=""
OWNER=""
TOKEN_URI=""
ASSET_TYPE="cw721-base"
ROYALTY_ADDRESS=""
ROYALTY_PERCENTAGE=""
EXPIRES_AT=""
NETWORK=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --contract) CONTRACT="$2"; shift 2 ;;
        --token-id) TOKEN_ID="$2"; shift 2 ;;
        --owner) OWNER="$2"; shift 2 ;;
        --token-uri) TOKEN_URI="$2"; shift 2 ;;
        --asset-type) ASSET_TYPE="$2"; shift 2 ;;
        --royalty-address) ROYALTY_ADDRESS="$2"; shift 2 ;;
        --royalty-percentage) ROYALTY_PERCENTAGE="$2"; shift 2 ;;
        --expires-at) EXPIRES_AT="$2"; shift 2 ;;
        --network) NETWORK="$2"; shift 2 ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

# Validate required args
if [[ -z "$CONTRACT" ]] || [[ -z "$TOKEN_ID" ]] || [[ -z "$OWNER" ]]; then
    echo '{"success": false, "error": "Missing required args: --contract, --token-id, --owner", "error_code": "MISSING_ARGS"}'
    exit 1
fi

# Build command as array (safe from injection)
CMD=(xion-toolkit asset mint --contract "$CONTRACT" --token-id "$TOKEN_ID" --owner "$OWNER" --asset-type "$ASSET_TYPE" --output json)

if [[ -n "$TOKEN_URI" ]]; then
    CMD+=(--token-uri "$TOKEN_URI")
fi

if [[ -n "$ROYALTY_ADDRESS" ]] && [[ -n "$ROYALTY_PERCENTAGE" ]]; then
    CMD+=(--royalty-address "$ROYALTY_ADDRESS" --royalty-percentage "$ROYALTY_PERCENTAGE")
fi

if [[ -n "$EXPIRES_AT" ]]; then
    CMD+=(--expires-at "$EXPIRES_AT")
fi

if [[ -n "$NETWORK" ]]; then
    CMD+=(--network "$NETWORK")
fi

# Execute command safely using array expansion
OUTPUT=$("${CMD[@]}" 2>&1)
EXIT_CODE=$?

# Output result and propagate exit code
if [[ $EXIT_CODE -ne 0 ]]; then
    echo "$OUTPUT"
    exit $EXIT_CODE
fi
echo "$OUTPUT"
