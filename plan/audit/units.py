#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""The audit's units: what each finder reads, with its size.

For step 86's audit (plan/notes/audit/method.md, the grid). Reads
`units.json` beside this file, prints the units as a Markdown table
(files, lines and bytes of source, test files apart) with every source
file no unit holds, and writes each unit's files, one per line with its
line count, to OUT_DIR/UNIT/files.txt for the finder's brief.

usage: units.py DEDUXIT_ROOT ROCQ_ROOT OUT_DIR
"""

import json
import os
import sys

SOURCES = [
    "core/src",
    "cli/src",
    "web/src",
    "web/build.rs",
    "web-render/src",
    "web-render/build.rs",
    "web-tab/src",
]
TEST_NAMES = ("tests.rs", "reference.rs", "reference_fo.rs", "generate.rs", "oracle.rs")
KEPT = (".rs", ".v", ".nix", ".md", ".opam", ".json")


def is_test(path):
    """Says whether a Rust file holds only tests."""
    name = os.path.basename(path)
    return name in TEST_NAMES or name.endswith("_tests.rs")


def expand(root, path):
    """Yields the source files under a path of a unit, test files apart."""
    full = os.path.join(root, path)
    if os.path.isfile(full):
        yield path
        return
    for d, dirs, names in os.walk(full):
        dirs.sort()
        for name in sorted(names):
            rel = os.path.relpath(os.path.join(d, name), root)
            if name.endswith(KEPT) or name in ("Makefile", "_RocqProject"):
                if not is_test(rel):
                    yield rel


def size(root, rel):
    """Returns a file's lines and bytes."""
    with open(os.path.join(root, rel), "rb") as f:
        data = f.read()
    return data.count(b"\n"), len(data)


def main():
    deduxit, rocq, out = sys.argv[1], sys.argv[2], sys.argv[3]
    with open(
        os.path.join(os.path.dirname(__file__), "units.json"), encoding="utf-8"
    ) as f:
        units = json.load(f)
    held = set()
    print("| unit | area | what | files | lines | kB | rules files |")
    print("|---|---|---|---:|---:|---:|---|")
    total_lines = 0
    for name, unit in units.items():
        root = rocq if unit.get("root") == "rocq" else deduxit
        files = [rel for path in unit["paths"] for rel in expand(root, path)]
        sizes = [size(root, rel) for rel in files]
        if root == deduxit:
            held.update(files)
        lines = sum(n for n, _ in sizes)
        total_lines += lines
        os.makedirs(os.path.join(out, name), exist_ok=True)
        with open(os.path.join(out, name, "files.txt"), "w", encoding="utf-8") as f:
            for rel, (n, _) in zip(files, sizes, strict=True):
                f.write(f"{os.path.join(root, rel)}\t{n} lines\n")
        rules = ", ".join(f"`{r}`" for r in unit["rules"])
        print(
            f"| {name} | {unit['area']} | {unit['title']} | {len(files)} | {lines}"
            f" | {sum(b for _, b in sizes) // 1000} | {rules} |"
        )
    print(f"\n{len(units)} units, {total_lines} lines.\n")
    missing = []
    for source in SOURCES:
        for rel in expand(deduxit, source):
            if rel.endswith(".rs") and rel not in held:
                missing.append(rel)
    print(f"Source files in no unit: {len(missing)}.")
    for rel in missing:
        print(f"- `{rel}`")


main()
