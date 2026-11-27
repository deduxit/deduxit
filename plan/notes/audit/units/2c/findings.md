# Unit 2c: findings

Read in full: 12 files. Not read in full: none.

The finder's remarks: I read all 12 files in full and could not judge the inverse method's performance (no journey or target row runs it, F-2c-05); I did not re-derive the soundness arguments of core-horn.md or core-inverse.md, and found no `std` SipHash map in the unit (every table uses `crate::hash`). On F-2b-03: the Horn engine needs of the focused engine only `counts::refutation(forest, goal, fragment, mode, &Account, stop) -> Refutation` (horn/mod.rs:88), and the inverse method only `bias::signs(forest, Bias) -> Box<[Sign]>` (inverse/mod.rs:144); neither touches the count rows, `Switches` or anything else of `focus`. I confirm F-2a-06's leftover at inverse/mod.rs:98 (`found.result.map_err(|r| r.as_set(options, limits))`); the Horn engine applies no `as_set`. I agree with F-arch-07: `lltp::{clauses, Clauses, Status}` is TPTP clause syntax that ordinary logic's reader uses, and the 6 004 `read-tptp` blocks unit 4a saw are F-2c-02's per-line `Vec<char>`. For unit 1a and the API proposal: `mist.rs:308-405` (`Arena`: fresh atom, push term, roots, antecedents over the crate-private `Sequent::from_parts`) is the shape of the public sequent builder the consumer test's point 1 asks for, and families.rs builds every sequent by printing text and parsing it for want of one. For unit 8/bench: DHAT counts the whole process, so the input generation weighs in the journeys' tables (F-2c-12); and for unit 4a, ordinary's `Error::Tptp` messages may lack lines as mist's do (F-2c-03).

## F-2c-01

- dimension: api
- criterion: C-VALIDATE
- place: core/src/linear/families.rs:1111
- kind: refactor
- cost: small
- candidate: no

**Claim.** The public encodings `three_partition`, `three_partition_mll` and `partition`, offered "to build a problem from data of your own", panic on an item of size 0 or on 0 bins, which none of their `# Panics` sections names.

**Witness.** `power` writes nothing for n = 0, so the generated text has an empty operand, and `parse` panics on it:

```rust
// core/src/linear/families.rs:1099-1101
fn power(formula: &str, n: u32, separator: &str) -> String {
    vec![formula; n as usize].join(separator)
}
```
```rust
// core/src/linear/families.rs:704
            .map(|j| format!("({} * t{j} -o d{i})", power(&format!("b{j}"), a, " * ")))
```
```rust
// core/src/linear/families.rs:1110-1111
    Sequent::parse_within(text, &unbounded)
        .unwrap_or_else(|e| panic!("a generated sequent parses: {e}"))
```
A scratch crate outside the repository (default features, built on cores 13-15) called each one under `catch_unwind`. All five panicked at `families.rs:1111:29`:
```text
three_partition(&[0, 1, 2], 1, 3)      unexpected "*" at line 1, character 27
three_partition(&[1, 1, 1], 0, 3)      unexpected "," at line 1, character 1
three_partition_mll(&[0, 1, 2], 1, 3)  unexpected ")" at line 1, character 9
three_partition_mll(&[1, 1, 1], 0, 3)  unexpected end of input at line 1, character 34
partition(&[0, 2, 2])                  unexpected ")" at line 1, character 22
```
The docs' `# Panics` name only an odd total (`partition`), a non-power of two (`counter`), fewer than three variables (`qbf`) and `shared` 0 (`wide`).

**Change.** Make `power` write the unit of its separator for n = 0 (`1` for ` * `, `bot` for ` | `, `top` for ` & `), so that an item of size 0 and a problem of 0 bins have their encoding. Then name in `# Panics` the preconditions that remain, or return them as an `Error` like `Family::instance` does.

**Value.** A user who encodes their own instances gets a sequent, not a process abort, for every input the docs allow. Every input that worked before gives the same text.

**Net.** Every size above 0 writes the same text, so `verdicts_as_constructed` and the `batch-families` ceiling are unchanged; one unit test per encoding at size 0 pins the fix. No checker, search or certificate is touched.

## F-2c-02

- dimension: performance
- criterion: P2
- place: core/src/linear/lltp.rs:300
- kind: refactor
- cost: small
- candidate: no

**Claim.** `lltp::clauses`, which both the LLTP reader and ordinary logic's TPTP reader call, collects every line into a fresh `Vec<char>`, and these allocations are 57 % of the `read-lltp` journey's blocks and 32 % of `read-tptp`'s.

**Witness.** ```rust
// core/src/linear/lltp.rs:300-306
        let chars: Vec<char> = before.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            let name = |j: usize| {
                chars
                    .get(j)
                    .is_some_and(|&c| c.is_ascii_alphanumeric() || c == '_')
            };
```
DHAT (`target/audit/dhat.md`), journeys of 2 000 lines (`bench/src/journeys.rs:292`):
```text
read-lltp: 6007 blocks, 57 %  deduxit::linear::lltp::read (lltp.rs:120)
read-tptp: 6004 blocks, 32 %  deduxit::ordinary::parse::read_tptp (parse.rs:942)
```
That is about three blocks per line: `collect` from `Chars` starts below the line's length and grows. Every character the lookahead tests (`-`, `.`, `o`, ASCII name characters) is one byte, and a byte of a multi-byte character is never an ASCII name character.

**Change.** Walk `before.char_indices()` and do the lookahead (`name(i - 1)`, `name(i + 1)`, `chars[i + 1] == 'o'`, `name(i + 2)`) on byte offsets into `before.as_bytes()`. This gives the same answers and allocates nothing per line, and `code` keeps one character per character of the file, which `in_file` relies on.

**Value.** Reading an LLTP or TPTP file, in the command, the harness and ordinary logic's `read_tptp`, makes about 6 000 fewer allocations per 2 000 lines. That is half the LLTP reader's blocks and a third of the TPTP reader's.

**Net.** `lltp::tests::reads_problems` and `parse_errors_are_placed_in_the_file` cover the name and `-o` cases and the placing of errors, and the TPTP fuzz target covers the rest. The instruction effect is for the ratchet's `read-lltp` and `read-tptp` rows (a lowered ceiling is a commit of its own). No search or checker is touched.

## F-2c-03

- dimension: api
- criterion: L5
- place: core/src/linear/mist.rs:434
- kind: refactor
- cost: small
- candidate: no

**Claim.** Every error of the `.spec` reader names a token but never its line, so a user whose file has thousands of rules cannot find the fault. The command passes the message on unchanged.

**Witness.** The tokens keep no position:
```rust
// core/src/linear/mist.rs:434-439
enum Token<'a> {
    /// A name, a number or an operator.
    Word(&'a str),
    /// The end of a line.
    Line,
}
```
```rust
// core/src/linear/mist.rs:158-163
    let counter = |name: &str| {
        index
            .get(name)
            .copied()
            .ok_or_else(|| error(format!("the counter `{name}` is not declared")))
    };
```
The probe read a four-line file with an undeclared `c` in the second rule, and a guard written `b = 1`. The messages were:
```text
not a .spec problem: the counter `c` is not declared
not a .spec problem: expected `>=` at `=`
```
`cli/src/io.rs:435` passes `Err(e) => Err(e.into())` on unchanged. qcover's files have up to 213 625 rules (`core-inputs.md:80`). The LLTP reader places its parse errors at their line and column.

**Change.** Record each token's line, for example `Word(&str)` with its offset into `text` (the slice's pointer already gives it). Write the line into every `Error::Spec` message ("line 4: the counter `c` is not declared"), or give the `#[non_exhaustive]` variant a `line` field, which adds to the written form's `details` without a break.

**Value.** A user of `deduxit prove --file x.spec`, or of `mist::read` in a program, learns where the file is wrong, as the LLTP and text readers already say.

**Net.** `mist::tests::reads_problems` asserts three messages word for word and must change with them. The manual shows no `.spec` error (`grep` of `docs/src`). Verdicts and the `read-spec` journey are unaffected. No checker or search is touched.

## F-2c-04

- dimension: text
- criterion: C-FAILURE
- place: core/src/linear/lltp.rs:108
- kind: text
- cost: small
- candidate: no

**Claim.** The `# Errors` section of `lltp::read` says that a role other than `axiom`, `hypothesis` or `conjecture` is refused, while the same doc comment and the code take `lemma` and `theorem` too.

**Witness.** ```rust
// core/src/linear/lltp.rs:90-92
/// - Every clause with the role `axiom`, `hypothesis`, `lemma` or
///   `theorem` is a formula left of `⊢`, in the order of the file. The one clause with the role
///   `conjecture` is the formula right of it.
```
```rust
// core/src/linear/lltp.rs:108-111
/// - [`Error::Lltp`] for text that is not a list of `fof(name, role,
///   formula).` clauses: an annotation after a formula, an empty
///   formula, a role other than `axiom`, `hypothesis` or `conjecture`, no
///   conjecture, or `-o` between two characters of a name;
```
```rust
// core/src/linear/lltp.rs:379
            "axiom" | "hypothesis" | "lemma" | "theorem" => hypotheses.push((range, clausal)),
```
The probe read `fof(l, lemma, a). fof(t, theorem, b). fof(c, conjecture, a * b).` into `Ok("⊢ ~a, ~b, a ⊗ b")`.

**Change.** Rewrite the bullet: "a role other than `axiom`, `hypothesis`, `lemma`, `theorem` or `conjecture`".

**Value.** A user who reads the errors to learn what a file may hold is not told that `lemma` and `theorem` are refused when they are taken.

**Net.** Documentation only. The rustdoc build and the doc test of the module run.

## F-2c-05

- dimension: performance
- criterion: P1
- place: core/src/linear/search/inverse/mod.rs:88
- kind: build
- cost: small
- candidate: no

**Claim.** No instrument measures the inverse method: it runs in no journey of the ratchet or DHAT and in no row of the focused target set, although the default dispatch gives it a turn on two rows. A slowdown or a change in what it visits shows nowhere.

**Witness.** `target/audit/ratchet.out` lists 20 journeys. The search journeys are `search-qbf-20-2`, `search-chain-128`, `search-partition-no-5`, `search-wide-m2-256`, `search-spec-chain`, `search-chain-64-intuitionistic` and `search-additive-14`, and none of them forces or reaches `Engine::Inverse`. `grep -i inverse bench/src/journeys.rs` and `grep -i inverse bench/TARGETS.md` find nothing. The engine's place in the default is recorded:
```text
.claude/rules/core-inverse.md:197-198
- **The dispatch**: the engine takes a turn beside the default engine on
  two rows (`core-search.md`, `Row::probe`);
```
The families' claims build on it, for example `mix` and `contraction`: "the default gives it its turn after 2²⁴ units of work" (`families.rs:533`).

**Change.** Add a journey that runs the inverse method: `mix/12` or `contraction/8#1` under the default, or `wide-m1/256` with `--engine inverse`. Give it a ceiling in `bench/ceilings.csv` and a DHAT section, so that its counts and allocations are guarded like the other engines'.

**Value.** The machine and the next session gain a guard against regressions in the engine the default calls on Mix and on clauses used many times. The candidates F-2c-09 and F-2c-10 can then be measured.

**Net.** Adds a row to the ratchet (a commit of its own, as the ceilings' rule asks). No library code changes.

## F-2c-06

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/horn/reach.rs:684; core/src/linear/search/horn/equation.rs:303; core/src/linear/search/horn/cover.rs:316; core/src/linear/search/horn/mod.rs:241
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The Horn engine merges a transition's two sorted arc lists, inputs and outputs, by place in four hand-written loops, two of them with the same five-arm `match` ladder.

**Witness.** ```rust
// core/src/linear/search/horn/reach.rs:686-705 (moved)
    while i < inputs.len() || o < outputs.len() {
        let (p, taken, given) = match (inputs.get(i), outputs.get(o)) {
            (Some(&(a, w)), Some(&(b, v))) if a == b => {
                (i, o) = (i + 1, o + 1);
                (a, w, v)
            }
            (Some(&(a, w)), Some(&(b, _))) if a < b => {
                i += 1;
                (a, w, 0)
            }
            (Some(&(a, w)), None) => {
                i += 1;
                (a, w, 0)
            }
            (_, Some(&(b, v))) => {
                o += 1;
                (b, 0, v)
            }
            (None, None) => unreachable!("the loop's condition"),
        };
```
```rust
// core/src/linear/search/horn/equation.rs:304-323 (Tableau::new)
            while i < inputs.len() || o < outputs.len() {
                let (p, e) = match (inputs.get(i), outputs.get(o)) {
                    (Some(&(a, w)), Some(&(b, v))) if a == b => {
                        (i, o) = (i + 1, o + 1);
                        (a, i64::from(v) - i64::from(w))
                    }
                    (Some(&(a, w)), Some(&(b, _))) if a < b => {
                        i += 1;
                        (a, -i64::from(w))
                    }
```
`Program::caps` (`mod.rs:245-252`) does the same with `peekable`, and `Cover::before` (`cover.rs:316-347`) merges an element with the inputs and outputs. `reach.rs:526-547` repeats the ladder over the marked places and the outputs, and `mist.rs:630-661` (`Reader::rule`) over guards and changes.

**Change.** Give `Program` one method, `fn arcs(&self, t: &Transition) -> impl Iterator<Item = (u32, u32, u32)>`, that yields each place with the weight taken and the weight given. Write `moved`, the effect in `Tableau::new` and `caps` as folds over it.

**Value.** A reader of the engine checks one merge instead of four, and the state equation's effects and the search's distance provably read the arcs the same way.

**Net.** The Horn rows of the focused target set (Partition, 3-Partition, counter) against `bench/targets/47.csv` with `--counters`, and the `search-spec-chain` ratchet row. `certify`, the only part of the equation that bears on soundness, keeps its own loop.

## F-2c-07

- dimension: idiom
- criterion: E2
- place: core/src/linear/mist.rs:419
- kind: refactor
- cost: small
- candidate: no

**Claim.** `mist::merged` copies its sorted counts into a second vector to merge repeats, where `Vec::dedup_by` does it in place. Its allocations are 16 % of the `read-spec` journey's blocks.

**Witness.** ```rust
// core/src/linear/mist.rs:419-430
fn merged(mut counts: Counts) -> Counts {
    counts.sort_unstable();
    let mut merged: Counts = Vec::with_capacity(counts.len());
    for (x, k) in counts {
        match merged.last_mut() {
            Some((y, m)) if *y == x => *m = (*m).max(k),
            _ => merged.push((x, k)),
        }
    }
    merged.retain(|&(_, k)| k > 0);
    merged
}
```
DHAT `read-spec`: `2001 | 16 % | 32016 | deduxit::linear::mist::merged (mist.rs:421)`. It also appears in `search-spec-chain`, which reads the same file.

**Change.** `counts.sort_unstable(); counts.dedup_by(|later, kept| { let same = later.0 == kept.0; if same { kept.1 = kept.1.max(later.1); } same }); counts.retain(|&(_, k)| k > 0); counts`

**Value.** The std call replaces the hand-written loop, and reading a `.spec` file makes one allocation fewer per rule's guards and per target line (2 001 of 12 440 blocks in the journey).

**Net.** `mist::tests::reads_problems` covers two guards on one counter and a guard of zero. The `read-spec` and `search-spec-chain` ratchet rows give the instruction count. No search or checker is touched.

## F-2c-08

- dimension: performance
- criterion: P2
- place: core/src/linear/search/horn/mod.rs:699; core/src/linear/search/horn/equation.rs:328
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Reading a Horn program clones both arc lists of every clause under `?` only to look them up in a map, and the simplex's set-up keeps an uncharged clone of every distinct effect in a set.

**Witness.** ```rust
// core/src/linear/search/horn/mod.rs:699-704
            match seen.get(&(inputs.clone(), outputs.clone())) {
                Some(&t) => alike[t as usize].push(clause),
                None => {
                    push(Clause::Reusable(clause), &inputs, &outputs);
                    seen.insert((inputs, outputs), alike.len() as u32);
```
```rust
// core/src/linear/search/horn/equation.rs:328
            if !effect.is_empty() && seen.insert(effect.clone()) {
```
The second runs before the charge at `equation.rs:351-354`, so the set's copy of the effects is never charged. DHAT `search-spec-chain`, which reads 2 000 rules under `!`, shows `horn::{impl#4}::read` at 4 015 blocks (7 %). The frames there are inlined, so the clones' own share is not separated.

**Change.** Use `seen.entry((inputs, outputs))`: on `Vacant`, push the transition from `entry.key()` and insert; on `Occupied`, push to `alike`. For the effects, keep indices into `effects` in a set hashed by the slice (or sort the effects and `dedup`), so that each effect exists once and is charged.

**Value.** Two fewer allocations per clause under `!` when a goal is read (every `.spec` rule is one), and the tableau's set-up holds its effects once.

**Net.** Measure with DHAT `search-spec-chain` before and after. The Horn rows of the focused target set must keep their counters (`summary --against bench/targets/47.csv --counters`): transitions keep their order. `certify` is untouched.

## F-2c-09

- dimension: performance
- criterion: P2
- place: core/src/linear/search/inverse/index.rs:77
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Every subsumption query of the inverse method's tries allocates a fresh stack, and `within` allocates another for each node it visits, while the Horn engine's trie of the same kind keeps one stack for all its queries.

**Witness.** ```rust
// core/src/linear/search/inverse/index.rs:76-77
    fn walk(&self, start: u32, zone: &[OccId], mut each: impl FnMut(u32) -> bool) -> bool {
        let mut stack = vec![(start, 0usize)];
```
```rust
// core/src/linear/search/inverse/index.rs:135-139
        self.walk(0, gamma, |node| {
            self.children
                .get(&(node, SEPARATOR))
                .is_some_and(|&split| self.ends_within(split, theta, &mut hit))
        })
```
`subsumed` (`inverse/mod.rs:456-460`) asks both tries for every derived sequent (`keep`) and for every sequent taken up (`run`). The Horn engine's backward search keeps its stack:
```rust
// core/src/linear/search/horn/cover.rs:125-127
    /// The nodes a query has still to visit, each with where its children
    /// start in the marking's places.
    stack: RefCell<Vec<(u32, u32)>>,
```

**Change.** Keep two stacks in `Trie` (`RefCell<Vec<(u32, usize)>>`, one for the outer walk of `within` and one for `ends_within`), cleared at each query, as `Cover::dominated` does.

**Value.** The machine: one to several allocations fewer per derived sequent on the engine's hottest path.

**Net.** No journey runs the engine (F-2c-05): measure with DHAT and callgrind on the journey proposed there. `agrees_with_the_focused_engine_and_the_reference` and `the_engines_sets_pass` keep the verdicts and sets. The kept sequents and the counters do not change.

## F-2c-10

- dimension: performance
- criterion: P2
- place: core/src/linear/search/inverse/mod.rs:538; core/src/linear/search/inverse/mod.rs:865; core/src/linear/search/inverse/mod.rs:468
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** The inverse method's saturation allocates on every derived sequent and every premise choice: it clones a hole list in `fill`, builds a vector in `fits`, and collects and sorts a copy of the zone in `coherent`.

**Witness.** ```rust
// core/src/linear/search/inverse/mod.rs:537-540
        let candidates: Vec<u32> = match self.gamma(id).first() {
            Some(o) => self.goal_index.get(o).cloned().unwrap_or_default(),
            None => (0..self.goal.holes.len() as u32).collect(),
        };
```
```rust
// core/src/linear/search/inverse/mod.rs:864-866 (in the loop over premise choices, under a reading)
        if self.shape.reading.is_some() {
            let mut most: Vec<(u32, u64)> = Vec::new();
            for i in 0..=j {
```
```rust
// core/src/linear/search/inverse/mod.rs:468-469
        let mut all: Vec<OccId> = z.gamma.iter().chain(&z.theta).copied().collect();
        all.sort_unstable();
```
Beside these, `apply` allocates six vectors per application (`mod.rs:730-760`), `candidates` one per hole, `premise` one per candidate (`eval::excess`), and `eval::sum` one per `⊗` step (`eval.rs:71`).

**Change.** Iterate the hole list by index without cloning it (split the borrows of `goal_index` and `filled`). Keep `most` and the coherence buffer as scratch fields of `Saturation`, cleared per use. Merge two sorted slices in `coherent` without sorting, since `gamma` and `theta` are each sorted already.

**Value.** The machine: fewer allocations per derived sequent in the engine the default gives Mix and clauses used many times.

**Net.** Needs the journey of F-2c-05 (DHAT, callgrind) first. The differential test and `the_engines_sets_pass` keep the verdicts, and the kept sequents do not change. No checker is touched; the refutation's check (`saturated.rs`) is separate.

## F-2c-11

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/inverse/eval.rs:47; core/src/linear/proofs/multiset.rs:64
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The inverse method writes its own sorted-multiset operations over `Vec<OccId>` (insert, remove, sum, difference, inclusion), which the crate-private `proofs::multiset::Multiset` already provides to derivations, cut elimination and the sessions.

**Witness.** ```rust
// core/src/linear/search/inverse/eval.rs:47-51
pub(super) fn insert(list: &mut Vec<OccId>, o: OccId) {
    let at = list.partition_point(|&x| x <= o);
    list.insert(at, o);
}
```
```rust
// core/src/linear/proofs/multiset.rs:64-67
    pub(crate) fn insert(&mut self, o: T) {
        let i = self.0.partition_point(|&x| x <= o);
        self.0.insert(i, o);
    }
```
`eval.rs` `remove` (55), `sum` (67), `excess` (98) and `within` (115) match `Multiset::remove` (77), `sum` (110), `difference` (122) and `is_subset` (127). `Multiset::is_subset` allocates the difference where `eval::within` does not. `saturated.rs:495-536` keeps a third copy on purpose (`core-inverse.md:219`: the check "shares no code with `inverse/`"), and the proof checker uses neither (`core-proofs.md:93`).

**Change.** Make `Zone::gamma` a `Multiset` and move `eval::within`'s allocation-free walk into `Multiset::is_subset`, plus an in-place `sum`. Leave `saturated.rs` with its own functions, as its rule asks.

**Value.** A reader sees the linear zone as the same multiset everywhere in the linear suite, and `is_subset` stops allocating for every caller.

**Net.** The differential test `agrees_with_the_focused_engine_and_the_reference`, the `multiset` unit tests, and the derivation and session tests for the faster `is_subset`. The refutation's check stays independent of the engine.

## F-2c-12

- dimension: performance
- criterion: P1
- place: core/src/linear/families.rs:804; core/src/linear/families.rs:954
- kind: refactor
- cost: small
- candidate: no

**Claim.** The family generators write each formula piece as its own `String` and join them, and that input generation is 40 % of the `read-text` journey's DHAT blocks and 51 % of `search-additive-14`'s, so those tables measure the input rather than the library.

**Witness.** ```rust
// core/src/linear/families.rs:804-808
    let pairs: Vec<String> = (0..k)
        .map(|i| format!("(a{0} | b{0})", i / shared))
        .collect();
    let mut roots = vec![pairs.join(" * ")];
    roots.extend((0..k).map(|i| format!("~a{0} * ~b{0}", i / shared)));
```
DHAT (`target/audit/dhat.md`):
```text
read-text:           3096 (20 %) families::wide::{closure_env#0}, 3096 (20 %) {closure_env#1}
search-additive-14:  52090 (31 %) families::additive_tree (families.rs:962), 32758 (19 %) (families.rs:957)
check-wide-m1-2048:  3096 (4 %) + 3096 (4 %) families::wide closures
```

**Change.** Have the generators `write!` into one `String` (`wide`, `mix`, `additive_tree`, `power`), or have the harness generate its input outside what DHAT counts. Either way, the journeys' DHAT tables then show the library's own allocations first.

**Value.** The machine and the next performance session: the DHAT inventory points at reading, searching and checking instead of at test input. Generating a family also gets cheaper for the harness.

**Net.** The generated text must stay byte for byte the same (`verdicts_as_constructed` compares sequents, and the `batch-families` ceiling covers it). No search or checker is touched.

## F-2c-13

- dimension: idiom
- criterion: E6
- place: core/src/linear/search/horn/mod.rs:620; core/src/linear/search/horn/mod.rs:853
- kind: refactor
- cost: small
- candidate: no

**Claim.** `Reader::clause` repeats `clause_head`'s walk over a clause's factors, and the proof builder calls `clause_head` again twice for every firing, on clauses the program has already read.

**Witness.** ```rust
// core/src/linear/search/horn/mod.rs:624-631 (Reader::clause)
        while let Some(x) = factors.pop() {
            match forest.kind(x) {
                Kind::Tensor => factors.extend(forest.children(x)),
                Kind::One => {}
                _ if forest.sign(x) == Some(self.body) => inputs.push(self.place(x)),
                _ if head.is_none() && is_head(forest, self.body, x) => head = Some(x),
                _ => return None,
```
```rust
// core/src/linear/search/horn/mod.rs:856-862 (clause_head)
    while let Some(x) = factors.pop() {
        match forest.kind(x) {
            Kind::Tensor => factors.extend(forest.children(x)),
            Kind::One => {}
            _ if forest.sign(x) == Some(body) => {}
            _ if head.is_none() && is_head(forest, body, x) => head = Some(x),
            _ => return None,
```
`Reader::head_of` (`mod.rs:639-643`) calls `clause_head` again, and `proof.rs:134` and `proof.rs:196` call it per firing.

**Change.** Write `Reader::clause` as `clause_head` plus a pass that collects the body literals' places, or give `clause_head` a callback per body literal. Keep each clause's head occurrence in the `Program` (beside `Clause::Reusable` and in `once`), so that `replay` and `build` read it instead of walking again.

**Value.** A reader of the engine finds one definition of a clause's shape, which `focus::schedule::chains` shares too (`core-horn.md:33-35`).

**Net.** The Horn rows of the focused target set and the engine's tests (`refuses_at_its_limits`, the reference comparison). The proof built is the same node for node, and the checker judges it as before.

## F-2c-14

- dimension: text
- criterion: C-HIDDEN
- place: core/src/linear/lltp.rs:27; core/src/linear/mist.rs:46; core/src/linear/families.rs:45; core/src/linear/families.rs:63
- kind: text
- cost: small
- candidate: no

**Claim.** The three readers' examples import `prove`, `Options` and `Verdict` from `linear::search`, the second path of each, where the crate tells users to write `deduxit::linear::prove`.

**Witness.** ```rust
// core/src/linear/lltp.rs:27
//! use deduxit::linear::search::{Options, prove};
```
```rust
// core/src/linear/families.rs:45
//! use deduxit::linear::search::{Options, Verdict, prove};
```
`target/audit/surface.md:53` and `:92` list `Options` as `linear::Options`, `linear::search::Options` and `prove` as `linear::prove`, `linear::search::prove`. `core.md:113-114` says: "Users write `deduxit::linear::Sequent`, `deduxit::linear::Proof`, `deduxit::linear::prove`". `linear/mod.rs:43-45` re-exports all three.

**Change.** Write `use deduxit::linear::{Mode, Options, prove};` (and `Verdict`) in the four examples.

**Value.** A user who copies an example learns the one path the crate documents, which also survives F-1a-04's reduction of duplicate paths.

**Net.** The doc tests run the examples.

## F-2c-15

- dimension: idiom
- criterion: E6
- place: core/src/linear/search/horn/reach.rs:479; core/src/linear/search/horn/cover.rs:503
- kind: refactor
- cost: small
- candidate: no

**Claim.** Growing a `BinaryHeap` within the memory account is written twice in the Horn engine, beside the `Vec` helper `room`, which lives in `reach.rs` though `cover.rs` and `mod.rs` use it too.

**Witness.** ```rust
// core/src/linear/search/horn/reach.rs:479-491
        let before = self.frontier.capacity() * size_of::<Successor>();
        if self.frontier.capacity() - self.frontier.len() < successors.len() {
            let more = successors.len().max(self.frontier.capacity());
            let fits = more
                .checked_mul(size_of::<Successor>())
                .is_some_and(|bytes| self.charged.account().fits(bytes));
```
```rust
// core/src/linear/search/horn/cover.rs:508-516
        let more = self.queue.capacity().max(16);
        let fits = more
            .checked_mul(size)
            .is_some_and(|bytes| self.charged.account().fits(bytes));
        if !fits {
            return false;
        }
        let before = self.queue.capacity() * size;
        self.queue.reserve_exact(more);
```
`cover.rs:29` imports `use super::reach::room;` and `mod.rs:360` calls `reach::room`.

**Change.** Move `room` to `horn/mod.rs` beside `Parents`, and add `room_in_heap<T: Ord>(heap, more, charged)` there for the frontier and the queue.

**Value.** A reader checks once how the engine's growing buffers are charged, which `core-horn.md` ("the memory bound … counts every buffer that grows") relies on.

**Net.** `refuses_at_its_limits` and the Horn rows of the target set. The charges stay the same in bytes.

## F-2c-16

- dimension: api
- criterion: C-COMMON-TRAITS
- place: core/src/linear/lltp.rs:65; core/src/linear/mist.rs:75; core/src/linear/families.rs:84
- kind: refactor
- cost: small
- candidate: no

**Claim.** `lltp::Problem`, `mist::Problem` and `families::Instance` derive only `Clone` and `Debug`, though all their fields are `Eq`, so a user cannot compare two problems read or generated.

**Witness.** ```rust
// core/src/linear/lltp.rs:65-67
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Problem {
```
```rust
// core/src/linear/families.rs:84-86
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Instance {
```
`Sequent` derives `Clone, Debug, PartialEq, Eq` (`sequents/mod.rs:186`), and `Status` and `Safety` derive `PartialEq, Eq`. `verdicts_as_constructed` (`families.rs:1182-1185`) has to compare `instance.sequent` field by field.

**Change.** Derive `PartialEq, Eq` on the three types.

**Value.** A user's test of their own problem files can `assert_eq!` a whole problem. The change is additive, since no other crate can add these traits.

**Net.** Compiles or not, with no behaviour to keep. `breaks` lists the new impls as additions.

## F-2c-17

- dimension: idiom
- criterion: E6
- place: core/src/linear/search/inverse/rules.rs:768; core/src/linear/search/inverse/rules.rs:818
- kind: refactor
- cost: small
- candidate: no

**Claim.** `compile` and `goal` each build the rule writer with the same eleven-field struct literal.

**Witness.** ```rust
// core/src/linear/search/inverse/rules.rs:768-780 (and 818-830 the same)
    let mut writer = Writer {
        shape,
        rules: Rules::default(),
        reversed: Vec::new(),
        holes: Vec::new(),
        slots: Vec::new(),
        charged,
        held: 0,
        unpolled: 0,
        groups: 0,
        pending: 0,
        inner: 0,
    };
```

**Change.** Add `Writer::new(shape, charged)` and call it in both.

**Value.** A reader of the rules' compiler sees once what a writer starts with, and a field added later is initialised in one place.

**Net.** Behaviour is identical. The engine's tests and the differential test cover it.
