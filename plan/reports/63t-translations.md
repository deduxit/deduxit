# Step 63t: transformations of ordinary logic, checked

## Session 63t-a: the library, the command and the bindings

### Outcome

Classical sequents are translated into minimal and intuitionistic logic
by four negative translations, each translation's claim computed by
deciding both sequents; Craig interpolants are read off cut-free
derivations of LK and LJ by Maehara's method and checked by deciding
both halves and comparing atoms. Both are in the library
(`ordinary::negative`, `ordinary::interpolant`), the command (`seq
translate`, `interpolate`), the JSON at wire level 16 and the bindings
(`ordinary_translate`, `ordinary_interpolate`), with a manual chapter,
the guide, the changelog and the rules files. The page is 63t-b's.

The chain on `main@origin` (`moprkuwt`, the plan commit of 63t's
prompt):

| change | what |
|---|---|
| `yqvvklvx` | the library: translations and interpolants, both checked; the checker reports what each premise adds |
| `luospnss` | the level-16 documents; an ordinary outcome written nested |
| `xspnyzrz` | `interpolate` decides by the method first, the embedding only for a derivation without cuts |
| `kxsnmqkl` | the command: `seq translate`, `interpolate` |
| `mrxptnrv` | the behaviour lock: the new calls, and the command list in `error-no-command` |
| `kmluylqz` | the bindings' calls, `about`'s `transformations` |
| `kupmltuk` | the manual's chapter, the guide, the status chapter, the changelog |
| `txspvrvx` | the rules files |

### The translations and their checks

- **A table per translation** (`negative::row`, `negative::sides`),
  applied once down the arena to the nodes the sequent reaches; nothing
  recurses. Kolmogorov: `¬¬` before every node. Gödel and Gentzen: `¬¬`
  before every atom, `A ∨ B` as `¬(¬A ∧ ¬B)`, the rest kept (`⊥` stays
  `⊥`). Kuroda: `¬¬` before every formula of the sequent (propositionally
  the whole of it; the rest acts after `∀`). Glivenko: `¬¬` before the
  formula right of `⊢`, the hypotheses unchanged, as Glivenko's theorem
  for sequents states (`Γ ⊢c A` iff `Γ ⊢i ¬¬A`). Several formulas right
  of `⊢` are their left-nested disjunction, translated; none stays none.
- **Targets**: Kolmogorov's and Gödel and Gentzen's reach minimal logic
  (Troelstra and Schwichtenberg, *Basic Proof Theory*, 2.3), and with it
  intuitionistic logic; Kuroda's and Glivenko's intuitionistic logic only
  (`false -> a` is classically valid, `¬¬(⊥ → a)` is not minimally).
  Another target is `Error::NegativeTarget` (unsupported).
- **The check**: `compare` decides the sequent in classical logic and
  the translation in the target, each by its logic's default method
  under one stop; verdicts that differ are `Error::ClaimFails` (kind
  defect, code `claim_fails`), never an answer; an unknown on either side
  is `Agreement::Unknown`. Tests: twelve chosen sequents under every
  translation (`every_translation_keeps_the_verdict`) and 200 generated
  ones (`generated_sequents_keep_their_verdicts`), none disagreeing.

### The interpolants and their checks

- **The construction** is Maehara's (Takeuti, *Proof Theory*, §6;
  Troelstra and Schwichtenberg 4.4) over the checker's own rule table:
  `check_inference` now returns, per premise, the formulas the rule adds,
  so the interpolant has no second table. Halves go up the derivation
  (context formulas keep theirs, a split drawing from one pool and a
  shared context whole per premise; added formulas take the principal's),
  and in LJ a premise whose formula right of `⊢` would be in the first
  half takes its halves exchanged (`→L` and `¬L` on a first-half
  hypothesis). Interpolants come down: leaves `⊥`/`⊤`/`A`/`¬A`, one
  premise its own (negated where exchanged), two premises `∨` or `∧` by
  the principal's half, or `I₁ → I₂` where the first is exchanged.
  Constants are simplified only by equivalences of the logic (in minimal
  logic only `⊤`).
- **The three conditions are checked** in `checked`: every atom of `I` in
  both halves, then `Γ₁ ⊢ Δ₁, I` and `I, Γ₂ ⊢ Δ₂` decided by the logic's
  default method; a half not valid is `ClaimFails`, a half undecided
  leaves `Interpolant::is_checked` false (the command's exit 3).
  `the_check_refuses_what_is_no_interpolant` plants a foreign atom and a
  `⊤` that fails the second half; `generated_sequents_interpolate` splits
  200 generated sequents at random in each logic (more than 100
  interpolants found, every one checked).
- **Cut-free derivations**: a derivation with a cut is
  `Error::NotCutFree`. `interpolate` decides by the default method (the
  SAT solver, so a sequent that is not valid gets its countermodel at
  once) and where its derivation has a cut (LJT, the solver's read-back)
  or there is none (a clausal proof), decides again through the
  embedding, whose read-back has none; a method the caller named that
  gives a cut is refused instead (departs from the design: the prompt
  names the embedding's or the tableau's derivation; the default method
  first is what gives a countermodel for "not valid" in intuitionistic
  logic, where the embedding alone answered unknown at the time limit).
- **Partitions** (`interpolant::Partition`, the positions of the first
  half on each side; `Partition::hypotheses` is Craig's `A ⊢ B`): a
  position outside the sequent, one named twice, and outside classical
  logic a formula right of `⊢` in the first half are `Error::Partition`
  (malformed).
- **Bounds**: the halves within `limits.occurrences`, both decisions
  within the caller's limits and stop, the stop asked once per inference
  of the construction. The interpolant is hash-consed in the
  derivation's arena, so its nodes grow linearly with the derivation; its
  unfolded size may not, which the occurrence bound on the halves refuses.

### The command's and the bindings' forms

- **Partition syntax**: `--first L0,R1`, the addresses `interact --logic`
  already uses for formulas, default every hypothesis (Craig's split).
  Tools that compute interpolants are told the first group of formulas
  (the SMT solvers' `get-interpolants`, as SMTInterpol and MathSAT have
  it, takes the names of the assertions in A), so a list of the first
  half's formulas is the convention; the session's addresses keep one
  spelling in the command.
- **Naming a translated sequent**: the text's first line is "Kolmogorov's
  translation into minimal logic: SEQUENT", the JSON's keys `sequent` and
  `translated`. `seq translate` sits under `seq`, beside `print`, `json`
  and `fragment`, as the prompt's example names it; `interpolate` is a
  command of its own, since it decides.
- **Text**: a line per sequent with its verdict and evidence (the
  calculus and the number of inferences, or the Kripke model indented
  under it), then the agreement; no derivation is drawn (not overwhelming:
  the JSON holds the derivations). Exit statuses: 0 valid, 1 not valid, 3
  unknown, 2 an error, a disagreement or failed check included.
- **Options for every front end** (D15): `negative::Options`
  (`translation`, `target`) and `interpolant::Partition` (`left`,
  `right`), plain data with serde; the command maps `--by`, `--to`,
  `--first` onto them, the bindings take their JSON. They are inputs of
  the call, so no settings key holds them. The command's searches run on
  one thread where the settings say `auto` (the library's `auto` takes
  every thread from the start).
- **JSON** at wire level 16 (`wire::TRANSFORMS`, `wire::LEVEL` now 16),
  written only, each holding its sequents and their ordinary outcomes
  nested without `version`.
- **Bindings**: `ordinary_translate(sequent, options, settings)`,
  `ordinary_interpolate(sequent, logic, partition, settings)`; `about`
  lists them under the ordinary suite's `transformations`, with each
  translation's target logic.

### Checks

- **The gate** (capped, cores 7-9) passed at the head after the rebase
  onto `main@origin` `kvxnuxzq`. A first run failed its ratchet:
  `ordinary-pigeons` counted 4.34 % over its ceiling, because the
  checker returned heap vectors of the added formulas for every
  inference it checks. `derivation::Added` now keeps them inline (two
  premises of two formulas at most), and the journey is at +0.42 %.
  Folded into `yqvvklvx`.
- **The bindings' tests**: native `cargo test -p deduxit-web` passed;
  under Node in `nix flake check` (`web-wasm-tests`), below.
- **Every family at 5 s** (`deduxit-bench run --all-families --timeout
  5 --jobs 1`, release): 195 problems, no `MISMATCH`, "every problem
  decided in a mode has one verdict".
- **The ILTP agreement tests** (`cargo test --release -p deduxit --test
  iltp -- --ignored`, the library read from the store): all three passed
  (`ljt_agrees_with_the_embedding_on_iltp`,
  `sat_agrees_with_ljt_on_iltp`,
  `sat_agrees_with_the_embedding_in_classical_logic`, 787 s).
- **`nix flake check`** under the machine lock at the last code commit
  (`txspvrvx`, on `main@origin` `kvxnuxzq`): all checks passed, the
  bindings' tests under Node (`web-wasm-tests`, the ordinary file's ten
  among them), the module's budget and the manual's build and links
  included.
- **The rebase** onto `kvxnuxzq` (60b's chain and the baseline
  preparation) conflicted in `CHANGELOG.md` alone: both entries kept,
  60b's first. Its bullet still says level 15 is the highest the library
  reads; this step's bullet says 16 is now (the planning session may
  merge the two sentences).

### The reviewer's findings

`fresh-reviewer` (Fable 5.2, medium) read the chain against the prompt
and the brief, and ran 43 capped calls of the command. It found the
agreement computed on every path, the three conditions of an
interpolant checked on every path (an `Interpolant` is built only in
`checked`), Maehara's cases sound for every rule the checker accepts,
`Build`'s simplifications equivalences in each logic, and the exit
statuses and JSON as documented. One finding, fixed:

- **A false line** (`cli/src/transform.rs`): where the SAT solver
  proved the sequent and the embedding found no derivation without cuts
  within the limits, `interpolate` printed "valid" and then "No
  interpolant is read off a sequent the search did not decide". It now
  says that no derivation without cuts was found within the limits and
  names `--timeout` and `--work-limit`; the reviewer's witness (`--work-limit
  400` on `(a -> b) -> c, ~~(a -> b), c -> ~d, d |- false`, exit 3) is in
  `transformations_answer_with_their_checks`. Folded into `kxsnmqkl`.

### What the next sessions must know

- **63t-b (the page)**: call `ordinary_translate` and
  `ordinary_interpolate` (JSON in `web/src/ordinary.rs`'s doc comments);
  `about().suites[1].transformations` lists them, with each translation
  and its target. Both outcomes of a comparison carry their evidence
  (derivation or `countermodel`), drawable with the existing calls
  (`ordinary_model_drawing` takes an outcome). The bindings run without a
  stop and with the copy bound 3, so the embedding may answer unknown
  for an intuitionistic sequent; the interpolate call decides by the SAT
  solver first, so "not valid" is always decided. A disagreement is a
  thrown `claim_fails` (kind defect).
- **Step 77 (first order)**: the translation tables get rows for `∀`
  and `∃` (`negative::row`: Kuroda's `¬¬` after every `∀`; Gödel and
  Gentzen's `∃x A` as `¬∀x¬A`), and Glivenko's theorem fails at first
  order (`¬¬∀x(p(x) ∨ ¬p(x))`), so `reaches` must refuse it there.
  Maehara's method needs the eigenvariable condition: a term of one half
  only is quantified over in the interpolant, `∀` or `∃` by the half.
- **60b** (rules as data): `core/tests/rules.rs` compared a rule file's
  written level with `wire::LEVEL`; it now compares with 15, the rule
  file's level, since level 16 is this step's.

## Session 63t-b: the page

Opus 5.5 at `high`, the reused worker of 54s-c2 to baseline-prep (same
conversation), in the client's jj workspace `../deduxit.github.io-63tb`
(on its `main@origin` `zszutryy`) and this workspace for the report, from
the brief `start-63t-b.md`, on 2026-11-15, cores 4 to 6.

### Outcome

A started proof of classical ordinary logic offers two more buttons at
the foot of its tab's Logic popover, under the switches: **Translate into
intuitionistic logic** and **Find an interpolant**. Each opens a sheet
over the stage (as Practice is one) that answers with its verdict
sentence first. Nothing changes for a visitor who does not press them,
nor for a linear, intuitionistic or minimal tab.

- **The translation's sheet**: one sentence on what a negative
  translation is, the four translations as a segmented control
  (Glivenko, Kuroda, Gödel–Gentzen, Kolmogorov, from the least change to
  the most, Glivenko first), the chosen one's line of what it does and
  the logic it translates into, then the answer: "Both valid" (or "Both
  not valid", "Not decided") with the sentence that the verdicts agree,
  the translated sequent in the formulas' font at the large size, **Prove
  it in <logic>** (a new tab in the target logic's calculus, the proof
  it came from kept), and one line per logic: valid with a checked
  derivation, or not valid with the classical falsifying values in words
  and the translation's Kripke model drawn (55d's figure).
- **The interpolant's sheet**: one sentence on what an interpolant is,
  the sequent with each formula a button in its part (part 1 filled,
  part 2 dashed, its number written before it; Craig's split, the
  formulas left of ⊢, at the start), then "Interpolant: All three of its
  checks pass", the formula large, and the three checks with their marks:
  its atoms (named) occur in both parts, `Γ₁ ⊢ Δ₁, I` valid, `I, Γ₂ ⊢ Δ₂`
  valid, the halves as deduxit decided them. A half not decided is marked
  with a question mark and the keyword is "Not checked"; a sequent that
  is not valid says "No interpolant" and draws its countermodel; a valid
  one without a derivation free of cuts in the bounds says "Not found".

The client's commits, on its `main@origin` `zszutryy`:

| change | what |
|---|---|
| `yyrrpxqm` | flake.lock: deduxit's input moved to `main@origin` `09cfa99c` (63t-a's bindings) |
| `tpzumptk` | `src/lib/transforms.ts` (the forms, the translations' list, the words of verdicts and checks), its unit tests, the two calls in the API and the worker |
| `msqvnvks` | `src/islands/Transforms.tsx` (the two sheets), the Logic popover's two buttons, the styles, the client's `web.md` |
| `kmpzouss` | `tests/transforms.spec.ts`, the walk `transforms` in `tests/states.ts` (axe and targets), the `switches` helper matching the Logic popover's exact name |
| `uxuxvlps` | the review's fixes: a valid verdict said by how it was shown, and the join of the formulas right of ⊢ |

### How they were designed

- **Where they are asked for** (the cognitive-load analysis, the
  education skills' `cognitive-load-analyser`, and the frontend-design
  plugin):
  - Not as view tabs beside Table and Tableau. The stage's row of views
    holds 364 of a phone's 375 pixels already, so a fifth tab would need
    a second row or a menu inside the tablist.
  - In the Logic popover instead: it is where a student picks between
    classical, intuitionistic and minimal logic, so a translation into
    another logic sits beside the logics it relates. It is reached at
    every width by a control that says it is one (`Ordinary (LK) ▾`).
  - Shown only for a started classical proof; the author's rule (not
    overwhelming) holds, since the popover is opened only on purpose.
  - As sheets over the stage, not views in the drawing's place: the
    proof stays where it was, light dismiss and Escape close them as any
    menu, and the tablist keeps one selected tab (a view reached from
    outside the tablist would leave none selected).
- **What the analysis changed**:
  - Glivenko's translation first, and the four in order of how much they
    change the sequent: the first example a student sees is the easiest
    to read (`a → b ⊢ ¬¬(¬b → ¬a)`), Kolmogorov's last (`¬¬(¬¬a → ¬¬b) ⊢
    ¬¬(¬¬¬¬¬b → ¬¬¬¬¬a)`, which wraps on a phone).
  - Each translation says in one line what it does and its target, so
    the student need not hold four definitions in mind.
  - The translated sequent stands right under the verdict sentence that
    names it (no split attention), and the evidence is one line per
    logic. Only the translation's Kripke model is drawn; the classical
    countermodel is one world, said in words ("with a false, b true"),
    so there is never a second figure to compare.
  - The interpolant's halves are written once, as the sequents its
    checks name, not restated beside the partition (redundancy); the
    checks instantiate the definition the sheet's first sentence gives.
  - A formula's part is its number and its border, filled or dashed,
    never colour alone.
- **The look**: no new colour or font; the verdicts' keywords are the
  page's `kw` washes, and the one large thing in each sheet is the
  answer's formula in the formulas' font (`--text-math-large`). No
  motion, no cards inside the sheet; the answer is ruled off from the
  choices by one line.

### Departures

- **Sheets from the Logic popover, not tabs of the stage (departs from
  the design).** The brief leaves the place open ("a way to ask for");
  56b put the table and the tableau among the stage's views. These two
  would not fit that row on a phone, and they are asked for less often
  and relate the sequent to other logics, which the Logic popover
  already holds. 63g's workspace graph may give them another home.
- **The interpolant is offered for classical proofs alone**, as the brief
  says, though deduxit computes one in each of the three logics
  (`about`'s `logics`); an intuitionistic or minimal tab would need only
  the popover's button and the partition's rule that the formula right
  of ⊢ stays in the second part.

### Checks

- **Wire levels**: the page sends deduxit nothing of its own level and
  reads every answer through the module, so moving the input to level 16
  needed no change; the stored problems' sessions (older levels) and
  every call the suites make passed below at the new input.
- **Lint and type-check** (`lint`: Biome, `astro check`): clean.
- **The unit tests** (Vitest): 613 passed at the last commit, with
  `src/lib/transforms.test.ts` (8) against deduxit's real module: the
  translations equal `about`'s, every translated sequent read back as
  itself by the field's parser, the verdicts' words, a valid verdict
  shown by a clausal proof, the join of several formulas, the partition,
  the interpolant's atoms.
- **The e2e suite** (Playwright, cores 4 and 5, two workers): 249 passed
  before the review's fixes, with `tests/transforms.spec.ts` (6), the
  `transforms` walk under axe (light and dark, desktop and phone: no
  violation; about 10 seconds a walk, under the 20) and its targets (24
  pixels on a desktop, 44 on a phone); after the fixes the six again.
- **The client's `nix flake check`** under the machine lock at
  `uxuxvlps`: all checks passed; its e2e suite 248 passed and one flaky
  (`ordinary.spec.ts`, the structural switch with a goal open, passed on
  retry: a test this chain does not touch, which passed in the run
  above).
- **In a browser**: both sheets at 1280 and 375 pixels, driven by
  Playwright scripts against the dev server (cores 4 to 6), screenshots
  read; the reviewer drove the same.

### The reviewer

`fresh-reviewer` once (Fable 5.2, medium), which read the chain, the
brief and the prompt, drove the page with Playwright scripts at 1280 and
375 pixels, and called the module in Node. It found every translation
offered with its target, the verdicts and the checks true to deduxit's
answer, "Prove it" opening the target's tab with the original kept,
nothing new for any other tab, no sideways scroll on a phone, every new
control 44 pixels there, the focus and Escape right. Two findings, both
fixed in `uxuxvlps`:

1. **A valid verdict claimed a derivation deduxit did not give.** The
   SAT solver proves a classical sequent valid by a clausal proof, with
   no derivation (the reviewer's witness: `⊢ a ∨ ¬a ∧ … ∧ z ∨ ¬z`, 26
   atoms), and the line said "deduxit found a derivation and checked it".
   It now says what deduxit gave: a derivation, or that "the SAT solver
   showed that no values make it false, and deduxit checked its proof";
   a unit test runs the witness.
2. **Glivenko's line was false for a right side of several formulas or
   none**: `a ⊢ b, c` becomes `a ⊢ ¬¬(b ∨ c)`, and `a, ¬a ⊢` stays as it
   is. The line now says "if there is one", and under every translation
   a sequent with several formulas right of ⊢ gets "The formulas right of
   ⊢ are first joined by ∨ into one."; a unit test holds both.

### What the next sessions must know

- **63g (the workspace graph)**: "Prove it in <logic>" is
  `prover.open(text, ordinary(target))`, a new tab with no `origin` (that
  field is a goal's path in another tab). The graph's edge needs the
  tab it came from, the translation and its target: record them there
  when the edge is built, and the sheet's button becomes the edge's
  first use.
- **Both calls run without a stop**, as the table's and the tableau's do:
  the bindings decide with the copy bound 3 and no time limit, so a hard
  sequent keeps the worker until it ends; the stage's Stop terminates it
  as for any call. The sheets send `{}` as settings.
- **Playwright matches a dialog's name by substring**: "Logic" matched
  the sheet "Translate into intuitionistic logic". `switches` in
  `tests/prover.ts` now asks for the exact name; a test that finds
  another dialog by a short name while a sheet is open needs the same.
- **Interpolants in LJ**: offering the sheet in an intuitionistic or
  minimal tab is the popover's condition and `interpolate`'s `logic`
  argument; deduxit refuses a partition with the formula right of ⊢ in
  the first part there (code `partition`), which the sheet shows as
  deduxit's message today, so the partition's buttons should keep that
  formula in part 2.
