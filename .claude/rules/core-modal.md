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
  and `System`; `parse.rs` (behind `parse`): the precedence parser.
  The JSON forms are `core/src/serialize/modal.rs`. Nothing decides a
  sequent yet: the search, the model check and the proof are the
  design's sections 4 to 7.
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
- **What it copied** (for the kernel crate): from `ordinary`, the arena
  (`Formulas`, `Node`, `NodeId`, `add`, `atom`, `holds`, `occurrences`,
  `bound`), `Sequent`, the parser but for its TPTP dialect, and the
  sequent's serde form and reader; it uses `linear::sequents::{Stop,
  Stops}` and `linear::sequents::name` as they are.
- **Where quantifiers and agents go**: `Node` gains `Forall` and
  `Exists` and `Atom` terms, as the ordinary arena will; an agent is a
  second `u32` in `Box` and `Diamond`, which keeps the node at 12
  bytes.
