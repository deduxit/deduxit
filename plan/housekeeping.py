#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Lists what the planning session's housekeeping may remove; removes nothing.

    plan/housekeeping.py

It covers deduxit and the repositories beside it that sessions work in (the
web client, the Rocq library), and lists:
- every workspace but the default, with whether its working copy is empty,
  whether its work is on `main@origin`, and which background job works in it;
- the directories beside the checkouts left by earlier sessions: forgotten jj
  workspaces, and session output such as screenshots (`*-stepNN*`);
- changes off `main@origin` not touched for a week;
- the background jobs of deduxit's sessions, with their state;
- failed user units of the runs.

`plan/handoff.md` ("Housekeeping") says what to do with each. It reads with
`--ignore-working-copy`, so it snapshots no workspace and writes no commit.
"""

import datetime
import json
import os
import re
import subprocess

CHECKOUT = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
PARENT = os.path.dirname(CHECKOUT)
REPOS = [
    CHECKOUT,
    os.path.join(PARENT, "linlog-prover.github.io"),
    os.path.join(PARENT, "rocq-linlog"),
]
# The names of this project's directories beside the checkouts; other
# projects' share the parent directory.
OURS = ("deduxit-", "linlog-", "rocq-linlog-")
# Directories of session output: a step's number after `-step`.
SESSION_OUTPUT = re.compile(r"^(deduxit|linlog|rocq-linlog)-step\d")
STALE_AFTER = datetime.timedelta(days=7)


def run(*args):
    """Runs a command; its standard output, or "" when it fails."""
    r = subprocess.run(args, capture_output=True, text=True, stdin=subprocess.DEVNULL)
    return r.stdout if r.returncode == 0 else ""


def jj(repo, *args):
    """Runs a read-only jj command in `repo`, without snapshotting."""
    return run(
        "jj", "--no-pager", "--color=never", "-R", repo, "--ignore-working-copy", *args
    )


def section(title, lines):
    """Prints one part of the report."""
    print(f"## {title}")
    for line in lines or ["none"]:
        print(f"- {line}")
    print()


def workspaces(repo):
    """The repository's workspaces as (name, root, empty) triples."""
    out = jj(
        repo,
        "workspace",
        "list",
        "-T",
        'name ++ "\\t" ++ if(root, root, "") ++ "\\t" ++ if(target.empty(), "empty", "work") ++ "\\n"',
    )
    return [
        tuple(line.split("\t")) for line in out.splitlines() if line.count("\t") == 2
    ]


def jobs():
    """The background jobs `claude agents` lists, as dictionaries."""
    try:
        data = json.loads(run("claude", "agents", "--json") or "[]")
    except json.JSONDecodeError:
        return []
    return data if isinstance(data, list) else data.get("agents", [])


def users(root, rows):
    """The jobs whose working directory is in the workspace at `root`."""
    real = os.path.realpath(root)
    return [
        row
        for row in rows
        if row.get("cwd")
        and (os.path.realpath(row["cwd"]) + os.sep).startswith(real + os.sep)
    ]


def main():
    """Prints the report."""
    repos = [r for r in REPOS if os.path.isdir(os.path.join(r, ".jj"))]
    rows = jobs()
    roots = set()
    lines = []
    for repo in repos:
        for name, root, empty in workspaces(repo):
            if root:
                roots.add(os.path.realpath(root))
            if name == "default":
                continue
            merged = jj(
                repo,
                "log",
                "--no-graph",
                "-r",
                f'"{name}"@- & ::main@origin',
                "-T",
                '"y"',
            )
            state = "empty" if empty == "empty" else "holds work"
            on_main = (
                "its work is on main@origin"
                if merged
                else "its work is not on main@origin"
            )
            using = ", ".join(
                f"{row.get('name')} ({row.get('state')})"
                for row in users(root, rows)
                if root
            )
            in_use = f", in use by {using}" if using else ""
            lines.append(
                f"{os.path.basename(repo)}: {name} at {root or '?'}, {state}, {on_main}{in_use}"
            )
    section("Workspaces", lines)

    lines = []
    for entry in sorted(os.listdir(PARENT)):
        path = os.path.realpath(os.path.join(PARENT, entry))
        if (
            not entry.startswith(OURS)
            or not os.path.isdir(path)
            or path in roots
            or path in map(os.path.realpath, repos)
        ):
            continue
        if os.path.isfile(os.path.join(path, ".jj", "repo")):
            with open(os.path.join(path, ".jj", "repo")) as f:
                lines.append(
                    f"{entry}: a jj workspace no repository lists (it points at {f.read().strip()})"
                )
        elif SESSION_OUTPUT.match(entry) and not os.path.isdir(
            os.path.join(path, ".jj")
        ):
            lines.append(f"{entry}: session output, not a repository")
    section("Directories beside the checkouts", lines)

    lines = []
    today = datetime.date.today()
    for repo in repos:
        out = jj(
            repo,
            "log",
            "--no-graph",
            "-r",
            "heads(mutable() & ~::main@origin) ~ (working_copies() & empty())",
            "-T",
            'change_id.short() ++ "\\t" ++ committer.timestamp().format("%Y-%m-%d") ++ "\\t" ++ description.first_line() ++ "\\n"',
        )
        for line in out.splitlines():
            change, day, subject = (line.split("\t") + ["", ""])[:3]
            try:
                age = today - datetime.date.fromisoformat(day)
            except ValueError:
                continue
            if age >= STALE_AFTER:
                lines.append(
                    f"{os.path.basename(repo)}: {change} ({day}) {subject or '(no description)'}"
                )
    section("Changes off main@origin, untouched for a week", lines)

    lines = []
    for row in rows:
        if str(row.get("name", "")).startswith("deduxit-"):
            lines.append(
                f"{row.get('name')}: job {row.get('id')}, {row.get('state')}, in {row.get('cwd', '?')}"
            )
    section("Background jobs of deduxit's sessions", lines)

    out = run(
        "systemctl",
        "--user",
        "list-units",
        "--failed",
        "--no-legend",
        "--plain",
        "step*",
        "deduxit-*",
        "linlog-*",
    )
    section(
        "Failed units", [line.split()[0] for line in out.splitlines() if line.strip()]
    )


if __name__ == "__main__":
    main()
