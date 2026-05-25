#!/bin/bash
#
# tests/skills/test_skill_frontmatter.sh - SKILL.md frontmatter validation
#
# Usage:
#   ./test_skill_frontmatter.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=./lib.sh
source "$SCRIPT_DIR/lib.sh"

VALIDATE="$SCRIPT_DIR/../../scripts/validate-skill-frontmatter.sh"

test_skill_frontmatter_all_packages() {
    if [[ ! -f "$VALIDATE" ]]; then
        log_error "validate script not found: $VALIDATE"
        return 1
    fi
    if [[ ! -x "$VALIDATE" ]]; then
        chmod +x "$VALIDATE"
    fi

    if ! command -v cargo >/dev/null 2>&1; then
        log_error "cargo is required for frontmatter validation"
        return 1
    fi

    local output
    if ! output=$("$VALIDATE" 2>&1); then
        log_error "validate-skill-frontmatter.sh failed"
        echo "$output" >&2
        return 1
    fi

    if ! echo "$output" | grep -q "All skill frontmatter checks passed"; then
        log_error "unexpected validator output"
        echo "$output" >&2
        return 1
    fi

    return 0
}

main() {
    echo "=== Skill Frontmatter Tests ==="
    run_test "test_skill_frontmatter_all_packages" test_skill_frontmatter_all_packages
    test_exit
}

main "$@"
