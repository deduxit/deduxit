# Step 67: the kernel crate, if the suites show what they share

## Session 67a: the inventory and the boundary

Fable 5.2 at `high`, a reused worker (its conversation cleared), in the
jj workspace `../deduxit-wt-step-58b-8c0bbc`, from the brief
`start-67a.md` and `plan/67-kernel-crate.md`, on 2026-11-16, cores 10
to 12. The chain on `main@origin` (`ytokpkzq`):

| change | what |
|---|---|
| `vvnzzszt` | `plan/notes/kernel-boundary.md`: the inventory, the gate's answer, the boundary a split would follow, its sessions |
| `uxuwtkpr` | `plan/later.md`: the entry "The kernel crate: ended at the gate" with the list, and structure-5's row |
| this report | |

No code changed.

### Outcome

**The gate's answer is "not yet": step 67 ends here, with no extraction
session.** A kernel crate has nothing to stand under today, cannot be
unpublished as the prompt fixes it, could not hold the types the prompt
lists first, and would hold about 1 050 lines of copies. The list is in
`plan/later.md`, and the design for the day a split is decided is
`plan/notes/kernel-boundary.md`.

### The inventory's summary

Four read-only agents (Sonnet 5.5, `high`) compared the suites' code
pair by pair with file and line; I checked the anchors the note cites
and the facts the answer rests on by `grep` and `diff`. The note's
section 3 has the tables. In short:

- **Copied by at least two suites** (about 1 050 lines of the crate's
  156 447):
  - the propositional arena and sequent, `ordinary` to `modal`: about
    300 lines identical after renaming (`ordinary/mod.rs:539-956`,
    `modal/mod.rs:71-480`), as 65a recorded;
  - the parser: about 296 lines (`ordinary/parse.rs`, `modal/parse.rs`);
  - the sequent's JSON form: about 143 lines (`serialize/ordinary.rs`,
    `serialize/modal.rs`);
  - the goal tree's helpers, `linear` to `ordinary`: `subtree`,
    `postorder`, the core of `undo`, the graft's renumbering and the
    reader's history walk: about 60 lines the same text, at most 230
    with the functions that follow the same algorithm; the session for calculi
    given as data shares none of these algorithms;
  - two `Meter` structs (`rules/check.rs:29`, `modal/model.rs:376`) and
    one memory pre-check written twice.
- **Shared in one copy already** (about 4 600 lines that name no suite):
  `limits`, `ParseError`, `hash`, `kripke` and its drawing, the wire
  machinery, the renderers, the protocol's data types; and, under
  `linear` though other suites use them, the memory account, the name
  checks, the walk `Stops`, the layout trait `Drawn`, and `Reason`.
- **Sums over the suites**, which no crate below the suites can hold:
  `Error` (19 variants carry a suite's type), `Settings` (eight fields
  of suites' options), the wire level's table, `export::Drawable`,
  `Notation`, the protocol's registry.
- **One suite only**: the parallel runtime, each suite's checker, each
  suite's `Outcome`, `Verdict` and `Statistics`.

The records against the code: 65a's list holds line for line; 49a's
holds for the linear and ordinary sessions; 54a's is shape, not code;
60's is the pattern of a reader and the arena's idea. Structure-5's
candidate list was judged from the rules files, by its own word, and
the code moves the error family and `Settings` out of its generic
column.

### The boundary and its reasons

Three layers (the note's section 5), for when a split is decided:

- **The kernel**: the limits and refusals, `Clock`, `ParseError`,
  `ErrorKind`, the hash tables, the name checks, the walk, the memory
  account, a meter, the wire machinery, the Kripke model and its
  drawing, the fonts and the PNG and PDF renderers, the layout trait
  and the tree layouts once its signature stops naming linear types,
  the protocol's data. Reason: none names a suite.
- **The suites** keep their calculus: for `linear` the arena, the
  forest, the proof term, the checker, the engines and the parallel
  runtime (D17, D25).
- **The facade `deduxit`** keeps the sums and re-exports every
  published path. Reason: a suite's own error is a variant of `Error`,
  so `Error` stands above the suites.

The generic forms that would replace the copies are a crate-private
`Arena<N>` (the nodes, the hash-consing map, the foreign-id check, the
refusal at `u32`), a parser over operator tables, and three free
functions for the goal tree; none needs a crate, and the note's section
6 gives each the step that should take it. The cost of a split is four
to five sessions, a third crate on crates.io and a breaking list.

Measured for the cost (cores 10 to 12, three jobs, the dev profile with
default features): the library's unit is 20.0 s with incremental
compilation off (frontend 11.9 s, codegen 8.1 s) and 3.5 s for an
incremental rebuild after touching `lib.rs`. Step 47 had 11.8 s at
about 110 000 lines. A kernel of 3 % of the lines cannot move either
figure by more than its share, and every edit of it rebuilds the
library.

### The gate's answer, and what I decided

The prompt's rule decides the gate; four points it does not settle I
decided, for the planning session to review:

1. **A published crate cannot depend on an unpublished one** (the Cargo
   book, "Specifying dependencies", read today: "crates.io does not
   allow packages to be published with dependencies on code published
   outside of crates.io itself"). So "unpublished at 0.0.0" and
   "`deduxit` re-exports at the old paths" cannot both hold. A split
   publishes the kernel as `deduxit-kernel` with `deduxit`, at the same
   version, saying it promises nothing of its own. (departs from the
   design)
2. **`Error`, `Settings`, the wire level, `Drawable`, `Notation` and
   the protocol's registry stay in `deduxit` in any split.** (departs
   from the design: the prompt's candidates begin with the error family
   and `Settings`)
3. **A split is a 0.y release with a breaking list, not a change
   without a break**: the suite-free code answers `Error` today
   (`kripke::Model::check_world`, `wire::upgrade`, the arena's `add`),
   and below the suites it answers an error of the kernel's own.
   (departs from the design)
4. **The copies are not made one inside the crate by sessions of this
   step.** Kept as two public types, as the published paths ask, a
   shared arena saves about 115 lines a copy and adds a trait and its
   wrappers; step 74 gives `ordinary::Node` quantifiers and its parser
   binders, which decides what a shared core may assume. Each item
   waits for the step named with it in `later.md`.

The parallel runtime fails the gate outright: one suite uses it.

### Checks

- Every line anchor the note cites for the copies was read back by
  `grep -n` on this commit (`pub fn add` at `ordinary/mod.rs:664` and
  `modal/mod.rs:206`, the two `Meter` structs, `Drawn` and its four
  implementations, `Reason` in `ordinary/decide.rs:151` and
  `modal/decide.rs:47`, `SUITES` at `protocol/mod.rs:417`, and the
  rest).
- The identical-line counts are `diff` counts of the files after
  renaming `modal` to `ordinary`; my own count over the whole files
  gave 1 125 and 417 differing lines of the pairs, the agent's over the
  copied ranges 109 of 410 and 91 of 387.
- The build was timed once (`cargo build --locked -p deduxit --timings`
  in a capped scope, `CARGO_BUILD_JOBS=3`), beside 66c on cores 7 to 9.
- No gate, test or target set ran: nothing but `plan/` changed.

### The reviewer's findings

`fresh-reviewer` on Opus 5.5 at `high`, once, on the three changes with
the brief, the prompt and the criteria. It found the facts the gate's
answer rests on sound against the code (one library crate, `Error`'s 19
variants, `limits.rs` free of suites, the parallel runtime's single
user, `Reason` and the account, `Drawn`, the three `diff` counts, the
two meters, the totals, step 47's figures, the Cargo rule) and about 50
sampled anchors right but two. Seven findings, none changing the
answer; all hold, and the note and `later.md` are corrected:

1. **Medium: the boundary contradicted itself on `LEVEL`**, kept in the
   facade and listed among the kernel's re-exports, while the kernel's
   `version` compares against it. Fixed: the level is a sum over the
   suites' forms and stays the facade's constant; the kernel's reader
   takes the level from its caller, and `deduxit::wire::upgrade` stays
   the facade's function.
2. **Medium: an unmarked departure**: the kernel holds the propositional
   arena's core where one is built, and structure-5 keeps every formula
   arena out. Marked in the note and below.
3. **Low: the split's first session is on Fable 5.2**, where the brief
   speaks of the extraction's sessions on Opus 5.5. Kept and said: it
   is the boundary's session again, which the prompt gives to Fable.
4. **Low: the goal tree's "about 300 lines" and "the same text" were
   overstated.** The cited ranges are about 60 lines a side; `subtree`
   and `postorder` are the same text, the core of `undo` nearly, the
   renumbering and the history walk the same algorithm under other
   names. Fixed: 60 lines the same text, at most 230 with their
   functions, and the total 1 050 for 1 150.
5. **Low: the count of poll constants was wrong** (28 by `grep`, ten
   outside `linear/`, not eleven with three values). Fixed.
6. **Low: two anchors were off** (`notation.rs:81` and `:87` for
   `necessary` and `possible`, `protocol/ordinary.rs:475`). Fixed.
7. **Low: three statements were false as written**: one `Readable` impl
   is in `workspace/form.rs`; `deduxit_cli` is a library too; `styles`
   is the exports' sum, so `Settings` has eight fields of suites'
   options. Fixed.

It did not verify the build times, the count of inline poll sites, or
the Cargo book's wording against a live copy (I read the page today).

### Departures

The three marked above; a fourth in the note, where a split's kernel
holds the propositional arena's core if one was built (structure-5
keeps every formula arena out; D25's arena, the linear one, stays in
its suite) (departs from the design); a split's first session on Fable
5.2, the boundary again; and one of the brief: it says "you write only
your note and report", and the prompt's gate says the list "is kept in
`later.md`"; I wrote the `later.md` entry as a change of its own
(`uxuwtkpr`), which the planning session can drop or reword.

### For the planning session

- The step table's row 67 and D25 still describe a crate; the row
  wants "ended at the gate", and step 68's dependency on 67 is then
  met.
- `core.md` ("Layout") could gain one bullet saying why there is no
  kernel crate and where the list is; I did not write it, the brief
  keeping me to the note and the report.
- This workspace's `target/` holds 88 GB from the sessions before me.

### What the next sessions must know

- **Nothing is extracted, and nothing in the code changed.** A session
  that edits `linear/search/memory.rs`, `linear/sequents/name.rs`,
  `linear/sequents/fmt.rs` or the trait `Drawn` may move it to the root
  in the same change (`core.md`'s layout rule; no public path changes).
- **Step 68**, if its tableau checker needs a meter, takes the one of
  `rules/check.rs` to `limits` instead of writing a third.
- **Step 74** changes `ordinary::Node`, the ordinary parser and the
  ordinary session: it is the step that decides the shared arena core,
  the parser over tables and the goal tree's free functions, with the
  ratchet's `read-tptp`, `read-json` and `ordinary-pigeons` as its
  check, and `notes/kernel-boundary.md` (section 6) as the sketch.
- **A split, if one is decided**, starts from the note's sections 5 and
  8, and from the three constraints above: the kernel is published, the
  sums stay in the facade, the release has a breaking list.
