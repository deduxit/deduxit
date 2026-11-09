# Step 54w: every text a person reads, rewritten

The step's sessions each add a section here (`plan/54w-text.md`, "The
sessions"): 54w-r, the usability review; 54w-a, the web client; 54w-b,
the Rocq library; 54w-c, README, the manual, the changelog's Unreleased
section and the guide; 54w-d, the command's help and messages; 54w-e
and on, the crate's doc comments and comments.

## Session 54w-r: the usability review

Session 1 of `plan/54w-text.md` (Opus 5.5 at `high`, 2026-11-09), read
only: the one file it adds is `plan/notes/usability.md`, and this
section. Nothing else in any repository was changed, nothing was
pushed. W4 was answered by the author during the session (below). The page was reviewed by the sub-agent deduxit-web-usability
(general-purpose, Opus 5.5); the notes' claims by deduxit-fresh-reviewer.

### The checklist

| item | state | evidence |
|---|---|---|
| The prompt, the writing rules, README, SUMMARY, the newcomer's chapters (introduction, installation, syntax, commands, interactive, batch, cut, output formats, Rocq), SKILL.md and reference.md's library and limits sections, the crate root and the `linear`, `ordinary`, `export` and `lambda` module docs read | done | |
| The web client read from its `main` (`vpkkppkk`), not its stale working copy (16 commits behind) | done | copied read-only with `jj file show -r main` into the job's `tmp/client-main/` |
| Every task of the command run for real | done | `deduxit-cli 0.3.0` built from `main` in this workspace (release, capped, cores 12-15); 64 scripted lines and about ten more by hand, their output in the notes' sections 3 and 5 |
| The library's tasks run for real in a scratch crate outside the repository | done | `tmp/libuser` (path dependency on this workspace's `core`), eight tasks; the program and its output are the notes' appendices A and B |
| The page run for real, as it was cheap | done | headless Chromium against https://deduxit.github.io/ by the sub-agent, desktop and phone width, from the client checkout's installed Playwright (no build); its main claims checked against the source and the exported files |
| Per user kind: tasks, today's way with real output, frictions, proposed changes | done | `plan/notes/usability.md` sections 2-5 |
| The proposals that change the API, flags, behaviour or the page listed apart as questions, each with a recommendation, its cost and the docs either way | done | the notes' section 6: Q1-Q11 and W1-W14 |
| Text work for each later session | done | the notes' section 7 |
| fresh-reviewer's round, each finding answered | done | below: four points, three fixed in `kvyvkntl`, one answered with its witness |
| The notes and this section committed in `../linlog-step54w-r` on `main`, `jj st` clean, nothing pushed | done | the commits below, rebased onto `main` at `ryzrtzyo` |

### Outcome

The main frictions, in the order they hurt:

1. The crate root claims a recipe that decides what the command
   decides; it does not. On `!(A -o A * A), !(A -o B), !(B * B -o C),
   A |- C * ?D` it answers "unknown: the search was stopped" and the
   command "unprovable", because the command's stop lets the classical
   refutation run past the time limit (`cli/src/prove.rs:88-95`) and the
   recipe's stop does not. A wrong claim to fix in 54w-e (and in
   reference.md §16, 54w-c); Q1 asks for a helper.
2. On the page, every reason (a refused step, why a goal has no proof,
   "Cannot close", why an export is greyed) is a hover tip, never seen
   on touch, and the Undo beside a refusal undoes the last good step.
3. Lesson 1 explains about eight ideas before a worked proof of one
   step, and offers `⅋` and `⊗` as predictions before teaching them.
4. Exports and certificates do not stand alone: the page's LaTeX,
   Typst and Rocq files are fragments that do not say what they need;
   a logician needs two Rocq kernels installed two ways; the manual's
   `opam install rocq-deduxit` fails today (the package is not in the
   archive).
5. The command's first errors do not say what to do: no command named,
   an unquoted sequent (`-o B` is read as `--output B`), `->` in
   linear mode, `--format tex`.
6. Two-sided input, one-sided output and one-sided refutation texts,
   on the command and the page.
7. The library's printers want `Limits` and a `|_| false` stop at every
   call; `Term` has no `Display`; `Outcome` cannot be read back;
   intuitionistic input has two parsers and the common path shows the
   one with the worse error.
8. The manual's commands chapter is 726 lines under one heading, and
   README opens with a list and history before Install.

Two facts in the docs are wrong besides the first: `docs/src/introduction.md:66`
says the other provers have not run on the growing families (the
manual has that chapter, 2026-11-03), and the manual leads its Rocq
install with an opam line that fails today.

### The questions for the author

Short forms; the notes' section 6 has each with its recommendation,
cost and the docs either way.

- Q1 (library): a `Settings::stop(elapsed)` that stops as the command
  does. Recommended: yes, additive.
- Q2: `Display` for a lambda-term, and `to_string` exports without a
  stop. Recommended: yes, additive.
- Q3: `Outcome: Deserialize`, so the command's JSON reads back.
  Recommended: yes.
- Q4: the intuitionistic shape error names the formula, not "subformula
  0". Recommended: yes.
- Q5: `ordinary::decide` in the linear suite's shape (`decide` and
  `decide_within`) at 0.4; a counter-model for "not valid" later.
  Recommended: the shape at the next breaking release; the
  counter-model to `plan/later.md`.
- Q6 (command): hints on the first errors (missing `prove`, unquoted
  sequent, `->`, `tex` as an alias of `latex`, a lines file).
  Recommended: yes.
- Q7: two-sided derivations by default for two-sided input.
  Recommended: no; say it in the text and show `--classical-sides two`.
- Q8: refutations in the sides the user wrote, naming the flag that
  would help (`-a`). Recommended: yes.
- Q9: `interact` lists the mode's own rules in ASCII on an unknown
  rule, and warns after a losing split. Recommended: yes.
- Q10: `-h` hides the expert flags. Recommended: yes, with headings.
- Q11 (certificates): a flake app that checks a certificate; `--kernel
  deduxit` the default everywhere once rocq-deduxit is in opam.
  Recommended: yes, in that order.
- W1-W14 (page): reasons in view (yes), no Undo beside a refusal (yes),
  first-visit buttons for lesson 1 and the tour (yes), rule meanings in
  the popover (W4: answered by the author's request for hints on every
  rule label, now in 54w-a), a wrong hinge answer offers its
  problem (yes), answer buttons for "decide first" (yes, or drop the
  instruction), standalone exports (Rocq always, LaTeX and Typst as an
  item; yes), a link to the bare sequent (yes), the logic choice in
  view (yes), goals numbered among the open ones (yes), Practice's
  "proved" rule kept and said (keep), one-sided lessons kept with the
  rewrite as lesson 1's example (keep), example before explanation
  (text first), `deduxit-*` storage keys with a migration (yes), a
  "Go back" from a goal the student's step lost (W15, yes).

### The reviewer's round

deduxit-fresh-reviewer (the `fresh-reviewer` agent) reviewed
`syxnslpy` against session 1 of the prompt and the goal's criteria. It
re-ran 43 console examples of sections 2, 3 and 5 with the same binary
in a directory of its own: every output matched, exit statuses
included. It rebuilt appendix A from a copy and ran it: its output was
byte-identical to appendix B. It confirmed the repository claims
(`cli/src/prove.rs`, `core/src/lib.rs`, `introduction.md:66`,
`show.rs:392`, `nets/mod.rs:1404`, `rocq.md`'s opam lines) and
spot-checked the page's citations and live files, all holding. Its
four points and the answers:

1. "FC10's claim about commands.md is false: commands.md:51 already
   says `--work-limit N` gives the same answer on every machine." Not
   upheld: that sentence is `skills/deduxit/SKILL.md:51`;
   `docs/src/commands.md` names `--work-limit` only at :702, as a limit
   that ends a check (`grep -n work-limit docs/src/commands.md`). FC10
   now cites the exact lines (`commands.md:236-237` against
   `reference.md:384` and `SKILL.md:51`). Also fixed: the citation
   `cli/src/prove.rs:88` is the function `halting`, its
   `Phase::Refute` arm at :92; the notes and this report cite
   `:88-95`.
2. "FW6 sends a Go-back button to W2, which does not cover it."
   Upheld: the button is now its own question, W15, with its
   recommendation, cost and the text either way.
3. "Section 4's table has no cost column." Upheld: added, each cost
   checked against the client's tests (e.g. `ordinary.spec.ts:54`
   asserts the fieldset "Suite"; lesson 1's e2e runs press "Predict
   ax", which FW7's `choices` keep; no test asserts the empty stage's
   text or the export items' details).
4. "FW7's `choices` may be page behaviour." Kept as text work, with the
   reason in the notes' section 7: a lesson's front matter is the Learn
   material 54w-a rewrites, and the buttons only offer what the text
   has taught; if the planning session reads it otherwise, it joins
   W13. Its side remark that Q11's "docs either way" covered only the
   case without the change is fixed too.

### Departures from the design

None. The page was run against the live site rather than the client's
dev server, as the prompt allows ("only if cheap"): it needed no build.
No Rocq certificate was compiled (the notes say so, R3).

### What the next sessions must know

**For 54w-a (the page's text and teaching):**
- Work from the client's `main`, not its working copy: on 2026-11-09
  the working copy was 16 commits behind `main`.
- The text to rewrite and the traps are listed in the notes' section 7,
  "54w-a": where the text lives, the tests that assert exact strings
  (`tests/prover.spec.ts` alone has about 250), the front-matter rules
  `src/content/lessons.test.ts` enforces (every played rule in
  `choices`, one hinge option without a misconception, readings equal
  `READINGS`), lesson titles in two places, the tour's eight stops.
- Text you can fix without asking: the refusal and hint sentences end
  with the next action; "ladder", "rung", "context", "grafted", the
  net look "deduxit" go; lesson 1's opening keeps only what its worked
  proof shows (De Morgan to lesson 2) and its `choices` become `["ax",
  "no rule"]`; lesson 11 drops the command line; one name, "Logic",
  for the calculus mark; the export items name what each file needs
  (ebproof and cmll; curryst; NanoYalla or rocq-deduxit); Practice says
  it holds only provable problems and what counts as proved; the
  tour's stop 7 names the shapes, not only the colours.
- The author asked on 2026-11-09 for a hint on every rule label
  (`plan/54w-text.md`, session 2): that answers W4, and it opens on
  tap, unlike every tip of the page today ("no tip follows a touch",
  `src/islands/App.tsx:274`). Write the rule hints' sentences in the
  same keyword-and-sentence form as the reasons, so that W1, if
  accepted, can show them the same way.
- What you must not do without the author: anything else in W1-W15
  (a visible reason line, removing the Undo beside a refusal, new
  buttons, standalone exports, renamed storage keys, a "Go back"). Write the text so
  that it reads right today and still reads right if W1 (reasons in
  view) is accepted: each reason sentence is complete on its own.
- Most reasons are seen only on hover today. While W1 is open, do not
  move information into tips.

**For 54w-c (README, the manual, the guide):**
- Three facts to fix, not just reword: `introduction.md:66` (the
  provers on the growing sets did run); reference.md §16 ("the
  command's behaviour as data" holds for the options, not the stop);
  the Rocq chapter's opam line, which fails until the package is in
  the archive.
- The newcomer's path, from the review: install; decide a first
  sequent and read the verdict line and the exit status; why
  unprovable, and what unknown means and the flag to raise; `-i`;
  export (LaTeX, `--standalone`); `check`; step by step. The first
  proof says in one line that `⊢ ~A, A` is the axiom `A ⊢ A` and shows
  `--classical-sides two` (whatever Q7's answer, the text works).
- `commands.md` needs headings and a split: how-to pages per task, the
  four engines in an explanation page. README leads with one sentence
  of what deduxit does, then Install, then the first proof; history to
  the changelog.
- Add a "mistakes to avoid" how-to to the manual with the cases of the
  notes' C4 and their fixes as they work today (quote the sequent;
  `prove` is needed; `-o`, not `->`; `--input-format lines`); if Q6 is
  accepted later, the hints shorten it.
- One sentence for deterministic answers in both the manual and the
  guide: `--work-limit N` or `--copies N`.
- Every console example you touch must still run: `cli/tests/readme.rs`
  runs README's, the manual's and the guide's.

**For 54w-d (the command's help, messages, errors):** headings for `-h`
whatever Q10's answer; "does not act on formula 1" names the formula
and `rules G P`; `--lambda` on a classical proof names `-i`; ordinary
logic's "not valid … the image is unprovable" said in ordinary terms;
the `proof net` line of `--net` (its text is `Display` in
`core/src/linear/nets/mod.rs:1404`, so agree with 54w-e's nets session
who changes it). Q6, Q8 and Q9 are behaviour: not yours.

**For 54w-e and on (the crate's docs):** fix the crate root's recipe
(the first friction) so that its stop lets `Phase::Refute` run, or use
Q1's helper if the author accepts it first; the common path shows
`IllSequent` for intuitionistic input, `println!("{d}")` for a
disproof and what `|_| false` means; the root's 150-word sentence.

**For the planning session:** the questions go to the author with the
recommendations above; Q1's doc fix does not wait for the answer.

### Repositories and workspaces

- deduxit: the jj workspace `../linlog-step54w-r` (name `step54w-r`),
  made on `main` at `lxxwutox` and rebased onto `main` at `ryzrtzyo`
  ("Plan: review session 54w-b"), whose report section for 54w-b this
  file merges below; the commits below. Not pushed.
- The web client: read only, from `main` copied out; nothing changed.
- Scratch, outside every repository: the job directory's `tmp/`
  (the release build, `libuser/`, `cli/`, `client-main/`, `web/`).

Commits in `../linlog-step54w-r`, on `main` (`ryzrtzyo`), by change
id (the commit ids moved with the rebase):
- `syxnslpy` Notes: the usability review of step 54w
- `kvyvkntl` Notes: answer the review of the usability notes
- `kxxplkxn` Report: session 54w-r, the usability review
- `kkkmyxkv` Notes: W4 answered by the author's hints for rule labels

## Session 54w-b: the Rocq library's text

Session 3 of `plan/54w-text.md` (Opus 5.5 at `high`, background job,
2026-11-09), in the Rocq library's jj workspace `../rocq-linlog-54w-b`
(workspace `54w-b`) on its `main` (`tymnvlzv`, "Date the release of
0.3.0"). Nothing was pushed by the session: after "rocq ready",
deduxit-orchestrator pushed the chain, and `main@origin` is `qlxkqlrx`
(`df4b15ec`).

### What changed

The chain on rocq-deduxit's `main`, oldest first (change ids; the
commit ids are the ones the signing hook gave last):

| change | commit | what |
|---|---|---|
| `xkvouuxw` | `7e3d6f76` | `.claude/rules/writing.md`, a copy of deduxit's writing rule: the four rules word for word, the scope and reader paragraphs the library's (README, the doc comments `rocq doc` renders, every comment of `theories/`, `tests/`, `bridges/`, the changelog, the opam description, `CITATION.cff`, the flake's comments); no `paths:`, so it loads at launch. CLAUDE.md points to it under "Conventions" and lists it under "Claude Code setup" |
| `wkupmzuo` | `613e9aa5` | the comments of the propositional calculi and checkers: `Formulas`, `Calculus`, `Proofs`, `Check`, `Soundness`, `Mix`, `Intuitionistic`, `IntuitionisticCheck` |
| `nmrpzlkk` | `696905fd` | the comments of `Refutations` and `Weights` |
| `nxvmzomo` | `5c7c5602` | the comments of the first-order development: `Terms`, `TermsCheck`, `FirstOrder*` |
| `tknulkwv` | `0c491e16` | the comments of every test certificate under `tests/` |
| `utmpnqwr` | `5045ce35` | the comments of the bridges |
| `yswkruxx` | `2edf73b9` | README rewritten for its two readers: "Check a certificate" (the command, the certificate it writes shown and explained, the files a statement rests on, what a certificate trusts) and "Prove with the library" (a lemma by the rules, one by the checker, one refutation, the modules); then what it covers, upgrading from rocq-linlog, the repository, citing, licence. The AI disclaimer of the opam description and `CITATION.cff` added, as deduxit's README has it and the writing rule expects |
| `krywunsw` | `c0fefabe` | the opam description: says what the library checks, and how |
| `qnrlnwzm` | `8fb760c6` | a wrong claim fixed (the reviewer's finding 3): `bang` and `excess` in `Refutations.v` |
| `qlxkqlrx` | `df4b15ec` | README's list of the files a first-order statement rests on completed (the reviewer's finding 4) |

The rewrite keeps every theorem, definition, name and proof script.
Comments say what an item is or does for the reader; the module headers
of the trusted files say "read it to know what a certificate claims";
lists in doc comments render as `rocq doc` lists (checked in the built
pages).

### Wrong or misleading claims fixed

- `FirstOrderFormulas.v`'s header said the first-order development
  "shares no name" with the propositional one; it reuses the names
  (`formula`, `ll`, `proof`, `check`, …), as `rocq.md` says. Now it says
  so, and that a file needing both imports one and qualifies the other.
- `Refutations.v`: `bang` was "nothing where the atom is absent, else
  unbounded" and `excess` "[None] where … the atom lies below an
  exponential". The code keeps `(0, 0)` whenever the interval inside is
  exactly zero, atom present or not (`excess "A" (oc (A ⊗ A⊥))` is
  `Some (0, 0)`, reproduced by the reviewer). Both now say that.
- README listed `Terms.v`, `FirstOrderFormulas.v`, `FirstOrderCalculus.v`
  (or `FirstOrderIntuitionistic.v`) as what a first-order statement rests
  on; the statements also name `Calculus.rules` and
  `Intuitionistic.irules`, so `Calculus.v` and `Intuitionistic.v` are
  now listed.
- The opam description said the certificates "check every proof rule by
  rule", which describes NanoYalla's kernel, not this library's checker
  by computation.
- `tests/FromCommand.v` said its certificates were "translated from
  their JSON form by a scratch program" (how the file was made); now it
  says what they are.

### The checks

| check | result |
|---|---|
| No code change: every `.v` file of `theories/`, `tests/`, `bridges/` compared with `main`, comments stripped, whitespace collapsed (`strip_compare.py`, a lexer that drops nested `(* … *)` outside string literals) | 34 of 34 identical, exit 0, at `qlxkqlrx` |
| The same with string literals kept verbatim (the reviewer's `strict.py`, which closes the first one's blind spot of whitespace inside strings) | 34 files, 0 differ, exit 0 |
| `make` in place (capped, cores 12–15) | exit 0, 20 modules |
| README's examples: the certificate `deduxit prove --mix --format rocq --standalone` writes, a classical one with `--kernel deduxit`, a refutation, and the README's own lemmas, each `rocq compile -Q theories Deduxit` | all accepted; the README's certificate block equals the command's output byte for byte (the reviewer) |
| `nix build .#doc` | exit 0; the lists render as `<ul class="doclist">` |
| `nix flake check --max-jobs 4 --cores 8` under the machine lock, capped on cores 12–15 | exit 0 at `krywunsw` before the review; exit 0 at `qlxkqlrx` (`df4b15ec`) after the fixes, "all checks passed!" |

### The review

The `fresh-reviewer` agent read the chain `main..krywunsw` against the
brief and the writing rules: four findings, all low, each answered.

1. `IntuitionisticCheck.v`'s header said "the terms below a node
   determine its sequent", where the library's "below" means toward the
   root. Fixed in `wkupmzuo`: "a node and the terms of its premises".
2. A proof comment in `FirstOrderIntuitionisticCheck.v` said a premise's
   eigenvariables "are the conclusion's, less the one its step
   introduces"; for two-premise rules they are only among them. Fixed in
   `nxvmzomo`: "among the conclusion's, except …".
3. `bang` and `excess` (pre-existing, kept by the rewrite). Fixed in
   `qnrlnwzm`.
4. README's list of files for first-order statements (pre-existing).
   Fixed in `qlxkqlrx`.

No finding on the other criteria: no code change, no remaining
reference to how the library was made, the README examples true, the
opam description true, the four rules copied word for word.

### Departures from the prompt

- The prompt names "the docs pages": the library has no hand-written
  pages, so they are the doc comments `rocq doc` renders (and the
  forwarding pages `modules/doc.nix` writes, whose one sentence was left
  as it is).
- Beyond README, the doc pages and the comments: the opam description
  (a wrong claim) was rewritten. `CITATION.cff`'s abstract and the
  changelog's released sections were left: they are the 0.3.0 release's
  record, and the changelog has no Unreleased section.
- README gained the AI disclaimer, worded as the opam description's.
- The comments of the flake's modules (`modules/*.nix`) and scripts were
  not reworded: the prompt names `theories/`, `tests/` and `bridges/`.
  The copy of the writing rule covers them from now on.

### What the next sessions must know

- The library's `.claude/rules/writing.md` is a copy of deduxit's: a
  change to the four rules belongs in both repositories. Its scope and
  reader paragraphs are the library's own.
- In the library's comments, "below" a node means toward the root and
  "above" toward the leaves (`a [⊤] above absorbs`); keep that when
  writing new ones.
- A comment must keep an even number of `"`: Rocq lexes strings inside
  comments, so an odd count breaks the file.
- To show that a text-only change touches no code, compare each `.v`
  file with the base after stripping comments: the two comparators of
  this session are in the job's scratch directory, gone with it; the
  method is a lexer that skips nested `(* … *)`, keeps string literals
  verbatim and compares the remaining tokens.
- `CITATION.cff`'s abstract is one long sentence; reword it at the next
  release, when its metadata changes anyway.
- `rocq.md` still says `excess` is `None` "under an exponential that
  contains the atom (`bang` keeps `(0, 0)` alone)"; that parenthesis is
  the accurate part.

## Session 54w-a: the page's text and the rule hints

Session 2 of `plan/54w-text.md` (Opus 5.5 at `high`, background job,
2026-11-09), in the web client's jj workspace
`../linlog-prover.github.io-54w-a` (workspace `54w-a`) on its `main`
(`vpkkppkk`, "flake.lock: Update"). Nothing was pushed by the session: after "client ready",
deduxit-orchestrator pushed the chain, and the client's `main@origin` is
`qyxnytmo` (`223ba9ef`). It accepted the empty stage's example line and
"Logic" for "Calculus" as text.

### Outcome

Every text the page shows was read against the writing rules and the
notes' section 7, and changed where it broke them; every rule label now
has a hint; the client has its own copy of the writing rule. The page's
behaviour is unchanged but for the hint (below), and nothing of W1-W15
was built.

The chain on the client's `main`, oldest first (change ids; commit ids
as the signing hook gave them last):

| change | commit | what |
|---|---|---|
| `zmrylklr` | `3cdfaa55` | `.claude/rules/writing.md`, a copy of deduxit's: the four rules word for word, the scope and reader paragraphs the client's; no `paths:`, so it loads in every session. CLAUDE.md points to it ("## Writing"), `claude-infra.md` lists it |
| `yvorvkto` | `0514c81b` | refusals, verdicts, hints and settings say what to do next: each refusal sentence ends with the next action ("Apply their rules first.", "Choose another split."); "a split of the context" became "a split of the goal's formulas"; the third hint rung ends "Press Hint again to apply it.", the fourth asks "Can you say why this was the step to take?"; Unknown ends "try a larger budget"; the saturation and the ordinary not-valid sentences split; the invertible switch's tip in two sentences; the budget's "about under one second" fixed; the export details name what each file needs (ebproof, cmll and amssymb; curryst; NanoYalla or rocq-deduxit) |
| `lqpkmrwv` | `2c57ab6b` | Learn and the tour: lesson 1's opening cut to the goal, the dual and the axiom (one idea before its worked proof; the rewrite of `A ⊢ A` stays in its explain answer), `choices: ["ax", "no rule"]`, its hinge's `⊤`/`?w` clause replaced; De Morgan, the double dual and the classical rewrite moved to lesson 2; `A -o B` as `A⊥ ⅋ B` moved to lesson 6; lesson 7 in the page's words (Proved, checked; Unprovable; Unknown) with its reasons as a list; lesson 8 defines MLL, MELL and the switching condition; lesson 11 drops the command line for "click the mark beside the sequent field and choose Ordinary"; the tour's stop 7 names the shapes (check, cross, question mark), stop 8 drops "branch"; Practice says only provable problems are there, what counts as proved (W11's rule as it is today) and what difficulty counts; About leads with what the page does |
| `ummswzmk` | `ba017bf2` | one name, "Logic": the mark's tip and accessible name ("Logic: …") and the Status row; the empty stage gets a syntax line ("For example A * B \|- B * A: * is ⊗, …", the ordinary signs in an ordinary tab: FW3's text) |
| `vupupxnp` | `36154fc8` | comments: no "the author's D13", no deduxit `plan/` paths, no other repositories named in `modules/*.nix` and `derive.py` |
| `xxvkxvzm` | `1cfcdf51` | the rule hints (below), with `src/lib/rules.test.ts` and `tests/hints.spec.ts` (six tests) and a tap test in `tests/phone.spec.ts`; the client's `web.md` describes them |
| `ktynqmtn` | `226a7661` | README leads with what the page does, then "Using the page" in four steps, then a short list of features; the 60-line bullet list is gone |
| `qpwpwvvk` | `fc5b37f8` | jargon out: "context" in six problems' bodies (lesson 3 now defines it at first use), "rung" (Learn, the tour), "Proved, not grafted" became "Proved, but not added to the derivation", the net look "deduxit" became "Default"; reference.md's hover sentence; llms.txt's summary shortened |
| `mxtqnvkw` | `bb90409f` | the review's hint findings: a popover rule's hint stands beside the popover, never over its list (it covered the next rule and took its click, which `ordinary.spec.ts` caught); the popover and the drawing look rules up in separate lists; every hint and every rule's `says` in `src/content/rules/` is one sentence, which the unit test now checks; LK's and LJ's `→L` and LJ's `¬L` in Gentzen's split form |
| `qyxnytmo` | `223ba9ef` | the review's text findings: the Rocq detail per suite (NanoYalla for classical linear logic without Mix, weakening, cuts or quantifiers, rocq-deduxit the rest; nothing over `Prop`), LaTeX's amssymb, Typst's own layout when tall, lesson 7's "three reasons", lesson 6's "calculus's switch", reference.md's tap |

### The rule hints

- Where: every rule of the popover (hover, focus, and an "About <rule>"
  button with an ⓘ for a tap, since a press on the rule applies it);
  every label of the derivation (a transparent button `.lb` laid over
  deduxit's SVG label as the formulas' buttons are, under them, at least
  a control's size on screen; read off the drawing, so the two-sided
  names come out as drawn); Learn's "The step is …" lines and the
  refusal's last step (`RuleName`, a button); Learn's prediction buttons
  on hover and focus.
- What: the label and the full name ("⊗R: tensor, right"), the schema
  (the reference's `Schema`), one sentence. Screen readers get the same
  words as the label's description (`aria-describedby` to a hidden span
  per rule). Escape closes the hint alone; the pointer may rest on it;
  a tap keeps it until a tap elsewhere.
- Every suite: the reference's one-sided rules and ILL's two-sided ones
  from `src/content/rules/`; tables in `src/lib/rules.ts` for classical
  linear logic drawn two-sided (deduxit's `ClassicalRule` names, `~L`
  and `~R` among them), Mix, affine `wk`, the Lambek calculus, LK and LJ
  (G3's forms, `→L` and `¬L` in Gentzen's split form since grafts draw
  them so). `rules.test.ts` holds every label deduxit draws or offers,
  in each calculus, to a hint with a form and one sentence.
- It replaces the Learn-only schema tip of popover rules (`data-rule`,
  `ruleNamed`, both removed): the hint shows with Learn on and off.

### The checks

| check | result |
|---|---|
| Vitest (`tests`), capped on cores 12-15 | 579 passed, after the review's fixes |
| `lint` (Biome, `astro check`) | 0 errors |
| e2e of the touched specs (hints, phone, lessons, prover, ordinary, targets, dismiss) | 85 passed, after the review's fixes |
| the client's `nix flake check --keep-going --max-jobs 4 --cores 8`, under the machine lock, capped on cores 12-15 | exit 0 at `qyxnytmo` (`223ba9ef`): 579 unit tests and 151 e2e tests passed, every check built |

Two earlier runs of the flake check failed and were fixed at their
cause: treefmt (two files edited from the shell, formatted), then seven
e2e tests: three still named the mark "Calculus" (changed with the
text), and the hint covered the popover's next rule (fixed by placing
it beside the popover).

### The review

The `fresh-reviewer` agent read `main..qpwpwvvk` against the prompt, the
writing rules and the notes (read only, nothing run). Eight findings:

1. F1, the hint forms of LK's and LJ's `→L` and LJ's `¬L` were the shared
   form, while deduxit's read-back draws them split. Upheld: the split
   form, and a sentence that a formula may go to either premise or both
   (`mxtqnvkw`).
2. F2, the Rocq detail named NanoYalla for proofs with cuts or
   quantifiers, and appeared on ordinary tabs. Upheld (`qyxnytmo`).
3. F3, lesson 7 said deduxit gives three reasons; the page has six.
   Upheld: "The most common:".
4. F4, LaTeX also needs amssymb; Typst's own layout has no curryst.
   Upheld.
5. F5, in a two-sided classical tab the popover's `?c`/`?w` showed the
   right-side forms. Upheld: steps and drawing look rules up in
   separate lists.
6. F6, hints of two sentences, and a test that did not check one. Upheld:
   every hint and `says` is one sentence; the test strips formulas and
   allows one full stop.
7. F7, "one name, Logic" only partly: the dialog's fieldsets are still
   "Calculus" and "Suite", lesson 6 said "the calculus's switch". Lesson
   6 fixed; the fieldsets kept (a departure, below).
8. F8, the prediction buttons have no tap path to the hint, and
   reference.md promised a tap everywhere. reference.md now says where a
   tap works; the predictions keep hover and focus (a departure, below).

### Departures from the prompt

- The Logic dialog's accessible group names stay "Calculus" (the
  outer fieldset) and "Suite" (linear or ordinary): the inner groups are
  already named "Logic", so renaming the outer one would give two groups
  of one name, and the tests find the switches by these names. The
  visible names (the mark's tip, the Status row, the texts) say "Logic".
- Learn's prediction buttons show the hint on hover and focus but not on
  a tap, since a tap predicts; the step's rule after it (`RuleName`) and
  the drawing's label have the tap.
- The lessons' structure (worked proof, fading, practice, hinge) is
  unchanged: `worked-example-fading-designer`'s sequence is the one the
  lessons already follow, and moving parts is W13. The education skills
  used: `cognitive-load-analyser` on lesson 1 (eight elements in the
  opening, three used by its example; the cut above) and
  `ai-feedback-design-principles` on the refusals and hints (task and
  process feedback, no praise; its one fix, saying that the next Hint
  applies the step, is in `yvorvkto`).
- The problems' bodies, the examples' notes and the rules' sentences were
  read and changed only where they broke a rule (jargon, two sentences);
  most were already plain.

### What the next sessions must know

- The client's `.claude/rules/writing.md` is a copy of deduxit's: a
  change to the four rules belongs in deduxit, the client and the Rocq
  library.
- A new rule label deduxit draws needs a hint: an entry in
  `src/lib/rules.ts` (or `src/content/rules/` for a one-sided or ILL
  rule the reference draws) and its label in `src/lib/rules.test.ts`'s
  lists, which fail without it. Every hint is one sentence.
- The hint finds the drawing's labels as the next sibling of each `g#i<n>`
  (a text, or a group for a subscript); a change of deduxit's SVG there
  silently drops the label buttons (only `hints.spec.ts` would see it).
- If W1 (reasons in view) is accepted, the refusal sentences already end
  with the next action and stand alone; the hint's opening on tap is
  the precedent.
- Accessible names changed with the text: the logic mark's button is
  "Logic: …" (tests find it by `/^Logic/`); the net look button
  "Default"; the popover rows have an "About <rule's full name>" button,
  so a test that counts the popover's buttons counts rows instead.

### Repositories and workspaces

- The web client: the workspace `../linlog-prover.github.io-54w-a`
  (name `54w-a`), the chain above.
- deduxit: the workspace `../linlog-step54w-a` (name `step54w-a`) on
  `main` (`uwvntrmx`), this section only.
