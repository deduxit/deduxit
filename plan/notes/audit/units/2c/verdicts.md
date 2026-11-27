# Unit 2c: verdicts

14 confirmed, 3 candidate.

## F-2c-01: confirmed

Opened core/src/linear/families.rs:684-765 and 1099-1112. `power` joins `n` copies, so n = 0 gives the empty string; `three_partition` (704), `three_partition_mll` (727-730) and `partition` (754-760) splice it into text that `parse` (1110-1111) unwraps with a panic. The column of the first message matches the text the code builds (`b1, b1, b1, t1, t1, t1, ( * t1 …`, character 27). The docs of the two 3-Partition functions have no `# Panics` section and `partition`'s names only the odd total. core-inputs.md:98-101 records that `Family::instance` refuses a bad size before a generator's assertion is reached, but says nothing that covers the public encodings called directly, which the module doc (35-37) offers for the caller's own data. C-VALIDATE and C-FAILURE both apply.

Correction: The change is incomplete as written. The 0-bins panic of `three_partition` comes from `clauses.join(" & ")` at 706 (an empty hypothesis), not from `power`, which is never called with ` & `; an empty `items` likewise gives an empty goal (708-712) and, in `partition`, an empty first hypothesis (750-751). Either each of these joins gets its unit too, or the remaining preconditions are named in `# Panics` or returned as an `Error`. `partition`'s `sizes.iter().sum::<u32>()` (748) is a further undocumented precondition (a total past u32).

## F-2c-02: confirmed

Opened core/src/linear/lltp.rs:270-323 and 420-434, ordinary/parse.rs:942 and target/audit/dhat.md:104-155. The quote is at lltp.rs:300-306. DHAT attributes 6007 of read-lltp's 10465 blocks to the frame `lltp::read (lltp.rs:120)` and 6004 of read-tptp's 18418 to `read_tptp (parse.rs:942)`, both the inlined call of `clauses`; nothing else in `clauses` allocates per line (`code` has its capacity, the role and name are slices), so about three blocks a line of 2000 lines is the `collect` growing. Every character the lookahead tests is ASCII, and a byte of a multi-byte character is never an ASCII name character, so a walk by `char_indices` with byte lookahead answers the same; `code` still gets one character per character, which `in_file` (214-227) needs. handovers.md:110-112 already points at the same number. The blocks are measured; the instruction effect is the ratchet's to show.

Correction: The cold `glued` (lltp.rs:420) takes `&[char]` and must be given the line as `&str` in the same change.

## F-2c-03: confirmed

Opened core/src/linear/mist.rs:139-185 and 408-671, errors/mod.rs:75-85 and cli/src/io.rs:420-435. `Token` (434-439) holds a word or a line end and no position; every `error(...)` message names a token or a counter only; `Error::Spec` has the one field `message` and is `#[non_exhaustive]`; the command passes it on at io.rs:435. core-inputs.md:79-80 confirms files of up to 213 625 rules. No rules file records a choice against a line in the message. L5 (an expected error says what to do) fits. The LLTP reader's own `Error::Lltp` messages name the clause instead of a line, which a `.spec` rule has no equivalent of.

Correction: The line is cheaper to get than proposed: the reader already keeps `Token::Line` tokens, so the line of `self.at` is the count of `Line` tokens before it; the `counter` closure (158-163) has no reader, so its error gets the line where `rule` and the `init` loop call it. The net overstates the test: `reads_problems` matches these errors by variant (`Err(Error::Spec { .. })`), so check which asserts compare the text before assuming they change.

## F-2c-04: confirmed

Opened core/src/linear/lltp.rs:87-118 and 377-400. The `# Errors` bullet (line 110) names `axiom`, `hypothesis` or `conjecture`; the first bullet of the same comment (90-92), the match at 379 and the error message at 397-398 all take `lemma` and `theorem` too, as core-inputs.md:30-32 records. A plain contradiction inside one doc comment.

Correction: The phrase stands at lltp.rs:110 (108 is where the bullet starts).

## F-2c-05: confirmed

Opened bench/src/journeys.rs (journey list 42-132; no `inverse`, no `Engine::Inverse`), bench/TARGETS.md:300-311, bench/targets/47.csv:23-34, bench/targets.sh:167-170 and .claude/rules/bench.md:804-899. No journey of the ratchet or of DHAT runs the inverse method: that part holds. Two statements of the witness are wrong. `grep -i inverse bench/TARGETS.md` does find lines 300 and 306, and the focused target set has four rows the inverse method answers: `mix/8` to `mix/11` carry `inverse` in the `engine` column of the oracle 47.csv. But TARGETS.md:306-309 says why they guard nothing: no rule survives there (every `⊕` has a `0` side), so the saturation does no work and all its counters are 0. The engine's saturation (`derive`, the tries, `fill`, `fits`) is therefore still unmeasured, and the change is sound: a journey with a ceiling is a row of the ratchet and touches no library code.

Correction: Witness: four target rows (`mix/8`-`mix/11`) are answered by the inverse method, trivially. Change: do not take `mix/12` under the default as the journey; it would count the focused engine's 2²⁴ units and then an inverse turn with no rule. Take an instance on which the engine saturates and decides, forced with `Engine::Inverse` so the journey is deterministic (bench.md:818-826 gives the same reason for forcing the net engine): `wide-m1` forced, or `contraction` #1.

## F-2c-06: confirmed

Opened horn/reach.rs:503-558 and 677-713, horn/equation.rs:290-334, horn/cover.rs:306-349 and horn/mod.rs:241-257. `moved` (686-705) and `Tableau::new` (304-323) have the same five-arm ladder over a transition's inputs and outputs, and `caps` does the same merge with `peekable`: one iterator of (place, taken, given) serves these three. `moved` is a pure function of the arcs and the counts, so the distance, the frontier's order and the counters stay; the kind is right, and `moved` runs once per candidate transition in `expand`, so the ratchet's `search-spec-chain` row and the Horn target rows are the right net.

Correction: The count is three, not four. `Cover::before` (cover.rs:316-347) merges an element with the inputs and steps through the outputs with a saturating subtraction, and reach.rs:526-547 merges the marked places with the outputs; neither is the inputs-outputs merge and the proposed `arcs` does not replace them as written. mist.rs:630 is another module's reader and stays apart.

## F-2c-07: confirmed

Opened core/src/linear/mist.rs:412-430 and 619, and target/audit/dhat.md:117-128. The quote is exact; DHAT gives `mist::merged (mist.rs:421)` 2001 of read-spec's 12440 blocks (16 %). `Vec::dedup_by` passes the later element first and the kept one second, so the proposed closure keeps the larger count of a counter named twice (after `sort_unstable` on pairs the later one is the larger anyway), and `retain` then drops the zeros as before. Same result, the second vector gone. E2 fits.

## F-2c-08: candidate

Opened horn/mod.rs:677-725 and horn/equation.rs:290-354. Both clones are there: 699 clones both arc lists for a lookup and 703 moves the originals in; equation.rs:328 puts a clone of each distinct effect into `seen`, which lives to the end of `Tableau::new` and is not in the bytes charged at 346-354. Reading supports two allocations per clause under `?`, but DHAT's frame (search-spec-chain, 4015 blocks at an inlined closure of `Program::read`) does not separate the clones from the `Vec<u32>` lists of each clause, and the path runs once per task, not per step. Measure: DHAT blocks of `search-spec-chain` before and after; the Horn target rows' counters must stay.

Correction: Change, second half: `crate::hash` offers std's `HashMap` and `HashSet` only (core/src/hash.rs:20-23), so a set of indices hashed by the slice is not available without a new dependency, and "sort the effects and dedup" is not behaviour-neutral: it reorders the tableau's columns, hence the simplex's pivots and the point at which the equation refutes beside the search. Keep the first-seen order (for example a map from the effect to its index, emptied into `effects` by index). `entry` for the transitions is sound: `push` borrows `transitions` and `arcs`, not `seen`, and the transitions keep their order.

## F-2c-09: candidate

Opened inverse/index.rs:73-140, inverse/mod.rs:454-460 and 476-487, and horn/cover.rs:365-371. `walk` starts with `vec![(start, 0usize)]` on every call; `within` calls `ends_within`, hence `walk` again, for each visited node that has a separator child (not for every node, as the claim says); `subsumed` asks both tries for every derived sequent. The Horn trie does keep its stack in a `RefCell`. Two stacks are needed because the outer walk's closure runs the inner walk; `alive` does not touch the trie, so no borrow overlaps. The lookups counted (`looked`) and so the units of work do not change, so it is a refactor. No journey runs the engine: measure DHAT blocks and callgrind instructions on the journey of F-2c-05, before and after.

## F-2c-10: candidate

Opened inverse/mod.rs:462-471, 529-568, 725-760 and 854-878, and inverse/eval.rs:47-127. The three quotes are exact: `fill` clones the hole list of the sequent's first member (538) or collects every hole's index (539) per kept sequent, and the comment at 534-535 says a goal's phase can have millions of holes; `fits` builds `most` per premise choice under a reading; `coherent` collects and sorts `gamma` and `theta`, each already sorted (`theta` after an order-keeping `retain` at 477-478). The changes keep what is kept and the units counted (`candidates.len()` is the list's length either way). Reading supports it; only a measurement on a journey that saturates (F-2c-05) decides: DHAT blocks and callgrind, before and after, with `agrees_with_the_focused_engine_and_the_reference` for the verdicts.

## F-2c-11: confirmed

Opened inverse/eval.rs:27-127, proofs/multiset.rs (whole), proofs/mod.rs:31-35, search/saturated.rs:490-540 and core-proofs.md:91-99. The duplicate is real: `eval::insert` and `Multiset::insert` are the same two lines, and `remove`, `sum`, `excess` and `within` match `remove`, `sum`, `difference` and `is_subset` in meaning. The checker uses `Bag` (check/pass.rs:9, 611), so the rule that the check shares no code with an engine is not touched, and `saturated.rs` keeps its own copy as core-inverse.md:219 asks. The gain is tidiness at medium cost, on a path no instrument measures yet.

Correction: The change needs another shape. The engine compares slices of its store and of the goal's leaves, not zones (`within(gamma, leaves)` at inverse/mod.rs:555, 561, 573, 649, 711; `excess` at 558, 836, 973; the tries take `&[OccId]`), so retyping `Zone::gamma` alone leaves those calls with nothing to share: the shared form is functions over sorted slices (or `Multiset` methods that take `&[T]`), which `Multiset` then calls. `Multiset::merge` (89-107, a run count per id) is slower than `eval::sum`'s plain merge, so the engine's sum must not go through it; wait for the journey of F-2c-05 and compare its count. Value: the callers of `Multiset::is_subset` found are the test oracle's (proofs/oracle.rs:281, 593-594, compiled for tests only); pass.rs:613 and 964 call `Bag`'s, so "stops allocating for every caller" gains little.

## F-2c-12: confirmed

Opened core/src/linear/families.rs:803-810 and 954-963, target/audit/dhat.md:130-141 and 195-206, bench/src/journeys.rs:199-221 and bench.md:806-813. The numbers hold: 3096 + 3096 of read-text's 14919 blocks (41 %) and 52090 + 32758 of search-additive-14's 167253 (51 %) are the generators'. But the generators run outside `measured` (journeys.rs:207-209), which is all the ratchet counts (bench.md:809-811), so no ceiling and no user's hot path includes them; only the audit's DHAT run, which counts the whole process, does. P1 says to optimise what is hot, and a generator's allocations are not.

Correction: Place and kind: the mend belongs in the instrument, not in families.rs. Have plan/audit/dhat.py leave the frames of `deduxit::linear::families` and of `deduxit_bench` out of each journey's table and total (kind build, cost small). Rewriting the generators with `write!` is not supported by the criterion: the gain is a cleaner inventory, at the risk of changing text that must stay byte for byte.

## F-2c-13: confirmed

Opened horn/mod.rs:614-643 and 844-866, horn/proof.rs:125-141 and 190-202, and the callers by grep (`focus/schedule.rs:311-318`, mod.rs:535-537). `Reader::clause` (624-631) is `clause_head`'s walk (856-862) with one arm that also records a place, and `head_of`, proof.rs:134 and proof.rs:196 walk each fired clause again. One definition would serve all, `focus::schedule::chains` included (core-horn.md:33-35).

Correction: A condition the finding does not state: `Reader::place` gives places out in the order the walk meets the literals (mod.rs:594-601; core-horn.md:125-126), body literals in pop order and then the head's. The places' numbers order the arcs and the transitions indexed by place, and those break ties among successors (core-horn.md:216-218). So the shared form must be a callback per body literal inside the same walk; "`clause_head` plus a second pass" that numbers places in another order moves the target set's counters and is a search change. Keeping each clause's head in the `Program` means a head per clause of every class in `once`, not per transition, since a class's clauses have different head occurrences.

## F-2c-14: confirmed

Opened lltp.rs:23-41, mist.rs:42-64, families.rs:43-74, linear/mod.rs:43-46 and core.md:113-116. The four examples import `Options`, `Verdict` and `prove` from `deduxit::linear::search`; `linear` re-exports all three (mod.rs:43-46) and core.md tells users to write `deduxit::linear::prove`. Consistent with F-1a-04, which the verifier of unit 1a confirmed under the same criterion. The doc tests run the examples, so the change is checked.

Correction: The second families example's import is at families.rs:64 (63 is `use deduxit::linear::families;`).

## F-2c-15: confirmed

Opened horn/reach.rs:479-494 and 739-762, horn/cover.rs:29 and 501-519, horn/mod.rs:357-361. Both heaps grow by hand beside the `Vec` helper `room`, which lives in reach.rs and is imported by cover.rs:29 and called from mod.rs:360. Tidiness at small cost; no search order depends on it.

Correction: The net's "the charges stay the same in bytes" does not hold for one helper as described: the two policies differ. The frontier reserves `successors.len().max(capacity)` with no floor and tests `fits` on the whole of `more`; the queue grows by `capacity.max(16)` for one element. A shared `room_in_heap` either takes the growth as a parameter or changes one side's capacity by up to 15 elements near an empty heap, which moves a refusal only at a bound within a few hundred bytes; `refuses_at_its_limits` shows whether a pinned bound notices.

## F-2c-16: confirmed

Opened lltp.rs:64-85, mist.rs:74-96, families.rs:83-105, sequents/mod.rs:186 and fragment.rs:397. The three structs derive `Clone, Debug` only; their fields are `Sequent` (`PartialEq, Eq`), `Option<Status>`, `Option<Safety>`, `String`, `Mode` (`PartialEq, Eq, Hash`), `bool` and `Option<u32>`, all `Eq`. No rules file gives a reason to withhold the traits. C-COMMON-TRAITS says what the finding says; the change is additive.

Correction: The witness's last sentence overstates: `verdicts_as_constructed` (families.rs:1182-1185) compares the two instances' `sequent` fields because that is what it tests, not because it has to.

## F-2c-17: confirmed

Opened inverse/rules.rs:319-335, 762-780 and 813-830. `compile` and `goal` build `Writer` with the same eleven-field literal, differing in nothing. A `Writer::new(shape, charged)` is behaviour-neutral. The gain is small and so is the cost.
