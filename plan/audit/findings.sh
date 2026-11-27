#!/usr/bin/env bash
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# Writes plan/notes/audit/findings.md and findings-index.md of step 86's
# audit from the units' files: the head beside this script, the themes of
# themes.json (themes.py refuses a finding that is in no theme or in two),
# the tail beside this script and the rejected findings of rejected.md.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
notes=$root/plan/notes/audit
tools=$root/plan/audit

python3 "$tools/index.py" "$notes/units" >"$notes/findings-index.md"
{
  cat "$tools/findings-head.md"
  python3 "$tools/themes.py" "$notes/units"
  cat "$tools/findings-tail.md"
  grep -A 100000 -m 1 '^## Stage' "$notes/rejected.md" | python3 -c '
import sys
for line in sys.stdin:
    print("#" + line if line.startswith("## Stage") else line, end="")
'
} >"$notes/findings.md"
