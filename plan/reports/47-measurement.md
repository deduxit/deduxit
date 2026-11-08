# Step 47: the measurement before the collection grows

## Outcome

The costs a growing workspace will move are measured on `main` at
`utzkqlzy` (0.2.0 and the plan commits after it, no code changed since
the release), each with the command that repeats it:

- **The web module** `linlog_web_bg.wasm` is 6 913 020 bytes, 2 603 759
  at `gzip -9` and 1 854 124 at `brotli -q 11`. At least 39.9 % of it is
  the PNG and PDF renderers' crates (resvg, usvg, krilla, their font,
  shaping, raster and compression crates), 27.2 % linlog and linlog-web,
  14.2 % the standard library, 10.2 % data segments; the ordinary suite
  (`linlog::ordinary`) is 4 643 bytes.
- **The web client's critical path** is 120 230 bytes at `gzip -9`
  (102 360 at brotli), under the design's budget of about 170 KB (D35);
  the module loads after first paint, in the worker.
- **The build** of the workspace (`cargo build --workspace --timings`,
  six jobs on cores 4–9) takes 28.9 s clean and 3.0 s after touching
  `core/src/ordinary/translate.rs` in the dev profile, and 137.4 s clean
  and 116.3 s touched in the release profile, where fat link-time
  optimisation relinks every binary and `linlog-cli`'s link alone takes
  66.5 s. `target/` holds 2.36, 2.64, 3.12 and 3.12 GB after each.
- **The ratchet** passes on the head: all 20 journeys within the 2 %
  tolerance of `bench/ceilings.csv`, the closest `derivation-chain-64`
  at +1.95 % and `search-wide-m2-256` at +1.76 %.
- **The target sets** `47` and `net-47` have every verdict of `45d` and
  `net-45d` (the release's) and no counter differing on any of the 145
  and 64 rows decided alike on one thread; they are now the oracles
  (`bench/TARGETS.md`, CLAUDE.md's table, `bench.md`). The focused set
  ran in 14 s and the net set in 9 s.

`bench/footprint.py` (new) turns twiggy's CSV and cargo's timing reports
into the text tables below. Nothing is pushed.

## The machine

An Intel Core Ultra X9 388H, 16 cores without SMT (0–3 fast, 4–11
slower, 12–15 slowest), 62 GiB, Linux 7.3.1, Nix 2.34.9, rustc and
cargo 1.99.1, on 2026-11-08 from 15:37 to 15:55. Step 48a's session ran
beside the whole time (text edits in three repositories and `nix flake
check` under the machine lock, its capped work on cores 10–13, later
8–15); no other project's session ran. What ran beside each measurement
is named in its section. Cores 0 and 1 were never mine.

## 1. The web module

**Sizes** (15:37, load 0.58):

```sh
flock -o /tmp/linlog-machine.lock nix build .#linlog-web --max-jobs 2 --cores 6 -o web
nix shell --inputs-from . nixpkgs#brotli -c sh -c '
  stat -c %s web/linlog_web_bg.wasm
  gzip -9 -c web/linlog_web_bg.wasm | wc -c
  brotli -q 11 -c web/linlog_web_bg.wasm | wc -c'
```

| file | raw | gzip -9 | brotli -11 |
|---|--:|--:|--:|
| `linlog_web_bg.wasm` | 6 913 020 | 2 603 759 | 1 854 124 |
| `linlog_web.js` | 34 908 | 3 602 | 3 161 |

The package came from the store (its derivation was built before, 0.8 s);
gzip 1.15, brotli 1.2.0. Step 42 recorded 6.9 MB uncompressed and step
34 5.2 MB raw, 2.0 MB at gzip; no compressed figure after step 42
existed. `wasm-opt --metrics` (binaryen 132) of the package: 4 602
functions, 721 201 bytes of memory data, 2 891 086 instructions, 38
exports.

**Shares.** The package strips the name section, so twiggy (0.8.0, from
the flake's nixpkgs) sees only indices on it. The module was built again
with its names, by the package's own commands with `-g` added to
`wasm-opt` (15:38–15:39, capped on cores 4–9, six jobs; 70 s for the
wasm32 release build from nothing):

```sh
cargo build --locked --release --target wasm32-unknown-unknown --package linlog-web --target-dir DIR
wasm-bindgen --target web --out-dir OUT DIR/wasm32-unknown-unknown/release/linlog_web.wasm
wasm-opt -O3 -g OUT/linlog_web_bg.wasm -o OUT/linlog_web_bg.wasm
nix shell --inputs-from . nixpkgs#twiggy -c twiggy top -n 100000 --format csv OUT/linlog_web_bg.wasm > top.csv
python3 bench/footprint.py wasm top.csv
```

It is 7 541 544 bytes with its 650 915 bytes of names; stripped
(`wasm-opt --strip-debug`) it is 6 897 045, 0.23 % under the package,
since `-g` changes a little of what wasm-opt does. Of its 6 890 629
bytes of code and data:

| group | bytes | share |
|---|--:|--:|
| the PNG and PDF renderers' crates | 2 752 431 | 39.9 % |
| linlog and linlog-web | 1 873 254 | 27.2 % |
| the standard library (`core`, `alloc`, `std`, the allocator) | 978 420 | 14.2 % |
| data segments (no crate) | 705 748 | 10.2 % |
| the bindings' shims (no crate) | 393 608 | 5.7 % |
| the other crates (serde_json, hashbrown, serde, …) | 187 168 | 2.7 % |

The largest crates: linlog 1 655 716 (24.0 %), core 815 989 (11.8 %),
usvg 436 618 (6.3 %), skrifa 330 820 (4.8 %), tiny_skia 296 886
(4.3 %), krilla 242 269 (3.5 %), linlog_web 217 538 (3.2 %), read_fonts
212 853 (3.1 %), ttf_parser 183 130 (2.7 %), image_webp 166 566
(2.4 %), rustybuzz 125 981, subsetter 121 964, zune_jpeg 112 460,
serde_json 102 016. linlog's own largest modules: `search::focus`
228 239, `sequents` 162 534, `export::svg` 161 916, `proofs::derivation`
153 411, `proofs::interactive` 119 386, `nets` 103 427, `proofs::check`
103 276, `proofs::cut` 91 998, `serialize` 67 777, `search::inverse`
64 567, `search::horn` 49 929; `ordinary` 4 643. An item counts for the
crate its name names first, so a generic of `core` instantiated for
linlog counts as `core`'s (`bench.md` says so beside the script).

**The client's critical path** (`../linlog-prover.github.io` at
`ymrkqtrq`, its `main`, its working copy clean; built read-only, 15:38,
from the store):

```sh
flock -o /tmp/linlog-machine.lock nix build ~/Projects/own/linlog-prover.github.io#default --max-jobs 2 --cores 6 -o site
nix log site    # the build's own log
```

Astro 7.3.7's build prints no size table (its log lists the routes and
"3 page(s) built in 3.04s"), so the path was read off its output: what
`index.html` loads before the island renders.

| file | raw | gzip -9 | brotli -11 |
|---|--:|--:|--:|
| `index.html` | 276 284 | 39 151 | 32 033 |
| `_astro/Base.BcI64SuO.css` | 52 127 | 10 410 | 9 422 |
| `_astro/Base.astro_astro_type_script_index_0_lang….js` | 115 | 166 | 85 |
| `_astro/client.oOjy0wQO.js` (the island's renderer) | 987 | 619 | 516 |
| `_astro/App.BIIsGteW.js` (the island, `client:only`) | 188 084 | 57 443 | 48 990 |
| `_astro/store.Dakaum6t.js` (their one import) | 33 185 | 12 441 | 11 314 |
| **the critical path** | 550 782 | **120 230** | **102 360** |

Beside it: the two text fonts (Figtree 20 156 and Space Mono 16 520
bytes of woff2, with `font-display: swap`, so they do not hold the first
paint; with them 157 KB at gzip), and after first paint the worker
(17 778 bytes, 4 654 at gzip) and the module, which is byte for byte
the package above (`cmp`). Of `index.html`'s 276 KB, 214 574 characters
are the island's serialised props (the content handed to `App`) and
51 000 its 139 templates. Step 34 recorded 43 KB and later 55 KB on the
path; the growth since is that content and the island.

## 2. The build

```sh
systemd-run --user --unit=… -p MemoryMax=8G -p MemorySwapMax=0 taskset -c 4-9 \
  nix develop -c bash timings.sh OUT    # CARGO_BUILD_JOBS=6
# timings.sh: rm -rf target; for each step: cargo build --workspace --locked --timings [--release]; du -sb target
python3 bench/footprint.py timings dev-clean.html dev-touch.html release-clean.html release-touch.html
```

The steps in order, in one `target/`: dev from nothing, `touch
core/src/ordinary/translate.rs`, dev again, release from nothing (its
directory was empty), the touch, release again. Wall times are cargo's
own ("Finished … in"), rough figures on the slower cores; the shares are
of the summed unit time, which overlaps under six jobs. Beside it
(15:40:49–15:45:35): step 48a's session on cores 10–13; the load
average at each step's start was 0.88, 1.95, 2.03 and 5.85.

| build | units | wall | summed | `target/` after |
|---|--:|--:|--:|--:|
| dev, clean | 183 | 28.9 s | 93.5 s | 2 363 574 722 B |
| dev, one file of `ordinary/` touched | 5 | 3.0 s | 5.0 s | 2 640 564 893 B |
| release, clean | 187 | 137.4 s | 364.2 s | 3 122 354 802 B |
| release, the same touch | 5 | 116.3 s | 206.0 s | 3 122 397 582 B |

At the end `target/debug` held 2 640 371 227 bytes and `target/release`
481 615 155.

The crates' shares of the summed time:

| crate | dev clean | dev touched | release clean | release touched |
|---|--:|--:|--:|--:|
| linlog-cli (library and binary) | 6.2 s, 6.6 % | 1.3 s, 25.9 % | 102.0 s, 28.0 % | 101.1 s, 49.1 % |
| linlog | 11.8 s, 12.7 % | 2.0 s, 39.8 % | 52.0 s, 14.3 % | 48.8 s, 23.7 % |
| linlog-bench | 3.5 s, 3.8 % | 0.9 s, 18.8 % | 40.0 s, 11.0 % | 38.0 s, 18.5 % |
| linlog-web | 2.9 s, 3.1 % | 0.8 s, 15.6 % | 19.6 s, 5.4 % | 18.1 s, 8.8 % |
| write-fonts | 5.5 s, 5.9 % | – | 27.1 s, 7.4 % | – |
| read-fonts | 5.9 s, 6.3 % | – | 12.6 s, 3.5 % | – |
| krilla | 2.2 s, 2.4 % | – | 11.4 s, 3.1 % | – |
| skrifa | 2.4 s, 2.6 % | – | 8.9 s, 2.4 % | – |
| usvg | 2.5 s, 2.6 % | – | 7.7 s, 2.1 % | – |
| rustybuzz | 2.0 s, 2.2 % | – | 5.8 s, 1.6 % | – |
| syn | 3.0 s, 3.2 % | – | – | – |
| clap_builder | 2.6 s, 2.8 % | – | 4.3 s, 1.2 % | – |
| wasm-bindgen-macro-support | 2.2 s, 2.4 % | – | – | – |
| serde_derive | 1.8 s, 1.9 % | – | – | – |
| ttf-parser | 1.7 s, 1.9 % | – | 3.1 s, 0.8 % | – |
| the other crates | 37.2 s, 39.8 % | – | 69.8 s, 19.2 % (resvg 3.9 s, tiny-skia 3.6 s, subsetter 3.4 s among them) | – |

The workspace's units by target (seconds; frontend and codegen where
cargo reports them):

| unit | dev clean | release clean | release touched |
|---|--:|--:|--:|
| `linlog` library | 11.8 (7.7 + 4.2) | 52.0 (9.3 + 42.7) | 48.8 (8.3 + 40.5) |
| `linlog-cli` library | 5.4 | 35.4 (2.2 + 33.2) | 33.6 |
| `linlog` binary (the command) | 0.7 | 66.5 | 67.5 |
| `linlog-bench` binary | 3.5 | 40.0 | 38.0 |
| `linlog-web` library (host) | 2.9 | 19.5 | 18.1 |

`cargo build --workspace` builds `linlog-web` for the host only; its
wasm32 release build from nothing took 70 s (section 1).

## 3. The ratchet

```sh
systemd-run --user --scope -p MemoryMax=8G -p MemorySwapMax=0 taskset -c 4-9 \
  nix develop -c cargo run --release --locked --package linlog-bench -- ratchet --check --jobs 4
```

As the gate runs it (15:46–15:50, load 4.11 at the start, step 48a
beside): exit 0, every journey within `TOLERANCE` (2 %) of its ceiling
(`bench/ceilings.csv`, last re-recorded 2026-10-22 for rustc 1.99.1).

| journey | instructions | ceiling | change | L1 misses | change | LL misses | change |
|---|--:|--:|--:|--:|--:|--:|--:|
| search-qbf-20-2 | 485 424 578 | 481 419 142 | +0.83 % | 3 709 827 | +8.01 % | 216 512 | +0.09 % |
| search-chain-128 | 957 560 014 | 959 285 073 | −0.18 % | 1 033 924 | +4.78 % | 8 582 | +1.67 % |
| search-partition-no-5 | 7 074 239 | 7 028 620 | +0.65 % | 3 866 | +4.83 % | 1 551 | +7.56 % |
| search-wide-m2-256 | 264 987 646 | 260 393 363 | +1.76 % | 404 252 | −2.00 % | 4 842 | +6.89 % |
| search-spec-chain | 23 098 481 | 22 770 649 | +1.44 % | 145 055 | +0.93 % | 4 532 | +1.75 % |
| search-chain-64-intuitionistic | 121 615 168 | 121 737 110 | −0.10 % | 155 451 | +3.07 % | 4 050 | +3.47 % |
| search-additive-14 | 583 341 458 | 582 045 966 | +0.22 % | 6 534 966 | −0.09 % | 1 942 047 | +0.13 % |
| read-text | 12 436 083 | 12 269 416 | +1.36 % | 100 971 | −3.29 % | 7 353 | +7.75 % |
| read-json | 12 926 701 | 12 799 833 | +0.99 % | 40 014 | −8.22 % | 251 | +28.06 % |
| read-lltp | 28 574 554 | 28 344 056 | +0.81 % | 128 380 | +0.17 % | 38 365 | +0.12 % |
| read-tptp | 33 863 362 | 33 910 806 | −0.14 % | 71 504 | −0.26 % | 21 764 | −0.00 % |
| read-spec | 18 430 260 | 18 341 178 | +0.49 % | 114 282 | +0.28 % | 37 438 | +0.10 % |
| check-qbf-20-2 | 137 847 353 | 136 738 432 | +0.81 % | 210 083 | −10.41 % | 9 576 | +0.07 % |
| check-wide-m1-2048 | 8 587 637 | 8 575 388 | +0.14 % | 21 055 | +0.41 % | 171 | +2.40 % |
| derivation-chain-64 | 9 891 073 | 9 701 583 | +1.95 % | 14 388 | +2.54 % | 8 623 | +2.67 % |
| render-latex | 5 725 179 | 5 675 903 | +0.87 % | 5 454 | −0.07 % | 3 853 | +3.91 % |
| render-typst | 4 778 549 | 4 726 140 | +1.11 % | 6 162 | +0.79 % | 4 418 | +3.44 % |
| render-svg | 84 268 677 | 83 835 448 | +0.52 % | 102 077 | −5.49 % | 34 020 | +0.36 % |
| batch-families | 52 505 121 | 52 333 250 | +0.33 % | 70 326 | +8.83 % | 11 965 | +2.77 % |
| ordinary-pigeons | 31 087 306 | 30 679 397 | +1.33 % | 149 103 | +2.23 % | 28 481 | +7.00 % |

The ratchet named the misses of `search-qbf-20-2` and `check-qbf-20-2`
as moved by more than 5 % and 10 000, "a question for their time pinned
to one core … not a failure"; nothing here changed code, so there is no
before and after to pin.

## 4. The target sets

Asked of `linlog-orchestrator` before (answered "go, once your own
builds are done"); beside both ran step 48a's unit `linlog-48a-checks`
(a rustc on core 7 and a callgrind on core 1 at 15:51), which the
orchestrator let finish; it starts nothing new and heavy in the slot.
The counters compared are a function of the input on one thread, which
load does not move; `time_ms` and `cpu_ms` are reported and not argued
from.

```sh
taskset -c 4-9 nix develop -c bench/targets.sh 47        # 15:50:43–15:50:57, cores 2 and 3
taskset -c 4-9 nix develop -c bench/net-targets.sh 47    # 15:53:51–15:54:00, core 2
target/release/linlog-bench summary --against bench/targets/45d.csv bench/targets/47.csv --counters
target/release/linlog-bench summary --against bench/targets/net-45d.csv bench/targets/net-47.csv --counters
```

- Focused set: "Verdicts that differ: None", "Of the 145 problems both
  decide alike on one thread, 0 differ in nodes, splits, memo_hits,
  memo_entries, links, tests", nothing decided by one label and not the
  other. 284 rows, 257 decided in both; summed `time_ms` 15 636 against
  15 520, `cpu_ms` 15 040 against 14 940.
- Net set: "Verdicts that differ: None", "Of the 64 problems both decide
  alike on one thread, 0 differ"; summed `time_ms` 9 049 against 8 186,
  `cpu_ms` 8 800 against 8 000, with 48a's checks beside.
- Against the older oracles (`summary --against
  bench/targets/after-efficiency.csv …`): no verdict differs; 8 of the
  135 rows both decide differ in counters, each a change recorded in
  `bench/TARGETS.md` (the chain family's four, one memo entry fewer and
  one hit more since `after-generations`; `partition-table/1-1`, the
  focused engine's since `after-routing`; `mix/8` to `mix/10`, answered
  by the inverse method since `37b`). The net set equals
  `net-after-prunes.csv` on all 64.

The two files and their lines in `bench/TARGETS.md` are the commit
`tvzzkolw` (signed `08509acca50b`), alone.

## Decisions and assumptions

- **The client's critical path read off its output (departs from the
  design)**: the prompt asks for the path "as its own build reports
  it", and Astro's build prints no sizes. The path is the files
  `index.html` loads before the `client:only` island renders, each
  compressed by `gzip -9` and `brotli -q 11`; the fonts are counted
  beside, as step 34 did, since they swap in.
- **The shares from a module built again with its names**, by the
  package's commands with `wasm-opt -g`, 0.23 % from the package's size
  when stripped; the package itself is unchanged.
- **twiggy and brotli not added to the devshell**: `nix shell
  --inputs-from . nixpkgs#twiggy nixpkgs#brotli` takes them at the
  flake's nixpkgs, which repeats the measurement without a tool kept for
  a step every few months; so `new-tool` was not run. `bench.md` says
  where they come from.
- **`bench/footprint.py`**, a script that repeats the two tables, under
  `bench/` with its licence header and a line in `bench.md`, as the
  prompt allows; `typos.toml` admits `flate` (from the crate `flate2`
  the script lists), with its reason.
- **The oracles moved to `47` and `net-47`** in `bench/TARGETS.md`,
  CLAUDE.md's verification table and `bench.md`, since the prompt makes
  them the oracles the next steps compare with (D17); CLAUDE.md and
  `bench.md` are a commit of their own (`uzqqspoo`, signed
  `97f147277596`), apart from the target files.
- **The target sets' durations corrected** in CLAUDE.md and `bench.md`:
  about 15 s and 10 s at 0.2.0, where they said 20 and 5 minutes (45d's
  run took 14 s on 2026-11-02 as well).
- **One `target/` for all four builds**, release after dev, so the sizes
  after the release builds include the dev profile's 2.64 GB;
  `target/release` alone was 0.48 GB.

## What the next sessions must know

- **Step 49 (the suite registry, the Ordinary tab)**: the critical path
  is 120 230 bytes at gzip (102 360 at brotli) of D35's 170 KB, about
  50 KB of room at gzip. The largest part that grows with content is
  `index.html`'s serialised island props (214 574 characters raw, most
  of its 39 151 compressed bytes): every lesson, problem or suite whose
  content is handed to `App` as props lands on the critical path; content
  loaded after first paint (as the module is) does not. Remeasure with
  the commands of section 1 on the client's build output.
- **Step 63 (the suite protocol, the module question)**: the module is
  6.91 MB raw, 2.60 MB at gzip, 1.85 MB at brotli. A second module per
  suite would carry its own standard library and bindings' shims
  (19.9 % of this one, 1.37 MB raw), and serde besides; the PNG and PDF renderers'
  crates are 40 %, of which the raster decoders usvg links (image_webp
  167 KB, zune_jpeg 112 KB, png 60 KB, gif) decode images linlog's SVG
  never embeds. The ordinary suite adds 4.6 KB today, being an
  embedding; the first non-embedded suite is the size to measure, with
  `bench/footprint.py wasm` before and after.
- **Step 67 (the kernel crate)**: a dev rebuild after touching core is
  3.0 s, of which `linlog` 2.0 s; a release rebuild is 116 s of a clean
  137 s, because `lto = "fat"` and `codegen-units = 1` relink every
  binary whole (`linlog-cli`'s binary 66.5 s, `linlog-bench`'s 40 s)
  and `linlog`'s codegen is 42.7 s of one unit. Moving code into a
  kernel crate does not shorten the release relink; it can only shorten
  `linlog`'s own frontend (9.3 s release, 7.7 s dev). The font crates
  (write-fonts, read-fonts, skrifa, krilla) are 17.2 % of a clean dev
  build's summed time and 16.4 % of a release one's, from the `png` and `pdf` features the command enables.
- **The ratchet has little headroom**: `derivation-chain-64` is at
  +1.95 % and `search-wide-m2-256` at +1.76 % of the 2 % tolerance,
  drift since the ceilings of 2026-10-22 under the code of steps 42 to
  45. A step whose change touches derivations or the net engine may
  cross it without being slower; `bench.md` ("What a move means") says
  how to answer that, by pinned time, never by raising a ceiling unasked.
- **The oracles are `bench/targets/47.csv` and `net-47.csv`**; the
  target sets take seconds now, not minutes, so a step can run them at
  every commit that touches a search.
- **Not verified**: which encoding GitHub Pages serves (gzip is the
  figure to hold against D35, brotli the floor if a host serves it); the
  load beside the dev and release builds beyond the load averages above.

## Checklist

- [x] Read the brief, the prompt, `conduct.md`, the reading list
  (46a's report, `structure.md` section 5, `web-front-end.md` sections 1
  and 3, `bench.md`, `web.md`, `bench/TARGETS.md`, D17, D19, D24–D32).
- [x] Workspace `../linlog-step47` on `utzkqlzy`.
- [x] 1: the module's three sizes, its shares by crate and module, the
  client's critical path against D35.
- [x] 2: four timed builds, the crates' shares, `target/` after each.
- [x] 3: `ratchet --check --jobs 4`, exit 0, every journey within.
- [x] 4: the timed slot asked and answered; `targets.sh 47`,
  `net-targets.sh 47`; both compared with 45d, 0 counters differing;
  committed on their own (`tvzzkolw`).
- [x] The oracle pointers (`uzqqspoo`) and the script (`prumryws`).
- [x] This report committed; `jj st` clean; the last message to
  `linlog-orchestrator`.
