# Unit 6b: findings

Read in full: 17 files. Not read in full: none.

The finder's remarks: I read all 17 files of the unit and confirm F-arch-25 as its verifier narrowed it, with the dispatch in five places (F-6b-13), and F-arch-22 and F-arch-26 from the code: the three text formats share `models.rs`, `printed` and `math`, and `Drawable` is the only door. For handovers.md's question on labels: the LaTeX, Typst and SVG trees label a rule by `Drawn::markup`, so a user's `Labels::Table` reaches only linear rules (the docs say so at latex.rs:647, typst.rs:726 and svg/mod.rs:1441), and rule files and lambda drawings ignore it. I did not measure the instruction effect of F-6b-01, F-6b-06 or F-6b-07, or the wasm bytes of F-6b-02 and F-6b-06. The leader should know that `ratchet.out:22` shows render-svg at 79 169 276 instructions against a ceiling of 78 295 779 (+1.12 %), so it is already over. For unit 6a or arch: `Refusal::Stopped { phase }` is built by hand at 103 sites across the crate, and four modules define their own `stopped()` helper (svg/fitch.rs:158, modal/rocq.rs:77, ordinary/rocq_models.rs:98, lambda/mod.rs:1400); one constructor on `Refusal` would serve all of them. For unit 9 (the Q8 contract): the theorem names the exporter writes as strings sit in deduxit.rs:154 and :163-185, :1128-1206, :1282 and :1294.

## F-6b-01

- dimension: performance
- criterion: P2
- place: core/src/export/svg/mod.rs:1120; core/src/export/svg/tree.rs:110
- kind: refactor
- cost: small
- candidate: no

**Claim.** The SVG layout allocates a fresh String for every piece of text it lays out, even where only the width is wanted, and these allocations are 87 % of the render-svg journey's heap blocks.

**Witness.** `dhat.md:169-176` (render-svg): 52 499 blocks in all; `deduxit::export::svg::escape (mod.rs:1045)` 46 165 blocks (87 %), `RawVec<Piece>::grow_one` 1 637 blocks / 5.7 MB, `Escaping::write_str (mod.rs:1267)` 1 473 blocks. render-latex has 3 042 blocks, render-typst 3 046.

```rust
// svg/mod.rs:1120-1124
    let mut close = |piece: &mut String, shape: (i64, i64), formula, start, end| {
        piece.truncate(piece.trim_end_matches(' ').len());
        if !piece.is_empty() {
            emit(Piece {
                text: std::mem::take(piece),
```

`mem::take` leaves an empty String, so the next `escape` push allocates again. The width-only callers still pay it:

```rust
// svg/tree.rs:110
        let width = conclusion(&mut sequent, id).width;
```

(`conclusion` calls `run`, which collects every `Piece` with its owned text; net.rs:399 and mell.rs:397 call `lay(text, 1000, &style.font, |_| {})` for widths alone.) `Escaping::write_str` (mod.rs:1267) builds a `String` per call: `let mut text = String::with_capacity(s.len());`.

**Change.** Let `lay` keep one piece buffer and hand `emit` a borrowed piece (`&str` and its numbers), clearing the buffer instead of taking it; `run` copies only where pieces are kept (labels, the dots). Give the width passes a width-only call that never builds text, and let `Escaping` escape character by character into the writer.

**Value.** Every user who draws a proof: the command's `--format svg`, PNG and PDF, and the web page, which redraws after every interactive step. The journey's blocks should fall from ~52 000 towards ~6 000; by the perf book's rule of thumb that is worth several per cent of instructions, to be confirmed.

**Net.** The export snapshots (`core/tests/export.rs`, byte for byte) and the flake's `export` check; the ratchet's render-svg (re-run DHAT for blocks, callgrind for instructions; render-svg stands at +1.12 % over its ceiling in `ratchet.out:22`). No checker, search or certificate is touched.

## F-6b-02

- dimension: performance
- criterion: P4
- place: core/src/export/svg/kripke.rs:138; core/src/export/svg/mod.rs:1863, 2042, 2140
- kind: refactor
- cost: small
- candidate: no

**Claim.** `kripke::draw` takes its stop as `impl FnMut() -> bool`, and its three callers each hand-write their own polling closure, so the drawing is compiled three times where `limits::counting` over a `&mut dyn` stop would compile it once.

**Witness.** `llvm-lines.txt:182-189`: `export::svg::kripke::draw::<…countermodel…>` 1 369 lines, `::<…modal_countermodel…>` 1 369, `::<…first_order_model…>` 1 365, and each one's `{closure#14}` 1 372: about 8 200 LLVM lines for one function.

```rust
// svg/mod.rs:1863-1879 (repeated at 2042-2058 and 2140-2156)
    let mut polls = 0u64;
    kripke::draw(
        &countermodel.model,
        None,
        kripke::Arrows::Order,
        &title,
        &last,
        style,
        limits,
        || {
            polls += 1;
            stop(crate::limits::Progress::new(
                crate::limits::Phase::Write,
                1,
                polls,
            ))
        },
    )
```

`crate::limits::counting(stop, Phase::Write)` (`limits.rs:880`) is exactly this closure, and `svg::write` already passes the stop as `&mut dyn` (`svg/mod.rs:1468-1472`).

**Change.** Make `kripke::draw` take `stop: &mut dyn FnMut() -> bool` (or `&mut dyn FnMut(Progress) -> bool` and call `counting` inside), and drop the three hand-written closures, as `core.md` asks of every long body.

**Value.** The web module and the command: about 5 500 LLVM lines fewer (two copies of 2 740), a few KB of wasm to be measured with twiggy; a reader sees one polling idiom.

**Net.** The snapshots `kripke_*.svg` and `kripke_fo_*`, `structure_*` and the `export` check; the web module's budget (`budget` check) for the bytes. No checker, search or certificate is touched.

## F-6b-03

- dimension: architecture
- criterion: E5
- place: core/src/export/svg/mell.rs:363
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `mell::draw` is one function of about 1 390 lines, the largest function of the crate by LLVM lines, holding a dozen separate layout passes that share state only through local vectors.

**Witness.** ```rust
// svg/mell.rs:359-363
#[expect(
    clippy::too_many_lines,
    reason = "one layout in its passes, as the multiplicative drawing is"
)]
pub(super) fn draw(
```

The body runs to line 1751. `llvm-lines.txt:4`: `export::svg::mell::draw` 12 983 lines, first of the crate. Its seams, each a comment-headed pass: the layout tree (470-523), doors and the rules' levels (525-565), rows (567-635), extents (637-809), positions (811-829), lids and link rises (831-907), the raised line height (908-958), the doors' points (1005-1025), the cycle (1027-1044), vertices (1088-1193), the rules' nodes (1194-1419), wires (1420-1463), legs and door wires (1464-1598), boxes (1599-1634), links (1635-1669), cut bars (1671-1693), the document (1694-1750).

**Change.** Make the layout a struct (`Layout` with `x`, `row`, `extent`, `offset`, `doors`, `point`, `legs`) built by the first passes, and each emitting pass a method on it writing into the shared group buffers of F-6b-04; the reason in the `#[expect]` then no longer holds.

**Value.** A reader of the code, who today must hold 1 390 lines of closures over shared locals to change one pass; each pass becomes testable alone.

**Net.** The export snapshots of nets with boxes (`net-promotion.svg`, `essential-boxes*`, `net-cut-boxes.svg`), `net_looks`, `edges_keep_out_of_boxes`, `cut_elimination_is_drawn`, and the `export` check's rendering. No checker, search or certificate is touched.

## F-6b-04

- dimension: architecture
- criterion: E6
- place: core/src/export/svg/mell.rs:1694 and core/src/export/svg/net.rs:782
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The drawings of nets with and without boxes repeat the same emitters: the group buffers and their document body, the link arcs, the cut bars, the title, the description and the per-occurrence size estimate.

**Witness.** `duplicates.md`: `mell.rs:1710` ↔ `net.rs:797` (27 lines), `mell.rs:277` ↔ `net.rs:333`, `mell.rs:1689` ↔ `net.rs:777`. Word for word:

```rust
// mell.rs:1694-1703 and net.rs:782-791
    let group = |out: &mut String, colour: &str, extra: &str, content: &str| {
        writeln!(
            out,
            "<g fill=\"none\" stroke=\"{}\" stroke-width=\"{stroke}\"{extra}>",
            escaped(colour)
        )
        .unwrap();
        out.push_str(content);
        out.push_str("</g>\n");
    };
```

Also: link arcs `mell.rs:1635-1669` / `net.rs:721-754`; cut bars `mell.rs:1671-1693` / `net.rs:756-781`; title `mell.rs:1724-1729` / `net.rs:811-816`; description `mell.rs:1745-1749` / `net.rs:831-835`; the input colour's bytes `mell.rs:163-166` / `net.rs:245-248`; the chars-and-duals pass `mell.rs:172-194` / `net.rs:255-285`; a `⊗`/`⅋` formula's extent `mell.rs:720-729` / `net.rs:458-479`; the `write` closure `mell.rs:407-413` / `net.rs:406-412`; the links loop of `describe` `mell.rs:279-290` / `net.rs:335-346`.

**Change.** Move the shared emitters into `svg/net.rs` (or a `svg/emit.rs`): a `Groups` struct holding the seven buffers with `finish(style, title, description, size)`, and functions `link_arc`, `cut_bar`, `net_title`, `formula_extent`; both drawings call them.

**Value.** A reader, and the next change to a net's look, which today must be made twice (the textbook look, the essential drawing and cut links each touched both files).

**Net.** The net snapshots (`net*.svg`, `essential*`, `planar.svg`, `lambek-net.svg`), `net_limit` (estimate within four times), `cut_elimination_is_drawn`, and the `export` check. No checker, search or certificate is touched.

## F-6b-05

- dimension: architecture
- criterion: E6
- place: core/src/export/rocq/deduxit.rs:896 and core/src/export/rocq/deduxit.rs:943
- kind: refactor
- cost: small
- candidate: no

**Claim.** The propositional formula writers `formula` and `iformula` are one walk written twice, and each repeats inline the constructor table that `one_sided` and `two_sided` in the same file already hold.

**Witness.** `duplicates.md`: `deduxit.rs:404` ↔ `:905`, `:425` ↔ `:953`, `:918` ↔ `:963`.

```rust
// deduxit.rs:403-407
const fn one_sided(t: Term) -> &'static str {
    match t {
        Term::Atom(_) => "var ",
        Term::DualAtom(_) => "covar ",
        Term::One => "one",
```

```rust
// deduxit.rs:905-908, inside `formula`
                let name = match t {
                    Term::Atom(_) => "var ",
                    Term::DualAtom(_) => "covar ",
                    Term::One => "one",
```

The same holds for `two_sided` (`:424-438`) and `iformula`'s table (`:953-966`). `Fo::write_formula` (`:352-355`) already picks between the two tables with an `Option<&Reading>`.

**Change.** One `fn formula(forest, reading: Option<&Reading>, out, o, argument)` that calls `one_sided(t)` or `two_sided(t, output)`, as `Fo::write_formula` does; keep the propositional atom writer, since the rules keep that text byte for byte.

**Value.** A reader, and a change to the library's constructors, which today has three tables to keep in step with the Rocq library (Q8).

**Net.** The certificate snapshots `core/tests/snapshots/deduxit/` (byte for byte) and the flake's `rocq-deduxit` check, which compiles them. The certificates' text must not change.

## F-6b-06

- dimension: performance
- criterion: P4
- place: core/src/export/latex.rs:660; core/src/export/typst.rs:740
- kind: refactor
- cost: small
- candidate: yes

**Claim.** `latex::write` and `typst::write` pass the caller's own stop closure down into their generic tree writers, so each caller compiles its own copy of every tree emitter, where `svg::write` turns the stop into `&mut dyn` first.

**Witness.** DHAT frame names show the instantiation:

```text
// dhat.md:164 (render-latex)
{closure#0}<…Derivation, alloc::string::String, deduxit::limits::counting::{closure_env#0}<deduxit_bench::journeys::render::{closure#0}::{closure#0}::{closure_env#0}>> (latex.rs:
// dhat.md:176 (render-svg)
{closure#11}<…Derivation, alloc::string::String, deduxit::limits::counting::{closure_env#0}<&mut dyn core::ops::function::FnMut<(deduxit::limits::Progress), Output=bool>>> (tree.
```

```rust
// svg/mod.rs:1456-1463
pub fn write<'a>(
    derivation: impl Into<Drawable<'a>>,
    style: &Style,
    out: &mut impl Write,
    mut stop: impl FnMut(crate::limits::Progress) -> bool,
) -> Result<(), Error> {
    drawn(derivation.into(), style, out, &mut stop)
}
```

`latex.rs:666-685` and `typst.rs:746-765` have no such shim; `core.md` ("A long body takes the stop as `&mut dyn FnMut`") names `svg::write` and not these two.

**Change.** Give `latex::write` and `typst::write` the same one-line shim into a private `drawn(drawable, options, out, &mut dyn FnMut(Progress) -> bool)`.

**Value.** The web module and the command, which call the writers from several closures (session, protocol, CLI): fewer copies of five `tree` instantiations per format. Measure the wasm module's bytes (twiggy, `budget`) and `cargo llvm-lines` before and after; the render journeys should not move.

**Net.** The export snapshots and the `export` check; ratchet's render-latex and render-typst; the `budget` check for bytes. No checker, search or certificate is touched.

## F-6b-07

- dimension: performance
- criterion: P2
- place: core/src/export/latex.rs:747; core/src/export/typst.rs:1064; core/src/export/svg/tree.rs:249
- kind: refactor
- cost: small
- candidate: no

**Claim.** Each tree writer allocates a small string per inference for a number it could write into its buffer.

**Witness.** `dhat.md:164`: render-latex, 164 blocks of 164 bytes in all (one-byte strings, 5 % of the journey's blocks) from the closure in `latex.rs`; `dhat.md:190`: render-typst, the same 164 blocks from `typst.rs`; `dhat.md:176`: render-svg, 164 blocks of 2 296 bytes from `tree.rs`'s closure.

```rust
// latex.rs:746-747
                buffer.push_str("\\infer");
                buffer.push_str(&derivation.premises(id).len().to_string());
```

```rust
// typst.rs:1064-1067
            _ if derivation.is_bare(id) => ("-1".to_owned(), None),
            (true, OpenGoal::Mark(mark)) => ("0".to_owned(), Some(mark.as_str())),
            (true, _) => ("none".to_owned(), None),
            _ => (derivation.premises(id).len().to_string(), markup.as_deref()),
```

```rust
// svg/tree.rs:249
        let attributes = format!(r#" id="i{}""#, id.get());
```

**Change.** `write!(buffer, "\\infer{}", n)`; in Typst a `Cow<'static, str>` or a `write!` of the count; in SVG keep one `attributes` buffer cleared per inference.

**Value.** Every export of a derivation: one allocation fewer per inference in all three tree writers (5 % of render-latex's and render-typst's blocks).

**Net.** The export snapshots (byte for byte) and the ratchet's three render journeys; DHAT for the block counts. No checker, search or certificate is touched.

## F-6b-08

- dimension: text
- criterion: D4
- place: core/src/export/rocq/mod.rs:10
- kind: text
- cost: small
- candidate: no

**Claim.** The Rocq module's table of calls and the docs of `Options::prelude` leave out the modal suite's three certificates and their preludes, so a reader of the module's head finds no way to certify a modal proof or countermodel.

**Witness.** ```rust
// rocq/mod.rs:10-18
//! | to get | call |
//! |---|---|
//! | the certificate of a proof | [`to_string`] |
//! | the same, written into your own buffer, with a stop | [`write()`] |
//! | a certificate that a sequent has no proof | [`write_disproof`] |
//! | a certificate that an ordinary sequent is not valid | [`write_not_valid`] |
//! | the same, from a Kripke countermodel | [`write_countermodel`] |
//! | the same at first order, from a finite structure | [`write_structure`] |
//! | the same at first order, from a Kripke model with growing domains | [`write_kripke_structure`] |
```

`write_labelled` (`:1388`), `write_modal_outcome` (`:1414`) and `write_modal_countermodel` (`:1447`) are missing; `Options::prelude` (`:186-199`) lists the preludes but not `DEDUXIT_MODAL_PRELUDE`, `DEDUXIT_MODAL_COUNTERMODEL_PRELUDE` or `DEDUXIT_FIRST_ORDER_MODEL_PRELUDE`. `core-export.md:50-56`: "A new format or entry point joins both tables".

**Change.** Add three rows (a valid modal sequent's certificate, a modal outcome's, a modal countermodel's, with "cargo feature `serialize`"), and the three preludes to the list on `Options::prelude`.

**Value.** A user of modal logic, who reads the module's head to learn what can be certified.

**Net.** `cargo test --doc` and the `doc` check (links resolve). Nothing else.

## F-6b-09

- dimension: text
- criterion: T3
- place: core/src/export/mod.rs:6; core/src/export/latex.rs:22; core/src/export/typst.rs:22
- kind: text
- cost: small
- candidate: no

**Claim.** The exports' module docs tell a reader to start from a linear proof, and the LaTeX and Typst modules show no example for any other suite, though every format takes a derivation of LK or LJ, a rule file's drawing and a typing derivation.

**Witness.** ```rust
// export/mod.rs:6-10
//! Each format is a module with one options value: start from its
//! `default()` and set the fields you want to change. To export a proof,
//! take its derivation with
//! [`Proof::derivation`](crate::linear::Proof::derivation) and hand it to
//! the format:
```

The examples of `export/mod.rs:31-41`, `latex.rs:24-46`, `latex.rs:280-295`, `typst.rs:24-49`, `typst.rs:320-327` all start from `deduxit::linear`. The SVG and Rocq modules do have ordinary and modal examples (`svg/mod.rs:1782`, `:2077`; `rocq/mod.rs:1199`, `:1264`).

**Change.** Say "take a derivation of any suite (a linear `Proof::derivation`, an `ordinary::Derivation`, a `rules::Drawing`, a `lambda::Drawing`)" in the first paragraph, and add to the LaTeX and Typst modules one example of an LJ derivation from `ordinary::decide`, in the page's order of the suites.

**Value.** A user of ordinary logic (the crate's examples lead with it) who wants a LaTeX or Typst tree of an LK or LJ proof.

**Net.** `cargo test --doc` runs the new examples; the `doc` check.

## F-6b-10

- dimension: api
- criterion: D4
- place: core/src/export/latex.rs:526; core/src/export/typst.rs:596
- kind: API break
- cost: medium
- candidate: no

**Claim.** LaTeX and Typst write a first-order countermodel as a table but have no call for a propositional Kripke countermodel or a modal one, and the reason the rules give for that no longer holds.

**Witness.** `latex::structure` and `latex::kripke_structure` (`latex.rs:526`, `:552`, and the same in `typst.rs:596`, `:622`) write a `tabular` / `#table` with a row per world (world, above, domain, functions, facts). For `ordinary::Countermodel` and `modal::Countermodel` only `svg::countermodel` and `svg::modal_countermodel` exist. The reason recorded, `core-export.md:961-963`:

```text
  the `export` check renders. LaTeX and Typst draw no model: neither has
  a standard package that lays out such a graph without hand placement
  (TikZ needs coordinates, which would duplicate this layout in TeX).
```

yet `core-export.md:1075-1080` chose a table, which needs no graph package, for the first-order models.

**Change.** Add `latex::countermodel`, `latex::modal_countermodel` and their Typst twins (additive, no break): `models::Laid` from a `kripke::Model` with a `forces` column in place of domain and functions, the model's world in bold, checked first as the SVG calls are; or record why the table does not serve them.

**Value.** Users of intuitionistic and modal logic writing papers or teaching: their countermodels reach a LaTeX or Typst document as the first-order ones do. The command's `--format latex` on a not-valid verdict could then print the model.

**Net.** New snapshots `kripke_*.tex`, `.typ`, `kripke_modal.*` compiled by the `export` check; the guide and the manual's format chapter change with it. No checker, search or certificate is touched.

## F-6b-11

- dimension: architecture
- criterion: E6
- place: core/src/export/models.rs:107 and core/src/export/svg/mod.rs:1849
- kind: refactor
- cost: small
- candidate: no

**Claim.** The ordinary countermodel's closing sentence and its sequent-size estimate are written twice, in `models.rs` and in `svg::countermodel`.

**Witness.** Word for word, `models.rs:107-120` and `svg/mod.rs:1849-1862`:

```rust
    let last = match (logic, sequent.right().is_empty()) {
        (Logic::Classical, _) => format!(
            "World {w} makes every formula left of ⊢ true and every formula right of it \
             false, so {sequent} is not valid in classical logic."
        ),
```

and the bottom-up estimate over the ordinary arena, `models.rs:132-160` (`sequent_bytes`) and `svg/mod.rs:1822-1846` (`duplicates.md`: `models.rs:140` ↔ `svg/mod.rs:1823`; the modal copy `svg/mod.rs:2110-2132` ↔ `models.rs:148`).

**Change.** Make `models::sequent_bytes` and a `models::kripke_last(logic, sequent, w)` `pub(super)` and call them from `svg::countermodel` (its estimate is `sequent_bytes(sequent) * 12`).

**Value.** A reader, and the next change of a description's wording, which today drifts between the propositional and the first-order drawing.

**Net.** The snapshots `kripke_independence.svg`, `kripke_minimal.svg`, `kripke_fo_*`, `structure_*` and the `export` check. No checker, search or certificate is touched.

## F-6b-12

- dimension: performance
- criterion: Q10
- place: core/src/export/rocq/deduxit.rs:590
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** The term of a certificate for deduxit's library writes every node's subformulas out in full, so its text grows with the formulas' depth times the derivation's size (F-9-18 from the exporter's side).

**Witness.** ```rust
// deduxit.rs:596-604
        self.out.push_str(constructor);
        for &f in formulas {
            self.out.push(' ');
            if two_sided {
                self.iformula(f, 0, true);
            } else {
                self.formula(f, 0, true);
            }
        }
```

called for every `Tensor`, `Par`, `With`, `Bang`, `Copy` node (`:698-722`) with the principal formula's children, each written whole. Unit 9: `tests/FromCommand.v`, 192 nodes for `!A ⊢ A ⊗ … ⊗ A` (64 copies), 44 317 bytes.

**Change.** Bind each compound subformula the term names once, by a `let` inside the proof term or a local `Definition` before the lemma, written bottom-up over the forest; the library's statements and checker stay.

**Value.** Users who certify proofs of deep formulas: smaller certificates that Rocq reads faster, if the measurement confirms it.

**Net.** Measure first: file size and `rocq compile` time of `!A ⊢ A^⊗n`, n = 64, 128, 256, before and after, on pinned cores. It changes every certificate's text: the snapshots `core/tests/snapshots/deduxit/` and the flake's `rocq-deduxit` check.

## F-6b-13

- dimension: architecture
- criterion: E6
- place: core/src/export/latex.rs:676; core/src/export/typst.rs:756; core/src/export/svg/mod.rs:1483; core/src/export/rocq/mod.rs:1035
- kind: refactor
- cost: small
- candidate: no

**Claim.** The match from `Drawable` to a format's generic writer is written in four writers and in `judgement`, so a new kind of derivation adds an arm in five places.

**Witness.** ```rust
// latex.rs:676-685 (typst.rs:756-765 and svg/mod.rs:1483-1492 the same, `tree::draw` in SVG)
    match drawable {
        Drawable::Linear(derivation) => tree(derivation, options, judgement, out, stop),
        Drawable::Ordinary(derivation) => tree(derivation, options, judgement, out, stop),
        Drawable::Classical(view) => tree(view, options, judgement, out, stop),
        #[cfg(feature = "serialize")]
        Drawable::Rules(drawing) => tree(drawing, options, judgement, out, stop),
        #[cfg(feature = "serialize")]
        Drawable::Fitch(proof) => fitch(proof, options, out, stop),
        Drawable::Lambda(drawing) => tree(drawing, options, judgement, out, stop),
    }
```

F-arch-25 found the LaTeX/Typst pair; the SVG copy and `export/mod.rs:199-214` (`judgement`) and `rocq/mod.rs:1035-1045` make five. `core-export.md:116-118` records "an arm in each `write`".

**Change.** A crate-private visitor in `export/mod.rs`, `trait Emit { type Out; fn tree<T: Drawn>(self, &T) -> Self::Out; fn fitch(self, &rules::Fitch) -> Self::Out; }` with `Drawable::emit(self, e)`, used by the three tree formats; this also prepares F-arch-26's `Drawable::Other`.

**Value.** The author of the next drawable kind (a modal derivation, a natural-deduction view): one arm instead of five, and no format left without it.

**Net.** The export snapshots and the `export` check; the render journeys (the visitor is monomorphised as the match is, so they should not move). No checker, search or certificate is touched.

## F-6b-14

- dimension: idiom
- criterion: E6
- place: core/src/export/typst.rs:160; core/src/export/latex.rs:231
- kind: refactor
- cost: small
- candidate: no

**Claim.** The Typst string escape is written four times and the LaTeX upright-text escape twice.

**Witness.** ```rust
// typst.rs:159-166 (atom); the same loop at :174-181 (upright), :483-490 (label's text) and :791-798 (fitch's string)
    out.write_str("italic(\"")?;
    for c in name.chars() {
        if c == '"' || c == '\\' {
            out.write_char('\\')?;
        }
        out.write_char(c)?;
    }
    out.write_str("\")")
```

```rust
// latex.rs:232-236 (upright), repeated in label's `text` closure :399-403
    out.write_str(r"\mathrm{")?;
    for c in text.chars() {
        escape(out, c)?;
    }
    out.write_char('}')
```

**Change.** One `typst::quoted(out, text)` that writes `"…"` escaped, called by the four; `latex::label` calls `upright`.

**Value.** A reader, and the next character Typst strings must escape, added once.

**Net.** The unit tests `names_are_escaped` in both files, the export snapshots and the `export` check.

## F-6b-15

- dimension: idiom
- criterion: E7
- place: core/src/export/svg/net.rs:79; core/src/export/svg/kripke.rs:342; core/src/export/svg/tree.rs:147
- kind: refactor
- cost: small
- candidate: no

**Claim.** Six of the eight sites of `manual_midpoint`, a lint the workspace allows only because it fires today, are in the SVG layouts, beside code that already calls `i64::midpoint`.

**Witness.** `clippy.md`: `clippy::manual_midpoint` 8 sites, 6 in export; `Cargo.toml:131` `manual_midpoint = "allow"` under "Pedantic, and firing today".

```rust
// net.rs:79
    let middle = ((from.0 + to.0) / 2, (from.1 + to.1) / 2);
// kripke.rs:342-343
                        2 * (start.0.max(tip.0) + reach) - (start.0 + tip.0) / 2,
                        i64::midpoint(start.1, tip.1),
```

The others: `kripke.rs:645`, `tree.rs:147`, `net.rs:857` (a test).

**Change.** Use `i64::midpoint` at the six sites (it rounds toward zero, as `/ 2` does, and cannot overflow), fix the two outside export, and turn the lint to `warn`.

**Value.** A reader, and the lint list, which loses one allowance (E7).

**Net.** The export snapshots byte for byte (same rounding), the `export` check, Clippy in the gate.

## F-6b-16

- dimension: api
- criterion: C-VALIDATE
- place: core/src/export/pdf.rs:160
- kind: API break
- cost: small
- candidate: no

**Claim.** `pdf::Date` documents ranges for its public fields and derives `Deserialize`, but nothing checks them, and krilla clamps an out-of-range month or day silently, so a settings file with `"month": 13` writes December.

**Witness.** ```rust
// pdf.rs:157-166
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Date {
    /// The year.
    pub year: u16,
    /// The month, 1 to 12.
    pub month: u8,
    /// The day of the month, 1 to 31.
    pub day: u8,
```

krilla 0.8.2, `interchange/metadata.rs:340-342`: `self.month = Some(month.clamp(1, 12));`; `from_svg` (`pdf.rs:281-287`) passes the fields on unchecked.

**Change.** Check the fields in `from_svg` (or in a hand-written `Deserialize` and a `Date::new`) and answer `Error::InvalidOption { key: "pdf.date", … }`, as `typst::Length` and `rocq::Identifier` do for their options.

**Value.** A user of `pdf::Options` through JSON settings or the web page: a wrong date is refused instead of silently changed in an archival document.

**Net.** A unit test of a refused date; the `export` check's veraPDF run is unaffected.

## F-6b-17

- dimension: text
- criterion: E4
- place: core/src/export/latex.rs:453
- kind: text
- cost: small
- candidate: no

**Claim.** The doc comment of `begin` names a parameter `tree` that the function does not have.

**Witness.** ```rust
// latex.rs:453-455
/// Writes the start of a standalone document, with `ebproof` loaded if
/// `tree` is set.
fn begin(out: &mut impl Write, options: &Options, package: Option<&str>) -> std::fmt::Result {
```

It is called with `Some("ebproof")` (`:721`), `Some("fitch")` (`:785`) and `None` (`:475`).

**Change.** "Writes the start of a standalone document, loading `package` where one is given."

**Value.** A reader of the code.

**Net.** None needed.
