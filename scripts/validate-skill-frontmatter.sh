#!/usr/bin/env bash
# Validate SKILL.md frontmatter for Codex / Agent Skills loaders.
# Delegates to the Rust binary (serde_yaml) — same toolchain as the rest of this repo.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SKILLS_DIR="${1:-$REPO_ROOT/skills}"

if ! command -v cargo >/dev/null 2>&1; then
  echo "ERROR: cargo (Rust toolchain) is required. Install Rust: https://rustup.rs/" >&2
  exit 2
fi

cd "$REPO_ROOT"
exec cargo run --quiet --bin validate-skill-frontmatter -- "$SKILLS_DIR"
