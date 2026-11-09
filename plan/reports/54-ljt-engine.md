# Step 54: intuitionistic logic decided natively, by Dyckhoff's LJT

The step's sessions each add a section here: 54a (the engine), then 54b
(the default by measurement).

## Session 54a: the engine

Opus 5.5 at `high`, in the jj workspace `../linlog-step54a`, from
`plan/54-ljt-engine.md`. Paused at 11:18 on 2026-11-09 by the
orchestrator for the author's stop at 11:30; this section is the state
at the pause and is completed when the session resumes.

### Outcome so far

- **LJ's checker has Gentzen's cut** (change `sxlozrrw`, "Give LJ's
  checker Gentzen's cut"): `ordinary::Rule::Cut`, checked in LJ only,
  written by the Rocq certificate as a `let` and read back as a lambda
  term by substitution; no step of the goal state takes it; a form that
  holds one is written at wire level 9. Verified before its commit: the
  workspace's tests with `--no-fail-fast` (after the doctest fix), the
  doctests, the guide's test, clippy with `--deny warnings`.
- **The engine** (`core/src/ordinary/ljt.rs`, public as
  `ordinary::ljt::decide`): G4ip's search, invertible rules first,
  hypotheses as a set, a memo of every verdict, a stack of its own; its
  proofs translated into LJ derivations with cuts and checked;
  `Error::Logic` for classical logic. Its four unit tests passed
  (`agrees_with_the_embedding`, `decides_the_separating_sequents`,
  `a_planted_wrong_translation_is_refused`,
  `the_search_keeps_to_its_bounds`).

### Decisions

- **Cut in the checker rather than G4ip's rules or a cut-free
  translation** (departs from the design, which named either a sourced
  translation into LJ or the checker gaining G4ip's rules). LJT's
  `∧→L`, `∨→L` and `→→L` are derivable in LJ only with a cut: a
  cut-free LJ derivation has the subformula property, and their premises
  hold formulas (`A → (B → C)`, `B → C`) that are no subformulas of the
  conclusion. Dyckhoff's correction (JSL 83(4), 2018, p. 1680) says the
  soundness half of G4ip's equivalence with G3ip "is straightforward,
  using admissibility of Cut in G3ip". Gentzen's LJ has cut as a rule,
  so the checker gains one rule whose soundness is evident instead of
  four, and the translation is the derivation of each LJT rule with a
  cut. Eliminating the cuts in the translation would be cut elimination
  on LJ, with its blow-up, for no gain in trust.
- **Accepted by deduxit-orchestrator at the pause** (11:15): the cut in
  LJ only is a sound departure; the panel is to read the cut rule's check
  (`check_inference`) and the translation's cuts (`Lj::step`). The ILTP
  oracle's timed slot is granted first on resuming.
- **The cut is LJ's only**: LK refuses it, since the embedding's
  read-backs have none and the classical certificate has no term for
  one.
- **The API is a module of its own**, `ordinary::ljt` with `decide`,
  `Outcome` and `Statistics`, beside `ordinary::decide`, whose outcome
  holds the image's linear search; 54b joins the two under the options
  and the command.

### Sources read

- R. Dyckhoff, "Contraction-free sequent calculi for intuitionistic
  logic: a correction", JSL 83(4), 2018, 1680–1682 (the St Andrews
  repository's copy): the rules of G4ip, the multiset calculus, the
  soundness by Cut.
- R. Dyckhoff, "Intuitionistic decision procedures since Gentzen"
  (2016, the author's manuscript): `¬A` as `A → ⊥`, no rule for
  `⊥ → B`, the invertibility of the left rules but `→→L`.
- R. Iemhoff, "The G4i analogue of a G3i sequent calculus",
  arXiv:2011.11847, §2: the weight function (atoms and `⊥` 1, `∧` adds
  2, `∨` and `→` 1) and the multiset ordering.
- Dyckhoff and Negri, JSL 65(4), 2000: its abstract only (contraction
  and cut admissible in G4ip), which the set of hypotheses rests on.
  The 1992 paper itself was not reachable.

### The checklist

| item | state | evidence |
|---|---|---|
| cut in LJ's checker, committed | done | change `sxlozrrw` |
| engine and translation, unit tests, committed | done | change `morqmskz`; the four tests passed, workspace tests and clippy clean before it |
| depth test (`core/tests/depth.rs`) | done, in `morqmskz` | passed on a 256 KiB stack (the derivation's size bound lifted there: 50 000-character formulas) |
| rules (`core.md`, `core-ordinary.md`) | done, in `morqmskz` | |
| ILTP agreement test (`core/tests/iltp.rs`, ignored by default) | written, not committed, not run: it needs the oracle | |
| ILTP oracle `core/tests/iltp/embedding.csv`: the embedding's verdicts at 2 s, 6 runs (intuitionistic and minimal, cbn, cbv, 01) | to do: a timed run, slot from deduxit-orchestrator; the script is in the session's scratchpad (`iltp-oracle.sh`), about 20 minutes on cores 2 and 3 | |
| the gate at the engine's commit | passed, exit status 0 at 11:14 (the second run: the first found `Derivation::from_parts` compiled only with `interactive` or `serialize`, which the engine needs always; fixed in `morqmskz`) | the gate at `morqmskz` with `core/tests/iltp.rs` in the working copy |
| the panel (argument on Fable 5.1 at high, counterexamples, integers) | to do | |
| `nix flake check`, target sets 47 | to do | |
| manual's status chapter, guide | to check: the command does not offer the engine yet (54b) | |
| rebase onto `main@origin`, final message | to do | |

### What the next sessions must know

- **The engine's outcome is its own type** (`ordinary::ljt::Outcome`,
  with `Statistics`: `sequents`, `memo_hits`, `memo_entries`), not
  `ordinary::Outcome`, which holds the image's linear search. 54b joins
  them under the options and the command (an engine choice in
  `ordinary::Options`, the outcome's JSON form, the harness's engine
  axis); neither `ljt::Outcome` nor `Statistics` has a JSON form yet.
- **`Error::Logic`** (code `logic`) refuses classical logic; its message
  names LJT, which a second native engine would generalise.
- **Derivations from the engine hold cuts**, so their JSON is wire level
  9, their lambda-terms may hold redexes, and their certificates hold
  `let`s; LK's derivations and the embedding's read-backs never do.
- **The work unit is a sequent expanded**, the stop is asked every
  `ljt::POLL` (256) expansions, the first included; the translation and
  the check count against `derivation_bytes`, `memory_bytes` and the
  stop as errors, not verdicts, as `ordinary::decide`'s read-back does.
- **The derivation can be much larger than the search**: proofs share
  steps between branches and the translation unfolds them into a tree,
  with contractions below every `→L` split and weakenings at every leaf.
  On formulas tens of thousands of characters wide the size estimate
  passes the default `derivation_bytes` (the depth test lifts it).
- **The oracle for the ILTP test** is to be `core/tests/iltp/embedding.csv`
  (`problem,logic,translation,verdict`, the problem relative to
  `Problems/`), made from the timed run's JSON lines; the test is
  `#[ignore]`d, run with `cargo test -p deduxit --test iltp -- --ignored`.

### How to resume

In `../linlog-step54a`: `jj st` (`@` holds the uncommitted
`core/tests/iltp.rs`; the engine is `morqmskz`, the cut `sxlozrrw`), then `cargo test -p deduxit --all-features --lib ljt` and
`cargo test -p deduxit --test depth` in a capped scope on cores 4–9;
then ask deduxit-orchestrator for the ILTP slot.
