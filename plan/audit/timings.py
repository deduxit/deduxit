#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Compile and test times of the workspace, from an empty target directory.

For step 86's audit (plan/notes/audit/baselines.md, the build's cost).
Builds into TARGET_DIR, which it empties first, and prints Markdown: each
command with its wall time and the CPU time of its processes, then the
units that took longest to compile. Run it in a capped scope on the cores
the numbers are to be compared on (`stage0.sh timings`), with nothing
else on them: the times are of those cores and that job count.

usage: timings.py REPOSITORY_ROOT TARGET_DIR
"""

import json
import os
import shutil
import subprocess
import sys
import time


def run(root, target, log, command):
    """Runs a cargo command; returns its wall seconds, CPU seconds and status."""
    env = dict(os.environ, CARGO_TARGET_DIR=target)
    before = os.times()
    start = time.monotonic()
    with open(log, "a", encoding="utf-8") as f:
        f.write(f"\n== {' '.join(command)}\n")
        f.flush()
        status = subprocess.run(
            command, cwd=root, env=env, stdout=f, stderr=f, check=False
        ).returncode
    wall = time.monotonic() - start
    after = os.times()
    cpu = (after.children_user - before.children_user) + (
        after.children_system - before.children_system
    )
    return wall, cpu, status


def longest_units(target, count, report=None):
    """Returns the units of a `--timings` report that took longest: the
    last one's, or those of the report named."""
    report = report or os.path.join(target, "cargo-timings", "cargo-timing.html")
    marker = "const UNIT_DATA = "
    try:
        with open(report, encoding="utf-8") as f:
            text = f.read()
        units, _ = json.JSONDecoder().raw_decode(text, text.index(marker) + len(marker))
    except (OSError, ValueError):
        return []
    units.sort(key=lambda unit: -unit.get("duration", 0))
    return [
        (unit.get("name", "?"), unit.get("target", "").strip(), unit.get("duration", 0))
        for unit in units[:count]
    ]


def main():
    root, target = sys.argv[1], sys.argv[2]
    shutil.rmtree(target, ignore_errors=True)
    os.makedirs(target)
    log = os.path.join(target, "timings.log")
    lib = os.path.join(root, "core", "src", "lib.rs")
    bare = os.path.join(target, "core-default")
    none = os.path.join(target, "core-no-default")
    rows = []
    reports = {}

    def step(what, command, directory=target):
        wall, cpu, status = run(root, directory, log, command)
        rows.append((what, " ".join(command), wall, cpu, status))

    step(
        "the workspace, dev profile",
        ["cargo", "build", "--workspace", "--locked", "--timings"],
    )
    reports["the workspace, dev profile"] = longest_units(target, 12)

    # The core crate's root is touched and its time put back after, so that
    # no other target directory sees a change.
    kept = os.stat(lib)
    os.utime(lib, None)
    try:
        step(
            "again after the core crate's root changed",
            ["cargo", "build", "--workspace", "--locked"],
        )
    finally:
        os.utime(lib, ns=(kept.st_atime_ns, kept.st_mtime_ns))

    step("the tests built", ["cargo", "test", "--workspace", "--locked", "--no-run"])
    step("the tests run", ["cargo", "test", "--workspace", "--locked"])
    step(
        "the command, release profile",
        ["cargo", "build", "--release", "--locked", "--package", "deduxit-cli"]
        + ["--timings"],
    )
    reports["the command, release profile"] = longest_units(target, 12)
    step(
        "the core crate alone, dev profile, default features",
        ["cargo", "build", "--locked", "--package", "deduxit"],
        bare,
    )
    step(
        "the core crate alone, dev profile, no default features",
        ["cargo", "build", "--locked", "--package", "deduxit"]
        + ["--no-default-features"],
        none,
    )

    print("# Compile and test times\n")
    print(
        f"Cores {sorted(os.sched_getaffinity(0))},"
        f" `CARGO_BUILD_JOBS={os.environ.get('CARGO_BUILD_JOBS', 'unset')}`,"
        " each build from an empty target directory but the second.\n"
    )
    print("| what | command | wall s | CPU s | status |\n|---|---|---:|---:|---:|")
    for what, command, wall, cpu, status in rows:
        print(f"| {what} | `{command}` | {wall:.0f} | {cpu:.0f} | {status} |")
    for what, units in reports.items():
        if not units:
            continue
        print(f"\n## The units that took longest: {what}\n")
        print("| unit | target | s |\n|---|---|---:|")
        for name, kind, duration in units:
            print(f"| {name} | {kind} | {duration:.1f} |")


main()
