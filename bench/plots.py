# linlog © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

"""Draws the benchmarks' figures from their CSV rows alone, reading the
columns by their names, in the light or the dark scheme:

    python3 bench/plots.py comparison [--theme T] RUNS.csv... OUTDIR
    python3 bench/plots.py families [--theme T] FAMILIES.csv ENGINES.csv OUTDIR
    python3 bench/plots.py growth [--theme T] RUNS.csv... OUTDIR

`comparison` takes the rows of `linlog-bench run --tools`: a cactus plot
per mode (the problems each tool solves within the limit against the time
each took, sorted) and a scatter plot of linlog on one thread against each
other tool on the problems both cover, both times in log scale, a run not
solved drawn at the limit. `families` takes a baseline's `families.csv`
and `engines.csv`: the time against the size of every family under the
engine the command chooses (`families.svg`), and of the families that
both the focused and the net engine search under each (`engines.svg`).
`growth` takes the rows of `run` or `run --tools` on generated families:
the time against the size of each family's shape, a line per engine (the
default and each engine forced) or per tool, a configuration that switches
a mechanism off or adds threads dashed in its engine's colour
(`growth.svg`), and beside it the growth fitted to each line
(`growth.md`): a polynomial's degree or an exponential's base, whichever
fits the logarithm of the time better by least squares.
"""

import argparse
import csv
import math
import statistics
from collections import defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("svg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.lines import Line2D  # noqa: E402
from matplotlib.ticker import NullFormatter  # noqa: E402

# The same rows give the same file: no random ids, no date.
matplotlib.rcParams["svg.hashsalt"] = "linlog"

# Each scheme's surface, ink and categorical slots, these in a fixed order
# whose neighbours stay apart under colour blindness; the surfaces are the
# manual's. A series also has a marker of its own, so that no colour
# carries it alone.
THEMES = {
    "light": {
        "surface": "#fbfcfe",
        "ink": "#121a2e",
        "muted": "#46506c",
        "axis": "#b9c1d4",
        "grid": "#e9edf4",
        "slots": [
            "#2a78d6",
            "#eb6834",
            "#1baf7a",
            "#eda100",
            "#e87ba4",
            "#008300",
            "#6250d6",
            "#e34948",
        ],
    },
    "dark": {
        "surface": "#0c1222",
        "ink": "#e3e8f4",
        "muted": "#9ba5c2",
        "axis": "#5e6a90",
        "grid": "#1b2340",
        "slots": [
            "#3987e5",
            "#d95926",
            "#199e70",
            "#c98500",
            "#d55181",
            "#008300",
            "#9085e9",
            "#e66767",
        ],
    },
}
MARKERS = ["o", "s", "^", "D", "v", "P", "X", "*"]
REFERENCE = "linlog"
# Every engine keeps its slot in every figure.
ENGINES = ["focus", "net", "horn", "additive", "two-sided", "essential", "inverse"]


def solved(row):
    """Whether a row decided its problem within the time limit and not
    wrongly, as `linlog-bench ranking` counts it."""
    if row["verdict"] not in ("proved", "unprovable") or row.get("judged") == "wrong":
        return False
    return float(row["time_ms"] or "inf") <= float(row["timeout_s"]) * 1000


def judge(rows):
    """Marks as wrong, as `linlog-bench ranking` does, a refutation of a
    problem some tool proved with a checked proof, and a verdict on a
    generated problem against its verdict by construction."""

    def key(r):
        return (r["source"], r["family"], r["problem"], r["mode"])

    checked = {
        key(r) for r in rows if r["verdict"] == "proved" and r["checked"] == "ok"
    }
    for r in rows:
        family = r["source"] == "family"
        verdict, expected = r["verdict"], r["expected"]
        wrong = (verdict == "unprovable" and key(r) in checked) or (
            family
            and r["checked"] != "ok"
            and (verdict, expected)
            in (("unprovable", "provable"), ("proved", "unprovable"))
        )
        r["judged"] = "wrong" if wrong else ""


# The verdict of a growing set's size not run, past the limit its line met
# at a smaller size: no run, drawn apart in the time against size.
SKIPPED = "skipped"


def read(files, skipped=False):
    """The rows of the CSV files, judged, without the skipped sizes unless
    asked for."""
    rows = []
    for name in files:
        with open(name, newline="") as f:
            rows.extend(csv.DictReader(f))
    rows = [r for r in rows if skipped or r["verdict"] != SKIPPED]
    judge(rows)
    return rows


def style(ax, theme, title, xlabel, ylabel, size=11):
    """A recessive frame: muted axes, a light grid, the title in ink."""
    ax.set_facecolor(theme["surface"])
    ax.set_title(title, color=theme["ink"], loc="left", fontsize=size)
    ax.set_xlabel(xlabel, color=theme["muted"], fontsize=8)
    ax.set_ylabel(ylabel, color=theme["muted"], fontsize=8)
    ax.tick_params(colors=theme["muted"], labelsize=7)
    for side in ("top", "right"):
        ax.spines[side].set_visible(False)
    for side in ("left", "bottom"):
        ax.spines[side].set_color(theme["axis"])
    ax.grid(True, color=theme["grid"], linewidth=0.6)
    ax.set_axisbelow(True)


def save(fig, out):
    """Writes the figure without a date, so that it is a function of the rows."""
    fig.savefig(out, metadata={"Date": None})
    plt.close(fig)


def cactus(rows, mode, tools, theme, out):
    """The problems of a mode each tool solves, by the time it took."""
    fig, ax = plt.subplots(figsize=(8.5, 4.5), facecolor=theme["surface"])
    style(
        ax,
        theme,
        f"Problems solved within the limit, {mode}",
        "problems solved",
        "time per problem (s, log)",
    )
    for slot, tool in enumerate(tools[: len(MARKERS)]):
        times = sorted(
            float(r["time_ms"]) / 1000
            for r in rows
            if r["tool"] == tool and r["mode"].startswith(mode) and solved(r)
        )
        if not times:
            continue
        ax.plot(
            range(1, len(times) + 1),
            times,
            color=theme["slots"][slot],
            linewidth=2,
            marker=MARKERS[slot],
            markevery=max(1, len(times) // 8),
            markersize=5,
            label=f"{tool} ({len(times)})",
        )
    ax.set_yscale("log")
    ax.legend(
        frameon=False,
        fontsize=8,
        labelcolor=theme["ink"],
        loc="upper left",
        bbox_to_anchor=(1.01, 1),
    )
    fig.tight_layout()
    save(fig, out)


def scatter(by_problem, tool, theme, out):
    """linlog's time against the tool's on every problem both cover."""
    points = []
    for runs in by_problem.values():
        a, b = runs.get(REFERENCE), runs.get(tool)
        if not a or not b or "refused" in (a["verdict"], b["verdict"]):
            continue
        limit = float(a["timeout_s"])

        def seconds(row):
            return float(row["time_ms"]) / 1000 if solved(row) else limit

        points.append((seconds(b), seconds(a), limit))
    if not points:
        return
    limit = max(p[2] for p in points)
    low = max(1e-3, min(min(p[0], p[1]) for p in points) / 2)
    fig, ax = plt.subplots(figsize=(4.8, 4.8), facecolor=theme["surface"])
    style(
        ax,
        theme,
        f"{REFERENCE} against {tool}, {len(points)} problems",
        f"{tool} (s, log; not solved at the limit)",
        f"{REFERENCE} on one thread (s, log)",
    )
    ax.plot([low, limit], [low, limit], color=theme["axis"], linewidth=1)
    # A run not solved sits on the limit's line.
    ax.axhline(limit, color=theme["axis"], linewidth=1, linestyle="--")
    ax.axvline(limit, color=theme["axis"], linewidth=1, linestyle="--")
    ax.scatter(
        [p[0] for p in points],
        [p[1] for p in points],
        s=14,
        color=theme["slots"][0],
        edgecolors=theme["surface"],
        linewidths=0.5,
        alpha=0.8,
    )
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlim(low, limit * 1.5)
    ax.set_ylim(low, limit * 1.5)
    ax.set_aspect("equal")
    fig.tight_layout()
    save(fig, out)


def tools_of(rows):
    """The tools of the rows in their order, linlog's columns first, so
    that they keep the first slots."""
    tools = []
    for r in rows:
        if r["tool"] not in tools:
            tools.append(r["tool"])
    order = list(tools)
    tools.sort(key=lambda t: (not t.startswith(REFERENCE), order.index(t)))
    return tools


def comparison(files, theme, out):
    """The cactus plots and the scatter plots of a run of the tools."""
    rows = [r for r in read(files) if r.get("tool")]
    tools = tools_of(rows)
    by_problem = defaultdict(dict)
    for r in rows:
        key = (r["source"], r["family"], r["problem"], r["mode"])
        by_problem[key].setdefault(r["tool"], r)
    for mode in ("intuitionistic", "classical"):
        covering = [
            t
            for t in tools
            if any(
                r["tool"] == t
                and r["mode"].startswith(mode)
                and r["verdict"] != "refused"
                for r in rows
            )
        ]
        if covering:
            cactus(rows, mode, covering, theme, out / f"cactus-{mode}.svg")
    for tool in tools:
        if tool != REFERENCE:
            scatter(by_problem, tool, theme, out / f"scatter-{tool}.svg")


def points(rows):
    """Per size, the median time in seconds of the problems solved and
    whether some problem of that size was not: a problem counts as solved
    when most of its runs solved it, at the median of their times, and a
    size's time is the median of its problems'."""
    runs = defaultdict(list)
    for r in rows:
        # A problem outside the engine's or the tool's reach is no point,
        # and a size not run is drawn apart.
        if r["verdict"] not in ("refused", SKIPPED):
            runs[(int(r["size"]), r["problem"])].append(r)
    by_size = defaultdict(lambda: ([], False))
    for (size, _), each in sorted(runs.items()):
        times, missed = by_size[size]
        done = [float(r["time_ms"]) / 1000 for r in each if solved(r)]
        if len(done) * 2 > len(each):
            times.append(statistics.median(done))
        else:
            missed = True
        by_size[size] = (times, missed)
    return [
        (size, statistics.median(times) if times else None, missed)
        for size, (times, missed) in sorted(by_size.items())
    ]


def growth_line(ax, rows, slot, theme, limit, dashed=False):
    """One series of time against size: a line through the sizes solved,
    and a hollow marker on the limit's line at a size not solved. Returns
    the sizes, none when the rows are all refused."""
    series = points(rows)
    if not series:
        return []
    colour = theme["slots"][slot]
    ax.plot(
        [s for s, t, _ in series if t is not None],
        [t for _, t, _ in series if t is not None],
        color=colour,
        linewidth=2,
        linestyle="--" if dashed else "-",
        marker=MARKERS[slot],
        markersize=4,
    )
    missed = [s for s, _, m in series if m]
    ax.plot(
        missed,
        [limit] * len(missed),
        linestyle="none",
        marker=MARKERS[slot],
        markersize=6,
        markerfacecolor="none",
        markeredgecolor=colour,
        markeredgewidth=1.5,
    )
    # A size not run, past the limit at a smaller one: a small cross above
    # the limit's line, apart from a run that reached it.
    skipped = sorted({int(r["size"]) for r in rows if r["verdict"] == SKIPPED})
    ax.plot(
        skipped,
        [limit * 1.8] * len(skipped),
        linestyle="none",
        marker="x",
        markersize=4,
        color=colour,
        alpha=0.6,
    )
    return [s for s, _, _ in series] + skipped


def panels(groups, theme, title, series_of, out, columns, low):
    """Small multiples of time against size, one panel per group and the
    same time scale in every panel; `series_of` gives a group's series as
    (slot, label, rows), the slot a number or (number, dashed)."""
    height = -(-len(groups) // columns)
    # The title and the legend above the panels, in inches.
    header = 0.8
    fig, axes = plt.subplots(
        height,
        columns,
        figsize=(2.3 * columns, 2.1 * height + header),
        facecolor=theme["surface"],
        squeeze=False,
    )
    limit = max(
        float(r["timeout_s"])
        for _, group in groups
        for _, _, rows in series_of(group)
        for r in rows
    )
    seen = {}
    for ax, (name, group) in zip(axes.flat, groups, strict=False):
        style(ax, theme, name, "size (log)", "time (s, log)", size=9)
        ax.axhline(limit, color=theme["axis"], linewidth=1)
        sizes = set()
        for style_of, label, rows in series_of(group):
            slot, dashed = (
                style_of if isinstance(style_of, tuple) else (style_of, False)
            )
            drawn = growth_line(ax, rows, slot, theme, limit, dashed)
            if drawn:
                sizes.update(drawn)
                seen[label] = (slot, dashed)
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_ylim(low, limit * 3)
        # A tick at every size measured, thinned where they crowd: about
        # six steps of the panel's span apart at least.
        ticks = []
        step = max(1.15, (max(sizes) / min(sizes)) ** (1 / 6)) if sizes else 1
        for size in sorted(sizes):
            if not ticks or size >= ticks[-1] * step:
                ticks.append(size)
        ax.set_xticks(ticks, [str(s) for s in ticks])
        ax.xaxis.set_minor_formatter(NullFormatter())
    for ax in list(axes.flat)[len(groups) :]:
        ax.set_visible(False)
    seen = dict(sorted(seen.items(), key=lambda item: item[1]))
    handles = [
        Line2D(
            [],
            [],
            color=theme["slots"][slot],
            marker=MARKERS[slot],
            linewidth=2,
            linestyle="--" if dashed else "-",
            markersize=4,
        )
        for slot, dashed in seen.values()
    ]
    handles.append(
        Line2D(
            [],
            [],
            color=theme["muted"],
            marker="o",
            linestyle="none",
            markerfacecolor="none",
        )
    )
    notes = [f"not decided: the {limit:g} s limit or a bound"]
    if any(
        r["verdict"] == SKIPPED
        for _, group in groups
        for _, _, rows in series_of(group)
        for r in rows
    ):
        handles.append(
            Line2D([], [], color=theme["muted"], marker="x", linestyle="none")
        )
        notes.append("not run: past the limit at a smaller size")
    fig.legend(
        handles,
        [*seen, *notes],
        frameon=False,
        fontsize=8,
        labelcolor=theme["ink"],
        loc="upper left",
        bbox_to_anchor=(0, 1 - 0.35 / fig.get_figheight()),
        ncol=3,
    )
    fig.suptitle(title, color=theme["ink"], x=0.01, ha="left", fontsize=10)
    fig.tight_layout(rect=(0, 0, 1, 1 - header / fig.get_figheight()))
    save(fig, out)


def families(families_csv, engines_csv, theme, out):
    """Time against size of every family under the engine the command
    chooses, and of the families both forced engines search."""
    by_family = defaultdict(list)
    for r in read([families_csv]):
        by_family[r["family"]].append(r)

    def chosen(rows):
        """The engine the dispatch chose for the family, which ran its rows."""
        counts = defaultdict(int)
        for r in rows:
            if r["engine"]:
                counts[r["engine"]] += 1
        engine = max(counts, key=counts.get)
        return [(ENGINES.index(engine), engine, rows)]

    panels(
        sorted(by_family.items()),
        theme,
        "Time against size, every family, under the engine the command chooses",
        chosen,
        out / "families.svg",
        columns=3,
        low=1e-5,
    )
    forced = defaultdict(lambda: defaultdict(list))
    for r in read([engines_csv]):
        if r["source"] == "family":
            forced[r["family"]][r["engine_requested"]].append(r)

    def each_engine(engines):
        """A series per engine the rows force."""
        return [
            (ENGINES.index(engine), engine, engines[engine])
            for engine in ENGINES
            if engine in engines
        ]

    panels(
        sorted(forced.items()),
        theme,
        "Time against size, the focused engine against the net engine",
        each_engine,
        out / "engines.svg",
        columns=3,
        low=1e-5,
    )


# A time under this many seconds is the run's noise more than its growth,
# and a line needs this many sizes above it to be fitted.
FLOOR = 1e-3
FITTED = 3


def series_label(r):
    """A row's series: its tool, or the engine it asked for (the default for
    `auto`) and every option that departs from a plain run, and whether
    such an option makes the line a comparison, drawn dashed."""
    if r.get("tool"):
        return r["tool"], False
    engine = "default" if r["engine_requested"] == "auto" else r["engine_requested"]
    variants = []
    if r.get("jobs", "1") != "1":
        variants.append(f"{r['jobs']} threads")
    if r.get("memo_limit") == "0":
        variants.append("memo off")
    if r.get("copies") == "none":
        variants.append("no copy bound")
    if not variants:
        return engine, False
    return f"{engine}, {', '.join(variants)}", True


def fit(series):
    """The growth fitted to a line's solved sizes over the floor: a
    polynomial n^d or an exponential b^n, whichever leaves the smaller sum
    of squares in the logarithm of the time; None with too few sizes."""
    solved = [(n, t) for n, t, _ in series if t is not None and t >= FLOOR]
    if len(solved) < FITTED:
        return None

    def least_squares(xs, ys):
        mx, my = statistics.fmean(xs), statistics.fmean(ys)
        sxx = sum((x - mx) ** 2 for x in xs)
        slope = sum((x - mx) * (y - my) for x, y in zip(xs, ys, strict=True)) / sxx
        rss = sum((y - my - slope * (x - mx)) ** 2 for x, y in zip(xs, ys, strict=True))
        return slope, rss

    logs = [math.log(t) for _, t in solved]
    degree, polynomial = least_squares([math.log(n) for n, _ in solved], logs)
    rate, exponential = least_squares([float(n) for n, _ in solved], logs)
    sizes = f"{solved[0][0]} to {solved[-1][0]}"
    if polynomial <= exponential:
        return sizes, f"n^{degree:.1f}"
    return sizes, f"{math.exp(rate):.2f}^n"


def growth(files, theme, out):
    """Time against size of each generated family's shape, a line per
    engine or tool, and the growth fitted to each line."""
    rows = [r for r in read(files, skipped=True) if r["source"] == "family"]
    tools = tools_of([r for r in rows if r.get("tool")])
    shapes = defaultdict(lambda: defaultdict(list))
    dashed = {}
    for r in rows:
        if r["verdict"] != "refused":
            label, dashed[label] = series_label(r)
            shapes[(r["family"], int(r["index"]))][label].append(r)
    groups = [
        (f"{family} #{index}", shapes[(family, index)])
        for family, index in sorted(shapes)
    ]

    def slot(label):
        """A tool's slot by its order, an engine's by `ENGINES`, the
        default last; a tool past the slots dashed."""
        if label in tools:
            i = tools.index(label)
            return (i % len(MARKERS), i >= len(MARKERS))
        engine = label.split(",")[0]
        number = ENGINES.index(engine) if engine in ENGINES else len(ENGINES)
        return (number, dashed[label])

    def each_series(by_label):
        """A series per engine, configuration or tool that ran the shape."""
        return [
            (slot(label), label, by_label[label])
            for label in sorted(by_label, key=lambda label: (slot(label), label))
        ]

    title = (
        "Time against size, each family's shape, a line per tool"
        if tools
        else "Time against size, each family's shape, a line per engine"
    )
    panels(groups, theme, title, each_series, out / "growth.svg", columns=3, low=1e-4)
    lines = [
        "| shape | line | sizes fitted | fitted growth |",
        "|---|---|---|---|",
    ]
    for name, by_label in groups:
        for _, label, series_rows in each_series(by_label):
            fitted = fit(points(series_rows))
            sizes, growth_of = fitted or (
                "—",
                f"fewer than {FITTED} sizes solved over {FLOOR * 1000:g} ms",
            )
            lines.append(f"| {name} | {label} | {sizes} | {growth_of} |")
    (out / "growth.md").write_text("\n".join(lines) + "\n")


def main():
    """Reads the arguments and draws the figures into the directory."""
    parser = argparse.ArgumentParser(
        description=__doc__.split("\n\n")[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("figures", choices=["comparison", "families", "growth"])
    parser.add_argument("--theme", choices=sorted(THEMES), default="light")
    parser.add_argument("files", nargs="+", help="the CSV files, then OUTDIR")
    args = parser.parse_args()
    *files, outdir = args.files
    if not files:
        parser.error("no CSV file")
    out = Path(outdir)
    out.mkdir(parents=True, exist_ok=True)
    theme = THEMES[args.theme]
    if args.figures == "comparison":
        comparison(files, theme, out)
    elif args.figures == "families":
        if len(files) != 2:
            parser.error("families takes FAMILIES.csv ENGINES.csv OUTDIR")
        families(*files, theme, out)
    else:
        growth(files, theme, out)


if __name__ == "__main__":
    main()
