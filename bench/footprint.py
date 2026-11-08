#!/usr/bin/env python3
# linlog © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The workspace's footprint as text tables: the web module's bytes by crate,
# and a build's time by crate.
#
#   wasm TWIGGY.csv
#     The module's bytes by crate and linlog's by module, from
#     `twiggy top -n 1000000 --format csv` of a module that keeps its names.
#     The package strips them, so build it again with them kept, in a
#     target directory of its own:
#       cargo build --locked --release --target wasm32-unknown-unknown \
#         --package linlog-web --target-dir DIR
#       wasm-bindgen --target web --out-dir OUT \
#         DIR/wasm32-unknown-unknown/release/linlog_web.wasm
#       wasm-opt -O3 -g OUT/linlog_web_bg.wasm -o OUT/linlog_web_bg.wasm
#     An item is the crate its name names first (`<A as B>::f` is A's), so a
#     generic of `core` or `alloc` instantiated for linlog counts as theirs;
#     data segments and the bindings' shims have no crate.
#
#   timings REPORT.html...
#     Each crate's compile time and share, from the reports
#     `cargo build --timings` writes under target/cargo-timings/. A crate's
#     time is the sum of its units' (its library, binaries, build script and
#     the script's run), which overlap when jobs run in parallel: the shares
#     are of the summed time, not of the wall time the report's header gives.
import collections
import csv
import json
import re
import sys

# The crates of the PNG and PDF renderers: resvg and krilla with the
# libraries they reach in the web module's build (fonts, shaping, raster
# decoders, compression).
RENDERING = {
    "adler2",
    "arrayref",
    "arrayvec",
    "base64",
    "bytemuck",
    "core_maths",
    "crc32fast",
    "data_url",
    "euclid",
    "fdeflate",
    "flate2",
    "float_cmp",
    "font_types",
    "fontdb",
    "gif",
    "image_webp",
    "imagesize",
    "krilla",
    "krilla_svg",
    "kurbo",
    "libm",
    "miniz_oxide",
    "png",
    "read_fonts",
    "resvg",
    "roxmltree",
    "rustybuzz",
    "simd_adler32",
    "simplecss",
    "skrifa",
    "smallvec",
    "strict_num",
    "subsetter",
    "svgtypes",
    "tiny_skia",
    "tiny_skia_path",
    "tinyvec",
    "ttf_parser",
    "unicode_bidi",
    "unicode_bidi_mirroring",
    "unicode_ccc",
    "unicode_properties",
    "unicode_script",
    "unicode_vo",
    "usvg",
    "weezl",
    "write_fonts",
    "xmlwriter",
    "xmp_writer",
    "zune_core",
    "zune_inflate",
    "zune_jpeg",
}

# linlog's modules shown with their submodule, where that is the unit a
# question about the module's size is asked of.
SPLIT = {"search", "export", "proofs"}


def table(title, counts, total, limit):
    """Prints the largest entries of `counts` with their share of `total`."""
    print(f"{title:44} {'bytes':>10} {'share':>7}")
    shown = counts.most_common(limit)
    for name, size in shown:
        print(f"{name:44} {size:10d} {100 * size / total:6.1f}%")
    rest = total - sum(size for _, size in shown)
    if rest:
        print(f"{'(the rest)':44} {rest:10d} {100 * rest / total:6.1f}%")
    print()


def wasm(path):
    """The module's bytes by crate, by group and linlog's by module."""
    crates = collections.Counter()
    modules = collections.Counter()
    names = 0
    for row in csv.DictReader(open(path, encoding="utf-8")):
        name, size = row["Name"], int(row["ShallowSize"])
        if name.startswith(("Σ", "... and")):
            continue
        if "subsection" in name:
            names += size
            continue
        if name.startswith("data segment"):
            crates["(data segments)"] += size
            continue
        crate = re.search(r"([A-Za-z_]\w*)\[[0-9a-f]{16}\]", name)
        crates[crate.group(1) if crate else "(bindings' shims, no crate)"] += size
        if crate and crate.group(1) == "linlog":
            path_ = re.search(r"linlog\[[0-9a-f]{16}\]::(\w+)(?:::(\w+))?", name)
            if path_ is None:
                modules["(no module)"] += size
            elif path_.group(1) in SPLIT and path_.group(2):
                modules[f"{path_.group(1)}::{path_.group(2)}"] += size
            else:
                modules[path_.group(1)] += size
    total = sum(crates.values())
    print(f"code and data {total} B; the name section, {names} B, left out\n")
    groups = collections.Counter()
    for crate, size in crates.items():
        if crate in RENDERING:
            groups["the PNG and PDF renderers' crates"] += size
        elif crate in ("linlog", "linlog_web"):
            groups["linlog and linlog-web"] += size
        elif crate in ("core", "alloc", "std", "compiler_builtins", "dlmalloc"):
            groups["the standard library"] += size
        else:
            groups[crate if crate.startswith("(") else "the other crates"] += size
    table("group", groups, total, 20)
    table("crate", crates, total, 30)
    table("linlog's module", modules, total, 30)


def timings(paths):
    """Each report's crates by summed unit time."""
    for path in paths:
        html = open(path, encoding="utf-8").read()
        units = json.loads(
            re.search(r"const UNIT_DATA = (\[.*?\]);\n", html, re.S).group(1)
        )
        total_time = re.search(r"Total time:</td><td>([^<]*)", html).group(1)
        crates = collections.Counter()
        for unit in units:
            crates[unit["name"]] += unit["duration"]
        summed = sum(crates.values())
        print(f"{path}: {len(units)} units, wall {total_time}, summed {summed:.1f}s")
        print(f"{'crate':32} {'seconds':>8} {'share':>7}")
        for name, seconds in crates.most_common(15):
            print(f"{name:32} {seconds:8.1f} {100 * seconds / summed:6.1f}%")
        rest = summed - sum(s for _, s in crates.most_common(15))
        print(f"{'(the other crates)':32} {rest:8.1f} {100 * rest / summed:6.1f}%\n")


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "wasm":
        wasm(sys.argv[2])
    elif len(sys.argv) >= 3 and sys.argv[1] == "timings":
        timings(sys.argv[2:])
    else:
        sys.exit("usage: footprint.py wasm TWIGGY.csv | timings REPORT.html...")
