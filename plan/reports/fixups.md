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

## Session fixup-14: the session's leftovers

Opus 5.5 at `high`, the session that was 74e, in the workspace
`../deduxit-wt-step-58a-610ed3`, cores 7 to 9, on `ktsrqsps` (74e), rebased at the end onto `wrnxnzrk`.
Nothing was pushed.

### Outcome

| change | subject |
|---|---|
| `yqsnzmmn` | Show a binding only through the inferences that hold its witness |
| `wsklvkyy` | Hold a typed witness and every instance to the bound on terms |
| `qsvuwtsx` | Answer the review of fixup-14 |
| `ukmypmlo` | Read a session's open witnesses through one walk |
| `nttvtsyp` | Raise the web module's budget for the ordinary session's open witnesses |

This report is the change after them.

- **A binding shows itself only where its witness is** (the 74e panel's
  finding 6). `Witnesses` keeps, per open witness, the inferences
  registered as showing it (`holders`) and, per inference, the witnesses
  it is registered under (`registered`). An inference is registered when
  it is recorded and when a binding changes it, and forgotten when an
  undo truncates it, so a witness's set holds every inference that shows
  it and every one a binding of it changed. `bind` walks the sets of the
  witnesses its axiom bound, `undo` those of the witnesses it unbound;
  the reader registers each inference under the witnesses its shown form
  holds and those its form as made reaches through the bindings
  (`reached`), so an undo of a binding read from a document finds every
  inference too.
  - The same states and documents as the walk over every inference:
    `a_binding_shown_through_its_holders_is_the_walk_s` runs 90 random
    sessions in LK, LJ and LM over eight first-order sequents (open
    witnesses, fillings, undos, documents read back), and after every
    step `walked_differs` rebuilds each inference from its form as made
    under all the bindings and compares, and checks every closed
    inference. With the undo's walk cut short by one inference the test
    fails at once.
- **A typed witness and every instance held to the bound on terms** (the
  74e reviewer's note). `instance_within` predicts, before `instantiate`
  builds anything, the arena's measure after the instance by the measure
  a document's reader takes (`arena_terms`): each atom of the body that
  holds a bound variable makes at most one atom more, every bound
  variable counted as the witness's size. A typed witness whose text
  names more symbols than the bound is refused before it is made. Both,
  and a binding past the bound (74e's `uswrllmz`), are the new
  `StepError::Terms { terms, limit }`, whose message says to start with
  a higher bound (the binding said "the axiom cannot fill the open
  witnesses: …" before, which an eigenvariable's step would have read as
  a fault of a witness). `an_instance_is_held_to_the_bound_on_terms`:
  a witness of five symbols under a bound of four, an instance past a
  bound of five, and 74e's reviewer's session under every bound from 3
  to 7, whose document reads back under the same bound whatever steps
  the bound let through. 74e's `a_binding_is_held_to_the_bound_on_terms`
  had a bound of 2 below its own sequent's measure; it has 8 now, where
  the instances pass and the filling (11) is refused.

### The timings

The 74e integers member's probe (a session of N conjuncts under an `∃`,
each closed by a filling axiom, beside W atoms), release builds of
`ktsrqsps` (before) and `yqsnzmmn` (after) on core 7:

| N | W | axioms before | axioms after | undo all before | undo all after | read back before | read back after |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 200 | 100 | 2.36 s | 0.059 s | 2.44 s | 0.006 s | 0.15 s | 0.20 s |
| 400 | 100 | 9.46 s | 0.139 s | 10.1 s | 0.013 s | 0.31 s | 0.42 s |
| 400 | 400 | 127.8 s | 1.85 s | 134.3 s | 0.064 s | 3.04 s | 4.19 s |

- An axiom at 1 200 steps of width 100 went from 24 ms to 0.35 ms, and
  the session's time from quadratic to about linear in its length
  (what is left grows with the goal's width: `scope`, `unifies` and the
  arena's measure are per step, over integers).
- Reading back costs about a third more: the reader registers every
  inference, walking its form as made and its shown form.
- The rest of the probe's 10.7 s at W = 400 is the probe's own search of
  each goal's formulas by their printed text.

### The ratchet

The gate at `wsklvkyy` counted `read-json` at 12 283 897 and `read-tptp`
at 33 812 683, above 74e's gate (12 226 553 and 33 743 026) and within
the tolerance. Neither journey reaches the session's code. Counted in
one build environment (`ratchet --only … --keep`), `ktsrqsps` gives
12 286 633 and 33 835 879 and `wsklvkyy` 12 283 897 and 33 812 683: the
fixup moves nothing (its counts are a little lower, the per-function
differences the allocator's and two addresses of one function); the
rise against 74e's gate is the base's, fixup-13 being in it, and the
build's.

### The web module's budget

The full check at the report's first commit failed one check, `budget`:
the bindings' module at 2 502 257 bytes at `gzip -9`, over its ceiling
of 2 500 000 (set that morning at 2 438 116, before 74c to fixup-14's
first-order session). `ukmypmlo` folds two walks the reader used
(`largest_meta`, `holds_open_witness`) into `metas_in`, which saved 149
bytes (2 502 108); `nttvtsyp` raises the ceiling to 2 560 000 with that
figure and what earned it, in `modules/budget.nix` and `web.md`, as the
file's own rule asks. (departs from the brief, which kept this session
to the session's code: the ceiling is a line of `modules/`.)

### The reviewer's round

`deduxit-review-fixup-14` (`fresh-reviewer`, Fable 5.2) read both
commits against the brief and the rules, argued the holders complete
(an undo returns to an earlier state with a superset of its
registrations; `reached` under the final bindings covers every prefix)
and the prediction an upper bound of the reader's measure, and ran a
differential fuzz of its own: 3 000 rounds of 60 actions over seven
sequents in the three logics under bounds from 4 to 43 (typed and open
witnesses, axioms, undos, documents read back and the session continued
from them): 1 530 fillings, 13 063 reads, 19 713 refusals at the bound,
35 388 undos, no difference from the walk, no reader refusal, no panic.
Its two findings, answered in `qsvuwtsx`:

1. **A session started past the bound** (`p(f(f(f(f(c))))) ⊢ q` under
   a bound of 3) wrote, with no step taken, a document its reader
   refused: `within` never measured the sequent. It refuses such a
   sequent now (`Refusal::Terms`), so the changelog's "a session writes
   no document its reader refuses" holds.
2. **The rules file named `StepError::Terms`** for the typed witness's
   text refusal, which is `StepError::Witness`; corrected, and the test
   pins the variant.

### What was checked, and by what

- The gate at `wsklvkyy` (covers `yqsnzmmn`), capped on cores 7 to 9:
  passed.
- Clippy, the core session's and the command's tests at `qsvuwtsx`.
- Clippy, the core tests and `cargo hack check --each-feature` at
  `yqsnzmmn`; clippy and the core and command tests at `wsklvkyy`.
- The timings above, both builds' sources swapped in one workspace (the
  probe kept out of the tree).
- `nix flake check` under the machine lock at the report's first
  commit: every check but `budget` passed (above); at the last commit:
  in the session's last message.
- The budget check alone (`nix build .#checks.x86_64-linux.budget`)
  after `ukmypmlo`: 2 502 108 bytes.

## Session fixup-15: shared terms walked once

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(75a's and 75c's conversation), cores 10 to 12, from 75c's report
("Left"): R2's input, `p0(a), ∀x. (p_k(x) → p_{k+1}(f(x, x))) ⊢ ∃y.
p_n(y)`, exponential in the connection provers' shared term code.
Nothing was pushed.

### What landed

- `pwnxvtom`: every walk of a shared term visits each part once.
  - `connection/terms.rs`: `occurs`, `unify`, `equal` and `ground` keep
    a set of the parts (or pairs) visited once a walk passes 64 visits
    (`Seen`), so a small term costs no set.
  - The classical read-back's `available` and `skolem_eigens` do the
    same; its `convert` already had a memo, and the intuitionistic
    read-back's walks were made graph walks in 75c.
  - Both read-backs build each instance once (`instances`). An atom is
    keyed by its text, so every build writes its arguments out in full,
    and 75c's binder children had lost the intuitionistic memo (R2 at
    n = 16: 2.6 s at the base, 0.04 s now).
  - `connection/sizes.rs`, `TermBudget`: an instance is held to
    `limits.terms` before it is built, by the measure a document's
    reader takes (the atoms' arguments unfolded, summed). Past it the
    read-back is `Refusal::Terms`, before an atom of gigabytes is
    written.
- `kyvlvztt`: the fresh review's two faults, fixed (below).
- No changelog line: the connection provers are new since 0.7.0.

### Timings

R2 at `--timeout 2s`, `--method connection`, one thread, release
builds of the base (`pupmmwkr`) and of `kyvlvztt`, cores 10 to 12,
under an 8 GiB scope; wall time and peak memory:

| n | classical before | classical after | intuitionistic before | intuitionistic after |
|--:|---|---|---|---|
| 16 | derivation bound, 0.03 s | derivation bound, 0.04 s | time limit, 2.04 s | unknown, 0.04 s |
| 20 | derivation bound, 0.14 s | derivation bound, 0.09 s | time limit, 3.02 s | unknown, 0.13 s |
| 22 | derivation bound, 0.55 s | derivation bound, 0.35 s | time limit, 3.85 s | unknown, 0.39 s |
| 24 | derivation bound, 2.01 s, 0.5 GB | terms bound, 0.53 s | time limit, 14.9 s | terms bound, 0.51 s |
| 26 | unknown, 7.66 s, 2.0 GB | terms bound, 0.54 s | time limit, 55.9 s, 2.0 GB | terms bound, 0.00 s |
| 30 | killed at 8 GiB, 50 s | terms bound, 0.56 s | killed at 8 GiB, 21 s | terms bound, 0.00 s |

- "Derivation bound" and "terms bound" are the refusals of the default
  `--derivation-limit` (64 MiB) and `--term-limit` (50 million): from
  n = 16 the derivation's atoms, written out, pass 64 MiB, and from
  n = 24 they unfold past 50 million terms, which any reader of the
  derivation would refuse. So no default bound lets R2 at n = 26 be
  proved: the answer within the time limit is the refusal, given before
  the long atoms are built. With both bounds lifted the classical
  prover proves n = 23 in 0.76 s; at n = 26 it stops at the time limit
  inside one instance's build (5.5 s at a 5 s limit), which no poll
  reaches.
- The intuitionistic search finds no proof of R2 at any n (unknown, as
  at the base): its read-back or prefixes reject what the classical
  search finds; not looked into further.
- The reviewer measured n = 100 and 400 at 0.5 to 0.6 s classically and
  at most 0.16 s intuitionistically, n = 1000 within 1.6 s.

### Verdicts

ILTP's first-order problems at 2 s, `--engines connection`, three
shards on cores 10 to 12: classical with the base build and with
`kyvlvztt`, intuitionistic with `kyvlvztt` beside 75c's last run
(`iltpfo75c5`, the base's code). Both logics decide the same problems
(600 classical, 1 of them not valid; 155 intuitionistic), with no
verdict against a status, no error but the TPTP reader's four (GEJ056,
GEJ057), and no row past 2.4 s. A run at `pwnxvtom` had two
intuitionistic rows at 4.4 s (GEJ054+2, GEJ055+1), which both builds
end at 2.0 s run alone and the run at `kyvlvztt` did not repeat: load.

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`) at `pwnxvtom`. Point 3 (R2
answered within 2 s at n = 26 and beyond) held: n = 26 to 400 refused
by the terms bound in 0.5 to 0.6 s classically and at most 0.16 s
intuitionistically, n = 1000 and 3000 within the limit. Three faults,
all fixed in `kyvlvztt`:
- **`unify` and `equal` skipped a pair never visited.** The pair's key
  was where each side's arguments start, and a constant's empty list
  starts where the next pattern's arguments do, so `(a, a)` stood for
  `(f(x), f(x))`. Past the first 64 visits unification then skipped
  `f`'s arguments: a valid sequent (`r(a), ∀x. t(g(f(x), a, k⁷⁰(e)), x),
  …`) read back an open branch (`Error::ReadBack`), and its not-valid
  form went from 0.07 ms to 891 ms intuitionistically. The key is now
  the symbol, the start, the length and the copy; the test
  `a_shared_part_is_named_whole_and_the_terms_bound_is_exact` fails on
  the old key.
- **The terms bound refused instances that fit.** `check` counted every
  atom of the body with a bound variable, also those the arena held:
  `p(f⁴(c)), ∀x. (p(x) → q) ⊢ q` was refused at a limit of 6 to 9 (its
  measure is 6), and R2 at n = 23 at 50.3 million terms where its
  measure is 33.6. Now each instance atom is found by the terms' ids
  and the atoms' symbols and arguments (never by its text), and counts
  only where the arena lacks it; both answer valid. The reviewer's
  question (an atom shared at two binder depths under a frame of two
  terms) is met by walking the body by node and depth.
- **The test's doc comment** claimed a proof in both logics; it now
  says what the test checks.

### Checks

- `cargo test --workspace` and `cargo clippy --workspace --all-targets
  -- --deny warnings` at both commits.
- The gate passed on the chain rebased onto `main@origin` (`mqxtzrzn`,
  75d landed): `ordinary-pigeons` at +1.88 % of its ceiling, within the
  tolerance (+1.83 % before the rebase; the journey is propositional,
  which this change does not reach).
- `nix flake check` under the machine lock at the report's commit
  (02:15 to 02:33): every check passed. Only this line changed after.

## Session fixup-16: the glue's licence and non-UTF-8 TPTP

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c and fixup-15), cores 10 to 12, from 75d's
two questions for the author, decided by the planning session
unattended. Nothing was pushed.

### What landed

- `oqylutum`: **`modules/_provers/ileancop_main.pl` is under the GPL.**
  It follows leanCoP 2.1's `leancop_main.pl` closely, and leanCoP's
  archive states "License: GNU General Public License" in every file
  without a version, which lets any version be chosen: the header's
  second line is `SPDX-License-Identifier: GPL-3.0-or-later`, followed by
  why, and that it is benchmark glue in no crate. The `conventions` check
  did not read Prolog or OCaml files at all; it now reads `.pl` and
  `.ml` too, and names this one file's licence line in a `case` (the
  four other `.pl` files and the `.ml` file keep the EUPL's, and a run
  with the glue's line put back to the EUPL's fails the check).
  Recorded in CLAUDE.md's conventions, `flake.md` and `bench/TOOLS.md`.
- `vupwskro`: **TPTP files whose comments are not UTF-8 are read.**
  `io::read_tptp` (the command's problem and its includes, and the
  batch's TPTP files) decodes the bytes with `tptp_text`: valid UTF-8 as
  it is; otherwise a byte that is no UTF-8 inside a `%` comment stands as
  U+FFFD, and one anywhere else (a formula, a quoted
  name, a string) is refused as "line N holds a byte that is not UTF-8
  outside a comment". The harness's `text_of` does the same in place of
  its lossy reading of every byte. All 107 of ILTP's first-order
  problems without equality that are not UTF-8 now read (`prove --logic
  classical` on each: none refused). Tests in `cli/tests/cli.rs`
  (`tptp_comments_need_not_be_utf8`, an included file too) and the
  harness's; the manual's TPTP section, the guide and the changelog say
  so.
- **The decoder is in two places**, the command's and the harness's: the
  harness depends on the core library alone, and the core's `read_tptp`
  takes text, so a shared one would be a new public function (bytes in,
  text out, or `read_tptp` taking bytes). An API change goes to the
  author: **recommended**, a public `ordinary::read_tptp_bytes(&[u8],
  &Limits)` (or `read_tptp` over `impl AsRef<[u8]>`), which the command,
  the harness and a library user reading ILTP from disk would share.
- The core's TPTP reader takes no `/* */` comment at all (a file with
  one is "not a TPTP problem"); ILTP writes `%` comments alone, and the
  decoders know the reader's `%` comments alone (the review's finding).

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`) at both commits. Points 1 and 2
held: the archive's licence line as the header says; the header rule
applied by hand to the 418 sources it globs; every input path of a
TPTP file (a file, standard input, a batch, `--isolate`, JSON,
`seq print`) reads a `%` comment's byte, and a byte in a formula, a
quoted name or an included file is refused with the right line; the
decoder alone on 2 million random byte strings never panicked, gave
`from_utf8_lossy`'s text whenever it accepted and never refused valid
UTF-8; every other input still refuses text that is not UTF-8. Two
false texts, fixed in `vupwskro`:
- `bench.md`'s comparison bullet still said the command refuses the 107
  files; it now says they are read, and that the run of 2026-11-21
  converted them.
- `tptp_text`'s and `text_of`'s docs said the reader skips `/* */`
  comments, which it does not; the decoders now know `%` comments
  alone, as the reader does.
- Left as it is: `bench/COMPARISON.md` records the run of 2026-11-21,
  when the command refused the files.

### Checks

- `cargo test --workspace` and `cargo clippy --workspace --all-targets
  -- --deny warnings`; the gate passed before the review's text fixes
  (`ordinary-pigeons` +1.88 %, within the tolerance), clippy and the
  TPTP tests after them; the 107 files read again on the final build.
- `nix build .#checks.x86_64-linux.conventions`: passed, and failed with
  the glue's line put back to the EUPL's.
- `nix flake check` under the machine lock at the report's commit
  (02:57 to 03:06), on the chain rebased onto `main@origin`: every check
  passed. Only this line changed after.

## Session fixup-17: one TPTP decoder

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c, fixup-15 and fixup-16), cores 10 to 12,
from fixup-16's recommendation, decided by the planning session
unattended. Nothing was pushed.

### What landed

- `zvpusorn`: **`ordinary::tptp_text(&[u8]) -> Result<Cow<str>,
  Error>`** (feature `parse`, beside `read_tptp`): a file's text for
  `read_tptp`, UTF-8 borrowed as it is, a byte that is no UTF-8 in a `%`
  comment read as U+FFFD, one elsewhere `Error::Tptp` ("line N holds a
  byte that is not UTF-8 outside a comment", kind malformed, code
  `tptp`). The command's `io::read_tptp` and the harness's `text_of`
  call it and lose their copies. Its doc comment has an example that
  runs; the crate docs' feature list, the guide's library section, the
  changelog's Added and the rules name it.
- **The shape departs from the brief's `read_tptp_bytes(bytes,
  limits)`**, as the brief allowed when argued: both callers inline a
  file's `include('NAME').` lines between decoding and parsing, from
  files the library cannot read (it reads none), so a function from
  bytes to a `Problem` would serve neither; the decoding is the shared
  step. A library user writes `read_tptp(&tptp_text(&bytes)?, &limits)`.
  `Cow` keeps the common case (UTF-8) a borrow; the name says what it
  returns, as `read_tptp` says what it reads.
- **`breaks 0.7.0`**: the diff of both public APIs names the change once,
  under Added (`pub fn deduxit::ordinary::tptp_text(&[u8]) -> …`); its
  one semver failure is the quantifiers' new variants of `Node` and
  `Rule`, already in the changelog's breaking list.

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`) at `zvpusorn`. Decoding is
identical: the removed copy against `tptp_text` on 2 million random byte
strings, equal text on every accepted one, borrowed exactly when UTF-8,
every refusal `Error::Tptp`, no panic. The argument for the shape held
(a bytes-to-`Problem` call could not sit between decoding and the
callers' inlining of includes), the doc example ran, and the public API
changed by the one addition. Two findings:
- **The refusal gained the library error's prefix**: `cannot read FILE:
  not a TPTP problem: line 2 holds …` where it read `cannot read FILE:
  line 2 holds …`, and a batch's record for such an entry now carries
  the library's `error` object (code `tptp`, kind malformed). Kept: it is
  the library's kind of error the brief asked for, and the old line was
  never released (fixup-16's reading is under Unreleased); no lock file
  or manual page quotes it.
- **"stands as"** in the public doc comment, which `writing.md` lists:
  now "is read as", in `zvpusorn`.

### Checks

- `cargo test --workspace` (the doc example among them) and `cargo
  clippy --workspace --all-targets -- --deny warnings`; the gate passed
  (`ordinary-pigeons` +1.88 %, within the tolerance); `breaks 0.7.0` as
  above.
- `nix flake check` under the machine lock at the report's commit, on
  the chain rebased onto `main@origin`: every check passed. Only this
  line changed after.

## Session fixup-18: LJT's leftovers

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c and fixups 15 to 17), cores 10 to 12,
from the entries step 54 left in `plan/later.md`. Nothing was pushed.

### What landed

- `nvqpvtmu`: **"limit of 0 B"**, traced. The Horn engine's memory
  refusal read its own account's limit; in the command's race that
  account is a part of the whole (`Account::part_of`: the bound less the
  input), so on an input near the bound it was zero, and the line said
  "the memory limit of 0 B was reached" under `--memory-limit 1MiB`
  (witness: `!(a0 -o a1), …, !(a19999 -o a20000), a0 |- a20000`, `-i`).
  The focused engine, the additive path and the inverse method applied
  `Reason::as_set` (the option's bound) themselves; the Horn and the
  essential engines did not. `conclude` now applies it to every engine's
  unknown, one place. `the_memory_limit_named_is_the_one_given` pins the
  line. The entry said "through the embedding, not traced"; the
  embedding's own engines named the right bound on every input tried.
- `mknsnsnv`: **the verdict line says why the default gave way**: where
  the SAT solver (or LJT) stops at a bound and the embedding decides,
  `prove`'s line ends `; the SAT solver gave no verdict: REASON, and the
  embedding took over`, a derivation past the bound written as "its
  derivation passes the --derivation-limit of L". Not for `Incomplete`: at first order grounding passes every
  sequent outside its classes on, and a note there said nothing a user
  can act on (it showed on the manual's first-order example). The
  batch's lines and the JSON say nothing of it. The manual's example
  (`a -> b, b -> c |- a -> c` under `--derivation-limit 1KiB`) and the
  guide say so; ILTP's SYJ202+1.004 shows it at the default bounds.
- `znzozxtz`: **`ordinary::Derivation::term` never panics**: each
  `expect` of the read-back (a hypothesis out of scope, a premise, an
  operand, an equivalence's implication or `⊥` missing, the LK-only
  arm, an empty results stack) is `Error::ReadBack` naming the
  inference (`unreadable`), a defect, since the checker refuses each and
  the crate makes none; the quantifier arms stay `unreachable!` behind
  `propositional`'s door; a pass before the walk refuses a derivation
  with no inference or a premise that does not come before its
  inference. The `# Panics` section was already gone (a derivation from
  JSON is checked first since 54a's follow-up); a test builds eleven
  broken derivations by hand
  (`a_derivation_the_read_back_cannot_read_is_refused`).
- `vnmxktzy`: **`check` reads derivations of LK and LJ**, propositional
  ones too (an LJT outcome, an embedding's LK outcome and a bare LJ
  derivation checked by hand; `cli/tests/cli.rs` and the lock's
  `check-ordinary` pin it): the entry was stale and is gone.
- Each closed entry is out of `plan/later.md`; the changelog has a line
  for the first three.

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`). Points 2 (the memory bound
named, under `-j 1`, `-j 3` and the race, the JSON too; the old binary
prints "0 B" on the test's input) and 4 (`check` of every method's
outcome and bare derivation, a changed rule refused) held. Five
findings, all fixed in the commits named:
- **The gave-way line printed a running count as the estimate**: the
  refusal's `estimate_bytes` is where the read-back stopped, so
  `--derivation-limit 1129` after "estimated at 1128 bytes" was refused
  again ("estimated at 1464"). The line names the limit alone now
  (`mknsnsnv`).
- **The manual's example sat under "`--method sat` reports the bound
  instead:"**, which it does not show; it follows the default's
  sentence now (`mknsnsnv`).
- **The read-back still indexed a premise past the end** (and an empty
  derivation's root, and looped on a premise naming its own inference),
  through `from_parts` alone: a pass before the walk refuses each
  (`znzozxtz`), and the rules' sentence lists them.
- **Seven of the new refusals had no test**: six more cases in the
  test (a premise past its inference, a principal past its sequent, an
  axiom with nothing right, `∨R` without a principal, `¬L` without `⊥`
  in the arena, a cut whose first premise has nothing right); the second
  operand's has none, being the first operand's in shape.
- **`term`'s doc contradicted itself** ("a derivation the crate made …
  which the crate never makes"): reworded.

### Checks

- `cargo test --workspace` and `cargo clippy --workspace --all-targets
  -- --deny warnings` after each commit and after the review's fixes;
  the gate passed on the chain rebased onto `main@origin`
  (`onzrmpum`) before the review's fixes, `ordinary-pigeons` at its
  ceiling (+0.00 %, raised in `vpwowvor`).
- `nix flake check` under the machine lock at the report's commit
  (04:33 to 04:50), on the chain rebased onto `main@origin`: every
  check passed. Only this line changed after.

## Session fixup-19: the verdict past the drawing's limit

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c and fixups 15 to 18), cores 10 to 12,
from the brief `/tmp/linlog-handoff/start-fixup-19.md`: the pigeonhole
entry's first way (the planning session's decision, unattended) and
the comparison script's ILTP pass. Nothing was pushed.

### What landed

- `ywwvtxow`: **a derivation of LK or LJ is bounded by memory, and
  `--derivation-limit` only what is drawn.** Every reader back (the
  embedding's `linear_derivation`, unfolded with `derivation_bytes`
  lifted, and its LK read-back; LJT's `Lj`; the SAT read-back; the
  classical connection read-back, now `Refusal::Memory`; the
  intuitionistic `Emit`) charges its inferences to `memory_bytes` and
  compares nothing with `derivation_bytes`. The command gates the
  drawing instead (`Written::drawn_bytes` in `show.rs`, by the new
  public `ordinary::Derivation::drawn_bytes`, the size estimate's
  measure): past the limit the drawing is left out with "the derivation
  is not written: its N inferences are estimated at X drawn, over the
  limit of Y; prove --format json writes it, …", the verdict and its
  exit status kept, in every drawn format, `--output`, the batch's
  directory and `check`. Three places still read `derivation_bytes` on
  purpose, from the review: the intuitionistic default keeps LJT's
  derivation only where its drawing fits (`ljt::decide_drawable`), the
  tableau method builds its tableau within it (it is what the method
  shows), and the protocol's `ordinary.draw`, `export` and
  `certificate` refuse a drawing past it (`drawable`). The manual's
  limits table and `commands.md`, `--help`, the guide, the changelog,
  `limits.rs`'s table and the rules say what each bound governs; the
  manual's gave-way example uses `--work-limit 6` now, since
  `--derivation-limit` no longer makes the default give way.
- `krpumtzr`: the lock's `help-prove.txt` and `help-check.txt`, the
  only lock files the change moved.
- `lkwyqolo`: **`bench/comparison.sh` runs ILTP as its third pass**:
  both packages (`iltp`, `iltp-first-order`) in classical and
  intuitionistic logic with the tools of ordinary logic (deduxit,
  intuit, intuitR, leanCoP, nanoCoP, nanoCoP-i, ileanCoP; the default's
  stream with `deduxit-default`), resumed and sharded as the other
  passes, empty under `--growing`; `--quick` takes SYJ101+1 and SYN036+1.
  The dry run found intuit and intuitR answering a parse error on a
  first-order problem: `tools::Ordinary` has `first_order`, false for
  those two, and such a run is `refused`/`fragment`. `plan/later.md`
  keeps the pigeonhole entry's second way (shared subderivations checked
  once) and gains the review's follow-up on LJT's preference (below).

### Measurements

All on core 10 (the ILTP runs on 10 to 12), release builds of
`pmoppnoz` ("before") and of the chain ("after"; "fixed" after the
review's fixes).

ILTP's SYJ202 in intuitionistic logic at the default bounds (2 s):

| size | before, `-q` | after, `-q` | after, JSON |
|---|---|---|---|
| 4 | unknown (embedding, time limit at copy bound 9) | valid, 0.09 s, 26 MB | 2.5 MB |
| 5 | unknown: the search ended, the time limit came before the check | valid, 2.11 s, 143 MB | 10.9 MB |
| 6 | the same | valid, 2.20 s, 453 MB | 136 MB, 2.65 s |

The drawn tree: at size 4 the default leaves it out ("its 20223
inferences are estimated at 68.7 MiB drawn, over the limit of 64.0
MiB"); with `--derivation-limit none` both builds draw it, 95.9 MB of
text in 0.25 s; at sizes 5 and 6 the drawing reaches the time limit and
is left out ("the time limit of 2s was reached"), the verdict kept.
Size 5 and 6 take the whole limit because the default's LJT search for
a derivation it may prefer runs on (1 019 980 sequents, 5 s, against
the solver's 28 ms), older than this chain and now a `later.md` entry.

ILTP's 274 propositional problems in the three logics at 2 s
(`--engines auto`, three shards), before against after (before the
review's fixes): 757 decided against 761 (755 and 756 within the
2 s; the rest a little past it). Gained six: SYJ202+1.004, .005 and
.006 in intuitionistic and minimal logic. Two classical rows went the
other way, SYJ202+1.010 and SYJ208+1.010; run three times with each
build, each build loses one run in three at the limit (1.85 to 2.11 s):
noise at the edge. The fixed build again: 761 decided, 756 within the
limit, nothing lost against the chain before the fixes, the same two
edge rows against `pmoppnoz`, and the decided rows' time summed 8.8 s
against 18.8 s (LJT's translation now stops at the drawing's bound).
`summary --strict` passes all three runs: no row against a status, no
error, no disagreement.

The comparison script's dry run (`--quick --timeout 2 --cores 10`,
built under the machine lock in 3 min, run in 3 min, exit 0):
`seq-1-1-iltp.csv` has 28 rows (2 problems, 2 logics, 7 tools) and
`ranking.md` its group "ILTP (ordinary logic)"; with the `first_order`
flag, intuit and intuitR are refused SYN036+1 and prove SYJ101+1.

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`). Points 2 (every drawing path
leaves the drawing out with the verdict and exit 0 kept, JSON at any
size, `--lambda` under its own bound), 4 (the shards partition, the
flag refuses only intuit and intuitR, `--growing` skips the pass) and 5
(`cargo hack check --each-feature`, `drawn_bytes` documented) held.
Its findings, all fixed in `ywwvtxow` unless named:
- **The default no longer drew 19 ILTP derivations it drew before**
  (SYJ201 from size 6, SYJ206 at 11 to 14): LJT's derivation, unbounded,
  displaced the one read off the SAT search (SYJ201+1.020: 184 826
  inferences, 14.7 GiB drawn, where the one read off is 11.7 MB of
  text; JSON 427 KB before, 310 MB). `prefer_ljt` now takes LJT's only
  where its drawing fits `derivation_bytes`, LJT's translation stopping
  past it (`decide_drawable`; `ljt_is_preferred_where_its_drawing_fits`).
  The fixed build writes the same bytes as before on SYJ201+1.006 and
  .020 and SYJ206+1.012, and gives SYJ202+1.004 its "valid" in 0.06 s
  (it drew no derivation before either).
- **The embedding's read-back charged nothing for its LK inferences**: a
  classical negation has no image, so each rule under d negations adds
  d inferences, each holding the whole sequent (2 000 hypotheses under
  20 000 negations reached 6.3 GB at a 1 GiB bound). `Out` charges each
  inference and the linear derivation beside it, and the chain asks the
  stop (`the_read_back_keeps_to_the_memory_bound`, whose linear proof
  unfolds within the bound its read-back passes).
- **The protocol drew a decided derivation of any size** (its `reads`
  name `derivation_bytes`, nothing applied it):
  `drawable` (`a_drawing_past_the_derivation_bound_is_refused`).
- **The tableau method still refuses past `--derivation-limit`**, against
  the new texts: they name the exception now (the method shows its
  tableau, which its steps build).
- **Stale text**: the changelog's fixup-18 line ("a derivation past
  `--derivation-limit` among them"), `limits.rs`'s table and the field's
  doc, LJT's and the SAT method's `# Errors` (`Refusal::Output`), the
  named method's advice to raise `--derivation-limit` (now only for a
  refusal of that bound), `gave_way`'s unreachable arm and its rules
  line, the line's "--format json writes it" under `check` (now
  "prove --format json"), `commands.md`'s "in every format … is not
  built", the guide's "made within its limits", and the comment that
  deduxit needs `$TPTP` for includes (every tool finds them above the
  problem; `lkwyqolo`).

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo test --workspace` after each commit and after the review's
  fixes (39 test binaries); the lock moved only in `help-prove.txt` and
  `help-check.txt`, reblessed in `krpumtzr`; the harness's tests on the
  `first_order` flag; shellcheck on the script.
- The gate passed twice: on the chain before the review, and on the
  fixed chain rebased onto `main@origin` (`rqqlsulk`). The ratchet:
  `ordinary-pigeons` +1.03 % over its ceiling's count (the read-back's
  charge per inference), within the 2 % tolerance; every other journey
  within it, misses moved on `search-chain-128` and `check-qbf-20-2`
  (a notice, not a failure).
- `nix flake check` under the machine lock at the report's commit
  (06:28 to 06:44), on the chain rebased onto `main@origin`: every
  check passed. Only this line changed after.

## Session fixup-20: LJT's preference within a bound

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c and fixups 15 to 19), cores 10 to 12,
from the entry fixup-19's review added to `plan/later.md`. Nothing was
pushed.

### What landed

- `zrkmnxmv`: **where the SAT solver has decided, the intuitionistic
  default gives LJT at most 2¹⁴ sequents** (`sat::PREFER_SEQUENTS`) to
  find the derivation it prefers, and past them keeps the derivation read
  off the search. `ljt::decide_preferred` takes that bound beside the
  drawing's (`Bounds`, replacing fixup-19's `decide_drawable`); it caps
  LJT's search alone, not `limits.work`, which also bounds the check's
  inferences and would have refused large derivations kept today.
  `ljt::decide` and `by_ljt` are unchanged. The size: every derivation
  the default kept from LJT on ILTP took at most 3 440 sequents
  (SYJ206+1.009), while LJT takes 1 019 980 sequents and 5 s on
  SYJ202+1.005; 16 384 sequents cost LJT under 0.1 s.
  `ljt_is_given_up_where_its_search_runs_long` builds SYJ202+1.005 (the
  file's sequent: the same solver counters and formulas) and holds the
  searches' polls under 2 000: 762 with the bound, 24 217 without it (it
  fails then, after 55 s in a debug build). The manual, the guide, the
  changelog and the rules say so; the `later.md` entry is gone, and one
  found on the way takes its place (below).

### Measurements

Release builds of `tutwkxpq` (before) and the commit (after), core 10
for single runs, 10 to 12 for the ILTP runs.

| `prove -q --logic intuitionistic` | before | after |
|---|---|---|
| SYJ202+1.004 | 0.06 s | 0.04 s |
| SYJ202+1.005 | 2.12 s | 0.05 s |
| SYJ202+1.006 | 2.20 s | 0.49 s |
| SYJ202+1.007, .008 | unknown at 2 s | the same |

The drawings fixup-19 kept are the same bytes (SYJ201+1.003, .006 and
.020, SYJ206+1.009 and .012, by hash). Through the command on all 274
problems in intuitionistic and minimal logic at 2 s (548 runs, JSON):
the same 237 valid, 272 not valid, 35 unknown, 4 without output (below),
the same 186 derivations kept from LJT, every decided outcome the same
size; 96.4 s against 88.2 s in all.

ILTP's propositional rows at 2 s in the three logics (`--engines auto`,
three shards): 763 decided, all within the limit, against 761 (756
within it); nothing lost, the two classical edge rows of fixup-19
(SYJ202+1.010, SYJ208+1.010) back, which this change does not touch
(classical logic never runs LJT). On the 761 rows both decide, the time
summed: intuitionistic 6.19 s to 2.10 s, minimal 8.47 s to 4.04 s,
classical 4.55 s and 4.61 s. `summary --strict` passes.

**Found on the way** (older than this chain, now in `plan/later.md`):
`prove --format json` writes nothing on standard output for an unknown
the time limit ended in the read-back or check (SYJ202+1.007 and .008:
the line on standard error, exit 3, no outcome).

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`). Criteria 1 (no verdict changed:
the same 548 rows, `-q` and JSON, every decided outcome byte-identical,
the summed time 16.60 s to 8.15 s), 2 (the bound is LJT's search's
alone), 4 (the texts) and 5 (no dead item in four feature sets) held.
Two findings, both fixed in `zrkmnxmv`:
- **The test's generator was not ILTP's SYJ202+1.005**: its flat
  disjunctions read nested to the left, the file's to the right (202
  conflicts against 184; LJT 1.76 million sequents against 1.02). The
  generator nests to the right now and gives the file's sequent; the
  test still fails without the bound (24 217 polls). Its comment on
  LJT's polls named one cadence of two.
- **A sentence of the rules was false**: "on ILTP LJT gave one or was
  refused at the derivation bound within 0.3 s on every valid problem"
  (fixup-19's); removed.

It noted, outside the commit, that `--no-default-features --features sat
--tests` warns of `Derive::Ljt`, a variant `#[cfg(test)]` whose tests
need `parse`; no gate run builds that combination.

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo test --workspace` (39 binaries) before the review, clippy on
  core and the two tests after its fixes (a test and a rule line).
- The gate passed on the commit before the review's fixes; the ratchet's
  `ordinary-pigeons` +1.03 %, as at fixup-19 (the embedding's journey,
  which this change does not reach).
- `nix flake check` under the machine lock at the report's commit
  (07:23 to 07:40), on `main@origin` (`lqwqkxor`): every check passed.
  Only this line changed after.

## Session fixup-21: JSON for a late unknown

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c and fixups 15 to 20), cores 10 to 12,
from what fixup-20 found. One question went to the planning session
(the library change below), answered within the hour. Nothing was
pushed.

### What landed

- `rpxwmqly`: **a bound or the stop that ends the read-back or a check
  after the search decided is an unknown with that reason and the
  search's statistics**, as a bound that ends the search was (the
  planning session's decision, unattended). `ordinary::decide::ended`
  maps `Refusal::{Stopped, Memory, Work, Index}` to their reasons and
  leaves every other error as it is. LJT, the SAT solver in all three
  logics (its read-back, LJT after it, the clausal proof's check, the
  model's check), both connection provers and the model checks of the
  truth table and the tableau answer through it. So `prove --format
  json` writes an outcome where it wrote nothing (SYJ202+1.007 and .008
  at 2 s: `"verdict":"unknown","reason":{"kind":"stopped"}` with the
  solver's statistics, exit 3), the batch writes an unknown record where
  it wrote an error record, and a named method's bound there is exit 3,
  not 2. The command keeps the line "the search ended, but the time
  limit … before its answer was checked" through `Ended::after_search`:
  the stop closure notes a poll outside `Phase::Search` (LJT and the SAT
  solver only, whose read-back follows the search; on SYJ202+1.007 the
  read-back ran out of memory and the stop fired in LJT's search after
  it, so the firing poll's phase alone said nothing). `check`'s JSON
  already wrote an unknown record; the protocol's `ordinary.decide` now
  writes these outcomes too. Pinned by `a_stop_after_the_search_is_an_unknown`
  (the library), `every_decided_outcome_reads_back` (the protocol) and
  a `--work-limit 5 --format json` case in `ordinary_logic` (the
  command). The `# Errors` sections, the manual's JSON section, the
  guide, the changelog and the rules say so; the `later.md` entry is
  gone.
  - **Departs from the decision:** the embedding's read-back in
    `ordinary::decide` keeps its refusal. Beside a proved image an
    unknown verdict is a document the outcome's reader refuses (the
    review's finding: a workspace edit lost); the command's own race
    answers with `Evidence::Image` there, as before. A truth table or a
    tableau that a bound ends while it is built (the tableau's until
    its derivation is checked) stays an error too: its outcome holds
    the table or the tableau, which is not there.
- `qznvnnuy`: **`Derive::Ljt` is `#[cfg(all(test, feature =
  "parse"))]`**, and the `features` check runs `cargo hack check
  --each-feature --tests` beside its two runs: 283 s to 345 s in the
  sandbox (rebuilt both under the lock). It found two more test-only
  items under one feature alone, fixed: `goal_tree`'s test module
  (`serialize`) and a protocol test's closure (`rocq`). Every pair with
  its tests takes eight minutes on three cores and fails in more places
  (tests that assume features they do not name), so pairs build no
  tests; `plan/later.md` lists what failed.

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`). Soundness held: `ended` maps only
the four bounds, defects stay errors, `prefer_ljt` and `by_ljt` keep
their behaviour, `gives_way` gives way in the same cases, and the
workspace still refuses a run its stop ended; the command's JSON, text
and batch held on SYJ202+1.007; `cargo hack check --each-feature
--tests` passed all 15 sets. Findings, all fixed in `rpxwmqly`:
- **The embedding's new unknown was a document the reader refuses**
  (`workspace apply … ordinary.decide --set method=embedding
  --work-limit 50` on a chain of 100 negations: the edit lost): reverted
  at that site, the departure above; `every_decided_outcome_reads_back`
  fails with the reverted code back.
- **The tableau's check after its search, and the manual's sentence**:
  its derivation's check refused at a bound stays an error, since the
  tableau is built from that derivation; the manual says so now, and
  its model check's stop is an unknown, as the table's.
- **Stale texts**: the rules on `gives_way` and on `--method ljt`'s
  refusals, `by_native`'s and the intuitionistic connection prover's
  `# Errors`.
- **The harness wrote `stopped` for a table's late stop** where it
  writes `timeout` (`bench/src/ordinary.rs`): one `unknown` mapping for
  its three sites now.

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo test --workspace` (39 binaries) after each commit and after the
  review's fixes; `cargo hack check --each-feature --tests` (15 sets,
  warnings denied).
- The gate passed on the chain before the review's fixes
  (`ordinary-pigeons` +1.03 %, as since fixup-19).
- `nix flake check` under the machine lock at the report's commit
  (08:58 to 09:16), on `main@origin` (`tvmvslwk`): every check passed,
  the `features` check with each feature's tests among them. Only this
  line changed after.

## Session fixup-22: tests in every pair of features

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c and fixups 15 to 21), cores 10 to 12,
from the entry fixup-21 added to `plan/later.md`. Nothing was pushed.

### What landed

- `mrnxrxny`: **`cargo hack check --feature-powerset --depth 2 --tests
  -p deduxit` passes, warnings denied.** The first full run (175 s warm)
  failed in two pairs, then a third file under the second; each test or
  test-only item now carries the `cfg` of its uses:
  - the linear session's `inferences()`, `serialize`'s and its tests':
    `any(feature = "serialize", all(test, feature = "parse"))`;
  - `tableau.rs`'s tests import `Interactive` and `expand` under
    `serialize`, as the one test that uses them is;
  - `tests/serialize.rs`: its three session tests and the session part of
    `wire_levels` under `interactive`, with their imports;
  - `tests/natural.rs` skips the tableau under `interactive` (the old
    filter named a variant that exists only there), and
    `tests/rules.rs`'s methods are one array with cfg'd elements (its
    `let mut` was unused without `sat` and `interactive`).
  The integration tests keep their files' `#![cfg(…)]` with inner
  `cfg`s, not `required-features`: `serialize.rs`, `natural.rs` and
  `rules.rs` each hold tests that need only `parse` and `serialize`
  beside ones that need `interactive` (and `latex`), so a whole-file
  requirement would skip tests or split the files (the reviewer agreed).
  No feature is turned on for tests alone (`core/Cargo.toml` untouched).
- **No test lost**: `cargo test -p deduxit --all-features -- --list`
  gives the same 923 tests before and after, line for line; every new
  gate is `interactive`, in the default features and in `test-core`'s.
- **The check**: `features-tests` (`modules/checks.nix`), the pairs with
  their tests on the crate with its integration tests (`core` and
  `core-tests`), in a CI group of its own (`modules/ci.nix`); `features`
  keeps its two runs without tests (fixup-21's `--each-feature --tests`
  is covered, each feature alone being among the pairs' sets).
  `flake.md` and CLAUDE.md say so. The `later.md` entry is gone.

### The check's cost and the decision

In the sandbox under the lock (`--max-jobs 4 --cores 8`): `features`
283 s before fixup-21, 345 s with each feature's tests; the pairs with
the unit tests added to it, 600 s. CI's `features` job took 8.2 min for
the 283 s check (`plan/reports/ci-speed.md`), about 1.6 times the local
build, so 600 s would have been about 16 minutes against the longest
job's 9: twice CI's wall time. Split, `features` takes 296 s (about 8
minutes in CI) and `features-tests` 373 s (about 10), each on a runner
of its own, so CI's wall time grows by a minute at most, for one more
runner. What it catches: a test or test-only item that breaks a pair's
test build, which no other check builds (fixup-21's `Derive::Ljt` was
one; this session fixed five); with a fault of `tests/rules.rs` put back
it fails, as it should.

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`): the run passes (78 sets, no
warning), no test is lost, each gate is its uses' `cfg`, the inner
`cfg`s are right for these files, the matrix evaluates. One finding,
fixed in `mrnxrxny`:
- **The check built no integration test**: its source was the part
  `core`, without `core/tests` (in the part `core-tests` alone), so in
  the sandbox `--tests` built the library's unit tests only, and the
  rules and comments claimed more (fixup-21's `--each-feature --tests`
  had the same gap). It takes `core-tests` now: 336 s became 373 s, and
  the check fails with `tests/rules.rs`'s old `let mut` put back.

### Checks

- `cargo test -p deduxit --all-features -- --list` before and after;
  the pairs with tests in the workspace (66 s warm, exit 0); clippy with
  the default features.
- The gate passed on the commit before the review's fix (which touched
  `checks.nix` alone): `ordinary-pigeons` +1.03 %, as since fixup-19.
- `nix flake check` under the machine lock at the report's commit
  (10:15 to 10:32), on `main@origin` (`uzkwvksz`): every check passed,
  `features-tests` among them. Only this line changed after, and the
  chain was rebased onto `zuwuvsum`, which adds plan files alone.

## Session fixup-23: the first-order defaults' open items

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c, fixups 15 to 22, 77a, 77b and 78b),
cores 10 to 12, from the brief `/tmp/linlog-handoff/start-fixup-23.md`,
the open items of 76a and 76b. Nothing was pushed.

### What landed

- `uzlvorpo`: **the library's default connection prover is bounded by
  work.** On a first-order classical sequent with no method given
  (`within`, and `by_grounding`'s fallback where grounding gives way) the
  prover searches within `CONNECTION_WORK`, 2²⁶ of its steps, or the
  caller's lower bound (`connected_by_default`, `connection_limits`),
  and past it gives way to the embedding and the model search, as at any
  other bound. The command calls the prover directly under its time
  share and is unchanged.
- `orltvzsn`: **on one thread the model search goes first** for a
  first-order intuitionistic or minimal sequent with no method given
  (`models_beside`, `model_first` in `cli/src/ordinary.rs`), within its
  grace (`model_grace`, 5% of the time limit), and the embedding takes
  the rest. `prove` calls it where it does not race (`--jobs 1`,
  `--deterministic`), and the batch for every such entry, so an entry
  answers as `prove --deterministic` does, which the batch's rules ask;
  with more threads `prove` keeps its race. Before, both left such a
  sequent to the embedding alone, which answers unknown on what it
  cannot prove. The manual's ordinary-logic chapter says so.

### The measurement

The harness's `--engines auto` (the library's whole default call) on
ILTP's 2 670 first-order problems, classical, 2 s, three shards pinned
to cores 10, 11 and 12.

- **First, before 78a landed** (the 1 730 equality problems refused),
  with a bound of 2²⁵: no verdict lost, eight gained by the bound (the
  embedding proved SYN067+1 and SYN349+1, the model search refuted
  SYN414+1 and six NLP problems once the prover gave way). The most work
  a theorem took was 20 539 978 steps (GEJ012+2).
- **After the rebase onto 78a** (equality decided with its axioms; the
  `before` rows `main@origin` at `luxmwxss`), 2²⁵ **lost two theorems**:
  SET024+1 and SET108+1, which the prover proves with the axioms in
  38 670 474 and 42 723 414 steps (1.2 s and 1.3 s). The bound is now
  2²⁶, 1.57 times the most.
- **The committed bound** (`bench/defaults/connection-work-iltp-first-order-{before,after}-{1,2,3}.csv`):

  | | proved | not valid | unknown |
  |---|--:|--:|--:|
  | before | 1 003 | 103 | 1 560 |
  | after | 1 006 | 103 | 1 557 |

  No verdict lost and none against a status. The three gained (GEJ019+2,
  GEJ048+1, SET616+3) are the prover finishing just under 2 s where it
  timed out just over before: timing. At 2 s the bound changes no
  verdict, since no row reaches 2²⁶ steps within the limit; it acts
  where the library runs without a clock. On the 1 106 rows decided
  alike the time is 67.8 s before and 66.0 s after in all, the median
  ratio 0.98 (the machine ran other sessions' checks meanwhile).
- **The brief's sequent** (irreflexive, transitive, serial): before, the
  default was still in the prover at 90 s and had no end in sight; after,
  it gives way at the bound and ends unknown (the embedding's
  `incomplete`) in 102 s on core 10. A step there costs ten times what it
  costs on GEJ012+2, so a bound in steps that keeps every theorem is
  minutes there; a bound that counted the unifier's work, closer to
  time, would cut it, and is a change of the prover's measure (left
  open).

### Fresh review

`fresh-reviewer` (Fable 5.2, `medium`). No wrong verdict and no lost
theorem (the CSVs checked row by row; the caller's work bound kept; the
bound also caps the derivation's check, which could refuse only a
derivation of more inferences than the bound, an unknown that gives way, never
a wrong verdict; the batch's lines and JSON equal `prove --deterministic`'s
on three sequents; `prove` with more threads still races). Three
findings, all fixed:
- **`prove` on one thread ignored the first Ctrl-C during the model
  search's grace**, which ran before the handler was installed: the
  process died with no verdict line. It installs the handler first now
  (the first Ctrl-C gives "unknown … interrupted", exit 3).
- **The grace ran without the terminal notice**: it starts one now.
- **The library test passed without the change**: it ran under a
  caller's bound that the old code already passed on. It now also holds
  `connection_limits` to the default's own bound where the caller sets
  none.
- The report's "eleven gained" counted three timing rows; the first run
  is described as eight gained by the bound. (The review read the chain
  before the rebase onto 78a and the bound's raise to 2²⁶.)

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo test --workspace` (39 binaries) after each commit and the
  review's fixes; the behaviour lock unchanged.
- The gate's first run was stopped: the rebase onto 78a changed the
  tree under it. The full check holds its parts.
- `nix flake check` under the machine lock at the report's commit
  (21:07 to 21:29), on `main@origin` (`luxmwxss`, 78a's code with it):
  every check passed. Only this line changed after.

## Session fixup-24: the shift's spelling and Herbrand's bounds

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58a-610ed3`
(the conversation of 75b, 75d, 76a to 76c and 77c), cores 7 to 9, from
`/tmp/linlog-handoff/start-fixup-24.md`: 77c's note on rocq-deduxit's
comments and 78b's panel's two items "not fixed, recorded". Nothing was
pushed.

### What landed

- rocq-deduxit `vxkypqzt` (1ff01e02): **the comments write the
  double-negation shift with its brackets**, `(∀x. ¬¬p(x)) → ¬¬(∀x.
  p(x))`, in `FirstOrderKripke.v`'s module header and `Chain` section,
  README and CHANGELOG (Unreleased); `.claude/rules/rocq.md` says why
  (deduxit's syntax reads the unbracketed spelling as another formula).
  No statement or proof changed. The planning session pushes it and
  moves deduxit's lock.
- `uqtvmqxv`: **Herbrand's instance search and its read-back keep to
  the memory bound** (`herbrand.rs`, shared by `herbrand` and
  first-order `resolve`).
  - The read-back charges the arena it grows (`ArenaSize`, `NODE` 192
    bytes a formula, measured about 160 with the maps), after each
    instance and each inference's replacement, the whole arena at the
    start; each inference (twice its size for the list's doubling, each
    vector with the allocator's header and rounding); and the frames
    waiting on its stacks.
  - The instance search charges each instance and the arena it grows
    (8⁸ instances of an eight-variable clause ran past 4 GB under the
    default 1 GiB, counted only against the occurrences, in the base
    too).
  - The solver and the read-back get the bound less what the instances
    and the arena hold (`Search::rest`), the solver also less the
    propositional copy it reads; a memory refusal names the bound the
    caller gave.
  - `sides` allocates each side exactly: collected from a filter, the
    sides held up to twice their length (119 MB of a 173 MB peak, by
    massif).
  - Test `the_read_back_keeps_to_the_memory_bound` (300 nested `∀`: a
    derivation by default, the clausal proof under 4 MiB), and
    `a_bound_or_the_stop_ends_the_search_for_its_reason` now checks the
    bound named at fourteen bounds (it failed with the share named).
- `rtvzszsy`: **a witness set per quantifier** (`Build::given`, cached
  per formula). A kept ground instance gives a `∀` hypothesis's or `∃`
  goal's variable the values all its atoms agree on (`match_terms`); an
  instance with an atom the body cannot match comes from another
  formula. The step order: the given witnesses, then a rule of two
  premises that closes one at once, then every other witness, then the
  other rules of two premises. The given values are among the uniform
  set, which follows them, so a short or wrong set costs order only.
  The manual's resolution example and the guide's Herbrand example show
  the shorter derivations (15 to 11 and 5 to 3 inferences). Test
  `each_quantifier_takes_the_witnesses_its_instances_give`;
  `the_read_back_asks_the_stop` in `first_order.rs` stops at the third
  step now (its six instances close in six).
- `xqpwqumk`: the lock's two first-order outputs, whose derivations are
  shorter (verdicts, instances and refutations unchanged), a commit of
  its own.

### Measurements

Peak RSS (the process starts at 11 MB), release builds of the base
`nwnrnrsu` and the head:

| input | command, bound | base | head |
|---|---|---|---|
| 1 500 nested `∀` over one atom (78b's witness) | `herbrand`, 128 MiB | 261 MB, derivation | 91 MB, clausal proof |
| the same | `resolve`, 128 MiB | 263 MB | 93 MB |
| the same | `herbrand`, default | 261 MB, 3 001 inferences | 262 MB, the same |
| 300 hypotheses `∀x. pᵢ(x)` | `herbrand`, 128 MiB | 93 MB, derivation | 96 MB, derivation |
| six nested `∀` over six constants | `herbrand`, 128 MiB | (not run) | 131 MB, clausal proof |
| `q(c1..c40), ∀x1 x2 x3. p(x1,x2,x3)` (64 040 instances, kept whole) | `herbrand`, 128 MiB | 205 MB | 162 MB |
| `q(c1..c20), ∀x1..x4. p(x1..x4)` | `herbrand`, 80 MiB | 143 MB | 105 MB, unknown at 80 MiB |

The last two stay above the bound by 10 to 17 MB past the process's
base: the solver's own accounting and the parts of the arena charged by
estimate. Neither is a regression.

The witness set, on nested quantifiers over constants (inferences, or
time):

| input | base | head |
|---|---|---|
| three nested `∀` over three constants, `herbrand` | 175 | 34 |
| four, `herbrand` (kept set whole, past `MOST_CUT`) | 1 729 | 1 729 |
| six over six, `resolve` | 60 s | 0.01 s |

ILTP's first-order problems through `herbrand` (`--timeout 2s`, three
at a time on cores 7 to 9): 2 670 problems, of which 1 840 end in the
reader's parse error (`!`, `=`) in both runs, the same set.

| verdict | base | head |
|---|---|---|
| valid, derivation read back | 326 | 346 |
| valid, clausal proof only | 139 | 119 |
| unknown | 365 | 366 |
| error | 1 840 | 1 839 |

No valid verdict and no derivation lost; 20 derivations gained. The 326
derived in both take 12.4 s instead of 15.7 s and 1 147 354 inferences
instead of 2 854 730, none slower nor longer. The one move between
unknown and error is SYN428+1, in the normal form at the 2 s boundary
(an error at 2 039 ms, the time limit at 2 051 ms).

A first order, with every rule of two premises before the uniform
witnesses, lost SYN549+1's derivation (4 872 inferences in 0.06 s
became a refusal at 598 MB): early splits made every branch repeat the
instances. Only a split that closes a premise at once goes first now,
and SYN549+1 reads back in 2 439 inferences.

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`) on the three commits: no answer
changes to a wrong one; item 1 and item 3 hold (`match_terms` right for
nested terms and repeated variables; 4 300 generated first-order
sequents through `herbrand` and `resolve` against the base: no verdict
or derivation lost, 48 read-backs gained, one base read-back error
fixed). Three findings in item 2, all fixed in `uqtvmqxv`:
- **An unknown at the memory bound named the share**, not the bound
  given (`the memory limit of 9891254 B` under `--memory-limit 80MiB`),
  against `core-search.md`'s rule: `Search::decide` puts the caller's
  bound back, in the outcome and in a refusal.
- **The solver phase overspent**: its propositional copy was uncharged.
  Its share is the rest less the copy now (124 to 105 MB on the
  reviewer's witness under 80 MiB).
- **The doubled inference charge rested on a copy that is not made**,
  and refused 300 hypotheses' derivation under 128 MiB though the
  process held 93 MB. Massif found what the factor had covered by
  accident: the sides' spare capacity. `sides` is exact now, and the
  charge counts the list's doubling and the allocator's blocks; the
  derivation is built under 128 MiB again.

### Checks

- rocq-deduxit: `nix flake check` at `vxkypqzt` passed.
- `cargo clippy --workspace --all-targets -- --deny warnings`, `cargo
  test --workspace` (through the gate) after each change.
- The gate passed on the chain before the rebase (the ratchet under its
  ceilings, `ordinary-pigeons` -0.82 %); clippy and `cargo test
  --workspace` again after the rebase onto `main@origin` (`luxmwxss`,
  78a's equality with it), without a conflict.
- `nix flake check` under the machine lock at the report's commit: the
  first run (21:54 to 22:14) failed in `msrv` alone, an `if let` guard
  in `Search::decide` past the `rust-version` 1.92 (rewritten as a plain
  arm in `uqtvmqxv`); the second run (22:15 to 22:37) passed every
  check. Then the chain was rebased onto `zrpoxqsw` (fixup-23's
  defaults, `decide.rs`, the CLI and the manual; only this report
  conflicted, both sections kept), and clippy and `cargo test
  --workspace` passed there again.

### For the planning session

- **The clausal normal form overruns its bounds on many hypotheses**
  (77a's `normal::rewrite`, outside this fixup): 1 500 hypotheses
  reached 54 GB in 100 s under `seq normalize --to clausal` (an
  uncapped run of mine), 300 reached 561 MB under
  `--memory-limit 256MiB`, and `herbrand` on 1 500 took 67 s under
  `--timeout 10s` there. A fixup's worth.
- ILTP's first-order problems with `=` parsed only after 78a; the
  measurement above is on the base without it.

## Session fixup-25: the clausal normal form within its bounds

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation that wrote `normal::rewrite`), cores 10 to 12, from
`/tmp/linlog-handoff/start-fixup-25.md`: fixup-24's note that the
clausal form overran its bounds on many hypotheses. Nothing was pushed.

### The cause

Three costs, none of them the rewriting itself (1 % of `herbrand`'s
time on 1 500 hypotheses, by callgrind):
- **Each law's claim copied the whole result arena** (`law_claim`
  cloned it per equivalence step to build `⊢ C ↔ C'`). The arena grows
  with every step, since each step rebuilds the path to its redex, so
  the check of `n` hypotheses held `n` copies of an arena of `n²`
  nodes: cubic in memory, the 54 GB of 1 500.
- **The first-order printer's setup was per formula**
  (`write_first_order`): a set of every symbol, and `eigen_names`, a
  scan of every term, for each formula printed. The step listing prints
  every step's whole sequent, `n` formulas times `n` steps, so it was
  cubic in time; it ran after the search, outside every bound: the
  base's `herbrand` on 1 500 took 119 s under `--timeout 10s`.
- **The embedding's read-back charged its inferences but not the
  formulas its instances add** to the derivation's arena. A law's claim
  over a deep nest of binders (50 `∀` over a conjunction) held 1.9 GB
  under the default 1 GiB.

### What landed

- `ppomsqvu`: **the printer keeps no per-formula setup.** A clash is a
  lookup in the interner's symbol map (`Formulas::has_symbol`); the
  eigenvariables' names are kept in the interner (`Interner::eigens`),
  computed at the first print and dropped by `Formulas::interner`, the
  one way ordinary code changes the tables (a new interner starts
  without them). The cache sits in the boxed interner: inline in
  `Formulas` it tipped `lambda::Inhabitation`'s variants past Clippy's
  `large_enum_variant`. Test `eigenvariable_names_follow_the_tables`.
- `kuppuuzy`: **the read-back charges the instances it builds**, by
  `ArenaSize` (herbrand's measured estimate, moved to `derivation.rs`),
  after each inference of the first-order path; the propositional loop
  is untouched. Test `the_read_back_charges_the_instances_it_builds`,
  which fails with the charge removed.
- `ynmmkusu`: **the check keeps to its bounds.**
  - A law's claim is built in an arena of its own holding its two
    formulas (`copy_formulas`: nodes and terms operands first on
    explicit stacks, binders with their names, atoms through
    `predicate_atom`, so their keys are made again).
  - The stop is asked before each claim; each decision gets the memory
    the kept checks leave (`left_over`, `checked_bytes` by
    `ArenaSize`), and the kept checks are refused past the bound
    (`kept_within`).
  - A refusal or an unknown at the memory bound names the bound the
    caller gave, with what the kept checks held added to the need
    (`given_refusal`, `given_bound`): the first version kept the
    decision's own need and said "53759984 bytes, more than the memory
    limit of 1073741824 bytes".
  - A memory or work refusal of the LJ attempt gives way to LK, as its
    unknown does: refused, it ended the whole check.
  - Tests `a_law_s_claim_copies_its_formulas_alone` (each claim's arena
    within the nodes its two formulas reach, plus the `↔`) and
    `the_clausal_form_keeps_to_its_bounds` (work and memory in the
    rewriting, memory and the stop in the check).
- `zrtuyvyl`: **the CLI's step listing is bounded**: `steps_text`
  stops at the time limit, at Ctrl-C, or once its text passes
  `--memory-limit`, with `(the steps are cut short after K of N: …)`,
  the claims' lines and the verdict after it; `seq normalize` and
  `herbrand` answer a memory or work refusal as unknown naming the flag,
  as `resolve` does. Test `the_clausal_form_keeps_to_the_memory_limit`.
  The steps hold about as much as their text on hypotheses with short
  names, so the cut fires mostly on long names (the test's 200-letter
  predicates).
- `.claude/rules/core-ordinary.md` (the printer, the read-back, the
  check) and `cli.md` (the listing) record the reasons.

The work bound: the rewriting charges its passes to it (as before);
each claim's decision gets it whole, as the propositional claims do.
What is left of the check's time is the claims' search itself: on 300
hypotheses 54 s, 99.8 % in the connection prover (callgrind on 25), the
last claim's derivation 91 200 inferences. It is bounded by the time and
the stop.

### Measurements

Release builds of the base (`uwuwmwxo`) and the head, each run in a
scope capped at 4 GB on cores 10 to 12 (peak RSS; the process starts
at 11 MB):

| witness | command, bound | base | head |
|---|---|---|---|
| 300 hypotheses `∀x. pᵢ(x)` | `seq normalize --to clausal`, 256 MiB | 56.4 s, 711 MB, checked | 54.0 s, 179 MB, checked |
| the same | the same, default | 56.3 s, 711 MB | 54.3 s, 180 MB |
| the same | `herbrand`, `--timeout 10s` | 1.20 s, 95 MB | 0.37 s, 95 MB |
| 1 500 hypotheses | `seq normalize --to clausal`, 256 MiB, 120 s | killed at the 4 GB cap after 109 s | unknown at 120 s, 125 MB |
| the same | the same, default | killed at the cap after 110 s | unknown at 120 s, 127 MB |
| the same | `herbrand`, `--timeout 10s` | 119 s, 907 MB, valid | 3.2 s, 907 MB, valid |
| `∀x₀…x₁₄₉₉. (q(x₀) ∧ … ∧ q(x₁₄₉₉)) ⊢ q(a)` | `seq normalize`, 256 MiB or default, 120 s | unknown at 120 s, 146 MB | the same |
| the same | `herbrand`, `--timeout 10s` | unknown at 10 s, 22 MB | the same |
| `∀x₀…x₁₄₉₉. p(x₀,…,x₁₄₉₉) ⊢ p(a,…,a)` (78b's witness) | `seq normalize`, 256 MiB or default | 4.1 s, 185 MB, checked | the same |
| the same | `herbrand`, `--timeout 10s` | 1.7 s, 262 MB | 2.0 s, 261 MB |

`herbrand` on 1 500 hypotheses holds 907 MB in both builds under the
default 1 GiB: the instance search's charge, within the bound. The text
it prints is 114 MB, under the bound, so it is not cut; under
`--memory-limit 256MiB` the process peaks at 310 MB, the text beside
the library's charge.

The deep nest under the head (`∀x₀…x₄₉`, a conjunction body), `seq
normalize --to clausal`: 1.9 GB under the default before the read-back
charge, then 546 MB; under 256 MiB 149 MB, unknown at the bound named.

ILTP's first-order problems through `herbrand` (`--timeout 2s`), both
builds per problem on one core, three shards on cores 10 to 12 (both
builds on `uwuwmwxo`, before 78c):

| verdict | base | head |
|---|---|---|
| valid, derivation read back | 543 | 543 |
| unknown | 1 861 | 1 860 |
| error | 266 | 267 |

No answer lost, no derivation lost. The 543 valid in both take 394 s
instead of 452 s, none slower by more than 0.2 s. The one move is
SYN428+1, at the 2 s boundary in both (the time limit at 2.04 s, the
form's bound at 2.01 s), as in fixup-24's run.

### Fresh review

`fresh-reviewer` (Fable 5.2, `medium`) on the four commits, reading
only: no correctness finding (the cache has no stale path, `copy_formulas`
keeps binders, indices, atoms, `=` and refuses eigenvariables as
`transfer` already did, no claim is recorded checked without its
decision, the CLI's arms and the claims' lines after a cut hold). Two
requirements:
- **The report was missing**: this section.
- **The read-back's charge had no test**: the bound tests passed on
  `kept_within` alone. `the_read_back_charges_the_instances_it_builds`
  now pins it (a bound above the inferences, the linear derivation and
  the view's own least bound, below the instances' growth; it fails
  with the charge removed), folded into `kuppuuzy`.

Its question: a stop during the check now drops the steps (`seq
normalize` under `--timeout` prints only `unknown: …`), where the base
printed them with the remaining claims unknown. Intended: `cli.md`
allows it, the claims would all be unknown, and no example shows the
old output.

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and the
  tests after each change.
- The gate passed on the chain before the rebase: the ratchet under
  every ceiling (largest rise +1.61 % on `search-spec-chain`, journeys
  the chain does not touch; `ordinary-pigeons` −0.80 %).
- Rebased onto `main@origin` (`mrlkmlxy`, 78c with it) without a
  conflict.
- Rebased again onto `qoltpvxo` (0.8.0's release chain), the changelog
  entry folded into 0.8.0's normal forms entry, without a conflict.
- `nix flake check` under the machine lock at the report's commit (01:40
  to 02:02): every check passed. Only this line changed after.

### For the planning session

- **The claims' decision of a long clausal form is the check's cost**:
  300 hypotheses take 54 s, almost all of it the connection prover
  deriving the last claim (91 200 inferences). Bounded now; deciding
  the whole-sequent claims through Herbrand's instances and the SAT
  solver (`herbrand` decides the same 300 hypotheses in 0.4 s) would be
  the next step.

## Session fixup-26: the connection prover's setup within its bounds

Opus 5.5 at `high`, keeping the conversation of 78a and 78c, in the
workspace `../deduxit-wt-step-78a-94ff76`, cores 4 to 6, from the brief
`/tmp/linlog-handoff/start-fixup-26.md` on `opqvwylk`. Nothing was
pushed.

### The cause

Profiled with callgrind on smaller witnesses of the same shape (perf
could open no event here):
- **The classical read-back, not the matrix, held the congruence
  witness.** The prover proves a congruence axiom of 1 000 arguments in
  about 0.1 s; the read-back then builds the derivation of LK. It expands
  a branch before making its inference, so a branch of quantifier steps
  (a congruence of 3 000 arguments is a nest of 6 000) was expanded
  whole, each step an instance as large as the rest of the nest, before
  the first `push` asked the stop; and only the inferences were charged,
  not the arena the instances grew (nodes, terms, argument lists of
  3 000 terms, atoms' keys): 20 s and 2.5 GB under `--timeout 2s` and
  the default 1 GiB.
- **The matrix build** asked the stop in its unfolding and every 65 536
  nodes of `free_slots`; `literals_of` and `build` never asked, and
  `literals_of` walked the binder chain once per bound variable,
  quadratic in the nest. `free_slots` polled by node index, so a few
  thousand nodes, each a list of thousands of slots, asked once.
- **The default's test of whether grounding applies** (`grounds`, six
  times a command) was most of the predicate witness's 1.3 s at 0.1 s:
  its normal form counted a node made as one unit, though making a
  quantifier's node copies its body's free variables (thousands long
  in such a nest), and `literal` walked the scopes once per variable.
  2²¹ of those units took about 0.2 s per test.
- The intuitionistic prover's set-up (the same matrix, its prefixes) and
  its read-back, which charges its arena already, kept to the limits on
  both witnesses; it proves neither congruence.

### What landed

- `ouylvzkp`: the read-back asks the stop at every branch it expands
  (`account`, in `expand` and `push`) and charges its arena
  (`arena_bytes`: nodes, terms and lists with their indices, argument
  lists twice, atoms with their keys, every figure twice an entry's size
  for the tables' doubling) and the sides of an inference waiting for its
  premises. The matrix's `literals_of`, `build` and `free_slots` ask the
  stop by the work they do (`free_slots` charges the lists it keeps and
  counts the per-clause ones as work alone), and a variable's binder
  comes from the literal's chain, found once per literal. Tests:
  `the_read_back_asks_the_stop_and_charges_its_arena` (at least as many
  polls as the nest is deep before the first inference; unknown at the
  memory bound under 4 MiB where it proves at the default) and
  `the_matrix_asks_the_stop_by_its_work`.
- `zvwkzyxn`: grounding's normal form polls by `Nnf::work`, `literal`
  reads the scopes through a chain found once per literal, and
  `GROUNDS_WORK` is 2¹⁸ (ILTP's problems in the classes take at most
  25 683 units under the new counting, those outside 755 233; none
  reaches the bound). Test: `the_grounding_test_ends_at_its_work_bound`
  (a predicate's congruence of 500 arguments lies in the class but
  passes the bound; a small sequent still grounds).
- `ponlwnzr`: the rows below (`bench/defaults/connection-setup-iltp-*.csv`),
  the rules notes (`core-ordinary.md`, `bench.md`) and the changelog.

### Measurements

The two witnesses, time and peak RSS, on core 5 under a 4 GiB scope:

| witness | before | after |
|---|---|---|
| congruence 3 000, `--method connection --timeout 2s` | 20.05 s, 2 530 MiB | 2.09 s, 164 MiB |
| the same, intuitionistic | 2.05 s, 293 MiB | 2.07 s, 293 MiB |
| the same, `--timeout 120s --memory-limit 1GiB` | (20 s, 2.5 GB at 2 s) | 6.53 s, 958 MiB, unknown at the bound |
| the same, `--memory-limit 256MiB` | | 2.81 s, 269 MiB, unknown at the bound |
| predicate 4 000, default, `--timeout 0.1s` | 1.29 s, 185 MiB | 0.20 s, 19 MiB |
| the same, `--method connection --timeout 0.1s` | 0.25 s, 141 MiB | 0.11 s, 97 MiB |
| the same, `--method connection --timeout 2s` | 2.03 s, 278 MiB | 2.03 s, 263 MiB |

A congruence of 1 000 arguments still proves (2.9 s, 574 MiB, where it
took 4.5 s). The figures are the final build's, after the review's
fixes. What is left past 0.1 s on the predicate's default is the
six grounding tests (some milliseconds each) and the reading; on a
congruence of 1 000 arguments at 0.1 s, one stretch of about 50 ms
inside a search step.

**ILTP's 2 670 first-order problems through `--engines connection`**,
2 s, classical and intuitionistic, three shards on cores 4 to 6, the
builds one after the other:

| logic | build | proved | not valid | unknown | error |
|---|---|--:|--:|--:|--:|
| classical | before | 935 | 1 | 1 730 | 4 |
| classical | after | 937 | 1 | 1 728 | 4 |
| intuitionistic | before | 249 | 0 | 2 417 | 4 |
| intuitionistic | after | 249 | 0 | 2 417 | 4 |

No verdict lost, none against a status; the two gained (SET591+3,
SET201+3) are theorems proved just under 2 s, timing. The four errors
are GEJ056 and GEJ057's reader errors, in both builds. Every row
decided alike has the same `nodes` and `work` in both builds, so the
search is the same. On those rows the time is 58.0 s before and 54.4 s
after classically (CPU 56.1 s and 52.8 s, the median row 1.004 times)
and 4.62 s and 4.58 s intuitionistically (median 1.009): the set-up's
polls cost nothing measurable. The after rows are the final build's
(the pass ran again after the review's fixes; the first after pass
gave the same verdicts and counters).

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`), on the chain to `ponlwnzr`. The
chains compute the binders and scopes the old walks did; the ILTP rows
have equal counters on every row decided alike; the witnesses as
measured. Four findings, all fixed in `lxyxzllr`:
- **A regression: the inferences that close a branch were charged no
  sides** (high). `push` had stopped charging an inference's sides,
  since `expand` charged them as the step was taken, but the axiom's
  weakenings and `⊤R`/`⊥L`, which `close` pushes itself, never went
  through `expand`: `q0, …, q29999, p ⊢ p` proved at a peak of 1 752 MiB
  under the default 1 GiB, where the base refused at the bound. `push`
  charges every inference's sides again and the `Emit` frame gives back
  what `expand` charged while they waited. Test
  `the_read_back_charges_the_sides_of_an_axiom_s_weakenings`; the
  witness ends at the bound in 0.5 s (1 054 MiB).
- **The matrix still passed the bounds in three places** (medium, as in
  the base). A node's slots were a union per child, each copying the
  growing list: a clause of 200 000 elements ran 33 s at `--timeout
  1s`; now the children's lists are joined, sorted and deduplicated once
  (0.37 s). The slots barred for a matrix kept, per clause, the union of
  the elements after each one, quadratic in memory and counted as work
  alone (791 MiB under `--memory-limit 64MiB`); now a count per slot
  tells which another element has (30 MiB). One literal of 10 000 Skolem
  terms made its patterns before its poll (971 MiB under 64 MiB); it is
  polled per argument now (60 MiB, unknown at the bound).
- **The matrix test did not pin `free_slots`' polls** (low): with them
  reverted it still passed. It now counts a hundred literals under 3 000
  binders (20 polls; with `free_slots` silenced 2, which fails it) and a
  literal of 1 000 Skolem terms (9).
- **The figures of `GROUNDS_WORK` were not the measure it compares**
  (low): `done` counts the normal form and miniscoping together, at most
  52 154 in ILTP's classes (SYJ212+1.010), up to 2 355 202 outside; the
  bound is five times the most, not ten. The docs and the rules say so;
  `grounds` agreed with the unbounded test on all 2 666 problems that
  read.

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings`, the
  ordinary suite's tests (203), the new tests.
- The gate on the chain before the review: passed (the ratchet's
  journeys none over a ceiling). After the review's fixes and the
  rebase onto `main@origin` (`wxwmmvuy`, 0.8.0's release chain and
  fixup-25): passed again.
- The changelog's entry is in 0.8.0's section, a sentence each in the
  entries of the connection prover and of grounding, both new in 0.8.0.
- `nix flake check` under the machine lock at the report's commit
  (`uorkokwz`): every check passed. Only this line changed after.

## Session fixup-27: every dependency updated

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-fixup-27-9c5358`
(cores 10 to 15), from the brief `/tmp/linlog-handoff/start-fixup-27.md`
and the `update-deps` skill, in deduxit, rocq-deduxit
(`../rocq-deduxit-fixup27`) and the client (`../deduxit.github.io-fixup27`).
I pushed nothing; the planning session pushed rocq-deduxit and deduxit
as each was ready.

### What landed

rocq-deduxit, on `zxknsvsx` (0.8.0), pushed as `main` at `uytplsqq`
(`20b1a20f`):
- `unszlzrl` keeps shfmt's two-space indent and `-s` explicitly. The new
  treefmt-nix made tabs and no `-s` shfmt's defaults (its `shfmt.nix`
  between `0d6d739a` and `b7b56d3c`), which failed `treefmt` on
  `scripts/opam-install.sh`; the settings are the old defaults written
  out, valid on either lock.
- `xlmlqsws` "flake.lock: Update": nixpkgs 2026-11-06 -> 2026-11-22
  (`4fe25e12` -> `619c39f6`), treefmt-nix 2026-10-13 -> 2026-11-25.
  NanoYalla (`6b1c25ff`) and Yalla (`a1384376`) are their upstream heads
  already. Rocq stays 9.1.1 (the package `rocq9.1-deduxit-0.8.0`).
- `uytplsqq` cachix/install-nix-action v31.11.3 -> v31.11.5.

deduxit, on `tmosnvps`, pushed as `main` at `urzxlzqy` (`a5118a71`) after
the full check on GitHub's runners (run 45721959978):
- `mvkktoxo` "Cargo update": anyhow 1.0.104 -> 1.0.105, clap and
  clap_builder 4.6.7 -> 4.6.10, clap_derive 4.6.7 -> 4.6.9, clap_lex
  1.1.1 -> 1.1.2, ctrlc 3.5.2 -> 3.5.3 (taking nix 0.31.3 -> 0.32.0),
  cc 1.11.2 -> 1.11.3, crc32fast 1.5.5 -> 1.5.6, hashbrown 0.17.1 ->
  0.17.2, pin-project-lite 0.2.17 -> 0.2.18, smallvec 1.16.5 -> 1.16.6,
  walkdir 2.5.0 -> 2.5.1, zlib-rs 0.6.8 -> 0.6.9. The direct
  dependencies among them are anyhow and clap (the command's and the
  harness's) and ctrlc (the command's); the core crate's moved none.
  Every version builds on 1.92 (cargo locked "to highest Rust 1.92
  compatible versions"); `cargo deny check advisories`: ok.
- `xklllltq` keeps shfmt's settings explicitly, as in rocq-deduxit,
  before the lock that needs it.
- `swmolmkm` "flake.lock: Update": crane 2026-11-04 -> 2026-11-18,
  nixpkgs 2026-11-12 -> 2026-11-22 (`2074cc67` -> `619c39f6`),
  rocq-deduxit `e3eecb30` -> `20b1a20f`, rust-overlay 2026-11-14 ->
  2026-11-25, treefmt-nix 2026-10-13 -> 2026-11-25.
- `xrsvquvl` the fuzz nightly 2026-11-13 -> 2026-11-25 (rustc
  1.102.0-nightly `739adecd4`).
- `xwxzvokq` the fuzz workspace's Cargo update: cc 1.11.2 -> 1.11.3;
  `cargo check --bins` on the nightly passed, its advisories ok.
- `urzxlzqy` cachix/install-nix-action v31.11.3 -> v31.11.5 in all seven
  workflows.

rustc is 1.100.0 before and after: rust-overlay's `stable` is still
1.100.0. wasm-bindgen 0.2.130 is the newest release and the
newest `wasm-bindgen-cli_0_2_130` of the new nixpkgs, so its exact pins
stay. Kani 0.68.0 is its latest release, with CBMC 6.11.0 as nixpkgs has
it. `cargo-fuzz` is nixpkgs' 0.13.2.

### Verified

- rocq-deduxit: `nix flake check` at `xlmlqsws` (with the shfmt settings
  under it) passed: the library and its tests on Rocq 9.0 to 9.3, the
  bridges, treefmt, conventions, typos, deadnix, actionlint. `actionlint`
  and `treefmt` again at `uytplsqq`.
- deduxit: the gate passed at the Cargo update and again at the flake
  update (both on cores 10 to 15). At the flake update (with the fuzz and
  actions edits in the tree), 22 flake checks built clean under the lock:
  clippy-core, test-core, doc, wasm, msrv, web-package, web-wasm-tests,
  web-core-wasm32, web-bindgen, budget, features, ratchet, treefmt,
  actionlint, deny, shear, rocq, rocq-deduxit, rocq-deduxit-library,
  export, check-certificate, manual. The full check ran on GitHub's
  runners at `urzxlzqy` and passed.
- The ratchet: no journey over its ceiling before or after either bump.
  Its cache model notes misses moved by more than 5 % and 10 000 on
  search-chain-128 and check-qbf-20-2 in both gates, the Cargo update's
  as well as the flake update's, so neither bump moved them; the note is
  not a failure, and nothing was timed for it.

### Held back

- **resvg** 0.47 (0.50 out): krilla-svg 0.8.1, the newest, still uses
  usvg 0.47, and the PDF path hands resvg's tree to it.
- **skrifa** 0.42.1 (0.50 out): the core crate's dev-dependency that
  `every_table_is_its_font_s` reads the font files with, kept at the
  version krilla 0.8.2 locks (`core-export.md`).
- **minicov** 0.3.8 (0.3.9 out): wasm-bindgen-test 0.3.80 pins it with
  `=0.3.8`, and moves with wasm-bindgen's pin.
- No other major release is open: none of the engines' crates moved, so
  no target set was needed.
- **cachix/install-nix-action v32** (2026-11-20) and v32.1.0
  (2026-11-23) are younger than the fourteen days; v31.11.5
  (2026-11-11) is the newest past them. Every other action is at its
  latest release.

### The client

On `uxtxusrn` (the stack spec's record), not pushed by me:
- `uspuuuyq` the npm dependencies, each at its newest release at least
  fourteen days old (cut-off 2026-11-11 23:10 UTC): astro 7.4.4 ->
  7.4.11, @astrojs/solid-js 7.1.2 -> 7.1.5, solid-js 1.9.18 -> 1.9.19,
  lucide-solid 1.66.0 -> 1.72.0, tailwindcss and @tailwindcss/vite
  4.3.3 -> 4.3.5, vitest 5.0.5 -> 5.1.0, then `npm update` for the
  packages under them and `npm dedupe`. Without the dedupe `npm update`
  raised @axe-core/playwright's peer `playwright-core` (`>= 1.0.0`) to
  1.65.0 beside @playwright/test's 1.64.1, and `typecheck` failed on two
  `Page` types (`tests/accessibility.spec.ts:135`); deduped, one
  playwright-core 1.64.1 remains, as before (363 -> 358 packages).
- `rmzrxzrn` cachix/install-nix-action v31.11.3 -> v31.11.5.
- `tnvosplw` shfmt's settings written out, as in the other two
  repositories (the client follows deduxit's treefmt-nix).
- `nix flake check` passed at `rmzrxzrn` (the npm and actions changes
  together, on the client's current lock); `treefmt` at `tnvosplw`.

Held in the client:
- **The flake update** (deduxit `c3d80cfa` -> `a5118a71`, and with it
  nixpkgs 2026-11-12 -> 2026-11-22): every check passes but `unit`
  (`nix flake check --keep-going`), whose test "offers the forms deduxit
  lists" (`src/lib/clausal.test.ts`) fails: deduxit's `about` now lists
  the five first-order normal forms (prenex, miniscope, skolem,
  herbrand, clausal; `yptmxuux`, `pvqsnwxz`, step 77) and the page's
  `FORMS` (`src/lib/clausal.ts`) the five propositional ones. Offering
  them is new page behaviour with its own words, and step 77's prompt
  makes the page's side step 79's, so it waits for 79a or the author;
  the lock was backed out (`jj restore flake.lock`). The lock that
  passes is one `nix flake update` away once `FORMS` has them.
- **Dropping the Biome override** (`modules/biome.nix`) waits with that
  lock: Biome 2.6.8 of the new nixpkgs is on cache.nixos.org
  (`nix path-info --store https://cache.nixos.org
  /nix/store/5f5f2yvryd768jc4pp2qpvbffyw26gw7-biome-2.6.8` prints it).
  The test the module and `.claude/rules/flake.md` give, `nix path-info
  --store https://cache.nixos.org --inputs-from . nixpkgs#biome`, does
  not answer that question: it tries to copy the derivation to the
  binary cache and failed with HTTP 404 although the output is there. The command to write there is the store path's lookup above
  (`nix eval --raw --inputs-from . nixpkgs#biome.outPath` gives it).
- **@playwright/test** stays 1.64.1: the new nixpkgs' `playwright-driver`
  is still 1.64.1. **@types/node** stays 24.19.2 (24.19.3 and 24.19.4 are
  from 2026-11-25), **typescript** 6.0.3 (6.x's newest), **fast-check**
  4.10.2 (4.10.3 is from 2026-11-18), **@axe-core/playwright** 4.14.0
  (4.14.1 from 2026-11-24); the fontsource packages, @astrojs/check and
  yaml are at their newest.
- **Solid 2.0** (the stack spec's §3.4 check): condition 1 fails,
  `solid-js`'s `latest` is 2.0.4, a stable 2.x, but from 2026-11-25
  (2.0.0 from 2026-11-17), under fourteen days; condition 2 fails,
  @astrojs/solid-js 7.1.6 (its `latest`) peers `solid-js ^1.9.13`. No
  port. No package moved past a version line of §2 (Astro 7.x,
  Tailwind 4.3, TypeScript 6, Vitest 5, Biome 2, npm 11.19.1, Node.js
  24.22.2), and `.npmrc` is as §3.8 has it.
- The client's `flake.nix` says its followed inputs are "all of them
  past the fourteen days an update waits"; deduxit's lock takes
  treefmt-nix and rust-overlay at their newest (2026-11-25), and neither
  repository's skill gates flake inputs by age. A question for the
  author: either the comment goes or deduxit's update holds those
  inputs back too.

### For the author

- The client's flake update and the Biome override's removal wait for
  the page to offer the first-order normal forms (step 79).
- The client's chain (`uspuuuyq`, `rmzrxzrn`, `tnvosplw` on
  `uxtxusrn`) was handed to the planning session to push.
- The fourteen days and the flake inputs, above.

## Session fixup-28: the redesigned page's frame at every size, and Export for every view

Opus 5.5 at `high`, 2026-11-26, from the brief
`/tmp/linlog-handoff/start-fixup-28.md`, in the client workspace `step79a`
(`/home/tux/Projects/own/deduxit.github.io-79a`) on the redesign's head
`svusqksw`, and in the deduxit workspace `deduxit-wt-step-79a-…` on
`main@origin` (this report alone), cores 4 to 9. Nothing pushed; the
client's `main` untouched; no change to the library or the bindings.

### Outcome

The four faults of the frame that 79e's walk left open are gone at their
cause, and Export writes natural deduction, a modal decision and a
lambda-term through their own calls. A second walk at 79e's fifteen sizes,
light and dark, found nothing: no control of the zoom group covered, no
logic outside the stage, no sheet's close button focusable where it is not
drawn, More uncovered, no sideways scroll.

The client's commits on the redesign's chain (change ids, oldest first):
- `lqynnsvr` Keep the bottom box and its notes clear of the zoom group;
- `uvttxyly` Lay the new tab view's logics two by two where the stage is
  narrower than the row of four;
- `rwqnonxp` Keep a sheet's close button out of the tab order where no
  sheet shows it;
- `voytrqrn` Let a compact window under 480 high scroll its page, its rows
  apart;
- `woywszlv` Walk the frame at the window sizes where its regions met, in
  the axe and target-size tests;
- `rozvwqtl` Export natural deduction, modal decisions and lambda-terms
  through their own calls;
- `sskwzpwv` Describe the bottom box beside the zoom, the sheets, the short
  compact frame and Export's views in the client's rules;
- `okuuzlrq` End a long panel above the bottom box, and hide the zoom under
  a phone's sheet (found by the walk);
- `ynvuuuyn` Answer the review (the chain's head).

### Each fault, its cause and its fix

- **The zoom group under the bottom box** (600 to about 890 wide, and with a
  panel open at 840 and wider). Cause: the box was centred on the stage's
  region at up to 560 wide, whatever stood at its sides; the zoom (132
  pixels, at the stage's lower right, 24 from the bottom like the box)
  shared its band once the region was narrower than about 870, and a panel
  in an expanded window narrowed the stage (moving the zoom left) while the
  box stayed centred on the region. Fix: the box is centred where the zoom
  leaves room and otherwise narrows and moves left, its right edge a gap
  from the zoom (`--box-clear`, `--box-width` in `app.css`), from the zoom's
  width, which `Stage.tsx` measures onto the stage's region
  (`--size-zoom`, none while no zoom shows, so the new tab view's box is
  centred as before), and from how far a panel moves the drawing aside
  (`--size-aside`, which the stage's own `right` now takes too). The notes
  are the box's width, so no note covers the zoom either. A short window
  keeps the box at the lower left, as narrow as the zoom needs. At 1280 with
  no panel the box is where it was.
- **The zoom group under a verdict note in a compact window.** Cause: the
  stage ends above the box, but the notes rise from the box over the
  stage's bottom, where the zoom stood 8 from the edge. Fix: the zoom
  stands a gap above the notes (`--size-bottom`, the notes and the box as
  the page measures them). That lifted the popovers' sheets with it
  (`placeOver` put a sheet above the zoom), and axe's walk of a goal's
  reasons at a phone's width found a goal's formulas under the goal menu's
  sheet: a sheet now keeps its old place, above where the zoom stands with
  no note, and the zoom hides while a sheet is open.
- **Panels over the zoom** (found by this session's walk: Export in a short
  window, 844x390 and 640x400, and every compact sheet). Fix: a panel of a
  medium or expanded window ends a gap above the bottom box and its notes
  and scrolls inside (it lay to 10 from the stage's bottom before), and a
  compact window's sheet (a panel, the list of views) hides the zoom while
  it is open, as a popover's sheet does.
- **The new tab view's logics wider than the stage at 600 to 840.** Cause:
  four columns of 184 with their gaps and the view's padding need 51rem
  (816 pixels), and a medium window's stage is 540 to 779 wide (an expanded
  one's from 780). Fix: a container query on the stage lays them two by two
  below 51rem, as the compact window does (departs from the design, below).
- **The sheets' close button focusable where it is not drawn.** Cause:
  `.sheet-close { display: none }` came before `.tool { display:
  inline-flex }` in `app.css` with the same weight, so `.tool` won and the
  button stood in `.panels`' flow under the toggles or a panel. Fix: the
  rule is `.tool.sheet-close`, which outweighs it; a compact window shows
  it as before.
- **More covered at 640x400, rows overlapping at 320x200.** At 640x400 the
  box and its notes reached under the tools' column; with the box clear of
  the zoom (whose column is wider than the tools') they no longer do. At
  320x200 (compact and short) the top row, the views' row, the box and the
  bar need more than 200 pixels: the app now keeps 30rem of height
  (`--size-frame-least`) in a compact window under 480 high and the page
  scrolls up and down, never sideways (WCAG 1.4.10 allows one direction).

### Export for natural deduction, modal and lambda views

The panel tells the view by `kind()` and writes from `subject()`:
- natural deduction (the ordinary suite's `natural` method, its tree view):
  the session `Natural.tsx` keeps (`naturalSession`) through `rules_export`,
  with `view.layout` the tree or the Fitch lines as the toggles show them;
  SVG, PNG and PDF from that SVG, LaTeX and Typst as text. Its panel's own
  LaTeX, Typst and SVG buttons are gone, and the file is
  `deduxit-natural-deduction.tex` (it was `natural-deduction.tex`);
- a modal decision: LaTeX and Typst of a valid outcome's derivation through
  `modal_export`; the pictures through `modal_draw`, which draws the
  derivation or, for a sequent not valid, the countermodel
  (`deduxit-countermodel.svg`); LaTeX and Typst of a sequent not valid say
  that only a valid one has a derivation and point to the picture;
- a lambda-term: LaTeX and Typst of the checked term's typing through
  `lambda_export`, the pictures through `lambda_draw`
  (`deduxit-typing.svg`); a type with no term checked says to enter one.

Text, Session and Rocq say for each of the three that deduxit writes it as
LaTeX, Typst or a picture. The links of a modal or lambda tab hold the
sequent or term alone, as the address bar's do (they never showed before:
the link waited on a session these tabs have not). The Export tool and its
key `E` are open wherever a proof has something to write (`exportable`):
a session, a modal outcome, a lambda tab's term or type; natural
deduction's tool was greyed before.

### Decisions where the guide left a choice

- **The box gives way to the zoom, not the zoom to the box**: the guide puts
  both at the stage's bottom and says nothing of a stage too narrow for
  both; the box keeps its height, its distance from the bottom and its
  centre where there is room, and only its width and place change.
- **A compact window's zoom rises above the notes**, and hides under a sheet:
  the guide's compact layout gives the zoom no place of its own.
- **The panel's height** stops above the box and its notes in a medium or
  expanded window: the guide says a panel lies over the stage there, not
  over the box.
- **Natural deduction's Session entry is off**: the page's session file of
  that proof would hold the sequent, not the natural deduction steps.

### Departures from the design (for the author)

- **The new tab view's logics are two by two on a stage narrower than 51rem**
  (departs from the design, whose medium window keeps the expanded row of
  four): the row is wider than a medium window's stage. An expanded window
  up to about 876 wide lays them two by two too.

### The walk at every size

A Playwright script (`/tmp/claude-fixup-28/walk.mjs`), not a fresh agent: the
page served from a fresh build, at 320x700, 360x740, 390x844, 599, 600, 700,
839, 840 (each x900), 1024x768, 1280x800, 1440x900, 1920x1080, 844x390,
640x400 and 320x200, light and dark, in five states each: the new tab view,
a proof closed with its verdict, Steps open, Export's LaTeX, and a modal
decision's Export. In each it checked every zoom button and More for the
element at their centre, the logics against the stage, every sheet's close
button for being focusable but not drawn, and sideways scrolling. Its first
run found the zoom under Export in a short window (844x390, 640x400) and
under every compact sheet (Steps, Export); fixed in `okuuzlrq`, the second
run found nothing. Screenshots: `/tmp/claude-fixup-28/screens/` (150, the
fifteen sizes, five states, both schemes); the runs' logs
`/tmp/claude-fixup-28/runs/`. Held by eye against the guide at 700 (the
logics two by two), 600 (the box narrowed beside the zoom), 844x390 (a panel
ending above the box) and 390x844 (a modal decision's Export sheet).

### Checks

- `nix flake check` in the client under `flock -o /tmp/linlog-machine.lock`
  with `--max-jobs 2 --cores 6`: "all checks passed!", exit 0, at
  `okuuzlrq` (Vitest 733, e2e 357 passed, none flaky) and again at the
  head `ynvuuuyn` (the same counts, none flaky).
- Before it, on cores 4 to 9 with six workers against a fresh build: the
  whole Playwright suite 357 passed; Vitest 733; Biome and `tsc` clean. The
  new window-size walks and frame tests passed six times over in both
  schemes (48 runs).
- New tests: `tests/frame.spec.ts` (the zoom uncovered at six sizes and
  under Steps and Export, a short window's panel, a phone's sheet hiding
  it; the logics inside the stage at 700, 840 and 1280; the sheet's close
  hidden in a wide window and shown in a compact one; the 320x200 page's
  rows apart and its scrolling); `natural.spec.ts`, `modal.spec.ts`,
  `lambda.spec.ts` (Export of each view, its text, its file, an entry
  deduxit cannot write). `tests/states.ts`: the walk `windowSizes` (six
  states, in the axe and target-size tests at a desktop's and a phone's
  width) and an Export state in the natural deduction, modal and lambda
  walks.
- deduxit: no code change; the report alone.

### The review

`fresh-reviewer` (Fable 5.2 at `medium`) read the chain `lqynnsvr` to
`okuuzlrq` against the brief, the guide's sections, the client's rules and
the bindings' export calls; it ran nothing. It found the box's and the
zoom's arithmetic sound at every size it worked through, the logics, the
close button and the reflow sound, the three calls given settings they
read, the sequent proofs' paths unchanged, and no test weakened. Its
findings and the answers (all in `ynvuuuyn`):
1. *A modal outcome with neither a derivation nor a countermodel (unknown,
   or not valid without one) was offered as a countermodel to draw, and
   `modal_draw` refused the save.* Held: every entry then says deduxit found
   neither, LaTeX and Typst ask for a derivation, the file is
   `deduxit-countermodel` only when there is one. Not tested: no sequent was
   at hand that stops on the budget in the page's search.
2. *`started`'s doc comment sat on `exportable`.* Held; moved.
3. *Natural deduction's picture had no test.* Held: the test saves its SVG
   and reads the Fitch lines in it.
4. Question: *the rules said every new view's link holds the sequent alone,
   but natural deduction's holds the ordinary proof's session.* The
   behaviour is the address bar's (no link holds natural deduction's
   steps); the rules' sentence now says so.
5. Question: *More's rows may lie over the zoom* (a short window's column
   grown by More, a compact bar's third row). Not this chain's: More's
   rows are open until a tool is pressed; left open below.

### Left open

- More's rows open over the zoom group in a short window, and a compact
  bar wrapped to a third row, may cover it while open (the reviewer's
  question; More closes on any tool's press). Not measured.
- A modal outcome with nothing to draw is handled by reading, not by a test.
- What 79e left for the author stands, with this session's departure added
  to "Before approving the page".

## Session fixup-29: Steps named as the literature names them, "Use its proof" up to order, the search's engine said

Opus 5.5 at `high`, 2026-11-26, from the brief
`/tmp/linlog-handoff/start-fixup-29.md`, in the client workspace `fixup-29`
(`/home/tux/Projects/own/deduxit.github.io-fixup-29`) on the client's
`main@origin` (`ynvuuuyn`), and in the deduxit workspace
`deduxit-wt-step-fixup-29-…` on `main@origin` (this report alone), cores 10
to 15. Nothing pushed; no bookmark moved; no change to the library or the
bindings.

### Outcome

The author's three answers that were the client's are built. Steps names
each step by its rule and the formula it acts on ("⊗ on B ⊗ A", "∧R on
b ∧ a", "cut on A ⊗ B"). "Use its proof" is offered for every finished proof
at the top, in the same calculus, whose sequent has the goal's formulas on
each side in any order, and the replay finds each step's formulas in the
goal it acts on. A search that closes goals says which engine did, and the
hint's line names the engine whose proof it follows. The client's full
`nix flake check` passes at the head, `xxslynwr`.

The client's commits (change ids, oldest first):
- `kxvvqqzk` Name each step in Steps by its rule and principal formula;
- `mvprowtp` Offer every proof made apart of the goal's formulas, in any
  order;
- `krkwzlrz` Say which engine the search used: on Close all, a goal's close
  and Hint;
- `xxslynwr` Answer the review: eigenvariables by path, answers for their
  session, ordered calculi (the chain's head).

### What was taken for Steps, and why

A research agent (Sonnet 5.5 at `high`) read thirteen sources, each by its
URL. The textbooks label an inference by its rule alone, beside its line:
the Open Logic Project's sequent calculus chapter (∧L, ∧R, "named for the
main operator"), Pfenning's CMU 15-414 lecture 12 on sequents (∧R, and "the
principal formula of the rule"), Girard's *Proofs and Types*, chapter 5 (R∧,
L1∧, "the principal symbol"), and Wikipedia's sequent calculus page. The
tools say which formula by where the visitor presses or by a position:
Click and coLLecT (a press on a formula picks the rule of its connective),
the Proof Tree Builder paper (arXiv 2303.05865), the Sequent Calculus
Trainer (arXiv 1507.03666), the KeYmaera X cheat sheet ("apply tactic t at
position j"), Lean's tactics chapter of *Theorem Proving in Lean 4* (a
tactic and a hypothesis's name), the Isabelle tutorial (numbered subgoals)
and the Carnap-based forall x guide (a Fitch line cites its rule, then the
lines). No source puts a goal number in a step's label, and every one names
the rule by its connective. KeY's and Logitext's own pages could not be
read, so nothing is claimed from them.

So a step reads as its rule on its principal formula, the term the textbooks
use: "⊗ on B ⊗ A", "∧R on b ∧ a", "ax on b" (an axiom names its formulas),
"cut on A ⊗ B" (the cut formula), with no goal number; that is also the
guide's "each step by its rule and formula". A search's close keeps "The
⟨engine⟩ closed goal N": it has no rule.

### Each change

- **Steps** (`StepsPanel` in `Panels.tsx`). The session document numbers
  formulas and holds no text, so the text is deduxit's own. A linear
  session's steps and cuts are replayed in the worker from the sequent (one
  request, `principals` in `steps.ts`, over `play` with an observer), each
  goal read from the view just before its step; the searches' grafts are
  left out, since no later step lies under one. An ordinary step's goal is
  opened again (`reopen` in `ordinary.ts`, what undo already did) and read
  from `ordinary_view` (`principalTexts`). Until the answer for the session
  shown arrives, a step reads as its rule alone. With quantifiers each
  eigenvariable is written by the path of the step that made it and lettered
  back as the session letters it.
- **"Use its proof"** (`apartFor`, `useApart`, `replayOnto` in `state.tsx`;
  the goal's menu in `Goal.tsx` and `Stage.tsx`). Each candidate's sequent is
  read as deduxit's start writes it (`rootOf`, cached per proof and
  sequent) and matched to the goal's formulas by text and side (`placesOf`),
  as multisets. With several, each reads "Use the proof of ⟨tab⟩"; with one,
  "Use its proof". A proof written in the goal's order replays as before.
  In another order each step's principal formula, and what a split or a cut
  sends to its first premise, are found again in the goal the step acts on
  by their text (`principals` of the source, `principalTexts` for an
  ordinary one): no position is carried over, no substitution and no
  unification is made, and every step still goes through deduxit, so a
  refused replay stays "Not used". Formulas written alike are taken as one
  formula, since they are interchangeable in a proof. In an ordered calculus
  (cyclic, Lambek) a sequent in another order is another sequent, so only
  one in the goal's order is offered.
- **The engine said** (`closedBy`, `engineOf` in `engines.ts`; the line in
  `App.tsx`; the hint's line in `Stage.tsx`). A search that closes goals
  leaves a line over the bottom box, a notice of something done in the
  guide's sense: "Goal 2 closed by the focused search.", "Every goal closed
  by the focused search.", "3 goals closed: 2 by the focused search, 1 by
  the Horn engine.". It goes at the next change or when another search
  starts. The hint's line carries the engine's chip after "Hint.", as a
  goal's note already carried it after its verdict. The engine is the one
  the outcome names: in ordinary logic the image's engine under the
  embedding, and for a first-order goal (closed or hinted by the finite
  model search) the method, which the page now names "finite model search"
  with its sources as deduxit's `models.rs` cites them (McCune's MACE,
  Claessen and Sörensson's Paradox as Reger, Suda and Voronkov 2016 state
  it; taken from deduxit's doc comment, not read in the works).

### Checks

- Vitest: 37 files, 736 tests (new: `principals` read off a session in
  `steps.test.ts`, `closedBy` and `engineOf` in `engines.test.ts`). Biome
  and `tsc` clean; `astro check` 0 errors, 0 warnings.
- Playwright, new or changed: Steps' names in linear and in ordinary logic
  (`prover.spec.ts`, `ordinary.spec.ts`); a proof made apart in another
  order used, and another sequent's proof not offered (`prover.spec.ts`,
  `ordinary.spec.ts`); the same where the goal's eigenvariables take other
  letters, and Steps' letters after a search's graft (`first-order.spec.ts`;
  the first fails with the goal's letters left unconverted, checked once);
  the engine's line after a goal's close and Close all, the hint's chip
  (`prover.spec.ts`, `ordinary.spec.ts`), the finite model search named on a
  countermodel's note (`countermodel.spec.ts`). The hint test's four lines
  now read "Hint. Focused search …": that behaviour was meant to change.
- `tests/states.ts`: Steps named by rule and formula, the goal's menu
  offering two proofs made apart in other orders, and the engine's line on
  the proved state; the axe walks (both schemes, desktop and phone) and the
  target-size walks pass.
- `nix flake check` in the client under `flock /tmp/linlog-machine.lock`
  with `--max-jobs 2 --cores 6`: "all checks passed!", exit 0, at
  `krkwzlrz` (Vitest 736, e2e 360 passed, none flaky), and again at the head
  `xxslynwr` (Vitest 736, e2e 362 passed, none flaky).
- Screenshots at 390x844 and 1280x800, light and dark: the hint's chip, the
  engine's line after Close all, Steps named, the goal's menu with two
  proofs made apart, and the goal closed by one
  (`/tmp/claude-fixup-29/screens/`, 20 files; the script is
  `/tmp/claude-fixup-29/shots.mjs`).

### The review

`fresh-reviewer` (Fable 5.2 at `medium`) read the first three commits
against the brief, the guide's sections and the client's rules; it ran
nothing. It found no fault of soundness (every replayed step goes through
deduxit, and a replay that leaves a goal open is refused). Its findings and
the answers (all in `xxslynwr`):
1. *A proof made apart whose eigenvariables take other letters than the
   goal's (each session letters them in the order its ∀ steps come) was
   found by its text and refused as "Not used".* Held: the source's
   formulas are written with each eigenvariable by the path of the step
   that made it (`byPaths`), and so are the goal's, relative to the goal;
   tested.
2. *Steps' replay leaves the searches' grafts out, so a ∀ by hand after a
   graft's took another letter than the session shows.* Held: the same
   paths, lettered back as the session letters them (`lettered`); tested
   (the hand's ∀ takes `b` after the search's `a`).
3. *Proofs in another order were offered in the ordered calculi (cyclic,
   Lambek), where they are other sequents.* Held: only one in the goal's
   order is offered there.
4. *Steps showed the last answer's formulas, by index, until the new one
   came.* Held: an answer is kept with its session and used only for it.
5. *The doc comment said a step whose replay failed has no formulas; the
   observer had read them before the step.* Held: such a step, and every
   one under it, names nothing now.
- Question: *with the guide off the hint has no line, so no engine.* Kept:
  the hint's words show with the guide on (79b's design), and its ring
  says nothing in words.
- Question: *a closed goal's line stayed over a later search's verdict.*
  Held: a new search takes the line away.
- Question: *an ordinary goal's countermodel is the SAT solver's, while the
  chip names the image's engine.* Kept: the outcome deduxit answers records
  the embedding's search alone; naming the solver needs the bindings to
  record it (below).
- Question: *a cut formula with eigenvariables read "{0.1}" in Steps.*
  Held: lettered as the session letters it.

### Left open

- An ordinary first-order proof made apart in another order whose
  eigenvariables differ is still compared by its letters, and so is "Not
  used" where the letters differ: the ordinary goal state names its
  eigenvariables by number and the page has no path for them. No wrong
  proof follows.
- An ordinary goal's Kripke countermodel names the image's engine: the
  bindings' `ordinary_close` would need to record that the SAT solver found
  the model (deduxit, not this client).
- Two or more "Use the proof of …" buttons share one sign; their names show
  with the guide on and in their hover.

## Session fixup-30: no suite named as the main one

The author's word of 2026-11-26 ("linear logic is not the 'main'
anymore"), `.claude/rules/writing.md` rule 3. Opus 5.5 at `high`, in
`../deduxit-wt-fixup-30-3e23c4`, on `main@origin` (`tttkyssp`).

### What changed where

- `lmxszxrl` README and the manual:
  - README's opening lists the suites in the page's order (ordinary,
    linear, modal, lambda-terms, rule files). "A first proof" leads with
    ordinary logic (`--logic`), then linear logic, then one modal and one
    lambda-term example. The linlog history sentence is gone (it is the
    changelog's), and "other logics" became "in each logic".
  - `docs/src/introduction.md`: the same opening, with each suite linked
    to its chapter. The tour goes one suite at a time: first proof and
    countermodel in ordinary logic (with the exit statuses), then linear
    logic (one-sided trees, `A, B |- A` and `-a`, `-i` and `!A`, the
    `-i` tree dropped, `commands.md` has it), then modal logic (`--system
    T` valid, `--system K` refuted), lambda-terms (`--lambda`, `term
    check`), and calculi of your own (`check --calculus`, `interact
    --calculus`). Export and check are unchanged.
  - `mistakes.md` links `#linear-logic` in place of the removed heading;
    `ordinary-logic.md`'s first line no longer says "also decides …
    propositional"; `architecture.md` lists every suite, not three.
- `mulvomtq` the guide: `SKILL.md`'s description and summary name the
  suites evenly and ordinary logic as first-order; "The logic" comes
  before "Syntax" and starts with `--logic`; quantifiers are said to work
  under `--logic` and in linear logic but for `--cyclic` and `--lambek`;
  the summary names `term check|inhabit` and `check --calculus`, and the
  library line names a module per suite. Budget: description 831
  characters, body 91 lines and 7 986 bytes (cuts: the fragment list after
  `seq fragment`, the web page under Optional, the pointer to section
  19). `reference.md`: the opening, the `--logic` row of section 3 and
  section 13's first sentence say first-order too; section 1 lists every
  command.
- `vmkuxsvu` crate docs: the front page names `ordinary`, `linear`,
  `modal`, `lambda` and `rules`, shows the ordinary module's example
  before the linear one, and orders "What you can do" by suite. The
  `ordinary` module's docs said "propositional" and "Nothing decides
  first-order validity here", false since 0.8.0: corrected from
  `Options::method_for` and `decide`.
- `opynoykl` the crates' descriptions, `CITATION.cff` (abstract and
  keyword order) and `deduxit --help`'s summary, ordinary logic first;
  the core crate's keyword `proof-nets` gave way to `logic`, and
  `linear-logic` moved last in both. One changelog entry (Changed),
  since user-facing descriptions changed.
- `tnmlvstn` the behaviour lock reblessed: `error-no-command.txt`, the
  help's first line, alone.
- `wnrylqsy` CLAUDE.md's opening and its line on the core crate.

### The reviewer

`fresh-reviewer` on Fable 5.2: one finding, held. `SKILL.md` still said
"a SAT solver decides it" of ordinary logic, false at first order
(`prove --logic classical "forall x. p(x) |- p(a)"` answers by the
connection prover): now "`--method` picks how it is decided", folded
into `mulvomtq`. One question, kept: `deduxit --help`'s syntax text below
the summary shows linear syntax first; it follows the real default (no
flag reads linear logic) and says how `--logic` reads ordinary logic.
Everything else checked: every new console block run, the flags,
commands, anchors, section numbers, the module paragraph against the
code, the budget.

### Checks

- `cargo test -p deduxit-cli --test readme`: 7 passed, after the last
  change too.
- The gate at `wnrylqsy` before the reviewer's one-line fix (capped,
  cores 10-12): passed; the ratchet's misses moved on
  `search-chain-128` and `check-qbf-20-2`, a note and never a failure, on
  a change of text alone.
- `nix build .#checks.x86_64-linux.manual
  .#checks.x86_64-linux.manual-links` under the machine lock: built.

### For the author (asked through the planning session)

- `prove` without `--logic` or `--system` reads linear logic, so every
  text introduces ordinary logic with a flag. Recommendation: keep the
  default (scripts and the web page rely on it, and the texts now name
  the flag first); a required logic flag would be the even alternative.
- Left as they are: the reference's sections 2 to 12 and the manual's
  how-to order still give linear logic the most room, and
  `docs/src/syntax.md` is linear logic's syntax with ordinary logic's in
  its own chapter. Evening those out is a restructuring of its own, not
  a wording fix. `docs/src/status.md` keeps its order, as the brief says.

## Session fixup-31: the Kripke check and equality, resolution's read-back bounded

Opus 5.5 at `high`, in `../deduxit-wt-step-82-141614` on `main@origin`
(`supnspsz`), cores 4 to 6 (the panel's on 7 to 9), in the conversation
of steps 82 and 83. Paused 09:18 to 09:40 for the author (the planning
session's "Pause" and "Resume"); the panel's first run was cut there and
run again whole. Nothing pushed; rocq-deduxit unchanged.

### What landed

- `prxrsxml` (F-4d-01, soundness): `KripkeStructure::refutes` reads `=`
  as identity at every world (`Forcing::atom`) and refuses a world that
  lists `=`, as `Structure` does, so a correct model with `=` is still
  accepted (`|- exists x. ~x = a` on a growing domain). The audit's
  witness (one world, no facts, `⊢ a = a`) is refused at the library's
  `check` (`equality_is_identity_at_every_world`), at `deduxit check`
  (`a_kripke_model_does_not_refute_reflexivity`, exit 1, "invalid
  refutation") and at the protocol's re-check
  (`a_forged_kripke_model_with_equality_fails`); all three fail with
  the identity reading switched off. Changelog ("Fixed"), the manual's
  `ordinary-logic.md`, `core-ordinary.md`.
- `vqozxsnq` (the Rocq side): rocq-deduxit's first-order Kripke
  semantics (`FirstOrderKripke`, `FirstOrderCheck`) reads `=` as a plain
  predicate, and the exporter listed no `=` facts, so **every
  certificate of a countermodel of a sequent with `=` true anywhere did
  not compile**, the equality axioms of `prove --format rocq` included
  (`prove --logic classical --format rocq '|- a = b'`: "Unable to unify
  true with false" through `nix run .#check-certificate`). `facts` now
  writes `("=", [e; e])` per element of each world where the sequent
  has `=`. The command's two certificates and two new snapshots
  (`deduxit/structure_equality.v`, `deduxit/countermodel_fo_equality.v`)
  compile, "proved, with no axiom". The audit's forged model would have
  had a certificate that compiles and is true (without the axioms LJ
  does not derive `⊢ a = a`), so nothing false was certified, and the
  library needs no change.
- `xuupvryk` (F-4c-01): the read-back's `mapped_atom` is held to
  `limits.terms` before the key is written (`within_terms`, shared with
  `applied_atom`), each key's memory is charged before it is written
  (`mapped_atom_charged`) and the arena it grew after, the stop is asked
  per instance and per root copied, and `bindings_text` cuts a binding
  at 4 096 bytes with `…` (`Capped`). The protocol's check takes a
  document's whole binding for the cut text (`same_bindings`), so a
  document of 0.8.0 still checks; the manual (`resolution.md`), the
  schema's `unifier` and the guide say the cut. The audit's family under
  `resolve --timeout 1s`, release: n = 20 to 23 end at 1.07 to 1.18 s
  (the read-back stopped), n = 24 and 26 at 0.04 s (the terms bound);
  debug n = 23 at 2.3 s; before, debug n = 22 took 4.6 s and n = 24
  22 s. n = 23 under `--memory-limit 1MiB` is refused in 20 ms and fits a
  20 MB scope. Test `the_read_back_keeps_to_the_bounds_on_shared_terms`:
  its terms half fails with the terms check removed, its memory half with
  the charge removed (it then gets Herbrand's own refusal).
- `kmxoxyxv` (the argument member's older finding): the model search
  on a sequent with `=` in intuitionistic or minimal logic, with
  `--model-size` given and no `--model-worlds`, ran its loop to
  `u32::MAX` levels without asking the stop and then overflowed asking
  for a Kripke model of that many worlds (`prove --logic intuitionistic
  --method models --model-size 2 --timeout 2s '|- a = b \/ ~a = b'`:
  26 s, then a panic). With `=` the levels are the structures' sizes
  alone and the end is `ModelSize` (`equality_ends_at_the_size_bound`);
  the witness now answers unknown in under a millisecond.

### The panel (on `prxrsxml` and `vqozxsnq`)

- `panel-argument` (Fable 5.2, high): the argument holds for both. The
  invariant stated from the code (an accepted model is a Kripke model of
  the equality axioms: shared elements, global constants, tables
  agreeing along the order, so identity is persistent and forces every
  axiom), checked at every site (the evaluator, the up-front passes, the
  callers, `From<Structure>`, minimal logic); the diagonal facts satisfy
  the library's growth check and its reading agrees with the Rust one.
  One older finding, the model search's loop, fixed in `kmxoxyxv`. A
  note: a certificate states the sequent as the countermodel holds it,
  so for a sequent without the axioms Rocq proves the weaker statement
  (`=` uninterpreted); nothing false.
- `panel-counterexamples` (Opus 5.5, high): no counterexample. Its own
  forcing evaluator (recursive, every refusal re-implemented) agreed with
  `refutes` on 3.5 million random models of random sequents with `=`
  (1 to 4 worlds, mutations), and with the protocol on 452 760 forged
  documents; `ordinary::decide` never called a refuted sequent valid
  (13 601 cases); 23 valid sequents with `=` refuted by no model;
  15 forged documents through `deduxit check` answered right. One older
  finding, below.
- `panel-integers` (Sonnet 5.5, high): no refutation. `=` of another
  arity or role is refused by the readers, so the identity arm always
  matches; work and stop counted; the memory charge before any build;
  the exporter's bounds: with `=` a domain of 1 023 elements or more is
  `LargeModel` (the diagonal weighs n(n + 1) against `MOST_WEIGHT`),
  4 096 without, a refusal and never a wrong certificate. Noted:
  `has_equality` is true where the symbol `=` is declared though no atom
  uses it, which can only refuse a certificate earlier.

### The reviewer

`fresh-reviewer` (Fable 5.2, medium), on all five commits: the soundness
fix, the Rocq facts and the model search hold. Four findings on the
read-back, all answered in `xuupvryk`:
- the memory charge came after the key was written (100 to 150 MB
  resident under `--memory-limit 1MiB` at n = 23): now charged before;
- the time limit passed between polls (debug n = 23, 3.5 s at 1 s): a
  poll per root copied added; one key of up to `limits.terms` symbols is
  still written between two polls, 0.18 s past the limit in release,
  1.3 s in debug, as the connection prover's read-back writes one
  instance unpolled (`core-ordinary.md`);
- the test's memory half could pass on Herbrand's refusal: it now asks
  for the read-back's own words and fails without the charge;
- a 0.8.0 document with a long binding failed the protocol's check:
  `same_bindings`, and the manual, schema and guide say the cut.

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings`: clean.
- `cargo test --workspace` (before `kmxoxyxv` and the review's fixes):
  every binary passed; after them the gate's run.
- The gate at the head (capped, cores 4-6): passed, the tests among it;
  the ratchet's misses moved on `search-chain-128` and `check-qbf-20-2`,
  a note and never a failure, on no search change.
- The Rocq certificates with `=`: four compiled with `nix run
  .#check-certificate` (two the command writes, the two snapshots).
- `nix flake check` at `kmxoxyxv` under the machine lock (cores 4-6):
  all checks passed (`rocq-deduxit` compiling the two new snapshots,
  `wire-schema`, `web-core-wasm32` among them).

### Follow-ups (for the planning session)

- **The protocol's check of `decides` ignores the call's input**
  (the counterexamples member, older than this session): `decides` reads
  the output alone, so a valid derivation or a countermodel of another
  sequent is `Held` (input `|- a = a`, output a structure refuting `|- a
  = b`: `Held`). It needs the input's sequent compared with the
  evidence's (formulas' texts as multisets, as `ordinary.instances`'
  check does), for every kind of evidence. Its own fixup.
- `Error::Structure` says "the finite structure is wrong" for a Kripke
  model too (`deduxit check` of the audit's document).
- The Kripke model search tries no model for a sequent with `=`; with
  the check reading identity it could, at least for identity worlds.

## Session fixup-35: four ILTP problems the TPTP reader refuses

Opus 5.5 at `high`, in `../deduxit-wt-step-82-141614` on `tuxxnwsr`
(87c4), cores 10 to 12, in the conversation of steps 82, 83, 87b2, 87c2
and 87c4. No panel (the fix changes a message, not how a formula is
built). Nothing pushed.

### What the defect is

`GEJ056+1`, `GEJ056+2`, `GEJ057+1` and `GEJ057+2` write their conjecture
as `((! [A,B] : (point(A) & point(B) & dipt(A,B))) => (? [X] : (line(X)
=> ((~ apt(A,X)) & (~ apt(B,X))))))` (and with `L`, `M`): the
quantifier's scope closes before the consequent, which still uses `A`
and `B`. These are free variables of a `fof` formula, which TPTP does
not allow: its language description (tptp.org,
`UserDocs/TPTPLanguage/TPTPLanguage.shtml`, "Annotated Formulae") says
that outside the CNF language every variable must be bound by a
preceding quantification with adequate scope. (Its BNF moved to
`bnf.tptp.org`, which the fetch could not read; the rule is the
description's prose, and a context-free grammar could not state it
anyway.) So the reader's refusal is right and the files are not valid
TPTP; the reader's message only said "expected a bound variable or a
constant", at the word's first letter.

### What landed

- `oyqxutpm` "Name a TPTP variable outside its quantifier's scope, and
  why it is refused": the error spans the whole word and carries a hint,
  "an upper-case word is a variable in TPTP, and no quantifier around it
  binds it: each variable of a fof formula stands inside its
  quantifier's scope". A test of the smallest failing formula,
  `(! [Ab] : p(Ab)) => q(Ab)`, in `reads_tptp_quantifiers` (fails
  before: the old message names `"A"` and has no hint). The changelog
  (`[Unreleased]`, Changed) and `core-ordinary.md` say so.

### Checks

- Every ILTP first-order problem read before and after (`deduxit seq
  print --logic classical --input-format tptp`, both release builds,
  output and status digested per problem): of 2 670, exactly the four
  read differently, by the hint alone; the other 2 666 are byte for
  byte the same, the fifth error among them (an occurrence limit,
  unchanged).
- The parser's tests; `gate` passed (the ratchet's `read-tptp`
  34 275 138 instructions, the same as at `tuxxnwsr`: the path is cold).
  No local `nix flake check`: the planning session runs every flake check
  on GitHub's runners before anything lands (its rule since that day),
  and the one started here was stopped while it waited for the lock.

### The reviewer

fresh-reviewer (Opus 5.5, high; Fable's roles moved to Opus that day):
the code change is sound and the brief is met. The diagnosis holds, the
error is made only where it was, the span is the raw word's bytes (taken
before NFC normalisation), `Clauses::located` keeps the span and the
hint, no retry of `read_tptp` depends on the error's form, and the test
fails without the fix. One false statement in this report, fixed: the
follow-up called a universal closure over the whole formula the intended
statement, which it is not (the antecedent keeps its own `∀A∀B`); the
rules file had the same gap.

### Follow-up (for the author, through the planning session)

The four problems stay refused. ILTP rates them "Theorem" (0.50), so
some provers read them anyway, presumably taking the free variables as
universally closed. That closure does not give the intended statement
("when there are two distinct points, a line connecting them can be
constructed", `∀A∀B. ((point(A) ∧ point(B) ∧ dipt(A,B)) → ∃X. …)`):
closed over the whole formula, the antecedent keeps its own `∀A∀B` and
says that every pair is two distinct points, a different formula and
likely a trivial one where `dipt` is irreflexive (the reviewer's
witness). The files' intended meaning needs the quantifier's scope moved,
an edit of the problem, not a reading. My recommendation: keep refusing
(TPTP's rule, and no reading gives the intended statement), and leave the
four out of the ILTP counts with this reason.

## Session fixup-33: an unknown first-order sequent gets no Rocq certificate

Opus 5.5 at `high`, in `../deduxit-wt-step-87c1-64a1a5` on `nvwqyuvr`
(87e3), cores 7 to 9, in the conversation of steps 87c1, 87c8, 87c7,
87c6, 87e4 and 87e3. Panel (argument and counterexamples, as the brief
asks) and the reviewer. Nothing pushed.

### What the defect is

`prove --logic classical --method embedding --format rocq 'p(a) |-
q(b)'` (87d4's witness, the smallest there is) answers unknown, exit 3:
the image is unprovable, and at first order the translation may lose
proofs (`ordinary::incomplete`). It still wrote the image's classical
assignment as a certificate over `Prop`, `Lemma certificate : ~ (forall
p_28_a_29_ q_28_b_29_ : Prop, p(a) -> q(b))`: `falsified` binds the
atoms by their identifiers and the formula printer writes a first-order
atom by its predicate and terms, so Rocq refused the file. In
intuitionistic and minimal logic the same unknown printed `no
certificate: only a classical assignment …`, the library's refusal of a
refutation that is no assignment.

### What landed

- `ovskpxkq` "Write no Rocq certificate for an unknown first-order
  sequent": `prove.rs` writes nothing for `--format rocq` where the
  command's line says unknown for that reason (`Shown::Nothing`, as for
  every other unknown; under `--linear` too, whose line says unknown as
  well, though its exit status is the image's). The manual's Rocq
  chapter and `status.md` give a certificate to a decided verdict only,
  and the honest certificate of such a sequent's non-validity is the
  first-order one from a checked structure, which the default path
  (grounding, the model search) writes. A test,
  `an_unknown_first_order_embedding_has_no_certificate` (classical and
  intuitionistic), fails before the fix. The changelog's `Fixed` and
  `cli.md` say so; no example of the manual or the guide shows this
  case.

### The net

- The gate at `ovskpxkq`, on cores 7 to 9: "gate: passed" (Clippy,
  all 39 test suites, both `cargo hack` runs, the ratchet).
- `nix build .#checks.x86_64-linux.rocq .#checks.x86_64-linux.check-certificate`
  under the machine lock: both built. `rocq-deduxit` was not run: no
  certificate's text changed, only whether one is written.
- No local `nix flake check` (the planning session's rule).

### The panel

- **Argument** (`panel-argument`, Opus 5.5 at `high`): no break. Every
  path to a certificate over `Prop` (`prove`, the batch, `check`,
  `interact`, the workspace, the protocol, the web bindings) either
  goes through the guarded arm or never meets a first-order arena (the
  classical countermodel path is refused at first order by its check,
  `check` writes the image's linear certificate); a certificate now goes
  with a decided verdict wherever the command writes one; writing
  nothing is right against a statement Rocq would accept, which would
  state only the propositional abstraction under a line that says
  unknown. Two notes, both outside the change (below).
- **Counterexamples** (`panel-counterexamples`, Opus 5.5 at `high`): no
  certificate written for an unknown in about 16 100 inputs (every
  logic and method, `--linear`, the batch, `check` of the outcomes,
  linear sequents in every mode and kernel, atom names that are Rocq's
  words); 6 998 certificates written, each compiled by the flake's
  `check-certificate`, 6 996 accepted. The head and the base agree on
  9 940 deterministic inputs but for the 101 certificates of an unknown
  the head no longer writes (Rocq refused all 101 of the base's) and 256
  `no certificate: …` lines it no longer prints. Its own reference
  (a truth table, Dyckhoff's G4ip) agreed on 2 249 propositional
  verdicts. The two certificates Rocq did not accept are below.

### The reviewer

`fresh-reviewer` (Opus 5.5 at `high`): no finding. It ran the new test
on the old `prove.rs` (it fails, with the refused certificate as the
witness) and on the new (it passes); checked that the guard's condition
is the verdict line's (`decide.rs::finish`, the one place `Searched` is
built) and placed before the old arm, so every other case writes what it
wrote, and ran the neighbouring inputs (propositional not valid in each
logic and under `--linear`, the default path's first-order certificate)
to see them unchanged; and found the records true and the manual, the
guide and the behaviour lock without an example of this output.

### Follow-ups (for the planning session)

- **A certificate over `Prop` can run Rocq out of memory**: two the base
  writes alike (classical by the SAT method, and minimal under
  `--linear`, both propositional and not valid, 607 bytes and less)
  took over 8 GB in `tauto` after the `specialize` and were killed after
  35 and 52 minutes; a third compiled after 17 minutes. The statements
  are true. A proof that rewrites the formula by the assignment
  (`cbv`, or a reflected evaluation) in place of `tauto` would end
  quickly; the inputs are in the counterexamples member's scratch
  (`/tmp/claude-1000/panel-cx-f33/work/slow1112.seq`, `slow1600.seq`).
- **The library's `rocq::write_not_valid` still writes the refused text
  when handed a ground first-order image** (the command no longer does):
  refusing it needs an `Unsupported` variant or a statement over the
  first-order calculus, part 2's matter.
- **`check` confirms a hand-edited `not_valid` outcome of a first-order
  sequent** by the image's linear refutation (exit 0, a true lemma about
  the image), though deduxit writes such an outcome as unknown
  (`serialize/ordinary.rs` takes "not valid" over an unprovable image at
  any order). The argument member could not turn it into a wrong
  answer.

## Session fixup-34: the library's default gives the model search a turn under a clock

Opus 5.5 at `high`, in `../deduxit-wt-fixup-30-3e23c4` on `yqumnktz`
(87c5), cores 13 to 15, in the conversation of 87c3, 87d4 and 87c5. The
panel on the change of `decide_within` (argument and counterexamples on
Opus, integers on Sonnet, all `high`), fresh-reviewer on Opus at `high`.
Nothing pushed.

### The defect

87c9b's F-7b-05: `ordinary::decide_within` ran the model search only
after the embedding ended unknown without being stopped. Under a clock
the provers run until the stop on most sequents they do not prove, so
the model search never ran, and the command kept an order of its own
(time shares, `by_connection`'s race, `races_the_embedding`,
`model_first`).

### Choosing the order by measurement

Everything measured on ILTP's first-order problems at 2 s on cores 13 to
15, with the harness patched for the measurement only (never committed)
to record the units each method's stop is told of; the rows are in the
session's scratch directory.

- **Model search first, within a budget** (the brief's second option)
  does not work. Its units are the solver's decisions, not its
  propagations: on the 474 problems it refutes it spends 25 to 92
  thousand units per ms (the most any needs is 26.7 M, NLP240+1 in
  classical logic, 289 ms), on theorems a median of 4.6 thousand and as
  few as 58 (SYN083+1: 2 s of `batsat`'s `propagate` on 36 atoms). A
  budget that covers the 474 (2²⁵) is seconds on most theorems, so
  every theorem would wait for it. batsat's stop hook sees no
  propagations (its `propagation_budget` has no setter), so a fairer
  unit needs batsat changed: a follow-up.
- **A budget for the embedding before the model search** (the first
  option) works only as part of two rounds: first attempts within small
  budgets (grounding, the connection prover in classical logic, the
  embedding), then the model search within 2²⁵, then the long attempts.
  A theorem the first attempts miss waits for the model search, which is
  what the order costs.
- The budgets came from a simulation over the measured rows (each
  attempt's cost at the row's own rate), then the real run. Per
  attempt: the prover's theorems need at most 2²¹ steps for 812 of
  848 (2²² saves 21 more but leaves SYN324+1's structure no time: its
  prover's 2²¹ steps take 1.7 s); the classical embedding's 60 theorems
  all fit 2²⁰ (its affine image's search costs six times the time per
  unit of the intuitionistic one); the intuitionistic embedding 2²³
  (2²⁴ would leave two Kripke models no time); grounding 2²² of its
  summed units (456 of its 467 classical verdicts and 82 of 90
  intuitionistic ones).
- A first full run without a budget for grounding lost three of the
  474 (SYN514+1, SYN518+1, SYN520+1) and SYJ036+1's theorem: grounding
  ran until the stop or until its node bound at 1.8 s, where the old
  order cut it at 90% of the limit. Grounding now has its first attempt
  too.

### What landed

- `wkkwqwwv` "Give the model search a turn before the long attempts of
  the first-order default" (`core/src/ordinary/decide.rs`): `first_order`
  runs the first attempts (grounding within `FIRST_GROUNDING_WORK`, the
  connection prover within `FIRST_CONNECTION_WORK`, the embedding within
  `first_embedding_work`), the model search within `MODEL_WORK`
  (`model_turn`), the attempts that ended at their budget again
  (`Attempt::Budget`; a caller's lower `limits.work` stays the bound),
  and the model search again where its budget ran out and the embedding
  ends unknown, unstopped. `within_budget` counts grounding's and the
  model search's budgets as the summed `Progress::work` of their polls
  (their preparation and encoding count no `limits.work`). The
  embedding's steps are `embedded`, which both rounds call. No clock and
  no thread: the order is a function of the input. The new constants
  are private (no change of the public API). A test,
  `the_model_search_has_its_turn_before_the_long_attempts`, refutes a
  classical and an intuitionistic sequent (the manual's Kripke example)
  under a stop that stands for a clock; both fail on the old order
  (`Unknown(Stopped)`). `decide_within`'s docs give the order and why,
  and the rules file says why these budgets.
- `tstwmlro` "Decide a first-order sequent by the library's default in
  the command": `natively` hands a first-order sequent with no method
  (and no `--net`) to `decide_within` whole (`by_library`,
  `by_default`, as 87c9b's parked `psvsuuvz` did on an older base); the
  shares, the race, `model_first` and `Deadline::share` are gone. An
  outcome the library decided through the embedding is written with the
  embedding's verdict line and the linear statistics; a refusal is an
  unknown line naming its flag in `prove` and the library's error in a
  batch. The manual's Kripke example names `--timeout 10s`: the README
  test runs a debug build, in which the embedding's first attempt takes
  6.6 s on that sequent (0.3 s in a release build), past the default
  limit. The manual, the guide, `cli.md` and the changelog say the new
  order. The behaviour lock needed no change.

### The measurement

`bench/command-first-order.py BIN bench/iltp-first-order/Problems
AFTER.csv 13,14,15` with a release build of `tstwmlro`, nothing built
beside it, against `bench/defaults/command-first-order-before.csv`; the
after file is `bench/defaults/command-first-order-after.csv`.

- **No decided verdict flipped.** Every one of the 474 refutations the
  model search gave comes back (207 classical, 267 intuitionistic).
- Decided rows: classical 1 290 before, 1 271 after; intuitionistic 859
  before, 861 after.

| logic | method | lost | gained |
|---|---|---|---|
| classical | connection prover, valid | 32 | |
| classical | grounding, valid | 2 | |
| classical | model search, not valid | | 15 |
| classical | SAT solver, valid (timing at the limit) | 1 | 1 |
| intuitionistic | embedding, valid | 42 | 2 |
| intuitionistic | model search, not valid | | 42 |

- The lost theorems need more than their first attempt's budget and
  then wait for the model search's turn, seconds on a theorem: the
  prover's (ALG, GEJ, SET, KRS above all; 3 to 4 M steps, which took
  0.2 to 0.5 s when the prover had 70% of the limit), the intuitionistic
  embedding's (GEJ, SET, KRS above all), and grounding's KRS162+1 and
  SYN723+1 (0.6 s before). The SAT rows are propositional problems at
  the limit whose code did not change (SYJ202+1.010 lost, SYJ208+1.010
  gained).
- The gains are refutations the model search finds now that it runs
  in every logic after the first attempts (NLP and SYN above all), and
  two intuitionistic theorems.
- Times: the classical refutations of the 474 now take a median of
  0.25 s (1.41 s before, after the prover's 70%); the intuitionistic
  ones 0.26 s (0.01 s before: the model search went first on one
  thread), the embedding's first attempt being most of it. One of the
  474 is at the edge: SYN539+1 in intuitionistic logic at 1.99 s
  (1.83 s before), where grounding spends its budget (2²² units take
  1.7 s there) before the model search finds a one-element structure in
  10 ms.

### The panel

Argument and counterexamples on Opus, integers on Sonnet, all `high`,
from the brief in the session's scratch directory.

- **A break, found by all three, fixed.** The embedding's first attempt
  handed its budget to the read-back and to `Derivation::check`, which
  refuses past `limits.work` inferences with `Refusal::Work`, an error
  that `attempt` filed as given way for good: no long attempt, and the
  answer was "the call did the most work allowed, 1048576 units; raise
  it with --work-limit N", a bound the user never set and could not
  lift. Witnesses: a classical chain of 1 200 000 negations (the
  argument member; `--method embedding` proves it in 2.3 s, the head
  said unknown), and intuitionistic sequents whose image's proof is a
  few thousand units and whose derivation of LJ has 9.6 million
  inferences (the other two). Fixed in `wkkwqwwv`: the budget bounds
  the image's search alone, the read-back and the check run under the
  caller's limits (`embedded`'s `(limits, searched)`), with
  `a_budget_bounds_the_embedding_s_search_alone`. The classical witness
  is valid at the head; the intuitionistic one is too by the
  counterexamples member's build of the head.
- **The stop after the model search's turn** (argument): where the
  caller's stop ended the turn, `first_order` went on to the long
  attempts, relying on the stop to fire again, and a long embedding
  translates the whole image before its first poll. Fixed: a stop in
  the turn ends the call with the model search's unknown
  (`Turn::Answer`).
- **`--translation` alone reached the model search** (argument), against
  `--help` ("--translation and --linear name the embedding"): fixed,
  `by_library` takes a sequent with no translation given; with one, the
  command's own embedding decides, as before.
- **`--method`'s help** still described the race (argument): fixed in
  `tstwmlro`; the lock's `help-prove` reblessed in `tnvpowzr`.
- Remarks kept: grounding's polls report `POLL` units whatever a step's
  weight, so its summed budget can overshoot by the weight per poll
  (1.8 times on a nest of 8 000 quantifiers; bounded by the occurrence
  and memory limits, deterministic); a caller's `limits.work` of the
  budget plus one repeats the first attempt in the long one; the order is
  a function of the input on one thread only (a pool's search can end its
  budget at another point); a refusal line names the method the options
  choose, not the one that refused.
- **No wrong verdict** (counterexamples): 858 random first-order
  sequents in three logics, the default against grounding, the
  embedding, the model search and the connection prover, and a
  brute-force classical evaluator on every sequent any method called
  valid; 4 050 runs under stops at many work counts, every verdict equal
  to the unstopped one, at most two polls after a stop, no `WorkLimit` the
  caller did not set; 316 pairs identical twice over; 1 152 runs at work
  limits around the budgets, each naming the caller's value; 154 runs of
  the command's output flags.
- **A follow-up, not this change** (counterexamples): the model search
  returns 0.4 to 1.5 s after its stop fires on SYJ001+1.020 and
  SYJ003+1.013 (no poll after the stop: the time is spent after its
  last poll). `--method models` does the same on the base, and the old
  command's model search ran under the same clock; the default now
  reaches it on every first-order sequent the first attempts leave
  open.

### The reviewer

fresh-reviewer (Opus 5.5, high): the requirements hold. It compared the
two CSVs itself (no flip; all 474 refutations not valid after; every
figure of the table and the changelog as stated; the committed after
file byte for byte the run's) and re-ran the 474 with the head's
release binary at 2 s: all refuted. It found, all fixed:

- the new test starts a thread, which `web-core-wasm32` (WASI) cannot:
  it joins `core32Skipped` in `modules/web.nix`, with the counts in
  `web.md` and `flake.md`;
- `by_library`'s doc gave the model search before the provers;
- `Options::method_for`'s public doc and the test
  `the_default_asks_the_model_search_last` (now `…_model_search`) still
  said the model search runs last;
- four doc comments justified their numbers by the order this change
  removed ("the command's own order", "when the prover had 70% of the
  limit"): reworded to what the numbers measure.

It also noted that the read-back fix could turn a measured row into a
stop as well as a verdict; no row of the after file names a work limit,
so none met it.

### Checks

- The library's tests, the two new ones among them (the first also run
  on the old order, where it fails); the workspace's tests and clippy at
  the head; `gate` passed before the panel's fixes (the ratchet's
  ordinary journeys within their ceilings, as at 87c5), and again at the
  head after the panel's fixes (before the reviewer's, which are docs
  and the wasm32 skip list); `web-core-wasm32` built for the skip list.
  The rustdoc,
  public and private, builds with warnings denied (a link to the private
  `CONNECTION_WORK` in `decide_within`'s docs was replaced by the
  number). `breaks 0.8.0`: none of this change's items in the
  public API's diff; the new constants are private.
- The measurement ran on the build before the panel's fixes. They can
  only turn a first attempt's refused read-back into a verdict, end a
  stopped call sooner, and route `--translation`, which the run does not
  use; so the table stands as a lower bound of the head's verdicts.

## Session fixup-36: a JSON document's atom keys held to the memory bound

Opus 5.5 at `high`, in `../deduxit-wt-step-82-141614` on `lozrkyxm`
(87e5, stacked on 87d2), cores 10 to 12, in the conversation of 87e5.
No panel (no check accepts or refuses anything else: the change is a
refusal at read time). Nothing pushed.

### What the defect is (the brief's premise did not hold)

87c5's integers member found `check --term-limit none --timeout none`
of a first-order document with a deeply shared term taking time and
memory doubling per level "after the work refusal". Reproduced with its
generator (`|- p(t)`, `t = g(t', t')` to depth n, an ordinary outcome
with a one-element structure): with `--work-limit 1000`, 0.19 s, 0.75 s,
2.9 s at depths 22, 24, 26. Callgrind at depth 18: 97 % of the time in
`FirstOrder::atom_key` called from `Tables::read`, the JSON reader. The
time is in the read, before the check, not after the refusal: the
reader writes every atom's key as its full text, and `limits.terms` was
its one bound. Under the defaults it was bounded (the term limit
refuses depth 30 at once, and with it lifted the 2 s time limit ends
the read); with both lifted nothing was, not even the memory limit. So
keeping the refusal's text while cutting the time, as the brief asked,
was impossible; the planning session chose (A): hold the keys to
`limits.memory_bytes` where `limits.terms` is none.

### What landed

`nylxknny` "Hold a JSON document's atom keys to the memory bound, and
read the command's JSON sequents within its flags":
- `Tables::read` (`core/src/serialize/sequents.rs`): after the term
  bound, the keys' bytes as written (`FirstOrder::unfolded_bytes` and
  `applied_bytes`, `first_order.rs`: each name once per term it heads,
  brackets and separators; the largest term's text beside the atoms')
  are held to `limits.memory_bytes`, `Refusal::Memory` in `Phase::Read`
  before any key is written. Applied whatever the term bound (departs
  from the plan, which said only where `limits.terms` is none): the
  reviewer showed that a count of terms is no bound on bytes, a 21 KB
  file whose function symbol has a 10 000-character name passing the
  default term limit and taking 3.9 GB. At the defaults a document is
  refused only where its keys alone would pass 1 GiB.
- The command (`cli/src/io.rs`): `prove`, `seq` and `interact` read a
  JSON sequent within the settings' `limits.terms` and
  `limits.memory_bytes` (`Most::memory_bytes`); before, a linear JSON
  sequent was read within the library's defaults, ignoring
  `--term-limit` and `--memory-limit`, and an intuitionistic one the
  memory flag (the reviewer's first finding; departs from the plan, which
  did not name it, but without it the new refusal could not be raised
  and `--term-limit none` did nothing there). A reader's memory refusal
  names `--memory-limit` (`io::refused`, `check`'s `limit_advice`).
- A test, `a_deep_shared_term_s_keys_are_held_to_memory`
  (`core/tests/serialize.rs`): depth 40 without a term bound and depth 12
  with a 1 000-byte name under the default term bound are refused under
  1 MiB; depths 4 read.
- The changelog (`[Unreleased]`, Fixed, two lines), `core-sequents.md`
  (the keys bounded by the term limit and by memory, a stale line on the
  reader's count corrected), `cli.md` (the JSON sequent's bounds), the
  manual's `--term-limit` paragraph (`docs/src/options.md`).

### Checks

- The command at the head, release build: depth 30 and 40 under
  `--term-limit none --timeout none` refused at once (exit 2, naming
  `--memory-limit`); depth 22 and 26 read and answer as before; the
  reviewer's witnesses: `prove --file lin22.json --term-limit none
  --memory-limit 1MiB`, `prove -i` of the ILL form and `seq fragment`
  refused naming the flag; `prove --file ln26.json --term-limit none`
  no longer refused at the term limit (refused at the memory bound);
  `prove -i --file ill30.json --term-limit none --memory-limit none`
  ends at the time limit; `check lo17.json` (the long name) refused
  under 1 MiB and at the defaults.
- The core's `serialize`, `first_order` and `stability` tests; every
  CLI test (the behaviour lock, README's and the manual's examples);
  clippy on both crates. `gate` passed at the head (the ratchet's
  `read-json` 12 279 707 instructions against 12 279 710, every journey
  within its tolerance).

### The reviewer

fresh-reviewer (Opus 5.5, high), on the first version (the bound only
under `limits.terms` none, counting terms): the refusal is sound (every
unfolded term writes at least a byte), every reader goes through
`Tables::read`, no other path writes keys from a document unbounded,
the defaults unchanged, every command that reaches `limit_advice` has
`--memory-limit`. Two findings, both fixed above: the command's
`prove`, `seq` and `interact` never passed `--term-limit` or
`--memory-limit` to the JSON readers (so the manual's new sentence was
false there, and an intuitionistic file could be refused by a bound no
flag raised); and the count of terms is no bound on the keys' bytes
(long names). The reviewer runs once; the fixes were checked by the
runs above, not reviewed again.

### Later candidate

Keying atoms by structure (the symbol and the argument list's id, which
the interner hash-conses) would make the keys' size the document's and
need neither bound. Every caller keys atoms by text (`atom_key` has
nine), so it is a redesign, not a fixup.
