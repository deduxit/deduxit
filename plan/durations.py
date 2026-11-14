#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""How long each step session took, read from the Claude Code transcripts.

Writes one row per session transcript of this project (the planning
session's own long-running transcripts included, marked `planner`): the
step it ran, the model, its start (local time, CET), its wall-clock
hours, and its phases in minutes from the start: the first commit, the
first panel member, the fresh reviewer, the last commit; and the gate's
and the flake check's runs in minutes, background runs included.

Usage: python3 -I plan/durations.py OUT.tsv
"""

import glob
import json
import os
import re
import sys
from datetime import datetime, timedelta

CET = timedelta(hours=1)
OWN = re.compile(
    r"You are step session (\d{2}[a-z0-9-]*) of"
    r"|start-(\d{2}[a-z0-9-]*)\.md"
    r"|/goal Step \d+[a-z]?'s session (\d{2}[a-z0-9-]*)"
    r"|^# (?:Step|Session) (\d{2}[a-z0-9-]*)"
)
STEP = re.compile(
    r"(?:[Ss]tep session|[Ss]ession|[Ss]tep)\s+(\d{2}[a-z]?\d?(?:-[a-z]+\d*)?)"
    r"|start-(\d{2}[a-z0-9-]*)\.md|goal-(\d{2}[a-z0-9-]*)\.txt"
)
NOTE = re.compile(r"<tool-use-id>(toolu_[A-Za-z0-9]+)</tool-use-id>")


def when(stamp):
    """The transcript's UTC timestamp as local time."""
    return datetime.fromisoformat(stamp.replace("Z", "+00:00")) + CET


def kind(command):
    """The check a shell command runs, or None."""
    if "nix flake check" in command:
        return "flake"
    if re.search(r"-c gate\b", command):
        return "gate"
    if "jj commit" in command:
        return "commit"
    return None


def texts_of(content):
    """The text parts of a user message's content."""
    if isinstance(content, str):
        return [content]
    return [
        x.get("text", "")
        for x in content or []
        if isinstance(x, dict) and x.get("type") == "text"
    ]


def scan(path):
    """One row of the table for a transcript file, or None."""
    marks, pending, runs, texts, model = [], {}, {"gate": [], "flake": []}, [], None
    start = end = None
    with open(path, errors="replace") as handle:
        for line in handle:
            try:
                entry = json.loads(line)
            except ValueError:
                continue
            stamp = entry.get("timestamp")
            if not stamp:
                continue
            now = when(stamp)
            start = start or now
            end = now
            message = entry.get("message") or {}
            content = message.get("content")
            if entry.get("type") == "assistant":
                model = model or message.get("model")
                for part in content or []:
                    if not isinstance(part, dict) or part.get("type") != "tool_use":
                        continue
                    given = part.get("input") or {}
                    if part.get("name") in ("Agent", "Task"):
                        marks.append(
                            (now, "agent:" + str(given.get("subagent_type") or ""))
                        )
                    if part.get("name") == "Bash":
                        found = kind(given.get("command", ""))
                        if found:
                            marks.append((now, found))
                            if found != "commit":
                                pending[part["id"]] = (
                                    found,
                                    now,
                                    bool(given.get("run_in_background")),
                                )
            elif entry.get("type") == "user":
                if len(texts) < 8:
                    texts.extend(t[:4000] for t in texts_of(content))
                for part in content if isinstance(content, list) else []:
                    if isinstance(part, dict) and part.get("type") == "tool_result":
                        done = pending.get(part.get("tool_use_id"))
                        if done and not done[2]:
                            runs[done[0]].append(now - done[1])
                            del pending[part["tool_use_id"]]
                for text in texts_of(content):
                    for ident in NOTE.findall(text):
                        done = pending.pop(ident, None)
                        if done:
                            runs[done[0]].append(now - done[1])
    if start is None:
        return None
    step = ""
    for text in texts[:8]:
        own = OWN.search(text)
        if own:
            step = next(g for g in own.groups() if g).rstrip("-")
            break
    if not step:
        found = [
            next(g for g in m.groups() if g)
            for t in texts[:8]
            for m in STEP.finditer(t)
        ]
        step = max(found, key=len) if found else ""
    first = " ".join(texts[:2])[:200]
    hours = (end - start).total_seconds() / 3600
    if (
        re.search(r"Handoff|act on it: you are the planning session", first)
        or hours > 12
    ):
        step = "planner"

    def at(prefix, last=False):
        """Minutes from the start to the first (or last) mark of a kind."""
        times = [t for t, k in marks if k.startswith(prefix)]
        if not times:
            return ""
        return str(
            round(((times[-1] if last else times[0]) - start).total_seconds() / 60)
        )

    def minutes(spans):
        """The runs' durations in whole minutes."""
        return " ".join(str(round(s.total_seconds() / 60)) for s in spans)

    return [
        os.path.basename(path)[:8],
        step,
        (model or "").replace("claude-", ""),
        start.strftime("%Y-%m-%d %H:%M"),
        f"{hours:.2f}",
        at("commit"),
        at("agent:panel"),
        at("agent:fresh"),
        at("commit", last=True),
        str(sum(1 for _, k in marks if k == "commit")),
        minutes(runs["gate"]),
        minutes(runs["flake"]),
    ]


def main():
    """Scans every transcript of the project's folders and writes the table."""
    folders = [
        d
        for d in glob.glob(os.path.expanduser("~/.claude/projects/*"))
        if re.search(r"own-(linlog|deduxit|rocq)", d) and "scratchpad" not in d
    ]
    rows = [
        r for f in folders for f in glob.glob(f + "/*.jsonl") for r in [scan(f)] if r
    ]
    rows.sort(key=lambda r: r[3])
    header = "file step model start hours first_commit_min panel_min review_min last_commit_min commits gate_min flake_min"
    with open(sys.argv[1], "w") as out:
        out.write("\t".join(header.split()) + "\n")
        for row in rows:
            out.write("\t".join(row) + "\n")
    print(len(rows), "transcripts")


if __name__ == "__main__":
    main()
