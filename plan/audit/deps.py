#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Which module of the core crate names which.

For step 86's audit (plan/notes/audit/method.md, the inventories). Reads
every `crate::` path of `core/src` (in `use` trees and in code, test
files apart) and prints Markdown: the modules as a matrix of who names
whom, then, for every pair of different suites and for the shared
modules, the items named, so that the layering shows. A path through
`super::` stays inside its parent and is not counted.

usage: deps.py REPOSITORY_ROOT
"""

import os
import re
import sys
from collections import Counter, defaultdict

SUITES = ("ordinary", "linear", "modal", "lambda", "rules")
TEST_NAMES = ("tests.rs", "reference.rs", "reference_fo.rs", "generate.rs", "oracle.rs")
TOKEN = re.compile(r"[A-Za-z_][A-Za-z0-9_]*|::|[{},*;]")
USE = re.compile(r"\buse\s+(crate::[^;]*);", re.S)
INLINE = re.compile(r"\bcrate((?:::[A-Za-z_][A-Za-z0-9_]*)+)")
COMMENT = re.compile(r"//[^\n]*")


def expand(tokens, position, prefix, out):
    """Expands a `use` tree from `position`; returns the position after it."""
    path = list(prefix)
    while position < len(tokens):
        token = tokens[position]
        if token == "{":
            position += 1
            while position < len(tokens) and tokens[position] != "}":
                position = expand(tokens, position, path, out)
                if position < len(tokens) and tokens[position] == ",":
                    position += 1
            return position + 1
        if token in (",", "}", ";"):
            break
        if token == "as":
            position += 2
            continue
        if token not in ("::", "self", "*"):
            path.append(token)
        position += 1
    out.append(tuple(path))
    return position


def paths_of(text):
    """Returns the `crate::` paths a source names, as tuples of segments."""
    text = COMMENT.sub("", text)
    found = []
    for tree in USE.findall(text):
        tokens = TOKEN.findall(tree)
        expand(tokens, 2, [], found)
    for tail in INLINE.findall(USE.sub("", text)):
        found.append(tuple(tail.split("::")[1:]))
    return [path for path in found if path]


def module_of(segments, depth):
    """Names the module of a path or file: its first `depth` lowercase segments."""
    module = []
    for segment in segments:
        if not segment or not segment[0].islower() or len(module) == depth:
            break
        module.append(segment)
    if module and module[0] not in SUITES:
        module = module[:1]
    return "::".join(module) or "(root)"


def main():
    root = os.path.join(sys.argv[1], "core", "src")
    edges = Counter()
    items = defaultdict(Counter)
    for d, dirs, names in os.walk(root):
        dirs.sort()
        for name in sorted(names):
            if not name.endswith(".rs") or name in TEST_NAMES or name.endswith("_tests.rs"):
                continue
            rel = os.path.relpath(os.path.join(d, name), root)
            here = rel[: -len(".rs")].split(os.sep)
            if here[-1] in ("mod", "lib"):
                here = here[:-1]
            source = module_of(here, 2)
            with open(os.path.join(d, name), encoding="utf-8") as f:
                for path in set(paths_of(f.read())):
                    target = module_of(path, 2)
                    if target == source:
                        continue
                    edges[(source, target)] += 1
                    items[(source.split("::")[0], target.split("::")[0])]["::".join(path)] += 1

    modules = sorted({m for pair in edges for m in pair})
    top = sorted({m.split("::")[0] for m in modules})
    print("# Which module names which (`core/src`, test files apart)\n")
    print(
        "A cell counts the files of the row's module that name a path of the"
        " column's, each path once per file.\n"
    )
    print("| names → | " + " | ".join(f"`{m}`" for m in top) + " |")
    print("|---|" + "---:|" * len(top))
    for source in top:
        row = []
        for target in top:
            n = sum(
                count
                for (s, t), count in edges.items()
                if s.split("::")[0] == source and t.split("::")[0] == target
            )
            row.append(str(n) if n else "")
        print(f"| `{source}` | " + " | ".join(row) + " |")

    print("\n## Between the suites\n")
    for source in SUITES:
        for target in SUITES:
            named = items.get((source, target))
            if source == target or not named:
                continue
            print(f"\n### `{source}` names `{target}`: {len(named)} paths\n")
            for path, n in named.most_common(60):
                print(f"- `{path}`: {n} files")

    print("\n## What the shared modules name of a suite\n")
    for source in top:
        if source in SUITES:
            continue
        for target in SUITES:
            named = items.get((source, target))
            if not named:
                continue
            print(f"\n### `{source}` names `{target}`: {len(named)} paths\n")
            for path, n in named.most_common(40):
                print(f"- `{path}`: {n} files")

    print("\n## Inside the linear and the ordinary suite\n")
    for suite in ("linear", "ordinary"):
        inner = sorted(m for m in modules if m.startswith(suite + "::"))
        print(f"\n### `{suite}`\n")
        print("| names → | " + " | ".join(f"`{m.split('::')[1]}`" for m in inner) + " |")
        print("|---|" + "---:|" * len(inner))
        for source in inner:
            row = [str(edges.get((source, target), "") or "") for target in inner]
            print(f"| `{source.split('::')[1]}` | " + " | ".join(row) + " |")


main()
