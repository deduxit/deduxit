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
  tableau), `rocq.rs` (behind `rocq` too: the certificates for the Rocq
  library's `Deduxit.Modal`, `core-export.md`); `search_tests.rs` the
  search's tests. The JSON forms are
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
- **Sessions in the calculus** are `rules::Interactive` on
  `calculus(system)` (`interact --calculus G3S4`): backwards as any
  calculus given as data, forwards by `Interactive::extend` for the
  rules whose premise only adds before `⊢` (`L∧`, `L□`, `L◇`, the frame
  rules), `core-rules.md`. A label a step makes is named after the
  rule's variable (`y`, `y1`); a refused step may still have made the
  name, so the next is numbered on.
- **The search** (`search.rs`): one branch is worlds (two bit sets over
  the arena each, sorted `sees` and `seen_by`), an agenda of members
  added and not yet acted on, the formulas waiting for a split, the
  `□` false and `◇` true waiting for a witness, and the segment of
  steps since the last split. The relation is kept closed under the
  system's frame rules as each edge is acted on (each derived edge a
  step), so a box reaches a world by `L□` along a derived edge. Order:
  the agenda to a fixpoint (closures first, then one-premise rules and
  propagation), then a split (the oldest world), then a witness (the
  oldest world not blocked; in K4, KD4 and S4 the walk in depth below),
  then `Ser` in a serial system without reflexivity. A split clones the branch for the second premise. The
  pending formulas keep their order (`remove`, never `swap_remove`): a
  swap once put the last diamond first and changed which world was made
  next.
- **Every scan is work** (`Search::owed`): a scan over a world's
  arrows, a bit set, the pending formulas or an earlier world's signature
  adds its length to `owed`, which the next `tick` counts against
  `limits.work` and the poll; uncounted, a world of 200 000 formulas
  made each edge cost a pass over all of them for one unit (the
  integers panel of 65b). The step and proof-node indices are `u32`:
  past them `full` is set and the next tick answers `IndexLimit`.
- **Blocking only where the relation is transitive and neither
  symmetric nor euclidean** (K4, KD4, S4): a world is blocked by the
  first earlier world that no world blocks and whose signature (its
  formulas, with every `□` true and `◇` false at a world that sees it)
  includes its own; signature and blocker are kept on the world once
  found (`World::signature`, `World::blocked`), each blocker found
  after every earlier world's. In those systems no edge leads back to
  an older world and formulas reach only worlds made later, so when a
  world is first tested (only where nothing is left but to make a
  world) every existing world's signature is final, and stays so on
  this branch and its clones. Ancestors only (the design's, from Garg,
  Genovese and Negri) left siblings of equal content each expanding:
  the counterexamples panel of 65b found a 14-node sequent of KD4 at
  40 million units, now 105 000. The open branch's model gives a
  serial system's dead end that no world blocks a loop first, then a
  blocked world that waits (a demand without a witness, or a dead end)
  its blocker's arrows as the model has them (so never none), closes
  those under transitivity by a worklist (the
  branch's relation is closed already, and the blocking systems are
  neither symmetric nor euclidean), each new pair charged. A closure
  of the whole relation by rounds built every 2-path, n³/6 pairs uncharged: 7 to 9 times the
  memory bound (the integers panel of 65b).
- **In K4, KD4 and S4 the worlds are made in depth** (`Search::deepest`,
  `Branch::open`, a stack): the newest unfinished world makes a world
  for its first demand, or by `Ser`, unless it is blocked, and the new
  world acts next; a world that makes none is popped, finished, and its
  part of the branch is it and every world made after it. A walk that
  made all of a world's witnesses before entering any of them did worse
  on the measured sets (fixup-10's report). The other twelve systems
  keep the oldest-first order unchanged, and no cache: their verdicts,
  units and counters are as before.
- **A world's key** (`Search::keyed`): its own formula (operand and
  side, or 0 for `Ser`) and every `□` true and `◇` false at its maker
  `x` and the worlds that see `x`, computed when the world is made. In
  these systems formulas reach only worlds made later, so the key alone
  decides the world's part of the branch. Keys are matched exactly (a
  hash map), never by inclusion.
- **The sat side of the cache** (`Search::keep`, `Entry`, `Piece`):
  when a world is popped and its key has no entry, its part is kept as
  an entry where every world of it that waits (a demand without a
  witness, or a dead end where `Ser` applies) is blocked by a world of
  the part; a dead end where `Ser` does not apply gets a loop, a waiting
  world its blocker's arrows (as `model` does). An entry holds direct
  arrows only (maker to made, the loops of S4 and of a dead end), which
  the model's worklist closes into the branch's relation, and a world of
  the part whose own part was kept on this branch (`World::kept`) stands
  as that entry, its other worlds skipped (`World::end`): copying every
  part's closed arrows cost n³ on a K4 chain, paid in one tick after the
  walk (the integers panel of fixup-10). Never another entry of the same
  key there: the descendants a part had may realise its ancestors'
  demands, which another model of the key need not. The walk ticks after
  every popped world. A world made later with
  that key is `Blocked::Cached`: never expanded (no betas, no demands,
  no `Ser`), never a blocker, and `model` makes it the entry's first
  world and adds the entry's other worlds once (an entry within an entry
  in turn; a second world with the same entry sees the same added
  worlds, which satisfy both since what reaches them is the key's), then
  closes the arrows by the worklist; each added world's atoms are
  charged as the names a model holds (48 bytes an atom) and counted, so a
  graft past the memory bound is a refusal (a graft of 400 worlds of
  25 000 atoms each once held 9 times a 64 MiB bound uncharged, the
  integers panel). An entry is taken only where
  `Account::spares` allows it (an eighth of the bound left), charged and
  never released; losing one costs only time.
- **The unsat side** (`Search::learn`, called in `fold` at a used step
  that makes a world): the key goes into `unsat` where the proof above
  the step uses only the world's own formula, the edge `x R y`, edges
  between older worlds, and `□` true and `◇` false at older worlds that
  the key holds. The argument is on `learn`: read every older world as
  one label `o`; the proof stays one of `o R y`, the own formula and
  those modal formulas; a world of another branch with the key gives
  `o` as itself (S4) or as a new point that sees it and all it sees
  (K4, KD4). A world made with an `unsat` key closes its branch by a
  step marked `cached` (`Search::refuted`), whose written members are
  what such a proof could use at most, so backjumping stays correct.
- **No derivation rests on the unsat side**: `run` checks the proof
  found (`rests_on_cache`) and, where it holds a `cached` step, empties
  steps, proofs and `unsat`, rewinds the account to the entries' bytes
  and runs the tableau again with `refute` off; both runs count their
  work. A wrong unsat entry could therefore only cost time: a "valid"
  comes from the second run's checked derivation, and an open branch is
  never made by the unsat side.
- **Its measured cost** (fixup-10): the panel's KD4 sequent from 20.5
  million units to about 58 000. On generated sets every verdict is the
  same and the blocking systems' totals fall, but single rows got
  slower, up to 29 times on small ones (a deep part searched before a
  sibling that closes at once, or a rerun); the report has the rows.
- **A step is kept only where the proof above uses it** (`fold`,
  `join`): a closed part carries the members of its conclusion its
  proof uses; a step whose *new* members (those not on the branch
  before it) none of them uses is left out, and a split whose first
  premise's proof uses none of its new members stands for the
  conclusion, its second premise never searched: worlds that each split
  once beside one that closes cost a split each, against `2^n`
  (`independent_splits_do_not_multiply`; 65b measured 20 such worlds
  at 20 splits against an unknown at 2²⁶ units without it). Emission
  (`decide::derivation`) adds each kept step's whole premise list (not
  only the new members), so a member another, left-out step put on the
  branch first is still in the premise the rule writes out. Every
  member a kept step writes out in its conclusion is in the needed set
  below it, so its adder is kept.
- **The derivation's size is estimated before it is made** (each
  node's members are the root's and its path's additions, an upper
  bound, and a term of `TERM_BYTES` for each formula of the arena and
  each member put on a sequent), against `limits.derivation_bytes`;
  past it `Refusal::Output`, never a verdict lost. The terms are made
  once, in the arena's order (`Terms::formula`; a rescan from the start
  per member was quadratic), and the build asks the stop every 4 096
  members (`Phase::View`).
- **A "not valid" whose model is too large to check** (past
  `MODEL_CELLS` or the memory bound) drops the model and keeps the
  verdict, as the ordinary suite does: there the verdict rests on the
  search's arguments (the panel of 65b found them sound). Every other
  verdict's evidence passes its check first.
- **`Outcome::check`** is the check of an outcome read back: the
  derivation's root must be `w0 : Γ ⊢ w0 : Δ` (its terms made in a copy
  of the derivation, so a term it lacks shows as a difference), it must
  rest on no assumption (the generic checker accepts root-scope
  hypotheses as assumptions) and be in the outcome's system's calculus;
  a countermodel must be of the outcome's system. `Derivation::check`
  alone takes no sequent, so a document's derivation of `⊢ w0 : p`
  passes it for any sequent. A derivation the checker refuses is
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
  `crate::name`, `rules::{Calculus, Derivation}` and the
  rule files' forms as they are.
- **Where quantifiers and agents go**: `Node` gains `Forall` and
  `Exists` and `Atom` terms, as the ordinary arena will; an agent is a
  second `u32` in `Box` and `Diamond`, which keeps the node at 12
  bytes.
