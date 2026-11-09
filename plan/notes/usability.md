# Usability review (step 54w, session 54w-r), 2026-11-09

What the people who use deduxit want to do, how they do it today, where
it is harder than it need be, and what would remove each friction. Read
only: nothing outside this file and the report was changed. The
proposals that change the library's API, the command's flags or
behaviour, or the page's behaviour are the questions for the author in
section 6; everything else is text work for the 54w sessions (section
7).

## The main frictions

1. **The library's documented way to "decide what the command
   decides" does not** (L3): on `!(A -o A * A), !(A -o B), !(B * B -o
   C), A |- C * ?D` it answers "unknown: the search was stopped", the
   command "unprovable". The crate root's claim is wrong (fix in
   54w-e); Q1 asks for a helper.
2. **Reasons are hidden on the page** (FW1, FW2): why a step was
   refused or a goal has no proof is a hover tip, never seen on touch,
   and the Undo beside a refusal undoes a good step.
3. **Lesson 1 explains eight ideas before a one-step example** (FW7),
   and its predictions offer rules not yet taught.
4. **Exports and certificates do not stand alone** (FR1, FW9): the
   page's LaTeX, Typst and Rocq are fragments that do not say what
   they need; checking a certificate needs two kernels installed two
   ways, and the manual's `opam install rocq-deduxit` fails today.
5. **The command's first errors do not say what to do** (FC4): no
   command named, an unquoted sequent (`-o B` becomes `--output B`),
   `->` in linear mode, `--format tex`.
6. **Output and refutations speak one-sided to people who wrote
   two-sided sequents** (FC1, FC2), on the command and the page alike.
7. **The library's printers want `Limits` and a `|_| false` stop at
   every call** (FL3), `Term` has no `Display`, `Outcome` cannot be
   read back (FL4), and intuitionistic input has two parsers of which
   the common path shows the worse (FL5).
8. **The manual's commands chapter is 726 lines under one heading**;
   README opens with a list and history before Install.

The questions for the author are in section 6: Q1-Q5 the library,
Q6-Q10 the command, Q11 the certificates, W1-W14 the page.

## 1. How it was run

- The command: `deduxit-cli 0.3.0` built from `main` (`lxxwutox`,
  "Plan: step 54w, every text a person reads rewritten") in the jj
  workspace `../linlog-step54w-r`, `cargo build --release -p
  deduxit-cli` in a capped scope on cores 12-15. Every console block
  below is the real output of that binary (the script was
  `run.sh`/`run2.sh` in the session's job directory; each command ran
  under `timeout 30`, `[exit N]` is its exit status as bash saw it).
- The library: a scratch crate outside the repository
  (`~/.claude/jobs/fb1da658/tmp/libuser`, `deduxit = { path =
  ".../linlog-step54w-r/core" }` with default features, plus
  `serde_json = "1"`), `cargo run` in a capped scope on cores 12-15. The
  whole program is in appendix A, its output in appendix B.
- The page: read from the client's `main` (`vpkkppkk`, "flake.lock:
  Update", copied out read-only with `jj file show -r main`; the
  client's working copy was 16 commits behind). See section 4 for what
  was run of it.
- Rocq: no certificate was compiled. Checking one needs NanoYalla or
  `rocq-deduxit` installed, and neither is on this machine outside the
  flake's checks (section 5, task R3).

## 2. A Rust programmer using the library

### Tasks and how they go today

**L1. Decide a sequent and print the proof.** The crate root's "common
path" does it in five lines, and it works:

```rust
let sequent: Sequent = "A, A -o B |- B".parse()?;
let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
match &outcome.verdict {
    Verdict::Proved(proof) => println!("provable\n{}", proof.derivation()?),
    Verdict::Unprovable(_) => println!("unprovable"),
    Verdict::Unknown(_) => println!("unknown"),
}
```

```text
== T1: is it provable? print the proof
provable
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
```

**L2. Say why it is unprovable.** `Disproof` has `Display`, which gives
the command's reason:

```text
== T2: why unprovable
Display of the disproof: ~A occurs 1 more time than A in the one-sided sequent, so they cannot all meet in axioms
```

No root-level example shows this; a first attempt reaches for `{:?}`,
which prints the whole arena (`Disproof { sequent: Sequent { terms:
[DualAtom(Atom(0)), …`).

**L3. Get the same answer as the command.** The crate root says the
command's verdicts differ from `Options::default()`'s on purpose (the
default bounds the copies at 3), and gives a recipe: search with
`Settings::default().search`, stop at the clock's time limit. On three
sequents (appendix A, `t3`):

```text
-- !(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D
Options::default(): unprovable
  reason: the classical reading is false where A, B and C are true and every other atom false (! and ? erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), and every provable sequent reads as a tautology
Settings::default() + clock: unknown
  reason: the search was stopped
-- ?A par ?A |- ?(A * A)
Options::default(): unprovable
  reason: the search was exhaustive
Settings::default() + clock: unprovable
-- !(A -o B), !(B -o C), !(C -o D), !(D -o E), !(E -o F), A |- F * !(A -o A)
Options::default(): unknown
  reason: the copy bound of 3 was reached
Settings::default() + clock: proved
```

The command on the first and third:

```console
$ deduxit prove -q "!(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D"
unprovable (MELL, classical, focus engine): the classical reading is false where A, B and C are true and every other atom false (! and ? erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), and every provable sequent reads as a tautology
[exit 1]
$ deduxit prove -q "!(A -o B), !(B -o C), !(C -o D), !(D -o E), !(E -o F), A |- F * !(A -o A)"
provable (MELL, classical, focus engine)
[exit 0]
```

So the default really does differ (third sequent), and **the documented
recipe does not reproduce the command** (first sequent): the command's
stop ignores the clock while the classical refutation runs after the
search (`cli/src/prove.rs:88`, `halting`: `Phase::Refute =>
interrupted()`), while the recipe's stop fires in every phase, so the
refutation never runs and the verdict is "unknown: the search was
stopped". This is a wrong claim in the crate root (`core/src/lib.rs`,
the paragraph "The command's verdicts differ…" and its example) and in
`skills/deduxit/reference.md` §16 ("`Settings::default()` is the
command's behaviour as data"). 54w-e must fix the claim; Q1 asks for the
helper that would make it a one-liner.

**L4. An intuitionistic proof, two-sided, as LaTeX.** Works, but a
printing call takes four arguments, one of them a stop closure:

```rust
let derivation = proof.derivation()?;
let mut out = String::new();
deduxit::export::latex::write(&derivation, &deduxit::export::latex::Options::default(), &mut out, |_| false)?;
```

```text
\begin{prooftree}
\infer0[$\mathrm{ax}$]{A &\vdash A}
\infer0[$\mathrm{ax}$]{B &\vdash B}
\infer2[$\multimap\mathrm{L}$]{A, A \multimap B &\vdash B}
\end{prooftree}
```

**L5. The lambda-term of an intuitionistic proof.** Two calls with
`Limits` and a stop each, and `Term` has no `Display`:

```rust
let term = proof.derivation()?.term(&Limits::default(), |_| false)?;
let mut out = String::new();
term.write_judgement(&mut out, &deduxit::lambda::Options::default(), &Limits::default(), |_| false)?;
```

```text
f : A ⊸ B, g : B ⊸ C ⊢ λx. g (f x) : A ⊸ C
```

The command does it with one flag (`prove -i --lambda`, section 3).

**L6. Errors.** The intuitionistic path has two parsers, and only one
gives a readable error:

```text
== T6: errors a user meets
parse error Display: cannot parse the sequent: unexpected "|" at line 1, character 9, expected a formula
ILL of a par, Display: not an intuitionistic sequent: subformula 0 is neither an intuitionistic formula nor the negation of one (⅋ only as A ⊸ B, that is ~A ⅋ B, and ? only under a negation) (code not_intuitionistic)
IllSequent parse, Display: cannot parse the sequent: unexpected "par" at line 1, character 8: intuitionistic linear logic has no par: write ~A par B as A -o B
IllSequent then prove(ill.sequent(), INTUITIONISTIC): proved
```

Parsing with `Sequent` and proving in `Mode::INTUITIONISTIC` (what the
doc examples on `Proof::derivation` do) gives "subformula 0", a number
the reader cannot map to their text without `describe(&forest)`.
`IllSequent` gives the command's hint, but the crate root never names
it.

**L7. Check a proof file the command wrote.** `Proof` reads the
command's outcome JSON (it ignores the extra keys), but the mode is a
separate key the caller must read, and `Outcome` cannot be read at all
(it is `Serialize` only: `the trait bound
deduxit::linear::Outcome: serde::Deserialize<'de> is not satisfied`).

```rust
let proof: Proof = serde_json::from_str(&text)?;
let value: serde_json::Value = serde_json::from_str(&text)?;
let mode: Mode = serde_json::from_value(value["mode"].clone())?;   // or value["mode"].as_str()…parse::<Mode>()
proof.check(mode)?;
```

```text
== T7: check a proof file the command wrote
read as Proof; check(CLASSICAL): Ok(())
the file's mode key: "classical"
Mode read from the key: Mode { intuitionistic: false, affine: false, mix: false, order: Commutative, empty_antecedents: false }
read as Disproof; check: Ok(())
```

An unprovable outcome reads as `Disproof`, whose `check()` needs no mode
(the disproof carries it); a proof does not carry its mode.

**L8. Ordinary logic.** `ordinary::decide` takes five arguments, two
option types of the same name, and its "not valid" carries no reason:

```rust
let options = ordinary::Options::default().with_logic(ordinary::Logic::Intuitionistic);
let outcome = ordinary::decide(&sequent, &options, &Options::default(), &Limits::default(), |_| false)?;
```

```text
== T8: ordinary intuitionistic logic
~~a -> a intuitionistically: Discriminant(1)
NotValid
```

The reason is the linear image's disproof, `outcome.linear.verdict`,
two levels down and phrased about the image.

### Frictions and the change that removes each

| # | friction | change | cost elsewhere | kind |
|---|---|---|---|---|
| FL1 | The documented way to match the command returns "unknown" where the command says "unprovable" (L3). | Fix the claim in the crate root and reference.md §16: the stop must let `Phase::Refute` run. And a helper (Q1). | None for the doc fix. | text (54w-e, 54w-c) + Q1 |
| FL2 | `Options::default()` differs from the command (copy bound 3), and matching the command takes about ten lines with `Instant`. | Q1. | | Q1 |
| FL3 | Printing a term or exporting a derivation needs `Limits` and a `\|_\| false` stop at every call; `Term` has no `Display`. | The crate's own `derivation()`/`derivation_within()` pair, extended (Q2). | Larger surface. | Q2 |
| FL4 | A proof file of the command needs its `mode` key read by hand; `Outcome` is not `Deserialize`. | Q3. | | Q3 |
| FL5 | Proving a `Sequent` in intuitionistic mode reports "subformula 0"; `IllSequent` is the right parser and is not on the common path. | Show `IllSequent` in the crate root's common path (text). Optionally Q4. | None. | text (54w-e) + Q4 |
| FL6 | `{:?}` of an outcome dumps the arena; `{}` of `Disproof` and `Reason` is what one wants and is not shown at the root. | Show `Verdict::Unprovable(d) => println!("{d}")` in the common path. | None. | text (54w-e) |
| FL7 | `ordinary::decide` has five arguments and no short form; `ordinary::Options` beside `linear::Options`; "not valid" without a reason. | Q5. | | Q5 |
| FL8 | The crate root's second paragraph is one sentence of about 150 words listing modules. | A short list of what a caller does, each linked. | None. | text (54w-e) |

## 3. Someone at a terminal or in a script using the command

### Tasks and how they go today

**C1. Decide a sequent.** README's first proof works exactly as shown:

```console
$ deduxit prove "A, A -o B |- B"
provable (MLL, classical, focus engine)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
[exit 0]
```

The user wrote a two-sided sequent and gets a one-sided tree; README
says "one-sided sequent calculus" but not that `⊢ ~A, A` is the axiom
`A ⊢ A`. The two-sided form exists behind a flag:

```console
$ deduxit prove "A, A -o B |- B" --classical-sides two
provable (MLL, classical, focus engine)
───── ax   ───── ax
A ⊢ A      B ⊢ B
──────────────── ⊸L
  A, A ⊸ B ⊢ B
[exit 0]
```

**C2. A verdict in a script.** `-q` and the exit status do it; JSON for
more:

```console
$ deduxit prove -q "A |- B"
unprovable (MLL, classical, focus engine): ~A occurs 1 more time than A in the one-sided sequent, so they cannot all meet in axioms
[exit 1]
$ deduxit prove -q --format json "A |- B"; echo
{"version":1,"deduxit":"0.3.0","verdict":"unprovable","refutation":{"kind":"unbalanced","atom":0,"least":-1,"most":-1},"fragment":"MLL","mode":"classical","engine":"focus","statistics":{"nodes":1,"memo_hits":0,"memo_entries":1,"splits":0,"links":0,"tests":0,"copies":0,"forward_copies":0,"work":16},"sequent":{"terms":[{"D":0},{"V":1}],"roots":[0,1],"atoms":["A","B"],"antecedents":1}}

[exit 0]
```

The refutation is phrased one-sided ("~A … in the one-sided sequent")
although the user wrote `A |- B`; with `--classical-sides two` it is
the same text. In intuitionistic mode it speaks of sides:

```console
$ deduxit prove --classical-sides two "A |- B"
unprovable (MLL, classical, focus engine): ~A occurs 1 more time than A in the one-sided sequent, so they cannot all meet in axioms
[exit 1]
$ deduxit prove -i "A |- B"
unprovable (IMLL, intuitionistic, two-sided engine): A occurs negatively 1 more time than positively (positive right of ⊢, negative left of it, an implication's antecedent on the other side), so they cannot all meet in axioms
[exit 1]
```

**C3. Intuitionistic, affine.** One flag each:

```console
$ deduxit prove -i "A, A -o B |- B"
provable (IMLL, intuitionistic, two-sided engine)
───── ax   ───── ax
A ⊢ A      B ⊢ B
──────────────── ⊸L
  A, A ⊸ B ⊢ B
[exit 0]
$ deduxit prove -i "A, B |- A"
unprovable (IMLL, intuitionistic, two-sided engine): B occurs negatively 1 more time than positively (positive right of ⊢, negative left of it, an implication's antecedent on the other side), so they cannot all meet in axioms
[exit 1]
$ deduxit prove -a "A, B |- A"
provable (MLL, classical affine, focus engine)
  ─────── ax
  ⊢ ~A, A
─────────── wk
⊢ ~A, ~B, A
[exit 0]
```

The refutation of `A, B |- A` is right but does not say the plain
reason a newcomer needs (B is never used; `-a` allows that).

**C4. Common mistakes.** What a newcomer meets:

```console
$ deduxit "A |- A"
error: unrecognized subcommand 'A |- A'

Usage: deduxit <COMMAND>

For more information, try '--help'.
[exit 2]
$ deduxit prove A -o B
error: cannot parse the sequent
  A
   ^ unexpected end of input, expected a connective, `,` or `|-`
[exit 2]
$ deduxit prove "A -> B |- A"
error: cannot parse the sequent
  A -> B |- A
     ^ unexpected ">", expected the `o` of `-o`
[exit 2]
$ deduxit prove -i "A |- B par C"
error: cannot parse the sequent
  A |- B par C
         ^ unexpected "par"
  intuitionistic linear logic has no par: write ~A par B as A -o B; --ill-syntax one-sided reads the classical syntax
[exit 2]
$ deduxit prove --format tex "A |- A"
error: invalid value 'tex' for '--format <FORMAT>'
  [possible values: text, json, latex, typst, svg, png, pdf, rocq]

  tip: a similar value exists: 'text'

For more information, try '--help'.
[exit 2]
```

The `-i` hint is the model to follow. The others leave the user to
guess: no command named (suggest `deduxit prove "A |- A"`), an unquoted
sequent (the shell split it; suggest quoting), `->` in linear mode (it
is ordinary logic's arrow; suggest `-o` or `--logic`), `tex` (clap
suggests `text`, not `latex`).

**C5. Ordinary logic.**

```console
$ deduxit prove "a -> b, a |- b" --logic classical
valid (classical logic by the affine translation into affine MALL, focus engine)
──────── ax   ──────── ax
a ⊢ a, b      b, a ⊢ b
────────────────────── →L
     a → b, a ⊢ b
[exit 0]
$ deduxit prove --logic intuitionistic "~~a -> a"
not valid (intuitionistic logic by the cbn translation into ILL, two-sided engine): the image is unprovable: the search was exhaustive
[exit 1]
$ deduxit prove -q --logic intuitionistic "a |- b"
not valid (intuitionistic logic by the cbn translation into ILL, horn engine): the image is unprovable: b occurs positively 1 more time than negatively (positive right of ⊢, negative left of it, an implication's antecedent on the other side), so they cannot all meet in axioms
[exit 1]
```

"Not valid" is said about "the image", in linear terms, with no
counter-model, which is what a user of intuitionistic logic expects
(a Kripke model; a valuation for classical logic).

**C6. Many sequents.**

```console
$ printf "id: A |- A\nmp: A, A -o B |- B\nbad: A |- B\n" > s.txt; deduxit prove --input-format lines --file s.txt
id: provable (MLL, classical, focus engine)
mp: provable (MLL, classical, focus engine)
bad: unprovable (MLL, classical, focus engine): ~A occurs 1 more time than A in the one-sided sequent, so they cannot all meet in axioms
[exit 1]
$ printf "A |- A\nA, A -o B |- B\n" | deduxit prove --input-format lines
-:1: provable (MLL, classical, focus engine)
-:2: provable (MLL, classical, focus engine)
[exit 0]
```

Works. `--input-format lines` is needed even for a `.txt` file; without
it the file is read as one sequent (run outside the script, same scope):

```console
$ deduxit prove --file s.txt
error: cannot parse the sequent
  id: A |- A
    ^ unexpected ":" at line 1, character 3, expected a connective, `,` or `|-`
[exit 2]
```

The error could say that `--input-format lines` reads one sequent per
line (Q6).

**C7. Step by step.**

```console
$ printf "goals\nrules 0 2\napply 0 2 * 0\nclose\nproof\n" | deduxit interact "A, B |- A * B"
goal 0: ⊢ 0: ~A, 1: ~B, 2: A ⊗ B
⊗ (with a split)
opened goal 1: ⊢ 0: ~A, 1: A
opened goal 2: ⊢ 0: ~B, 1: B
goal 1: provable (MLL, classical, focus engine)
goal 2: provable (MLL, classical, focus engine)
no goal is open: `proof` checks the proof
valid proof (classical)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, ~B, A ⊗ B
[exit 0]
$ printf "apply 0 2 tensor 0\n" | deduxit interact "A, B |- A * B"
error: unknown rule `tensor`: ax, ⊗, ⅋, 1, ⊥, &, ⊕₁, ⊕₂, ⊤, !, ?d, ?c, ?w, mix, wk, cut, ∀, ∃, ⊸L, ⊸R, ⊗L, ⊗R, &L₁, &L₂, &R, ⊕L, ⊕R₁, ⊕R₂, 1L, 1R, 0L, ⊤R, !L, !R, !c, !w, ∀R, ∃L, ∃R, ∀L, open, \L, \R, /L or /R
[exit 1]
$ printf "apply 0 1 * 0\n" | deduxit interact "A, B |- A * B"
error: the rule ⊗ does not act on formula 1
[exit 1]
$ printf 'apply 0 2 * 1\nclose\n' | deduxit interact "A, B |- A * B"
opened goal 1: ⊢ 0: ~B, 1: A
opened goal 2: ⊢ 0: ~A, 1: B
goal 1: unprovable (MLL, classical, focus engine): A occurs 1 more time than ~A in the one-sided sequent, so they cannot all meet in axioms
goal 2: unprovable (MLL, classical, focus engine): ~A occurs 1 more time than A in the one-sided sequent, so they cannot all meet in axioms
[exit 1]
```

(The last command ran outside the script, in the same capped scope;
its `[exit 1]` is the session's status.) The unknown-rule list mixes
every mode's names, in Unicode, though the session takes `*`, `par`,
`+1`; "does not act on formula 1" does not say what formula 1 is (`~B`)
or that `rules 0 1` lists what does; a split that leaves both premises
unprovable is accepted silently, and `close` does not suggest `undo`.
The web page says at the split whether it can still close.

**C8. Help.**

```console
$ deduxit prove --help | wc -l
441
[exit 0]
$ deduxit prove -h | wc -l
93
[exit 0]
```

`-h` lists 65 options in one block (`deduxit prove -h | grep -cE '^\s+(-[a-zA-Z], )?--'`), in no order a newcomer can use
(the first five after `--file` are `--input-format`,
`--occurrence-limit`, `--term-limit`, `--ill-syntax`, then `-i`). The
top-level `deduxit --help` is good: the commands, then the syntax of
both suites.

### Frictions and the change that removes each

| # | friction | change | cost elsewhere | kind |
|---|---|---|---|---|
| FC1 | Two-sided input, one-sided tree, and the text never says `⊢ ~A, A` is `A ⊢ A`. | Say it in README and the manual's first proof, and show `--classical-sides two` there. Q7 asks about the default. | None for the text. | text (54w-c) + Q7 |
| FC2 | Classical refutations speak of "the one-sided sequent", even with `--classical-sides two`. | Phrase them in the sides the user wrote, as `-i` does (Q8). | Code in the reason's writer; behaviour lock lines. | Q8 |
| FC3 | A refutation that a flag would turn into a proof does not name the flag (`-i "A, B \|- A"` and `-a`). | Q8. | | Q8 |
| FC4 | `deduxit "A \|- A"`, an unquoted sequent, `->` in linear mode, `--format tex`: errors that do not say what to do. | Q6. | | Q6 |
| FC5 | The unknown-rule error lists all modes' Unicode names. | List the session's own rules with their ASCII spelling (Q9). | | Q9 |
| FC6 | "the rule ⊗ does not act on formula 1" names no formula and no next step. | Name the formula and point to `rules G P` (text of a message, 54w-d; it needs the formula at the error site, which the session has). | None. | text (54w-d) |
| FC7 | A losing split is accepted without a word. | Q9: say after `apply` when a premise is refuted by the counts, as the page does. | A cheap count per split. | Q9 |
| FC8 | `-h` is 93 lines, 65 options in one block. | clap's `help_heading` groups (the logic, the output, the limits, the search's tuning) and the input flags last (54w-d; layout only). Hiding expert flags from `-h` is Q10. | None for headings. | text (54w-d) + Q10 |
| FC9 | Ordinary logic's "not valid" is about "the image", with no counter-model. | Say it in ordinary terms (54w-d: "no proof in LJ; the search through the translation was exhaustive"); a counter-model is a feature (Q5). | | text + Q5 |
| FC10 | Two pieces of advice on deterministic answers: commands.md says "names its bound (`--copies`) or lifts the limit (`--timeout none`)", reference.md §6 and SKILL.md add `--work-limit`. | One sentence, the same in both: `--work-limit N` (or `--copies N`) for the same verdict everywhere. | None. | text (54w-c) |

## 4. A student or teacher on the web page

Read from the client's `main` copy (cited `src/…:line`) by the
sub-agent deduxit-web-usability, and run for real against the live
site https://deduxit.github.io/ with headless Chromium (Playwright
1.64.1 from the client checkout's `node_modules`, read only; no build),
capped (MemoryMax 4G, cores 12-15, `timeout 300`). Scripts `run.cjs`
(1280×800) and `phone.cjs` (390×844, touch), and their output
(`live/texts.md`, screenshots 01-12, the three exported files) are in
the session's job directory, `tmp/web/`. The run typed `A * B |- B * A`,
applied `⅋`, had `ax` on `B⊥` refused, opened the `⊗` split, pressed
Hint three times and Close all, exported LaTeX, Typst and Rocq, opened
Learn's lesson 1 and predicted `⊗`, and repeated the refusal at phone
width. I checked the main claims against the source and the files
(listed with each).

### Tasks and how they go today

**W-S1. A student proves a first sequent by clicking.** Type in the
field (placeholder `A * B |- B * A`, `src/islands/TopBand.tsx:277`),
Enter, click a formula, pick a rule from a popover of bare names (`⅋`,
`ax`, `⊗` with "split", `Stage.tsx:1574-1610`), choose the split, and
the page says "Proved: every goal is closed". The empty stage says only
"Type a sequent, or open one from Practice." (live, screenshot 01):
no syntax, no word of Learn or the Tour. A rule's meaning shows only
with Learn on and only on hover. The goal bar reads "Goal 2 · 1 open"
(goal ids). `A |- A` is shown as `⊢ A⊥, A` without a word.

**W-S2. Understand a refused step.** The note is a keyword, `ax
refused: not alone`; the sentence ("ax needs B⊥ alone in the goal with
its dual; other formulas are still there.") is only in `data-tip` and
the aria-label (`Stage.tsx:1513-1514`, checked), and no tip follows a
touch (`App.tsx:274`, checked; live at phone width: nothing). Beside
the refusal is **Undo** (`Stage.tsx:1538-1541`, checked: shown for a
"bad" note without an outcome, calls `prover.undo()`), which takes back
the last step that succeeded, since the refusal changed nothing.

**W-S3. A split.** "Choose the formulas that go to the left premise."
with Left/Right ("nothing else" when empty) and "Passes" / "Cannot
close", whose reason is again a tip (`Stage.tsx:1771-1830`). It never
says how to choose (click formulas on the stage).

**W-S4. A hint.** Three rungs (live): "Work on this goal: a split of
the context comes next." / "Look at B ⊗ A." / "Apply ⊗ to B ⊗ A, with
B⊥ going left."; a fourth press applies the step and asks "Applied. Why
was this the step to take?" with nowhere to answer
(`src/lib/words.ts:328-350`, `App.tsx:250-268`). "ladder", "rung",
"context" are jargon; nothing warns that the next press applies.

**W-S5. Let the search close a goal.** "Close goal" and "Close all",
with an engine chip. In Practice, a proof the search finished is not
marked proved (`Practice.tsx:57-77`), and the page does not say so.

**W-S6. Why a sequent has no proof.** Keywords ("Unprovable, checked:
atom balance", "Unknown: gave up", `words.ts:242-306`); the sentence is
in the status popover and only for the root goal (`state.tsx:790-795`).
A goal the student's own wrong step made unprovable reads like an
unprovable sequent. Lesson 7 says "Provable / Unprovable / Unknown",
the page "Proved, checked"; "Proved, not grafted" is jargon.

**W-S7. Lesson 1.** The opening (`src/content/lessons/1-reading.md`,
checked) introduces sequent, Γ, commas, dual, classical mode, the move
of hypotheses across `|-`, De Morgan and the double dual, before a
worked proof of one step (`A ⊢ A`). The prediction buttons offer `ax ⅋
⊗ no rule` before `⅋` and `⊗` are taught. With one step, the explain
prompt and "Proved" come at once, and "Your turn" has nothing left to
do. "Practice: decide first whether each has a proof." gives no
control to decide (`Learn.tsx:309`). A wrong hinge answer offers only
"Try the faded proof again", though each option names a problem
(`Learn.tsx:741-750`). Lesson 11 cites `deduxit seq print --logic
intuitionistic`, a command the page has not.

**W-S8. A Practice problem.** Filters (connectives, logic, source,
difficulty), "A random problem"; "opens as a proof on a new tab"
reads as a browser tab; the difficulty sentence is hard to read; the
fragment ("MLL") is not explained; Practice holds only provable
problems, which it does not say.

**W-T1. A teacher checks whether an exercise has a proof.** Type it,
Close all, read the note and the status popover; the logic is chosen
by the tab's `LL` mark, which does not look like a control and is
named three ways ("Calculus" in the tip, "Logic" in the dialog,
"Suite" in the fieldset). The restart warning and "Unknown … That says
nothing about provability." with "Try N units" are good.

**W-T2. Share an exercise.** Export › "Link — Copy a link to this
proof" (`BottomBand.tsx:644-648`); once a step is taken there is no
link to the bare sequent. The address bar updates live, never said.

**W-T3. Check a student's proof.** Import: "Paste a session or a
sequent's JSON, as deduxit writes them" (`BottomBand.tsx:712`); it
also takes a typed sequent, which the label does not say.

**W-L1. A logician exports.** "LaTeX — The derivation for ebproof",
"Typst — The derivation for curryst": both are fragments (live:
`\begin{prooftree}` and `#prooftree(` first, checked); the packages a
document needs (ebproof and cmll; curryst) are not named. "Rocq — The
finished proof's certificate for Rocq": the file starts at `Lemma
certificate (A B : formula) : ll […]` with no `Require` line (checked),
so it does not compile alone, and the page never says which kernel
(NanoYalla or rocq-deduxit) it needs. "Not derivable" (the certificate
of a refutation) has the same gaps; why an item is greyed is a tip.
The page passes no `form`, and the library's default is the fragment.
The net look named "deduxit" means nothing to a reader.

**W-L2. Ordinary logic on the page.** `LL` › Ordinary, LJ by default;
the syntax hint is inside the popover only; nothing visible names
ordinary logic (the h1 is for screen readers, `src/pages/index.astro:109`).
The λ-term item and the engine choice work well.

**Stale names.** The privacy page lists the storage keys
`linlog-settings`, `linlog-theme`, `linlog-practice`,
`linlog-proofs:…`, `linlog-proofs-previous` and the cache
`linlog-site-…` (`src/pages/privacy.astro:171-196`, checked), which a
user also sees in the browser's tools. No "sequentium" and no
user-facing "linlog" elsewhere (the `/rocq-linlog/` forward is
intended).

### Frictions and the change that removes each

| # | friction | change | kind |
|---|---|---|---|
| FW1 | Reasons (refusal, verdict, "Cannot close", greyed export) are tips only; touch never sees them. | Show the sentence as a visible line under the keyword (W1). | W1 |
| FW2 | Undo beside a refusal undoes the last good step. | Remove it, or offer "Other rules" (W2). | W2 |
| FW3 | The first visit gives no syntax and no way to Learn or the Tour. | A syntax line (text); buttons for lesson 1 and the Tour (W3). | text + W3 |
| FW4 | Rules are bare symbols; their meaning is a hover with Learn on. | Show each rule's one-line `says` under its name with Learn on (W4). | W4 |
| FW5 | Refusal and hint sentences say what is wrong, not what to do; "ladder", "rung", "context"; the fourth Hint applies silently. | Rewrite (54w-a): add the next action ("Apply the other formulas' rules first"; "Press Hint again to apply it"). | text |
| FW6 | A goal lost by the student's step reads like an unprovable sequent. | "This goal has no proof. The sequent may still have one: an earlier step lost it." (54w-a); a "Go back" button is optional (W2). | text |
| FW7 | Lesson 1 explains eight ideas before a one-step example; predictions offer untaught rules. | Cut the opening to what the example shows; De Morgan to lesson 2; `choices: ["ax", "no rule"]` (54w-a). A longer worked proof or the opening below it: W13. | text + W13 |
| FW8 | "Decide first" with no control; a wrong hinge answer does not offer its problem. | W5, W6. | W5, W6 |
| FW9 | Exports do not compile alone and do not name what they need. | Name the packages and the kernel in each item's detail (54w-a); a standalone form (W7). | text + W7 |
| FW10 | No link to a bare sequent once a step is taken. | W8. | W8 |
| FW11 | The logic mark is not a visible control and has three names. | One name, "Logic" (54w-a); a visible label on wide screens (W9). | text + W9 |
| FW12 | "Goal 2 · 1 open" numbers goals by id. | W10. | W10 |
| FW13 | Practice's "proved" rules are unstated. | Say it (54w-a); W11 asks which rule. | text + W11 |
| FW14 | Lesson 11 cites a command-line call; Practice's "new tab"; the difficulty sentence; "Proved, not grafted"; the net look "deduxit". | Rewrite (54w-a). | text |
| FW15 | `linlog-*` storage keys on the privacy page. | W14. | W14 |


## 5. A logician checking a proof or exporting it

### Tasks and how they go today

**R1. A proof tree for a paper.** One flag, or the file's extension:

```console
$ deduxit prove --format latex --output p.tex "A, A -o B |- B"; cat p.tex
% provable (MLL, classical, focus engine)
\begin{prooftree}
\infer0[$\mathrm{ax}$]{\vdash A^\bot, A}
\infer0[$\mathrm{ax}$]{\vdash B^\bot, B}
\infer2[$\otimes$]{\vdash A^\bot, A \otimes B^\bot, B}
\end{prooftree}
[exit 0]
$ deduxit prove --output proof.tex "A |- A"; head -3 proof.tex
% provable (MLL, classical, focus engine)
\begin{prooftree}
\infer0[$\mathrm{ax}$]{\vdash A^\bot, A}
[exit 0]
$ deduxit prove --format latex --standalone "A |- A"
% provable (MLL, classical, focus engine)
\documentclass[border=5pt]{standalone}
\usepackage{amssymb}
\usepackage{cmll}
\usepackage{ebproof}
\begin{document}
\begin{prooftree}
\infer0[$\mathrm{ax}$]{\vdash A^\bot, A}
\end{prooftree}
\end{document}
[exit 0]
$ deduxit prove -i --format typst "A, A -o B |- B"
// provable (IMLL, intuitionistic, two-sided engine)
#prooftree(
  rule(
    name: $⊸ upright(L)$,
    rule(name: $"ax"$, $A ⊢ A$),
    rule(name: $"ax"$, $B ⊢ B$),
    $A, A ⊸ B ⊢ B$,
  ),
)
[exit 0]
$ deduxit prove --classical-sides two "A * B |- B * A"
provable (MLL, classical, focus engine)
───── ax   ───── ax
B ⊢ B      A ⊢ A
──────────────── ⊗R
  A, B ⊢ B ⊗ A
  ───────────── ⊗L
  A ⊗ B ⊢ B ⊗ A
[exit 0]
```

Good. A paper on classical LL with two-sided proofs needs
`--classical-sides two` (C1, Q7).

**R2. Check a proof file, invalid proofs included.**

```console
$ deduxit prove --format json "A * B |- B * A" > p.json; deduxit check p.json; deduxit check -q p.json
valid proof of ⊢ ~A ⅋ ~B, B ⊗ A (classical)
─────── ax   ─────── ax
⊢ ~B, B      ⊢ ~A, A
──────────────────── ⊗
  ⊢ ~A, ~B, B ⊗ A
  ──────────────── ⅋
  ⊢ ~A ⅋ ~B, B ⊗ A
valid proof of ⊢ ~A ⅋ ~B, B ⊗ A (classical)
[exit 0]
$ sed 's/{"ax":\[4,2\]}/{"ax":[4,1]}/' p.json > bad.json; deduxit check bad.json
invalid proof of ⊢ ~A ⅋ ~B, B ⊗ A (classical): node 0 (ax on B, ~A): the literals are not an atom and its negation
[exit 1]
```

(The second ran outside the script, in the same capped scope.) Clear.
Writing a proof by hand means writing the JSON of numbered occurrences;
the practical way is an `interact` session's `proof FILE`.

**R3. A Rocq certificate, and checking it.**

```console
$ deduxit prove --format rocq "A * B |- B * A"
(* provable (MLL, classical, focus engine) *)
Lemma certificate (A B : formula) : ll [parr (dual A) (dual B); tens B A].
Proof.
apply (parr_r_ext []); cbn_sequent.
apply (ex_perm_r [2; 0; 1] [dual B; tens B A; dual A]).
apply (tens_r_ext [dual B]); cbn_sequent.
{
  ax_expansion.
}
{
  ax_expansion.
}
Qed.
[exit 0]
$ deduxit prove -i --format rocq "A, A -o B |- B"
(* provable (IMLL, intuitionistic, two-sided engine) *)
Lemma certificate : ill (atom := string) intuitionistic [ivar "A"; ilmap (ivar "A") (ivar "B")] (ivar "B").
Proof.
  apply (soundness_ill String.eqb String.eqb_spec intuitionistic
    (ILmapL (ivar "A") (ivar "B") (IAx "A") (IAx "B"))).
  vm_compute. reflexivity.
Qed.
[exit 0]
```

Writing it is one flag. Checking it was not run here: the classical
default kernel, NanoYalla, is installed by hand from a subdirectory of
the Click & coLLecT repository (`docs/src/rocq.md`), and the other
modes' kernel, `rocq-deduxit`, from opam or Nix. The manual's first
install line, `opam install rocq-deduxit` after adding
`rocq-released`, fails today: the package is not in the archive
(`https://github.com/rocq-prover/opam/tree/master/released/packages/rocq-deduxit`
answered 404 on 2026-11-09). The manual says "once the release is
there", but a reader copies the block. So a logician needs two kernels,
installed two ways, to check certificates of two modes.

**R4. Proof nets and drawings.**

```console
$ deduxit prove --net "A * B |- B * A"
provable (MLL, classical, focus engine)
⊢ ~A ⅋ ~B, B ⊗ A
~A[1] — A[5]
~B[2] — B[4]
proof net
[exit 0]
$ deduxit prove --net --format svg --output net.svg "A * B |- B * A"; head -c 114 net.svg; echo
<!-- provable (MLL, classical, focus engine) -->
<svg xmlns="http://www.w3.org/2000/svg" role="img" width="134.56"
[exit 0]
```

The last line, `proof net`, is the correctness criterion's verdict; it
reads as a stray label. A PDF or PNG without `--output` goes to a pipe,
and the command refuses to write it to a terminal (`cli/src/show.rs:392`).

**R5. Cut elimination.**

```console
$ printf "cut 0 B 0 1\nclose\nproof cut.json\n" | deduxit interact -i "A, A -o B, B -o C |- C" >/dev/null; deduxit eliminate cut.json
1. a commutative case on the cut of B
2. the axiom case on the cut of B
cut-free proof of A, A ⊸ B, B ⊸ C ⊢ C (intuitionistic) after 2 steps
           ───── ax   ───── ax
           B ⊢ B      C ⊢ C
───── ax   ──────────────── ⊸L
A ⊢ A        B, B ⊸ C ⊢ C
───────────────────────── ⊸L
   A, A ⊸ B, B ⊸ C ⊢ C
[exit 0]
```

Good.

**R6. The lambda-term.**

```console
$ deduxit prove -i --lambda "A -o B, B -o C |- A -o C"
provable (IMLL, intuitionistic, two-sided engine)
           ───── ax   ───── ax
           B ⊢ B      C ⊢ C
───── ax   ──────────────── ⊸L
A ⊢ A        B, B ⊸ C ⊢ C
───────────────────────── ⊸L
   A ⊸ B, B ⊸ C, A ⊢ C
   ──────────────────── ⊸R
   A ⊸ B, B ⊸ C ⊢ A ⊸ C
lambda-term: f : A ⊸ B, g : B ⊸ C ⊢ λx. g (f x) : A ⊸ C
[exit 0]
$ deduxit prove --lambda "A |- A"
provable (MLL, classical, focus engine)
─────── ax
⊢ ~A, A
no lambda-term: a proof of classical logic has several conclusions, which no lambda-term types
[exit 0]
```

The second could name the flag that gives a term (`-i`).

### Frictions and the change that removes each

| # | friction | change | cost elsewhere | kind |
|---|---|---|---|---|
| FR1 | Checking a certificate needs NanoYalla (by hand) for classical proofs and rocq-deduxit for the rest; `opam install rocq-deduxit` fails today. | Q11 (one kernel by default, a one-command checker). The manual's opam block says plainly that the package is not yet in the archive and leads with the pin or Nix (text, 54w-c). | | Q11 + text (54w-c) |
| FR2 | `--net`'s last line `proof net` reads as a label. | `the correctness criterion holds: a proof net` or similar (text of `Display`, 54w-e nets / 54w-d). | Behaviour lock and manual lines. | text |
| FR3 | `--lambda` on a classical proof says why there is no term but not how to get one. | Add "prove it with -i" (54w-d). | None. | text |

## 6. Questions for the author

Each changes the library's API, the command's flags or behaviour, or
the page's behaviour, so none is made in step 54w. Each has a
recommendation of the idiomatic, current practice, what it costs other
uses, and what the rewritten docs say either way.

### The library

**Q1. A stop that does what the command does.** Should the library
give the command's stop condition, so that a caller matches the
command's verdicts in one line?
- *Recommendation:* yes, with the clock injected, as the crate already
  takes its time from the caller: `Settings::stop(&self, elapsed: impl
  FnMut() -> Duration) -> impl FnMut(Progress) -> bool`, which fires
  past `clock.time_limit_ms` in every phase but `Phase::Refute` (the
  rule `cli/src/prove.rs:88` applies). A caller writes
  `prove_within(&s, mode, &settings.search, &settings.limits,
  settings.stop(|| start.elapsed()))`. It keeps the crate free of a
  clock (wasm): the caller supplies the time, as `prove_within`'s stop
  already does.
- *Cost:* one public item, additive (0.3.x). It ties `Settings` to the
  meaning of the phases.
- *Docs either way:* the crate root's recipe must be fixed in 54w-e in
  any case (L3). With Q1, it is the one line above. Without it, the
  recipe's closure matches on `progress.phase` and lets `Phase::Refute`
  run, about five lines, with a sentence saying why.

**Q2. Printing without a stop.** Should a built term print with
`Display`, and each export have a call without `Limits` and stop?
- *Recommendation:* yes, following the crate's own pair
  `Proof::derivation()` / `derivation_within(view, limits, stop)` and
  the Rust API Guidelines' C-COMMON-TRAITS (`Display` where a value
  has one obvious text): `impl Display for lambda::Term` (default
  options), and `export::latex::to_string(derivation, &options) ->
  Result<String, Error>` (the same for `typst`, `svg`, `rocq`), the
  `write` functions staying for streams and stops.
- *Cost:* five additive items. A very large derivation exported
  without a stop runs to the end, bounded by the default `Limits` as
  `derivation()` is.
- *Docs either way:* L4 and L5's examples lose the `|_| false` and the
  `String` buffer with Q2; without, the docs say once, at the crate
  root, that `|_| false` is "never stop" and every printer takes it.

**Q3. Reading the command's JSON in the library.** Should `Outcome`
implement `Deserialize`, so that a program reads `prove --format
json`'s document with its mode and verdict?
- *Recommendation:* yes. The API Guidelines' C-SERDE asks data types to
  implement both; the wire levels already promise that later releases
  read earlier documents. `Proof` alone stays readable as today.
- *Cost:* reading outcomes becomes a promise at every wire level,
  `statistics` and `engine` included (an unknown engine name would
  have to be read, not refused).
- *Docs either way:* with Q3, "check a file the command wrote" is
  `let o: Outcome = serde_json::from_str(&text)?;` then
  `o.verdict`'s proof `.check(o.mode)`. Without, the docs show reading
  `Proof` and parsing the `mode` key with `str::parse::<Mode>()` (L7).

**Q4. Errors that show the formula.** Should `prove` in
intuitionistic mode, given a `Sequent` with a formula that has no
intuitionistic reading, say which formula, as `IllSequent`'s parser
does?
- *Recommendation:* yes, carry the formula's text in the error where
  the call has the forest (it does), so that `Display` reads "not an
  intuitionistic sequent: A ⅋ B has no intuitionistic reading; write
  ~A par B as A -o B". The design of ids plus `describe` stays for
  errors raised far from the forest.
- *Cost:* an allocation on an error path; the `Display` text changes
  (not promised).
- *Docs either way:* the common path shows `IllSequent` for
  intuitionistic input (text work, FL5), so most callers never meet
  this error.

**Q5. Ordinary logic in one call, and why "not valid".** Should
`ordinary` take the linear suite's shape, `decide(&sequent, &options)`
beside `decide_within(&sequent, &options, &search, &limits, stop)`, and
should "not valid" come with a counter-model?
- *Recommendation:* the shape, yes, at the next breaking release
  (0.4): one convention for both suites is what a reader of `prove`
  expects. The counter-model (a valuation for classical logic, a
  Kripke model for intuitionistic) is the standard answer of an
  intuitionistic prover and is worth a step of its own; put it in
  `plan/later.md`, not in 54w.
- *Cost:* a breaking rename of `decide`'s signature; the counter-model
  is new search work.
- *Docs either way:* the module doc's example shrinks to three lines
  with the new shape. Without it, the docs name the five arguments in
  a list and say that `|_| false` never stops. "Not valid" is
  explained in ordinary terms either way (FC9).

### The command

**Q6. Errors that say what to do.** Should the command add hints to
the errors a newcomer meets first (C4)?
- *Recommendation:* yes, in the style of the existing `-i` hint
  (what went wrong, then what to write), as git and cargo do for an
  unknown subcommand:
  - `deduxit "A |- A"`: "to decide a sequent, run `deduxit prove "A
    |- A"`";
  - an unquoted sequent: `deduxit prove A -o B` reads `-o B` as
    `--output B` and the sequent as `A`; when the parse ends early
    and the command line had more words, say "quote the sequent:
    `deduxit prove "A -o B"`";
  - `->` in linear mode: "`->` is ordinary logic's arrow: write `-o`,
    or add `--logic classical`";
  - `--format tex`: accept `tex` as an alias of `latex` (clap's
    `value(alias = …)`), since clap's own tip suggests `text`;
  - a file of `NAME: SEQUENT` lines read as one sequent: "to read one
    sequent per line, add `--input-format lines`".
- *Cost:* code in the error paths, behaviour lock lines; an alias is
  one more accepted spelling.
- *Docs either way:* the manual's "mistakes to avoid" (reference.md
  §19, and a short how-to in the manual) lists these; with Q6 it can
  show the hint, without it it must say the fix itself.

**Q7. Two-sided input, two-sided proof?** Should a classical sequent
written with `|-` and formulas on both sides print a two-sided
derivation by default?
- *Recommendation:* no. The one-sided calculus is the standard one for
  classical linear logic (Girard 1987; Click & coLLecT), it is what
  the checker, the JSON and the NanoYalla certificates use, and
  `--classical-sides two` exists. Fix it in the text: the first proof
  in README and the manual says in one line that `⊢ ~A, A` is the
  axiom `A ⊢ A` and shows `--classical-sides two`.
- *Cost of the change, if made:* every classical example in README,
  the manual, the guide and the behaviour lock changes; scripts that
  parse the text tree break.
- *Docs either way:* as above; with the change, the one-sided tree
  becomes the flag's example instead.

**Q8. Refutations in the user's terms.** Should a classical
refutation speak of the sequent as written (sides), and name the flag
that would make the sequent provable where one does?
- *Recommendation:* yes. The intuitionistic reason already speaks of
  sides; the classical one says "in the one-sided sequent" even for
  `A |- B` and with `--classical-sides two`. Phrase it "A on the left
  has no A on the right to meet in an axiom" where the input was
  two-sided. Where weakening would close the gap (an atom only on the
  left), add "with -a (affine), an unused hypothesis is allowed".
- *Cost:* the reason's writer needs the reading (it has it in
  intuitionistic mode); behaviour lock lines; the JSON `refutation`
  is unchanged.
- *Docs either way:* the manual's "why unprovable" section explains
  the counts once in plain words; with Q8 the examples read without
  that explanation.

**Q9. `interact` that guides.** Should `interact` list the session's
own rules with their ASCII spellings on an unknown rule, and warn after
a split whose premise the counts refute?
- *Recommendation:* yes, both. The page already warns at a split; the
  command should not be the harsher of the two. The unknown-rule error
  lists only the rules of the session's mode, ASCII first (`*`,
  `par`, `+1` …).
- *Cost:* a cheap count per split (the counts the refutation already
  computes); behaviour lock lines.
- *Docs either way:* the interactive chapter's example shows the
  warning with Q9; without it, it shows `close` refuting both goals
  and `undo`.

**Q10. A short `-h`.** Should `prove -h` hide the expert flags (the
limits past `--timeout` and `--memory-limit`, `--bias`,
`--forward-copies`, `--memo-limit`, `--pool-after`, the bench inputs),
leaving them to `--help`?
- *Recommendation:* yes, clap's `hide_short_help` with
  `help_heading` groups, as ripgrep and cargo keep `-h` to the common
  flags. 65 options in `-h` is past what a reader scans.
- *Cost:* a user who knew a flag from `-h` finds it in `--help`.
  `cli/tests/readme.rs` checks that every flag the guide names exists,
  not that `-h` shows it (to confirm in 54w-d).
- *Docs either way:* the help says at its end "`--help` shows every
  option"; headings are 54w-d's work whatever the answer.

### The logician

**Q11. Checking certificates without a scavenger hunt.** Should
checking a certificate be one command, and one kernel by default?
- *Recommendation:* yes, in two steps. Now: an app in this flake,
  `nix run github:deduxit/deduxit#check-certificate -- proof.v`,
  which compiles a certificate against the pinned rocq-deduxit (and
  NanoYalla for the classical default), as the checks `rocq` and
  `rocq-deduxit` already do. When rocq-deduxit is in the opam
  archive: make `--kernel deduxit` the default in every mode, so that
  one library checks every certificate; NanoYalla stays a flag for
  Click & coLLecT users.
- *Cost:* a flake app to keep; the default's change alters every
  classical certificate in the manual and the behaviour lock.
- *Docs either way:* the Rocq chapter leads with the shortest working
  way (today: the pin or `nix build`, never the unpublished opam line
  first).

### The page

Proposed by deduxit-web-usability, recommendations checked against the
sources it cites; the client's tests that assert exact strings are
listed in section 7.

**W1. Reasons in view.** Should every reason (a refused step, a
verdict, "Cannot close", a greyed export item) show as a visible line
under its keyword instead of a tip?
- *Recommendation:* yes. NN/g's tooltip guidelines say a tooltip must
  not hold information the task needs and does not work on touch
  (https://www.nngroup.com/articles/tooltip-guidelines/); Nielsen's
  heuristic 9 (errors that say how to recover) points the same way.
- *Cost:* the dock grows by a line; the long saturation sentence needs
  a "more"; e2e tests that use `toHaveText` on keywords move to
  `toContainText`.
- *Text either way:* each sentence says what to do next (FW5); with
  W1 it is read by everyone, without it only on hover and by screen
  readers.

**W2. No Undo beside a refusal.** Should the Undo button beside a
refused step go (`Stage.tsx:1538`), optionally replaced by "Other
rules", which reopens the popover?
- *Recommendation:* yes; a refusal changed nothing, so Undo there
  takes back a good step. No test found that needs it.
- *Cost:* none found.
- *Text either way:* the refusal sentence ends with the next action.

**W3. Help on the first visit.** Should the empty stage offer
"Start lesson 1" and "Take the tour" as buttons, beside a line of
syntax?
- *Recommendation:* yes, as buttons, never an automatic tour (NN/g,
  empty states in complex applications,
  https://www.nngroup.com/articles/empty-state-interface-design/).
- *Cost:* a busier empty stage.
- *Text either way:* "Type a sequent, such as `A * B |- B * A` (`*`
  ⊗, `|` ⅋, `-o` ⊸, `|-` ⊢), or open one from Practice." The buttons'
  words come with W3.

**W4. What a rule does, in the popover.** With Learn on, should each
rule in the rule popover show its one-line `says` under its name?
- *Recommendation:* yes (recognition rather than recall, heuristic 6).
- *Cost:* a taller popover; nothing changes with Learn off.
- *Text either way:* the `says` lines are one clause each, every term
  defined (54w-a).

**W5. A wrong hinge answer offers its problem.** Should a wrong answer
in Learn's check offer "Try ⊢ A, A yourself" (the problem the option
names) before "Try the faded proof again"?
- *Recommendation:* yes (feedback that says where to go next, Hattie
  and Timperley 2007; hinge questions, Wiliam 2011).
- *Cost:* one button.
- *Text either way:* the misconception line stays a clause read after
  "This answer comes from the idea that …".

**W6. Answer buttons for "decide first".** Should Learn's practice
rows have "Has a proof" / "No proof" with immediate feedback?
- *Recommendation:* yes, or drop the instruction "decide first"
  (retrieval practice with feedback, Roediger and Karpicke 2006).
- *Cost:* more controls per row; the "Answer" disclosure becomes the
  feedback.
- *Text either way:* without W6, the instruction must not ask for a
  decision the page cannot take.

**W7. Exports that compile alone.** Should the Rocq export always be
standalone, and LaTeX and Typst offer a "Whole document" item?
- *Recommendation:* yes; the library has `Form::Standalone`, and the
  command offers `--standalone`. The fragment stays the default for
  LaTeX and Typst (it is what one pastes into a paper).
- *Cost:* one item; the e2e tests on exports use `toContain`/`toMatch`
  and keep passing (`tests/prover.spec.ts:186-190`,
  `tests/ordinary.spec.ts:157-159`, per the sub-agent).
- *Text either way:* each item's detail names what the file needs
  (ebproof and cmll; curryst; NanoYalla or rocq-deduxit).

**W8. A link to the bare sequent.** Should Export offer "Link to the
sequent" beside "Link to this proof"?
- *Recommendation:* yes; a teacher shares exercises, not solutions.
- *Cost:* one item; the link codec already takes a sequent alone.
- *Text either way:* "Link" becomes "Share a link".

**W9. The logic choice in view.** Should the tab show "Linear (LL) ▾"
or "Ordinary (LJ) ▾" on wide screens, the short mark on phones?
- *Recommendation:* yes (heuristics 4 and 6), with one name, "Logic".
- *Cost:* a wider tab; `tests/prover.ts` finds `.pt-mark` and the
  dialog "Logic", unaffected if they stay.
- *Text either way:* one name, "Logic", everywhere (54w-a).

**W10. Goals numbered among the open ones.** Should the goal bar say
"Open goal 1 of 1" instead of "Goal 2 · 1 open"?
- *Recommendation:* yes for the bar; the Steps list keeps the ids.
- *Cost:* two numberings on screen, one per view.
- *Text either way:* the bar's words.

**W11. What counts as proved in Practice.** Should steps that Hint
applied keep counting, and the search's closes keep not counting?
- *Recommendation:* keep today's rule and say it in Practice's text.
- *Cost:* none.
- *Text either way:* "A problem counts as proved when you close every
  goal yourself; Hint may help."

**W12. One-sided lessons.** Should lessons 1-5 keep the one-sided
calculus, and lesson 1 show the rewrite `A |- A` to `⊢ A⊥, A` with the
Two-sided switch on, then off?
- *Recommendation:* keep the one-sided calculus (the textbook one, half
  the rules); make the rewrite lesson 1's worked example.
- *Cost:* a lesson-1 interaction in the lesson view.
- *Text either way:* lesson 1 says in one sentence what the rewrite is.

**W13. Example before explanation.** If cutting the openings (FW7) is
not enough, should a lesson's opening move below its worked proof, and
lesson 1 get a worked proof of two or three steps?
- *Recommendation:* the text cut first (54w-a); the move only if
  needed (Sweller and Cooper 1985; Renkl 2014).
- *Cost:* none for the text; the move changes the lesson view.
- *Text either way:* each opening holds only what its worked proof
  shows.

**W14. `deduxit-*` storage keys.** Should the `linlog-*` keys be
renamed, migrating on load for one release?
- *Recommendation:* yes, read the old key, write the new, delete the
  old; update the privacy page.
- *Cost:* the tests that name the keys (`tests/prover.spec.ts`,
  `tests/targets.spec.ts`, `tests/engines.spec.ts`,
  `src/lib/shelf.test.ts`, the offline spec's cache prefix).
- *Text either way:* the privacy page lists the keys as they are.

## 7. Text work for the 54w sessions

What the 54w sessions can do without asking, because it changes text
only. "Fact" marks a claim that is wrong today and must be fixed, not
just reworded.

### 54w-c: README, the manual, the guide

- **Fact.** `docs/src/introduction.md:66` says "the other provers on
  the growing families are not yet run"; the manual has "The other
  provers on the growing sets, 2026-11-03". README's paragraph on the
  manual already says they ran.
- **Fact.** `skills/deduxit/reference.md` §16: "`Settings::default()`
  is the command's behaviour as data" holds for the options, not for
  the stop (L3, FL1); say that the stop must let the refutation run.
- **Fact (misleading).** `docs/src/rocq.md`: `opam install
  rocq-deduxit` fails until the package is in the archive (R3). Lead
  with what works today (the pin, `nix build`).
- The first proof (README, introduction, commands): one line that
  `⊢ ~A, A` is the axiom `A ⊢ A` read one-sided, and
  `--classical-sides two` beside it (FC1).
- One sentence on deterministic answers, the same in the manual and
  the guide: `--work-limit N` or `--copies N` (FC10).
- `docs/src/commands.md` is 726 lines under one heading, mixing the
  how-to (decide a sequent, read the answer, limits) with the
  explanation of four engines. Split it: a how-to page per task with
  headings, the engines in an explanation page. The newcomer's path
  is: decide a sequent, read the verdict line and the exit status,
  what unknown means and the flag to raise, `-i`, export, `check`.
- README opens with a 60-word list ("Logic suites for all your
  needs, …") and two paragraphs of history before Install. Lead with
  one sentence of what it does, then Install and the first proof; the
  history goes to the changelog.
- `introduction.md` repeats README's opening paragraphs; let it be
  the tutorial's first page instead (a first proof, a first
  unprovable sequent and how to read why, a first `-i` proof).
- The manual has no "mistakes to avoid" (only reference.md §19 has
  one): add a how-to with C4's cases and their fixes (quote the
  sequent; `prove` is needed; `-o` not `->`; `--input-format lines`).
- "Ordinary logic": say what "not valid" means in ordinary terms
  before the translation (FC9).
- Installation's "Which to prefer" (musl against glibc timings) comes
  before the per-system sections; it is a note for long searches, not
  the first thing a newcomer reads.

### 54w-d: the command's help, messages and errors

- `help_heading` groups for `prove -h` (FC8), whatever Q10's answer.
- "the rule ⊗ does not act on formula 1": name the formula and point
  to `rules G P` (FC6).
- `--net`'s last line `proof net`: say it is the criterion's verdict
  (FR2; the text is `Display` in `core/src/linear/nets/mod.rs:1404`
  and `nets/mell/mod.rs:708`, so 54w-e's nets session owns it; agree
  who changes it).
- `--lambda` on a classical proof: add that `-i` gives a term (FR3).
- Ordinary logic's "not valid … the image is unprovable": the line is
  read by people who never asked for the image (FC9).
- The behaviour lock's changed lines are a commit of their own (the
  step's rule).

### 54w-e and on: the crate's docs

- **Fact.** The crate root's paragraph "The command's verdicts differ
  from `Options::default()`'s on purpose" and its example claim to
  decide what the command decides; they do not (L3). Fix the example
  so its stop lets `Phase::Refute` run, or use Q1's helper.
- The common path: show `Verdict::Unprovable(d) => println!("{d}")`
  (FL6), `IllSequent` for intuitionistic input (FL5), and that
  `|_| false` means "never stop" (FL3).
- The root's second paragraph is one sentence of about 150 words
  (FL8).
- `ordinary`'s module doc: where the reason of "not valid" is
  (`outcome.linear.verdict`), until Q5 is answered.

### 54w-a: the page

- Where the text lives: `src/content/texts/*.md`, `tour/*.md`,
  `lessons/*.md` (front matter `steps[].reason`, `explain`, `hinge`,
  `detour`, `attempt`, `gallery`, `unknown`, `refusal`,
  `eliminate.cases`), `rules/*.md` (`says`), the problems' bodies,
  `examples.json` (`note`); in code `src/lib/words.ts`,
  `src/lib/ordinary.ts:287-364`, `engines.ts`, `practice.ts`,
  `suites.ts`, the islands, `src/pages/index.astro` and
  `privacy.astro`.
- FW5, FW6, FW7 (text part), FW9 (the details: packages, kernel),
  FW11 (one name), FW13, FW14, and the first-visit line of FW3.
- Lesson 7's words against the page's ("Provable" against "Proved,
  checked"); lesson 8's "MLL or MELL" and "switching condition"
  undefined; lesson 2's `A -o B` belongs in lesson 6; lesson 11's
  command line becomes "click the mark beside the field and choose
  Ordinary".
- The tour's stop 7 explains the verdict by colour alone: name the
  shapes too (WCAG 1.4.1); stop 8's "branch" is undefined.
- Practice's opening: "a new proof in a tab beside the field";
  "Difficulty counts the steps a proof by hand takes and the choices
  it makes"; only provable problems are here, the unprovable ones in
  Learn.
- Writing-rule breaches: `BottomBand.tsx:812-815` ("about under one
  second"), `:840` ("The most steps of cut elimination "To the end"
  takes."), the invertible tip's three ideas; `texts/reference.md`
  promises schemas "on hover", which touch never shows.
- Traps (the sub-agent's list): formulas go between backticks
  (`ui.tsx:141-148`); lesson titles live in the front matter and in
  `LESSONS` (`Learn.tsx:66-78`, asserted by `lessons.test.ts:61-75`);
  `choices` holds every played rule and `faded`, `explain.after` stay
  below the step count (`lessons.test.ts:77-91`); one hinge option has
  no misconception, and an option's problem is provable exactly when
  it is right (`:267-282`); a misconception is a clause; the
  reference's readings equal `READINGS` (`words.ts:414-430`); the
  additive engine's entry keeps "Table 1.4" and Whitman after
  "Related:" (`reference.test.ts:18-27`); lesson 7's "Press Try"
  matches the button "Try N units"; the tour has 8 stops
  (`tests/learn.spec.ts:67-74`); CC BY 4.0 is shown on every page of
  written content (`modules/_checks/licences.py`).
- Tests that assert exact UI strings (counts of string assertions per
  the sub-agent): `tests/prover.spec.ts` [253] and `tests/prover.ts`,
  `lessons.spec.ts` [143], `targets.spec.ts` [79], `ordinary.spec.ts`
  [76], `cut.spec.ts` [59], `learn.spec.ts` [50], `engines.spec.ts`
  [50], `first-order.spec.ts` [46], `modes.spec.ts` [33],
  `page.spec.ts` [32], `practice.spec.ts` [29], `phone.spec.ts` [25],
  `dismiss.spec.ts` [23], `term.spec.ts` [15], `invertible.spec.ts`
  [14], `offline.spec.ts` [6]; unit tests `src/lib/words.test.ts`
  (the hint rungs, `:80-110`), `steps.test.ts`,
  `src/content/lessons.test.ts`, `reference.test.ts`. Change a label
  and its test in one commit; prefer `toContainText` where the words
  will move.

## Appendix A: the library program

`Cargo.toml`:

```toml
[package]
name = "libuser"
version = "0.1.0"
edition = "2024"

[dependencies]
deduxit = { path = "/home/tux/Projects/own/linlog-step54w-r/core" }
serde_json = "1"

[workspace]
```

`src/main.rs`, run as `cargo run -q -- PATH/p.json` with `p.json` and `refuted.json` beside each other, written by `deduxit prove --format json "A * B |- B * A" > p.json` and `deduxit prove --format json "A |- B" > refuted.json`:

```rust
use deduxit::linear::{Mode, Options, Sequent, Verdict, prove};
use deduxit::{Limits, Settings};
use std::time::{Duration, Instant};

type R = Result<(), Box<dyn std::error::Error>>;

fn t1_prove() -> R {
    println!("== T1: is it provable? print the proof");
    let sequent: Sequent = "A, A -o B |- B".parse()?;
    let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
    match &outcome.verdict {
        Verdict::Proved(proof) => println!("provable\n{}", proof.derivation()?),
        Verdict::Unprovable(_) => println!("unprovable"),
        Verdict::Unknown(_) => println!("unknown"),
    }
    Ok(())
}

fn t2_why_not() -> R {
    println!("== T2: why unprovable");
    let sequent: Sequent = "A |- B".parse()?;
    let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
    if let Verdict::Unprovable(d) = &outcome.verdict {
        println!("Display of the disproof: {d}");
    }
    Ok(())
}

fn t3_default_vs_command() -> R {
    println!("== T3: library default vs the command, on three sequents");
    for s in ["!(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D", "?A par ?A |- ?(A * A)", "!(A -o B), !(B -o C), !(C -o D), !(D -o E), !(E -o F), A |- F * !(A -o A)"] {
    println!("-- {s}");
    let sequent: Sequent = s.parse()?;
    let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
    println!("Options::default(): {}", name(&outcome.verdict));
    match &outcome.verdict { Verdict::Unknown(r) => println!("  reason: {r}"), Verdict::Unprovable(d) => println!("  reason: {d}"), _ => {} }
    let settings = Settings::default();
    let limit = settings.clock.time_limit_ms.map(Duration::from_millis);
    let start = Instant::now();
    let outcome = deduxit::linear::prove_within(&sequent, Mode::CLASSICAL, &settings.search, &settings.limits, |_| {
        limit.is_some_and(|l| start.elapsed() > l)
    })?;
    println!("Settings::default() + clock: {}", name(&outcome.verdict));
    if let Verdict::Unknown(r) = &outcome.verdict { println!("  reason: {r}"); }
    }
    Ok(())
}

fn name(v: &Verdict) -> &'static str {
    match v { Verdict::Proved(_) => "proved", Verdict::Unprovable(_) => "unprovable", Verdict::Unknown(_) => "unknown" }
}

fn t4_ill_latex() -> R {
    println!("== T4: intuitionistic proof, two-sided, as LaTeX");
    let sequent: Sequent = "A, A -o B |- B".parse()?;
    let outcome = prove(&sequent, Mode::INTUITIONISTIC, &Options::default())?;
    let Verdict::Proved(proof) = &outcome.verdict else { panic!() };
    let derivation = proof.derivation()?;
    let mut out = String::new();
    deduxit::export::latex::write(&derivation, &deduxit::export::latex::Options::default(), &mut out, |_| false)?;
    println!("{out}");
    Ok(())
}

fn t5_lambda() -> R {
    println!("== T5: the lambda-term of an intuitionistic proof");
    let sequent: Sequent = "A -o B, B -o C |- A -o C".parse()?;
    let outcome = prove(&sequent, Mode::INTUITIONISTIC, &Options::default())?;
    let Verdict::Proved(proof) = &outcome.verdict else { panic!() };
    let term = proof.derivation()?.term(&Limits::default(), |_| false)?;
    let mut out = String::new();
    term.write_judgement(&mut out, &deduxit::lambda::Options::default(), &Limits::default(), |_| false)?;
    println!("{out}");
    Ok(())
}

fn t6_errors() {
    println!("== T6: errors a user meets");
    match "A, A -o |- B".parse::<Sequent>() {
        Ok(_) => println!("parsed?"),
        Err(e) => println!("parse error Display: {e}"),
    }
    let sequent: Sequent = "A par B |- A".parse().unwrap();
    match prove(&sequent, Mode::INTUITIONISTIC, &Options::default()) {
        Ok(o) => println!("ILL of a par: {}", name(&o.verdict)),
        Err(e) => println!("ILL of a par, Display: {e} (code {})", e.code()),
    }
    match "A |- B par C".parse::<deduxit::linear::IllSequent>() {
        Ok(_) => println!("IllSequent parsed?"),
        Err(e) => println!("IllSequent parse, Display: {e}"),
    }
    let ill: deduxit::linear::IllSequent = "A, A -o B |- B".parse().unwrap();
    let o = prove(ill.sequent(), Mode::INTUITIONISTIC, &Options::default()).unwrap();
    println!("IllSequent then prove(ill.sequent(), INTUITIONISTIC): {}", name(&o.verdict));
}

fn t7_check_cli_json(path: &str) -> R {
    println!("== T7: check a proof file the command wrote");
    let text = std::fs::read_to_string(path)?;
    match serde_json::from_str::<deduxit::linear::Proof>(&text) {
        Ok(proof) => {
            let r = proof.check(Mode::CLASSICAL);
            println!("read as Proof; check(CLASSICAL): {r:?}");
        }
        Err(e) => println!("read as Proof failed: {e}"),
    }
    let value: serde_json::Value = serde_json::from_str(&text)?;
    println!("the file's mode key: {}", value["mode"]);
    match serde_json::from_value::<Mode>(value["mode"].clone()) {
        Ok(mode) => println!("Mode read from the key: {mode:?}"),
        Err(e) => println!("Mode from the key failed: {e}"),
    }
    let refuted = std::fs::read_to_string(path.replace("p.json", "refuted.json"))?;
    match serde_json::from_str::<deduxit::linear::Disproof>(&refuted) {
        Ok(d) => println!("read as Disproof; check: {:?}", d.check()),
        Err(e) => println!("read as Disproof failed: {e}"),
    }
    Ok(())
}

fn t8_ordinary() -> R {
    use deduxit::ordinary;
    println!("== T8: ordinary intuitionistic logic");
    let sequent: ordinary::Sequent = "~~a -> a".parse()?;
    let options = ordinary::Options::default().with_logic(ordinary::Logic::Intuitionistic);
    let outcome = ordinary::decide(&sequent, &options, &Options::default(), &Limits::default(), |_| false)?;
    println!("~~a -> a intuitionistically: {:?}", std::mem::discriminant(&outcome.verdict));
    match &outcome.verdict {
        ordinary::Verdict::Valid(d) => println!("valid\n{d}"),
        other => println!("{other:?}"),
    }
    let sequent: ordinary::Sequent = "a -> b, a |- b".parse()?;
    let outcome = ordinary::decide(&sequent, &ordinary::Options::default(), &Options::default(), &Limits::default(), |_| false)?;
    if let ordinary::Verdict::Valid(d) = &outcome.verdict { println!("valid (default logic)\n{d}"); }
    Ok(())
}

fn main() {
    for (n, r) in [("t1", t1_prove()), ("t2", t2_why_not()), ("t3", t3_default_vs_command()), ("t4", t4_ill_latex()), ("t5", t5_lambda())] {
        if let Err(e) = r { println!("{n} failed: {e}"); }
    }
    t6_errors();
    if let Some(p) = std::env::args().nth(1) { if let Err(e) = t7_check_cli_json(&p) { println!("t7 failed: {e}"); } }
    if let Err(e) = t8_ordinary() { println!("t8 failed: {e}"); }
}
```

## Appendix B: its output

```text
== T1: is it provable? print the proof
provable
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
== T2: why unprovable
Display of the disproof: ~A occurs 1 more time than A in the one-sided sequent, so they cannot all meet in axioms
== T3: library default vs the command, on three sequents
-- !(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D
Options::default(): unprovable
  reason: the classical reading is false where A, B and C are true and every other atom false (! and ? erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), and every provable sequent reads as a tautology
Settings::default() + clock: unknown
  reason: the search was stopped
-- ?A par ?A |- ?(A * A)
Options::default(): unprovable
  reason: the search was exhaustive
Settings::default() + clock: unprovable
-- !(A -o B), !(B -o C), !(C -o D), !(D -o E), !(E -o F), A |- F * !(A -o A)
Options::default(): unknown
  reason: the copy bound of 3 was reached
Settings::default() + clock: proved
== T4: intuitionistic proof, two-sided, as LaTeX
\begin{prooftree}
\infer0[$\mathrm{ax}$]{A &\vdash A}
\infer0[$\mathrm{ax}$]{B &\vdash B}
\infer2[$\multimap\mathrm{L}$]{A, A \multimap B &\vdash B}
\end{prooftree}
== T5: the lambda-term of an intuitionistic proof
f : A ⊸ B, g : B ⊸ C ⊢ λx. g (f x) : A ⊸ C
== T6: errors a user meets
parse error Display: cannot parse the sequent: unexpected "|" at line 1, character 9, expected a formula
ILL of a par, Display: not an intuitionistic sequent: subformula 0 is neither an intuitionistic formula nor the negation of one (⅋ only as A ⊸ B, that is ~A ⅋ B, and ? only under a negation) (code not_intuitionistic)
IllSequent parse, Display: cannot parse the sequent: unexpected "par" at line 1, character 8: intuitionistic linear logic has no par: write ~A par B as A -o B
IllSequent then prove(ill.sequent(), INTUITIONISTIC): proved
== T7: check a proof file the command wrote
read as Proof; check(CLASSICAL): Ok(())
the file's mode key: "classical"
Mode read from the key: Mode { intuitionistic: false, affine: false, mix: false, order: Commutative, empty_antecedents: false }
read as Disproof; check: Ok(())
== T8: ordinary intuitionistic logic
~~a -> a intuitionistically: Discriminant(1)
NotValid
valid (default logic)
───── ax   ───── ax
a ⊢ a      b ⊢ b
──────────────── →L
  a → b, a ⊢ b
```
