#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""The web module's bytes by the module of the library they come from.

For step 86's audit (plan/notes/audit/method.md, the measurements).
Reads `twiggy top -n 1000000 --format json` of the bindings' module as
cargo builds it (before wasm-bindgen and wasm-opt, names kept) and
prints Markdown: the raw bytes by suite and shared module of the
library, by other crate, the data segments, and the largest items. An
item is counted for the first path of deduxit its name holds, so a
generic of another crate instantiated with a type of a suite counts for
that suite. The shares are of the raw module; gzip and wasm-opt shrink
the parts unevenly, so read them as proportions.

usage: wasm.py TWIGGY_TOP_JSON
"""

import json
import re
import sys
from collections import Counter

OWN = re.compile(r"\bdeduxit(?:_web_tab|_web_render|_web)?::((?:[a-z_][a-z0-9_]*::)*)")
OTHER = re.compile(r"\b([a-z_][a-z0-9_]*)::")
DEEP = ("linear", "ordinary")
# The hash rustc's symbol names carry after each crate's name.
DISAMBIGUATOR = re.compile(r"\[[0-9a-f]{16}\]")


def place(name):
    """Returns the group an item is counted for: a module, a crate, or data."""
    name = DISAMBIGUATOR.sub("", name)
    crate = re.search(r"\b(deduxit(?:_web_tab|_web_render|_web)?)::", name)
    if crate:
        found = OWN.search(name)
        modules = [part for part in found.group(1).split("::") if part]
        if crate.group(1) != "deduxit":
            return "the bindings (" + crate.group(1) + ")"
        if not modules:
            return "deduxit (root)"
        depth = 2 if modules[0] in DEEP else 1
        return "deduxit::" + "::".join(modules[:depth])
    if name.startswith(("data[", "data segment")):
        return "data segments"
    if name.startswith(
        ("elem[", "table[", "type[", "import ", "export ", "global[", "memory[")
    ):
        return "tables, types, imports and exports"
    if "section" in name:
        return "sections of names and producers"
    other = OTHER.search(name)
    return "crate " + other.group(1) if other else "other"


def generic(name):
    """Returns a name without its last list of type arguments, `::<...>`."""
    if not name.endswith(">"):
        return name
    depth = 0
    for i in range(len(name) - 1, -1, -1):
        if name[i] == ">":
            depth += 1
        elif name[i] == "<":
            depth -= 1
            if depth == 0:
                return name[: i - 2] if name[i - 2 : i] == "::" else name
    return name


def main():
    with open(sys.argv[1], encoding="utf-8") as f:
        items = json.load(f)
    sizes = Counter()
    for item in items:
        sizes[place(item["name"])] += item["shallow_size"]
    total = sum(sizes.values())
    print("# The web module's bytes, by where they come from\n")
    print(
        f"{len(items)} items, {total} bytes raw (before wasm-bindgen and wasm-opt).\n"
    )

    suites = Counter()
    for group, size in sizes.items():
        if group.startswith("deduxit::"):
            suites["deduxit::" + group.split("::")[1]] += size
    print("## By module of the library\n")
    print("| module | bytes | share |\n|---|---:|---:|")
    for group, size in suites.most_common():
        print(f"| `{group}` | {size} | {100 * size / total:.1f} % |")
    own = sum(suites.values()) + sizes["deduxit (root)"]
    print(f"| all of deduxit | {own} | {100 * own / total:.1f} % |")

    for suite in DEEP:
        print(f"\n## Inside `deduxit::{suite}`\n")
        print("| module | bytes | share |\n|---|---:|---:|")
        inner = {g: s for g, s in sizes.items() if g.startswith(f"deduxit::{suite}")}
        for group, size in sorted(inner.items(), key=lambda kv: -kv[1]):
            print(f"| `{group}` | {size} | {100 * size / total:.1f} % |")

    print("\n## Everything else\n")
    print("| group | bytes | share |\n|---|---:|---:|")
    rest = {g: s for g, s in sizes.items() if not g.startswith("deduxit::")}
    for group, size in sorted(rest.items(), key=lambda kv: -kv[1])[:25]:
        print(f"| {group} | {size} | {100 * size / total:.1f} % |")

    groups = {}
    for item in items:
        base, size = generic(DISAMBIGUATOR.sub("", item["name"])), item["shallow_size"]
        groups.setdefault(base, []).append(size)
    repeated = [
        (sum(sorted(sizes)[:-1]), len(sizes), base)
        for base, sizes in groups.items()
        if len(sizes) > 1
    ]
    again = sum(extra for extra, _, _ in repeated)
    print("\n## Generic functions compiled more than once\n")
    print(
        f"{again} bytes ({100 * again / total:.1f} %) are the second and later"
        " instantiations of a function: what one shared body would save at most.\n"
    )
    print("| bytes beyond the largest copy | copies | function |\n|---:|---:|---|")
    for extra, copies, base in sorted(repeated, reverse=True)[:30]:
        print(f"| {extra} | {copies} | `{base[:170]}` |")

    print("\n## The 40 largest items\n")
    print("| bytes | item |\n|---:|---|")
    for item in sorted(items, key=lambda i: -i["shallow_size"])[:40]:
        name = DISAMBIGUATOR.sub("", item["name"])
        print(f"| {item['shallow_size']} | `{name[:200]}` |")


main()
