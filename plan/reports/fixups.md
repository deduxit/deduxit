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
