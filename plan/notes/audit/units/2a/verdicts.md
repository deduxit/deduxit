# Unit 2a: verdicts

13 confirmed, 1 candidate.

## F-2a-01: confirmed

Opened core/src/linear/search/mod.rs:328-398 and :1140-1156, :1195-1211. `on_own_forest` re-checks the carried proof with `check_within(mode, limits, stop)` and returns `Err(Error::Check(e))` for `CheckError::Refused`, while `conclude` maps the same refusal through `unchecked` to `Verdict::Unknown`. The docs of `prove` (:200-204) and core-search.md (the paragraph on `conclude`, 'never an error, since a refusal is no fault of the input') promise the verdict. core-search.md:763-776 records `on_own_forest` and says only that `Error::Rejected` would be a defect of the shift; it records no choice of an error for a refused second check. Both `prove_goal` (:520) and `race` (:573) go through it. No test in the search module names `Error::Check`, so nothing locks the present behaviour.

Correction: Kind: a fix of behaviour against the documented contract (an error becomes a verdict); it removes nothing from the API, so 'API break' overstates it, though a caller that matched `Error::Check` here sees the change.

## F-2a-02: confirmed

The witnesses hold: the private trait at mod.rs:1477-1506 polls with units, `poll` (:924-932) makes the `Progress` and tests the work bound, `Counter::new` (:2253) is private, each of the six `admits` builds `Error::EngineRefused` with its own engine and nothing else (focus/mod.rs:167-184, additive.rs:58, horn/mod.rs:62, net.rs:59-78, essential.rs:89-104, inverse/mod.rs:80), and `Answer::of_arena` panics on a malformed arena (:1456). The proposal refines F-arch-04, which its verifier confirmed. Points the finder did not weigh: `Engine` is fieldless and `Copy` today, and core.md ('The break against the last release') notes that a downstream `as` cast of a fieldless enum compiles, so `Engine::External(&'static str)` is a break and not additive; an outside engine's counters must be keys of `Statistics` (`counters_are_fields`, :3924), so it can relabel the existing fields but not add one; and F-2a-13 proposes another type for the same field, so the two must be decided as one design.

Correction: Kind: API break (a data-carrying variant on the fieldless `Engine`, a public trait and public constructors), not refactor. To be decided together with F-2a-13 and F-arch-04.

## F-2a-03: confirmed

mod.rs:407-467: `Goal`'s fields are private, the public methods are `conclusion`, `new`, `is_conclusion` and `forest`, and `occurrences` (:460) is private. For the conclusion a holder can read `goal.forest().roots()`, but for any other goal there is no public reader of the members, so the claim holds for exactly that case. Making the method public is additive. core.md's 'An item without a caller waits for one' applies: its caller is the engine interface of F-arch-04, so it should land with it.

Correction: Criterion: X4 (or X2) fits better than X1, which is about formulas and sequents. Kind: an additive API change, behaviour kept.

## F-2a-04: confirmed

mod.rs:560-603: `race` takes `threads: usize` and overwrites `options.jobs` on every path (`with_jobs(threads)` at :592, `with_jobs(1)` and `with_jobs(threads - 1)` at :601-602); its docs (:531-544) speak only of `threads` and do not say that `Options::jobs` is ignored. No rules file records a reason for the separate parameter (grep of core-search.md and core-parallel.md for `race`). The two callers are cli/src/prove.rs:76 and bench/src/run.rs:1119. One part of the value is overstated: the branch both callers write chooses between racing and not racing (`threads.alone`, `args.pool_after`), so it stays; each caller loses one argument and the command's `searched` gains a `with_jobs` on the race path. `Jobs::Auto` then also reaches `race`, which must resolve it as `Options::threads` does.

Correction: Value: the callers' branch does not go, only the argument.

## F-2a-05: confirmed

mod.rs:3479-3582: `Reason` is `#[non_exhaustive]`, and `RecursionLimit`, `CopyBound`, `MemoryLimit`, `WorkLimit`, `Unchecked`, `WitnessDepth` and `Instances` each carry `#[non_exhaustive]`, while `PathLength` (:3559), `ModelSize` (:3566) and `KripkeSize` (:3576) do not. core.md (Decisions, :382-389) states the rule for every struct-like variant of a marked enum. No crate outside the core builds these variants: cli/src/prove.rs:920-928 matches them with `..`, and web, bench, docs and skills do not name them, so the mark breaks no caller in the workspace.

Correction: The witness's detail is off: `Reason` is not among the enums core.md's parenthesis lists by name; the rule applies because `Reason` itself is marked `#[non_exhaustive]`.

## F-2a-06: confirmed

`Reason::as_set` (mod.rs:3628-3638) is idempotent, and `conclude` applies it to every unknown (:1137), as core-search.md (the memory bound, '`Reason::as_set`, which `conclude` applies to every engine's unknown') records. The seven engine-side calls are at additive.rs:115, focus/mod.rs:238 and :256, focus/parallel.rs:98, focus/schedule.rs:120 and :630, inverse/mod.rs:98. Between an engine and `conclude` the reason is only matched by variant (`handed_over` :958-966, `Turns::take` :1731-1738, the `WorkLimit` rewrite at :905); a grep for comparisons of a reason's value in the search modules outside tests found none. Every engine's `Err` reaches `conclude` through `Answered::concluded`, in a race too, with the same copy bound and limits. The one test that calls an engine directly (focus/tests.rs:2221) asserts `IndexLimit`, which `as_set` leaves alone. No search step changes.

## F-2a-07: confirmed

mod.rs:831 asks `probes(&task, options)` and only its true branch builds the closure that calls `turns.borrow_mut().take(…)` (:842) with the same task and the same (shadowed) options, so the test inside `take` (:1704-1709) can never return early and `asked` is dead; `take`'s doc (:1673) and the field's (:1660) still describe it. `probes` (:1532-1544) scans `DISPATCH` again after `dispatch` (:1515) did in `prepare`. The poll closure is a temporary of the one `decide` call, and `turns` is next used at :860 after it, so a `&mut` borrow would serve. The recorded choice (core-search.md, 'Why two closures') is about keeping the turn's comparison out of the plain poll, which the change keeps. The dead test is in the cold `take`, not in the poll, but the closure's captures change, so the ratchet's search journeys are the right net.

## F-2a-08: candidate

mod.rs:591 calls `engine_for` (:1242-1255: `fragment_of`, `read`, `prepare`), and `decide_goal`/`answer_goal` (:770-782) compute the same again, a third time on the pool's thread (:629). Reading supports the repeat, but it is narrower than claimed: `||` short-circuits, so `engine_for` runs only with three or more threads in an unordered mode; in classical mode `read` (:1282-1290) returns `None` without a pass, so the repeat there is `goal_fragment`'s walk of the goal's subtrees (:1858-1866) and the dispatch's test of the goal's shape (a Horn program's reading on MELL goals); only in intuitionistic mode is `Reading::of_mode` repeated. On ordinary sequents these are microseconds; core-search.md's set-up figures are for a forest of 27.8 million occurrences. Sharing the task also changes which `OnceLock` holds the Horn program, which is no search change.

Correction: Measure: pinned time to the first `Progress` of `deduxit prove --jobs 4` on `SYJ212+1.020` (cbv translation) in intuitionistic mode, and in classical mode on a large classical input, before and after; no callgrind journey goes through `race`. Value applies only from three threads on, not 'every command run on every sequent'.

## F-2a-09: confirmed

mod.rs:3432-3443: the variants' docs say nothing of the check. `Answered::concluded` sets `checked` only when `options.check` is on (:743), and with it off `conclude` returns the engine's proof after a `debug_assert` (:1143-1146). In `conclude` the counts' refutation (:1058-1068) and the classical one (:1069-1073) go into the disproof without a call of a checker; only a saturated set or a basis is checked (:1074-1117). refutation.rs:856-895 says `check` verifies a disproof read from elsewhere and that reading does not check the refutation. handovers.md (unit 2a) asks exactly this of `Verdict`. X3 asks that no value reads as checked without the check; here the gap is in what the docs promise.

Correction: Place: `Verdict` is at mod.rs:3433 (the derive at :3432).

## F-2a-10: confirmed

The three places hold: `poll` (mod.rs:924-932), the turn's stop in `Turns::take` (:1720-1725) and the check's poll in `conclude` (:1082-1086) with the same test repeated at :1108-1111. They differ in what counts (the shared `Counted` in the first two, a local `done` and `limits.work` in the third), in the phase, and in the turn's extra budget, so the common part is three lines each. The gain inside the front door is small tidiness; the real gain is the public meter of F-arch-03, whose verifier noted that moving the meter to `limits` is already recorded in plan/later.md. `poll` runs at every poll of every engine, and core-search.md ('Why two closures') records that seven instructions a poll took a journey past its ceiling, so the ratchet decides whether the shared type is free.

Correction: Value: mostly that of F-arch-03's public meter; decide it with that finding.

## F-2a-11: confirmed

mod.rs:696-707 (`Answered`), :985-996 (`Concluding`), :1362-1380 (`Task`) repeat forest, occurrences, fragment, mode and roots; `Answered::concluded` (:712-753) takes the `Goal` again and unpacks into `conclude` (:1028-1037), which has eight arguments under `#[expect(clippy::too_many_arguments)]` (:1024-1027), `result` and `refutation` being two fields of `Answer` and `members` a field of the `Goal`. The `unreachable!` at :1103 follows from keeping the last-resort refutation as an `Option<Refutation>`. `Goal` is `Copy`, so `Answered` can hold it. No search or checker is touched; the gain is for a reader.

## F-2a-12: confirmed

batch.rs:78-80, :176-186 and :265: `batch::Options::mode` defaults to `Mode::CLASSICAL` and a `Problem` with `mode: None` takes it, while `prove`, `prove_within`, `prove_goal` and `race` all take the mode as an argument. core-batch.md records no reason for the default (no line of it names the mode). D1 as written binds the commands, not the library; what makes this a finding is handovers.md (unit 2a), which asks what the library's defaults should be once the command has none. The command always sets the field from its flags (cli/src/settings.rs:250, cli/src/batch.rs:1459). The change touches the settings' wire form (`batch.mode` in `Settings`, core/src/settings.rs:141, a `serde(default)` object) and the manual (docs/src/batch.md:91-92, docs/src/settings.md:51-52), and it reaches the cli crate, so it is not within one module.

Correction: Cost: large (core's batch and settings, the command's settings and batch, the manual, the wire schema). Criterion: D1 by way of the handover; the library half is the author's to decide with the API proposal.

## F-2a-13: confirmed

mod.rs:1330-1337 refuses `Some(Engine::Unknown)` at run time, :2643 has the `unreachable!`, and serialize/search.rs:845-853 reads any unknown name as `Engine::Unknown`, whose `name()` then writes `unknown`, so a read and re-written outcome loses the name. The choice is recorded (core-search.md:381-388); its reason is that an outcome's unknown engine name is read and never refused, and the finding answers it: that reason concerns the outcome's field, not the options' type, and a separate type for `Outcome::engine` keeps it. The cost is understated: `Outcome::engine` is a public field that the command and the harness read (cli/src/prove.rs, cli/src/ordinary.rs, bench/src/run.rs, bench/src/tools.rs), with `counters()`, `name()` and comparisons against variants on it, so a new type there reaches several crates. It overlaps F-2a-02's `Engine::External`.

Correction: Cost: large (the outcome's field is read in cli and bench as an `Engine`). Decide together with F-2a-02.

## F-2a-14: confirmed

mod.rs:3470-3478 lists eleven kinds (stopped, recursion_limit, copy_bound, memory_limit, index_limit, work_limit, unchecked, witness_depth, instances, incomplete, path_length) and has no separator after the `witness_depth` example (:3475); `Reason::NAMES` (:3587-3601) has thirteen, with `model_size` and `kripke_size`, whose variants are at :3566 and :3576. I did not open the serializer to confirm the two JSON examples the finding gives for them; whoever writes the doc should take the field names from the wire form.
