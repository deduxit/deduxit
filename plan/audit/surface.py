#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""The public surface of the core crate, from its rustdoc JSON.

For step 86's audit (plan/notes/audit/method.md, the inventories). Prints
Markdown: the items reachable under several paths, the public items
without an example in their docs, and the public items that nothing
outside `core/src` names, which are candidates for `pub(crate)`.

usage: surface.py RUSTDOC_JSON REPOSITORY_ROOT

The JSON is built as the devshell's `breaks` builds it:
`RUSTC_BOOTSTRAP=1 cargo rustdoc --manifest-path core/Cargo.toml
--all-features --lib -- -Z unstable-options --output-format json`.
Uses are counted by identifier, so a name that several items share is
counted for all of them together and marked.
"""

import json
import os
import re
import sys
from collections import Counter, defaultdict

# Where the crate is used from: the other crates, the tests, the examples.
OUTSIDE = [
    "cli",
    "web",
    "web-render",
    "web-tab",
    "bench/src",
    "bench/tests",
    "compat",
    "core/tests",
    "fuzz",
    "docs/src",
    "skills",
    "README.md",
]
IDENTIFIER = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
TOP_LEVEL = ("struct", "enum", "trait", "function", "constant", "type_alias", "static")


def kind_of(item):
    """Returns the kind of a rustdoc item: the one key of its `inner`."""
    return next(iter(item["inner"]))


def identifiers(path):
    """Counts the identifiers of a file."""
    try:
        with open(path, encoding="utf-8", errors="replace") as f:
            return Counter(IDENTIFIER.findall(f.read()))
    except OSError:
        return Counter()


def files(root, sub):
    """Yields the text files under `root/sub`, or `root/sub` itself."""
    base = os.path.join(root, sub)
    if os.path.isfile(base):
        yield base
        return
    for d, dirs, names in os.walk(base):
        dirs[:] = [x for x in dirs if x not in ("target", "node_modules", "snapshots")]
        for name in names:
            if name.endswith((".rs", ".md", ".toml", ".json", ".txt")):
                yield os.path.join(d, name)


class Surface:
    """The crate's public items and the paths each is reachable under."""

    def __init__(self, doc):
        self.index = doc["index"]
        self.paths = defaultdict(set)
        self.walk(str(doc["root"]), (), set())

    def item(self, ident):
        """Returns the item of an id, or None for another crate's."""
        return self.index.get(str(ident))

    def walk(self, module, path, seen):
        """Records the path of every item a module holds or re-exports."""
        if (module, path) in seen or len(path) > 8:
            return
        seen.add((module, path))
        for child in self.index[module]["inner"]["module"]["items"]:
            item = self.item(child)
            if item is None or item["visibility"] != "public":
                continue
            kind = kind_of(item)
            if kind == "use":
                use = item["inner"]["use"]
                target = self.item(use["id"]) if use["id"] is not None else None
                if target is None:
                    continue
                if use["is_glob"]:
                    if kind_of(target) == "module":
                        self.walk(str(target["id"]), path, seen)
                    continue
                self.record(target, path + (use["name"],), seen)
            else:
                self.record(item, path + (item["name"],), seen)

    def record(self, item, path, seen):
        """Records one path of an item, and walks it if it is a module."""
        self.paths[item["id"]].add("::".join(path))
        if kind_of(item) == "module":
            self.walk(str(item["id"]), path, seen)

    def methods(self, item):
        """Yields the public inherent methods and constants of a type."""
        inner = item["inner"][kind_of(item)]
        for impl_id in inner.get("impls", []):
            impl = self.item(impl_id)
            if impl is None or impl["inner"]["impl"]["trait"] is not None:
                continue
            for member_id in impl["inner"]["impl"]["items"]:
                member = self.item(member_id)
                if member is not None and member["visibility"] == "public":
                    yield member


def has_example(item):
    """Says whether an item's docs hold a code block."""
    return "```" in (item.get("docs") or "")


def place(item):
    """Returns an item's file and line."""
    span = item.get("span")
    return f"{span['filename']}:{span['begin'][0]}" if span else "?"


def main():
    doc_path, root = sys.argv[1], sys.argv[2]
    with open(doc_path, encoding="utf-8") as f:
        surface = Surface(json.load(f))

    outside = Counter()
    for sub in OUTSIDE:
        for path in files(root, sub):
            outside.update(identifiers(path))
    inside = {}
    for path in files(root, "core/src"):
        inside[os.path.relpath(path, root)] = identifiers(path)
    inside_total = Counter()
    for counts in inside.values():
        inside_total.update(counts)

    items = [
        (surface.item(i), sorted(paths))
        for i, paths in surface.paths.items()
        if kind_of(surface.item(i)) in TOP_LEVEL
    ]
    items.sort(key=lambda pair: pair[1][0])
    shared = Counter(item["name"] for item, _ in items)

    print("# The public surface\n")
    print(
        f"Items at module level (types, traits, functions, constants): {len(items)}.\n"
    )

    several = [(item, paths) for item, paths in items if len(paths) > 1]
    print(f"## Items reachable under several paths: {len(several)}\n")
    print("| item | kind | paths |\n|---|---|---|")
    for item, paths in several:
        print(
            f"| `{item['name']}` | {kind_of(item)} | {', '.join(f'`{p}`' for p in paths)} |"
        )

    print("\n## Items without an example in their docs\n")
    by_kind = Counter()
    without = Counter()
    listed = []
    for item, paths in items:
        kind = kind_of(item)
        by_kind[kind] += 1
        if not has_example(item):
            without[kind] += 1
            listed.append((paths[0], kind, place(item)))
    methods = no_example_methods = 0
    per_type = []
    for item, paths in items:
        if kind_of(item) not in ("struct", "enum"):
            continue
        own = [m for m in surface.methods(item) if kind_of(m) == "function"]
        bare = [m for m in own if not has_example(m)]
        methods += len(own)
        no_example_methods += len(bare)
        if own:
            per_type.append((len(bare), len(own), paths[0]))
    print("| kind | items | without an example |\n|---|---:|---:|")
    for kind, n in by_kind.most_common():
        print(f"| {kind} | {n} | {without[kind]} |")
    print(f"| inherent method | {methods} | {no_example_methods} |")
    print("\nTypes by methods without an example (the 40 with most):\n")
    print("| type | methods | without an example |\n|---|---:|---:|")
    for bare, own, path in sorted(per_type, reverse=True)[:40]:
        print(f"| `{path}` | {own} | {bare} |")

    print("\n## Items that nothing outside `core/src` names\n")
    print(
        "Counted by identifier in "
        + ", ".join(f"`{s}`" for s in OUTSIDE)
        + '. "elsewhere in core" counts the identifier in `core/src` outside the item\'s'
        " file. A name several items share is marked and counted for all of them.\n"
    )
    print(
        "| item | kind | at | elsewhere in core | shared name |\n|---|---|---|---:|---|"
    )
    unused = 0
    for item, paths in items:
        name = item["name"]
        if outside[name]:
            continue
        unused += 1
        span = item.get("span") or {}
        own_file = inside.get(span.get("filename", ""), Counter())[name]
        mark = "yes" if shared[name] > 1 else ""
        print(
            f"| `{paths[0]}` | {kind_of(item)} | `{place(item)}` |"
            f" {inside_total[name] - own_file} | {mark} |"
        )
    print(f"\n{unused} of {len(items)} items.\n")

    print("## Methods that nothing outside `core/src` names\n")
    owners = defaultdict(set)
    members = []
    for item, paths in items:
        if kind_of(item) not in ("struct", "enum"):
            continue
        for m in surface.methods(item):
            if kind_of(m) == "function":
                owners[m["name"]].add(item["id"])
                members.append((paths[0], m))
    print(
        "Only methods whose name one public type alone has, so that the count"
        " is of that method.\n"
    )
    print(
        "| type | methods with a name of their own | not named outside | of those, not named elsewhere in core |"
    )
    print("|---|---:|---:|---:|")
    table = defaultdict(lambda: [0, 0, 0])
    dead = []
    for path, m in members:
        name = m["name"]
        if len(owners[name]) != 1:
            continue
        row = table[path]
        row[0] += 1
        if outside[name] == 0:
            row[1] += 1
            span = m.get("span") or {}
            own_file = inside.get(span.get("filename", ""), Counter())[name]
            if inside_total[name] - own_file == 0:
                row[2] += 1
                dead.append((path, name, place(m)))
    for path, (own, quiet, lone) in sorted(table.items(), key=lambda kv: -kv[1][1]):
        if quiet:
            print(f"| `{path}` | {own} | {quiet} | {lone} |")
    print("\nNamed nowhere but in their own file:\n")
    print("| method | at |\n|---|---|")
    for path, name, at in sorted(dead):
        print(f"| `{path}::{name}` | `{at}` |")

    print("\n## Items without an example, listed\n")
    print("| item | kind | at |\n|---|---|---|")
    for path, kind, at in listed:
        print(f"| `{path}` | {kind} | `{at}` |")


main()
