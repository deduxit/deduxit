#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""The audit's findings by theme, from `themes.json` and the units' files.

For step 86's audit (plan/notes/audit/method.md, the synthesis).
`themes.json` beside this file groups the findings into themes, in the
order of their rank, each with what it is, the change and the net. This
checks that every finding a verifier did not reject is in exactly one
theme, and prints the body of `findings.md`: each group, each theme, and
under it its findings with verdict, kind, cost, place and claim. With
`--check` it prints only what is missing, doubled or unknown.

usage: themes.py UNITS_DIR [--check]
"""

import json
import os
import sys

sys.path.insert(0, os.path.dirname(__file__))


def load_index():
    """Returns the functions of index.py without running its main."""
    path = os.path.join(os.path.dirname(__file__), "index.py")
    with open(path, encoding="utf-8") as f:
        source = f.read().rsplit("\nmain()", 1)[0]
    scope = {}
    exec(compile(source, path, "exec"), scope)  # noqa: S102
    return scope["read_findings"], scope["read_verdicts"]


def main():
    units = sys.argv[1]
    check_only = "--check" in sys.argv[2:]
    read_findings, read_verdicts = load_index()
    findings = {}
    for unit in sorted(os.listdir(units)):
        path = os.path.join(units, unit, "findings.md")
        if not os.path.isfile(path):
            continue
        verdicts = read_verdicts(os.path.join(units, unit, "verdicts.md"))
        for finding in read_findings(path):
            finding["unit"] = unit
            finding["verdict"] = verdicts.get(finding["id"], "unverified")
            findings[finding["id"]] = finding
    with open(
        os.path.join(os.path.dirname(__file__), "themes.json"), encoding="utf-8"
    ) as f:
        data = json.load(f)

    placed = {}
    problems = []
    for theme in data["themes"]:
        for ident in theme["ids"]:
            if ident not in findings:
                problems.append(f"{theme['id']} names {ident}, which no unit has")
            elif findings[ident]["verdict"] == "rejected":
                problems.append(f"{theme['id']} holds {ident}, which was rejected")
            elif ident in placed:
                problems.append(f"{ident} is in {placed[ident]} and in {theme['id']}")
            else:
                placed[ident] = theme["id"]
    for ident, finding in findings.items():
        if finding["verdict"] != "rejected" and ident not in placed:
            problems.append(f"{ident} is in no theme: {finding['claim'][:110]}")
    if problems:
        print("\n".join(problems), file=sys.stderr)
    if check_only:
        print(
            f"{len(placed)} findings in {len(data['themes'])} themes; {len(problems)} problems"
        )
        return

    for group in data["groups"]:
        themes = [theme for theme in data["themes"] if theme["group"] == group["id"]]
        count = sum(len(theme["ids"]) for theme in themes)
        print(f"## {group['id']}. {group['title']}\n")
        print(f"{group['intro']} ({count} findings in {len(themes)} themes.)\n")
        for theme in themes:
            members = [findings[i] for i in theme["ids"] if i in findings]
            confirmed = sum(1 for m in members if m["verdict"] == "confirmed")
            kinds = sorted({m.get("kind", "") for m in members})
            print(f"### {theme['id']}. {theme['title']}\n")
            print(
                f"{len(members)} findings ({confirmed} confirmed,"
                f" {len(members) - confirmed} candidates); kinds: {', '.join(kinds)}.\n"
            )
            print(f"**What.** {theme['what']}\n")
            print(f"**Change.** {theme['change']}\n")
            print(f"**Net.** {theme['net']}\n")
            for member in members:
                print(
                    f"- {member['id']} ({member['verdict']}, {member.get('kind', '')},"
                    f" {member.get('cost', '')}), `{member.get('place', '')}`:"
                    f" {member['claim']}"
                )
            print()


main()
