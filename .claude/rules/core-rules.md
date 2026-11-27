---
paths:
  - "core/src/rules/**"
  - "core/src/serialize/rules.rs"
  - "core/tests/rules.rs"
  - "core/tests/natural.rs"
---

# deduxit core: calculi given as data

Loaded, beside `core.md`, when a file of `core/src/rules/` is read or
changed: the rule file, the generic derivation and its checker. The
design, with the reasons and the alternatives weighed, is
`plan/notes/rule-schemas.md`; this file holds what the code relies on.

- **Layout.** `mod.rs`: the public names, `FORMULA`, the rule files'
  text `LK`, `LJ`, `LM`, `LL`, `TABLEAU`, natural deduction's `NJ`,
  `NK`, `NM`, `NK_DNE`, `NK_LEM`, `NJ_DEFINED`, `NK_DEFINED`,
  `NM_DEFINED` (`calculi/*.json`) and
  `SCHEMA` (`calculus.schema.json`); `terms.rs`: `Terms`, the
  hash-consed arena of a derivation's terms; `calculus.rs`: `Calculus`,
  read from a `CalculusForm` and compiled (`Rule`, `Shape`, `Entry`,
  `Find`, `Equation`); `derivation.rs`: `Derivation`, its flat storage,
  `Node` (what `push` takes), `NodeRef`, `Step`, scopes; `check.rs`:
  `Derivation::check`; `print.rs`: `Shown`, the printer; `text.rs`: the text syntax; `interactive.rs`: the session (feature `interactive`); `natural.rs`:
  `Derivation::natural`, natural deduction from LK, LJ and LM;
  `search.rs`: the session's `close` and `hint` in natural deduction
  (features `interactive` and `parse`); `ordinary.rs`:
  `Derivation::from_ordinary`; `tableau.rs`: `Derivation::from_tableau`
  (feature `interactive`). The
  JSON forms are `core/src/serialize/rules.rs`, both at wire level 15
  (`wire::RULES`).
- **The module needs `serialize`**: a calculus is made only by reading
  a rule file (`Calculus::from_form`, through `wire::Readable`), so
  without serde there is nothing to check against. `Calculus` keeps its
  form and writes it back, so a file reads and writes as the same JSON
  value.
- **A rule file is strict** (`deny_unknown_fields` on every form): it
  is written by hand and is the trusted base of every verdict, and a
  misspelt `premises` would read as an axiom. A key added later raises
  the wire level. A derivation is a data form and ignores keys it does
  not know.
- **Every name of a rule file is declared**: a string in a pattern is a
  declared variable, a declared context or a connective, else the file
  is refused (`Compiler::name`). Never add a fallback that reads an
  unknown name as a variable.
- **A pattern is nested `DEPTH` (64) deep at most**: `PatternForm`'s
  reader counts the depth itself (`Nested`), so the recursions over a
  pattern (the reader, `Serialize`, `Compiler::pattern`, the drop) are
  bounded whatever deserializer is used. Everything after the compiler
  walks flat arenas: a rule's patterns lie operands first, each written
  member's nodes together (`Item::first..=root`), which `instance`
  relies on.
- **A derivation knows names only** and is read without its calculus:
  `Terms` holds connective and sort names, `Derivation` rule names.
  `check` resolves them once (`Checker::terms`: `global`, `local`, the
  sort of every term) and refuses a term the signature does not allow,
  used or not.
- **Equal terms are one id** (`Terms::add` through `find`, a chain per
  hash in `latest`/`earlier`): the checker compares members as numbers
  and sorts them for multiset equality. The JSON reader interns as it
  reads, so a document that writes a term twice reads as one.
  `Terms::find` builds nothing: `instance` looks an instantiated
  pattern up, and a term the arena lacks is in no sequent.
- **How a written member is found is fixed when the file is read**
  (`Compiler::rule`, `Find`): `Start`/`End` where the zone's entry
  count or a sequence with at most one context forces the place;
  `Position` for every other member of the conclusion and for a
  premise's member with a variable not bound by then or in a sequence;
  `Value` for a premise's bound member in a set or multiset. Within a
  zone the checker resolves the `Position`, `Start` and `End` members
  before the `Value` ones: a `Value` member binds nothing, and taken
  first it could take the place a later position names and refuse an
  instance (the panel of 60b). A context under a connective that is
  empty maps to nothing whether or not a term has that connective. The order
  is the conclusion's zones, then each premise's, and `Rule::positions`
  counts the `Position`s; a node gives exactly that many (`at`).
  Changing the order or the rule changes every stored derivation: it is
  a wire-level change.
- **Contexts** (`Checker::equations`, `sets`): multiset and sequence
  zones are solved in the order `Compiler::equations` found (each
  equation has at most one context not yet defined; a rule with none
  such is refused, "cannot be told apart"). That order is found by a
  worklist (each equation counts the contexts it waits on, each context
  lists its equations), and each `Equation` keeps its segment's
  contexts, so neither the compiler nor the checker rescans a zone: a
  rescan per pass and a scan per segment were quadratic in the rule
  (the panel of 60b: 2.9 s for a chain of 8 000 premises, 22 ms now). set zones by the largest
  value of each context (what all its zones share), then a cover check.
  A context belongs to one discipline and one member sort
  (`Compiler::contexts`), and stands at most once in a zone.
- **Given its positions, a rule node passes exactly when it is an
  instance of its rule.** The argument: positions and forced places fix
  which member each pattern matches; first-order matching then has one
  answer; a multiset or sequence equation with one unknown has one
  solution; for sets every constraint is "the context is within this
  zone" or "this zone is covered", both monotone, so the largest
  contexts satisfy them if any do. The panel reviews it in 60b.
- **Scopes** (`Checker::scopes`): a premise lies in the node's scope or
  one around it; a premise of a rule that discharges may instead lie in
  a scope directly inside one of those, and then every hypothesis node
  of that scope is an instance of one of the premise's `discharges`
  (compared per zone by `sort_by`: as written in a sequence, sorted in
  a multiset, sorted and deduplicated in a set; a set compared as a
  multiset refused an instance, the panel of 60b). A hypothesis of the
  root scope is an assumption. The invariant: a node's sequent follows
  from the hypotheses of its scope and the scopes around it. It is a
  statement per node, so `check` also asks that the root lie in the
  root scope: a last node inside a scope passed every node's check and
  showed `b` from no assumption (the review of 60a).
- **Freshness** (`Checker::conditions`): the variable is bound to an
  atom that occurs neither in the node's sequent nor in a hypothesis of
  its scope or a scope around it, and that no other variable of the rule
  is bound to (checked last, so a name in the conclusion is said as
  such). Only a variable bound from a premise alone can share it:
  `Ser`'s `x`, with which `w1 R w1` for a new `w1` passed, "seriality
  gives a world that sees itself" (sound for what the calculus derives,
  which the Rocq library's `g3` proves for the wider rule, but no step a
  session should take). `occurs` walks the term DAG once per
  walk number (`seen`), skipping ids below the atom's, and counts every
  member it is handed: a node of a `fresh` rule looks through every
  hypothesis in force, which was quadratic and uncounted before.
- **Bounds.** `check` estimates what it holds before it allocates
  (`Refusal::Memory`); every loop goes through `Meter::tick` (a node, a
  member, a term or scope walked: `limits.work`, saturating; the stop
  every `POLL` units and at every node); nothing recurses. The readers
  bound the terms by `limits.occurrences` and every index by `u32`
  (`Refusal::Index`). `Derivation::push` checks every index, so the
  checker indexes without a test.
- **Two more rule files and their cross-checks**: `LL` (one-sided
  linear logic, propositional, no cut, no Mix, not affine; `~` over an
  atom with `atomic` is the dual atom; promotion keeps `["?", "Γ"]`) is
  checked on the one-sided, uncompacted derivations of the linear
  suite's proofs of 28 sequents (hand-written and the families' smallest
  provable classical or intuitionistic instances), by a converter in
  the test (`from_linear`), which gives the axiom the positions of its
  atom and its dual. `TABLEAU` (Smullyan's signed tableau; one `set`
  zone of sort `signed`; an expansion keeps the formula it expands, as
  a tableau's branch does) is checked on the tableaux of the method
  `tableau` and those `Tableau::of` reads off the embedding's
  derivations, by `Derivation::from_tableau` (feature `interactive`,
  `tableau.rs`): a node per tableau node, its branch's signed formulas
  once each, children as premises, an open branch an open goal. A
  closed tableau passes and an open one is refused. 56a closes a branch
  on any formula, so `×` has no `atomic`. `F→₁` and `F→₂` take one result of `F→`
  alone, as `T∧₁` and `F∨₂` do (Smullyan's rule A). `T↔′` and `F↔′` expand an equivalence by
  its sides' values, as textbooks draw it (no LK rule maps to them, so
  `from_tableau` never writes them); `ordinary::WrittenTableau::review`
  checks a student's tableau as a derivation in the file
  (`core-ordinary.md`). Affine and Mix modes, cut and
  quantifiers have no rule file yet.
- **The cross-check** (`core/tests/rules.rs`,
  `the_rule_files_agree_with_the_ordinary_checker`): derivations of
  every method in the three logics, and each broken once per inference
  and guard, must get the same verdict from `ordinary::Derivation::check`
  and from `from_ordinary` then `check`. `from_ordinary` judges only
  what the generic form cannot say (the root's statement, a principal
  on the wrong side, a premise's order) and picks between a rule's
  sharing and splitting names by the premises' sizes, which cannot both
  fit unless the context is empty. It also refuses an equivalence rule
  whose two implications the arena lacks, as the ordinary checker does
  for `↔L₁` and `↔L₂` too, which a rule file cannot ask; no engine
  makes such an arena, so the test has that document by hand.
- **Natural deduction's files** (`calculi/n*.json`, written once by a
  script and kept as data, as LK's were): a judgement is one formula
  (one multiset zone of exactly one member, so no rule needs a
  position), a hypothesis a node, discharge by scope; every elimination
  takes its major premise first; the classical rule is `RAA` (Prawitz's
  `⊥c`), `¬¬E` or `LEM`, minimal logic drops `⊥E`, and a `-defined`
  file has no `¬`. Each file's `description` names its sources; change
  them only with a source read. The cross-check (`core/tests/natural.rs`)
  holds "derivable in the file exactly when the ordinary suite says
  valid in its logic" both ways: every rule, read as the sequent
  `⋀(discharged → premise) ⊢ conclusion` with its variables as atoms, is
  valid in the file's logic and in no weaker one unless it needs none
  (`needs`: the classical rules classical, `⊥E` intuitionistic), which
  with the scope invariant makes every accepted derivation valid; and
  the ordinary suite's derivations (embedding, LJT, tableau; the fixed
  list and 300 generated sequents, each logic) translated into every
  file whose logic is at least as strong pass and conclude the sequent's
  formula from hypotheses of it. The translator is in the test: open
  leaves are named by formula, so a left rule substitutes a derivation
  for every open leaf of a formula and no occurrence is tracked; LK
  becomes `⊥` from `Γ, ¬Δ` and then the classical rule. Breaking each
  small derivation once per node (formula, rule, premise order, scope)
  and deciding what every accepted break shows checks the first
  direction on samples; `a_weaker_file_refuses_what_needs_a_stronger_rule`
  pins that a proof needing `RAA`, `¬¬E` or `⊥E` is refused for the rule.
  The ILTP part is ignored (the library is not in the tree) and leaves
  the tableau method out, which held several times its memory bound on
  SYJ202+1.004.
- **Natural deduction from sequent proofs** (`natural.rs`,
  `Derivation::natural`): the shipped files are told by name and rule
  list (`Profile::of`, `FILES`; a file of that name with other rules is
  not one, and the result is checked against the file in any case).
  The translation is the test's converter (`core/tests/natural.rs`,
  which stays as an independent second one) made a loop over explicit
  stacks: a left rule replaces every open leaf of its formulas by an
  elimination from the principal hypothesis, so an elimination's major
  premise is a hypothesis or an elimination. Two places replace a leaf
  by an introduction (`iff_left`'s implications, LK's `∨R`/several
  right formulas' negations), and where that leaf is the major premise
  of `→E`/`¬E`, `replace` reduces the detour at once (the introduction's
  premise with its discharged leaves replaced by the minor premise);
  `iff_right` takes an `→I`'s premise at once. Without the reduction
  `a ∨ b ⊢ b ∨ a` into NK had a detour (`translations_are_normal_and_checked`
  holds every file to normality: the major premise of `∧E`, `∨E`, `→E`,
  `¬E`, `↔E` is never the conclusion of its introduction). The
  classical rules (`¬¬E` after `¬I`, `LEM` with `∨E`) are not counted
  as eliminations. The source is checked first and the tree's unfolded
  size charged to the memory bound before the derivation is written.
- **The session's search** (`search.rs`, `Interactive::close`, `hint`):
  the goal with the lines in force (`lines`, hypotheses and derived
  lines) as an ordinary sequent in the file's logic, decided by the
  embedding alone (its derivations have no cut, which the translation's
  `Cut` arm would substitute and could leave a detour), translated and
  checked, then taken by `apply` with the premises' formulas
  (`Apply::premises`, crate-private: `bind_premises` matches each
  premise's first written member, so what the goal leaves open is bound
  without text) and `assume` for every hypothesis leaf, each step
  checked by the session; a refused step undoes every step before it.
  A graft is many steps, which `undo` takes back one at a time (the page
  groups them).
- **Hints follow one proof** (`hint`, `path_to`, `found_at`): the
  search starts at the session's root goal (the lines in force at a
  closed node read as at an open one) and its proof is followed down
  the steps taken while each is the proof's (rule and premises' first
  members, compared by term in a clone's arena); where the session left
  it, the search starts again from that node, or below it where the
  proof's own root already differs. The search is pinned to one thread
  so that a node's proof is the same each time. Each goal searched alone
  went round for ever in the classical files: on `⊥` under `¬P` the
  proof is `¬E` with `P`, on `P` it is `⊥E` or `RAA` back to `⊥`
  (the fresh review's witness, Peirce's law in NK;
  `following_classical_hints_proves_a_goal` walks the four classical
  files). The translation helps on its own too (`concluded`): on a goal
  `⊥` no classical rule (each `¬⊥` leaf is `¬I` over `⊥`, which the
  elimination it meets reduces away), and where `¬one` is in force the
  derivation of `one` that `¬E` refutes it with, else `⊥E`. `hint`
  answers `assume` where the goal is a line in force (the embedding's
  derivations are η-long and take a compound line apart first);
  otherwise the proof's rule with `given`, the names `apply` would ask
  for (`StepNeeds`) bound by the premises (`premise_bindings`), as text.
- **What the reader refuses beyond names and sorts**: a connective
  over a context that does not take exactly one argument (index 0 of
  an empty list panicked), a variable named twice in `fresh` or
  `atomic`, and a zone's key written twice in any sequent (`Once`; a
  plain map kept the last, so a conclusion `{"s": ["⊥"], "s": ["A"]}`
  read as `A`). It refuses a file whose rules would compile into more
  than `limits.memory_bytes` at `SLOT_BYTES` per zone of each sequent
  (`Refusal::Memory`): a thousand premises over a thousand zones is a
  short file and was a grid of 56 MB. Names, sorts and zones are looked
  up in hash maps, here, in `Terms::atom` and in `Checker::terms`;
  linear scans made n sorts cost n².
- **The memory estimate** (`Derivation::check`) counts per term 32
  bytes (its sort, its walk mark, its connective's or sort's index, and
  a place on the freshness walk's stack, which marks a term as it
  pushes it and so holds each once, its vector up to twice its length),
  per connective, variable and context of the calculus its slot in the
  checker's tables, per node 8, per sequent of the widest rule its zones' slots and the
  widest sequent's copies, every context at the widest sequent, and
  each rule's pattern stacks and discharged instances. The panel of 60b
  found three structures it left out (the walk's stack, which grew by
  the arity per level; the contexts; the slots of empty zones), each
  past a 1 MiB bound by tens of megabytes; the fresh review of 60b two
  more (the stack's doubling, 1.5 times the bound, and the calculus's
  tables, 1.6 MB under 4 KiB).
- **No control character in a file's printed texts**: the calculus's
  name, every connective's name and symbol, every zone's name and mark
  and every rule's name (`from_form`), since the drawings take U+0001 to
  U+0005 as their own marks and LaTeX sets none of them.
- **A connective takes `ARITY` (255) arguments at most**: the number
  comes from the file and sizes a vector, and 2³² of them reserved
  16 GiB under a 1 MiB memory bound.
- **The printer** (`print.rs`, `Calculus::show`, `show_sequent`,
  `Shown`): a loop over an explicit stack, written through `Bounded`,
  which takes `Shown::LIMIT` (4096) characters unless `limit` says
  otherwise and ends a cut text with `…`. A term is a DAG and may
  unfold far past its arena, so the bound is what keeps the time too:
  the stack holds at most about 2·arity items per character written.
  A connective without `fixity` is prefix at arity 1, infix at 2, a
  function otherwise; precedence defaults to 0 for infix and the
  highest for the rest; an operand is bracketed by precedence and
  associativity (`Shown::bracketed`). An error writes the failing
  node's sequent within 200 characters (`check::SHOWN`).
- **The text syntax** (`text.rs`, `Calculus::parse_term`,
  `parse_sequent`, crate-private `parse_list`): what `show` writes reads
  back. A lexer by the calculus's names, symbols and marks (a word is
  read whole, a symbol by longest match; `ASCII` spellings such as `->`
  and `|-` only where the calculus declares the target and not the
  spelling, so LL's `~` stays its own), then a shunting-yard over an
  explicit stack (prefix, infix by precedence and associativity, a
  chain of a non-associative connective refused, postfix, `f(a, b)`),
  then the sorts from the root down, atoms made of the sort their place
  asks. A text's tokens count against `limits.occurrences`. `[]` and
  `<>` are the ASCII spellings of `□` and `◇`, so a labelled modal
  sequent reads as the modal suite's text does.
- **The session** (`interactive.rs`, `rules::Interactive`, features
  `interactive`): a top-down arena (`Goal`: sequent, `Done`, premises,
  scope, `opened`), scopes (`Scope`: parent, owner node and premise,
  the hypotheses it offers; the root scope's are the assumptions) and
  the history of goals closed. `apply` plans a backward step (`plan`:
  the conclusion's members at the positions given, numbered across the
  zones, matched; given values parsed; each conclusion zone's contexts
  solved by its discipline, a set's at their largest, two unknown in
  one segment asked for by name, `Error::StepNeeds`, before anything
  else; fresh variables not given get a new atom, `new_atom`, named
  after the variable and unused anywhere; premises built with
  `Terms::apply`, a set zone deduplicated, a premise's position members
  located at the place each was built, a set's first copy after its
  duplicates go; locating them by value took the wrong copy of a
  sequence's equal members), then `commit` checks every premise's
  shape before it pushes anything (a refusal in the loop once left
  orphan goals that no `undo` removed), appends the premises (a
  discharging premise in a new scope owned by the goal), sets the goal,
  and **runs the checker on that node** (`verify`: the whole session
  converted by `derived`, then `check_node`); a refusal is rolled back
  (`back`), so no state holds a step the checker refuses. `assume`
  closes a goal by a hypothesis its scope chain offers and re-checks
  every closed node (`verify_all`, `check_open`): the owner's scope check
  verifies the discharge, and a rule's `fresh` condition reads the
  hypotheses in force, which the new one joins (a name given as new
  and then assumed finished into a refusal when only the owner was
  re-checked); an assumption (root scope) is the session's own. `undo` relies on the
  suffix property: a step's premises and scopes are the last the
  session holds. `rules` tries every rule at every choice of the
  conclusion's position members (at most `RULE_CHOICES`) on a clone.
- **Every rule with its status** (`Interactive::candidates`; `Candidate`,
  `Status`): `rules` is `candidates` less the refused ones. Each choice
  of members is a step on a clone (`plan`, then `commit`); a rule that
  no choice makes legal is one candidate with `at` empty and the first
  choice's error, or `StepFault::NoPlace` where there is no choice. A
  bound refusing `commit` refuses that step alone (as `rules` dropped
  it); a bound refusing `plan` ends the call, as before.
- **`step_refused` carries a `StepFault`** (`errors/step.rs`), one
  variant per reason, its `Display` the sentence it had; the checker's
  own reason (`StepFault::Check`) is still the checker's sentence,
  since `Derivation::check` writes its faults as text.
- **Forward steps and lines** (`Interactive::derive`, `lines`, `cite`,
  `Derive`, `LineId`, `Undone`): a line is a hypothesis a scope offers
  or a node a forward step derived (`Goal::line`), numbered from 1 by
  `line_places` in the order the steps made them (a scope enters with
  the premise its step opened in it, a line with its own node; both
  keys are node places, which grow with the steps), so a number never
  changes and the JSON needs no table of them. A line is in force at a
  goal when its scope is in the goal's scope chain (`chain`), which is
  also what makes a citation of a closed subproof's line a refusal. A
  forward step takes rules without contexts and without a discharging
  premise (those apply backwards, which opens the subproof, as Jape's
  box proofs do), matches each cited line against its premise as
  written, pushes the line node and then a hypothesis node per cited
  hypothesis (opened in the line's scope, lying in the hypothesis's),
  and runs `verify_all`, since a hypothesis node joins a scope whose
  owner must check it. A derived line cited again is the same node, so
  the session and its derivation are a DAG: `derived` emits the lines
  first (an earlier line before a later that cites it) and then the
  tree, a goal closed by a line (`Done::Line`) taking that line's node,
  which `derivation_goals` skips. The reader checks that a line is no
  root, does not discharge, is cited only by later lines, that a goal
  closed by one equals it and has it in force, and the history's
  suffix (a forward step's nodes are the line and its hypothesis
  copies, last; a goal closed by a line after the line's step). Such a
  session is written at wire level 18 (`wire::FORWARD`, the keys
  `line` and `cites`); one without stays at 17.
- **Forward steps in a sequent calculus** (`Interactive::extend`):
  lines are nodes, which a calculus with contexts never has, and
  `derive` refuses every rule with contexts. `extend` takes a rule
  whose conclusion writes members of the first zone alone (a right rule
  is goal-directed: `R¬`'s premise adds only before `⊢`, and passed the
  other tests, the review of 66a), plans its backward step on the goal
  (`plan`, the members by position as `apply` takes them) and takes it
  only where its one premise is the goal with members added to the
  first zone and every member of the goal kept (compared as multisets);
  a rule with no or two premises, one that discharges, drops a member,
  adds to another zone or adds nothing is refused with that reason.
  `forward_steps_are_the_rules_that_add_hypotheses` pins the set per
  system: `L∧`, `L□`, `L◇` and the frame rules. The node is `apply`'s, so the derivation and
  the JSON are unchanged. It reads forwards because the labelled
  calculus keeps every principal formula: `L□`, `L∧`, `L◇` and the
  frame rules add hypotheses. The command's `derive` takes it in a
  calculus that declares contexts (`Calculus::contexts`), citing member
  numbers; `random_labelled_steps_keep_a_checked_session`
  (`core/tests/rules.rs`) walks the fifteen labelled calculi.
- **The Fitch proof** (`fitch.rs`, `rules::Fitch`, `FitchOptions`):
  the lines are ordered per scope by Kahn's algorithm over its parts (a
  node of the scope, or a subproof inside it as one part), a part after
  every part it cites, hypotheses first, the scope's conclusion (the
  root for the main proof, the premise a rule takes from the subproof
  otherwise) held back to the last, ties in the derivation's order;
  then each subproof is expanded in place by a stack of its own. A part
  that cites across scopes waits at the scope that holds both
  (`Climb::inside` walks up the scopes, each step counted against
  `limits.work`). It refuses with `Error::NotFitch` (unsupported,
  `not_fitch`), with the reason, a last node inside a subproof, a
  premise out of reach, a subproof two rules take with different
  conclusions, and an order that cannot put the conclusion last (a
  line inside a subproof uses its conclusion, or a cycle through a
  subproof); the checker accepts some of those, which can still be
  drawn as a tree. A node cited by several lines is one line. Its size
  is estimated before anything is laid out (`LINE`, `MEMBER`, and `BAR`
  per proof each line lies in, against `limits.derivation_bytes`,
  `Refusal::Output { what: "Fitch proof" }`): the text writes a `│` per
  depth on every line, so a proof nested 20 000 deep wrote 1.28 GB
  under an estimate of 6.4 MB before `BAR` (the review of 62c). Its
  edges are deduplicated by a set and each scope's first node is found
  once (a scan per citation and per subproof made a fan of 200 000
  citations take 16 s).
  `Display` is the text form (`│` per proof, `├───` under hypotheses,
  the justifications aligned). The choice between the two layouts is
  `ViewOptions::layout` (`Layout::Tree`, `Fitch`), the command's
  `--layout`; `Styles::fitch` holds `FitchOptions`.
- **The drawing** (`draw.rs`, `rules::Drawing`): the exports' view of
  a derivation, `core-export.md` has its rules. A session's tree is
  numbered alike in `Interactive::derived` and in the drawing (both
  postorder, premises in order), which `Interactive::derivation_goals`
  and the web view's `inferences` rely on; change one order and the
  other with it. That holds only without derived lines: a line cited
  twice is one node and two drawn inferences, and `Drawing::nodes`
  gives each inference's node. In the Fitch proof the hypotheses of one
  scope with equal sequents are one line (a hypothesis is a node per
  use: per citation in a session, per leaf in 62a's translation). A
  session's Fitch proof takes `Interactive::fitch_derivation`, which
  adds every hypothesis its scopes offer as a node, cited or not, a
  scope's just before its first node or, where no node lies in it,
  before the rule that discharges it: `derivation` has a hypothesis only
  where a step uses it, so a subproof whose hypothesis no step cited yet
  (or never, a vacuous discharge) opened with another line, which read as
  its assumption (found on the page, 62d). All of them first instead
  sorted every subproof ahead of the lines derived before it (the
  review of 62d), since the order ties by the derivation's. It also
  maps each session line to the node showing it, an equal hypothesis to
  the scope's first. A conclusion that is a hypothesis (a goal closed by
  one) stands with the hypotheses, not held back last (`a ⊢ a` and a
  subproof with a line beside its hypothesis were refused before). A derivation read from a file has no such record: its
  vacuous discharge still has no hypothesis line. A rule whose premise
  lies outside the subproof it discharges (the goal closed by an outer
  hypothesis) cites that line, `→I 1`; a Fitch proof would add a
  reiteration line, which the module does not write.
- **A session's JSON** (wire level 17, `wire::SESSION`, 18 with a derived line; reader
  `Interactive::from_parts`) checks the tree, each scope owned by one
  rule node whose discharging premise was opened in it, inside that
  node's scope, every hypothesis in the scope it was opened in or one
  around it (two forged documents passed with weaker checks, and `undo`
  then left a node in a removed scope), where each goal was opened, every root-scope hypothesis an
  assumption, every open goal's bounds, every closed node by
  `check_open` (the checker without the open goals), and the history's
  suffix property from the last step back. The scopes' hypothesis lists
  are only what `assume` tries; its re-check is what trusts them.
- **Errors**: `Error::StepRefused` (invalid, `step_refused`),
  `Error::StepNeeds` (malformed, `step_needs`, the names), and a
  finished session with goals open `Error::OpenGoals`;
  `Error::InvalidCalculus` (malformed, `invalid_calculus`),
  `Error::InvalidDerivation` (invalid, `invalid_derivation`, the first
  node that fails), `Error::Zones` (malformed, `zones`); the checker's
  `Fault` keeps a refusal apart from a fault.
- **Where quantifiers go**: a connective gains `binds`, `Terms` a bound
  variable, `Pattern` a substitution; nodes gain nothing, since a
  witness is found by matching the stored premise. `fresh` is already
  the eigenvariable condition.
- **What it copied** (for the kernel crate): the hash-consed arena's
  idea from `ordinary::Formulas`, the meter's cadence from the ordinary
  checkers, the reader's pattern (`wire::Readable`, a form, `within`),
  and it uses `crate::name` (`normalized`, `is_identifier`)
  for atom names.
