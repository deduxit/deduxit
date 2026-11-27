# Unit 2b: verdicts

13 confirmed, 3 candidate.

## F-2b-01: confirmed

Opened core/src/linear/search/focus/counts.rs:328-362 and 1240-1263, core/src/linear/proofs/interactive/expand.rs:740-775, cli/src/interact/mod.rs:536, web/src/calls.rs:815. split_passes builds Counts::new on every call, and Counts::new is new_until with Account::new(None) and a stop that never fires; new_until's own doc (counts.rs:353-355) says the rows can be quadratic in the forest and are therefore charged, which this path switches off. The public doc at expand.rs:743 calls it a cheap test. No rules file records the unaccounted pass as a choice (core-focus.md:845-848 and core-derivations.md:489-492 only say it is the engine's own test). I did not rerun the measurement; the figure (about n^2/2 row entries at 16 000 atoms, 1.5 GiB) agrees with the row model the code documents. S1 of audit-rubric.md:51 says what the finding says.

Correction: Kind: the fix need not break the API. StepError is #[non_exhaustive] (interactive/mod.rs:87-88), so a new variant for the refusal is additive, and the session can keep the limits it was built within; only a changed signature (taking &Limits, or another error type) is a break. Caching the counts per session does not help the web bindings, which rebuild the session on every call (web/src/calls.rs:815-819); the bound does.

## F-2b-02: confirmed

Opened keyed.rs:105-165 and mod.rs:1340-1440, 1525-1535. The push is the same eleven statements at keyed.rs:151-165 and mod.rs:1344-1360; the pop with the own depth at keyed.rs:112-117 and mod.rs:1374-1379; the settling of a dependency at keyed.rs:125-129 and mod.rs:1390-1394; the Complete/Hereditary choice at keyed.rs:134, mod.rs:1432 and mod.rs:1532. One implementation serves both paths. core-focus.md records no reason for the copies. prove_listed is on Ground's hot path and on the recursion cycle (core-focus.md:1007-1027, 1075-1085), so the helpers must stay inlined in a release build and a debug frame must not grow: the ratchet's search journeys are the net, as the finding says, and the helper calls return before the recursion, so they add no level.

## F-2b-03: confirmed

Opened focus/mod.rs:327-371 (Switches), counts.rs:10, 1167-1199, 1247-1263, and grepped core/src for focus::counts and focus::bias: the outside callers are linear/search/mod.rs:1060 and horn/mod.rs:88 (counts::refutation), inverse/mod.rs:144 (bias::signs) and proofs/interactive/expand.rs:768 (split_passes), as quoted. counts.rs does import the engine's private Switches. core-focus.md:41 and 690 name the two files under focus/ but give no reason for the placement. The second half (a public linear::Counts) is a question for the author, not part of the finding.

Correction: The claim's 'four engines' is loose: the counts are read by the focused engine, the Horn engine, the front door (search/mod.rs) and the interactive session, and the bias by the inverse method. Switches should not move whole: its fields exponentials, affine and stack are the engine's own (mod.rs:339-346); what counts.rs needs is intervals, equation and mix, so the move splits those three off (or passes them) and leaves the rest in focus.

## F-2b-04: confirmed

grep '#[allow(' over core/src gives exactly the four sites named (split.rs:606, split.rs:660, mod.rs:1698, scratch.rs:218), none with a reason; split.rs:878-881 uses #[expect(clippy::too_many_arguments, reason = ...)] for the same lint and scratch.rs:123-126 an expect for the sibling box lint. Cargo.toml:107-109 lists allow_attributes and allow_attributes_without_reason under 'Picked, and firing today: each goes to warn once its sites are fixed', and target/audit/clippy.md:42-43 counts 5 sites in the workspace, 4 of them these. So this is E7's own case: a lint whose sites can be fixed so that it goes to warn. The attribute change alone is safe; the regrouping of the argument lists is optional and touches the recursion cycle.

Correction: The value is larger than stated: with these four and the one site outside the core crate turned into expect, both lints of Cargo.toml:108-109 can go to warn. Keep the attribute change (small) apart from the regrouped arguments (a frame on the recursion cycle, PER_LEVEL measured again per core-focus.md:1024-1027).

## F-2b-05: confirmed

Opened split.rs:606-716. The two loops are the same next_split loop with the same join and the same closing match; the first-order one adds choices().enter, the leaf counter with its skip below choice.start, take and fail. core-focus.md:993 and 1301 name both functions but record no reason for keeping two. One loop behind if Z::FIRST_ORDER is how decompose and prove_unkeyed do it.

Correction: Cost and risk: search_splits is on the recursion cycle. Merged, a debug build's frame for Ground gains the Choice and the leaf counter although the branches fold away in release, so Limits::PER_LEVEL is measured again (core-focus.md:1024-1027). The fallback join_sides would add a frame per level between search_splits and premises unless it is #[inline(always)] (core-focus.md:1014-1019 gives that reason for the five that are). Pinned time on Ground, not callgrind alone, decides (core-focus.md:1346-1352).

## F-2b-06: confirmed

Outlined framed.rs by its item heads: the variables, bind and unify_pairs at 416-677, the matching walks equal, fits, instance_of, passes, pattern at 680-876, the keys' comment and code at 888-1350 (tags, KeyScratch, write_keys, write_member, name, kept, apply, own_key, canonical_key), impl Zone at 1353-1929, Closing at 1933-2029, Members and Set at 2034-2119. The seams are the ones named, and E5 asks for the seam. Child modules of a framed/ directory read Framed's private fields, so nothing need become more visible. core-focus.md names framed.rs in its layout paragraph, which the move updates.

## F-2b-07: confirmed

Opened memo.rs:233-250, waiting.rs:300-324, framed.rs:969-978 and 1874-1895, proofs/check/zones.rs:25-35, search/memory.rs:267. Three forms are there. memo's (floor division, with the two small cases) and waiting's (div_ceil) agree on every capacity a hashbrown table reports, so merging them moves no number. framed's two places charge capacity * (size + 1): no bucket rounding (about an eighth fewer) and no group.

Correction: Leave proofs/check/zones.rs:29 out, or raise the cost: it counts from the most members held, not a capacity, in u64, and a Kani harness (check/harnesses.rs:36) proves its bound, so folding it in touches the checker's integers (web-core-wasm32 and the Kani run). For framed.rs the change is not behaviour kept: it raises the first-order zone's charge, which can move the counters of a first-order row that relieves its memory (core-focus.md:1078-1081); the finding says so in its net, and the kind should say it too. Cost: medium (memory.rs, memo.rs, waiting.rs, framed.rs).

## F-2b-08: confirmed

Opened schedule.rs: the attribute stands before the doc comment at 324, 330, 335, 353, 457, 472, 494 and 593, and before impls at 366, 463 and 477; memo.rs:800 has it before the doc of Shared::new. Lines 324-638 of schedule.rs are one run of items that exist only with the feature, so one module behind one attribute holds them; the three use lines at 26-39 would move with it. E3 in criteria.md:113 lists 'doc comments before attributes'. Nothing in core-focus.md or core-parallel.md records the layout as chosen.

## F-2b-09: candidate

Opened framed.rs:680-682 (equal), 715-717 (fits), 745-760 (instance_of), 807-817 (passes), 820-829 (pattern), 1758 (unify calls passes unless replaying), 1765-1774 (open_answers). Each walk makes a new Vec and HashSet per call, and passes runs instance_of for every pattern of every open rule that searches its first premise, after every unification that succeeds. core-focus.md:1278-1281 records only that these temporary sets are not charged to the account, not that reusing them was tried and dropped, so no recorded choice stands against it. The walks take &self because Zone's matching methods read the zone (core-focus.md:1271), so the scratch needs a RefCell or mem::take, as the finding says.

Correction: Measure with DHAT on a first-order family that opens answers (witness-fo, chain-fo at the target set's sizes): allocations per million instructions from equal, fits, instance_of and pattern; then pinned time of the first-order rows. The counters unifications and bindings must stay equal.

## F-2b-10: candidate

Opened the nine sites (counts.rs:261, 271, 729, 811, 962, 977, 1026, 1101; zone.rs:555). core-focus.md:1014-1019 indeed records a reason only for the split search's inlined functions. But three of the nine rest on a recorded measurement elsewhere: plan/reports/42-first-order.md:988-996 and 1231-1234 record that callgrind found Ground::dual_in out of line (80 million instructions on the chain, search-chain-128 +2.6 %) and the counts' atom mapping outlined in the set-up pass, each fixed by #[inline(always)]: that is dual_in, counted_atom and open_atom. plan/reports/28-audit-and-refactor.md:1016-1018 records the PLAIN monomorphization (up to 10 % of a journey's instructions), which is why the four _as functions exist, though not what forcing their inlining adds over #[inline].

Correction: Place and scope: drop zone.rs:555, counts.rs:261 and counts.rs:271 (measured; the change there is to copy the numbers of plan/reports/42-first-order.md into core-focus.md). The candidate is the six left: excluded_as (729), apply_as (811), Column::excludes (962), count_bad (977), shift_as (1026), flip_as (1101). Measure each with #[inline] under the ratchet's search journeys (callgrind) and pinned time on the focused target rows; counters cannot move.

## F-2b-11: confirmed

Opened choices.rs:91-118 and 157-167, split.rs:672, 729-737, first_order.rs:59 and 279, resumed.rs:28. The sentinel is tested by hand at five sites, and Taken::Settled(u32) (choices.rs:77) carries only a bool: its one writer is split.rs:736 (u32::from(fail)) and its one reader split.rs:730 (fail == 1). A method on Choice costs nothing and no rules file records the sentinel as chosen (core-focus.md:1316 says only that a diverged replay's choice points fail at once).

Correction: resumed.rs:28 reads `if choice.start != u32::MAX {`, not `==`. A settled entry also has start == u32::MAX (choices.rs:161), so the method's name must not claim failure there: parts_fail_logged tests settled first (split.rs:729-733). The criterion fits loosely: C-CUSTOM-TYPE is about the arguments of an API, and these are private fields of one module; the gain is a reader's, and small.

## F-2b-12: confirmed

Opened framed.rs:1270-1350. Both functions sort the two zones, clear metas and numbers, write each zone's length, each member's copies and first word, then the member's words, then one word per metavariable; they differ in the comparator, in how a word is mapped (own_key numbers nothing, canonical_key renames META and ranks EIGEN) and in the level written. One writer with the word mapping and the level as parameters serves both. Nothing in core-focus.md records two copies as chosen.

Correction: The gain is small (about ten shared lines, private to one file), and the three-closure signature proposed is heavier than what it replaces: a word mapper that takes &mut KeyScratch conflicts with the loop's borrow of s.words and s.copies unless the member's range is copied out first. A plain enum of the two kinds matched inside one loop is the simpler form. This path runs per stable sequent of a first-order search, so the first-order rows' pinned time is part of the net, not only memo_hits and memo_entries.

## F-2b-13: confirmed

Opened counts.rs:356-609: one function with the passes at the lines named (the exponential atoms 404-425, the one-sided switch 426-438, the ranks 439-458, the literals under a ? and linear 459-480, the rows from 481 to 593), 253 lines by the file and 206 by Clippy's count (target/audit/clippy.md). The passes read each other only through a few vectors (exponential, rank, ranked, copied, unrestricted), so they separate. It is set-up, run once per search, so no hot path is at stake.

Correction: The criterion fits loosely: E5 is about files, and too_many_lines is one of the ten lints Cargo.toml:95-100 sets aside on purpose for a reviewer to judge; this is such a judgement, with the seams named. The helpers share the closures stopped, atom_of and open and the account, so their signatures are wide; the polls and the charges (counts.rs:392-403, 494 on) must fall at the same points, which the target set's counters do not show: a test with a counting stop does.

## F-2b-14: confirmed

Opened scratch.rs:20-248. The pairs are there: pop, clear, else charge and make at take_set (22), take_context (41), take_tally (104), take_split (127); pop().unwrap_or_default(), clear, and settle on give at take_list (77), take_words (91), take_trail (146), take_links (159). Pools has ten Vec fields and a hand-written Default, which it needs because a derive would ask Z: Default.

Correction: There are ten pairs, eight of them in the two shapes; take_context_any, take_frames and the cursors (reset on give) fit neither. The gain is tidiness alone, on Ground's hottest helpers (core-focus.md:1028-1042, 1075-1081: the pools and their charges must stay as they are). A generic Pool<T> with a trait for clear costs more than it saves here; two private helpers, one per shape, taking the pool's Vec and the scratch, remove the repetition at small cost (one file). Kind refactor stands; cost small in that form, medium as proposed.

## F-2b-15: confirmed

Opened mod.rs:753-780 and 1585-1623. remember's two match blocks at 1590-1600 and 1606-1616 differ only in the flag passed to nodes.collect; statistics and report each add the zone's two counters behind Z::FIRST_ORDER. A helper for each is a few lines.

Correction: report is #[cfg(feature = "parallel")] (mod.rs:771), so statistics cannot be written on top of it as it stands: either report loses its cfg, or the shared part is a third private function both call; otherwise the build without parallel breaks or warns of dead code (both cargo hack runs are the net). remember is the memo's overflow path, cold, so nothing is measured.

## F-2b-16: candidate

Opened mod.rs:1754-1759, 1971-1974 (one_of_each sorts by (class, o) and dedups by class, so the candidates' classes, hence the members, are distinct) and 2073-2089 (focus_class walks a tensor's children and asks forced_side of each, reading only &self). The keys (focus_class(o), o) are therefore unique, and even two equal keys would be two equal values, so sort_unstable on precomputed pairs gives the same order: the search visits what it did. slice::sort_by_key calls the key function at every comparison and allocates above twenty elements, the very cost core-focus.md:1043-1047 records for the copies before they were moved off it. Whether it shows depends on how many tensors a stable sequent holds, which only a profile says.

Correction: Measure the ratchet's journeys search-partition-no-5 and search-qbf-20-2 and a Petri-net row under callgrind for the instructions inside focus_class and the sort, before and after; the focused target set's counters against bench/targets/47.csv must be equal. Keep the candidates' buffer as it is (a second pooled buffer for the pairs would have to be charged).
