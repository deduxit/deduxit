# Step 83: natural deduction proofs found, from sequent proofs

Opus 5.5 at `high`, 2026-11-27, in the deduxit workspace
`deduxit-wt-step-82-141614` on `main@origin` (`xlnuroym`) and the client
workspace `step82` (`/home/tux/Projects/own/deduxit.github.io-82`) on the
client's `main` (`zlsovzxz`), cores 4 to 6, in the conversation of step
82. Nothing pushed by this session.

## Outcome

### deduxit

- **Step 82's open item first** (`lpxzzsoy`): a grafted first-order
  derivation's own symbols (Herbrand's fresh `c1`, a Skolem name) are
  renamed apart from the session's names in `transfer::copy`
  (`Parameters::Back`'s `known`, `apart`), so a `c1` that another branch
  has as a predicate no longer makes `close` an `Error::Symbol` though a
  proof exists. `a_derivation_s_own_symbols_are_renamed_apart` fails
  without it (`Symbol { name: "c1", fault: Role }`).
- **Natural deduction from sequent proofs**:
  `rules::Derivation::natural(&derivation, &calculus, &limits, stop)`
  rewrites a checked derivation of LK, LJ or LM into a proof of any of
  the eight natural deduction rule sets deduxit ships (NJ, NK, NM,
  NK-DNE, NK-LEM and the three `-defined` ones whose `¬A` is `A → ⊥`),
  checked against the rule set, refusing a weaker logic than the
  source's and a first-order derivation (the rule files are
  propositional). The proof is normal: no elimination's major premise
  is the conclusion of the introduction of its connective;
  `translations_are_normal_and_checked` holds every file to it on
  eighteen sequents in three logics, and fails with the detour
  reduction turned off (`a ∨ b ⊢ b ∨ a` into NK).
- **The session's search**: `rules::Interactive::close` and `hint` in
  those rule sets decide a goal from the lines in force in the sequent
  calculus of the rule set's logic (through the embedding into linear
  logic, whose derivations have no cut), translate the derivation,
  check it, and take it step by step through `apply` and `assume`, each
  step checked by the session's own checker. `hint` gives the proof's
  first step with what it needs given (`rules::Hinted`), or `assume`
  where the goal is a line in force. A calculus that is no shipped
  natural deduction file is `not_applicable`.
- **The command**: `close [G]` in `interact --calculus` for those rule
  sets ("valid in intuitionistic logic: closed by 7 steps of NJ", "not
  valid in intuitionistic logic: the goal has no proof in NJ"). `prove`
  gets no natural deduction calculus: its outputs are the sequent
  calculi's, and the session is where a natural deduction proof is
  built.
- **The bindings**: `rules_close` and `rules_hint`, `describe`'s
  `close` and `hint` of the `rules` suite.

## Decisions

- **The translation, not a search of the rules suite's own**, as the
  prompt asks: every rule set deduxit ships is covered, the
  intuitionistic and minimal ones from LJ and LM, the classical ones
  from LK with the file's classical rule (`RAA`, `¬¬E` after `¬I`,
  `LEM` with `∨E`).
- **The test's converter made the library's**: `core/tests/natural.rs`
  already translated the ordinary suite's derivations into every file
  to cross-check the rule files; the library's translation follows it
  (open leaves named by formula, a left rule substituting an elimination
  from its principal hypothesis), rewritten as loops over explicit
  stacks and charged to the bounds. The test keeps its own converter as
  an independent second one.
- **Normality by construction plus one reduction**: substitution puts
  eliminations under eliminations; the two places that put an
  introduction in a leaf's place (an equivalence's implications on the
  left, LK's negated right formulas) reduce a resulting `→I`/`→E` or
  `¬I`/`¬E` detour at once, and `↔R` takes an `→I`'s premise directly.
- **Steps, not one graft**: the proof enters the rules session as its
  own steps, each checked by the session's checker as a visitor's would
  be, so the session's document, reader and undo stay as they are; the
  page groups a close's steps for Undo.
- **The embedding alone as the search**: the propositional default
  (the SAT solver, LJT) may give a derivation with cuts, which the
  translation substitutes but could leave a detour; the ordinary goal
  state chose the embedding for the same reason.
- **A hint on a line in force is `assume`**: the embedding's
  derivations are η-long and take a compound line apart first, and
  hints that follow such a proof went round for ever on `b → c` given
  `b → c` (`following_the_hints_proves_a_goal`).

## Departures

- None from the prompt's "Fixed now". The CLI's `prove` has no place
  for a natural deduction calculus (it decides sequents and draws
  sequent derivations), so the command's part is `close` in the
  session, as the prompt allows ("if it has a place for it").

### The client

- Natural deduction's `?` holds "Close goal" (the search, its proof
  taken step by step; a note "Closed by the search" with the steps it
  took; Undo takes a close back whole), "Hint" (its step in words with
  "Take it") and, on a goal with no proof, "Go back" (undoes steps
  nearest first to a goal the search proves), beside "Prove apart".
- The marking asks the search of each open goal within the page's
  marking budget and writes "no proof" on the goal and its button.
- The steps list takes `rules_candidates`: the legal steps as before,
  then every rule that does not apply, greyed, its reason shown on a
  press.

## The budget

The bindings' module grew by 29.3 KB at gzip with this step: 2 925 319
bytes on `main@origin` (`lvzsyywy`, with step 82's first-order close and
step 84's untyped lambda-terms), 2 954 598 with this chain, past the
ceiling of 2 950 000 that step 84 had set (its module at 2 915 974
without step 82). The translation and the session's search are new code
the bindings link through `rules_close` and `rules_hint`. `yntvykyq`,
rewritten over step 84's raise as the planning session asked, sets the
ceiling at the combined size and about 2 %, rounded up: 3 020 000. Every
visitor downloads the module, so the growth goes to the audit as a
finding.

## Checks (deduxit)

On the chain on `main@origin` (`orukyztp`, then rebased onto
`lvzsyywy` with step 84's library), cores 4 to 6, capped:
- `cargo clippy --workspace --all-targets -- --deny warnings`: clean.
- `cargo test --workspace`: every test binary passed (the new
  `rules::natural` and `rules::search` tests, the bindings'
  `natural_deduction_has_a_search`, the manual's and the guide's new
  examples through `cli/tests/readme.rs`).
- `cargo hack check --feature-powerset --depth 2 --tests -p deduxit`:
  clean after one fix (`Terms` was imported unused without
  `interactive`).
- `gate`: passed.
- `nix build .#checks.x86_64-linux.web-wasm-tests`: passed;
  `.#checks.x86_64-linux.budget`: passed at the new ceiling.
- `breaks 0.8.0`: this step adds `rules::Derivation::natural`,
  `rules::Interactive::close` and `hint`, and `rules::{Closed, Hint,
  Hinted}` (in `core.md`'s list of marked types); the removals listed
  are step 81's, in the changelog's breaking list already.
- No target set: no engine's search changed.

## Checks (client)

Against the local deduxit head through `--override-input`: `lint`
clean, Vitest 737 passed, Playwright's `natural.spec.ts` 9 passed (the
four new tests among them: close and Undo whole, the hint and Take it,
the greyed rule's reason, the marking and Go back; the greyed rule's
first run failed by the test, which pressed an `aria-disabled` button
without `force`, as the first-order spec does).

After "83: deduxit pushed" (`main@origin` `yntvykyq`, `45558e26`):
the client's input moved to it (`zmvvktrn`, "flake.lock: Update"), and
the client's full `nix flake check` passed at that head: Vitest 737 of
737, Playwright 370 passed, none flaky.

The commits: deduxit `lpxzzsoy` (82's renaming), `twpwkzvu` (the
library, the command, the docs; the review's fixes folded in),
`spmpxvok` (the bindings), `yntvykyq` (the budget), all on `main`; the
client's `tlrmzywr` (the page) and `zmvvktrn` (the input moved).

## The review

`fresh-reviewer` (Fable 5.2, medium) read the five commits (deduxit
`lpxzzsoy`, `twpwkzvu`, `spmpxvok`, `yntvykyq`; client `tlrmzywr`) by
reading alone, and found three breaks, all answered:
- **Following the hints never ended in the classical files** (Peirce's
  law in NK: on `⊥` the hint was `RAA` again). Confirmed by
  `following_classical_hints_proves_a_goal` (four classical files, three
  goals), which looped until the memory bound. Two fixes, folded into
  `twpwkzvu`: the translation concludes a goal `⊥` without the classical
  rule and, where `¬A` is in force, with the derivation of `A` that the
  proof refutes it with or `⊥E` (`concluded`); and **hints follow one
  proof**: the search starts at the session's root goal and its proof is
  followed down the steps taken while they are its own, starting again
  where the session left it, on one thread so that a node's proof is the
  same each time. The first fix alone still looped (each goal searched
  alone re-proved `P` from `¬P`), and so did listing the newest lines
  first; following one proof ends because that proof is finite.
- **Three or more formulas right of `⊢`** made the translation add a
  disjunct twice (`right[i.max(1)..]` for `right[i + 1..]`), an error
  where a proof exists. Fixed, with `|- a, b, ~b` in
  `translations_are_normal_and_checked`.
- **Go back could freeze the page**: during its search the view was
  not busy, and a step pressed then made its loop spin. Go back now
  holds the view busy throughout and undoes through the calls directly.
- Its two questions, both answered in the client: Go back takes a close
  by the search back whole, as Undo does, and `putNatural` clears the
  notes, hint and close records a lesson's proof would otherwise
  inherit.

## Left open

- A hint's outcome is the search's whose proof the step follows, which
  may be an ancestor goal's sequent; its verdict says whether the goal
  has a proof, which is what the page reads. A hint on a deep goal
  searches from the root, so it costs the root's search.
- `Derivation::natural` given a derivation of LJ with cuts (the default
  propositional methods may give one; the session never passes one)
  checks but may leave an `∧`, `∨` or `↔` detour.
- `core/tests/natural.rs` keeps its own converter beside the library's,
  as an independent second translation.
- The module's growth (29.3 KB at gzip) is the audit's to weigh.
