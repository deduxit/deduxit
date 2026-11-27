# Unit 6b: verdicts

16 confirmed, 1 candidate.

## F-6b-01: confirmed

Opened svg/mod.rs:1037-1175 and 1263-1271, svg/tree.rs:90-110 and 254, net.rs:399, mell.rs:397, dhat.md:15-17 and 169-180. `lay` takes its piece buffer with `mem::take` at :1124, so every piece starts a fresh String in `escape`; tree.rs:110 lays every conclusion through `run` for its width alone and :254 lays it again; the two net layouts call `lay` with an empty `emit`. DHAT's 46 165 of 52 499 blocks at `escape` and 1 473 at `Escaping::write_str` are as quoted, and render-svg stands at +1.12 % in ratchet.out:22. 583 blocks per million instructions is hot by P2's rule of thumb. No rules file records the owned pieces as a choice. The block counts are measured; the projected fall to about 6 000 blocks and the gain in instructions are not: kept pieces (the second pass of tree.rs, labels) still need text unless they are written straight out.

Correction: The witness line for the allocation is svg/mod.rs:1124 (`mem::take`), with DHAT's frame at :1045; the target of ~6 000 blocks and 'several per cent' are estimates to measure with DHAT and callgrind on render-svg.

## F-6b-02: confirmed

Opened svg/kripke.rs:130-176, svg/mod.rs:1810-1880, 2040-2059, 2100-2158, limits.rs:880-889, core.md:327-336, llvm-lines.txt:182-189, wasm.md:186. `kripke::draw` takes `impl FnMut() -> bool` (:138) and the three callers each write the closure that `limits::counting` is (the hand-written one adds 1 without saturation, which no run reaches). llvm-lines shows three instantiations of about 1 370 lines each with their closure. core.md:327 states the rule and does not list this function; `draw` already turns the stop into `&mut dyn` inside for `halt` (:167). No caller outside svg/mod.rs. The LLVM lines are measured, the wasm bytes are not (wasm.md:186 has `modal_countermodel` at 18 058 bytes with the drawing inlined).

## F-6b-03: confirmed

Opened svg/mell.rs:359-363 and the pass comments from :372 to :1735, llvm-lines.txt:4, core-export.md:610-629. The function runs from :363 to :1751 under an `#[expect(too_many_lines)]`, is first in llvm-lines with 12 983 lines, and its comment-headed passes are where the finding puts them (:470, :525, :567, :637, :811, :831, :908, :1005, :1027, :1194, :1420, :1464, :1599, :1671). E5 asks for the seam and the finding names them. The rules file describes the layout, not a reason to keep it one function; the `expect`'s reason is the only record. The gain is for a reader only, and the passes are closures over many shared locals, so the snapshots of nets with boxes are the whole net: tidiness at a real risk of a byte moving.

Correction: Cost: medium is the low end; the passes share a dozen locals through closures, so a struct needs most of them as fields.

## F-6b-04: confirmed

Opened svg/mell.rs:1694-1750 against svg/net.rs:782-838 with diff: the document tail differs in five hunks (names of two buffers, the dashed condition, the room for cut bars, one argument of `describe`); the `group` closure is word for word, and both files have the `write` closure (mell.rs:407, net.rs:406) and `describe`/`estimate`/`described_bytes` in step. core-export.md:492-499 and 610-629 record the two drawings as one look and give no reason for two copies. One set of emitters would serve both; the differences in the tail (cut room, dashed premises) must stay parameters, so `finish` takes more than the finding lists.

## F-6b-05: confirmed

Opened rocq/deduxit.rs:340-438, 550-566, 896-983. `formula` (:905-921) and `iformula` (:953-966) repeat the tables of `one_sided` (:403) and `two_sided` (:424) arm for arm, and the two walks differ only in the table and the `output` flag; `Fo::write_formula` (:352-355) already picks by `Option<&Reading>`. The only difference is that the inline tables are `unreachable!` for `Forall`/`Exists`; the shared tables name them, so the merged function loses that guard unless it keeps an assertion. The propositional output is unchanged, which the snapshots under core/tests/snapshots/deduxit/ and the `rocq-deduxit` check hold.

Correction: Keep the guard that a binder never reaches the propositional writer (a `debug_assert!` or a match before the table).

## F-6b-06: confirmed

Opened latex.rs:660-686 and :712-718, typst.rs:740-766, svg/mod.rs:1456-1493, core.md:327-336, dhat.md:164, 179, 190, wasm.md:106-186. Both writers pass `counting(stop)` of the caller's own closure into the generic `tree`, where `svg::write` shims into `drawn` with `&mut dyn`; core.md's list names `svg::write` and not these. The claim is measured for Typst: wasm.md:128 has `deduxit::export::typst::write` in 6 copies, 29 344 raw bytes beyond the largest (the largest 20 272, :175). `latex::write` is not in that list, so for LaTeX the claim rests on the DHAT frame names alone. The 29 KB is an upper bound and is raw, not gzip.

Correction: Not only a candidate for Typst: witness wasm.md:128 (6 copies, 29 344 bytes). `rocq::write` has the same shape (rocq/mod.rs:1028-1046; wasm.md:133, 4 copies, 22 046 bytes) and belongs to the same change. For LaTeX, measure llvm-lines and the module's bytes.

## F-6b-07: confirmed

Opened latex.rs:745-747, typst.rs:1062-1069, svg/tree.rs:249, dhat.md:164, 179, 190. The three quotes are in place and DHAT has 164 blocks from each closure (one byte each in LaTeX and Typst, 14 in SVG). In render-latex that is 164 of 3 042 blocks in 5.7 M instructions, about 29 per million, which P2's rule of thumb puts at a few per cent; in render-svg it is 0.3 % of the blocks and lost beside F-6b-01. The changes keep the bytes. The value is small, and real for LaTeX and Typst.

## F-6b-08: confirmed

Opened rocq/mod.rs:10-18, :186-199, the list of public items (`write_labelled` :1388, `write_modal_outcome` :1414, `write_modal_countermodel` :1447; `DEDUXIT_FIRST_ORDER_MODEL_PRELUDE` :340, `DEDUXIT_MODAL_PRELUDE` :359, `DEDUXIT_MODAL_COUNTERMODEL_PRELUDE` :367) and core-export.md:50-56 and 1082-1100. The table stops at the ordinary certificates and the prelude list names none of the three constants, against the rule that a new entry point joins the table. Text only.

## F-6b-09: confirmed

Opened export/mod.rs:4-59, latex.rs:20-47 and 270-296, the `use` lines of typst.rs (:24-25, :320-321) and svg/mod.rs (:1782-1784, :2077-2079). The module doc's first paragraph sends the reader to `linear::Proof::derivation`; every example of export/mod.rs, latex.rs and typst.rs imports `deduxit::linear` alone, while SVG has ordinary and modal examples. export/mod.rs:59 does list what `Drawable` takes, further down, which softens the claim for the root module but not for the two formats. T3 names D4.

## F-6b-10: confirmed

Opened latex.rs:526-559, core-export.md:938-963 and 1071-1081, and grepped typst.rs and svg/mod.rs for the model calls. LaTeX and Typst have `structure` and `kripke_structure` only; the recorded reason (:961-963) is about laying out a graph, and :1075-1078 later chose a table for the first-order models, which needs no graph package. The finding answers the recorded choice. It is a missing feature more than a broken criterion: D4 fits only in that modal logic and the propositional models lack what first-order ordinary logic has. A modal relation with loops and arrows both ways needs a column of the worlds each world sees, not 'above'.

Correction: Kind: an addition to the API, no break (four new functions). Cost: medium to large with the snapshots, the manual's format chapter, the guide and, if the command prints it, cli.

## F-6b-11: confirmed

Opened models.rs:96-160 and svg/mod.rs:1810-1862 and 2105-2139. The closing sentence is word for word in both (models.rs:107-120, svg/mod.rs:1849-1862). `models::sequent_bytes` (:132) is the estimate of svg/mod.rs:1823-1844 plus the binder arm; on a propositional sequent the binder list is empty, so it returns the same number and `svg::countermodel` multiplies by 12 as now. The modal copy (svg/mod.rs:2110-2131) walks `modal::Node`, another arena, and is not served by the same function.

Correction: The modal estimate stays where it is; the finding's change covers the ordinary pair only.

## F-6b-12: candidate

Opened rocq/deduxit.rs:550-566, 588-609, 690-729 and units/9/verdicts.md:71-73. `node` writes each named formula whole through `formula`/`iformula` for every `Tensor`, `Par`, `With`, `Bang`, `Copy` and `Weaken` node, so the text grows with depth times size, as F-9-18 (a candidate there too) says from the library's side. No rules file records the full writing as a choice. The gain is unproven: the inventory has the whole test file compile in 0.4 s. It changes every such certificate's text, so the snapshots and the `rocq-deduxit` check are the net. Measure file size and `rocq compile` time for `!A ⊢ A^⊗n`, n = 64, 128, 256, before and after.

Correction: As F-9-18's verdict: keep the sharing as `let` inside the term handed to the soundness theorem; no `Definition` before the lemma, since the statement is written out for a reader to check.

## F-6b-13: confirmed

Opened latex.rs:676-685, typst.rs:756-765, svg/mod.rs:1483-1492, export/mod.rs:190-218, rocq/mod.rs:1028-1046, core-export.md:110-118 and arch/verdicts.md:402-421. The three tree formats have the same match; F-arch-25's corrected change already shares the dispatch for LaTeX and Typst, so this adds the SVG copy. `judgement` and `rocq::write` are not copies: each arm there does something else (a term, a refusal, another writer), and the finding's own visitor leaves them out, so a new kind costs three arms, not one. The exhaustive match already makes the compiler name every site a new variant misses. The visitor needs a struct and an impl per format, which is more lines than the 27 it removes. Sound, and tidiness alone.

Correction: Narrowed: three identical matches, not five; merge with F-arch-25. The value 'one arm instead of five' is three instead of five, and 'no format left without it' is already the compiler's.

## F-6b-14: confirmed

Opened typst.rs:159-166, 173-181, 483-490, 790-798 and latex.rs:231-236, 397-404. The Typst escape of `"` and `\` is the same loop four times, twice into `W: Write + ?Sized` and twice into a `String`, which one function over `fmt::Write` serves. In LaTeX `label`'s `text` closure is `upright`.

Correction: `latex::upright` is `#[cfg(feature = "serialize")]` (:230); `label` is not, so the gate goes when `label` calls it.

## F-6b-15: confirmed

Opened net.rs:79 and :857, kripke.rs:342-343 and :645-646, tree.rs:147, Cargo.toml:133, clippy.md:37, Cargo.toml's `rust-version` (1.92). Six sites in export (two on net.rs:79), two of them beside an `i64::midpoint` on the next line. `i64::midpoint` is `(a + b) / 2` in a wider type, rounded towards zero, so the snapshots keep their bytes. E7 asks for a lint whose sites can all be fixed, and eight sites is that.

## F-6b-16: confirmed

Opened pdf.rs:156-211 and 233-293, krilla 0.8.2 src/interchange/metadata.rs:338-348, cli/src/show.rs:1298-1308, web-render/src/lib.rs:24, and grepped core-export.md for the date (only :457-458, `NoDate`). The fields are public with documented ranges, `Deserialize` is derived, `from_svg` passes them to krilla unchecked, and krilla clamps month, day, hour, minute and second. The command builds its date with `from_unix`, so the exposure is the library and the page's `styles.pdf.date`. The struct is `#[non_exhaustive]`, so outside code cannot write a literal but can set a field or deserialize one.

Correction: Kind: a new refusal at run time, no change of a signature; it is a break only for a caller that relied on the clamp. A range check still passes 30 February.

## F-6b-17: confirmed

Opened latex.rs:453-459, :475, :721, :785. The doc names `tree`; the parameter is `package: Option<&str>`, given as `Some("ebproof")`, `Some("fitch")` or `None`.
