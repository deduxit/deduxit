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

## Session 54w-d: the command's help, messages and errors

Item 5 of `plan/54w-text.md` (Opus 5.5 at `high`, 2026-11-14, cores 4-6,
jj workspace `deduxit-wt-step-54w-d-96dffb`), the author away: every
decision below was taken by the writing rules, the code's own idiom and
the tests that read the help, each with its reason.

### Outcome

The command's `--help` and `-h`, its own messages and hints, the
library's messages that the brief lists, the proof-net verdict and the
refutation reasons are rewritten to `.claude/rules/writing.md`. No flag,
default, exit status, JSON key or API changed. The chain, oldest first:

| change | what |
|---|---|
| `twqqvqwm` | the help (`cli/src/argument_parsing.rs`'s doc comments) |
| `uyuzkrxm` | the library's messages, with the manual's, the guide's and the tests' copies |
| `krwwwpxo` | the command's own messages |
| `zpqwnvmy` | the behaviour lock's changed lines, alone |
| (this section) | the report |

Nothing is pushed.

### What was rewritten

- **The help.** Every command's long help is split into short paragraphs
  of one task each (`prove`: deciding, a batch, its lines, its exit
  status; `eliminate`: the steps, the cases, the statuses, `--net`;
  `interact`: starting, the session commands, the rules, the statuses).
  Every flag whose help was one long paragraph, and so one long line of
  `-h`, now has a short first line and the rest under `--help`
  (`--settings`, `--cyclic`, `--lambek`, `--net`, `--output`, `--lambda`,
  `--structural`, `--style-file`, `--invertible`, `--workers`,
  `--batch-memory`, `--at`). `--style`'s field list is a paragraph per
  format. Passive and "is X'd" phrasing became active where it read
  better ("the command finds the sequent's fragment", "Give the sequent
  as an argument"), and words like "detected", "rather than",
  "alternating" went.
- **The library's messages**: `ShapeError` (`⅋ only in A ⊸ B (~A ⅋ B)`,
  "write the sequent with both sides", "right of ⊢" throughout),
  `CheckError` ("the rule is not allowed in this mode", "the rule does
  not act on F", "the premises of & do not have the same context", "the
  premise of ! has formulas besides the promoted one that are not ?
  formulas", in an intuitionistic check "the premise of !R has a
  hypothesis that is not a ! formula", "the two premises do not have the
  same context", "lies under a quantifier but is named without the terms
  of its instance", "formula" for "occurrence", "the check stopped at node
  N without a verdict"), `StepError` (the split, the eigenvariable, the
  cut symbol, the exchange, the witness), the witness reader's, the
  session replay's and `replay_cut`'s refusals, `canonical`'s ("term N is
  an open witness, which a proof cannot hold", "frame N adds no closed
  term"), `make_premises`' reason ("as many eigenvariables as it can
  number"), and `lambda::NoTerm`'s classical and quantified reasons.
- **The proof-net verdict** (`ProofStructure` and `MellStructure`'s
  `Display`): `proof net (the correctness criterion holds)`, `proof net
  with Mix (the correctness criterion with Mix holds)`, `planar proof net
  (the correctness criterion holds, and no two links cross)`; `not a proof
  net: …` as before. The SVG drawings' description carries it.
- **The refutation reasons** (`refutation.rs`): the intuitionistic count
  speaks of sides as the two-sided classical one does ("A occurs 1 more
  time on the left than on the right, so they cannot all meet in axioms
  (an implication's antecedent counts on the other side)"), and the
  saturated set reads "its N sequents are closed under the rules, and
  none of them yields the goal".
- **The command's messages**: a rule that does not act on a formula is
  now refused with the formula and the command that helps (FC6: "the rule
  ⅋ does not act on formula 2, A ⊗ B; `rules 0 2` lists the rules that
  do"); ordinary logic's "not valid" names "the translated sequent"
  (FC9), and `--logic`'s help defines the image; an empty input, a
  settings file given as `-`, TPTP without `--logic`, a style no format
  has, and `--logic` on a format it does not read say what to write.

### Claims found wrong, and fixed

- `interact --help` said `proof … [FILE]` writes JSON for `check` "with
  a FILE and no format": it writes the format the file's extension names,
  and JSON only where the extension names none (the session's own `help`
  says so); `show` likewise writes text then. It also left out the session
  commands `invertible on|off` and `lambda`, which `help` lists.
- `Placement::Uncut` said the vertex "lies in a cut formula above every
  cut link": the variant is an occurrence above *no* cut link's end, so
  no vertex (its own doc comment says so). Now "lies in a cut formula
  above no cut link, so it is not a vertex, yet a link names it".
- `NoTerm::Classical` said a classical proof "has several conclusions":
  `⊢ 1` has one. Now "may have several conclusions …; a term needs an
  intuitionistic proof" (FR3: the line says where a term comes from; it
  names no flag, since it is the library's text and the page shows it
  too).

### Decisions

- **The settings key stays in brackets at the end of a flag's first
  line** (`(search.copies)`): `--settings` tells the reader that the flags
  name the keys there, and `cli/tests/readme.rs` holds every key the guide
  names to the help. Moving keys to the long help alone would have hidden
  them from `-h`.
- **What the tests read in the help stays word for word**: the batch's
  order of exit statuses ("an error (2) before unknown (3) before
  unprovable (1) before proved (0)"), the defaults in the forms
  `guide_states_the_defaults` reads (`cbn (by default)`), `rocq:` in the
  style list (`the_style_help_names_every_option`), and no hidden flag in
  a short help (`short_help_keeps_to_the_common_flags`).
- **The net verdict keeps its first words** and adds whose verdict it is
  in brackets (FR2), so a script that looks for a line starting `proof
  net` still finds it; one that matches the whole line `proof net` does
  not, which the changelog should say (below).
- **FC6 is the command's**: `StepError::Rule` names a position only, and
  giving it the formula would be an API change; the session has the goal,
  so `apply` names the formula and `rules G P`. The page can do the same.
- **"the search was exhaustive" stays**: it is the refutation every other
  phrase falls back to, plain enough, and quoted across the manual.
- **The intuitionistic refutation counts by sides without saying "A on
  the left has no A on the right"**: the classical phrase needs `placed`,
  a pass the intuitionistic writer does not make; adding it is a code
  change (a proposal below).

### How no behaviour changed

- **The code**: a lexer (in the scratch directory, gone with it) drops
  comments and doc attributes, replaces every string literal by one
  token, and compares each changed `.rs` file with its base
  (`jj file show -r @-`). Of the 24 files, all but four have no change
  beyond comments and string texts; the four are the FC6 arm in
  `cli/src/interact/mod.rs` (a `map_err` that builds the message);
  `CheckError`'s `Kind` arm, whose `write_str` moved before the formula
  ("the rule does not act on F"), and its `NotEmpty` arm, which picks its
  text by `reading.is_some()` as `NotDual` beside it does (the reviewer's
  first finding); and braces around the longer strings of the nets'
  verdict and `Placement::Uncut`.
- **The help**: every `-h` and `--help` of the nine commands, captured
  before the change from the base binary and after, compared by their
  option lines, `[default: …]`, `[possible values: …]`, value lists,
  headings and exit status: the same on all eighteen.
- **The outputs**: the behaviour lock's diff (`zpqwnvmy`) changes text
  lines only; no `--- status` line moved.

### Checks

Every run capped (`systemd-run --user --scope -p MemoryMax=8G -p
MemorySwapMax=0 taskset -c 4-6`, three build jobs and test threads); the
flake check under `flock -o /tmp/linlog-machine.lock` with `--max-jobs 4
--cores 8`.

- `cargo test --workspace`: the first run after the rewrite failed
  exactly on the tests, snapshots and examples that pin the old phrases
  (the lock, `readme`'s guide examples, the SVG snapshots' `<desc>`, the
  core tests of the reading, the checker and the nets, the bindings'
  eigenvariable test, one help test that reads `rocq:`); each was changed
  with its message, and the second run passed all but the lock, which
  was then blessed in its own commit.
- `cargo test -p deduxit-cli --test cli --test readme` at the library's
  commit `uyuzkrxm` alone, in a temporary jj workspace at that revision
  (removed after): 59 and 7 passed, so the commit stands without the
  command's.
- `cargo clippy --workspace --all-targets -- --deny warnings`: clean.
- The gate at the rebased chain's head (`zpqwnvmy`): "gate: passed". Its
  note that the cache model's misses moved on `search-qbf-20-2` and
  `check-qbf-20-2` is the same note 54u-b's gate gave; no journey runs a
  line this chain changed but the texts written on a refusal.
- `nix flake check` runs at the report's commit, the chain's last; its
  result is in the session's final message, since a line written here
  after it would make another commit.

### The reviewer

`fresh-reviewer` (Fable 5.2 at `medium`, its own definition) read the
chain, the brief, the prompt and the code, compared the help screens
before and after and ran the built binary on small inputs. It found
three points; all held and were fixed in the commit they belong to.

- **Two checker faults stated something false in an intuitionistic
  check**: "the premises of & …" under a `⊕L` node, and "… are not ?
  formulas" for a `!R` whose extra formula is a hypothesis that is not a
  `!` formula (its witnesses were hand-made proof files checked with
  `-i`). `Differ` now says "the two premises do not have the same
  context", true of every `&` node, and `NotEmpty` speaks of `!R` and a
  hypothesis under the reading, as `NotDual` and `NotUnderQuest` beside
  it already did.
- **"an intuitionistic proof has a term" was false**: an intuitionistic
  proof with a quantifier, with Mix or of the Lambek calculus has none
  (the manual's next example shows one). Now "a term needs an
  intuitionistic proof", which is true.
- **"A small sequent is so decided at once"** in `--jobs` read badly; now
  "So a small sequent is decided at once …". The lock's two help files
  were blessed again in the lock's commit.

Everything else it checked held: the help's facts against the code
(`--deterministic`, the forward search's bound, `interact`'s `proof
FILE`, the session commands), `Placement::Uncut`, the intuitionistic
refutation's sides, the new `map_err` in three modes, the help screens'
flags, defaults, values and headings, and the lock's diff.

### Proposals for the author

- The intuitionistic count refutation could say "A on the left has no A
  on the right" where the side with fewer literals has none, as the
  two-sided classical one does: `placed` run under the reading in
  `refutation.rs` (code, not text).
- `StepError::Rule` could carry what the formula is (its connective), so
  that every front end names it without the session's help; today the
  command adds it.
- The root's `Error::GoalOutputs` still says "on the right of ⊢" and
  `ordinary::interactive`'s refusals were not in this brief; 54w-e4 and
  54w-e5 meet them.

### What the next sessions must know

- **54w-c (the changelog's Unreleased section)**: two outputs a script
  might match changed: `--net`'s last line (`proof net (the correctness
  criterion holds)`, and its Mix and planar forms) and the intuitionistic
  refutation (no "positively"/"negatively"). Ordinary logic's "not
  valid" says "the translated sequent" for "the image". The manual's and
  the guide's examples of each are changed already.
- **The client (54u-d's e2e tests)** may assert the old net verdict, the
  intuitionistic refutation or a step's refusal (`cannot stand there` is
  now `cannot be used here`); the bindings' own test changed with it.
- **The help's constraints** are in the decisions above: a text session
  that rewrites `argument_parsing.rs` again runs `cargo test -p
  deduxit-cli` (the help tests, the readme's names and defaults, the
  lock) before anything else.
- **`cli/src/interact/mod.rs` builds FC6's message** from the library's
  `StepError::Rule`; reword the library's phrase and the command's
  stays as it is (it does not match on it).

## Session 54w-e3: the comments of export

Item 6 of `plan/54w-text.md` for the module `export` (Opus 5.5 at
`high`, 2026-11-14, cores 7-9), in the jj workspace
`../deduxit-wt-step-54w-e3-5faac9`, started on `llmypmxy` (the reviewed
chains not yet pushed) and rebased onto `main@origin` at the end.
Nothing was pushed.

### Outcome

Every doc comment and comment of the 14 files under `core/src/export/`
is rewritten to `.claude/rules/writing.md`: 2 247 comment lines before,
3 002 after (1 952 lines in, 1 179 out). No code changed (the method is
under "How no code changed"). I wrote the module docs of `export`,
`notation.rs` and `styles.rs` first, as the calibration; four writers
(Opus 5.5 at `high`, sub-agents, no builds) took the rest in disjoint
groups of files. I checked each writer's list of fixed claims against
the code, ran every check, and had the `fresh-reviewer` read the chain.

What a caller gets:
- `export`'s module doc opens with a table: what to get (a LaTeX or
  Typst proof tree, an SVG drawing, a net, a PNG, a PDF, a Rocq
  certificate), the one call that gives it, and its cargo feature, then
  an example with `latex::to_string`, a section on `write` (writing as
  you go, with a stop) and a list of what every format shares.
- Each format's module doc opens with a table of its own calls,
  `to_string` first, and its example calls it: LaTeX, Typst and SVG
  (with a proof, a sequent and a proof net), Rocq (a NanoYalla
  certificate, and a whole file for deduxit's library), PNG and PDF
  (`no_run`: they read a font file).
- Rocq's module doc says which kernel a certificate needs and when the
  proof picks deduxit's library, what neither kernel takes, and how to
  check a certificate (`nix run github:deduxit/deduxit#check-certificate
  -- proof.v`, with its exit statuses).
- `svg::Style`'s doc is the reference: how to build one, the units, the
  maxima, the JSON form, an example; each field gives its meaning, unit
  and default (checked against `impl Default`).
- Measurements that are a reason stay, said as measurements ("Measured:
  …"); history is gone ("hid what followed it" is now what would happen
  without the code).

The chain, oldest first (change ids):

| change | what |
|---|---|
| `nwylqpqx` | `export`'s module doc (the table of calls), its private items (the render's measure and costs), `notation.rs`, `styles.rs` (an example on `Styles`) |
| `qsuzoxvr` | LaTeX and Typst; examples on both `Options` and on `typst::Length` |
| `zppnoowp` | the proof-net layouts, `svg/net.rs` and `svg/mell.rs` |
| `swnmomot` | Rocq, `rocq/mod.rs` and `rocq/deduxit.rs`; an example on `write_disproof` |
| `yppqxozx` | PNG and PDF; an example on `pdf::Date::from_unix` |
| `yquskzuw` | SVG, `svg/mod.rs`, `font.rs`, `tree.rs`; examples of a proof and a net drawn, and on `Style` |
| `qqprxssl` | `core-export.md`: the docs lead with the table of calls |
| (this section) | the report |

### Claims found wrong, and fixed

The module and its errors:
- `export`'s module doc said Rocq certificates are for NanoYalla; there
  are two kernels. `Drawable`'s doc left out the two-sided view of a
  classical derivation (`Drawable::Classical`).
- `write`'s errors (LaTeX, Typst, SVG) named only `Stopped` and
  `WriteFailed`. With the `lambda` option set, `write` also answers
  `Error::NoTerm` and the term's `Refusal::Output`, before anything is
  written (`export::judgement`), so `to_string`'s "as `write`, but never
  stopped" left it with no error at all.
- `sequent` (LaTeX, Typst) refuses through `Forest::within` only in an
  intuitionistic mode (`printed` returns before it otherwise). SVG's
  `sequent` gave its estimate as "128 bytes an occurrence and its longest
  atom name": it is 128 bytes plus the name's length, per occurrence.

LaTeX and Typst:
- `greek` maps the lowercase omicron to `o` too, not only capitals.
- Typst's open goal is drawn under dots by default only (`OpenGoal`); a
  standalone sequent and deduxit's own layout write no import line.
- `LAYOUT`: the strut stands before a sequent, not above it; an open goal
  under `OpenGoal::Mark` has `0` premises and the mark as label, not
  `none`.

SVG:
- `AXIS` said the inference lines sit on the math axis; they sit at
  `(line_height + HEIGHT − DEPTH) / 2` above the baseline, and only a
  rule name's axis is put on the line (`tree.rs`).
- The font family is written `'Euler Math', 'Neo Euler', serif`.
- `Style::par`, `net` and `mell_net` said the `⅋` edges and the wires are
  always dashed; only under `Premises::Dashed` (the default look) and
  without a reading. `mell_net` said one door per `?`: under
  `WhyNot::Rules` it is one per copy.
- `Style::literal_gap` is also the gap between the conclusions' trees;
  `Style::highlight` also marks a cut-elimination step and an essential
  net's directed cycle; `Style::line` also draws boxes and cut links;
  `Style::check` also bounds `link_cap`.
- `net.rs`: `ARROW` is an arrowhead's bytes alone (the colours are
  counted per text piece, the two groups by `DOCUMENT`); the dead
  occurrences are those of a cut formula above no cut link's end (as
  `ProofStructure::vertex` says), not "above every cut link".
- `mell.rs`: `Doors` holds the sorted pairs only; the copies of a `?`
  stand on the row above it only when it has several at its depth, or
  under the rules; the lowered `!` conclusion's arrow always points down,
  since a `!` vertex is always an output; `room_beside` is what the legs
  cannot use, not the room left for them.

Rocq, PNG and PDF:
- The kernel is not chosen by the mode alone: a classical proof that
  uses Mix, affine weakening, a cut or a quantifier also goes to
  deduxit's library, and so does a two-sided proof that records no mode.
- Both kernels refuse a proof of the cyclic mode
  (`Unsupported::Ordered`), not only an open goal and a compact run.
- Atom names are renamed for NanoYalla only (deduxit's library writes
  them as strings), and a clash gets `'`, then `'2`, `'3`, not primes
  until free.
- `write`'s errors left out NanoYalla's refusal of a cut and of a
  quantifier; `Unsupported`'s doc said every variant is a missing rule
  (`Disproof`, `NotValid`, `Ordered`, `Saturated` and `Exhausted` are
  not).
- `deduxit.rs`'s `disproof`: `affine_refutation` serves a state equation
  in affine mode only; a one-sided count is `weighted_refutation` in
  every mode.
- `png::from_svg` also answers `Error::RenderFailed` when the encoder
  fails; its link to `Style::background` pointed at the struct.

### How no code changed

A lexer (in the session's scratch directory, gone with it) reads each
file into tokens: it drops `//` comments and nested `/* */` comments
outside string, raw-string, byte-string and char literals, keeps every
literal whole, then drops the doc attributes (`#[doc = …]`,
`#![doc = …]`, and `cfg_attr(…, doc = …)` whose last argument is the
doc). It compares the tokens of each file with its version at
`llmypmxy` (`jj file show`). Shown to catch a planted `depth + 1` →
`depth + 2` beside the comment edits. Result at the chain's last commit:
`checked 14 files against llmypmxy, 0 with a code difference`. The only
attributes added are doc-test fences.

### The checks

| check | result |
|---|---|
| the no-code check above, all 14 files | 0 with a code difference |
| `cargo test --doc -p deduxit --all-features` | 78 passed |
| `cargo test --doc -p deduxit --no-default-features` | 5 passed, 48 ignored |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features` | exit 0 |
| `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items -p deduxit -p deduxit-cli` (the flake's `doc-private`) | exit 0 |
| `cargo clippy --workspace --all-targets -- --deny warnings` | exit 0 |
| `rustfmt --check --edition 2024` on the 14 files | clean |
| no comment naming history (`F12`-style numbers, review, panel, step, `plan/`) | none |
| the gate (capped, cores 7-9), at `qqprxssl` before the reviewer's four prose fixes | `gate: passed`, the journeys within their ceilings |
| `nix flake check --max-jobs 4 --cores 8` under the machine lock, at this section's commit on `main@origin` (with the placeholder of this row) | "all checks passed!", exit 0 |

All capped on cores 7-9 (`systemd-run --user --scope -p MemoryMax=8G
-p MemorySwapMax=0 taskset -c 7-9`, three jobs and test threads); the
flake check under `/tmp/linlog-machine.lock` with `--max-jobs 4 --cores
8`.

### The review

The `fresh-reviewer` agent (Fable 5.2 at `medium`) read the chain
against the brief, the writing rules and the code. It found no code
changed (a lexer of its own: equal token streams in all 14 files, the
fences the only attribute differences), the writing rules met, and the
fixed claims true; it checked `Style`'s stated defaults, the kernel
choice, the renaming, the exit statuses, `Advances`' JSON form and the
tables of calls. Two findings and two questions, all answered in the
commits they belong to:

1. LaTeX's atom names listed `\` among the characters that take a
   backslash and among those written as text; `escape` writes it as
   `\mbox{\textbackslash}`. The base had the same sentence. Fixed
   (`qsuzoxvr`).
2. `svg::sequent` listed the refusal of `Forest::within` without "in an
   intuitionistic mode", as LaTeX's and Typst's now say. Fixed
   (`yquskzuw`).
3. The question: `affine_refutation` "counts the outputs alone" was
   looser than the library's `overweight_affine` (one output in all, the
   count positive whether or not the formulas without an output are
   dropped). Reworded to the theorem's condition (`swnmomot`).
4. The question: "Every feature but `png` and `pdf` is on by default"
   read as a claim on the crate, where `parallel` is off too. Now "Of
   these features" (`nwylqpqx`).

The four fixes are prose; the no-code check holds after them.

### Proposals for the author

- `latex::Options`, `typst::Options` and `svg::Style` have no
  `with_lambda`, although `core-export.md` says every field has its
  `with_*`. I recommend adding the three (`#[must_use]`, additive), as
  for every other field.

### What the next sessions must know

- **Parallel writers and the formatter hook**: the four writers here
  edited in parallel with the Edit tool before the shared rules gained
  "one Edit at a time"; `jj log` showed no divergent change afterwards
  and every writer's claimed edit is in the files, but the safe way is
  writers that edit one at a time, or that edit by script in their own
  scratch directory and leave the formatting to the session (`nix fmt`
  or `rustfmt --check` before the commit).
- Strings and names that read oddly and are code, left as they are:
  `typst::Length::new`'s message "`…` is no Typst length: digits, a
  fraction or not, and one of …"; the test name
  `every_table_is_its_font_s`. The serde alias `"linlog"` on
  `Kernel::Deduxit`, `typst::Layout::Deduxit` and `svg::NetPreset::Deduxit`
  stays on purpose (settings files of 0.2.0).
- `core-export.md` still words several points as history ("the review's
  finding", "the integers panel's witness", "before, two wires … read as
  one", F-numbers); rules files are working notes for sessions, outside
  the writing rules, so they stay.
- `ordinary::Derivation::term` panics on a derivation its `check`
  refuses (its `# Panics`, a choice `core-ordinary.md` explains), so an
  export with the `lambda` option of an unchecked derivation of LK or LJ
  read from JSON panics; the exports' `write` docs do not say so. For
  54w-e4 (`ordinary`), to say on the read-from-JSON path.
- The PNG and PDF examples are `no_run`: they read `Euler-Math.otf` from
  the working directory (TeX Live's `euler-math` package has it; the
  command embeds it from `cli/fonts/`).

## Session 54w-c: README, the manual, the changelog and the guide

Item 4 of `plan/54w-text.md` (Opus 5.5 at `high`, 2026-11-14, cores 4-6,
jj workspace `deduxit-wt-step-54w-c-e93b9d`), from the brief
`start-54w-c.md`, on top of 54u-e's and 54w-d's chains (`llmypmxy`). The
author was away; every decision below follows the writing rules,
`plan/notes/usability.md` §6-§7 and the reports of 54b, 54s-a, 54u-a,
54u-b, 54u-e and 54w-d.

### Outcome

README, the manual (every chapter under `docs/src/` but the dated
results), the changelog's Unreleased section and the guide for AI
models are rewritten to `.claude/rules/writing.md`, and the manual is
split by Diátaxis's four kinds. No behaviour, flag, default or API
changed. The one code file touched is `cli/tests/readme.rs`: its
doctest rule now covers the manual's Rust examples too (below).

| change | what |
|---|---|
| `rwquttvk` | the manual, the readme test's doctest rule, `cli.md`'s line on it |
| `luvywlyk` | README |
| `tpnmmmtr` | the changelog's Unreleased section |
| `urswspwm` | the guide: `skills/deduxit/reference.md` and `SKILL.md` |
| (this section) | the report |

Five sub-agents on Opus 5.5 at `high` (`deduxit-docs-formats-rocq`,
`deduxit-docs-interact-nets`, `deduxit-docs-other-logics`,
`deduxit-docs-project-pages`, `deduxit-guide-reference`) rewrote the
chapters I did not take, each on its own files, under one shared brief
(the rules, the new layout, the shared sentence on deterministic
answers, the changes since 0.3.0 the text must show). I wrote the
layout, the tour, the split of the commands chapter, the new pages,
README, the changelog and `SKILL.md`, and checked their reports.

### The manual's new layout

| part | chapter | kind |
|---|---|---|
| (front page) | `introduction.md` "Getting started" | tutorial: first proof (with Q7's line that `⊢ ~A, A` is `A ⊢ A`, and `--classical-sides two`), first unprovable sequent and its reason, `-a`, the exit status, first `-i` proof, an export, `check` |
| How-to guides | `installation.md` "Install deduxit" | how-to; the musl/glibc note moved after the systems as "Long searches" |
| | `commands.md` "Decide a sequent" | how-to: the verdict, why unprovable (Q8's phrases, read without the counts' explanation), unknown and the flag to raise, the same answer every time (FC10), the logic, `-i`, showing the derivation, JSON |
| | `mistakes.md` "Avoid the common mistakes" (new) | how-to with Q6's hints: `prove` missing, an unquoted sequent, `->`, a file of named lines, classical symbols under `-i`, `tex`, an unused hypothesis |
| | `check.md` "Check a proof or a refutation" (new) | how-to, from the commands chapter |
| | `output-formats.md` "Export a proof" | how-to; names the exports' `to_string` |
| | `rocq.md` "Check a certificate in Rocq" | how-to: Q11's `nix run …#check-certificate` first, the kernels, the Nix ways before opam, the plain statement that `rocq-deduxit` is not yet in opam's archive, and 54u-e's sentence on a file that redefines its statement's names |
| | `interactive.md` "Prove step by step" | how-to, with Q9's rule list and the warning at a split that cannot be closed |
| | `batch.md`, `cut.md`, `proof-nets.md` | how-to ("Decide many sequents at once", "Eliminate cuts", "Draw a proof net"); the net verdict in its three forms |
| | `first-order.md`, `ordinary-logic.md`, `terms.md`, `cyclic.md`, `lambek.md` | how-to; "not valid" said in ordinary terms before the translation (FC9); `Display` for terms named |
| | `library.md` "Use the library" (new) | how-to: decide and print, Q1's `Settings::stop`, Q2's `to_string` and `{}`, Q3's `Outcome` read back, the error with its formula |
| Reference | `syntax.md` "The syntax" | reference: a table of connectives, precedence, the intuitionistic and first-order syntax |
| | `options.md` "Limits, defaults and exit statuses" (new) | reference: exit statuses per command, every limit with its default, threads, the JSON outcome's keys; Q10's "`--help` shows every option" |
| | `settings.md` "The settings file" (new) | reference, moved out of the formats chapter |
| Explanation | `engines.md` "How deduxit searches" (new) | the engines, the copy bound and the bias, the Horn engine, the inverse method, the intuitionistic engines, why a sequent is unprovable, threads |
| | `architecture.md`, `status.md` | explanation |
| Results | `benchmarks.md`, `results/*.md` | the dated records unchanged |
| The project | `development.md` | how-to for a contributor |

File names are kept, so links into the published manual (README's,
the web page's) still resolve; `commands.md` holds "Decide a sequent".

### What else was rewritten

- **README** opens with one paragraph of what deduxit does, then
  Install, the first proof with the one-sided line and
  `--classical-sides two`, and where to go next. The two paragraphs of
  history went (the changelog has them; one line says deduxit was linlog
  up to 0.2.0). Citing and the licences are unchanged in substance.
- **The changelog's Unreleased section** has "What breaks", a new
  "Changed" for the outputs a script may match (`--net`'s line in its
  three forms, the two-sided classical refutation and the `-a` note,
  the intuitionistic refutation without "positively"/"negatively",
  ordinary logic's "the translated sequent", the help and messages, the
  manual's layout) and "Added" (Q6's hints and `tex`, Q9, `Settings::stop`,
  `to_string` and `Display`, `Deserialize` for `Outcome` and
  `Engine::Unknown`, the formula in `ShapeError`, LJT, the SAT engine
  `ordinary::sat` in the library, the harness's ordinary rows, the
  certificate app).
- **`SKILL.md`**: the shared sentence on deterministic answers, the
  library line naming `Settings::stop`, the manual's description.
- **One sentence on deterministic answers**, the same in the manual
  (`commands.md`, `options.md`, `first-order.md`), `reference.md` §6 and
  `SKILL.md` (FC10): "For the same verdict on every machine, bound the
  search with `--work-limit N` or `--copies N` and lift the time limit
  with `--timeout none`. `--deterministic` also makes the proof and
  `--stats` the same on every run."

### Claims found wrong, and fixed

- `introduction.md` said the other provers had not yet run on the growing
  families; the chapter of 2026-11-03 has them. The sentence went with
  the old introduction.
- The commands chapter said the additive and the Horn engine get no
  pool; `--pool-after`'s help, which follows the code, names the
  essential and inverse engines too. `engines.md` names all four.
- The manual advised `--copies` or `--timeout none` for a stable answer,
  the guide `--work-limit`: one sentence now (FC10).
- `settings.md` (from the formats chapter) listed 7 top-level keys of
  the settings file; the command takes 11 (`interactive`, `input`, `cut`,
  `lambda` were missing), shown by the error a file with an unknown key
  gets.
- `installation.md` left out the default feature `sat`.
- `rocq.md` left implicit that a refutation's certificate is always
  rocq-deduxit's (`--kernel nanoyalla` says so on `A |- B`).
- `terms.md` said `--lambda-types` gives `false` its goal's type; the
  help and the output say `false` and `abort` (`abort_a(x)`).
- `architecture.md` said four crates (six, with `compat/`), described
  certificates as NanoYalla's only, and said one exchange per `⊗` (only
  before a `⊗` whose goal is out of order).
- `cut.md` left out the `above` key of the JSON record's steps;
  `proof-nets.md` named only `MellStructure` as the net read back (an MLL
  net is a `ProofStructure`).
- The guide's §1 left `eliminate` out of the commands; §12 said a batch
  goes across the cores by default (it goes within when there are fewer
  sequents than workers, and on a stream); §15 left out the settings key
  `interactive`; §16 left out the feature `sat`; §17 left out the web
  bindings' `term` and `ordinary_term` and said every call takes
  settings (`about` and `defaults` take none); the engine table said
  the essential engine runs only when named (it decides the Lambek
  calculus, and the net engine the cyclic mode). `engines.md` says the
  same.
- One "fix" a writer made was itself wrong and was undone: it said
  `Error::NotIntuitionistic`'s message names occurrences by id, from the
  doc comment on that variant in `core/src/errors/mod.rs`. Since 54u-a's
  Q4, `ShapeError::Formula`'s `Display` writes the formula
  (`core/src/linear/occurrences/reading.rs:207`); that doc comment is
  stale (below).

### A race in the working copy, and what it cost

Three parallel Edits of `cli/tests/readme.rs` each ran the formatter
hook's `nix fmt`, whose jj snapshots raced: the working-copy change went
divergent, and the disk was set back to an older snapshot of some files
(`cut.md`, `cyclic.md`, `output-formats.md`, `settings.md`, parts of
`reference.md`, my own edits of `readme.rs` and `cli.md`). Both versions
were saved to the scratchpad, every writer re-checked its files and
redid what was lost, and the orphan was abandoned. No text was lost in
the end. A session edits a file the hook formats (`.rs`, `.nix`,
`.toml`, `.sh`, `.py`) one Edit at a time.

### Checks

Every cargo run capped on cores 4-6 (three jobs and test threads); the
nix builds under `flock -o /tmp/linlog-machine.lock` with `--max-jobs 4
--cores 8`.

- `cargo test -p deduxit-cli --test readme`: 7 passed, after the
  reviewer's fixes too (every console block of README, the manual and
  the guide, the names, the defaults, the doctest rule, the summary's
  budget: `SKILL.md` 94 lines and 7 996 bytes of body).
- `cargo clippy -p deduxit-cli --tests -- --deny warnings`: clean.
- `nix build` of `manual`, `manual-links`, `typos`, `conventions` and
  `treefmt`: pass.
- `nix flake check`: "all checks passed!" on the four commits before the
  reviewer's fixes. The fixes change text, one test's counting, and no
  code the checks build otherwise; the readme test and clippy above
  cover them.
- The gate was not run: the one code file changed is a test of the
  command's examples (CLAUDE.md asks for the gate on a commit that
  touches code; `nix flake check` ran the workspace's tests and clippy).

### The reviewer

`fresh-reviewer` (Fable 5.2 at `medium`) read the four commits, the
brief and the prompt, and probed the binary. Three findings and four
lesser points; all held and are fixed in the commit they belong to.

1. `reference.md` §10 quoted the eigenvariable error as "cannot stand
   there", which 54w-d reworded to "cannot be used here". Fixed.
2. `commands.md` said a JSON, LLTP or `.spec` sequent stays one-sided
   under `--classical-sides two`; the derivation is two-sided, its
   formulas in their one-sided form (`A, ~A ⅋ B ⊢ B`). Fixed, checked
   by running it.
3. The doctest rule counted the guide's and the manual's Rust blocks
   together, so the guide could lose its block unnoticed. It now asserts
   each has one.
- Lesser: `syntax.md` named `check` among the readers of the sequent
  syntax (it reads a proof file); `reference.md` §3's `--logic` row
  still said "through a translation" (LJT since 54b); `status.md` left
  out `eliminate`; the changelog's `ordinary::Search` left out `Sat`.
  All fixed.

Everything else it checked held: the defaults, exit statuses, engines,
the library's calls, the certificate app, the changelog against the
reports, Q1-Q11 and Q7, the shared sentence (word for word in five
places), the writing rules, and every link.

### Proposals for the author

- **`--settings`'s help names 8 of the settings file's 11 top-level
  keys** (`interactive`, `cut` and `lambda` are missing). The help could
  name all; `settings.md` lists the 11.
- **`deduxit prove --net "!A |- A * 1"`** says nets with boxes exist
  "not for MELL in classical mode", where the unit is the reason. The
  message could name the units ("MELL with units").
- **`--engine inverse`'s help** says it runs "only when named here", but
  without `--engine` it also takes turns beside the engine and takes over
  at the memory bound. A sentence there would match the manual.
- **A one-call `--deterministic` in the library**: matching
  `prove --deterministic` takes `settings.search.with_jobs(1)` (and the
  schedule of turns, which one job takes anyway) beside `Settings::stop`;
  a helper such as `Settings::deterministic()` would make it one call.
- `rocq-deduxit` in the Rocq opam archive (already in `plan/later.md`
  with Q11's second half), after which the Rocq chapter's opam lines
  lead.

### What the next sessions must know

- **The manual's layout** is `SUMMARY.md`'s four parts: a tour
  (`introduction.md`), how-to guides, reference, explanation, then the
  results and the project. A new command, flag or format goes into the
  how-to of its task with a real invocation, its limit and default into
  `options.md`, its settings key into `settings.md`, and how it works
  into `engines.md`. File names were kept, so `commands.md` is "Decide a
  sequent".
- **The manual has Rust examples now** (`docs/src/library.md`), held
  by `rust_examples_are_doctests` to the crate root's doctests in
  `core/src/lib.rs` as the guide's are. **54w-e5** rewrites the crate
  root: a doctest it changes changes `library.md`'s and
  `reference.md`'s copies in the same commit, or the readme test fails.
- **A stale doc comment for 54w-e5**: `core/src/errors/mod.rs` says
  `Error::NotIntuitionistic`'s message names occurrences by id; since
  Q4 it names the subformula (`reading.rs`'s `Display`). One of this
  session's writers was misled by it.
- **54s-b** may add a line to `ordinary-logic.md` or the guide's §13:
  the chapter kept its headings "How a sequent is decided" and "Step by
  step"; `status.md`, `architecture.md` and the changelog say the
  command does not offer the SAT engine yet, which its wiring changes.
- **The shared sentence on deterministic answers** stands word for word
  in `commands.md`, `options.md`, `first-order.md`, `reference.md` §6
  and `SKILL.md`; change it in all five.
- **`SKILL.md` is at its byte budget** (7 996 of 8 000 bytes of body):
  a line added there needs one cut.
- **The formatter hook and parallel edits**: editing one `.rs`, `.nix`,
  `.toml`, `.sh` or `.py` file with several Edits in one message runs
  the hook's `nix fmt` concurrently, whose jj snapshots race; edit such
  files one at a time.
