#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Writes the result files the audit's agents were not let write.

For step 86's audit (plan/notes/audit/method.md). The harness refuses a
subagent's `Write` of a file it takes for a report ("return findings as
text"), so a finder's `findings.md` and a verifier's `verdicts.md` exist
only as the refused call's content in the agent's transcript. This reads
the transcripts of workflow runs and writes each such file under
UNITS_DIR, the last content an agent gave for it, unless the file is
there already. Prints what it wrote.

usage: recover.py UNITS_DIR TRANSCRIPT_DIR...
"""

import glob
import json
import os
import sys


def writes(path):
    """Yields the path and content of every `Write` call of a transcript."""
    with open(path, encoding="utf-8") as f:
        for line in f:
            try:
                record = json.loads(line)
            except ValueError:
                continue
            message = record.get("message")
            content = message.get("content") if isinstance(message, dict) else None
            if record.get("type") != "assistant" or not isinstance(content, list):
                continue
            for block in content:
                if block.get("type") == "tool_use" and block.get("name") == "Write":
                    given = block.get("input") or {}
                    if given.get("file_path") and given.get("content"):
                        yield given["file_path"], given["content"]


def main():
    units = os.path.realpath(sys.argv[1])
    last = {}
    for directory in sys.argv[2:]:
        for path in sorted(glob.glob(os.path.join(directory, "agent-*.jsonl"))):
            for target, content in writes(path):
                target = os.path.realpath(target)
                if target.startswith(units + os.sep):
                    last[target] = content
    for target, content in sorted(last.items()):
        if os.path.exists(target):
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(target, "w", encoding="utf-8") as f:
            f.write(content)
        print(f"wrote {os.path.relpath(target, units)} ({len(content)} bytes)")


main()
