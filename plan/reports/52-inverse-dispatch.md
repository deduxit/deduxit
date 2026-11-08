# Step 52: dispatch rows for the inverse method, where it wins

## Outcome

One of the three classes now gets the inverse method by default; the
other two were given turns, measured, and lost them, each with its
reason:

- **`wide-m1`: a handover** (departs from the design: neither a row nor a
  turn). Where the focused engine meets its memory bound on a unit-free
  MLL sequent whose atoms occur once with each sign, in classical mode
  without Mix, the inverse method takes the sequent over before the net
  engine, and the net engine follows where the inverse method meets a
  bound of its own (`handed_over`, `answer_goal` in
  `core/src/search/mod.rs`). `wide-m1` with 4 096, 8 192 and 16 384
  pairs is proved in 1.48, 3.05 and 11.8 s against 3.09, 9.48 and
  37.4 s, and 32 768 in 49.3 s where the default ran out its 60 s; at the
  command's 2 s limit 4 096 is proved in 1.47 s where it was unknown. On
  held-out shapes that reach the memory bound (shuffled wide sequents,
  triples, a right-nested one) it lost no row and took 0.017 to 0.41 of
  the time. After the recursion limit, under Mix and in intuitionistic
  mode the net engine takes over as before: under Mix the inverse method
  ran past a minute on `wide-m1` at 4 096 and 8 192, which the net
  engine proves.
- **Classical `chain`: no turn**, dropped after measurement (a decision,
  with `linlog-orchestrator`'s agreement of 2026-11-09). A row giving
  the inverse method turns on classical goals with an exponential decided
  `chain` from 256 clauses on (512 in 0.25 s against 2.35 s, 1 024 to
  8 192 where the focused engine meets its recursion limit) and, at the
  command's 2 s limit, 100 classical readings of the LLTP library that
  the default leaves unknown. But at that limit on one thread (`--jobs
  1`, `--deterministic`, the web build) it lost
  `ILLTP-SYJ-cbn/SYJ212+1.015`, which the focused engine refutes in 1.86
  to 1.88 s, to the limit in each of four rounds on cores 2 and 3, and
  made 22 of 62 decided rows slower (5 above 1.2, worst 1.78). D19 lets
  a turn lose no row the default decides, so it went; `later.md` keeps
  the case.
- **Affine `pigeonhole`: no turn**, dropped after measurement (with
  `linlog-orchestrator`'s agreement). A row giving turns on intuitionistic
  affine goals decided #0 at 5 holes in 0.54 s against 2.13 s and 6
  holes in 18 s where the two-sided engine passes a minute, but #1 at 7
  holes, which the two-sided engine refutes in about a second, took 1.27
  times as long in pinned CPU time; a share small enough to spare it
  (about 512 against 32) would leave 6 holes far past the minute (some
  72 G units by the turn schedule, against the 10.9 G the two-sided
  engine does in it).
- **Why both turns cost so much**: a turn's share is counted in the
  inverse method's units, and on these goals they run much slower than
  the engine's (on the classical SYJ readings its set-up does about
  13 000 a millisecond against the focused engine's 146 000), so a
  sixteenth of the units was up to half the time, not a sixteenth.
- **The harness fix came first**: `linlog_read` (`bench/src/tools.rs`)
  takes as the reason the bound the verdict line names first, so "the
  recursion limit of 2048 was reached … at a copy bound of 410" is
  `recursion_limit` (`linlog_reason_is_the_bound_named_first`), commit
  `nvuvtusx`, before any number was read. Rebased onto 51b, which reads
  linlog's outcome as JSON, the reason is the outcome's `reason.kind`
  (the one bound that stopped the search, every kind the library has),
  and the verdict line, read as above, is the fallback where no outcome
  comes; the test checks both, the JSON case on a search that meets the
  recursion limit at a copy bound of 1.
- **Session 43c's intuitionistic entry** (`!(a -o a * a * a), !(a -o a),
  a |- a * (a + 0)`, unknown after 2 s at the time) is answered by step
  37's turn on the intuitionistic exponential row: the default refutes it
  from the inverse engine in 137 ms (16.9 M units, just past the head
  start), saturated at 14 sequents. `the_inverse_method_takes_its_turn`
  now pins it at the library's head start and share with the command's
  deepening, and the entry left `later.md`.

The target sets equal 47's on every row (verdicts and counters), so no
new oracle. The gate, the manual's checks and `nix flake check` passed
(the checklist).

## The machine

An Intel Core Ultra X9 388H, 16 cores without SMT (0–3 fast, 4–11
slower, 12–15 slowest), 62 GiB, Linux 7.3.1, from 2026-11-08 22:55 to
2026-11-09 01:30. Beside this session: 50b (the client page, finished
23:34), 51b (certificates in linlog: held during the timed runs, then on
cores 10–15; its gate on 13–15 and panel on 10–12 during the 00:13
pinned run) and 53m (held during the timed runs); light renders on cores
10–15 during the last slot. Every timed run on cores 2 and 3 started on
`linlog-orchestrator`'s "go". Cores 0 and 1 were never used; untimed
work ran on cores 4–9 in capped scopes. Scripts, CSV files and analyses
are under `/tmp/linlog-handoff/52-measure/` (each named below).

The builds: `bench-base` is main (`e0e7a812`) with the harness fix
(`nvuvtusx`), built by `bench/targets.sh` in `../linlog-step52-base`
(`linlog-bench` alone); the final `bench-lift` is `spxtwkst`'s, built the
same way (the commit's documentation got its final numbers afterwards,
comments only). Earlier lifts are kept under `round1/` (both turns) and
`round2/` (the classical turn). The reviewer's note led to finding that
round one's growing and pinned runs and the default view compared that
base with a lift built together with the command, which unifies four
more features of `linlog` (`interactive`, `pdf`, `png`, `rocq`), none on
the search's path; the numbers the outcome rests on were taken again
with builds made alike.

## The design

- **Why a handover for `wide-m1`.** A turn cannot help: the focused
  engine meets its memory bound after 0.88 s at 4 096 pairs, before
  turns of a sixteenth of its work could give the inverse method the
  0.65 s it needs. A row by "forced linking" would take the deep chains
  too, where the focused engine and then the net engine are faster. The
  dispatch already hands exactly these sequents over at a limit, so the
  change is which engine takes them.
- **The taker, by the limit met.** Probes (untimed, cores 5 and 9), the
  inverse method against the net engine forced: the chain `a₀, a₀ ⊸ a₁,
  … ⊢ aₙ`, which meets the recursion limit, 51 ms and 1.0 s against 35 ms
  and 0.55 s at 1 024 and 4 096 links in either mode; wide products
  `a₀, …, aₙ ⊢ a₀ ⊗ … ⊗ aₙ` 5 to 65 ms against 0.8 to 4.1 s at 4 096 and
  8 192 atoms; `wide-m1`, which meets the memory bound, three to four
  times faster. So the memory bound goes to the inverse method and the
  recursion limit to the net engine; the limit stands in for deep against
  wide, and no static feature was looked for.
- **Mix and intuitionistic mode keep the net engine.** The held-out set's
  `wide-m1` under Mix at 4 096 and 8 192 was proved by the net engine and
  ran past 60 s on the inverse method, so the taker became classical
  without Mix; intuitionistic memory-bound goals were not measured and
  keep what they had. `another_engine_takes_over_at_a_limit` pins the
  three cases (the inverse method, the net engine after it, Mix).
- **The turns.** Built as the Mix row's (a row with `probe: true`, the
  head start `PROBE_AFTER` and share `TURN_SHARE` unchanged), measured,
  dropped (the outcome). The rule applied to both: keep a turn whose
  measurement wins rows the default does not and loses none; drop one
  that loses a decided row, or slows one as often as it wins one.
- **No options** (D15, D16, D17): no constant changed and none was added;
  the handover reads no option. Quantifiers: the inverse method refuses
  `LL1`, so first-order goals take no inverse handover.

## The measurements

"Cores 2 and 3" means both builds at once, base on 2 and lift on 3, then
swapped (the same cores for before and after).

### The target sets (counters)

`bench/targets.sh 52` and `bench/net-targets.sh 52` with the final
build, cores 2 and 3, 01:01 to 01:02, then `linlog-bench summary
--against bench/targets/47.csv bench/targets/52.csv --counters` (and
`net-47.csv` against `net-52.csv`): no verdict differs, 0 of the 145
decided rows and 0 of the 64 differ in any counter. Base (`targets.sh
52-base`, 23:13): the same. With the classical turn (00:07) the only
rows that differed were `chain/256` and `growing/1024`, decided by it.
The files are `targets-final/`; no oracle was committed.

### `wide-m1` (times)

`timed3.sh`: `linlog-bench run --family wide-m1=… --jobs 1`, at 60 s and
at the command's 2 s with copies deepening, two rounds on cores 2 and 3,
01:02 to 01:05, load 1.9 to 2.7. Median times in ms:

| row | base | lift | lift/base |
|---|--:|--:|--:|
| 2 048, 60 s | 207 focus | 209 focus | 1.008 |
| 4 096, 60 s | 3 089 net | 1 477 inverse | 0.478 |
| 8 192, 60 s | 9 477 net | 3 050 inverse | 0.322 |
| 16 384, 60 s | 37 387 net | 11 831 inverse | 0.316 |
| 32 768, 60 s | unknown at the limit, net | 49 296 inverse | |
| 2 048, 2 s | 204 focus | 206 focus | 1.009 |
| 4 096, 2 s | unknown at the limit, net | 1 466 inverse | |
| 8 192, 2 s | unknown at the limit, net | unknown at the limit, inverse | |

### The held-out shapes for the handover

`timed2.sh` part A, `held-out-handover.txt` from `handover-variants.py`
(seed 52): 60 s, one thread, two rounds on cores 2 and 3, 00:44 to
00:48, load 1.1 to 1.7; the lift had the classical turn, which these
sequents (MLL without exponentials) never take. Median times in ms:

| shape | base | lift | lift/base |
|---|--:|--:|--:|
| shuffled 4 096 | 3 644 net | 1 155 inverse | 0.317 |
| shuffled 8 192 | 10 804 net | 425 inverse | 0.039 |
| shuffled 16 384 | 43 043 net | 725 inverse | 0.017 |
| triples 2 048 | 283 focus | 285 focus | 1.006 |
| triples 4 096 | 5 608 net | 1 829 inverse | 0.326 |
| triples 8 192 | 18 427 net | 3 237 inverse | 0.176 |
| right-nested 4 096 | 2 254 net (recursion limit) | 2 269 net | 1.007 |
| right-nested 8 192 | 7 448 net | 3 015 inverse (memory bound) | 0.405 |

No row lost, no verdict changed. A balanced tensor tree was left out of
the set because the focused engine decides it in 9 ms, before any limit.

### Pinned CPU time

`python3 bench/pinned-time.py run bench-base bench-lift RUNS.csv`, then
`summary`; five rounds, cores 2 and 3 swapped half-way, its list as on
main, 01:05 to 01:08, load 2.4 to 2.6 (`final2/pinned-summary.csv`):

| row | base ms | lift ms | ratio | range |
|---|--:|--:|--:|---|
| `search-qbf-20-2` | 984 | 978 | 0.993 | 0.982 to 1.009 |
| `search-chain-128` | 1 076 | 1 080 | 1.004 | 0.997 to 1.023 |
| `search-partition-no-5` | 840 | 831 | 0.990 | 0.973 to 1.010 |
| `search-chain-64-intuitionistic` | 925 | 924 | 0.999 | 0.989 to 1.010 |
| `chain/512` | 2 360 | 2 380 | 1.009 | 0.996 to 1.013 |
| `qbf/34#1` | 1 230 | 1 240 | 1.008 | 0.992 to 1.016 |
| `mix/9` | 6 040 | 6 030 | 0.998 | 0.984 to 1.013 |
| `chain/384-intuitionistic` | 990 | 990 | 1.000 | 1.000 to 1.020 |
| `chain/512-intuitionistic` | 2 360 | 2 360 | 1.000 | 0.996 to 1.009 |
| the seven `other` rows | | | 0.993 to 1.003 | 0.964 to 1.053 |
| sum `focused` | 16 805 | 16 813 | 1.000 | 0.992 to 1.011 |
| sum `other` | 8 856 | 8 846 | 0.999 | 0.994 to 1.001 |

Every row's range spans 1: no row is slower beyond noise.

### The turns that were dropped

- **Round one** (both turns; the lift built with the command):
  `growing.sh`, the growing sets at 60 s on cores 2 and 3, 23:34 to
  23:43, load 1.2 to 2.2: `chain` 256 0.83, 512 0.108, 1 024 to 4 096
  decided in 0.27 to 0.79 s where base ends at the recursion or time
  limit, 8 192 in 1.9 s; `pigeonhole` 5#0 0.27, 6#0 proved in 18 s,
  7#1 1.34. Pinned (23:43 to 23:46): `pigeonhole/7#1` 1.27 (1.24 to
  1.27).
- **The held-out set** (`held-out.sh`, `analyse.py`; 5 s, copy bound 3,
  one thread, base on cores 4–6 and lift on 7–9 at once, untimed, 23:25
  to 23:28): LLTP's intuitionistic problems read classically and its
  classical problems (1 375), its exponential-free intuitionistic
  problems in affine mode (84), `wide-m1` under Mix (3). Every row base
  decided, lift decided alike once the Mix taker was narrowed (rerun at
  23:32, `out-mix-fixed/`); 90 undecided rows ran turns, six of them now
  ending at the time limit instead of the copy bound.
- **The command's default** (`default-view.sh`, 2 s, deepening, pool
  after 100 ms, three threads, base on 4–6 and lift on 7–9 at once,
  untimed, 23:51 to 00:04): classical readings decided 970 → 1 070, none
  lost with the pool; of the decided rows the turns touched 25 faster
  (up to 11.4, `SYJ205+1.011` of cbn), 11 within 5 %, 26 slower (1.05
  to 1.49). Not a same-core comparison (the reviewer's second finding).
- **The same rows on cores 2 and 3** (`timed2.sh` part B, the 62 rows
  the turn touched or base decided in over a second, 2 s, deepening,
  one thread, four rounds, 00:48 to 00:50): `SYJ212+1.015` of cbn
  refuted by base in 1.86 to 1.88 s and unknown at 2.05 s with the turn
  in every round (the focused engine had done 568 M of the 1 081 M
  units it needs); 20 faster, 20 within 5 %, 22 slower, 5 above 1.2
  (`SYJ212+1.013` of cbn 1.78, `.014` 1.61, `SYJ208+1.007` of cbv 1.44).
  This decided the drop.
- **Pinned with the classical turn** (00:13 to 00:16, load 2.0 to 2.3,
  51b's gate and panel on cores 10–15 and a `cargo test` of mine on
  cores 4–9 in its first two minutes): `chain/512` 0.107, `wide-m1/4096`
  0.480, `SYJ208+1.007` read classically 1.29, `SYJ206+1.015` at copy
  bound 3 2.77; the unchanged rows 0.99 to 1.03.

### Untimed probes

The 43c sequent (cores 4–9, the release command): `linlog prove -i
--stats …` three times, 136 to 137 ms, 16.9 M units, inverse engine. The
chain and product sequents above: `linlog prove --engine E
--deterministic --stats` on core 5 or 9.

## Departures from the design

- **`wide-m1` gets a handover, not a row or a turn** (departs from the
  design): the prompt asks for a row or a turn per class; neither fits
  (the design section says why). The handover runs only after the
  focused engine gave up, but it can still lose a row: if the inverse
  method ran to the time limit, the net engine would not get the sequent
  (this report's first draft claimed otherwise; the reviewer's first
  finding). The held-out shapes lost none.
- **`chain` and `pigeonhole` get nothing**, which the prompt allows with
  a reason (the outcome), after turns for both were built and measured.

## The reviewer

A fresh-context reviewer (Opus 5.5 at high, `linlog-step52-reviewer`)
checked the measurement against D19 while the change still had the
classical turn. It confirmed the target sets, the held-out set's verdicts
and the default view's counts, and found three things, all of which held:

1. **The handover's final rule had no held-out row** (the Mix rows had
   been used to narrow it), and the report's claim that it "cannot lose
   a decided row" was false. *Answer*: eight shapes not used to choose,
   at 60 s on cores 2 and 3 (the table above): none lost, every one that
   reaches the memory bound faster; the claim is corrected (departures).
2. **The default view's times compared different cores at once.**
   *Answer*: the 62 rows that matter were timed again on cores 2 and 3,
   four rounds. That found `SYJ212+1.015` lost on one thread, and the
   classical turn was dropped for it.
3. **The stated cost of a turn ("up to some seventy percent") was below
   the measured 2.77.** *Answer*: the measured figures (2.8 times on
   `SYJ206+1.015`, 1.29 on the decided `SYJ208+1.007`) are what
   `core-search.md` and `later.md` now give; with the turn dropped,
   `Engine`'s documentation states why classical goals with an
   exponential get none.

Its unverified note (two lift binaries of different size) led to the
build mismatch described under the machine; the final slots used builds
made alike.

## What the next sessions must know

- **The handover's taker is chosen by the limit met** (`handed_over`,
  `answer_goal`): the memory bound in classical mode without Mix goes to
  the inverse method, then to the net engine if the inverse method meets
  a bound of its own; everything else goes to the net engine. A new shape
  that reaches the memory bound on a forced linking should be checked
  against both takers, and under Mix the inverse method is not one.
- **A turn's share is counted in the inverse method's units, not in
  time**, and those run up to some fifteen times slower than the
  engines' on the shapes measured, so a turn can cost half the engine's
  time and lose a row near the limit. The case for a turn bounded in time
  (`chain`, the 100 classical readings, `pigeonhole`) is in `later.md`
  with the rows to measure again.
- **Build both sides of a timed comparison the same way.** `cargo build
  -p linlog-cli -p linlog-bench` unifies the command's features into the
  harness's `linlog`; `bench/targets.sh` builds `linlog-bench` alone. A
  binary copied after the script ran is the alone build.
- **`systemd-run --same-dir` overrides `-p WorkingDirectory`**: a
  pinned run started from the main checkout ran main's
  `bench/pinned-time.py` (`final-main-list/`, kept as main's list).
- **The oracles stay `47.csv` and `net-47.csv`**: the change alters no
  row of the target sets.
- **The harness reads linlog's reason from the JSON outcome** since 51b
  (`reason.kind`, which names every bound, the index limit, the witness
  depth and the instances included); the verdict line is read only where
  no outcome comes.

## Commits

- `nvuvtusx` Read linlog's reason as the bound its verdict line names
  first.
- `spxtwkst` Hand wide forced linkings at the memory bound to the
  inverse method (with its tests, the documentation of the turns not
  given, the manual, the guide and `core-search.md`).
- This report and `later.md`.

The commits of the classical turn's oracle (`vqsvlyvr`), its pointers
(`ynvoqqmp`) and pinned-time's rows for the turns (`uxmztyko`) were
abandoned with the turn.

## Checklist

- [x] Read the brief, the prompt, `conduct.md`, the reading list
  (`scope-engines.md` engines-5 and -12, `later.md`'s two entries, 37's
  "The dispatch", 45b's rows, `core-search.md`, `core-inverse.md`,
  `bench.md`, `bench/TARGETS.md`, D8, D19).
- [x] Workspace `../linlog-step52` on main after `jj git fetch`.
- [x] The harness fix and its test first (`nvuvtusx`).
- [x] Each class: `wide-m1` a handover with its measurement at the code
  (`handed_over`, `Engine`'s documentation), in `docs/src/commands.md`
  and `status.md` and in the guide (`reference.md`); `chain` and
  `pigeonhole` no turn, the reasons in `Engine`'s documentation,
  `core-search.md`, `later.md` and here.
- [x] Target sets against 47 by counters: no row changed, no oracle.
  Pinned CPU time with `bench/pinned-time.py` on cores 2 and 3, before
  and after; every timed run after "go".
- [x] The held-out set (LLTP readings, affine problems, Mix, the
  handover's shapes) and the command's default.
- [x] The 43c entry answered and pinned by a test.
- [x] Gate (exit 0, 01:00, on the final code), the manual's checks (exit
  0), `nix flake check` at the last commit (exit 0, after this report's
  commit).
- [x] The reviewer's round, every finding answered.
- [x] This report committed; `jj st` clean; the last message to
  `linlog-orchestrator`.
