#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Clippy's and rustc's warnings by lint and by area, and the long functions.

For step 86's audit (plan/notes/audit/baselines.md). Reads cargo's JSON
messages of a Clippy run in which every lint that `Cargo.toml` allows is a
warning (`cargo clippy --workspace --all-targets --message-format=json --
-W LINT ...`), counts each place once, and prints Markdown: the lints by
area, then every function Clippy's `too_many_lines` names, longest first.

usage: clippy.py CLIPPY_JSON
"""

import json
import re
import sys
from collections import Counter, defaultdict

AREAS = [
    ("core/src/linear/search", "linear::search"),
    ("core/src/linear/proofs", "linear::proofs"),
    ("core/src/linear/nets", "linear::nets"),
    ("core/src/linear", "linear (rest)"),
    ("core/src/ordinary", "ordinary"),
    ("core/src/modal", "modal"),
    ("core/src/lambda", "lambda"),
    ("core/src/rules", "rules"),
    ("core/src/export", "export"),
    ("core/src/protocol", "protocol"),
    ("core/src/workspace", "workspace"),
    ("core/src/serialize", "serialize"),
    ("core/src", "core (root)"),
    ("core/tests", "core tests"),
    ("cli", "cli"),
    ("bench", "bench"),
    ("web", "web*"),
    ("compat", "compat"),
]


LENGTH = re.compile(r"\((\d+)/\d+\)")


def area(path):
    for prefix, name in AREAS:
        if path.startswith(prefix):
            return name
    return "other"


def main():
    seen = set()
    by_lint = Counter()
    by_area = defaultdict(Counter)
    long = []
    with open(sys.argv[1], encoding="utf-8") as f:
        for line in f:
            if not line.startswith("{"):
                continue
            msg = json.loads(line)
            if msg.get("reason") != "compiler-message":
                continue
            m = msg["message"]
            code = (m.get("code") or {}).get("code")
            if not code or m.get("level") not in ("warning", "error"):
                continue
            spans = [s for s in m.get("spans", []) if s.get("is_primary")]
            if not spans:
                continue
            s = spans[0]
            key = (code, s["file_name"], s["line_start"], s["column_start"])
            if key in seen:
                continue
            seen.add(key)
            by_lint[code] += 1
            by_area[code][area(s["file_name"])] += 1
            length = LENGTH.search(m.get("message", ""))
            if code == "clippy::too_many_lines" and length:
                long.append(
                    (int(length.group(1)), f"{s['file_name']}:{s['line_start']}")
                )
    names = [name for _, name in AREAS]
    used = [n for n in names if any(by_area[c][n] for c in by_lint)]
    print(f"sites: {sum(by_lint.values())}, lints that fire: {len(by_lint)}\n")
    print("| lint | sites | " + " | ".join(used) + " |")
    print("|---|---:|" + "---:|" * len(used))
    for code, n in by_lint.most_common():
        row = " | ".join(str(by_area[code][u] or "") for u in used)
        print(f"| `{code}` | {n} | {row} |")
    print(f"\n## Functions over Clippy's line limit: {len(long)}\n")
    print("| lines | at |\n|---:|---|")
    for n, at in sorted(long, reverse=True):
        print(f"| {n} | `{at}` |")


main()
