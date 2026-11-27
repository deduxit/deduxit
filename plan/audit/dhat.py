#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Each journey's allocations, from DHAT's profiles.

For step 86's audit (plan/notes/audit/method.md, the inventories). Reads
the `JOURNEY.json` files `stage0.sh dhat` wrote and prints Markdown: per
journey the bytes and blocks allocated and the instructions run, then the
places that allocate most blocks, each named by its innermost frame in
deduxit's own code.

usage: dhat.py DHAT_DIR
"""

import glob
import json
import os
import re
import sys
from collections import defaultdict

FRAME = re.compile(r"^0x[0-9A-Fa-f]+: (.*)$")
SITES = 8


def own_frame(profile, point):
    """Returns the innermost frame of a program point in deduxit's code."""
    for index in point["fs"]:
        found = FRAME.match(profile["ftbl"][index])
        text = found.group(1) if found else profile["ftbl"][index]
        if "deduxit" in text:
            return text
    return "(no frame of deduxit)"


def main():
    directory = sys.argv[1]
    rows = []
    details = []
    for path in sorted(glob.glob(os.path.join(directory, "*.json"))):
        journey = os.path.basename(path)[: -len(".json")]
        with open(path, encoding="utf-8") as f:
            profile = json.load(f)
        points = profile["pps"]
        total_bytes = sum(p["tb"] for p in points)
        blocks = sum(p["tbk"] for p in points)
        rows.append((journey, total_bytes, blocks, profile["te"]))
        by_frame = defaultdict(lambda: [0, 0])
        for point in points:
            entry = by_frame[own_frame(profile, point)]
            entry[0] += point["tbk"]
            entry[1] += point["tb"]
        top = sorted(by_frame.items(), key=lambda kv: -kv[1][0])[:SITES]
        details.append((journey, blocks, top))
    print("# Each journey's allocations (DHAT)\n")
    print(
        "| journey | bytes allocated | blocks | instructions | instructions per block |"
    )
    print("|---|---:|---:|---:|---:|")
    for journey, total_bytes, blocks, instructions in rows:
        per = instructions // blocks if blocks else 0
        print(f"| {journey} | {total_bytes} | {blocks} | {instructions} | {per} |")
    for journey, blocks, top in details:
        print(f"\n## {journey}: the frames of deduxit that allocate most blocks\n")
        print("| blocks | share | bytes | frame |\n|---:|---:|---:|---|")
        for frame, (count, size) in top:
            share = 100 * count // blocks if blocks else 0
            print(f"| {count} | {share} % | {size} | `{frame[:220]}` |")


main()
