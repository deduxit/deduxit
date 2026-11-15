# The release of 0.4.0: notes, commands and checks

Written by session 59 (2026-11-15) for the planning session and the
author. The tags are `v0.4.0` in `deduxit/rocq-deduxit` first, then in
`deduxit/deduxit`, as for 0.3.0 (`plan/notes/release-0.3.0.md`). This
file says what each release's notes will say, what to check before and
after the tags, and what the author does.

## What 0.4.0 is

"Intuitionistic logic decided, with Kripke countermodels, tableaux and
truth tables" (D27): steps 54 to 58 with 54s, 54u and 54w, and on the
page 55d's countermodel and 56b's truth table and tableau. The crates'
changelog names the page's part in its section's first paragraph, so
the release holds until 56b is on the client's `main`.

## The GitHub release notes

Both release workflows write the notes themselves from the tagged tree,
so the drafts at the end of this file are the text they will produce,
generated from the changelogs at the commits to tag by the workflows'
own `awk`:

- **deduxit** (`.github/workflows/release.yml`, job `release`): the
  title `deduxit 0.4.0`; the notes are `CHANGELOG.md`'s section
  `## [0.4.0] - 2026-11-15` (to the next `## [`), then the line naming
  the manual's Installation chapter and `SHA256SUMS`. Attached:
  `deduxit-0.4.0.crate` and `deduxit-cli-0.4.0.crate` from
  static.crates.io, the archives and Linux packages of `packages.yml`,
  `SHA256SUMS`, each attested.
- **rocq-deduxit** (its `release.yml`): the title `rocq-deduxit 0.4.0`;
  the notes are its `CHANGELOG.md`'s section `## [0.4.0] - 2026-11-15`,
  then the line `The source archive for opam: …, sha512=…`.

A change of either text is a change of that changelog before the tag;
after it, `gh release edit v0.4.0 --notes-file …` (an administrative
act, the author's yes first).

**The date.** Both changelogs, both `CITATION.cff` files and the Rocq
library's opam tag `date:` say 2026-11-15. If the tags are made on
another day, the planning session moves all five to that day before
tagging (the release workflows check only that the section is dated).

## What a user can do now

What the release notes' first paragraph sums up, for an announcement:

- Decide intuitionistic and minimal logic by deduxit's own engines (a
  SAT solver by default, LJT with `--method ljt`) and classical logic
  by the SAT solver, with the embedding's derivation and certificate
  kept where it gives them.
- Get a checked Kripke countermodel with every "not valid": printed a
  world per line, in the JSON, drawn as SVG, PNG or PDF, and certified
  for Rocq (`--format rocq`).
- Check a Kripke model you wrote: `deduxit check --logic intuitionistic
  --sequent "…" model.json`.
- Decide classical logic by its truth table (`--method table`) or a
  signed tableau (`--method tableau`).
- Check a Rocq certificate with nothing but Nix: `nix run
  github:deduxit/deduxit/v0.4.0#check-certificate -- proof.v`.
- In the library: decide as the command does (`Settings::stop`), print
  without a stop (`export::…::to_string`, `Display` for a lambda-term),
  read the command's JSON back (`Outcome: Deserialize`).
- Meet fewer dead ends: hints on a newcomer's first errors, `interact`
  listing its rules, messages that say what to do, `prove -h` short and
  grouped, the manual reorganized as a tour, guides, reference and
  explanation.
- On the page: a goal that is not valid shows its countermodel, and
  classical logic has its truth table and tableau.

## Before the tags

- 56b's page is on the client's `main` (the planning session holds the
  release for it).
- `kani.yml`'s run on `main` is green: since 2026-11-15 `nix flake
  check` leaves Kani's harnesses out, and 58a changed the checker's
  arithmetic. `gh run list --repo deduxit/deduxit --workflow kani.yml`
  (a read, no ask).
- `ci.yml` is green on `main` at the commit to tag.
- A baseline night, if the author wants the new engines' times recorded
  (`bench/baseline.sh --detach --fresh`, about 3 h); otherwise the
  target sets and the harness's ILTP runs stand.

## After the tags

- The publish job asks the author's approval in the environment
  `crates-io`. `deduxit` and `deduxit-cli` exist on crates.io with their
  trusted publishers, so the job publishes both through Trusted
  Publishing (`crates-io-auth-action`); `linlog` and `linlog-cli` are
  skipped, since crates.io has their 0.3.0; no secret is used.
- The planning session moves the client's input `deduxit` to the tagged
  commit, and checks the live site and docs.rs.
- The Zenodo check, as for 0.3.0 (`plan/notes/release-0.3.0.md`, "After
  the tags: the Zenodo check"), with `v0.4.0`: a record titled
  `deduxit` under 10.5281/zenodo.23448588 and one titled
  `rocq-deduxit` under 10.5281/zenodo.23899109.
- **opam waits** (the author, 2026-11-09: "Let's wait with OPAM until we
  have more features"). When the author decides, the submission in
  `~/Projects/own/rocq-opam` (`rocq-deduxit.0.3.0`, unsent) is redone
  for the release current then, as `plan/notes/release-0.3.0.md` says:
  its file renamed, the checksum and date filled in, the installs per
  Rocq version, the pull request. `--kernel deduxit` as the default for
  every proof waits for the package to be in the archive (Q11).

## The author's checklist

Before the tags:

- [ ] Nothing to set: the trusted publishers of all four crates name
  `deduxit`/`deduxit`/`release.yml`/`crates-io`, and no secret is
  needed. Confirm on crates.io that `deduxit` and `deduxit-cli` still
  list theirs.
- [ ] Say whether a baseline night should record the new engines' times
  first.

After the tags (the planning session makes them, rocq-deduxit first):

- [ ] Approve the job `cargo publish` in the environment `crates-io`.
- [ ] The Zenodo check above, for both releases, if the planning
  session has not reported it.
- [ ] Decide when to submit rocq-deduxit to the Rocq opam archive.

## The notes as the workflows will write them

### deduxit 0.4.0

````markdown

Intuitionistic logic decided by its own engines, with Kripke
countermodels, and truth tables and tableaux for classical logic. A SAT
solver decides classical, intuitionistic and minimal logic by default,
with Dyckhoff's calculus LJT beside it. A sequent that is not valid
comes with a Kripke countermodel, which is checked, printed, drawn and
certified in Rocq, and `check` checks a model you give. The Horn
engine's affine refutations are checked, the command guides a
newcomer's first errors, and the help, the messages and the manual are
rewritten. On the web page, a goal that is not valid shows its
countermodel, and classical logic has its truth table and tableau. The
Rocq library rocq-deduxit 0.4.0 adds intuitionistic and minimal logic
with their Kripke models, which the countermodels' certificates need.

### What breaks, and what to do instead

- **`prove --logic L --format json` writes the ordinary outcome under
  every method**, at wire level 12 (14 for the methods `table` and
  `tableau`). Through the embedding it wrote the
  translated sequent's outcome at the top; now that outcome is nested
  under `search.image`. Under every method the keys `engine`,
  `translation` and `statistics` moved into `search`, an object tagged
  by `method` (`embedding`, `ljt` or `sat`). Read `search.method`,
  `search.translation`, `search.statistics` and `search.image`, or give
  `--linear` for the translated sequent's outcome alone, as before. The
  batch's JSON Lines and files under `--logic` are the same.
- **`ordinary::decide` takes the sequent and the options alone**, within
  the default limits, as `linear::prove` does. The call with a search's
  options, limits and a stop is `ordinary::decide_within(&sequent,
  &options, &search, &limits, stop)`.
- **`prove --logic intuitionistic` and `--logic minimal` decide by a
  SAT solver by default**, whose search always ends, with the derivation
  of LJ that Dyckhoff's calculus LJT gives where it gives one. The
  verdict line reads `valid (intuitionistic logic by the SAT solver)` or
  `not valid (…): a checked countermodel refutes it`, `--stats` prints
  the solver's counters, and `--format json` writes the outcome of the
  ordinary sequent (`verdict` `valid`, `not_valid` or `unknown`,
  `search.method` `sat`) in place of the translated sequent's. Where a
  bound other than
  the time limit stops the solver, the embedding decides as before. For
  the old behaviour, give `--method embedding` or `--translation`.
- **`prove --logic classical` decides by the SAT solver by default**,
  and keeps the embedding's output wherever the embedding gives a
  derivation of LK or a refutation within 2²⁰ units of work
  (`ordinary::EVIDENCE_WORK`): its verdict line, derivation,
  refutation and certificate, as before. Where the embedding proves
  the translated sequent but its derivation of LK passes the
  derivation limit, `--format json` writes the solver's clausal proof
  in place of the translated sequent's proof. Otherwise the solver's answer stands: `valid (classical logic
  by the SAT solver): a checked clausal proof of …`, with no derivation,
  or `not valid (…): a checked countermodel refutes it`. On the ILTP
  library at 2 s the default decides 254 problems where it decided 124.
  For the old behaviour, give `--method embedding`.
- **`ordinary::Outcome` says how the sequent was searched**, in its new
  field `search` (`ordinary::Search::Embedding { translation, linear }`,
  `ordinary::Search::Ljt` or, with the feature `sat`,
  `ordinary::Search::Sat`), which replaces the fields `translation`
  and `linear`. `ordinary::decide` decides every logic by the SAT
  solver (LJT and the embedding without the feature `sat`) unless
  `ordinary::Options` names a translation or a method; in classical
  logic it then asks the embedding for its evidence within
  `ordinary::EVIDENCE_WORK`.
- **`ordinary::Verdict::Valid` holds the evidence**, an
  `ordinary::Evidence`: `Evidence::Derivation` with the checked
  derivation of LK or LJ, as before, or, with the feature `sat`,
  `Evidence::Clausal` with a checked clausal proof. Match
  `Verdict::Valid(Evidence::Derivation(derivation))`, or call
  `evidence.derivation()`, which refuses a clausal proof with the new
  `Error::NoDerivation` (code `no_derivation`; its field `evidence`
  names what the sequent is valid by).
- **`ordinary::Rule` has the variant `Cut`**, Gentzen's cut in LJ,
  which LJT's derivations hold; it comes before `Open`, which moves
  `Open`'s place in the derived order and its number. A `match` over
  `ordinary::Rule` without a wildcard adds an arm for `Rule::Cut`.
- **`Error::Logic` names the method** that does not decide the logic,
  in its new field `method`: LJT for classical logic, as before, and the
  truth table and the tableau for intuitionistic and minimal logic.
  A pattern that names its fields adds `method` or `..`.

### Changed

These outputs changed their words; a script that matches them needs
the new text. The JSON forms, exit statuses, flags and defaults are
unchanged.

- **The refusals of a proof of LK or LJ step by step** (`interact
  --logic`, `ordinary::StepError`) say what to do: a rule the logic
  does not allow is "not available in L logic", with the way out where
  there is one (`∨R₁` or `∨R₂` for `∨R` in LJ), and a position past the
  goal's formulas says positions start at 0.
- **`--net`'s last line** says whose verdict it is: `proof net (the
  correctness criterion holds)`, `proof net with Mix (the correctness
  criterion with Mix holds)` and `planar proof net (the correctness
  criterion holds, and no two links cross)`. It still starts with
  `proof net` or `planar proof net`.
- **A classical refutation of a sequent written with two sides** speaks
  of the sides: `A on the left has no A on the right to meet in an
  axiom`, and counts left and right where both have some. Where an
  unused hypothesis is the problem, the command adds `; with -a
  (affine), an unused hypothesis is allowed`. The JSON `refutation` is
  unchanged.
- **Three refusals say what to do**: in `interact`, a goal that is no
  number says ``"x" is not a goal: a goal is a number, and `goals` lists
  them`` instead of Rust's parse error, and a position that is no number
  says ``"x" is not a position: a position is a number, and `goals`
  lists them``; `prove --net` on MELL with units
  says `proof nets with boxes exist only for MELL without units, and the
  sequent has a unit`.
- **The intuitionistic count refutation** counts by sides too, without
  "positively" and "negatively": `A occurs 1 more time on the right than
  on the left, so they cannot all meet in axioms (an implication's
  antecedent counts on the other side)`. Where one side has no `A`, it
  says so: `A on the right has no A on the left to meet in an axiom (an
  implication's antecedent counts on the other side)`.
- **An SVG drawing's description** (`<desc>`, which a screen reader
  reads, and a PNG and an accessible PDF carry) writes a negated atom
  `A⊥`, as the drawing and its title do, where it wrote `~A`. A net's
  description is short sentences in place of its text form: what it
  concludes, each link with its formulas by number, each box, and
  whether it is a proof net.
- **Ordinary logic's "not valid"** through the embedding says `the
  translated sequent is unprovable` in place of "the image".
- **The help and the messages are rewritten** to say what went wrong and
  what to do. `prove -h` lists the common options under the headings
  Logic, Input, Output, Search and Limits and ends with "--help shows
  every option."; `--help` lists every option. In `interact`, a rule
  that does not act on the formula named now names the formula and
  `rules G P`.
- **The manual is reorganized**: a short tour, a guide for each task
  (among them "Avoid the common mistakes", "Check a proof or a
  refutation" and "Use the library"), the reference, and an explanation
  of how the search works.

### Added

- **Truth tables and tableaux, the methods of a first logic course**,
  for classical logic: `prove --logic classical --method table` prints
  the sequent's truth table, its falsifying rows marked, and `--method
  tableau` a signed tableau (`T` true, `F` false) with numbered lines,
  each naming the rule of LK and the line it expands, and each branch
  closed (`×`) or open. A closed tableau is a derivation of LK built
  step by step in the goal state and checked whole; an open branch and
  the first falsifying row give a checked countermodel. The table is
  bounded by the new `Limits::table_atoms` (10 atoms, 1024 rows, by
  default; `--table-atoms N|none`), refused past it with
  `Refusal::Atoms` (code `too_many_atoms`). In the library:
  `ordinary::Method::Table` and `Method::Tableau`, `ordinary::truth_table`
  and `TruthTable`, `ordinary::Tableau` (`Tableau::of` reads any
  derivation of LK as one) with `TableauNode`, `Sign`, `Signed` and
  `End`, `ordinary::Search::Table` and `Search::Tableau`, and
  `ordinary::Evidence::Table`. The bindings' `ordinary_table` and
  `ordinary_tableau` answer either method's outcome, and `about` lists
  them as the ordinary suite's methods.
- **A "not valid" comes with its Kripke countermodel**: the SAT solver's
  model in every logic, and in classical logic the embedding's
  assignment as a model of one world, checked before the verdict.
  `prove --logic` prints the SAT solver's after the verdict line, a
  world per line (the embedding's verdict line names its assignment, and
  where its refutation is none the solver's model follows it);
  its JSON holds it under `countermodel`, with the sequent it refutes,
  and `check` checks it there. LJT's "not valid" has none. A model too
  large to check (past `ordinary::MODEL_CELLS` cells or the memory
  limit) is reported as too large to show, the verdict kept.
- **`check --logic L --sequent SEQUENT FILE` checks a Kripke model you
  give** against a sequent: that it is a model of the logic, then
  whether the sequent holds at `--world N` or at every world, and where
  it fails, which world and what it forces of each subformula (exit
  status 0, 1, or 2 for no model of the logic). The bindings' call
  `ordinary_model` does the same. A model that fails its check is the
  new `Error::Model` (code `invalid_model`, kind invalid).
- **A Kripke countermodel is drawn**: `--format svg`, `png` and `pdf`
  on a "not valid" with a countermodel draw it, each world a box with
  the atoms it forces (and `⊥` in minimal logic), an arrow up to each
  world directly above it, the model's world outlined in the highlight
  colour, with a title and a description in sentences. `check` draws an
  outcome's countermodel and a model you give at the world where it
  fails. LaTeX and Typst draw none and say so. In the library,
  `export::svg::countermodel`.
- **The bindings give the page a countermodel**: `ordinary_close` and
  `ordinary_hint` answer a goal that is not valid with the SAT solver's
  countermodel in the outcome, and `ordinary_model_drawing` and
  `ordinary_model_certificate` draw it as SVG and write its Rocq
  certificate.
- **A Kripke countermodel has a Rocq certificate**: `--format rocq`
  on a "not valid" of intuitionistic or minimal logic writes the lemma
  `~ lj (atom := string) intuitionistic [Γ] C` (or `minimal`) for the
  Rocq library, proved by its `kripke_refutation` from the model as
  data, which the library checks by computation; a classical model of
  one world gets the certificate over `Prop`. `check --format rocq`
  writes it for an outcome's countermodel and for a model you give, at
  the world where it fails. In the library,
  `export::rocq::write_countermodel` and the prelude
  `DEDUXIT_KRIPKE_PRELUDE`. It needs the Rocq library's
  `Deduxit.Intuitionistic`, which its 0.4.0 adds.
- **`deduxit::kripke::Model`**, the countermodel shared by the suites:
  worlds, their atoms by name, `falsity` for minimal logic, the arrows
  of the order and the world it is about. Ordinary logic checks it
  (`Model::forcing`, `Model::refutes`, `ordinary::Forcing`), and
  `ordinary::Countermodel` pairs it with the sequent it refutes, in the
  new `ordinary::Outcome::countermodel`. It replaces `ordinary::sat::Model`
  and `World`, whose atoms were indices.
- **`check` reads every outcome `prove --logic` writes**, and a
  derivation of LK or LJ: it checks the derivation by the rules of its
  calculus (LJ's cut included) and the translated sequent's proof or
  refutation where the embedding found one. A clausal proof, written as
  its counts, a "not valid" of LJT or the SAT solver, which carries no
  refutation, and an unknown hold nothing to check: `unknown`, exit
  status 3. Its
  JSON names the `logic` in place of the `mode` for these.
- **The ordinary outcome reads back**: `ordinary::Outcome` is
  `Deserialize` and read by `wire::upgrade`, its derivation and the
  translated sequent's proof unchecked until their checkers run, as the
  linear outcome is; `ordinary::ljt::Statistics` and
  `ordinary::sat::Statistics` read back too.
- **`ordinary::Image::outcome(linear, &limits, stop)`** makes the ordinary
  outcome from a search of the translated sequent that the caller ran,
  as the command does: a proof's derivation is read back and checked, or,
  where a bound or the stop refuses it, the new `ordinary::Evidence::Image`
  says the translated sequent's proof stands alone (`evidence.derivation()`
  answers the new `Error::NotBuilt`, code `not_built`).
- **Hints on a newcomer's first errors**: `deduxit "A |- A"` suggests
  `deduxit prove "A |- A"`; an unquoted sequent suggests the command
  quoted; `->` in linear logic says to write `-o` or add `--logic
  classical`; a file of `NAME: SEQUENT` lines read as one sequent
  suggests `--input-format lines`. `--format tex` is read as `latex`.
- **`interact` guides**: an unknown rule's error lists the session's own
  rules with their ASCII spellings, and after a split whose premise has
  no proof a warning says so and that `undo` takes it back.
- **Deciding as the command does, in the library**: `Settings::stop`
  builds the command's stop from the time since the start. It fires at
  the time limit and lets the classical reading after the search run
  past it.
  `Settings::deterministic()` gives the settings of `prove
  --deterministic`: one thread throughout.
- **Printing without a stop**: `export::{latex, typst, svg,
  rocq}::to_string(derivation, &options)` beside each `write`, and
  `Display` for `lambda::Term`.
- **Reading the command's JSON**: the linear `Outcome` implements
  `Deserialize`, so a file of `prove --format json` reads back with its
  verdict, mode and engine. An engine name the library does not know
  reads as the new `Engine::Unknown`.
- **A family refuses an index past its instances**: `Family::instance`
  returns the new `Error::FamilyIndex` (code `family_index`), where it
  returned an instance whose expected verdict could be wrong. `deduxit-bench
  one --problem family:NAME:SIZE:INDEX` refuses it too.
- **`with_lambda`** on `export::latex::Options`, `export::typst::Options`
  and `export::svg::Style`, as every other field has its `with_*`.
- **A refused step names its formula**: `StepError::Rule` carries
  `formula`, the formula as the session writes it, and its message says
  it (`the rule ⅋ does not act on formula 1, A ⊗ ~B`). The web bindings'
  error has it under `details`.
- **Errors show the formula**: `ShapeError::Formula` carries the
  formula's text, so `prove` in an intuitionistic mode names the
  subformula that has no intuitionistic reading.
- **LJT**: `ordinary::ljt::decide` decides intuitionistic and minimal
  logic by Dyckhoff's contraction-free calculus. Its proofs are
  translated into derivations of LJ with Gentzen's cut, which
  `Derivation::check` accepts. `ordinary::Method`,
  `ordinary::Options::method` and `ordinary::gives_way` choose between it
  and the embedding, and the command has `--method embedding|ljt` (the
  key `ordinary.method`).
- **A SAT-based engine for classical, intuitionistic and minimal
  logic**, in the library: `ordinary::sat::decide`
  (`ordinary::Method::Sat`, behind the new default feature `sat`, the
  SAT solver batsat). In intuitionistic and minimal logic every "valid"
  comes with a derivation of LJ that the checker accepts. In classical
  logic it comes with the solver's clausal proof (`sat::ClausalProof`),
  which a checker of deduxit's own accepts before the verdict is given;
  it has no derivation, which `Method::Embedding` gives. Every "not
  valid" comes with a model that `Model::refutes` checks: a finite
  Kripke model (`kripke::Model`), of one world in classical logic. The
  command offers it as `--method sat` (the key `ordinary.method`), with
  its counters under `--stats`, and the harness as the engine `sat`. It
  is the default for intuitionistic and minimal logic, where on the ILTP
  library's 274 problems at 2 s it decides 252 in intuitionistic and 253
  in minimal logic, LJT 200, and takes LJT's derivation where LJT gives
  one (`ordinary::sat::decide_preferring_ljt`), and for classical logic,
  where it keeps the embedding's derivation, refutation and
  certificate wherever the embedding gives them within
  `ordinary::EVIDENCE_WORK`.
- **The harness runs ordinary logic**: `deduxit-bench run --tptp PATH
  --logics …` with the engines `embedding`, `ljt` and `auto`, and
  `summary` lists the problems two configurations decide differently.
- **A certificate is checked in one command**: `nix run
  github:deduxit/deduxit#check-certificate -- proof.v` compiles a
  certificate written with `--standalone` against the pinned NanoYalla
  or rocq-deduxit, the kernel read from its import lines. It refuses a
  lemma that rests on an axiom, such as a proof left `Admitted`. The
  package of the same name installs it (`nix profile install
  github:deduxit/deduxit#check-certificate`).
- **A checked refutation for the Horn engine's backward search**: in
  affine mode, where only that search refutes a Petri net, the
  unprovable verdict carries the markings it kept
  (`Refutation::Basis`, with `Basis` and `Marking`), checked before the
  verdict and by `Disproof::check` and `deduxit check` without the
  search. It was "the search was exhaustive" before. `--format rocq`
  has no certificate for it yet (`Unsupported::Basis`).

### Fixed

- `ordinary::Derivation::term`, and every export that writes a
  lambda-term, panicked on a derivation of LJ read from JSON that breaks
  a rule. Such a derivation is now checked first, and refused with the
  check's `Error::ReadBack`; one that `check` accepted reads as before,
  in either view of its structural rules.

### Wire forms

- Level 9 adds LJ's cut, `"rule": "cut"`, in a derivation of LJ and
  a proof in progress that holds one, written at level 9.
- Level 10 adds the outcome of a sequent valid by a clausal proof:
  `"clausal_proof": {"variables", "clauses", "steps"}` in place of
  `derivation`. 0.4.0 writes every ordinary outcome at level 12 or 14,
  so no document of it is at level 10.
- Level 11 adds the refutation by a basis, `{"kind": "basis", "body",
  "classes", "markings"}`, in a disproof or an unprovable outcome that
  holds one.
- Level 12 writes every ordinary outcome with its `search` tagged by
  `method`, the translated sequent's outcome nested under the embedding's
  `image` (an outcome's keys without `version` and `deduxit`), and reads
  it back; an ordinary outcome below level 12, which no release read, is
  refused. Every other document is written as before.
- Level 13 adds the Kripke model, `{"worlds": [{"atoms", "falsity",
  "arrows"}], "world"}`, as a document, which refuses a key it does not
  know (a model is often written by hand). An ordinary outcome holds one
  under `countermodel` and stays at level 12: a reader of level 12
  ignores the key without misreading the verdict.
- Level 14 adds the truth table, `{"sequent", "rows": ["TFT…"],
  "falsifying": […]}`, and the tableau, `{"sequent", "nodes":
  [{"lines": [["T", formula]], "expands": [line, rule], "children":
  […]} | {"lines", "end": {"kind": …}}]}`, as documents and as the
  `search` of an ordinary outcome of the methods `table` and `tableau`,
  which is written at level 14. Reading a table computes it again from
  its sequent; reading a tableau checks its shape and indices.


Install it as the manual's [Installation](https://deduxit.github.io/deduxit/manual/installation.html) says; `SHA256SUMS` holds the files' sums.
````

### rocq-deduxit 0.4.0

````markdown

Intuitionistic and minimal logic with their Kripke models, beside
deduxit 0.4.0, whose certificates for `--kernel deduxit` and of a
Kripke countermodel it checks.

### Added

- Intuitionistic and minimal logic, under `Deduxit.Intuitionistic`:
  the formulas (`⊤`, `¬A` and `A ↔ B` defined from `⊥`, `∧`, `∨` and
  `→`) and the calculus `lj R Γ C` (`Calculus.v`), G1i with shared
  contexts, ex falso (`intuitionistic`, LJ) and cut as options, so
  `minimal` (LM) has no rule for `⊥`.
- Kripke models (`Kripke.v`): `kripke_soundness`, every derivable
  sequent holds at every world of every model of its logic, with cut
  too; `kripke_countermodel`, a world that forces the hypotheses and
  not the conclusion shows a sequent underivable; and
  `classical_refutation`, its one-world case, an assignment.
- Finite models as data and their checker (`KripkeCheck.v`):
  `refutes` checks that a model is one of the logic's (a transitive
  relation, a monotone valuation, no world forcing `⊥` in LJ) and that
  a world refutes the sequent, and `kripke_refutation` turns its answer
  into `~ lj R Γ C`, so a certificate states a countermodel and proves
  it by `vm_compute`.


The source archive for opam: `https://github.com/deduxit/rocq-deduxit/archive/refs/tags/v0.4.0.tar.gz`, `sha512=…` (the tag's archive, summed by the workflow).
````
