# Unit 3 notes (finder)

## nets/mod.rs
- mod.rs:1562 Display runs is_correct(|_| false): full criterion + cycle witness (one deletion per edge) unbounded, inside Display. Witness cost recorded in core-nets.md as "for humans". Candidate: time of Display on big incorrect net.
- mod.rs:2171 `impl Display for In<'_, MellStructure>` sits after the tests module, apart from the other In impl at :1590.
- mod.rs:1598-1606 tests mod is `#[cfg(all(test, feature = "parse"))]` and inner items repeat `#[cfg(feature = "parse")]` (imports 1601,1603,1605; tests 1645,1653,1682,1787,1942,2000,2059,2099) — dead redundancy.
- mod.rs:275 NetError::NoVertex { vertex: u32, vertices: u32 } while every other variant names VertexId.
- mod.rs:1011 from_proof generic stop (impl FnMut) with ~60-line body; is_correct generic but delegates to dyn. Minor.
- mod.rs:1169 cut_partner linear scan over cuts: fine (cold).

## nets/graph.rs
- graph.rs:504 `parts.iter_mut().find(|(l, _)| *l == label)` per vertex: quadratic in the number of parts; MELL fixed the same (core-nets.md:428-433, "parts of a disconnection indexed by their root", panel 40 000 parts 3.4 s). MLL is_correct's Disconnected witness unpolled. Candidate measure: `|- a1, ~a1, ..., ak, ~ak` k=200k linked pairwise without Mix -> is_correct time.
- graph.rs:283-342 search/discover duplicate mell/graph.rs:511-? (duplicates.md 303 tokens, 116 tokens).
- graph.rs:294-295 search fills disc each round (recorded measured).

## nets/skeleton.rs
- fine. union returns bool ignored at build.

## nets/mell/mod.rs, mell/graph.rs
- mell/graph.rs:686-698 parts with an index table per root (the fix); MLL graph.rs:504 linear find -> quadratic. Same algorithm otherwise.
- mell/graph.rs:497-662 search/discover/present/deletable/acyclic/twin/cycle = graph.rs:283-470 nearly verbatim; Scratch twice (graph.rs:68 OccSet bitsets; mell/graph.rs:708 Vec<bool>). Difference in deletable: MLL reads forest.kind==Par and "slots.len()<3" (root Par has no parent slot); MELL has slot 0 always parent (NONE if absent) + `switched` flags. If MLL kept slot 0 for every vertex (NONE for a root), deletable is the same predicate.
- mell/mod.rs:815-824 verdict lines duplicate nets/mod.rs:1562-1578 (E6 "verdict line"); links sorting 803-808 = mod.rs:1532-1537; cut_partner/cuts/is_cut_free/partner/links accessors duplicate mod.rs:1158-1204 (duplicates.md 110 tokens).
- MellStructure::exist (assoc fn) vs nets::exist (free fn) for ProofStructure: two shapes of one question.
- mell/mod.rs:694 is_correct(impl FnMut) -> graph.correct(impl FnMut) generic ~45 lines (core.md wants &mut dyn for long bodies). minor.

## nets/sequentialize.rs vs mell/sequentialize.rs
- sequentialize.rs:261-281 Mix branch: labels.contains per conclusion and of(&gamma) per label: O(parts*|gamma|) in one stage, stop asked once per stage. MELL (mell/sequentialize.rs:317-340) sorts labels, recorded fix (core-nets.md:428-433 "Mix's parts sorted by label"; stage of 320 000 roots 3.7 s). MLL kept the quadratic one.
- step loop sequentialize.rs:177-219 = mell/sequentialize.rs:187-236 (duplicates.md 138 tokens): Sequentialization{nodes,steps,proved}, push, premise, Mix/Tensor/Cut arms.

## desequentialize, cut/, essential
- mell/desequentialize.rs: fine; take() uses Vec::remove (O(len)) per take; open lists per occurrence; ok.
- cut/mod.rs:69 sealed trait Net, documented as sealed. ok. cut/mod.rs:344-426 MLL reduce; cut/mell.rs:200 MELL reduce (dup 108 tokens: Axiom case lines 368-383 vs 229-255).
- nets/essential.rs:131 is_essential(mut stop: impl FnMut) body 130 lines generic (clippy.md >100 lines) — core.md rule "a long body takes &mut dyn, public impl FnMut a one-line shim". callers: export/svg/mod.rs:1611 (&mut stop), tests. Also ProofStructure::is_correct (mod.rs:1384, 45 lines), from_proof (mod.rs:1011, 60 lines), sequentialize (sequentialize.rs:80), MellStructure::from_proof (desequentialize.rs:97), mell graph.correct (mell/graph.rs:451) all generic bodies. llvm-lines shows none of them in top list -> few copies; low value.
- essential.rs:142-145 clones whole structure for cut case to run is_correct without mix.

## front ends pick the net type by hand (3 places)
- cli/src/show.rs:1470-1494 net_of: exponentials -> Mell; else ProofStructure::from_proof, on Placement::Cut fall back to Mell.
- web/src/calls.rs:2293-2313 same.
- core/src/protocol/linear.rs:556-563 "linear.net": exponentials -> Mell, else ProofStructure::from_proof WITHOUT the fallback -> errors on a checked MLL proof that cuts an occurrence twice (core-nets.md:200-209; test a_cut_occurrence_twice_needs_a_structure_with_boxes).
- nets_exist written twice: cli/src/show.rs:1590-1598, web/src/calls.rs:1490-1496 (exponentials -> MellStructure::exist else nets::exist).

## search/net.rs, essential.rs, planar.rs
- net.rs:274-279 `if if mix { .. } else { .. } {` — nested if as condition.
- net.rs:1211-1213 test doc "rather than in every order" (text.md).
- search/essential.rs:337-362 two loops identical but for the filter/order (duplicates.md 103 tokens).
- planar.rs:274-295 vs 463-495 test helpers duplicate (test code; low).
- planar.rs:111 is_planar (debug) = ProofStructure::is_planar stack scan; debug only.
- net.rs:243-246 panics on non-refusal NetError after a passed exact test: recorded invariant.
- Correctness trait (net.rs:558) is crate-private; fine.

## either-net enums in front ends
- core/src/protocol/linear.rs:362 enum Net {Plain, Boxes}; cli/src/show.rs:1314 Net {Mll(Cow), Mell}; web/src/calls.rs:2242 Net {Mll, Mell} + NetRef 2251. Each reads a net document by probing `vertices` (protocol :380-386, web :2329-2331).

## NetError::Mode message
- mod.rs:627-630 "proof nets exist in linear mode only, with or without Mix, not in {mode} mode". Used by build() for a cyclic criterion with Mix (mode "cyclic with Mix") and for cut links under the cyclic order (mode "cyclic"): mod.rs:877-885; and MellStructure::from_parts cyclic (mell/mod.rs:402-404). Cyclic IS a linear mode -> wrong reason.

## lints
- clippy.md: must_use_candidate 36 in linear::nets; too_many_lines 7; cast_possible_truncation 84.

## perf (candidates)
- mell/graph.rs:385 doors_in / cuts_in scan all doors per box: O(boxes x doors) over a sequentialization; within the documented quadratic bound, polled per stage. low.

## equality of nets
- sorted links (x.min(y), x.max(y)) + sort written by hand: nets/mod.rs:1532 (Display), mell/mod.rs:803 (Display), export/svg/net.rs:337-342, export/svg/mell.rs:281-286, tests nets/mod.rs:1987, search/net.rs:1098, search/essential.rs:702, cut/tests.rs:298. ProofStructure has no PartialEq; docs promise "two proofs ... give the same net".

## paths
- linear/mod.rs:30-32 re-exports 8 net types; nets::{exist, Scratch, cut} only under nets; cli uses both paths.

## final selection drafted: 15 findings (see StructuredOutput). All 14 unit files read in full.
