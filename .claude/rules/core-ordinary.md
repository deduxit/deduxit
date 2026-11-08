---
paths:
  - "core/src/ordinary/**"
---

# linlog core: ordinary logic through its embeddings

Loaded, beside `core.md`, when a file of `core/src/ordinary/` is read or changed:
classical, intuitionistic and minimal propositional logic decided by
translating into linear logic, and the linear proof read back as LK or LJ.

- **Layout.** `mod.rs`: `Logic`, `Translation` (`Affine`, `CallByName`,
  `CallByValue`, `ZeroOne`; short names `affine`, `cbn`, `cbv`, `01`),
  `Options`, the hash-consed arena `Formulas` (`Node`, `NodeId`; equal
  formulas are one id, which the checker's multiset comparisons rely on),
  `Sequent` (two-sided, its syntax documented on it) and the printers;
  `parse.rs` (behind `parse`): the precedence parser in two dialects,
  this crate's (`->`, `/\`, `\/`, `~`, `<->`, `true`, `false` and their
  Unicode symbols) and TPTP's `fof` (`read_tptp`, through
  `lltp::clauses`, which `lltp::read` shares); `translate.rs`: the
  pattern table, `translate`, `Image`, `Target` (`Translation::target`:
  MALL in classical affine mode or LL in intuitionistic mode, whose
  `Display` the command prints, `affine MALL`, `ILL`); `derivation.rs`:
  `Rule`, `Inference` (private fields, accessors), `Derivation`, the
  read-back (`Image::read_back`) and the checker (`Derivation::check`);
  `decide.rs`: `decide`, translation, search, read-back and check in one
  call, answering an `Outcome` (`Verdict::Valid` with the derivation,
  `NotValid`, `Unknown`, and the linear outcome); `rocq.rs` (behind
  `rocq`): the certificate over `Prop`, public as
  `export::rocq::ordinary`; `interactive.rs` (behind `interactive`): the
  goal state `Interactive`, below. Its JSON forms and those of the
  sequent, the derivation and the outcome are in
  `core/src/serialize/ordinary.rs`, wire level 8 (`wire::ORDINARY`).
- **Every entry keeps to the occurrence bound**: `Sequent::parse_within`
  and `read_tptp` refuse at the first formula of the arena past
  `limits.occurrences`, and at the formula of the sequent past it,
  since the arena interns a repeated formula and each repeat is an
  occurrence of the image all the same (`FromStr` within the
  defaults), and at the operator past it of those waiting for their
  operand (`Parser::wait`: they lie on one branch, so each will be a
  formula of its own; a run of `~` or of `->` piled up 100 bytes per
  byte of text before), and at the open parenthesis past it, which
  makes no formula but would pile up as well, and `translate`
  refuses an image that unfolds past it, before a forest is laid out
  (`names_read_back_and_bounds_hold`). The command reads, parses and
  translates within its `--occurrence-limit`. `Logic` and
  `Translation` have `ALL`, `NAMES` and `FromStr` (`Error::UnknownName`).
- **The arena refuses what is not its own**: `Formulas::add` refuses an
  operand that is no node of the arena (an id of another one) and an
  atom outside its names, and `Sequent::new` an id outside its arena
  (`Error::IndexOutOfBounds`, `Space::Formula` or `Space::Atom`), so
  that no later index panics; a full arena is `Refusal::Index`.
- **The ordinary syntax refuses the linear symbols** (`&`, `|`, `*`,
  `-o`): later.md's requirement that nobody writes `&` and gets the
  wrong connective. TPTP's `&` and `|` are read only by `read_tptp`.
- **One table is both the translation and the read-back's map**
  (`translate::pattern`): per node and translation function (classical:
  the side, `RIGHT`/`LEFT`; 0/1: `T0`/`T1`; the others one), the `!`s on
  top, the linear connective, and each operand with its function and
  added `!`s. `translate` builds every image bottom-up from it (both the
  image and its dual, so a hypothesis of ILL is the dual), and the
  read-back walks the forest of the image in preorder with the same table
  to tag every occurrence with the ordinary formula, function, `!`s left
  and side. A change to a translation is a change to the table alone;
  never build an image any other way.
- **The 0/1 translation is LLTP's** (`tptpparser/translation.ml`: `t₀`
  for hypotheses under `!`, `t₁` for the goal, `¬A` as `A → false`,
  `A ↔ B` as `(A → B) ∧ (B → A)`), so that this crate's images equal
  LLTP's translated files wherever LLTP's parser read the problem right
  (its `~` takes the widest scope, `plan/notes/lltp-headers.md`), and
  `cbn` writes true as `⊤` where LLTP wrote the atom `T`.
- **Extra nodes**: `translate` adds to its copy of the arena the
  implications `A → B` and `B → A` of every `A ↔ B` and the node false,
  which `¬A` is `A → false` of in ILL; `false` is made first in the build
  order, since a negation anywhere refers to it. The derivation's arena is
  that copy, so its rules name those nodes (`↔R`'s premises, the second
  premise of LJ's `¬L`).
- **The roots are matched by position.** The linear `Sequent` keeps its
  roots in the order `translate` writes them (the hypotheses' images,
  then the formulas right of `⊢`), so `Image::roots[i]` is the linear
  root `i`; `antecedents` is the number of hypotheses in ILL, whose images
  are duals, and `0` classically, where every image stands right of `⊢`.
- **Minimal logic**: false is the atom `FALSE` (`false`, which no
  ordinary atom can be named: `Formulas::atom` refuses the keywords of
  both syntaxes, so a TPTP problem naming `false` is `Error::AtomName`,
  `false_is_no_atom_of_the_sequent`), translated as any atom is, its
  `!` under cbv included (without it a hypothesis `false` was linear,
  and `false |- ~true` was refuted under cbv alone; the fresh-context
  review's differential against G4ip with `⊥` an atom found it), so no
  `0` exists and `⊥L` cannot arise; the checker
  refuses `⊥L` in minimal logic anyway. An intuitionistic or minimal
  sequent with nothing right of `⊢` is decided as `Γ ⊢ ⊥`, and `Image::ordinary`
  is that sequent; more than one formula right is `Error::Succedents`.
- **The read-back takes the proof, not a derivation** (`read_back(&proof,
  &limits, stop)`), and a proof of the roots only (a goal proof is
  `Error::GoalProof`, as for `from_proof` and the Rocq writer): it
  unfolds the proof itself, never compact, within
  the limits, so a compact derivation (a run of weakenings as one
  starred inference, which the probe found read as one `WL` that fails
  the checker, H22) cannot reach it; it and the checker ask the stop
  once per inference, and the checker checks `limits.work` inferences
  at most. It reads the linear derivation that
  `Image::linear_derivation` (crate-private) builds (two-sided for ILL, which keeps the
  goal on the premise without absorbed hypotheses at a `⊗`; one-sided
  for affine MALL; never compact) inference by inference in its order,
  mapping each to zero or more ordinary inferences: `!`/`?d` none (the
  ordinary sequents coincide), `?c` `CL`, `?w`/`wk` `WL`/`WR`, a
  connective rule the rule of the ordinary connective under its tag
  (`rule_of`, by node, side and the classical rule name), and below it
  the `¬` rules a classical negation needs, since it has no image of its
  own (`unwind`). LJ's `→L` is Gentzen's, context split; `∧R` and `∨L`
  share it under cbn and 01, split it under cbv (`⊗`).
- **The checker trusts nothing of the read-back**: per inference, the
  rule against the principal formula (a position given), the premises as
  multisets either sharing the context or splitting it (G1c/G1i/G1m of
  Troelstra and Schwichtenberg with additive variants, all derivable),
  at most one formula right in LJ, exactly one in minimal logic (G1m: an
  empty right side is `⊥` to LK's `¬L` and anything to `WR` or a split
  `∨L`, which together are ex falso; a review forged `a, ¬a ⊢ b` that way,
  `minimal_logic_refuses_an_empty_right_side` pins it), no `⊥L` in minimal logic, the root
  concluding the image's ordinary sequent. `the_checker_refuses_each_break`
  breaks a read-back derivation once per guard and asserts that guard's
  reason, so a guard removed shows (F11: `check_one` replaced by
  `Ok(())` survived the suite). `¬L` and `¬R` take LK's
  forms (one premise; `Γ, A ⊢` for `¬R`) in every logic and that of
  `A → ⊥` in LJ only: the classical certificate reads LK's forms alone,
  and a classical derivation document in the other form made the Rocq
  writer panic before the checker refused it (the goal state's review). It has
  no counter that can wrap: it compares sorted vectors.
- **The certificate** (`rocq.rs`) is a term, not a tactic script: an LJ
  sequent `Γ ⊢ C` is a term of type `C` (`False` for none), an LK sequent
  a term of `False` with a continuation `k : ~ D` per formula `D` right,
  and the rules that move a formula right use `NNPP` (standalone files
  import `Stdlib`'s `Classical_Prop`). Variables are `h'n`/`k'n`, which no
  atom identifier contains; atoms go through `export::rocq::identifiers`
  plus the names the terms use (`USED`). Every `match` has a `return`,
  and `↔` goes through a cast to the conjunction it unfolds to; a
  statement without atoms or hypotheses binds nothing (`exact (I)`, no
  `fun =>`, H29). A
  negation is bracketed like a binary formula, since it is an
  application. Minimal logic's `⊥` is Rocq's `False`, so a certificate
  of minimal logic proves the intuitionistic statement only (minimality
  rests on the checker).
- **Deciding** is `decide(&sequent, &ordinary::Options, &search::Options,
  &limits, stop)`, which the ordinary journey runs; the command still
  runs the steps itself (its race and its output), with its defaults
  (no copy bound, the time limit), affine classical mode for `Affine`,
  intuitionistic mode otherwise. A proof whose read-back a bound or the
  stop refuses is that refusal from `decide`, never a verdict. The
  default translation for intuitionistic and minimal logic is
  `Translation::DEFAULT_INTUITIONISTIC`, chosen by the ILTP run
  (`plan/reports/25-ordinary-logic.md`).
- **Where quantifiers go**: `Node` gains `Forall`/`Exists` with a bound
  variable and `Atom` gains terms; the pattern table gets their rows
  (Girard: `∀` as `∀`, `∃` as `∃!`), and the read-back's tags carry no
  more than the node.

## The goal state (`interactive.rs`)

A native state of ordinary sequents, not a session over the image: the
goals are two-sided sequents, the rules `Rule`'s, and a finished state
is a `Derivation` that `Derivation::check` validates, so nothing is
added to what is trusted. The linear session (`core-derivations.md`,
"Interactive proving") is its model; what was copied is listed in
`plan/reports/49-ordinary-interactive.md`, and nothing generic is
factored out of the two until the kernel crate is.

- **Every inference a step makes passes `check_inference` before the
  state changes** (`expand`): the checker's own per-inference check
  (the logic's right-side counts, then the rule), factored out of
  `Derivation::check` so that both read one function. A step the
  construction got wrong is an `unreachable!` panic, never a state the
  final check would refuse; `random_steps_finish_as_checked_derivations`
  walks random steps, splits and undos over generated sequents in
  every logic and finishes every state the search can close.
- **`shape` is the one rule table**: which rule acts on which
  connective and side, which the logic refuses (`WR`, `CR`, `⊥R` and
  `∨R` with both disjuncts outside LK, `⊥L` in minimal logic), and what
  each premise holds in the principal's place and on the other side.
  `rules` lists exactly the rules `shape` accepts (plus the axiom where
  the formula stands on both sides), so the list and `apply` cannot
  disagree. A premise keeps the goal's order, the principal's place
  taken by its subformulas, a formula crossing `⊢` appended.
- **In LJ and minimal logic every goal has exactly one formula right of
  `⊢`**: `Γ ⊢` starts as `Γ ⊢ ⊥` (as `translate` decides it, so the
  search's derivation of a goal concludes the goal itself), the rules
  whose premise would have none or two are refused, and `¬R`/`¬L` take
  the form of `→R`/`→L` for `A → ⊥`; classically they are LK's
  one-premise forms. The reader refuses an open goal of LJ without one
  formula right of `⊢`.
- **The axiom closes a goal with a context** by a run of weakenings
  below `A ⊢ A` (the last formula left of `⊢` first, then right), one
  step that `undo` takes back whole. A rule with two premises shares the
  context unless the step splits it (`Context::Split`, positions per
  side going to the first premise). The checker accepts a binary rule
  either sharing or splitting, never both at once, so LJ's `→L` and `¬L`
  (their first premise has its own formula right of `⊢`) share by
  `contracted`: a `CL` of every other hypothesis below, each copy right
  after its formula, then the rule split, first copies to the first
  premise, second copies and the goal's formula right of `⊢` to the
  second; the admissible view (the default, `DEFAULT_STRUCTURAL`) leaves
  the contractions out and shows G3's `→L` with the context in both
  premises (the supervisor's correction of the first design, which made
  a first course split by hand). The principal is not kept in the
  premises: a hypothesis used twice is contracted by hand.
- **The arena holds the extra nodes from the start** (`Extra::add`:
  false and both implications of every `↔`), so `shape` finds them by
  `Formulas::ids`, and the derivation `decide` returns for a goal is
  over an equal arena; `ids_of` maps its nodes anyway, adding any the
  state lacks.
- **`close` and `hint` decide the goal as a sequent of its own**
  (`ordinary::decide` with the state's logic): `close` grafts the
  derivation as one step (root in the goal's place keeping the goal's
  order, its principal found by formula; the rest appended in reverse),
  refused past `derivation_bytes` by `estimate`; `hint` reads the root's
  rule, its formula and, for two premises, whether they share the
  context or which positions go to the first (`first_step`), as a
  `Step` that `apply` takes. A refutation or an unknown leaves the goal
  open with the outcome saying why; a stop is `Unknown(Stopped)`, a
  refused read-back `decide`'s error.
- **The structural rules as a display choice** (`ViewOptions::structural`,
  `Structural::{Explicit, Admissible}`, admissible by default): `Derivation::shown`/`admissible`
  leaves out every `WL`, `WR`, `CL`, `CR` whose branch the rules above
  close, giving its conclusion to the first rule above that is no
  structural one (principal found by formula), which reads as G3 (the
  axiom with a context, a contraction absorbed into the rule that uses
  both copies). A run above an open goal stays, so an open goal shows
  its own sequent. What is shown admissible fails the checker by
  design; `finished` always checks the explicit derivation.
  `derivation_ids(structural)` maps a drawn inference to the state's
  goal, for both views.
- **Bounds**: `within` and every reader refuse a sequent past
  `limits.occurrences` (subformula occurrences, saturating), the readers
  also an arena longer than it; an axiom's weakenings and a shared
  rule's contractions hold the square of the goal's size, so `apply`
  refuses such a run past the `derivation_bytes` the state was started
  or read under (`limit`, not serialized; `run_within`'s upper estimate,
  `StepError::Size`) before it is made (the second review measured 2 000
  hypotheses at 6 million ids, unbounded before); `derivation_within` and the graft refuse
  past `derivation_bytes` by `estimate` (the size estimate's constants,
  per formula its characters with brackets and two for its separator)
  before anything is built; the stop is asked by `derivation_within`,
  by `decide` inside `close`/`hint` and by the check in `finished`. Every
  walk is a loop (`postorder`, `subtree`, the reader's history walk),
  and the arena-order passes (`occurrences`, `estimate`, `ids_of`) need
  no recursion since every operand precedes its formula.
- **Reading a session** (`Interactive::from_parts`): the root first,
  every other inference the premise of exactly one before it, an open
  goal without rule, principal or premises, every closed inference
  passing `check_inference` with its premises, and the history walked
  from the last step back as the linear reader walks it (each entry a
  distinct closed inference below the arena's end then, what its step
  added the suffix of that arena), so a state read is as sound as one
  built. The derivation's reader checks only its shape (a tree, its root
  last, indices in range), so that a drawing of any document is safe;
  the Rocq writer checks a derivation before it writes one and refuses
  an open goal (`Unsupported::Open`).
- **What the review found** (all fixed, each pinned in
  `a_forged_document_is_refused`): a session document whose history
  closed an LJ goal without one formula right of `⊢` let `undo` reopen
  it, and `close` then grafted the search's `Γ ⊢ ⊥` onto `Γ ⊢`; now the
  reader refuses such a step and `graft` refuses a root that does not
  conclude the goal (`Error::GoalMismatch`). The admissible view hides a
  structural rule only where its conclusion holds the principal of the
  rule above; the derivation reader takes an open goal only as a leaf
  (the text layout's height underflowed under a bare goal with a
  premise).
- **`Rule` reads from text** (`FromStr`, `derivation.rs`): its name, a
  subscript as a digit (`∧L1`), and the ASCII spellings of `ASCII`, the
  connectives as the parser reads them (`/\R`, `->L`, `~R`, `<->L1`) and
  as words (`andR`, `impL`, `notR`, `iffL1`, `falseL`, `trueR`), the
  word forms because the command's word splitter takes `\` as an
  escape. `open` is refused: no step takes it. The serde form stays the
  name alone.
- **`Rule::Open`** marks an open goal of a proof in progress and appears
  nowhere else: the checker refuses it first ("an open goal"), `Drawn`
  draws it without a bar or label, as the linear `Rule::Open`.

## Decisions

The author's answers for the release (`plan/notes/api.md` §14), which
the fixes implement and later rounds judge against. Where a bullet above
still describes code that a decision changes, the decision holds, and
the commit that lands it rewrites that bullet.

- **A TPTP file with several conjectures is refused**, naming the
  second, as the LLTP reader refuses one (`core-inputs.md`).
