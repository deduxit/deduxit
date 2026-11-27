# Unit 1a: notes

## Inventories

- surface.md: every type of the unit is reachable under 2 or 3 paths (Atom, Term, TermId, Kind: `linear::`, `linear::sequents::`, `linear::sequents::term::`; Formula `…::fmt::Formula`; Visit/Walk `…::fmt::`; IllSequent `…::occurrences::ill::`, Reading `…::reading::`, Written `…::written::`, FrameId/Instances `…::instances::`, Mode/Order/Fragment `linear::fragment::`; FoTerm/Symbol/Args/FoTermId/SymbolFault `linear::sequents::fo::`). Cause: `pub mod term`, `pub mod fmt`, `pub mod fo` in sequents/mod.rs:9-16 and `pub mod` children of occurrences, plus `pub mod fragment` in linear/mod.rs:13, plus re-exports in linear/mod.rs:29-47.
- surface.md: Forest 33 methods / 0 examples; Sequent 26/22; Fragment 13/13; Mode 11/11; Reading 12/10; Kind 7/7; IllSequent 7/7; ClassicalSequent 9/9; Instances 13/13.
- surface "nothing outside names": `linear::occurrences::Polarity` (mod.rs:139), `IllFormula` (reading.rs:804), `FrameId`, `ShapeError`(!), `SymbolFault`, `Walk`; methods `Forest::same_root`, `Fragment::has_multiplicatives`, `has_multiplicative_units`, `Instances::first_term`.
- duplicates.md:33: fragment.rs:191 and :224, 209 tokens.
- clippy.md: >100-line fns: first_order.rs:419 (121), notation.rs:839 (109, written_in), instances.rs:551 (127), ill.rs:690 (102).
- deps.md: ordinary/modal/lambda/rules name `linear::sequents::{Stop, Stops}`, `linear::sequents::notation::{Notation, TEXT, Atoms}`, `linear::occurrences::instances::{meta_name, Terms}`, `linear::sequents::fo::FirstOrder`. Shared machinery living in the linear suite (D4).
- text.md: "refer to" lines (first_order.rs:115, sequents/mod.rs:193, :788, term.rs:56, instances.rs:539), "rather than" occurrences/mod.rs:1018, parse/mod.rs:490.

## linear/mod.rs

- :29-47 re-exports every type at `linear::` while the modules stay `pub` → the two/three-path problem (consumer test point 9).

## sequents/mod.rs

- :9-16 `pub mod fmt; pub mod fo; pub(crate) mod notation; pub mod term;` and `pub use` of their items at :18-22 → each type at 2-3 paths.
- :215-226 `Sequent::new()` is the empty sequent; no public way to add a term, an atom or a root. `from_parts` :415 is `pub(crate)` and `#[cfg(feature = "parse")]`; `raw` :991 test-only. Confirms consumer point 1. Public &mut methods: `add_cut` :288, `optimize` :779, `add` :850.
- :208 `impl std::default::Default for Sequent` fully qualified (Default is in the prelude). trivial.
- :294-295, :851-852 `self.atoms.len() as u32` / `self.terms.len() as u32` unchecked in `add`/`add_cut` (cast_possible_truncation): a sum of two sequents past u32 terms wraps silently. Only reachable with >4G terms; low value.
- :288-307 add_cut and :850-878 add repeat the same offset/extend/join_first_order/merge_atoms sequence.
- :827-841 merge_atoms: the same retain closure twice (atoms and fo.atoms).
- :449-454 `atom(name)` linear scan.
- :325-326 comment "Not `entry`: while `optimize_terms` is its only user, the compiler inlines it into the parser's path." – unclear (why `insert` rather than entry here), refers to the other function.

## sequents/term.rs

- TermId/Atom: `new`, `get`, `index`. Term exhaustive (closed by decision). Kind repr(u8). Hash manual. Fine.
- :98 size assert 12.

## sequents/fo.rs

- re-export of crate::first_order types under linear::sequents::fo; FirstOrder/Interner aliases with TermId.

## sequents/fmt.rs

- :18-130 `Stop`/`Stops`: a generic walk over any formula type, `pub(crate)`, used by ordinary (3 files) and modal (2 files) per deps.md. It lives in `linear::sequents::fmt`. D4: shared machinery in a suite.
- :139-195 public `Visit`/`Walk` only for linear sequents and readings; ordinary/modal have no public walk? (check).
- :210 doc example `.parse().unwrap()` (C-QUESTION-MARK).
- :8-9 module doc in sequents/mod.rs "Printing sequents and formulas in one-sided notation." – but fmt holds the generic walk as well.

## sequents/notation.rs

- :28-90 `Notation` holds the spellings of linear, modal (`necessary`, `possible` :76-87, used only by rules) and ordinary (`ordinary: Symbols` :89) formulas; `Notation::ordinary` :1209 writes an ordinary sequent. It lives in `linear::sequents::notation` and is named from rules (3 files), ordinary, lambda, export. D4: the shared printer belongs at the root.
- :236 `symbols: Option<std::collections::HashSet<&'a str>>` – SipHash std set beside crate::hash::HashSet (P6). Also first_order.rs:598, instances.rs:947 and outside: unify.rs:15, modal/model.rs:273, ordinary/model.rs:299, ordinary/normal.rs:71, ordinary/instance.rs:771, proofs/interactive/quantified.rs:959,984, search/reference.rs:44 (test-only?). No clippy.toml with disallowed_types.
- The two-sided layout loop "left, ', ', ' ', align, turnstile, ' ', right" is written 4 times: classical :998-1017, framed :1110-1136, two_sided :1179-1202, ordinary :1231-1248; and the one-sided loop 3 times: one_sided :1022-1029, sequent :1063-1071, framed :1097-1105. Plus `(open, close) = if marks { ("\u{2}", "\u{3}") }` 3 times (:987, :1054, :1217): a magic pair with no named const. E6.
- :972-975, :1040-1043 too_many_arguments expects: `classical` 8 args, `sequent` 7 args with `aligned: bool, marks: bool` (C-CUSTOM-TYPE: two bools).

## parse/mod.rs

- :477-487, :552-559, :658-662: "u32::try_from(items.len()) ... interner.list(&items) ... self.symbol(&symbol, arity, false, from..self.at) ... interner.term(FoTerm::App(symbol, args))" three times (application closed into a term). Small E6.
- :222-229 `first_order_within` calls `fo_size()` twice on refusal (cold).
- :1080-1158 `parse_within`, `parse_ordered`; FromStr. Fine.
- Pending::Binder(bool) internal bool.

## fragment.rs

- :190-246 `name` and `name_in` the same 12-arm match twice (duplicates.md:33). E6 small.
- :360-414 `Mode` and :432 `Order` live in module `fragment` → `linear::fragment::Mode` path. E5/C-HIDDEN.
- :339-357 `fragment()` reachability pass, same as optimize_terms' (sequents/mod.rs:677-692).
- Mode Default = CLASSICAL (D1 note, library).

## occurrences/mod.rs

- :8-19 `pub mod ill; pub mod instances; pub mod reading; pub mod written;` + `pub use` → 2-3 paths.
- :738 `let subterms: Vec<TermId> = node.subterms().collect();` inside the per-occurrence loop of `Forest::build`: one heap allocation per compound occurrence (P2 candidate). parse/mod.rs:1038-1040 shows the allocation-free form (`operands()`, extend right then left).
- :595 `dual_literals` pub(crate); consumer test re-implemented it (mll.rs:219-222).
- :139 `Polarity` (focus concept) lives here; Kind's methods in term.rs, fragment.rs (`fragment`), occurrences/mod.rs (`sign`, `polarity`).
- :289-308 held_bytes omits sequent.cuts and fo tables (undercount, minor).

## occurrences/set.rs — crate-private bitset; fine.

## occurrences/reading.rs

- `Side {Input, Output}` vs `ordinary::Side {Left, Right}`; `occurrences::Sign {Atom, Dual}` vs `ordinary::Sign` (signed formula). Same names, different meanings (point 8).
- ShapeError messages fine.

## occurrences/ill.rs

- :188 `into_sequent(self)` clones: `self.forest.sequent().clone()`; written.rs:252 same (C-CONV: into_ should move). No `Forest::into_sequent`.
- :205-219 PartialEq duplicates written.rs:316-332 (same `name` fn and comparison).
- :283-345 `IllSyntax`, `InputOptions` (front-end input settings) live in occurrences::ill.

## occurrences/written.rs — see above.

## occurrences/instances.rs

- :20 `use crate::linear::proofs::Node;` — occurrences names proofs; `canonical` :551-690 and `Canonical` :693-940 renumber proof nodes: proof logic in the data module (layering cycle occurrences ↔ proofs).
- :513-518 `malformed` → `Error::Json { form: "instance table" }`, whose message is "not an instance table of deduxit's wire form: …" (errors/mod.rs:167). `Proof::with_instances` (proofs/mod.rs:470) built through the Rust API reports a JSON error, code `json`.
- :1030-1070 `Framing::write_term` repeats first_order.rs:290-330 `write_with` (explicit-stack term writer). `unify::Terms` (unify.rs:19-45) is already a trait over both stores.
- :947 std HashSet.
- :972 `meta_name` used by ordinary/instance.rs:183.

## first_order.rs

- Root module, crate-private; its public types (Symbol, FoTermId, Args, Eigen, FoTerm, SymbolFault) are published only as `linear::sequents::fo::*` / `linear::sequents::*` / `linear::Eigen`. Ordinary's arena holds the same tables (`Interner<NodeId>`, ordinary/mod.rs:788) but `ordinary::Formulas` has no accessor for them (public-api: Formulas has add, atom, atom_name, atom_names, formula, has_equality, is_empty, is_first_order, len, node) and `ordinary::Node::Atom(u32)`; modal `Node::Atom(u32)`. Linear Sequent has `atom_predicate`, `symbols`, `fo_terms`, `arguments`, `binder_name`, `fo_size`, `atom_is_open`.
- :593 `Interner::eigens` OnceLock: ordinary-only cache in the shared struct (minor).
- :598 EigenNames with std HashSet.
- :536-542 binders out of order reported as NotTopological{index 0, parent 0} (misleading; reachability unclear).

## name.rs — shared, crate-private, at root: right place. KEYWORDS (linear), ORDINARY_KEYWORDS, RESERVED. Fine.

## Cross-suite naming (point 8)

- formula node: linear `Term` via `Sequent::term(TermId)`; ordinary `Node` via `Formulas::node(NodeId)`; modal `Node` via `Formulas::node(NodeId)`; rules `Term<'_>` via `Terms::term(TermId)`; lambda `Node` via `Term::node(TermId)`.
- ids: linear TermId/Atom/OccId/Symbol/FoTermId/Args new+get+index; ordinary NodeId index only; modal NodeId index only; lambda TermId index only; rules TermId new+index, NodeId index. Atoms: linear `Atom` newtype; ordinary/modal `Node::Atom(u32)`, `Formulas::atom_name(u32)`.
- `atom(name)`: linear `Sequent::atom(&self, &str) -> Option<Atom>` (lookup); ordinary/modal `Formulas::atom(&mut self, &str) -> Result<NodeId>` (constructor).
- `NodeId`: linear proof node vs ordinary/modal formula node.

## Point 1 witnesses

- sequents/mod.rs:217 `Sequent::new()` empty; :415 `from_parts` pub(crate) cfg(parse); :991 `raw` test-only.
- protocol/linear.rs:455 uses `Sequent::from_parts`; ordinary/translate.rs:587 builds `crate::linear::Sequent { … }` by struct literal; families.rs:1110 builds by formatting text and parsing.
- ordinary mod.rs:864 `Formulas::add`, :926 `Formulas::atom`, :1687 `Sequent::new(formulas, left, right)`; modal mod.rs:209, :264, :405 same shape.
