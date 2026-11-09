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

## Session 54w-e2: the comments of core/src/linear/search and core/src/linear/nets

Session 6 of `plan/54w-text.md` ("54w-e and on") for the proof search
and the proof nets (Opus 5.5 at `high`, background job, 2026-11-09), in
the jj workspace `../linlog-step54w-e2` (name `step54w-e2`), started on
`main` at `lytrpkmw` (`d5443cc4`) and rebased onto `main@origin`,
`ntmtoowo` (`106f56fc`, "Keep sessions out of EnterWorktree, which asks
the author"). Nothing was pushed by the session.

### Outcome

Every doc comment and comment of the 59 files under
`core/src/linear/search` and `core/src/linear/nets` is rewritten to
`.claude/rules/writing.md`: 10 323 comment lines before, 12 311 after
(7 565 lines added, 5 540 removed). No code changed: no item,
signature, type, attribute, string literal or behaviour (the method is
under "The checks"). The text was written by eight writers (Opus 5.5 at
`high`, sub-agents), each over a disjoint group of files, briefed with
the writing rules, the module's rules files and a calibration example
(`search/work.rs`, rewritten first by the session); none of them ran a
build. The session then checked every claim a writer said it had
changed, fixed what the checks found, and had the `fresh-reviewer`
review the chain.

What a reader gets:
- A doc comment opens with what the item is or does for its caller; the
  long sentences of colons, semicolons and nested parentheses are split
  into sentences, paragraphs and lists; a focusing, Petri-net,
  inverse-method or proof-net term (stable sequent, zone, copy bound,
  marking, firing sequence, subsumption, switching, axiom link, …) is
  said plainly at its first use in a file.
- Every invariant, bound and soundness argument is kept; a measurement
  that is the reason for a choice stays, said as the reason.
- Nothing refers to how the code came to be: "the spec", "used to",
  "when this was written", "a panel found", "before the swap" and the
  like are gone (the conventions pattern finds nothing in the two
  modules).
- Doc examples where a caller wants one, each run by the doc tests:
  `prove_within` (three stops: never, a deadline, and a deadline for the
  search alone that lets the refuters and the check run, which is the
  command's behaviour), `engine_for`, `Options` (a longer builder
  chain), `Outcome`, `Verdict` (matching all three verdicts, printing a
  disproof with `{d}`), `Disproof`, `Disproof::check` (a disproof read
  back from JSON, and a tampered one refused with
  `invalid_refutation`), `batch::run`, `search::Pool`, `nets::exist`,
  `ProofStructure::from_proof`, `is_correct`, `is_essential`,
  `sequentialize`, `nets::cut::eliminate`, `MellStructure`,
  `MellStructure::exist` and `MellStructure::from_proof`; and a "JSON"
  section on `MellStructure`, checked against `serialize/nets.rs`.

### The commits

Oldest first, change ids (the commit ids are those after the last
rebase; the signing hook gives new ones at each rewrite):

| change | what |
|---|---|
| `xwqtolqt` | the search's front door: `search/mod.rs`, `memory.rs`, `work.rs`, `additive.rs` |
| `lykupswp` | the refutations and their checks: `refutation.rs`, `weighted.rs`, `classical.rs`, `saturated.rs` |
| `xporutvm` | the batch and the pool: `batch.rs`, `parallel.rs` |
| `upqqmwwy` | the focused engine: `search/focus/` |
| `puvywxxl` | the Horn engine: `search/horn/` |
| `rvnwxnup` | the inverse method: `search/inverse/` |
| `orukmxqk` | the reference provers and the generator: `reference.rs`, `reference_fo.rs`, `generate.rs` |
| `uppkywsr` | the proof nets and their cuts: `nets/mod.rs`, `graph.rs`, `essential.rs`, `sequentialize.rs`, `skeleton.rs`, `nets/cut/` |
| `nspoklmu` | the MELL nets: `nets/mell/` |
| `tootqqym` | the net engines: `search/net.rs`, `search/essential.rs`, `search/planar.rs` |
| (this section) | the report |

### Wrong claims fixed

Each was checked against the code by the session or the reviewer.

The search's front door and its memory:
- `memory.rs`'s module doc measured against "the bound of the options";
  the bound is `Limits::memory_bytes`.
- `Stop::flags`: a worker's count adds its units in batches of
  `Work::BATCH` with `parallel` and the rest when it drops, not only
  "when the worker ends".
- `answer_goal`: a comment about allocations sat above the clamp of the
  threads and `held = account.used()`; it now says what those lines do.
- `Statistics::work` said only the net engine's handover adds its
  units; it is `work.done()`, every engine's, the inverse method's turns
  and handovers included.
- `Engine`'s doc on the inverse method's turns: no turn comes again
  after one that met the memory bound "or an index limit".
- The docs of the tests `dispatch`, `dispatch_intuitionistic` and
  `embedding_agrees_with_the_two_sided_engine` described the dispatch
  from before the net engine lost its row; they now say what each test
  asserts (`Focus` and `TwoSided`, `Additive`, a forced `Net`).
- `additive.rs`'s `memoized`: "fewer pairs than there are subformula
  pairs" is "at most as many" (`<= 255 * 255`).

The refutations:
- `RefutationFault::NotApplicable`'s `invariant` named two values; the
  code uses seven, so the doc gives examples.
- `saturated.rs`: `Calculus::positive`'s list lacked `∃`;
  `Calculus::outputs` returns a count, not the members.
- `classical.rs`: the work bound is checked before each evaluation, so
  the count never passes it (not "by at most one evaluation").

The focused engine:
- `memo.rs`: `Entry::code` is the number a record's two entry words
  hold, not "one word"; `insert_hashed` replaces a weaker entry, not
  "anything" (an old `Proved` stays against any failure, an old
  `Hereditary` against a new `Complete`).
- `keyed.rs` named a file `framed/key.rs` that does not exist; the keys
  are written in `framed.rs`.
- `schedule.rs`'s `FIRST_TURN`: the forward search gets it, the
  backward one `BACKWARD_SHARE` times as much, not "each of the two".
- `focus/parallel.rs`: the other workers stop at their next poll, not
  "at their next stable sequent".
- `split.rs`'s `Forced::Dual` comment gained the caveat
  `core-focus.md` records (the swap may move a copy to another branch).

The Horn engine and the inverse method:
- `horn/cover.rs`: `Cover`'s index is a trie over every place, not "by
  their first place"; its `work` counts the index's nodes and lookups
  too; `search` also polls at every marking computed.
- `horn/equation.rs`: `fraction` gives the last convergent within the
  bound (a semiconvergent can be nearer), not "the nearest fraction";
  `integers` returns `None` for a weight that is not finite or when all
  are near zero; `State::Yielded` also covers a tableau never built for
  lack of room.
- `inverse/mod.rs`: `Kept::rank` is below the number of sequents kept,
  not "below its id"; the units of `apply` count the linear members.
- `inverse/index.rs`: `Trie::insert` returning false leaves the nodes it
  made before.
- `inverse/tests.rs`: a comment named "two of the same tensor's factors"
  for `|- a & ~a`, the two sides of a `&`.

The reference provers and the generator (test-only):
- `reference.rs`: `contractions` returns trees only, not text as well;
  `judge` checks through `prove_within`, not `prove`.
- `reference_fo.rs`: `F` holds both calculi, not "a one-sided sequent";
  the doc of `random_ill_sequent` sat on `random_ill_text` (moved);
  both random sequents change "at most one" site, a word, not "a
  character", and change a predicate, not "a literal";
  `the_focused_engine_agrees` asserts that the engine proves some, not
  "what it is given"; `undo` also resets the eigenvariable counter.
- `generate.rs`: the conclusion is kept as trees, not as text; a twin is
  `1 ⊗ a`, not `a ⊗ 1`, and may be a `&` of two twins; `Ill` and `ill`
  also return the contractions.

The proof nets:
- `nets/mod.rs`: the doc "Essential nets: …" sat on `pub mod cut`, so
  the public `cut` page opened with it; it is on `essential` now.
- `ProofStructure::new` refuses more than 1 431 655 764 occurrences
  (`MOST`), not "past 1 431 655 763 vertices".
- `link` is logarithmic (two `find`s on a union-find by rank without
  path compression), not constant; `unlink` is constant.
- The errors of `link` lacked `Placement::Uncut`, those of `is_correct`
  `Crossing` (cyclic criterion), those of `exist` `Mode` in an ordered
  mode with Mix.
- A cut formula's occurrences that are no vertices lie "above no cut
  link's end", not "above every cut link" (five places, and
  `Placement::Uncut`'s doc in `nets/mell/mod.rs`).
- The skeleton joins the cut links too (`skeleton.rs`, `nets/mod.rs`).
- `sequentialize`'s proof is in the cyclic mode under a criterion of the
  cyclic order; `is_essential` asks the stop after the search for a
  cycle as well.
- The `Display` docs' samples (`A⊥ ⅋ B⊥`, `A⊥[3] — A[2]`) are not what
  the code writes (`~A ⅋ ~B`, the smaller id first).

The MELL nets and the net engines:
- `nets/mell/mod.rs`: `MOST` counts a slot per edge plus one where the
  parent edge is absent, and refuses at `u32::MAX` slots; a root lies
  outside every box (it has no parent); `exist` refuses the ordered
  modes.
- `nets/mell/sequentialize.rs` and `graph.rs`: the term's nodes include
  cut; `search` starts from the given roots and returns the number of
  trees; `an_unfolding_past_the_vertices_is_refused` builds 100 nodes,
  not "a few dozen".
- `search/net.rs`: `Prunes::PLANAR` applies the balance alone; the
  search also polls per failed exact test (and the essential engine in
  its closure's set-up); `Symmetry::new` reads `prunes.conclusions`;
  both engines' `admits` refuse a forest past the vertex bound; two test
  helpers' docs named parameters that do not exist; a comment said
  "every fourth link" above a run with a test period of 7.
- `search/essential.rs`: `reserve` makes room for one change and fails
  on the account or the allocator.

### The checks

| check | result |
|---|---|
| No code change: each of the 59 files and its version at `d5443cc4` with every comment and every doc attribute (`#[doc = "…"]`, `#[cfg_attr(…, doc = "…")]`, the examples' fences) removed and whitespace outside literals collapsed, compared | `checked 59 files against d5443cc4ec69, 0 with a code difference`, at every commit of the chain's last state |
| `cargo clippy --workspace --all-targets -- --deny warnings` (capped, cores 10–15) | exit 0 |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features` | exit 0 |
| the same with `--document-private-items`, with every feature and with the defaults (`-p deduxit -p deduxit-cli`, as the flake's `doc-private`) | exit 0, exit 0 |
| `cargo test --doc -p deduxit --all-features` | 53 passed, among them every new example |
| `cargo test --doc -p deduxit --no-default-features` | 1 failure, `sequents/fmt.rs`'s `Sequent::walk` example (not fenced on `parse`), a file this session does not touch: on `main` already |
| `nix fmt` on the 59 files | 0 changed |
| `gate` (capped, cores 10–15), at the report's commit, the chain's last | exit 0 |
| `nix flake check --max-jobs 4 --cores 8`, under the machine lock, at the same commit | exit 0 |

The first clippy and rustdoc runs found what the writers could not see
without a build: three doc comments followed by a blank line before
their item (`empty_line_after_doc_comments`), six explicit link targets
that rustdoc denies as redundant, and two module docs (`//!`) whose
short links did not resolve in the private docs (`Reading` in
`nets/essential.rs`, `Bias` in `focus/schedule.rs`, now full paths).
Each fix went into its file's commit.

### The review

The `fresh-reviewer` agent read the chain against the brief, the
writing rules and the code, checked every claim in the writers' lists
of fixes, sampled the public docs and the soundness arguments, and ran
the no-code check and judged its method. One finding, and one question:

1. `weighted.rs`'s module doc: "`!A` nothing" had become "nothing below
   zero", listed as a fix. The old claim was right: `!A` contributes
   exactly zero (the Rocq library's `out_oc`, and the code's
   `Low::From(0)` is the bound of that set). Fixed: "`!A` zero".
2. The question: the test comments' "when this was written" figures had
   become present-tense claims ("These seeds give 10 to 17 of 40")
   without a run. Answered by marking them as measurements ("Measured
   with these seeds: …", "would visit", "would take") in
   `generate.rs` and `focus/tests.rs`, which claims what the old text
   claimed.

Its verdict otherwise: no code change (the method sound for these
files: no `cfg_attr` mixes `doc =` with another attribute, no fence
changed, no doc-test line removed), no reference to history left but
the identifier below, every listed fix true.

### Departures from the prompt

- **The proof-net verdict's `Display` text** (`"proof net"`, `"proof net
  with Mix"`, …, in `nets/mod.rs` and `nets/mell/mod.rs`), which
  `plan/notes/usability.md` §7 gives to this session (FR2), is a string
  the command prints: left for 54w-d, which changes it with the manual's
  examples and the behaviour lock, as deduxit-orchestrator agreed.
- **Strings and identifiers are code**, so these stay as they are,
  although wrong or about history:
  - `Placement::clause()` for `Uncut` (`nets/mell/mod.rs`, near line
    189) says "lies in a cut formula above every cut link, and is
    linked"; it should say above no cut link's end. It is the error
    text of `NetError::Placement` (for 54w-d, or a follow-up).
  - The test `the_review_s_witnesses` (`search/essential.rs`) is named
    after a review.
- **The rules files were not changed** (the session touches no file
  outside the two modules). One note there looks wrong:
  `core-focus.md`, "Every proof passes the checker", says the generator
  reports "the most derelictions on one branch"; the code sums every
  branch (a `⊗` gives `lc + rc`, a `&` `2 * copies`), as the doc of
  `Provable` explains.
- **Five measured figures** in test comments ("when this was written")
  were not measured again; they are marked as measurements.
- The crate root's wrong claim on deciding what the command decides
  (`core/src/lib.rs`, usability L3) is the root session's; the
  `prove_within` example here shows the stop that does it.

### What the next sessions must know

- **The no-code check** used here: a lexer that drops `//` and nested
  `/* */` comments outside string, raw-string and char literals, drops
  the doc attributes (`#[doc = "…"]` and `#[cfg_attr(CFG, doc = "…")]`,
  CFG nested two deep), collapses whitespace, and compares each file
  with its base version (`jj file show -r BASE`). It stays in the job's
  scratch directory and goes with it; the method is all there is to it.
  A new doc example's fences are doc attributes, so a check that keeps
  attributes reports them.
- **Writers that edit by script bypass the formatter hook**; run `nix
  fmt` on the changed files before committing (it changed nothing
  here). Give each sub-agent a scratch directory of its own: two writers
  shared `/tmp/e2scripts/apply.py`, one overwrote it, and a third's
  replacements ran into the new version (it raised an error, and the
  writer redid them).
- **What only a build shows** in a comment-only change, and the writers
  missed: a blank line between a doc comment and its item, a link
  target the label already resolves to, and a short link in a `//!` of
  a private module, which may not resolve even where the name is
  imported. Run clippy and both rustdoc builds (public with every
  feature, private with the defaults) before the review.
- **The dispatch table** in `Engine`'s doc (`search/mod.rs`, "Which
  engine decides a goal") is parsed by
  `the_documented_dispatch_is_the_dispatch`: reword around it, never
  its rows.
- **Doubts left for whoever reads these areas next**, none shown wrong:
  `Statistics::memo_entries` sums the default bias's pair while
  `Statistics::add` takes the larger (both right in their case);
  `Answer::of_arena`'s comment maps any error of
  `Proof::with_instances` to an index limit; `random_sequent` says its
  texts stay well formed while `the_focused_engine_agrees` skips those
  that do not parse; `engines_agree_on_horn_programs`' doc says more
  than its asserts; the split cost "a seventh" (`split.rs`) against "up
  to a fifth" (`core-focus.md`).
- **The `doc tests without default features` failure** in
  `sequents/fmt.rs` (`Sequent::walk`'s example parses without the
  `parse` fence) is the sequents session's to fence.

## Session 54w-e1: the comments of core/src/linear/sequents, core/src/linear/parse, core/src/linear/occurrences, core/src/linear/fragment.rs and core/src/linear/proofs (the checker included)

Session 6 of `plan/54w-text.md` for two of its groups, `linear::sequents`,
`parse` and the forest, and `linear::proofs` with the checker (Opus 5.5
at `high`, background job, 2026-11-09), in the jj workspace
`../linlog-step54w-e1` (name `step54w-e1`), branched from `main` at
`lytrpkmw` and rebased onto `main@origin` (`ryuqpssm`). Nothing was
pushed.

### Outcome

Every doc comment and comment of the 43 Rust files under
`core/src/linear/sequents/`, `parse/`, `occurrences/`, `proofs/` and
`fragment.rs` is rewritten to `.claude/rules/writing.md`: about 6 200
doc-comment lines and 1 000 comment lines, 4 574 lines in and 3 479 out.
No code changed, apart from eight pairs of
`#[cfg_attr(feature …, doc = "```")]` / `doc = "```ignore"` lines that
fence a doc test on the features it needs, the crate's existing form.
Six Opus helpers each rewrote a disjoint group of files from one shared
brief. I committed each module's files as they were finished.

The chain, oldest first (change ids):

| change | what |
|---|---|
| `mtuvnkmq` | sequents (`mod`, `fmt`, `fo`, `name`, `notation`, `term`) and `fragment.rs`: `Sequent`'s reference split under headings, with a table of the connectives and tables of the JSON keys; `Mode` with a table of its nine names and what each allows; examples on `Mode` and `Fragment` |
| `kvryyopp` | the parser; an example on `Sequent::parse_within` |
| `zxvxtnsv` | the occurrence forest, the sets, the written sides, `IllSequent` (now said to be the way to read intuitionistic input, with an example), the instances and the intuitionistic reading |
| `nmmmrxsz` | proof terms (`proofs/mod.rs`), the rules, the multiset, the rule labels (`style.rs`) and the text tree (`fmt.rs`); `Proof::derivation`'s example reads its input with `IllSequent` |
| `vtytxxll` | `Sequent::walk`'s example fenced on `parse` (it failed `cargo test --doc --no-default-features` on main; found by 54w-e2, asked by deduxit-orchestrator) |
| `szpxvpyl` | the checker (`check/`), the oracle and the first-order proofs' tests; an example on `CheckError` showing a verdict and a refusal |
| `wpxsrkpy` | cut elimination, the size estimate and the derivation view; examples on `ViewOptions`, `Derivation`, `Proof::derivation_size` and `cut::eliminate` |
| `rzouqrml` | interactive proving; the module doc shows the shortest session |
| `oruknsqk` | the reviewer's finding, and an unsupported reason dropped |
| this one | this section |

### Wrong claims fixed

Sequents and modes:
- `name.rs`: atom names were said to avoid words "reserved for a later
  version"; `forall` and `exists` are keywords today.
- `Display` for `Formula` and the intuitionistic printer bracket a nested
  quantifier too, not only binary formulas.
- `Sequent::optimize` keeps the terms cut formulas reach;
  `occurrences` counts cut formulas.
- `walk` and `formula` panic on a foreign id when iterated or printed in
  a release build, not at the call (only a `debug_assert!` is there).

The parser:
- `owed` and `prefix` count binders as well as `!` and `?`;
  `implications` records `\` as well as `⊸`; the right operand of `/`
  is negated too; an identifier before `(` is a function symbol even
  where a binder has that name (the code tests `(` first).
- `Sequent::parse_within` can refuse with `Refusal::Terms`.

The forest and the readings:
- `IllSequent::read_one_sided`'s example gave a one-sided form that was
  not one-sided (`⊢ ~(A ⊗ ~B), ~A, B` for `⊢ A ⊗ ~B, ~A, B`); the
  `goal` field is the root on the output side, not the last root; the
  roots' order is the parse's only for parsed input.
- `Polarity` and the reading's position lists omitted `∀` and `∃`.
- `Forest::is_empty` and `has_binders` cover cut formulas.
- `Reading` can find two possible goals whenever each of the two roots
  holds a `⊤` or `0`, not only in roots built from them (`a ⊗ ⊤`);
  `Reading::new` can fail with `ShapeError::CutFormula`; `ShapeError`'s
  doc quoted a message the code never writes.
- `instances.rs`: a canonical table's terms may take open arguments; the
  frame check catches them. A metavariable as a table term is refused.
- `IllSequent`'s new text (the reviewer's finding, below).

Proof terms, rules, labels, the text tree:
- `Proof`'s JSON and the module's rule list omitted the cut node and
  the quantifier rules; `Named::name` and `style.rs`'s symbols omitted
  the quantifiers; `style.rs` claimed to say why a derivation was cut
  short, which it does not.
- `Place::bar` is `None` only for an open goal drawn bare;
  `write_steps` ends an open goal's line in ", open.".

The checker:
- `check_within`'s doc named a `memory` argument that does not exist;
  the bounds are `limits.memory_bytes`, `limits.work` and the stop,
  which is asked every `POLL` nodes or `POLL_WORK` units of work; the
  work count includes the term walks of `bring` and the axioms' frames.
- The cut fills `shared`, `absent` and `needs`; a cut frame's
  eigenvariables are walked by `Instancing::bring`, not the frame pass.
- The zone bounds count members, each a `u32`, which may be instances
  past the forest (the bounds hold unchanged).
- A refusal before the first node names node 0; a saturated zone ends
  in `Fault::Surplus`, a verdict; in intuitionistic mode Mix is
  forbidden outright.
- The oracle's memory is not quadratic (it keeps `BTreeSet`s and sorted
  multisets); its `proofs()` includes the Lambek proofs; a parameter
  name and a missing eigenvariable check in its docs.
- Test docs: a test said 4 001 nodes poll once (fewer than `POLL`, so
  never); another claimed to check "the height exactly".

Cut elimination, sizes, the derivation view:
- `Derivation::new` and `two_sided` are bounded by the limits, not the
  view, and ask the stop once per node unfolded; `derivation_size` is
  bounded by `limits.work` too and polls by work as well as nodes.
- `cut::Options::drawn_bytes` bounds eliminations on proofs too (the web
  bindings use it so); at first order `Rewrite::apart` also asks the stop.

Interactive proving:
- `!` needs a context of `?` formulas, not an empty one; the rules
  offered include `∀` and `∃`, and `∀R`/`∃L` are invertible; a comment
  said a session holds no binder, which first-order sessions do;
  `framed_premises` also fails on the session's bound on terms; `rebase`
  moves only its own map; the first-order bound also caps the table.

### The checks

All at the chain's head after the rebase onto `main@origin` (`ryuqpssm`), compared with `main@origin`.

| check | result |
|---|---|
| No code change: the 43 files, comments stripped (a small lexer that knows strings, raw strings, chars and nested block comments), against `main@origin` (`strip_compare.py`); then keeping each line's indentation (`strict_compare.py`) | only the eight fence pairs, in six files, differ (one split over four lines by rustfmt); with `--ignore-doc-test-fences` 43 of 43 identical, exit 0, both tools. The lexer was shown to catch a hidden `==` → `!=` beside a comment edit |
| Only in-scope files changed from the base | none outside the five paths and this report |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features`, and `-p deduxit --document-private-items` | exit 0, exit 0 |
| `cargo test -p deduxit --all-features --doc`; `--no-default-features --doc` | 63 passed; 5 passed, 44 ignored (54w-e2's examples included); exit 0 both |
| `gate` and `nix flake check` | reported with their exit statuses in the message to deduxit-orchestrator, run at this section's commit |

### The review

The `fresh-reviewer` agent read the chain against the brief, the
writing rules and the code (facts, the rules, no code changed): one
finding and one question, both answered in `oruknsqk`.

1. `IllSequent`'s new doc said text with a classical symbol, parsed as a
   `Sequent`, is refused when proved, with a subformula's number. False:
   `~(A * ~B), A |- B` has the reading `A ⊸ B, A ⊢ B` and is proved, and
   a refusal can be any `ShapeError`. Fixed: it says both.
2. The question: a comment in `quantified.rs` had gained a reason
   ("keeps reading linear") that the code does not support. Dropped; the
   comment says what `main` said.

The reviewer found the commit messages' fixed claims, the public items'
docs, every integer-range and memory-bound statement of the checker,
and the doc tests sound, and agreed by a second method that only fence
pairs changed as code.

### Departures from the prompt

- Eight `cfg_attr` fence pairs are attribute lines, not comments. A new
  doc test that parses needs one under the crate's feature set, and one
  fixes a failing doc test on main; the strip-compare lists them.
- The two groups of session 6 ran as one session, its files split among
  six helpers by module (the goal's scope).
- One helper edited by exact string replacement from small scripts
  instead of the Edit tool, so the format hook did not run on those
  seven files; `rustfmt --check` was clean, and the flake's `treefmt`
  check covers it.
- Code text was left as it is (the brief): messages and test names are
  listed below for whoever owns them.

### What the next sessions must know

- `.claude/rules/core-sequents.md` (the paragraph on `Fragment`'s
  quantifier bit) still says no engine admits the bit and the dispatch
  answers `NoEngine` for first-order fragments; the focused engine
  decides `Fragment::LL1` (`core/src/linear/search/focus/mod.rs:154`).
  A rules file is outside this session's scope: fix it with the search
  sessions' notes.
- `core-sequents.md` splits the parser's inlining measurement (1.5 % for
  the loop out of line, 2 to 7 % for a copy per reader) differently
  from the comment in `parse::read` ("1.5 % to 7 %"), which was kept.
- Possible bug, unconfirmed: `Sequent::join_first_order` takes
  `self.fo.atoms.len()` as this side's atom count when `self.fo` is not
  empty; a JSON sequent with binder names but no atom predicates would
  then count 0 own atoms and could hand its atoms the other side's
  predicates. Only such a document reaches that state.
- Messages that read badly but are code (the crate's `Display` texts):
  `ShapeError`'s "(⅋ only as A ⊸ B, that is ~A ⅋ B, and ? only under a
  negation)" and its "on the right of ⊢" beside "right of ⊢";
  `canonical`'s "term {id} is a metavariable, an open witness" and
  "frame {n} pushes no closed term"; `CheckError`'s "the premises
  differ", "is not under a hypothesis's !", "stands under a binder and
  is named without its instance", and "occurrence" for what may be an
  instance; `StepError`'s "the witness is no term here", "which the
  sequent has not with this arity and role", "is no symbol of the
  sequent", "symbol {} is none of the sequent"; `make_premises`'s "a
  u32 numbers"; `replay_cut`'s and `replay.rs`'s refusals of a
  malformed session. The test names `agrees_with_the_first_implementation`
  and `the_first_implementation_agrees` keep the history wording.
- To show that a comment-only change touches no code: strip comments
  with a lexer that keeps string literals verbatim, drop blank lines,
  and compare with the base, once per line stripped and once keeping
  indentation; a doc-test fence is the only attribute such a change may
  add. The two tools are in the job's scratch directory, gone with it.
- New doc tests use the crate's fence (`#[cfg_attr(feature = "parse",
  doc = "```")]` and its `not` twin; `#![…]` in a module doc), not a
  hidden `fn main` gated by `cfg`: both work, the crate uses the first.
- A fence whose `cfg` is long (`not(all(feature = "parse", feature =
  "interactive"))`) passes 100 columns, and rustfmt splits it over four
  lines; the flake's `treefmt` check failed on one such line here until
  `nix fmt` reformatted it. Run `nix fmt` on a file edited outside the
  Edit tool before the flake check.
