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
  `export::rocq::ordinary`; `interactive.rs` (behind `interactive`): the
  goal state `Interactive`, below. Its JSON forms and those of the
  sequent, the derivation and the outcome are in
  `core/src/serialize/ordinary.rs`, wire level 8 (`wire::ORDINARY`).
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
  application. Minimal logic's `⊥` is Rocq's `False`, so a certificate
  of minimal logic proves the intuitionistic statement only (minimality
  rests on the checker).
- **Deciding** is `decide(&sequent, &ordinary::Options, &search::Options,
  &limits, stop)`, which the ordinary journey runs, by
  `Options::method()`: the method given, else the embedding when a
  translation is given, else `Method::default_for(logic)`, LJT for
  intuitionistic and minimal logic and the embedding for classical
  logic, chosen by the ILTP run of `plan/reports/54-ljt-engine.md`
  (`bench/defaults/ljt-iltp.csv`). With no method given, LJT gives way
  to the embedding where `gives_way` says so: an unknown at a bound
  other than `Reason::Stopped`, or an `Error::Refused` other than the stop, never an unsupported input (the
  derivation past `derivation_bytes`, which SYJ201+1.004 and .005 reach
  in milliseconds and the embedding decides at 2 s). The outcome's
  `search` (`Search::Embedding` with the translation and the image's
  outcome, or `Search::Ljt` with LJT's statistics) says which decided,
  and its JSON writes `engine` `ljt` and no `translation` for LJT. The
  command still runs the steps itself (its race and its output, the
  dispatch by `gives_way` in `cli/src/ordinary.rs`), with its defaults
  (no copy bound, the time limit), affine classical mode for `Affine`,
  intuitionistic mode otherwise. A proof whose read-back a bound or the
  stop refuses is that refusal from `decide`, never a verdict. The
  default translation for intuitionistic and minimal logic is
  `Translation::DEFAULT_INTUITIONISTIC`, chosen by the ILTP run
  (`plan/reports/25-ordinary-logic.md`). The goal state's `close` and
  `hint` pass `Method::Embedding`: its steps take no cut.
- **Where quantifiers go**: `Node` gains `Forall`/`Exists` with a bound
  variable and `Atom` gains terms; the pattern table gets their rows
  (Girard: `∀` as `∀`, `∃` as `∃!`), and the read-back's tags carry no
  more than the node.

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
- **`Derivation::term` assumes a checked derivation or a view of one**
  (its `# Panics`): checking first would refuse the admissible view,
  which `term` reads too, and a session's derivation with open goals; a
  derivation read from JSON is checked before its term is read.
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
the derivation, `model.rs` the Kripke model and its check. Its search,
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
- **The model is data** (`Model`, `World`: atoms by index, `⊥` forced in
  minimal logic only, edges to later worlds), kept on `sat::Outcome`,
  not on `ordinary::Outcome`. `refutes` computes forcing for every
  formula of the arena at every world, the last first, an implication
  from its local value and the worlds directly above; `¬A` is `A → ⊥`,
  so in minimal logic a world forcing `⊥` forces `¬A` too (the first
  check read it as intuitionistic negation and refused minimal models).
  Its cost is `MOST_CELLS` (worlds and edges times formulas), refused
  past it as memory. A model that fails is `Error::Countermodel`, a
  defect; a model a user gives would need an `invalid` kind.

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
  own. Step 70's outside proofs will need an error of kind `invalid`.
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
- **The JSON form** writes `"clausal_proof": {"variables", "clauses",
  "steps"}` in place of `derivation`, at wire level 10 (`wire::CLAUSAL`):
  a reader of level 9 would take a valid outcome without a derivation
  for a broken one. An outcome with a derivation keeps level 8 or 9 and
  its bytes.

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
- **`Rule::Open`** marks an open goal of a proof in progress and appears
  nowhere else: the checker refuses it first ("an open goal"), `Drawn`
  draws it without a bar or label, as the linear `Rule::Open`.

## Decisions

The author's answers for the release (`plan/notes/api.md` §14), which
the fixes implement and later rounds judge against. Where a bullet above
still describes code that a decision changes, the decision holds, and
the commit that lands it rewrites that bullet.

- **A TPTP file with several conjectures is refused**, naming the
  second, as the LLTP reader refuses one (`core-inputs.md`).
