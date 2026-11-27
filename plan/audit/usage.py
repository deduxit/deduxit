#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""The tokens a workflow's agents used, by agent and by model.

For step 86's audit (plan/notes/audit/method.md, the cost). Reads the
transcripts of one or more workflow runs (`agent-ID.jsonl` with
`agent-ID.meta.json` beside each) and prints Markdown: per agent its
label, model, requests and tokens (new input, written to the cache, read
from the cache, output), then the sums per model. A message's usage is
counted once, however many lines of the transcript carry it.

usage: usage.py TRANSCRIPT_DIR...
"""

import glob
import json
import os
import sys
from collections import defaultdict

FIELDS = (
    "input_tokens",
    "cache_creation_input_tokens",
    "cache_read_input_tokens",
    "output_tokens",
)


def read(path):
    """Returns an agent's usage per model, its requests and its compactions."""
    messages = {}
    compactions = 0
    with open(path, encoding="utf-8") as f:
        for line in f:
            try:
                record = json.loads(line)
            except ValueError:
                continue
            if record.get("isCompactSummary") or "compact" in str(record.get("type", "")):
                compactions += 1
            message = record.get("message")
            if record.get("type") != "assistant" or not isinstance(message, dict):
                continue
            usage = message.get("usage")
            if usage and message.get("id"):
                messages[message["id"]] = (message.get("model", "?"), usage)
    sums = defaultdict(lambda: [0, 0, 0, 0])
    for model, usage in messages.values():
        for i, field in enumerate(FIELDS):
            sums[model][i] += usage.get(field) or 0
    return sums, len(messages), compactions


def thousands(n):
    """Formats a count with spaces between the thousands."""
    return f"{n:,}".replace(",", " ")


def main():
    total = defaultdict(lambda: [0, 0, 0, 0])
    print("| run | agent | model | requests | new input | cache written | cache read | output | compactions |")
    print("|---|---|---|---:|---:|---:|---:|---:|---:|")
    for directory in sys.argv[1:]:
        run = os.path.basename(os.path.normpath(directory))
        for path in sorted(glob.glob(os.path.join(directory, "agent-*.jsonl"))):
            label = "?"
            try:
                with open(path[: -len(".jsonl")] + ".meta.json", encoding="utf-8") as f:
                    label = json.load(f).get("description", "?")
            except (OSError, ValueError):
                pass
            sums, requests, compactions = read(path)
            for model, counts in sums.items():
                for i, n in enumerate(counts):
                    total[model][i] += n
                print(
                    f"| {run} | {label} | {model} | {requests} | "
                    + " | ".join(thousands(n) for n in counts)
                    + f" | {compactions} |"
                )
    print("\n| model | new input | cache written | cache read | output | all |")
    print("|---|---:|---:|---:|---:|---:|")
    for model, counts in sorted(total.items()):
        print(
            f"| {model} | "
            + " | ".join(thousands(n) for n in counts)
            + f" | {thousands(sum(counts))} |"
        )


main()
