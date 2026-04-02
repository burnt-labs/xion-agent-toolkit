#!/bin/bash
#
# xion-treasury/grant-config.sh - Configure Authz Grants for Treasury
#
# This script configures Authz Grants for a Treasury, supporting all Developer Portal features
# including MsgExecuteContract with ContractExecutionAuthorization.
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
Usage: grant-config.sh <command> [options]

Commands:
  add     Add a grant configuration
  remove  Remove a grant configuration
  list    List all grant configurations

Common Options:
  --address <ADDRESS>     Treasury contract address (required)
  --network <NETWORK>     Network to use: local, testnet, mainnet (default: testnet)

Add Command Options:
  --config <FILE>         JSON config file (alternative to flags)
  
  # Basic authorization
  --type-url <URL>        Message type URL (e.g., /cosmos.bank.v1beta1.MsgSend)
  --auth-type <TYPE>      Authorization type: generic, send, contract-execution, stake, ibc-transfer
  --description <TEXT>    Grant description
  
  # For send authorization
  --spend-limit <AMOUNT>  Spend limit (e.g., 1000000uxion)
  --allow-list <ADDRS>    Comma-separated list of allowed recipients
  
  # For contract-execution authorization
  --contract <ADDRESS>    Contract address (can be used multiple times)
  --max-calls <NUM>       Maximum number of calls (can be used multiple times)
  --max-funds <AMOUNT>    Maximum funds per contract (can be used multiple times)
  --filter-type <TYPE>    Filter type: allow-all, accepted-keys (default: allow-all)
  --keys <KEYS>           Comma-separated list of accepted message keys
  
  # Preset shortcuts
  --preset <TYPE>         Use preset (see Supported Presets below)

Remove Command Options:
  --type-url <URL>        Type URL of the grant to remove (required)

Supported Presets:
  Banking:
    send                   - /cosmos.bank.v1beta1.MsgSend (send authorization)
  
  CosmWasm:
    execute                - /cosmwasm.wasm.v1.MsgExecuteContract (contract-execution)
    instantiate            - /cosmwasm.wasm.v1.MsgInstantiateContract
    instantiate2           - /cosmwasm.wasm.v1.MsgInstantiateContract2
  
  Staking:
    delegate               - /cosmos.staking.v1beta1.MsgDelegate
    undelegate             - /cosmos.staking.v1beta1.MsgUndelegate
    redelegate             - /cosmos.staking.v1beta1.MsgBeginRedelegate
    withdraw-rewards       - /cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward
  
  Governance:
    vote                   - /cosmos.gov.v1beta1.MsgVote
    gov-deposit            - /cosmos.gov.v1beta1.MsgDeposit
    gov-submit-proposal    - /cosmos.gov.v1beta1.MsgSubmitProposal
  
  IBC:
    ibc-transfer           - /ibc.applications.transfer.v1.MsgTransfer
  
  Authz:
    authz-exec             - /cosmos.authz.v1beta1.MsgExec
    authz-revoke           - /cosmos.authz.v1beta1.MsgRevoke
  
  Feegrant:
    feegrant-grant         - /cosmos.feegrant.v1beta1.MsgGrantAllowance
    feegrant-revoke        - /cosmos.feegrant.v1beta1.MsgRevokeAllowance
  
  Other:
    unjail                 - /cosmos.slashing.v1beta1.MsgUnjail
    crisis-verify          - /cosmos.crisis.v1beta1.MsgVerifyInvariant
    evidence-submit        - /cosmos.evidence.v1beta1.MsgSubmitEvidence
    vesting-create         - /cosmos.vesting.v1beta1.MsgCreateVestingAccount
    tokenfactory-mint      - /osmosis.tokenfactory.v1beta1.MsgMint
    tokenfactory-burn      - /osmosis.tokenfactory.v1beta1.MsgBurn

Supported Message Types (for --type-url):
  /cosmos.bank.v1beta1.MsgSend                    - Send funds
  /cosmwasm.wasm.v1.MsgInstantiateContract        - Instantiate smart contract
  /cosmwasm.wasm.v1.MsgInstantiateContract2       - Instantiate2 smart contract
  /cosmwasm.wasm.v1.MsgExecuteContract            - Execute on smart contract
  /cosmos.staking.v1beta1.MsgDelegate             - Delegate tokens
  /cosmos.staking.v1beta1.MsgUndelegate           - Undelegate tokens
  /cosmos.staking.v1beta1.MsgBeginRedelegate      - Redelegate tokens
  /cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward - Withdraw rewards
  /cosmos.gov.v1beta1.MsgVote                     - Vote on proposal
  /ibc.applications.transfer.v1.MsgTransfer       - IBC transfer

Examples:
  # List all grants
  grant-config.sh list --address xion1abc...

  # Add send authorization with spend limit
  grant-config.sh add --address xion1abc... \\
    --type-url "/cosmos.bank.v1beta1.MsgSend" \\
    --auth-type send \\
    --spend-limit "1000000uxion" \\
    --description "Allow sending funds"

  # Add contract-execution authorization (REQUIRED for MsgExecuteContract)
  # Note: Generic authorization is NOT allowed for MsgExecuteContract for security
  grant-config.sh add --address xion1abc... \\
    --type-url "/cosmwasm.wasm.v1.MsgExecuteContract" \\
    --auth-type contract-execution \\
    --contract "xion1contract..." \\
    --max-calls 100 \\
    --max-funds "1000000uxion" \\
    --filter-type allow-all \\
    --description "Execute contract with limits"

  # Add contract-execution with accepted message keys
  grant-config.sh add --address xion1abc... \\
    --type-url "/cosmwasm.wasm.v1.MsgExecuteContract" \\
    --auth-type contract-execution \\
    --contract "xion1contract..." \\
    --max-calls 100 \\
    --filter-type accepted-keys \\
    --keys "transfer,approve" \\
    --description "Execute specific contract methods"

  # Use preset for quick setup
  grant-config.sh add --address xion1abc... \\
    --preset send \\
    --spend-limit "1000000uxion" \\
    --description "Allow sending funds"

  # Use preset for contract execution (requires contract params)
  grant-config.sh add --address xion1abc... \\
    --preset execute \\
    --contract "xion1contract..." \\
    --max-calls 100 \\
    --max-funds "1000000uxion" \\
    --description "Execute contract with limits"

  # Use config file
  grant-config.sh add --address xion1abc... --config grant-config.json

  # Remove a grant
  grant-config.sh remove --address xion1abc... \\
    --type-url "/cosmos.bank.v1beta1.MsgSend"

EOF
}

# ==============================================================================
# Preset Mappings
# ==============================================================================

get_preset_type_url() {
    local preset="$1"
    case "$preset" in
        send)
            echo "/cosmos.bank.v1beta1.MsgSend"
            ;;
        execute)
            echo "/cosmwasm.wasm.v1.MsgExecuteContract"
            ;;
        instantiate)
            echo "/cosmwasm.wasm.v1.MsgInstantiateContract"
            ;;
        instantiate2)
            echo "/cosmwasm.wasm.v1.MsgInstantiateContract2"
            ;;
        delegate)
            echo "/cosmos.staking.v1beta1.MsgDelegate"
            ;;
        undelegate)
            echo "/cosmos.staking.v1beta1.MsgUndelegate"
            ;;
        redelegate)
            echo "/cosmos.staking.v1beta1.MsgBeginRedelegate"
            ;;
        withdraw-rewards)
            echo "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward"
            ;;
        vote)
            echo "/cosmos.gov.v1beta1.MsgVote"
            ;;
        gov-deposit)
            echo "/cosmos.gov.v1beta1.MsgDeposit"
            ;;
        gov-submit-proposal)
            echo "/cosmos.gov.v1beta1.MsgSubmitProposal"
            ;;
        ibc-transfer)
            echo "/ibc.applications.transfer.v1.MsgTransfer"
            ;;
        authz-exec)
            echo "/cosmos.authz.v1beta1.MsgExec"
            ;;
        authz-revoke)
            echo "/cosmos.authz.v1beta1.MsgRevoke"
            ;;
        feegrant-grant)
            echo "/cosmos.feegrant.v1beta1.MsgGrantAllowance"
            ;;
        feegrant-revoke)
            echo "/cosmos.feegrant.v1beta1.MsgRevokeAllowance"
            ;;
        unjail)
            echo "/cosmos.slashing.v1beta1.MsgUnjail"
            ;;
        crisis-verify)
            echo "/cosmos.crisis.v1beta1.MsgVerifyInvariant"
            ;;
        evidence-submit)
            echo "/cosmos.evidence.v1beta1.MsgSubmitEvidence"
            ;;
        vesting-create)
            echo "/cosmos.vesting.v1beta1.MsgCreateVestingAccount"
            ;;
        tokenfactory-mint)
            echo "/osmosis.tokenfactory.v1beta1.MsgMint"
            ;;
        tokenfactory-burn)
            echo "/osmosis.tokenfactory.v1beta1.MsgBurn"
            ;;
        *)
            echo ""
            ;;
    esac
}

get_preset_auth_type() {
    local preset="$1"
    case "$preset" in
        send)
            echo "send"
            ;;
        execute)
            # MsgExecuteContract must use contract-execution for security
            echo "contract-execution"
            ;;
        ibc-transfer)
            echo "ibc-transfer"
            ;;
        instantiate|instantiate2|delegate|undelegate|redelegate|withdraw-rewards|vote|\
        gov-deposit|gov-submit-proposal|authz-exec|authz-revoke|feegrant-grant|\
        feegrant-revoke|unjail|crisis-verify|evidence-submit|vesting-create|\
        tokenfactory-mint|tokenfactory-burn)
            echo "generic"
            ;;
        *)
            echo ""
            ;;
    esac
}

# ==============================================================================
# Argument Parsing
# ==============================================================================

ADDRESS=""
TYPE_URL=""
CONFIG_FILE=""
NETWORK="testnet"
ACTION=""

# For add command
AUTH_TYPE=""
DESCRIPTION=""
SPEND_LIMIT=""
ALLOW_LIST=""

# For contract-execution
CONTRACTS=()
MAX_CALLS=()
MAX_FUNDS=()
FILTER_TYPE="allow-all"
KEYS=""

# Preset
PRESET=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        add|remove|list)
            ACTION="$1"
            shift
            ;;
        --address)
            ADDRESS="$2"
            shift 2
            ;;
        --type-url)
            TYPE_URL="$2"
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
        --auth-type)
            AUTH_TYPE="$2"
            shift 2
            ;;
        --description)
            DESCRIPTION="$2"
            shift 2
            ;;
        --spend-limit)
            SPEND_LIMIT="$2"
            shift 2
            ;;
        --allow-list)
            ALLOW_LIST="$2"
            shift 2
            ;;
        --contract)
            CONTRACTS+=("$2")
            shift 2
            ;;
        --max-calls)
            MAX_CALLS+=("$2")
            shift 2
            ;;
        --max-funds)
            MAX_FUNDS+=("$2")
            shift 2
            ;;
        --filter-type)
            FILTER_TYPE="$2"
            shift 2
            ;;
        --keys)
            KEYS="$2"
            shift 2
            ;;
        --preset)
            PRESET="$2"
            shift 2
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            show_usage
            output_json '{
                "success": false,
                "error": "Unknown option: '"$1"'",
                "error_code": "INVALID_ARGS"
            }'
            exit 1
            ;;
    esac
done

# ==============================================================================
# Validation
# ==============================================================================

if [[ -z "$ACTION" ]]; then
    log_error "Missing required argument: action (add, remove, or list)"
    show_usage
    output_json '{
        "success": false,
        "error": "Missing required argument: action (add, remove, or list)",
        "error_code": "INVALID_ARGS"
    }'
    exit 1
fi

if [[ -z "$ADDRESS" ]]; then
    log_error "Missing required argument: --address"
    output_json '{
        "success": false,
        "error": "Missing required argument: --address",
        "error_code": "INVALID_ARGS"
    }'
    exit 1
fi

# ==============================================================================
# Build Grant Config JSON from Flags
# ==============================================================================

build_grant_config_json() {
    local preset_type_url=""
    local preset_auth_type=""
    
    # Handle preset
    if [[ -n "$PRESET" ]]; then
        preset_type_url=$(get_preset_type_url "$PRESET")
        preset_auth_type=$(get_preset_auth_type "$PRESET")
        
        if [[ -z "$preset_type_url" ]]; then
            log_error "Invalid preset: $PRESET. Valid presets: send, execute, instantiate, instantiate2, delegate, undelegate, redelegate, withdraw-rewards, vote, gov-deposit, gov-submit-proposal, ibc-transfer, authz-exec, authz-revoke, feegrant-grant, feegrant-revoke, unjail, crisis-verify, evidence-submit, vesting-create, tokenfactory-mint, tokenfactory-burn"
            output_json '{
                "success": false,
                "error": "Invalid preset: '"$PRESET"'",
                "error_code": "INVALID_PRESET"
            }'
            exit 1
        fi
        
        # Override with preset values if not explicitly set
        [[ -z "$TYPE_URL" ]] && TYPE_URL="$preset_type_url"
        [[ -z "$AUTH_TYPE" ]] && AUTH_TYPE="$preset_auth_type"
    fi
    
    # Validate required fields for add
    if [[ -z "$TYPE_URL" ]]; then
        log_error "Missing required argument: --type-url or --preset"
        output_json '{
            "success": false,
            "error": "Missing required argument: --type-url or --preset",
            "error_code": "INVALID_ARGS"
        }'
        exit 1
    fi
    
    if [[ -z "$DESCRIPTION" ]]; then
        log_error "Missing required argument: --description"
        output_json '{
            "success": false,
            "error": "Missing required argument: --description",
            "error_code": "INVALID_ARGS"
        }'
        exit 1
    fi
    
    if [[ -z "$AUTH_TYPE" ]]; then
        log_error "Missing required argument: --auth-type or --preset"
        output_json '{
            "success": false,
            "error": "Missing required argument: --auth-type or --preset",
            "error_code": "INVALID_ARGS"
        }'
        exit 1
    fi
    
    # Build authorization based on auth_type
    local auth_json=""
    case "$AUTH_TYPE" in
        generic)
            # Security check: MsgExecuteContract must use contract-execution authorization
            if [[ "$TYPE_URL" == *"MsgExecuteContract"* ]]; then
                log_error "MsgExecuteContract requires contract-execution authorization for security"
                log_error "Generic authorization is NOT allowed as it permits unlimited contract execution"
                output_json '{
                    "success": false,
                    "error": "MsgExecuteContract requires contract-execution authorization. Generic authorization is not allowed for security.",
                    "error_code": "INVALID_AUTH_TYPE"
                }'
                exit 1
            fi
            auth_json='"auth_type": "generic"'
            ;;
        send)
            if [[ -z "$SPEND_LIMIT" ]]; then
                log_error "Missing required argument: --spend-limit for send authorization"
                output_json '{
                    "success": false,
                    "error": "Missing required argument: --spend-limit for send authorization",
                    "error_code": "INVALID_ARGS"
                }'
                exit 1
            fi
            auth_json='"auth_type": "send", "spend_limit": "'"$SPEND_LIMIT"'"'
            if [[ -n "$ALLOW_LIST" ]]; then
                # Convert comma-separated list to JSON array
                local allow_list_json=$(echo "$ALLOW_LIST" | tr ',' '\n' | sed 's/^/"/;s/$/"/' | tr '\n' ',' | sed 's/,$//')
                auth_json='"auth_type": "send", "spend_limit": "'"$SPEND_LIMIT"'", "allow_list": ['"$allow_list_json"']'
            fi
            ;;
        contract-execution)
            if [[ ${#CONTRACTS[@]} -eq 0 ]]; then
                log_error "Missing required argument: --contract for contract-execution authorization"
                output_json '{
                    "success": false,
                    "error": "Missing required argument: --contract for contract-execution authorization",
                    "error_code": "INVALID_ARGS"
                }'
                exit 1
            fi
            
            # Build grants array
            local grants_json="["
            for i in "${!CONTRACTS[@]}"; do
                [[ $i -gt 0 ]] && grants_json+=","
                grants_json+="{"
                grants_json+='"address": "'"${CONTRACTS[$i]}"'"'
                
                # Add max_calls if provided
                if [[ $i -lt ${#MAX_CALLS[@]} ]]; then
                    grants_json+=', "max_calls": '"${MAX_CALLS[$i]}"
                fi
                
                # Add max_funds if provided
                if [[ $i -lt ${#MAX_FUNDS[@]} ]]; then
                    grants_json+=', "max_funds": "'"${MAX_FUNDS[$i]}"'"'
                fi
                
                grants_json+=', "filter_type": "'"$FILTER_TYPE"'"'
                
                # Add keys for accepted-keys filter
                if [[ "$FILTER_TYPE" == "accepted-keys" ]]; then
                    if [[ -z "$KEYS" ]]; then
                        log_error "Missing required argument: --keys for accepted-keys filter"
                        output_json '{
                            "success": false,
                            "error": "Missing required argument: --keys for accepted-keys filter",
                            "error_code": "INVALID_ARGS"
                        }'
                        exit 1
                    fi
                    local keys_json=$(echo "$KEYS" | tr ',' '\n' | sed 's/^/"/;s/$/"/' | tr '\n' ',' | sed 's/,$//')
                    grants_json+=', "keys": ['"$keys_json"']'
                fi
                
                grants_json+="}"
            done
            grants_json+="]"
            
            auth_json='"auth_type": "contract_execution", "grants": '"$grants_json"
            ;;
        stake)
            log_error "Stake authorization requires config file. Use --config option."
            output_json '{
                "success": false,
                "error": "Stake authorization requires config file. Use --config option.",
                "error_code": "INVALID_ARGS"
            }'
            exit 1
            ;;
        ibc-transfer)
            log_error "IBC transfer authorization requires config file. Use --config option."
            output_json '{
                "success": false,
                "error": "IBC transfer authorization requires config file. Use --config option.",
                "error_code": "INVALID_ARGS"
            }'
            exit 1
            ;;
        *)
            log_error "Invalid auth-type: $AUTH_TYPE. Valid types: generic, send, contract-execution, stake, ibc-transfer"
            output_json '{
                "success": false,
                "error": "Invalid auth-type: '"$AUTH_TYPE"'",
                "error_code": "INVALID_ARGS"
            }'
            exit 1
            ;;
    esac
    
    # Build complete JSON
    cat << EOF
{
    "type_url": "$TYPE_URL",
    "description": "$DESCRIPTION",
    "authorization": {$auth_json},
    "optional": false
}
EOF
}

# ==============================================================================
# Main Logic
# ==============================================================================

log_info "Processing grant config: action=$ACTION for treasury $ADDRESS"

# Build the command as array (safe from injection)
CMD=(xion-toolkit --no-interactive --network "$NETWORK")

case "$ACTION" in
    add)
        if [[ -n "$CONFIG_FILE" ]]; then
            # Use config file
            CMD+=(treasury grant-config add --address "$ADDRESS" --config "$CONFIG_FILE")
        else
            # Build config from flags
            GRANT_CONFIG_JSON=$(build_grant_config_json)
            
            # Write to temp file
            TEMP_FILE=$(mktemp)
            echo "$GRANT_CONFIG_JSON" > "$TEMP_FILE"
            
            log_info "Generated grant config:"
            log_info "$GRANT_CONFIG_JSON" >&2
            
            CMD+=(treasury grant-config add --address "$ADDRESS" --config "$TEMP_FILE")
        fi
        ;;
    remove)
        if [[ -z "$TYPE_URL" ]]; then
            log_error "Missing required argument: --type-url for remove action"
            output_json '{
                "success": false,
                "error": "Missing required argument: --type-url for remove action",
                "error_code": "INVALID_ARGS"
            }'
            exit 1
        fi
        CMD+=(treasury grant-config remove --address "$ADDRESS" --type-url "$TYPE_URL")
        ;;
    list)
        CMD+=(treasury grant-config list --address "$ADDRESS")
        ;;
esac

# Execute command safely using array expansion
log_info "Executing: ${CMD[*]}" >&2
OUTPUT=$("${CMD[@]}" 2>&1)
EXIT_CODE=$?

# Clean up temp file if created
if [[ -n "$TEMP_FILE" && -f "$TEMP_FILE" ]]; then
    rm -f "$TEMP_FILE"
fi

# Check if successful
if [ $EXIT_CODE -eq 0 ]; then
    output_json "$OUTPUT"
else
    log_error "Command failed with exit code $EXIT_CODE"
    log_error "Output: $OUTPUT" >&2
    output_json "{
        \"success\": false,
        \"error\": \"Command failed with exit code $EXIT_CODE: $OUTPUT\",
        \"error_code\": \"COMMAND_FAILED\"
    }"
    exit 1
fi