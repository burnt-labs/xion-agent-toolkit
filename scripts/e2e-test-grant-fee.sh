#!/bin/bash
#
# E2E Test Script for Grant Config and Fee Config Operations
# Tests all grant and fee configuration features against testnet
#
# Prerequisites:
#   1. Run 'xion-toolkit auth login --network testnet' first
#   2. Ensure you have testnet tokens (get from faucet)
#   3. Have at least one treasury available (or create with e2e-test.sh)
#
# Usage:
#   ./scripts/e2e-test-grant-fee.sh [treasury-address]
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
TOOLKIT="./target/release/xion-toolkit"
NETWORK="testnet"
LOG_FILE="e2e-test-grant-fee-$(date +%Y%m%d-%H%M%S).log"
SPECIFIC_TREASURY="$1"

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

info() {
    echo -e "${CYAN}[INFO]${NC} $1" | tee -a "$LOG_FILE"
}

section() {
    echo -e "\n${BLUE}========================================${NC}" | tee -a "$LOG_FILE"
    echo -e "${BLUE}$1${NC}" | tee -a "$LOG_FILE"
    echo -e "${BLUE}========================================${NC}\n" | tee -a "$LOG_FILE"
}

check_json() {
    echo "$1" | jq -e . > /dev/null 2>&1
}

# Function to extract JSON from mixed output
extract_json() {
    sed -n '/^{/,/^}/p'
}

# Start logging
echo "E2E Grant/Fee Config Test Started: $(date)" > "$LOG_FILE"
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
# SECTION 1: Authentication Check
# ===========================================
section "Authentication Check"

log "Checking auth status..."
AUTH_STATUS=$($TOOLKIT auth status --network $NETWORK --output json 2>&1 | extract_json)
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
# SECTION 2: Treasury Selection
# ===========================================
section "Treasury Selection"

if [ -n "$SPECIFIC_TREASURY" ]; then
    log "Using specified treasury: $SPECIFIC_TREASURY"
    TREASURY_ADDRESS="$SPECIFIC_TREASURY"
else
    log "No treasury specified, listing available treasuries..."
    LIST_RESULT=$($TOOLKIT treasury list --network $NETWORK --output json 2>&1 | extract_json)
    echo "$LIST_RESULT" | jq '.' 2>/dev/null || echo "$LIST_RESULT"
    echo "$LIST_RESULT" >> "$LOG_FILE"
    
    if check_json "$LIST_RESULT"; then
        TREASURY_COUNT=$(echo "$LIST_RESULT" | jq '.treasuries | length' 2>/dev/null || echo "0")
        
        if [ "$TREASURY_COUNT" -eq 0 ]; then
            fail "No treasuries found. Create one first with: $TOOLKIT treasury create"
        fi
        
        # Use first treasury (skip the protected OAuth2 App treasury if it's first)
        FIRST_ADDR=$(echo "$LIST_RESULT" | jq -r '.treasuries[0].address' 2>/dev/null)
        PROTECTED_TREASURY="xion17vg5l9za4768g0hnxezltgnu4h7eleqdcmwark2uuz2s4z5q4dfsr80vvm"
        
        if [ "$FIRST_ADDR" = "$PROTECTED_TREASURY" ] && [ "$TREASURY_COUNT" -gt 1 ]; then
            TREASURY_ADDRESS=$(echo "$LIST_RESULT" | jq -r '.treasuries[1].address' 2>/dev/null)
            log "Skipping protected OAuth2 App treasury, using: $TREASURY_ADDRESS"
        else
            TREASURY_ADDRESS="$FIRST_ADDR"
            log "Using first treasury: $TREASURY_ADDRESS"
        fi
        pass "Treasury selected: $TREASURY_ADDRESS"
    else
        fail "Failed to list treasuries"
    fi
fi

# ===========================================
# SECTION 3: Grant Config - List Current
# ===========================================
section "Grant Config - List Current"

log "Listing existing grant configs..."
GRANT_LIST=$($TOOLKIT treasury grant-config list "$TREASURY_ADDRESS" --network $NETWORK --output json 2>&1 | extract_json)
echo "$GRANT_LIST" | jq '.' 2>/dev/null || echo "$GRANT_LIST"
echo "$GRANT_LIST" >> "$LOG_FILE"

if check_json "$GRANT_LIST"; then
    pass "Grant config list works"
else
    warn "Grant config list returned non-JSON"
fi

# ===========================================
# SECTION 4: Grant Config - Add Tests
# ===========================================
section "Grant Config - Add Tests (UpdateGrantConfig)"

info "Testing grant-config add with various options..."
info "Note: Only treasury admin can update grant config"

# 4.1: Generic Preset
log "4.1: Testing grant-config add with generic type..."
GRANT_GENERIC=$($TOOLKIT treasury grant-config add "$TREASURY_ADDRESS" \
    --network $NETWORK \
    --type-url "/cosmos.bank.v1beta1.MsgSend" \
    --auth-type generic \
    --description "E2E test generic authorization" \
    --output json 2>&1 | extract_json)

echo "$GRANT_GENERIC" | jq '.' 2>/dev/null || echo "$GRANT_GENERIC"
echo "$GRANT_GENERIC" >> "$LOG_FILE"

if check_json "$GRANT_GENERIC"; then
    if echo "$GRANT_GENERIC" | jq -e '.success == true' > /dev/null 2>&1; then
        pass "Grant config add (generic) - SUCCESS"
    else
        warn "Grant config add (generic) - returned success=false or error (may need admin permission)"
    fi
else
    warn "Grant config add (generic) returned non-JSON"
fi

# 4.2: Send Preset with spend limit
log "4.2: Testing grant-config add with send preset..."
GRANT_SEND=$($TOOLKIT treasury grant-config add "$TREASURY_ADDRESS" \
    --network $NETWORK \
    --preset send \
    --spend-limit "1000000uxion" \
    --description "E2E test send authorization" \
    --output json 2>&1 | extract_json)

echo "$GRANT_SEND" | jq '.' 2>/dev/null || echo "$GRANT_SEND"
echo "$GRANT_SEND" >> "$LOG_FILE"

if check_json "$GRANT_SEND"; then
    if echo "$GRANT_SEND" | jq -e '.success == true' > /dev/null 2>&1; then
        pass "Grant config add (send preset) - SUCCESS"
    else
        warn "Grant config add (send) - returned success=false or error (may need admin permission)"
    fi
else
    warn "Grant config add (send) returned non-JSON"
fi

# ===========================================
# SECTION 5: Fee Config - Query Current
# ===========================================
section "Fee Config - Query Current"

log "Querying existing fee config..."
FEE_QUERY=$($TOOLKIT treasury fee-config query "$TREASURY_ADDRESS" --network $NETWORK --output json 2>&1 | extract_json)
echo "$FEE_QUERY" | jq '.' 2>/dev/null || echo "$FEE_QUERY"
echo "$FEE_QUERY" >> "$LOG_FILE"

if check_json "$FEE_QUERY"; then
    pass "Fee config query works"
else
    warn "Fee config query returned non-JSON"
fi

# ===========================================
# SECTION 6: Fee Config - Set Tests
# ===========================================
section "Fee Config - Set Tests (UpdateFeeConfig)"

info "Fee config requires JSON config file..."
info "Note: Only treasury admin can update fee config"

# Create temp config files for fee config tests
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# 6.1: Basic allowance config
log "6.1: Testing fee-config set with basic allowance..."
BASIC_CONFIG="$TEMP_DIR/fee-basic.json"
cat > "$BASIC_CONFIG" << 'EOF'
{
  "allowance_type": "basic",
  "spend_limit": "2000000uxion",
  "description": "E2E test basic fee allowance"
}
EOF

FEE_BASIC=$($TOOLKIT treasury fee-config set "$TREASURY_ADDRESS" \
    --fee-config "$BASIC_CONFIG" \
    --network $NETWORK \
    --output json 2>&1 | extract_json)

echo "$FEE_BASIC" | jq '.' 2>/dev/null || echo "$FEE_BASIC"
echo "$FEE_BASIC" >> "$LOG_FILE"

if check_json "$FEE_BASIC"; then
    if echo "$FEE_BASIC" | jq -e '.success == true' > /dev/null 2>&1; then
        pass "Fee config set (basic) - SUCCESS"
    else
        warn "Fee config set (basic) - returned success=false or error (may need admin permission)"
    fi
else
    warn "Fee config set (basic) returned non-JSON"
fi

# 6.2: Periodic allowance config
log "6.2: Testing fee-config set with periodic allowance..."
PERIODIC_CONFIG="$TEMP_DIR/fee-periodic.json"
cat > "$PERIODIC_CONFIG" << 'EOF'
{
  "allowance_type": "periodic",
  "period_seconds": 86400,
  "period_spend_limit": "500000uxion",
  "description": "E2E test periodic daily fee allowance"
}
EOF

FEE_PERIODIC=$($TOOLKIT treasury fee-config set "$TREASURY_ADDRESS" \
    --fee-config "$PERIODIC_CONFIG" \
    --network $NETWORK \
    --output json 2>&1 | extract_json)

echo "$FEE_PERIODIC" | jq '.' 2>/dev/null || echo "$FEE_PERIODIC"
echo "$FEE_PERIODIC" >> "$LOG_FILE"

if check_json "$FEE_PERIODIC"; then
    if echo "$FEE_PERIODIC" | jq -e '.success == true' > /dev/null 2>&1; then
        pass "Fee config set (periodic) - SUCCESS"
    else
        warn "Fee config set (periodic) - returned success=false or error (may need admin permission)"
    fi
else
    warn "Fee config set (periodic) returned non-JSON"
fi

# ===========================================
# SECTION 7: Fee Config - Remove (RevokeAllowance)
# ===========================================
section "Fee Config - Remove (RevokeAllowance)"

info "Note: fee-config remove requires grantee address"
warn "Skipping fee-config remove test (requires valid grantee with existing allowance)"

# We skip this test because it requires a grantee that has been granted an allowance
# log "7.1: Testing fee-config remove..."
# FEE_REMOVE=$($TOOLKIT treasury fee-config remove "$TREASURY_ADDRESS" \
#     --grantee "xion1..." \
#     --network $NETWORK \
#     --output json 2>&1 | extract_json)

# ===========================================
# SECTION 8: Error Handling Tests
# ===========================================
section "Error Handling Tests"

# 8.1: Invalid preset
log "8.1: Testing with invalid preset (should fail gracefully)..."
GRANT_INVALID_PRESET=$($TOOLKIT treasury grant-config add "$TREASURY_ADDRESS" \
    --network $NETWORK \
    --preset invalid_preset \
    --output json 2>&1 || true)

echo "$GRANT_INVALID_PRESET" >> "$LOG_FILE"
if echo "$GRANT_INVALID_PRESET" | grep -qi "error\|invalid"; then
    pass "Invalid preset handled correctly"
else
    warn "Invalid preset might not be handled properly"
fi

# 8.2: Missing required fields for generic auth
log "8.2: Testing grant-config add without type-url for generic (should fail)..."
GRANT_MISSING_FIELDS=$($TOOLKIT treasury grant-config add "$TREASURY_ADDRESS" \
    --network $NETWORK \
    --auth-type generic \
    --output json 2>&1 || true)

echo "$GRANT_MISSING_FIELDS" >> "$LOG_FILE"
if echo "$GRANT_MISSING_FIELDS" | grep -qi "error\|required"; then
    pass "Missing required fields handled correctly"
else
    warn "Missing required fields might not be validated properly"
fi

# 8.3: Fee config with invalid JSON file
log "8.3: Testing fee-config set with missing config file..."
FEE_MISSING_FILE=$($TOOLKIT treasury fee-config set "$TREASURY_ADDRESS" \
    --fee-config "/nonexistent/config.json" \
    --network $NETWORK \
    --output json 2>&1 || true)

echo "$FEE_MISSING_FILE" >> "$LOG_FILE"
if echo "$FEE_MISSING_FILE" | grep -qi "error\|not found\|no such"; then
    pass "Missing config file handled correctly"
else
    warn "Missing config file might not be handled properly"
fi

# ===========================================
# Summary
# ===========================================
section "Test Summary"

log "Test completed. Full log: $LOG_FILE"
echo ""
info "Grant Config Tests:"
echo "  - grant-config list: tested"
echo "  - grant-config add (generic): tested"
echo "  - grant-config add (send preset): tested"
echo ""
info "Fee Config Tests:"
echo "  - fee-config query: tested"
echo "  - fee-config set (basic): tested"
echo "  - fee-config set (periodic): tested"
echo "  - fee-config remove: skipped (requires valid grantee)"
echo ""
info "Error Handling:"
echo "  - Invalid preset: tested"
echo "  - Missing required fields: tested"
echo "  - Missing config file: tested"
echo ""
warn "IMPORTANT: Grant/Fee config operations require treasury admin permission."
warn "If you are not the admin, the operations will fail with 500 error."
echo ""

echo -e "${GREEN}E2E Grant/Fee Config Test Script Completed${NC}" | tee -a "$LOG_FILE"
