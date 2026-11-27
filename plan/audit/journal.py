#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Writes each unit's findings and verdicts from a workflow's journal.

For step 86's audit (plan/notes/audit/method.md). A finder returns its
findings and a verifier its verdicts through a schema; the workflow's
`journal.jsonl` keeps each agent's return value. This writes
UNITS_DIR/UNIT/findings.md and verdicts.md from them in the layout of
`plan/notes/audit/brief.md`, leaving a file that is there already, and
prints one line per unit: its findings by verdict.

usage: journal.py UNITS_DIR TRANSCRIPT_DIR...
"""

import json
import os
import sys
from collections import Counter


def results(directory):
    """Yields the return value of every agent of a run that finished."""
    with open(os.path.join(directory, "journal.jsonl"), encoding="utf-8") as f:
        for line in f:
            try:
                record = json.loads(line)
            except ValueError:
                continue
            if record.get("type") == "result" and isinstance(
                record.get("result"), dict
            ):
                yield record["result"]


def findings_text(result):
    """Returns a finder's result as the Markdown of `findings.md`."""
    unread = ", ".join(result.get("filesNotReadInFull") or []) or "none"
    parts = [
        f"# Unit {result['unit']}: findings\n",
        f"Read in full: {result.get('filesReadInFull', '?')} files."
        f" Not read in full: {unread}.\n",
    ]
    if result.get("remarks"):
        parts.append(f"The finder's remarks: {result['remarks']}\n")
    for finding in result["findings"]:
        parts.append(
            f"## {finding['id']}\n\n"
            f"- dimension: {finding['dimension']}\n"
            f"- criterion: {finding['criterion']}\n"
            f"- place: {finding['place']}\n"
            f"- kind: {finding['kind']}\n"
            f"- cost: {finding['cost']}\n"
            f"- candidate: {'yes' if finding.get('candidate') else 'no'}\n\n"
            f"**Claim.** {finding['claim']}\n\n"
            f"**Witness.** {finding['witness']}\n\n"
            f"**Change.** {finding['change']}\n\n"
            f"**Value.** {finding['value']}\n\n"
            f"**Net.** {finding['net']}\n"
        )
    return "\n".join(parts)


def verdicts_text(result):
    """Returns a verifier's result as the Markdown of `verdicts.md`."""
    counts = Counter(verdict["verdict"] for verdict in result["verdicts"])
    summary = ", ".join(f"{n} {name}" for name, n in counts.most_common())
    parts = [f"# Unit {result['unit']}: verdicts\n", f"{summary}.\n"]
    for verdict in result["verdicts"]:
        parts.append(
            f"## {verdict['id']}: {verdict['verdict']}\n\n{verdict['reason']}\n"
        )
        if verdict.get("correction"):
            parts.append(f"Correction: {verdict['correction']}\n")
    if result.get("beside"):
        parts.append("## Beside the findings\n")
        parts += [f"- {item}" for item in result["beside"]]
        parts.append("")
    return "\n".join(parts)


def write(path, text):
    """Writes a file unless it is there; says whether it wrote."""
    if os.path.exists(path):
        return False
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)
    return True


def main():
    units = sys.argv[1]
    found, judged = {}, {}
    for directory in sys.argv[2:]:
        for result in results(directory):
            if "verdicts" in result:
                judged[result["unit"]] = result
            elif "findings" in result:
                found[result["unit"]] = result
    for unit, result in sorted(found.items()):
        full = all("witness" in finding for finding in result["findings"])
        wrote = full and write(
            os.path.join(units, unit, "findings.md"), findings_text(result)
        )
        verdicts = judged.get(unit)
        counts = Counter()
        if verdicts:
            write(os.path.join(units, unit, "verdicts.md"), verdicts_text(verdicts))
            counts = Counter(verdict["verdict"] for verdict in verdicts["verdicts"])
        print(
            f"{unit}: {len(result['findings'])} findings"
            f" ({counts['confirmed']} confirmed, {counts['candidate']} candidates,"
            f" {counts['rejected']} rejected{'' if verdicts else ', not verified'})"
            f"{'; findings.md written' if wrote else ''}"
        )


main()
