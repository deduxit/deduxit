#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Line counts per module, of deduxit and of rocq-deduxit.

For step 86's audit (plan/notes/audit/baselines.md). Prints Markdown.

usage: lines.py DEDUXIT_ROOT ROCQ_ROOT
"""

import os
import sys
from collections import defaultdict


def lines(path):
    with open(path, "rb") as f:
        return sum(1 for _ in f)


def walk(root, suffix):
    for d, dirs, files in os.walk(root):
        dirs[:] = [
            x for x in dirs if x not in ("target", ".git", ".jj", "node_modules")
        ]
        for name in files:
            if name.endswith(suffix):
                yield os.path.join(d, name)


def is_test(rel):
    parts = rel.split(os.sep)
    return (
        "tests" in parts
        or parts[-1]
        in ("tests.rs", "reference.rs", "reference_fo.rs", "generate.rs", "oracle.rs")
        or parts[-1].endswith("_tests.rs")
    )


def table(title, rows):
    print(f"\n## {title}\n")
    print("| module | files | lines | of which test files | largest file (lines) |")
    print("|---|---:|---:|---:|---|")
    total = [0, 0, 0]
    for key in sorted(rows):
        files, n, t, big = rows[key]
        total[0] += files
        total[1] += n
        total[2] += t
        print(f"| `{key}` | {files} | {n} | {t} | `{big[1]}` ({big[0]}) |")
    print(f"| **total** | {total[0]} | {total[1]} | {total[2]} | |")


def group(root, sub, depth, suffix=".rs"):
    base = os.path.join(root, sub)
    rows = defaultdict(lambda: [0, 0, 0, (0, "")])
    allfiles = []
    for path in walk(base, suffix):
        rel = os.path.relpath(path, base)
        parts = rel.split(os.sep)
        key = os.path.join(*parts[:depth]) if len(parts) > depth else rel
        if len(parts) > depth:
            key += os.sep
        n = lines(path)
        r = rows[key]
        r[0] += 1
        r[1] += n
        if is_test(rel):
            r[2] += n
        if n > r[3][0]:
            r[3] = (n, rel)
        allfiles.append((n, os.path.join(sub, rel)))
    return rows, allfiles


def main():
    root, rocq = sys.argv[1], sys.argv[2]
    everything = []
    for title, sub, depth in [
        ("core/src, top level", "core/src", 1),
        ("core/src/linear", "core/src/linear", 1),
        ("core/src/linear/search", "core/src/linear/search", 1),
        ("core/src/linear/proofs", "core/src/linear/proofs", 1),
        ("core/src/ordinary", "core/src/ordinary", 1),
        ("core/src/export", "core/src/export", 1),
        ("core/tests", "core/tests", 1),
        ("cli", "cli", 2),
        ("bench", "bench", 2),
        ("web", "web", 2),
        ("web-render", "web-render", 2),
        ("web-tab", "web-tab", 2),
        ("compat", "compat", 2),
    ]:
        rows, files = group(root, sub, depth)
        table(title, rows)
        if sub in (
            "core/src",
            "core/tests",
            "cli",
            "bench",
            "web",
            "web-render",
            "web-tab",
            "compat",
        ):
            everything += files
    print("\n## The 40 largest Rust files\n")
    print("| lines | file |")
    print("|---:|---|")
    for n, f in sorted(everything, reverse=True)[:40]:
        print(f"| {n} | `{f}` |")
    over = [n for n, _ in everything]
    print(
        f"\nFiles: {len(over)}; over 1 000 lines: {sum(1 for n in over if n > 1000)}; "
        f"over 3 000: {sum(1 for n in over if n > 3000)}; over 5 000: {sum(1 for n in over if n > 5000)}."
    )
    for title, sub, depth in [
        ("rocq-deduxit theories", "theories", 1),
        ("rocq-deduxit tests", "tests", 1),
        ("rocq-deduxit bridges", "bridges", 1),
    ]:
        rows, files = group(rocq, sub, depth, ".v")
        table(title, rows)
        if sub == "theories":
            print("\n| lines | file |")
            print("|---:|---|")
            for n, f in sorted(files, reverse=True):
                print(f"| {n} | `{f}` |")


main()
