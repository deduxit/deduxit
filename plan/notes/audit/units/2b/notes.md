# Unit 2b notes

## Context read
- brief, criteria, rejected, handovers, consumer-test, core-focus.md (all), core.md 200-370.
- dhat.md: focus frames are small in every journey (scratch.rs:60 closure on Ground, split Frame Vec grow mod.rs:198 (Vec<Vec<Frame>>), Tally::add counts.rs:790). Allocation is not where focus cost is.
- inline(always): counts.rs 261,271,729,811,962,977,1026,1101; zone.rs:555; split.rs 314,373,768,849; mod.rs 1112,1124 (holding, two_premises: rules say for debug stack frames, measured).
- duplicates.md: split.rs:637/696 (176 tok); keyed.rs:151 / mod.rs:1344 (146 tok); framed.rs:1281/1314 (115).
- clippy too_many_lines: counts.rs:356 (206), mod.rs:1250 prove_listed (161), waiting.rs:363 (109).

## mod.rs
- mod.rs:1698 `#[allow(clippy::too_many_arguments)]` on decide_with: crate rule says #[expect(lint, reason)] at site, never allow. Check others.
- mod.rs:754-780 statistics() and report() repeat the FIRST_ORDER unifications/bindings add.
- mod.rs:1589-1616 remember: two identical match blocks collect(Some(node), false|true) -> helper.
- mod.rs:1543 record_failure recomputes `repeats` by scanning for memo::REPEATED.
- mod.rs:736/743 left/right `.unwrap()` on forest.left/right.
- mod.rs:1344-1360 stack push duplicated with keyed.rs:151 (per duplicates.md).
- mod.rs:1250 prove_listed 161 lines: memo lookup / loop check / waiting / push stack / decide / close / record. Seams: lookup (1258-1343), push (1344-1360), record (1415-1449).
- mod.rs:132-150 work-unit constants; OCCURRENCES_PER_WORK charge per stable sequent even though key sparse: recorded on purpose.
- Decide interface: Focused{two_sided} with ONE_SIDED/TWO_SIDED consts; decide() dispatches on parallel + first_order: 4 arms.

## zone.rs, context.rs, scratch.rs
- zone.rs:39-338 Zone trait: ~20 hook methods with `unreachable!` defaults (mark, undo, forall, introduced, exists, opposite, unify, open_answers, searching_first, record, exhausted, close_answers, keys, excluding, take_steps, counts, held, close, choices): first-order hooks guarded by `if Z::FIRST_ORDER` const. Recorded decision (generic over Z); a FirstOrder sub-trait or assoc type would make misuse a compile error. Weigh vs. rules (core-focus 1058-1085, 1146-1155).
- zone.rs:527-532 Ground::fork == Ground::new body.
- zone.rs:151 `Zone::same` — check callers.
- scratch.rs:218 `#[allow(clippy::vec_box)]` (allow, no reason) vs crate rule #[expect(..., reason)]. mod.rs:1698 too.
- scratch.rs:20-201 nine take_/give_ pairs of the same two shapes (clear-or-make-and-charge; pop-or-default+clear / settle+push). A generic pool would halve it (E6).
- scratch.rs:290-302 Pooled<T> implements Deref/DerefMut to Vec (C-DEREF: only smart pointers). Crate-private.
- context.rs: fine; Clone impl manual because clone_from custom.

## arena.rs, keyed.rs, waiting.rs
- keyed.rs:151-165 push_stack is mod.rs:1344-1359 verbatim (duplicates.md 146 tok). prove_listed should call push_stack.
- keyed.rs:112-117 own_depth == mod.rs:1374-1379; keyed.rs:125-129 dependency settle == mod.rs:1390-1394; complete/hereditary choice 3x: keyed.rs:134-138, mod.rs:1432-1436, mod.rs:1532-1536 -> `Failure::complete(hereditary)`.
- waiting.rs:130 index HashMap<u64,u32> keyed by an already-mixed hash: foldhash hashes it again (candidate: identity hasher). cold path? find() per stable sequent only when waiting non-empty.
- waiting.rs:309-324 grown() re-derives hashbrown layout; memo.rs:238-250 Interner::buckets another formulation of the same (GROUP 16, 8/7, pow2). One helper.
- arena.rs ok. arena.rs:153 `crate::linear::search::lock` full path.

## memo.rs
- memo.rs:800 `#[cfg(feature = "parallel")]` before doc comment (E3: doc before attributes).
- memo.rs: many `crate::linear::search::lock(` full paths (820,837,845,854,856,875,877,885,893,924).
- Table enum dispatch Own/Shared per call (match per get/insert): fine.

## counts.rs, classes.rs, bias.rs, split.rs
- ALL FOUR `#[allow(` of the core crate are in this unit (grep: 4 total): scratch.rs:218 vec_box, split.rs:606, split.rs:660, mod.rs:1698 (too_many_arguments). split.rs:878 has `#[expect(clippy::too_many_arguments, reason = ...)]` in the same file. core.md/CLAUDE.md: new allowance is #[expect(lint, reason)].
- counts.rs/bias.rs are shared machinery outside focus: counts::refutation used by search/mod.rs:1060 and horn/mod.rs:88; bias::signs by inverse/mod.rs:144; counts::split_passes by proofs/interactive/expand.rs:768 (public Interactive::split_passes). They live in focus/ (`pub(crate) mod bias; pub(crate) mod counts;` mod.rs:60,68). Architecture: move to linear/search/ (or linear::counts). counts.rs also imports super::Switches (counts.rs:10) for refutation/split_passes -> Switches belongs with counts.
- counts.rs:356 new_until 253 lines (too_many_lines 206): seams = exponential pass (404-438), ranks (439-458), unrestricted+linear (459-480), row pass (481-593).
- counts.rs:1167 refutation recomputes Counts::new_until (whole set-up again) for the goal's refutation, with Bias::Rarer; when the focus engine just refuted, counts exist already (candidate; cold, once per refutation).
- Predicates::of computed in bias::signs (bias.rs:59) and again in new_until (counts.rs:382) and refutation (1187): cheap for propositional (early return).
- bias.rs:60 `let mut literals` closure is Fn; `mut` + `&mut literals` unnecessary (trivial).
- split.rs:607-655 search_splits and split.rs:661-716 search_splits_framed: same loop (duplicates.md 176 tok); join match repeated 638-643 / 697-701.
- split.rs: rest fine; chain/frames recorded.

## schedule.rs, choices.rs, resumed.rs, first_order.rs
- schedule.rs: 11 items each behind its own `#[cfg(feature = "parallel")]`, placed BEFORE the doc comment (324,330,335,353,366,457,463,472,477,494,593): SLICE, POLL, Baton, Turns, impl Baton, StopOnPanic(+Drop), Ended(+Drop), alternate, merged. One `#[cfg(feature="parallel")] mod alternate;` (or a nested module) would hold them; E3 doc-before-attribute. memo.rs:800 same order.
- schedule.rs:288 plan(): signs() computed twice to compare; cheap.
- schedule.rs:62-66 `leveled` closure; turns() fine.
- choices.rs: `Choice::start == u32::MAX` sentinel = "fails at once" read at split.rs:672, split.rs:732, first_order.rs:59, first_order.rs:279, resumed.rs:28 -> a method `Choice::fails()` or Option (C-CUSTOM-TYPE). Also Settled(u32) carries a bool for Parts (split.rs:730 `fail == 1`, 736 `u32::from(fail)`).
- first_order.rs:99-109 and 133-146 same match on Unified (No/Cut undo). Minor.
- first_order.rs:270 answers(): long but structured.

## framed.rs
- framed.rs 2119 lines, four things: variables/unification/answers (416-886), keys (888-1351: KeyScratch, write_keys, write_member, name, kept, apply, meta_level, own_key, canonical_key), Zone impl (1353-1929), Closing (1931-2029), Members/Set (2031-2119). Seams for E5: framed/keys.rs, framed/answers.rs (Pat, Pattern, Answers, instance_of, fits, equal, pattern, passes), framed/closing.rs.
- framed.rs:1275-1303 own_key and 1311-1350 canonical_key: same double loop (duplicates.md 1281/1314), differing only in the word mapping and the sort comparator; one fn with a mapping closure.
- framed.rs:680 equal, 715 fits, 745 instance_of, 820 pattern, 1771 open_answers: each allocates vec!/HashSet per call (`let mut pairs = vec![(a,b)]; let mut met = HashSet::default();`). `passes` (807) runs instance_of per pattern after every unification while the first premise of an `answers` rule runs (unify 1758). Candidate P2: reuse scratch (but &self; Framed has pairs/met/walk scratch already, used by unify_pairs). No first-order journey in dhat.md -> instrument: DHAT on a first-order family with answers (witness-fo / chain-fo).
- framed.rs:1444-1478 fork lists all 33 fields again (new 1402-1434); `adopt` uses struct update.
- framed.rs:383 steps AtomicU64 for Sync with one thread: fine.
- framed.rs:1534/1544 dual_in, dual_from linear scans (first-order only). recorded.

## parallel.rs
- read in full; search_goal mirrors mod.rs's (recorded: SetUp/prepare shared). Collected/Premise fine. No finding of weight.

## Measured
- split_passes (counts.rs:1247-1263 -> Counts::new 333-343, Account::new(None), no stop) on `|- c0 * (a1 * (a2 * ... a16000)), ~c0`, target/debug/deduxit interact --file wide.txt: `quit` alone max RSS 35 796 KiB 0.1 s; `apply 0 0 *` (calls split_passes from cli/src/interact/mod.rs:536) max RSS 1 539 156 KiB 3.6 s. Default Limits memory 1 GiB. Rows quadratic (n^2/2 entries). Interactive::split_passes doc (expand.rs:743) says "a cheap test". web/src/calls.rs:815 calls it too. S1 must-fix. StepError has no refusal variant.
- No SipHash map in unit (all crate::hash). Account use: charge/resize/release/spares/over/limit/used/share/fork; Charged; bytes_of.

## Findings chosen (order)
1 split_passes S1; 2 keyed/mod dup; 3 counts+bias placement; 4 #[allow] x4; 5 search_splits dup; 6 framed.rs seams; 7 hash-table bytes 3 ways; 8 schedule cfg module; 9 Framed allocs candidate; 10 inline(always) candidate; 11 Choice sentinel; 12 own/canonical key dup; 13 new_until seams; 14 pools generic; 15 remember/statistics dups; 16 candidates sort candidate.
