# Unit 1b: findings

Read in full: 17 files. Not read in full: none.

## F-1b-01

- dimension: architecture
- criterion: D4
- place: core/src/linear/proofs/derivation/view.rs:130
- kind: API break
- cost: large
- candidate: no

**Claim.** The drawing layer that every suite uses lives inside the linear suite. This covers the view options (`ViewOptions` with `Compact`, `Sides`, `ClassicalSides`, `Structural`, `Layout`), the text tree (`TextOptions`, `fmt::write_text`, `text_size`, `write_steps`), the labels (`Labels`, `OpenGoal`, the trait `Drawn`) and `InfId`. So ordinary logic, lambda-terms, rule files, the protocol and `Settings` name `crate::linear::proofs::…` for a notion that is not linear logic.

**Witness.**

`ViewOptions` holds fields that only other suites read (view.rs:148-159):

```rust
    /// Whether a derivation of LK or LJ shows its weakenings and
    /// contractions. ...
    pub structural: Structural,
    /// Whether a front end writes a derivation in a calculus given as data
    /// as a proof tree or as a Fitch proof, ...
    pub layout: Layout,
```

Users outside the suite:

```rust
// core/src/ordinary/derivation.rs:9-12
use crate::linear::proofs::style::Drawn;
use crate::linear::proofs::{
    Compact, InfId, Labels, Rule as Linear, Sides, Size, Structural, TextOptions, ViewOptions,
};
// core/src/lambda/draw.rs:10-12 and core/src/rules/draw.rs:10-12
use crate::linear::InfId;
use crate::linear::proofs::Labels;
use crate::linear::proofs::style::Drawn;
// core/src/protocol/rules.rs:226
    if settings.view.layout == crate::linear::proofs::derivation::Layout::Fitch {
// core/src/ordinary/derivation.rs:1108
        crate::linear::proofs::fmt::write_text(
```

`Settings` (the root) imports `linear::ViewOptions` (core/src/settings.rs:16, :137). `Drawn` has five implementors, and only two are linear: style.rs:318, derivation/classical.rs:823, ordinary/derivation.rs:1149, lambda/draw.rs:597, rules/draw.rs:276.

**Change.** Move `style.rs`, `fmt.rs`'s layout and `TextOptions`, `ViewOptions` with its five enums, and `InfId` to a root module (for example `crate::draw`). The root then holds what every suite shares, as `core.md` ("Layout") asks. Keep `linear::proofs::Derivation` and its `Drawn` impl in the suite. Say on each `ViewOptions` field which suites read it.

**Value.** A user of ordinary logic, lambda-terms or rule files finds the options of their drawing at a neutral path, not under `linear::proofs`. The design meets D4 and core.md's own rule, which says an item a second suite shares moves to the root.

**Net.** The behaviour lock, README's and the manual's examples (`cli/tests/readme.rs`), and the web bindings' tests for the paths in `web/`. No checker, search or certificate is touched. The JSON forms of `ViewOptions` and `TextOptions` stay the same.

## F-1b-02

- dimension: performance
- criterion: P2
- place: core/src/linear/proofs/check/pass.rs:764
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** Almost every heap allocation of a check is the checker making a leaf's sequent. Each leaf (`ax`, `1`, `⊤`) gets a boxed `State` and then a hash table for its linear zone, so a proof of n leaves costs about 2n allocations.

**Witness.**

```rust
    fn just(&self, ids: impl IntoIterator<Item = Member>, any: bool) -> Box<State> {
        let mut d = Box::new(State {
            theta: Zone::default(),
            gamma: Bag::default(),
```

DHAT on the journey `check-qbf-20-2` (target/audit/dhat/check-qbf-20-2.json, 129 111 blocks in all): `Box<State>::new` in `Pass::just` takes 31 743 + 30 720 blocks (pass.rs:912 and the other `just` site), and the first `reserve_rehash` of the `Bag`'s `HashMap<Member, u32>` takes 31 743 + 30 720 more. That is 124 926 blocks, 97 % of the journey's. The journey runs 138.1 M instructions (baselines.md:56). On `check-wide-m1-2048`, `just` makes 4 096 boxes.

**Change.** Hold a zone of one or two members inline (`Bag` as a small array that turns into the table past two), or reuse the boxes that `join` and `over` drop through a free list in `Pass`. A reused table must stay counted by the slots it has: `table_bytes` counts `most`, and a recycled table keeps its old capacity. The free list should therefore reuse boxes only, or count the table's capacity.

**Value.** The checker runs after every proof a search finds (`Options::check` is on by default) and in `deduxit check`. Fewer allocations speed up every proved outcome on the command line and in the web page.

**Net.** It touches the checker. The oracle's differential test (`agrees_with_the_oracle`), `the_bound_is_exact`, `holds_no_more_than_its_bound`, Kani's harnesses (they build `Bag` directly: harnesses.rs:188-200), the mutation baseline of the checker, and `web-core-wasm32`. To measure: `check-qbf-20-2` and `check-wide-m1-2048` under callgrind (the ratchet) and DHAT, before and after.

## F-1b-03

- dimension: performance
- criterion: P4
- place: core/src/linear/proofs/check/pass.rs:176
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** The derivation's record and the size estimate choose an `Instancing` (`Ground` or `Framed`), and then `examine` chooses again. This compiles the whole pass for observer/instancing pairs that never run together. At first order it also runs the frame pass twice for each pass of the checker.

**Witness.**

```rust
// check/pass.rs:184-189
    match proof.instances() {
        None => examine_with(proof, goal, mode, reading, allowance, observer, || Ground),
        Some(table) => examine_with(proof, goal, mode, reading, allowance, observer, || {
            Framed::new(proof, table)
        }),
    }
// size.rs:1003-1009, then size.rs:1040 calls check::examine
    match proof.instances() {
        None => measured_with(proof, &Ground, goal, mode, reading, allowance),
        Some(table) => {
            let inst = Framed::new(proof, table);
            measured_with(proof, &inst, goal, mode, reading, allowance)
        }
    }
// derivation/build.rs:104-107, then build.rs:144 calls check::examine
    match proof.instances() {
        None => made.unfold(proof, &Ground, stop),
        Some(table) => made.unfold(proof, &Framed::new(proof, table), stop),
    }
```

target/audit/llvm-lines.txt lists `Pass::rule` 12 times, at 3 422 to 3 428 lines each. The observers `()` and `cut::Record` give 2 × 2 pairs that can run. `build::Record<Ground|Framed>` and `size::Measure<Ground|Framed>` give 2 × 4 pairs, of which 4 are unreachable (`Record<Ground>` with `Framed`, and so on). Those four account for about 13 700 lines in `rule` alone, before `run`, `flow`, `premise` and `examine_with`. At first order, `Framed::new` builds four hash tables and walks the proof (frames.rs:179-263). A derivation at first order runs it four times (two passes, each with its own and the caller's), where two would do.

**Change.** Let a caller that already holds its `Instancing` pass it to the pass (an `examine_with` taking the caller's instancing, or the observer generic over the same `I` as the pass), so that each observer is compiled once per instancing and `Framed::new` runs once per pass.

**Value.** Less code in the WebAssembly module, which every visitor downloads (B6, B7), and a faster first-order derivation, size and cut step.

**Net.** It touches how the checker is entered, not its rules. The oracle's test, `first_order.rs`'s tests, `is_the_size_of_the_derivation_built`, and the ratchet's `check-*`, `derivation-*` and `render-*` journeys. To measure: `cargo llvm-lines` before and after, the `budget` check's gzip bytes of the module, and a first-order derivation of `first_order::cases()` under callgrind (no journey has one).

## F-1b-04

- dimension: api
- criterion: D4
- place: core/src/linear/proofs/style.rs:53
- kind: API break
- cost: medium
- candidate: no

**Claim.** `Labels::Table`, the one public way to give rules your own labels, is keyed by the linear rule `Named`. Four of the five drawings silently ignore it: the classical two-sided view, ordinary logic, lambda-terms and rule files.

**Witness.**

```rust
// style.rs:53
    Table(BTreeMap<Named, String>),
// derivation/classical.rs:837
            Labels::Upright | Labels::Table(_) => rule.markup(false),
// ordinary/derivation.rs:1162
            Labels::Upright | Labels::Table(_) => Some(rule.markup(false)),
// lambda/draw.rs:609 and rules/draw.rs:288
            Labels::Upright | Labels::Subscript | Labels::Table(_) => {
```

The exports document the gap (export/latex.rs:647, export/typst.rs:726, export/svg/mod.rs:1441).

**Change.** Key the table by the rule's name as the derivation writes it (`Drawn::name`: `⊸L`, `→L`, `∀R`, a rule file's own name), as `BTreeMap<String, String>`. Each `Drawn::markup` then looks its own names up. The JSON form `{"table": {"⊗": …}}` keeps its shape.

**Value.** A user who exports LK, LJ, a classical two-sided view, a lambda typing or a rule file's derivation gets the labels they asked for, as a user of linear logic does.

**Net.** The export test and snapshots, `every_label`, and the behaviour lock for `--labels`. No checker, search or certificate is touched.

## F-1b-05

- dimension: architecture
- criterion: E6
- place: core/src/linear/proofs/check/pass.rs:790
- kind: refactor
- cost: small
- candidate: no

**Claim.** The pass changes the weight sum of the unrestricted zone (`State::unrestricted`) with plain `+=` and `-=` inline in `Pass`. These are outside the `State` functions Kani's harnesses call, and the harnesses fix that sum at 0. So the rules file's claim that "the arithmetic of the pass lives in functions the harnesses call" does not hold for it.

**Witness.**

```rust
// pass.rs:789-791 (Pass::unite)
            if l.theta.insert(a) {
                l.unrestricted += self.quest_weight(a);
// pass.rs:1014 (the `?` rule)
                    d.unrestricted -= u64::from(self.observer.weight(m, o));
// pass.rs:1027 (the copy rule)
                    d.unrestricted += self.quest_weight(a);
// harnesses.rs:74 and :197
        unrestricted: 0,
```

The rule in `core-proofs.md:260-263` reads: "The arithmetic of the pass lives in functions the harnesses call (`State::put`, `remove`, `add_linear`, `recount`, `pass::room`, `table_bytes`): a change of it goes there". The `-=` holds only if the weight added for a copied member, under `Framed::parent`'s fallback `unwrap_or(Member::from(p))` (frames.rs:302), equals the weight the `?` step subtracts for its own member. A release build would wrap where a debug build panics.

**Change.** Move the three updates into `State` methods (for example `add_unrestricted` and `remove_unrestricted`), as `put` and `remove` are for the linear zone. Add a harness that puts and takes a copied member with any weights.

**Value.** A reader and Kani check the size estimate's input sums where the rules file says they are checked. The `derivation_bytes` bound rests on these sums.

**Net.** It touches the checker's arithmetic, not its rules. Kani's `kani-harnesses` package, the oracle's test, `is_the_size_of_the_derivation_built`, and the mutation baseline. The size, the derivation and the check give the same results.

## F-1b-06

- dimension: api
- criterion: E6
- place: core/src/linear/proofs/derivation/classical.rs:28
- kind: API break
- cost: medium
- candidate: no

**Claim.** `ClassicalRule` spells out every pair of a one-sided rule and a side as its own variant (37 of them). Six parallel tables map it back. This is the shape that `core-proofs.md`'s decision removed from `Rule`/`Named`.

**Witness.**

```rust
// classical.rs:217-222 (one of the six tables: ALL, UPRIGHT, SUBSCRIPT, name, rule, side, applied)
    pub const fn rule(self) -> Option<Rule> {
        use ClassicalRule::*;
        Some(match self {
            Axiom => Rule::Ax,
            TensorRight | ParLeft | LollipopLeft => Rule::Tensor,
            TensorLeft | ParRight | LollipopRight => Rule::Par,
```

The decision (`core-proofs.md:671-674`): "`Rule` is a one-sided rule, and `Named` adds the side of `⊢`: a two-sided rule name is a `(rule, side)` pair, not a variant. Consumers lose their unreachable arms, and a new connective adds one rule, not three." `ClassicalRule` came later and was not brought under it. Its names differ from `Named`'s (`⊗` on the input side is `⅋L` here, `⊸L` there), which explains why it is a type of its own, not why it is a flat enum.

**Change.** Make `ClassicalRule` a struct of the one-sided rule (`None` for a negation step), the side, and whether an implication was written, with `name()` and the label tables keyed as `Named`'s are.

**Value.** A front end that matches on the classical view's rules writes one arm per rule, not per side. A new connective or quantifier adds one row, where today it adds four variants and six table entries.

**Net.** `every_inference_parses_back_and_reads_as_its_term`, `rules_are_labelled_in_declaration_order`, the export snapshots (`first_order_classical`), and the behaviour lock for `--classical two`. No checker, search or certificate is touched.

## F-1b-07

- dimension: architecture
- criterion: E6
- place: core/src/linear/proofs/mod.rs:679
- kind: refactor
- cost: small
- candidate: no

**Claim.** `Proof::formula`, `Derivation::formula` and `Interactive::formula` are three copies of one function. Each builds a `String` and returns it as `impl Display + '_`, and only the first cuts an instance's terms.

**Witness.**

```rust
// proofs/mod.rs:679, derivation/view.rs:720, interactive/mod.rs:824
    pub fn formula(&self, member: Member) -> impl Display + '_ {
// the same body in all three:
            let mut text = String::new();
            crate::linear::sequents::notation::TEXT
                .instance(
                    &mut text,
            ...
            return text;
        }
        match reading {
            Some(reading) => reading.formula(member.occ()).to_string(),
            None => self.forest.formula(member.occ()).to_string(),
        }
```

`Proof::formula` passes `Some(crate::errors::describe::TERM)` (mod.rs:696). The other two pass `None` (view.rs:732, interactive/mod.rs:840). Clippy's `redundant_clone` fires on `.formula(m).to_string()` at classical.rs:1289, size.rs:1120 and first_order.rs:972, because the `Display` value is already a `String`. Callers in cli/src/eliminate.rs:748 and web/src/calls.rs:1906 copy it again with `to_string()`. `Proof::formula` also recomputes the reading, a pass over the forest, on every call (mod.rs:680-683).

**Change.** Write one crate-private function of the forest, the reading, the framing, the member and the cut. Have the three call it, and either return a lazy `Display` type, as `Forest::formula` returns `Formula<'_>`, or return `String` and say so.

**Value.** A reader changes one function. A caller gets one allocation per formula, not two, and the same cut of long terms wherever it asks.

**Net.** `a_derivation_writes_instances`, `a_first_order_view_reads_as_its_term`, `ill.rs`'s checks of formulas, and the behaviour lock (`interact` and `eliminate` print these). No checker is touched.

## F-1b-08

- dimension: api
- criterion: C-HIDDEN
- place: core/src/linear/proofs/mod.rs:21
- kind: API break
- cost: small
- candidate: no

**Claim.** The modules `check`, `derivation`, `size` and `style` are public, and their items are re-exported at `proofs::` and again at `linear::`. So the unit's types are reachable under two or three paths each.

**Witness.**

```rust
pub mod check;
pub mod cut;
pub mod derivation;
...
pub mod size;
pub mod style;
...
pub use check::{CheckError, Dyadic, Fault, Invalid, Refused};
pub use derivation::{
    ClassicalRule, ClassicalSides, Compact, Derivation, InfId, Inference, Layout, Sides,
    Structural, TwoSided, ViewOptions,
};
```

target/audit/surface.md ("Items reachable under several paths") lists 27 of this unit's types: `CheckError`, `Derivation`, `InfId`, `Inference`, `Size`, `TwoSided` and `ViewOptions` under three paths each, and `Fault`, `Invalid`, `Refused`, `Dyadic`, `Compact`, `Sides`, `Labels`, `OpenGoal` and others under two. This is consumer test point 9. Outside `core/src` only web/src/calls.rs:1254 and :1304-1305 write a nested path (`proofs::derivation::InfId`).

**Change.** Make the four modules private, or `pub(crate)`, and keep the re-exports at `proofs::`. Move the label markup's description, which docs link as `crate::linear::proofs::style`, to `Labels`' own doc.

**Value.** Rustdoc and the compiler's messages show each type at one path, and the listing of public items shrinks by about 30 lines.

**Net.** The doc check (intra-doc links), README's and the manual's examples, `web-wasm-tests`. No checker is touched.

## F-1b-09

- dimension: api
- criterion: X2
- place: core/src/linear/proofs/mod.rs:443
- kind: API break
- cost: small
- candidate: no

**Claim.** An outside engine cannot bound or stop the building of its proof. `Proof::new` polls a stop that never fires, and the variant that takes the engine's stop, `Proof::build`, is crate-private. Yet the code says an engine's proof can have hundreds of millions of nodes.

**Witness.**

```rust
    pub fn new(forest: Forest, nodes: Vec<Node>, root: NodeId) -> Result<Self, Error> {
        if forest.has_binders() {
            let instances = Instances::new(&forest);
            return Self::with_instances(forest, instances, nodes, root);
        }
        Self::build(forest, nodes, root, &mut || false)
    }
```

mod.rs:492-494 (on `build`): "Both passes call `stopped` every [`BUILD_POLL`] nodes, since an engine's proof can have hundreds of millions of nodes." `build` is `pub(crate)` (mod.rs:500).

**Change.** Give `new`, `new_of_goal` and `with_instances` a `_within` form that takes `stop: impl FnMut(Progress) -> bool`, as every other long call does (core.md, Decisions). The crate's engines would use it too.

**Value.** A researcher's engine (X2, B5) refuses at the caller's stop as the crate's engines do.

**Net.** `construction`, Kani's `a_kept_proof_is_ordered_and_reached` (it calls `Proof::reach`), the readme test if the how-to uses it. No checker rule is touched. The proof term's building is not the check.

## F-1b-10

- dimension: api
- criterion: C-COMMON-TRAITS
- place: core/src/linear/proofs/mod.rs:414
- kind: API break
- cost: small
- candidate: no

**Claim.** `Proof` implements neither `PartialEq` nor `Eq`, and neither does its `Forest`. So a user, and the crate's own tests, compare two proofs field by field.

**Witness.**

```rust
// proofs/mod.rs:414
#[derive(Clone, Debug)]
pub struct Proof {
// first_order.rs:1137-1147
            assert_eq!(read.nodes(), case.proof.nodes(), "{}", case.name);
            assert_eq!(read.instances(), case.proof.instances(), "{}", case.name);
            ...
            assert_eq!(read.mode(), case.proof.mode(), "{}", case.name);
```

`Forest` derives `Clone, Debug` only (core/src/linear/occurrences/mod.rs:251). `Sequent` and `Instances` have `PartialEq, Eq`.

**Change.** Implement `PartialEq` and `Eq` on `Forest` (by its sequent, from which it is built deterministically) and derive them on `Proof`.

**Value.** A user checks a round trip through JSON, or compares an engine's proof with the crate's, in one `assert_eq!`. The guideline asks for every trait that fits, since no other crate can add one.

**Net.** Additive. No behaviour changes. The tests compile and pass.

## F-1b-11

- dimension: api
- criterion: C-CALLER-CONTROL
- place: core/src/linear/proofs/mod.rs:470
- kind: API break
- cost: small
- candidate: no

**Claim.** `Proof::with_instances` takes the instance table by value but only reads it, so a caller that keeps its table must clone it.

**Witness.**

```rust
    pub fn with_instances(
        forest: Forest,
        instances: Instances,
        nodes: Vec<Node>,
        root: NodeId,
    ) -> Result<Self, Error> {
        ...
        let table = instances::canonical(&proof.forest, &instances, &mut nodes)?;
```

Clippy's allowed `needless_pass_by_value` fires at mod.rs:472. The crate's own mutant loop clones the table for every mutant (first_order.rs:696-701: `Proof::with_instances(proof.forest().clone(), table.clone(), nodes, proof.root())`).

**Change.** Take `instances: &Instances`.

**Value.** An engine that builds many candidate proofs over one table, as a backtracking search does, does not copy the table each time.

**Net.** `first_order.rs`'s tests and the JSON reader (`serialize/proofs.rs`). No checker is touched.

## F-1b-12

- dimension: architecture
- criterion: E6
- place: core/src/linear/proofs/rule.rs:498
- kind: refactor
- cost: small
- candidate: no

**Claim.** Rule names are spelled in parallel tables kept in step by hand. Inside `rule.rs`, `KNOWN` and the Unicode half of `FromStr` repeat what `Named::name` returns for every entry of `Named::ALL`.

**Witness.**

```rust
// rule.rs:498-502
const KNOWN: &[&str] = &[
    "ax", "⊗", "⅋", "1", "⊥", "&", "⊕₁", "⊕₂", "⊤", "!", "?d", "?c", "?w", "mix", "wk", "cut", "∀",
    "∃", "⊸L", "⊸R", "⊗L", "⊗R", "&L₁", "&L₂", "&R", "⊕L", "⊕R₁", "⊕R₂", "1L", "1R", "0L", "⊤R",
// rule.rs:435 already reads the divisions off their names:
        if let Some(&named) = Self::DIVISIONS.iter().find(|named| named.name() == text) {
```

The same names stand again in `Named::name` (rule.rs:305-340), `FromStr`'s match (rule.rs:438-479), `Rule::name` (rule.rs:83-106), `Node::TAGS` and `Node::name` (mod.rs:205-208, :295-316), and `style.rs`'s `UPRIGHT` and `SUBSCRIPT` (style.rs:62-74). Tests pin them (`names_round_trip`, `name_lists_follow_the_variants`).

**Change.** Read the Unicode names in `FromStr` off `Named::ALL` and `DIVISIONS`, as the divisions are read today, and keep only the ASCII spellings in a match. Build `KNOWN` from the same lists in a `const` block, since `name` is a `const fn`.

**Value.** A new rule is named in one place, not three, within the file a reader changes for it.

**Net.** `names_round_trip` in `rule.rs` and in `style.rs`, the behaviour lock for `--labels` and session commands that read rule names. No checker is touched.

## F-1b-13

- dimension: performance
- criterion: P6
- place: core/src/linear/proofs/size.rs:30
- kind: build
- cost: small
- candidate: no

**Claim.** The crate keeps a fast hasher for its tables (`crate::hash`, foldhash), but 13 places still build `std::collections` tables with SipHash, and nothing stops a new one.

**Witness.**

```rust
use std::collections::{HashMap, HashSet};
```

The other places: core/src/first_order.rs:598, unify.rs:15, linear/occurrences/instances.rs:947, linear/proofs/interactive/quantified.rs:959 and :984, linear/search/reference.rs:44, linear/sequents/notation.rs:236, modal/model.rs:273, ordinary/model.rs:299, ordinary/instance.rs:771, ordinary/normal.rs:71, lambda/tests.rs:503. The repository has no `clippy.toml`.

**Change.** Use `crate::hash::{HashMap, HashSet}` at these places. Add a `clippy.toml` whose `disallowed-types` names `std::collections::HashMap` and `HashSet`, the means the performance book gives.

**Value.** One hasher everywhere, and a rule a machine checks, not a reviewer.

**Net.** The gate's Clippy run. The places in this unit (`printed_names` in `size.rs`) are no hot path, so the ratchet should not move. `bench/` and `cli/` would need the same type or an exemption.

## F-1b-14

- dimension: text
- criterion: S11
- place: core/src/linear/proofs/style.rs:21
- kind: text
- cost: small
- candidate: no

**Claim.** The public module doc of `style`, and the doc of `Drawn`, say that the linear derivations and those of LK and LJ implement `Drawn`. Five types implement it. The module doc also names a crate-private trait to a reader who cannot use it.

**Witness.**

```rust
//! Every output draws a derivation through the crate-private trait
//! `Drawn`, which the linear derivations and those of LK and LJ
//! implement.
```

style.rs:227-230 says the same. The implementors are style.rs:318, derivation/classical.rs:823, ordinary/derivation.rs:1149, lambda/draw.rs:597 and rules/draw.rs:276. fmt.rs:10-11 says "the linear ones and those of LK and LJ" too.

**Change.** In the public module doc write "Every output draws the derivations of every suite the same way: proofs of linear logic, LK and LJ, lambda-terms and rule files." Keep the trait's list in its own private doc, and keep it complete.

**Value.** A reader learns which derivations the exports draw, without the name of a private trait.

**Net.** The doc check. Nothing else.

## F-1b-15

- dimension: performance
- criterion: P5
- place: core/src/linear/proofs/derivation/view.rs:343
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Every inference of a derivation (88 bytes) allocates a vector for its premises, though a rule has two at most. It also stores its principal position as an `Option<usize>` of 16 bytes.

**Witness.**

```rust
    pub(crate) principal: Option<usize>,
    /// The premises, in the rule's order.
    pub(crate) premises: Vec<InfId>,
```

target/audit/type-sizes.txt:42611-42619: `Inference` is 88 bytes, `.principal` 16, `.premises` 24. The builder makes each premise list with `self.done.split_off(self.done.len() - premises)` (build.rs:458). DHAT on `derivation-chain-64` shows `split_off<InfId>` at 130 of 2 873 blocks. `Rule::premises` returns 0, 1 or 2 (rule.rs:121-129).

**Change.** Store the premises inline (`[InfId; 2]` with a count, or `ArrayVec`) and the principal as an `Option<u32>`. The accessors `premises() -> &[InfId]` and `principal() -> Option<usize>` stay as they are.

**Value.** One allocation fewer per inference, and a smaller derivation in memory, for every text tree, export and session view.

**Net.** The derivation tests, the export snapshots, and the `derivation-*` and `render-*` journeys under the ratchet. No checker is touched. To measure: DHAT and callgrind on `derivation-chain-64` and `render-latex`.
