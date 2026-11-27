# What earlier units ask of later ones (step 86, phase B)

A finder that met something outside its unit said so in its remarks.
Each line below is such a pointer, by the unit it is for: not a finding,
a place to look. A finder reads the lines for its unit and for "every
unit", judges them as it judges its own reading, and reports what holds
as findings of its own, naming the finding it follows.

## Every unit

- `std::collections::HashMap` and `HashSet` (SipHash) beside the crate's
  `hash` module (foldhash): units 1a and 1b found 10 and 13 library
  sites (F-1a-12, F-1b-13), and no `clippy.toml` with `disallowed-types`
  keeps new ones out. List your unit's sites in one finding, and say
  which are on a hot path.
- Types reachable under two or three paths because a leaf module is
  `pub` and re-exported (F-1a-04, F-1b-08, F-6a-06): `surface.md` lists
  them. Say which path your unit's users should have.
- The linear suite holds what every suite uses (F-1a-03, F-1b-01,
  F-1b-04; `architecture.md`, section 3): the formula walk and the
  printers' table (`linear::sequents::{Stop, Stops, Walk}`,
  `notation::Notation`), the drawing layer (`linear::proofs::{Drawn,
  Labels, OpenGoal, TextOptions, Structural, InfId, GoalId}`),
  `linear::search::Reason`, the memory account. Where your unit names
  one of them, say what it needs of it, so that the shared module gets
  the right interface.

- The architecture reader's map of the crate is
  `target/audit/units/arch/architecture.md`, its findings `findings.md`
  beside it, and their verification `verdicts.md` (25 of 27 confirmed,
  2 candidates; the section "architecture.md" at its end corrects the
  map's wrong claims and says what recorded decisions stand against its
  proposals). Read the parts that name your unit's modules. Where your
  reading of the code agrees or disagrees with a finding `F-arch-NN`
  that touches your unit, say so in a finding of your own or in your
  remarks: you read all of the code, the architecture reader read its
  outlines.

## Unit 2a (the front door of the linear search)

- F-arch-01 (`Reason` belongs at the root), F-arch-04 (the engine
  interface `Decide` is crate-private and `Outcome` cannot be built
  outside; the proposal is `architecture.md`, section 5, part 3, and
  its verifier adds that `Outcome`'s fields are public, so `checked`
  and `verdict` stay writable, and that an outside engine must be
  refused as a forced engine), F-arch-19 (`search/mod.rs` holds five
  things): judge each from inside the file.

- The consumer test's point 5, corrected by unit 1b: `Proof`'s docs say
  that `check` is the boundary; `Verdict` (`linear/search/mod.rs:3411`)
  and `Disproof::new` do not say what a value in them promises.
- `linear::search::Reason` carries reasons that are ordinary logic's
  alone (`PathLength`, `ModelSize`, `KripkeSize`) and is the reason of
  `lambda::Inhabitation::Unknown` (unit 6a's remark; F-arch-01).
- The library's default of `linear::Mode` is `CLASSICAL` (unit 1a's
  remark): D1 removes the command's default logic; say what the
  library's defaults should be.

## Unit 4a (the core of ordinary logic)

- F-arch-02: four options that only ordinary logic's engines read
  (`path_length`, `restricted_backtracking`, `model_size`,
  `model_worlds`) are fields of `linear::search::Options`
  (`linear/search/mod.rs:3016`), so `ordinary::decide_within` takes two
  options values (`ordinary/decide.rs:751`).
- F-arch-13: what refutes a sequent is three optional fields beside the
  verdict in the ordinary outcome (`ordinary/decide.rs:72`), where the
  linear verdict carries its disproof in the variant.
- F-arch-08: the hash-consed arena of propositional formulas, its
  sequent, its parser and its JSON form exist twice, in `ordinary` and
  in `modal` (`ordinary/mod.rs:824`, `modal/mod.rs:170`).
- F-arch-20: `ordinary/mod.rs` is the decision's vocabulary and the
  formula arena in one file.
- The consumer test's point 7 is stale: `ordinary::Options` has
  `with_logic` (`ordinary/mod.rs:610`). Its point 6 stands:
  `Options::default()` is intuitionistic, which D1 questions.

## Unit 4c (the first-order engines of ordinary logic)

- F-arch-24: the intuitionistic connection prover repeats the classical
  one's search loop (thirteen duplicates of up to 115 lines,
  `connection/prove.rs:650` and `connection/intuitionistic/prove.rs:705`).
  Confirm or correct it, say what one generic search would be, and name
  what pins the two searches today (statistics, tests) so that a merge
  can be shown to keep them.

## Unit 4d (countermodels, certificates, modal logic)

- Six files of countermodels at the top of `ordinary/` with near names
  (`model.rs`, `models.rs`, `kripke_search.rs`, `kripke_structure.rs`,
  `structure.rs`, `rocq_models.rs`): would a directory hold them, and
  under which names?
- F-arch-21: the shared `kripke::Model` has its methods written in two
  suites (`ordinary/model.rs:175`, `modal/model.rs:176`), and the modal
  check is bounded by `ordinary::MODEL_CELLS` (`modal/model.rs:18`).
- F-arch-08, from the modal side: the arena, the parser and the JSON
  form that `modal` copies from `ordinary`.

## Units 4e and 4f (the step-by-step states)

- F-arch-12: the three types named `Interactive` have no interface in
  common, and their names disagree where they mean the same:
  `is_complete` (linear, ordinary) against `is_finished`
  (`rules/interactive.rs:430`); `goals()` an iterator against a `Vec`
  (`rules/interactive.rs:416`); `goal(id)` a `Result` against a slice
  that panics on a foreign id (`rules/interactive.rs:436`); `undo` an
  `Option<GoalId>` against `Option<Undone>`. List what your unit's
  session calls each of these, with its signature.
- `Interactive::formula` (linear) is the third copy of one body, with
  `Proof::formula` and `Derivation::formula` (F-1b-07).

## Unit 5a (lambda-terms)

- F-arch-05: a `lambda::Term` can be made only from text or JSON
  (`lambda/mod.rs:212`).

## Unit 5b (calculi given as rule files, the workspace)

- F-arch-09: `rules`, which belongs to no suite, holds 2 213 lines of
  ordinary logic (`rules/ordinary.rs`, `natural.rs`, `tableau.rs`,
  `search.rs`), and `rules` and `ordinary` name each other
  (`ordinary/written/mod.rs:32`).
- F-arch-16: the feature `serialize` also turns on `rules` and modal
  logic's decision procedure; `core-rules.md:33-37` records why the
  calculus stores its serde form.

## Unit 8 (the bindings and the protocol)

- F-arch-10 and F-arch-11: `protocol::Suite`'s docs say a suite of
  another crate may implement it while its data types have no
  constructor and the registry is a static; the registry is written
  four times in `protocol/mod.rs`, and `KINDS` names none of the lambda
  suite's five kinds.
- F-arch-12 and F-arch-15, against the client's calls: every session
  call is written three times in the bindings (`candidates`,
  `ordinary_candidates`, `rules_candidates`) and described in a second
  protocol there (`web/src/protocol.rs:151`); the linear suite's calls
  carry no prefix and the others' do.
- F-arch-18: the module's size. `plan/later.md` ("The web module's size
  after the workspace") holds an attribution of the module's bytes by
  twiggy from an earlier step, and `web.md` the budget's history; say
  what a registry built from the page's list of suites would need of
  the bindings.

## Units 6b, 5a, 5b (the exports, lambda-terms, calculi given as data)

- `Labels::Table` is keyed by the linear suite's `Named`
  (`linear/proofs/style.rs:53`), so the two-sided view, ordinary logic,
  lambda-terms and rule files ignore a user's own labels (F-1b-04):
  how does your unit label a rule?
- One spelling for the names of values (unit 7a's remark):
  `lambda::Calculus` and `Strategy` are written in snake case, `Engine`
  and `negative::Translation` in kebab case; the command mirrors them in
  18 enums of its own (F-7a-03).

## Unit 6b (the exports)

- F-arch-25: the LaTeX and the Typst writer are the same code over two
  spellings (six duplicates, `export/latex.rs:480` and
  `export/typst.rs:550`); F-arch-22: `latex`, `typst` and `svg` are
  three features with no dependency of their own (258 `cfg` sites);
  F-arch-26: the exports take a closed list of the crate's own
  derivations (`export/mod.rs:235`).
- The Rocq certificates from the exporter's side: a certificate's term
  restates each tensor node's subformulas in full, so its text grows
  quadratically with a formula's depth (F-9-18, a candidate: the mend
  is a `let` inside the term the exporter writes, in
  `export/rocq/deduxit.rs`); and the names the exporter writes as
  strings (`soundness_mix`, `weight_of`, `TermsCheck.terms_eqb`, the
  record fields) are a contract with the Rocq library that neither
  repository lists (F-9-01, confirmed: the library's tests use the
  names, so a rename fails there, but nothing marks them as the
  contract).

## Unit 7b (the command's other files)

- `cli/src/interact/ordinary.rs:85` holds the same hand-written list of
  mode flags as `prove.rs:748`, without `--cyclic` and `--lambek`
  (F-7a-02).
- `cli/src/hint.rs`: the hint after a parse error, "add --logic
  classical", sends a user of `seq print` to the linear image
  (F-7a-04).
- `cli/src/grade.rs` chooses its suite in a way of its own, which the
  selector of F-7a-01 would replace (D1).

## The leader (stage 6)

- A first-order sequent parsed from text and the same one read from
  JSON compare unequal under `Sequent`'s derived `Eq`, because the JSON
  reader numbers argument lists in another order (unit 1a's remark; an
  open follow-up in `core-sequents.md`).
- `cli/src/io.rs:411` reads a JSON sequent with unbounded limits, so
  `prove --file seq.json --term-limit 1` accepts a sequent the text
  reader refuses (unit 6a's remark, beside F-6a-01), and the command
  answers exit status 2 for every refusal at reading.
- D1 reaches further than the command: the library's `Settings` keeps
  `ordinary.logic` and `batch.mode`, `ordinary::Options::default()` is
  intuitionistic and `linear::Mode`'s default classical; the Rocq
  library's README line 66 shows a `deduxit prove` that relies on the
  default.
- `core-derivations.md:297` says `Named::ALL` is "a public array of
  34"; it has 41 (unit 1b's remark): a rules file to correct.
- `Sequent::add` and `add_cut` offset indices with an unchecked `+`
  (`linear/sequents/mod.rs:851`, `:863`, `:869`, `:294`): out of reach
  in practice (an arena of 2^32 terms), but no rules file records the
  bound (unit 1a's verifier, beside the findings).
