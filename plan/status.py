# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""Prints the plan's status as Markdown: the steps done in the last hours,
the ones running, and the ones planned next, each with what it depends
on, what it does, its model, effort and agents, and its time.

    python3 -I plan/status.py [--hours 36] [--planned 12] [--now "YYYY-MM-DD HH:MM"]

Reads plan/README.md's table of steps, plan/notes/step-log.tsv (one row
per session: started and finished, which the planning session keeps)
and the rules of thumb of plan/notes/step-durations.md. The running
rows' ETAs are those rules minus the time spent; the planned rows' are
the sessions the table names times the hours of their kind. Times are
estimates: the planning session corrects them from the live sessions.
"""

import argparse
import csv
import datetime as dt
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
# Median hours of one session of each kind (plan/notes/step-durations.md,
# "Rules of thumb"); a kind's range is said beside its median there.
HOURS = {
    "text": 0.8,
    "small": 0.8,
    "client": 1.05,
    "panel": 2.2,
    "large": 5.0,
    "design": 8.5,
    "release": 1.5,
    "run": 1.5,
}
WORDS = {"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6}
FMT = "%Y-%m-%d %H:%M"


def table(text):
    """The step table's rows: step, what, prompt, models, effort, deps, release."""
    rows, release = [], ""
    for line in text.splitlines():
        if not line.startswith("| "):
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) < 6:
            continue
        if cells[0] == "" and cells[1].startswith("**Release"):
            release = cells[1].strip("*").split(":")[0].replace("**", "")
            continue
        if not re.fullmatch(r"\d+[a-z]?", cells[0]):
            continue
        rows.append(
            {
                "step": cells[0],
                "what": cells[1],
                "models": cells[3],
                "effort": cells[4],
                "deps": cells[5],
                "release": release,
            }
        )
    return rows


def sessions_of(what):
    """The number of sessions a row names, the larger of a range."""
    m = re.search(
        r"\((?:[^()]*?)(one|two|three|four|five|six)(?: to (one|two|three|four|five|six))? sessions?",
        what,
    )
    if not m:
        return 1
    return WORDS[m.group(2) or m.group(1)]


def kind_of(row):
    """The kind of a planned step's sessions, for its hours."""
    what, models, effort = (
        row["what"].lower(),
        row["models"].lower(),
        row["effort"].lower(),
    )
    if "release of" in what:
        return "release"
    if "panel" in effort:
        return "panel"
    if "in the client" in what or "on the page" in what and "sessions" not in what:
        return "client"
    return "small"


def first(text, limit=160):
    """The row's text cut to its first clause, for a line of the overview."""
    text = re.sub(r"\s*\((?:[^()]*sessions?[^()]*)\)\s*$", "", text)
    return text if len(text) <= limit else text[: limit - 1].rsplit(" ", 1)[0] + "…"


def hours(a, b):
    return (b - a).total_seconds() / 3600


def fmt_h(h):
    return f"{h:.1f} h" if h >= 1 else f"{round(h * 60)} min"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--hours", type=float, default=36)
    ap.add_argument("--planned", type=int, default=12)
    ap.add_argument("--now")
    a = ap.parse_args()
    now = (
        dt.datetime.strptime(a.now, FMT)
        if a.now
        else dt.datetime.now().replace(second=0, microsecond=0)
    )

    rows = table((ROOT / "plan/README.md").read_text())
    by_step = {r["step"]: r for r in rows}
    log = list(
        csv.DictReader((ROOT / "plan/notes/step-log.tsv").open(), delimiter="\t")
    )

    def base(step):
        m = re.match(r"(\d+[a-z]?)", step)
        s = m.group(1) if m else step
        return s if s in by_step else re.sub(r"[a-z]$", "", s)

    out = [f"# Status at {now.strftime(FMT)}", ""]
    # A row of kind "step" marks a whole step complete; the others are sessions.
    complete = {e["step"] for e in log if e["kind"] == "step"}
    log = [e for e in log if e["kind"] != "step"]
    done = [
        e
        for e in log
        if e["finished"]
        and hours(dt.datetime.strptime(e["finished"], FMT), now) <= a.hours
    ]
    running = [e for e in log if not e["finished"]]

    out += [
        f"## Done in the last {a.hours:g} hours",
        "",
        "| step | what | settings | started | finished | took |",
        "|---|---|---|---|---|---|",
    ]
    for e in sorted(done, key=lambda e: e["finished"]):
        s, f = (
            dt.datetime.strptime(e["started"], FMT),
            dt.datetime.strptime(e["finished"], FMT),
        )
        out.append(
            f"| {e['step']} | {e['note']} | {e['model']} {e['effort']}, {e['agents']} | {s:%a %H:%M} | {f:%a %H:%M} | {fmt_h(hours(s, f))} |"
        )

    out += [
        "",
        "## Running",
        "",
        "| step | what | settings | started | so far | typical | left |",
        "|---|---|---|---|---|---|---|",
    ]
    for e in running:
        s = dt.datetime.strptime(e["started"], FMT)
        spent, typical = hours(s, now), HOURS.get(e["kind"], 1.0)
        left = max(typical - spent, 0.25)
        out.append(
            f"| {e['step']} | {e['note']} | {e['model']} {e['effort']}, {e['agents']} | {s:%a %H:%M} | {fmt_h(spent)} | {fmt_h(typical)} ({e['kind']}) | about {fmt_h(left)} |"
        )

    touched = {base(e["step"]) for e in log}
    started = {base(e["step"]) for e in log}
    finished_steps = set(complete)
    for r in rows:
        subs = [e for e in log if base(e["step"]) == r["step"]]
        if (
            subs
            and all(e["finished"] for e in subs)
            and len(subs) >= sessions_of(r["what"])
        ):
            finished_steps.add(r["step"])
    order = [r for r in rows if r["step"] not in finished_steps]
    # From the first step not finished that the log or the table's order reaches.
    first_open = next(
        (
            i
            for i, r in enumerate(rows)
            if r["step"] in touched and r["step"] not in finished_steps
        ),
        None,
    )
    if first_open is None:
        first_open = next(
            (i for i, r in enumerate(rows) if r["step"] not in finished_steps),
            len(rows),
        )
    planned = [r for r in rows[first_open:] if r["step"] not in finished_steps][
        : a.planned
    ]

    out += [
        "",
        "## Planned next (in the table's order)",
        "",
        "| step | release | depends on | what | settings | sessions left | estimate |",
        "|---|---|---|---|---|---|---|",
    ]
    for r in planned:
        n = sessions_of(r["what"])
        done_n = len([e for e in log if base(e["step"]) == r["step"] and e["finished"]])
        run_n = len(
            [e for e in log if base(e["step"]) == r["step"] and not e["finished"]]
        )
        left = max(n - done_n - run_n, 0)
        k = kind_of(r)
        est = (
            HOURS["design"] + (left - 1) * HOURS[k]
            if "first session" in r["models"].lower()
            and "fable" in r["models"].lower()
            and done_n == 0
            and run_n == 0
            else left * HOURS[k]
        )
        settings = f"{r['models']}; {r['effort']}"
        out.append(
            f"| {r['step']} | {r['release']} | {r['deps']} | {first(r['what'])} | {settings} | {left} of {n} | {fmt_h(est) if left else 'running'} |"
        )
    out.append("")
    sys.stdout.write("\n".join(out) + "\n")


if __name__ == "__main__":
    main()
