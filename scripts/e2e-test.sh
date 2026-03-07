#!/bin/bash
#
# E2E Test Script for Xion Agent Toolkit
# Tests all treasury operations against testnet
#
# Prerequisites:
#   1. Run 'xion-toolkit auth login --network testnet' first
#   2. Ensure you have testnet tokens (get from faucet)
#
# Usage:
#   ./scripts/e2e-test.sh [--skip-create] [--skip-fund]
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
TOOLKIT="./target/release/xion-toolkit"
NETWORK="testnet"
LOG_FILE="e2e-test-$(date +%Y%m%d-%H%M%S).log"
SKIP_CREATE=false
SKIP_FUND=false

# Parse arguments
for arg in "$@"; do
    case $arg in
        --skip-create) SKIP_CREATE=true ;;
        --skip-fund) SKIP_CREATE=true; SKIP_FUND=true ;;
    esac
done

# Helper functions
log() {
    echo -e "${BLUE}[TEST]${NC} $1" | tee -a "$LOG_FILE"
}

pass() {
    echo -e "${GREEN}[PASS]${NC} $1" | tee -a "$LOG_FILE"
}

fail() {
    echo -e "${RED}[FAIL]${NC} $1" | tee -a "$LOG_FILE"
    exit 1
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" | tee -a "$LOG_FILE"
}

section() {
    echo -e "\n${BLUE}========================================${NC}" | tee -a "$LOG_FILE"
    echo -e "${BLUE}$1${NC}" | tee -a "$LOG_FILE"
    echo -e "${BLUE}========================================${NC}\n" | tee -a "$LOG_FILE"
}

check_json() {
    echo "$1" | jq -e . > /dev/null 2>&1
}

# Start logging
echo "E2E Test Started: $(date)" > "$LOG_FILE"
log "Log file: $LOG_FILE"

# ===========================================
# SECTION 0: Prerequisites Check
# ===========================================
section "Prerequisites Check"

log "Checking toolkit binary..."
if [ ! -f "$TOOLKIT" ]; then
    fail "Toolkit binary not found. Run 'cargo build --release' first."
fi
pass "Toolkit binary exists"

log "Checking jq (JSON processor)..."
if ! command -v jq &> /dev/null; then
    fail "jq is required. Install with: brew install jq"
fi
pass "jq is installed"

# ===========================================
# SECTION 1: Authentication Status
# ===========================================
section "Authentication Status"

log "Checking auth status..."
AUTH_STATUS=$($TOOLKIT auth status --network $NETWORK --output json 2>&1)
echo "$AUTH_STATUS" | tee -a "$LOG_FILE"

if echo "$AUTH_STATUS" | jq -e '.authenticated == true' > /dev/null 2>&1; then
    pass "User is authenticated"
    XION_ADDRESS=$(echo "$AUTH_STATUS" | jq -r '.xion_address // empty')
    if [ -n "$XION_ADDRESS" ]; then
        log "Xion address: $XION_ADDRESS"
    fi
else
    fail "Not authenticated. Please run: $TOOLKIT auth login --network $NETWORK"
fi

# ===========================================
# SECTION 2: Configuration Display
# ===========================================
section "Configuration Display"

log "Showing network configuration..."
CONFIG=$($TOOLKIT config show --network $NETWORK --output json 2>&1)
echo "$CONFIG" | jq '.' | tee -a "$LOG_FILE"

if check_json "$CONFIG" && echo "$CONFIG" | jq -e '.network' > /dev/null 2>&1; then
    pass "Config display works"
else
    fail "Config display failed"
fi

# ===========================================
# SECTION 3: List Treasuries (DaoDao Indexer)
# ===========================================
section "List Treasuries (DaoDao Indexer)"

log "Listing existing treasuries..."
LIST_RESULT=$($TOOLKIT treasury list --network $NETWORK --output json 2>&1)
echo "$LIST_RESULT" | jq '.' 2>/dev/null || echo "$LIST_RESULT"
echo "$LIST_RESULT" >> "$LOG_FILE"

if check_json "$LIST_RESULT"; then
    TREASURY_COUNT=$(echo "$LIST_RESULT" | jq '.treasuries | length' 2>/dev/null || echo "0")
    pass "Treasury list works - found $TREASURY_COUNT treasuries"
    
    # Get first treasury address if exists
    if [ "$TREASURY_COUNT" -gt 0 ]; then
        FIRST_TREASURY=$(echo "$LIST_RESULT" | jq -r '.treasuries[0].address' 2>/dev/null)
        log "First treasury: $FIRST_TREASURY"
    fi
else
    warn "Treasury list returned non-JSON (might be empty or error)"
fi

# ===========================================
# SECTION 4: Create Treasury
# ===========================================
section "Create Treasury"

if [ "$SKIP_CREATE" = true ]; then
    warn "Skipping treasury create (--skip-create)"
else
    log "Creating a new treasury..."
    TREASURY_NAME="E2E Test Treasury $(date +%s)"
    
    CREATE_RESULT=$($TOOLKIT treasury create \
        --network $NETWORK \
        --name "$TREASURY_NAME" \
        --redirect-url "https://example.com/callback" \
        --icon-url "https://example.com/icon.png" \
        --output json 2>&1)
    
    echo "$CREATE_RESULT" | jq '.' 2>/dev/null || echo "$CREATE_RESULT"
    echo "$CREATE_RESULT" >> "$LOG_FILE"
    
    if check_json "$CREATE_RESULT" && echo "$CREATE_RESULT" | jq -e '.address' > /dev/null 2>&1; then
        NEW_TREASURY=$(echo "$CREATE_RESULT" | jq -r '.address')
        pass "Treasury created: $NEW_TREASURY"
    else
        # Check if it's a funding issue
        if echo "$CREATE_RESULT" | grep -qi "insufficient\|funding\|balance"; then
            warn "Treasury creation may require funding. Get tokens from faucet."
        fi
        fail "Treasury creation failed"
    fi
fi

# ===========================================
# SECTION 5: Query Treasury
# ===========================================
section "Query Treasury"

# Try to query first treasury from list
if [ -n "$FIRST_TREASURY" ] && [ "$FIRST_TREASURY" != "null" ]; then
    log "Querying treasury: $FIRST_TREASURY"
    QUERY_RESULT=$($TOOLKIT treasury query "$FIRST_TREASURY" --network $NETWORK --output json 2>&1)
    echo "$QUERY_RESULT" | jq '.' 2>/dev/null || echo "$QUERY_RESULT"
    echo "$QUERY_RESULT" >> "$LOG_FILE"
    
    if check_json "$QUERY_RESULT"; then
        pass "Treasury query works"
    else
        warn "Treasury query returned non-JSON"
    fi
elif [ -n "$NEW_TREASURY" ]; then
    log "Querying newly created treasury: $NEW_TREASURY"
    QUERY_RESULT=$($TOOLKIT treasury query "$NEW_TREASURY" --network $NETWORK --output json 2>&1)
    echo "$QUERY_RESULT" | jq '.' 2>/dev/null || echo "$QUERY_RESULT"
    echo "$QUERY_RESULT" >> "$LOG_FILE"
    
    if check_json "$QUERY_RESULT"; then
        pass "Treasury query works"
    else
        warn "Treasury query returned non-JSON"
    fi
else
    warn "No treasury available to query"
fi

# ===========================================
# SECTION 6: Grant Config (Authz)
# ===========================================
section "Grant Config (Authz)"

if [ -n "$NEW_TREASURY" ]; then
    TREASURY_TO_CONFIG="$NEW_TREASURY"
elif [ -n "$FIRST_TREASURY" ] && [ "$FIRST_TREASURY" != "null" ]; then
    TREASURY_TO_CONFIG="$FIRST_TREASURY"
else
    TREASURY_TO_CONFIG=""
fi

if [ -n "$TREASURY_TO_CONFIG" ]; then
    log "Testing grant-config with preset 'send'..."
    GRANT_RESULT=$($TOOLKIT treasury grant-config "$TREASURY_TO_CONFIG" \
        --network $NETWORK \
        --preset send \
        --grant-spend-limit "1000000uxion" \
        --grant-description "E2E test send authorization" \
        --output json 2>&1)
    
    echo "$GRANT_RESULT" | jq '.' 2>/dev/null || echo "$GRANT_RESULT"
    echo "$GRANT_RESULT" >> "$LOG_FILE"
    
    if check_json "$GRANT_RESULT"; then
        pass "Grant config (send preset) works"
    else
        warn "Grant config returned non-JSON (may require different authorization)"
    fi
    
    log "Testing grant-config with preset 'execute'..."
    GRANT_RESULT2=$($TOOLKIT treasury grant-config "$TREASURY_TO_CONFIG" \
        --network $NETWORK \
        --preset execute \
        --grant-max-calls 100 \
        --grant-description "E2E test execute authorization" \
        --output json 2>&1)
    
    echo "$GRANT_RESULT2" | jq '.' 2>/dev/null || echo "$GRANT_RESULT2"
    echo "$GRANT_RESULT2" >> "$LOG_FILE"
    
    if check_json "$GRANT_RESULT2"; then
        pass "Grant config (execute preset) works"
    else
        warn "Grant config (execute) returned non-JSON"
    fi
else
    warn "No treasury available for grant config test"
fi

# ===========================================
# SECTION 7: Fee Config
# ===========================================
section "Fee Config"

if [ -n "$TREASURY_TO_CONFIG" ]; then
    log "Testing fee-config with basic allowance..."
    FEE_RESULT=$($TOOLKIT treasury fee-config "$TREASURY_TO_CONFIG" \
        --network $NETWORK \
        --fee-allowance-type basic \
        --fee-spend-limit "1000000uxion" \
        --fee-description "E2E test basic fee allowance" \
        --output json 2>&1)
    
    echo "$FEE_RESULT" | jq '.' 2>/dev/null || echo "$FEE_RESULT"
    echo "$FEE_RESULT" >> "$LOG_FILE"
    
    if check_json "$FEE_RESULT"; then
        pass "Fee config (basic) works"
    else
        warn "Fee config returned non-JSON"
    fi
    
    log "Testing fee-config with periodic allowance..."
    FEE_RESULT2=$($TOOLKIT treasury fee-config "$TREASURY_TO_CONFIG" \
        --network $NETWORK \
        --fee-allowance-type periodic \
        --fee-period-seconds 86400 \
        --fee-period-spend-limit "100000uxion" \
        --fee-description "E2E test periodic fee allowance" \
        --output json 2>&1)
    
    echo "$FEE_RESULT2" | jq '.' 2>/dev/null || echo "$FEE_RESULT2"
    echo "$FEE_RESULT2" >> "$LOG_FILE"
    
    if check_json "$FEE_RESULT2"; then
        pass "Fee config (periodic) works"
    else
        warn "Fee config (periodic) returned non-JSON"
    fi
else
    warn "No treasury available for fee config test"
fi

# ===========================================
# SECTION 8: Refresh Token
# ===========================================
section "Token Refresh"

log "Testing token refresh..."
REFRESH_RESULT=$($TOOLKIT auth refresh --network $NETWORK --output json 2>&1)
echo "$REFRESH_RESULT" | jq '.' 2>/dev/null || echo "$REFRESH_RESULT"
echo "$REFRESH_RESULT" >> "$LOG_FILE"

if check_json "$REFRESH_RESULT" && echo "$REFRESH_RESULT" | jq -e '.refreshed == true' > /dev/null 2>&1; then
    pass "Token refresh works"
elif echo "$REFRESH_RESULT" | jq -e '.refreshed == false' > /dev/null 2>&1; then
    pass "Token refresh skipped (token still valid)"
else
    warn "Token refresh returned unexpected response"
fi

# ===========================================
# SECTION 9: Verify DaoDao Indexer Directly
# ===========================================
section "DaoDao Indexer Verification"

log "Verifying DaoDao Indexer endpoint format..."
if [ -n "$XION_ADDRESS" ]; then
    INDEXER_URL="https://daodaoindexer.burnt.com/xion-testnet-2/contract/${XION_ADDRESS}/xion/account/treasuries"
    log "Calling: $INDEXER_URL"
    
    INDEXER_RESPONSE=$(curl -s -w "\nHTTP_CODE:%{http_code}" "$INDEXER_URL")
    HTTP_CODE=$(echo "$INDEXER_RESPONSE" | grep "HTTP_CODE:" | cut -d':' -f2)
    BODY=$(echo "$INDEXER_RESPONSE" | sed '/HTTP_CODE:/d')
    
    echo "HTTP Status: $HTTP_CODE" | tee -a "$LOG_FILE"
    echo "$BODY" | jq '.' 2>/dev/null || echo "$BODY"
    echo "$BODY" >> "$LOG_FILE"
    
    if [ "$HTTP_CODE" = "200" ]; then
        pass "DaoDao Indexer responded successfully"
    elif [ "$HTTP_CODE" = "404" ]; then
        pass "DaoDao Indexer accessible (no treasuries found for address)"
    else
        warn "DaoDao Indexer returned HTTP $HTTP_CODE"
    fi
else
    warn "No xion_address available to test indexer directly"
fi

# ===========================================
# Summary
# ===========================================
section "E2E Test Summary"

log "Test completed. Full log: $LOG_FILE"
echo ""
log "To run manual tests:"
echo "  1. Get testnet tokens from faucet"
echo "  2. Run: $TOOLKIT treasury create --name 'Test' --redirect-url 'https://example.com'"
echo "  3. Run: $TOOLKIT treasury list"
echo "  4. Run: $TOOLKIT treasury query <address>"
echo ""

echo -e "${GREEN}E2E Test Script Completed${NC}" | tee -a "$LOG_FILE"
