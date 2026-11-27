# Unit 4a: findings

Read in full: 12 files. Not read in full: none.

The finder's remarks: I agree with F-arch-02, -08, -13, -20, -21 and -23 from reading all of the code: the four ordinary-only fields in `linear::search::Options`, the duplicated arena (`mod.rs:864` ↔ `modal/mod.rs:206`), the three countermodel fields beside `Verdict::NotValid` (they also force ten `structure: None, kripke: None` lines across `decide.rs` and `table.rs`), the split of `mod.rs` at line 709, `Countermodel::check` being a one-line wrapper of `Model::refutes`, and the missing `is_checked()` on `Derivation`. The ordinary analogue of F-arch-04 also holds: `ordinary::Outcome` is `#[non_exhaustive]` with no constructor and `Search` has no variant for an outside method. So an outside engine for ordinary logic cannot answer in the crate's terms (X4); unit 2a's proposal should cover both suites. Neither the instruction effect of F-4a-05 and F-4a-06 nor pinned time could be measured here: the leader should run DHAT and callgrind on `ordinary-pigeons` and `read-tptp` before and after. For unit 7b: `cli/src/interact/ordinary.rs:672` (`binders`) walks a hash-consed formula with no seen-set, which is exponential on a shared arena read from JSON. For units 2c/1a: 6 004 of `read-tptp`'s 18 418 allocations are in `linear::lltp::clauses`, called at `ordinary/parse.rs:942`; for unit 6a: `ordinary::Derivation` implements serde's `Deserialize` publicly, outside the limits `wire::upgrade` applies (`derivation.rs:2661` uses it).

## F-4a-01

- dimension: api
- criterion: X3
- place: core/src/ordinary/derivation.rs:755
- kind: API break
- cost: small
- candidate: no

**Claim.** `ordinary::Derivation::check` reports every derivation that breaks a rule, including one a user wrote or another engine built, as `Error::ReadBack`, whose kind is `defect` and whose message says "the proof read back", where the generic checker reports the same claim as `InvalidDerivation` (kind `invalid`).

**Witness.** ```rust
// core/src/ordinary/derivation.rs:754
        let calculus = self.logic.calculus();
        let fail = |n: usize, why: String| Error::ReadBack {
            calculus,
            reason: format!("inference {n} ({}): {why}", self.inferences[n].rule),
        };
```

```rust
// core/src/errors/mod.rs:672
    /// A derivation read back from a linear proof is not one of LK or LJ,
    /// for the reason given. This is a defect of this crate.
    #[error("the proof read back is not a derivation of {calculus}: {reason}")]
```

```rust
// core/src/errors/mod.rs:983 and :1021
        Self::SequentMismatch { .. }
            | Self::Model { .. }
            | Self::Structure { .. }
            | Self::InvalidDerivation { .. } => Invalid,
...
            Self::Rejected(_)
            | Self::ReadBack { .. }
            | Self::Countermodel { .. }
            | Self::Reduction(_) => Defect,
```

The front ends work around it: `cli/src/check.rs:736-739` treats any non-refusal as "no" and prints `invalid derivation in LK of … : the proof read back is not a derivation of LK: inference 0 (ax): …` (`:778`); `core/src/protocol/ordinary.rs:758` folds `Invalid | Defect` into one `Failed`. A countermodel check, by contrast, answers `Error::Model` (invalid) and turns it into the defect `Countermodel` only for an engine's model (`model.rs:83`).

**Change.** Have `Derivation::check` answer `Error::InvalidDerivation { calculus: "LK"/"LJ", reason }` (kind invalid). The crate's own read-back paths (`decide.rs:493`, `:498`, `:894`, `Image::checked`) map a check failure of a derivation they built to `ReadBack`, as `Countermodel::found` does for models.

**Value.** A user who checks a derivation from a file or another engine gets kind `invalid` and a message about their derivation, so a front end can tell a wrong proof (exit 1) from a crate defect without special cases; the error JSON `code` becomes `invalid_derivation`.

**Net.** Touches the checker's error value only, not its rules. `the_checker_refuses_each_break`, `the_quantifier_rules_check` and the other tests that match `Error::ReadBack` change their pattern; the behaviour lock and wire-schema for `check`'s JSON error code; the mutation baseline of the checker should show no new survivors.

## F-4a-02

- dimension: api
- criterion: X1
- place: core/src/ordinary/mod.rs:1123
- kind: API break
- cost: medium
- candidate: no

**Claim.** A first-order ordinary formula can be neither built nor read through the public API: the constructors of terms, predicates and binders are crate-private and behind `parse`, an atom with arguments is readable only as its canonical text, and `Inference::term` returns a term id with no public way to read the term.

**Witness.** ```rust
// core/src/ordinary/mod.rs:1123
    #[cfg(any(test, feature = "parse"))]
    pub(crate) fn function(&mut self, name: &str, args: &[FoTermId]) -> Result<FoTermId, Error> {
...
// :1140
    pub(crate) fn variable(&mut self, index: u32) -> FoTermId {
...
// :1155
    #[cfg(any(test, feature = "parse"))]
    pub(crate) fn predicate(&mut self, name: &str, args: &[FoTermId]) -> Result<NodeId, Error> {
...
// :1245
    pub(crate) fn binder(
...
// :1049
    /// Returns the first-order tables, empty where the arena has none.
    pub(crate) fn first_order(&self) -> &FirstOrder<NodeId> {
```

```rust
// core/src/ordinary/mod.rs:775
    /// The atom names, by the index `Node::Atom` holds. The name of an
    /// atom with arguments is its canonical text, `p(f(a), #0)`.
```

```rust
// core/src/ordinary/derivation.rs:448
    /// Returns the term of a quantifier rule, an id of the derivation's
    /// first-order terms: the witness of `∀L` and `∃R`, the eigenvariable
    /// of `∀R` and `∃L`; `None` for every other rule.
    pub const fn term(&self) -> Option<FoTermId> {
```

`public-api.txt:5323-5333` lists every public method of `Formulas`: `add`, `atom`, `atom_name`, `atom_names`, `formula`, `has_equality`, `is_empty`, `is_first_order`, `len`, `node`; none reads or makes a term. The consumer test covered no first-order sequent (`consumer-test.md`, "What the test did not cover").

**Change.** Make `function`, `variable`, `predicate` and `binder` public and ungated (they already validate: names, arities, foreign ids), and add readers: `Formulas::atom_parts(u32) -> Option<(&str, &[FoTermId])>` and `Formulas::term(FoTermId) -> FoTerm` (or the shared `deduxit::terms` reader F-arch-07 proposes), with `term_text` public for printing.

**Value.** A researcher's engine for first-order ordinary logic (a connection prover, a tableau) can build its sequents, cut formulas and instances without printing and parsing them, read an atom's arguments, and print a derivation's witnesses; a library user with `default-features = false` gets first-order formulas at all.

**Net.** Additive in behaviour: the constructors keep the checks the parser relies on, and `Sequent::new` still refuses a loose variable. Tests of `parse.rs` and `first_order_tests`; `core/tests/depth.rs` for the new readers. No checker or search changes.

## F-4a-03

- dimension: api
- criterion: D1
- place: core/src/ordinary/mod.rs:171
- kind: API break
- cost: medium
- candidate: no

**Claim.** `Logic` has `#[default] Intuitionistic` and `ordinary::Options` derives `Default`, so the first call a library user writes answers for intuitionistic logic without the logic being named, and the module's leading example relies on it.

**Witness.** ```rust
// core/src/ordinary/mod.rs:162
/// An ordinary propositional logic.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
...
pub enum Logic {
    /// Classical logic, whose derivations are LK's.
    Classical,
    /// Intuitionistic logic, whose derivations are LJ's.
    #[default]
    Intuitionistic,
```

```rust
// core/src/ordinary/mod.rs:43
//! The excluded middle is not valid in intuitionistic logic, the default:
...
//! let outcome = decide(&sequent, &Options::default())?;
```

Consumer test, point 6: `decide(&sequent, &Options::default())` said `a \/ ~a` is not valid, and the toy engine's comparison failed until the logic was set. `core/src/settings.rs:170` and `web/src/calls.rs:87` read this default.

**Change.** Remove `Default` from `Logic`; give `ordinary::Options` a constructor `Options::new(logic)` and no `Default` (or a `Default`-free serde form with `logic` required), and pass the logic in `decide`'s examples. `Settings` and the web tab name their own default where they need one.

**Value.** A library user always says which logic a verdict is about, as D1 asks of the command; no silent intuitionistic answer to a classical question.

**Net.** The examples of the crate docs, README, the manual and the guide (`cli/tests/readme.rs`), the settings JSON (`wire-schema`, behaviour lock) and `web-tab`'s defaults; no search or checker changes.

## F-4a-04

- dimension: text
- criterion: T3
- place: core/src/ordinary/mod.rs:546
- kind: text
- cost: small
- candidate: no

**Claim.** Three doc comments of `ordinary/mod.rs` state what the code does not: `Method::decides` says the connection prover decides classical logic only and omits grounding and the model search, `mod table` carries the translation module's doc, and `Logic` and `Sequent` call themselves propositional.

**Witness.** ```rust
// core/src/ordinary/mod.rs:546
    /// Returns whether the method decides the logic: the embedding and the
    /// SAT solver every logic, LJT intuitionistic and minimal logic, the
    /// truth table, the tableau and the connection prover classical logic.
    pub const fn decides(self, logic: Logic) -> bool {
...
            Self::Connection => matches!(logic, Logic::Classical | Logic::Intuitionistic),
            #[cfg(feature = "sat")]
            Self::Grounding | Self::Models => true,
```

```rust
// core/src/ordinary/mod.rs:113
/// The translations into linear logic.
mod table;
#[cfg(feature = "interactive")]
mod tableau;
/// Formulas copied between arenas, for the goal state's search.
#[cfg(feature = "interactive")]
mod transfer;
mod translate;
```

```rust
// core/src/ordinary/mod.rs:162 and :1611
/// An ordinary propositional logic.
/// A sequent of ordinary propositional logic, `Γ ⊢ Δ`: the hypotheses `Γ`
```

**Change.** `decides`: "the embedding, the SAT solver, grounding and the model search every logic, LJT intuitionistic and minimal logic, the connection prover classical and intuitionistic logic, the truth table and the tableau classical logic". Move the doc line to `mod translate;` and give `mod table;` "The truth table of a classical sequent." `Logic`: "A logic of ordinary formulas." `Sequent`: "A sequent of ordinary logic, propositional or first-order".

**Value.** A user choosing a method for an intuitionistic first-order sequent reads the right answer in rustdoc; a reader of the module list finds the truth table where it is.

**Net.** Docs only; `cargo doc` and the doc tests.

## F-4a-05

- dimension: performance
- criterion: P2
- place: core/src/ordinary/derivation.rs:783
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** The ordinary checker and read-back allocate several vectors per inference (a `Vec<&Inference>` of the premises, copies of both sides of the conclusion, one more per premise), and the `ordinary-pigeons` journey makes 2 226 allocations per million instructions, the most of every journey.

**Witness.** ```rust
// core/src/ordinary/derivation.rs:783
            let premises: Vec<&Inference> = inference
                .premises
                .iter()
                .map(|&p| &self.inferences[p.index()])
                .collect();
```

```rust
// core/src/ordinary/derivation.rs:894 and :900
    let mut context = (inference.left.clone(), inference.right.clone());
...
    let with = |added: &[(Side, NodeId)]| {
        let mut sequent = context.clone();
```

`target/audit/dhat.md`, ordinary-pigeons: 60 124 blocks for 27 026 459 instructions (449 instructions per block; every other journey has at least 815). Frames: `with_capacity_in<ordinary::NodeId>` 27 910 blocks (46 %), `<InfId>` 8 271 (13 %), `<(NodeId, Side)>` 4 647 (7 %, the read-back's `members`, `:1502`), `<&ordinary::derivation::Inference>` 4 135 (6 %; on this journey only `derivation.rs:783` collects one). `check_inference(…, premises: &[&Inference])` forces the same collect at seven sites (`derivation.rs:783`, `interpolant.rs:831`, `interactive.rs:1761`, `:2276`, `:2797`, `:3548`, `:5478`).

**Change.** Compare each premise with the conclusion as multisets without building `with` (premise = context minus principal plus added, counted in two reused scratch buffers kept across inferences), and give `check_inference` the inferences and an `&[InfId]` instead of a collected `Vec<&Inference>`. Separately, an `Inference` whose premises (at most two) are inline would drop the read-back's `Vec<InfId>` per inference.

**Value.** The machine: every "valid" through the embedding, LJT and the goal state is checked, and large derivations (ILTP's SYJ202, 20 223 inferences) check with a fraction of the allocations.

**Net.** Touches the checker, the trust boundary: `the_checker_refuses_each_break` (one case per guard), the mutation baseline of the checker, and the ratchet's `ordinary-pigeons`. Measure before: DHAT blocks and callgrind instructions of `ordinary-pigeons`, then the same after; pinned time if the count drops more than a few per cent.

## F-4a-06

- dimension: performance
- criterion: P2
- place: core/src/ordinary/mod.rs:935
- kind: refactor
- cost: small
- candidate: yes

**Claim.** `Formulas::atom` stores every atom name twice, in `atoms` and as the key of `names`, two allocations per atom, which are 43 % of the `read-tptp` journey's blocks.

**Witness.** ```rust
// core/src/ordinary/mod.rs:935
                let index = self.atoms.len() as u32;
                self.atoms.push(name.to_owned());
                self.names.insert(name.to_owned(), index);
```

`target/audit/dhat.md`, read-tptp (18 418 blocks): `<deduxit::ordinary::Formulas>::atom (mod.rs:936)` 4 012 blocks, `(mod.rs:937)` 4 001 blocks. `predicate_atom` (`mod.rs:1227`) and `of_tables` (`:1021`) clone the key the same way.

**Change.** Keep one copy of each name: `atoms: Vec<Arc<str>>` shared with `names: HashMap<Arc<str>, u32>`, or a hash table of indices into `atoms` keyed by the name's hash (hashbrown's `HashTable`, if the dependency is taken).

**Value.** The machine: reading a TPTP file or an ordinary sequent with many atoms makes half the allocations for names, and the arena holds each name once.

**Net.** No behaviour change; the parse and serialize tests, the readers' fuzz targets. Measure DHAT blocks and callgrind instructions of `read-tptp` and `ordinary-pigeons` before and after.

## F-4a-07

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/derivation.rs:188; cli/src/interact/ordinary.rs:625
- kind: refactor
- cost: small
- candidate: no

**Claim.** The list of LK and LJ rules and their ASCII spellings are crate-private, so the command keeps its own copies of both tables, which drift independently of the parser that reads them.

**Witness.** ```rust
// core/src/ordinary/derivation.rs:188
    pub(crate) const ALL: [Self; 31] = {
...
// :292
const ASCII: [(&str, Rule); 40] = [
    ("/\\L", Rule::AndLeft),
```

```rust
// cli/src/interact/ordinary.rs:625
pub(super) const ASCII: &[(&str, &str)] = &[
    ("∧L", "andL"),
    ("∧L₁", "andL1"),
...
// :657
    const LK: [&str; 29] = [
        "ax", "WL", "WR", "CL", "CR", "∧L", "∧L₁", "∧L₂", "∧R", "∨L", "∨R", "∨R₁", "∨R₂", "→L",
```

`duplicates.md` lists the pair (`cli/src/interact/ordinary.rs:626` ↔ `core/src/ordinary/derivation.rs:159`). `Logic::ALL` and `Method::ALL` are public; `Rule::ALL` is not.

**Change.** Publish `Rule::ALL` (a closed enum, so an array is allowed), `Rule::STEPS` (every rule but cut and open) and `Rule::spellings(self) -> &'static [&'static str]` from the `ASCII` table; the command builds its help and completion lists from them.

**Value.** A front end (the command, the web page) lists exactly the rules and spellings the library reads; a new rule appears everywhere at once.

**Net.** `rules_read_from_their_names` and `labels_are_the_names`; the command's `interact` help output under the behaviour lock and `cli/tests/readme.rs`.

## F-4a-08

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/instance.rs:122; core/src/ordinary/herbrand.rs:760
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The walk "every term in these formulas, each shared node and term once" is written five times, with two seen-sets and two stacks each.

**Witness.** ```rust
// core/src/ordinary/instance.rs:122 (mentions); the same at :247 (largest_eigenvariable) and :375 (metas_in)
        let tables = self.tables();
        let mut nodes_seen = HashSet::default();
        let mut terms_seen = HashSet::default();
        let mut nodes: Vec<NodeId> = roots.collect();
        let mut terms: Vec<FoTermId> = Vec::new();
        while let Some(k) = nodes.pop() {
            if !nodes_seen.insert(k) {
                continue;
            }
```

```rust
// core/src/ordinary/herbrand.rs:760; the same at :345
    let tables = formulas.first_order();
    let mut seen_nodes: HashSet<NodeId> = HashSet::default();
    let mut seen: HashSet<FoTermId> = HashSet::default();
    let mut nodes = roots.to_vec();
    let mut terms: Vec<FoTermId> = Vec::new();
```

`duplicates.md`: `instance.rs:127` ↔ `:256` ↔ `:384` (19 lines), `:251` ↔ `:379` (25 lines), `herbrand.rs:356` and `:769` ↔ the three.

**Change.** One crate-private iterator `Formulas::terms_in(roots, extra_terms) -> impl Iterator<Item = FoTermId>` (each reachable term once); `mentions`, `largest_eigenvariable`, `metas_in` and herbrand's two become a `find`, a `max` and a `filter` over it.

**Value.** A reader, and the next change to the term tables (a new `FoTerm` variant) is made once.

**Net.** `mentions` is the checker's eigenvariable condition (`derivation.rs:986`): `the_quantifier_rules_check`, the checker's mutation baseline; `first_order_proofs_read_back`; herbrand's tests. No search counter moves.

## F-4a-09

- dimension: api
- criterion: C-INTERMEDIATE
- place: core/src/ordinary/derivation.rs:647
- kind: API break
- cost: medium
- candidate: no

**Claim.** `Derivation::sequent` returns the conclusion as a pair of id slices although the derivation holds an arena and both sides, so a caller who wants to print or decide the conclusion rebuilds a `Sequent` or its text by hand.

**Witness.** ```rust
// core/src/ordinary/derivation.rs:646
    /// Returns the sequent the derivation concludes, its two sides.
    pub fn sequent(&self) -> (&[NodeId], &[NodeId]) {
        (&self.left, &self.right)
    }
```

```rust
// cli/src/check.rs:743
    let (left, right) = d.sequent();
    let side = |ids: &[ordinary::NodeId]| {
        ids.iter()
            .map(|&id| d.formulas().formula(id).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let sequent = abbreviated(
        &match left {
            [] => format!("⊢ {}", side(right)),
            _ => format!("{} ⊢ {}", side(left), side(right)),
```

`decide.rs:1198` and `:1211` rebuild one with `Sequent::new(derivation.formulas().clone(), …to_vec(), …to_vec())`; `cli/src/interact/ordinary.rs:610` repeats the text for the goal state.

**Change.** Hold the conclusion as a `Sequent` inside `Derivation` and return `&Sequent` from `sequent()` (its `Display`, `left()`, `right()` and `formulas()` then serve every caller).

**Value.** A user prints, decides or exports a derivation's conclusion in one call; the command's hand-written `Γ ⊢ Δ` goes.

**Net.** The checker reads the same two lists; the derivation JSON is unchanged (`core/tests/serialize.rs`, wire-schema); `breaks 0.8.0` lists the signature.

## F-4a-10

- dimension: idiom
- criterion: AIP-193
- place: core/src/ordinary/decide.rs:885
- kind: refactor
- cost: small
- candidate: no

**Claim.** Two places decide between "unknown" and "defect" by comparing an error's message text with a sentinel string.

**Witness.** ```rust
// core/src/ordinary/decide.rs:882
        search::Verdict::Proved(proof) => match image.read_back(proof, limits, &mut stop) {
            // A proof that takes an open witness as a quantifier's term,
            // which no derivation names.
            Err(Error::ReadBack { reason, .. }) if reason == super::derivation::OPEN_WITNESS => {
                Verdict::Unknown(Reason::Incomplete)
            }
```

```rust
// core/src/ordinary/derivation.rs:1872
pub(super) const OPEN_WITNESS: &str =
    "a quantifier's term would be an open witness of the sequent, which no derivation names";
```

The same comparison is at `decide.rs:618` (`Image::checked`).

**Change.** Make the read-back's open-witness end a value of its own: `read_back_with` returns `Result<Option<Derivation>, Error>` (or a private `ReadBack::OpenWitness` outcome) and `Image::read_back` maps it to the public error; the two callers match on the value.

**Value.** A reader; a rewording of the message can no longer turn an unknown into a reported defect.

**Net.** `a_first_order_sequent_is_valid_or_unknown` (the open-witness case under `interactive`); no search change.

## F-4a-11

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/bruijn.rs:222; core/src/ordinary/instance.rs:621
- kind: refactor
- cost: medium
- candidate: no

**Claim.** Two walkers substitute for de Bruijn variables over the same arena, each with its own memo of `(NodeId, depth)` and its own term walk: `bruijn.rs` for lifting, lowering, open substitution and abstraction, `instance.rs` for closed instances, frames and open witnesses.

**Witness.** ```rust
// core/src/ordinary/bruijn.rs:230
    fn mapped_in(
        &mut self,
        root: NodeId,
        map: Map<'_>,
        nodes: &mut HashMap<(NodeId, u32), NodeId>,
        terms: &mut HashMap<(FoTermId, u32), FoTermId>,
    ) -> Result<NodeId, Error> {
```

```rust
// core/src/ordinary/instance.rs:563
struct Substitution {
    /// What is replaced.
    replace: Replace,
    /// Every node's instance under so many binders below the removed one.
    nodes: HashMap<(NodeId, u32), NodeId>,
    /// Every term's instance under so many binders below the removed one.
    terms: HashMap<(FoTermId, u32), FoTermId>,
}
```

`Map::Substitute` (`bruijn.rs:31`) and `Replace::Variables` (`instance.rs:549`) both put terms in for a removed binder's variable; only `instance.rs` has the `Maker` that also finds without building (`Find`, the checker's lookup).

**Change.** One walker: `instance.rs`'s `Substitution` over its `Maker`, with `bruijn`'s `Map` as what it replaces (shift, open term, closed frame, open witnesses, abstraction), so lifting and abstraction can also run as a lookup.

**Value.** A reader of the first-order code learns one de Bruijn walk; a fix to one (a missed shift) is made once.

**Net.** Touches the checker's `instance` lookup: `the_quantifier_rules_check`, `an_instance_replaces_the_binder_s_variable_at_every_depth`, `a_term_inside_a_function_is_replaced_and_deep_bodies_cost_no_stack`, the checker's mutation baseline; the transformations' tests (`normal`, `interpolant`) for `bruijn`'s callers.

## F-4a-12

- dimension: architecture
- criterion: D4
- place: core/src/kripke.rs:36
- kind: API break
- cost: small
- candidate: no

**Claim.** The Kripke model shared by ordinary and modal logic documents and prints its arrows in ordinary logic's reading (an order, "directly above", "below"), so the modal command writes its own printer.

**Witness.** ```rust
// core/src/kripke.rs:36
/// A finite Kripke model: worlds, arrows from each world to the worlds
/// directly above it, whose reflexive and transitive closure is the
/// order, and the atoms each world forces.
```

```rust
// core/src/kripke.rs:118
            if !world.arrows.is_empty() {
                let above: Vec<String> = world.arrows.iter().map(u32::to_string).collect();
                write!(f, "; below {}", above.join(", "))?;
```

```rust
// cli/src/modal.rs:327
        if seen.is_empty() {
            text.push_str("; sees no world");
        } else {
            let seen: Vec<String> = seen.iter().map(u32::to_string).collect();
            write!(text, "; sees {}", seen.join(", ")).expect("a string takes any text");
```

**Change.** Say on `Model` what the data is in neutral terms (each world's arrows, read by each suite as its relation) and move the two readings into the suites: `ordinary::Countermodel`'s `Display` writes "below", `modal::Countermodel`'s writes "sees" (the command's `worlds_text` then goes), in line with F-arch-21.

**Value.** A modal user reads a correct description of the shared type, and both suites print their models from the library.

**Net.** The command's output for ordinary and modal countermodels under the behaviour lock and the manual's examples (`cli/tests/readme.rs`); no check changes.

## F-4a-13

- dimension: api
- criterion: C-WORD-ORDER
- place: core/src/ordinary/negative.rs:520
- kind: API break
- cost: small
- candidate: no

**Claim.** The transformations name their short and full calls the other way round from the rest of the crate: `compare_default` is the short form and `compare` takes the search options, bounds and stop, where `prove`/`prove_within` and `decide`/`decide_within` use the plain name for the short form.

**Witness.** ```rust
// core/src/ordinary/negative.rs:468
pub fn compare(
    sequent: &Sequent,
    options: &Options,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Comparison, Error> {
...
// :520
pub fn compare_default(sequent: &Sequent, options: &Options) -> Result<Comparison, Error> {
```

The same pair is `herbrand::herbrand_default` (`herbrand.rs:195`), `interpolant::interpolate_default` (`interpolant.rs:317`) and `normal::normalize_default` (`normal.rs:3178`), against `linear::prove_within` (`linear/search/mod.rs:306`), `ordinary::decide_within` (`decide.rs:748`) and `modal::decide_within` (`modal/decide.rs:255`).

**Change.** Rename to `compare`/`compare_within`, `herbrand`/`herbrand_within`, `interpolate`/`interpolate_within`, `normalize`/`normalize_within`.

**Value.** A user who learned `decide`/`decide_within` guesses every other long call's two forms.

**Net.** Renames only; the examples of the manual and the guide that call them (`cli/tests/readme.rs` checks every name the guide uses); `breaks 0.8.0`.

## F-4a-14

- dimension: api
- criterion: X9
- place: core/src/ordinary/mod.rs:641
- kind: API break
- cost: small
- candidate: no

**Claim.** `Options::method()` is public but answers wrongly for a first-order sequent and has no caller outside `ordinary`: every front end asks `method_for(sequent)`, which `decide` follows.

**Witness.** ```rust
// core/src/ordinary/mod.rs:639
    /// Returns the method these options choose: the one given, else the
    /// embedding when a translation is given, else the logic's default.
    pub const fn method(&self) -> Method {
```

`grep -rn 'method_for(' cli/src core/src` finds `cli/src/ordinary.rs:189`, `:240`, `:353`, `:601`, `:706`, `cli/src/batch.rs:743`, `:932`, `cli/src/prove.rs:336`, `decide.rs:816`; `options.method()` is called only by `method_for` itself (`mod.rs:672`) and a test (`decide.rs:950`).

**Change.** Make `method()` private (or fold it into `method_for`), so the one public question is "which method decides this sequent".

**Value.** A user who asks the options which method will run gets the answer `decide` follows, also at first order.

**Net.** `the_options_choose_the_method` moves to `method_for`; no behaviour change.

## F-4a-15

- dimension: api
- criterion: X9
- place: core/src/ordinary/decide.rs:403
- kind: API break
- cost: small
- candidate: no

**Claim.** `CONNECTION_WORK` is declared `pub` and documented like the public `EVIDENCE_WORK` beside it, but its module is private and nothing re-exports it, so no user can name the bound that ends the default connection prover.

**Witness.** ```rust
// core/src/ordinary/decide.rs:403
pub const CONNECTION_WORK: u64 = 1 << 26;
```

```rust
// core/src/ordinary/mod.rs:125
pub use decide::{
    EVIDENCE_WORK, Evidence, Outcome, Search, Verdict, decide, decide_within, gives_way,
};
```

`public-api.txt` has `EVIDENCE_WORK` (`:5585`) and no `CONNECTION_WORK`; nothing outside `decide.rs` names it.

**Change.** Either re-export it beside `EVIDENCE_WORK` and link it from `decide_within`'s docs (the default's first-order classical answer "unknown, work limit" depends on it), or make it `pub(crate)`.

**Value.** A user who sees the default connection prover give way learns the bound from rustdoc; or the surface stops claiming an item it does not have.

**Net.** Docs and visibility only; `the_default_s_prover_gives_way_past_its_bound`.

## F-4a-16

- dimension: api
- criterion: X1
- place: core/src/ordinary/translate.rs:677
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The embedding builds its linear image with a private hash-consing builder and a struct expression of `linear::Sequent`'s crate-private fields: the crate itself needs the linear sequent builder that F-arch-05 and R59 propose for outside engines.

**Witness.** ```rust
// core/src/ordinary/translate.rs:677
/// A linear arena under construction, every term made once.
#[derive(Default)]
struct Builder {
    /// The terms, each after its subterms.
    terms: Vec<Term>,
    /// The index of every term made.
    ids: HashMap<Term, TermId>,
```

```rust
// core/src/ordinary/translate.rs:587
    let mut linear = crate::linear::Sequent {
        terms: builder.terms,
        roots: terms,
        atoms,
        antecedents: Some(antecedents),
        cuts: vec![],
        fo,
    };
    linear.optimize();
```

**Change.** When the public builder lands (F-arch-05, `plan/notes/api.md` R59: `Sequent::builder()` with `atom`, `term`, `dual`, `finish(&Limits)`), build the image through it and delete `Builder`; the translation is then the builder's first caller and its test.

**Value.** A reader sees one way to make a linear sequent; the builder gets a caller inside the crate, which answers the recorded reason it waits ("waits for its first caller").

**Net.** The image must stay identical: `names_read_back_and_bounds_hold`, `decides_and_reads_back`, the ratchet's `ordinary-pigeons` (the translation is on its path) and the read-back tests that match roots by position.

## F-4a-17

- dimension: api
- criterion: C-COMMON-TRAITS
- place: core/src/ordinary/mod.rs:1669
- kind: refactor
- cost: small
- candidate: no

**Claim.** `ordinary::Sequent` and `Formulas` implement neither `PartialEq` nor `Eq` while `linear::Sequent` does, so a user compares two ordinary sequents by their printed text.

**Witness.** ```rust
// core/src/ordinary/mod.rs:1669
#[derive(Clone, Debug)]
pub struct Sequent {
```

```rust
// core/src/ordinary/mod.rs:771
#[derive(Clone, Debug, Default)]
pub struct Formulas {
```

```rust
// core/src/linear/sequents/mod.rs:186
#[derive(Clone, Debug, PartialEq, Eq)]
```

The crate's own tests compare printed text (`bruijn.rs:447`, `parse.rs:1236` compares ids). `modal::Sequent` (`modal/mod.rs:388`) and `modal::Formulas` (`:143`) lack them too.

**Change.** Implement `PartialEq`/`Eq` on `Formulas` by its nodes, atoms and tables (ignoring the lookup maps and the eigen-name cache) and derive them on `Sequent`; the same for the modal pair.

**Value.** A user and a test compare sequents directly, as in the linear suite; two parses of one text compare equal.

**Net.** Additive; a test that two parses of a text are equal.

## F-4a-18

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/mod.rs:212; core/src/ordinary/negative.rs:145
- kind: refactor
- cost: small
- candidate: no

**Claim.** Four enums of this unit repeat the same `FromStr` body (find by name in `ALL`, else `UnknownName`), and `Method`'s feature list is restated with its `cfg`s in six places.

**Witness.** ```rust
// core/src/ordinary/mod.rs:217 (Logic); the same at :363 (Translation), :577 (Method), negative.rs:150
    fn from_str(name: &str) -> Result<Self, Error> {
        Self::ALL
            .iter()
            .copied()
            .find(|logic| logic.name() == name)
            .ok_or_else(|| Error::UnknownName {
                what: "logic",
                name: name.into(),
                known: Self::NAMES,
            })
    }
```

`duplicates.md`: `ordinary/mod.rs:581` ↔ `negative.rs:154`; `mod.rs:472` ↔ `:490`. `Method`'s `cfg(feature = "sat")` list is in `ALL` (`:437`), `NAMES` (`:454`), `name` (`:472`), `title` (`:490`), `logics` (`:508`), `decides` (`:549`).

**Change.** A crate-private helper `named::<T>(ALL, name, what, |t| t.name())` for the four `FromStr` bodies, and `NAMES` computed from `ALL` (or checked against it in one test, as `names_read_back_and_bounds_hold` does for two of them).

**Value.** A reader; a new method is added in fewer places.

**Net.** `names_read_back_and_bounds_hold`; the command's `--method` and `--logic` errors under the behaviour lock.

## F-4a-19

- dimension: performance
- criterion: P6
- place: core/src/ordinary/instance.rs:771; core/src/ordinary/model.rs:299
- kind: refactor
- cost: small
- candidate: no

**Claim.** Two sites of this unit use std's SipHash tables beside the crate's foldhash `hash` module; neither is on a journey's hot path.

**Witness.** ```rust
// core/src/ordinary/instance.rs:771
    let taken: std::collections::HashSet<&str> =
        tables.symbols.iter().map(|(s, _)| s.as_str()).collect();
```

```rust
// core/src/ordinary/model.rs:299
        let mut index: std::collections::HashMap<&str, u32> = formulas
```

`eigen_names` runs once per arena state (cached by `eigen_table`), the model check once per countermodel.

**Change.** Use `crate::hash::{HashMap, HashSet}` at both, and add `disallowed-types` for the std tables in `clippy.toml` (the finding F-1a-12 and F-1b-13 follow).

**Value.** One hasher crate-wide; the lint keeps new sites out.

**Net.** No behaviour change (neither table's order is observed); the model and eigenvariable-name tests.

## F-4a-20

- dimension: text
- criterion: T4
- place: core/src/ordinary/mod.rs:720
- kind: text
- cost: small
- candidate: no

**Claim.** Seven doc comments and comments of this unit use "refers to" and "stands as", which the writing rules replace with "is" and "has".

**Witness.** ```rust
// core/src/ordinary/mod.rs:720
/// A node of an ordinary formula, which refers to its subformulas by
/// arena index.
```

```rust
// core/src/ordinary/derivation.rs:543
    /// Returns the formulas the inferences refer to.
```

The others (`target/audit/text.md`): `decide.rs:566` ("stands as"), `negative.rs:342` ("stand as"), `translate.rs:73`, `:415` ("refers to"), `:537` ("stands as").

**Change.** `mod.rs:720`: "A node of an ordinary formula; its subformulas are arena indices." `derivation.rs:543`: "Returns the arena of the inferences' formulas." `translate.rs:73`: "The extra nodes a translation uses: …". `translate.rs:537`: "in ILL a hypothesis is its dual". `decide.rs:566`: "the image's proof is the evidence ([`Evidence::Image`])".

**Value.** A reader of rustdoc and the code.

**Net.** Docs and comments only.
