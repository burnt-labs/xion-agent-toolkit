#!/bin/bash
#
# Xion Agent Toolkit - Skills Test Runner
# Discovers and runs all skill test suites
#
# Usage:
#   ./run_all.sh              # Run all tests
#   MOCK_ENABLED=true ./run_all.sh  # Run with mock responses
#   ./run_all.sh oauth2       # Run tests for specific skill
#
# Environment:
#   MOCK_ENABLED    - Set to 'true' to use mock responses (default: false)
#   MOCK_DIR        - Directory containing mock JSON files (default: ./mocks)
#   CI              - Set by GitHub Actions for CI mode
#

set -euo pipefail

# =============================================================================
# Configuration
# =============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LIB_PATH="$SCRIPT_DIR/lib.sh"
TEST_DIR="$SCRIPT_DIR"
MOCK_DIR="${MOCK_DIR:-$SCRIPT_DIR/mocks}"
MOCK_ENABLED="${MOCK_ENABLED:-false}"
CI="${CI:-false}"

# Colors (disabled in CI mode)
if [[ "$CI" != "true" ]]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    NC='\033[0m'
else
    GREEN=''
    RED=''
    YELLOW=''
    BLUE=''
    CYAN=''
    BOLD=''
    NC=''
fi

# =============================================================================
# Load Test Framework
# =============================================================================

if [[ ! -f "$LIB_PATH" ]]; then
    echo "[ERROR] Test framework library not found: $LIB_PATH" >&2
    exit 1
fi

# shellcheck source=./lib.sh
source "$LIB_PATH"

# =============================================================================
# Helper Functions
# =============================================================================

print_header() {
    echo ""
    echo -e "${BOLD}========================================${NC}"
    echo -e "${BOLD}Xion Agent Toolkit - Skills Tests${NC}"
    echo -e "${BOLD}========================================${NC}"
    echo ""
    echo "Mode: $([ "$MOCK_ENABLED" == "true" ] && echo "Mock" || echo "E2E")"
    echo "Mock Directory: $MOCK_DIR"
    echo "Test Directory: $TEST_DIR"
    echo ""
}

find_test_files() {
    local skill_filter="${1:-}"
    
    if [[ -n "$skill_filter" ]]; then
        # Run specific skill tests
        local pattern="test_${skill_filter}.sh"
        find "$TEST_DIR" -name "$pattern" -type f 2>/dev/null | sort
    else
        # Find all test files
        find "$TEST_DIR" -name "test_*.sh" -type f 2>/dev/null | sort
    fi
}

validate_test_file() {
    local file="$1"
    
    # Check file is executable
    if [[ ! -x "$file" ]]; then
        log_warn "Test file not executable: $file"
        chmod +x "$file" 2>/dev/null || true
    fi
    
    # Check for required functions
    if ! grep -q "^test_" "$file" 2>/dev/null; then
        log_warn "No test functions found in: $file"
        return 1
    fi
    
    return 0
}

run_test_file() {
    local file="$1"
    local filename
    filename=$(basename "$file")
    
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Running: $filename${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    # Run the test file
    local result=0
    if bash "$file"; then
        log_success "Test file passed: $filename"
    else
        log_error "Test file failed: $filename"
        result=1
    fi
    
    return $result
}

# =============================================================================
# Pre-flight Checks
# =============================================================================

preflight_checks() {
    local errors=0
    
    # Check jq
    if ! command -v jq >/dev/null 2>&1; then
        log_error "jq is required but not installed"
        ((errors++)) || true
    fi
    
    # Check bash version
    local bash_major
    bash_major=$(bash --version | head -1 | grep -oE '[0-9]+' | head -1)
    if [[ "$bash_major" -lt 4 ]]; then
        log_warn "Bash version 4+ recommended (found: $bash_major)"
    fi
    
    # In mock mode, check mock directory
    if [[ "$MOCK_ENABLED" == "true" ]]; then
        if [[ ! -d "$MOCK_DIR" ]]; then
            log_error "Mock directory not found: $MOCK_DIR"
            ((errors++)) || true
        fi
    fi
    
    # In E2E mode, check xion-toolkit
    if [[ "$MOCK_ENABLED" != "true" ]]; then
        if ! command -v xion-toolkit >/dev/null 2>&1; then
            log_warn "xion-toolkit not found in PATH - some tests may fail"
            log_info "Install with: ./skills/xion-toolkit-init/scripts/install.sh"
        fi
    fi
    
    return $errors
}

# =============================================================================
# Main
# =============================================================================

main() {
    local skill_filter="${1:-}"
    
    print_header
    
    # Run pre-flight checks
    log_info "Running pre-flight checks..."
    if ! preflight_checks; then
        log_error "Pre-flight checks failed"
        exit 1
    fi
    log_success "Pre-flight checks passed"
    
    # Find test files
    log_info "Discovering test files..."
    local test_files
    test_files=$(find_test_files "$skill_filter")
    
    if [[ -z "$test_files" ]]; then
        log_warn "No test files found"
        if [[ -n "$skill_filter" ]]; then
            log_info "Looking for: test_${skill_filter}.sh"
        fi
        exit 0
    fi
    
    local file_count
    file_count=$(echo "$test_files" | wc -l | tr -d ' ')
    log_info "Found $file_count test file(s)"
    
    # Run tests
    local failed=0
    local passed=0
    local skipped=0
    
    while IFS= read -r file; do
        if [[ ! -f "$file" ]]; then
            continue
        fi
        
        if ! validate_test_file "$file"; then
            ((skipped++)) || true
            continue
        fi
        
        if run_test_file "$file"; then
            ((passed++)) || true
        else
            ((failed++)) || true
        fi
    done <<< "$test_files"
    
    # Print summary
    echo ""
    echo -e "${BOLD}========================================${NC}"
    echo -e "${BOLD}Final Summary${NC}"
    echo -e "${BOLD}========================================${NC}"
    echo -e "Test Files: $file_count"
    echo -e "Passed:     ${GREEN}$passed${NC}"
    echo -e "Failed:     ${RED}$failed${NC}"
    echo -e "Skipped:    ${CYAN}$skipped${NC}"
    echo -e "${BOLD}========================================${NC}"
    
    if [[ $failed -gt 0 ]]; then
        echo -e "${RED}Some tests failed${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
}

# Run main with all arguments
main "$@"