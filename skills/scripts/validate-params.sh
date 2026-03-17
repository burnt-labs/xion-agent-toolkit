#!/bin/bash
#
# validate-params.sh - Validate skill parameters against schema
#
# Usage: validate-params.sh <skill> <command> '<json-params>'
#
# Example:
#   validate-params.sh xion-treasury grant-config-add '{"address": "xion1abc...", "auth-type": "send"}'
#
# Output:
#   {
#     "valid": false,
#     "missing": ["type-url", "spend-limit"],
#     "errors": [
#       {"param": "spend-limit", "message": "Required when auth-type=send"}
#     ]
#   }
#
# Exit codes:
#   0 - Validation passed (valid: true)
#   1 - Validation failed (valid: false or error)
#   2 - Script/usage error

set -e
set -o pipefail

# ==============================================================================
# Configuration
# ==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKILLS_DIR="$(dirname "$SCRIPT_DIR")"

# ==============================================================================
# Helper Functions
# ==============================================================================

output_json() {
    echo "$1"
}

log_error() {
    echo "[ERROR] $1" >&2
}

show_usage() {
    cat << 'EOF' >&2
Usage: validate-params.sh <skill> <command> '<json-params>'

Arguments:
  skill       - Skill name (e.g., xion-treasury, xion-asset, xion-oauth2)
  command     - Command name with dashes (e.g., grant-config-add, create, mint)
  json-params - JSON string with parameters to validate

Examples:
  # Validate treasury grant-config add
  validate-params.sh xion-treasury grant-config-add '{"address": "xion1abc...", "auth-type": "send"}'

  # Validate asset mint
  validate-params.sh xion-asset mint '{"contract": "xion1abc...", "token-id": "1", "owner": "xion1..."}'

  # Validate oauth2 login
  validate-params.sh xion-oauth2 login '{}'

Output Format:
  Success:
    {"valid": true, "missing": [], "errors": []}

  Missing parameters:
    {"valid": false, "missing": ["param1", "param2"], "errors": [...]}

  Invalid dependencies:
    {"valid": false, "missing": [], "errors": [{"param": "...", "message": "..."}]}
EOF
}

# ==============================================================================
# Argument Parsing
# ==============================================================================

if [[ $# -lt 3 ]]; then
    log_error "Missing required arguments"
    show_usage
    output_json '{
        "valid": false,
        "error": "Missing required arguments: skill, command, json-params",
        "error_code": "MISSING_ARGS"
    }'
    exit 2
fi

SKILL="$1"
COMMAND="$2"
PARAMS_JSON="$3"

# ==============================================================================
# Find Schema File
# ==============================================================================

SCHEMA_FILE="$SKILLS_DIR/$SKILL/schemas/$COMMAND.json"

if [[ ! -f "$SCHEMA_FILE" ]]; then
    output_json "{
        \"valid\": false,
        \"error\": \"Schema file not found: $SCHEMA_FILE\",
        \"error_code\": \"SCHEMA_NOT_FOUND\",
        \"suggestion\": \"Check if the skill and command names are correct\"
    }"
    exit 1
fi

# ==============================================================================
# Validation Logic
# ==============================================================================

# Check if jq is available
if ! command -v jq &> /dev/null; then
    output_json '{
        "valid": false,
        "error": "jq is required but not installed",
        "error_code": "JQ_NOT_FOUND"
    }'
    exit 2
fi

# Parse parameters
if ! PARAMS=$(echo "$PARAMS_JSON" | jq -e . 2>/dev/null); then
    output_json '{
        "valid": false,
        "error": "Invalid JSON in parameters",
        "error_code": "INVALID_JSON"
    }'
    exit 1
fi

# Extract schema parameters - handle both flat parameters and subcommands
SCHEMA_HAS_SUBCOMMANDS=$(cat "$SCHEMA_FILE" | jq -r 'has("subcommands")')

if [[ "$SCHEMA_HAS_SUBCOMMANDS" == "true" ]]; then
    # Schema uses subcommands structure - extract based on action parameter
    ACTION_VALUE=$(echo "$PARAMS" | jq -r '.action // empty')
    
    if [[ -z "$ACTION_VALUE" ]]; then
        output_json '{
            "valid": false,
            "missing": ["action"],
            "errors": [{"param": "action", "message": "Required parameter to determine subcommand"}]
        }'
        exit 1
    fi
    
    # Check if action is a valid subcommand
    VALID_ACTION=$(cat "$SCHEMA_FILE" | jq -r --arg action "$ACTION_VALUE" '.subcommands | has($action)')
    
    if [[ "$VALID_ACTION" != "true" ]]; then
        VALID_ACTIONS=$(cat "$SCHEMA_FILE" | jq -r '.subcommands | keys | join(", ")')
        output_json "{
            \"valid\": false,
            \"missing\": [],
            \"errors\": [{\"param\": \"action\", \"message\": \"Invalid action '$ACTION_VALUE'. Valid actions: $VALID_ACTIONS\"}]
        }"
        exit 1
    fi
    
    SCHEMA_PARAMS=$(cat "$SCHEMA_FILE" | jq -r --arg action "$ACTION_VALUE" '.subcommands[$action].parameters // []')
else
    SCHEMA_PARAMS=$(cat "$SCHEMA_FILE" | jq -r '.parameters // []')
fi

PRESETS=$(cat "$SCHEMA_FILE" | jq -r '.presets // {}')

# Arrays to collect results
MISSING_PARAMS=()
ERRORS=()

# Check required parameters
while IFS= read -r param; do
    NAME=$(echo "$param" | jq -r '.name')
    REQUIRED=$(echo "$param" | jq -r '.required')
    CONFLICTS_WITH=$(echo "$param" | jq -c '.conflicts_with // empty')
    DEPENDS_ON=$(echo "$param" | jq -c '.depends_on // empty')
    DEFAULT=$(echo "$param" | jq -r '.default // empty')
    
    # Convert parameter name to JSON key (support both - and _ formats)
    JSON_KEY_UNDERSCORE=$(echo "$NAME" | tr '-' '_')
    JSON_KEY_HYPHEN="$NAME"
    
    # Check if parameter is provided (in either format)
    HAS_PARAM_UNDERSCORE=$(echo "$PARAMS" | jq --arg key "$JSON_KEY_UNDERSCORE" 'has($key)')
    HAS_PARAM_HYPHEN=$(echo "$PARAMS" | jq --arg key "$JSON_KEY_HYPHEN" 'has($key)')
    
    if [[ "$HAS_PARAM_UNDERSCORE" == "true" || "$HAS_PARAM_HYPHEN" == "true" ]]; then
        HAS_PARAM="true"
    else
        HAS_PARAM="false"
    fi
    
    # Check for conflicts (supports both string and array formats)
    if [[ -n "$CONFLICTS_WITH" && "$CONFLICTS_WITH" != "null" ]]; then
        # Detect if conflicts_with is an array or string
        if [[ "$CONFLICTS_WITH" == "["* ]]; then
            # Array format - iterate over each conflict
            while IFS= read -r CONFLICT_KEY; do
                CONFLICT_KEY_UNDERSCORE=$(echo "$CONFLICT_KEY" | tr '-' '_')
                CONFLICT_KEY_HYPHEN="$CONFLICT_KEY"
                HAS_CONFLICT_UNDERSCORE=$(echo "$PARAMS" | jq --arg key "$CONFLICT_KEY_UNDERSCORE" 'has($key)')
                HAS_CONFLICT_HYPHEN=$(echo "$PARAMS" | jq --arg key "$CONFLICT_KEY_HYPHEN" 'has($key)')
                
                if [[ "$HAS_CONFLICT_UNDERSCORE" == "true" || "$HAS_CONFLICT_HYPHEN" == "true" ]]; then
                    HAS_CONFLICT="true"
                else
                    HAS_CONFLICT="false"
                fi
                
                if [[ "$HAS_PARAM" == "true" && "$HAS_CONFLICT" == "true" ]]; then
                    ERRORS+=("{\"param\": \"$NAME\", \"message\": \"Conflicts with $CONFLICT_KEY - use only one\"}")
                fi
            done < <(echo "$CONFLICTS_WITH" | jq -r '.[]')
        else
            # String format - single conflict
            CONFLICT_KEY_UNDERSCORE=$(echo "$CONFLICTS_WITH" | tr '-' '_')
            CONFLICT_KEY_HYPHEN="$CONFLICTS_WITH"
            HAS_CONFLICT_UNDERSCORE=$(echo "$PARAMS" | jq --arg key "$CONFLICT_KEY_UNDERSCORE" 'has($key)')
            HAS_CONFLICT_HYPHEN=$(echo "$PARAMS" | jq --arg key "$CONFLICT_KEY_HYPHEN" 'has($key)')
            
            if [[ "$HAS_CONFLICT_UNDERSCORE" == "true" || "$HAS_CONFLICT_HYPHEN" == "true" ]]; then
                HAS_CONFLICT="true"
            else
                HAS_CONFLICT="false"
            fi
            
            if [[ "$HAS_PARAM" == "true" && "$HAS_CONFLICT" == "true" ]]; then
                ERRORS+=("{\"param\": \"$NAME\", \"message\": \"Conflicts with $CONFLICTS_WITH - use only one\"}")
            fi
        fi
    fi
    
    # Check enum constraint if parameter is provided
    if [[ "$HAS_PARAM" == "true" ]]; then
        ENUM_VALUES=$(echo "$param" | jq -c '.enum // empty')
        if [[ -n "$ENUM_VALUES" && "$ENUM_VALUES" != "null" ]]; then
            # Get the actual value (try both formats)
            ACTUAL_VALUE=$(echo "$PARAMS" | jq -r --arg key_u "$JSON_KEY_UNDERSCORE" --arg key_h "$JSON_KEY_HYPHEN" '
                if has($key_u) then .[$key_u] 
                elif has($key_h) then .[$key_h] 
                else empty end
            ')
            
            # Check if value is in enum
            IS_VALID_ENUM=$(echo "$ENUM_VALUES" | jq --arg val "$ACTUAL_VALUE" 'index($val) != null')
            if [[ "$IS_VALID_ENUM" == "false" ]]; then
                VALID_ENUMS=$(echo "$ENUM_VALUES" | jq -r 'join(", ")')
                ERRORS+=("{\"param\": \"$NAME\", \"message\": \"Invalid value '$ACTUAL_VALUE'. Must be one of: $VALID_ENUMS\"}")
            fi
        fi
    fi
    
    # Check required status
    if [[ "$REQUIRED" == "true" ]]; then
        # Check if there's a preset that might satisfy this
        HAS_PRESET=$(echo "$PARAMS" | jq 'has("preset")')
        
        # If missing and no preset, check if conditionally required
        if [[ "$HAS_PARAM" == "false" ]]; then
            # Check if this is conditionally required based on depends_on
            IS_CONDITIONAL=false
            
            if [[ -n "$DEPENDS_ON" && "$DEPENDS_ON" != "null" ]]; then
                IS_CONDITIONAL=true
                # Detect if depends_on is a string or object
                if [[ "$DEPENDS_ON" == "\""* ]]; then
                    # String format - parameter name only, any non-empty value triggers
                    DEP_PARAM=$(echo "$DEPENDS_ON" | tr -d '"')
                    DEP_VALUES="any"  # Any value satisfies
                else
                    # Object format
                    DEP_PARAM=$(echo "$DEPENDS_ON" | jq -r '.parameter // empty')
                    # Check if values key exists
                    HAS_VALUES_KEY=$(echo "$DEPENDS_ON" | jq 'has("values")')
                    if [[ "$HAS_VALUES_KEY" == "true" ]]; then
                        DEP_VALUES=$(echo "$DEPENDS_ON" | jq -r '.values // [] | @json')
                    else
                        DEP_VALUES="any"  # Any value satisfies
                    fi
                fi
                
                if [[ -n "$DEP_PARAM" ]]; then
                    DEP_KEY_UNDERSCORE=$(echo "$DEP_PARAM" | tr '-' '_')
                    DEP_KEY_HYPHEN="$DEP_PARAM"
                    
                    DEP_VALUE_UNDERSCORE=$(echo "$PARAMS" | jq -r --arg key "$DEP_KEY_UNDERSCORE" '.[$key] // empty')
                    DEP_VALUE_HYPHEN=$(echo "$PARAMS" | jq -r --arg key "$DEP_KEY_HYPHEN" '.[$key] // empty')
                    
                    # Use whichever format was provided
                    if [[ -n "$DEP_VALUE_UNDERSCORE" ]]; then
                        DEP_VALUE="$DEP_VALUE_UNDERSCORE"
                    else
                        DEP_VALUE="$DEP_VALUE_HYPHEN"
                    fi
                    
                    if [[ -n "$DEP_VALUE" ]]; then
                        # Check dependency logic
                        if [[ "$DEP_VALUES" == "any" ]]; then
                            # Any non-empty value triggers requirement
                            MISSING_PARAMS+=("\"$NAME\"")
                            ERRORS+=("{\"param\": \"$NAME\", \"message\": \"Required when $DEP_PARAM is set\"}")
                        else
                            # Object format with values: only specific values trigger
                            IS_IN_DEP_VALUES=$(echo "$DEP_VALUES" | jq --arg val "$DEP_VALUE" 'index($val) != null')
                            if [[ "$IS_IN_DEP_VALUES" == "true" ]]; then
                                MISSING_PARAMS+=("\"$NAME\"")
                                ERRORS+=("{\"param\": \"$NAME\", \"message\": \"Required when $DEP_PARAM=$DEP_VALUE\"}")
                            fi
                        fi
                    fi
                fi
            fi
            
            # Only add to missing if not conditional (conditional params handled above)
            if [[ "$IS_CONDITIONAL" == "false" ]]; then
                MISSING_PARAMS+=("\"$NAME\"")
            fi
        fi
    fi
done < <(echo "$SCHEMA_PARAMS" | jq -c '.[]')

# Check for conditionally required parameters (required: false but has depends_on)
# When dependency is met, these parameters become required
while IFS= read -r param; do
    NAME=$(echo "$param" | jq -r '.name')
    REQUIRED=$(echo "$param" | jq -r '.required')
    DEPENDS_ON=$(echo "$param" | jq -c '.depends_on // empty')
    
    # Only check parameters that are not marked as required but have depends_on
    if [[ "$REQUIRED" == "false" && -n "$DEPENDS_ON" && "$DEPENDS_ON" != "null" ]]; then
        JSON_KEY_UNDERSCORE=$(echo "$NAME" | tr '-' '_')
        JSON_KEY_HYPHEN="$NAME"
        
        # Check if parameter is provided
        HAS_PARAM_UNDERSCORE=$(echo "$PARAMS" | jq --arg key "$JSON_KEY_UNDERSCORE" 'has($key)')
        HAS_PARAM_HYPHEN=$(echo "$PARAMS" | jq --arg key "$JSON_KEY_HYPHEN" 'has($key)')
        
        if [[ "$HAS_PARAM_UNDERSCORE" == "true" || "$HAS_PARAM_HYPHEN" == "true" ]]; then
            continue  # Parameter is provided, no need to check
        fi
        
        # Parse dependency
        if [[ "$DEPENDS_ON" == "\""* ]]; then
            DEP_PARAM=$(echo "$DEPENDS_ON" | tr -d '"')
            DEP_VALUES="any"
        else
            DEP_PARAM=$(echo "$DEPENDS_ON" | jq -r '.parameter // empty')
            HAS_VALUES_KEY=$(echo "$DEPENDS_ON" | jq 'has("values")')
            if [[ "$HAS_VALUES_KEY" == "true" ]]; then
                DEP_VALUES=$(echo "$DEPENDS_ON" | jq -r '.values // []')
            else
                DEP_VALUES="any"
            fi
        fi
        
        if [[ -n "$DEP_PARAM" ]]; then
            DEP_KEY_UNDERSCORE=$(echo "$DEP_PARAM" | tr '-' '_')
            DEP_KEY_HYPHEN="$DEP_PARAM"
            
            DEP_VALUE_UNDERSCORE=$(echo "$PARAMS" | jq -r --arg key "$DEP_KEY_UNDERSCORE" '.[$key] // empty')
            DEP_VALUE_HYPHEN=$(echo "$PARAMS" | jq -r --arg key "$DEP_KEY_HYPHEN" '.[$key] // empty')
            
            if [[ -n "$DEP_VALUE_UNDERSCORE" ]]; then
                DEP_VALUE="$DEP_VALUE_UNDERSCORE"
            else
                DEP_VALUE="$DEP_VALUE_HYPHEN"
            fi
            
            # Check if dependency is met
            if [[ -n "$DEP_VALUE" ]]; then
                if [[ "$DEP_VALUES" == "any" ]]; then
                    MISSING_PARAMS+=("\"$NAME\"")
                    ERRORS+=("{\"param\": \"$NAME\", \"message\": \"Required when $DEP_PARAM is set\"}")
                else
                    IS_IN_DEP_VALUES=$(echo "$DEP_VALUES" | jq --arg val "$DEP_VALUE" 'index($val) != null')
                    if [[ "$IS_IN_DEP_VALUES" == "true" ]]; then
                        MISSING_PARAMS+=("\"$NAME\"")
                        ERRORS+=("{\"param\": \"$NAME\", \"message\": \"Required when $DEP_PARAM=$DEP_VALUE\"}")
                    fi
                fi
            fi
        fi
    fi
done < <(echo "$SCHEMA_PARAMS" | jq -c '.[]')

# Validate preset requirements
PRESET_NAME=$(echo "$PARAMS" | jq -r '.preset // empty')
if [[ -n "$PRESET_NAME" && "$PRESET_NAME" != "null" ]]; then
    # Check if preset exists in schema (don't use -e to avoid exit on false)
    PRESET_EXISTS=$(echo "$PRESETS" | jq --arg name "$PRESET_NAME" 'has($name)')
    if [[ "$PRESET_EXISTS" != "true" ]]; then
        AVAILABLE_PRESETS=$(echo "$PRESETS" | jq -r 'keys | join(", ")')
        ERRORS+=("{\"param\": \"preset\", \"message\": \"Invalid preset value '$PRESET_NAME'. Valid presets: $AVAILABLE_PRESETS\"}")
    else
        # Preset exists, check its requirements
        PRESET_REQUIRES=$(echo "$PRESETS" | jq -r --arg name "$PRESET_NAME" '.[$name].requires // [] | @json')
    
        if [[ "$PRESET_REQUIRES" != "null" && "$PRESET_REQUIRES" != "[]" ]]; then
            while IFS= read -r req; do
                REQ_KEY_UNDERSCORE=$(echo "$req" | tr '-' '_')
                REQ_KEY_HYPHEN="$req"
                
                HAS_REQ_UNDERSCORE=$(echo "$PARAMS" | jq --arg key "$REQ_KEY_UNDERSCORE" 'has($key)')
                HAS_REQ_HYPHEN=$(echo "$PARAMS" | jq --arg key "$REQ_KEY_HYPHEN" 'has($key)')
                
                if [[ "$HAS_REQ_UNDERSCORE" == "true" || "$HAS_REQ_HYPHEN" == "true" ]]; then
                    HAS_REQ="true"
                else
                    HAS_REQ="false"
                fi
                
                if [[ "$HAS_REQ" == "false" ]]; then
                    MISSING_PARAMS+=("\"$req\"")
                    ERRORS+=("{\"param\": \"$req\", \"message\": \"Required for preset '$PRESET_NAME'\"}")
                fi
            done < <(echo "$PRESET_REQUIRES" | jq -r '.[]')
        fi
    
        # Build effective params that includes preset values for dependency checks
        PRESET_VALUES=$(echo "$PRESETS" | jq -r --arg name "$PRESET_NAME" '.[$name] # get preset object')
        if [[ "$PRESET_VALUES" != "null" && -n "$PRESET_VALUES" ]]; then
            # Merge preset values into params (preset values don't override user-provided params)
            EFFECTIVE_PARAMS=$(echo "$PARAMS" | jq --argjson preset "$PRESET_VALUES" '$preset * .')
        else
            EFFECTIVE_PARAMS="$PARAMS"
        fi
    fi
else
    EFFECTIVE_PARAMS="$PARAMS"
fi

# Check parameter dependencies
while IFS= read -r param; do
    NAME=$(echo "$param" | jq -r '.name')
    DEPENDS_ON=$(echo "$param" | jq -c '.depends_on // empty')
    
    JSON_KEY_UNDERSCORE=$(echo "$NAME" | tr '-' '_')
    JSON_KEY_HYPHEN="$NAME"
    
    HAS_PARAM_UNDERSCORE=$(echo "$PARAMS" | jq --arg key "$JSON_KEY_UNDERSCORE" 'has($key)')
    HAS_PARAM_HYPHEN=$(echo "$PARAMS" | jq --arg key "$JSON_KEY_HYPHEN" 'has($key)')
    
    if [[ "$HAS_PARAM_UNDERSCORE" == "true" || "$HAS_PARAM_HYPHEN" == "true" ]]; then
        HAS_PARAM="true"
    else
        HAS_PARAM="false"
    fi
    
    # If parameter is provided but dependency is not met
    if [[ "$HAS_PARAM" == "true" && -n "$DEPENDS_ON" && "$DEPENDS_ON" != "null" ]]; then
        # Detect if depends_on is a string or object
        if [[ "$DEPENDS_ON" == "\""* ]]; then
            # String format - parameter name only, any non-empty value triggers
            DEP_PARAM=$(echo "$DEPENDS_ON" | tr -d '"')
            DEP_VALUES="any"  # Any value satisfies
        else
            # Object format
            DEP_PARAM=$(echo "$DEPENDS_ON" | jq -r '.parameter // empty')
            # Check if values key exists
            HAS_VALUES_KEY=$(echo "$DEPENDS_ON" | jq 'has("values")')
            if [[ "$HAS_VALUES_KEY" == "true" ]]; then
                DEP_VALUES=$(echo "$DEPENDS_ON" | jq -r '.values // []')
            else
                DEP_VALUES="any"  # Any value satisfies
            fi
        fi
        
        if [[ -n "$DEP_PARAM" ]]; then
            DEP_KEY_UNDERSCORE=$(echo "$DEP_PARAM" | tr '-' '_')
            DEP_KEY_HYPHEN="$DEP_PARAM"
            
            DEP_VALUE_UNDERSCORE=$(echo "$EFFECTIVE_PARAMS" | jq -r --arg key "$DEP_KEY_UNDERSCORE" '.[$key] // empty')
            DEP_VALUE_HYPHEN=$(echo "$EFFECTIVE_PARAMS" | jq -r --arg key "$DEP_KEY_HYPHEN" '.[$key] // empty')
            
            # Use whichever format was provided
            if [[ -n "$DEP_VALUE_UNDERSCORE" ]]; then
                DEP_VALUE="$DEP_VALUE_UNDERSCORE"
            else
                DEP_VALUE="$DEP_VALUE_HYPHEN"
            fi
            
            if [[ -z "$DEP_VALUE" ]]; then
                ERRORS+=("{\"param\": \"$NAME\", \"message\": \"Requires $DEP_PARAM to be set\"}")
            elif [[ "$DEP_VALUES" == "any" ]]; then
                # Any non-empty value is valid - no error
                :
            else
                # Object format with values: check if value is in allowed list
                IS_IN_DEP_VALUES=$(echo "$DEP_VALUES" | jq --arg val "$DEP_VALUE" 'index($val) != null')
                if [[ "$IS_IN_DEP_VALUES" == "false" ]]; then
                    EXPECTED=$(echo "$DEP_VALUES" | jq -r 'join(", ")')
                    ERRORS+=("{\"param\": \"$NAME\", \"message\": \"Only valid when $DEP_PARAM is one of: $EXPECTED (got $DEP_VALUE)\"}")
                fi
            fi
        fi
    fi
done < <(echo "$SCHEMA_PARAMS" | jq -c '.[]')

# ==============================================================================
# Build Output
# ==============================================================================

if [[ ${#MISSING_PARAMS[@]} -eq 0 && ${#ERRORS[@]} -eq 0 ]]; then
    output_json '{
        "valid": true,
        "missing": [],
        "errors": []
    }'
    exit 0
else
    MISSING_STR="[]"
    ERRORS_STR="[]"
    
    if [[ ${#MISSING_PARAMS[@]} -gt 0 ]]; then
        MISSING_STR="[$(IFS=,; echo "${MISSING_PARAMS[*]}")]"
    fi
    
    if [[ ${#ERRORS[@]} -gt 0 ]]; then
        ERRORS_STR="[$(IFS=,; echo "${ERRORS[*]}")]"
    fi
    
    output_json "{
        \"valid\": false,
        \"missing\": $MISSING_STR,
        \"errors\": $ERRORS_STR
    }"
    exit 1
fi