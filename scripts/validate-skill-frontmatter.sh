#!/usr/bin/env bash
# Validate SKILL.md frontmatter for Codex / Agent Skills loaders.
# Checks: --- delimiters, parseable YAML header, description length <= 1024.
set -euo pipefail

MAX_DESC_LEN=1024
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SKILLS_DIR="$REPO_ROOT/skills"
FAIL=0

check_skill() {
  local file="$1"
  local name
  name="$(basename "$(dirname "$file")")"

  if ! head -n1 "$file" | grep -q '^---$'; then
    echo "FAIL $name: missing opening --- frontmatter delimiter"
    FAIL=1
    return
  fi

  local desc_len err
  err="$(
    python3 - "$file" "$MAX_DESC_LEN" <<'PY' 2>&1
import re, sys
from pathlib import Path

path = Path(sys.argv[1])
max_len = int(sys.argv[2])
text = path.read_text(encoding="utf-8")
if not text.startswith("---\n"):
    print("missing opening ---"); sys.exit(1)
match = re.search(r"\n---\r?\n", text[4:])
if match is None:
    print("missing closing ---"); sys.exit(1)
end = 4 + match.start()
block = text[4:end]
# Minimal YAML: only fields we need
lines = block.splitlines()
data = {}
i = 0
while i < len(lines):
    line = lines[i]
    if not line.strip() or line.lstrip().startswith("#"):
        i += 1
        continue
    if ":" not in line:
        i += 1
        continue
    key, rest = line.split(":", 1)
    key = key.strip()
    rest = rest.strip()
    if rest in ("|", ">"):
        i += 1
        parts = []
        while i < len(lines) and (lines[i].startswith("  ") or lines[i] == ""):
            parts.append(lines[i][2:] if lines[i].startswith("  ") else "")
            i += 1
        data[key] = "\n".join(parts).strip()
        continue
    if rest.startswith('"') and rest.endswith('"'):
        data[key] = rest[1:-1]
    elif rest.startswith("'") and rest.endswith("'"):
        data[key] = rest[1:-1]
    else:
        data[key] = rest
    i += 1

if "name" not in data:
    print("missing name field"); sys.exit(1)
if "description" not in data:
    print("missing description field"); sys.exit(1)
desc = data["description"]
if not isinstance(desc, str):
    print("description must be a string"); sys.exit(1)
print(len(desc))
PY
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
