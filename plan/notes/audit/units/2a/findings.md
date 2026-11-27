# Unit 2a: findings

Read in full: 6 files. Not read in full: none.

The finder's remarks: I agree with F-arch-19's seams for `search/mod.rs`, with two additions: the test module (lines 3845 to 5552, a third of the file) is a seam of its own (`search/tests.rs`), and `the_documented_dispatch_is_the_dispatch` reads `include_str!("mod.rs")` to find `Engine`'s table, so moving `Engine` must move that path too. I agree with F-arch-01 and F-arch-02; from inside, `Reason::as_set` reads the linear `Options::copy_bound`, so it stays a crate-private function of `linear::search` when `Reason` moves to the root. The unit has no SipHash map (the additive memo uses `crate::hash::HashMap`), and `Answer` is 1 216 bytes with `ProofStructure` inline, which I did not raise since it moves a few times per search, not in a loop. `engine_for` has no caller outside `core/src`, and `core-search.md:403-407`, which says the command calls it, is stale (the command calls `race`). Unit 2b should look at the engine-side `as_set` calls of F-2a-06 in `focus/` and `inverse/` and at `focus/mod.rs:178`, where each `admits` names its own engine (F-2a-02). Unit 7a should check `cli/src/prove.rs:66-85` against F-2a-04, and the leader should note that F-2a-01 changes behaviour (an error becomes a documented verdict), so it is a fix, not a refactor.

## F-2a-01

- dimension: idiom
- criterion: E6
- place: core/src/linear/search/mod.rs:387; core/src/linear/search/mod.rs:1150
- kind: API break
- cost: small
- candidate: no

**Claim.** The check of a proof is mapped to a verdict in two places that disagree: `conclude` turns a refused check into `Verdict::Unknown`, while `on_own_forest` (a goal of a forest with cut formulas) returns it as `Err(Error::Check)`, against `prove`'s documented contract.

**Witness.** `on_own_forest`, after the search on the cut-free forest:

```rust
    if outcome.checked {
        match carried.check_within(mode, limits, &mut *stop) {
            Ok(()) => {}
            Err(e @ CheckError::Invalid(_)) => return Err(Error::Rejected(Box::new(e))),
            Err(e @ CheckError::Refused(_)) => {
                return Err(Error::Check(e));
            }
        }
    }
```

`conclude`, for every other proof:

```rust
    match proof.check_within(mode, &left, stop) {
        Ok(()) => Ok(verdict),
        Err(e @ CheckError::Invalid(_)) => Err(Error::Rejected(Box::new(e))),
        Err(CheckError::Refused(refused)) => {
            Ok(Verdict::Unknown(unchecked(refused.refusal, limits)?))
        }
    }
```

`prove`'s docs (`mod.rs:200`): "A check of the proof that a bound or the stop gives up is no error. The verdict is then [`Verdict::Unknown`]", and `core-search.md:44-49` says the same "never an error". A sequent with a cut whose second check meets the caller's stop (the command's time limit fires in `Phase::Check`) or the memory or work bound returns an error, with the statistics lost.

**Change.** Route the carried proof's check through the same mapping as `conclude` (a shared `fn checked_verdict(proof, mode, limits, stop) -> Result<Verdict, Error>` used by both), so a refused re-check yields `Verdict::Unknown(unchecked(..))` with the outcome's statistics kept.

**Value.** A library caller (and the command) gets the documented `Unknown(Stopped|WorkLimit|Unchecked)` verdict for a sequent with cut formulas, as for any other sequent, instead of an error it has to sort itself; a reader finds the check-to-verdict rule once.

**Net.** A new test in `mod.rs` (a sequent with a cut, a stop that fires at `Phase::Check`) and the behaviour lock; the checker and the searches are untouched, no target-set counter moves (it runs after the search).

## F-2a-02

- dimension: architecture
- criterion: X4
- place: core/src/linear/search/mod.rs:1477; core/src/linear/search/mod.rs:2253; core/src/linear/search/focus/mod.rs:178
- kind: refactor
- cost: medium
- candidate: no

**Claim.** Judged from inside the front door, the public engine interface of F-arch-04 can be smaller than proposed and needs three things the proposal leaves out: the stop as units (the front door already owns the work count and `Progress`), `admits` answering a `NotTaken` instead of an `Error`, and a name and counters for an outside engine, which `Engine::External` and the private `Counter::new` cannot carry.

**Witness.** The private trait already hands engines a stop of units, and `poll` turns it into `Progress` and the work bound (`mod.rs:924-931`):

```rust
fn poll(
    mine: &mut Counted<'_>,
    stop: &mut impl FnMut(Progress) -> bool,
    units: u64,
) -> (bool, u64) {
    let done = mine.add(units);
    let stop = stop(Progress::new(Phase::Search, units, done));
```

(quoted from `:928-930`: `let done = mine.add(units);` / `let stopped = stop(Progress::new(Phase::Search, units, done));` / `(stopped || mine.passed(), done)`).

Each of six `admits` builds the error with its own engine, which the front door already knows (`focus/mod.rs:178-184`):

```rust
        let engine = if self.two_sided {
            super::Engine::TwoSided
        } else {
            super::Engine::Focus
        };
        Err(Error::EngineRefused { engine, because })
```

(the same in `additive.rs:58`, `horn/mod.rs:62`, `net.rs:59`, `essential.rs:89`, `inverse/mod.rs:80`). A counter list cannot be made outside: `const fn new(key: &'static str, label: &'static str, meaning: &'static str) -> Self {` (`mod.rs:2253`, private), and `Engine::counters` returns `&'static [Counter]` per variant, so a fieldless `Engine::External` writes one name and no counters for every outside engine. `Answer::of_arena` panics on a malformed arena (`mod.rs:1456`), so an outside proof must come as a `Proof` made by `Proof::new`.

**Change.** Public `trait Decide { fn name(&self) -> &'static str; fn counters(&self) -> &'static [Counter]; fn admits(&self, goal: Goal<'_>, mode: Mode) -> Result<(), NotTaken>; fn decide(&self, goal: Goal<'_>, mode: Mode, options: &Options, limits: &Limits, stop: &mut dyn FnMut(u64) -> bool) -> Result<Found, Error>; }` with `Found { Proved(Proof), Unprovable(Option<Refutation>), Unknown(Reason) }`, run by `prove_with` through `poll` and `conclude`; `Counter::new` and `affine` public; `Engine::External(&'static str)` (written by name, read back as `Unknown`); the private `admits` returns `NotTaken` and `prepare` wraps it with the engine. Memory an outside engine holds is not counted (as the net engine's structures are not), said on the trait.

**Value.** A researcher's engine reports work in its own unit and gets the work bound, the caller's `Progress` and the refuters for free, with no need for `Progress` or `Refusal` constructors on this path; its outcome names it and lists its counters as the command and the bindings print them.

**Net.** Additive for the public part; the built-in engines' loops do not change, so the target sets' counters and the ratchet's search journeys show nothing moved; the checker stays the trust boundary through `conclude`.

## F-2a-03

- dimension: api
- criterion: X1
- place: core/src/linear/search/mod.rs:407; core/src/linear/search/mod.rs:460
- kind: refactor
- cost: small
- candidate: no

**Claim.** A `Goal` hides its members: its fields and `occurrences()` are private, so whoever is handed a `Goal` (an outside engine under F-arch-04's interface, a front end) cannot read what it must prove.

**Witness.** ```rust
pub struct Goal<'a> {
    /// The forest.
    forest: &'a Forest,
    /// The members, as given; empty for the conclusion.
    members: &'a [Member],
    /// Whether the members are the roots.
    conclusion: bool,
}
```

The public methods are `conclusion`, `new`, `is_conclusion` and `forest`; the reader of the members is private:

```rust
    fn occurrences(&self) -> std::borrow::Cow<'a, [OccId]> {
```

The architecture reader's proposal (`architecture.md`, section 5, part 3) passes `goal: Goal<'_>` to `Decide::admits` and `decide`.

**Change.** Make `occurrences()` public (the roots in the forest's order for the conclusion, the members as given otherwise), with a doc example; keep the fields private.

**Value.** An engine or front end handed a goal reads its occurrences through the public API; the proposed engine interface becomes usable.

**Net.** Additive; `unreachable_pub` and the doc test; no search or checker is touched.

## F-2a-04

- dimension: api
- criterion: C-BUILDER
- place: core/src/linear/search/mod.rs:560
- kind: API break
- cost: small
- candidate: no

**Claim.** `race` takes a thread count beside the options and ignores `options.jobs` entirely, so the one search that the command and the harness run by default has two thread knobs, and both callers write the same branch around it.

**Witness.** ```rust
pub fn race(
    goal: Goal<'_>,
    mode: Mode,
    options: &Options,
    limits: &Limits,
    threads: usize,
    mut add_pool: impl FnMut(Progress) -> bool,
    stop: impl Fn(Progress) -> bool + Sync,
) -> Result<Outcome, Error> {
```

It overwrites `jobs` on every path (`:592`, `:601-602`):

```rust
        let options = options.clone().with_jobs(threads);
```

```rust
        options.clone().with_jobs(1),
        options.clone().with_jobs(threads - 1),
```

The two callers each branch on it: `cli/src/prove.rs:66-85` (`None => prove_goal(goal, mode, &options.clone().with_jobs(threads.jobs), …)` / `Some(alone) => … race(goal, mode, options, limits, threads.jobs, …)`) and `bench/src/run.rs:1113-1126` (`prove_within(…)` else `race(…, args.jobs, …)`).

**Change.** Drop the `threads` parameter: `race` reads `options.threads()` as the threads in all, the calling one counted, as `prove_goal` reads `jobs`.

**Value.** A library user sets the threads in one place; `race(…, &options.with_jobs(8), …, 2, …)` no longer runs on two threads silently; the command's `searched` and the harness's `decide` lose an argument.

**Net.** The race's tests in `mod.rs` and the behaviour lock; no engine changes; the target sets run one thread and do not pass through `race`.

## F-2a-05

- dimension: api
- criterion: C-STRUCT-PRIVATE
- place: core/src/linear/search/mod.rs:3559; core/src/linear/search/mod.rs:3566; core/src/linear/search/mod.rs:3576
- kind: API break
- cost: small
- candidate: no

**Claim.** Three struct variants of the marked enum `Reason` lack `#[non_exhaustive]`, which every other struct variant of it has and the crate's decision requires, so adding a field to them later is a break.

**Witness.** ```rust
    #[non_exhaustive]
    Instances {
        /// The bound on instances.
        instances: u32,
    },
    /// A search that may miss proofs ended without one, so the sequent may
    /// still hold. An ordinary first-order sequent whose image the linear
    /// search finds unprovable is unknown for this reason: the embedding
    /// is not shown complete at first order (the classical image has no
    /// contraction).
    Incomplete,
    /// The connection prover reached its bound on the active path
    /// ([`Options::path_length`]) without a proof.
    PathLength {
        /// The bound.
        length: u32,
    },
```

`ModelSize {` (`:3566`) and `KripkeSize {` (`:3576`) likewise. `core.md` (Decisions): "every other public enum … is `#[non_exhaustive]`, and so is every struct-like variant of a marked enum"; `Reason` is in the marked list.

**Change.** Add `#[non_exhaustive]` to `PathLength`, `ModelSize` and `KripkeSize` in 0.9.0, where a break is allowed; if outside engines must build reasons (F-2a-02), add constructors for all variants at once.

**Value.** A later field on these reasons (a bound on terms, say) is no break for downstream matches; the enum follows the rule its siblings follow.

**Net.** `breaks 0.8.0` lists the three variants; `cargo check` of `ordinary` (which builds them inside the crate) and the behaviour lock; nothing runs differently.

## F-2a-06

- dimension: idiom
- criterion: E6
- place: core/src/linear/search/additive.rs:115; core/src/linear/search/mod.rs:1137
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `Reason::as_set` is applied by the engines and again by `conclude`, which applies it to every engine's unknown; the engine-side calls are leftovers of one rule written seven times.

**Witness.** `conclude`, the one place an answer becomes a verdict:

```rust
            // The option's bound, whatever share of it the engine's
            // account had (a race's part leaves out the input, and a
            // share of nothing is zero): every engine, the Horn engine's
            // too, which named its account's.
            None => Verdict::Unknown(reason.as_set(options, limits)),
```

and before it, in the engines:

```rust
    let result = engine.pair(*x, *y).map_err(|r| r.as_set(options, limits));
```

(`additive.rs:115`), also `focus/mod.rs:238` and `:256`, `focus/parallel.rs:98`, `focus/schedule.rs:120` and `:630`, `inverse/mod.rs:98`. `as_set` is idempotent (`CopyBound` to `options.copy_bound()`, `MemoryLimit` to `limits.memory_bytes.unwrap_or(bytes)`), and every engine's `Err(reason)` reaches `conclude` through `Answered::concluded`; between them only the variant is matched (`handed_over`, `Turns::take`).

**Change.** Remove the seven engine-side `as_set` calls and keep the one in `conclude`, as `core-search.md:851` already describes the rule.

**Value.** A reader finds the rule that a reason names the option's bound in one place; a new engine need not remember it.

**Net.** The behaviour lock and `the_memory_limit_named_is_the_one_given` (cli tests); the target sets' counters (no search step changes, only an error value's field, which `conclude` sets anyway).

## F-2a-07

- dimension: idiom
- criterion: E6
- place: core/src/linear/search/mod.rs:788; core/src/linear/search/mod.rs:1704; core/src/linear/search/mod.rs:1532
- kind: refactor
- cost: small
- candidate: no

**Claim.** `Turns::asked` and the `probes` call inside `Turns::take` are dead: `take` is reached only from the branch that already found `probes(&task, options)` true for the same task and options, and the dispatch table is scanned three times for the one row.

**Witness.** `answer_goal` (`:831-842`):

```rust
    let mut answer = if probes(&task, options) {
```

```rust
            let (ended, next) = turns.borrow_mut().take(
```

`Turns::take` (`:1704-1709`):

```rust
        if !self.asked {
            self.asked = true;
            if !probes(task, options) {
                return (false, u64::MAX);
            }
        }
```

Its doc (`:1673`) still says "On the first turn it asks whether the row gives turns". `probes` repeats `DISPATCH.iter().find(|row| row.takes(task))`, which `dispatch` (`:1515`) already ran in `prepare`. `turns` is a `RefCell` (`:788`) only because the closure borrows it; the closure is a temporary of one call, so a `&mut` would do.

**Change.** Let `dispatch` return the row; keep `row.probe` in what `prepare` returns, compute `probes` once from it, drop `asked`, the second `probes` and the `RefCell`, and correct `take`'s doc.

**Value.** A reader of the turns sees one decision where today three look alike; the poll closure loses a borrow flag.

**Net.** The ratchet's search journeys (the poll closure's code changes; `core-search.md:631-638` records that a comparison per poll moved `search-qbf-20-2`), `the_inverse_method_takes_its_turn`, and the target sets' counters, which must not move.

## F-2a-08

- dimension: performance
- criterion: P1
- place: core/src/linear/search/mod.rs:591; core/src/linear/search/mod.rs:771
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** `race`, the command's default search, runs the set-up passes twice before any engine polls (three times once the pool starts): `engine_for` computes the fragment, the intuitionistic reading and the dispatch (with a Horn program's reading), and `answer_goal` computes them again.

**Witness.** ```rust
    if threads < 3 || mode.is_ordered() || !engine_for(goal, mode, options)?.parallel() {
```

`engine_for` (`:1243-1253`) is `goal_fragment`, `read(goal.forest, mode)`, `prepare(…)`; then `answer_goal` (`:771-782`):

```rust
    let fragment = fragment_of(forest, &occurrences, options)?;
    let reading = read(forest, mode)?;
```

and `prepare(goal, &occurrences, mode, fragment, options, reading.as_ref())`, each with a fresh `Task` and its own `OnceLock` for the Horn program. `Reading::of_mode` is a pass with a byte per occurrence; `core-search.md:716-728` measures the set-up passes on `SYJ212+1.020` (27.8 million occurrences) at about 0.1 s each and none of these is polled before `set_up_stopped`.

**Change.** Have `race` compute the fragment, reading and engine once and hand them to `answer_goal` for the single side (the pool's side may share the same `Task`, which is `Sync`), or decide `parallel()` from the dispatch row without a second reading.

**Value.** The machine: one set-up pass fewer per command run on every sequent (two fewer with the pool), and an earlier first poll on large inputs.

**Net.** Measure first: pinned time to the first `Progress` of `deduxit prove --jobs 4` on `SYJ212+1.020` in its cbv translation, before and after, or a callgrind journey through `race` (none exists); the target sets' counters must not move.

## F-2a-09

- dimension: text
- criterion: X3
- place: core/src/linear/search/mod.rs:3432; core/src/linear/search/mod.rs:1058
- kind: text
- cost: small
- candidate: no

**Claim.** `Verdict` does not say what its values promise: a `Proved` proof has passed the checker only when the outcome's `checked` says so, and an `Unprovable` refutation from the counts or the classical refuter is computed and not checked by `Disproof::check` before it is returned (only a saturated set or a basis is).

**Witness.** ```rust
pub enum Verdict {
    /// The sequent is provable, with a proof. It is boxed because a proof
    /// carries its forest.
    Proved(Box<Proof>),
    /// The sequent is not provable, with what can be said of why. The
    /// search was exhaustive, or a refuter found a refutation. It is boxed
    /// as a proof is.
    Unprovable(Box<Disproof>),
```

`conclude` checks only the refutations of last resort (`:1076-1080`: "The set or the basis is checked as a proof is, by code apart from the engine that found it"); the counts' refutation (`focus::counts::refutation`, `:1058-1068`) and the classical one (`:1069-1073`) go into the disproof unchecked. `Disproof`'s docs (`refutation.rs:862`) say `check` "verifies a disproof read from elsewhere". Consumer test, point 5; handovers.md asks this of `mod.rs:3411`.

**Change.** Say on `Verdict::Proved` that the proof has passed the checker when a search of this crate returned it and `Outcome::checked` is true, and that a value built or read elsewhere has not until `Proof::check`; on `Unprovable`, that the verdict is the search's and that its refutation is checked by `Disproof::check`, which the search runs itself only for `Saturated` and `Basis`.

**Value.** A user who passes verdicts between programs, or builds one (`Disproof::new` is public), knows which call is the trust boundary.

**Net.** Docs only; the doc tests.

## F-2a-10

- dimension: architecture
- criterion: E6
- place: core/src/linear/search/mod.rs:924; core/src/linear/search/mod.rs:1720; core/src/linear/search/mod.rs:1082
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The front door writes the same meter three times (count units, tell the caller a `Progress`, test the work bound), the shape F-arch-03 proposes as a public `limits::Meter`.

**Witness.** `poll` (`:929-931`):

```rust
    let done = mine.add(units);
    let stopped = stop(Progress::new(Phase::Search, units, done));
    (stopped || mine.passed(), done)
```

`Turns::take`'s stop (`:1721-1724`):

```rust
                turn = turn.saturating_add(units);
                let done = mine.add(units);
                ended = stop(Progress::new(Phase::Search, units, done)) || mine.passed();
```

`conclude`'s check of a set or a basis (`:1082-1086`):

```rust
                        let mut polled = |units: u64| {
                            done = done.saturating_add(units);
                            stop(Progress::new(Phase::Check, units, done))
                                || limits.work.is_some_and(|limit| done > limit)
                        };
```

and after it the same test again to choose the reason (`:1108-1111`).

**Change.** One meter type over a `Counted` (the search's shared count) or a local count, with `tick(units) -> bool` and `reason()`; `poll`, the turn and the check use it, and the public `Meter` of F-arch-03 is the same type.

**Value.** A reader sees one rule for polls and the work bound; an outside engine gets the meter the built-in ones use.

**Net.** `the_search_counts_its_work`, `a_turn_tells_the_caller_every_poll`, `the_work_bound_holds_beside_the_second_search`; the ratchet's search journeys, since `poll` is on every poll; the target sets' counters.

## F-2a-11

- dimension: idiom
- criterion: E6
- place: core/src/linear/search/mod.rs:696; core/src/linear/search/mod.rs:985; core/src/linear/search/mod.rs:1028
- kind: refactor
- cost: small
- candidate: no

**Claim.** The goal's description travels through three private structs with the same fields (`Task`, `Answered`, `Concluding`), and `conclude` takes eight arguments, two of them the parts of an `Answer` and one the goal's members that its `Goal` already has.

**Witness.** ```rust
struct Answered<'a> {
    /// The engine's answer, with the counters.
    answer: Answer,
    /// The goal's occurrences, as the engine was handed them.
    occurrences: std::borrow::Cow<'a, [OccId]>,
    /// The fragment searched in.
    fragment: Fragment,
    /// The mode.
    mode: Mode,
    /// The engine that ran.
    engine: Engine,
}
```

```rust
struct Concluding<'a> {
    /// The forest.
    forest: &'a Forest,
    /// The goal's occurrences, as the engine was handed them.
    occurrences: &'a [OccId],
    /// The fragment searched in.
    fragment: Fragment,
    /// The mode.
    mode: Mode,
    /// Whether the goal is the sequent's roots.
    roots: bool,
}
```

```rust
fn conclude(
    result: Result<Option<Proof>, Reason>,
    refutation: Option<Refutation>,
    goal: Concluding<'_>,
    members: &[Member],
```

with `#[expect(clippy::too_many_arguments, …)]`; inside, the refutation kept for last is an `Option<Refutation>` that needs `_ => unreachable!("only a set or a basis is kept for last")` (`:1103`).

**Change.** Give `Answered` the `Goal` (forest, members, roots) and let `conclude` take `(&Answered-like goal, Answer, options, limits, account, stop)`; keep the last-resort refutation as a private two-variant enum so the match is total.

**Value.** A reader of the verdict's making follows one value instead of three; the `unreachable!` and the lint's exception go.

**Net.** `a_rejected_proof_is_an_error` and the front door's tests; the target sets' counters (the search is untouched).

## F-2a-12

- dimension: api
- criterion: D1
- place: core/src/linear/search/batch.rs:80; core/src/linear/search/batch.rs:181
- kind: API break
- cost: medium
- candidate: no

**Claim.** The batch is the one entry of the linear search with a default logic: `batch::Options::mode` defaults to classical and every `Problem` without a mode takes it, where `prove`, `prove_within`, `prove_goal` and `race` all require the mode.

**Witness.** ```rust
    /// The mode of a problem that names none.
    pub mode: Mode,
```

```rust
    fn default() -> Self {
        Self {
            mode: Mode::CLASSICAL,
```

```rust
            let mode = problem.mode.unwrap_or(mode);
```

(`batch.rs:265`). `Settings` carries it as `batch.mode` (`core/src/settings.rs:141`), which the command fills from its mode flags (`cli/src/settings.rs:250`).

**Change.** Make the mode a required part of a problem (`Problem::new(name, sequent, mode)`) and drop `batch::Options::mode`, or make it `Option<Mode>` with `None` the default and a problem without a mode an error; the command's required logic (D1) fills it.

**Value.** A library user of the batch names the logic as every other linear entry requires; the command's D1 change has no silent classical default left beneath it.

**Net.** The batch's tests, the behaviour lock and the manual's batch chapter (`docs/src/batch.md:92`, `docs/src/settings.md:51`), the `wire-schema` check for `Settings`; no search changes.

## F-2a-13

- dimension: api
- criterion: C-VALIDATE
- place: core/src/linear/search/mod.rs:2643; core/src/linear/search/mod.rs:1331
- kind: API break
- cost: medium
- candidate: no

**Claim.** `Engine::Unknown`, which only describes an outcome read from a later version, is a value `Options::engine` accepts and the front door refuses at run time, and writing such an outcome back loses the engine's name.

**Witness.** ```rust
            Engine::Unknown => unreachable!("the front door refuses an engine it does not know"),
```

```rust
    let engine = match options.engine {
        Some(Engine::Unknown) => {
            return Err(Error::UnknownName {
                what: "engine",
                name: Engine::Unknown.name().into(),
                known: Engine::NAMES,
            });
        }
```

The reader (`serialize/search.rs:851`): `Ok(Self(name.parse().unwrap_or(Engine::Unknown)))`, and `name()` writes `"unknown"`. `core-search.md:381-388` gives the reason for the variant: an unknown name in an outcome is read, never refused, so a new engine needs no new wire level. That reason is about reading an outcome; it does not need the options to accept the value.

**Change.** Keep `Engine` for the engines that run (the options' type), and give `Outcome::engine` a type that also holds an engine named in a document, with its name (`enum Ran { Engine(Engine), Named(String) }` or the `External(&'static str)` of F-2a-02 plus a read-back form); `implementation` then needs no `unreachable!`.

**Value.** A user cannot force an engine that does not exist, the type says so; a workspace or tool that reads and re-writes outcomes keeps the engine's name.

**Net.** `names_are_listed`, the `wire-schema` check and the serialize tests (the JSON form stays a name); no search changes.

## F-2a-14

- dimension: text
- criterion: T3
- place: core/src/linear/search/mod.rs:3470
- kind: text
- cost: small
- candidate: no

**Claim.** `Reason`'s reference of its JSON form lists eleven of its thirteen kinds and misses a separator, while `Reason::NAMES` has all thirteen.

**Witness.** ```rust
/// In JSON (feature `serialize`), a reason is tagged by `kind`:
/// `{"kind": "stopped"}`, `{"kind": "recursion_limit", "depth": 2048}`,
/// `{"kind": "copy_bound", "copies": 3}`, `{"kind": "memory_limit",
/// "limit_bytes": 1073741824}`, `{"kind": "index_limit"}`, `{"kind":
/// "work_limit", "limit": 1000000}`, `{"kind": "unchecked",
/// "limit_bytes": 1073741824}`, `{"kind": "witness_depth", "depth": 64}`
/// `{"kind": "instances", "instances": 65536}`, `{"kind": "incomplete"}`
/// or `{"kind": "path_length", "length": 8}`.
```

`model_size` (`{"kind": "model_size", "size": 6}`) and `kripke_size` (`{"kind": "kripke_size", "worlds": 4, "size": 6}`) are missing; a comma is missing after the `witness_depth` example.

**Change.** List every kind, one per line as a list, in `NAMES`' order, with `model_size` and `kripke_size`.

**Value.** A reader of outcomes in JSON (a script, the web client) finds every reason a search of any suite gives.

**Net.** Docs only; the `wire-schema` check already validates the forms.
