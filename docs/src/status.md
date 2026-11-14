# What exists and what is planned

This page lists what deduxit does today, suite by suite, and what is
planned for the next releases. The chapters it links to show each part
in use.

## Linear logic

### Sequents

- Sequents are read and printed in [the syntax](syntax.md), with an
  ASCII and a Unicode spelling of every connective, and have a compact
  JSON form.
- deduxit detects the fragment a sequent lives in: MLL, MLL with units,
  ALL, MALL, MELL, LL, and their intuitionistic counterparts IMLL to
  ILL. The modes are a choice of the user: classical, affine (weakening
  allowed, in every fragment), intuitionistic and Mix.
- An intuitionistic sequent is written two-sided, the hypotheses left of
  `⊢` and one goal right of it. The classical symbols are refused with a
  hint, or read from the classical syntax with `--ill-syntax one-sided`.
  Every output writes the sequent back two-sided: derivations,
  refutations, nets, errors, exports and terms. It has a JSON form of its
  own.
- First-order linear logic has quantifiers over terms, predicates with
  arguments, function symbols and constants. Its sequents are read,
  printed one- and two-sided and written as JSON. A sequent with
  arguments but no quantifier is decided as a propositional one.

### Proofs and the checker

- A proof is a compact term over the subformula occurrences of its
  sequent. An independent checker decides whether a term proves its
  sequent, in one pass and in memory proportional to the proof. In
  intuitionistic mode it also checks that every sequent of the proof has
  one goal. Every proof the search finds passes the checker, in every
  build.
- A derivation view unfolds a term into the tree of the standard sequent
  calculus, one-sided or two-sided with the rules of ILL, printed as
  text. On request a classical derivation is drawn two-sided, as its
  sequent was written: the rules are named by side, and a negation step
  stands where a negation was written.
- The size of a derivation is computed from the term without building
  it. Every path that builds a derivation (the text tree, the exports,
  the search inside an interactive proof) keeps within a bound on that
  size, which is an option.
- First-order proofs have `∀` and `∃` steps with eigenvariables and
  witnesses. The checker checks the eigenvariable condition. Their
  derivations are printed one- and two-sided and drawn in LaTeX, Typst
  and SVG with every instance written out.
- `deduxit check` reads a proof, or an unprovable outcome, and checks it
  ([Check a proof or a refutation](check.md)).

### Proof search

- The search decides every fragment, MLL to LL and IMLL to ILL, with or
  without Mix. It answers with a checked proof, with "unprovable" and the
  reason, or with "unknown" and the bound or limit that ended it.
  [How deduxit searches](engines.md) explains the engines.
- The engines are:
  - a focused sequent engine for every fragment, one- and two-sided,
    with a bound on the copies of `?` formulas that deepens until the
    search decides or the time limit passes;
  - a proof-net engine for MLL without units, and for IMLL without units
    over essential nets, used where the focused engine stops at a limit
    or when asked;
  - a recursion on pairs of subformulas, for two formulas built from
    additives alone;
  - the focused inverse method, a forward search from the axioms, which
    decides without exponentials, proves with them without a copy bound,
    and refutes where its search ends;
  - a Horn engine for Horn programs with clauses under `!`, read as
    Petri nets: it searches the reachable markings in linear mode,
    decides coverability in affine mode, and refutes by the net's state
    equation.
- Which engine decides a goal is a table by fragment, mode and one more
  feature. Each row is the engine measured fastest there, or the only one
  that decides there. The library's documentation of `Engine` has the
  table with the measurements.
- "Unprovable" comes from an exhaustive search, a saturated set of the
  inverse method, a count (an atom whose literals cannot pair up, the
  count equation that fails), a Petri net's state equation, or an
  assignment that makes the sequent false read classically. Each
  refutation is checked apart from the search. A classical refutation of
  a sequent written with two sides speaks of the sides, and names `-a`
  where an unused hypothesis is the problem.
- First-order sequents are decided by unification with the occurs check:
  by the focused engine in the classical modes, Mix and affine included,
  and by the two-sided engine in the intuitionistic modes. MLL1 and MALL1
  are decided, and with exponentials within the copy bound.
  `--witness-depth` and `--instances` bound the search.
- The search runs on threads behind the library's feature `parallel`,
  which the command has on. The focused engine and the net engine share
  their first choices out among a pool of threads, and one stop reaches
  every thread. By default one thread searches alone for a tenth of a
  second, and then the other cores join it.
- The limits hold. The time limit is checked wherever a search can
  spend time, and the command's `--timeout` counts from its start,
  reading included, to within a fraction of a second. A search holds at
  most `--memory-limit` bytes, and a sequent unfolds to at most
  `--occurrence-limit` occurrences. No walk over a formula, a net or a
  derivation recurses on the depth of the input.

### Proof nets

- Proof nets exist for MLL with or without Mix, for MELL without units
  with or without Mix, and for intuitionistic MELL without units. Each
  has an independent correctness criterion (Danos–Regnier, decided by
  Yeo's deletion test), which names the switching cycle or the
  disconnection when it fails. A net turns into a checked proof, and a
  proof into its net. Nets have a text form, a JSON form and a drawing,
  every `!`-box a rectangle.
- In MELL the net of a proof is fixed only up to the axiom link each
  weakening hangs on: the one the proof's weakening rule has above it.
  So two proofs that differ only in the order of their rules can give
  different nets. The same happens where a sequent holds two copies of
  one `?` formula, since a rule does not say which copy it acts on.

### Cyclic linear logic and the Lambek calculus

- [Cyclic linear logic](cyclic.md), multiplicative and without units,
  keeps the order of a sequent's formulas up to rotation. It is decided
  by a search for planar proof nets, and checked, proved step by step
  and drawn with no exchange rule.
- [The Lambek calculus](lambek.md), with or without empty antecedents,
  is read two-sided in the order written, with its two divisions. It is
  decided by a search for planar essential nets, and checked, proved step
  by step and drawn two-sided with the rules of the divisions.

### Step by step and cuts

- [Proving step by step](interactive.md) holds a derivation with open
  goals. A rule applied to a formula of a goal is checked first: the
  connective, the mode, the context a promotion or an axiom needs, and
  one goal per premise in intuitionistic mode. A session can undo a
  step, close any goal by the search, and apply every invertible rule on
  request. The finished derivation becomes a term that the checker
  validates. A session has a JSON form. An unknown rule lists the
  session's rules with their ASCII spellings, and a split whose premise
  has no proof gets a warning.
- Proofs may have cuts in every commutative mode: the checker has the
  cut rule, a session lets the user name a cut formula, every view and
  export draws the cut, and a certificate for deduxit's Rocq library
  states the calculus with cut. The search stays cut-free.
- [Cut elimination](cut.md) on proof terms works in every mode with the
  cut (classical, affine, with Mix, intuitionistic) and at first order:
  one step at a time on the cut the user names, or to the end within a
  bound on steps and on the proof's size. Every step is named by its case
  and checked.
- Cut elimination on proof nets works with and without boxes (classical,
  with Mix, intuitionistic): one step at a time, each named by its case
  and checked by the criterion, and each drawn with the cut link and
  what the step acts on highlighted.

## Ordinary logic

- [Ordinary logic](ordinary-logic.md) is classical, intuitionistic and
  minimal propositional logic, in a syntax of its own or read from TPTP
  problems.
- By default intuitionistic and minimal logic are decided by LJT,
  Dyckhoff's contraction-free calculus, whose search always ends. A run
  of the ILTP library chose this default. Where a bound other than the
  time limit stops LJT, the embedding below decides instead. LJT's proofs
  become derivations of LJ with cuts.
- All three logics are decided through their embeddings into linear
  logic: classical logic into affine MALL; intuitionistic logic into ILL
  by Girard's call-by-name and call-by-value translations or the 0/1
  translation; minimal logic the same, with false an atom. `seq print`
  prints the translated sequent, and `--method embedding` decides through
  it.
- The linear proof is read back as a derivation of LK or LJ, which a
  checker of its own validates and every output draws. Its weakenings and
  contractions are drawn, or left out where they are admissible. A valid
  sequent gets a certificate over `Prop` for Rocq, and so does a sequent
  that an assignment of its atoms makes false.
- Sequents are proved step by step in LK and LJ, in the command and in
  the web bindings.
- In the library, behind the default feature `sat`, `ordinary::sat`
  decides intuitionistic and minimal logic with a SAT solver. A valid
  sequent comes with a derivation of LJ, and one that is not valid with
  a Kripke model, which is checked. The command does not offer this
  engine yet.

## Proofs as programs

- [Every intuitionistic proof reads as a program](terms.md): a proof of
  intuitionistic linear logic as a term of the linear lambda calculus of
  Benton, Bierman, de Paiva and Hyland (the affine one in affine mode, a
  cut a redex), and a derivation of LJ as a simply typed term.
- Terms are printed in Unicode or ASCII, with or without types, by
  `prove`, `check` and `interact`, in JSON, and under the conclusion in
  LaTeX, Typst, SVG, PNG and PDF. What has no term is refused with the
  reason. In the library `lambda::Term` has `Display`.

## Outputs and certificates

- [Exports](output-formats.md) write sequents and derivations, finished
  or in progress, as LaTeX (ebproof trees) and Typst (curryst trees, and
  a layout of deduxit's own for trees of any height), as fragments or
  standalone documents that choose no font.
- SVG draws sequents, derivations and proof nets. A net can have every
  subformula written or be compact, and be drawn in deduxit's look or the
  textbooks'. The text is laid out with the widths of Euler Math, STIX
  Two Math, Garamond-Math, Asana Math or New Computer Modern Math, or of
  a font the caller gives. A switching cycle or the parts of a
  disconnected net are highlighted. Intuitionistic nets can be drawn as
  essential nets. A drawing can carry a text for screen readers, and
  renders as PNG, as archival PDF (PDF/A-4, PDF/A-2u) or as accessible
  PDF (PDF/UA-1).
- Every output format has one options value, with serde: rule labels,
  the shape of an open goal, alignment, preambles, the SVG's font, sizes
  and colours, and the certificate's names. The command sets them with
  `--style` and `--style-file`, and writes a derivation as it is made.
- [Rocq certificates](rocq.md): a finished proof becomes a Rocq script.
  The NanoYalla kernel takes classical proofs, and intuitionistic ones as
  the classical proofs they are. deduxit's own Rocq library takes every
  mode (`--kernel deduxit`, the default for Mix, affine, intuitionistic
  and first-order proofs). Flake checks compile both kinds.
  `nix run github:deduxit/deduxit#check-certificate -- proof.v` checks a
  certificate of either kernel with nothing but Nix installed.
- For an unprovable sequent whose refutation is a classical
  countermodel, an atom's count, the count equation, a weighted count or
  a Petri net's state equation, `--format rocq` writes the lemma that the
  calculus does not derive it, checked by computation.
- deduxit's Rocq library is the package `rocq-deduxit`, in its own
  repository,
  [deduxit/rocq-deduxit](https://github.com/deduxit/rocq-deduxit), on
  Rocq 9 and its standard library alone. It has:
  - the one-sided calculus of classical linear logic, with Mix and
    weakening as parameters, and the two-sided calculus of
    intuitionistic linear logic and its affine variant, each with the cut
    as an optional rule;
  - deduxit's proof terms as datatypes, its checkers as functions, and
    for every mode the theorem that a term the checker accepts proves its
    sequent, closed under the global context;
  - the same at first order, with the quantifier rules and the
    eigenvariable condition;
  - the invariants every derivation keeps (it is a tautology read
    classically, an atom's literals balance, the count equation, a
    weighted count), on which the certificates of refutations rest;
  - the proof that a derivation with Mix gives one of `?(⊥⊗⊥), Γ` without
    Mix and without cut;
  - bridges, checked against the kernels' own files, showing that its
    classical calculus derives exactly the sequents NanoYalla derives,
    and its intuitionistic calculus exactly those of Yalla's
    `nanoill.v`.

  deduxit's flake compiles the certificates the command writes against
  the library, pinned to a commit.

## The command

- The `deduxit` command has `prove`, `check`, `eliminate`, `interact` and `seq`. It
  has time limits, Ctrl-C, statistics, JSON output, proof nets, LaTeX,
  Typst, SVG and Rocq output, and `--jobs`, `--pool-after` and
  `--deterministic` for the search. [Decide a sequent](commands.md)
  shows them.
- Every setting a flag sets can also come from one file, given with
  `--settings` ([The settings file](settings.md)). A front end keeps its
  settings in that form.
- `check` runs under a time limit and under limits of work and memory,
  and answers "unknown" where one of them ends it.
- The defaults suit a newcomer, and each is an option. The copy bound
  deepens while a time limit of two seconds lasts. One thread searches
  for a tenth of a second before the other cores join it. "Unknown" says
  which bound or limit ended the search, after how long, at which copy
  bound, and with the flag that changes it. "Unprovable" says why, where
  the counts of the sequent tell or an assignment makes it false.
- `-h` shows the common flags under headings, and `--help` every one.
  The first errors say what to write instead: `deduxit "A |- A"` suggests
  `deduxit prove "A |- A"`, an unquoted sequent suggests quotes, `->` in
  linear logic suggests `-o` or `--logic classical`, and a file of
  `NAME: SEQUENT` lines suggests `--input-format lines`. `--format tex`
  is read as `latex`.
- A proof tree is printed on a terminal where it fits. Where the whole
  tree would not fit, or would pass `--derivation-limit`, a run of one
  structural rule is drawn as one inference. A derivation past the limit
  either way is left out, with a line that says so.
- [Many sequents at once](batch.md): files, directories, lists of
  paths and standard input, in the text syntax, as JSON Lines, in the
  harness's problem files and in the LLTP library's format. There is one
  answer per sequent, in input order, a time limit per sequent and for
  the batch, a memory bound for the batch, and on request a child
  process per sequent.

## The library and the web

- The library `deduxit` has every engine, checker and output the
  command has ([Use the library](library.md)). `Settings::stop(elapsed)`
  stops a search as the command does.
  `export::{latex, typst, svg, rocq}::to_string(derivation, &options)`
  writes an export into a string. A linear `Outcome` reads back from its
  JSON with `serde_json::from_str`.
- The unpublished crate `deduxit-web` in `web/` compiles the library to
  WebAssembly for a web page. Its calls take JSON and give JSON or SVG:
  a sequent read, a proof step by step, a goal closed by the search
  within a budget of work, a hint, and every export, for linear logic and
  for LK and LJ. Its defaults fit a browser tab. The flake builds it as
  a package and tests it under Node. The library's checker and memo are
  tested on a 32-bit target there too.
- [deduxit.github.io](https://deduxit.github.io/) is a web page for
  proving step by step in the browser on those bindings, in linear logic
  and in LK and LJ, with each intuitionistic proof's lambda-term and
  lessons. It lives in a repository of its own.

## Benchmarks

- Readers for the problems of the LLTP library and for coverability
  problems in the `.spec` format of the Mist tool, and generated families
  with known verdicts: the hard families of the literature and the cases
  where one engine is known to be slow.
- `deduxit-bench` runs them with a time limit per run, writes CSV and
  summarises it. Scripts take a whole baseline on an idle machine, run
  the focused engine's target set, and compare two baselines problem by
  problem. Growing sets of families, each chosen for a claim about an
  engine or another prover, are drawn as time against size with the
  growth fitted. [Benchmarks and the comparison](benchmarks.md) shows
  them.
- A comparison with the other provers of linear logic: each built from
  its own source at a pinned version, each problem translated into its
  syntax, every run measured by BenchExec under one time and memory
  limit. It covers the libraries with the engines of 0.1.0, and the
  growing families to each tool's limit with those of 0.2.0. Anyone can
  start it as a CI workflow.
- Four baselines measured the focused engine's performance work: one
  before it, one after it, and one at each of the releases 0.1.0 and
  0.2.0.

## Releases

- The crates `deduxit` and `deduxit-cli` are on crates.io. Up to 0.2.0
  they were `linlog` and `linlog-cli`, whose last release, 0.3.0, points
  here. A changelog states what each version promises and what it
  breaks.
- A workflow publishes a version tag with the command built for Linux
  (an archive, a `.deb`, an `.rpm` and an Arch Linux package), macOS
  and Windows, their checksums and a provenance attestation.

## Planned

The collection grows by a suite at a time, release by release:

- 0.4.0: intuitionistic logic decided by its own engine on the page and
  in a session's `close`, with Kripke countermodels for what is not
  provable, checked, drawn and certified in Rocq; classical tableaux and
  truth tables; a checked certificate for the affine refutations of the
  Horn engine.
- 0.5.0: custom rules, a calculus given as data, with derivations in it
  checked and built in the command and on the page; natural deduction
  for ordinary logic in Gentzen's and Fitch's styles.
- 0.6.0: the modal logics K, T, K4, S4 and S5 decided, every refutation
  a checked Kripke countermodel, and a labelled calculus proved sound in
  Rocq; tools for teachers.
- 0.7.0: propositional resolution at teaching scale, and the simply
  typed lambda calculus, its typing derivations checked and built.

Not yet placed in a release: first-order proving on the web page, and
the comparison with the other provers on the libraries with the engines
of 0.2.0.

## Where it comes from

deduxit owes much to [Click & coLLecT](https://www.click-and-collect.linear-logic.org).
It aims to cover more uses, faster. Feature requests and contributions
are welcome. deduxit is licensed under the EUPL; see
[Licence](https://github.com/deduxit/deduxit/blob/main/README.md#licence).
