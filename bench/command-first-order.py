#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Runs the command on every ILTP first-order problem in classical and
intuitionistic logic and writes one row per run: the problem, the logic,
the exit status, the first line of the answer and the seconds it took.

command-first-order.py BIN PROBLEMS OUT.csv CORES

BIN is a deduxit binary, PROBLEMS the problems' directory
(bench/iltp-first-order/Problems), CORES the cores to pin one run each to
(4,5,6). A second start skips the rows OUT.csv holds. Each run is
`prove --input-format tptp --logic L --jobs 1 --timeout 2s --quiet
--file P`: the command's own decision of a first-order sequent on one
thread, which bench/defaults/command-first-order-before.csv holds for the
build before the library's default replaced it.
"""

import csv, os, subprocess, sys, time
from concurrent.futures import ThreadPoolExecutor

binary, problems, out, cores = (
    sys.argv[1],
    sys.argv[2],
    sys.argv[3],
    sys.argv[4].split(","),
)
files = sorted(
    os.path.join(d, f)
    for d, _, fs in os.walk(problems, followlinks=True)
    for f in fs
    if f.endswith(".p")
)
jobs = [(f, logic) for logic in ("classical", "intuitionistic") for f in files]
done = set()
if os.path.exists(out):
    with open(out) as handle:
        for row in csv.reader(handle):
            done.add((row[0], row[1]))
jobs = [j for j in jobs if (os.path.relpath(j[0], problems), j[1]) not in done]
print(f"{len(jobs)} runs left", flush=True)
free = list(cores)


def run(job):
    path, logic = job
    core = free.pop()
    try:
        start = time.monotonic()
        try:
            result = subprocess.run(
                [
                    "taskset",
                    "-c",
                    core,
                    binary,
                    "prove",
                    "--input-format",
                    "tptp",
                    "--logic",
                    logic,
                    "--jobs",
                    "1",
                    "--timeout",
                    "2s",
                    "--quiet",
                    "--file",
                    path,
                ],
                capture_output=True,
                text=True,
                timeout=60,
            )
            status, text = (
                result.returncode,
                (result.stdout or result.stderr).splitlines(),
            )
            first = text[0] if text else ""
        except subprocess.TimeoutExpired:
            status, first = -1, "killed after 60 s"
        elapsed = time.monotonic() - start
    finally:
        free.append(core)
    return (
        os.path.relpath(path, problems),
        logic,
        status,
        first.replace(",", ";")[:200],
        f"{elapsed:.2f}",
    )


with open(out, "a", newline="") as handle:
    writer = csv.writer(handle)
    with ThreadPoolExecutor(len(cores)) as pool:
        for n, row in enumerate(pool.map(run, jobs), 1):
            writer.writerow(row)
            handle.flush()
            if n % 200 == 0:
                print(f"{n}/{len(jobs)}", flush=True)
