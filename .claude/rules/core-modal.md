---
paths:
  - "core/src/modal/**"
  - "core/src/serialize/modal.rs"
---

# deduxit core: modal logic

Loaded, beside `core.md`, when a file of `core/src/modal/` or its serde
is read or changed: propositional modal logic, the systems from K to S5.
The design, with its reasons, alternatives and sources, is
`plan/notes/modal-design.md`; this file holds what the code relies on.

- **Layout.** `mod.rs`: the hash-consed arena `Formulas` (`Node`,
  `NodeId`; equal formulas are one id), `Sequent` (two-sided, its syntax
  and JSON documented on it) and the printer; `system.rs`: `Condition`
  and `System`; `parse.rs` (behind `parse`): the precedence parser;
  `model.rs`: the check of a Kripke model (`Model::frame_fault`,
  `Model::truth`, `Truth`, `Frame`, `Countermodel`, `CHECKED`), in every
  build; behind `serialize` (the generic derivation needs it):
  `calculus.rs` (`calculus(system)`, the labelled rule file built in
  code, `calculus::Rule`), `search.rs` (the tableau), `decide.rs`
  (`decide`, `decide_within`, `Options`, `Outcome`, `Verdict`,
  `Evidence`, `Statistics`, and the derivation read off a closed
  tableau); `search_tests.rs` the search's tests. The JSON forms are
  `core/src/serialize/modal.rs`: the sequent, the system and the
  outcome.
- **A sequent is read at one world** (local consequence): `Γ ⊢ Δ` is
  valid exactly when `⋀Γ → ⋁Δ` is. Hypotheses true at every world are
  another relation, which nothing here reads.
- **`□` and `◇` are both nodes**, so a formula prints as written.
  `Node::is_modal` is the one test of them; the JSON level reads it.
- **A system is a closed set of conditions** (`system::closed`): it
  holds every condition that follows on every frame from the ones it
  was made of, so `==` on systems is equality of their frame classes
  and the 32 sets are the fifteen systems of `ALL`. Every constructor
  goes through `closed` (`with`, `of`, the constants); a `System` made
  any other way breaks `name` (which then answers the last name) and
  `==`. `a_system_holds_what_follows_on_every_frame` checks the five
  implications against every relation on up to three worlds; a sixth
  condition needs its implications there and its systems in `ALL` and
  `NAMES`, which stay parallel.
- **Names**: `name` writes one per system; `FromStr` reads those in
  either case, `K` with the axioms' letters in any order, and a leading
  `D` as `KD`. A lone `T` or `B` is the system, so `T4` is no name.
  In a comment or the manual write the serial systems with their `K`
  (`KD45`): the `conventions` check reads a `D` followed by digits as
  a plan decision's number and fails.
- **The syntax is the ordinary one with two prefixes** (`[]`/`□`,
  `<>`/`◇`), and an atom's name passes `name::check_ordinary`, so a
  formula without modalities is one text in both suites. `<>` is read
  only where a formula starts and `<->` only after one. `!` and `?`,
  another prover's box and diamond, are refused: they are linear
  logic's.
- **Every entry keeps to the occurrence bound**, as the ordinary
  readers do: the arena's formulas, the sequent's formulas, the
  operators and parentheses that wait (`Parser::wait`), and the
  sequent's unfolding (`bound`, which the text reader calls too, where
  the ordinary one leaves it to `translate`).
- **Nothing recurses**: the parser has its own stack, the printer walks
  `sequents::Stops`; `deep_modal_formulas_need_no_stack` in
  `core/tests/depth.rs` pins both.
- **The JSON level is read from the content** (`serialize::modal::level`):
  a sequent whose arena holds neither `□` nor `◇` is written at
  `wire::ORDINARY` with the bytes of the ordinary sequent it is, one
  with either at `wire::MODAL`; an ordinary sequent's document reads as
  a modal one. The level is read from the whole arena, not from what
  the two sides reach: the form writes every node, and a reader of
  level 8 refuses the tag `□`.
  The reader maps the document's indices to the arena it builds, so a
  formula written twice is one node.
- **A modal model's arrows are the relation itself** (`model.rs`):
  nothing is closed when a model is read or checked, so a model of S4
  lists its loops, and `frame_fault` names the first condition the
  arrows as written fail, in `CHECKED`'s order (reflexive before
  serial, so a world without arrows in T is told it does not see
  itself), with the worlds that show it. The ordinary suite reads the
  same key as an order's generating step; the two readings never meet.
  `falsity` is refused: a modal world is classical. A model that is no
  model of the system is `Error::Model` with `Frame`'s text; the
  condition as data is `frame_fault`'s answer (not an error variant of
  its own, which would touch the shared error family).
- **The check's bounds**: `truth` refuses past `ordinary::MODEL_CELLS`
  cells or `limits.memory_bytes` before it allocates (as the ordinary
  check), and both passes count work (`Meter`: a world, its atoms, its
  arrows, and for transitivity or euclideanness the arrows of each
  world seen) against `limits.work`, asking the stop every 4 096 units.
- **The calculus** (`calculus.rs`): Negri's G3K over items `x : A` and
  `x R y` (sorts `label` and `item`), two set zones, every principal
  formula kept in its premises (so that a tableau branch, which keeps
  what it expands, is a derivation as it is), the axiom on any formula
  at one label, `↔` rules of our own, and the frame rule of every
  condition the system has (implied ones too). Its name is
  `calculus::name`, a table parallel to `System::ALL`. The order of the
  members a node gives by position follows the rule compiler
  (`rules::calculus::Compiler::rule`): the conclusion's written
  members, left zone then right, in the rule's order, then the first
  premise's member with a variable not bound yet (the new edge of `R□`,
  `L◇`, `Ser`, and `Ref`'s `x R x`). `Step::written` and
  `Step::located` are built in that order; a change of a rule's
  written order is a change of both.
- **The search** (`search.rs`): one branch is worlds (two bit sets over
  the arena each, sorted `sees` and `seen_by`), an agenda of members
  added and not yet acted on, the formulas waiting for a split, the
  `□` false and `◇` true waiting for a witness, and the segment of
  steps since the last split. The relation is kept closed under the
  system's frame rules as each edge is acted on (each derived edge a
  step), so a box reaches a world by `L□` along a derived edge. Order:
  the agenda to a fixpoint (closures first, then one-premise rules and
  propagation), then a split (the oldest world), then a witness (the
  oldest world not blocked), then `Ser` in a serial system without
  reflexivity. A split clones the branch for the second premise.
- **Blocking only where the relation is transitive and neither
  symmetric nor euclidean** (K4, KD4, S4): a world is blocked when a
  proper ancestor's signature (its formulas, with every `□` true and `◇`
  false at a world that sees it) includes its own. In those systems no
  edge leads back to an older world, so when a world is tested (after
  the agenda and every split) its ancestors' signatures and its own are
  final. The open branch's model gives a blocked world that waits
  (a demand without a witness, or a dead end in a serial system) its
  blocker's arrows, then closes the relation under the system's
  conditions, then gives a dead end a loop in a serial system.
- **A step is kept only where the proof above uses it** (`fold`,
  `join`): a closed part carries the members of its conclusion its
  proof uses; a step whose *new* members (those not on the branch
  before it) none of them uses is left out, and a split whose first
  premise's proof uses none of its new members stands for the
  conclusion, its second premise never searched. Emission
  (`decide::derivation`) adds each kept step's whole premise list (not
  only the new members), so a member another, left-out step put on the
  branch first is still in the premise the rule writes out. Every
  member a kept step writes out in its conclusion is in the needed set
  below it, so its adder is kept.
- **The derivation's size is estimated before it is made** (each
  node's members are the root's and its path's additions, an upper
  bound), against `limits.derivation_bytes`; past it `Refusal::Output`,
  never a verdict lost. A derivation the checker refuses is
  `Error::ReadBack` (calculus `G3…`, a defect), a model the check
  refuses `Error::Countermodel`.
- **The outcome's JSON** (`serialize/modal.rs`, level `wire::MODAL`):
  `{version, deduxit, verdict, reason?, system, statistics, sequent,
  derivation?, countermodel?: {model}}`, the derivation the rules
  derivation's form without its version (`serialize::rules::
  DerivationForm`, crate-visible for it), the model `ModelForm`. It
  reads back unchecked; a verdict without its evidence, or with
  another's, is `Error::Json`.
- **What it copied** (for the kernel crate): from `ordinary`, the arena
  (`Formulas`, `Node`, `NodeId`, `add`, `atom`, `holds`, `occurrences`,
  `bound`), `Sequent`, the parser but for its TPTP dialect, and the
  sequent's serde form and reader, the model check's shape and bounds
  from `ordinary/model.rs` (the cell and byte estimates, its errors),
  `found` from `ordinary::Countermodel`, and the poll cadence and
  memory account of LJT (`linear::search::memory::Account` as it is,
  `Reason` as it is); it uses `linear::sequents::{Stop, Stops}`,
  `linear::sequents::name`, `rules::{Calculus, Derivation}` and the
  rule files' forms as they are.
- **Where quantifiers and agents go**: `Node` gains `Forall` and
  `Exists` and `Atom` terms, as the ordinary arena will; an agent is a
  second `u32` in `Box` and `Diamond`, which keeps the node at 12
  bytes.
