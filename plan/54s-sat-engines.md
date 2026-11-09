# Step 54s: SAT-based engines for intuitionistic, minimal and classical logic

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`). CLAUDE.md applies throughout:
- jj only, never git;
- thematic commits as soon as a unit is done;
- doc comments on every item, and every text a person reads follows
  `.claude/rules/writing.md`;
- the verification table;
- never push (deduxit-orchestrator pushes after your messages).

Written on 2026-11-09 by the planning session. The author asked whether
the engines follow the most efficient algorithms known, then accepted
the planning session's recommendations ("Go with what you recommend"),
and asked for the best practices and the availability of SAT solvers in
Rust to be researched first: `plan/notes/research/sat-rust.md`.

## Why

Step 54 made LJT (Dyckhoff's G4ip) the default for intuitionistic and
minimal logic: per logic it decides 200 of ILTP's 274 problems at 2 s,
the default dispatch 202, the embedding 188 (`plan/reports/54-ljt-engine.md`).
The fastest published provers for intuitionistic propositional logic
are SAT-based:
- Claessen and Rosén, *SAT modulo intuitionistic implications* (LPAR
  2015, LNCS 9450, 622-637), the prover `intuit`, the best on about
  1 200 standard benchmarks (ILTP among them) in 2015;
- Fiorentini and Ferrari, *Efficient SAT-based Proof Search in
  Intuitionistic Propositional Logic* (CADE 2021), `intuitR`: intuit
  with restarts, faster than intuit, fCube and intHistGC on the same
  benchmarks, with a Kripke countermodel for every formula that is not
  valid; a 2026 comparison on ILTP 1.1.2 still finds it ahead
  (arXiv 2605.01845);
- Fiorentini, Goré and Graham-Lengrand, *A proof-theoretic perspective
  on SMT-solving for intuitionistic propositional logic* (Tableaux
  2019), which reads intuit's search as a sequent calculus related to
  G4ip, the route from the search to a derivation.

Classical logic goes through a translation into affine MALL and decides
156 of the 274 at 2 s (`plan/reports/25-ordinary-logic.md`), where a
CDCL solver decides problems of this size at once. Step 46 had kept
classical logic out of reach of a SAT solver ("not a SAT solver",
`plan/later.md`); the author's answer of 2026-11-09 lifts that for an
engine whose every answer is checked: a falsifying assignment for "not
valid", a resolution proof (LRAT) checked by deduxit's own checker for
"valid". deduxit still does not offer itself as a general SAT solver:
no DIMACS service, the engine decides ordinary sequents.

Step 46's research kept this candidate as "engines-1: a SAT-based
intuitionistic engine, LJT first" (`plan/later.md`,
`plan/notes/research/next/scope-engines.md`). LJT is now done, so this
is its turn. It also gives step 55 its countermodels from the search
itself.

## The sessions

Each on Opus 5.5 at `high`, with its own brief and goal from the
planning session, its own jj workspace, and the panel of step 26's
prompt for the engine and its check (the argument on Fable 5.1 at
`high`).

1. **54s-a, the engine.**
   - Read the three papers above first, and intuitR's sources only if
     their licence allows reading them for an EUPL-1.2 project; write
     the engine from the papers, never by copying code.
   - The SAT solver is a dependency (`new-tool` skill): pure Rust,
     incremental (assumptions and clause addition between calls), a
     licence `deny.toml` allows (MPL-2.0 is not: `splr` is out), built
     for `wasm32-unknown-unknown` without threads or a clock, and
     bounded by deduxit's stop, work and memory. Step 46's research
     names RustSAT with Batsat (MIT) as the candidate; confirm it at
     the version the lock would pin.
   - The algorithm: intuit's loop over a SAT solver with implication
     clauses, with intuitR's restarts. Minimal logic is intuitionistic
     logic with `⊥` read as an atom.
   - **"Valid" comes with an LJ derivation that the checker validates**,
     read off the search by the Tableaux 2019 correspondence, or, where
     that is not reached, by LJT on the same sequent within the limits.
     A verdict whose derivation passes the derivation limit is
     reported as LJT's is today (`plan/reports/54-ljt-engine.md`).
   - **"Not valid" comes with a finite Kripke model** (worlds, the
     preorder, the monotone valuation, the refuting world), checked by
     computing forcing within a bound declared at its type; step 55
     makes it an output kind, certifies and draws it. Here it is data
     and a test.
   - It is a method of `ordinary` beside `embedding` and `ljt`
     (`ordinary::Method`), with its statistics.
2. **54s-b, classical logic by CDCL.**
   - The same SAT dependency if it gives proofs, else the one the
     research note names for proofs; a classical sequent becomes the
     clauses of its negation (Tseitin), decided by CDCL.
   - **"Not valid"** is the falsifying assignment, the one-world case
     of step 55's countermodel, checked by evaluation as today.
   - **"Valid"** comes with an LRAT proof checked by a small checker
     of deduxit's own (`linear::search::refutation`'s style: integers
     that cannot wrap, said at their declarations; the panel reviews
     it), and with the LK derivation where the existing read-back
     reaches it within the limits. Step 70 later reuses this checker
     for outside solvers' proofs.
   - A method of `ordinary` beside the embedding for classical logic.
3. **54s-c, the measurement and the default** (D19).
   - ILTP's 274 problems at 2 s, in the three logics, as a timed run on
     cores 2 and 3 under the machine lock: the new engines, LJT, the
     embedding and the dispatch, in one build.
   - The default moves only where the new engine wins, and no verdict
     may differ from 54a's oracle; whatever it loses, the dispatch
     keeps (as 54b kept SYJ201+1.004 and .005).
   - The command's `--method`, the manual, the guide, the changelog,
     `core-ordinary.md`, the bench rules.
   - **intuitR and intuit beside deduxit** (the author, 2026-11-09:
     "IntuitR would also be nice to have"): both join the comparison's
     tools as the other provers did (`modules/_provers/`,
     `modules/comparison.nix`, `bench/TOOLS.md`), built from their
     sources at a pinned commit and run as programs, never linked; then
     ILTP's propositional problems run under BenchExec for deduxit's
     default, intuitR and intuit, as the comparison does, and the
     manual gains the result. That is the measure of how close the new
     engine comes to the tool whose algorithm it follows.
   - The target sets equal 47's (nothing linear changes).

## The order

- After 54b (done). It may run beside 54u and 54w's sessions; it
  touches `core/src/ordinary/`, which 54w-e4 rewrites after step 55's
  first session, and 55's first session reshapes `ordinary`'s calls: it
  runs after 55's first session or before it, never beside it.
- 54s-b may run beside 54s-a once the dependency is in (54s-a adds it).
- Step 55's countermodel session waits for 54s-c: the model comes from
  the default's search.

## Rules for every session

- **The checker stays independent**: no engine's word is trusted; every
  "valid" is a checked derivation, every countermodel checked.
- **The panel** for the engine and the model's check (`conduct.md`).
- **Never the `EnterWorktree` tool:** work in your jj workspace through
  absolute paths.

## Deliverables

The commits, the oracles' rows (`bench/defaults/`), and
`plan/reports/54s-sat-engines.md`: the algorithms as built, the
dependency and why, the measurement per problem class and logic, the
defaults' reasons, what steps 55 and 70 take over.
