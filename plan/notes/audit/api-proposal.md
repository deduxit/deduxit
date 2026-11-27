# The 0.9.0 surface: a proposal for the author's sign-off (step 86)

What the audit proposes to change in what users meet: the library's
public API, the command, the bindings' calls, the features and the Rocq
library's interface. It is one decision in 25 parts. Each part says what
changes, why (the findings, by id, in `findings.md`), what it breaks and
what I recommend; where there is a real choice, the alternatives are
named. The refactor (phase D) implements what is signed and nothing
else of this kind.

**How to answer.** "Yes to all recommendations" is a complete answer.
Otherwise name the parts to change by their number (P1 to P25). The
four parts where I most expect another view are marked **(choice)**.

**Decided already** (`criteria.md`, D1 to D4): no logic is the default
in 0.9.0; the API may break where users gain; the exit statuses 0, 1, 2,
3 and 130 stay; no suite is the main one, and where an order is needed
it is ordinary logic, linear logic, modal logic, lambda-terms.

**What the proposal rests on**: 379 findings in 22 units, 371 standing
(`findings.md`); an outside crate's attempt to build two engines
(`consumer-test.md`); the architecture reader's map
(`units/arch/architecture.md`) with its verifier's corrections; and the
recorded decisions a proposal must answer (`standing.md`; section 12
below answers each).

## The parts at a glance

| part | what | size | breaks |
|---|---|---|---|
| P1 | the command's selector of the logic **(choice)** | large | every example |
| P2 | no default logic in the library | small | `Default` impls, two settings |
| P3 | shared root modules | large | public paths; labels reach every suite's drawings |
| P4 | one options value per front door | medium | ordinary logic's signatures |
| P5 | the suites as peers in settings, errors, the registry **(choice)** | large | settings keys, a wire level |
| P6 | builders for sequents, formulas, derivations, terms | medium | `linear::Sequent::new()`, the ids of a rule file's derivation and of a lambda-term's atoms |
| P7 | refusals and a meter an engine can use | small | additive |
| P8 | the engine interface | medium | `Outcome`'s fields, `Engine` |
| P9 | drawing a derivation from outside | small | docs only |
| P10 | the manual's page "Write your own engine" | small | none |
| P11 | evidence inside the verdict | medium | outcomes, a wire level |
| P12 | errors a front end can act on | medium | error variants |
| P13 | one table of names | large | renames |
| P14 | one description of a suite: the protocol and the sessions | large | the bindings' calls |
| P15 | one call for a proof's net | small | additive |
| P16 | the `seq` commands and the batch for every suite | medium | `seq`'s output |
| P17 | the common flags, and one name for a translation | small | one flag |
| P18 | shell completions and a man page **(choice)** | small | two or three packages |
| P19 | the sessions at a terminal | small | none |
| P20 | one path per type, and the unused public items | medium | public paths |
| P21 | common traits and small signatures | small | mostly additive |
| P22 | examples on the types a user starts from | medium | none |
| P23 | one crate | none | none |
| P24 | the features **(choice)** | medium | feature names |
| P25 | the web module's size, in an order | medium | none |

## 1. No logic is the default (D1)

### P1. The command's selector (choice)

Today every command that reads a sequent falls back to classical linear
logic, by five mechanisms (F-7a-01 has the table per command).

**Recommended: one flag per suite, exactly one of them required**, as a
required clap `ArgGroup`:

| suite | flag | values |
|---|---|---|
| ordinary logic | `--ordinary LOGIC` | `classical`, `intuitionistic`, `minimal` |
| linear logic | `--linear MODE` | the library's mode names: `classical`, `affine`, `mix`, `affine-mix`, `intuitionistic`, `intuitionistic-affine`, `cyclic`, `lambek`, `lambek-star` |
| modal logic | `--modal SYSTEM` | the fifteen systems: `K`, `D`, `T`, `S4`, `S5` and ten more |
| a calculus given as data | `--calculus NAME-OR-FILE` | a built-in calculus or a rule file |

- Without one, clap answers with its error for a missing required
  group, which names the four flags, and the command adds a tip with an
  example per suite.
- An input that states its own logic needs none and is never refused
  for lacking one: a proof or outcome file, a saved session, a batch
  whose records name a mode, a `.spec` file (always affine).
- A command that serves one logic names it too, since D1 asks it of
  every command that reads a sequent: `resolve`, `herbrand` and `seq
  normalize` take `--ordinary classical`, the one value they accept.
  `interpolate` loses its default.
- The group differs per command (`prove` has no `--calculus`, `interact`
  no `--modal`), so it is one argument struct per set.
- The linear mode flags `-i`, `-a`, `--mix`, `--cyclic`, `--lambek`,
  `--empty-antecedents` go: `--linear MODE` spells a mode as the batch,
  the workspace and the JSON forms already do. `prove`'s present
  `--linear` (show the image's proof) becomes `--image`.
- The `term` commands read a term, not a sequent, and take no selector.
  Their `--calculus` (one of the library's nine lambda calculi) stays:
  the verifier of F-7a-05 judged that the flag names the calculus to
  work in on every command that has it, and that a rename is not worth
  a break.

Why this form: each suite's values keep their own type and help; no
value needs a prefix to be unambiguous (`classical` is a logic of two
suites); and the flags carry the suites' names evenly (D4), where
`--logic` for ordinary logic alone reads as "the" logic.

**The alternative**: the same required group over the present flags,
`--logic` (ordinary) and `--system` (modal) unchanged, `--linear MODE`
new. It changes only the linear examples. It keeps the uneven names,
and it keeps the letter of the decision of 2026-11-09, "`--logic`
chooses the suite", which the recommendation departs from in the flag's
name: four flags named for the suites in place of one.

**Not recommended**: one flag `--logic` with every logic of every suite
as a value (about 25 values, with prefixes such as `linear-affine`); a
subcommand per suite (the decision of 2026-11-09 rules it out).

Breaks: every console block of README, the manual and the guide; the
behaviour lock; `hint.rs`'s tips; the Rocq library's README (line 66).

### P2. No default logic in the library

- `ordinary::Logic`, `linear::Mode` and `modal::System` lose `Default`.
  `ordinary::Options::new(logic)`, `modal::Options::new(system)`;
  `linear::search::batch::Options::new(mode)`, or every `Problem` names
  its mode (F-2a-12, F-4a-03, unit 4d's remark).
- `Settings`: `ordinary.logic` and `batch.mode` have no default; a
  front end that needs one names it.
- The bindings' `about` stops publishing a default logic to the page;
  each call already names its suite (unit 8's remark).

Breaks: `..Default::default()` on those types; two settings keys.

## 2. What every suite uses leaves the linear suite (D4)

### P3. Shared root modules

About forty findings are one fact: what all suites use lives under
`linear` because it came first. `plan/later.md` has listed the
crate-private half of the move as due since step 67; D2 and D4 now allow
the public half.

| today | proposed | public? | findings |
|---|---|---|---|
| `linear::search::Reason` | `deduxit::Reason`, defined in `limits` beside `Refusal`, at the one path P20 gives the shared items | yes | F-arch-01, F-4d-14 |
| `linear::search::memory::Account` | `deduxit::memory` | crate-private | F-arch-06 |
| `linear::sequents::{Stop, Stops, Walk, Visit}`, `notation::{Notation, Atoms}` | `deduxit::notation` | `Walk` and `Visit` public as today; `Stop`, `Stops`, `Notation` and `Atoms` crate-private as today | F-1a-03, F-arch-06 |
| `linear::sequents::fo::{FoTerm, FoTermId, Symbol, Args, Eigen}` (defined in the root's `first_order.rs`) | `deduxit::terms` | yes | F-1a-02, F-arch-07 |
| `linear::proofs::{InfId, Labels, OpenGoal, TextOptions, Structural, ViewOptions, Layout}`, the trait `Drawn` | `deduxit::view` (`Drawn` crate-private in `export`) | yes | F-1b-01, F-1b-04, F-arch-07 |
| `linear::proofs::GoalId`, `interactive::{Inverted, Options}` | `deduxit::session` | yes | F-4e-03, F-4f-09 |
| `linear::lltp::{Clauses, Status, clauses}` | `deduxit::tptp` (`Status` public, the reader crate-private) | partly | F-arch-07 |
| `ordinary::MODEL_CELLS`; `kripke::Model`'s methods in two suites | `deduxit::kripke`, the methods in one place, the docs in words both suites use | yes | F-arch-21, F-4a-12 |
| `Labels::Table` keyed by the linear suite's `Named` | keyed by a rule's name as text, so that ordinary logic, lambda-terms and rule files take a user's labels | yes | F-1b-04, F-5a-09 |

The linear suite re-exports none of them: one path per type (P20).
`Error` stays the one module that names every suite ("one error
family").

Breaks: public paths, and no behaviour of a search or a checker (the
target sets' counters and the ratchet are the proof). One row changes
output: with `Labels::Table` keyed by a rule's name, a user's labels
apply in the drawings of ordinary logic, lambda-terms and rule files,
which ignore them today. The changelog carries the table as the
migration.

### P4. One options value per front door

Four options that only ordinary logic's engines read (`path_length`,
`restricted_backtracking`, `model_size`, `model_worlds`) are fields of
`linear::search::Options`, so `ordinary::decide_within` takes two
options values, and normal forms, interpolation, the connection prover
and the model search each take the linear one to read two fields
(F-arch-02, F-4b-15, F-4c-04, F-8-10).

**Recommended**: the four fields move to `ordinary::Options`, which also
holds the linear search's options for the embedding it runs
(`embedding: linear::search::Options`). Every call of ordinary logic
takes `&ordinary::Options` alone. `lambda::inhabit` follows.

Breaks: the signatures of `ordinary::decide_within` and ten other
public calls that take the linear options today (the model search, the
negative translations' comparison, the connection prover, the clauses
of resolution, the normal forms and their check, the session's `close`
and `hint`, the interpolants); the settings keys of the four bounds
(with P5).

### P5. The suites as peers in settings, errors and the registry (choice)

- **Settings** (F-6a-09, F-arch-14): the linear suite's options have the
  generic keys `search`, `view`, `input`, `cut`, `interactive`, `batch`,
  and ordinary logic's bounds are set through `search`.
  **Recommended**: a table per suite (`ordinary`, `linear`, `modal`,
  `lambda`, `rules`) beside the shared ones (`limits`, `view`, `export`,
  `session`). **The alternative**: keep the keys and document them as
  the linear suite's; no break, the unevenness stays.
- **Errors**: `Error::Step` (linear) beside `Error::OrdinaryStep` and a
  third type for rule files becomes one refusal of a step (P12).
- **The registry and the bindings** (F-arch-15): the page's order
  everywhere an order shows (`SUITES`, `describe`, the manual's
  chapters); every suite's calls of the bindings carry its name, or
  none does (P14 proposes none, with the suite as an argument).

Breaks: a settings file's keys and the page's stored settings, once,
with one wire level for all of the proposal's JSON changes (section 11).

## 3. Building values and running an engine from outside

### P6. Builders

| value | today | proposed | findings |
|---|---|---|---|
| a linear sequent | text or JSON only; the crate works around it in two places (the embedding, the families) | `linear::Formulas` (`atom(name)`, `add(Term)`, `dual(id)`) and `Sequent::new(formulas, left, right)`, the shape ordinary and modal logic have | F-1a-01, F-arch-05, F-4a-16 |
| a first-order ordinary formula | a binder can be built (`Formulas::add(Node::Forall(…))`); a term and an atom with arguments cannot be built or read | constructors and readers for terms and for predicates with arguments on `ordinary::Formulas` | F-4a-02 |
| an ordinary derivation | `from_parts` is crate-private | `Inference::new(…)` with `with_principal`, `with_premises`, `with_term`; `Derivation::new(logic, formulas, left, right, inferences)`, unchecked until `check` | F-arch-05 |
| a lambda-term | text or JSON only; no public walk; a type cannot be printed | a builder on `lambda::Term`, a public iterator over subterms, `Display` for a type | F-5a-02, F-5a-03, F-5a-04 |
| a linear proof under a stop | `Proof::new` polls a stop that never fires | `Proof::build(…, limits, stop)` public | F-1b-09 |
| what a `Goal` holds | private | accessors for its members | F-2a-03 |
| axiom partners | `Forest::dual_literals` is crate-private | public | F-1a-08 |
| a rule file's derivation | raw `u32` beside typed ids; positions unsaid | typed ids throughout; the order of positions documented on `Rule` | F-5b-03, F-5b-04 |

The recorded sketch of the sequent builder (R59, `plan/notes/api.md`)
is `Sequent::builder()` with `finish(&Limits)`. I recommend the
`Formulas` shape instead, because two suites already have it and a
third spelling of the same thing is what F-1a-05 finds too many of. It
keeps R59's substance: indices checked, hash-consed as the parser does,
so that a built sequent equals the parsed one.

Breaks, for this part is not additive: `linear::Sequent::new()`, today
the empty sequent, takes the formulas and the two sides as in the other
suites, and the empty sequent stays `Sequent::default()`; a rule file's
`Node` and `scope` take typed ids where they take `u32` today; a
lambda-term's atoms get an id type.

### P7. Refusals and a meter

`Refusal::stopped(phase)`, `Refusal::work(limit)`,
`Refusal::memory(phase, limit, needed)`; `limits::Meter` (counts work,
asks the stop every so many units, refuses at `limits.work`), which
replaces five private meters; `Progress::new` public beside the
`Default` it has (F-arch-03, F-6a-03, F-2a-10, F-5a-05). The crate uses
them first: `Refusal::Stopped` is built by hand at 103 sites today.

The rule "a front end never builds an error of the library" stays: the
constructors' docs say they are for engines.

### P8. The engine interface

For the linear suite, the architecture reader's sketch as unit 2a
refined it from inside the front door (F-arch-04, F-2a-02):

```rust
/// A search from outside the crate.
pub trait Decide {
    /// Its name, as an outcome and the statistics write it.
    fn name(&self) -> &'static str;
    /// The counters it reports, labelled.
    fn counters(&self) -> &'static [Counter];
    /// Says why it does not take a goal.
    fn admits(&self, goal: Goal<'_>, mode: Mode) -> Result<(), NotTaken>;
    /// Decides the goal. `stop` takes the work done since it was last
    /// asked, in the engine's own unit; the front door turns it into
    /// `Progress` and the work bound.
    fn decide(&self, goal: Goal<'_>, mode: Mode, options: &Options, limits: &Limits,
              stop: &mut dyn FnMut(u64) -> bool) -> Result<Found, Error>;
}

#[non_exhaustive]
pub enum Found {
    Proved(Proof),
    Unprovable(Option<Refutation>),
    Unknown(Reason),
}

/// Decides `goal` by `engine` and answers as `prove_goal` does: the proof
/// checked unless `options.check` is off, the refutation checked.
pub fn prove_with(engine: &dyn Decide, goal: Goal<'_>, mode: Mode, options: &Options,
                  limits: &Limits, stop: impl FnMut(Progress) -> bool) -> Result<Outcome, Error>;
```

With it:
- `Outcome`'s fields become private with accessors, so that `checked`
  and `verdict` cannot be written; `prove_with` and the built-in front
  door are then the only ways to an outcome that says "checked".
- What an outcome names as its engine becomes a small type that holds a
  built-in `Engine` or a name; `Engine` stays the fieldless list of the
  built-in engines that an option can force, and loses `Unknown`
  (F-2a-13). An outside engine cannot be forced by name.
- The built-in engines keep their private trait and loops: the target
  sets' counters show that nothing moved.
- For ordinary logic, whose evidence is a derivation or a model:
  `Outcome::of_derivation(derivation, limits, stop)` and
  `Outcome::of_countermodel(…)`, each running the check, and
  `Outcome::unknown(logic, reason)` (unit 4a's remark).
- A refutation of linear logic: `Disproof::check_within(limits, stop)`
  (F-1c-01), constructors for the certificate kinds an engine can state
  (F-1c-02), and the docs of what each kind promises (P11).

### P9. Drawing a derivation from outside

The exports take a closed list of the crate's derivations
(F-arch-26). **Recommended for 0.9.0**: say in the exports' docs that a
derivation of another logic is drawn as a `rules::Derivation`; keep
`Drawn` private until someone needs it public.

### P10. The manual's page "Write your own engine"

A worked example in two ways: a new search for linear logic through
`Decide` and `prove_with`, and a new logic through a rule file and
`rules::Derivation::check`. The readme test runs it. It is the caller
that `core.md`'s rule "an item without a caller waits for one" asks of
P6 to P8, and the acceptance test of this section.

## 4. Verdicts, evidence and errors

### P11. The evidence inside the verdict, and what a value promises

- `ordinary::Outcome` holds three optional countermodel fields beside
  its verdict, so it can say valid and hold a countermodel; the linear
  verdict carries its disproof in the variant (F-arch-13).
  **Recommended**: `Verdict::NotValid` carries what refutes, an enum
  with one variant per kind of countermodel, in ordinary and modal
  logic alike. It needs a name of its own (`Counterexample`, say):
  `Evidence` is what `Valid` carries today.
- One rule, on the types' docs: a value built or read has passed no
  check until `check` is called, and each type says what a value inside
  a verdict has passed. Today that differs: a proof in `Proved` has
  passed the checker when the outcome says `checked`; of the linear
  refutations the front door checks the saturated set and the basis,
  and computes the counts' and the classical one itself without
  checking them again (F-2a-09). `Verdict`, `Refutation`,
  `Disproof::new` and the countermodel types each say which of their
  values are certificates (nine of the ten refutations of linear logic
  are) and which are a search's word (F-arch-23, F-1c-03, F-4d-13).
  Whether the front door should check every refutation it returns, so
  that the rule has no exception, is a small change of behaviour I
  recommend to decide with the engine interface (P8).
- `NormalForm` and `Interpolant` keep their results private behind
  accessors (F-4b-04). That removes a use: setting a result by hand and
  calling `check` on it. **Recommended**: a constructor that builds an
  unchecked value from a result, so that a normal form or an interpolant
  from elsewhere can still be checked; the alternative is to drop that
  use.
- A first-order refutation says why it was not read back as an enum
  (F-4c-05).

Breaks: the fields of the ordinary and modal outcomes and their JSON
form (one wire level, section 11).

### P12. Errors a front end can act on

- A structured fault for each checker that answers with a `String`
  today: the Kripke model, the structures, a rule file's derivation, a
  calculus (F-6a-15, F-5b-05). The page's hints and the command can then
  point at the world or the node.
- One refusal of a step for the three sessions, with one code
  (F-4e-06).
- `Error`'s variants take the names of their stable codes where the two
  disagree (F-6a-13 lists them); the codes do not change.
- A public call that writes an error's JSON form with its details, so
  that the bindings stop writing errors by hand (F-8-05).

## 5. One description of what a suite offers

### P13. One table of names

| thing | today | proposed |
|---|---|---|
| intern an atom by name | `atom(name)` constructs in ordinary and modal logic and looks up in linear logic | `atom(name)` constructs everywhere; the lookup is `find_atom` |
| a session is finished | `is_complete` (linear, ordinary), `is_finished` (rule files) | `is_finished`, beside `finished()` |
| a session's result | `proof()` (linear), `finished()` (the others) | `finished()` |
| a session's open goals | an iterator, an iterator, a `Vec` | an iterator |
| one goal of a session | `Result`, `Result`, a panic on a foreign id | `Result` |
| undo | `Option<GoalId>`, `Option<GoalId>`, `Option<Undone>` | `Option<GoalId>` |
| the short and the bounded form of a call | `prove` and `prove_within`; but `normalize_default` and `normalize`, and three more pairs of ordinary logic | `X` and `X_within` everywhere |
| names of values in JSON and as flag values | snake case in `lambda::Calculus` and `Strategy`, kebab case in `Engine` and `Translation`; 18 mirror enums in the command | kebab case everywhere; each enum's `NAMES` public, and the command reads them |
| an `Error` variant and its code | `Model` and `invalid_model`, `Reduction` and `reduction_rejected`, and others | the variant named for the code |

Not proposed: renaming the linear suite's `Term` and `TermId` to the
`Node` and `NodeId` of the other formula stores (F-1a-05). It would be
the largest rename of the release, and a linear proof's `Node` holds
the name. The verbs above give most of the gain.

### P14. The protocol and the sessions

- The registry is written once, in the page's order; `OPERATIONS`,
  `KINDS` and `statement` are read from it (F-arch-11: `KINDS` has
  drifted and names no kind of lambda-terms).
- The registry becomes a value (`Registry::all()`, or one built from a
  list of suites), which the workspace's runs take. `Suite` stays
  unsealed, as recorded: a suite of another crate joins a registry. It
  is also what lets the bindings link only the suites a page uses
  (P25).
- The session becomes part of the protocol: a trait `Session` over JSON
  documents (`start`, `view`, `candidates`, `apply`, `undo`, `close`,
  `hint`), which each suite with step-by-step proving implements, and
  the step and candidate forms get a wire form in the library
  (F-arch-12; the linear session's hint and invertible steps move from
  the bindings into the library, F-4f-01).
- The bindings then export one set of session calls with the suite as
  an argument, in place of three sets written by hand, and their own
  table of calls goes. The command's `interact` keeps the typed
  sessions.
- Library calls for what the front ends write by hand: read one
  document from text and refuse what follows it (45 sites, F-8-04);
  tell which document a text is (six places, F-8-16); the built-in
  calculi by name (two tables, the command's and the bindings', with
  different contents, F-5b-02).
- A protocol operation that renders a lambda-term to the kind `term`,
  with its text and judgement, as `linear.term` and `ordinary.term` do.
  The stored forms of an inhabitation and a reduction stay as they are:
  their verifier showed that adding to them would break the cache's
  keys and the check of a reduction (F-8-09).
- The workspace's states and the id newtypes get serde forms (F-8-11,
  F-6a-07).

Breaks: the names of the bindings' session calls, in one step with the
client.

### P15. One call for a proof's net

Which net a proof gets is chosen by hand in three front ends, and the
protocol's copy is wrong (F-8-01). **Recommended**:
`linear::nets::Net::of(&proof, limits, stop)`, a public sum of
`ProofStructure` and `MellStructure`, with the drawing and the
criterion on it.

## 6. The command, beside its selector

### P16. The `seq` commands and the batch for every suite

`seq print --logic classical 'a -> b |- b'` prints the linear image, not
the sequent, and `seq json` and `seq fragment` take linear logic alone
(F-7a-04). **Recommended**: `seq print` and `seq json` take the selector
of P1 and print the sequent of the logic named; the linear image is
`seq print --image`, the flag `prove` gets for the same thing in P1.
`seq translate` stays the negative translation. `seq fragment` is
linear logic's. The batch's records name their logic.

### P17. The common flags, and one name for a translation

- A translation has two names for one thing and one name for two:
  `prove --translation` is the embedding into linear logic, while the
  negative translation is `seq translate --by` on the command and
  `translation` in the workspace's settings. **Recommended**: `seq
  translate --by` takes the workspace's name, and the help of `prove
  --translation` says which translation it means. The other renames
  F-7a-05 proposed (`--to`, `--calculus`) are not: its verifier found
  that each names the same thing on every command that has it.
- `--timeout`, `--memory-limit`, `--work-limit`, `--settings` and
  `--quiet` on every command that runs something, declared once
  (F-7a-13, F-7b-04, F-7b-18); a bound that has a flag on one command
  has it on all that honour it.
- `--output -` writes to standard output (F-7a-07).
- `eliminate` offers only the flags it reads (F-7b-03).

### P18. Shell completions and a man page (choice)

The command has 34 subcommands and 80 flags and ships neither
(F-7a-11). **Recommended**: `deduxit completions SHELL` through
`clap_complete` (one package); the man pages generated by `clap_mangen`
in the flake's package build, from a small program of the workspace
(two packages). `Cargo.lock` holds 175 packages of 180. **The
alternative**: completions alone.

### P19. The sessions at a terminal

On a terminal `interact` shows the goal it starts from and one line
that names `help` and `quit` (F-7b-12); a session in a rule file draws
in the formats the other two have (F-7b-06); the three sessions word
the same answers alike (F-7b-19).

## 7. The surface kept small

### P20. One path per type, and the unused public items

- 93 items are reachable under two or three paths (F-1a-04, F-1b-08,
  F-3-08, F-6a-06). **Recommended**: a suite's users name its items at
  the suite's root (`deduxit::linear::Proof`) or in one named submodule
  where the root would be crowded (`linear::nets`, `linear::search`);
  the leaf modules that only exist to be re-exported become private.
  The shared items have one path too: `deduxit::Error`, `Limits`,
  `Progress`, `Refusal`, `Reason`, `Phase` and `Settings` at the root,
  their modules public only for what the root does not hold (the
  constants of `limits`, the tables of `settings`).
- 82 items are named by nothing outside `core/src`
  (`inventories/surface.md`). Each becomes `pub(crate)` unless the
  manual or the guide documents a use for it; the refactor step lists
  its choices, and `breaks 0.8.0` shows them.

### P21. Common traits and small signatures

`PartialEq` and `Eq` for `Proof`, `Forest`, `ProofStructure` (by its
canonical link list), `ordinary::Sequent`, `Formulas` and the readers'
problems; `Send + Sync` for the public `Walk`; `#[non_exhaustive]` on
the three variants of `Reason` that lack it; `race` reads its thread
count from the options; `Derivation::sequent` returns a `Sequent`; calls
that only read borrow (group B8 of `findings.md`).

### P22. Examples

210 of 242 structs and 963 of 985 methods have no example.
**Recommended**: a runnable example on every type a user starts from,
and no rule for the rest: each suite's `Sequent`, `Formulas`, `Options`,
`Outcome` and `Interactive`; `Proof`, `Disproof`, the three
`Derivation`s; `kripke::Model`; `lambda::Term`; `rules::Calculus`;
`Workspace`; `Limits`, `Settings`; each export's `write`; `Decide`.

## 8. The crate and its features

### P23. One crate

No split into crates for 0.9.0. The architecture reader and its
verifier agree with step 67's note: a split would shorten no user's
build that a feature could not, would put two or more new names on
crates.io, and would need `Error` and the other sums divided first. The
layering of P3 is what a later split would need anyway.

### P24. The features (choice)

- **Recommended**: one feature `export` in place of `latex`, `typst`
  and `svg`, with `png` and `pdf` on top (no consumer enables them
  apart; 258 `cfg` sites; the feature checks build 45 pairs in place of
  66). F-arch-22.
- The JSON feature, `serialize`, also gates the rule files' checker and
  modal logic's decision procedure. The recorded reason (a calculus
  stores its serde form) has lapsed but for the built-in rule files
  that are read from JSON: the modal suite builds its calculus's form
  with no deserializer (the verifiers of F-6a-04 and F-arch-16).
  **Recommended**: take the rule forms out from behind the feature, so
  that it adds the JSON forms and nothing else, and then name it
  `serde` (C-FEATURE, F-6a-17). The rename alone, with the gating kept,
  is not worth its break: if the first is declined, the name stays
  `serialize` and its docs say what it gates.
- **Asked**: a feature that gives the data model and the checkers
  without the engines (X7, F-arch-17, F-6a-04). It is not a feature per
  suite, which the decision of 2026-11-09 rules out, but it is the same
  kind of cost: `cfg` sites on every engine, and `interactive`,
  `protocol` and the rule files' session would each imply it. I
  recommend to measure first (how much of the 16 s that a build without
  default features takes are the engines) and to decide on the number,
  after 0.9.0 if it is small.

## 9. The web module and the client

### P25. The web module's size, in an order

The module is 2 987 284 bytes at gzip, 1 % under its ceiling, and grew
3.5 % in four steps. Measured at this commit
(`inventories/wasm-opt.md`, after `wasm-opt`): the linear suite is 19 %
of it, ordinary logic 14 %, the serde code 12 %, the protocol 8 %; and
20 % is second and later copies of one generic function (the sorts 66
times, serde's buffered `Content` 40 times, the protocol's `document`
13 times). **Recommended**, each measured before the next:

1. the stop shims the crate's own rule asks for, and one `never` stop
   in the bindings in place of five (51 functions compiled twice);
2. serde forms that do not buffer through `Content` for `lambda::Node`
   and `lambda::Type`, a third of that suite's bytes (F-5a-07);
3. the registry as a value (P14), built by the bindings from the page's
   list, so that the workspace links only the operations the page uses:
   the earlier measurement put the workspace's reach at about 1.35 MB
   raw;
4. a module per suite only if the size after these still asks for it:
   the suites contain each other (ordinary logic links the linear
   search), so two suites side by side could weigh more than today.

**What each part costs the bindings and the client**: P3 and P4 change
no call. P5 changes the settings' keys once. P11 and P12 change the
outcomes' and errors' JSON once. P14 changes the session calls' names
once and removes the bindings' own table. All of it is one wire level
and one step of the client, made in the pair of changes that moves the
client's pinned module.

## 10. The Rocq library's interface

- The names deduxit's exporter writes into certificates (the theorems,
  `weight_of`, `terms_eqb`, the record fields, the formula
  constructors) are listed as the library's stable surface, in a README
  section and a test `tests/Interface.v` that states each with its type
  without printing (F-9-01, F-4d-08). A rename is then a break both
  repositories see.
- Renamings inside the library (F-9-17: one name and shape per notion
  across `Linear`, `Intuitionistic` and `Modal`; `Deduxit.Base` for
  what two namespaces share) keep every statement a certificate uses,
  or move the exporter in the same pair of changes, with the
  changelog's list.
- The opam file bounds `rocq-stdlib` above, and CI installs the package
  from its opam file on each Rocq it admits (F-9-12, F-9-03).
- No trusted statement changes.

## 11. What it costs a user, and how it is shown

- **One wire level** for every change of a JSON form (P5, P11, P12,
  P13's value names, P14), bumped once, with `wire::upgrade` reading the
  0.8.0 forms as it reads older ones.
- **The changelog** carries a migration table for library users (old
  path or name, new one), built from P3, P13 and P20, and one line per
  command for P1, P16 and P17.
- **`breaks 0.8.0`** (cargo-semver-checks, then cargo-public-api's diff)
  after each refactor step; nothing public changes that the proposal
  does not name.
- **Behaviour is kept** everywhere but in the fixes of group A: the
  target sets equal their oracles at the refactor's base
  (`baselines.md`), and every step compares against them.

## 12. The recorded decisions, answered

| decision (`standing.md`) | this proposal |
|---|---|
| one crate, a module per suite, no feature per suite (2026-11-09) | kept (P23); P24 asks about a feature that is not per suite, and recommends a measurement first |
| linear logic is the command's default through 0.x (2026-11-09) | replaced by D1 (P1, P2) |
| verbs come first and `--logic` chooses the suite; no subcommand per suite (2026-11-09) | verbs first and no subcommand kept; the recommendation of P1 names the selector per suite (`--ordinary`, `--linear`, `--modal`, `--calculus`) where the decision has one `--logic`, a departure in the flag's name that P1 puts as a choice |
| the kernel crate ended at its gate (step 67) | kept; P3 makes the moves that note lists as due, and the public ones D2 and D4 now allow |
| an item without a caller waits for one (`core.md`) | the caller is the manual's page (P10) and the consumer test |
| the checked builder's recorded design, R59 | its substance kept, its shape changed to the one two suites have (P6); asked |
| a front end never builds an error of the library | kept; the refusal constructors are for engines (P7) |
| every long call takes `&Limits` and `stop: impl FnMut(Progress) -> bool`, no `Stop` trait | kept for every call a user makes, `prove_with` included, and eleven calls are brought under it (group A2). One departure, on the engine's side: `Decide::decide` is given a stop that takes work units, as the crate's private engine trait is today, because the front door owns the work count and makes the `Progress` (F-2a-02) |
| the linear outcome's fields are public | changed to private with accessors, so that the checker stays the trust boundary beside an outside engine (P8) |
| `Suite` is not sealed, on purpose | kept; the registry becomes a value (P14) |
| `serialize` gates the rule files because a calculus stores its serde form | asked to change: the reason has lapsed but for the built-in rule files read from JSON (P24) |
| a module per suite gains little (54x6); the registry built from the page's list was left to the author | P25 takes the registry as its third step and a module per suite last |
| the Rocq library's first-order files leave the propositional ones untouched | asked to change for the checkers' shared code, outside the trusted statements (`findings.md`, H3); not part of the public interface |
| the Rocq library keeps `Notation` for abbreviations because `Abbreviation` exists only from Rocq 9.2 | not part of the interface; `findings.md` (H2, F-9-04) proposes to write the applications out, which works on all four versions, or to silence the one warning by name; the author's choice in step 87r3 |
| the Rocq library's installation from its opam file is not a flake check, because it needs the network | kept; section 10 asks for a job of the library's CI, which has the network |
| the batch's record is assembled as text; `GlivenkoFirstOrder`'s message carries its countermodel | kept (both findings were rejected for it) |

## 13. What is not proposed

- A crate per suite, or a kernel crate (P23).
- A generic goal state for the three sessions, one `Verdict` type for
  every suite, a trait over the checkers, the two structures of nets
  merged into one (P15's `Net` is a sum of the two, which stay): the
  architecture reader judged each to cost more than it saves
  (`units/arch/architecture.md`, section 4).
- Renaming the linear suite's `Term` to `Node` (P13).
- A public `Drawn` (P9).
- Any change of an exit status (D3), of a trusted statement of the Rocq
  library, or of what a checker accepts beyond the fix of F-4d-01.
