# Unit 1a: findings

Read in full: 16 files. Not read in full: none.

The consumer test's points about this unit:

- **Point 1 (no constructor for a linear sequent)** is confirmed and widened. `Sequent::new()` (`core/src/linear/sequents/mod.rs:217`) is the empty sequent, and the only `&mut` methods are `add_cut` (:288), `optimize` (:779) and `add` (:850). `from_parts` (:415) is `pub(crate)` and also needs `#[cfg(feature = "parse")]`. `raw` (:991) exists only in `#[cfg(test)]`. The crate cannot build a sequent without these either: `protocol/linear.rs:455` calls `from_parts`, `ordinary/translate.rs:587` builds `crate::linear::Sequent { … }` by struct literal, and `families.rs:1110` formats text and parses it. F-1a-01 says what the constructors should be.
- **Point 8 (the suites name the same things differently)** is confirmed and widened to five arenas, plus two more name clashes (`Side`, `Sign`) and `atom(name)`, which means one thing in linear and another in ordinary. See F-1a-05.
- **Point 9 (one type under several paths)** is confirmed and corrected. `Kind` has three paths, not two: `linear::Kind`, `linear::sequents::Kind` and `linear::sequents::term::Kind`. In this unit 33 types have two or three paths. See F-1a-04.
- **D4: `first_order.rs` and `name.rs`.** Both sit at the root, which is right for things every suite shares. `name.rs` needs nothing. The first-order types, though, are public only under the linear suite's path, and only the linear suite can read them (F-1a-02). The suites' shared walk and printer live in `linear::sequents` (F-1a-03).

## F-1a-01

- dimension: api
- criterion: X1
- place: core/src/linear/sequents/mod.rs:217
- kind: API break
- cost: medium
- candidate: no

**Claim.** A linear sequent cannot be built through the public API without text or JSON. Ordinary and modal logic both can, through `Formulas::{atom, add}` and `Sequent::new(formulas, left, right)`.

**Witness.**

```rust
    /// Returns the empty sequent.
    pub const fn new() -> Self {
```

```rust
    #[cfg(feature = "parse")]
    pub(crate) fn from_parts(
```

The crate's own callers have to get around this. `core/src/ordinary/translate.rs:587` uses a struct literal over private fields:

```rust
    let mut linear = crate::linear::Sequent {
```

`core/src/linear/families.rs:1110` formats text and parses it, so the families exist only with `parse`:

```rust
    Sequent::parse_within(text, &unbounded)
```

The other suites have the constructors (`core/src/ordinary/mod.rs:864`, `:926`, `:1687`; `core/src/modal/mod.rs:209`, `:264`, `:405`):

```rust
    pub fn add(&mut self, node: Node) -> Result<NodeId, Error> {
```

```rust
    pub fn new(formulas: Formulas, left: Vec<NodeId>, right: Vec<NodeId>) -> Result<Self, Error> {
```

**Change.** Give the linear suite the shape ordinary and modal logic have:

- a `linear::Formulas` arena of terms, atoms and the first-order tables;
- `Formulas::atom(&mut self, name: &str) -> Result<TermId, Error>`: composes the name to NFC, calls `name::check` and interns the atom;
- `Formulas::add(&mut self, term: Term) -> Result<TermId, Error>`: runs `check_bounds` against the arena so far, so the topological order holds by construction;
- `Formulas::dual(&mut self, id: TermId) -> TermId`: the dualisation `push_cuts` already does;
- `Sequent::new(formulas, left, right) -> Result<Sequent, Error>`: negates the left formulas, sets `antecedents` and optimizes.

The empty sequent becomes `Sequent::default()`, which already exists. Then route `from_parts`, `translate.rs:587` and the families through the new constructors.

**Value.** A researcher's engine can build cut formulas, interpolants and translated images without printing and parsing them (consumer attempt 4). With `default-features = false` there is a sequent to work on at all (X7). The generated families no longer need the parser. `ordinary` stops writing `linear::Sequent`'s private fields.

**Net.** The behaviour lock and `core/tests/serialize.rs`: the families must give the same arena, since occurrence ids follow it. Also `deduxit-bench run --all-families --timeout 5` for the verdicts, and the focused target set's counters against `bench/targets/47.csv`. No checker or search code changes. The search inputs must be byte-identical.

## F-1a-02

- dimension: architecture
- criterion: D4
- place: core/src/linear/sequents/fo.rs:9
- kind: API break
- cost: large
- candidate: no

**Claim.** The first-order types are shared by both arenas but are public only under the linear suite's path. Only a linear sequent can read an atom's predicate and arguments. An ordinary sequent has no accessor for the same tables.

**Witness.**

```rust
pub use crate::first_order::{Args, Eigen, FoTerm, FoTermId, Symbol, SymbolFault};
```

The tables' own module (`core/src/first_order.rs:9-10`) says:

```rust
//! The tables know no logic: the linear and the ordinary arenas each hold
//! one, keyed by their own binders' ids (`K`).
```

The ordinary arena holds the same `Interner<NodeId>` (`core/src/ordinary/mod.rs:788`). `public-api.txt` lists no first-order accessor for it: `ordinary::Formulas` has only `add`, `atom`, `atom_name(u32)`, `atom_names`, `formula`, `has_equality`, `is_empty`, `is_first_order`, `len` and `node`, and its atom is `ordinary::Node::Atom(u32)`. The linear `Sequent` has `atom_predicate`, `symbols`, `fo_terms`, `arguments`, `binder_name`, `fo_size` and `atom_is_open` (`core/src/linear/sequents/mod.rs:472-539`).

**Change.**

- Publish the types from the root module that defines them (`pub mod first_order`, or a public `deduxit::terms`), at one path.
- Give the read accessors one shared home: a borrowed view `first_order::Tables<'_>` with `predicate(atom)`, `symbols()`, `terms()`, `arguments(args)` and `binder_name(id)`, returned by `linear::Sequent::first_order()` and `ordinary::Formulas::first_order()`.
- Move the term-store trait `unify::Terms` (`core/src/unify.rs:19`, behind `interactive`) into `first_order.rs`.

`name.rs` stays as it is.

**Value.** A user of first-order ordinary logic can inspect an atom's arguments (X1) without parsing the key text `p(f(a), #0)`. The shared data model gets a shared path (D4). A future first-order modal suite gets the accessors without new code.

**Net.** The behaviour lock (the JSON forms do not change), `core/tests/first_order.rs`, and the rustdoc JSON diff (`breaks 0.8.0`). No checker, search or certificate logic changes, only paths and accessors.

## F-1a-03

- dimension: architecture
- criterion: D4
- place: core/src/linear/sequents/notation.rs:20
- kind: API break
- cost: large
- candidate: no

**Claim.** The crate-wide formula walk (`Stop`, `Stops`, the public `Walk` and `Visit`) and the printers' spelling table (`Notation`, `Atoms`) live in `linear::sequents`. Ordinary logic, modal logic, the exports, the lambda printer and the rule files import them from there. The linear suite in turn imports ordinary logic only to hold its symbols.

**Witness.**

```rust
use crate::ordinary::{Formulas, NodeId, Symbols};
```

`deps.md` ("`linear` names `ordinary`: 3 paths") lists only `Symbols`, `NodeId` and `Formulas`, all from this file. `Notation` carries other suites' spellings (`core/src/linear/sequents/notation.rs:76-89`): `necessary` and `possible`, used only by calculi given as data, and `ordinary: Symbols`. `Notation::ordinary` (:1209) writes an ordinary sequent. The walk is imported the other way:

```rust
pub(crate) use fmt::{Stop, Stops};
```

(`core/src/linear/sequents/mod.rs:19`), and `core/src/ordinary/mod.rs:154` and `core/src/modal/mod.rs:71` both have:

```rust
use crate::linear::sequents::{Stop, Stops};
```

Only a linear sequent or a reading can make the public walk (`Sequent::walk`, `fmt.rs:232`; `Reading::walk`, `reading.rs:733`). An ordinary or modal formula has none.

**Change.** Move `Stop`, `Stops`, `Walk` and `Visit` to a root module (`crate::walk`, public as `deduxit::walk::{Walk, Visit}`). Move `Notation` and `Atoms` to a root `crate::notation`, beside `export/notation.rs`. Add `ordinary::Formulas::walk(id)` and `modal::Formulas::walk(id)`. The code itself does not change.

**Value.** Linear logic stops depending on ordinary logic, which removes a cycle between suites (D4). Users of ordinary and modal formulas get the walk that is safe at any depth and that `core.md` asks every outside printer to follow (X1). A reader finds the shared printer where shared things are.

**Net.** The ratchet's `read-text` and `render-*` journeys: the printers' `#[inline(always)]` and `#[cold]` layout must survive the move, so compare counts. Also the behaviour lock and `core/tests/depth.rs`. No checker or search code changes.

## F-1a-04

- dimension: api
- criterion: C-HIDDEN
- place: core/src/linear/sequents/mod.rs:9
- kind: API break
- cost: medium
- candidate: no

**Claim.** In this unit 33 public types are reachable under two or three paths. The cause is that every leaf module is `pub` and is also re-exported by its parent and again by `linear`.

**Witness.**

```rust
/// Printing sequents and formulas in one-sided notation.
pub mod fmt;
/// The first-order terms, symbols and binders of a sequent.
pub mod fo;
```

```rust
/// The terms an arena is built from.
pub mod term;

pub use fmt::Formula;
pub(crate) use fmt::{Stop, Stops};
pub use fmt::{Visit, Walk};
pub use fo::{Args, Eigen, FoTerm, FoTermId, Symbol, SymbolFault};
pub use term::{Atom, Kind, Term, TermId};
```

Every child of `occurrences` is also `pub` and re-exported (`core/src/linear/occurrences/mod.rs:8-19`), and so is `pub mod fragment` (`core/src/linear/mod.rs:13`). The 33 types, from `surface.md`, are:

- three paths: `Atom`, `Term`, `TermId`, `Kind`, `Formula`, `Eigen`, `IllSequent`, `IllSyntax`, `InputOptions`, `Instances`, `FrameId`, `Reading`, `ShapeError`, `Side`, `ClassicalSequent`, `Written`;
- two paths: `Visit`, `Walk`, `Args`, `FoTerm`, `FoTermId`, `Symbol`, `SymbolFault`, `Forest`, `Member`, `OccId`, `Sign`, `Fragment`, `Mode`, `Order`, `IllConnective`, `IllFormula`, `Sequent`.

Some modules also hold things that do not belong to their topic: `Mode` and `Order` are in `fragment.rs`, so one path is `linear::fragment::Mode`. The front end's input settings `IllSyntax` and `InputOptions` are in `occurrences::ill` (`ill.rs:283-345`).

**Change.**

- Make the leaf modules private:
  - in `sequents`: `term`, `fmt`, `fo`;
  - in `occurrences`: `ill`, `instances`, `reading`, `written`;
  - in `linear`: `fragment`.
- Give `Mode` and `Order` a file of their own (`mode.rs`).
- Choose one public path per type. The flat `linear::X` is what ordinary and modal logic use (`ordinary::Node`, `modal::Node`).
- Leave `sequents` and `occurrences` as private modules, or as public ones that only hold docs, with `#[doc(no_inline)]`.

**Value.** A reader of rustdoc or of an error message sees one path to write. The listing shrinks by about 40 lines for this unit alone (X9, B4).

**Net.** `cargo hack` runs and the `unreachable_pub` and `unnameable_types` check named in `core.md`. `breaks 0.8.0` lists the removed paths for the changelog. No logic changes.

## F-1a-05

- dimension: api
- criterion: C-WORD-ORDER
- place: core/src/linear/sequents/mod.rs:449
- kind: API break
- cost: large
- candidate: no

**Claim.** The five formula arenas name the same things in five ways. One name, `atom(name)`, is a lookup in linear logic and a constructor in ordinary and modal logic. Two type names (`Side`, `Sign`) mean different things in different suites.

**Witness.** The linear lookup:

```rust
    /// Returns the atom called `name`, if the sequent has one.
    pub fn atom(&self, name: &str) -> Option<Atom> {
```

The ordinary constructor (`core/src/ordinary/mod.rs:926`):

```rust
    pub fn atom(&mut self, name: &str) -> Result<NodeId, Error> {
```

From `public-api.txt`, the node types, accessors and ids:

- linear: `Sequent::term(TermId) -> Term`. `TermId`, `Atom`, `OccId`, `Symbol`, `FoTermId` and `Args` each have `new`, `get` and `index`.
- ordinary: `Formulas::node(NodeId) -> Node`. `NodeId` has `index` only, and the atom is `Node::Atom(u32)`.
- modal: `Formulas::node(NodeId) -> Node`. `NodeId` has `index` only, and the atom is `Node::Atom(u32)`.
- rules: `Terms::term(TermId) -> Term<'_>`. `TermId` has `new` and `index`.
- lambda: `Term::node(TermId) -> &Node`. `TermId` has `index` only.

`linear::NodeId` is a node of a proof, while `ordinary::NodeId` and `modal::NodeId` are nodes of a formula.

The clashing type names:

- `linear::Side` is `{Input, Output}`, but `ordinary::Side` is `{Left, Right}`.
- `linear::Sign` is `{Atom, Dual}`, but `ordinary::Sign` is a tableau's sign.

**Change.**

- Use one id convention: every arena id gets `new`, `get` and `index`, from one crate-private macro.
- Make an atom a newtype in every suite.
- `atom(&mut, name)` always makes or interns an atom. The lookup becomes `find_atom(&self, name)`.
- Rename linear `Side` to `Position`, the word its method `Reading::position` already uses.
- Whether the linear formula node becomes `Node`/`NodeId`, which forces the proof node to be renamed, is a question for the API proposal, together with unit 1b.

**Value.** A researcher who works in two suites learns one vocabulary. Error messages and rustdoc stop showing two `Side`s and two `NodeId`s that mean different things.

**Net.** `breaks 0.8.0`, and the examples of README, the manual and the guide (`cargo test -p deduxit-cli --test readme`, which checks the names they use). No logic changes.

## F-1a-06

- dimension: performance
- criterion: P2
- place: core/src/linear/occurrences/mod.rs:738
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Building a forest allocates a `Vec` for every compound occurrence, only to push its children in reverse. A forest of the LLTP library's largest problem has 27.8 million occurrences, and every search and every proof read from JSON builds one.

**Witness.**

```rust
                let subterms: Vec<TermId> = node.subterms().collect();
                for &k in subterms.iter().rev() {
                    stack.push((k, o, d + 1));
                }
```

The parser does the same walk without allocating (`core/src/linear/parse/mod.rs:1038-1040`):

```rust
                let (left, right) = self.terms[term.index()].operands();
                stack.extend(right);
                stack.extend(left);
```

**Change.** Write `let (left, right) = node.operands(); stack.extend(right.map(|k| (k, o, d + 1))); stack.extend(left.map(|k| (k, o, d + 1)));`. The numbering does not change.

**Value.** The machine: one allocation fewer per binary or unary occurrence, on every forest built. To be measured with DHAT on `search-chain-128` and `check-wide-m1-2048`, and with the ratchet's instruction counts. The book's rule of thumb is that ten allocations fewer per million instructions is about 1 %.

**Net.** The forest's invariant test (`check_invariants`), the ratchet, and the target sets' counters, which must stay equal because ids do not change. No checker or search code changes.

## F-1a-07

- dimension: api
- criterion: C-GOOD-ERR
- place: core/src/linear/occurrences/instances.rs:513
- kind: API break
- cost: small
- candidate: no

**Claim.** An instance table built through the Rust API (`Instances::new`, `apply`, `frame`, `member`) and handed to `Proof::with_instances` is refused as a JSON document. Its error has the code `json` and the message "not an instance table of deduxit's wire form".

**Witness.**

```rust
/// Returns the error of a malformed instance table.
fn malformed(message: String) -> Error {
    Error::Json {
        form: "instance table",
        message,
    }
}
```

The message comes from `core/src/errors/mod.rs:167`:

```rust
    #[error("not {} {form} of deduxit's wire form: {message}", article(form))]
```

The `Instances` example (`instances.rs:68-88`) shows that tables are built in Rust.

**Change.** Add a variant `Error::InstanceTable { message }`, with code `instance_table` and kind malformed, or a specific type that converts into `Error`, which `canonical` returns. The proof's JSON reader passes it through as it does today.

**Value.** An outside engine that builds a first-order proof gets an error that names its table, not a wire form it never wrote (X3). A front end that branches on `code` can tell a bad table from bad JSON.

**Net.** The behaviour lock, if it pins a malformed table's error (`core/tests/lock.rs`), and `Error::CODES`. The check of the table is unchanged.

## F-1a-08

- dimension: api
- criterion: X2
- place: core/src/linear/occurrences/mod.rs:595
- kind: refactor
- cost: small
- candidate: no

**Claim.** The one test of axiom partners is crate-private, so an outside engine writes its own. That copy will be wrong for atoms with arguments if the test changes, which is the reason the rules give for having one predicate.

**Witness.**

```rust
    pub(crate) fn dual_literals(&self, x: OccId, y: OccId) -> bool {
        self.is_literal(x) && self.atom(x) == self.atom(y) && self.sign(x) != self.sign(y)
    }
```

The consumer test's engine re-implements it (`consumer-test.md`, `mll.rs`):

```rust
            && forest.atom(a).is_some()
            && forest.atom(a) == forest.atom(b)
            && forest.sign(a) != forest.sign(b)
```

`core-forest.md` says: "Axiom partners are one predicate … so that atoms with arguments change one function (F58) … A new site that pairs literals asks it."

**Change.** Make `Forest::dual_literals` public, with an example. This is additive.

**Value.** An outside engine pairs literals as the checker does, and stays right when the pairing of first-order atoms changes.

**Net.** `cargo test`. No logic changes.

## F-1a-09

- dimension: architecture
- criterion: E5
- place: core/src/linear/occurrences/instances.rs:20
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `occurrences/instances.rs` holds the proofs' canonicalisation (`canonical` and `Canonical`, about 430 lines). That code renumbers `proofs::Node`s, so the data module `occurrences` names `proofs`, while `proofs` names `occurrences`.

**Witness.**

```rust
use crate::linear::proofs::Node;
```

```rust
pub(crate) fn canonical(
    forest: &Forest,
    table: &Instances,
    nodes: &mut [Node],
) -> Result<Instances, Error> {
```

`canonical` is at :551 and `Canonical::finish` matches every `Node` variant at :901-936. `clippy.md` lists `canonical` among the functions over 100 lines (127).

**Change.** Move `canonical`, `Canonical` and `malformed` (`instances.rs:512-940`) to `linear/proofs/`, next to `Proof::with_instances` (`proofs/mod.rs:470`), their one caller. Keep the table, `Terms`, `Numbering`, `Framing` and the names in `occurrences`, which then names nothing of `proofs`.

**Value.** A reader finds the check of a proof's table where the proof is built. The forest module reads as data alone (E5).

**Net.** `cargo test`, the behaviour lock, and the checker's mutation baseline, since `canonical` is on the path into the trust boundary. This is a move with no change of logic.

## F-1a-10

- dimension: idiom
- criterion: E6
- place: core/src/linear/occurrences/instances.rs:1030
- kind: refactor
- cost: small
- candidate: no

**Claim.** The term printer that uses an explicit stack is written twice, line for line: once over the sequent's tables and once over a proof's two arenas. A trait over both stores already exists.

**Witness.** `core/src/linear/occurrences/instances.rs:1036-1069` (`Framing::write_term`):

```rust
        let mut open: Vec<(Args, u32)> = Vec::new();
        let mut next = Some(term);
        loop {
            if let Some(t) = next.take() {
                match self.terms.term(t) {
```

`core/src/first_order.rs:299-329` (`FirstOrder::write_with`):

```rust
        let mut open: Vec<(Args, u32)> = Vec::new();
        let mut next = Some(term);
        loop {
            if let Some(t) = next.take() {
                match self.terms[t.index()] {
```

The rest of both loops is the same, from `let Some(&mut (args, ref mut i)) = open.last_mut() else` through `open.pop();`. `unify::Terms` (`core/src/unify.rs:19-45`) is implemented for both `FirstOrder<K>` and `instances::Terms`.

**Change.** Move `unify::Terms` into `first_order.rs` (see F-1a-02), add a method that gives a symbol's name, and write one `write_term<S: Terms>(store, out, term, symbol, bound, other)`. `FirstOrder::write_with` and `Framing::write_term` both call it.

**Value.** A reader of the code: one walk over first-order terms, so a new kind of term is one arm, not two.

**Net.** The ratchet's `render-*` journeys, `core/tests/first_order.rs` and the behaviour lock. No checker code changes.

## F-1a-11

- dimension: idiom
- criterion: E6
- place: core/src/linear/sequents/notation.rs:998
- kind: refactor
- cost: small
- candidate: no

**Claim.** The printer writes the layout of a two-sided sequent four times and that of a one-sided sequent three times. The layout is the formulas separated by commas, the alignment and the turnstile. The pair of mark characters is spelled three times, without a name.

**Witness.** `classical` (:998-1012):

```rust
        let mut any = false;
        for (m, negated) in left {
            if any {
                out.write_str(", ")?;
            }
            formula(out, m, negated)?;
            any = true;
        }
        if any {
            out.write_char(' ')?;
        }
        if aligned {
            out.write_str(self.align)?;
        }
        out.write_str(self.turnstile)?;
```

The same lines are in `framed` (:1110-1130), `two_sided` (:1179-1195) and `ordinary` (:1231-1243). The one-sided loop is in `one_sided` (:1022-1028), `sequent` (:1063-1070) and `framed` (:1097-1104). The mark pair `let (open, close) = if marks { ("\u{2}", "\u{3}") } else { ("", "") };` is at :987, :1054 and :1217.

**Change.** Write one private `sides(&self, out, left, right, aligned, marks, write_one)` and a `const MARKS: (&str, &str)` that every writer calls. Check the web module's bytes: the writers are generic over `W`.

**Value.** A reader of the code. A change of layout, such as a separator, is made once and the exports cannot drift apart.

**Net.** The behaviour lock, the `render-*` journeys, and the web module's size budget. No logic changes.

## F-1a-12

- dimension: performance
- criterion: P6
- place: core/src/linear/sequents/notation.rs:236
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** The crate's fast hasher has std's SipHash tables beside it in ten places of library code. No `clippy.toml` with `disallowed-types` keeps out new ones.

**Witness.**

```rust
    symbols: Option<std::collections::HashSet<&'a str>>,
```

The other library sites:

- `core/src/first_order.rs:598` (`EigenNames`)
- `core/src/linear/occurrences/instances.rs:947`
- `core/src/unify.rs:15`
- `core/src/modal/model.rs:273`
- `core/src/ordinary/model.rs:299`
- `core/src/ordinary/normal.rs:71`, used at :1344-1347, :2757 and :2872
- `core/src/ordinary/instance.rs:771`
- `core/src/linear/proofs/interactive/quantified.rs:959` and `:984`

`core/src/linear/search/reference.rs:44` is test-only. The repository has no `clippy.toml`.

**Change.** Use `crate::hash::{HashMap, HashSet}` at every site. Add a `clippy.toml` with `disallowed-types = ["std::collections::HashMap", "std::collections::HashSet"]`, with an `#[expect]` in `hash.rs` where the aliases are defined. The sites I opened only look up keys, so no output order changes.

**Value.** The machine: no SipHash on these tables. Most sites are cold, so the size of the gain needs measuring: the ratchet's `ordinary-pigeons` and `render-*` journeys under callgrind. One rule for the crate, kept by a lint.

**Net.** The ratchet and the behaviour lock. If a site iterates, its output order must be checked, which the lock shows. No checker code changes.

## F-1a-13

- dimension: api
- criterion: C-CONV
- place: core/src/linear/occurrences/ill.rs:188
- kind: refactor
- cost: small
- candidate: no

**Claim.** `IllSequent::into_sequent` and `ClassicalSequent::into_sequent` consume the value but clone the sequent anyway, because `Forest` cannot give its sequent up.

**Witness.**

```rust
    /// Returns the one-sided sequent, for a caller that hands it on.
    pub fn into_sequent(self) -> Sequent {
        self.forest.sequent().clone()
    }
```

The same code is at `core/src/linear/occurrences/written.rs:252-254`.

**Change.** Add `Forest::into_sequent(self) -> Sequent`, which moves the owned sequent out, and use it in both.

**Value.** A caller that hands on a large sequent saves a copy of the arena and every atom name. `into_` then means what C-CONV says it means.

**Net.** `cargo test`. No logic changes.

## F-1a-14

- dimension: api
- criterion: X1
- place: core/src/linear/sequents/mod.rs:449
- kind: API break
- cost: small
- candidate: no

**Claim.** `Sequent::atom(name)` compares the name as given. A decomposed `é` therefore misses the atom that every reader stored composed, while the ordinary arena composes before it looks up.

**Witness.**

```rust
    pub fn atom(&self, name: &str) -> Option<Atom> {
        self.atoms
            .iter()
            .position(|n| n == name)
            .map(|i| Atom::new(i as u32))
    }
```

The ordinary lookup (`core/src/ordinary/mod.rs:926-927`):

```rust
    pub fn atom(&mut self, name: &str) -> Result<NodeId, Error> {
        let name = &*crate::name::normalized(name);
```

`core-sequents.md` says: "a table's names are composed. A new reader of names composes them too."

**Change.** Compose with `crate::name::normalized(name)` before the scan, and change the name as F-1a-05 says (`find_atom`).

**Value.** A user looking an atom up by its name finds it however the accents are encoded. This matches every reader and ordinary logic.

**Net.** `cargo test`. A decomposed name now finds its atom. No other result changes.

## F-1a-15

- dimension: idiom
- criterion: E7
- place: core/src/linear/parse/mod.rs:1005
- kind: refactor
- cost: small
- candidate: no

**Claim.** The parser matches `Term` with a wildcard arm. The parse module is the one module of the calculus's syntax that does not deny `wildcard_enum_match_arm`, so a new connective would be passed by unchanged in the ordered dualisation without a compile error.

**Witness.**

```rust
                    term = match term {
                        Term::Tensor(l, r) => Term::Tensor(r, l),
                        Term::Par(l, r) => Term::Par(r, l),
                        other => other,
                    };
```

`sequents/mod.rs:6` and `occurrences/mod.rs:6` have `#![deny(clippy::wildcard_enum_match_arm)]`. `parse/mod.rs` does not. `core.md` ("No wildcard arm over the calculus's enums") gives the reason, a bug of this kind (F85).

**Change.** Add the deny at the top of `parse/mod.rs` and spell the other variants out. Add `parse` to the list in `core.md`.

**Value.** A variant added later, such as an n-ary or ordered connective, must decide what the cyclic dual does with it.

**Net.** The build. No change in behaviour.

## F-1a-16

- dimension: idiom
- criterion: E6
- place: core/src/linear/fragment.rs:220
- kind: refactor
- cost: small
- candidate: no

**Claim.** `Fragment::name_in` repeats the twelve-arm table of `Fragment::name`, differing only by an `I` before each name. `duplicates.md` lists the pair (209 tokens).

**Witness.** `name` (:194-200):

```rust
        match (
            self.has_quantifiers(),
            self.has_exponentials(),
            additive,
            multiplicative,
        ) {
            (false, true, true, _) => "LL",
```

`name_in` (:227-233) has the same scrutinee and arms, with `"ILL"`, `"IMELL"` and so on.

**Change.** Compute the index of the named fragment once, in a `const fn named(self) -> usize` into `NAMED`. Return `NAMES[i]`, or `INTUITIONISTIC_NAMES[i]` in an intuitionistic mode. The tables stay `const`.

**Value.** A reader. A new named fragment is one row in two tables of names, not two arms in two matches.

**Net.** The tests in `fragment.rs` (`names`, `intuitionistic_names`, `names_read_back`) and the behaviour lock, through outcomes' `fragment`.

## F-1a-17

- dimension: idiom
- criterion: E6
- place: core/src/linear/parse/mod.rs:556
- kind: refactor
- cost: small
- candidate: no

**Claim.** The parser closes an application into a term with the same four lines in three places.

**Witness.** `core/src/linear/parse/mod.rs:556-559`:

```rust
                let arity = u32::try_from(items.len()).map_err(|_| self.past_bound())?;
                let args = self.interner.list(&items);
                let symbol = self.symbol(&symbol, arity, false, from..self.at)?;
                id = self.interner.term(FoTerm::App(symbol, args));
```

The same lines are at :478-483, which ends `let left = self.interner.term(FoTerm::App(symbol, args));`, and at :659-662.

**Change.** Write one cold `fn applied(&mut self, symbol: &str, from: usize, items: &[FoTermId]) -> Result<FoTermId, Error>`.

**Value.** A reader of the parser. Arity and role are checked in one place.

**Net.** `core/tests/parse.rs` (`error_positions`), the `read-text` journey, which takes the cold path only for first-order text, and the lock.

## F-1a-18

- dimension: text
- criterion: T1
- place: core/src/linear/sequents/mod.rs:325
- kind: text
- cost: small
- candidate: no

**Claim.** The comment that explains why `push_cuts` calls `insert` and not `entry` cannot be understood without knowing the codegen it refers to.

**Witness.**

```rust
        // Not `entry`: while `optimize_terms` is its only user, the
        // compiler inlines it into the parser's path.
```

**Change.** Rewrite it as: "`insert`, not `entry`: `optimize_terms` is the only caller of `entry` on this map, so the compiler inlines that call into the parser's path."

**Value.** A reader of the code, who can tell whether the reason still holds.

**Net.** None needed.

## F-1a-19

- dimension: text
- criterion: C-QUESTION-MARK
- place: core/src/linear/sequents/fmt.rs:210
- kind: text
- cost: small
- candidate: no

**Claim.** The one example of the public walk, which `core.md` points every outside printer to, uses `unwrap` where every other example of the unit uses `?`.

**Witness.**

```rust
/// let sequent: Sequent = "⊢ a ⊗ !b".parse().unwrap();
```

**Change.** Write `.parse()?` and end the example with `# Ok::<(), deduxit::Error>(())`, as `Reading::walk`'s example (`reading.rs:711`, `:731`) does.

**Value.** A user who copies the example gets code that propagates errors.

**Net.** The doctest.

## F-1a-20

- dimension: api
- criterion: C-SEND-SYNC
- place: core/src/linear/sequents/fmt.rs:165
- kind: refactor
- cost: small
- candidate: no

**Claim.** The public `Walk` boxes its operands' closure without `Send + Sync`, so a walk over a `Sync` sequent is neither `Send` nor `Sync`. No test asserts either.

**Witness.**

```rust
type Operands<'a, T> = Box<dyn Fn(T) -> (Option<T>, Option<T>) + 'a>;
```

Both constructors' closures capture only `&Sequent` or `&Reading` (`fmt.rs:234`, `reading.rs:734`), and both are `Sync`.

**Change.** Box `dyn Fn(T) -> (Option<T>, Option<T>) + Send + Sync + 'a`, and add a `const` assertion beside the one for `Error` (`errors/mod.rs:1300`).

**Value.** A front end can hand a walk to a worker thread, as it can the sequent.

**Net.** The build. No logic changes.
