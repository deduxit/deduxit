# Unit 1c: findings

Read in full: 7 files. Not read in full: none.

The finder's remarks: For the handover on what each refutation promises: `Exhausted` promises only that a search ran out (anyone can build it, `check` answers `uncertified`); `Classical`, `Unbalanced`, `OneSided`, `Equation`, `StateEquation` and the two symbol forms are certificates backed by Rocq lemmas (`Refutations.v`, `Weights.v`, `FirstOrderWeights.v`), while `Saturated` and `Basis` have no Rocq theorem and rest on their Rust check's argument alone (the Rocq writer refuses them, `export/rocq/deduxit.rs:1208-1210`); an outside engine can today build only `Exhausted`, `Saturated` and `Basis`, and only for the roots (F-1c-02). Cut elimination meets X5: `eliminate` takes any `&Proof` and checks it before its first step. No journey of the ratchet or DHAT runs cut elimination or any refutation check, so F-1c-10, F-1c-12 and F-1c-13 have no instrument yet; the leader could add a journey for a saturated set's check. Unit 2a, or whoever writes the engine interface (F-arch-04), should take F-1c-01's `Disproof::check_within`, since `prove_goal`'s last-resort block (`search/mod.rs:1080-1115`) becomes a call to it. Unit 7a should look at `cli/src/check.rs:454-456`, which leaves a thread running past `--timeout`. I could not judge whether `cut::Options::drawn_bytes`, a field the library never reads, belongs in the library's options; that is a settings question for unit 6a.

## F-1c-01

- dimension: api
- criterion: S1
- place: core/src/linear/search/refutation.rs:1173
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `Disproof::check` takes no limits and no stop, so a saturated set or a basis read from elsewhere is checked without bound, and every caller that has a stop works around it.

**Witness.** `core/src/linear/search/refutation.rs:1173-1175` and `:1210-1216`:

```rust
    pub fn check(&self) -> Result<(), crate::Error> {
        let fault = |fault| crate::Error::Refutation(fault);
        let forest = crate::linear::occurrences::Forest::new(&self.sequent)?;
```

```rust
                match super::saturated::check(&forest, &goal, self.mode, saturated, &mut |_| false)
                {
                    Ok(()) => Ok(()),
                    Err(super::saturated::Failed::Fault(fault)) => Err(fault),
                    Err(super::saturated::Failed::Stopped) => {
                        unreachable!("a check without a stop runs to its end")
                    }
```

Its docs (`:1131-1135`): "The check has no stop, though. For a set read from elsewhere it can take time exponential in the `&` under `⅋` of the goal, … So a caller checks such a set within a time limit of its own."

The callers: `cli/src/check.rs:454-456` runs it on a thread that is left running past the limit:

```rust
    // A saturated set's check has no stop of its own.
    let checked = disproof.clone();
    let result = deadline.within(move || checked.check())?;
```

`core/src/protocol/linear.rs:711-716` has `limits` and `stop` in hand and uses them for the proof but cannot for the disproof:

```rust
                        let checked_proof = proof
                            .check_within(outcome.mode, limits, &mut *stop)
```
```rust
                    Found::Unprovable(disproof) => match disproof.check() {
```

`web/src/calls.rs:1767` (wasm, no thread): `Verdict::Unprovable(disproof) => Some(disproof.check().is_ok()),`. Meanwhile the search wraps the same two checks with a stop and the work bound itself (`core/src/linear/search/mod.rs:1081-1086`), and the siblings take both: `Proof::check_within(mode, &limits, stop)` (`proofs/mod.rs:927`), `ordinary::resolution::Refutation::check(&limits, stop)` (`ordinary/resolution/refutation.rs:191`). `Forest::new` also uses the default limits, so a disproof read with a raised `--occurrence-limit` is refused at its check.

**Change.** Add `Disproof::check_within(&self, limits: &Limits, stop: impl FnMut(Progress) -> bool) -> Result<(), Error>` (a one-line shim over a `&mut dyn` body), which builds the forest with `Forest::within(limits)` and polls the saturated and basis checks with `Phase::Check` under `limits.work`, as `search/mod.rs:1081-1104` does today; `check()` becomes `check_within(&Limits::default(), |_| false)`. Then `prove_goal` calls it for its last-resort refutation, the protocol passes its limits and stop, the command its stop instead of the detached thread, and the two `unreachable!` arms go.

**Value.** A user of the library, the protocol and the web page can check a disproof read from elsewhere within a bound and stop it, as they check a proof; the command no longer leaves a thread running; the crate's rule that every long call takes `&Limits` and a stop holds for the one checker that breaks it.

**Net.** Touches a checker's entry point, not its logic: `refutation.rs`'s `a_wrong_refutation_is_rejected` and `a_wrong_count_with_copies_or_state_equation_is_refused`, `saturated.rs`'s and `basis.rs`'s tests (`the_check_asks_the_stop`), the behaviour lock, the manual's `check` examples and the protocol's check of `linear.decide`; the verdicts are the same within the bounds.

## F-1c-02

- dimension: api
- criterion: X4
- place: core/src/linear/search/refutation.rs:222
- kind: refactor
- cost: small
- candidate: no

**Claim.** An outside engine can build only the refutations `Exhausted`, `Saturated` and `Basis`, and only for the sequent's roots: seven of the ten payload structs are `#[non_exhaustive]` without a constructor, and `Disproof::of_goal` is crate-private.

**Witness.** `core/src/linear/search/refutation.rs:222-228`, no `impl Classical` anywhere in the crate (likewise `Unbalanced` :131, `OneSided` :152, `Equation` :170, `StateEquation` :201, `UnbalancedSymbol` :98, `OneSidedSymbol` :112):

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Classical {
    /// The value of each atom of the sequent, by index. An atom the goal
    /// does not contain has one too.
    pub assignment: Vec<bool>,
}
```

while the same file gives the other three constructors (`:297`, and `Stable::new` :285, `Basis::new` :350, `Marking::new` :362):

```rust
    pub fn new(positive: Vec<Sign>, sequents: Vec<Stable>) -> Self {
        Self { positive, sequents }
    }
```

and `:921`: `pub(crate) fn of_goal(self, goal: Box<[Member]>) -> Self {`. So the consumer test's toy engine (point 5) can answer only `Disproof::new(sequent, mode, Refutation::Exhausted)`, the one kind `check` refuses as `uncertified`; a falsifying assignment, the simplest certificate, cannot be written outside the crate except as JSON.

**Change.** Give each payload a `new` (`Classical::new(assignment)`, `Unbalanced::new(atom, least, most)`, `OneSided::new(atom, least, most, copied)`, `Equation::new(…)`, `StateEquation::new(atoms, clauses, dropped)` and the two symbol forms), and a public `Disproof::of_goal(sequent, mode, goal: &[Member], refutation) -> Result<Self, Error>` that refuses a member the sequent lacks, as the JSON reader does. The values stay unchecked until `Disproof::check`, which stays the boundary.

**Value.** A researcher's engine answers with a certificate that deduxit's own check and Rocq writer accept (X3, X4, the consumer test's points 4 and 5), and a session or a front end can refute a goal inside a sequent as the search does.

**Net.** No checker changes: the constructors only build values `Disproof::check` already decides, as the JSON reader does (`serialize/search.rs:656-663`). Additive; `breaks 0.8.0` lists nothing.

## F-1c-03

- dimension: text
- criterion: X3
- place: core/src/linear/search/refutation.rs:15
- kind: text
- cost: small
- candidate: no

**Claim.** The docs of `Refutation` and `Disproof::new` do not say what a refutation promises: they say most refutations rest on a search, when nine of the ten kinds are certificates that `Disproof::check` verifies and only `Exhausted` rests on the search and can be built by anyone.

**Witness.** `core/src/linear/search/refutation.rs:15-20`:

```rust
/// Most refutations rest on a search that tried every way to a proof.
/// Others rest on the sequent alone: its literals or its connectives rule
/// out a proof, and the refutation says which. The focused engine tests
/// these on every sequent it searches. The Horn engine may refute a Horn
/// program by its state equation, whatever its search found.
```

`:60-61`:

```rust
    /// Every way to prove the sequent was tried and failed.
    Exhausted,
```

`:909-910`:

```rust
    /// Returns the disproof of a sequent in a mode.
    pub fn new(sequent: Sequent, mode: Mode, refutation: Refutation) -> Self {
```

The kinds the check verifies are listed only under `Disproof::check` (`:1066-1120`); `RefutationFault::Uncertified` (`:972-978`) is where a reader learns that `Exhausted` has no check. The handover for this unit (consumer test point 5, F-2a-09 on `Verdict`) asks for this.

**Change.** Rewrite the summary of `Refutation`: "Every kind but `Exhausted` is a certificate: `Disproof::check` verifies it from the sequent alone, with no code of the search. `Exhausted` says only that a search ran out of ways to a proof; nothing checks it." On `Exhausted`: "A search tried every way to a proof. No check verifies this." On `Disproof::new`: "The disproof is not checked: any refutation can be given. `check` decides it." Name which kinds the Rocq writer certifies (all but `Exhausted`, `Saturated`, `Basis`).

**Value.** A user who receives or builds a disproof knows that `check` is the trust boundary and which answers carry evidence, without reading the check's long doc comment.

**Net.** Docs only; the doc examples of `Disproof` and `Disproof::check` run unchanged.

## F-1c-04

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/weighted.rs:284; core/src/linear/search/basis.rs:124
- kind: refactor
- cost: small
- candidate: no

**Claim.** The two checks that read a goal as a Horn program, the state equation's dead atoms and the basis, each write their own reader: the same head test, the same clause loop and the same least-fixpoint of reached places.

**Witness.** `core/src/linear/search/weighted.rs:284-289` and `core/src/linear/search/basis.rs:124-129`, identical:

```rust
    let head = |o: OccId| {
        forest.subtree(o).all(|x| match forest.kind(x) {
            Kind::Par | Kind::Bot => true,
            _ => forest.sign(x) == Some(!body),
        })
    };
```

The clause loop, `weighted.rs:315-324` against `basis.rs:133-144`:

```rust
        while let Some(x) = factors.pop() {
            match forest.kind(x) {
                Kind::Tensor => factors.extend(forest.left(x).into_iter().chain(forest.right(x))),
                Kind::One => {}
                _ if forest.sign(x) == Some(body) => inputs.push(atom(x)),
                _ if found.is_none() && head(x) => found = Some(x),
                _ => return None,
            }
        }
```

and the propagation of reached atoms, `weighted.rs:331-355` (`missing`, `waiting`, `ready`) against `basis.rs:218-252` (`live`: `waiting`, `by_input`, `ready`). `basis.rs` also sorts and run-length-counts atoms twice (`counted` :97-111 and inside `clause` :145-152).

**Change.** One private Horn reader for the checks, returning the clauses (body counts, optional head counts, under `?` or used once) and the markings; `basis::read` adds its two conditions (a reusable clause has a head, exactly one goal) and `dead_atoms` is `named && !live(...)` over its result. It stays apart from the Horn engine (`core-search.md:289-307`), as both checks are today.

**Value.** A reader of the checks reads one Horn reading, and a fix of the shape (such as the one the panel's note on `Program::read_with` asks for) is made once.

**Net.** Touches two checkers: `weighted.rs`'s and `basis.rs`'s tests, `refutation.rs`'s `a_wrong_count_with_copies_or_state_equation_is_refused`, the Rocq certificates of a state equation (`export/rocq/deduxit.rs:1177` writes the dead set `state_equation` returns) under the `rocq-deduxit` check, and the mutation baseline of the checkers.

## F-1c-05

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/refutation.rs:1379; core/src/linear/search/refutation.rs:1508
- kind: refactor
- cost: small
- candidate: no

**Claim.** The balance check and the one-sided count check each compute the excess interval over the goal with the same walk, and each repeats the test that no atom of the predicate is open.

**Witness.** `core/src/linear/search/refutation.rs:1382-1395` (`check_balance`) and `:1527-1540` (`check_copies`), equal but for the exponential's arm (CPD lists the pair, 242 tokens):

```rust
        for o in forest.subtree(member).rev() {
            let side = |side: Option<OccId>| side.and_then(|s| span[s.index()]);
            let (l, r) = (side(forest.left(o)), side(forest.right(o)));
            let literal = |e| Some(if counted(o) { (e, e) } else { (0, 0) });
            span[o.index()] = match forest.kind(o) {
                Atom => literal(1),
                DualAtom => literal(-1),
                One | Bot | Zero => Some((0, 0)),
                Top => None,
                Tensor | Par => l.zip(r).map(|((a, b), (c, d))| (a + c, b + d)),
                With | Plus => l.zip(r).map(|((a, b), (c, d))| (a.min(c), b.max(d))),
                Bang | Quest => l.filter(|&s| s == (0, 0)),
                Forall | Exists => l,
```

(`check_copies` has `Bang | Quest => l.map(|_| (0, 0)),`). And `:1303-1316` against `:1427-1440` (CPD, 140 tokens), the same block with another invariant's name:

```rust
    if let Some((symbol, _)) = sequent.atom_predicate(atom)
        && (0..sequent.atom_names().len()).any(|a| {
            let a = Atom::new(a as u32);
            sequent.atom_predicate(a).map(|(s, _)| s) == Some(symbol) && sequent.atom_is_open(a)
        })
```

**Change.** One `excess(forest, member, counted, at_exponential)` that returns a member's interval, with the exponential's rule as a two-variant parameter (closed where zero, or counted as nothing), used by both checks; and one `predicate_open(sequent, atom) -> bool` for both atom forms.

**Value.** A reader checks the count once; a change to how a connective counts (a new connective, a binder) is made in one place in a checker.

**Net.** Touches a checker: the two tests of `refutation.rs` that plant wrong counts, `classical::tests::checked_proofs_read_as_tautologies`, the Rocq certificates of counts (Refutations.v `balance`, Weights.v), the mutation baseline.

## F-1c-06

- dimension: architecture
- criterion: E5
- place: core/src/linear/search/refutation.rs:1245
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The checks of refutations, the crate's second trust boundary in the linear suite, are spread over four files of `search/` beside the engines, one of them also holding the refutation types and 450 lines of phrases.

**Witness.** `refutation.rs` (2 072 lines) holds the ten types (`:59-365`), the phrases (`:399-846`), `Disproof` (`:896-966`), and the checks of five kinds (`check_classical` :1246, `check_unbalanced` :1295, `check_balance` :1359, `check_one_sided` :1419, `check_copies` :1489, `check_equation` :1592); the other three checks are `search/saturated.rs`, `search/basis.rs` and `search/weighted.rs` (`pub(crate) mod weighted`, `search/mod.rs:60`, which the Rocq writer calls as `crate::linear::search::weighted::state_equation`). `search/mod.rs:22-60` declares them among the engines (`basis`, `classical` the refuter, `refutation`, `saturated`, `weighted`). The module doc of `refutation.rs` is two lines and names no check.

**Change.** A directory `linear/search/refutation/`: `mod.rs` (the types and `Disproof`), `phrase.rs` (the `Display` wording), `check.rs` (`Disproof::check` and the counts', assignment's and equation's checks), `weighted.rs`, `saturated.rs`, `basis.rs`; the refuter `classical.rs` stays with the engines. Its `mod.rs` doc says that nothing in it shares code with a search.

**Value.** A reader finds the checks of non-derivability in one place and sees their independence from the engines as a module boundary; F-arch's feature that keeps the checkers without the engines (F-arch on `search`) needs this split.

**Net.** Moves only; the tests move with their files. No checker logic changes; the Rocq writer's path to `state_equation` changes.

## F-1c-07

- dimension: architecture
- criterion: E6
- place: core/src/linear/proofs/cut/framed.rs:311
- kind: refactor
- cost: small
- candidate: no

**Claim.** `framed.rs` writes the memoised postorder walk over first-order terms three times and the walk down a frame chain three times.

**Witness.** The term walk, `core/src/linear/proofs/cut/framed.rs:320-333` (`names_term`), `:419-433` (`substituted_term`) and `:615-626` (`Transfer::term`):

```rust
        let mut walk = vec![(id, false)];
        while let Some((t, expanded)) = walk.pop() {
            if t.index() < base || named.terms.contains_key(&t) {
                continue;
            }
            let found = match terms.term(t) {
                FoTerm::Eigen(a) => sigma.contains_key(&a),
                FoTerm::App(_, args) => {
                    let list = terms.list(args);
                    if !expanded {
                        walk.push((t, true));
                        walk.extend(list.iter().map(|&a| (a, false)));
                        continue;
                    }
```

The frame chain, `:291-305` (`names_member`), `:384-406` (`substituted_frame`) and `:646-663` (`Transfer::frame`):

```rust
        let mut chain = Vec::new();
        let mut next = Some(frame);
        let mut below = None;
        while let Some(f) = next {
            if let Some(&done) = self.frame_at.get(&f) {
                below = Some(done);
                break;
            }
            chain.push(f);
            next = self.table.instances.frame_parts(f).0;
        }
```

**Change.** Two private helpers: a term fold `fold_term(terms, base, id, memo, leaf, app)` that walks with its own stack and calls `app` with the folded arguments, and `frame_chain(instances, frame, memo) -> (Vec<FrameId>, Option<T>)`; the three passes (find, substitute, copy) become their closures.

**Value.** A reader of first-order cut elimination reads each walk once; a fix to the non-recursive walk (the crate's rule that nothing recurses over a term) is made once.

**Net.** Cut elimination's tests (`copies_are_charged_against_the_bounds` and the first-order steps of `cut/tests.rs`), each of which checks the proof a step made with the checker; no checker changes.

## F-1c-08

- dimension: architecture
- criterion: E6
- place: core/src/linear/proofs/cut/mod.rs:341; core/src/linear/proofs/cut/mod.rs:464
- kind: refactor
- cost: small
- candidate: no

**Claim.** `step` and `eliminate` each write the choice of the cut (the caller's `at` if it is a cut, else `NoCut`, else the strategy's) and check the proof a step made in two different ways.

**Witness.** `core/src/linear/proofs/cut/mod.rs:341-352`:

```rust
    let cut = match at {
        Some(at) if matches!(proof.nodes().get(at.index()), Some(Node::Cut(..))) => at,
        Some(at) => {
            return Err(Error::NoCut {
                node: Some(at.get()),
            });
        }
        None => options
            .strategy
            .pick(proof)
            .ok_or(Error::NoCut { node: None })?,
    };
```

and `:464-472`:

```rust
        let cut = match at.filter(|_| steps.is_empty()) {
            Some(at) if matches!(current.nodes().get(at.index()), Some(Node::Cut(..))) => Some(at),
            Some(at) => {
                return Err(Error::NoCut {
                    node: Some(at.get()),
                });
            }
            None => options.strategy.pick(&current),
        };
```

`step` checks the made proof by calling `check::reading` and `check::examine` itself (`:355-368`), `eliminate` by the next round's `Record::of` (`:446-460`), with its own mapping of `Refused` and `Invalid` to `Error::Reduction`.

**Change.** A private `chosen(proof, at, strategy) -> Result<Option<NodeId>, Error>` for both, and `step` as `Record::of` on the made proof (it checks and is the same pass), so that the mapping of the check's errors to `Error::Reduction` lives once.

**Value.** A reader of the elimination sees one rule for which cut is reduced and one for what a failed check of a made proof means.

**Net.** `cut/tests.rs` and the doc example of `eliminate`; the command's `eliminate` examples in the manual; no checker change (the same `examine` pass runs).

## F-1c-09

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/saturated.rs:81; core/src/linear/search/basis.rs:58
- kind: refactor
- cost: small
- candidate: no

**Claim.** The saturated set's check and the basis's check each define the same private `Failed` enum, so every caller matches the two alike twice.

**Witness.** `core/src/linear/search/saturated.rs:81-86` and `core/src/linear/search/basis.rs:58-63`, each with its own `From<RefutationFault>`:

```rust
pub(crate) enum Failed {
    /// The certificate does not hold.
    Fault(RefutationFault),
    /// The stop fired.
    Stopped,
}
```

matched twice in `core/src/linear/search/mod.rs:1087-1104`:

```rust
                                    .map_err(|failed| match failed {
                                        saturated::Failed::Stopped => None,
                                        saturated::Failed::Fault(fault) => Some(fault),
                                    })
```

and twice in `refutation.rs:1210-1236`.

**Change.** One `Failed` beside `RefutationFault` (or, with F-1c-01, the checks return `Result<(), Error>` with `Refusal::Stopped` and `Refusal::Work`), and one match in `Disproof::check_within`.

**Value.** A reader and the next check of a refutation (a new kind) share one failure type and one place that turns it into the crate's error family.

**Net.** `saturated.rs`'s and `basis.rs`'s tests (`the_check_asks_the_stop`), the search's last-resort check; no check logic changes.

## F-1c-10

- dimension: performance
- criterion: P2
- place: core/src/linear/search/saturated.rs:640
- kind: refactor
- cost: small
- candidate: yes

**Claim.** The saturated set's check clones each premise's zones for every choice of premises and partners it evaluates, and allocates two more lists per sequent derived, in its innermost loop.

**Witness.** `core/src/linear/search/saturated.rs:640`, inside `derive`, called once per choice of premises times partners (`:1104-1105`):

```rust
                Step::Hole(h) => set[premises[h]].clone(),
```

and per derived sequent in `closed` (`:1048`):

```rust
        let all = sum(&c.linear, &c.unrestricted);
```

`core-inverse.md:260-266`: the check costs 17 to 90 % of the search on the rows only it refutes, `SYJ207+1.002` in cbv 37 s beside 46 s of search, "so a refutation near the time limit can become `Unknown`". No journey of the ratchet or DHAT runs this check (`target/audit/ratchet.out`, `dhat.md`).

**Change.** Evaluate a hole by reference (the stack holding `Cow<Zone>` or indices until a step changes the zone), keep one scratch buffer for `all` and the coherence test, and measure before and after.

**Value.** The machine: a cheaper check leaves fewer refutations of last resort `Unknown` at the time limit, in the search and in `deduxit check`.

**Net.** Candidate: measure the check alone on the saved disproof of `SYJ207+1.002` (cbv) and `SYJ208+1.005` (cbn) under DHAT and pinned time (`bench/pinned-time.py`), and add a journey that checks a saturated set. Touches a checker; `the_engines_sets_pass`, `a_wrong_set_is_refused`, the differential test's 69 sets.

## F-1c-11

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/saturated.rs:489
- kind: refactor
- cost: small
- candidate: no

**Claim.** The saturated set's check writes its own sorted-multiset operations, which `proofs::multiset::Multiset`, the proof checker's, already has.

**Witness.** `core/src/linear/search/saturated.rs:489-548`: `insert` (`partition_point` then `insert`), `sum` (merge), `excess` (multiset difference), `within` (multiset inclusion), against `core/src/linear/proofs/multiset.rs:64-67` `insert`, `:110` `sum`, `:122` `difference`, `:127` `is_subset`:

```rust
fn insert(list: &mut Vec<OccId>, o: OccId) {
    let at = list.partition_point(|&x| x <= o);
    list.insert(at, o);
}
```
```rust
    pub(crate) fn insert(&mut self, o: T) {
        let i = self.0.partition_point(|&x| x <= o);
        self.0.insert(i, o);
    }
```

CPD lists `saturated.rs:519` against `inverse/eval.rs:98`; that copy is on purpose (`core-inverse.md:219`: the check shares no code with `inverse/`), but `Multiset` belongs to the checker of proofs, not to the engine.

**Change.** Make `Zone`'s linear zone a `Multiset<OccId>` and use its `insert`, `sum`, `difference`, and an allocation-free `is_subset` (today it allocates a difference); keep the set union (`union` with `dedup`) local, which `Multiset::union` (max of counts) is not. `inverse/` keeps its own copy.

**Value.** A reader of the check meets the crate's one multiset; the check's independence from the engine stays as the rule records it.

**Net.** Touches a checker: `saturated.rs`'s tests and the differential test; `Multiset`'s own test `counts`. If `is_subset` becomes allocation-free, the proof checker's callers gain too (measure with `check-qbf-20-2`).

## F-1c-12

- dimension: performance
- criterion: P4
- place: core/src/linear/proofs/cut/mod.rs:426
- kind: refactor
- cost: small
- candidate: yes

**Claim.** `cut::eliminate` and `cut::step` are long bodies generic over the caller's stop (and `eliminate` over `each`), so each caller compiles its own copy, against the crate's rule of a one-line shim over a `&mut dyn` body.

**Witness.** `core/src/linear/proofs/cut/mod.rs:426-434`, a 78-line body:

```rust
pub fn eliminate(
    proof: &Proof,
    mode: Mode,
    at: Option<NodeId>,
    options: &Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
    mut each: impl FnMut(&Proof, &Step),
) -> Result<Elimination, Error> {
```

and `step` (`:332-339`, 38 lines). Callers with distinct closure types: `cli/src/eliminate.rs:174` and `:821`, `web/src/calls.rs:2163` (`step`) and `:2200`, `core/src/protocol/linear.rs:568`. `core.md` ("A long body takes the stop as `&mut dyn FnMut(Progress) -> bool`") names the rule; the handover asks every unit to list such bodies (F-3-10, F-4b-02, F-4d-04, F-6b-02).

**Change.** Make both public functions one-line shims that pass `&mut stop` and `&mut each` to private bodies taking `&mut dyn FnMut`.

**Value.** The web module and the command carry one copy of the elimination loop; the rule holds everywhere it is stated.

**Net.** Candidate: count the copies with `cargo llvm-lines` and the web module's bytes at gzip before and after (`budget`). Behaviour kept: `cut/tests.rs`, the doc example, the bindings' tests; no checker change.

## F-1c-13

- dimension: performance
- criterion: P1
- place: core/src/linear/search/classical.rs:132
- kind: search change
- cost: small
- candidate: yes

**Claim.** The classical refuter evaluates every term the goal reaches at every node of its backtracking, with no propagation, so its work bound of 2²⁶ term evaluations allows about 2²⁶ / terms assignments before a search that ended `Unknown` stays `Unknown`.

**Witness.** `core/src/linear/search/classical.rs:116` and `:128-132`:

```rust
    let cost = (order.len() as u64).max(1);
```
```rust
        if done.saturating_add(cost) > work || stop(Progress::new(Phase::Refute, cost, done)) {
            return None;
        }
        done += cost;
        let value = evaluate(terms, &order, &members, &assigned, &mut values);
```

`evaluate` (`:168-198`) walks all of `order` each time. Ordinary logic has a DPLL (`ordinary::resolution`) and, with `sat`, a SAT solver over the same reading's shape.

**Change.** Measure first: on the target sets, count the rows whose search ends `Unknown` or `Exhausted` and whose refuter stops at `classical_work`. If there are such rows, evaluate incrementally (re-evaluate only the terms above the atom just set, in `order`), or hand the reading to the ordinary suite's DPLL; the assignment found is checked by `Disproof::check` as now.

**Value.** A user gets a checked `Classical` refutation where the refuter gives up today; the verdict of an `Unknown` row may become `Unprovable`.

**Net.** Candidate: the target sets' verdicts (`bench/targets.sh`, `summary --against … --counters`) and a journey with a goal of many atoms; a search change, since it changes verdicts. The refutation's check and its Rocq lemma (`classical_refutation`) are untouched.

## F-1c-14

- dimension: text
- criterion: E6
- place: core/src/linear/search/refutation.rs:746
- kind: refactor
- cost: small
- candidate: no

**Claim.** `write_unbalanced` builds the two-sided phrase and the intuitionistic phrase with the same code twice, differing only in the parenthesis that ends them.

**Witness.** `core/src/linear/search/refutation.rs:746-782` and `:783-817`; each computes the same:

```rust
        let (more, fewer) = if least > 0 {
            ("right", "left")
        } else {
            ("left", "right")
        };
```

and the same `(place, copies)` match over `copied` (`:764-776` and `:798-810`), then the same two `write!` templates, the intuitionistic one with " (an implication's antecedent counts on the other side)" where the two-sided one has `{crossed}`.

**Change.** One branch for both, with the closing note as the only difference: `Words::Two(placed)` gives `crossed` from `placed`, `Words::Intuitionistic` always the implication note.

**Value.** A change of the wording (the writing rules' rework) is made once and the two readings cannot drift apart.

**Net.** `a_two_sided_sequent_is_refuted_by_its_sides`, `an_intuitionistic_count_names_the_empty_side`, the manual's and README's `prove` examples that print a refutation (`cli/tests/readme.rs`).
