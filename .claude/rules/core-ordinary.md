---
paths:
  - "core/src/ordinary/**"
---

# deduxit core: ordinary logic through its embeddings

Loaded, beside `core.md`, when a file of `core/src/ordinary/` is read or changed:
classical, intuitionistic and minimal propositional logic decided by
translating into linear logic, and the linear proof read back as LK or LJ.

- **Layout.** `mod.rs`: `Logic`, `Translation` (`Affine`, `CallByName`,
  `CallByValue`, `ZeroOne`; short names `affine`, `cbn`, `cbv`, `01`),
  `Options`, the hash-consed arena `Formulas` (`Node`, `NodeId`; equal
  formulas are one id, which the checker's multiset comparisons rely on),
  `Sequent` (two-sided, its syntax documented on it) and the printers;
  `parse.rs` (behind `parse`): the precedence parser in two dialects,
  this crate's (`->`, `/\`, `\/`, `~`, `<->`, `true`, `false` and their
  Unicode symbols) and TPTP's `fof` (`read_tptp`, through
  `lltp::clauses`, which `lltp::read` shares); `translate.rs`: the
  pattern table, `translate`, `Image`, `Target` (`Translation::target`:
  MALL in classical affine mode or LL in intuitionistic mode, whose
  `Display` the command prints, `affine MALL`, `ILL`); `derivation.rs`:
  `Rule`, `Inference` (private fields, accessors), `Derivation`, the
  read-back (`Image::read_back`) and the checker (`Derivation::check`);
  `decide.rs`: `decide`, translation, search, read-back and check in one
  call, answering an `Outcome` (`Verdict::Valid` with its `Evidence`,
  `NotValid`, `Unknown`, and the linear outcome); `rocq.rs` (behind
  `rocq`): the certificate over `Prop`, public as
  `export::rocq::ordinary`, and a countermodel's
  (`export::rocq::write_countermodel`, core-export.md); `model.rs`: the check of a Kripke model
  (`Model::forcing`, `refutes`, `Forcing`, `Countermodel`,
  `MODEL_CELLS`); `negative.rs` and `interpolant.rs`: the
  transformations, below; `normal.rs`: the normal forms, below; `interactive.rs` (behind `interactive`): the
  goal state `Interactive`, below. Its JSON forms and those of the
  sequent, the derivation and the outcome are in
  `core/src/serialize/ordinary.rs`: the sequent, the derivation and the
  goal state at wire level 8 (`wire::ORDINARY`, 9 with LJ's cut), the
  outcome at 12 (`wire::SEARCH`), the Kripke model at 13.
- **Every entry keeps to the occurrence bound**: `Sequent::parse_within`
  and `read_tptp` refuse at the first formula of the arena past
  `limits.occurrences`, and at the formula of the sequent past it,
  since the arena interns a repeated formula and each repeat is an
  occurrence of the image all the same (`FromStr` within the
  defaults), and at the operator past it of those waiting for their
  operand (`Parser::wait`: they lie on one branch, so each will be a
  formula of its own; a run of `~` or of `->` piled up 100 bytes per
  byte of text before), and at the open parenthesis past it, which
  makes no formula but would pile up as well, and `translate`
  refuses an image that unfolds past it, before a forest is laid out
  (`names_read_back_and_bounds_hold`). The command reads, parses and
  translates within its `--occurrence-limit`. `Logic` and
  `Translation` have `ALL`, `NAMES` and `FromStr` (`Error::UnknownName`).
- **The arena refuses what is not its own**: `Formulas::add` refuses an
  operand that is no node of the arena (an id of another one) and an
  atom outside its names, and `Sequent::new` an id outside its arena
  (`Error::IndexOutOfBounds`, `Space::Formula` or `Space::Atom`), so
  that no later index panics; a full arena is `Refusal::Index`.
- **The ordinary syntax refuses the linear symbols** (`&`, `|`, `*`,
  `-o`): later.md's requirement that nobody writes `&` and gets the
  wrong connective. TPTP's `&` and `|` are read only by `read_tptp`.
- **One table is both the translation and the read-back's map**
  (`translate::pattern`): per node and translation function (classical:
  the side, `RIGHT`/`LEFT`; 0/1: `T0`/`T1`; the others one), the `!`s on
  top, the linear connective, and each operand with its function and
  added `!`s. `translate` builds every image bottom-up from it (both the
  image and its dual, so a hypothesis of ILL is the dual), and the
  read-back walks the forest of the image in preorder with the same table
  to tag every occurrence with the ordinary formula, function, `!`s left
  and side. A change to a translation is a change to the table alone;
  never build an image any other way.
- **The 0/1 translation is LLTP's** (`tptpparser/translation.ml`: `t₀`
  for hypotheses under `!`, `t₁` for the goal, `¬A` as `A → false`,
  `A ↔ B` as `(A → B) ∧ (B → A)`), so that this crate's images equal
  LLTP's translated files wherever LLTP's parser read the problem right
  (its `~` takes the widest scope, `plan/notes/lltp-headers.md`), and
  `cbn` writes true as `⊤` where LLTP wrote the atom `T`.
- **Extra nodes**: `translate` adds to its copy of the arena the
  implications `A → B` and `B → A` of every `A ↔ B` and the node false,
  which `¬A` is `A → false` of in ILL; `false` is made first in the build
  order, since a negation anywhere refers to it. The derivation's arena is
  that copy, so its rules name those nodes (`↔R`'s premises, the second
  premise of LJ's `¬L`).
- **The roots are matched by position.** The linear `Sequent` keeps its
  roots in the order `translate` writes them (the hypotheses' images,
  then the formulas right of `⊢`), so `Image::roots[i]` is the linear
  root `i`; `antecedents` is the number of hypotheses in ILL, whose images
  are duals, and `0` classically, where every image stands right of `⊢`.
- **Minimal logic**: false is the atom `FALSE` (`false`, which no
  ordinary atom can be named: `Formulas::atom` refuses the keywords of
  both syntaxes, so a TPTP problem naming `false` is `Error::AtomName`,
  `false_is_no_atom_of_the_sequent`), translated as any atom is, its
  `!` under cbv included (without it a hypothesis `false` was linear,
  and `false |- ~true` was refuted under cbv alone; the fresh-context
  review's differential against G4ip with `⊥` an atom found it), so no
  `0` exists and `⊥L` cannot arise; the checker
  refuses `⊥L` in minimal logic anyway. An intuitionistic or minimal
  sequent with nothing right of `⊢` is decided as `Γ ⊢ ⊥`, and `Image::ordinary`
  is that sequent; more than one formula right is `Error::Succedents`.
- **The read-back takes the proof, not a derivation** (`read_back(&proof,
  &limits, stop)`), and a proof of the roots only (a goal proof is
  `Error::GoalProof`, as for `from_proof` and the Rocq writer): it
  unfolds the proof itself, never compact, within
  the limits, so a compact derivation (a run of weakenings as one
  starred inference, which the probe found read as one `WL` that fails
  the checker, H22) cannot reach it; it and the checker ask the stop
  once per inference, and the checker checks `limits.work` inferences
  at most. It reads the linear derivation that
  `Image::linear_derivation` (crate-private) builds (two-sided for ILL, which keeps the
  goal on the premise without absorbed hypotheses at a `⊗`; one-sided
  for affine MALL; never compact) inference by inference in its order,
  mapping each to zero or more ordinary inferences: `!`/`?d` none (the
  ordinary sequents coincide), `?c` `CL`, `?w`/`wk` `WL`/`WR`, a
  connective rule the rule of the ordinary connective under its tag
  (`rule_of`, by node, side and the classical rule name), and below it
  the `¬` rules a classical negation needs, since it has no image of its
  own (`unwind`). LJ's `→L` is Gentzen's, context split; `∧R` and `∨L`
  share it under cbn and 01, split it under cbv (`⊗`).
- **The checker trusts nothing of the read-back**: per inference, the
  rule against the principal formula (a position given), the premises as
  multisets either sharing the context or splitting it (G1c/G1i/G1m of
  Troelstra and Schwichtenberg with additive variants, all derivable),
  at most one formula right in LJ, exactly one in minimal logic (G1m: an
  empty right side is `⊥` to LK's `¬L` and anything to `WR` or a split
  `∨L`, which together are ex falso; a review forged `a, ¬a ⊢ b` that way,
  `minimal_logic_refuses_an_empty_right_side` pins it), no `⊥L` in minimal logic, the root
  concluding the image's ordinary sequent. `the_checker_refuses_each_break`
  breaks a read-back derivation once per guard and asserts that guard's
  reason, so a guard removed shows (F11: `check_one` replaced by
  `Ok(())` survived the suite). `¬L` and `¬R` take LK's
  forms (one premise; `Γ, A ⊢` for `¬R`) in every logic and that of
  `A → ⊥` in LJ only: the classical certificate reads LK's forms alone,
  and a classical derivation document in the other form made the Rocq
  writer panic before the checker refused it (the goal state's review). It has
  no counter that can wrap: it compares sorted vectors.
- **LJ has Gentzen's cut** (`Rule::Cut`), for the derivations a native
  engine translates: the cut formula is the one formula right of `⊢` in
  the first premise, the second premise holds it left, and the context
  is split between them (`binary` with the whole conclusion as context,
  since a cut has no principal formula; its shared form would put the
  conclusion's goal beside `C` in the first premise, which LJ's count
  refuses, so it is taken only for a conclusion with nothing right of
  `⊢`). LK
  refuses it ("LK's derivations have no cut"): the embedding's read-back
  makes none and the classical certificate has no term for one. The
  certificate writes it `(let h : C := (first) in second)`, the
  lambda read-back substitutes the first premise's term for the formula
  (`Task::Applied` without a function), which may leave a redex. No step
  of the goal state takes it, and `Rule::from_str` refuses `cut` as it
  refuses `open` (`NAMES` is every rule but the last two). A derivation,
  outcome or session that holds one is written at wire level 9
  (`wire::ORDINARY_CUT`, `serialize::ordinary::level`), since a reader
  of level 8 refuses the rule's name.
- **The certificate** (`rocq.rs`) is a term, not a tactic script: an LJ
  sequent `Γ ⊢ C` is a term of type `C` (`False` for none), an LK sequent
  a term of `False` with a continuation `k : ~ D` per formula `D` right,
  and the rules that move a formula right use `NNPP` (standalone files
  import `Stdlib`'s `Classical_Prop`). Variables are `h'n`/`k'n`, which no
  atom identifier contains; atoms go through `export::rocq::identifiers`
  plus the names the terms use (`USED`). Every `match` has a `return`,
  and `↔` goes through a cast to the conjunction it unfolds to; a
  statement without atoms or hypotheses binds nothing (`exact (I)`, no
  `fun =>`, H29). A
  negation is bracketed like a binary formula, since it is an
  application. LK's one-premise `¬L`, which the checker takes in LJ
  too, is `h (premise)` in LJ (the premise a term of `A`) and a
  continuation `k : ~ A` in LK; until a test read such a derivation from
  JSON the LJ arm wrote LK's term, which Rocq refused
  (`ordinary_certificate_of_the_unary_negation_rules`, whose snapshots
  `ordinary_negation_*.v` the `rocq` check compiles). Minimal logic's `⊥` is Rocq's `False`, so a certificate
  of minimal logic proves the intuitionistic statement only (minimality
  rests on the checker).
- **Deciding** is `decide(&sequent, &ordinary::Options)` within the
  default limits and search options, or `decide_within(&sequent,
  &ordinary::Options, &search::Options, &limits, stop)`, which the
  ordinary journey runs (the author's Q5: the linear suite's
  `prove`/`prove_within` shape), by
  `Options::method()`: the method given, else the embedding when a
  translation is given, else `Method::default_for(logic)`: the SAT
  solver for every logic (without the feature `sat` LJT, and the
  embedding for classical logic), chosen by the ILTP runs of
  `plan/reports/54s-sat-engines.md` (`bench/defaults/sat-iltp.csv`,
  `sat-default.csv`, `sat-classical.csv`; before them LJT,
  `ljt-iltp.csv`). **The classical default keeps the embedding's
  evidence** (`with_evidence`, decided while the author was away,
  2026-11-14): once the solver decided, the embedding searches within
  `EVIDENCE_WORK` (2²⁰, or the caller's lower `limits.work`), and its
  outcome, `Search::Embedding` with the derivation read back and checked
  under the caller's limits or `NotValid`, replaces the solver's; where
  it gives no verdict, a bound refuses the derivation or the stop fires,
  the solver's stands, so the stop never costs a verdict the solver
  gave. A verdict of the embedding that contradicts the solver's is
  `Error::ReadBack`, a defect. The bound covers the 124 ILTP problems
  whose evidence the old default gave at 2 s (123 derivations and one
  refutation, 177 136 units at most); the attempt costs a median of
  16 ms where it gives nothing. **It translates within as many
  occurrences** (`EVIDENCE_WORK`, or the caller's lower bound): the
  translation and the search's set-up visit every occurrence before the
  search counts a unit, and the set-up's polls report none, so on
  ILTP's largest images (SYJ206 and SYJ212 at size 20, 12.6 million
  occurrences) the attempt took 0.8 s and 775 MB for no evidence; the
  124 images with evidence have at most 175 occurrences. Bounding the
  image, not making the set-up's polls report work, keeps every
  search's counters and the inverse method's turns as they were
  (`a_large_image_gives_no_evidence_and_costs_no_search`). The command
  does the same with its own search (`cli.md`). The intuitionistic default's SAT solver is
  `sat::decide_preferring_ljt`: the search's verdict, LJT's derivation
  where LJT gives one (smaller, its terms with fewer redexes), else the
  one read off; LJT never runs past the derivation it was asked for, and
  where it is stopped the derivation read off stands, so no verdict is
  lost (on ILTP LJT gave one or was refused at the derivation bound
  within 0.3 s on every valid problem). With no method given, the
  method gives way to the embedding where `gives_way` (on any method's
  `Outcome`) says so: an unknown at a bound
  other than `Reason::Stopped`, or an `Error::Refused` other than the
  stop (the derivation past `derivation_bytes` included: it is a
  refusal, `Refusal::Output`). The outcome's
  `search` (`Search::Embedding` with the translation and the image's
  outcome, `Search::Ljt` with LJT's statistics, `Search::Sat` with the
  solver's, LJT's among them where it gave the derivation) says which
  decided, and its JSON tags `search` by `method` (`ljt` and `sat` with
  `statistics`, no `translation`). The
  command still runs the steps itself (its race and its output, the
  dispatch by `gives_way` in `cli/src/ordinary.rs`), with its defaults
  (no copy bound, the time limit), affine classical mode for `Affine`,
  intuitionistic mode otherwise. A proof whose read-back a bound or the
  stop refuses is that refusal from `decide`, never a verdict. The
  default translation for intuitionistic and minimal logic is
  `Translation::DEFAULT_INTUITIONISTIC`, chosen by the ILTP run
  (`plan/reports/25-ordinary-logic.md`). The goal state's `close` and
  `hint` pass `Method::Embedding`: its steps take no cut.
- **First-order formulas** (`plan/notes/first-order-design.md` is the
  design): `Node::Forall(body)` and `Node::Exists(body)`, last, over de
  Bruijn indices, so formulas equal up to bound names are one node and
  the checker's comparison by id holds through binders. What the code
  relies on:
  - **The tables are the linear suite's** (`crate::first_order`,
    `FirstOrder<NodeId>` and `Interner<NodeId>`), held as
    `Formulas::fo: Option<Box<Interner>>`, made at the first binder,
    predicate with arguments or term: a propositional arena pays a
    pointer (the whole interner inline made `lambda::Inhabitation`'s
    variants lopsided and every arena larger). `first_order()` lends
    `NO_TABLES` where there are none.
  - **First-order exactly when the tables are not empty**
    (`is_first_order`): every binder gets its name entry when `add`
    makes it (`x` until `binder` names it; the first name written wins
    for a shared node), and from the first binder or predicate on every
    atom has its predicate entry (`enter_first_order` gives the earlier
    atoms nullary ones, `atom` the later). An atom's name is its
    canonical text (`p(f(a), #0)`), so atoms stay keyed by names.
  - **Closed formulas only**: `Sequent::new` refuses a root with a loose
    variable (`Error::LooseVariable`, the cold `closed` pass), and the
    derivation reader runs `closed` over every inference's formulas,
    which `Derivation::from_parts` does not (the panel's finding: a
    derivation of `p(#0) ⊢ p(#0)` read and checked). `instantiate` and
    `instance` refuse a term with a bound variable (`is_closed`), which
    the instance would capture: the checker's witness is any term id a
    document names.
  - **Every propositional method refuses at its door**
    (`Formulas::propositional(method)`, `Error::FirstOrder`): `decide`'s
    `within`, `ljt::run`, `sat::run`, `truth_table`, `Tableau::of`,
    `normal`'s `rewrite_dyn`, `Clauses::of_sequent`, `interpolate` and
    `interpolant`, `negative::translate`, `translate` (the embedding),
    `Model::forcing_with`, the goal state's `close` and `hint` (through
    `decide`; the goal state itself takes first-order sequents),
    `WrittenTableau::review_dyn` (its own and the expected sequent),
    the Rocq certificate (`rocq::write`), `Derivation::term`,
    `lambda::enumerate` (whose search reads the types itself; `has_type`
    answers `false`), `NormalForm::claims` (its fields are public),
    `rules::Derivation::from_ordinary` and `from_tableau`, and the
    tableau's reader. Every inner arm on a binder is
    `unreachable!("{}", REFUSED)` (`panic!` in a `const fn`) resting on
    those doors; a new method gets its door and its line in
    `every_propositional_method_refuses_a_first_order_sequent`. A
    derivation of a first-order sequent exists and the checker takes it.
  - **Instances are nodes** (`instance.rs`): `instantiate(body, t)`
    builds `A[t/x]` by an iterative walk memoised per node and depth (a
    node shared at two depths is substituted at each, `p(#0)` under an
    inner binder is not `p(x)`), the closed `t` needing no shift; a
    binder it builds keeps the copied one's name. `instance` is the
    find-only form for the checker, which builds nothing (the interner's
    `find_term`, `find_list`); `instantiate`'s first caller outside the
    tests is the goal state.
  - **The checker's quantifier rules** (G3c, G3i, G3m of Troelstra and
    Schwichtenberg, §3.1; `Rule::{ForallLeft, ForallRight, ExistsLeft,
    ExistsRight}`, last in `Rule::ALL` after `Cut` and `Open`, so `NAMES`
    and `TAKEN` skip those two by name): every quantifier inference
    records its term (`Inference::term`, `"term": k` in the JSON), and a
    term on any other rule is refused, so a document has one reading.
    `∀L` and `∃R` take any closed term of the arena, the premise keeping
    the principal (G3) or not (G1; `∃R` keeps it in LK only); `∀R` and
    `∃L` take an `Eigen` term that occurs in no formula of the conclusion
    (`Formulas::mentions`, a walk over nodes and terms): the condition is
    local because every inference holds its whole sequent. The instance is
    found by `instance` (the checker builds nothing; a miss is a premise
    that does not hold it). `Derivation::check` refuses a witness holding
    an open witness (`Meta`), which `check_inference` takes for a goal
    state's steps; the derivation's reader takes `{"E": n}` and refuses
    `{"M": k}`, as the goal state's reader does, since no step makes an
    open witness (`serialize::sequents::Extension`). `Tables::read`
    counts the largest term against `limits.terms` beside the atoms'
    arguments: a witness no atom names is written out in full as its
    instance's key, and a shared term of a few hundred bytes unfolded
    past any memory in the checker (the panel's finding). A method that
    reads a derivation's rules asks `Derivation::propositional` (the arena
    and no quantifier rule), not the arena's door alone: a propositional
    document may name `∀L`, and `Tableau::of` and `from_ordinary` reached
    their `unreachable!` arms on rule names.
  - **Eigenvariables are named by number** (`Formulas::eigen_names`): `a`,
    `b`, … `z`, `a_1`, … skipping the symbols, the linear proofs' scheme;
    the printer counts them as taken when it renames a binder, and an
    atom's key writes an eigenvariable `@n` and an open witness `?k`
    (`FirstOrder::write_with`; no name of the text syntax starts with
    either). The admissible view keeps a weakening whose formulas hold
    the eigenvariable of the `∀R` or `∃L` above it.
  - **The printer renames** (`write_first_order`, cold, only for a
    first-order arena): a binder whose name a symbol or an enclosing
    binder has is written `x_1`, `x_2` (the linear printer's scheme),
    a binder inside another formula is bracketed but a binder's body
    binder is not, and a binary body is bracketed. `Symbols` has
    `forall`, `exists` and `dot` per target; `Formulas::write` takes a
    `name` writer beside `atom` for symbols and variables.
  - **The parser** (`parse.rs`): `starts_binder` is one byte compare on
    the operand path (white space is skipped by then), `binder`,
    `predicate` and `close_binder` are cold, and the cold paths read names
    with `word`, a copy of `identifier`, and add atoms by `entry`, never
    `Formulas::atom`'s `get` and `insert`: a function the operand loop
    is the one caller of inlines there, and each second caller cost the
    `read-tptp` journey hundreds of thousands of instructions (with them,
    1.5 M more; now 0.5 % over the parser before, the clause splitter's
    bracket arm a third of it). `rest` stays out of line: inlined, the
    operator tokens' comparisons became `memcmp` calls. Natively a
    binder (`forall x y.`, `∀x.`) is a `Pending::Binder` that `reduce`
    does not take while a connective follows, so it reaches right; in
    TPTP (`! [X] :`) it takes its unit at once. **TPTP names are renamed
    by retrying** (`read_tptp`, `Renames`): a reading refused for a
    keyword name (`Error::AtomName` of an identifier) or for a name in two
    roles (`SymbolFault::Role`) is read again with that name renamed
    (`par_`, `name_f`, `_` added until no word of the file is it), one
    name more per try; a keyword is renamed in the atom's error path
    (`renamed_atom`) and the cold paths (`symbol_name`), a function symbol
    in term position alone, so a file that needs none reads as before on
    the hot path. The LLTP reader renames nothing: its syntax is the
    linear one, whose `top` and `bot` are the units LLTP writes. A name in term position
    is the innermost bound variable of that name, else a constant or
    function symbol; in TPTP an unbound upper-case word is refused. `=`
    and `!=` get the `EQUALITY` hint. `lltp::clauses` counts `[`/`]`
    with the parentheses, or a quantifier's `[X, Y]` read as an
    annotation.
  - **JSON at level 25** (`wire::ORDINARY_TERMS`): the sequent proxy has
    the linear sequent's `symbols`, `fo`, mixed `atoms` and `binders`
    (one name per binder in node order), written through
    `serialize::sequents::Tables::of_tables` and read through its
    generic `read`, then `Formulas::of_tables`, which interns every
    symbol, term and list again, so the arena holds each term once
    whatever the document repeats (with a term listed twice, `instance`
    missed what `instantiate` built: the panel's finding), and dedupes
    atoms by key; an atom form names the arena's atom by index. Tables
    without a term whose atoms are all nullary read as a propositional
    sequent (`None`), so first-order means a binder or arguments. A sequent or derivation is
    written at 25 when its arena is first-order, a goal state too.
    `wire/level-25.txt` pins a
    sequent and a derivation.

## The native engine (`ljt.rs`)

Dyckhoff's LJT (G4ip) for intuitionistic and minimal logic, public as
`ordinary::ljt::decide`, beside the embedding and the default method of
both logics (`Method::Ljt`). Its sources are cited in the module's docs;
its search and translation are untrusted, the checker is the trust.

- **Terminates by the calculus**: every rule's premises are lighter than
  its conclusion in the multiset ordering of Dyckhoff's weights (`∧`
  adds 2, `∨` and `→` 1; `¬A` as `A → ⊥`, `↔` as the conjunction of its
  implications on the left and in an antecedent), so the search has no
  loop check and no copy bound. It is depth first with a stack of its
  own (`Search::prove`, `Frame`), never a recursion: a frame's depth is
  bounded by the weight, but a formula may be as deep as it is long.
- **Hypotheses are a set** (`set`): contraction is admissible in G4ip
  (Dyckhoff and Negri 2000), so a sequent and its set are provable
  together; the translation weakens back the copies a rule's premise
  has (`Lj::weaken_to`) and the root weakens to the sequent as written,
  its hypotheses then rewritten in the written order (`Lj::in_order`,
  the principal found again by formula): the checker reads them as a
  multiset, but `Derivation::term` takes its context, the certificate
  its statement and the drawings their root from the root's order, and
  a root in the set's order gave a term whose context did not match the
  sequent (`the_root_keeps_the_written_order`).
- **Invertible first**: a leaf rule (the goal among the hypotheses, `⊥`
  outside minimal logic, `⊤` the goal) or an invertible rule is the only
  move tried; only `∨R₁`, `∨R₂` and `→→L` are alternatives. LJT's rule
  for an atom is taken for any antecedent among the hypotheses
  (`ModusPonens`), and the axiom for any formula: both shorten proofs and
  stay invertible and lighter. Minimal logic is the same with `⊥` an
  atom: no `⊥L`, and `⊥ → C` acts only where `⊥` is a hypothesis.
- **`→→L`'s second premise first**: `C, Γ ⊢ G` follows from
  `(A → B) → C, Γ ⊢ G` (Dyckhoff 2018, Prop. 2.2), so where it fails the
  frame fails at once, every alternative left untried
  (`Search::prove`); `close` stores the premises back in the rule's
  order for the translation. The `→→L` alternatives are tried nearest
  the goal first (`levels`, `heads`): an order, never a prune. Korn and
  Kreitz's formulas (ILTP's SYJ205) went from doubling per size to
  `15 + 5n` sequents.
- **Failure subsumption** (`subsumed`, `entailed`): a sequent fails
  without search when a failed sequent of the same goal has hypotheses
  `Δ` from which every hypothesis `x` of the new one follows by a rule
  that needs no search: `x ∈ Δ`, `x = ⊤`, `x = A → A`, `x = A → C` with `C ∈ Δ`, or
  `x = (A → B) → B` with `A ∈ Δ`. A proof of the new sequent and those
  derivations would cut to one of `Δ ⊢ G`. Each entailment holds in
  minimal logic (`⊥ → A` is left out: it does not). It makes no proof,
  so `Valid` still rests on the checker; it turned ILTP's SYJ209 and
  SYJ211 (non-theorems the backward search refuted in `2.1ⁿ` and
  `3.6ⁿ` sequents) polynomial. Only the latest `SUBSUMING` failures of
  a goal are compared, which bounds the cost and loses no verdict.
- **The memo is the verdict of every sequent closed**, proved (with its
  step) or failed: the search has no history, so a sequent's verdict
  does not depend on the branch. Proofs are steps in an arena, premises
  first, shared between branches; the translation unfolds them into a
  tree, so its size is bounded by `derivation_bytes` (the size
  estimate's measure, `derivation::inference_bytes`) and the memory
  account, never assumed linear.
- **The translation into LJ** (`Lj::step`) is one fixed derivation per
  rule. LJT's `∧→L`, `↔→L`, `∨→L` and `→→L` are derivable in LJ only
  with a cut: each cuts the formula the rule makes, derived from the
  implication taken apart (`P ⊢ A → (B → C)`, `P ⊢ A → C`, `P ⊢ B → C`,
  `P ⊢ B → C` for `P = (A → B) → C`); `→L` is split, so the hypotheses
  both premises need are contracted below it (`Lj::contract_to`).
  `Plant` plants a fault per gadget in tests, and
  `a_planted_wrong_translation_is_refused` shows the checker refuses each.
- **Bounds**: a sequent expanded is a unit of `limits.work`; the stop is
  asked every `POLL` expansions (the first included), and at any
  expansion once those since the last ask held `POLL` hypotheses
  (`unpolled`): an expansion costs about its sequent's size, and a chain
  of 100 000 implications overshot a 1 s limit by 11 s at every 256th
  (the integers panel of 54b, `a_large_sequent_asks_the_stop_at_each_expansion`);
  and once per inference of the translation; the search charges its frames, memo
  entries, steps and the formulas it adds to an `Account` and answers
  `Reason::MemoryLimit` past it; the translation charges its own and
  refuses with `Refusal::Memory` or `Refusal::Output`. Classical logic is
  `Error::Logic`.
- **`Derivation::term` checks a derivation read from JSON first**
  (`Derivation::trusted`, an `AtomicBool` so that `check(&self)` can set
  it): `from_parts` and the crate's own builders make a trusted one, the
  JSON reader an untrusted one (`untrusted`), `check` trusts it once it
  passes, and a view (`shown`) inherits the flag. So `term` never panics
  on input (`simple::read` indexes by the checker's invariants), and a
  checked derivation's admissible view, which `check` itself would
  refuse, still reads; an unchecked one's view is refused with the
  check's error. Checking every derivation first would refuse those
  views and a session's open goals.
- **What it copied from the linear engines** (for the kernel crate): the
  memo keyed by the sequent, the arena of proof steps, the stop's poll
  cadence and the memory account (`linear::search::memory::Account`,
  used as is).

## The SAT engine (`sat/`, behind `sat`)

intuit's search (Claessen and Rosén, LPAR 2015) with intuitR's restarts
(Fiorentini, CADE 2021), public as `ordinary::sat::decide` and
`Method::Sat` (`sat`), intuitionistic and minimal logic; classical logic
below. `mod.rs` the
clauses and the loop, `solver.rs` the trait over batsat, `read_back.rs`
the derivation; the model's check is `ordinary/model.rs`. Its search,
read-back and model are untrusted: the checker and `Model::refutes` are
the trust.

- **The solver is behind `solver::Solver`** (new variable, add clause,
  solve under assumptions, value, failed assumptions, calls,
  conflicts); `Batsat` is the one implementation, a fork or another
  solver replaces it there. batsat's `unsat_core` is MiniSat's final
  conflict, the negations of the failed assumptions: `failed` negates
  it back. Its `Callbacks::stop` is polled at every decision and holds
  the caller's stop in a `RefCell` (batsat polls through `&self`); a
  conflict (a learnt clause) and a call of `solve` are each a unit of
  `limits.work`; learnt clauses are charged to the account and released
  when `reduce_db` deletes them (it reports learnt clauses alone).
  `MOST_VARIABLES` is batsat's index bound; past it `Reason::IndexLimit`
  (the test lowers it through `run`'s `most`).
- **A variable is a formula of the arena**, not a fresh atom, so every
  clause is a formula's own rule and the read-back needs no
  definitions. Polarity (`polarities`, one pass from the last formula
  down, as intuit's one-directional definitions): a negative formula
  gets its left rule as clauses, a positive one its right rule, and a
  positive implication or negation its implication clause; an
  equivalence gives both polarities to its parts. `translate`'s extra
  nodes (`⊥`, both implications of every `↔`) are added first. The
  countermodel refutes the sequent because at every world a negative
  formula whose variable is true is forced and a positive formula that
  is forced has its variable true (the induction of intuit's
  completeness), which is all the root's hypotheses and goal need.
- **The loop is intuitR's `proveR`**: worlds are the solver's models;
  a pair (world, implication `(a → b) → c`) is realised when `a`, `b` or
  `c` holds there or a world above has `a` and not `b`; an unrealised
  pair calls the solver under the world and `a`, `¬b`; a model is a new
  world, a refutation learns `failed \ {a} → c` and restarts with no
  worlds. Realisation is monotone in the worlds, so each world keeps a
  cursor over the implications; the oldest world is examined first (the
  newest first left SYJ212+1.020 undecided). The call assumes `a` and
  `¬b` before the world: the solver blames a conflict on the first
  assumptions that imply it, and with the world first the clauses learnt
  carried the world's arbitrary choices, so SYJ205 learnt one per subset
  (41 at size 3 against 4, 32 890 at size 13 against a few hundred). The
  scan polls the stop every `POLL` pairs (`Solver::poll`, `Progress::work`
  `POLL`): the last round's scan makes no call and was cubic in the
  worlds without a poll. Setup charges each variable (`VARIABLE_BYTES`)
  and clause as it is made and refuses once over the bound (a goal of
  millions of conjuncts once built gigabytes before the first poll). The
  worlds of the last round, ordered by inclusion (sorted by size, edges
  the covering pairs), are the Kripke model; building it polls the stop
  once per world.
- **The derivation is read off the learnt clauses** (TABLEAUX 2019's
  correspondence): each lemma `B → c` becomes the formula
  `b₁ → (… → c)` derived by `→R` from a tableau of `Γ, earlier lemmas,
  B, a ⊢ b`, and cut below the tableau of `Γ, lemmas ⊢ C`. A tableau
  saturates by the rules the clauses stand for (left rules on negative
  formulas and lemma formulas, right rules on positive ones, the latter
  cut) and splits on a negative disjunction only where none applies;
  since the clauses entail the goal classically, every branch closes, and
  a branch that does not is `Error::ReadBack`, a defect. Derivations
  keep only the hypotheses they use (`tidy` contracts), a step whose
  made formulas are unused is left out, and a split whose first branch
  ignores its disjunct keeps that branch alone. Facts derived are the
  read-back's unit of `limits.work`.
- **LJT where a bound refuses the read-back** (not the stop): its
  derivation and statistics (`Statistics::ljt`); where the stop ends
  LJT, the stop; where another bound does, the read-back's refusal. LJT refuting a sequent the search proved is
  `Error::ReadBack`.
- **The model is the shared `kripke::Model`** (root module, step 55b):
  worlds with atoms by name, `falsity` (minimal logic only), arrows to
  the worlds directly above, and `world`, the world it is about. The SAT
  method's model is `ordinary::Countermodel` (the sequent and the model)
  on `sat::Outcome::model`, carried into `ordinary::Outcome::countermodel`.
  Every engine's model goes through `Countermodel::found`: checked,
  `None` where the check refuses it for its size (past `MODEL_CELLS`, or
  `limits.memory_bytes`), `Error::Model` turned into
  `Error::Countermodel` (a defect), a stop during its check the stop. So
  a model too large to check drops the model, never the verdict, and so
  does the SAT method's order past its memory account (the prompt's "too
  large to show"; such a verdict rests on the search, as LJT's does): the SAT method's
  pre-test counts worlds alone and the check adds the arrows, and the
  band between once lost the verdict as an error (the panels of 55b).
  The embedding's model (`Image::countermodel`) is also dropped where
  the stop ends its check, and the classical default takes the solver's
  where the embedding's refutation is not an assignment.
- **The check** (`model.rs`): `Model::forcing` checks the model is one of
  the logic (the model's world among its worlds, arrows to worlds of the
  model and none round: Kahn's order on the arrows, a world after every
  world above it; atoms and `falsity` growing along every arrow, by
  names, so an atom the sequent does not name counts too; `⊥` in minimal
  logic only; one world classically), then computes forcing for every
  formula of the arena at every world in that order, an implication from
  its local value and the worlds directly above; `¬A` is `A → ⊥`, so in
  minimal logic a world forcing `⊥` forces `¬A` too. `Forcing::holds(w)`
  (the sequent at every world at or above `w`), `failing(w)` (a world at
  or above that forces Γ and not C), `at(w)` (each formula's forcing).
  `refutes` asks that the model's world forces Γ and not C. Past
  `MODEL_CELLS` cells, or past `limits.memory_bytes` with the few words
  per world, arrow and atom the passes hold (`WORLD_BYTES`,
  `ATOM_BYTES`), `Refusal::Memory`; a model that fails is `Error::Model`
  (kind invalid, code `invalid_model`). The stop is asked once per world
  in each of its three passes, the first before any arrow is checked;
  an arrow written twice is one (`ups`, sorted and deduplicated), and a
  world's atoms are sorted indices (the sequent's first, so its own
  are a prefix) looked up in each world directly above: before, a
  world of 30 000 atoms and 30 000 copies of one arrow ran a minute
  under a 1 s limit. `failing_anywhere` is a scan of `fails`; a
  `failing` per world was quadratic. The panel of 55b reviewed it
  (`plan/reports/55-kripke-countermodels.md`).
- **The classical one-world model of the embedding**
  (`Image::countermodel`): the image's `Refutation::Classical`
  assignment over the image's atoms (the ordinary atoms, by name),
  checked by `refutes` against `Image::ordinary` before it is kept.

### Classical logic (`sat/classical.rs`, `sat/drup.rs`)

`sat::decide` and `Method::Sat` decide classical logic too: one call of
the solver on the clauses of the sequent's negation. The encoder and the
clausal proof's checker are the trust for "valid"; `Model::refutes` for
"not valid".

- **The encoder** (`encode`) is Tseitin's with polarities
  (Plaisted–Greenbaum): every formula the sequent holds is a variable, a
  formula that must be true where the sequent fails (`TRUE`: hypotheses,
  the parts that make a true formula true) gets "formula → definition",
  one that must be false (`FALSE`) "definition → formula", an
  equivalence gives both to its parts; units make every hypothesis true
  and every formula right of `⊢` false. Its soundness argument: every
  definitional clause holds when each variable takes its formula's value
  under any assignment of the atoms, and the units hold exactly where
  that assignment falsifies the sequent; so the clauses have a model
  wherever the sequent is falsifiable. A change to a clause must keep
  that. It has no `⊥` variable of its own: `⊥` and `⊤` are formulas like
  others (`⊥` true is refused by a unit, `⊤` false likewise).
- **"Not valid"** is a `Model` of one world, the atoms the solver's model
  makes true; `Model::refutes` in classical logic demands one world, no
  `⊥`, every hypothesis true and every formula right of `⊢` false, so it
  is the classical evaluation and needs nothing of the encoder.
- **The proof is recorded by `Batsat::recording`**: `Bridge` appends every
  learnt clause (`on_new_clause`) and every deleted one
  (`on_delete_clause`, from `reduce_db`) as a `drup::Step`, charged to the
  account; a refuted call under assumptions reports its final conflict as
  learnt, which `solve` pops (classical logic calls with none). The run
  appends the empty clause, which the solver never learns: a conflict at
  level 0 returns at once. drat-trim (nixpkgs' 2023-05-22) accepts the
  stream (`drat_trim_accepts_the_proof`, ignored: `nix shell
  nixpkgs#drat-trim`).
- **What batsat does not log**: `remove_satisfied` drops satisfied
  clauses and trims literals false at level 0 from the others in place,
  both silently; `add_clause_reuse` drops level-0-false literals and
  duplicates. So a later deletion of a trimmed clause matches nothing
  kept, which the checker ignores (drat-trim warns "deleted clause …
  does not occur"), or another kept clause of the same literals, which
  it deletes (sound: a deletion only weakens), and the longer clause
  stays, which RUP tolerates: the level-0 units it lacks are propagated
  again in each check. The
  account's release of a trimmed clause is short of its charge
  (conservative).
- **The checker** (`drup::check`) keeps the formula's and the proof's
  clauses not deleted, checks each added clause by reverse unit
  propagation and accepts at the first empty clause added. Each check
  starts from no assignment and undoes all after, so nothing but the
  clauses and their watches carries between checks, and deleting any
  clause, a unit or a reason, cannot leave a stale assignment (the usual
  fault of DRUP checkers that keep the top-level trail). Clauses are
  sorted and deduplicated when kept; a tautology is never kept; a
  deletion of a clause not kept is ignored; a literal past the formula's
  variables is a flaw. Its soundness needs that propagation assigns a
  clause's last literal when the others are false and calls a conflict
  only on a clause all false, and for that the watch invariant: a clause
  in the list of literal `l` has `l` among its first two literals, since
  `propagate` takes the second to be the literal just falsified without
  testing it (kept by `keep`, the move to a new watch and both `drain`s).
  Completeness of the watches only avoids false refusals.
- **Its bounds**: a step is a unit of `limits.work` (a check costs at
  most the literals kept, and repeats the unit clauses' propagation, so
  the whole check is quadratic in the worst case), the clauses, watches
  and index are charged (`CLAUSE_BYTES` beside 8 bytes a literal; the
  first estimate was 2.4 times short, the integers panel measured) and
  refused as `Refusal::Memory` past the bound, the stop is asked every
  `POLL` units of the checker's work: a step, a clause of the formula
  kept, a unit clause assigned, a literal propagated, a clause visited
  (a check of a million unit clauses once ran seconds between polls
  when only steps and visits counted). Clause indices are `usize`
  bounded by the vector's length; literal codes `2·var + sign` fit a
  `u32` since `var < MOST_VARIABLES`. A failed check is
  `Error::ReadBack` (calculus LK), a defect: the stream is the engine's
  own. An outside proof's flaw (`resolution::check_proof`) is
  `Error::InvalidRefutation`, kind invalid; `ClausalProof::checked` and
  the modules `drup` and `solver` are `pub(crate)` for it.
- **The setup polls too**: the encoder asks the stop every `POLL`
  formulas, and the solver is asked (`Solver::poll`: stop, work, memory)
  every `POLL` clauses it is given, since it copies them and
  `add_clause` only charges; a goal of millions of conjuncts once ran
  1.7 s before the first poll and held 2.5 times the memory bound. The
  solver and the check each get the whole of `limits.work` in turn.
- **"Valid" is the checked clausal proof** (`Evidence::Clausal`, a
  `ClausalProof`): the author's decision of 2026-11-14, as Lean's
  `bv_decide` and the SAT Competition's certified track take a checked
  DRUP/LRAT proof as the certificate. `ClausalProof` has private fields
  and one constructor, `ClausalProof::checked` (`drup.rs`), which runs
  `drup::check` and builds the value only when it accepts, so no code
  makes a classical "valid" from an unchecked stream; it holds the
  counts (variables, clauses, steps), not the proof, which is dropped
  after the check. `Method::Sat` no longer asks the embedding for a
  derivation: its cost was the embedding's whole search, which made the
  method no faster than the embedding on valid sequents. A caller that
  needs the derivation decides with `Method::Embedding`, the classical
  default; `Evidence::derivation` refuses a clausal proof with
  `Error::NoDerivation` (unsupported, `no_derivation`), whose message
  names that method, and every reader that needs a derivation (the
  goal state's `close` and `hint`, the command's drawing, its lambda-term,
  the batch's file) goes through it.

## Evidence (`decide.rs`)

- **`Verdict::Valid(Evidence)`**: `Evidence::Derivation(Box<Derivation>)`,
  checked by `Derivation::check`, from every method but the SAT solver
  in classical logic; `Evidence::Clausal(sat::ClausalProof)` (feature
  `sat`), checked by `drup::check`. Both are built only after their
  check: a derivation by `decide`, `ljt::decide` and `sat::decide` after
  `check`, a clausal proof by its constructor. The type proves that a
  check ran; that it ran on this sequent's clauses rests on the one call
  of `ClausalProof::checked` (`classical::decide`, on `encode`'s output,
  unmodified): `ClausalProof` is `Copy` and the variants are public, so
  a library caller could move one outcome's proof into another's
  verdict, as it can wrap any derivation (the argument panel's note).
  The enum is
  `#[non_exhaustive]` (step 70's outside proofs, or LRAT, may add
  evidence), so the command and the harness match the variant they need
  and take the rest with `_`.
- **The exports take a `Derivation` by type**, so none can be handed a
  clausal proof: the refusal is `Evidence::derivation`'s, at the one
  place a verdict turns into a derivation. An export, a drawing or a
  term of an outcome goes through it.
- **`Evidence::Image`**: the image's proof, which the linear checker
  accepted, where its derivation was not built (a bound or the stop
  refused it). Only `Image::outcome` gives it, for a front end that ran
  its own search of the image (the command's race, the batch); `decide`
  never does, a refused derivation staying that refusal there.
  `Evidence::derivation` answers `Error::NotBuilt` for it.
- **The JSON form** (wire level 12, `wire::SEARCH`, every ordinary
  outcome, the author's Q12): `{version, deduxit, verdict, reason?,
  logic, search, derivation? | clausal_proof?}`, `search` internally
  tagged by `method` (serde's `tag`, as a refutation is by `kind`):
  `embedding` with `translation` and `image`, the image's whole outcome
  nested; `ljt` and `sat` with `statistics`. Neither evidence key under
  a valid verdict is `Evidence::Image`. It reads back
  (`wire::Readable`, form `ordinary outcome`): the derivation and the
  image's proof or disproof by their readers, unchecked; it refuses an
  outcome below level 12 (no `search`), one valid by a clausal proof
  (the document holds the counts, not the proof: a `ClausalProof` is
  only ever built by its checker) and one whose verdict the image's
  outcome does not bear out. 55b's countermodel is a key beside
  `derivation` for `not_valid`, read the same way.

## Truth tables and tableaux (`table.rs`, `tableau.rs`)

The methods `Method::Table` and `Method::Tableau` (behind `interactive`),
classical logic only (`Error::Logic`, which names the method).

- **The truth table** (`truth_table`, `TruthTable`): a bit per row and
  formula of the sequent's arena, rows from every atom true to every
  atom false (the first atom slowest, `assigns`), the falsifying rows
  those where every hypothesis is true and every formula right of `⊢`
  false. Refused past `limits.table_atoms` (`Refusal::Atoms`,
  `DEFAULT_TABLE_ATOMS` 10: 1024 rows, about what a reader scans), past
  `limits.memory_bytes` (the bits, and 8 bytes a row for the falsifying
  list, which every row may join), past `limits.work` (a row a unit),
  and where the cells pass a `usize` (`Refusal::Index`). A "not valid"
  is the first falsifying row as a one-world countermodel through
  `Countermodel::found`; a "valid" is `Evidence::Table`, which has no
  derivation (`NoDerivation`, its `evidence` "its truth table").
- **The tableau is LK upside down** (the brief's design): `Tableau::of`
  reads any derivation of LK, its structural rules skipped, an
  inference on a formula an expansion of that formula's line, an axiom,
  `⊥L` and `⊤R` closed branches, an open goal `End::Open` (finished:
  `next_step` has nothing) or `End::Unexpanded`. A node's lines are what
  the premise has beyond the conclusion counted as a multiset, so an
  expansion's results are written even where the branch has them (as a
  textbook writes them) and a contraction adds nothing. Each formula of
  an inference's sequent is tied to its line by position (`Place`,
  `placed`), carried through the structural rules, so an expansion and a
  closed branch cite the line of the occurrence the inference acts on:
  a search of the branch by formula cited the latest equal line, and a
  formula written twice was cited twice (the review of 56a). A
  structural rule without a premise is `Error::ReadBack`, never an index
  past the end. Nodes and lines are numbered in preorder, the first
  premise first.
- **The method builds the tableau in the goal state**
  (`tableau::decide`): `next_step` takes a closing rule where one
  applies (axiom, `⊥L`, `⊤R`), else the first formula of a one-premise
  rule (Smullyan's α: `∧L`, `¬L`, `↔L`, `∨R`, `→R`, `¬R`, the left side
  first), else of a two-premise one (β: `∨L`, `→L`, `∧R`, `↔R`); `⊤`
  true and `⊥` false are left as they are. Every step goes through
  `Interactive::apply`, so every inference is checked by
  `check_inference`; a closed state's derivation is `finished` (checked
  whole) and is the evidence; an open state's derivation (open goals
  as `Rule::Open`) gives the tableau, and the first open goal's true
  atoms (once each) the countermodel, checked by `Countermodel::found`.
  A step is a unit of `limits.work`, the stop is asked once per step and
  once per branch closed, and every goal a step opens is charged to
  `limits.memory_bytes`
  (`GOAL_BYTES` and 8 bytes a formula): the goal state grows with the
  tableau, and the pigeonhole principle for 8 pigeons once took 4 GB
  under a 16 MiB bound. **The branches close after the search**
  (`expand` returns each closing step untaken, `decide` applies them
  after): the axiom weakens every other formula of its goal away, one
  inference holding the rest each, so closing as the search met them
  grew the state with the square of each goal, and freeing it after the
  stop overran a 2 s limit by 0.54 s (4.4 million inferences on ILTP's
  SYJ202+1.008; no step was slow). The tree, and so the tableau and the
  derivation, are the same. What is left is freeing the expansions, every
  premise a copy of its goal in three vectors: 0.2 s at 2 s on
  SYJ206+1.020 (1.7 million inferences); one allocation per inference
  would be the next step (`Inference`'s storage). **Each axiom's
  weakenings are charged when the search meets it** (`axiom_bytes`:
  `n − 1` inferences of `n(n − 1)/2` ids for a goal of `n` formulas),
  so the search refuses with `Refusal::Memory` before the closing phase
  builds them: uncharged, 2 048 branches of 1 200 formulas took 6 GB under
  a bound of 256 MiB. The charge is about twice what the state holds
  (ids are 4 bytes, charged 8, as an opened goal's are; 1.7 GB resident
  for 3.1 GiB charged), so a refusal comes early rather than late. The
  bound covers the goal state; the derivation `finished` and
  `derivation_within` copy from it, its check and `Tableau::of` are the
  proof returned, bounded by `derivation_bytes` (by the estimate, 2 to 9
  times their resident size): under `--derivation-limit none` a run
  just within the memory bound peaked 1.1 to 1.7 times it. It ends:
  every rule takes a formula apart and the goal state keeps no
  principal formula (G3c's invertible rules).
- **Why not a view of the decided outcome's derivation alone**: a "not
  valid" has no derivation, and the SAT solver's classical "valid" has a
  clausal proof; a tableau of the embedding's read-back would also
  follow the linear search's order, not a tableau's. The method builds
  its own LK derivation and `Tableau::of` stays the view of any
  derivation of LK.
- **Their JSON** (wire level 14, `serialize/ordinary.rs`): `TableForm`
  (rows as `T`/`F` strings over the form's formulas; the reader
  computes the table again and refuses a row or a falsifying list that
  differs; the outcome's reader refuses a verdict its table or tableau
  does not bear out, `valid` with a falsifying row or an open branch) and `TableauForm` (nodes with `lines`, `expands`,
  `children`, `end`; the reader checks the tree's shape, a child after
  its parent and the child of one node, every line index in range, a
  node with children expanding a line and a leaf with an end, not the
  expansions).

## The invertible rules (`invert.rs`)

`Interactive::invert`, `invert_all` and `is_invertible` (feature
`interactive`), the linear session's `invert.rs` read for LK and LJ:
`interactive.invertible` and the front ends' switch drive them.

- **The set** (`invertible`, one table by logic, side and connective):
  the axiom, and in LK every rule of a connective with a shared context
  (`⊤L`, `⊥R` included); in LJ and minimal logic all but `→L`, `¬L`
  (the first premise drops the formula right of `⊢`) and `∨R₁`, `∨R₂`
  (a choice); minimal logic has no `⊥L`. The partial rules (`∧L₁`,
  `↔L₂`), the structural rules and a split context are never taken.
  These are G3's invertibility facts for rules that drop their
  principal; the test checks them semantically, every goal of a valid
  sequent valid after the run, on 200 generated sequents in each logic.
- **It ends**: every premise is lighter than its conclusion with `↔`
  weighed as `2(w(A) + w(B)) + 3`, above its two implications.
- **Each step goes through `apply`**, the public step, so the checker's
  rule checks every one, and a step `apply` refuses (the axiom's
  weakenings past the session's bound) leaves the goal open. Every
  inference a step added (`added_bytes`: from the count before it, and
  the goal's own) is charged by the derivation's estimate
  (`derivation::inference_bytes`, which `derivation_within` bounds a
  derivation by) to `limits.memory_bytes`, `Refusal::Memory` in
  `Inverted::finished` after the step that passed it, the steps taken
  standing. Charging only the goals opened let an axiom's weakenings
  (n inferences of up to n formulas) pass: 100 atoms made 2.2 MB under a
  bound of 256 KiB, and a page's session too large to draw (the review
  of 54x7).
- **`invertible_steps`** lists the goals the history's steps closed by
  an invertible rule: an axiom step is its goal's chain of weakenings
  (premises no other step closed) ending in the axiom, whose goal holds
  the first weakening; a two-premise rule counts as shared where each
  premise holds at least as many formulas as its goal. Read off the
  drawing it named the axiom's inference instead of the step's goal,
  and a split that dropped one formula per premise passed. `Inverted` is the linear
  session's type, its goals the ordinary session's ids.
- **The order**: a closing step (the axiom, `⊥` true, `⊤` false) first,
  else the first formula's rule, the hypotheses before the formulas
  right of `⊢`; depth first, `Inverted::open` left to right.

## A tableau written by hand (`written/`)

`WrittenTableau` (features `parse`, `serialize`, `interactive`): a
student's signed tableau read from text and reviewed step by step
against the rule file `rules::TABLEAU`, which the caller reads and
passes (core has `serde_json` only under `protocol`).

- **The text is what `Tableau`'s `Display` prints**, so `prove
  --method tableau`'s output reads back: numbered lines (`3` or `3.`,
  `T`/`F`, the formula in the ordinary syntax, then optionally a rule's
  name and the number of the line expanded), `×` (`x`, `*`, `⊗`) with
  zero, one or two line numbers, `open`. The formula is parsed by
  `parse::formula_in` into one arena for the whole tableau, so equal
  formulas are one id; an error is a `ParseError` placed in the whole
  text. The justification is split off from the right: a last word of
  digits is the line expanded, a word before it that `Named::read`
  takes is the rule (no ordinary atom is all digits, and a rule's name
  is no formula's last word).
- **A word before the line number that reads as a rule's name** (`¬R`,
  `∧L`) is taken as the rule only where the formula then reads; else it
  is the formula's last word (`T p ∧ ¬R 1`, atom `R` negated). A
  formula cannot end with a negation's operand missing, so the two
  readings never both parse with `¬`; with `∧L` both do (`p ∧ L`), and
  the rule wins, as the printer writes it.
- **A formula's "end of input" error** is placed in the whole text at
  the character after the formula (`parse::formula_in`): the parser
  reads only up to the formula's end, and the command's caret otherwise
  went to the text's last line.
- **The layout**: a branch starts with a marker (`├─`, `└─`, `+-`,
  `\-`, `` `- `` or `-`; `|-` counted at the bar's column) at the column of its parent's lines,
  its own lines at the column after the marker; a line without a marker
  goes on the open branch whose lines stand in its column (a stack of
  open branches, each with its column). So the drawn tree, its ASCII
  form and a plain list read alike. A line after a split, after a mark,
  or indented under no branch is a `ParseError` with a hint.
- **The first lines that cite no line are the sequent's** (`root_lines`),
  hypotheses `T`, conclusions `F`; a later line without a citation is a
  fault of the review, not of the reader.
- **The review's own matcher gives the words; the rule file gives the
  verdict.** `Walk` goes through the branches in text order (which is
  preorder: branches are made as the text reads), keeping the current
  branch's lines (`path`, `on_path`, `present`), groups consecutive lines
  citing one line into one expansion (`group`: as long as they fit one
  result list of `shapes`), and matches each against `shapes`, which
  lists exactly the rules of `TABLEAU` that expand the line (the partial
  ones `T∧₁`, `F∨₁`, `F→₁`, `T↔₁`… marked `partial`; every rule that
  does not split has a part per result, so a group within a result list
  always completes one, which `expansion`'s `unreachable!` rests on). A group passes where it is
  within a result list as a multiset and the branch with it holds the
  whole list, which is exactly when the rule's node is an instance (the
  branch is a set). A split's two branches must start with the two
  result lists in either order. Every fault is kept only if it is at an
  earlier text line than the one held, and the walk goes on, so the
  answer is the first fault in the text whatever the branch order. A
  split at fault passes over its branches' first lines citing alike
  (`passed`): read again as expansions they were said again, at a line
  earlier than the other branch's fault.
- **No fault, then the derivation**: `Walk::plan` writes a node per
  expansion, split and leaf, each holding its branch's signed formulas
  once (first occurrence order), `derivation` pushes them in reverse
  (premises first), and `review` runs `Derivation::check` (all closed)
  or `check_open` (open and unmarked branches as open goals). A refusal
  of the rule file is `Standing::Wrong` with its reason: the matcher's
  pass is never the verdict, so a matcher that accepted too much shows
  as that message, never as `Closed` or `Open`. A closed tableau is a
  checked derivation in `TABLEAU`; an open one's first open branch gives
  the one-world model of its true atoms, checked by
  `Countermodel::found`.
- **`finished`** (an `open` mark, the unfinished message) is the
  Hintikka condition read off `shapes`: every line has a whole result
  list of one of its non-partial rules on the branch, and nothing
  closes it (`closer`: a formula true and false, `⊥` true, `⊤` false).
  It decides only what the student is told: the open verdict rests on
  the countermodel's check.
- **Bounds**: the planned nodes are charged `NODE_BYTES` and
  `MEMBER_BYTES` per member against `limits.memory_bytes`
  (`Refusal::Memory`), since each holds its whole branch (nodes times
  depth); the formulas count against `limits.occurrences`; lines and
  branches are `u32` (`Refusal::Index`); the stop is asked per branch
  and by the checker per node.
- **The expected sequent** (`review`'s `expected`) is compared as two
  sets of signed formulas with the first lines, after mapping its arena
  into a copy of the tableau's.
- **JSON** (`serialize/ordinary.rs`, `ReviewForm`, written only, level
  14): `{version, standing, sequent, text_line?, line?, message,
  countermodel?}`; nothing reads it back.

## The goal state (`interactive.rs`)

A native state of ordinary sequents, not a session over the image: the
goals are two-sided sequents, the rules `Rule`'s, and a finished state
is a `Derivation` that `Derivation::check` validates, so nothing is
added to what is trusted. The linear session (`core-derivations.md`,
"Interactive proving") is its model; what was copied is listed in
`plan/reports/49-ordinary-interactive.md`, and nothing generic is
factored out of the two until the kernel crate is.

- **Every inference a step makes passes `check_inference` before the
  state changes** (`expand`): the checker's own per-inference check
  (the logic's right-side counts, then the rule), factored out of
  `Derivation::check` so that both read one function. A step the
  construction got wrong is an `unreachable!` panic, never a state the
  final check would refuse; `random_steps_finish_as_checked_derivations`
  walks random steps, splits and undos over generated sequents in
  every logic and finishes every state the search can close.
- **`shape` is the one rule table**: which rule acts on which
  connective and side, which the logic refuses (`WR`, `CR`, `⊥R` and
  `∨R` with both disjuncts outside LK, `⊥L` in minimal logic), and what
  each premise holds in the principal's place and on the other side.
  `rules` lists exactly the rules `shape` accepts (plus the axiom where
  the formula stands on both sides), so the list and `apply` cannot
  disagree. A premise keeps the goal's order, the principal's place
  taken by its subformulas, a formula crossing `⊢` appended.
- **In LJ and minimal logic every goal has exactly one formula right of
  `⊢`**: `Γ ⊢` starts as `Γ ⊢ ⊥` (as `translate` decides it, so the
  search's derivation of a goal concludes the goal itself), the rules
  whose premise would have none or two are refused, and `¬R`/`¬L` take
  the form of `→R`/`→L` for `A → ⊥`; classically they are LK's
  one-premise forms. The reader refuses an open goal of LJ without one
  formula right of `⊢`.
- **The axiom closes a goal with a context** by a run of weakenings
  below `A ⊢ A` (the last formula left of `⊢` first, then right), one
  step that `undo` takes back whole. A rule with two premises shares the
  context unless the step splits it (`Context::Split`, positions per
  side going to the first premise). The checker accepts a binary rule
  either sharing or splitting, never both at once, so LJ's `→L` and `¬L`
  (their first premise has its own formula right of `⊢`) share by
  `contracted`: a `CL` of every other hypothesis below, each copy right
  after its formula, then the rule split, first copies to the first
  premise, second copies and the goal's formula right of `⊢` to the
  second; the admissible view (the default, `DEFAULT_STRUCTURAL`) leaves
  the contractions out and shows G3's `→L` with the context in both
  premises (the supervisor's correction of the first design, which made
  a first course split by hand). The principal is not kept in the
  premises: a hypothesis used twice is contracted by hand.
- **The arena holds the extra nodes from the start** (`Extra::add`:
  false and both implications of every `↔`), so `shape` finds them by
  `Formulas::ids`, and the derivation `decide` returns for a goal is
  over an equal arena; `ids_of` maps its nodes anyway, adding any the
  state lacks.
- **`close` and `hint` decide the goal as a sequent of its own**
  (`ordinary::decide` with the state's logic): `close` grafts the
  derivation as one step (root in the goal's place keeping the goal's
  order, its principal found by formula; the rest appended in reverse),
  refused past `derivation_bytes` by `estimate`; `hint` reads the
  derivation's first steps (`first_steps`): from the root up to and
  including the first rule that is not structural, each step's rule,
  formula and, for two premises, whether they share the context or which
  positions go to the first (`step_at`), each applied in a copy of the
  state to find the goal the next acts on. A contraction hinted alone was
  answered, on the contracted goal, by a weakening of the copy, and the
  two alternated for ever (the review of 49b; the linear bindings' hint
  had the same fault, `web.md`); `following_the_hints_proves_a_sequent`
  pins it. A refutation or an unknown leaves the goal
  open with the outcome saying why; a stop is `Unknown(Stopped)`, a
  refused read-back `decide`'s error.
- **The structural rules as a display choice** (`ViewOptions::structural`,
  `Structural::{Explicit, Admissible}`, admissible by default): `Derivation::shown`/`admissible`
  leaves out every `WL`, `WR`, `CL`, `CR` whose branch the rules above
  close, giving its conclusion to the first rule above that is no
  structural one (principal found by formula), which reads as G3 (the
  axiom with a context, a contraction absorbed into the rule that uses
  both copies). A run above an open goal stays, so an open goal shows
  its own sequent. What is shown admissible fails the checker by
  design; `finished` always checks the explicit derivation.
  `derivation_ids(structural)` maps a drawn inference to the state's
  goal, for both views.
- **Bounds**: `within` and every reader refuse a sequent past
  `limits.occurrences` (subformula occurrences, saturating), the readers
  also an arena longer than it; an axiom's weakenings and a shared
  rule's contractions hold the square of the goal's size, so `apply`
  refuses such a run past the `derivation_bytes` the state was started
  or read under (`limit`, not serialized; `run_within`'s upper estimate,
  `StepError::Size`) before it is made (the second review measured 2 000
  hypotheses at 6 million ids, unbounded before); `derivation_within` and the graft refuse
  past `derivation_bytes` by `estimate` (the size estimate's constants,
  per formula its characters with brackets and two for its separator)
  before anything is built; the stop is asked by `derivation_within`,
  by `decide` inside `close`/`hint` and by the check in `finished`. Every
  walk is a loop (`postorder`, `subtree`, the reader's history walk),
  and the arena-order passes (`occurrences`, `estimate`, `ids_of`) need
  no recursion since every operand precedes its formula.
- **Reading a session** (`Interactive::from_parts`): the root first,
  every other inference the premise of exactly one before it, an open
  goal without rule, principal or premises, every closed inference
  passing `check_inference` with its premises, and the history walked
  from the last step back as the linear reader walks it (each entry a
  distinct closed inference below the arena's end then, what its step
  added the suffix of that arena), so a state read is as sound as one
  built. The derivation's reader checks only its shape (a tree, its root
  last, indices in range), so that a drawing of any document is safe;
  the Rocq writer checks a derivation before it writes one and refuses
  an open goal (`Unsupported::Open`).
- **What the review found** (all fixed, each pinned in
  `a_forged_document_is_refused`): a session document whose history
  closed an LJ goal without one formula right of `⊢` let `undo` reopen
  it, and `close` then grafted the search's `Γ ⊢ ⊥` onto `Γ ⊢`; now the
  reader refuses such a step and `graft` refuses a root that does not
  conclude the goal (`Error::GoalMismatch`). The admissible view hides a
  structural rule only where its conclusion holds the principal of the
  rule above; the derivation reader takes an open goal only as a leaf
  (the text layout's height underflowed under a bare goal with a
  premise).
- **`Rule` reads from text** (`FromStr`, `derivation.rs`): its name, a
  subscript as a digit (`∧L1`), and the ASCII spellings of `ASCII`, the
  connectives as the parser reads them (`/\R`, `->L`, `~R`, `<->L1`) and
  as words (`andR`, `impL`, `notR`, `iffL1`, `falseL`, `trueR`), the
  word forms because the command's word splitter takes `\` as an
  escape. `open` is refused: no step takes it. The serde form stays the
  name alone.
- **The quantifier steps** (`instance_step`, apart from `expand`, since
  they build the instance with `instantiate` and so need `&mut self`):
  `∀R` and `∃L` take no witness (`StepError::NoWitness`) and name a fresh
  eigenvariable, numbered past every eigenvariable the live inferences'
  formulas and terms hold (`largest_eigenvariable`), so that an undo and
  the same step again name the same one (`undo` clears the inference's
  `term`, which that count reads); `∀L` and `∃R` need a witness
  (`Step::with_witness`, `StepError::Witness`), held to the goal's scope
  (`held`, `scope`: the eigenvariables of the `∀R` and `∃L` below the
  goal; an eigenvariable of a sibling branch is refused though it would
  be sound as a constant, as the linear session does). `∀L` keeps its
  principal (G3), `∃R` only in LK. `Interactive::witness(goal, text)`
  reads a term of the sequent's function symbols and the scope's
  eigenvariables by their printed names. `∀R` and `∃L` are invertible,
  `∀L` and `∃R` not (a choice); the invertible run recomputes its
  widths when a step grew the arena. A quantifier step runs `Extra::add`
  after `instantiate`: an instance may hold an equivalence the arena
  lacked, whose implications `shape` looks up (`rules` on one panicked).
  The session reader runs the `closed` pass and takes no `{"M": k}`, as
  the derivation reader does; `derivation_ids(Admissible)` maps a
  first-order state with its formulas and terms, since the admissible
  view reads them there.
- **Open witnesses** (`Witnesses`, `None` until a step leaves one
  open): `∀L` and `∃R` without a witness put `Meta(k)` (`$1` for
  `k = 0`) in the instance, its level one more than the largest
  eigenvariable in the goal's scope. The axiom, where no formula on the
  other side is equal to the step's, takes the first that unifies
  (`unify_axiom`: `Formulas::term_pairs` of the two formulas, left of
  `⊢` first, then the shared `crate::unify::Unifier`, trailed under the
  step's number), refusing an eigenvariable at or above a witness's
  level, one outside the goal's scope (`StepError::Binding`, which
  names it), and a binding after which the arena's terms would pass
  `limits.terms` by the measure a document's reader takes
  (`arena_terms`: the atoms' arguments written out, summed over the
  atoms, or the largest term), predicted before anything is built: each
  atom holding a newly bound witness makes at most one atom more, of its
  arguments resolved (`unfolded`). Shared terms bound in a chain stand
  for exponentially long ones (20 links took 3.9 s and 44 MB), and a
  bound per binding alone let a session of 96 steps write a document of
  129 KB that its own reader refused (1.5 to 4.8 GB at its peak); the
  arena only grows, so the check cannot come after the build. Why levels and scope: the eigenvariables a binding may name
  are those of the `∀R`/`∃L` below the step that left the witness open,
  so none lands in its own rule's conclusion.
  - **Every inference shows the bindings in force** (`bind`: a
    `Resolution` over the arena replaces each bound witness, transitively,
    in every inference's sides and term), so `check_inference`, the
    views, the exports and the commands read them as any inference;
    every inference that changes is checked first and a failure refuses
    the binding, changing nothing. A binding builds nodes, among them
    equivalences the arena lacked, so `bind`, `finishing` and the reader
    add their implications (`implications_from`, from the arena's length
    before): `shape` and the checker look them up, and without them a
    later `rules` panicked and a finished proof failed its check. What an inference was made as is kept
    beside it (`Witnesses::raw`, `Some` only where a binding changed it;
    the goal of a later step keeps its sides as made and takes the
    step's term), and `undo` takes back the step's trail and witnesses
    and shows the rest again from it (`reshow`); the witnesses go once
    none is left.
  - **A finished derivation holds no open witness**
    (`finishing`, in `derivation_within` when no goal is open): each one
    no axiom bound becomes an eigenvariable numbered past every one the
    arena holds, which no `∀R` or `∃L` introduces, so the checker reads
    it as a constant (the linear session's rule). While a goal is open
    the derivation shows `$1`.
  - The invertible run's axiom closes equal formulas only, and
    `invertible_steps` leaves out an axiom that bound a witness: a
    binding is a choice. `rules` lists the axiom where a formula unifies
    (on a copy of the unifier). `bindings()` gives each bound witness's
    name and term as text, which the command prints after a step.
  - **The JSON** writes the inferences as made (`parts`, a `Cow` that
    borrows where there is no witness) and `"bindings": [[k, term,
    step]]`, written only where there is one, in the level-25 form
    (level 25 was not released before them); `{"M": k}` is read in a
    session's terms (`Extension::Session`) and nowhere else.
  - **The reader replays them** (`Replay::run`, only for a document with
    a bound or opened witness, so a propositional one reads as before):
    every closed inference belongs to the step that made it (the goal
    tree's `added` ranges), and one no step closed is refused; a
    document whose formulas hold a `Meta` anywhere takes this path too.
    Per step in order: the witness of its one `∀L`/`∃R` whose term is the
    next `Meta` (any other term holding one is refused) is made at the
    level the tree gives (`Tree::level`, one pass down the tree); no
    inference of the step, nor its premises' sequents, holds a witness
    not made yet (`largest_meta`; a premise's term is its own step's);
    an axiom step (its goal's weakenings, each closed by this step, end
    in an `A ⊢ A` made unequal) binds by `unify_axiom` on those two
    formulas resolved under the bindings before the step, which are the
    formulas the live step unified (the goal of an axiom may keep sides
    made before an earlier binding, so its formulas as made are not);
    the scope is read by the walk's intervals (`Tree::in_scope`); and
    every inference of the step is checked as it shows the bindings
    then. At the end every inference again under the final bindings (a
    later binding can bring an eigenvariable in, never take one out),
    rebuilt once (per step the reader builds the step's inferences only;
    rebuilding all per step was quadratic, 32 s for 4 000 steps), and the
    bindings found must equal the document's. The sequent may hold no
    `Meta`. A weakening the user took before an axiom is that earlier
    step's, so the chain stops at a goal another step closed.
  - `undo` keeps the nodes a step built, as for an instance (the arena
    only grows), so the document after an `apply` and an `undo` may hold
    more formulas than before it; the state is the same otherwise.
- **`Rule::Open`** marks an open goal of a proof in progress and appears
  nowhere else: the checker refuses it first ("an open goal"), `Drawn`
  draws it without a bar or label, as the linear `Rule::Open`.

## Transformations (`negative.rs`, `interpolant.rs`)

Public as `ordinary::negative` and `ordinary::interpolant`; the command's
`seq translate` and `interpolate`, the bindings' `ordinary_translate`
and `ordinary_interpolate`. Each claim is computed, never assumed:
what fails it is `Error::ClaimFails` (kind defect, code `claim_fails`).

- **A negative translation is a table** (`negative::row` per
  translation and node: the connective kept or `A ∨ B` as `¬(¬A ∧ ¬B)`,
  and the `¬` before it; `sides`: the `¬` before each hypothesis and
  before the formula right of `⊢`), applied once down the arena to the
  nodes the sequent reaches (`reached`, a pass from the last node), so
  nothing recurses. Several formulas right of `⊢` are their disjunction,
  the first leftmost, built in a copy of the source arena; none stays
  none (`Γ ⊢ ⊥` in the target). Kuroda's translation is propositionally
  `¬¬` before every formula of the sequent, Glivenko's before the right
  side alone; the rows for `∀` and `∃` (Kuroda's `¬¬` after every `∀`,
  Gödel and Gentzen's `∃x A` as `¬∀x ¬A`) go into `row` at first order.
  The result is bounded by `limits.occurrences` (`bound`, now in every
  build).
- **The targets** (`Translation::reaches`): Kolmogorov's and Gödel and
  Gentzen's reach minimal and intuitionistic logic, Kuroda's and
  Glivenko's intuitionistic logic only (`false -> a` is classically
  valid and `¬¬(⊥ → a)` not minimally), none classical logic;
  `Error::NegativeTarget` otherwise, before anything is decided.
- **`compare`** decides the sequent classically and its translation in
  the target, each by its logic's default method, under the caller's
  search options, limits and one stop; both verdicts known and different
  is `ClaimFails`, either unknown `Agreement::Unknown`.
  `every_translation_keeps_the_verdict` and
  `generated_sequents_keep_their_verdicts` (200 generated sequents, every
  translation) pin it.
- **Maehara's method reads the rule table of the checker**:
  `check_inference` returns, per premise, the formulas the rule adds to
  the conclusion's context (`derivation::Added`), so the interpolant
  has no table of its own to drift. Going up (`maehara`), each premise's
  formulas get their halves: an added formula the principal formula's,
  every other one drawn from the conclusion's context by side and
  formula, from one pool for a split and from the whole context per
  premise for a shared one (`premise_parts`: split tried first, shared
  where a draw fails). Equal formulas are interchangeable, so which
  occurrence of a repeated formula is drawn does not matter.
- **LJ's halves are exchanged** where a premise's formula right of `⊢`
  would be in the first half: the first premise of `→L` and of LJ's
  two-premise `¬L`, and the premise of the one-premise `¬L`, on a
  hypothesis of the first half. Going down, an exchanged premise's
  interpolant enters negated (one premise) or as the antecedent
  (`I₁ → I₂`); otherwise a leaf gives `⊥`/`⊤` by the principal's half
  (an axiom `⊥`, `⊤`, `A` or `¬A` by its two sides' halves), one premise
  its own, two the disjunction (principal in the first half) or the
  conjunction. `Build` drops `⊤` and `⊥` where the logic proves the
  result equivalent: in minimal logic only `⊤`, since `⊥ ∨ A ⊢ A` needs
  ex falso.
- **The check** (`checked`): the interpolant's atoms in both halves
  (`atoms_of`, a pass down the arena), both halves within
  `limits.occurrences`, then both decided by the logic's default method;
  a half not valid is `ClaimFails`, a half unknown leaves
  `Interpolant::is_checked` false (the command's exit 3).
  `the_check_refuses_what_is_no_interpolant` plants a foreign atom and a
  `⊤` that makes the second half fail; `generated_sequents_interpolate`
  splits 200 generated sequents at random in every logic.
- **Which derivation**: `interpolate` decides by the caller's method
  (by default the SAT solver, so a sequent that is not valid gets its
  countermodel at once), and where that derivation has a cut (LJT, the
  SAT solver's read-back) or there is none (a clausal proof) and no
  method was named, decides again through the embedding, whose read-back
  has no cut; a named method's cut is `Error::NotCutFree`. The embedding
  unknown leaves the method's verdict with no interpolant; the embedding
  refuting what the method proved is `Error::ReadBack`.
- **At first order** the tables gain the quantifier rows, and
  Maehara's method needs the eigenvariable condition kept in the
  interpolant: a term of one half only is quantified over.

## Normal forms (`normal.rs`)

Public as `ordinary::normal`; the command's `seq normalize`, the
protocol's `ordinary.nnf`, `cnf`, `dnf`, `tseitin` and
`plaisted_greenbaum`. Each promise is checked, never assumed: a claim
decided not valid is `Error::ClaimFails` (kind defect).

- **Two tables, no recursion.** `law` (per phase and node: which law
  applies at a node's root) and `apply` (what each law makes) are the
  rewriting; `definition` (per connective and direction) is the
  encodings'. A change of a law is a change of these tables alone.
  Every walk is a pass over the nodes a sequent reaches, collected by a
  stack and sorted by id (`Passes::reach`; the arena's order puts
  operands first), or a stack of its own (`in_form`, `nnf`).
- **A step rewrites every occurrence of its subformula at once**: equal
  formulas are one node, so `substitute` maps the node wherever it
  occurs. The redex is the leftmost outermost of the phase (`redex`:
  `found` marks a node at or above a redex, then a descent from the
  first root that has one). Phases in order: arrows, negations,
  constants (CNF and DNF only; NNF keeps `⊤` and `⊥` but rewrites `¬⊤`
  and `¬⊥`), distribution. Within a phase no law makes a redex of an
  earlier phase, so the result is in the form (`in_form` checks it;
  failing is `ClaimFails`).
- **Every step's ids index the result's arena**, which is the input's
  arena grown: the input's ids stay valid there, and every intermediate
  formula is kept, so the JSON's step numbers index the result's
  `formulas` (the sequent's writer writes the arena whole). A step
  stores the input's formulas after it (`left`, `right`) and the clauses
  it `added`; the sequent after step k is `left` with the clauses of
  steps 1 to k, so storage is linear in the steps for an encoding too.
- **The bound** is `limits.form_occurrences` (default 10 000, a
  teaching scale like `table_atoms`): every sequent on the way, the
  input included, unfolded, else `Refusal::Form`. Each pass is charged
  to `limits.work` (a unit a node reached), the steps and new nodes to
  `limits.memory_bytes` (`NODE_BYTES`, `STEP_BYTES`, estimates), and the
  stop is asked once per step. Steps are linear in the bound (each phase
  changes the unfolded size one way: arrows and distribution grow it,
  constants shrink it, De Morgan and double negation move a measure of
  the negations' depth), each costs a pass over the reached nodes, so a
  form near the bound costs about the bound squared: 1.4 s for 512
  clauses of 17 literals in a debug build.
- **The encodings** move each formula right of `⊢` left negated (a
  `Negate` step), then name every compound subformula the negation
  reaches, innermost first (`order` ascending), by `t1`, `t2`, … (`prefix`
  adds `_` until no input atom is the prefix and digits). A subformula
  whose current form is a literal (`¬t` after its operand was named) is
  not named. Plaisted–Greenbaum's polarity is computed on the input's
  nodes (`polarities`, the classical encoder's rule: flipped under `¬`
  and left of `→`, both under `↔`); full Tseitin takes both directions.
  A clause with `⊤` (or `¬⊥`) is dropped, `⊥` (or `¬⊤`) removed, an
  empty clause is `⊥`; the input's formulas, literals at the end, are
  unit clauses: a negated constant is rewritten (`NotTrue`, `NotFalse`)
  and `⊤` left out (`TrueHypothesis`), each a step, so that the sequent
  after the last step is the result (the generated test holds every
  form to that).
- **The checks** (`NormalForm::claims`, `check`): equivalent forms
  decide `⊢ ∧ᵢ (Aᵢ ↔ Aᵢ')`; encodings check `fresh` (each definition's
  atom new, its formula over the input's atoms and earlier new atoms,
  so every assignment of the input's atoms extends uniquely; one pass
  ranks the atoms, one gives each formula its highest rank, so it is
  linear: an `atoms_of` per definition was cubic on a deep chain, 15 s
  at depth 1 500) and decide
  `N, t ↔ G… ⊢ ∧C` (Extends) and `C ⊢ ∧N` (Restricts), `N` the input's
  negation. Together: the clauses are satisfiable exactly when `N` is,
  so the result is valid exactly when the input is. The claims copy the
  input into the result's arena by atom names (`transfer`), so a result
  from elsewhere (the protocol's output, a test's) is judged as it is.
  They are decided by `Method::Sat` (its clausal proof checked) where
  `sat` is on, else the classical default; the classical default with
  `sat` would also run the embedding for a derivation, which costs the
  embedding's search.

## Resolution and DPLL (`resolution/`)

Public as `ordinary::resolution`; the command's `resolve` and `check`
of a refutation or (`--dimacs`) of an outside proof, the protocol's
`ordinary.resolve`. Teaching scale: bounded by `Options::clauses` and
`steps` (2 000 and 100 000), unknown past them.

- **The checker is the trust** (`Refutation::check`): each resolvent's
  two clauses come before it, `first` holds the atom and `second` its
  negation, the clause is exactly `first ∖ {p}` joined with `second ∖
  {¬p}`, and some clause is empty. Only the pivot's own literal leaves
  each clause: a parent that holds both `p` and `¬p` keeps the other one.
  Dropping both from both (the first version) let `{a ∨ ¬a}` be "refuted"
  in one step; the panel's counterexamples and argument members found it
  (`a_tautology_is_no_way_to_the_empty_clause`). `resolve` (by hand) and
  saturation's own `resolve` use the same rule.
- **`decide` checks before it answers**: a refutation by `check` and
  against the input set, a model by evaluating every clause
  (`Clauses::falsified`); a failure is `ClaimFails`, a defect. The
  searches are untrusted.
- **Saturation**: shortest clause first, tautologies and clauses a used
  one subsumes left out (forward subsumption only), resolution only on
  pairs with exactly one clashing atom (two give tautologies). Saturated
  without `□` means satisfiable, and DPLL then finds the model, whose
  verdict (or unknown) is the answer.
- **DPLL** is classic: no learning, every decision's second branch tried
  (chronological backtracking), units first, then pure literals, then
  the first atom of an open clause decided true. Its refutation is read
  off going back (`back`): a conflict clause resolved with the unit
  reasons above the decision, and at a decision both branches' clauses
  combined (`Reason::Flipped` keeps the first branch's). The invariant:
  the clause carried is false under the trail that remains. Every scan
  is over all clauses per step (teaching scale).
- **Literals** are `2·atom + sign` in a `u32`: a set holds at most
  `Literal::MOST_ATOMS` atoms, `Literal::new` answers `None` from there,
  and the crate's `Literal::of` is used only for atoms of a set (the
  integers member found the wrap: `resolve` with atom 2³¹ aliased atom 0).
- **Memory**: `from_dimacs` refuses a header past `memory_bytes` at
  `ATOM_BYTES` an atom before making names (a 32-byte header once held
  7 GB); `decide` charges the set and per-atom arrays first, saturation
  its clauses, DPLL its events and resolvents; the LRAT reader charges
  each clause twice (the step and its numbering).
- **Outside proofs** (`outside.rs`, feature `sat`): the readers only
  turn text or bytes into `drup::Step`s; every addition is checked by
  RUP, deletions only weaken, so a misread can lose a valid proof but not
  accept a wrong one. A binary DRAT proof starts with `a` or holds a
  control byte among its first 64 (drat-trim's rule; letters of a
  comment are text). LRAT hints are ignored; RAT steps fail.
- **Wire level 22** (`serialize/resolution.rs`): a refutation is read
  and written (`check` reads one), a clause set decided is written only;
  literals as DIMACS writes them, clauses numbered from 0, atoms from 1.
- **Where quantifiers go**: first-order resolution needs unification on
  `sequents::fo` terms and a TPTP `cnf` reader (`plan/later.md`); the
  types here are propositional.

## Decisions

The author's answers for the release (`plan/notes/api.md` §14), which
the fixes implement and later rounds judge against. Where a bullet above
still describes code that a decision changes, the decision holds, and
the commit that lands it rewrites that bullet.

- **A TPTP file with several conjectures is refused**, naming the
  second, as the LLTP reader refuses one (`core-inputs.md`).
