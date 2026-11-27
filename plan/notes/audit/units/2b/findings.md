# Unit 2b: findings

Read in full: 18 files. Not read in full: none.

The finder's remarks: I read all 18 files in full. I also read core-focus.md in full, since nearly all of it is about this unit, and the parts of core.md the unit touches. The unit has no std SipHash map (every table uses crate::hash). Of the shared memory account it uses charge, resize, release, spares, over, limit, used, share and fork, plus Charged and bytes_of; it uses Reason as a plain value; for F-arch-06 that is the interface a root memory module must keep. On F-arch-04, Focused::decide (mod.rs:189) needs an Account, a Work counter and a stop counted in work units, and turns them into its own Stop enum: a public Decide must carry these three or hide them behind one Limits+stop argument. Units 2a, 2c and 4e should look at F-2b-03 (horn/mod.rs:88, inverse/mod.rs:144, search/mod.rs:1060 and interactive/expand.rs:768 reach into focus::counts and focus::bias), and unit 4e at F-2b-01's caller side (cli/src/interact/mod.rs:536, web/src/calls.rs:815). I could not judge the first-order zone's performance claims against a journey: dhat.md and the ratchet have no first-order search journey, and adding one would itself serve P1.

## F-2b-01

- dimension: performance
- criterion: S1
- place: core/src/linear/search/focus/counts.rs:1254
- kind: API break
- cost: medium
- candidate: no

**Claim.** The public `Interactive::split_passes`, documented as a cheap test, builds the whole forest's count rows with no memory account and no stop on every call, so one ⊗ step on a nested tensor takes quadratic memory past the 1 GiB the search itself honours.

**Witness.** ```rust
// core/src/linear/search/focus/counts.rs:1254
    let counts = Counts::new(forest, Bias::Auto);
```
```rust
// core/src/linear/search/focus/counts.rs:335-342
        Self::new_until(
            forest,
            bias,
            Mode::CLASSICAL,
            &Account::new(None),
            &mut |_| false,
        )
        .expect("nothing stops it")
```
```rust
// core/src/linear/proofs/interactive/expand.rs:743
    /// It is a cheap test to run before trying a split; a split that
```
Measured with `target/debug/deduxit interact --file wide.txt`, where wide.txt is `|- c0 * (a1 * (a2 * … a16000)), ~c0` (165 KB), under `systemd-run --scope -p MemoryMax=8G taskset -c 13-15`, peak RSS from `getrusage(RUSAGE_CHILDREN)`: the session alone (`quit`) took 35 796 KiB in 0.1 s; one `apply 0 0 *` (which calls `split_passes` at cli/src/interact/mod.rs:536) took 1 539 156 KiB in 3.6 s. The rows are about n²/2 entries, so 32 000 atoms need about four times that. `web/src/calls.rs:815` calls it per request.

**Change.** Have the session keep the bound it was built `within` (or take `&Limits`), compute the counts with `Counts::new_until` under that account and a stop, and answer a refusal (a `StepError` variant for it, or `Result<bool, Error>`); better, compute the counts once per session, not once per call. Then drop `Counts::new` outside tests.

**Value.** A user of the command's `interact`, of the web page, or of the library's session gets a refusal at the memory bound instead of the process being killed; each ⊗ step stops recomputing the whole forest's rows.

**Net.** `core/src/linear/proofs/interactive/mod.rs`'s split_passes tests and `cli/tests/readme.rs` keep their answers on small sequents; a new test runs the nested tensor under a small memory bound and expects the refusal. No checker, search or certificate is touched; the search's counts are untouched (`new_until` is the same pass).

## F-2b-02

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/focus/keyed.rs:151; core/src/linear/search/focus/mod.rs:1344
- kind: refactor
- cost: small
- candidate: no

**Claim.** The propositional and first-order paths through the memo and the loop check repeat their bookkeeping: the branch-stack push is copied verbatim, and the pop with its depth, the settling of a dependency on the sequent itself and the choice between Complete and Hereditary are each written two or three times.

**Witness.** ```rust
// core/src/linear/search/focus/keyed.rs:151-158
    fn push_stack(&mut self, key: Zones<'_>, hash: u64) {
        if self.stack_len == self.stack.len() {
            self.scratch
                .charge(size_of::<Key>() + size_of::<u64>() + size_of::<usize>());
            self.stack.push(Key::default());
```
```rust
// core/src/linear/search/focus/mod.rs:1344-1348
        if self.problem.rules.stack {
            if self.stack_len == self.stack.len() {
                self.scratch
                    .charge(size_of::<Key>() + size_of::<u64>() + size_of::<usize>());
                self.stack.push(Key::default());
```
The pop and depth: `mod.rs:1374-1379` and `keyed.rs:112-117` (`self.stack_len -= 1; (self.above() + self.stack_len) as u32`). The settling: `mod.rs:1390-1394` and `keyed.rs:125-129` (`if cuts.dependency >= own_depth { NO_DEPENDENCY } else { cuts.dependency }`). `let complete = if hereditary { Failure::Hereditary } else { Failure::Complete };` at `mod.rs:1432`, `mod.rs:1532` and `keyed.rs:134`. duplicates.md lists keyed.rs:151 / mod.rs:1344 (146 tokens). `prove_listed` is 161 lines (clippy.md, too_many_lines).

**Change.** Move `push_stack` to mod.rs and call it from `prove_listed`. Add `Run::pop_stack(&mut self) -> u32` (the own depth), `Cuts::settled_at(self, own_depth) -> Cuts` and `Failure::complete(hereditary: bool)`, and use each at every place.

**Value.** A reader of the loop check finds each rule once. A later change to the stack or the waiting failures, which core-focus.md says must stay alike on both paths, is made in one place. `prove_listed` shrinks toward the lint's bound.

**Net.** The focused target set against bench/targets/47.csv with `--counters` (nodes, splits, memo_hits, memo_entries equal on every decided row), the ratchet's search journeys, and `a_close_settles_by_every_rule`. A search is touched, its behaviour kept; no checker or certificate.

## F-2b-03

- dimension: architecture
- criterion: E5
- place: core/src/linear/search/focus/mod.rs:60
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The count invariants and the atom bias are shared machinery of four engines and the interactive session, yet they live as modules of the focused engine, and the counts module reaches back into the engine for `Switches`.

**Witness.** ```rust
// core/src/linear/search/focus/mod.rs:59-68
/// The atom bias.
pub(crate) mod bias;
...
/// The count invariants the engine prunes with.
pub(crate) mod counts;
```
```rust
// core/src/linear/search/focus/counts.rs:10
use super::Switches;
```
Callers outside the engine: `linear/search/mod.rs:1060` (`focus::counts::refutation(`), `linear/search/horn/mod.rs:88` (`super::focus::counts::refutation(`), `linear/search/inverse/mod.rs:144` (`super::focus::bias::signs(`), `linear/proofs/interactive/expand.rs:768` (`focus::counts::split_passes(`). The consumer test's MLL engine (consumer-test.md, mll.rs) enumerates all 2ⁿ splits with no prune, because the interval check and the count equation are not reachable.

**Change.** Move `counts.rs` (with `Switches`, whose only other reader is the engine) and `bias.rs` to `linear/search/` beside the other shared parts, and keep `focus` for the engine. Separately, ask the author whether a public `linear::Counts` (rows, `Tally::balanced`, `Split::feasible`) should be offered for X2.

**Value.** A reader of the Horn engine, the inverse method or the session finds the counts where shared machinery lives, not inside another engine's private module. The move also prepares the X2 option of letting an outside engine prune as the built-in ones do.

**Net.** The gate (same code, other paths); the focused and net target sets' counters stay equal and the ratchet does not move. No checker, search or certificate changes.

## F-2b-04

- dimension: idiom
- criterion: E7
- place: core/src/linear/search/focus/split.rs:606; core/src/linear/search/focus/split.rs:660; core/src/linear/search/focus/mod.rs:1698; core/src/linear/search/focus/scratch.rs:218
- kind: refactor
- cost: small
- candidate: no

**Claim.** The core crate's only four `#[allow(` attributes are in this unit, without a reason, where the crate's rule is `#[expect(lint, reason = …)]` at the site, and the same file uses that form for the same lint.

**Witness.** ```rust
// core/src/linear/search/focus/split.rs:606
    #[allow(clippy::too_many_arguments)]
    pub(super) fn search_splits(
```
```rust
// core/src/linear/search/focus/split.rs:878-881
    #[expect(
        clippy::too_many_arguments,
        reason = "the split search's state, passed through to the two focuses"
    )]
```
`grep -rn '#\[allow(' core/src` finds 4 sites: split.rs:606, split.rs:660, mod.rs:1698 (`too_many_arguments`) and scratch.rs:218 (`clippy::vec_box`). CLAUDE.md: "a new allowance is an `#[expect(lint, reason = "…")]` at the site, never a crate-wide `allow`".

**Change.** Make each an `#[expect(…, reason = …)]`. The vec_box site takes the reason its field doc gives (the boxed counts keep a split frame a pointer wide). For the three argument lists, pass the split's state as one struct (`Opened` exists already) where that does not grow a recursion frame.

**Value.** A reader sees why each lint is silenced. If a signature later shrinks, the lint says the exception has gone stale, which `allow` never does.

**Net.** The gate's Clippy runs. Attributes alone touch nothing; a regrouped argument list on `search_splits` is on the recursion cycle, so `Limits::PER_LEVEL` is measured again as core-focus.md asks, and the target set's counters stay equal.

## F-2b-05

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/focus/split.rs:607; core/src/linear/search/focus/split.rs:661
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `search_splits` and `search_splits_framed` are the same loop over `next_split` with the same join, and the first-order one adds only the choice log's three calls.

**Witness.** ```rust
// core/src/linear/search/focus/split.rs:637-643
            self.work += (self.problem.forest.len() / OCCURRENCES_PER_LEAF) as u64;
            let joined = match join {
                Join::Tensor(f, a, b) => self.premises(theta, left, right, f, a, b, budget),
                // A Mix needs two parts.
                Join::Mix if right.is_empty() => Ok(Found::NOTHING),
                Join::Mix => self.parts(theta, left, right, budget),
            };
```
```rust
// core/src/linear/search/focus/split.rs:696-701
            self.work += (self.problem.forest.len() / OCCURRENCES_PER_LEAF) as u64;
            let joined = match join {
                Join::Tensor(f, a, b) => self.premises(theta, left, right, f, a, b, budget),
                Join::Mix if right.is_empty() => Ok(Found::NOTHING),
                Join::Mix => self.parts(theta, left, right, budget),
            };
```
duplicates.md lists split.rs:637 / split.rs:696 (176 tokens).

**Change.** Keep one loop and put the log's `enter`, `take` and `fail` behind `if Z::FIRST_ORDER`, as `decompose`, `run` and `prove_unkeyed` already do. Failing that, at least move the join into `fn join_sides(&mut self, …) -> Step`.

**Value.** A reader has one split search to follow. A change to the cut or to the work counting is made once, where today the first-order copy could be missed.

**Net.** The focused target set's counters equal 47.csv, the ratchet's search journeys stay within their ceilings, and pinned time on `Ground` holds, as core-focus.md asks of any change to the generic engine. `search_splits` is on the recursion cycle, so `PER_LEVEL` is measured again. A search is touched, behaviour kept.

## F-2b-06

- dimension: architecture
- criterion: E5
- place: core/src/linear/search/focus/framed.rs:888
- kind: refactor
- cost: medium
- candidate: no

**Claim.** framed.rs (2 119 lines) holds four separable parts: the zone's variables, unification and answers; the keys of first-order sequents; the `Zone` impl; and the closing of a proof.

**Witness.** ```rust
// core/src/linear/search/focus/framed.rs:888-889
// The keys of a first-order search's stable sequents, which its memo and
// loop check read.
```
The parts by line: answers and unification 416-886 (`bind`, `unify_pairs`, `equal`, `fits`, `instance_of`, `passes`, `pattern`); keys 888-1351 (`KeyScratch`, `write_keys`, `write_member`, `name`, `kept`, `apply`, `own_key`, `canonical_key`, the word tags `TAG`…`VALUE`, `KEY_WORDS`); the `Zone` impl 1353-1929; `Closing` 1931-2029; `Members` and `Set` 2031-2119.

**Change.** Make framed a directory: `framed/mod.rs` with `Framed`, its variables and the `Zone` impl; `framed/keys.rs` with the key writing (about 460 lines); `framed/answers.rs` with `Pat`, `Pattern`, `Answers` and the matching walks; `framed/closing.rs` with `Closing`.

**Value.** A reader who works on the keys (the memo of first-order searches) or on the answers can hold that part in mind without the other 1 600 lines.

**Net.** The gate; the first-order families' rows of the target set (chain-fo, witness-fo, lambek-fo) with `--counters`, whose unifications and bindings are functions of the input. Code is moved only; no search, checker or certificate changes.

## F-2b-07

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/focus/memo.rs:238; core/src/linear/search/focus/waiting.rs:309; core/src/linear/search/focus/framed.rs:1878
- kind: refactor
- cost: small
- candidate: no

**Claim.** The bytes of a hash table are estimated in three different ways in this unit, and the first-order zone's way counts fewer than the other two.

**Witness.** ```rust
// core/src/linear/search/focus/memo.rs:242-249
        const GROUP: usize = 16;
        let buckets = match capacity {
            0 => return 0,
            1..4 => 4,
            4..8 => 8,
            _ => (capacity * 8 / 7).next_power_of_two(),
        };
        buckets * (size_of::<(OccSet, u64)>() + 1) + GROUP
```
```rust
// core/src/linear/search/focus/waiting.rs:311-315
        const GROUP: usize = 16;
        let index = match self.index.capacity() {
            0 => 0,
            held => {
                (held * 8).div_ceil(7).next_power_of_two() * (size_of::<(u64, u32)>() + 1) + GROUP
```
```rust
// core/src/linear/search/focus/framed.rs:1878
        let entries = |slots: usize, size: usize| slots * (size + 1);
```
framed.rs:977 (`self.numbers.capacity() * (size_of::<(u32, u32)>() + 1)`) does the same. Outside the unit, `linear/proofs/check/zones.rs:33` is a fourth form.

**Change.** Add one `memory::table_bytes::<K, V>(capacity: usize) -> usize` beside `bytes_of` and use it at all five places, with the bucket rule and the group stated once.

**Value.** A reader of the memory account sees one model of a table's memory. The first-order zone's charge becomes as true as the memo's.

**Net.** For memo.rs and waiting.rs the numbers are equal, so the ratchet and the focused target set's counters do not move (the account's numbers decide when the memo is emptied). framed.rs's charge grows slightly, which can move a first-order row that relieves its memory: run the first-order rows of the target set and say so. `Ground` is untouched. No checker or certificate.

## F-2b-08

- dimension: idiom
- criterion: E3
- place: core/src/linear/search/focus/schedule.rs:324
- kind: refactor
- cost: small
- candidate: no

**Claim.** The threaded schedule's nine items each carry their own `#[cfg(feature = "parallel")]`, placed before the doc comment, where one module behind one attribute would hold them.

**Witness.** ```rust
// core/src/linear/search/focus/schedule.rs:324-328
#[cfg(feature = "parallel")]
/// The work a search does before it gives way to the other, when two
/// alternate on one core. It is about a thousand stable sequents of a
/// small problem, against which a change of threads costs little.
const SLICE: u64 = 1 << 16;
```
The same attribute-before-doc form is at schedule.rs:330 (`POLL`), 335 (`Baton`), 353 (`Turns`), 366 (`impl Baton`), 457, 463 (`StopOnPanic`), 472, 477 (`Ended`), 494 (`alternate`), 593 (`merged`), and memo.rs:800 (`Shared::new`). The Style Guide asks for doc comments before attributes.

**Change.** Move `SLICE`, `POLL`, `Baton`, `Turns`, `StopOnPanic`, `Ended`, `alternate` and `merged` into `schedule/alternate.rs` (or an inline `mod alternate`) behind one `#[cfg(feature = "parallel")]`. Put memo.rs:800's doc comment first.

**Value.** A reader sees at once which part of the schedule exists only with threads, and the docs read first as the Style Guide asks. Twelve attributes become one.

**Net.** Both cargo hack runs (`--each-feature`, `--feature-powerset --depth 2`) and the gate. The code is unchanged, so the counters and the ratchet do not move.

## F-2b-09

- dimension: performance
- criterion: P2
- place: core/src/linear/search/focus/framed.rs:681
- kind: refactor
- cost: small
- candidate: yes

**Claim.** The first-order zone's term walks allocate a new vector and a new hash set on every call, and `passes` runs one per recorded pattern after every unification while a rule's first premise is searched.

**Witness.** ```rust
// core/src/linear/search/focus/framed.rs:680-682
    fn equal(&self, fo: &FirstOrder, a: TRef, b: TRef) -> bool {
        let mut pairs = vec![(a, b)];
        let mut met = HashSet::default();
```
The same at `fits` (716-717), `instance_of` (746-760: `theta`, `walk`, `met`), `pattern` (822-823, and `vec!` per meta at 829) and `open_answers` (1771-1774). `instance_of` calls `fits` or `equal` per pattern variable. `unify` calls `self.passes(fo)` (1758), which runs `instance_of` for every pattern of every open rule (807-816). The zone already keeps scratch for `unify_pairs` (`pairs`, `met`, `walk`, `depths`), but these walks take `&self`.

**Change.** Measure first: a DHAT and callgrind run of a first-order family that opens answers (witness-fo or chain-fo at the target set's sizes). If the allocations show, give the walks scratch buffers they can borrow (a `RefCell` or `&mut self` with the buffers split off, as `name` does with `mem::take`).

**Value.** The machine: fewer allocations per unification on first-order searches with shared metavariables, where the rule of answers runs its patterns over and over.

**Net.** The first-order rows of the focused target set with `--counters` (unifications and bindings included) stay equal. Pinned time of those rows. A search is touched, behaviour kept; `Ground` is untouched.

## F-2b-10

- dimension: performance
- criterion: P7
- place: core/src/linear/search/focus/counts.rs:729
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Nine `#[inline(always)]` sites of the unit rest on a stated reason but no recorded measurement, where core-focus.md records one only for the five on the recursion cycle.

**Witness.** ```rust
// core/src/linear/search/focus/counts.rs:726-730
    /// Returns what [`excluded`](Self::excluded) does. `PLAIN` says that no
    /// atom has copies of one sign, so a loop over a row decides once whether
    /// to read their signs at all.
    #[inline(always)]
    fn excluded_as<const PLAIN: bool>(&self, rank: usize, lo: i32, hi: i32) -> bool {
```
The others: counts.rs:261 (`counted_atom`), 271 (`open_atom`), 811 (`apply_as`), 962 (`Column::excludes`), 977 (`count_bad`), 1026 (`shift_as`), 1101 (`flip_as`), and zone.rs:555 (`Ground::dual_in`). core-focus.md:1015-1019 justifies `free_split`, `open_split`, `next_split`, `holding` and `two_premises` by a debug build's stack per level; clippy.md counts 15 `inline_always` sites in linear::search, all in this unit.

**Change.** Run the ratchet's search journeys under callgrind, and pinned time on the target set's focused rows, with `#[inline]` in place of `#[inline(always)]` at these nine. Keep `always` where the count rises and record the number in core-focus.md; turn the rest into `#[inline]`.

**Value.** The machine and a reader: each forced inlining rests on a number, as P7 asks, and any that does not pay is no longer forced into every caller.

**Net.** The ratchet (instruction counts) and pinned time decide; the target set's counters cannot move, since inlining changes no step. No checker, search or certificate.

## F-2b-11

- dimension: idiom
- criterion: C-CUSTOM-TYPE
- place: core/src/linear/search/focus/choices.rs:99
- kind: refactor
- cost: small
- candidate: no

**Claim.** A choice point that must fail at once is signalled by `start == u32::MAX`, which five callers test by hand, and a Mix's yes or no is logged as a `u32`.

**Witness.** ```rust
// core/src/linear/search/focus/choices.rs:97-99
    /// The first alternative to try. It is past every alternative where
    /// the choice point fails at once.
    pub(super) start: u32,
```
`if choice.start == u32::MAX {` at split.rs:672, split.rs:732, first_order.rs:59 and resumed.rs:28, and `choice.start == u32::MAX || choice.resumed` at first_order.rs:279. The bool: `return Ok(fail == 1);` (split.rs:730) and `settle(&choice, u32::from(fail))` (split.rs:736).

**Change.** Give `Choice` a method `fails_at_once(&self) -> bool` (or make `start` an `Option<u32>`) and use it at the five sites. Log a Mix's answer as a `bool` (`Taken::Settled(bool)`), or name the encoding where it is written.

**Value.** A reader of the first-order rules sees what the test means without the module doc of choices.rs. A sixth choice point cannot get the sentinel wrong.

**Net.** choices.rs's tests and the first-order rows of the target set with `--counters`; behaviour is identical. A search is touched, behaviour kept.

## F-2b-12

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/focus/framed.rs:1275; core/src/linear/search/focus/framed.rs:1311
- kind: refactor
- cost: small
- candidate: no

**Claim.** `own_key` and `canonical_key` write the same two-zone layout in the same double loop and differ only in the sort order and in how a word is mapped.

**Witness.** ```rust
// core/src/linear/search/focus/framed.rs:1285-1292
        for members in [&order[..n], &order[n..]] {
            // A zone's distinct members are fewer than a word counts: each
            // is a member id, a `u32`.
            key.push(OccId::new(members.len() as u32));
            for &i in members {
                let range = s.range(i);
                key.push(OccId::new(s.copies[i]));
                key.push(OccId::new(s.words[range.start]));
```
```rust
// core/src/linear/search/focus/framed.rs:1318-1323
        for members in [&order[..n], &order[n..]] {
            key.push(OccId::new(members.len() as u32));
            for &i in members {
                let range = s.range(i);
                key.push(OccId::new(s.copies[i]));
                key.push(OccId::new(s.words[range.start]));
```
duplicates.md lists framed.rs:1281 / framed.rs:1314 (115 tokens).

**Change.** Write the layout once: `fn write_key(&self, s: &mut KeyScratch, n: usize, order: impl Fn(usize, usize) -> Ordering, word: impl FnMut(&mut KeyScratch, u32) -> u32, level: impl Fn(&KeyScratch, u32) -> u32, key: &mut Vec<OccId>)`, called by the two.

**Value.** A reader of the memo of first-order searches sees one key layout. A change to it, such as a new tag, cannot reach only one of the two keys.

**Net.** `a_deep_witness_is_one_word_of_a_key`, `the_memo_and_the_loop_check_key_first_order_sequents`, and the first-order rows of the target set with `--counters` (memo_hits, memo_entries). A search is touched, behaviour kept.

## F-2b-13

- dimension: idiom
- criterion: E5
- place: core/src/linear/search/focus/counts.rs:356
- kind: refactor
- cost: small
- candidate: no

**Claim.** `Counts::new_until` is one function of 253 lines that makes four passes in a row, each with its own state.

**Witness.** ```rust
// core/src/linear/search/focus/counts.rs:356-362
    pub(crate) fn new_until(
        forest: &Forest,
        bias: Bias,
        mode: Mode,
        account: &Account,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Result<Self, Reason> {
```
clippy.md lists it at 206 lines in too_many_lines. The passes: the atoms below an exponential (404-438), the ranks (439-458), the literals under a `?` and the `linear` test (459-480), and the rows, weights, absorption and forcing per occurrence (481-593).

**Change.** Split it into `exponential_atoms(forest, atom_of, stopped) -> Result<(Vec<[bool; 2]>, bool), Reason>`, `ranks(&exponential) -> (Vec<u32>, Vec<Atom>, Vec<Option<Sign>>)` and `rows(…) -> Result<…, Reason>`, keeping the one `stopped` counter across them.

**Value.** A reader can check each pass against the argument in `Counts`'s doc on its own; the soundness of the one-sign rows rests on the first two.

**Net.** counts.rs's tests, `copies_of_one_sign_bound_the_count_one_way`, and the target set's counters (the rows decide every prune). The polls must fall at the same counts of steps. A search's input is touched, its behaviour kept.

## F-2b-14

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/focus/scratch.rs:22
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The engine's buffer pools are nine hand-written take/give pairs of two shapes, one per kind of buffer.

**Witness.** ```rust
// core/src/linear/search/focus/scratch.rs:104-115
    pub(super) fn take_tally(&mut self) -> Tally {
        match self.pools.tallies.pop() {
            Some(mut tally) => {
                tally.clear();
                tally
            }
            None => {
                self.scratch.charge(self.problem.counts.tally_bytes());
                self.problem.counts.tally()
            }
        }
    }
```
The same shape for `take_set` (22), `take_context` (41) and `take_split` (127). The other shape (`pop().unwrap_or_default()`, `clear`, and `settle` on give) is at `take_list` (77), `take_words` (91), `take_trail` (146) and `take_links` (159). `Pools` has ten `Vec` fields (205-231) and a hand-written `Default` (233-248).

**Change.** A generic `Pool<T>` with `take(&mut self, make: impl FnOnce() -> (T, usize), scratch: &mut Charged) -> T` and `give`, and a small trait for `clear`, so that each kind is one field and one line. The charges must stay as they are.

**Value.** A reader of the allocation rule ("no allocation per node once warm") checks one pool, not nine.

**Net.** The ratchet's search journeys. The focused target set's counters must stay equal: the charges decide when the memo is emptied for memory, which core-focus.md says moves counters. dhat.md's search frames should not grow. A search is touched, behaviour kept.

## F-2b-15

- dimension: idiom
- criterion: E6
- place: core/src/linear/search/focus/mod.rs:1589; core/src/linear/search/focus/mod.rs:754
- kind: refactor
- cost: small
- candidate: no

**Claim.** In mod.rs, `remember` writes its collect-and-rename `match` twice, differing only in one flag, and `statistics` and `report` repeat the first-order counters' addition.

**Witness.** ```rust
// core/src/linear/search/focus/mod.rs:1590-1600
        let entry = match entry {
            Entry::Proved(node) => Entry::Proved(
                self.nodes
                    .collect(Some(node), false)
                    .expect("the root given"),
            ),
            failed => {
                self.nodes.collect(None, false);
                failed
            }
        };
```
The same at 1606-1616 with `true`. `statistics` (754-766) and `report` (772-780) each have `if Z::FIRST_ORDER { let (unifications, bindings) = self.zone.counts(); statistics.unifications += unifications; statistics.bindings += bindings; }`.

**Change.** Add `fn collected(&mut self, entry: Entry, tight: bool) -> Entry` and call it twice. Write `statistics` as `report()` plus the memo's two counters.

**Value.** A reader of the memo's overflow path sees its two attempts as two calls of one step.

**Net.** The gate, `proofs_survive_collections` and the target set's counters. A search is touched, behaviour kept.

## F-2b-16

- dimension: performance
- criterion: P8
- place: core/src/linear/search/focus/mod.rs:1759
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Each stable sequent's focus candidates are sorted with a stable sort whose key, `focus_class`, walks the tensor's children and their forcing on every comparison, although the keys are unique and an unstable sort gives the same order.

**Witness.** ```rust
// core/src/linear/search/focus/mod.rs:1756-1759
        self.one_of_each(candidates);
        // Forced splits first, then `⊕`, then the free splits; by id within
        // a class, so that the run is deterministic.
        candidates.sort_by_key(|&o| (self.focus_class(o), o));
```
```rust
// core/src/linear/search/focus/mod.rs:2079-2085
            Kind::Tensor => {
                let forced = self
                    .problem
                    .forest
                    .children(self.occ(o))
                    .any(|c| self.forced_side(c).is_some());
                if forced { 1 } else { 3 }
```
The key holds `o`, and `one_of_each` has just removed repeats, so no two keys are equal. The copies were moved off a stable `sort_by_key` for this reason (core-focus.md:1043-1047).

**Change.** Compute the class once per candidate (pairs, or a partition into the four classes in one pass) and sort with `sort_unstable`.

**Value.** The machine: fewer key computations per stable sequent where Γ holds many tensors (a Petri net's clause bodies used once, a 3-Partition).

**Net.** The order is identical, so the target set's counters stay equal. Measure the ratchet's search journeys (search-partition-no-5, search-qbf-20-2) under callgrind and keep the change only if they fall. A search is touched, behaviour kept.
