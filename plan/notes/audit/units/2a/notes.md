# Unit 2a notes (finder)

## mod.rs 1-2000
- mod.rs:788-860 + 1657-1709: `Turns::asked` and the `probes()` re-check inside `Turns::take` are dead: `take` is only called in the `if probes(&task, options)` branch of answer_goal (837-854), same task, same options. Docs of take (1673) "On the first turn it asks whether the row gives turns" describe an earlier lazy design. Also `RefCell<Turns>` only needed because closure borrows; plain `&mut` works (closure is a temporary).
- mod.rs:1080-1086 conclude: hand-rolled meter (done += units; stop(Progress::new(Check, units, done)) || limit) — same pattern in Turns::take 1720-1725 and poll() 924-932; arch's Meter proposal.
- mod.rs:1103 `unreachable!("only a set or a basis is kept for last")` — `last` is Option<Refutation> though only two variants; a private enum Last{Saturated,Basis} would make it total.
- mod.rs:1456 panic! in Answer::of_arena on a build error other than Stopped (internal invariant).
- mod.rs:560 race: 7 params, threads beside options.jobs (two thread knobs); stop is `Fn + Sync` while prove_goal's is FnMut.
- mod.rs:591 race calls engine_for (whole prepare pass incl. reading) then answer_goal repeats fragment_of/read/prepare: double set-up passes (fragment, reading) on race path. candidate perf on large forests.
- mod.rs:1242 engine_for pub; surface says 0 uses outside core/src (rules say "the command asks both").
- mod.rs:345 `members.len() as u32 - 1` cast.
- mod.rs:1532 probes() scans DISPATCH again (dispatch already found the row in prepare) — row found twice; Task could carry the row.

## mod.rs 2000-5552
- mod.rs:3559,3566,3576 Reason::PathLength/ModelSize/KripkeSize lack #[non_exhaustive] while every other struct variant (RecursionLimit 3486 ... Instances 3546) has it: breaks core.md Decision "every struct-like variant of a marked enum". A later field = break.
- mod.rs:3470-3477 Reason JSON doc misses model_size, kripke_size; missing comma after witness_depth example.
- mod.rs:2643 Engine::implementation unreachable!(Unknown); Options::engine: Option<Engine> admits Unknown, refused at run time (prepare 1331). Recorded choice (core-search.md 381-388).
- mod.rs:3008-3037 four ordinary-only fields (F-arch-02 agrees).
- mod.rs:3404-3408 Outcome.checked pub writable (F-arch-23).
- mod.rs:3460 Verdict has proof() only; no disproof()/reason() accessors. minor.
- mod.rs:3823-3842 Statistics::add / add_run: `+=` (no saturating); fine.
- mod.rs:2226 Display for Engine docs list names without `unknown`.
- mod.rs:3855 redundant #[cfg(feature="parse")] inside cfg(all(test,parse)) module.
- tests of Refutation Display in mod.rs tests (4631-4696) belong to refutation.rs (unit 1c).
- race signature 7 args; tests call race(goal, mode, &options, &limits, 3, |_| true, |_| false) — positional ints/closures unclear (C-CUSTOM-TYPE?).

## memory.rs, work.rs, additive.rs
- additive.rs:115 `.map_err(|r| r.as_set(options, limits))` duplicates conclude's `reason.as_set(options, limits)` (mod.rs:1137) applied to every engine's unknown. Redundant.
- additive.rs:100 unreachable! on goal len != 2 (admits guarantees). ok.
- additive.rs:217 `.unwrap()` on f.left/right for With (invariant). ok.
- memory.rs: Account crate-private (arch: move to root). charge(bytes as u64) casts.
- work.rs:69-85 lock/taken helpers in `work` module (module doc says "Also the helpers") — mixed module, tiny.
- Pattern "count units; stop(Progress::new(phase, units, done)) || limit" at mod.rs:924-932 (poll), mod.rs:1720-1725 (Turns::take), mod.rs:1082-1086 (conclude's set check): three hand-written meters in one file; arch's Meter (F-arch-03) would serve all and outside engines.

## batch.rs
- batch.rs:79-80,181 `batch::Options::mode` defaults to Mode::CLASSICAL; Problem::mode Option<Mode> falls back to it (265). D1: a batch silently classical. Settings `batch.mode` (handover leader).
- batch.rs:520-641 Workers::start 120 lines (clippy too_many_lines list: batch.rs:520), worker loop closure 549-609 inline; seam: Shared::take_next(&self) -> Taken{Problem, End, Panicked}.
- batch.rs:906/932 duplicate test bodies (duplicates.md) — two panic tests differ by where the panic is.
- batch.rs:137 Options::plan pub(crate) but Plan pub with pub fields + Plan::alone; ok.
- batch.rs:252 prove(): stop `|_| cancel.is_cancelled()`; no time limit; fine.
- batch.rs:624 unreachable! after Arc::try_unwrap; relies on spawn dropping closure on error.

## parallel.rs
- parallel.rs:368 crate-private `race` beside public search::race (mod.rs:560): same name, different meaning (two works on two pools vs one thread + pool).
- parallel.rs:28 threads(): available_parallelism per search when jobs>1.
- No std HashMap/HashSet in the unit (additive uses crate::hash::HashMap); BTreeMap in batch is fine.

## cross-checks
- mod.rs:387-394 on_own_forest maps CheckError::Refused to Err(Error::Check) while conclude (1150-1156) maps it to Verdict::Unknown via unchecked(); prove_goal doc 200 says "no error". Drift between two copies.
- Reason::as_set called in engines (additive.rs:115, focus/mod.rs:238,256, focus/parallel.rs:98, focus/schedule.rs:120,630, inverse/mod.rs:98) and again in conclude mod.rs:1137; idempotent, every Err passes conclude.
- Goal (mod.rs:407) hides members: occurrences() private (460); arch's public Decide over Goal would hand an engine a goal it cannot read.
- Counter::new private (2253): outside engine cannot list counters.
- race(threads) ignores options.jobs; cli/src/prove.rs:66-85 and bench/src/run.rs:1113-1127 both branch prove_goal-with-jobs vs race(jobs).
- race: engine_for (591) then answer_goal (771-782) repeat fragment_of/read/prepare: set-up twice (thrice with pool). candidate.
- conclude doesn't Disproof::check counts/classical refutations (1058-1073); Verdict docs silent.
- type sizes: Answer 1216 B (net 608 inline), Turns 1240 B; moved a few times per search; negligible, not raised.
- engine_for: no caller outside core/src (surface); core-search.md:403-407 says the command asks it (stale: the command calls race).
