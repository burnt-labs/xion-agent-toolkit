#!/usr/bin/env bash
# Validate SKILL.md frontmatter for Codex / Agent Skills loaders.
# Checks: --- delimiters on their own lines, YAML-safe frontmatter (Ruby Psych),
# required name/description fields, description length <= 1024.
set -euo pipefail

MAX_DESC_LEN=1024
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SKILLS_DIR="$REPO_ROOT/skills"
FAIL=0

if ! command -v ruby >/dev/null 2>&1; then
  echo "ERROR: ruby is required for frontmatter validation (YAML parsing)." >&2
  exit 2
fi

check_skill() {
  local file="$1"
  local name
  name="$(basename "$(dirname "$file")")"

  if ! head -n1 "$file" | grep -q '^---$'; then
    echo "FAIL $name: missing opening --- frontmatter delimiter"
    FAIL=1
    return
  fi

  local err desc_len
  err="$(
    ruby - "$file" "$MAX_DESC_LEN" <<'RUBY' 2>&1
require "yaml"

path = ARGV[0]
max_len = Integer(ARGV[1])
text = File.read(path, encoding: "UTF-8")

unless text.start_with?("---\n") || text.start_with?("---\r\n")
  puts "missing opening ---"
  exit 1
end

# Closing delimiter must be a line containing only --- (optional trailing whitespace).
match = text.match(/\A---[ \t]*\r?\n(.*?)\r?\n---[ \t]*\r?(?:\n|\z)/m)
unless match
  puts "missing or invalid closing --- delimiter (must be exactly --- on its own line)"
  exit 1
end

begin
  data = YAML.safe_load(match[1], permitted_classes: [], aliases: false)
rescue Psych::SyntaxError => e
  puts "invalid YAML: #{e.message}"
  exit 1
end

unless data.is_a?(Hash)
  puts "frontmatter must be a YAML mapping"
  exit 1
end

unless data.key?("name")
  puts "missing name field"
  exit 1
end
unless data.key?("description")
  puts "missing description field"
  exit 1
end

desc = data["description"]
unless desc.is_a?(String)
  puts "description must be a string (use quoted or block scalar, not a mapping/list)"
  exit 1
end

puts desc.length
RUBY
  )"

  if [[ ! "$err" =~ ^[0-9]+$ ]]; then
    echo "FAIL $name: $err"
    FAIL=1
    return
  fi
  desc_len="$err"

  if ((desc_len > MAX_DESC_LEN)); then
    echo "FAIL $name: description length $desc_len exceeds $MAX_DESC_LEN"
    FAIL=1
    return
  fi
  echo "OK   $name (${desc_len} chars)"
}

echo "Validating skills under $SKILLS_DIR"
for skill_dir in "$SKILLS_DIR"/*/; do
  [[ -f "${skill_dir}SKILL.md" ]] || continue
  check_skill "${skill_dir}SKILL.md"
done

if ((FAIL != 0)); then
  exit 1
fi
echo "All skill frontmatter checks passed."
