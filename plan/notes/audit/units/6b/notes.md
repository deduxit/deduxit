# Unit 6b notes

## Step 1-2 (criteria, handovers, inventories)
- Handovers for 6b: F-arch-25 (latex/typst dup, verified narrowed: sequent, model tables, PER_OCCURRENCE, dispatch shareable; tree emitters differ), F-arch-22 (three features latex/typst/svg), F-arch-26 (Drawable closed; verifier: kind text), F-9-18 (certificate term restates subformulas, quadratic; mend a `let` in deduxit.rs), F-9-01 (names written as strings = contract).
- Labels::Table keyed by linear Named: latex.rs:647, typst.rs:726 doc says LK/LJ labels stay upright.
- clippy.md export column: match_bool 27 of 28 sites in export; cast_precision_loss 23/43; cast_possible_wrap 12/25; too_long_first_doc_paragraph 23; doc_markdown 22; elided_lifetimes_in_paths 12; similar_names 15; cast_possible_truncation 49.
- too_many_lines: svg/net.rs:374 (389 lines), svg/kripke.rs:130 (250), svg/tree.rs:45 (219), rocq/deduxit.rs:1009 (191), svg/fitch.rs:20 (118), svg/mod.rs:1945 (109), rocq/deduxit.rs:742 (101).
- duplicates.md: font.rs:311/316; deduxit.rs:425/953, 404/905, 918/963; mell.rs:1710/net.rs:797 (27 lines), mell.rs:277/net.rs:333, mell.rs:1689/net.rs:777; models.rs:140/svg/mod.rs:1823, models.rs:148/svg/mod.rs:2118.

## export/mod.rs, notation.rs, styles.rs, models.rs, latex.rs, typst.rs
- mod.rs:198 `judgement` reads term back under `Limits::default()` (by design for to_string; write has no limits param).
- latex.rs:453-455 doc of `begin` stale: "with `ebproof` loaded if `tree` is set" but param is `package: Option<&str>` (also loads `fitch`).
- latex.rs:676-685 / typst.rs:756-765 identical dispatch (F-arch-25). Check svg::write too.
- typst.rs: string escaping loop `if c == '"' || c == '\\' { push('\\') }` written 4 times: atom :160-165, upright :175-180, label text closure :483-490, fitch string :790-799.
- latex.rs: `\mathrm{` + escape loop twice: upright :231-237, label text :398-404.
- latex.rs:747 `buffer.push_str(&derivation.premises(id).len().to_string())` alloc per inference (render-latex journey); typst.rs:1064-1067 `"-1".to_owned()`, `"0".to_owned()`, `"none".to_owned()`, `.to_string()` per inference in own layout -> &str/write! would do (P2 candidate).
- models.rs:292-306 `domain`, `numbers` build Vec<String> then join (small, cold).
- latex/typst model_table: Vec<String> cells + join per world; cold.
- typst.rs:803-806 odd multi-line string literal "  open,\n".
- typst::Length: no AsRef<str>/Borrow; has as_str, Display, FromStr. OK.

## svg/mod.rs
- svg/mod.rs:1483-1492 third copy of the Drawable dispatch (latex.rs:676, typst.rs:756). svg::write has the &mut dyn shim (`drawn`), latex::write / typst::write do not (generic tree per caller closure) -> P4 candidate; llvm-lines top 200 has no latex/typst tree entry, so unmeasured.
- Sequent byte estimate over ordinary arena three times: models.rs:132-160 `sequent_bytes`; svg/mod.rs:1822-1846 (countermodel, inline); svg/mod.rs:2110-2132 (modal, over modal arena). duplicates.md: models.rs:140<->svg/mod.rs:1823, models.rs:148<->svg/mod.rs:2118.
- `last` sentence of an ordinary Kripke countermodel: models.rs:107-120 == svg/mod.rs:1849-1862 verbatim.
- polls closure FnMut(Progress)->FnMut()->bool hand-written 3 times: svg/mod.rs:1863-1879, 2042-2058, 2140-2156; `limits::counting(stop, Phase::Write)` (limits.rs:880) does exactly that and is used by write fns. llvm-lines: kripke::draw instantiated 3 times (lines 182-189 of llvm-lines.txt: 1369+1369+1365 and closure#14 1372x3 = ~8216 lines); one instantiation via &mut dyn would save ~5.5k LLVM lines.
- match_bool: 27/28 workspace sites in export (svg/mod.rs:1752, mell.rs x ~17, net.rs ~...). E7: fix -> lint warn.
- Escaping (svg/mod.rs:1263-1271) allocates a String per write_str: used by tree.rs:209 write_steps (desc of every derivation: render-svg journey) and fitch.rs:96. P2 candidate: escape char-by-char into W directly (write_char) without temp String.
- Style: 27 pub fields, colors String (escaped at use). by design.
- refused(): fine.
- `text` fn at svg/mod.rs:1183: writeln!(...).unwrap() on String: fine.

## font.rs, tree.rs, fitch.rs + DHAT
- DHAT render-svg (dhat.md:169-180): 52 499 blocks, 46 165 (87 %) in `svg::escape (mod.rs:1045)` = pushes into the per-piece String that `lay`'s `close` hands away by `std::mem::take(piece)` (svg/mod.rs:1124) -> one allocation per piece per layout; 1 637 blocks / 5.7 MB `RawVec<Piece>::grow_one` (run's Vec without capacity, mod.rs:1090); 1 473 blocks `Escaping::write_str` (mod.rs:1267, a String per call). render-latex 3 042 blocks, typst 3 046. ratchet.out:22 render-svg 79.2M instr vs latex 5.7M, typst 4.7M.
- tree.rs:110 first pass `conclusion(&mut sequent, id).width` builds a whole Run (pieces with owned Strings) to read its width alone; tree.rs:254 lays it out again to write. `line()` (mod.rs:1318) already shows the width-only pattern with `lay(...,|_| pieces += 1)`, but lay still allocates per piece through mem::take.
- tree.rs:249 `format!(r#" id="i{}""#, id.get())` per inference; tree.rs:242 size string once (fine).
- fitch.rs:86 `lines.len() as u32 - 1` underflows on empty; check whether Fitch can be empty.
- fitch.rs:38 number(n) runs n.to_string; fine (cold).
- font.rs OK; ADVANCES u16 table; MathFont::column duplicate of index in ALL (test checks).
- Font family/advances: Font::euler() allocates family String each call (Style::default) - fine.

## kripke.rs, net.rs
- kripke.rs:130-139 draw takes `mut stop: impl FnMut() -> bool` -> 3 instantiations (llvm-lines). halt closure kripke.rs:167 already takes &mut dyn.
- kripke.rs:167-176 `halt` = same as notation::flush's stop check / fitch.rs:158 `stopped()` / tree.rs:105-109 inline Refusal::Stopped{phase: Write} -- the "stopped" error built in 6+ places in export (grep).
- kripke.rs:251 `format!("w\u{4}{w}\u{5}")` per world fine.
- kripke.rs:419-426 and 464-472 "worlds X and Y are above it" written twice in describe (contents vs plain branch).
- net.rs:399 `width = |text| lay(text,...,|_| {})`: lay allocates per piece even when discarded (mem::take) -> same P2 as tree.rs:110.
- net.rs:245-248 / mell.rs:163-166 `colour = match read {true => escaped(&style.input).len()+8, false => 0}` duplicate (dup list mell.rs:277/net.rs:333 etc).
- net.rs:607-614 seven Strings for groups, copied into document (recorded F5: "net's groups written from their five strings").
- net.rs:52-101 geometry helpers arrowhead/arrow_along/towards_by/towards: towards and towards_by near duplicates (towards = towards_by(from, to-from, by)).

## mell.rs
- mell.rs:359-1751 `draw` is ~1390 lines, one fn under #[expect(too_many_lines, reason="one layout in its passes...")]; largest fn of the crate in LLVM lines (12 983, llvm-lines.txt:4). Seams: layout tree (470-523), doors/rules levels (525-565), rows (567-635), extents (637-809), positions (811-829), lids/arcs (831-907), line height (908-958), geometry closures (959-1025), cycle (1027-1044), vertex emit (1088-1193), rules nodes (1194-1419), wires (1420-1598), boxes (1599-1634), links (1635-1669), cuts (1671-1693), body/doc (1694-1750).
- mell vs net duplicates: link arcs mell.rs:1635-1669 / net.rs:721-754; cut bars mell.rs:1671-1693 / net.rs:756-781; group+body mell.rs:1694-1723 / net.rs:782-810; title mell.rs:1724-1729 / net.rs:811-816; description mell.rs:1745-1749 / net.rs:831-835; estimate colour mell.rs:163-166 / net.rs:245-248; per-occurrence chars/duals bottom-up mell.rs:172-194 / net.rs:255-285; tensor/par formula extent mell.rs:720-729 / net.rs:458-479; literal extent mell.rs:705-713 / net.rs:447-454; write closures mell.rs:407-413 / net.rs:406-412; symbol closures; describe's link loop mell.rs:279-290 / net.rs:335-346.
- mell.rs:1075 `format!(r#" font-size="{label_size}"{}"#, fill(v))` per label; 1080 circle id format; 1092 id(v) format per vertex. cold-ish (no journey on MELL nets).
- mell.rs:1749 `describe(...).unwrap()` writing into String fine.

## png.rs, pdf.rs
- png.rs/pdf.rs: Refusal::Memory{phase: Render,...} built twice (png.rs:155-163, pdf.rs:246-252) after `measure.estimate(...)`: could be one `Measure::within(&costs, pixels, limits)` in mod.rs. small E6.
- pdf::Date (pdf.rs:157-173): pub fields, serde Deserialize derived, no validation; docs say "month, 1 to 12", "day 1 to 31"; krilla 0.8.2 `DateTime::month` clamps silently (metadata.rs:340-342) -> a JSON date {"month":13} writes December without error. C-VALIDATE; low value.
- png.rs:179 `scale as f32` fine.

## rocq/mod.rs
- rocq/mod.rs:10-18 module table lists 7 calls; misses write_labelled (:1388), write_modal_outcome (:1414), write_modal_countermodel (:1447) -- the modal suite's certificates; rules (core-export.md:50-56) say every entry point joins the table. Options::prelude doc (:186-199) lists preludes but omits DEDUXIT_MODAL_PRELUDE, DEDUXIT_MODAL_COUNTERMODEL_PRELUDE, DEDUXIT_FIRST_ORDER_MODEL_PRELUDE. D4/T3.
- export/mod.rs:6-10 and every format's example start from linear `Proof::derivation`; latex/typst/rocq examples linear only; svg has ordinary + modal countermodel examples. D4 text.
- rocq/mod.rs:1028-1046 fourth Drawable dispatch (different: Classical -> view.derivation()).
- rocq/mod.rs: ten pub prelude constants + NANOYALLA, DEDUXIT version consts; surface.md: 0-2 uses outside core.
- rocq/mod.rs:819-828 `apply` builds `format!` then line(): alloc per tactic; nanoyalla cold.
- write_disproof takes no stop/limits; others do. by design? (refutation small).
- Unsupported messages long (LargeModel 1 sentence of 40 words). T1 maybe.

## rocq/deduxit.rs
- deduxit.rs:896-938 `formula` repeats `one_sided` (403-420) inline at 905-921; `iformula` 943-983 repeats `two_sided` (424-438) at 953-966; and formula/iformula are the same walk (dup list 404/905, 425/953, 918/963).
- deduxit.rs:590-604 `node` writes every child formula in full at each node (F-9-18 quadratic text); candidate.
- deduxit.rs:886-892 `atom` -> `string(name)` allocs format+replace per atom written (2 allocs); candidate, no journey.
- deduxit.rs:111-115 and 1067-1071 (atom, module) pair twice.

## Other checks
- No std HashMap/HashSet in export (crate::hash used); VecDeque kripke.rs:535, BTreeMap font.rs (public Advances::Table): nothing to report.
- Refusal::Stopped{phase: Write} built 5x in export, 103x crate-wide; fitch.rs:158, modal/rocq.rs:77, rocq_models.rs:98, lambda/mod.rs:1400 each define `stopped()` -> unit 6a / arch.
- manual_midpoint: export sites net.rs:79 (x2), net.rs:857 (test), kripke.rs:342, 645, tree.rs:147; kripke.rs:343/619/646 already use i64::midpoint.
- DHAT render-latex: 164 blocks / 164 bytes from latex.rs closure (= `premises.len().to_string()` at latex.rs:747); render-typst same from typst.rs (own layout :1064-1067); render-svg 164 blocks / 2296 bytes from tree.rs closure#11 (= format! id at tree.rs:249).
- ratchet.out:22 render-svg 79 169 276 vs ceiling 78 295 779 (+1.12 %) -- over its ceiling.
- latex/typst have tables for FO countermodels (latex.rs:526,552) but none for propositional Kripke (svg::countermodel) or modal ones; core-export.md:961-963 reason "no package lays out such a graph" vs core-export.md:1075-1080 tables for FO.
