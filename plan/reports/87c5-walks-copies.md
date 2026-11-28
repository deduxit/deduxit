# Step 87c5: walks, copies, and the repeats of ordinary and modal logic (report)

Run on 2026-11-27 and 28 by one session on Opus 5.5 at `high`, in the
workspace `../deduxit-wt-fixup-30-3e23c4`, from `plan/87-refactor.md`
(87c5, wave C, themes C5 and C10) and the planning session's briefs
`start-87-common.md` and `start-87c5.md`, on cores 13 to 15 (the
connection pin on 2 and 3 at the planning session's word). The base is
`main@origin` (87d4 on it). A refactoring: no output, refusal or
statistic changes, the public API is kept (everything new is
`pub(super)` or `pub(crate)`), so no changelog line.

## Outcome, per finding

| finding | commit | written once now |
|---|---|---|
| F-4d-15 | `nmxzsxws` | the modal search's masked bits (`ones(word, bits)`, by value, so the loops may change the branch) and `has_own_modal` |
| F-4b-11 | `ywpozuzs` | the normal forms' `Negate` loop (`negated_right`) and their fresh prefix (`fresh_prefix(…, "t")` in place of `prefix`) |
| F-4b-10 | `zzlmyvou` | the classical encoding handed to the solver (`solve`), for `decide` and `falsify` |
| F-4b-16 | `ywxxnwot` | LJT's answer for the SAT method (`ljt_answer`); the test-only refusal built in its branch |
| F-4a-18 | `oyompypp` | an option read by its name (`name::named`) at nine sites of ordinary logic, modal logic and lambda-terms; a test that every `NAMES` is its `ALL`'s names |
| F-4b-06 | `xqsztuvt` | Plaisted and Greenbaum's polarities (`ordinary::polarity`), for the classical encoder and the normal form; the intuitionistic pass keeps its own, as the verdict says |
| F-4a-08, F-4d-07, F-4c-16 | `pkpqtxnr` | the walk to the terms formulas reach (`Formulas::terms_in`, `walk.rs`), for nine readers: `mentions`, `largest_eigenvariable`, `metas_in`, Herbrand's universe and witnesses, resolution's door, both countermodel checks' up-front pass, `applied`, `reached` |
| F-4b-07, F-4e-10 | `rpnxukwp`, `tmlssuzv`, `sqtzxwsp` | `transfer::copy` for the goal state's propositional graft and the tableau review's expected sequent, `copy_first_order` for the normal forms' input; `transfer` loses its `interactive` gate |
| F-4a-11 | `kvyrwuln`, `zmrrpzvx` | one de Bruijn substitution walk (`instance.rs`'s `Substitution`, its `Replace` carrying the closed instances and `bruijn.rs`'s shifts, open substitution, table and abstraction) |
| F-5a-03 | `rlklzpur` | a lambda node's variables and a type's operands (`Node::variables`, `map_variables`, `Type::operands`, `map_operands`, crate-private) |
| F-4d-02 | `ozwsqlms` | one world's reading of terms and atoms, `=` as identity, and the meter (`semantics.rs`), for the finite and the Kripke check |
| F-4d-03 | `syrvxlzp` | one writer core of the finite and the Kripke model searches (`encode.rs`: `Core`, `over_tuples`, `Denotes`, `lookup`, which grounding shares) |

## Departures

- **F-4b-07: a law's claim keeps its own copy** (departs from the plan).
  `transfer::copy` builds in index order; `normal::copy_formulas` walks
  depth first from the two roots, and a claim's document numbers its
  formulas in the order they were made, so the copy renumbered every law
  claim in `seq normalize`'s JSON and, for a first-order result whose
  step's formulas hold no term, made the claim's arena propositional and
  so decided by another method (the argument member's witness). The
  input's copy (`normal::transfer`) and the two propositional ones do
  use `transfer::copy`.
- **F-4a-07: nothing changes**: the plan keeps the table of LK's and
  LJ's rules crate-private for part 1, so the command keeps its copy.
- **F-4b-16**: the three evidences LJT never gives map to one
  `ReadBack` defect in both functions, where one gave `Ok` and the other
  two other errors (no new panic, the verdict's choice for the author).
- **F-4d-02**: the two formula walks and the two up-front passes stay
  per check; running the finite check as a one-world Kripke model would
  move its work counts at the bound (ILTP's NLP problems meet 2²⁶).
- **F-4d-03**: the Kripke search keeps its own `lookup` (by de Bruijn
  index, the finite one by name); `holds`, `values` and the memoised
  walks stay per search, as the verdict narrows it.
- **F-5a-03**: `operands`/`renumber` over a node's subterms, and
  `check::written` and `reduce::binders`, stay: other sets, as the
  verdict says.
- F-1c-07 (the linear cut elimination's walks) is 87c6's, by the brief.

## The panel (F-4d-02, and with it the other checkers' paths)

The three members on `side-session-notes/…/87c5-panel-brief.md` (five
claims: the countermodels' reading, the de Bruijn walk, the writer core,
the term walk, the copies):

- **The argument** (Opus 5.5, `high`): claims 1 to 4 hold, line by line;
  claim 5 failed at `law_claim`'s copy (the departure above), fixed in
  `tmlssuzv`. A second point: `normal::transfer`'s refusal of an
  eigenvariable came after the copy, where a symbol clash of a
  hand-built `NormalForm` would have answered first; the check now comes
  before anything is copied.
- **The counterexamples** (Opus 5.5, `high`): no refutation over about
  47 000 inputs compared byte for byte with `main@origin` (first-order
  `prove` by models, grounding and connection under size, world, work
  and memory bounds; `check` of 10 237 finite structures and Kripke
  models, mutated and random, against a Python checker of its own with
  no contradiction; bound sweeps; normal forms, translations,
  interpolants, Herbrand, resolution, tables, tableaux and their
  reviews, modal logic in 15 systems, lambda-terms and System F,
  `interact` sessions with open witnesses, derivations' checks). The one
  difference, the law claims, is equal at `tmlssuzv`.
- **The integers** (Sonnet 5.5, `high`): no difference at any limit: the
  meter's work counts at their exact thresholds and its polls, both
  checks' memory refusals at the byte, table sizes and indices at
  `checked_pow`'s edges, the writer core under occurrence, memory and
  work sweeps (21 sweeps), the merged walk through prenex, Skolem,
  Herbrand, clausal forms and interpolants over nests of up to 200
  binders, `ones` over 70 atoms in eight systems. Found, outside this
  change (equal at `main@origin`): `check --term-limit none` of a term
  shared as a deep DAG takes time and memory doubling per level after
  the work refusal (n = 30: 29 s).

## The reviewer

`fresh-reviewer` on Opus at `high` (Fable's share past half, the planning
session's rule), once, at `tmlssuzv`: four findings.
1. `Copied`'s fields `metas` and `names` dead without `interactive`:
   not so (`cargo check --no-default-features` and with `sat` warn
   nothing; the fields are read in the copy's own body).
2. `core-ordinary.md` named `Evaluator::term`, gone: fixed (`sqtzxwsp`).
3. `copy_first_order` entered first-order before the terms where main
   did at the first atom, so a hand-built `NormalForm` with a
   propositional result and a first-order input numbered its claim's
   symbols otherwise: fixed, it enters at the first atom (`sqtzxwsp`).
4. A doc comment told history (`terms_in`): reworded (`sqtzxwsp`).

## The net

- Per commit: `cargo clippy -p deduxit --all-features --all-targets`,
  `cargo check` without default features and with `sat`, `interactive`,
  `parse`, `serialize`, `rocq` or `latex,png` alone, and the tests of
  the area touched; the library's 761 unit tests and the first-order,
  ILTP, lock and depth tests after the de Bruijn walk.
- The model searches against `main@origin`: 120 runs (20 first-order
  sequents, three logics, models and grounding, `--model-size 3
  --model-worlds 2`), JSON with statistics: equal. Under a 20 s time
  limit 10 differed, every one stopped on both sides at another size.
- The normal forms against `main@origin`: all ten forms on ten sequents
  as JSON, equal at `tmlssuzv` (55 had differed at `syrvxlzp`).
- `gate` at `tmlssuzv`: clippy, the tests and both `cargo hack` runs
  pass; the ratchet failed on `ordinary-resolution-first-order`, 3.27 %
  over its ceiling (the base itself +0.76 %). Herbrand's deskolemization
  built the merged walk's `Substitution` and moved its two memos in and
  out once per formula of every inference; `Replaced` now holds the one
  walk, made once per derivation (`zmrrpzvx`): 5 944 021 instructions,
  +0.23 %, below the base. `ordinary-pigeons` -0.27 %, `read-tptp`
  +1.37 % (the base's own drift; no walk of the reader changed).
- The connection pin (`bench/connection-targets.sh 87c5`, cores 2 and 3
  at the planning session's word) against `connection-87a.csv`: no
  verdict differs, and of the 1 312 problems both decide on one thread
  none differs in `nodes`, `work`, `unifications` or `bindings`.
- `breaks 0.8.0`: the earlier steps' items only; nothing of this step's.
- `nix flake check`: the first run failed `doc-private` on a link in
  `encode.rs`'s module doc (`[`Core`]`, unresolved there), fixed in
  `yqumnktz`; the second, at `yqumnktz`, passed every check.

## The rebase onto the stack top

The planning session stacked the chain on `lkylvptn` (main with 87c6,
fixup-35 and 87e1; 87c7 on main), and three commits conflicted with
87c7's forms, each resolved in place: `ywxxnwot` (`ljt_answer` with
`Refusal::stopped`), `xqsztuvt` (the polarity import beside
`crate::memory::Account`) and `ozwsqlms` (the checks' `tick` through the
shared `Meter`, which now builds its stop's refusal with
`Refusal::stopped`). The gate passes at the new head; no resolution
touched a walk the connection provers use, so the pin was not run again.
