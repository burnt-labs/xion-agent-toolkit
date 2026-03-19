#!/bin/bash
#
# xion-treasury/fee-config.sh - Configure Fee Grants for Treasury
#
# This script configures Fee Grants for a Treasury using the xion-toolkit CLI.
#
# Usage:
#   ./fee-config.sh <ADDRESS> --action <ACTION> [--config FILE] [--network NETWORK]
#
# Actions:
#   set    - Set fee configuration
#   remove - Remove fee configuration
#   query  - Query current fee configuration
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

show_usage() {
    cat << 'EOF' >&2
Usage: fee-config.sh <ADDRESS> --action <ACTION> [OPTIONS]

Arguments:
  ADDRESS           Treasury contract address (required)

Options:
  --action ACTION   Action: set, remove, query (required)
  --config FILE     JSON configuration file (required for 'set' action)
  --network NETWORK Network: testnet, mainnet (default: testnet)

Actions:
  set    - Set fee configuration from JSON file
  remove - Remove fee configuration (revoke allowance)
  query  - Query current fee configuration

Examples:
  # Set fee configuration
  fee-config.sh xion1treasury... --action set --config fee-config.json

  # Query fee configuration
  fee-config.sh xion1treasury... --action query

  # Remove fee configuration
  fee-config.sh xion1treasury... --action remove

Example Config (fee-config.json):
  {
    "basic": {
      "spend_limit": "1000000uxion",
      "description": "Basic fee allowance"
    }
  }

  For periodic allowance:
  {
    "periodic": {
      "basic_spend_limit": "10000000uxion",
      "period_seconds": 86400,
      "period_spend_limit": "1000000uxion",
      "description": "Daily allowance"
    }
  }
EOF
}

# ==============================================================================
# Argument Parsing
# ==============================================================================

ADDRESS=""
ACTION=""
CONFIG_FILE=""
NETWORK="testnet"

# Check for help flag first
if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
    show_usage
    exit 0
fi

# First argument is address
if [[ $# -gt 0 ]]; then
    ADDRESS="$1"
    shift
fi

while [[ $# -gt 0 ]]; do
    case $1 in
        --action)
            ACTION="$2"
            shift 2
            ;;
        --config)
            CONFIG_FILE="$2"
            shift 2
            ;;
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --help|-h)
            show_usage
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            show_usage
            output_json '{
                "success": false,
                "error": "Unknown option",
                "error_code": "INVALID_ARGS"
            }'
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

if [[ -z "$ACTION" ]]; then
    output_json '{
        "success": false,
        "error": "Missing required argument: --action (set, remove, or query)",
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

# Validate action
case "$ACTION" in
    set|remove|query)
        ;;
    *)
        output_json '{
            "success": false,
            "error": "Invalid action. Must be: set, remove, or query",
            "error_code": "INVALID_ACTION"
        }'
        exit 1
        ;;
esac

# Validate config file for 'set' action
if [[ "$ACTION" == "set" ]] && [[ -z "$CONFIG_FILE" ]]; then
    output_json '{
        "success": false,
        "error": "Missing required argument: --config (required for set action)",
        "error_code": "MISSING_ARGS"
    }'
    exit 1
fi

if [[ "$ACTION" == "set" ]] && [[ ! -f "$CONFIG_FILE" ]]; then
    output_json "{
        \"success\": false,
        \"error\": \"Config file not found: $CONFIG_FILE\",
        \"error_code\": \"FILE_NOT_FOUND\"
    }"
    exit 1
fi

# ==============================================================================
# Main Logic
# ==============================================================================

log_info "Processing fee config: action=$ACTION for treasury $ADDRESS"

# Build command as array (safe from injection)
CMD=(xion-toolkit --network "$NETWORK" treasury fee-config)

case "$ACTION" in
    set)
        CMD+=(set --config "$CONFIG_FILE")
        ;;
    remove)
        CMD+=(remove)
        ;;
    query)
        CMD+=(query)
        ;;
esac

CMD+=("$ADDRESS" --output json)

log_info "Executing: ${CMD[*]}"

# Execute command safely using array expansion
RESULT=$("${CMD[@]}" 2>&1)
EXIT_CODE=$?

if [[ $EXIT_CODE -eq 0 ]]; then
    log_info "Fee config operation completed successfully"
    output_json "$RESULT"
else
    log_error "Command failed with exit code $EXIT_CODE"
    log_error "Output: $RESULT"
    output_json "{
        \"success\": false,
        \"error\": \"Command failed: $RESULT\",
        \"error_code\": \"COMMAND_FAILED\"
    }"
    exit 1
fi
