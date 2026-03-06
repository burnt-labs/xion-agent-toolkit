#!/bin/bash

# Xion Agent Toolkit - Authentication and Treasury Test Script
# This script tests the OAuth2 authentication and basic treasury operations

set -e

echo "================================"
echo "Xion Agent Toolkit Test Script"
echo "================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ $1${NC}"
    else
        echo -e "${RED}✗ $1${NC}"
    fi
}

# 1. Check if CLI binary exists
echo "1. Checking CLI binary..."
if [ -f "./target/release/xion" ]; then
    echo -e "${GREEN}✓ CLI binary found${NC}"
else
    echo -e "${RED}✗ CLI binary not found. Please run: cargo build --release${NC}"
    exit 1
fi
echo ""

# 2. Check authentication status
echo "2. Checking authentication status..."
AUTH_STATUS=$(./target/release/xion auth status --output json 2>&1)
if echo "$AUTH_STATUS" | grep -q '"authenticated": true'; then
    echo -e "${GREEN}✓ User is authenticated${NC}"
    echo ""
    echo "Authentication Details:"
    echo "$AUTH_STATUS" | jq '.'
    
    # Check if xion_address is present
    XION_ADDRESS=$(echo "$AUTH_STATUS" | jq -r '.xion_address // empty')
    if [ -n "$XION_ADDRESS" ]; then
        echo -e "${GREEN}✓ xion_address found: $XION_ADDRESS${NC}"
    else
        echo -e "${RED}✗ xion_address is missing or null${NC}"
        echo -e "${YELLOW}Please login again with: ./target/release/xion auth login${NC}"
        exit 1
    fi
else
    echo -e "${RED}✗ User is not authenticated${NC}"
    echo -e "${YELLOW}Please login first with: ./target/release/xion auth login --network testnet${NC}"
    exit 1
fi
echo ""

# 3. List treasuries
echo "3. Listing treasuries..."
TREASURY_LIST=$(./target/release/xion treasury list --output json 2>&1)
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Treasury list retrieved successfully${NC}"
    echo ""
    echo "Treasuries:"
    echo "$TREASURY_LIST" | jq '.'
    
    # Count treasuries
    TREASURY_COUNT=$(echo "$TREASURY_LIST" | jq '.treasuries | length')
    echo ""
    echo -e "${GREEN}Found $TREASURY_COUNT treasury(ies)${NC}"
else
    echo -e "${RED}✗ Failed to list treasuries${NC}"
    echo "$TREASURY_LIST"
fi
echo ""

# 4. Test query (if treasuries exist)
if [ "$TREASURY_COUNT" -gt 0 ]; then
    echo "4. Querying first treasury..."
    FIRST_TREASURY=$(echo "$TREASURY_LIST" | jq -r '.treasuries[0].address')
    
    TREASURY_QUERY=$(./target/release/xion treasury query "$FIRST_TREASURY" --output json 2>&1)
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Treasury query successful${NC}"
        echo ""
        echo "Treasury Details:"
        echo "$TREASURY_QUERY" | jq '.'
    else
        echo -e "${RED}✗ Treasury query failed${NC}"
        echo "$TREASURY_QUERY"
    fi
    echo ""
else
    echo -e "${YELLOW}No treasuries found. Skipping query test.${NC}"
    echo ""
fi

# Summary
echo "================================"
echo "Test Summary"
echo "================================"
echo -e "${GREEN}✓ CLI binary: OK${NC}"
echo -e "${GREEN}✓ Authentication: OK${NC}"
echo -e "${GREEN}✓ xion_address: OK${NC}"
echo -e "${GREEN}✓ Treasury list: OK${NC}"
if [ "$TREASURY_COUNT" -gt 0 ]; then
    echo -e "${GREEN}✓ Treasury query: OK${NC}"
fi
echo ""
echo -e "${GREEN}All basic tests passed!${NC}"
echo ""
echo "Next steps:"
echo "  - Test fund: ./target/release/xion treasury fund <address> <amount>"
echo "  - Test withdraw: ./target/release/xion treasury withdraw <address> <amount>"
