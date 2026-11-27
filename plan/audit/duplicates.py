#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Duplicate code in the Rust sources, found by PMD's copy/paste detector.

For step 86's audit (plan/notes/audit/method.md, the inventories). CPD's
Rust lexer stops at a non-ASCII character and at some string and
character literals ('∨', "\\/", "~") and skips the file, so the sources
are first copied with every comment, string and character literal
emptied and every other non-ASCII character replaced by `u`, which keeps
each line where it was. Prints Markdown: each duplicate with its length
and its places.

usage: duplicates.py PMD REPOSITORY_ROOT SCRATCH_DIR [MINIMUM_TOKENS] [DIR ...]

PMD is the `pmd` launcher (`nix build --inputs-from . nixpkgs#pmd`). Run
it in a capped scope: CPD starts a JVM.
"""

import csv
import os
import re
import subprocess
import sys

DIRS = ["core/src", "cli/src", "web/src", "web-render/src", "web-tab/src", "bench/src"]
# Test-only files: a duplicate between tests is no finding.
SKIP = ("tests.rs", "reference.rs", "reference_fo.rs", "generate.rs", "oracle.rs")


RAW_STRING = re.compile(r'b?r(#*)"')


def neutral(text):
    """Returns Rust source with every comment, string and character
    literal emptied, line breaks kept, so that CPD's lexer reads it."""
    out = []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        raw = RAW_STRING.match(text, i) if c in "br" else None
        if text.startswith("//", i):
            end = text.find("\n", i)
            i = n if end < 0 else end
        elif text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            out.append("\n" * text.count("\n", i, j))
            i = j
        elif raw and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            close = '"' + raw.group(1)
            end = text.find(close, raw.end())
            end = n if end < 0 else end + len(close)
            out.append('"s"' + "\n" * text.count("\n", i, end))
            i = end
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append('"s"' + "\n" * text.count("\n", i, j))
            i = j + 1
        elif c == "'" and i + 1 < n and text[i + 1] == "\\":
            end = text.find("'", i + 3)
            out.append("'c'")
            i = n if end < 0 else end + 1
        elif c == "'" and i + 2 < n and text[i + 2] == "'":
            out.append("'c'")
            i += 3
        else:
            out.append(c if ord(c) < 128 else "u")
            i += 1
    return "".join(out)


def copy_ascii(root, scratch, dirs):
    """Copies the sources under `dirs` to `scratch`, made neutral."""
    for sub in dirs:
        for d, _, names in os.walk(os.path.join(root, sub)):
            for name in names:
                if not name.endswith(".rs") or name in SKIP:
                    continue
                source = os.path.join(d, name)
                target = os.path.join(scratch, os.path.relpath(source, root))
                os.makedirs(os.path.dirname(target), exist_ok=True)
                with open(source, encoding="utf-8") as f:
                    text = f.read()
                with open(target, "w", encoding="ascii") as f:
                    f.write(neutral(text))


def main():
    pmd, root, scratch = sys.argv[1], sys.argv[2], sys.argv[3]
    minimum = sys.argv[4] if len(sys.argv) > 4 else "100"
    dirs = sys.argv[5:] or DIRS
    copy_ascii(root, scratch, dirs)
    result = subprocess.run(
        [pmd, "cpd", "-l", "rust", "--minimum-tokens", minimum, "-d", scratch]
        + ["-f", "csv", "--no-fail-on-violation", "--no-fail-on-error"],
        capture_output=True,
        text=True,
        check=False,
    )
    skipped = [line for line in result.stderr.splitlines() if "Skipping file" in line]
    rows = list(csv.reader(result.stdout.splitlines()))[1:]
    found = []
    for row in rows:
        lines, tokens = int(row[0]), int(row[1])
        places = [
            f"{os.path.relpath(row[i + 1], scratch)}:{row[i]}"
            for i in range(3, len(row) - 1, 2)
        ]
        found.append((tokens, lines, places))
    found.sort(reverse=True)
    print(f"# Duplicate code (CPD, at least {minimum} tokens)\n")
    print(f"{len(found)} duplicates; {len(skipped)} files skipped by the lexer.\n")
    for line in skipped:
        print(f"- {line}")
    print("\n| tokens | lines | places |\n|---:|---:|---|")
    for tokens, lines, places in found:
        print(f"| {tokens} | {lines} | {'<br>'.join(f'`{p}`' for p in places)} |")
    across = sum(
        1 for _, _, places in found if len({p.split(":")[0] for p in places}) > 1
    )
    print(f"\n{across} of them are between different files.")


main()
