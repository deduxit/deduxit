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

## Unit 2a (the front door of the linear search)

- The consumer test's point 5, corrected by unit 1b: `Proof`'s docs say
  that `check` is the boundary; `Verdict` (`linear/search/mod.rs:3411`)
  and `Disproof::new` do not say what a value in them promises.
- `linear::search::Reason` carries reasons that are ordinary logic's
  alone (`PathLength`, `ModelSize`, `KripkeSize`) and is the reason of
  `lambda::Inhabitation::Unknown` (unit 6a's remark; F-arch-01).
- The library's default of `linear::Mode` is `CLASSICAL` (unit 1a's
  remark): D1 removes the command's default logic; say what the
  library's defaults should be.

## Unit 4f (the step-by-step state of linear logic)

- `Interactive::formula` is the third copy of one body, with
  `Proof::formula` and `Derivation::formula` (F-1b-07).

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

- The Rocq certificates from the exporter's side: a certificate's term
  restates each tensor node's subformulas in full, so its text grows
  quadratically with a formula's depth (F-9-18, a candidate); and the
  names the exporter writes as strings (`soundness_mix`, `weight_of`,
  `TermsCheck.terms_eqb`, the record fields) are a contract with the
  Rocq library that no file states (F-9-01, rejected as a finding of
  the library: its tests use every name; the exporter's side is yours).

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
