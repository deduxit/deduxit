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
