# Step 82: a first-order goal closed by proof search, or refuted

Opus 5.5 at `high`, 2026-11-27, in the deduxit workspace
`deduxit-wt-step-82-141614` on `main@origin` (`tttkyssp`, rebased onto
fixup-30's `vprlyonw`) and the client workspace `step82`
(`/home/tux/Projects/own/deduxit.github.io-82`) on the client's `main`
(`osruxkzq`), cores 4 to 6. Nothing pushed by this session.

deduxit's commits (on `main` since the planning session's push,
`e377ca9d`): `vswlmrws` (the library, the command's session, the
manual, the guide, the changelog), `qtxpnvlx` (the bindings) and
`omvzqutm` (the graft held to the bound on terms, the review's finding).
The client's: `xynkzqoo` (the page) and `zlsovzxz` (the input moved).

## Outcome

### deduxit

- **`close` and `hint` decide a first-order goal.** The ordinary goal
  state (`ordinary::Interactive`) searches a first-order goal for a
  proof as `prove --logic` does (grounding or the connection prover, the
  embedding), grafts the derivation found as one step, and checks every
  inference of it in the session's arena before the state changes.
  Where no proof is found it searches for a finite countermodel (a
  structure, or a Kripke model with growing domains in intuitionistic
  and minimal logic), which refutes the goal; otherwise the goal is
  unknown with the reason that stopped the search. `hint` reads the
  first steps off the same derivation. The same holds in the command's
  session (`interact --logic`, `close`) and in the bindings
  (`ordinary_close`, `ordinary_hint`), whose countermodel-only branch is
  gone.
- **An eigenvariable of the goal is a fresh constant** for the search:
  the goal is copied into an arena of its own
  (`core/src/ordinary/transfer.rs`), each eigenvariable a constant named
  as the session prints it, so a structure's constant has the goal
  text's name.
- **Nothing grafted unchecked.** The derivation is copied back, each
  constant to its eigenvariable and each of the derivation's own
  eigenvariables numbered past every one the proof holds, and every
  inference passes the checker's `check_inference` before `record`
  takes the step. Grounding's evidence for a valid sequent is a clausal
  proof, no derivation of LK: its translation is Herbrand's theorem's
  derivation (`ordinary::herbrand`), built from ground instances and
  checked, which the goal state asks for where the outcome has no
  derivation (`with_derivation`, its own test).
- **A goal holding an open witness is never refuted**: a countermodel
  of the witness as a constant shows nothing of the terms an axiom may
  still bind it to, so the outcome is unknown (`incomplete`).

### The client

- A first-order ordinary goal closed by the search is closed on the
  page: "Goal N closed by the connection prover", the proof finished
  and checked; 79c's departure ("refuted but never proved") is gone.
- `closeOrdinary` takes a goal as closed only where the view no longer
  lists it: a goal deduxit finds valid without a derivation of the goal
  alone (grounding's clausal proof, or `=` without its axioms) stays
  open with "Valid, but not added to the derivation".
- The engines' list names the connection prover and the grounding search
  beside the finite model search; a countermodel grounding found is now
  named "Grounding search" (before, every first-order refutation went to
  the model search alone).
- A hinted `∃R` or `∀L` says its witness: "Press ∃x. p(x) and take ∃R
  with the witness c." (`hintSteps` keeps it).
- The "unknown" sentence of the reason `incomplete` says that the proof
  search may miss a proof and that a goal holding an open witness is
  never refuted.

## Decisions

- **What is idiomatic, from current documentation.** Isabelle's `try`
  "attempts to prove or disprove a subgoal using a combination of
  provers and disprovers" (the Isar reference manual, chapter "Proof
  tools", isabelle.in.tum.de/doc/isar-ref.pdf, read 2026-11-27):
  Sledgehammer finds a proof with external provers and reconstructs it
  through the kernel (sledgehammer.pdf, its proofs "preplayed"), and
  Nitpick, a finite-model finder, runs on the same goal (nitpick.pdf).
  Lean's Aesop elaborates the proof it finds into a term the kernel
  checks; Plausible, its counterexample search, is a tactic of its own
  (both READMEs on GitHub). Rocq's `firstorder` tries no countermodel
  (refman V9.0). I took Isabelle's shape: proof first, its proof
  rechecked by the trusted checker, then a finite countermodel, then
  unknown.
- **The goal in an arena of its own**, not over a clone of the
  session's: the session's arena holds every eigenvariable and open
  witness ever made, on every branch, and the model search has an
  `unreachable!` on an eigenvariable among a sequent's terms; a fresh
  arena of the goal's formulas alone keeps them out, and no eigenvariable
  of a sibling branch can become a witness of the derivation (the
  session refuses those as witnesses).
- **The library's default dispatch, not a list of the goal state's
  own.** `decide_within` with no method is already "prove, then
  refute" at first order (grounding or the connection prover, the
  embedding, then the model search); the goal state passes the
  translation the caller gives, as before.
- **Herbrand for grounding's clausal proof.** A translation of a DRUP
  proof into LK would be a new module; Herbrand's derivation is built
  from ground instances of the same clauses, already checked, and
  complete for grounding's classes. Where it builds none within the
  bounds, the verdict stays valid and nothing is grafted
  (`Closed::grafted` is `Ok`, "there was none"), as the docs of
  `Closed` say.
- **A hint's witness only where the session holds the term.** A
  witness the search made up is no term of the proof; a term built new
  by the derivation has no id in the session (the hint works on a
  clone), so it is left open. `Interactive::term_text` (new, public)
  writes a witness as text `witness` reads, which the bindings send as
  the step's `witness`.

## Departures

- **`ordinary::Interactive::term_text` is a new public method** (an API
  addition the step needs: the bindings had no way to write a hint's
  witness as text). (departs from the design: an API change not asked
  of the author first; it is additive.)

## Checks (deduxit)

On the chain before the rebase onto fixup-30 (`main@origin` then
`tttkyssp`), cores 4 to 6, capped:
- `cargo clippy --workspace --all-targets -- --deny warnings`: clean.
- `cargo test --workspace`: every one of the 39 test binaries passed.
- `cargo hack check --feature-powerset --depth 2 --tests -p deduxit`: no
  warning (the new tests sit behind `sat` and `serialize`, the module
  behind `interactive`).
- `gate` (both `cargo hack` runs, clippy without `parallel`, the tests,
  the ratchet): passed; every journey under its ceiling (the largest
  rise `search-spec-chain` +1.61 %, `check-qbf-20-2` +1.53 %; no journey
  goes through the goal state's first-order path, so these are noise).

On the rebased head (`qtxpnvlx` on `vprlyonw`):
- `cargo test -p deduxit-cli --test readme` (README, the manual, the
  guide): passed.
- `nix build .#checks.x86_64-linux.web-wasm-tests` (the bindings' tests
  under Node): passed.
- `nix build .#checks.x86_64-linux.budget`: passed; the bindings'
  module is 2 894 918 bytes at `gzip -9`, 5 082 under the ceiling of
  2 900 000 (2 885 650 after step 81).
- `breaks 0.8.0`: no break of this step's; the one addition is
  `ordinary::Interactive::term_text`. The removals it lists
  (`StepError::Witness::reason` and the others) are step 81's, in the
  changelog's breaking list already.
- The focused and net engines' target sets were not run: no search
  changed.

## The measurement

The client stores no first-order problem of ordinary logic; its
first-order problems are the linear suite's (ten, two of them the same
in ordinary logic once `!` goes). I measured `ordinary_close` (the
bindings' call, natively, release build, the tab's settings, best of
five) on their ordinary readings (`⊗` and `&` as `∧`, `⅋` and `⊕` as
`∨`, `⊸` as `→`) and the drinker's sequent, in classical and
intuitionistic logic, 20 closes, by a scratch test not committed:
- median 0.06 ms, slowest 1 545.5 ms;
- every classical close under 0.07 ms (grounding, the connection
  prover, or the model search);
- the slowest is intuitionistic `∀x. ∃y. r(x, y) ⊢ ∃y. ∀x. r(x, y)`:
  the embedding searches the image to its copy bound of 3, the image
  unprovable, before the model search refutes it with a Kripke model;
  the other nineteen take at most 0.14 ms.

## Checks (client)

- Before deduxit was pushed, against the local head through
  `--override-input deduxit git+file://…`: Vitest (737 passed, the new
  `closes a first-order goal by the search, and keeps a hint's witness`
  among them), `lint` (Biome and `astro check`, clean), and Playwright on
  `countermodel`, `ordinary` and `first-order` (all passed; the new
  `a first-order goal is closed by the search, its eigenvariable a
  constant there` and `a first-order goal's hint carries the witness of
  its ∃R`).
- After "82: deduxit pushed": the input moved to `e377ca9d`
  (`zlsovzxz`, "flake.lock: Update"), and the client's full `nix flake
  check` passed at that head: Vitest 737 of 737, Playwright 366 passed,
  none flaky.

## The review

`fresh-reviewer` (Fable 5.2, medium) read the three commits (deduxit
`vswlmrws`, `qtxpnvlx`; client `xynkzqoo`) against the prompt and found
no confirmed break of its criteria; it ran `ordinary::interactive`.
- **A graft was not held to the bound on terms** (probable, with a
  witness): the search bounds the goal's own arena, which lacks the
  other branches' atoms, so a graft could make a session document its
  reader refuses. Confirmed by `a_graft_is_held_to_the_bound_on_terms`
  (it fails without the fix), fixed in `omvzqutm`: the graft is refused
  with `Refusal::Terms` in `Closed::grafted`, the goal left open.
- **Symbols a derivation brings in** (a question): Herbrand's derivation
  may carry its fresh constant `c1` or a Skolem symbol's name, chosen
  apart from the goal's arena, not the session's; where another branch
  has that name in another role, `close` answers `Error::Symbol` though
  a proof exists. An error, never a wrong verdict; left open below.
- **A propositional goal of a first-order session in the bindings** (a
  question): `with_countermodel` gives no SAT model in a first-order
  session; it could not before either, since the solver refuses the
  session's arena. The verdict is right; `web.md` now says so.

## Left open

- A derivation's own symbols (Herbrand's `c1`, a Skolem name) may clash
  with a symbol of another branch of the session, which makes `close`
  an error where a proof exists; renaming them apart from the session's
  symbols in `transfer::copy` would close it.
- A goal holding an open witness is closed only where the derivation
  takes no term holding the witness as a quantifier's witness: the
  session reader's replay refuses a closed `∀L`/`∃R` whose term holds an
  earlier step's witness, so such a proof is unknown. A replay that
  takes those would let `close` fill a witness by search.
- The goal state decides a first-order goal in the library's sequence
  (proof search, then the model search); on the intuitionistic
  `∀x. ∃y. r(x, y) ⊢ ∃y. ∀x. r(x, y)` the embedding's search to its copy
  bound takes 1.5 s before the model search answers. The command races
  the two; a race or a share in the library would shorten it.
- A propositional goal of a first-order session that is not valid gets
  no Kripke model in the bindings (the image's assignment only).
- The bindings' module is 5 KB under its budget.
