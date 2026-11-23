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
