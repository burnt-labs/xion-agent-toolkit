#!/bin/bash
#
# security-utils.sh - Shared security utilities for skill scripts
#
# This script provides reusable security functions for all skill scripts:
# - Operation confirmation prompts
# - Input validation
# - Audit logging
#
# Usage:
#   source ./scripts/security-utils.sh
#
# Security best practices:
# - Always confirm sensitive operations (fund, withdraw, grant config)
# - Log all operations for audit trail
# - Validate inputs before execution

# ==============================================================================
# Configuration
# ==============================================================================

# Set to "true" to enable audit logging, "false" to disable
: "${XION_AUDIT_LOG:=true}"

# Audit log file location
: "${XION_AUDIT_LOG_FILE:=$HOME/.xion-toolkit/audit.log}"

# Set to "true" to skip all confirmation prompts (useful for CI/CD)
: "${XION_SKIP_CONFIRM:=false}"

# ==============================================================================
# Color Codes
# ==============================================================================

if [[ -t 2 ]]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    BOLD='\033[1m'
    NC='\033[0m' # No Color
else
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    BOLD=''
    NC=''
fi

# ==============================================================================
# Logging Functions
# ==============================================================================

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1" >&2
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

log_audit() {
    local operation="$1"
    local details="$2"
    local timestamp
    
    if [[ "$XION_AUDIT_LOG" == "true" ]]; then
        timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
        
        # Ensure log directory exists
        mkdir -p "$(dirname "$XION_AUDIT_LOG_FILE")" 2>/dev/null || true
        
        # Append to audit log
        echo "[$timestamp] $operation: $details" >> "$XION_AUDIT_LOG_FILE" 2>/dev/null || true
    fi
}

# ==============================================================================
# Confirmation Functions
# ==============================================================================

# confirm_operation - Prompt user for confirmation
# Usage: confirm_operation "Operation description" [--show-details "key: value"]
# Returns: 0 if confirmed, 1 if rejected
confirm_operation() {
    local description="$1"
    local show_details=""
    
    # Parse optional arguments
    while [[ $# -gt 1 ]]; do
        case "$2" in
            --show-details)
                show_details="$3"
                shift 2
                ;;
            *)
                shift
                ;;
        esac
    done
    
    # Skip confirmation in non-interactive mode or when explicitly disabled
    if [[ "$XION_SKIP_CONFIRM" == "true" ]] || [[ ! -t 0 ]]; then
        log_info "Skipping confirmation (non-interactive mode)"
        return 0
    fi
    
    # Display confirmation prompt
    echo "" >&2
    echo -e "${BOLD}${YELLOW}⚠  Confirmation Required${NC}" >&2
    echo -e "${BOLD}${YELLOW}─────────────────────${NC}" >&2
    echo -e "  ${BOLD}Operation:${NC} $description" >&2
    
    if [[ -n "$show_details" ]]; then
        echo -e "  ${BOLD}Details:${NC}" >&2
        echo "$show_details" | while IFS= read -r line; do
            echo -e "    $line" >&2
        done
    fi
    
    echo "" >&2
    echo -e -n "${BOLD}Proceed? [y/N]${NC} " >&2
    
    # Read response
    local response
    read -r response
    
    # Check response
    case "$response" in
        [yY][eE][sS]|[yY])
            log_info "Operation confirmed"
            log_audit "CONFIRM" "$description"
            return 0
            ;;
        *)
            log_warn "Operation cancelled by user"
            log_audit "CANCEL" "$description"
            return 1
            ;;
    esac
}

# confirm_sensitive_operation - Enhanced confirmation for high-risk operations
# Usage: confirm_sensitive_operation "Operation description" "amount" "recipient"
confirm_sensitive_operation() {
    local description="$1"
    local amount="${2:-}"
    local recipient="${3:-}"
    
    local details=""
    [[ -n "$amount" ]] && details+="  Amount: $amount"$'\n'
    [[ -n "$recipient" ]] && details+="  Recipient: $recipient"
    
    if ! confirm_operation "$description" --show-details "$details"; then
        return 1
    fi
    
    # For very sensitive operations, require double confirmation
    echo "" >&2
    echo -e "${RED}${BOLD}⚠  WARNING: This action is irreversible!${NC}" >&2
    echo -e -n "${BOLD}Type 'yes' to confirm:${NC} " >&2
    
    local response
    read -r response
    
    if [[ "$response" == "yes" ]]; then
        log_info "Operation confirmed (double-confirm)"
        return 0
    else
        log_warn "Operation cancelled"
        return 1
    fi
}

# ==============================================================================
# Input Validation Functions
# ==============================================================================

# validate_address - Validate Xion address format
# Usage: validate_address "xion1..." || exit 1
validate_address() {
    local address="$1"
    
    if [[ -z "$address" ]]; then
        log_error "Address is required"
        return 1
    fi
    
    if [[ ! "$address" =~ ^xion1[a-z0-9]{38}$ ]]; then
        log_error "Invalid Xion address format: $address"
        log_error "Expected format: xion1... (39 characters after xion1)"
        return 1
    fi
    
    return 0
}

# validate_amount - Validate coin amount format
# Usage: validate_amount "1000000uxion" || exit 1
validate_amount() {
    local amount="$1"
    
    if [[ -z "$amount" ]]; then
        log_error "Amount is required"
        return 1
    fi
    
    if [[ ! "$amount" =~ ^[0-9]+uxion$ ]]; then
        log_error "Invalid amount format: $amount"
        log_error "Expected format: <number>uxion (e.g., 1000000uxion)"
        return 1
    fi
    
    return 0
}

# validate_url - Validate URL format
# Usage: validate_url "https://example.com" || exit 1
validate_url() {
    local url="$1"
    
    if [[ -z "$url" ]]; then
        return 0  # Empty URL is valid (optional)
    fi
    
    if [[ ! "$url" =~ ^https?:// ]]; then
        log_error "Invalid URL format: $url"
        log_error "URL must start with http:// or https://"
        return 1
    fi
    
    return 0
}

# ==============================================================================
# Utility Functions
# ==============================================================================

# get_confirmation_env - Check environment for confirmation settings
get_confirmation_env() {
    if [[ "$XION_SKIP_CONFIRM" == "true" ]]; then
        echo "skip"
    elif [[ -t 0 ]]; then
        echo "interactive"
    else
        echo "non-interactive"
    fi
}

# format_coin - Format coin amount for display
# Usage: format_coin "1000000uxion" -> "1 XION"
format_coin() {
    local amount="$1"
    
    if [[ "$amount" =~ ^([0-9]+)uxion$ ]]; then
        local num="${BASH_REMATCH[1]}"
        if (( num >= 1000000 )); then
            echo "$(( num / 1000000 )) XION"
        else
            echo "${num} uxion"
        fi
    else
        echo "$amount"
    fi
}

# ==============================================================================
# Export functions for use in other scripts
# ==============================================================================

# The following functions are available after sourcing this file:
# - log_info, log_warn, log_error
# - log_audit
# - confirm_operation, confirm_sensitive_operation
# - validate_address, validate_amount, validate_url
# - get_confirmation_env, format_coin
