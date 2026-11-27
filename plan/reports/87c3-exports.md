# Step 87c3: the exports (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-fixup-30-3e23c4`, from `plan/87-refactor.md` (87c3, wave
C, theme C3) and the planning session's briefs `start-87-common.md` and
`start-87c3.md`, on cores 13 to 15 (the panel's too), beside 87b1, 87c1
and 87c2. The base is `main@origin`. No output changes: every snapshot
byte for byte, no certificate's text, no refusal's numbers. The public
API is kept (everything new is `pub(crate)` or private), so no changelog
line: nothing a user sees changes.

## Outcome, per finding

| finding | commit | what is written once now |
|---|---|---|
| F-4d-08, F-4d-09 | `usultlov`, `uxnxkmxt` | `export/rocq/library.rs`: one table per module of rocq-deduxit (`formula`, `iformula`, `BOUND`/`EIGEN`/`TAPP`, `proof`, `iproof`, `QUEST`/`LEFT`/`RIGHT`, `ordinary`, `modal_node`, `modal`, `modal_proof`, `LAB`/`REL`), the one place the crate names a constructor of the library; and the writers' helpers `string`, `stopped`, `Bounded` (with `push_all`) |
| F-6b-05 | `usultlov` | the kernel's two propositional formula walks are one (`formula` with an optional `Reading`), the binder guard kept as an `assert!` |
| F-6b-11 | `rsmxwnur` | the ordinary countermodel's last sentence (`models::last`) and size estimate (`models::sequent_bytes`), which `svg::countermodel` now calls; the modal estimate stays where it is, as the verdict says |
| F-6b-14 | `qmtzznpl`, `kxyqwymu` | Typst's string escape (`quoted`, inlined) at its four sites; LaTeX's `upright` called by the label closure, its `serialize` gate gone |
| F-6b-04 | `uswqyxlp`, `mospkzln` | the net drawings' document (`net_document` over `NetGroups`: groups, title, description), `cut_bar`, `link_arc`, `net_text` and `net_links` in `svg/mod.rs` |
| F-arch-25, F-6b-13 | `nnwqolwk` | the dispatch (`export::write_drawable` over a `Target` trait LaTeX, Typst and SVG implement) and the text formats' sequent, structure and Kripke structure (`text_sequent`, `text_structure`, `text_kripke_structure` over `Text`), with one `PER_OCCURRENCE`; the tree emitters stay per format, as the verdict narrows it |

`mlvzktzo` records it in `.claude/rules/core-export.md`; `uxnxkmxt`,
`mospkzln` and `zrrwzvsm` are the panel's and the reviewer's fixes,
below.

**The table against rocq-deduxit's interface.** Every name of
`library.rs` is the one the writers wrote before (the argument member
checked each arm against the old matches; the reviewer did again), and
all 107 occur in rocq-deduxit's `tests/Interface/` at its `main`. The
theorems' names, the records' fields and the preludes stay where they
are written, as the rules file says: they are statements, which 87d4
changes on purpose.

## Departures

- **F-6b-04** (departs from the plan): the finding's change also lists
  `formula_extent`, and its witness the chars-and-duals pass of the
  estimate and the input colour's bytes. They stay in each drawing. The
  extent reads its drawing's own children and brackets (an occurrence's
  two operands, or a vertex's children with boxes), and the estimate
  counts per occurrence in one and per vertex in the other; both are the
  layout F-6b-03 makes a struct in 87e6, where they can be shared
  without a signature of closures written only to be replaced. The rules
  file says so.

## The panel (certificates)

The three members from a fresh context on the brief
`side-session-notes/…/87c3-panel-brief.md` (eight claims):

- **The argument** (Fable 5.2, `high`): two refutations, both fixed in
  `uxnxkmxt`.
  - **The bound's refusal named a different length.** `rocq_models.rs`
    pushed a constructor and its space as two `Bounded::push`es where
    one `"var "` was one, and `Bounded` compares after each push, so a
    finite or Kripke structure's certificate refused at the same bound
    with `estimate_bytes` up to five bytes smaller (witness: `a = a ⊢
    a = b` at `derivation_bytes` 59, 65 against 60). Now
    `Bounded::push_all` compares once after the pieces; the integers
    member's sweep at `uxnxkmxt` matched `main@origin` in 46 206 lines.
  - **Writers still named constructors.** `strip_prefix("var ")` and
    `("covar ")` in the kernel's `literal` (now what follows the first
    space), the modal `head`'s match on library names (now on the
    calculus's rule names), `lab` and `rel` (now `LAB`, `REL` in the
    table) and `"bot"` for an empty right side (now `library::ordinary(
    Node::False)`).
  - Beside them: the modal table items were dead with `rocq` but not
    `serialize` (now gated as their one user is), and three files were
    unformatted (`nix fmt`, in `uxnxkmxt`).
- **The counterexamples** (Opus 5.5, `high`): no difference at
  `uxnxkmxt` over about 59 600 inputs against `main@origin`, stdout,
  stderr and exit status. They covered:
  - every export format and both kernels across the suites;
  - every `--derivation-limit` from 0 to the output's length on 39
    inputs;
  - `seq print`, rule files with quotes and backslashes, and cut proofs
    through `check` and `eliminate`;
  - the net drawings per step.

  It reproduced the argument member's finding before the fix. The only
  other differences were 161 time-limited unknowns naming another copy
  bound on both sides. After `mospkzln` I ran its 886 net cases and the
  24 per-step drawings of its seven cut proofs again: identical.
- **The integers** (Sonnet 5.5, `high`): nothing in the diff wraps,
  truncates or panics. It swept every bound from 0 to the length on the
  Rocq, LaTeX, Typst and SVG writers, the shared sequent estimate, the
  net documents at `Style::MOST_LENGTH`, and the stop at every poll.
  **Found, not fixed** (in `main@origin`, out of this byte-for-byte
  step): in classical logic `rocq::write_countermodel` ignores
  `derivation_bytes` (`falsified` returns before the estimate). A shared
  formula graph doubles the text with each level (1.8 MB at depth 18
  under a bound of 0; depth 30 grows past gigabytes). The planning
  session gives it to 87d4, whose `let`s both shrink and can bound that
  text.

## The reviewer

`fresh-reviewer` on Fable, once, at `kxyqwymu`. Two findings, both
answered:
1. F-6b-04 was done only in part: the link arcs, the formula text
   closure and the description's links loop were still twice. Now in
   `svg/mod.rs` (`mospkzln`); what stays apart is the departure above.
2. Two comments told history (`typst.rs`'s inlining, `rocq_models.rs`'s
   push). They say why now (`zrrwzvsm`).

## The net

- The export snapshots (49) after every commit; `cargo test --lib`
  over the exports, Rocq, modal and ordinary tests.
- `gate` at `kxyqwymu`: passes. At `mlvzktzo` the ratchet failed on
  `render-typst`, 6.4 % over its ceiling: the shared `quoted` was called
  from `atom`, where the loop had been inline (18 % of the journey in
  the call). With `#[inline(always)]` it is 4 780 947 against 4 726 140,
  +1.16 %, within the tolerance; `render-latex` +0.01 %, `render-svg`
  0.00 %. Writing whole runs with `write_str` was worse (+7.6 %).
- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo check -p deduxit --no-default-features --features rocq` (then
  `svg`) at `mospkzln`.
- `breaks 0.8.0` at `kxyqwymu`: only earlier steps' items (the lambda
  System F variants, the step errors' `reason`, `Workspace::read_own`
  deprecated); nothing of the exports.
- `nix build` of the checks `export`, `rocq`, `rocq-deduxit` and
  `check-certificate` at `kxyqwymu`: pass.
- `nix flake check` at the report's head (`loqwlmwu`, before this line
  was added): all checks passed, `features`, `export`, `rocq`,
  `rocq-deduxit`, `check-certificate` and `ratchet` among them.
- Not run: the focused and net target sets (no search, no
  `ProofStructure` touched); `web-core-wasm32` (no integer of the
  checker, the size estimate or the memo).
