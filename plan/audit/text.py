#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Sentences that match the patterns `.claude/rules/writing.md` names.

For step 86's audit (plan/notes/audit/method.md, the inventories). Reads
what a person reads: the comments and doc comments of the Rust sources,
their `#[error(...)]` messages, the Markdown of README, the manual, the
guide and the changelog, and the comments and README of the Rocq
library. Prints Markdown: a count per pattern, then every match with its
place. A match is a candidate: a reader decides whether it breaks the
rule ("underscore" is a character, "robust" may be the right word).

usage: text.py DEDUXIT_ROOT [ROCQ_ROOT]
"""

import os
import re
import sys
from collections import defaultdict

PATTERNS = {
    "says more than is or has": r"\b(serves? as|stands? as|boasts?|refers? to)\b",
    "inflated word": (
        r"\b(crucial|pivotal|landscape|testament|delves?|underscores?|showcases?"
        r"|intricate|fosters?|leverages?|seamless(ly)?|robust)\b"
    ),
    "negative parallelism": (
        r"\bnot (just|only|merely)\b.{0,60}\bbut\b|\bit'?s not\b.{0,40}\bit'?s\b"
    ),
    "rather than": r"\brather than\b",
    "trailing -ing commentary": (
        r",\s+(highlighting|ensuring|allowing|making|providing|enabling|reflecting"
        r"|showcasing|underscoring|emphasi[sz]ing|demonstrating)\b"
    ),
    "filler to the reader": (
        r"\b(it'?s|it is) (important|worth) (to note|noting|mentioning)\b"
        r"|\b(in summary|note that|please note|i hope|let me know|should not be confused)\b"
    ),
    "vague attribution": (
        r"\b(experts (agree|say)|it is (widely|generally) (known|accepted)|studies show)\b"
    ),
    "narration of how it was made": (
        r"\b(this session|previously|used to|no longer|in the past|formerly"
        r"|we (now|used|decided|chose))\b"
    ),
    "first person": r"\b(we|our|let's)\b",
    "curly quote": r"[“”‘’]",
    "emoji or dingbat": r"[\U0001F300-\U0001FAFF☀-➿]",
}
COMPILED = {
    name: re.compile(pattern, re.IGNORECASE) for name, pattern in PATTERNS.items()
}
RUST = ["core/src", "cli/src", "web/src", "web-render/src", "web-tab/src", "bench/src"]
MARKDOWN = ["README.md", "CHANGELOG.md", "docs/src", "skills"]


def walk(base, suffix):
    """Yields the files under `base` with a suffix, or `base` itself."""
    if os.path.isfile(base):
        yield base
        return
    for d, dirs, names in os.walk(base):
        dirs[:] = [x for x in dirs if x not in ("target", "snapshots", "results")]
        for name in sorted(names):
            if name.endswith(suffix):
                yield os.path.join(d, name)


def rust_prose(path):
    """Yields the comment lines and error messages of a Rust file."""
    with open(path, encoding="utf-8") as f:
        for number, line in enumerate(f, 1):
            text = line.strip()
            if text.startswith("//") or "#[error(" in text:
                yield number, text


def markdown_prose(path):
    """Yields the lines of a Markdown file outside its code blocks."""
    fenced = False
    with open(path, encoding="utf-8") as f:
        for number, line in enumerate(f, 1):
            if line.lstrip().startswith("```"):
                fenced = not fenced
            elif not fenced:
                yield number, line.strip()


def rocq_prose(path):
    """Yields the comment lines of a Rocq file."""
    depth = 0
    with open(path, encoding="utf-8") as f:
        for number, line in enumerate(f, 1):
            opened = depth > 0 or "(*" in line
            depth += line.count("(*") - line.count("*)")
            if opened:
                yield number, line.strip()


def main():
    root = sys.argv[1]
    rocq = sys.argv[2] if len(sys.argv) > 2 else None
    sources = []
    for sub in RUST:
        sources += [(p, rust_prose) for p in walk(os.path.join(root, sub), ".rs")]
    for sub in MARKDOWN:
        sources += [(p, markdown_prose) for p in walk(os.path.join(root, sub), ".md")]
    if rocq:
        for sub in ("theories", "tests", "bridges"):
            sources += [(p, rocq_prose) for p in walk(os.path.join(rocq, sub), ".v")]
        sources.append((os.path.join(rocq, "README.md"), markdown_prose))
        sources.append((os.path.join(rocq, "CHANGELOG.md"), markdown_prose))
    hits = defaultdict(list)
    lines = 0
    for path, prose in sources:
        if not os.path.isfile(path):
            continue
        base = rocq if rocq and path.startswith(rocq) else root
        shown = os.path.relpath(path, os.path.dirname(base) if base == rocq else base)
        for number, text in prose(path):
            lines += 1
            for name, pattern in COMPILED.items():
                if pattern.search(text):
                    hits[name].append((shown, number, text))
    print("# Text that matches a pattern of the writing rules\n")
    print(f"{lines} lines of prose read in {len(sources)} files.\n")
    print("| pattern | matches |\n|---|---:|")
    for name in PATTERNS:
        print(f"| {name} | {len(hits[name])} |")
    for name in PATTERNS:
        print(f"\n## {name}: {len(hits[name])}\n")
        for shown, number, text in hits[name]:
            print(f"- `{shown}:{number}` {text[:200]}")


main()
