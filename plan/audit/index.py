#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""One table of every unit's findings with their verdicts.

For step 86's audit (plan/notes/audit/method.md, the synthesis). Reads
each UNIT/findings.md and UNIT/verdicts.md under UNITS_DIR, in the
layout of `plan/notes/audit/brief.md`, and prints Markdown: the counts
by verdict, dimension, kind and criterion, then every finding as a row
(id, verdict, dimension, criterion, kind, cost, place, claim), the
confirmed ones first within a unit's order.

usage: index.py UNITS_DIR
"""

import os
import re
import sys
from collections import Counter

HEADING = re.compile(r"^## (F-[A-Za-z0-9]+-\d+)(?::\s*(\w+))?")
FIELD = re.compile(r"^- (dimension|criterion|place|kind|cost|candidate): (.*)$")
CLAIM = re.compile(r"^\*\*Claim\.\*\*\s*(.*)$")


def read_findings(path):
    """Returns a unit's findings as dictionaries, in the file's order."""
    findings, current, in_claim = [], None, False
    with open(path, encoding="utf-8") as f:
        for line in f:
            heading = HEADING.match(line)
            if heading:
                current = {"id": heading.group(1), "claim": ""}
                findings.append(current)
                in_claim = False
            elif current is not None:
                field = FIELD.match(line)
                claim = CLAIM.match(line)
                if field:
                    current[field.group(1)] = field.group(2).strip()
                elif claim:
                    current["claim"] = claim.group(1).strip()
                    in_claim = True
                elif in_claim and line.strip() and not line.startswith("**"):
                    current["claim"] += " " + line.strip()
                else:
                    in_claim = False
    return findings


def read_verdicts(path):
    """Returns a unit's verdicts by finding id."""
    verdicts = {}
    try:
        with open(path, encoding="utf-8") as f:
            for line in f:
                heading = HEADING.match(line)
                if heading and heading.group(2):
                    verdicts[heading.group(1)] = heading.group(2).lower()
    except OSError:
        pass
    return verdicts


def main():
    units = sys.argv[1]
    rows = []
    for unit in sorted(os.listdir(units)):
        path = os.path.join(units, unit, "findings.md")
        if not os.path.isfile(path):
            continue
        verdicts = read_verdicts(os.path.join(units, unit, "verdicts.md"))
        for finding in read_findings(path):
            finding["unit"] = unit
            finding["verdict"] = verdicts.get(finding["id"], "unverified")
            rows.append(finding)
    print("# Every finding of the audit, with its verdict\n")
    print(f"{len(rows)} findings in {len({row['unit'] for row in rows})} units.\n")
    for title, key in (
        ("verdict", "verdict"),
        ("dimension", "dimension"),
        ("kind", "kind"),
        ("cost", "cost"),
    ):
        counts = Counter(row.get(key, "?") for row in rows)
        print(
            f"By {title}: "
            + ", ".join(f"{name} {n}" for name, n in counts.most_common())
            + ".\n"
        )
    by_unit = Counter()
    kept = Counter()
    for row in rows:
        by_unit[row["unit"]] += 1
        kept[row["unit"]] += row["verdict"] != "rejected"
    print("| unit | findings | not rejected |\n|---|---:|---:|")
    for unit in sorted(by_unit):
        print(f"| {unit} | {by_unit[unit]} | {kept[unit]} |")
    criteria = Counter(
        row.get("criterion", "?") for row in rows if row["verdict"] != "rejected"
    )
    print(
        "\nBy criterion (not rejected): "
        + ", ".join(f"{c} {n}" for c, n in criteria.most_common())
        + ".\n"
    )
    print("| id | verdict | dimension | criterion | kind | cost | place | claim |")
    print("|---|---|---|---|---|---|---|---|")
    for row in rows:
        claim = row["claim"].replace("|", "\\|")
        place = row.get("place", "").replace("|", "\\|")
        print(
            f"| {row['id']} | {row['verdict']} | {row.get('dimension', '')} |"
            f" {row.get('criterion', '')} | {row.get('kind', '')} | {row.get('cost', '')} |"
            f" `{place}` | {claim} |"
        )


main()
