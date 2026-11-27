#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Every `--help` of the command, and its flags as one table.

For step 86's audit (plan/notes/audit/method.md, the inventories). Runs
`BINARY ... --help` for the command and every subcommand it finds under
"Commands:", writes each text to OUT_DIR, and prints Markdown: the
commands, then each flag with its short form, its value and the
commands that take it, so that a flag that differs between commands
shows.

usage: help.py BINARY OUT_DIR
"""

import os
import re
import subprocess
import sys
from collections import defaultdict

FLAG = re.compile(r"^\s+(?:(-\w), )?(--[\w-]+)(?:[ =]?(\[?<[^>]+>\]?(?:\.\.\.)?))?")


def run(binary, path, flag):
    """Returns what `binary path... flag` prints."""
    result = subprocess.run(
        [binary, *path, flag], capture_output=True, text=True, check=False, timeout=60
    )
    return result.stdout or result.stderr


def subcommands(short_help):
    """Returns the names under the "Commands:" heading of a short help."""
    names, inside = [], False
    for line in short_help.splitlines():
        if line.strip() == "Commands:":
            inside = True
        elif inside and not line.strip():
            break
        elif inside and line.startswith("  ") and not line.startswith("   "):
            name = line.split()[0]
            if name != "help":
                names.append(name)
    return names


def main():
    binary, out = sys.argv[1], sys.argv[2]
    os.makedirs(out, exist_ok=True)
    flags = defaultdict(list)
    commands = []
    queue = [[]]
    while queue:
        path = queue.pop(0)
        short = run(binary, path, "-h")
        name = " ".join(path) or "(top level)"
        with open(os.path.join(out, "-".join(path or ["top"]) + ".txt"), "w") as f:
            f.write(run(binary, path, "--help"))
        heading = ""
        count = 0
        for line in short.splitlines():
            if line and not line.startswith(" ") and line.rstrip().endswith(":"):
                heading = line.strip().rstrip(":")
            found = FLAG.match(line)
            if found:
                count += 1
                flags[found.group(2)].append(
                    (name, found.group(1) or "", found.group(3) or "", heading)
                )
        commands.append((name, count, len(short.splitlines())))
        queue += [path + [child] for child in subcommands(short)]
    print("# The command's help\n")
    print("| command | flags | lines of `-h` |\n|---|---:|---:|")
    for name, count, length in commands:
        print(f"| `{name}` | {count} | {length} |")
    print(f"\n## The flags: {len(flags)}\n")
    print("| flag | short | value | heading | commands |\n|---|---|---|---|---|")
    differing = []
    for flag in sorted(flags):
        uses = flags[flag]
        shapes = {(short, value, heading) for _, short, value, heading in uses}
        if len(shapes) > 1:
            differing.append(flag)
        for short, value, heading in sorted(shapes):
            takers = [n for n, s, v, h in uses if (s, v, h) == (short, value, heading)]
            print(
                f"| `{flag}` | `{short}` | `{value}` | {heading} | {', '.join(takers)} |"
            )
    print(
        f"\n## Flags whose short form, value or heading differs between commands: {len(differing)}\n"
    )
    for flag in differing:
        print(f"- `{flag}`")
    shorts = defaultdict(set)
    for flag, uses in flags.items():
        for _, short, _, _ in uses:
            if short:
                shorts[short].add(flag)
    print("\n## One-letter forms that stand for more than one flag\n")
    for short, names in sorted(shorts.items()):
        if len(names) > 1:
            print(f"- `{short}`: {', '.join(f'`{n}`' for n in sorted(names))}")


main()
