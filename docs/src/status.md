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
  arguments, function symbols and constants, and `s = t` written between
  its terms. Its sequents are read,
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
  inverse method, the basis of the Horn engine's backward search in
  affine mode, a count (an atom whose literals cannot pair up, the
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
- By default intuitionistic and minimal logic are decided by a SAT
  solver, below, with the derivation of LJT, Dyckhoff's contraction-free
  calculus, where LJT gives one. Runs of the ILTP library chose this
  default. Where a bound other than the time limit stops the solver,
  the embedding below decides instead. LJT alone is `--method ljt`; its
  proofs become derivations of LJ with cuts.
- Classical logic also has the methods of a first logic course:
  `--method table` prints the sequent's truth table with its falsifying
  rows marked, and `--method tableau` a signed tableau whose lines name
  their rules. A closed tableau is a checked derivation of LK, and an
  open branch or a falsifying row a checked countermodel. The table is
  bounded at 10 atoms by default.
- A tableau a student writes by hand, as text, is checked step by step
  (`check --tableau`): the first step that breaks a rule is named with
  its line and what the rule gives, a branch marked wrongly or not
  finished is said, and a right tableau without a fault is checked
  again as a derivation in the calculus `tableau`; an open one's branch
  gives a checked countermodel.
- Every "not valid" of the SAT solver comes with a Kripke countermodel,
  and in classical logic the embedding's assignment is a model of one
  world. The model is checked before the verdict, printed a world per
  line, written in the JSON, drawn as SVG, PNG or PDF, and certified for
  deduxit's Rocq library. `check --logic L --sequent SEQUENT FILE`
  checks a Kripke model you give and says at which world the sequent
  fails.
- All three logics can also be decided through their embeddings into
  linear logic: classical logic into affine MALL; intuitionistic logic into ILL
  by Girard's call-by-name and call-by-value translations or the 0/1
  translation; minimal logic the same, with false an atom. `seq print`
  prints the translated sequent, and `--method embedding` decides through
  it.
- The linear proof is read back as a derivation of LK or LJ, which a
  checker of its own validates and every output draws. Its weakenings and
  contractions are drawn, or left out where they are admissible. A valid
  sequent gets a certificate over `Prop` for Rocq, and so does a sequent
  that an assignment of its atoms makes false; a Kripke countermodel of
  intuitionistic or minimal logic gets a certificate for deduxit's Rocq
  library.
- Sequents are proved step by step in LK and LJ, in the command and in
  the web bindings, and a session can apply the invertible rules by
  itself. A goal's `close` and `hint` decide through the embedding; in the bindings a goal that is not valid gets the SAT
  solver's countermodel.
- [First-order sequents](ordinary-logic.md#first-order-sequents) are
  read from text, JSON and TPTP, `fof` and `cnf` (of ILTP's 2 670
  first-order problems, all but four whose conjecture uses a variable
  outside its quantifier), with [equality](ordinary-logic.md#equality)
  decided by its axioms as extra hypotheses and refuted only by a checked
  structure that reads `=` as identity, proved
  step by step in LK, LJ and LM with the quantifier rules
  (a typed witness, a fresh eigenvariable), checked, and drawn in LaTeX,
  Typst and SVG. `prove` decides the Bernays–Schönfinkel and the
  monadic class by grounding over the constants and the SAT solver, in
  every logic: valid in classical logic with a checked clausal proof
  (and the connection prover's derivation of LK where it finds one),
  not valid with a finite structure that deduxit checks against the
  sequent. It decides other classical sequents by a connection prover
  (nanoCoP's non-clausal calculus) and other intuitionistic and minimal
  ones through the embedding: valid with a checked derivation; not
  valid from the connection prover where no quantifier needs a witness
  term (every `∀` one to prove and every `∃` one to assume, negations
  and implications taken into account); otherwise unknown. Under the
  command's time limit the first-order method gets 90% and the
  embedding the rest. A search for finite structures (`--method
  models`, MACE's method through the SAT solver) refutes a first-order
  sequent with function symbols in every logic, with a checked
  structure; in classical logic it runs beside the connection prover,
  and it refutes 210 of ILTP's first-order problems at 2 seconds. In
  intuitionistic and minimal logic it also finds Kripke models with
  growing domains, checked by the forcing relation, beside the
  embedding. Both kinds of model are drawn (SVG, PNG and PDF as boxes,
  LaTeX and Typst as tables) and certified in Rocq. The other methods
  refuse them by name. A
  [comparison](https://github.com/deduxit/deduxit/blob/main/bench/COMPARISON.md)
  runs them beside leanCoP, nanoCoP, nanoCoP-i and iLeanCoP on ILTP's
  first-order problems. `prove --szs` ends its output with the SZS
  status line that TPTP's tools read.
- [Classical sequents are translated](transformations.md) into
  minimal logic by Kolmogorov's and Gödel and Gentzen's negative
  translations, and into intuitionistic logic by Kuroda's and Glivenko's;
  all but Glivenko's translate first-order sequents too. Glivenko's
  refuses them and says why: its theorem fails at first order, on
  `¬¬∀x. (p(x) ∨ ¬p(x))` and the double-negation shift, which no finite
  Kripke model refutes; rocq-deduxit proves that an infinite one does. `seq translate` decides the sequent and its translation
  and checks that the verdicts agree. `interpolate` reads a Craig
  interpolant off a derivation of LK or LJ without cuts by Maehara's
  method, first-order ones included, and checks it by deciding both
  halves and comparing the atoms, or at first order the predicates,
  functions and constants. The library, the web bindings and the web
  page have both, the page for propositional sequents.
- [Classical sequents are rewritten into normal forms](transformations.md#rewrite-a-sequent-into-a-normal-form)
  step by step: negation, conjunctive and disjunctive normal form by
  equivalence laws, and the clauses of Tseitin's and of Plaisted and
  Greenbaum's encodings. `seq normalize` shows each step and checks with
  the SAT solver that each form is equivalent to its input, or for an
  encoding that the clauses can be satisfied exactly when the sequent
  can be false. At first order `seq normalize` makes the prenex form,
  the miniscope form, the Skolem, Herbrand and clausal forms, each
  equivalence step checked by a derivation of its law (of LJ where the
  law holds there) and each Skolem symbol checked new. The library and
  the protocol have them; the web page does not yet.
- [Herbrand's theorem proves first-order sequents](transformations.md#prove-a-first-order-sequent-by-herbrands-theorem):
  `herbrand` puts closed terms into the clausal form's clauses, depth by
  depth, until the SAT solver refutes the instances, checks its clausal
  proof, and builds from the instances it needs a derivation of LK of
  the sequent as written, each Skolem term an eigenvariable again, which
  the checker accepts. The library and the protocol have it.
- [Clauses are refuted by resolution and by DPLL](resolution.md) at
  teaching scale, within bounds of clauses and steps: a sequent's
  negation turned into clauses, or a DIMACS file. Every refutation is
  checked step by step, every model clause by clause, and DPLL prints
  each step. A refutation written by hand, and an outside SAT solver's
  DRAT or LRAT proof, are checked by `check`. At first order the
  clausal form's clauses resolve with the most general unifier and are
  factored, each step with its unifier, and a sequent with `=` is
  resolved by paramodulation, an equation rewriting a subterm of
  another clause, with reflexivity as the one axiom (or with every
  equality axiom, `--equality axioms`); a refutation is checked step by
  step, each rewrite made again, and read back as a checked derivation
  of LK where it reaches (not through a rewrite, which LK has no rule
  for). The library and the protocol have them; the web page does not
  yet.
- In the library, behind the default feature `sat`, `ordinary::sat`
  decides classical, intuitionistic and minimal logic with a SAT
  solver. A sequent that is not valid comes with a model, which is
  checked. A valid sequent comes with a checked derivation of LJ in
  intuitionistic and minimal logic, and in classical logic with the
  solver's clausal proof, which a checker of deduxit's own accepts. It
  is the default of every logic; in classical logic the default keeps
  the embedding's derivation of LK, refutation and certificate wherever
  the embedding gives them within 2²⁰ units of work. The command offers
  it as `--method sat`.

## Proofs as programs

- [Every intuitionistic proof reads as a program](terms.md): a proof of
  intuitionistic linear logic as a term of the linear lambda calculus of
  Benton, Bierman, de Paiva and Hyland (the affine one in affine mode, a
  cut a redex), and a derivation of LJ as a simply typed term.
- Terms are printed in Unicode or ASCII, with or without types, by
  `prove`, `check` and `interact`, in JSON, and under the conclusion in
  LaTeX, Typst, SVG, PNG and PDF. What has no term is refused with the
  reason. In the library `lambda::Term` has `Display`.
- [A term you write is checked](terms.md#check-a-term) (`term check`;
  in the library `lambda::Term::parse` and `lambda::Term::check`): a term
  of the simply typed, the linear or the affine lambda calculus, from
  text or JSON.
- [A simple type's term is found](terms.md#find-a-term-of-a-type) by
  deciding its formula (`term inhabit`), or shown to have none, and
  [its terms are counted](terms.md#count-a-types-terms) up to
  βη-equality (`term enumerate`): exactly, infinitely many, or at least
  as many as a bounded search met, for types without `∨` and `⊥`.
- [A well-typed term's typing derivation is drawn](terms.md#a-typing-derivation-drawn)
  in LaTeX, Typst, SVG, PNG and PDF by the rules of natural deduction
  (`lambda::Drawing`), its contexts split as linear logic splits them in
  the linear calculi.

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
- For a first-order sequent the model search refutes, `--format rocq`
  writes the lemma that LK, or LJ or LM, does not derive it, proved from
  its finite structure or Kripke model with growing domains.
- For a modal sequent, `--format rocq` writes the lemma that it is valid
  over the system's frames, proved from its labelled derivation, or
  that it is not, proved from its countermodel. A derivation built step
  by step in a labelled calculus gets one too.
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
  - the calculi of intuitionistic and minimal logic, their Kripke
    models with the theorem that a derivable sequent holds in every
    model, and a checker of finite countermodels, on which the
    certificates of countermodels rest; at first order, LJ, LM and LK,
    Kripke models with growing domains and finite structures with the
    same theorems and checkers, and the infinite model that refutes
    `¬¬∀x. (p(x) ∨ ¬p(x))`, which no finite model refutes;
  - the modal systems from K to S5 with their Kripke models, the
    labelled sequent calculus with the theorem that it derives only
    valid sequents, and checkers of labelled proofs and of finite
    countermodels, on which the modal certificates rest;
  - bridges, checked against the kernels' own files, showing that its
    classical calculus derives exactly the sequents NanoYalla derives,
    and its intuitionistic calculus exactly those of Yalla's
    `nanoill.v`.

  deduxit's flake compiles the certificates the command writes against
  the library, pinned to a commit.

## The command

- The `deduxit` command has `prove`, `check`, `eliminate`, `interact`,
  `interpolate`, `resolve`, `grade`, `term`, `seq` and `workspace`. It
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

- [`grade`](grading.md) grades an answer to a problem in one call: a
  session script that must finish the proof by its own steps, a proof
  or an outcome in JSON, a countermodel, a tableau or a derivation in a
  calculus given as data, about the problem's sequent in its logic.
  Its exit status is correct, incorrect or unknown.
- [A dataset of problems with verified answers](grading.md#problems-with-verified-answers)
  for evaluating models, in every suite with a source whose licence
  allows it: each problem with the verdict deduxit decided and an answer
  `grade` accepts, and its card. A flake check decides every tenth
  problem again and grades its answer.

## The library and the web

- The library `deduxit` has every engine, checker and output the
  command has ([Use the library](library.md)). `Settings::stop(elapsed)`
  stops a search as the command does.
  `export::{latex, typst, svg, rocq}::to_string(derivation, &options)`
  writes an export into a string. A linear `Outcome` reads back from its
  JSON with `serde_json::from_str`.
- `deduxit::protocol` lists what each suite offers as data: the kinds of
  values, the operations between them with their parameters, the
  settings each reads and what each promises, and runs and checks them
  over JSON ([What each suite offers](protocol.md)). A JSON Schema
  describes every JSON form.
- `deduxit workspace` keeps sequents and what was made from them in a
  file ([Keep a sequent and what is made from it](workspace.md)): a
  changed sequent marks what follows from it out of date until it runs
  again, and undo and redo take back whole edits. The web page links
  tabs the same way.
- The library reads and writes sequents of propositional modal logic
  (`deduxit::modal`): the syntax of ordinary logic with `[]` or `□`
  and `<>` or `◇`, and a JSON form. It names the fifteen normal systems
  from K to S5, each by the conditions its frames meet (serial,
  reflexive, symmetric, transitive, euclidean). It decides a modal
  sequent in any of them: a valid one with a derivation in the system's
  labelled sequent calculus, which the checker of calculi given as data
  accepts, one that is not valid with a Kripke countermodel, checked
  where it is small enough to check. It
  checks a Kripke model you give against a sequent and a system. The
  command does both under `--system` ([Decide modal logic](modal-logic.md)),
  and proves a labelled sequent step by step in any system's calculus,
  backwards and, for the rules that add before `⊢`, forwards
  (`interact --calculus G3S4`). The protocol offers the decision, and
  the web page decides a modal sequent, draws its derivation or
  countermodel, and proves it step by step in the system's calculus.
- `check` checks a derivation in a calculus you give as data
  ([Check a derivation in your own rules](rules.md)): a rule file in
  JSON declares the connectives, the zones of a sequent (each a set, a
  multiset or a sequence) and the rules as premises and a conclusion
  over schematic formulas and contexts, and each step of the
  derivation is matched against its rule, without a search. The
  verdict is "valid in the calculus given": the rules are yours, and
  nothing checks that they are sound. Hypotheses that a rule
  discharges and nested scopes serve natural deduction, sets of signed
  formulas a tableau, and sorts for labels a labelled modal calculus.
  LK, LJ, minimal logic's calculus, one-sided linear logic and signed
  tableaux are built in; derivations of LK and LJ and the tableaux of
  ordinary logic agree with their own checkers. Natural deduction is
  built in too ([Check a proof in natural deduction](natural-deduction.md)):
  NJ, NK and NM with the variants courses use (`¬` defined as `A → ⊥`,
  double negation elimination or excluded middle in place of `RAA`),
  each deriving exactly what the ordinary suite calls valid in its
  logic. `interact --calculus`
  builds such a derivation step by step, each step checked, what the
  goal leaves open asked for by name: backwards from a goal, or
  forwards from numbered lines in force, as a Fitch-style proof is
  written. Such a derivation is drawn as a tree in LaTeX, Typst and
  SVG, natural deduction with each discharged hypothesis in brackets
  and numbered, and as a Fitch proof, as text, as SVG, for the LaTeX
  package `fitch` and for the Typst package `fitchcraft`.
- The unpublished crate `deduxit-web` in `web/` compiles the library to
  WebAssembly for a web page. Its calls take JSON and give JSON or SVG:
  a sequent read, a proof step by step, a goal closed by the search
  within a budget of work, a hint, and every export, for linear logic,
  for LK and LJ and for a calculus given as data; the translations and
  interpolants; a classical sequent's normal forms, and its clauses
  refuted by resolution, by DPLL or step by step, each step checked;
  a lambda-term read and checked with its typing derivation drawn, and a
  type's terms found, listed and counted; modal logic's decision, countermodels and a model
  checked against a sequent; the review of a tableau written by hand;
  the protocol's description of each suite; and the workspace. Its defaults fit a browser tab. PNG and PDF are
  rendered by a second module, `deduxit-web-render` in `web-render/`,
  which a page loads only for its first such export. The flake builds both as
  a package and tests them under Node. The library's checker and memo are
  tested on a 32-bit target there too.
- [deduxit.github.io](https://deduxit.github.io/) is a web page for
  proving step by step in the browser on those bindings, in linear logic
  and in LK and LJ, with each intuitionistic proof's lambda-term, the
  countermodel of a goal that is not valid, the truth table and the
  tableau of classical logic. It proves in natural deduction (NK, NJ
  and NM, as a tree or as Fitch's lines) and in a rule file of your
  own, translates a classical sequent and finds an interpolant, steps
  through its normal forms one law at a time and refutes its clauses by
  resolution, as a list and as a graph, or by DPLL with its trace,
  checks a lambda-term and draws its typing derivation, finds a type's
  terms, and
  links the tabs made from one sequent into a workspace. A switch
  applies the invertible rules by themselves, in linear and in LK and
  LJ tabs alike. Its modal tab
  decides a sequent in any of the fifteen systems, draws its derivation
  or countermodel and proves it step by step in the labelled calculus.
  A model editor changes a countermodel, or builds one, and checks it
  against the sequent, ordinary or modal; a tableau editor checks a
  tableau you write and names its first wrong step. Learn and Practice
  follow the tab's suite: linear logic has a course of eleven lessons,
  ordinary logic one of six and a store of problems. For teachers, a
  course pack is a link to a set of problems, with the search allowed
  or barred; a student hands in a link or a file, which the check view
  reads again step by step. The page sends nothing: the student sends
  the hand-in. A pack or a proof can be embedded in another page and
  printed. It lives in a repository of its own.

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
  0.2.0. The latest, of 2026-11-16, is in `bench/RESULTS.md`.

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

- 0.8.0: first-order ordinary logic: classical, intuitionistic and
  minimal, searched, with finite countermodels and Rocq certificates; a
  witness left open in a session for an axiom to fill;
  all of it on the web page, the normal forms, Herbrand's theorem,
  resolution and equality among it, the linear tab at first order too.

Not yet placed in a release: the comparison with the other provers on
the libraries with the engines of 0.2.0.

## Where it comes from

deduxit owes much to [Click & coLLecT](https://www.click-and-collect.linear-logic.org).
It aims to cover more uses, faster. Feature requests and contributions
are welcome. deduxit is licensed under the EUPL; see
[Licence](https://github.com/deduxit/deduxit/blob/main/README.md#licence).
