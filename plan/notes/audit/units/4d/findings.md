# Unit 4d: findings

Read in full: 16 files. Not read in full: none.

The finder's remarks: F-4d-01 is a correctness defect of a checker, shown by a run of the tree's debug binary (scratch file `/tmp/claude-1000/-home-tux-Projects-own-deduxit/efa0eac2-7b9e-491f-88c1-186bcdc487e8/scratchpad/eq.json`); the leader should treat it as a fix, not a refactor. I agree with F-arch-08, F-arch-13 and F-arch-21 from inside the code: the modal arena, parser and model check copy the ordinary ones as `core-modal.md:263-272` records, and `modal::Outcome::check` compares sequents by their printed text (`modal/decide.rs:134`), which F-4a-17's `Eq` would replace. The one SipHash site of this unit is `modal/model.rs:273` (cold), and `modal::decide`'s own doc example prints a modal model through `kripke::Model`'s `Display`, which says \"below\" (F-4a-12). D1 reaches modal too: `modal::Options::default()` is K. I could not measure any performance claim: no journey of the ratchet or DHAT runs modal logic, grounding, the model search or the Kripke search, so unit 2a's or the leader's journey list should gain one of each. `models::decide` taking `&linear::search::Options` for two bounds (`models.rs:151`) is another site of F-arch-02, and units 6b and 9 should read F-4d-08 and F-4d-09 with F-6b-05 and F-9-01.

## F-4d-01

- dimension: api
- criterion: S4
- place: core/src/ordinary/kripke_structure.rs:908
- kind: API break
- cost: small
- candidate: no

**Claim.** `KripkeCountermodel::check` reads `=` as an ordinary predicate and never refuses it, so it accepts a one-world model with no facts as a checked refutation of `⊢ a = a`, which `prove` proves valid.

**Witness.** The Kripke evaluator's atom has no equality case:

```rust
// core/src/ordinary/kripke_structure.rs:908
    fn atom(&mut self, a: u32, w: u32, env: &[u32]) -> Result<bool, Error> {
        let formulas = self.sequent.formulas();
        let tables = formulas.first_order();
        let (name, args): (&str, &[FoTermId]) = match tables.atoms.get(a as usize) {
            Some(&(symbol, args)) => (tables.symbols[symbol.index()].0.as_str(), tables.list(args)),
            None => (formulas.atom_name(a), &[]),
        };
```

while the finite structure's has one (`structure.rs:492-497`, `// Equality is identity in every structure.`) and refuses a listed `=` (`structure.rs:169`). `core-ordinary.md:595-596`: "A Kripke model with domains (step 76) must read `=` as identity at each world, or be refused beside `=`." `grep EQUALS kripke_structure.rs` finds nothing.

Run (the tree's `target/debug/deduxit`), with `eq.json` an ordinary outcome whose `kripke_structure` is `{"sequent": {"formulas":[{"V":0}],"atoms":[{"P":[1,[0,0]]}],"symbols":[["a",0],["=",2]],"fo":[{"F":[0,[]]}],"left":[],"right":[0]}, "model": {"worlds":[{"size":1}],"constants":[["a",0]],"world":0}}`:

```text
$ deduxit check eq.json
valid refutation of ⊢ a = a (intuitionistic logic): a Kripke model of 1 world refutes it
$ echo $?
0
$ deduxit prove --logic intuitionistic '|- a = a'
valid (intuitionistic logic by the cbn translation into ILL, two-sided engine), its last 3 hypotheses the equality axioms
```

**Change.** In `KripkeStructure::evaluate`, either refuse a sequent with `=` (`has_equality`) as `Error::Structure`, or read `=` as identity at each world and refuse a world that lists `=`, as `Structure` does (identity satisfies every equality axiom, so it is sound, only less complete). Add the forged case above as a test of the check.

**Value.** Every user of `KripkeCountermodel::check`, `deduxit check`, the protocol's re-check of `decides` and `export::rocq::write_kripke_structure`: the checker is the trust boundary (X3) and today certifies a false "not valid".

**Net.** Touches a checker. The model search never builds such a model (`models.rs:214` skips Kripke models with `=`), so the tests `kripke_models_decided_by_hand` and `kripke_models_agree_with_the_embedding` keep passing; a new test pins the refusal; the Rocq certificate path checks first, so it follows.

## F-4d-02

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/structure.rs:510 and core/src/ordinary/kripke_structure.rs:927
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The finite structure and the Kripke model with growing domains each have their own evaluator of one semantics: the term walk, the atom lookup, the work tick, the up-front pass over the sequent's terms and the `check` wrapper are written twice, and the copies have already drifted on `=` (F-4d-01).

**Witness.** The term walks are the same loop:

```rust
// core/src/ordinary/structure.rs:533-537
                    } else if applied {
                        // An application costs its arguments too: a
                        // function may take millions of them.
                        self.tick_by(1 + args.len() as u64)?;
                        let table = self.functions.get(name).ok_or_else(|| {
```

```rust
// core/src/ordinary/kripke_structure.rs:949-953
                    } else if applied {
                        // An application costs its arguments too: a
                        // function may take millions of them.
                        self.tick_by(1 + args.len() as u64)?;
                        let table = self.functions[w as usize].get(name).ok_or_else(|| {
```

`tick_by` is `structure.rs:463` and `kripke_structure.rs:890`; `atom` `:481` and `:908`; the memory estimate `:135-157` and `:212-240`; the terms pass `:192-260` and `:396-450`; `FiniteCountermodel::check` `:603-632` and `KripkeCountermodel::check` `:1019-1046`, both branching on a string (`if side == "hypothesis"`, `structure.rs:618`, `kripke_structure.rs:1037`). `duplicates.md` lists `kripke_structure.rs:392`↔`structure.rs:187`, `:930`↔`:512`, `:876`↔`:449`, `:959`↔`:543`, `:908`↔`:481`, `:425`↔`:221`. `From<Structure> for KripkeStructure` (`kripke_structure.rs:127`) and the test at `:1155-1175` already assert that the two agree on one world. `kripke_structure.rs` also polls the stop inline at `:299`, `:360` and `:495` beside its own `poll` (`:567`).

**Change.** Move the shared parts into one crate-private evaluator core over a world (constants, function tables, predicate sets, work counter, stop): `term`, `atom` with `=` as identity, `tick_by`, the up-front terms pass and the memory estimate. Keep the two formula walks, or run `Structure::refutes` as the one-world model in classical logic. That second option moves the work counts at the bound (ILTP's NLP problems meet 2²⁶), so it needs a check against them. Replace the string `side` with a two-variant enum.

**Value.** A reader, and the next change to first-order semantics (equality, sorts), made once; the equality defect of F-4d-01 could not have happened with one atom lookup.

**Net.** Touches two checkers (the trust of every first-order "not valid"): `function_tables_interpret_terms`, `a_table_that_does_not_fit_is_refused`, `a_model_that_breaks_a_rule_is_refused`, the agreement tests in `models.rs` and `grounding.rs`, and the `rocq-deduxit` certificate checks (`fo_classical_refutation`, `fo_kripke_refutation`). No search counter moves if the work counts are kept.

## F-4d-03

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/models.rs:605 and core/src/ordinary/kripke_search.rs:363; core/src/ordinary/grounding.rs:1282 and core/src/ordinary/models.rs:726
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The three SAT encodings of first-order models (grounding, the finite model search, the Kripke model search) each write their own copy of one writer: the bounded `tick`, `junction`, `over_tuples`, `Denotes`, the symmetry formulas, the function-table read-off, and the memoised walk of the normal form under a chain of bindings.

**Witness.** `duplicates.md`: `kripke_search.rs:358`↔`models.rs:599` (58 lines, the largest pair of the crate's list after one), `kripke_search.rs:173`↔`models.rs:518`, `:204`↔`:581`, `:1011`↔`:857`, `grounding.rs:1354`↔`models.rs:967`, `grounding.rs:1413`↔`models.rs:1002`.

```rust
// core/src/ordinary/models.rs:624-628 and kripke_search.rs:382-385
        // An odometer over the open arguments' elements.
        let mut choice = vec![0u32; open.len()];
        let mut disjuncts = Vec::new();
        loop {
            self.tick(stop)?;
```

The binding chain lookup is a function in `models.rs:726` and a closure in `grounding.rs:1282`, with the same body; the task enums `Enter`/`Expand`/`Join`/`Store` with a `memo: HashMap<(u32, Vec<u32>), NodeId>` are `grounding.rs:1292-1460` and `models.rs:919-1027`; `symmetry` is `kripke_search.rs:739` and inline at `models.rs:787-798`.

**Change.** A crate-private `ordinary::models::encode` module: a `Writer` core (arena, entries numbered by atom index, `held`/`most`/`memory`/`steps` and `tick_by`, `junction`, `over_tuples`, `Denotes`, `symmetry`, the function-table read-off) that the three encoders hold, and one memoised walk of the normal form generic over what a literal and a quantifier become.

**Value.** A reader of three files of 1 000 to 1 700 lines; a fix to a bound or a poll is made once (the panels found several such bounds missing one copy at a time, `core-ordinary.md:1256-1265`).

**Net.** No journey runs these searches; the net is `sequents_of_the_classes_decided_by_hand`, `grounding_agrees_with_the_connection_prover`, `sequents_decided_by_hand`, `kripke_models_decided_by_hand`, `kripke_models_agree_with_the_embedding`, and the statistics (`atoms`, `nodes`, `size`) of each, which must stay equal. Untrusted searches: every answer is checked after.

## F-4d-04

- dimension: performance
- criterion: P4
- place: core/src/modal/decide.rs:255
- kind: refactor
- cost: small
- candidate: no

**Claim.** `modal::decide_within` passes the caller's stop closure generically through the whole tableau and the derivation's emission, against the crate's rule that a long body takes its stop as `&mut dyn`, so the search is compiled once per caller.

**Witness.** ```rust
// core/src/modal/decide.rs:255-263
pub fn decide_within(
    sequent: &Sequent,
    options: &Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    let system = options.system;
    let mut search = Search::new(sequent, system, limits);
    let found = search.run(&mut stop);
```

`search.rs:458` `pub(super) fn run(&mut self, mut stop: impl FnMut(Progress) -> bool)`, and `tableau`, `saturate`, `split`, `witness`, `deepest`, `keep`, `blocker`, `model` all take `&mut impl FnMut`. `target/audit/llvm-lines.txt` shows two copies in the library alone:

```text
2261 ... <deduxit::modal::search::Search>::model::<&mut &mut dyn ...FnMut<(Progress,), Output = bool>>
2261 ... <deduxit::modal::search::Search>::model::<&mut deduxit::modal::decide::decide::{closure#0}>
1700 ... deduxit::modal::decide::derivation::<&mut dyn ...>
1700 ... deduxit::modal::decide::derivation::<deduxit::modal::decide::decide::{closure#0}>
1483 ... <Search>::keep::<&mut &mut dyn ...>
1483 ... <Search>::keep::<&mut deduxit::modal::decide::decide::{closure#0}>
```

The command adds a third (`cli/src/modal.rs:115`, `|_| halt()`). `core.md:327-336` records the rule and the 129 KB it took off the web module. `Model::truth` and `frame_fault` (`modal/model.rs:196`, `:225`) take `impl FnMut` into bodies of 110 lines the same way.

**Change.** Make `decide_within`, `Model::truth`, `Model::frame_fault` and `Countermodel::check` one-line shims that pass `&mut stop` to private bodies taking `&mut dyn FnMut(Progress) -> bool`, as `ordinary::decide_within` does.

**Value.** The web module and the command: one copy of the modal tableau instead of one per caller; compile time.

**Net.** `search_tests.rs`, `decide.rs`'s doc example, the modal statistics (`worlds`, `splits`, `blocked`, `work`) unchanged; the budget check (`web-package`, module bytes at gzip) shows the saving, which should be measured there. No search change.

## F-4d-05

- dimension: performance
- criterion: P2
- place: core/src/ordinary/grounding.rs:1394; core/src/ordinary/models.rs:541; core/src/ordinary/kripke_search.rs:196
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** The three encoders name every atom they make `g_N`, `m_N` or `k_N` and parse the number back out of the name to find its entry, though the atom's index in the arena is that number.

**Witness.** ```rust
// core/src/ordinary/models.rs:540-541
        let number = self.entries.len();
        let id = self.formulas.atom(&format!("m_{number}"))?;
```

```rust
// core/src/ordinary/models.rs:826-830
        for &a in true_atoms {
            let Some(number) = names[a as usize]
                .strip_prefix("m_")
                .and_then(|n| n.parse::<usize>().ok())
            else {
```

The same at `kripke_search.rs:196`/`:963-965` (`k_`) and `grounding.rs:1394`/`:1496-1502` (`g_`, there over the SAT model's world's atom names as `String`s, `:569`). In each writer every atom of the arena is made by that one call, so `a` already is the entry number. Each atom costs a `format!`, the name's NFC normalisation and identifier check (`Formulas::atom`), and the name stored twice (F-4a-06).

**Change.** Index `entries` by the atom index the solver returns (`models.rs`, `kripke_search.rs` get indices from `sat::falsify` already); for grounding, have `sat::decide` hand back the true atoms' indices too, and give the arena a crate-private unnamed atom (or a name made only when printed).

**Value.** The machine: fewer allocations per ground atom (ILTP's largest grounding decided in 2 s has 65 540 atoms, `core-ordinary.md:1175`), and a reader loses a round trip through text.

**Net.** No journey runs grounding or the model searches (handover, leader stage 6): measure with DHAT on `deduxit prove --logic classical --method grounding` over ILTP's SYJ212+1.010 and `--method models` on a problem the model search refutes, before and after. Statistics `atoms` and `nodes` stay; the structures are checked after.

## F-4d-06

- dimension: performance
- criterion: P2
- place: core/src/ordinary/models.rs:555; core/src/ordinary/kripke_search.rs:247; core/src/ordinary/grounding.rs:1365
- kind: refactor
- cost: small
- candidate: yes

**Claim.** The encoders' memo tables are keyed by `(u32, Vec<u32>)`, so every lookup, a hit included, allocates the key with `to_vec` or `clone`, and an insert allocates it twice more.

**Witness.** ```rust
// core/src/ordinary/models.rs:555-561
    fn holds(&mut self, predicate: u32, tuple: &[u32]) -> Result<NodeId, Error> {
        if let Some(&id) = self.holds.get(&(predicate, tuple.to_vec())) {
            return Ok(id);
        }
        let id = self.atom(Entry::Holds(predicate, tuple.to_vec()))?;
        self.holds.insert((predicate, tuple.to_vec()), id);
```

The same in `models.rs:567` and `:579-580` (`values`, which also clones the whole value row on a hit, `:568`), `:668-676` and `:978`; `kripke_search.rs:248`, `:253`, `:264`, `:319`, `:352`, `:354`, `:803`; `grounding.rs:220`, `:236`, `:1195`, `:1365`. `over_tuples` calls `holds` or `values` once per tuple of the domain.

**Change.** Key the maps by one `Vec<u32>` (or `Box<[u32]>`) whose first element is the predicate, symbol or node, and probe them with a `&[u32]` built in a kept buffer (`Vec<u32>: Borrow<[u32]>`); return value rows as slices of `principals`.

**Value.** The machine: an allocation less per memo probe on the expansion's hot loop.

**Net.** Measure with DHAT as F-4d-05 (no journey covers it); the encoders' statistics and the checked structures stay identical.

## F-4d-07

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/transfer.rs:290; core/src/ordinary/structure.rs:192; core/src/ordinary/kripke_structure.rs:396; core/src/ordinary/rocq_models.rs:309
- kind: refactor
- cost: small
- candidate: no

**Claim.** The walk "the nodes and terms the sequent's formulas reach, each once" is written four more times in this unit, beside the five F-4a-08 found, here with `Vec<bool>` seen-sets.

**Witness.** ```rust
// core/src/ordinary/structure.rs:202-218, and verbatim at kripke_structure.rs:406-422 and rocq_models.rs:320-336
        while let Some(n) = stack.pop() {
            if std::mem::replace(&mut seen[n.index()], true) {
                continue;
            }
            match formulas.node(n) {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        terms.extend_from_slice(tables.list(args));
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => stack.push(a),
```

`transfer.rs:290` (`reached`) is the same walk returning both bitmaps. `duplicates.md`: `kripke_structure.rs:396`↔`rocq_models.rs:310`↔`structure.rs:192`, and `structure.rs:209`, `rocq_models.rs:327`, `kripke_structure.rs:413` each ↔ `transfer.rs:306`.

**Change.** Make `transfer::reached` (or F-4a-08's `Formulas::terms_in`) the one walk, ungated from `interactive`, and have the two checks and `rocq_models::applied` iterate its terms.

**Value.** A reader; the next term or node variant is added once.

**Net.** Touches the two first-order countermodel checks' up-front pass: the refusal tests (`a_table_that_does_not_fit_is_refused`, `a_model_that_breaks_a_rule_is_refused`) and the certificate checks of `rocq-deduxit`.

## F-4d-08

- dimension: api
- criterion: Q8
- place: core/src/ordinary/rocq.rs:928; core/src/ordinary/rocq_models.rs:442; core/src/modal/rocq.rs:95; core/src/modal/rocq.rs:522
- kind: refactor
- cost: small
- candidate: no

**Claim.** The Rocq library's formula constructors (`var`, `top`, `bot`, `neg`, `wedge`, `vee`, `imp`, `equiv`, `box`, `dia`, `fall`, `fex`), part of the certificates' contract with `rocq-deduxit`, are spelt out in four walks of this unit, beside the two of the exporter (F-6b-05).

**Witness.** ```rust
// core/src/ordinary/rocq.rs:946-952
                    Node::True => out.push_str("top"),
                    Node::False => out.push_str("bot"),
                    Node::Not(_) => out.push_str("neg "),
                    Node::And(..) => out.push_str("wedge "),
                    Node::Or(..) => out.push_str("vee "),
                    Node::Implies(..) => out.push_str("imp "),
                    Node::Iff(..) => out.push_str("equiv "),
```

The same names at `rocq_models.rs:471-479`, `modal/rocq.rs:96-106` (by the rule file's connective) and `modal/rocq.rs:548-554` (by `modal::Node`). F-9-01 (confirmed) found that no list marks the exporter's names as the contract.

**Change.** One crate-private table of the library's constructor names, keyed by connective, in `export::rocq` beside the preludes, that the four walks and `export/rocq/deduxit.rs` read; list it as the contract F-9-01 asks for.

**Value.** The maintainer of `rocq-deduxit`: a rename in the library changes one table here, and the contract is written down.

**Net.** Touches certificates: the `rocq` and `rocq-deduxit` flake checks compile them; the snapshots of the export test must stay byte-identical.

## F-4d-09

- dimension: idiom
- criterion: E6
- place: core/src/modal/rocq.rs:24 and core/src/ordinary/rocq_models.rs:44; core/src/modal/rocq.rs:57, core/src/ordinary/rocq.rs:969 and core/src/export/rocq/deduxit.rs:1301
- kind: refactor
- cost: small
- candidate: no

**Claim.** The five certificate writers of this unit repeat their helpers (a bounded text, the Rocq string quote, the stop's refusal) and follow three conventions for bounding their text and two for their stop.

**Witness.** `struct Bounded` is `modal/rocq.rs:24-54` and `rocq_models.rs:44-95` (the second adds a weight); `fn string` is `modal/rocq.rs:57`, `ordinary/rocq.rs:969` and `export/rocq/deduxit.rs:1301`, each

```rust
    format!("\"{}\"", name.replace('"', "\"\""))
```

`fn stopped()` is `modal/rocq.rs:77` and `rocq_models.rs:98`, written inline at `ordinary/rocq.rs:526`, `:736`, `:793`, `:903`. The text is bounded by a `Bounded` buffer (`modal/rocq.rs`, `rocq_models.rs`), by an estimate before streaming (`ordinary/rocq.rs:726-761`), or not at all (`ordinary/rocq.rs:393`, whose stop is `impl FnMut() -> bool` and whose check runs with `work: None`, `:410-414`), where every other writer takes `impl FnMut(Progress) -> bool` and `&Limits`.

**Change.** Move `Bounded` (with the optional weight), `string` and `stopped` into `export::rocq` and use them from all five writers; give `ordinary::rocq::write` the `&Limits` and progress stop of its siblings.

**Value.** A reader of the certificate writers; one place that keeps a certificate within `limits.derivation_bytes`.

**Net.** Touches certificates: the export test's snapshots and the `rocq`, `rocq-deduxit` checks; the text must stay byte-identical.

## F-4d-10

- dimension: architecture
- criterion: E5
- place: core/src/ordinary/mod.rs:89
- kind: refactor
- cost: medium
- candidate: no

**Claim.** Countermodels of ordinary logic are seven flat files of `ordinary/` with near names (`model.rs`, `models.rs`, `structure.rs`, `kripke_structure.rs`, `kripke_search.rs`, `rocq_models.rs`, beside `grounding.rs` that feeds them), among some thirty modules, so a reader cannot tell the data and checks from the searches and the certificates.

**Witness.** ```rust
// core/src/ordinary/mod.rs:89-97
#[cfg(feature = "sat")]
mod kripke_search;
...
mod kripke_structure;
pub mod ljt;
...
mod model;
#[cfg(feature = "sat")]
pub mod models;
```

and `mod structure;` at `:112`, `pub(crate) mod rocq_models;` at `:108`. Sizes: `model.rs` 594, `structure.rs` 845, `kripke_structure.rs` 1 274 (data and checks); `models.rs` 1 429, `kripke_search.rs` 1 068 (searches); `rocq_models.rs` 552 (certificates). The handover for unit 4d asks the question.

**Change.** Group without moving a public path: `ordinary/countermodel/` (private, re-exported at `ordinary::` as today) with `mod.rs` the propositional `Countermodel` (now `model.rs`), `finite.rs` (`structure.rs`) and `kripke.rs` (`kripke_structure.rs`); `ordinary/models/` (the public `ordinary::models`) with `mod.rs` the driver, `finite.rs` the expansion, `kripke.rs` (`kripke_search.rs`) and the shared writer of F-4d-03; `ordinary/rocq/` with `mod.rs` (`rocq.rs`) and `models.rs` (`rocq_models.rs`).

**Value.** A reader finds the trust (the checks) apart from the untrusted searches and the certificates.

**Net.** File moves only: the build, `cargo hack` (features `sat`, `rocq`), `unreachable_pub`; no public path changes (`breaks 0.8.0` shows none).

## F-4d-11

- dimension: performance
- criterion: P1
- place: core/src/modal/rocq.rs:401
- kind: refactor
- cost: small
- candidate: yes

**Claim.** The certificate of a valid modal outcome checks its derivation twice and builds the system's calculus twice: once in `Outcome::check`, then again in `labelled`.

**Witness.** ```rust
// core/src/modal/rocq.rs:399-402
        (super::Verdict::Valid(super::Evidence::Derivation(derivation)), _) => {
            // The root is the sequent at w0, which the check asks.
            outcome.check(limits, &mut stop)?;
            labelled(
```

`Outcome::check` ends with `let calculus = calculus::calculus(self.system, limits)?; derivation.check(&calculus, limits, stop)` (`modal/decide.rs:201-202`), and `labelled` begins with `derivation.check(&super::calculus(system, limits)?, limits, &mut stop)?;` (`modal/rocq.rs:434`).

**Change.** Split `Outcome::check` into the root comparison and the derivation check, and call only the root comparison before `labelled`, which checks the derivation.

**Value.** A user certifying a large modal proof: one check instead of two.

**Net.** Measure pinned time of `deduxit` writing the Rocq certificate of a large valid modal sequent (a deep S4 family) before and after; the certificate text is unchanged and the `rocq-deduxit` check compiles it.

## F-4d-12

- dimension: performance
- criterion: P2
- place: core/src/modal/search.rs:1163
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Each saturation of a modal branch rebuilds, for every pending formula of two premises, both premises' member vectors only to test whether one already holds, and again to pick the split.

**Witness.** ```rust
// core/src/modal/search.rs:1163-1169
        let mut betas = std::mem::take(&mut branch.betas);
        betas.retain(|&(world, id, right)| {
            let (_, premises) = self.beta(world, id, right);
            !premises
                .iter()
                .any(|p| p.iter().all(|&m| Self::holds(branch, m)))
        });
```

`beta` (`:1196-1217`) allocates two `Vec<Member>` per call. Likewise `act` clones a world's `sees` per `□` true (`:1000`, `:1013`) and `edge` clones `sees` and `seen_by` (`:1104-1107`, `:1131`), and every `linear` call receives freshly allocated `written` and `premise` vectors that it drops at once where the premise holds already (`:833-835`).

**Change.** Let `beta` return its rule and members as fixed arrays (at most two members a premise); iterate the seen lists by index instead of cloning them; test whether the premise holds before allocating its vectors.

**Value.** The machine, on modal searches with many disjunctions.

**Net.** No journey runs modal logic (`grep modal bench/src` finds nothing): a modal journey for the ratchet and DHAT is needed first (P1). The search's statistics (`worlds`, `splits`, `blocked`, `work`) must stay identical, so it is a refactor, not a search change.

## F-4d-13

- dimension: text
- criterion: X3
- place: core/src/modal/model.rs:498; core/src/ordinary/structure.rs:579; core/src/ordinary/kripke_structure.rs:996
- kind: text
- cost: small
- candidate: no

**Claim.** The countermodel types' field docs state as fact what only `check` establishes, while their `new` builds them unchecked, so a reader takes a value built by any crate for a checked one.

**Witness.** ```rust
// core/src/modal/model.rs:498-501
    /// The model. Its relation meets the system's frame conditions, and at
    /// its world every formula left of the sequent's `⊢` is true and every
    /// one right of it false, as [`check`](Self::check) checks.
    pub model: Model,
```

and `:505-506` `/// Returns the countermodel of this sequent, system and model, /// unchecked.` The same at `structure.rs:579-581` ("It makes every formula left of the sequent's `⊢` true…") and `kripke_structure.rs:996-997`. `FiniteCountermodel`'s own type doc says it well: "A finite structure said to refute a first-order sequent" (`structure.rs:566`).

**Change.** Rewrite each field doc as a claim: "The model said to refute the sequent: `check` checks that its relation meets the system's conditions and that the sequent fails at its world." Say on each type that an outcome of the crate's search holds a checked one and that a value from elsewhere must be checked.

**Value.** A user who reads a countermodel from JSON or builds one: the docs say where the trust is (X3, consumer test point 5).

**Net.** Docs only; `cargo doc` and the doc tests.

## F-4d-14

- dimension: architecture
- criterion: D4
- place: core/src/modal/search.rs:65
- kind: API break
- cost: small
- candidate: no

**Claim.** The modal suite takes its unknown reasons and its memory account from the linear search (`linear::search::Reason`, `linear::search::memory::Account`), so the shared module F-arch-01 proposes must offer what modal uses of them, which is a small part.

**Witness.** ```rust
// core/src/modal/search.rs:65-66
use crate::linear::search::Reason;
use crate::linear::search::memory::Account;
```

and `modal/decide.rs:14`, `:47` (`Unknown(Reason)`). The modal search produces only `Reason::Stopped`, `WorkLimit { limit }`, `MemoryLimit { limit_bytes }` and `IndexLimit` (`search.rs:399-423`, `:875`), and uses `Account::holding`, `charge`, `release`, `rewind`, `spares`, `over` and `limit`. A modal user matching `Verdict::Unknown` must import from `deduxit::linear::search`.

**Change.** With F-arch-01's move of `Reason` to the root, move `Account` beside it (`limits::Account` or a root `memory` module) with those seven methods, and import both from there in `modal/`.

**Value.** A user of modal logic alone, who no longer reaches into the linear suite; the next suite gets the account from the root.

**Net.** `breaks 0.8.0` for the moved path; the modal search's statistics and tests unchanged.

## F-4d-15

- dimension: idiom
- criterion: E6
- place: core/src/modal/search.rs:735
- kind: refactor
- cost: small
- candidate: no

**Claim.** The modal search iterates the set bits of a world's truth and falsity masked by its box, diamond or atom bits in four hand-written loops, and tests "has a `□` true or `◇` false of its own" in three.

**Witness.** ```rust
// core/src/modal/search.rs:735-738
                let mut boxes = world.truth[i] & self.boxes[i];
                while boxes != 0 {
                    let node = (i * 64) as u32 + boxes.trailing_zeros();
                    boxes &= boxes - 1;
```

The same loop at `:745-748`, `:1061-1065`, `:1078-1080`, and as `flat_map`/`from_fn` at `:1485-1496` and `:1766-1778`; the test `w.truth[i] & self.boxes[i] != 0 || w.falsity[i] & self.diamonds[i] != 0` at `:1296-1298`, `:1338-1340` and `:1544-1545`.

**Change.** Two small helpers on `Search`: `ones(set, mask) -> impl Iterator<Item = u32>` and `has_own_modal(&World) -> bool`.

**Value.** A reader of a 1 959-line search.

**Net.** `search_tests.rs` and the statistics; no search change.
