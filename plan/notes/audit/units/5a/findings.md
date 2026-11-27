# Unit 5a: findings

Read in full: 12 files. Not read in full: none.

The finder's remarks: All 12 files of the unit were read in full, besides the brief, the criteria, the consumer test, the rejections, the handovers and the parts of core.md and core-lambda.md on the module. What lambda needs of the linear suite: the atom-name writer `notation::Atoms`, `InfId`, `Labels` and `Drawn` for the exports, `search::Reason` for `Inhabitation::Unknown`, and `linear::Options::default()`, which `inhabit` passes twice to `ordinary::decide_within` (inhabit.rs:237, :815). That agrees with F-arch-01 and F-arch-02 from this side; `inhabit` and `enumerate` also take an `ordinary::Sequent`, since the lambda suite has no type value of its own outside a `Term`. I did not judge the soundness of the read-backs' `expect`s in linear.rs (unlike simple.rs, it relies on a checked derivation), nor whether `Error::IllTyped` should expose the private `Fault` kinds for the web page's hints; unit 6a or 8 may weigh the latter. Unit 6b should look at F-5a-09 and F-5a-14 with F-arch-06 (label tables keyed by the linear `Named`, and the Drawn layout copied in rules/draw.rs), and unit 5b at the same label behaviour for rule files. Unit 7a should settle the one spelling of value names that F-5a-06 depends on; F-5a-01 was measured with a scratch crate under the scratchpad (target in target/agents), not with a journey.

## F-5a-01

- dimension: performance
- criterion: P2
- place: core/src/lambda/reduce.rs:627
- kind: refactor
- cost: small
- candidate: no

**Claim.** Every step of a System F reduction copies the term with all the types and type variables of every earlier step, so a reduct carries a type arena that grows with the square of the steps.

**Witness.** ```rust
// core/src/lambda/reduce.rs:627-628 (copied)
        types: Types::of(&term.types, 0),
        type_variables: term.type_variables.clone(),
```
Each copied `ΛX` then pushes a fresh type variable (`fresh_type`, `reduce.rs:431-437`) and a `Type::Variable` (`reduce.rs:500`), and nothing is ever dropped (`Builder::finish` trims nodes only).

Measured with a scratch crate (`deduxit` at this tree, features `parse`), reducing `y:a |- (/\X. \x:X. x) [a] (… (y))` nested `n` deep in `Calculus::SystemF` with `ReduceOptions::default().with_steps(Some(100_000))`:

```text
n=10: start nodes 51 types 25 type_variables 10; after 20 steps: nodes 1 types 115 type_variables 100 end NormalForm
n=20: start nodes 101 types 45 type_variables 20; after 40 steps: nodes 1 types 425 type_variables 400 end NormalForm
n=40: start nodes 201 types 85 type_variables 40; after 80 steps: nodes 1 types 1645 type_variables 1600 end NormalForm
```
The normal form `y` (one node) keeps 1 645 types and 1 600 type-variable names. Each step's `check` rebuilds a hash table of all of them (`Types::of`, `check.rs:1084`), and the reduct's JSON writes them all.

**Change.** Compact the copy's arenas in `copied` as `Builder::finish` compacts nodes: keep only the types and type variables the copy's nodes, variables, context and goal reach, renumbered. Optionally reuse the source's `TypeVarId` for a `ΛX` copied once.

**Value.** A user of `term reduce --calculus system-f` and of `Term::reduce` gets a reduct whose JSON and memory are its own size; long reductions no longer pay a growing check per step.

**Net.** `system_f_tests.rs` (substitution, type β, capture) and the manual's `term reduce --calculus system-f` examples (`cli/tests/readme.rs`); type ids in printed JSON change. Touches the stepper only; the checker re-checks every reduct as now.

## F-5a-02

- dimension: api
- criterion: X1
- place: core/src/lambda/mod.rs:809
- kind: API break
- cost: medium
- candidate: no

**Claim.** A `lambda::Term` can be made only from text or JSON: the builder and `from_parts` are crate-private, the shape check is behind `serialize`, and the ids have no constructor (confirms F-arch-05 from inside the module).

**Witness.** ```rust
// core/src/lambda/mod.rs:184-185
#[cfg(feature = "serialize")]
pub(crate) mod read;
```
```rust
// core/src/lambda/read.rs:139
    pub(crate) fn from_parts(parts: Parts, limits: &Limits) -> Result<Self, Error> {
```
```rust
// core/src/lambda/mod.rs:212-219
pub struct TermId(u32);

impl TermId {
    /// Returns the index as a `usize`, for indexing the nodes.
    pub const fn index(self) -> usize {
```
`VarId`, `TypeId` and `TypeVarId` (`mod.rs:228`, `:244`, `:261`) likewise have `index` alone, and `Type::Atom { atom: u32 }` (`mod.rs:464-467`) names an atom by a bare `u32`. A program that generates terms (a translation, an enumerator, a test generator) must print text and parse it.

**Change.** Make `read::Parts` a public plain struct (or `Term::from_parts(parts, &limits)`) outside the `serialize` gate, keeping its shape check, and give the ids `new`; `check` stays the trust boundary. Consider an `AtomId` newtype for `Type::Atom`.

**Value.** A user who builds terms in code (a researcher's translation, the web page's exercises) makes and checks them without a text round trip, and with `default-features = false`.

**Net.** `tests.rs` `agrees` round trips and the JSON reader's tests keep the shape check; no checker change (the checker still decides every term).

## F-5a-03

- dimension: architecture
- criterion: E6
- place: core/src/lambda/mod.rs:1304
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The walks over a node's subterms and variables and over a type's operands are written ten times across the module, each listing every variant, and none is public, so a user who follows a term's subterms writes an eleventh.

**Witness.** Node walks:
```rust
// core/src/lambda/mod.rs:1304
pub(crate) fn operands(node: &Node, mut f: impl FnMut(TermId)) {
// core/src/lambda/mod.rs:1352
fn renumber(node: &mut Node, f: impl Fn(TermId) -> TermId) {
// core/src/lambda/read.rs:107
fn variables_of(node: &Node, mut f: impl FnMut(VarId)) {
// core/src/lambda/reduce.rs:557
fn binders(node: &Node) -> Vec<VarId> {
// core/src/lambda/reduce.rs:585
fn rebind(node: &mut Node, f: impl Fn(VarId) -> VarId) {
// core/src/lambda/check.rs:1184
pub(crate) fn written(term: &Term, node: &Node, mut f: impl FnMut(TypeId)) {
```
Type walks:
```rust
// core/src/lambda/read.rs:52
fn type_operands(ty: Type, mut f: impl FnMut(TypeId)) {
// core/src/lambda/read.rs:74
fn retyped(ty: Type, f: impl Fn(TypeId) -> TypeId) -> Type {
// core/src/lambda/poly.rs:94
fn operands(ty: Type, depth: u32, mut f: impl FnMut(TypeId, u32)) {
// core/src/lambda/print.rs:480-485 (free_in)
        match ty {
            Type::Tensor { left, right }
            | Type::With { left, right }
            | Type::Plus { left, right } => {
                operands.extend([left, right]);
            }
```
`poly.rs:55-83` (`rebuild`'s `Leave` arm) repeats `retyped`. Under `#![deny(clippy::wildcard_enum_match_arm)]` (`mod.rs:173`) System F's two nodes and three types were added to every copy.

**Change.** Give `Node` public `operands()` (an iterator of `TermId`), `operands_mut()` and `variables_mut()`, and `Type` public `operands()` and `map_operands()`; write `renumber`, `rebind`, `binders`, `variables_of`, `type_operands`, `retyped` and the walks in `poly` and `print` on them.

**Value.** A reader changes the calculus in one place per walk; a user walks a term's DAG or a type through the public API (X1: subterms followed).

**Net.** `tests.rs`, `untyped_tests.rs`, `system_f_tests.rs` and the manual's term examples; the checker's rule (`check::rule`) is untouched, `Pass::node` keeps its arms.

## F-5a-04

- dimension: api
- criterion: X1
- place: core/src/lambda/print.rs:348
- kind: API break
- cost: small
- candidate: no

**Claim.** A user cannot print a type: the type printer is crate-private, and `Typing::types()` holds types the term does not, which only a judgement of the whole term ever prints.

**Witness.** ```rust
// core/src/lambda/print.rs:348
pub(crate) fn write_type(
```
```rust
// core/src/lambda/check.rs:42-46
    /// Returns the types the typing names: the term's, then those the
    /// rules made, each after its operands.
    pub fn types(&self) -> &[Type] {
        &self.types
    }
```
`Typing::ty(node)` returns a `TypeId` into that list; to show the type of a subterm (a hover on the web page, a tutor's hint) a user must write a printer of the 12 `Type` variants with the `∀` renaming of `print::free_in`.

**Change.** Add a public `Term::write_type(out, ty, &Options)` (or a `Display` adapter) that prints a type of the term's arena or of a `Typing` over the term's atoms and type variables, within the existing length bound.

**Value.** Front ends and library users print the type of any node, as they print terms, in Unicode or ASCII.

**Net.** `agrees` in `tests.rs` (printer and parser agree); no checker or search touched.

## F-5a-05

- dimension: architecture
- criterion: E6
- place: core/src/lambda/check.rs:762
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The suite has three work meters of its own (`check::Meter`, `reduce::Budget`, `inhabit::Search::tick`), and they report `Progress` in three different units, none as `Progress` documents.

**Witness.** ```rust
// core/src/limits.rs:408-412
    /// The units of work done since the previous poll, in the unit of
    /// [`Limits::work`].
    pub work: u64,
    /// The units of work done since the call began.
    pub done: u64,
```
```rust
// core/src/lambda/check.rs:785-787: asked once per 1024 units, through counting(), which reports work 1, done = polls
        if self.work >= self.next {
            self.next = self.work.saturating_add(POLL);
            if (self.stop)() {
```
```rust
// core/src/lambda/reduce.rs:322: once per 4096 units, work 1, done = nodes of the current copy (restarts each step, reduce.rs:394)
            if (self.stop)(Progress::new(Phase::Reduce, 1, done)) {
```
```rust
// core/src/lambda/inhabit.rs:792: once per 1024 units, work and done in units
            if (self.stop)(Progress::new(Phase::Search, POLL, self.work)) {
```
The same pattern exists outside the unit (`modal/model.rs:376`, `ordinary/normal.rs:2228`, `linear/search/work.rs:20`, `ordinary/kripke_search.rs:172`, `rules/fitch.rs:632`, among others).

**Change.** One crate-private meter in `limits` (work, limit, memory, poll, phase) whose `tick(units)` reports `Progress::new(phase, units since the last poll, total)`; the three lambda meters become it.

**Value.** A front end that shows progress or slices work by `done` gets one unit from every lambda call; a reader sees one meter instead of three.

**Net.** Verdicts and refusals unchanged; the numbers a stop sees change. The lambda tests and the limits' stop tests; no checker rule touched.

## F-5a-06

- dimension: api
- criterion: L8
- place: core/src/lambda/mod.rs:345
- kind: API break
- cost: medium
- candidate: no

**Claim.** `Calculus::NAMES` and `Strategy::NAMES` say the command writes these names, but the command spells them in kebab case, so the JSON's names do not read back as flags and the library does not read the flags' names.

**Witness.** ```rust
// core/src/lambda/mod.rs:345-346, 353
    /// The names, as the JSON form and the command write them.
    pub const NAMES: &'static [&'static str] = &[
        "system_f",
```
```text
// docs/src/terms.md:337
$ deduxit term check --calculus system-f '/\X. \x:X. x'
```
`Strategy::NAMES` is `["normal_order", "applicative_order"]` (`reduce.rs:118-119`) while `--strategy` takes clap's `normal-order` (`cli/src/argument_parsing.rs:2119-2122`, a `ValueEnum`). `"system-f".parse::<Calculus>()` fails with `UnknownName`. Each name lives in four hand-kept places: `ALL`, `NAMES`, `name()` (`mod.rs:359-371`) and serde's `rename_all`, with no test that ties them.

**Change.** Settle one spelling for the names of values crate-wide (unit 7a's question), make `FromStr` accept the other meanwhile, derive `name()` from `NAMES` by index, and add a test that `ALL`, `NAMES`, `name()` and serde agree.

**Value.** A script that reads a term's JSON passes its `calculus` back as a flag; a library user reads the flag's spelling; the web page and the command offer the same names.

**Net.** `cli/tests/readme.rs` (the manual's and guide's examples), the wire schema check; no checker or search touched.

## F-5a-07

- dimension: performance
- criterion: B6
- place: core/src/lambda/mod.rs:615
- kind: build
- cost: small
- candidate: yes

**Claim.** `Node` and `Type` deserialize as internally tagged enums, which serde buffers through `Content`, and their two functions are among the web module's largest, about a third of the lambda suite's bytes.

**Witness.** ```rust
// core/src/lambda/mod.rs:614-615
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(tag = "kind", rename_all = "snake_case"))]
```
(`Type` the same at `mod.rs:460-461`.) `target/audit/wasm.md`:
```text
| `deduxit::lambda` | 671609 | 6.0 % |
| 73806 | 2 | `<deduxit::lambda::Node as serde_core::de::Deserialize>::deserialize` |
| 32492 | 2 | `<deduxit::lambda::Type as serde_core::de::Deserialize>::deserialize` |
```
Two copies each: about 148 KB and 65 KB raw. The only reader is `TermForm`'s `types: Vec<Type>` and `nodes: Vec<Node>` (`core/src/serialize/lambda.rs:170`, `:182`).

**Change.** Read nodes and types in `serialize/lambda.rs` through flat forms (`kind` plus optional fields, `deny_unknown_fields`) converted by one `match`, so `Node`'s and `Type`'s derived `Deserialize` is no longer instantiated for the module; the JSON accepted stays the same.

**Value.** Every visitor of the web page downloads fewer bytes, against a budget at 2 885 650 of 2 900 000.

**Net.** Measure with twiggy on the module as `wasm.md` did, and the budget check; the JSON reader's tests and the wire-schema check keep the form. No checker touched.

## F-5a-08

- dimension: architecture
- criterion: E6
- place: core/src/lambda/parse.rs:1328
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The untyped terms have a second parser beside the typed one, with its own stack of open constructs, context reader, `λ` reader and application run, for a subset of the same syntax.

**Witness.** ```rust
// core/src/lambda/parse.rs:1234 (typed judgement)
        if TURNSTILE.iter().any(|token| self.input.contains(token)) && !self.eat_any(TURNSTILE) {
```
```rust
// core/src/lambda/parse.rs:1331-1332 (untyped)
        let declared = TURNSTILE.iter().any(|token| self.input.contains(token));
        if declared && !self.eat_any(TURNSTILE) {
```
`enum Frame` with `Run`, `Bracket`, `Lambda` (`parse.rs:61-84`) against `enum Open { Run, Bracket, Lambda }` (`parse.rs:1268-1275`); `lambda` (`:577-597`) against `untyped` (`:1352-1373`); `deliver`'s `Frame::Run` (`:711-737`) against `untyped_apply` (`:1425-1437`). The subscript-digit loop is written twice too:
```rust
// core/src/lambda/parse.rs:332 and :1038
            let Some(digit) = (c as u32).checked_sub(0x2080).filter(|&d| d < 10) else {
```

**Change.** Read untyped terms with the one parser: a `Value` whose type is optional, `make` skipping `check::rule` in an untyped calculus, and `operand` making a free variable where no binder or context declares the name; one helper for subscript digits.

**Value.** A change of the syntax (a token, a bracket, an error hint) is made once; untyped terms get the typed parser's errors and spans.

**Net.** `untyped_tests.rs`, `tests.rs` (`agrees`), the manual's untyped examples in `docs/src/terms.md`; no checker or search touched.

## F-5a-09

- dimension: api
- criterion: X5
- place: core/src/lambda/draw.rs:606
- kind: API break
- cost: medium
- candidate: no

**Claim.** A typing derivation ignores the user's label choice: `Subscript` and a user's `Table` draw the same fixed labels as `Upright`, and the labels live in two copies of one table.

**Witness.** ```rust
// core/src/lambda/draw.rs:606-613
    fn markup<'a>(&'a self, rule: usize, labels: &'a Labels) -> Option<&'a str> {
        match labels {
            Labels::Off => None,
            Labels::Upright | Labels::Subscript | Labels::Table(_) => {
                LABELS.get(rule).map(|row| row[self.column()])
            }
        }
    }
```
```rust
// core/src/lambda/draw.rs:36 (LABELS) and :61 (NAMES)
    ["&E_1", "∧E_1"],
    ["&E₁", "∧E₁"],
```
`Labels::Table` is keyed by the linear suite's `Named` (`linear/proofs/style.rs:53`), which has no key for `→I` or `∀E`; `rules/draw.rs:285-291` does the same for rule files (F-1b-04).

**Change.** Key a label table by the rule's name, which every `Drawn` already gives (`Drawn::name`), so one table serves every suite, or say on `Labels` that tables apply to linear proofs alone; derive `NAMES` from `LABELS`.

**Value.** A user who sets their own labels for the LaTeX, Typst or SVG export gets them on a typing derivation, or is told they do not apply.

**Net.** The export check (`nix build .#checks.x86_64-linux.export`) and the exports' tests; no checker touched.

## F-5a-10

- dimension: text
- criterion: T3
- place: core/src/lambda/mod.rs:4
- kind: text
- cost: small
- candidate: no

**Claim.** The module docs present lambda-terms as proofs read back, and lead with a linear proof search, while the module is a suite of its own that reads, checks, reduces, types and inhabits terms.

**Witness.** ```rust
// core/src/lambda/mod.rs:4-5
//! A proof as a program: an intuitionistic proof read back as a
//! lambda-term.
```
```rust
// core/src/lambda/mod.rs:16-20
//! # Get a term
//!
//! Prove a sequent in an intuitionistic mode, then call
//! [`Derivation::term`](crate::linear::Derivation::term) on its
//! derivation:
```
The first example imports `deduxit::linear::{Mode, Options, Sequent, Verdict, prove}`; parsing, checking, reduction and principal types come after it.

**Change.** Summary: "Lambda-terms: read, check, reduce and print terms of the untyped, simply typed, linear and polymorphic calculi; find a type's terms; read a term off a proof." Lead with "Write and check a term", then reduction, then inhabitation, then reading a term off a linear or an LJ proof.

**Value.** A user who wants terms alone finds them first, and no suite is given the place of honour (D4).

**Net.** The module's doctests (run by `cargo test`); text only.

## F-5a-11

- dimension: text
- criterion: T3
- place: core/src/lambda/parse.rs:1473
- kind: text
- cost: small
- candidate: no

**Claim.** `Term::parse`'s syntax reference omits System F (`ΛX. M`, `M [A]`, `∀X. A`) and still speaks of "the other two" calculi, and `Term::reduce` says only a `λ` application is a redex after naming the type redex.

**Witness.** ```rust
// core/src/lambda/parse.rs:1473-1478
    /// A type is written as a formula of the calculus's logic: in the
    /// simple calculus as
    /// [`ordinary::Sequent`](crate::ordinary::Sequent) reads one, with
    /// `¬A` for `A → ⊥` and `A ↔ B` for `(A → B) ∧ (B → A)`; in the
    /// other two with `⊸` (`-o`), `⊗` (`*`), `&`, `⊕` (`+`), `!`, `1`,
    /// `0` and `⊤` (`top`).
```
```rust
// core/src/lambda/reduce.rs:785-787
    /// as a tree, which is the term the first step is taken on. Only the
    /// application of a `λ` is a redex: a pair taken apart, a `case` of
    /// an injection and the linear calculus's other constructs stay as
```

**Change.** Add rows for `ΛX. M` (`/\X. M`), `M [A]` and `∀X. A` (`forall X. A`) and say which calculi take them; replace "the other two" with "the linear and affine calculi and their System F"; write "Only `(λx. M) N` and `(ΛX. M) [A]` are redexes".

**Value.** A reader of the reference learns System F's syntax where the syntax is listed, and the reduction doc no longer contradicts itself.

**Net.** Doctests; text only.

## F-5a-12

- dimension: architecture
- criterion: E6
- place: core/src/lambda/reduce.rs:766; core/src/lambda/print.rs:536
- kind: refactor
- cost: small
- candidate: no

**Claim.** Two pieces of naming code are each written twice: the fresh name with the least free index, and the read-back's letters for a new variable.

**Witness.** ```rust
// core/src/lambda/reduce.rs:766-770
fn fresh_name(name: &str, taken: &HashSet<Box<str>>) -> Box<str> {
    let stem = name.trim_end_matches(|c: char| c.is_ascii_digit());
    let stem = if stem.is_empty() { name } else { stem };
    (1u64..)
        .map(|index| format!("{stem}{index}").into_boxed_str())
```
```rust
// core/src/lambda/print.rs:536-543
pub(crate) fn fresh(name: &str, taken: impl Fn(&str) -> bool) -> Box<str> {
    if !taken(name) {
        return name.into();
    }
    let stem = name.trim_end_matches(|c: char| c.is_ascii_digit());
    let stem = if stem.is_empty() { name } else { stem };
    (1u64..)
        .map(|index| format!("{stem}{index}"))
```
```rust
// core/src/lambda/mod.rs:1224-1226 (Builder::variable)
        let class = usize::from(!matches!(self.types.get(ty), Type::Lollipop { .. }));
        let letters = LETTERS[class];
        let n = self.named[class];
```
```rust
// core/src/lambda/inhabit.rs:716-718 (Search::extend)
        let class = usize::from(!matches!(self.base.types.get(ty), Type::Lollipop { .. }));
        let letters = super::LETTERS[class];
        let n = named[class];
```

**Change.** `rename` and `rename_types` call `print::fresh(name, |c| taken.contains(c))` (every caller's name is taken, so the result is the same); one `fn letter_name(class_count: &mut [u32; 2], function: bool) -> String` serves the builder and the enumerator.

**Value.** A reader keeps one rule for renaming and one for naming; a change of either cannot leave the stepper, the printer and the enumerator disagreeing.

**Net.** `untyped_tests.rs` (capture and renaming), `system_f_tests.rs`, the inhabitation tests' printed terms; no checker touched.

## F-5a-13

- dimension: performance
- criterion: P1
- place: core/src/lambda/print.rs:775
- kind: build
- cost: medium
- candidate: yes

**Claim.** No journey of the ratchet or of DHAT runs the lambda suite, so its reading candidates (allocation per node printed, quadratic list merging, arena clones per term listed) cannot be measured.

**Witness.** `grep -ci lambda target/audit/ratchet.out target/audit/dhat.md` gives 0 and 0; the twenty journeys are search, read, check, derivation, render, batch and ordinary ones. Candidates from reading:
```rust
// core/src/lambda/print.rs:1139: a Vec of pieces per node at every place written, again per node in lengths() (:1018)
                    stack.extend(self.pieces(node).into_iter().rev());
```
```rust
// core/src/lambda/inhabit.rs:368: O(max_terms²) comparisons of whole terms
        let rest: Vec<(u64, Term)> = terms.into_iter().filter(|t| !best.contains(t)).collect();
```
```rust
// core/src/lambda/inhabit.rs:813: the formula arena cloned per shape decided
        let sequent = Sequent::new(self.formulas.clone(), left, right)?;
```
```rust
// core/src/lambda/inhabit.rs:1223: the builder with its hash tables cloned per term listed
        let mut builder = self.base.clone();
```

**Change.** Add journeys `term-reduce` (a Church-numeral computation, untyped and System F), `term-print` (a shared DAG near the output bound) and `term-enumerate` (an infinite type with a cut list of a few thousand), then measure the candidates above.

**Value.** The suite's performance becomes measurable and guarded by ceilings like the other suites'.

**Net.** The ratchet and its ceilings (`bench/ceilings.csv`); no behaviour changes.

## F-5a-14

- dimension: architecture
- criterion: E6
- place: core/src/lambda/draw.rs:619; core/src/rules/draw.rs:298
- kind: refactor
- cost: small
- candidate: no

**Claim.** The typing derivation and a rule file's drawing implement `Drawn`'s tree layout (`root`, `premises` over an end list, `times`) with the same code.

**Witness.** ```rust
// core/src/lambda/draw.rs:619-630, the same as core/src/rules/draw.rs:298-309
    fn root(&self) -> InfId {
        // `new` makes the root's inference last, and at least one.
        InfId::new(u32::try_from(self.nodes.len() - 1).unwrap_or(u32::MAX))
    }

    fn premises(&self, id: InfId) -> &[InfId] {
        let start = match id.index() {
            0 => 0,
            index => self.ends[index - 1] as usize,
        };
        &self.premises[start..self.ends[id.index()] as usize]
    }
```
(`duplicates.md`: 136 tokens, `lambda/draw.rs:610` and `rules/draw.rs:289`.)

**Change.** A small shared type for an inference tree in post-order (nodes, ends, premises) with `root` and `premises`, which both drawings hold; this fits F-arch-06's move of `Drawn` to `export`.

**Value.** A reader of the exports sees one layout; a third drawing (a modal tableau, a natural-deduction view) reuses it.

**Net.** The export check and the exports' tests; no checker touched.
