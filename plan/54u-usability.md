# Step 54u: the usability review's accepted changes

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`), and for some sessions in the web client
(`../linlog-prover.github.io`, `deduxit/deduxit.github.io`). CLAUDE.md
applies throughout:
- jj only, never git;
- thematic commits as soon as a unit is done;
- doc comments on every item, and every text a person reads follows
  `.claude/rules/writing.md`;
- the verification table;
- never push (deduxit-orchestrator pushes after your messages).

Written on 2026-11-09 by the planning session, from the author's answers
of that day to the usability review.

## Goal

The usability review (step 54w's first session) asked the author
twenty-six questions about the library, the command and the page
(`plan/notes/usability.md`, section 6). On 2026-11-09 the author
accepted every recommendation. This step builds the ones that change
the library, the command, the flake or the page, so that a user gets
what they want in fewer steps and with errors that say what to do.

Read first:
- `plan/notes/usability.md`: section 6 (each item's question, its
  accepted recommendation, its cost, what the docs say with it) and the
  tables of sections 2 to 5 that point to it (the frictions `FL*`,
  `FC*`, `FR*`, `FW*`, `C*`, `L*`);
- `.claude/rules/writing.md`, for every message, hint, label and doc
  comment the step adds;
- the rules files of the areas a session touches.

## Where the other items went

- **Q5 and Q12** (ordinary logic in one call; one JSON shape for
  ordinary logic) are breaking. They are step 55's first session, so
  the countermodel joins the new shape at one wire level
  (`plan/55-kripke-countermodels.md`).
- **Q11's second half** (`--kernel deduxit` the default in every mode)
  waits for rocq-deduxit in the opam archive (`plan/later.md`, the
  opam section).
- **Q7, W11, W12 and W13** keep things as they are. Q7's text (the
  first proof says `⊢ ~A, A` is `A ⊢ A`, and shows
  `--classical-sides two`) is 54w-c's. W11, W12 and W13 are 54u-d's,
  since they touch Learn and Practice.
- **W4** (a hint for every rule label) was done by 54w-a.

## The sessions

Each on Opus 5.5 at `high`, with its own brief and goal from the
planning session, its own jj workspace, and the `fresh-reviewer` agent
once before its final message.

1. **54u-a, the library** (`core/`): four additions, none breaking.
   - **Q1:** `Settings::stop(&self, elapsed: impl FnMut() -> Duration)
     -> impl FnMut(Progress) -> bool`. It fires past
     `clock.time_limit_ms` in every phase but `Phase::Refute`, the rule
     `cli/src/prove.rs` applies today; the command then uses it, so
     that the two cannot drift apart. The crate reads no clock: the
     caller passes the elapsed time.
   - **Q2:** `impl Display for lambda::Term` with the default options,
     and `export::{latex, typst, svg, rocq}::to_string(derivation,
     &options) -> Result<String, Error>` beside the `write` functions,
     bounded by the default `Limits`, as `Proof::derivation()` is.
   - **Q3:** `Deserialize` for `Outcome`, at every wire level the
     crate writes; an unknown engine name is read, not refused. A test
     reads a document of each level the behaviour lock holds.
   - **Q4:** in intuitionistic mode, the error for a formula with no
     intuitionistic reading carries the formula's text where the call
     has the forest, so that `Display` names it ("A ⅋ B has no
     intuitionistic reading; write ~A par B as A -o B"). The ids and
     `describe` stay for errors raised far from the forest
     (`core/src/errors/describe.rs`).
   - **Checks:** `breaks 0.3.0` shows only additions; the manual and
     the guide name each new call where they describe its area, in the
     same commit (54w-c then rewrites the chapters around them).
2. **54u-b, the command** (`cli/`, and the reason and rule texts in
   `core/` that the command prints).
   - **Q6:** hints on the errors a newcomer meets first, each saying
     what to write: `deduxit "A |- A"` (run `deduxit prove "A |- A"`);
     an unquoted sequent whose parse ends early while the command line
     had more words (quote it); `->` in linear mode (write `-o`, or add
     `--logic classical`); a file of `NAME: SEQUENT` lines read as one
     sequent (add `--input-format lines`). `--format tex` is accepted
     as `latex` (clap's alias).
   - **Q8:** a classical refutation speaks of the sequent as written:
     "A on the left has no A on the right to meet in an axiom" where
     the input was two-sided (`core/src/linear/search/refutation.rs`
     and the reason in `search/mod.rs`). Where weakening would close
     the gap, it adds that `-a` allows an unused hypothesis. The JSON
     `refutation` does not change.
   - **Q9:** `interact`'s unknown-rule error lists the session's own
     rules, ASCII spelling first. After a split whose premise the
     counts refute, it warns, as the page does.
   - **Q10:** `prove -h` shows the common flags in `help_heading`
     groups (the logic, the input, the output, the limits); the expert
     flags (`--bias`, `--forward-copies`, `--memo-limit`,
     `--pool-after`, the limits past `--timeout` and `--memory-limit`,
     the benchmark inputs) are `hide_short_help`, and `-h` ends with
     "`--help` shows every option". Check that `cli/tests/readme.rs`
     still finds every flag the guide names.
   - **Checks:** the behaviour lock's changed lines are a commit of
     their own that says why; the manual's and the guide's examples
     that show these messages change in the same commit as the code.
3. **54u-c, the page's prover** (the client): the stage, the dock and
   the goals.
   - **W1:** every reason (a refused step, a verdict, "Cannot close", a
     greyed export item) is a visible line under its keyword, not a
     tip; the long saturation sentence gets a "more".
   - **W2:** no Undo beside a refused step (`Stage.tsx`); "Other
     rules" reopens the popover instead.
   - **W3:** the empty stage offers "Start lesson 1" and "Take the
     tour" as buttons (never an automatic tour), beside one line of
     syntax.
   - **W9:** the tab shows "Linear (LL) ▾" or "Ordinary (LJ) ▾" on
     wide screens and the short mark on phones; one name, "Logic".
   - **W10:** the goal bar says "Open goal 1 of 1"; the Steps list
     keeps the ids.
   - **W15:** when the search refutes a goal that is not the root, a
     "Go back" button undoes the steps on that branch back to the last
     goal that still had a proof, from the verdicts already kept or one
     search per ancestor within the budget.
   - **Checks:** unit and e2e tests for each; the e2e tests that assert
     a moved text change with it (`toHaveText` on keywords may become
     `toContainText`); the client's `nix flake check`.
4. **54u-d, the page's Learn, Practice and Export** (the client), after
   54u-c, since both touch the shared islands.
   - **W5:** a wrong answer in Learn's check offers "Try ⊢ A, A
     yourself" (the problem the option names) before "Try the faded
     proof again".
   - **W6:** Learn's practice rows get "Has a proof" and "No proof"
     with immediate feedback; the "Answer" disclosure becomes the
     feedback.
   - **W7:** the Rocq export is always standalone (`Form::Standalone`);
     LaTeX and Typst gain a "Whole document" item, the fragment staying
     their default. Each item's detail names what the file needs.
   - **W8:** Export offers "Link to the sequent" beside "Link to this
     proof"; "Link" becomes "Share a link".
   - **W11:** Practice's text says what counts: "A problem counts as
     proved when you close every goal yourself; Hint may help."
   - **W12:** lesson 1's worked example shows the rewrite of `A |- A`
     to `⊢ A⊥, A`, with the Two-sided switch on, then off; lessons 1
     to 5 stay one-sided.
   - **W13:** after 54w-a's shorter openings, judge each lesson: move
     an opening below its worked proof only where it still holds more
     than the proof shows, and say in the report which moved and why.
   - **W14:** the storage keys `linlog-*` become `deduxit-*`; on load,
     read the old key, write the new and delete the old, for one
     release; the privacy page lists the new keys.
   - **Checks:** as 54u-c; the tests that name the storage keys
     (`tests/prover.spec.ts`, `tests/targets.spec.ts`,
     `tests/engines.spec.ts`, `src/lib/shelf.test.ts`, the offline
     spec's cache prefix) change with them, and one test migrates an
     old key.
5. **54u-e, checking a certificate in one command** (`modules/`, the
   flake): Q11's first half.
   - A flake app, `nix run github:deduxit/deduxit#check-certificate --
     proof.v`, that compiles a certificate against the pinned
     rocq-deduxit, or against NanoYalla for the classical default's
     certificates, as the checks `rocq` and `rocq-deduxit` do. It
     tells which kernel a file needs from the file itself, and exits 0
     when the file compiles.
   - A flake check runs it on one certificate of each kernel.
   - The manual's Rocq chapter leads with this command, before the
     pin and `nix build`.
   - **Checks:** `nix flake check`, and the app run from a clean
     directory with a certificate the command wrote.

## The order

- **On go:** 54u-a, 54u-b and 54u-c beside step 54s's first session
  (`plan/54s-sat-engines.md`, on 0.4.0's longest path), at most four
  step sessions at once; 54u-e when a slot frees. 54u-a and 54u-b both touch `core/` and the behaviour lock:
  the second to finish rebases onto the first and blesses the lock
  again.
- **After 54u-c:** 54u-d.
- **Step 54w's later sessions wait for what they document:**
  - 54w-c (README, the manual, the guide) after 54u-a, 54u-b and 54u-e;
  - 54w-d (the command's help and errors) after 54u-b;
  - 54w-e3 (export) after 54u-a;
  - 54w-e4 (ordinary and lambda) after 54u-a and step 55's first
    session, which reshapes `ordinary`'s calls;
  - 54w-e5 (the root, `web/`, `bench/`, `compat/`) after 54u-a;
  - 54w-e6 (linear's `lltp`, `mist` and families) on go, if a slot is
    free.

## Rules for every session

- **What the author accepted, and no more.** A change the item does not
  name goes to the report as a proposal, for the author.
- **Behaviour changes are tested,** one focused test per behaviour
  (`plan/conduct.md`), and the behaviour lock's changed lines are a
  commit of their own that says why.
- **The docs move with the code,** in the same commit: the manual and
  the guide for AI models where they describe the changed feature
  (CLAUDE.md's verification table); 54w's later sessions rewrite them
  after.
- **Never the `EnterWorktree` tool:** work in your jj workspace through
  absolute paths, `cd /home/tux/Projects/own/WORKSPACE && …` inside each
  Bash call.

## Deliverables

The commits, and `plan/reports/54u-usability.md`, one section per
session: what was built, the checks with their results, what the
reviewer found, and the proposals that went beyond the item.
