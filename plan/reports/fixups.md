# Fixups: the side sessions beside the steps

A fixup session takes loose ends that are not a step of the plan:
reviewers' findings not worth a step, proposals the author accepted,
and `plan/later.md` items. It runs in a free slot beside the step chain
and is named `fixup-NN` (the author, 2026-11-20), outside the steps'
numbering. Sessions 54x to 54x9 were the same kind under the number of
step 54; their reports are in `plan/reports/54x-proposals.md`. The count
goes on from there, so the first fixup is `fixup-10`.

## Session fixup-10: KD4's cost

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58a-610ed3`, from the brief
`/tmp/linlog-handoff/start-fixup-10.md` (first named 54x10), on 2026-11-20 and 21. Cores 7-9,
the panel on 13-15. The orchestrator paused it once at 22:33 for the machine's stop and resumed
it the next day.

### Outcome

- **The modal search keeps a cache of world keys across branches in K4, KD4 and S4.** The panel's
  KD4 sequent (`⊢ ◇((q → (◇⊤ ↔ ⊥)) → □□(p → q)), (◇¬◇(s ↔ q) → ⊤) → ◇(((p ↔ □□s) ∧ ◇(◇s ↔
  ◇q)) ↔ s)`) went from 20 469 073 units (3.8 s) to 58 100 units (4.5 ms). Its countermodel
  is checked.
- On three generated sets (64 500 decisions), every verdict is the same. In the other twelve
  systems every row's units are the same. In K4, KD4 and S4 the totals fall in units and in time
  on every set, but single rows got slower, up to 29 times on small ones (below).
- The panel found no unsound answer. It found three bound faults, all fixed (`tnpoypmo`,
  `vksyopzx`): an uncharged graft, an unpolled n³ copy, and an uncounted quadratic merge.

### The research and the choice

Read for this session, by a research sub-agent (Sonnet 5.5, high), from the papers' texts:
- **Goré and Nguyen**, "EXPTIME tableaux with global caching for description logics with
  transitive roles, inverse roles and role hierarchies", TABLEAUX 2007 (LNAI 4548): an and-or
  graph with one node per exact content (Alg. 2, step 2g; Prop. 1). Transitivity is handled by
  copying the transitive role's universal formulas into the successor, so a node's content
  decides its subproblem. Statuses are set only on irrevocable evidence (Fig. 5: an and-node is
  unsat when one successor is), and the rest is sat at the end: unsat is a least fixpoint, sat
  the greatest (Lemmas 5-7, Thm. 2, Cor. 1 EXPTIME).
- **Goré and Widmann**, "Sound global state caching for ALC with inverse roles", TABLEAUX 2009:
  "Caching unsatisfiable nodes is sound across different and-trees, but caching satisfiable
  nodes can only be done within the same and-tree." Their CADE 2009 (PDL) and IJCAR 2010 (PDL
  with converse, Def. 4) give an open node its dependencies (`prs`, potential rescuers among
  its ancestors) and keep "closed" irrevocable.
- **Horrocks and Patel-Schneider**, AISC 1998, §3: cache the satisfiability of a node by its
  initial label, "completely determined by this set" when no inverse roles exist. **Horrocks
  and Sattler** (arXiv cs/0005013, §6.5): that fails with inverses. **Ding and Haarslev**, DL
  2006, Lemma 1: subset reuse of a sat entry under a back-propagation analysis (inverse roles).
- **Nguyen and Szałas**, arXiv 0904.0721 (PDL), Alg. 1″, Thm. 6.7, §7: or-nodes by
  backtracking, simple nodes cached globally across it; a fully expanded subgraph is decided
  whatever the rest. This is the closest match to a depth-first search over branches.
- Not read: Goré, Kupke and Pattinson (TACAS 2010, IJCAR 2010; abstract only), Donini and
  Massacci (AIJ 2000; seen only as cited by Goré and Nguyen: unsat sets cached for good, sat sets
  only on the current branch). No paper on global caching specific to K4, KD4 or S4 was found.

The choice, with the alternatives argued:
- **The key.** In K4, KD4 and S4 nothing flows back or sideways, so a world's own formula plus
  the `□` true and `◇` false that reach it decide its part of the branch: Horrocks and
  Patel-Schneider's "initial label". Keys are matched exactly, as Goré and Nguyen's nodes are.
  Subset reuse (sat) and superset reuse (unsat) are sound by monotonicity but need a scan of the
  entries per new world; exact matching found 57 to 78 keys behind thousands of worlds on the
  sequents measured, so it was enough.
- **Per search, not per branch.** Both sides live as long as the search. Per branch would lose
  exactly the repetition across branches that costs KD4.
- **Unsat entries.** Sound across branches, as Goré and Widmann state. An entry is taken from a
  closed part whose needed set stays inside the key (`learn`'s argument). The catch here is
  ours: a "valid" must come with a derivation of calculus rules, and a branch closed by the
  cache has none. Replaying the earlier proof under a renaming does not work in general, because
  the boxes the key holds may sit at other worlds in the new context, so the derivation would
  need other `Trans` steps. Instead, a proof that rests on an unsat entry is searched again
  without them. The verdict then never rests on the unsat side, and that side can only cost
  time.
- **Sat entries.** Sound only once final (Goré and Widmann; Nguyen and Szałas §7). Here an
  entry is final when the world's part is finished and every waiting world in it is blocked
  inside the part, so its model needs nothing outside. Entries that depend on a blocker outside
  (Goré and Widmann's dependencies, Goré and Nguyen's greatest fixpoint) were not built: on the
  sets measured the finished-inside entries were enough. Each sat entry stores its worlds as a
  model fragment. The countermodel grafts it, and the model check decides the result as before.
- **The order.** A finished part is well defined only if a world's part is searched before the
  search leaves it. The oldest-first order of 65b interleaves all parts, so in K4, KD4 and S4
  the worlds are now made in depth. Two walks were measured. Making all of a world's witnesses
  before entering any of them was worse on every set (below), so the newest unfinished world
  acts next.

### What landed

Four commits on `main@origin`: `znvtknry` (the cache, its tests, `core-modal.md`, the changelog,
the `later.md` section removed), `tnpoypmo` (the integers member's fixes) and `vksyopzx` (the
argument member's fix, and two docs it found overstated). All of it is in
`core/src/modal/search.rs`. Only K4, KD4 and S4, the systems that block, take the new path.
- **Why only those three.** The sat side needs a part that depends on its key alone, and in the
  symmetric and euclidean systems formulas flow back to older worlds. The other systems have no
  repetition that costs: they end by modal depth or in one cluster, and none of them showed
  KD4's cost on the panel's 630 150 decisions.
- **The walk in depth** (`deepest`, `make`, `Branch::open`): the newest unfinished world makes a
  world for its first demand, or by `Ser`, unless it is blocked. A world that makes none is
  popped, and its part is it and every later world.
- **The key** (`keyed`): the own formula, and the `□` true and `◇` false at the maker and at the
  worlds that see it.
- **The sat side** (`keep`, `Entry`, `Piece`, `Blocked::Cached`, the graft in `model`):
  - An entry holds direct arrows, and a part kept on this branch stands as its entry.
  - Every waiting world must be blocked inside the part.
  - Entries are charged, taken only where `Account::spares` allows, and never released.
  - Grafts of one entry share its worlds, and their atoms are charged.
- **The unsat side** (`learn` in `fold`, `refuted`, the `cached` step) and the rerun (`run`,
  `rests_on_cache`).
- **Tests** (`search_tests.rs`):
  - `a_finished_world_stands_for_a_world_with_its_key`: a hit, its countermodel checked, in
    the three systems;
  - `a_key_that_closed_once_closes_again_and_the_proof_is_made_anew`: an unsat hit, then the
    rerun and a checked derivation;
  - `kd4_decides_the_panels_sequent_by_the_cache`: within 2²⁰ units.

### Measurements

Every figure is from release builds on one core (7), from a scratch harness (`decide_within`,
work bound 2²⁴, in the scratchpad). The baseline is the library at `okrtouwo` (main@origin when
the session started).

**The panel's sequents** (the command, `--stats`):

| sequent | system | before: units, time | after: units, time |
|---|---|---|---|
| the 37-node witness above | KD4 | 20 469 073, 3.85 s | 58 100, 4.5 ms |
| same | S4 | 38 903, 6.0 ms | 15 403, 1.7 ms |
| `⊢ □(□□q ↔ r) → ◇□(□◇q ↔ r)` | KD4 | 104 708, 13 ms | 5 071, 0.4 ms |
| same | S4 | 7 516, 0.6 ms | 2 123, 0.3 ms |
| `□(□(p ↔ q) ↔ □q ∨ p) ⊢ ◇□◇(p ∧ q), q` | KD4 | 162 207, 31 ms | 5 286, 0.4 ms |
| same | S4 | 1 813, 0.3 ms | 1 007, 0.2 ms |

In K4 all three are decided in 4 to 7 units, before and after.
The 50-node original of the witness was not on disk, so it was not measured.

**Generated sets** (random formulas over four atoms; `gen:SEED:N:DEPTH`, one or two formulas
right of `⊢`). Each cell gives the total units and the total time in milliseconds,
before → after:

| set | K4 | KD4 | S4 |
|---|---|---|---|
| `gen:2:3000:5`, 15 systems | 275 903 → 215 991; 31.7 → 26.4 | 781 742 → 507 707; 85.7 → 54.6 | 369 857 → 321 716; 52.8 → 48.6 |
| `gen:3:1000:6`, 15 systems | 799 608 → 210 067; 57.7 → 14.5 | 8 544 342 → 546 216; 572 → 38.3 | 1 228 060 → 244 854; 71.1 → 24.4 |
| `gen:4:1500:7`, three systems | 1 608 417 → 841 913; 166 → 54.5 | 4 031 752 → 3 149 282; 371 → 172 | 2 511 686 → 829 599; 196 → 63.5 |

- **Verdicts:** none differs among the 64 500 decisions. No countermodel the old code had is lost
  (no row is "not valid" without a model where it had one).
- **The other twelve systems:** every row's units are equal. Their times moved 5 to 15 %
  downwards because they ran later on a quieter core, which is noise.
- **Single rows got slower**, most by a few units: the keys' scans are counted.
  - `gen:2`: 1 533, 2 263 and 1 802 of 3 000 rows in K4, KD4 and S4.
  - The largest ratios are on small rows:
    - row 275 of `gen:4`: KD4 1 382 → 39 654, S4 961 → 27 909;
    - row 916: K4 390 → 8 589;
    - row 266: KD4 47 182 → 556 737 (a deep part searched before a sibling that closes at
      once, which the old breadth-first order met first);
    - row 61 of `gen:3`: KD4 and K4 695 → 3 260 (valid, a rerun).
- **Two walks were measured before one was kept.** Walk A, kept: a new world acts at once. Walk
  B: a world makes all its witnesses before entering any. B was worse everywhere:
  - `gen:3`, KD4: 3.05M units for B, 0.58M for A;
  - `gen:4`, KD4: 7.05M for B, 3.50M for A;
  - `gen:2`: B above the baseline in units in all three systems.
- **Without the unsat side** (walk A, sat side only), row 275 of `gen:4` took 576 078 units in KD4.
  There 78 keys repeated over 2 801 worlds, and no sat entry was hit, so the unsat side was added.
- **Chains** (the integers member's witnesses, K4, before → after fixes, against the old code):
  - 1 000 worlds of 300 atoms: 282M units, 6.4 s, against 253M, 6.3 s;
  - 2 000 worlds: 2.09G, 57 s, against 1.90G, 56 s;
  - longest gap between polls: 5 ms or less.

### The panel's findings

On `znvtknry`, then `tnpoypmo` for the members still running. All three ran at `high`, on cores
13-15.

**`panel-integers`** (Sonnet 5.5, core 15) found three bound faults, all fixed in `tnpoypmo`:
1. **A graft's atoms were neither charged nor counted.** With 400 worlds grafting one entry of
   25 000 atoms under 64 MiB, the search peaked at 688 MB and answered "not valid" without a
   model; the old code refused at the bound.
   - Now each grafted world's atoms are charged at 48 bytes each (the names a model holds) and
     counted.
   - The same input is refused at the bound, at a peak of 140 MB (106 MB of it the input).
2. **The walk's unwinding was unpolled, and `keep` copied each part's closed arrows**: n³/5
   units on a K4 chain, paid in one tick at the end. The longest gap was 1.4 s at 1 000 worlds and
   4.3 s at 2 000, and a 300M work bound stopped at 477M.
   - Now an entry holds direct arrows, and a part kept on this branch stands as its entry, so
     a world is copied once along a branch.
   - The walk ticks after every pop.
   - The longest gap is now 5 ms or less; the units and time are within 11 % and 2 % of the old
     code.
3. **Entries were charged 1.2 to 1.35 times too little** (vector capacity, the struct, the
   allocator). They are now charged with capacities and per-piece overhead.
- **Checked sound:** the `own` encoding and its decoding, the indices, keys sharing one `Rc`,
  the rerun's cumulative work and its refusal at every work bound from 20 to 66 on a rerun
  witness, and `rewind` to exactly the entries.
- **Noted, a refusal and not a fault:** a 1 000-world chain of one atom takes 452M units against
  238M, so a 300M work bound now refuses where the old code decided.

**`panel-counterexamples`** (Opus 5.5, cores 13-14): **no refutation.**
- About 1.08 million decisions over both commits, against the old code, its own Kripke
  evaluator and frame check (150 random frames of 1 to 5 worlds per "valid"), the order of the
  systems, and panics and defect errors.
- On `znvtknry`: 31 500 decisions used the cache, 5 400 reran, 9 300 models shared a graft, and
  2 700 had entries within entries.
- On `tnpoypmo`: 13 000 models had shared grafts and 12 000 nested entries.
- No wrong verdict, no model failing its check, no defect.
- **One decision the old code made is now refused:** a valid KD4 sequent (`⊢ ◇((q → (◇s ↔ ⊥))
  → ¬□□(r → s)), ((◇¬◇(r ↔ s) → □q) → ◇(((r ↔ □□q) ∧ ◇(◇p ↔ ◇p)) ↔ (s ↔ q)))`). Its rerun's
  derivation has 2 233 inferences against the old 444, so at work 2 000 000 the derivation's
  check passes its own work bound (`Refusal::Work`, allowed by `decide_within`'s docs). At 2·10⁷
  it is valid. Recorded under what is left.

**`panel-argument`** (Fable 5.2): **no way to a wrong verdict**, a failing model or a `cached`
step in a derivation, in either commit.
- **Confirmed:** the key's sufficiency for formulas; the walk's part being exactly the
  descendants; the sat entry being a model of the key in any context (shared grafts a
  bisimulation, nesting well founded); `learn`'s flattening argument; the rerun; the twelve
  other systems unchanged.
- On `tnpoypmo`, answering my two questions: direct arrows closed by the worklist give the
  branch's relation within a part, and a kept part standing as its entry keeps the ancestors'
  demands realised. A kept world's entry is built from this branch's own worlds; a world whose
  `keep` found the key taken is copied world by world.
- **Finding 1, fixed in `vksyopzx`:** a waiting world's borrowed arrows were deduplicated by a
  scan per arrow, counted as linear and unpolled inside `keep`. With 16 384 waiting worlds and a
  blocker of 4 096 children, the longest gap was 6.4 s.
  - `keep` now merges two sorted lists and asks the stop after each waiting world.
  - The gap on that input is now 17 ms; on `r` = 2 048 it was 1.9 s and is 14 ms.
- **Finding 2, in `znvtknry` and closed by `tnpoypmo`:** the candidate entry was built uncharged.
  The peak was 1.48 GiB under 1 GiB, and is now 369 MiB.
- **Docs that overstated the key:** "two worlds with one key start the same search" is true of
  formulas, not of blocking. Corrected in `vksyopzx`, in `keyed`'s doc and the module doc.
- `learn`'s doc leaves out two facts that hold: a later sibling in the proof writes a `◇` true
  or `□` false at an older world, which the test refuses; and a `cached` leaf's older edges
  collapse to `o R o`, which drops out as `Trans` does.

### The reviewer's findings

`fresh-reviewer` (Fable 5.2) read the chain `main@origin..vksyopzx`, the brief and the code, and
ran a differential test against the old `search.rs`: about 66 000 sequents, all fifteen systems,
and memory bounds of 40 000 and 200 000 bytes. It found no wrong verdict and no `cached` step in a
derivation. Three findings:
1. **The other twelve systems refused at a memory bound where they decided before.** `bytes()`
   charged every system's split clone for a key (`48·words + 104` a world for `32·words + 80`).
   - In T at 200 000 bytes, a sequent went from "not valid" (1 889 units) to `MemoryLimit`;
     36 of 72 000 rows were alike.
   - Held. The estimate is as before outside K4, KD4 and S4 (`mmtmpstq`), and the witness decides
     again: "not valid", 1 889 units, model checked.
2. **The report section was not in the chain yet.** Held: this section, committed with the chain.
3. **A test's doc comment and name referred to the panel** (writing rule 2). Held: renamed
   `kd4_decides_a_sequent_of_repeated_worlds_by_the_cache`, its comment rewritten (`mmtmpstq`).

### Checks

- **Modal tests:** 32 pass (`cargo test -p deduxit --lib modal`), the three new ones among them,
  after every commit.
- **Workspace:** `cargo test --workspace` passes at `znvtknry` (README's, the manual's and the
  guide's examples included: no printed output changed).
- **Clippy** with warnings denied: clean after every commit.
- **The gate** (capped, cores 7-9) passed at `znvtknry` and again on the rebased chain at
  `vksyopzx`, its ratchet within the ceilings. The misses note on `search-chain-128` and
  `check-qbf-20-2` is not a failure, and those journeys are not modal.
- **`nix flake check`** under the machine lock: "all checks passed!" at the report's commit
  `wlktzrpv`. The first run failed on `treefmt` alone: edits made from the shell had skipped the
  formatter. `nix fmt` changed three lines of `search.rs`, folded into `mmtmpstq`.
- **The panel's and the reviewer's witnesses** were rerun on the fixes, with the figures above.

### Departures

- **A walk in depth in K4, KD4 and S4** (departs from the design, which kept 65b's oldest-first
  order). A finished part needs it. It costs single rows up to 29 times on small sequents, though
  the totals fall.
- **A rerun without the unsat side** where a proof rests on it (departs from the design: the
  brief asked for the cache, not for a second run). It is ours, so that every "valid" keeps a
  derivation of calculus rules.
- **Exact keys only**, no subset or superset reuse, and no sat entry that depends on a blocker
  outside its part (the research's alternatives, argued above).
- **No public counter** of cache hits. `Statistics` is unchanged, and `Counts::cached` and
  `Counts::refuted` are crate-private, read by the tests. A `--stats` line would change the
  command's output, the manual and the guide; it would be a question for the author.

### What is left

- **Rows the walk makes slower**: the sequents above, and the valid KD4 sequent whose rerun's
  derivation is five times the old one's (a refusal at work 2·10⁶ where the old code decided).
  - A walk that met a closing sibling first without breaking the finished parts would answer
    both.
  - So would a replay of an unsat entry's proof under a renaming. It needs other `Trans` steps
    when the key's boxes sit at other worlds, which is why it was not built.
  - Neither is in `plan/later.md`: they are for when a user meets one.
- **The 50-node original** of the panel's witness was not on disk and was not measured.

## Session fixup-11: the renderers' module

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58a-610ed3` and the client's checkout,
from the brief `/tmp/linlog-handoff/start-fixup-11.md` (`plan/later.md`, "The web module's size
after the workspace", its second point), on 2026-11-21. Cores 7-9, the client's e2e on 13-15.

### Outcome

- **deduxit** (`nvpuqykz`): the PNG and PDF renderers are a WebAssembly module of their own.
  - `web-render/` is the crate `deduxit-web-render`, a cdylib with the one call `render(svg,
    format, font, settings)`, the old `calls::render` moved whole: the same settings read over the
    same tab's defaults, the same error JSON, its tests moved with it plus one for its failures.
  - `web-tab/` is `deduxit-web-tab`, a library alone: the tab's defaults and overlay (`tab.rs`
    less `STACK_BYTES`) and `Failure`, which both modules need. `web/src/tab.rs` re-exports it and
    keeps `STACK_BYTES`, which its `build.rs` sets.
  - `web/` drops `render` and the library's `png` and `pdf` features.
  - `modules/web.nix` builds each module by a cargo of its own (built together, cargo would
    unify the library's features and compile the bindings with `png` and `pdf`), puts both
    modules and their glue in the package `deduxit-web` with one `THIRD-PARTY-NOTICES.txt`, a
    section per module, and runs the three crates' tests under Node and Clippy for wasm32;
    `web-features` validates both modules. `modules/budget.nix` checks each module under its own
    ceiling. `modules/workspace.nix` adds the crates to the `web` part and selection, so
    `clippy-web`, `test-web` and `test-debug-assertions-web` cover them.
  - Docs: `web.md`, `flake.md`, CLAUDE.md, the manual's architecture table and status page, the
    guide for AI models, the plan's call table and the changelog.
- **The client** (`stkwzklo`): the worker imports `deduxit_web_render.js` on the first `render`
  request and keeps the promise (`loadRenderers`); one that fails to load is dropped, so the next
  export tries again, and the request is answered with `the PNG and PDF renderers did not load:
  …`, which the export menu shows as it shows every failure. The service worker puts the
  renderers' two files in `LATER` (the build fails unless it finds two), so it does not fetch
  them at install and caches them when first asked, as it does a chosen font. The new e2e test
  in `tests/offline.spec.ts` exports SVG, LaTeX, Text and Session under the service worker and
  finds no request for either file and none in its cache; then PNG, PDF and PNG fetch each file
  once, and the cache holds both. The About dialog's notices line says "modules".

### Why a shared crate

The renderers need the tab's settings reader and the error form. Three ways were tried or
weighed:
- **A dependency on `deduxit-web`** would link every `#[wasm_bindgen]` export of the bindings
  into the renderers' cdylib (rustc exports every upstream `#[no_mangle]` symbol of a cdylib),
  and gating them behind a feature breaks as soon as cargo unifies features.
- **`#[path]` includes of `tab.rs` and `failure.rs`** worked, but cargo-shear (the `shear` check)
  does not follow them: it called `serde_json` a misplaced dependency and listed the bindings'
  files as unlinked. Dropped.
- **A crate both depend on** (`deduxit-web-tab`) is the usual way, and the one taken. Its cost
  is a third manifest and `STACK_BYTES` staying in the bindings.

### Measurements

The modules at `gzip -9`, from the nix packages:

| | the bindings' module | the renderers' module | both |
|---|---:|---:|---:|
| before (`main@origin`, `nklpmrvn`) | 3 913 893 | none | 3 913 893 |
| after (`morwkrov`) | 2 438 116 | 1 639 598 | 4 077 714 |

Raw after `wasm-opt`: 10 979 011 bytes before, 7 192 062 and 4 382 359 after. The two modules
together are 164 KB more than the one before: what both carry (the standard library, serde and
serde_json, the reader of the whole settings and the library's error writing). The renderers'
module was 1 609 148 until the reviewer's finding below made it read every key of the settings.
It is resvg, usvg, krilla, skrifa, rustybuzz, write-fonts, zune-jpeg, the PNG decoder and the
filters; the tab's settings reader was 20 KB raw of it before that finding (twiggy). The brief's figure of 3 903 905 is the day before; `main@origin` had 3 913 893.

The budget's ceilings: the bindings' 2 500 000 (from 3 950 000), the renderers' 1 700 000; the
lock's packages 175 (two crates more), under 180.

The worker's time to its first answer (the island's `about` on mount), by a Playwright script
(not committed; it wraps `Worker` in an init script and takes the time from the worker's creation
to its first message), each run a fresh browser context with service workers blocked, Chromium on
cores 13-15, the sites built by nix (the client at `vyxlolom` with deduxit at `nklpmrvn`, and
the client at `stkwzklo` with deduxit at `nvpuqykz`). The client has no measure of its own.

| served | before, median of 5 | after, median of 5 |
|---|---:|---:|
| from disk, uncompressed (twice) | 102 ms, 105 ms | 84 ms, 86 ms |
| at gzip -9, 10 Mbit/s per response | 3 176 ms | 1 992 ms |

Runs: from disk 129, 101, 110, 102, 90 and 123, 105, 105, 103, 91 before; 120, 83, 89, 83, 84
and 108, 86, 82, 86, 84 after. Throttled 3198, 3176, 3176, 3175, 3176 before; 2013, 1990, 1992,
1990, 1992 after. The throttle is per response, so parallel requests each get 10 Mbit/s; the
module is the largest by far, so the figure is about its download. The timings were taken at
`nvpuqykz`; the later commits change only the renderers' module, which the first answer does not
load.

### Commits

- deduxit: `nvpuqykz` the renderers' module, `unyuwpyu` the same settings in both modules (the
  reviewer's finding), `morwkrov` the renderers' ceiling, then this report.
- The client: `stkwzklo`.

### Checks

- deduxit: the gate passed at `nvpuqykz` and at `unyuwpyu` (cores 7-9). `web-package`, `budget`,
  `web-features`, `web-wasm-tests` and `web-clippy-wasm32` passed at `nvpuqykz`; `web-package`,
  `web-wasm-tests` and `budget` again at `unyuwpyu`. `nix flake check` passed at `nvpuqykz` and
  at the head (below).
- `a_feature_s_keys_are_read` was seen to fail with the tab crate's first feature list
  (`serialize`, `svg`) and to pass with the bindings' list.
- The client, against this workspace (`--override-input deduxit git+file:///…`): `lint` (Biome
  and `astro check`, no errors), `tests` (716 passed), `e2e` in the dev shell on cores 13-15 (365
  passed, the new test among them), and its `nix flake check` (all checks, `e2e`, `licences` and
  `csp` among them) at `stkwzklo` with deduxit at `unyuwpyu`.

### The reviewer

`fresh-reviewer` (Fable 5.2, medium) on both chains:
- **Finding: the renderers' module refused settings the old `render` took.** It built the library
  with `png`, `pdf`, `serialize` and `svg` alone, and `Settings` refuses an unknown key: without
  `interactive` the key `interactive` is unknown, without `sat` the method `sat`. The tests did not
  see it, since the renderers' tests depend on the bindings, which turn every feature on.
  Answered in `unyuwpyu`: `deduxit-web-tab` takes the bindings' features but `png` and `pdf`, and
  its test `a_feature_s_keys_are_read` runs under Node in a cargo of its own, where the features
  are the tab crate's alone. The page sends neither key to `render` today.
- **Question: a failed dynamic `import()` may stay failed for the worker's life** in some
  browsers, so "the next export tries again" holds for the module's file but perhaps not for its
  glue. Not tested here; the client's rules and the loader's comment now say so. The worker is
  never wedged: the request gets its error answer, and a cancel or a crash starts a new worker.

### Departures and what is left

- **The tab's settings and the error form moved to a crate of their own**, `deduxit-web-tab`, a
  third crate the brief did not name (departs from the design: the brief named a crate beside
  `web/` or a second target of it). A second cdylib target of one package does not exist in
  cargo, and the reasons for the shared crate are above.
- **The renderers' module keeps an 8 MiB stack** (`web-render/build.rs`), the bindings', since
  the renderers ran on it inside them; it costs nothing at download.
- Not done: a throttled timing inside a real browser profile with the service worker (the
  measure blocks it, so every run downloads); a measure on a phone.

### What the client's lock bump needs

The client's commit `stkwzklo` needs deduxit's package with `deduxit_web_render.js` and
`deduxit_web_render_bg.wasm`: against the pinned deduxit its worker's import fails to resolve
(`astro check` reports it, and the build fails). So the lock moves to a deduxit `main` that holds
`nvpuqykz` to `morwkrov` in the same push as `stkwzklo`, or before it: `nix flake update deduxit`
in the client, then its `nix flake check`. The client's pin is behind deduxit's `main` today
(`c40a994f`, 0.6.0's package); its checks passed here against 0.7.0 and this chain, so nothing
else in the bump is known to break.

## Session fixup-12: three latent faults

Three entries of `later.md` that a reviewer had found by reading. Each now has a test: two were
faults and are fixed, the third no longer held and a check now holds it. Opus 5.5 at high, cores
7-9, on `main@origin` `yunvyupu`.

### What each test showed

- **A first-order table with binders and no atoms: a fault, fixed in `upmnxlsv`.** A JSON sequent
  `⊢ ∀x. (a ⅋ ~b)` written with plain atom names and a `binders` key has `fo.atoms` empty and
  `fo.binders` not, so `FirstOrder::is_empty` is false. `join_first_order` then took its own atom
  count from `self.fo.atoms.len()`, which is 0. With `⊢ p(c), q` added, it printed
  `⊢ ∀x. (p(c) ⅋ ~q), p(c), q`: the sequent's atoms read the other side's table. `add_cut` is
  the same. The test `a_table_of_binders_alone_joins_by_its_atoms` (`core/tests/first_order.rs`)
  failed so before the fix. It covers `add` both ways round and `add_cut`. Neither side's tables
  can tell the count, so `add` and `add_cut` now pass their atom count from before the extension.
  The other side's count is then the rest. A sequent joined to such a table the other way round
  kept its names before too (an atom without a table entry gets its own name), and the test pins
  that.
- **LJ's one-premise ¬L in the Rocq writer: a fault, fixed in `pytzrzwn`.** The derivation
  `a ⊢ ¬¬a` (axiom, LK's one-premise `¬L` to `a, ¬a ⊢`, `¬R`) read from a level-8 document passes
  `Derivation::check` in LJ and in LK. In LJ the writer wrote `let k : ~ a := h in (premise)`,
  LK's continuation, of type `a` where `False` was due. `nix run .#check-certificate` refused it:
  "The term "h'1" has type "a" while it is expected to have type "False"." The LK certificate
  passed ("proved, with the excluded middle (classic) as its one axiom"). The LJ arm is now
  `h (premise)`. Rocq accepts it ("proved, with no axiom"), and the LK certificate is byte for
  byte the same. The test `ordinary_certificate_of_the_unary_negation_rules`
  (`core/tests/export.rs`) pins both as `core/tests/snapshots/ordinary_negation_{lj,lk}.v`, which
  the flake's `rocq` check compiles. Minimal logic refuses the form (its right side would be
  empty). The lambda read-back (`lambda/simple.rs`) already applied `¬A` to the premise's term.
- **`cargo test -p deduxit` without `parallel`: no warning left, a check added in `sxklpwvs`.**
  `cargo clippy -p deduxit --all-targets -- --deny warnings`, `cargo test -p deduxit --no-run` and
  the same Clippy with `--no-default-features` printed no warning at `yunvyupu`. The warnings in
  `search/focus/tests.rs` and `search/reference_fo.rs` were fixed since step 49a. Most likely the
  `wasm` check kept them out: it runs Clippy with `--all-targets` and every feature but
  `parallel`, warnings denied, for wasm32. Nothing held the host's default build. So a flake check
  `clippy-core-default` (Clippy on the core crate and its tests with the default features, from
  `core` and `core-tests`) and the same line in the gate now hold it, so the full check still holds
  every part of the gate. It built in 13 s on the shared dependencies, and the gate line took
  about 15 s on warm dependencies. CI runs it in the group `rest`.

### The reviewer

`fresh-reviewer` (Fable 5.2, medium) on the three commits, from reading (the gate held the target
directory): no finding that breaks correctness or the brief. It traced the old `join_first_order`
to the changelog's wrong text. It also argued that a one-premise `¬L` in LJ always has an empty
right side (its premise holds `A` right of `⊢`), so `h (premise)` is a term of `False` wherever
the checker accepts it. It noted that the report section was still to come, and that the "about
15 s" was unverified. That figure is the measured Clippy run above.

### The checks

- The gate (with its new line) at `sxklpwvs`: passed. Before that it passed at `upmnxlsv`. A run
  at `pytzrzwn` failed on the new snapshot test, whose standalone certificate ends with a newline
  that the snapshot helper trims from the file only. That is fixed in `pytzrzwn` (`trim_end`).
- `nix build .#checks.x86_64-linux.clippy-core-default`: passed.
- `nix run .#check-certificate` on both certificates, before and after the fix, as above.
- `nix flake check` at `sxklpwvs`, under the machine lock: all 47 checks passed, among them
  `rocq`, which compiled the two new snapshots, and the new `clippy-core-default`.

### Left open

Nothing of the three entries. The signing hook signed every commit.

## Session fixup-13: TPTP names and the command's costs

Opus 5.5 at `high`, the conversation of 74c and 74d, in the workspace
`../deduxit-wt-step-58b-8c0bbc`, cores 10 to 12, on 74d's `zyvpyomm`.
Nothing was pushed.

### Outcome

The three items landed, one commit each:

| change | subject |
|---|---|
| `ovstsxkv` | Rename TPTP names this syntax keeps for itself, or uses in two roles, on reading |
| `kwxpkskr` | Leave out an ordinary derivation's tree in check --tree never |
| `mmktqzrk` | Inline a TPTP problem's includes in the command |
| `sztmxwxm` | Answer the review of fixup-13 |

- **Item 1, TPTP names renamed** (the planning session's decision of
  2026-11-21): `ordinary::read_tptp` reads a file again when its reading
  was refused for a keyword name (`Error::AtomName` of an identifier:
  `par`, `true`, `exists` …) or for a name in two roles
  (`SymbolFault::Role`), with that name renamed: a keyword gets `_`
  after it, a function symbol `_f`, each with more `_` until no word of
  the file is it (`Renames::fresh`). One name more per try, so the tries
  end. The parser applies a keyword's new name only in the atom's error
  path (`renamed_atom`) and in the cold first-order paths
  (`symbol_name`), a function symbol's in term position alone, so a file
  that needs no renaming reads exactly as before. Tests: `reads_tptp`
  (a keyword atom, a keyword argument whose `_` name is taken, both
  orders of a function and a predicate of one name, a keyword predicate
  under a quantifier, each printed and read back in the native syntax)
  and `false_is_no_atom_of_the_sequent`, whose refusal of `false` is
  now a renaming to `false_` (the behaviour the test pinned was meant to
  change; its point, that the atom never is minimal logic's false,
  holds). **The LLTP reader renames nothing** (departs from the brief,
  which named both readers): its syntax is the linear one, where `top`
  and `bot` are the units LLTP's own files write (`lcm_*.p`,
  `nonconserv*.p`), so renaming them would change those problems.
- **Item 2, `--tree never`**: the size estimate was not the cost: `check`
  of an ordinary derivation ignored `--tree never` and wrote the whole
  tree (the size is computed only for a terminal's fit, which `never`
  turns off). `check_derivation` now leaves the derivation out under
  `never` before any view of it is made, as `prove` already did. A deep
  derivation (5 000 nested `∀R`, 700 KB) checked with `--tree never` in
  6.9 s in a debug build, where it ran past 300 s before (the check
  itself is the 6.9 s); the output without the flag is unchanged
  (`derivation_limit_and_tree_switch` pins the verdict line alone).
- **Item 3, `include` in the command**: `io::included` inlines each
  `include('NAME').` from `$TPTP`, where set, then the problem's
  directory and each above it (the working directory's for a problem
  given as the argument or on standard input), on a stack of its own,
  refusing a missing file by name, an include of itself through others
  and a nesting past 32; the batch inlines per entry with the entry's
  path. `tptp_includes_are_read` pins the three cases. The manual's
  TPTP section and the guide say so.

### ILTP's first-order problems, before and after

`deduxit-bench run --tptp bench/iltp-first-order/Problems --logics
intuitionistic,classical --engines auto --timeout 2`, three shards on
cores 10 to 12:

| rows | at 74d | after the renaming |
|---|--:|--:|
| refused, equality | 3 356 | 3 460 |
| refused, first_order | 1 580 | 1 732 |
| proved | 103 | 104 |
| unprovable | 34 | 34 |
| unknown, timeout | 3 | 2 |
| error, a keyword name | 156 | 0 |
| error, a name in two roles | 104 | 0 |
| error, a variable no quantifier binds | 4 | 8 |

No verdict disagrees with a status. The 130 problems refused for their
names read now (76 reach the first-order refusal, 52 the equality one,
and two more show the library's own fault): the four left with errors,
GEJ056+1, GEJ056+2, GEJ057+1 and GEJ057+2, use a variable outside its
quantifier in their conjecture (`(! [L,M] : …) => (? [X] : … apt(X,L)
…)`), a fault of the files. The last column is the run after the
review's answers; one run between them had one problem more at the time
limit, by day.

### The measurements (D17)

The ratchet at `ovstsxkv`: `read-tptp` 33 667 473 → 33 695 629 (+0.08 %;
0.99 % over its ceiling, inside the tolerance), the atom's `?` turned
into a `match` whose error arm is the renaming; `read-lltp` 28 412 648,
unchanged. `read-json` read 12 283 897 where 74d's gate read 12 226 553:
the base moved under the chain (fixup-12 on `main`), and no change of
this session touches the JSON reader.

### The reviewer's round

`deduxit-review-fixup-13` (`fresh-reviewer`, Fable 5.2 at `medium`)
found that the LLTP reader's reason for renaming nothing holds, the
retries end, `$true` and `$false` stay, function symbols are renamed in
term position alone, the include reader is a bounded stack honouring
`$TPTP`, and `--tree never` changes nothing else. Its five findings,
each answered in `sztmxwxm` with a test:

1. **A keyword that is also a constant** (`par | q(par)`) stayed
   refused: the second retry's key was the keyword's new name, which the
   lookup by the file's word never found. A function's new name is now
   looked up under the keyword's new name (`⊢ par_ ∨ q(par__f)`).
2. **Freshness compared raw words with names in NFC**, and split words
   at combining marks: `é` written decomposed could make a new name equal
   to the file's other function. Words are cut by `XID_Continue` and
   compared in NFC, and every lookup is by the NFC name.
3. **`check --lambda --tree never` lost the lambda-term line**: the early
   return came before it. The tree alone is left out now.
4. **A problem named relative to the working directory** (`--file x.p`)
   never searched the directories above its own: the path is made
   absolute first.
5. **A test's doc comment on the wrong test** (74c's insertion put
   `ordinary_session`'s above the tests inserted before it): moved
   back.

### What was checked, and by what

- The gate at `ovstsxkv`, at `mmktqzrk` (which covers `kwxpkskr`, whose
  own run picked up item 3's edits half-way and failed on them) and at
  `sztmxwxm`, capped on cores 10 to 12.
- `nix build` of the checks `manual` and `manual-links` at `mmktqzrk`.
- The ILTP pass before and after (above), and the deep derivation's
  `check --tree never` timed before and after.
- `nix flake check` at the last commit, after the rebase: in the
  session's last message.
