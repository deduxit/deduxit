# Unit 3: verdicts

11 confirmed, 2 candidate, 2 rejected.

## F-3-01: confirmed

Opened core/src/protocol/linear.rs:362-388 and :553-564, cli/src/show.rs:1474-1494 and :1589-1598, web/src/calls.rs:1491-1497, :2242-2331, and core/src/linear/nets/cut/mod.rs:69. The protocol's `linear.net` calls `ProofStructure::from_proof` with no fallback, while the command and the bindings fall back to `MellStructure::from_proof` on `Placement::Cut`. core-nets.md:206-209 says a front end that draws an MLL proof's net does the same as the command's `net_of`, so the protocol departs from the recorded rule; the rule gives no reason for leaving the choice to each front end. The three either-net enums, the two `nets_exist` and the two `vertices` probes are as quoted. X5 says what the finding says. The sealed `cut::Net` trait exists, so the proposed enum can implement it.

## F-3-02: confirmed

Opened nets/graph.rs:44-511 and nets/mell/graph.rs:451-769. `search`, `discover`, `acyclic`, `twin`, `cycle` and `Scratch` are the same code but for the set type (`OccSet` against `Vec<bool>`) and the deletion test, as quoted. core-nets.md records only that the hot-path functions stay non-generic with a `dyn` stop (lines 55-62), which the change keeps; no rule gives a reason for two kernels. The gain is one Yeo test to read and maintain; it is a checker and the net engine's hot path, so the instruments named are needed.

Correction: Three points the change must handle. (1) The MLL graph's layout changes on the hot path: a root gets a `NONE` slot 0, and `deletable` reads a bit where it now reads `forest.kind` and `slots.len() < 3`, so the ratchet's `search-wide-m2-256` decides whether it holds; with the checker and the measurement the cost is at the upper end of medium. (2) MLL's `cycle` clears `removed` when the stop fires (graph.rs:432-435) and MELL's does not (mell/graph.rs:632, harmless there because its scratch is made per call); the shared kernel must clear it. (3) The value's remark that F-3-03 is a fix the MLL copy never got does not support this finding: `parts` stays per structure in the proposed change.

## F-3-03: candidate

Opened nets/graph.rs:478-511, nets/sequentialize.rs:254-282, nets/mell/graph.rs:670-703, nets/mell/sequentialize.rs:314-340 and core-nets.md:428-433. The MLL `parts` does a linear `find` over the parts per conclusion-like vertex, and the Mix branch of `stage` does `labels.contains` per conclusion and then filters every conclusion once per label, all inside one stage, where the stop is not asked. The MELL copies index by root and sort with `binary_search`, and the rules record why. Reading supports quadratic growth in the number of parts; no measurement of the MLL side exists.

Correction: To measure: `is_correct` on `|- a1, ~a1, …, ak, ~ak` linked pairwise under `Criterion::MLL` (the `Disconnected` witness) and `sequentialize` of the same under Mix, at k = 20 000 and 80 000, by callgrind instructions or pinned time; a factor near 16 between the two sizes shows it. The port must keep the order of the Mix parts (by first conclusion in `gamma` then `cuts`), which the term's node order depends on.

## F-3-04: confirmed

Opened nets/mod.rs:623-630 and :871-885, and nets/mell/mod.rs:395-404. The one message says proof nets exist "in linear mode only, with or without Mix", and it is returned for a cyclic criterion with Mix, for cut links under the cyclic order (mode `CYCLIC`, for which nets do exist) and for a structure with boxes under the cyclic order. core-nets.md:512-516 records why these are refused, not the wording. Only a library user who builds a `Criterion` by its public fields reaches these cases, so the gain is small, as the cost is.

## F-3-05: confirmed

Opened nets/mod.rs:35-36, :778-779 and :1532-1537, nets/mell/mod.rs:299 and :803-808, export/svg/net.rs:341, export/svg/mell.rs:285, and the test helpers at nets/mod.rs:1991, search/net.rs:1102, search/essential.rs:706 and nets/cut/tests.rs:302. The normalise-and-sort is written at all eight places, and neither structure has `PartialEq`. No rules file records a reason (no mention of equality in core-nets.md). `sorted_links()` is sound and additive.

Correction: `Forest` derives only `Clone, Debug` (occurrences/mod.rs:251), so equality "by forest" does not exist yet: the `PartialEq` needs an equality of forests first, or compares the sequents. The kind is an addition, not a break. The test helpers are at the lines above, four lines after those the finding gives. With the export's two copies the cost is across two modules.

## F-3-06: confirmed

Opened nets/sequentialize.rs:126-219 and nets/mell/sequentialize.rs:151-236. The struct, `push`, `premise` and the `Mix`, `Tensor` and `Cut` arms are the same; MLL's `Pars` is MELL's `Opened` with `Open::Par` alone. MLL's `Prove` holds two lists (conclusions and cut ends) and MELL's one, which a generic parameter covers. No rules file records a reason for two copies. The shared part is about 45 lines per file, so the gain is tidiness; the stages, where the work is, stay apart.

Correction: The Mix partition cannot move to the shared assembly unchanged: MLL partitions two lists by `OccId` labels and MELL one list of graph vertices, so it is shared only if F-3-03's port gives both the same shape.

## F-3-07: confirmed

Opened nets/mod.rs:1160-1179 and :1562-1570, nets/mell/mod.rs:617-636 and :815-824, nets/cut/mod.rs:367-383 and nets/cut/mell.rs:229-255. The accessors are word for word the same, the verdict line is repeated (MLL has one more arm, for the planar net), and both axiom steps replace the first link and drop the rest. E6 names a copied verdict line. The gain is small and so is the cost.

Correction: The axiom step shares four lines only: MELL's also moves the weakenings hung on either literal and marks the ends erased (cut/mell.rs:246-253), and its links are `(u32, u32)`, MLL's `(VertexId, VertexId)`. The place spans three files of `nets`, so the cost is nearer medium than one file.

## F-3-08: confirmed

Opened core/src/linear/mod.rs:29-40 and core.md:56-58. The eight types have two paths and `nets::exist`, `Scratch` and `cut` have one. This is the matter F-1a-04 and F-1b-08 already confirmed for the rest of the linear suite (units/1a/verdicts.md:61-76).

Correction: The direction cannot be chosen for `nets` alone. core.md:56-58 records that `linear/mod.rs` re-exports its suite's public types, and `linear::Proof`, `linear::Forest` and the rest stay there; dropping only the nets' re-exports leaves the suite with two conventions. Decide it with F-1a-04, crate-wide. The cost is not small: eleven files of `cli`, `web`, `bench`, the manual and the guide name these types.

## F-3-09: rejected

Opened nets/mod.rs:273-280, :569-574, :939-945 and :1081-1092, and nets/mell/mod.rs:465-476. The field is a `u32` as quoted (and made at three places, :941 too). But the named gain is empty: the id names no vertex of the structure, so passing it to `partner` or `occurrence` answers `None` (core-nets.md:36-39), and the bare number says that it is no vertex, which the message's comment at :569 states ("A vertex outside the structure has no formula"). Every `VertexId` field of the other variants can be described against the structure; this one cannot. A break needs a gain a user can name, and this has none.

## F-3-10: candidate

Opened nets/essential.rs:131, nets/mod.rs:1011 and :1384, nets/sequentialize.rs:80-122, nets/mell/desequentialize.rs:97, nets/mell/graph.rs:451-465, nets/mell/mod.rs:694, nets/cut/mod.rs:191-265, core.md:327-336 and target/audit/wasm.md:106-141. The entries are generic over the stop with their bodies inline, against the crate's rule, and the hot-path rule of core-nets.md:55-62 is not touched. Whether the web module holds several copies is not measured.

Correction: `wasm.md`'s table of functions compiled more than once (down to 15 642 bytes beyond the largest copy) lists none of these, so each one's surplus is under 16 KB raw, and the whole gain is bounded well below the 129 KB the rule cites. To measure: the copies of these seven symbols in the `deduxit-web` module by twiggy, and the budget check's gzip bytes before and after. `is_essential` is 118 lines by clippy.md, not 133.

## F-3-11: confirmed

Opened target/audit/clippy.md (the seven rows), nets/cut/mell.rs:209-365 and nets/mell/sequentialize.rs:256-425. `reduce` has one arm per cut case with the four sub-arms named, and `stage` has the three phases named, so the seams are there. `too_many_lines` is set aside on purpose (Cargo.toml:95-100: a reviewer judges the cases that matter), which leaves room for a reviewer's case with seams; E5 speaks of files, and method.md:98-99 puts the long functions under it. The gain is readability alone.

Correction: The seams are named for four of the seven only. nets/mod.rs:530 is `NetError::write_over`, one arm per variant of the message, and mell/graph.rs:188 is `Graph::new`; the change names no seam for either, nor for search/essential.rs:285 (`new`, the set-up of the closure). `Symmetry::new` (search/net.rs:418) and that set-up belong to the search, so the net target set's counters must not move. Cost medium across `nets` and `search`.

## F-3-12: rejected

Opened Cargo.toml:95-100, criteria.md:140-147, baselines.md:207 and target/audit/clippy.md:6. `must_use_candidate` is one of the ten lints set aside on purpose ("they flag what this code means to do, and a reviewer judges the cases that matter"), not one of the 44 that only fire today. E7 admits a finding on a set-aside lint only if its reason no longer holds, and the finding gives none; core.md does not ask for it (no such sentence there). Marking 36 of 556 sites would not let the lint go to `warn`, and a dropped call to a pure getter is a no-op, not a silent bug.

## F-3-13: confirmed

Opened search/essential.rs:333-362. The two loops are the same but for the iterator (`ids().rev()` filtered to inputs, then `ids()` filtered to outputs); a `chain` of the two gives the same order of rows and the same polls of the stop, so the set-up and its charge are unchanged. Small gain at small cost.

## F-3-14: confirmed

Opened nets/mod.rs:1590-1606 and :2169-2177, and counted the inner attributes: eleven in nets/mod.rs at the lines given and four in nets/sequentialize.rs, all inside modules already under `cfg(all(test, feature = "parse"))`. The `In<'_, MellStructure>` impl stands after the test module, away from its sibling at :1590. No rule records either. No behaviour changes.

## F-3-15: confirmed

Opened search/net.rs:263-283. The `if if mix { … } else { … } { return false; }` is as quoted, and E3 names expression-oriented code with a `let` bound first. The rewrite keeps the count equation that core-nets.md says must not be relaxed. One site, a small gain.
