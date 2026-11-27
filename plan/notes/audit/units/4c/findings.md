# Unit 4c: findings

Read in full: 17 files. Not read in full: none.

The finder's remarks: I read all 17 files in full. I could not split F-4c-01's measured time between the read-back's key writing (`mapped_atom` → `predicate_atom`) and `bindings_text`. A release build or a DHAT run of the library alone would split it, and the unit 7a or 8 verifier should check whether the command and the protocol print `bindings_text` unbounded too. For the handovers: this unit uses `linear::search::Reason` (Stopped, WorkLimit, MemoryLimit, IndexLimit, Incomplete, PathLength, WitnessDepth) in every engine, and `resolution/outside.rs:19` uses `memory::Account` only through `holding`, `charge`, `over` and `limit` (F-arch-06). There is no SipHash map in the unit. `herbrand::herbrand_default`, named nowhere outside `core/src`, belongs to a pattern of four `*_default` functions in ordinary logic (`normal`, `negative`, `interpolant`, `herbrand`) against `decide`/`decide_within` elsewhere: a naming question for the API proposal (unit 4a or 4b). `resolution` and `resolution::first_order` reuse six names (`Literal`, `Clause`, `Clauses`, `Refutation`, `Statistics`, `clauses_of`) with other shapes (`text(&Clause) -> ClauseText` against `text(usize) -> String`), which the API proposal should settle with F-4c-10.

## F-4c-01

- dimension: performance
- criterion: S1
- place: core/src/ordinary/resolution/first_order.rs:702 (mapped_atom, called from read_back at :2004) and :216 (bindings_text)
- kind: refactor
- cost: small
- candidate: no

**Claim.** First-order resolution writes terms out in full with no bound: a unifier that doubles a term per variable makes `resolve --timeout 1s` run 22 s at n = 24, growing about fourfold every two variables.

**Witness.** Measured with the debug binary of this tree (`target/debug/deduxit`), in a capped scope on cores 13-15. The sequent for n is `forall x1..xn. ~p(x1, .., xn, x2, .., xn), forall y2..yn. p(f(y2, y2), .., f(yn, yn), a, y2, .., yn) |-`, which resolves to the empty clause in one step with `y2 := f(y3, y3)`, .., `yn := a`:

```
deduxit resolve --file sN.txt --timeout 1s            n=18: 1.12 s   n=22: 4.64 s (37 MB of text)
deduxit resolve --file sN.txt --timeout 1s --format json > /dev/null   n=20: 1.68 s   n=24: 21.99 s
```

The output says "no derivation of LK was read back (the read-back was stopped)". `applied_atom` refuses an atom before its key is written:

```rust
        if let Some(limit) = self.most_terms
            && size > limit
        {
            return Err(Error::Refused(Refusal::Terms { terms: size, limit }));
        }
```

The read-back's ground instances go through `mapped_atom`, which has no such check and calls `predicate_atom`, and that writes the key's text. The loop also asks no stop:

```rust
                let atom = set.mapped_atom(l.atom, &mut |formulas, j| {
                    theta
                        .get(j as usize)
                        .copied()
                        .or(default)
                        .unwrap_or_else(|| formulas.interner().term(FoTerm::Meta(j)))
                })?;
```

`bindings_text` also writes every binding whole (`write_term(&self.formulas, t, &mut text, &|j| name(j));`). The run cannot say how the time splits between these two writes and the CLI's printing.

**Change.** Measure each atom by the memoised `size` in `mapped_atom` (as `applied_atom` does), refuse with `Refusal::Terms` past `most_terms`, and ask the stop in the read-back's instance loop. Bound `bindings_text` too: abbreviate past a length, or refuse. This fixes behaviour past the bound, so it is not a pure refactor.

**Value.** A user of the command, the page or the library gets the time and memory limits they set. Today a 2 KB sequent ignores `--timeout` exponentially.

**Net.** `resolution/first_order.rs`'s tests (`a_term_that_unfolds_past_the_bound_is_refused` is the model for a new one on this sequent), the manual's `resolve` examples and the wire form at level 29. The refutation's check is untouched; the read-back that feeds Herbrand's checked derivation now refuses past the bound.

## F-4c-02

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/connection/prove.rs:347 and core/src/ordinary/connection/intuitionistic/prove.rs:358
- kind: refactor
- cost: medium
- candidate: no

**Claim.** F-arch-24 is confirmed from inside both files: the intuitionistic search is the classical one copied, with seven hook points, and nothing pins either search's counters today.

**Witness.** `duplicates.md` has 13 rows between the two files (`prove.rs:650`↔`ip:705` 108 lines, `:485`↔`:535` 101, `:83`↔`:61` 115, …). The types `PathNode`, `Item`, `Goal`, `Lemma`, `Cursor`, `Alt`, `Choice` and `Way` are identical, and so are `cons`, `elements`, `try_start`, `try_clause`, `may_be_lemma`, `on_path`, `try_literal`, `next_way` and `inside`. One example (`prove.rs:653`, `ip:708`):

```rust
    fn on_path(&self, prefix: u32, path: u32) -> bool {
        if prefix == NONE {
            return true;
        }
        let depth = self.paths[prefix as usize].depth;
```

The two files differ only here:
1. at `g == NONE`: `prefixes_unify` plus `accept` (the read-back), else backtrack;
2. at `Solved`: the classical search settles a ground literal, the intuitionistic one only under restricted backtracking (`let _ = mark;`);
3. `tick` adds the prefix characters read;
4. `same` also compares prefixes;
5. `in_clause_of`, `region_for` and `extension_clauses`: the classical search has an `is_ground` positional shortcut, the intuitionistic one counts `renames[clause]`;
6. `apply` adds `connect` and `renames`;
7. `End` has `Exhausted` only in the classical search.

The extra fields are `prefixes`, `renames`, `eqs` and `rejected` (and `Marks.eqs`, `Clock.work`). The tests pin verdicts only: `connection/tests.rs` (10 tests, one bound on steps at :310) and `intuitionistic/tests.rs` (3; `agrees_with_ljt` holds the unified-but-not-read-back count at 0). No ratchet journey and no row of `bench/RESULTS.md` runs either search.

**Change.** Make one crate-private `Search<'m, C: Calculus>` with a trait of the seven hooks, monomorphised twice so that no `dyn` enters the loop:

```rust
trait Calculus {
    type Proof;
    const POSITIONAL: bool;
    fn renames(&self, clause: u32) -> bool;
    fn extra_work(&mut self) -> u64;
    fn same(&mut self, m: &Matrix, t: &Terms, a: (u32, u32), b: (u32, u32)) -> bool;
    fn connect(&mut self, m: &Matrix, t: &Terms, a: (u32, u32), b: (u32, u32)) -> bool;
    fn settles(&self, ground_and_no_older: bool) -> bool;
    fn at_proof(&mut self, t: &Terms, used: &[(u32, u32)]) -> Result<Option<Self::Proof>, Ended>;
    fn mark(&self) -> u32;
    fn restore(&mut self, m: u32);
    fn held_bytes(&self) -> usize;
    fn longest(&self) -> usize;
}
```

The classical side is a zero-sized `Classical` whose hooks are constants; the intuitionistic side is `Prefixed { prefixes, renames, eqs, rejected, read_back }`. That also removes the `RefCell` around the stop in `intuitionistic/mod.rs:101`. Before the merge, record an oracle: `deduxit-bench run --tptp bench/iltp-first-order/Problems --engines connection` in both modes, on one thread, at the base commit. Add `reductions` and `lemmata` to the row beside `nodes` (extensions) and `work` (steps).

**Value.** A reader of about 700 duplicated lines. Every later change to the search (restricted backtracking, a strategy, a bound) is made once instead of twice.

**Net.** No instrument pins the counters today. The merge needs the harness rows above, compared on decided rows (`verdict`, `nodes`, `work`, plus `reductions` and `lemmata` once added), with `connection::Statistics` equal on each, plus both test files. It touches a search (P13): a moved counter makes it a search change. Every derivation stays checked.

## F-4c-03

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/herbrand.rs:1065 (Build) and core/src/ordinary/connection/read_back.rs:113 (ReadBack)
- kind: refactor
- cost: medium
- candidate: no

**Claim.** Herbrand's derivation builder repeats the classical connection read-back's construction of LK: frames, steps, positions, the axiom with weakenings, the inference push with its memory account, the rule order, and the one- and two-premise rules.

**Witness.** `duplicates.md`: `read_back.rs:664`↔`herbrand.rs:1307` (322 tokens), `:898`↔`:1562` (239), `:611`↔`:1256` (163), `:840`↔`:1468` (103) and `:1048`↔`:1669` (118). `Frame`, `Step`, `position`, `replaced` and `closes_a_premise` are the same code. `close` is the same up to how it returns. `read_back.rs:666` and `herbrand.rs:1309` read:

```rust
        let Some(&shared) = left.iter().find(|k| right.contains(k)) else {
            return Ok(None);
        };
        // `A ⊢ A`, then every other formula weakened in, in order.
        let mut id = self.push(
            vec![shared],
            vec![shared],
            Rule::Axiom,
```

`core-ordinary.md:2471` records that `Build` "follows the connection read-back's construction" and gives no reason for a copy.

**Change.** Move the shared machinery to one crate-private module, for example `ordinary::lk_build`: `Branch<I>`, `Frame`, `Step`, `sides`, `position`, `replaced`, `close`, `closes_a_premise`, `push` (stop, memory, `u32` index), `derive` and the rule order. Make it generic over an `Oracle` trait that says which items are relevant, which witnesses a γ-item takes next, and the eigenvariable of a δ-item. The connection read-back implements it by occurrences and needed witnesses, Herbrand by matching the kept instances.

**Value.** A reader, and every later fix to the construction (the stop, the memory charge, the order rule of the least nested witness): today each fix is made twice, and the charges already differ (the `push` estimates at `read_back.rs:525` and `herbrand.rs:1194`).

**Net.** Both derivations go through `Derivation::check`. The tests of `connection/tests.rs`, `herbrand.rs` and `resolution/first_order.rs` cover them, and `herbrand::Statistics::inferences` and the derivations' sizes in those tests pin the construction. The certificates are untouched and no search is touched.

## F-4c-04

- dimension: api
- criterion: D4
- place: core/src/ordinary/connection/mod.rs:120 and core/src/ordinary/resolution/mod.rs:842
- kind: API break
- cost: medium
- candidate: no

**Claim.** The connection prover's public entry takes the linear suite's `search::Options` and reads two of its fields; first-order resolution's `clauses_of` takes the same type and only passes it on to `normalize`.

**Witness.** ```rust
pub fn decide(
    sequent: &Sequent,
    logic: Logic,
    options: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
```

The doc says "It reads the search options' path_length and restricted_backtracking and nothing else of them" (`mod.rs:94-97`). `run` turns them into the crate-private `prove::Config { path_length, restricted }` (`mod.rs:171-174`). `resolution::clauses_of(sequent, form, search: &crate::linear::search::Options, …)` exists only to pass the value to `normal::normalize`. This is F-arch-02 seen from the engines.

**Change.** Give the connection prover options of its own, `connection::Options { path_length: Option<u32>, restricted_backtracking: bool }` with `with_…` methods, or fields of `ordinary::Options`, and take them in `decide`. Drop the parameter from `clauses_of` once `normalize` takes the copy bound from its own options. Write a migration note.

**Value.** A user of ordinary logic no longer builds a value of the linear suite to bound the connection prover. D4 holds, and the settings keys can move to `ordinary.*` (F-arch-02).

**Net.** Verdicts and statistics unchanged: the same two values reach `prove::Config`. The tests of `connection/`, `decide.rs` and the harness's call (`bench/src/ordinary.rs:181`) must be updated. Neither the search nor the checker changes.

## F-4c-05

- dimension: api
- criterion: C-CUSTOM-TYPE
- place: core/src/ordinary/resolution/first_order.rs:1197
- kind: API break
- cost: medium
- candidate: no

**Claim.** A first-order refutation keeps why it was not read back as a `String` beside an `Option` derivation, so a stop, a memory refusal, the planned refusal for paramodulation and a read-back defect all become one text.

**Witness.** ```rust
    Refuted {
        /// The refutation, checked.
        refutation: Box<Refutation>,
        /// The derivation of LK of the sequent, checked; `None` where the
        /// read-back did not reach, and the verdict rests on the
        /// refutation and Skolem's theorem.
        derivation: Option<Box<Derivation>>,
        /// Why the read-back did not reach, where it did not.
        not_read_back: Option<String>,
    },
```

It is filled at `:1342-1346`:

```rust
                    Ok(derivation) => (Some(Box::new(derivation)), None),
                    Err(error) => (None, Some(error.to_string())),
```

The consumers match on the pair of options (`cli/src/resolve.rs:208-220`, with an unreachable `(None, None)` arm) or write the string (`serialize/ordinary.rs:1958`).

**Change.** Make it `derivation: Result<Box<Derivation>, Error>` (or `ReadBack::{Derived(..), NotRead(Error)}`), so that `Error::kind()` and `code()` tell a bound from a defect. The JSON form writes the error as the crate's error form, `{code, kind, message}`.

**Value.** A front end or a script can tell "stopped at the time limit" from "LK has no rule for paramodulation" and from a defect without reading prose, and gets the exit-status sorting the error family gives elsewhere.

**Net.** `resolution/first_order.rs`'s tests (`the_read_back_asks_the_stop`, `valid_sequents_are_refuted_and_read_back`), the manual's `resolve` examples, the wire level 29 (`wire-schema` check) and `cli/src/resolve.rs`. The checker and the search are untouched.

## F-4c-06

- dimension: performance
- criterion: P2
- place: core/src/ordinary/connection/prove.rs:532, :563, :815 and core/src/ordinary/connection/intuitionistic/prove.rs:582, :613, :645, :862
- kind: refactor
- cost: small
- candidate: yes

**Claim.** The connection searches allocate on every decomposition, every extension-clause test and (intuitionistic) every regularity or lemma comparison, where a borrowed slice or a reused buffer would do.

**Witness.** ```rust
            let nodes = self.matrix.kids(clause).to_vec();
            return Some(self.elements(&nodes, copy, path, positive, goal.next));
```

`self.matrix` is a `&'m Matrix`, so `let m = self.matrix; m.kids(clause)` borrows for `'m`, not from `self`, and the `to_vec` is needless. `region_for` builds `let mut copies = Vec::new();` on every call (`prove.rs:815`, `ip:862`), once per clause around each target. In the intuitionistic `same` closure, each element of the path and each lemma compared calls `search.prefixes.of_literal(..)`, which allocates the chain (`prefix.rs:200`) and the prefix (`prefix.rs:405`).

**Change.** Copy the `&'m Matrix` out of `self` before taking `kids`. Walk the path's copies lazily in `region_for` (the path's `ext` then `start`, as an iterator). Compare prefixes position by position, or into a scratch buffer the search keeps, instead of building a `Vec` per comparison.

**Value.** The machine: fewer allocations per step in both searches, the default first-order classical method included (`Method::Connection`, up to `CONNECTION_WORK` = 2^26 steps).

**Net.** Candidate: no journey runs the prover. Measure DHAT allocations and pinned time on the connection rows of the ILTP first-order set (`deduxit-bench run --tptp bench/iltp-first-order/Problems --engines connection`, one thread), with `nodes` and `work` equal on every decided row. It touches a search but must not move a counter.

## F-4c-07

- dimension: performance
- criterion: P8
- place: core/src/ordinary/resolution/first_order.rs:1692
- kind: refactor
- cost: small
- candidate: yes

**Claim.** The first-order given-clause search queues a resolution step for every pair of opposite-sign literals whatever their predicates, and each attempt clones both parent clauses and interns renamed copies of the second's atoms before `pairs` finds the predicates differ.

**Witness.** ```rust
                for (i, x) in clause.literals.iter().enumerate() {
                    for (j, y) in partner.literals.iter().enumerate() {
                        if x.positive != y.positive {
                            steps.push(Step::Resolution {
```

In `made` (`:298-338`): `let a = self.items.get(first).ok_or_else(out_of_range)?.clone();`, the same for `b`, then `atom: self.shifted(l.atom, shift)?` for every literal of `b` (interned into the arena for good), and only then `let Some(pairs) = self.pairs(x.atom, y_atom) else { return Ok(Err("the two literals are of different predicates")); };`. Paramodulation already prefilters on head symbols (`:577-579`). Failed attempts are not counted (`:1727-1730`).

**Change.** Push a resolution step only where the two atoms share a predicate symbol (or are the same propositional atom). In `made`, test the predicates before cloning and shifting.

**Value.** The machine and the arena: fewer clones, hash lookups and interned atoms per given clause in `resolve`, the protocol's `ordinary.resolve` and the step-by-step session's set.

**Net.** Candidate. `Statistics { clauses, steps }`, the refutation and its text stay the same, because only steps that fail are skipped. Arena ids change, but none is compared by order. Measure DHAT and callgrind on `resolve` of an ILTP first-order problem with paramodulation off; no journey runs it yet. The checker computes each kept step again.

## F-4c-08

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/connection/matrix.rs:46 and core/src/ordinary/connection/intuitionistic/prefix.rs:44; matrix.rs:1048 and prefix.rs:492; connection/read_back.rs:178, intuitionistic/read_back.rs:176, herbrand.rs:1858
- kind: refactor
- cost: small
- candidate: no

**Claim.** Small helpers of the connection module and of Herbrand exist two to four times word for word: the build's `Budget`, the sorted `union`, the scan for the first eigenvariable number past the arena's, the statistics copied from `Counts`, and the mapping of a stopped build to unknown.

**Witness.** Both `matrix.rs:45` and `prefix.rs:43` hold:

```rust
/// The build's account: the stop, the memory bound, and the work done.
struct Budget<'a> {
```

They have the same `poll`. `fn union(a: &[u32], b: &[u32]) -> Vec<u32> {` is at `matrix.rs:1048` and `prefix.rs:492` (`duplicates.md`: 197 and 175 tokens). The eigenvariable base appears at `read_back.rs:178-187`, `intuitionistic/read_back.rs:176-185` and `herbrand.rs:1858-1868` (and `derivation.rs:1909`):

```rust
            FoTerm::Eigen(e) => Some(e.get().saturating_add(1)),
```

`Statistics { steps: counts.steps, … }` is at `connection/mod.rs:199` and `intuitionistic/mod.rs:135`, and the `unknown` closure with the match on `Matrix::of`'s refusals at `mod.rs:158-170` and `intuitionistic/mod.rs:76-92`.

**Change.** Keep one `Budget` and one `union` in `matrix.rs`, imported by `prefix.rs`. Add a `FirstOrder` or `Formulas` method `next_eigen()` used by all four read-backs. Add `impl From<prove::Counts> for Statistics` and one `matrix_or_unknown` helper.

**Value.** A reader, and a later change to the build's polling or the eigenvariable numbering made once.

**Net.** The tests of `connection/`, `intuitionistic/` and `herbrand.rs`. No search order or counter changes, and the derivations stay checked.

## F-4c-09

- dimension: performance
- criterion: E2
- place: core/src/ordinary/herbrand.rs:1824, :1840, :1870
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Herbrand's builder walks witness terms as trees with no visited set, once per witness, item and step, and sorts by a key that walks the term on every comparison, where the connection read-back walks shared terms as graphs.

**Witness.** ```rust
    fn skolem_terms(&self, t: FoTermId) -> Vec<FoTermId> {
        let tables = self.formulas.first_order();
        let (mut found, mut stack) = (Vec::new(), vec![t]);
        while let Some(u) = stack.pop() {
            if let FoTerm::App(f, args) = tables.terms[u.index()] {
                if self.symbols.contains(&f) && !found.contains(&u) {
```

`available` (`:1817`) calls it for every witness tried in `step` (`:1418`, `:1439`), and `nesting` (`:1840`) is also a tree walk, called per witness at `:1461`. At `:1870`: `made.sort_by_key(|&s| self.nesting(s));`. The connection read-back's `available` and `skolem_eigens` use `Seen` (`read_back.rs:426`, `:448`), and `core-ordinary.md:854-868` records why: shared terms are exponential as trees. The witnesses reach Herbrand's builder from resolution's read-back, whose terms share parts (F-4c-01).

**Change.** Walk with `terms::Seen` (moved where both modules reach it), memoise `skolem_terms` and `nesting` per term in the builder, and use `sort_by_cached_key` at `:1870`.

**Value.** The machine: Herbrand's and resolution's read-back no longer pay a tree walk per witness per step on shared terms.

**Net.** Candidate. `herbrand.rs`'s tests (`each_quantifier_takes_the_witnesses_its_instances_give` bounds the inference count) and `first_order.rs`'s read-back tests. The derivations are checked. Measure with callgrind on `herbrand` of the F-4c-01 sequent family at small n.

## F-4c-10

- dimension: api
- criterion: S1
- place: core/src/ordinary/resolution/first_order.rs:1136
- kind: API break
- cost: small
- candidate: no

**Claim.** The first-order refutation's check takes a stop but no `&Limits`, unlike the propositional `Refutation::check(&limits, stop)` beside it and the crate's rule that every long call takes both.

**Witness.** ```rust
    pub fn check(&self, mut stop: impl FnMut(Progress) -> bool) -> Result<(), Error> {
```

It clones the arena (`formulas: self.clauses.formulas.clone()`) and computes every step's unifier and clause again, with no work or memory bound. The propositional one is at `refutation.rs:191`:

```rust
    pub fn check(&self, limits: &Limits, stop: impl FnMut(Progress) -> bool) -> Result<(), Error> {
```

`core.md:482`: "Every long call takes `&Limits` and `stop`".

**Change.** Take `limits: &Limits` and count a step against `limits.work` and the arena's growth against `limits.memory_bytes`, as the propositional checker counts its resolvents. Call it with the search's limits at `:1332`.

**Value.** A library user checks a refutation within the bounds they set, by the same signature for both kinds of resolution.

**Net.** It touches a checker, whose verdicts stay the same within the bounds: `the_checker_refuses_a_changed_refutation` and the other tests of `first_order.rs`, and the protocol's check of `ordinary.resolve`.

## F-4c-11

- dimension: architecture
- criterion: E5
- place: core/src/ordinary/resolution/first_order.rs:173 and core/src/ordinary/herbrand.rs:1063
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `resolution/first_order.rs` (2 563 lines) and `herbrand.rs` (2 211) each hold several things with clear seams.

**Witness.** `first_order.rs`:
- the clauses and their one inference function (`Clauses`, `made`, `paramodulant`, `concluded`, `:173-945`);
- the printers (`write_term`, `clause_text`, `:1030-1104`);
- the refutation and its checker (`:1106-1188`);
- `resolve` and the given-clause `Search` (`:1190-1875`);
- the read-back with `Copied`, `symbol_in` and `default_constant` (`:1877-2180`);
- tests (`:2182-2563`).

`herbrand.rs`:
- the instance search (`Universe`, `Search`, `propositional`, `:307-670`);
- the term-matching helpers (`:757-927`);
- the LK builder `Build` (`:929-1894`), which F-4c-03 shares with the connection read-back;
- tests (`:1964-2211`).

**Change.** Split `first_order.rs` into a directory: `clauses.rs` (inference), `check.rs` (`Refutation`), `search.rs`, `read_back.rs` and `text.rs`. Split `herbrand.rs` into the instance search and the builder, which moves to the shared module of F-4c-03.

**Value.** A reader can hold each file. The checker of first-order refutations is a file of its own, the trust boundary at a glance.

**Net.** Moves only: the tests of both files and the `serialize/ordinary.rs` forms. No search or checker logic changes.

## F-4c-12

- dimension: text
- criterion: C-EXAMPLE
- place: core/src/ordinary/connection/mod.rs:4
- kind: text
- cost: small
- candidate: no

**Claim.** The connection module, the public entry to the first-order prover of both classical and intuitionistic logic, has no example on the module or on `decide`, while `herbrand` and `resolution` lead with one.

**Witness.** `surface.md` lists `ordinary::connection::decide`, `Outcome` and `Statistics` among the items without an example (`:673-675`). The module docs (`mod.rs:4-39`) are prose only. `herbrand.rs:29-39` and `resolution/mod.rs:42-51` each have a running doctest.

**Change.** Add a doctest to the module docs that decides `forall x. p(x) |- p(f(a))` classically and `|- ~~(a \/ ~a)` intuitionistically, and reads `Verdict::Valid(Evidence::Derivation(_))` and `statistics.extensions`. Use the crate's `cfg_attr(feature = "parse", doc = "```")` pattern.

**Value.** A library user finds how to call the prover, which options value it needs and what it returns.

**Net.** The doctest itself, under `cargo test --doc` and the feature-pair check. No code changes.

## F-4c-13

- dimension: idiom
- criterion: E1
- place: core/src/ordinary/resolution/refutation.rs:228; saturate.rs:103, :115; dpll.rs:138
- kind: refactor
- cost: small
- candidate: no

**Claim.** The propositional resolution code tests an `Option` with `is_some_and` and then unwraps the same option with a dead `u64::MAX` default, where a `let` chain binds it once, as `first_order.rs:1722` already does.

**Witness.** ```rust
            if limits.work.is_some_and(|w| k as u64 >= w) {
                return Err(Error::Refused(Refusal::Work {
                    limit: limits.work.unwrap_or(u64::MAX),
                }));
            }
```

The same at `saturate.rs:103-108` (`options.steps`), `:115-120` (`options.clauses`) and `dpll.rs:138-141`. The first-order search writes `if let Some(limit) = self.options.steps && self.statistics.steps >= limit { … Bound::Steps { limit } }`.

**Change.** Write each as `if let Some(limit) = x && cond(limit) { … limit … }`. In DPLL, build `past` from the bound inside the branch that returns it.

**Value.** A reader, and no `u64::MAX` that reads like a real bound.

**Net.** `resolution/tests.rs` and the manual's `resolve` examples. The checker's logic is unchanged.

## F-4c-14

- dimension: performance
- criterion: P3
- place: core/src/ordinary/resolution/refutation.rs:248
- kind: refactor
- cost: small
- candidate: no

**Claim.** The propositional refutation checker clones both parent clauses of every resolvent only to borrow them.

**Witness.** ```rust
            let (a, b) = (
                &self.clause_unchecked(r.first),
                &self.clause_unchecked(r.second),
            );
```

The helper at `:286-290`:

```rust
    fn clause_unchecked(&self, k: usize) -> Clause {
        self.clause(k)
            .cloned()
            .unwrap_or_else(|| unreachable!("a number below the length"))
    }
```

`self.clause(k)` already returns `Option<&Clause>`.

**Change.** Return `&Clause` from `clause_unchecked`, dropping `.cloned()`.

**Value.** The checker of every resolution refutation (the command's `check`, `decide`'s own check, the protocol) makes two allocations fewer per resolvent.

**Net.** It touches a checker; behaviour is kept: `resolution/tests.rs` (`a_tautology_is_no_way_to_the_empty_clause` and the hand-built refutations) and the mutation baseline of the checkers.

## F-4c-15

- dimension: idiom
- criterion: E3
- place: core/src/ordinary/connection/intuitionistic/prefix.rs:447; intuitionistic/prove.rs:434; intuitionistic/read_back.rs:1079; matrix.rs:259
- kind: refactor
- cost: small
- candidate: no

**Claim.** Leftovers obscure the code: a parameter kept and discarded, a field bound and discarded, a method that only forwards, and a four-tuple of `&mut` passed as a type alias.

**Witness.** ```rust
        let _ = matrix;
        let chain = self.chain(o);
```

(`of_occurrence`, `prefix.rs:447`; its two callers pass a matrix for nothing).

```rust
                    let _ = mark;
```

(`intuitionistic/prove.rs:434`, after destructuring `mark` from `Item::Solved`).

```rust
    fn instance_term(&mut self, tm: u32) -> FoTermId {
        self.term(tm)
    }
```

(`intuitionistic/read_back.rs:1079`).

```rust
type Memo<'a> = (
    &'a mut [u32],
    &'a mut HashMap<FoTermId, u32>,
    &'a mut HashMap<FoTermId, u32>,
    &'a mut [u32],
);
```

(`matrix.rs:259`).

**Change.** Drop `of_occurrence`'s `matrix` parameter. Write `mark: _` (or `..`) in the pattern. Call `term` directly. Make `Memo` a struct with named fields (`closed`, `local`, `parameters`, `skolems`).

**Value.** A reader.

**Net.** The tests of `connection/` and `intuitionistic/`; nothing observable changes.

## F-4c-16

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/herbrand.rs:350, :759 and core/src/ordinary/resolution/first_order.rs:1396
- kind: refactor
- cost: small
- candidate: no

**Claim.** The walk that collects the terms of a formula's atoms, nodes and terms each seen once, is written again in Herbrand's universe, Herbrand's witnesses, resolution's open-witness check and three times in `instance.rs`.

**Witness.** `duplicates.md`: `herbrand.rs:350`↔`herbrand.rs:763` (185 tokens), and `herbrand.rs:356`/`:769`↔`instance.rs:132`, `:261`, `:389`. Each has the same shape:

```rust
            match formulas.node(n) {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        terms.extend_from_slice(tables.list(args));
                    }
                }
```

`first_order.rs:1396` (`refuse_open_witnesses`) writes it with `node.operands()`.

**Change.** Add one crate-private `Formulas::terms_of(roots) -> impl Iterator<Item = FoTermId>` (each term once, subterms included), and let each site filter what it wants: constants and function symbols, metavariables, the depth order.

**Value.** A reader, and one place to keep the walk's no-recursion and seen-once rules.

**Net.** The tests of `herbrand.rs`, `first_order.rs` and `instance.rs`. Herbrand's witness order is kept by the existing sort (`:793`), so its derivations and inference counts are unchanged.

## F-4c-17

- dimension: text
- criterion: T1
- place: core/src/ordinary/connection/mod.rs:29
- kind: text
- cost: small
- candidate: no

**Claim.** The module's key promise, when "not valid" can be trusted, is one sentence of five lines with two nested parentheses.

**Witness.** ```rust
//! defect). Where the matrix has no variable at all (no quantifier needs
//! a witness term: every `∀` is one to prove and every `∃` one to assume,
//! negations and implications taken into account), every copy of a
//! clause is the clause, the
//! search is the calculus's whole search and ends, and a search that finds
//! no proof shows the sequent not valid (the calculus's completeness,
//! Otten 2011, Theorem 4, with positive start clauses and regularity,
//! Lemmas 4 and 5).
```

**Change.** Rewrite: "The prover answers \"not valid\" only where no quantifier needs a witness term: every `∀` is proved and every `∃` assumed. Then each clause has one copy, the search tries every proof, and its failure shows the sequent not valid (Otten 2011, Theorem 4). Otherwise a failed search is \"unknown\"."

**Value.** A library user learns when "not valid" holds without parsing the sentence.

**Net.** Docs only: `cargo doc` and the `typos` check.
