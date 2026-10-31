# Step 45: the fourth baseline, and the second release, 0.2.0

The step's sessions each add a section here: 45b (the families that
grow), 45a (the Rocq library in its own repository), 45c (the fourth
baseline), 45d (the release prepared) and 45e (the package archives).

## Session 45b: families that grow

### Outcome

Twelve families with a growing set and a claim, five of them new
(`contraction`, `pigeonhole`, `euler`, `duplicate`, `reach-fo`) and seven
reused (`wide-m1`, `counter`, `mix`, `chain`, `chain-fo`, `witness-fo`,
`lambek-fo`). Each claim is written from measured rows, a fresh-context
reviewer checked every verdict by construction and every claim, and its
four findings that held are fixed. The library gained `Family::growth`
and `Family::claim` (public fields of a `#[non_exhaustive]` struct, an
addition; no enum variant). The harness gained `run --growing`, which
runs every growing set and skips a configuration's larger sizes after a
run refused or at its limit, written as `skipped` rows that no table
counts, and `run --memo-limit N`, a configuration axis (column
`memo_limit`), so that the memo's claim has its comparison line.
`bench/growing.sh` is the nights' run with each claim's comparison
passes, and `bench/comparison.sh --growing` runs the other provers on
the growing sets. `bench/plots.py growth` draws a line per engine,
configuration or prover and writes the fitted growth beside each figure;
the manual's chapter `results/growing-2026-10-31.md` shows both figures,
drawn at build time from the rows of this session's timed runs (by day,
cores 2 and 3, the runs that placed each family's sizes); the nights'
rows replace them. `--all-families` shows no mismatch; both target sets
equal 42k's counters; every code commit passed the gate. Not done at the
time of writing: the last three commits (the rows and the chapter, a
figure layout fix, this report) and `nix flake check` at the last
commit, which wait for the signing key (below, "What is left open").

### The research, and what each source gave

Three sub-agents on Sonnet 5.5 at `medium` read the sources below (the
literature's hard families for linear and intuitionistic logic, ILTP's
and TPTP's parametrised classes, the other provers' own sets and
documented limits); what they could not open they marked, and nothing
below rests on what was only recalled.

- **Chaudhuri, *The Focused Inverse Method for Linear Logic*, PhD thesis,
  CMU-CS-06-162 (2006), chapter 7** (read in full): Euler tours on the
  complete graph `K_n` as `edge₁ ⊗ … ⊗ edgeₘ ⊸ a ⊸ a` with an edge `(a ⊸
  b) & (b ⊸ a)` (§7.2.2, pp. 195–196; the inverse method took 9 s, 152 s
  and 3 043 s on its three sets, the unfocused prover failing on all),
  Hamiltonian tours, blocks world, QBF in MALL, affine encodings and the
  SICS problems under two embeddings. It gave `euler`; its verdict (a
  circuit exists iff every degree is even) is this session's derivation
  from Euler's theorem, and the graph is the square of a cycle instead
  of `K_n` (below).
- **Raths, Otten and Kreitz, "The ILTP problem library for
  intuitionistic logic", JAR 38 (2007), §2.1**, and the ILTP v1.1.2
  files themselves (`nix build .#iltp`: the headers of SYJ201 to SYJ212,
  each citing Dyckhoff, "Some benchmark formulas for intuitionistic
  propositional logic", 1997, and for SYJ203 and SYJ209 Franzén's SICS
  reports R87010B and R-89/89006), and the library's comparison table
  `ILTP-v1.1.2-prop-comparison.txt` (600 s per problem, 2007: SYJ202
  fails every prover from size 7 to 10, SYJ209 every prover from 11).
  It gave `contraction` (SYJ203 and SYJ209: "proof in LJ needs n
  contractions") and `pigeonhole` (SYJ202, Cook's pigeonhole principle);
  their linear readings are this session's, argued at the generators.
- **Olarte, de Paiva, Pimentel and Reis, "The ILLTP library for
  intuitionistic linear logic", EPTCS 292 (2019), arXiv:1904.06850,
  §3–4**: the SYJ classes kept at all 20 sizes in three translations,
  none of the non-theorems SYJ207 to SYJ212 refuted by the Maude prover
  in 5 minutes; Petri-net reachability as `⊗ !(•t ⊸ t•) ⊢ M ⊸ M'`. It
  confirmed that the intuitionistic non-theorems are where refutation
  is least explored, which `contraction` #1 takes up.
- **Hodas and Miller, "Logic programming in a fragment of intuitionistic
  linear logic", I&C 110 (1994), introduction**: the exponentially many
  splits of a context under `⊗`, delayed as long as possible; the
  claim of `euler`.
- **Lincoln and Winkler's announcement of 3-Partition in constant-only
  MLL (Types list, 1992)** and the abstracts of Kanovich's Horn
  fragments: linlog's families already have both.
- **The other provers' own sources**, read at the revisions the flake
  pins (`modules/_provers/`), and runs of the comparison's binaries by
  the agent (cores 4–7, 10 to 60 s): llprover (`llprover.pl` lines
  335–347 and 421–432, and its guide: a per-path threshold of 5
  contractions and cuts, "initial value is 5"), the Maude ILL prover
  (`ILL-Prover-flat.maude` lines 142, 286–310, and the LLTP script's
  copy limit 5), APLL (`prover.ml:10`, a bound of 3; `foc_ill_bwd.ml`,
  copies refilled once per round, `2^|Γ|` splits), Click & coLLecT
  (`focused_proof.ml` lines 460–476), linTAP (`lintap.pl` lines 56–58),
  lolli (`lolli-prove/src/search.rs` at `cb09efc`, lines about 293–352:
  a clause of the unrestricted zone used at most twice on a branch),
  LinearOne (`mill1.pl`, MILL1 only) and Sympli (its README). It gave
  `duplicate`, `!(a ⊸ a ⊗ a), a ⊢ a ⊗ … ⊗ a`, which the agent ran: lolli
  fails from 4 factors, APLL from 5, llprover and the Maude prover from
  7, Click & coLLecT times out from 8, Sympli proves 6 and 12.
- Not used: Tarau's generators of implicational formulas
  (arXiv:2009.10241, 1910.01775), which enumerate rather than grow;
  TPTP's SYN007 (Pelletier 71), classically valid and intuitionistically
  not, which needs the additive reading of `⇔`; Heijltjes and Houston's
  MLL with units, a question of proof equivalence, not provability.

### The families and their claims

Each family's verdicts are argued at its generator; "comparison" is the
line its claim is set against in the figure. Sizes are the growing set
(`Family::growth`), the ceiling the size where every line that admits
the family has met the 60 s limit or a bound in the timed run, or a
stated ceiling.

| family | claim (the line) | comparison | verdicts by construction | growing set, ceiling |
|---|---|---|---|---|
| `wide-m1` (reused) | the net engine past the focused engine's memory bound, to which the default hands over: 16 384 pairs in 35 s, quadratic | the focused engine (memory bound at 4 096); the inverse method forced is faster still (32 768 in 47 s), and no row gives it a turn | provable: a proof's identification of atoms keeps it a proof | 256 to 32 768, doubling; every line at the limit or memory by 32 768 |
| `counter` (reused) | the Horn engine, linear (262 144 tokens in 0.09 s) | the focused engine forced, about cubic (256 in 0.8 s, past a minute at 1 024); the inverse method forced, 16 in 16 s | provable within log₂ n copies (doubling clauses) | 2 to 262 144; stated ceiling 262 144 tokens, where the Horn engine takes 0.1 s and the sequent's text is megabytes |
| `mix` (reused) | the inverse method's turn on Mix (1 024 pairs in 34 ms forced; the default flat at 0.3 s after its turn) | the focused engine forced, eightfold per pair (10 in 51 s) | unprovable: every linking of a pair closes a cycle through its tensors, Mix too | 4 to 4 096 (extended after the run, where the inverse method took 34 ms at 1 024) |
| `chain` (reused) | the focused engine's memo on repeated subgoals, about cubic (512 in 2.3 s) | the memo off (`--memo-limit 0`: 32 in 0.23 s, past a minute at 64); no copy bound (no extra cost); the inverse method forced (2 048 in 4 ms), which no row gives a turn | provable within k + 2 copies | 16 to 8 192 (extended); the default and the focused engine meet the recursion limit from 1 024 |
| `contraction` (new) | the inverse method on a clause used n + 1 times on a branch (256 atoms in 1.8 s forced, about n³; the default proves 16 to 128 by its turn) | the two-sided engine forced (8 in 0.07 s, past a minute at 16) | #0 provable within n + 1 copies (Franzén's derivation); #1 unprovable: its image without linearity and `!` is ILTP's SYJ209, false in a two-world Kripke model (derived here, checked by the reviewer) | 2 to 512 (extended) |
| `pigeonhole` (new) | the two-sided engine's ⊕-left case splits, about fortyfold per hole (5 holes in 2.2 s, past a minute at 6) | the inverse method forced (6 in 0.27 s), which the default gives no turn without exponentials | #0 (n + 1 pigeons) provable in affine ILL by the pigeonhole principle on every branch; #1 (n pigeons) classically false | 2 to 8, by one; #0's lines at the limit or memory by 7, #1's by 8 |
| `euler` (new) | the focused engines' context splits: on a graph without an Euler circuit (#1) about 2.4-fold per vertex (18 in 11 s) | a circuit (#0) found in under a millisecond; the inverse method forced, slower | a cut-free proof of `x, E ⊢ y` is a trail using every edge once (the reviewer checked the lemma); the square of a cycle is connected and 4-regular, without `v1v2` two vertices have degree 3 | 5 to 24 vertices, by two; #1's lines at the limit or memory by 20 |
| `duplicate` (new) | the other provers' caps on contraction: lolli gives up at 4 factors, llprover's classical system passes 10 s at 4 (the research's runs: APLL from 5, llprover and the Maude prover from 7) | linlog's Horn engine, linear (262 144 in 0.07 s); the two-sided engine forced, 256 in 0.2 s | provable within m − 1 copies on one branch, #0 in ILL and #1 the same sequent classically | 2 to 262 144; the same stated ceiling as `counter` |
| `reach-fo` (new) | the first-order memo's loop check: a cycle of 10 nodes to its exit in 20 s, about fivefold per node | the memo off: 6 nodes in 1.1 s, past a minute at 8 | #0 provable within n + 1 copies (the path; the first edge's copy ends the branch); #1 unprovable: a classical first-order model, `r` true on the cycle alone | 1 to 16; at the limit by 12 |
| `chain-fo` (reused) | n instances of one clause, about n^2.6 (384 in 1.0 s), the recursion limit at 512 | LinearOne refuses it (MILL1 only); the resumption has no switch, so no comparison line | #0 provable within n copies; #1 unprovable by a classical countermodel, ends at its copy bound | 16 to 1 024; recursion limit from 512 |
| `witness-fo` (reused) | the pool of threads on the refutation #3: 10 in 1.7 s on two threads against 2.7 s on one | one thread; LinearOne decides #3 near flat (its own run of 2026-10-30), refuses #0 and #1 | as argued at its generator (42h) | 2 to 16, by two; #3 at the limit by 12 |
| `lambek-fo` (reused) | LinearOne's fragment: linlog's two-sided search within 0.5 s to 256 formulas | the recursion limit at 512 (#0, #2) and 1 024 (#3, #4) | as argued at its generator (42h) | 8 to 2 048 (extended) |

What was measured to settle the claims, beyond the timed run: the
verdict run of every growing set under the default and every engine at
5 s (cores 4–7, 570 rows, no verdict against its construction, no error,
no crash), and probes of the candidates with the command (Franzén's
formulas, the pigeonhole, Euler tours on complete graphs and on the
square of a cycle, reachability, the duplication). The `euler` family
moved from complete graphs to the square of a cycle when the complete
graphs went from 0.19 s to past the limit in one size, too few points
for a curve.

### The harness

- `run --growing` (`bench/src/growth.rs`): the growing sets, and a
  series (family, index, mode, engine, threads; for a tool, family,
  index, mode, tool) ends at a run refused, unknown at the time limit,
  the memory, killed or crashed, or decided past the limit. Its larger
  sizes are rows of verdict `skipped` with the reason "past the limit at
  size N" or "refused at size N" and the configuration's columns, which
  `summary::read_all` drops, so no table counts them, as a timeout or at
  all (the orchestrator's conditions, 2026-10-31). A bound of the
  search's own ends nothing. Every growing set is a constructed family,
  whose larger instance is no easier; a random family gets none. Under
  `--shard`, a shard takes whole shapes (`Shard::of_groups`), so that a
  series sees its end; `--resume` rebuilds the ends from the file.
- `run --memo-limit N` (column `memo_limit`, the last of `HEADER`; the
  resume key and the summary's label `memo off`): the memo's comparison.
  The resume key's test gained the field (its commit says why).
- `bench/growing.sh`: engines-K.csv (a stream per core, by shape),
  memo-off.csv, deepening.csv and pool.csv (`--jobs all` on
  `--pool-cores`, run alone after the streams). The first timed run
  failed its pool pass at once on `--jobs 4` with two cores; the script
  now takes every pool core, and the pass was rerun (2 minutes).
- `bench/comparison.sh --growing` (and `--quick`, the first two sizes of
  each set), building only the comparison package.
- `batch-families` (the ratchet's journey) takes a list of the
  seventeen families it was set on, so that its count stays the same
  batch (`+0.33 %` at the first gate, within the 2 % tolerance).
- `plots.py growth`: rows of `run` or `run --tools`; a line per engine
  (the default, each forced), per configuration (the memo off, threads,
  no copy bound, dashed in the engine's colour) or per tool; skipped
  sizes past the limit a cross above the limit's line; a refused line
  not drawn; `growth.txt` beside the SVG, per line the sizes over 1 ms
  and the better least-squares fit of the logarithm of the time, `n^d`
  or `b^n` (`.txt`, since a `.md` under `docs/src/figures/` would read as
  an unlisted chapter to the manual's test after a local preview). The
  header above the panels is sized by the legend's rows, and the title
  pinned in inches from the top: on the tall growth figure the title had
  run into the legend.

Options a front end would set (D15): none of the library's; `growth` and
`claim` are data of the families, read by the harness and by anyone
listing them.

### The graphs

Two figures in `docs/src/results/growing-2026-10-31.md`, both listed in
`docs/figures.sh` and drawn at build time in both themes by
`plots.py growth`: the engines' (`bench/growing/2026-10-31/*.csv`, the
timed run) and the other provers' (`bench/growing/2026-10-31/provers/
runs.csv`, the first two sizes of each set), each with its fitted growth
included from `growth.txt`. `modules/manual.nix` copies `bench/growing`
into the manual's build. The chapter says what each line shows, every
number read from the rows. The light and the dark engines figure were
rendered and read here. Step 29's cactus and scatter plots were already
shown by 44b's chapter "Beside the other provers, 2026-10-15".

### The nights' machine time, by arithmetic

**The first night (45c), the engines on the growing sets**, as
`bench/growing.sh` at 60 s. Measured by day: the searches took 2 227 s in
all (engines-1 1 107 s, engines-2 833 s, memo-off 185 s, deepening 40 s,
pool 62 s); the two streams ran 22 min 20 s (stream 1, 1 332 s of search,
so process starts and generation add about 1 %), and the pool pass 1.5
min. The sizes added since (the verdict check at 5 s placed them): up to
240 s for `contraction` at 512 (the default and the inverse method on
both shapes, a minute each at most), 2 s for `mix` at 2 048 and 4 096, up
to 180 s for `chain` at 4 096 (the default, the focused engine and the
deepening pass a minute each, 8 192 then skipped), up to 125 s for
`lambek-fo` at 2 048 (#2 a minute for each of its two lines), up to 170 s
for `duplicate`'s classical shape (the focused engine to its limit near
1 024, the inverse method to its limit at 16), about 0 for `reach-fo` at
1. So about 2 227 + 720 ≈ 2 950 s, 49 minutes of one core: **about 27
minutes on two streams plus 2 for the pool, about 14 on four streams
(cores 0 to 3, the baseline's sequential cores) plus 1 for the pool on
four cores**, beside the baseline's three hours.

**What the skipping saves**: 86 sizes past a line's limit were skipped,
each at least the full minute had it run, so at least 86 minutes of one
core, 1.75 times the run itself; and 1 002 sizes after a refusal, a
child start each (a few seconds in all).

**The second night, the other provers on the growing sets**, as
`bench/comparison.sh --growing` at 60 s: 75 series (a tool on a shape
in its fragment: linlog 25, LinearOne 7, the Maude prover's ILL system,
llprover's ILL, Sympli and APLL's ILL 5 each, the Maude prover's CLL,
Click & coLLecT, llprover's CLL, lolli and APLL's LL 4 each, linTAP 3,
from the check's rows). A series costs at most its decided sizes plus
one run at the limit, and on these families a line's times grow at
least geometrically with its sizes (the fits), so its decided sizes sum
to less than about one limit: at most 120 s a series, 75 × 120 s =
9 000 s, **2.5 hours of one core, about 25 minutes on comparison.sh's
six sequential streams (cores 6 to 11 as on 2026-10-15)**; linlog's
default stream (25 series, at most 50 minutes on its cores) bounds the
night's part at **about 50 minutes**. The check by day (two sizes, 10 s,
136 runs of 12 tools in 4 minutes on two cores) agrees in scale. Step
29's library comparison with 0.2.0's engines, if the night has room, is
its own length, as on 2026-10-15.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-reviewer-45b`)
checked every growing family's verdicts by construction, `copies` and
`least`, and every claim against the engines' documented behaviour and
the verdict rows; one round, six probes on cores 6–7. It found every
argument sound (contraction's Kripke model, the pigeonhole, the trail
lemma of `euler`, the copies of `duplicate` and `reach-fo`, the reused
families) and five findings:

- **`wide-m1` at size 0 is the empty sequent, marked provable** (and
  `wide-m2` to `wide-m4`): holds. Outside every size list, older than
  this session, but a wrong verdict of a family whose claim it reuses;
  fixed: the wide families start at one pair, the refusal test pins it,
  the changelog says so.
- **`reach-fo`'s claim "#1 is refuted by the loop check on two nodes"**
  is false at the family's own copy bound (it refutes only without one,
  at level 10): holds; the claim says #1 ends at its copy bound.
- **`counter`'s claim "the inverse method forced fills its memory"**:
  holds (it runs out of time at 16 tokens, the kept set no longer
  growing); the claim says it does not finish from 16 on.
- **`chain`'s claim "the deepening repeats k + 2 levels"**: holds (the
  proof is at about k/2 + 1); the timed run showed moreover that the
  deepening costs nothing, and the claim says so.
- **`reach-fo`'s `least` is 2 where size 1, a loop, has correct
  instances**: holds; `least` is 1 and the summary says one is a loop.

The reviewer also noted the `lambek-fo` claim leaves out #3 and #4's
recursion limit at 1 024 (incomplete, not false); it now names it.

### Departures and assumptions

- **Sizes by one where the time grows exponentially** (departs from the
  design, which asks for geometric sizes): `mix`'s first sizes,
  `pigeonhole`, `euler`, `reach-fo`, `witness-fo`; one doubling there
  jumps from milliseconds past the limit. The polynomial families double.
- **`euler` on the square of a cycle, not Chaudhuri's complete graphs**
  (departs from the source): the complete graphs' edges grow
  quadratically, and the unprovable shape went from 0.19 s to past the
  limit in one size; the square of a cycle grows by two edges a vertex.
- **A series skips its larger sizes after a refusal or its limit**, as
  rows `skipped` (departs from the design's "each family up to the size
  where every engine reaches the limit", which reads as running every
  size): the orchestrator's conditions of 2026-10-31 (monotone families
  only, the rows marked, never a timeout) are met; it saves about 86
  core-minutes of the first night.
- **No family shows the copy bound's deepening as a weakness** (departs
  from the design's expectation for "a long `!` chain"): on `chain`, the
  deepening without a bound costs nothing beside the family's own bound,
  the memo answering the repeated levels; the figure shows that line
  ("no copy bound") as measured.
- **The net engine's strength is shown past the focused engine's
  limits, beside a faster inverse method** (departs from the design's
  claim "the net engine on wide unit-free MLL"): the rows show the
  inverse method forced faster on `wide-m1` at every size; the claim
  says both.
- **The resumption of answers (42j) has no comparison line**: it has no
  switch, the option never built after the author's decision of
  2026-10-30; `chain-fo`'s claim no longer names it.
- **The first-order memo's family is new** (`reach-fo`, 42i's chosen
  reachability sequents made a family) and 42i's "lifted" rows are not
  reused: they are propositional problems under a vacuous quantifier,
  whose claim would be about the first-order zone's overhead.
- **The plan names llprover's "contraction cap of three"**: llprover's
  guide says its threshold is 5 (per path, contractions and cuts) and
  APLL's bound is 3; the claim of `duplicate` follows the sources.
- **The other provers ran on the first two sizes only** this session,
  at 10 s: their curves are the second night's; the chapter says so.
- **The figures are drawn at build time, not committed as SVG**: the
  brief's correction of the plan, 44b's practice.
- The public API: `Family::claim` and `Family::growth` added to a
  `#[non_exhaustive]` struct (an addition); `Family::least` of the wide
  families 0 → 1 (a wrong verdict fixed). No variant of a closed enum.
  The harness's `HEADER` gained `memo_limit` (an older file is resumed
  into a new one, as for every column added).

### What the next sessions must know

- **45c, the first night**: `bench/growing.sh --out bench/growing/DAY
  --cores 0,1,2,3 --pool-cores 0-3 --timeout 60` beside or after the
  baseline (about 15 minutes so, arithmetic above), from a build of the
  release's commit; then a chapter `results/growing-DAY.md` and two lines
  in `docs/figures.sh`, as this session's, and this session's chapter
  kept beside it as the placing run, or replaced, as 45d decides.
- **The second night**: `bench/comparison.sh --growing --cores
  6,7,8,9,10,11 --default-cores 12-15 --timeout 60 --out
  bench/comparison/DAY-growing` (about 50 minutes, bound by linlog's
  default stream); `ranking` and `plots.py comparison` and `growth`
  run inside it; its `runs.csv` is what the provers' figure of a night's
  chapter draws.
- **A claim is read from rows, never from expectation**: six first
  drafts were wrong (`.claude/rules/core-inputs.md`). When a night's
  rows change a claim's numbers or direction, the claim changes in
  `core/src/families.rs` and in the chapter.
- **New families add no rows to the target sets**, which name their
  families; `batch-families` names its seventeen families.
- **Findings for the dispatch** (in `plan/later.md`): the inverse method
  forced beats the default on `wide-m1`, classical `chain` and
  `pigeonhole`, where no row gives it a turn; `pigeonhole` #1's classical
  refutation comes only after the backward search ends.
- **Pending at the time of writing**: three commits and `nix flake
  check` wait for the signing key (below).

### What is left open

- **The signing key lapsed at 06:40** (the orchestrator's word; `gpg
  --clearsign` waits for its passphrase until the time limit), and the
  permission classifier of this session refused `jj --config
  signing.behavior=drop`, which the brief names for that case. Every jj
  command that snapshots the working copy signs, so the rows, the
  chapter, the figure layout fix and this report are on disk in
  `../linlog-step45b`, uncommitted, and `nix flake check` waits for them
  to be in the git tree (new files reach it only through a jj snapshot).
- The curves of the other provers and the nights' curves of the engines.
- The dispatch findings above, as follow-ups.

### Checklist

- [x] Research by three sub-agents on Sonnet 5.5 at `medium`, the
  sources named above.
- [x] Twelve families with a claim and a growing set, five new, each
  verdict argued at its generator; `verdicts_as_constructed` covers the
  smallest sizes.
- [x] `run --growing` under the default and every engine (the verdict
  run, 570 rows; the timed run, 1 762 rows), the comparison passes
  (`--memo-limit 0`, `--copies none`, the pool), the other provers
  through `bench/comparison.sh --growing --quick` (600 rows, 12 tools).
- [x] `--all-families --timeout 5`: no mismatch.
- [x] The graphs drawn at build time by `plots.py` in both themes,
  listed in `docs/figures.sh`, the fitted growth beside each, the
  chapter `results/growing-2026-10-31.md`.
- [x] The nights' machine time by arithmetic.
- [x] The fresh-context reviewer's round, each finding answered.
- [x] D17: `45b.csv` against `42k.csv`, 0 of 145 decided rows differ;
  `net-45b.csv` against `net-42k.csv`, 0 of 64; the ratchet within its
  ceilings in every gate, `bench/ceilings.csv` untouched.
- [x] The gate passed at each code commit (`lksmzxlu`, its rustfmt-only
  amend after; `wyvuxykp`, `kwrssksr`, `ttktylyk`, `vukmwwzr`).
- [x] `.claude/rules/bench.md` and `core-inputs.md` amended; the
  changelog, the manual's benchmark chapter and status chapter true.
- [ ] The rows, the chapter, the figure layout fix and this report
  committed, the layout fix gated, and `nix flake check` at the last
  commit: waiting for the signing key.
