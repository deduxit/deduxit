# Step 87f1: messages that say what is wrong (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-fixup-30-3e23c4`, from `plan/87-refactor.md` (87f1, wave
F, theme F2 of `plan/notes/audit/findings.md`) and the planning
session's briefs `start-87-common.md` and `start-87f1.md`, on cores 13
to 15. The base is `poutolpy` (87e2). The twelve findings of F2's first
part: F-4f-04, F-4e-08, F-4f-02, F-4f-11, F-4e-18, F-5b-06, F-5b-10,
F-6a-08, F-2c-03, F-3-04, F-7b-11, F-7b-19. *Changes output*; no check
changes what it accepts. No panel; the reviewer on Opus at `high`.

## What landed

One area per commit, each with its changelog line (Changed) and, where
the output is shown, the manual in the same commit:

- `onpznzum` Name the logic, not LJ, in the ordinary session's refusals
  (F-4e-08): `StepError::Logic` says "every goal there has exactly one
  formula" after the logic it names, and in minimal logic `⊥L` says
  "minimal logic has no ex falso"; `StepError::Succedents` names
  intuitionistic and minimal logic and says what to do; the session
  reader's two messages of a goal's count name the logic
  (`open_goal_succedents`, `closed_goal_succedents`). The minimal
  session's example in `docs/src/ordinary-logic.md` moved with it.
- `mtwwrnnq` Say what the linear session's axiom needs and what a saved
  session breaks (F-4f-02, F-4f-11): `NotAlone` on the axiom says "the
  axiom needs formula N with its dual literal and nothing else in the
  goal"; the stored session's reader names the fault, one message per
  check, where one sentence stated the rule ("a premise is past the last
  inference", "an inference is the premise of two inferences", "a cut
  names no cut formula", "an axiom or a Mix names a principal formula",
  "a session of an ordered mode has a cut", "a cut does not have two
  premises", …).
- `xkxzyvtx` Name what is wrong in a rule file's errors and its session
  reader's (F-4e-18, F-5b-06): an unbound variable names itself and
  where it stands ("the variable \"B\" of a condition occurs in no
  sequent of the rule"), a context written twice names the context and
  the zone, a context used elsewhere says whether another sort or
  another discipline, an argument's sort error names both sorts, the
  contexts that cannot be told apart are listed, a sort with an empty
  name is said as such (it was "declared twice"); the rules session's
  reader says "the session has no node" or "no scope".
- `mnnopzks` Word the term and node index messages by what they name
  (F-6a-08, F-5b-10): "index 7 names no term: there are 3 terms" (it
  called the index a root formula) and "index 99 names no node: there
  are 2 nodes" (it spoke of a proof's nodes); the workspace's missing
  node gives the workspace's count (`no_node`), where it said 0.
- `xuoqttzy` Name the line in a `.spec` file's errors (F-2c-03): every
  refusal of the sections is led by its line (`Reader::placed`: the line
  of the last token read, each check failing at or right after the token
  it names; the tokenizer's own error gives its line itself); "no
  target" has none.
- `zpmxxytm` Name the reason a mode has no proof net of the cyclic
  order (F-3-04): `NetError::Mode` says "proof nets of an ordered mode
  exist without Mix only: leave out Mix" with Mix, "a proof net of
  cyclic mode has no cut link, since its calculus has no cut" for cut
  links, and keeps its text for affine mode; `MellStructure::from_parts`
  under a cyclic criterion answers `NetError::NotMell` with the cyclic
  mode (same kind and code, `no_nets`), whose text names the modes that
  have nets with boxes (departs from the plan, below).
- `muwxrvok` Name `proof --json` in a session's line for a derivation
  left out (F-7b-11): `Show` knows whether a session shows the
  derivation (`session`, `whole()`), and the four left-out lines name
  `proof --json` there; the command's lines are unchanged.
- `ooxkltmx` Word the answers the three sessions share once (F-7b-19):
  `goal_number`, `NO_RULE` ("no rule applies there"), `unknown_command`
  ("`help` lists the commands") and `another_kind` (the kinds the other
  two sessions are, and how to start one) in `interact/mod.rs`, used by
  the linear, ordinary and rules sessions; the grading chapter's example
  moved with it. `pvkklxpr` reblesses the behaviour lock for it (two
  files: the session's error and grade's message), placed right after.
- `xknpxmuw` Refuse a linear split's position past the goal as no
  formula of it (F-4f-04): `StepError::NoFormula` ("the goal has no
  formula 9: it has 6 formulas"), where `Split` listed it among its
  causes.
- `vzzsputt` Name which cause holds in a linear split's and
  eigenvariable's refusal (F-4f-04, with the planning session's yes):
  `linear::SplitCause` (`Twice`, `Principal`) and `linear::EigenCause`
  (`OutOfScope`, `AfterWitness`), both `#[non_exhaustive]`, as the field
  `cause` of the `#[non_exhaustive]` variants `StepError::Split` and
  `StepError::Eigenvariable`, written `"cause"` in the error's details;
  each message names its cause alone ("the split names formula 0 twice:
  name each formula going left once"; "the eigenvariable b cannot fill
  the open witness: it was made after the witness, …"). Under the
  changelog's Added; the guide's errors section names the field; the
  wire schema holds an error's `details` as any object, so nothing there
  changes (departs from the plan, below).

The rules files: `core-inputs.md` (the `.spec` lines), `core-nets.md`
(`NotMell` under a cyclic criterion), `cli.md` (the shared answers).

## What the page shows that changed

The bindings show the library's messages as hints; these changed:

- the ordinary session (LK, LJ, minimal): every `StepError::Logic` (the
  rules `⊥L` in minimal logic, `∨R`, `CR`, `WR`, `⊥R` outside LK) and
  `StepError::Succedents`; reading a saved session of LJ or minimal
  logic whose goal has another count right of `⊢`;
- the linear session: `NotAlone` on the axiom; a split position past the
  goal, now "the goal has no formula N: it has M formulas" (kind
  `no_formula` in the error's JSON, where it was `split`); the `Split`
  text, one per cause, with `"cause"` in its details; the
  `Eigenvariable` text, one per cause, with `"cause"`; reading a saved
  session that breaks the form (each fault named);
- the rules session and rule files: every message of the list under
  `xkxzyvtx`, and "the session has no node" / "no scope" on reading;
- `Criterion::of` of an ordered mode with Mix (the net's refusal), and
  a net with cut links in the cyclic mode;
- an index out of range of terms or nodes (a document or a workspace
  edit naming one), and the workspace's missing node.

The `.spec` reader and the command's session answers are the command's
alone.

## Departures

- **Two public enums and a field** (departs from the plan, whose part 1
  adds no public item): F-4f-04's causes need a field on `Split` and
  `Eigenvariable`, which had only the position and the name. The
  planning session decided it (unattended): `SplitCause` and
  `EigenCause`, additions only, on `#[non_exhaustive]` variants. The
  out-of-range split moved to the existing `NoFormula` besides.
- **The wire schema is unchanged**: it describes an error's `details`
  as an object of the variant's fields, with no schema per variant, so
  `"cause"` is valid there already; the guide names it.
- **`NotMell` for a structure with boxes under the cyclic order**
  (departs from the plan): `NetError::Mode` with `Mode::CYCLIC` could not
  tell cut links from boxes, two causes with one value; the structure
  with boxes now answers the refusal of its own type, which names the
  modes nets with boxes have. Kind and code are the same; the error's
  JSON `kind` is `not_mell` there.
- **The ordinary session's `Split`** lists its causes too; it is not
  among the findings and is left as it was.
- **`UnknownName`'s `known` list** in a rule file (a sort's name) stays
  empty: the list is `&'static`, so naming a calculus's own sorts is an
  API change (part 2).

## The net

- **The gate** at `rqnpupok`, after the review's fixes, and at `vzzsputt` before them: passed (clippy on the workspace and on the
  core crate without `parallel`, the tests, both `cargo hack` runs, the
  ratchet: every journey within its ceiling). Its first two runs caught
  a CLI test and a web test that pinned the old eigenvariable text; both
  now check the cause, and the fixes are folded into `vzzsputt`.
- **The bindings' tests under Node** (`web-wasm-tests`): pass at the
  head, and passed before the cause enums too.
- **The behaviour lock**: two files changed, reblessed in `pvkklxpr`
  right after the change they pin; no other output of the corpus moved.
- **README's, the manual's and the guide's examples**
  (`cargo test -p deduxit-cli --test readme`): pass, the grading
  chapter's example and the guide's errors section changed with their
  outputs.
- **`breaks 0.8.0`**: the semver checks' three failures are the ones
  earlier steps left (`reason` fields renamed, System F's variants, a
  deprecation), unchanged; the public API's diff against the run before
  the cause enums adds `SplitCause`, `EigenCause` and the two `cause`
  fields and nothing else.
- **Tests added**: `a_mode_s_refusal_names_its_reason` (nets),
  the `.spec` reader's line cases, `the_sessions_word_their_shared_answers_alike`
  and the session case of `derivation_limit_and_tree_switch` (CLI), the
  causes in `a_session_keeps_the_eigenvariable_condition` (text and
  JSON), the index and workspace messages, the rule file's three new
  messages.

## The reviewer

fresh-reviewer (Opus 5.5, high) read the chain against the brief and the
findings, ran the head's command and a scratch crate, and found seven
things, all answered in commits of their own after `vzzsputt`:

- `NetError::Mode` gave an ordered affine mode the cut-link reason
  (`Mode::CYCLIC.with_affine()` "has no cut link"), since `Criterion::of`
  refuses every affine mode with it: the affine arm now comes first
  (`rzspqnkr`), and the test runs the cyclic and Lambek affine modes.
- The rule file's "cannot be told apart" listed contexts another
  equation had defined (a premise `Γ ⊢ A` beside `Γ, Δ, Σ ⊢ A` named Γ):
  only the undefined ones now (`zxqvupxr`, with a test).
- `SplitCause::Principal` said the split cannot send the formula left,
  false for a Mix, which sends it left by itself; the message and the
  doc now say the split does not name the step's own formula, true for
  both (`tkstoumw`).
- The axiom's and `1`'s refusals ended "in affine mode, weaken the other
  formulas first" in every mode, and for a goal of one formula, where no
  weakening helps: dropped (`tkstoumw`; the changelog records `1` too).
- `load`'s refusal listed both other kinds where the code knows the one
  the file holds: it names that kind now (`onkxttrr`; the test loads a
  linear file in the ordinary and rules sessions and an ordinary one in
  the linear session).
- `core.md`'s list of marked public types lacked the two new enums
  (`tkstoumw`).
- `core-inputs.md` said every refusal of the sections names its line; a
  counter named by a keyword is the shared `Error::AtomName`, which has
  no place for a line (`rqnpupok`; the changelog says so). Giving it one
  is a field of a public variant, left to part 2 (departs from the
  plan: F-2c-03 is done for every `Error::Spec`, not for that one).

It confirmed the rest: the `.spec` line count, the causes chosen, the
rule files' names, the saved linear session's faults, the workspace's
count, no "LJ" left in the ordinary session, `proof --json` in every
session line, and no stale pin in the tests, the manual, the guide or
the lock.
