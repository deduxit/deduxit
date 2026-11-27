# Unit 4b: findings

Read in full: 10 files. Not read in full: none.

The finder's remarks: Every performance claim here stays a candidate because no journey runs this unit's engines (F-4b-03); the leader should measure F-4b-09, F-4b-12, F-4b-13 and F-4b-14 once such journeys exist. I did not read `written/tests.rs` or the callers in `connection/` and `herbrand.rs` in full: F-4b-01 names their builders from the lines I opened, so unit 4c should confirm their charge rules and whether `herbrand.rs:1176` and `connection/read_back.rs:514` can take the shared builder. Unit 1a already listed the SipHash sites of `normal.rs:71`. They are cold (one rewrite or claim at a time), and F-4b-06 and F-4b-07 remove all five. Besides F-arch-06's `memory::Account` (ljt, all of `sat/`), this unit names these linear items: `linear::search::Reason` (ljt, sat, normal) and `linear::proofs::InfId` as the index type of every ordinary derivation (ljt, read_back). It also names `linear::proofs::{GoalId, Structural, ViewOptions}` in the tableau, and `linear::search::Options` as a public parameter (F-4b-15). A shared module needs only `InfId`'s u32 newtype, `Reason`'s variants and the view options' structural switch from these. F-arch-13 holds here too: `tableau.rs:889-896` and `:929-936` write `countermodel: None, structure: None, kripke: None`. F-4a-13 (the `_default` names) and F-4b-02 meet at the same functions: renaming them to `X`/`X_within` with a `&mut dyn` body fixes both.

## F-4b-01

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/ljt.rs:931; core/src/ordinary/sat/read_back.rs:840; core/src/ordinary/connection/intuitionistic/read_back.rs:1233; core/src/ordinary/connection/read_back.rs:514; core/src/ordinary/herbrand.rs:1176; core/src/ordinary/derivation.rs:1779
- kind: refactor
- cost: large
- candidate: no

**Claim.** Six private builders of a derivation of LK or LJ each write the same `push` (ask the stop, charge memory by an estimate of its own, refuse past 2³² inferences) and four of them the same `weaken_to`, `contract_to`, `in_order` and multiset `difference`, while no public way exists for an outside engine to build such a derivation for the checker.

**Witness.** The two copies in this unit (duplicates.md: `ljt.rs:958` ↔ `sat/read_back.rs:855`, 227 tokens; `ljt.rs:926` ↔ `sat/read_back.rs:836`):

```rust
// core/src/ordinary/ljt.rs:939 and core/src/ordinary/sat/read_back.rs:848, identical
        let n = self.inferences.len() as u64;
        if (self.stop)(Progress::new(Phase::ReadBack, 1, n + 1)) {
            return Err(Error::Refused(Refusal::Stopped {
                phase: Phase::ReadBack,
            }));
        }
```

The same `Refusal::Index { what: Space::Inference, count: n + 1, most: u64::from(u32::MAX) }` is at `ljt.rs:969`, `sat/read_back.rs:866`, `connection/read_back.rs:531`, `connection/intuitionistic/read_back.rs:1265`, `herbrand.rs:1208`, `derivation.rs:1799`. `weaken_to`/`in_order`/`difference` are at `ljt.rs:1032/1059/1330`, `sat/read_back.rs:893/954/782`, `connection/intuitionistic/read_back.rs:1283/1327/1197`. The memory charge per inference differs in each: `(l + r + p) * 4` plus capacity (`ljt.rs:957`), `size_of::<Inference>() + (l + r) * 4 + p * 4` (`connection/intuitionistic/read_back.rs:1249`), `2 * size_of::<Inference>()` plus 16-byte-rounded blocks (`herbrand.rs:1196`). Meanwhile `Derivation::from_parts` is `pub(crate)` (`derivation.rs:605`) and every field of `Inference` is `pub(crate)` (`derivation.rs:400-416`).

**Change.** Write one `ordinary::derivation::Builder` (inferences, account, stop as `&mut dyn FnMut`, one charge rule) with `push`, `axiom`, `weaken_to`, `contract_to`, `in_order` and `finish` returning an unchecked `Derivation`; the six engines use it. Once it exists, make it public so that an outside engine builds a derivation that `Derivation::check` then decides (X3).

**Value.** A reader finds one place for how a derivation is built and bounded; the memory bound means one thing in every engine; an outside engine of ordinary logic gets the constructor the consumer test never reached (its list of what it did not cover names `ordinary::Derivation`).

**Net.** The behaviour lock, the ILTP verdict rows of bench/RESULTS.md, `a_planted_wrong_translation_is_refused` (ljt) and each read-back's tests. The checker is untouched, but unifying the charge moves the point where `Refusal::Memory` refuses a read-back, so tests that pin a memory refusal may move; say so in the commit. No search is touched.

## F-4b-02

- dimension: performance
- criterion: P4
- place: core/src/ordinary/interpolant.rs:784; core/src/ordinary/interpolant.rs:385; core/src/ordinary/normal.rs:1074
- kind: refactor
- cost: small
- candidate: no

**Claim.** Maehara's method, the interpolant's check and `NormalForm::check` are generic over the stop, so each is compiled twice, the second copy for the closure of the `_default` helper, against the crate's rule that a long body takes its stop as `&mut dyn FnMut`.

**Witness.** `target/audit/llvm-lines.txt`:

```text
79:  2089 ... ordinary::interpolant::maehara::<&mut &mut dyn core::ops::function::FnMut<(Progress,), Output = bool>>
80:  2089 ... ordinary::interpolant::maehara::<&mut ordinary::interpolant::interpolate_default::{closure#0}>
109: 1822 ... ordinary::interpolant::checked::<&mut &mut dyn FnMut<...>>
110: 1822 ... ordinary::interpolant::checked::<&mut ordinary::interpolant::interpolate_default::{closure#0}>
192: 1360 ... <ordinary::normal::NormalForm>::check::<&mut &mut &mut dyn FnMut<...>>
193: 1360 ... <ordinary::normal::NormalForm>::check::<&mut ordinary::normal::normalize_default::{closure#0}>
```

```rust
// core/src/ordinary/interpolant.rs:784
fn maehara(
    derivation: &Derivation,
    root: Parts,
    out: &mut Formulas,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<NodeId, Error> {
```

where `normal::rewrite` already shims (`rewrite_dyn`, normal.rs:2314) and `core.md` records the rule (129 KB off the web module).

**Change.** Give `maehara` and `checked` a `stop: &mut dyn FnMut(Progress) -> bool`, and split `NormalForm::check` into a one-line generic shim over a private `check_dyn`, as `rewrite` and `WrittenTableau::review` do.

**Value.** The machine: 5 271 LLVM lines fewer in the native build (2 089 + 1 822 + 1 360), and one copy per further caller fewer in the web module; compile time.

**Net.** `cargo llvm-lines` shows one copy of each; the interpolant and normal-form tests and the examples of the manual's transformations chapter. No search, checker or certificate is touched.

## F-4b-03

- dimension: performance
- criterion: P1
- place: bench/src/journeys.rs:137
- kind: build
- cost: medium
- candidate: no

**Claim.** No journey of the ratchet or of DHAT runs any engine of this unit: `ordinary-pigeons` decides through the embedding, so LJT, the SAT method (the default of intuitionistic and minimal logic), the classical SAT method with its DRUP check, the normal forms, interpolation and the tableau have no instrument.

**Witness.** ```rust
// bench/src/journeys.rs:473
/// Decides a classical pigeonhole formula through its translation, reads
/// the proof back to LK and checks it.
fn ordinary() -> Result<()> {
```

The journeys are `search-*`, `read-*`, `check-*`, `derivation-chain-64`, `render-*`, `batch-families` and `ordinary-pigeons` (`journeys.rs:42-137`). `cli/src/ordinary.rs:400` calls `sat::decide_preferring_ljt` for the default method, which no journey reaches. Every performance claim on these engines (F-4b-09, F-4b-12, F-4b-13, F-4b-14) is therefore a candidate.

**Change.** Add journeys: `ordinary-sat-pigeons` (intuitionistic, `decide_preferring_ljt` on the SYJ202+1.005 text the test `pigeons_tptp` builds), `ordinary-ljt-korn` (LJT on Korn and Kreitz's family at size 20, from `the_benchmark_families_stay_small`), `ordinary-drup-pigeons` (classical SAT on `pigeons(7)`, which deletes clauses), and one of `normalize` to CNF; record their ceilings.

**Value.** The maintainers: the default method of two logics, the clausal checker and the normal forms get a ratchet and DHAT figures, so a later change to them is measured (P1) and the candidates here can be decided.

**Net.** New ceilings in bench/ceilings.csv in a commit of their own that says why; nothing in core changes.

## F-4b-04

- dimension: api
- criterion: X3
- place: core/src/ordinary/normal.rs:632; core/src/ordinary/interpolant.rs:176
- kind: API break
- cost: medium
- candidate: no

**Claim.** `NormalForm` and `Interpolant` keep their result and their checks in public mutable fields, so after `check` a caller can replace the result (or swap an outcome) and `is_checked()` still answers true for a value no check saw.

**Witness.** ```rust
// core/src/ordinary/normal.rs:632
pub struct NormalForm {
    pub form: Form,
    pub sequent: Sequent,
    pub steps: Vec<Step>,
    pub result: Sequent,
    pub checks: Vec<Checked>,
}
...
    pub fn is_checked(&self) -> bool {
        !self.checks.is_empty()
            && self
                .checks
                .iter()
                .all(|c| matches!(c.outcome.verdict, Verdict::Valid(_)))
```

(doc comments elided). The crate's own test does the mutation (`normal.rs:3348`: `cnf.result = wrong;`), and `claims` guards against it only when `check` runs again (`normal.rs:675`: "Its fields are public, so a first-order sequent can be set"). `Interpolant` (`interpolant.rs:179-186`) has `pub first`, `pub second`, `pub first_outcome`, `pub second_outcome`, and `is_checked` (`:204`) reads the two outcomes. The recorded acceptance for `Evidence` (core-ordinary.md:1589-1593) is about wrapping a proof that was checked; here `is_checked` vouches for `result` itself, which can change after the check.

**Change.** Make the fields private with getters (`form()`, `sequent()`, `steps()`, `result()`, `checks()`; `first()`, `second()`, `first_outcome()`, `second_outcome()`), so the only way to a value whose `is_checked()` is true is `check` or `normalize` and `interpolant`/`interpolate`.

**Value.** A library user who trusts `is_checked()` (the command's exit 3 rests on it) can rely on it for the value in hand; the checks stay the trust boundary (X3).

**Net.** The normal-form and interpolant tests (the one that sets `result` by hand becomes a test of `claims` on a value built in the crate), `cli/src/transform.rs:164-293` and the serializers read through getters; the manual's transformations examples; `breaks 0.8.0`. No checker logic changes.

## F-4b-05

- dimension: api
- criterion: C-VALIDATE
- place: core/src/ordinary/written/mod.rs:633
- kind: API break
- cost: small
- candidate: no

**Claim.** `WrittenTableau::review` takes any `rules::Calculus` and says only in its docs that it must be the built-in `TABLEAU`, so a standing of `Closed` rests on whatever rule file the caller passed, and each front end reads `TABLEAU` through serde_json itself.

**Witness.** ```rust
// core/src/ordinary/written/mod.rs:602
    /// Reviews the tableau rule by rule against `calculus`, which is the
    /// rule file [`TABLEAU`](crate::rules::TABLEAU) read, and, where
    /// `expected` is given, checks that it starts from that sequent.
...
    pub fn review(
        &self,
        calculus: &Calculus,
        expected: Option<&Sequent>,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<TableauReview, Error> {
```

The callers each build it: `cli/src/tableau.rs:85` (`crate::calculus::named("tableau".as_ref(), &limits)?`) and `web/src/ordinary.rs:811-813` (`serde_json::Deserializer::from_str(deduxit::rules::TABLEAU)` then `wire::upgrade`). `WrittenTableau` has no doc example (surface.md lists it among items without one), which this two-step setup makes hard to write. Nothing in `review_dyn` (:644-745) looks at `calculus` beyond handing it to `Derivation::check`.

**Change.** Validate the argument: refuse a calculus that is not the tableau's (its name and the rule names `shapes` writes, or a fingerprint of `TABLEAU`'s rules) with an error of kind unsupported, and add a doc example; or, if the author prefers, have `review` take no calculus and build `TABLEAU` once under the feature that already gates `written` (core-ordinary.md:1759 records why the caller passes it: serde_json is under `protocol` only).

**Value.** A library user cannot get `Closed` for a wrong tableau by passing another rule file by mistake, and finds the call in one example instead of reading two front ends.

**Net.** The written-tableau tests (`written/tests.rs`), the manual's grading and tableau examples via `cli/tests/readme.rs`, the bindings' tests under Node. The rule file's checker is unchanged; `review` gains a check before it.

## F-4b-06

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/sat/classical.rs:57; core/src/ordinary/normal.rs:2872; core/src/ordinary/sat/mod.rs:539
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The pass that gives every subformula its polarity is written three times, two of them the same function (the classical encoder's and the Plaisted–Greenbaum normal form's, the latter over a SipHash `HashMap`), the third differing only in `⊥` and `↔`.

**Witness.** ```rust
// core/src/ordinary/sat/classical.rs:72
        let flipped = ((p & TRUE) << 1) | ((p & FALSE) >> 1);
        match formulas.node(NodeId(id as u32)) {
            Node::Atom(_) | Node::True | Node::False => {}
            Node::Not(a) => polarity[a.index()] |= flipped,
```

```rust
// core/src/ordinary/normal.rs:2881
        let flipped = ((p & TRUE) << 1) | ((p & FALSE) >> 1);
        let mut give = |k: NodeId, q: u8| *polarity.entry(k).or_insert(0) |= q;
        match formulas.node(id) {
            Node::Atom(_) | Node::True | Node::False => {}
            Node::Not(a) => give(a, flipped),
```

```rust
// core/src/ordinary/sat/mod.rs:552
        let flipped = ((p & NEGATIVE) << 1) | ((p & POSITIVE) >> 1);
```

Each defines its own pair of flag constants (`TRUE`/`FALSE` at classical.rs:36-39 and normal.rs:2680-2682, `NEGATIVE`/`POSITIVE` at sat/mod.rs:143-146). core-ordinary.md:2349-2351 calls normal's "the classical encoder's rule". `normal.rs:2873` is one of the SipHash sites unit 1a listed.

**Change.** One `polarities(formulas, true_roots, false_roots) -> Vec<u8>` (a small flag type or two constants) in `ordinary`, outside the `sat` feature since `normal` needs it without it; the intuitionistic search keeps its two extra rules as a pass over that result or a flag.

**Value.** A reader checks Plaisted–Greenbaum's rule once; the normal form's check and the solver's encoding cannot drift; one SipHash map fewer.

**Net.** `agrees_with_the_truth_table_and_the_embedding` (classical SAT), `agrees_with_ljt` (intuitionistic SAT), `generated_sequents_keep_their_promises` (normal). It touches the classical encoder, which with the DRUP checker is the trust for a classical "valid" (core-ordinary.md:1484-1498): its soundness argument must be re-read against the shared function. No search order changes.

## F-4b-07

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/normal.rs:1338; core/src/ordinary/normal.rs:1437; core/src/ordinary/written/mod.rs:769; core/src/ordinary/transfer.rs:74
- kind: refactor
- cost: medium
- candidate: no

**Claim.** Copying formulas from one arena into another by atom and symbol names is written three more times in this unit beside the crate's `transfer::copy`, which already takes a set of roots or the whole arena.

**Witness.** ```rust
// core/src/ordinary/transfer.rs:74
pub(super) fn copy(
    target: &mut Formulas,
    source: &Formulas,
    roots: Option<&[NodeId]>,
    parameters: &Parameters<'_>,
) -> Result<Copied, Error> {
```

```rust
// core/src/ordinary/normal.rs:1338
fn copy_formulas(
    from: &Formulas,
    roots: &[NodeId],
    to: &mut Formulas,
) -> Result<Vec<NodeId>, Error> {
    let tables = from.first_order();
    let mut symbols: HashMap<crate::first_order::Symbol, crate::first_order::Symbol> =
        HashMap::new();
    let mut terms: HashMap<FoTermId, FoTermId> = HashMap::new();
    let mut nodes: HashMap<NodeId, NodeId> = HashMap::new();
```

`normal::transfer` (`:1437`) is the same copy of the whole arena in index order; `WrittenTableau::differs` (`written/mod.rs:769-786`) copies the expected sequent's arena node by node (duplicates.md: `interactive.rs:2660` ↔ `written/mod.rs:776`). The three maps of `copy_formulas` are SipHash (`normal.rs:71`).

**Change.** Call `transfer::copy(…, Some(roots) | None, &Parameters::Constants)` from all three and keep normal's refusal of a step that holds an eigenvariable or open witness by refusing where `Copied::constants` is not empty.

**Value.** A reader of the normal forms and of the tableau review meets the copy they already know; three SipHash maps go; about 170 lines fewer.

**Net.** `a_law_s_claim_copies_its_formulas_alone`, `a_result_set_by_hand_is_held_to_the_steps`, the written-tableau tests for an expected sequent. The checks' claims are built in the copied arena, so a defect would show as `ClaimFails` in the generated tests; no search or certificate is touched.

## F-4b-08

- dimension: architecture
- criterion: E5
- place: core/src/ordinary/normal.rs:1
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `normal.rs` (3 688 lines) holds five things along clear seams: the public vocabulary and its check, the propositional rewriting, the first-order rewriting with Skolem and Herbrand terms, the clausal encodings, and the form test.

**Witness.** The seams in the file: the types and `NormalForm::{claims, skolem_fresh, replay, fresh, check}` at `:74-1183`; `Stage`, `law`, `apply`, `Passes`, `Budget`, `Run` at `:1548-2531` (propositional) with `quantifier_law`, `apply_quantifier` at `:1622-2096` inside; `skolem_term`, `negation_normal`, `fresh_prefix`, `skolemized` at `:2533-2677` beside `law_claim`, `copy_formulas`, `transfer`, `holds_symbol` at `:1204-1505`; the encodings `encoded`, `polarities`, `definition`, `constant` at `:2679-3007`; `in_form`, `prefixed`, `reaches`, `nnf` at `:3009-3151`; and a test module of first-order cases at `:3386`.

**Change.** Make `normal` a directory: `mod.rs` (the types, `rewrite`, `normalize`, the claims and their check), `rewrite.rs` (stages, laws, passes), `first_order.rs` (quantifier laws, Skolem and Herbrand, `law_claim`), `encode.rs` (Tseitin and Plaisted–Greenbaum), `form.rs` (`in_form`).

**Value.** A reader of the propositional forms (the protocol's `cnf`, `dnf`, `tseitin`) holds one file of a few hundred lines; the first-order part, which unit 4c's engines use (`negation_normal`, `fresh_prefix`), is found by name.

**Net.** A move only: the normal-form tests, the manual's `seq normalize` examples; no behaviour change.

## F-4b-09

- dimension: performance
- criterion: P8
- place: core/src/ordinary/tableau.rs:620
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Writing a tableau as text finds each node's parent by scanning every earlier node's children, so printing a tableau of n nodes costs about n²/2 scans.

**Witness.** ```rust
// core/src/ordinary/tableau.rs:620
    fn parent_rule(&self, index: usize) -> Option<(u32, Rule)> {
        let child = u32::try_from(index).ok()?;
        self.nodes[..index]
            .iter()
            .rev()
            .find(|node| node.children.contains(&child))
            .and_then(|node| node.expands)
    }
```

called once per node from `write_text` (`:568`: `let from = self.parent_rule(index);`), which `Display` calls. The method builds tableaux of 1.7 million inferences within its bounds (core-ordinary.md:1680, SYJ206+1.020).

**Change.** Carry the parent's `expands` on `write_text`'s stack (`(index, prefix, marker, from)`), as the walk already knows it when it pushes a child.

**Value.** A user of `prove --method tableau` gets a large tableau printed in time linear in its size.

**Net.** `a_tableau_closes_or_gives_a_countermodel` pins the text; the manual's tableau examples. To measure: time `deduxit prove --method tableau` on ILTP's SYJ206+1.012 with output, before and after (pinned time), since no journey prints a tableau (F-4b-03). No search or checker is touched.

## F-4b-10

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/sat/classical.rs:253; core/src/ordinary/sat/classical.rs:376
- kind: refactor
- cost: small
- candidate: no

**Claim.** `classical::decide` and `falsify` repeat the same steps from the encoding to the solver's answer: a variable per formula, every clause added with a poll each `POLL` clauses, one call, the statistics.

**Witness.** Both places (duplicates.md: `classical.rs:256` ↔ `:379`, 114 tokens):

```rust
// core/src/ordinary/sat/classical.rs:257 (decide) and :380 (falsify)
        for _ in 0..clauses.variables {
            solver.new_var();
        }
        for (n, clause) in clauses.clauses.iter().enumerate() {
            solver.add_clause(clause);
            if n % POLL == 0
                && let Some(reason) = solver.poll(POLL as u64)
            {
                return Err(reason);
            }
        }
        let answer = solver.solve(&[]);
        statistics.calls = solver.calls();
        statistics.conflicts = solver.conflicts();
```

They differ only in `Batsat::recording` against `Batsat::new` and in what they do with the answer.

**Change.** One private `solve(encoding, solver, statistics) -> Result<Answer, Reason>` that both call after making their solver.

**Value.** A reader of the clausal proof's provenance sees one path from the encoder's clauses to the solver; a change of the poll cadence is made once.

**Net.** The classical SAT tests (`the_search_and_the_check_keep_to_their_bounds` pins the poll and work counts), the finite model search's tests (unit 4c calls `falsify`). Touches the path whose clauses `ClausalProof::checked` checks, not the checker.

## F-4b-11

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/normal.rs:2592; core/src/ordinary/normal.rs:2710; core/src/ordinary/normal.rs:2625; core/src/ordinary/normal.rs:2732
- kind: refactor
- cost: small
- candidate: no

**Claim.** `normal.rs` writes two of its helpers twice: the fresh name prefix (`fresh_prefix` and `prefix`) and the loop that moves the formulas right of `⊢` left negated, each a `Negate` step.

**Witness.** ```rust
// core/src/ordinary/normal.rs:2710
fn prefix(formulas: &Formulas) -> String {
    let mut prefix = String::from("t");
    let taken = |prefix: &str| {
        formulas.atom_names().iter().any(|name| {
            name.strip_prefix(prefix)
                .is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|c| c.is_ascii_digit()))
```

is `fresh_prefix(formulas, "t")` (`:2592`) without the symbols, which a propositional arena has none of. The `Negate` loop at `:2625-2640` (in `skolemized`) and `:2732-2747` (in `encoded`) is the same 16 lines (duplicates.md: `normal.rs:2625` ↔ `:2732`).

**Change.** Call `fresh_prefix(&formulas, "t")` and delete `prefix`; move the `Negate` loop into one function that returns the new left side and its steps.

**Value.** A reader of the encodings and of Skolem's form reads one rule for new names and one for negating the right side.

**Net.** `new_atoms_avoid_the_input`, `skolem_symbols_avoid_the_input`, `each_form_writes_its_steps` pin names and steps. No behaviour change.

## F-4b-12

- dimension: performance
- criterion: P3
- place: core/src/ordinary/interpolant.rs:262
- kind: refactor
- cost: small
- candidate: yes

**Claim.** `interpolate` clones the whole derivation out of the outcome and then keeps the outcome too, so the derivation is held twice where a borrow would do.

**Witness.** ```rust
// core/src/ordinary/interpolant.rs:260
    let cut_free = |outcome: &Outcome| match &outcome.verdict {
        Verdict::Valid(evidence) => match evidence.derivation() {
            Ok(d) if !d.inferences().iter().any(|i| i.rule == Rule::Cut) => Ok(Some(d.clone())),
```

The clone is passed by reference to `interpolant(&derivation, …)` (`:295`) and the outcome holding the same derivation is moved into `Interpolation.outcome` (`:306`).

**Change.** Decide which outcome holds the cut-free derivation, call `interpolant` on `outcome.verdict`'s evidence by reference, then move that outcome into the `Interpolation`.

**Value.** A user interpolating a large valid sequent holds one derivation, not two, under the memory bound.

**Net.** The interpolant tests. To measure: DHAT bytes of `interpolate` on a valid sequent with a derivation of a few thousand inferences (no journey covers it, F-4b-03). No search or checker is touched.

## F-4b-13

- dimension: performance
- criterion: P2
- place: core/src/ordinary/ljt.rs:518
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Ordering LJT's `→→L` alternatives allocates a fresh hash set in every call of `heads`, which `levels` calls twice per hypothesis per round for up to one round per hypothesis, so one choice point can make thousands of allocations.

**Witness.** ```rust
// core/src/ordinary/ljt.rs:518
    fn heads(&self, x: NodeId, heads: &mut Vec<NodeId>) {
        let mut seen: HashSet<NodeId> = HashSet::default();
        let mut stack = vec![x];
```

```rust
// core/src/ordinary/ljt.rs:546
        for _ in 0..left.len() {
            let mut changed = false;
            for &h in left {
                let Some((a, c)) = self.split(h) else {
                    continue;
                };
                self.heads(c, &mut heads);
```

(and `self.heads(a, &mut heads)` at `:559`; `rank` calls it again per implication at `:496`).

**Change.** Keep the seen-set and stack as scratch in `Search` (cleared per call, or a stamped `Vec<u32>` over the arena as `normal::Passes` does), so `heads` allocates nothing in the steady state.

**Value.** The machine: fewer allocations in the search that the logic's default runs first (up to `PREFER_SEQUENTS` sequents) and that `Method::Ljt` runs alone.

**Net.** The order of alternatives is unchanged, so the search visits the same sequents: `ljt::Statistics` (sequents, memo hits) on `the_benchmark_families_stay_small` must be equal before and after. To measure: DHAT blocks and callgrind instructions of LJT on Korn and Kreitz's family at size 20 (the journey F-4b-03 proposes).

## F-4b-14

- dimension: performance
- criterion: P5
- place: core/src/ordinary/sat/drup.rs:342
- kind: refactor
- cost: small
- candidate: yes

**Claim.** The DRUP checker keeps every clause's literals twice, once in the clause and once as the key of the index that only deletions use, and charges both, so the check reaches the memory bound at about half the proof it could hold.

**Witness.** ```rust
// core/src/ordinary/sat/drup.rs:332
        self.account
            .charge(2 * literals.len() * size_of::<u32>() + CLAUSE_BYTES);
...
        self.index.entry(literals.clone()).or_default().push(id);
        self.clauses.push(Clause {
            literals,
            live: true,
        });
```

with `index: HashMap<Box<[u32]>, Vec<usize>>` (`:211`); the watch loop reorders `clause.literals` in place (`:477`, `:489`), which is why the key is a sorted copy.

**Change.** Key the index by a hash of the sorted literals (`u64`) to a list of clause ids, and confirm a deletion against a sorted copy made only at deletion time, so a kept clause's literals are stored once.

**Value.** A user deciding a large classical sequent by the SAT method gets its clausal proof checked under a smaller memory bound before `Refusal::Memory`.

**Net.** Touches the checker that is the trust for a classical "valid": `a_planted_wrong_step_is_refused`, `the_bounds_refuse_a_check`, `drat_trim_accepts_the_proof` (ignored, run by hand) and `agrees_with_the_truth_table_and_the_embedding` must pass; a hash collision must fall back to comparison, never to deleting another clause (a wrong deletion only weakens, so it stays sound but could refuse a correct proof). To measure: DHAT peak bytes of the check on `pigeons(7)`.

## F-4b-15

- dimension: api
- criterion: D4
- place: core/src/ordinary/normal.rs:3160; core/src/ordinary/normal.rs:1074; core/src/ordinary/interpolant.rs:249; core/src/ordinary/interpolant.rs:350
- kind: API break
- cost: medium
- candidate: no

**Claim.** Following F-arch-02: the public calls of normal forms and interpolation take the linear search's `Options` as a parameter, so a user who only wants a CNF or an interpolant must construct another suite's options.

**Witness.** ```rust
// core/src/ordinary/normal.rs:3160
pub fn normalize(
    sequent: &Sequent,
    options: &Options,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<NormalForm, Error> {
```

with `use crate::linear::search;` (`normal.rs:70`, `interpolant.rs:55`); the same parameter on `NormalForm::check` (`:1076`), `interpolate` (`interpolant.rs:253`) and `interpolant` (`:353`). The propositional claims are decided by the SAT method by default (`CLAIMS`, `normal.rs:1511`), which reads none of it.

**Change.** Once F-arch-02 moves the ordinary engines' options out of `linear::search::Options`, take the ordinary decision's options (or nothing, with the embedding's defaults) here too; until then, document that only the embedding reads it.

**Value.** A user of the normal forms and interpolants works in ordinary logic's terms alone (D4).

**Net.** The normal-form and interpolant tests, `cli/src/transform.rs`, the protocol's `ordinary.cnf`/`interpolate` calls and the bindings; `breaks 0.8.0`. No search changes.

## F-4b-16

- dimension: idiom
- criterion: E6
- place: core/src/ordinary/sat/mod.rs:498; core/src/ordinary/sat/mod.rs:458
- kind: refactor
- cost: small
- candidate: no

**Claim.** The two places that take LJT's derivation for the SAT method handle the same impossible evidences differently (one invents errors, one ignores them) and write the same defect message twice, and the release build makes a placeholder refusal of zeros that only a test path uses.

**Witness.** ```rust
// core/src/ordinary/sat/mod.rs:516 (by_ljt)
        // LJT never gives a clausal proof or an image's proof.
        Verdict::Valid(Evidence::Clausal(_)) => Err(Error::NoDerivation {
            evidence: "a checked clausal proof",
        }),
        Verdict::Valid(Evidence::Image) => Err(Error::NotBuilt),
        Verdict::Valid(Evidence::Table) => Err(Error::NoDerivation {
            evidence: "its truth table",
        }),
```

against `prefer_ljt` (`:489-490`), which maps the same three to `Ok(derivation)`; `"LJT refutes a sequent the SAT search proved"` is at `:484` and `:530`. At `:374-379` every valid decision builds `Refusal::Output { what: "derivation", estimate_bytes: 0, limit_bytes: 0, least_bytes: None }`, used only for `Derive::Ljt`, a variant under `#[cfg(all(test, feature = "parse"))]` (`:264`).

**Change.** One helper that turns LJT's outcome into `Option<(Derivation, Statistics)>` or the refutation defect, with the impossible evidences `unreachable!` as the crate does elsewhere for what a callee never returns; build the test-only refusal inside the test branch.

**Value.** A reader of the SAT method's fallback sees one rule for LJT's answer, and no error text that can never be shown.

**Net.** `ljt_gives_the_derivation_the_read_back_cannot`, `ljt_is_preferred_where_its_drawing_fits`, `ljt_is_given_up_where_its_search_runs_long`. No search or checker is touched.
