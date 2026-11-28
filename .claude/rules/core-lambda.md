---
paths:
  - "core/src/lambda/**"
---

# deduxit core: lambda-terms, read back from proofs, read from text, checked

Loaded, beside `core.md`, when a file of `core/src/lambda/` is read or
changed: the read-back of a two-sided ILL derivation as a term of the
linear lambda calculus and of an LJ derivation as a simply typed term,
the term's data, its printer, its reader and its type checker. The
design, with its sources and the alternatives weighed, is
`plan/notes/typed-lambda-design.md`.

- **Layout.** `mod.rs`: the data (`Term`, `Node`, `Type`, `Variable`,
  the ids), `Calculus`, the print `Options` and `Notation`, `NoTerm`,
  the crate-private `Types` (the hash-consed arena) and `Builder` that
  the read-backs and the parser fill, and `names_variable`; `linear.rs`:
  `read` from a `proofs::Derivation` (`Derivation::term`); `simple.rs`:
  `read` from an `ordinary::Derivation` (`ordinary::Derivation::term`);
  `print.rs`: the `Spelling` tables (Unicode, ASCII, and LaTeX, Typst
  and SVG behind their features) and the one printer (`Term::write`,
  `write_judgement`, the crate-private `write_in` the exports call,
  `write_type` and `write_name` for the checker's messages);
  `draw.rs`: `Drawing`, the typing derivation laid out for the exports
  (`core-export.md` has its rules);
  `check.rs`: `Term::check`, `Typing`, the one rule per node (`rule`),
  `Fault` and its words (`describe`); `parse.rs` (feature `parse`):
  `Term::parse`; `read.rs` (feature `serialize`): `Term::from_parts`,
  the shape check of a document's parts, whose serde forms are
  `core/src/serialize/lambda.rs` (wire level 23, `wire::LAMBDA`);
  `inhabit.rs`: `inhabit` and `enumerate` (inhabitation of simple
  types, their answers' forms at level 24, `wire::INHABITANTS`) and the
  protocol's `has_type`;
  the untyped reader in `parse.rs` (`Parser::untyped`);
  `reduce.rs`: `Term::reduce`, `ReduceOptions`, `Strategy` (with
  `pick`), `Step`, `End`, `Reduction`, and the copy (`copied`) that
  `infer.rs` shares; `infer.rs`: `Term::principal_type`;
  `poly.rs`: System F's operations on types (`close`, `open`,
  `substitute`, `free`);
  `system_f_tests.rs`: System F's terms, substitution and type β;
  `tests.rs`: the round trips against a checker of the tests' own;
  `untyped_tests.rs`: untyped terms, the stepper against a de Bruijn
  reducer of the tests' own, and the principal type against every
  typing the checker accepts.
- **A document's nodes and types are read through flat forms**
  (`serialize/lambda.rs`, `NodeForm`, `TypeForm`: the kind and every
  field any kind has, read where it stands, then one `match`), not
  through `Node`'s and `Type`'s derived readers, which are tagged by
  `kind` and so hold the whole object in serde's `Content` and read it
  twice: those two functions were a third of the lambda suite's bytes in
  the web module. `NodeKind` and `TypeKind` list the variants in the
  enums' order (an unknown kind is refused with the same list); a name
  no kind has is ignored, a field of another kind of the wrong type (or
  given twice) is refused, and so is a node written as an array, which
  serde's tagged reader took (the planning session's decision). Two
  things keep the tagged reader's words: `kind` is read as an identifier
  (`kind_index`, "expected variant identifier", a map refused), and a
  missing field is raised where its object ends (`ReadType`, `ReadNode`:
  `try_from`, so serde_json gives its line and column)
  (`a_node_s_fields_are_read_where_they_stand`). The enums keep their
  derived `Serialize` and public `Deserialize`.
- **A node's variables and a type's operands are walked by one function
  each** (crate-private, in `mod.rs`): `Node::variables` and
  `map_variables` (the reader's shape check, the stepper's copy),
  `Type::operands` and `map_operands` (the reader, `poly`'s walks with
  their depth, the printer's free names). A new node or type adds its arm
  there; `operands`/`renumber` over a node's subterms stay as they are,
  and `check::written` and `reduce::binders` are other sets.
- **A variable is its `VarId`, with its type** (Church's own typing):
  a binder and every use name one entry of `Term::variables`, so a
  node's type depends on the node alone and a shared node is typed
  once. The name is for printing and reading only. Do not key anything
  on names.
- **A term's types hold no type twice** (`Types`, the `Builder`, and
  `from_parts`, which interns what a document writes): the rules compare
  types as ids. A term made any other way must keep this, or `check`
  refuses equal types as different.
- **The rules are written once** (`check::rule`): the checker calls it
  for every node and the parser for each node it closes, because a
  pattern's variables (`let`, `case`, `copy`, `promote`) take their
  types from the term taken apart, which the parser must know before it
  reads the body. A change of a typing rule is made there and nowhere
  else; the parser's own tests of a scrutinee's shape (`misshapen`) only
  say the same fault earlier, with the scrutinee's place.
- **The uses are counted over the DAG** (`check::Pass`): a node's map
  of free variables to counts is moved to its last user and copied for
  the others (`waiting`), so a tree costs what small-to-large merging
  costs and a shared node costs a copy per extra user, each copy ticked
  against `limits.work` and the maps kept held against
  `limits.memory_bytes`. Counts saturate; `Uses`' doc says why that
  accepts nothing.
- **`check` also refuses a capture by name** (`Fault::Captured`): where
  a variable is bound, no other variable of its name is free in the
  body. Without it a document could hold `λx:a. λx:b. x` with the outer
  `x` meant, well typed as `a → b → a`, and print a text that reads as
  another term. Only variables whose name another has (`twin`) pay for
  the scan. The order of binding is the text's: a pattern's later
  variable and a later declaration of the context are the inner ones,
  so `bind` takes them first (the review of 71a: taken left first, an
  affine `let z be x ⊗ x in x` with the first `x` meant passed). A
  pattern that names one variable twice is `Fault::Twice`. Names are
  compared as stored, so every way in composes them (NFC): the parser,
  and `from_parts`, which also makes two atoms of one name one atom.
- **The check's memory estimate counts what it makes at full size**
  (`NODE_BYTES`, `TYPE_BYTES`, `VARIABLE_BYTES`, `USE_BYTES`): the
  tables of types and of names are made with their room, so that no
  growth holds two copies; a table left to grow held 39 times the
  bound for a million variables (the review of 71a).
- **The simple calculus has fewer nodes and types** (`Fault::Foreign`):
  no `⊗`, `1`, `!`, no `let`, `promote`, `derelict`, `discard`, `copy`,
  and `true` and `false` without arguments. The linear and the affine
  calculus take every node.
- **Typed or untyped is the calculus's** (`Calculus::is_typed`): a
  typed term has every type (each variable's, the goal), an untyped one
  none, and no atoms or types at all. Every builder keeps it, and
  `from_parts` refuses a document that mixes them (`malformed`), so the
  crate's `Variable::typed` and the checker's rule may `expect` a type
  in a typed term. An untyped term has `Var`, `Lambda` and `Apply` only:
  `check` refuses any other node as `Fault::Foreign`
  (`untyped_foreign`), and the uses pass is the same as the typed
  terms', linear for `UntypedLinear` (`is_linear`), any count for
  `Untyped` (`is_unrestricted`). Its `Typing` is empty and its wire
  level 30 (`wire::UNTYPED`), its JSON without `atoms`, `types`, `goal`
  and the variables' `type`.
- **An untyped text's free names are its context** (`Parser::untyped` in `parse.rs`):
  without `⊢` every name no `λ` binds is a free variable, in the order
  the names first appear; with `⊢` the names before it are the context
  and another free name is refused. The judgement the printer writes
  (`x, y ⊢ M`) keeps the free variables a step dropped, which the bare
  text loses.
- **A β step copies the term as a tree, every binder a fresh variable**
  (`reduce.rs`, `Copier`): the redex's argument is copied at each use of
  the variable with fresh variables for its own binders, so no variable
  is bound twice and no use can be taken in by its own variable's
  binder; `standing` maps each source variable to the copy's at the
  place being copied and is restored when a binder's node is left, so
  even a source that binds one variable twice (a document can) is read
  by the innermost binder. A term with a shared node other than a
  variable's, or a variable bound twice (`needs_copy`), is copied once
  before the first step, so every redex a step names is at one place.
  Only `(λx. M) N` is a redex: the linear calculus's other constructs are
  copied as they are (`binders` names a node's binders,
  `Node::map_variables` every variable it names).
- **Names are fixed after the copy** (`rename`): a binder whose body
  has a free variable of its name, which a substitution can make
  (`(λx. λy. x) y`), is renamed to its letters with the least index the
  term does not use (`y₁`, `print::fresh`, the printer's own), so
  `check`'s capture condition holds and the text reads back. A new
  variable of a read-back or a search is named by `letter_name`, its
  class by `letter_class`, so both name alike. Only names that two variables share are scanned.
- **Every reduct is checked** (`Term::check` before the next step): a
  typed term keeps its type, an untyped linear or affine one its uses;
  a fault is `Error::ClaimFails` (a defect), a refusal ends the
  reduction with `End::Refused` and the last term checked. The bound on
  steps ends with `End::Steps`, which says "not finished", never "no
  normal form" (Ω has none, and an untyped unrestricted term need not
  normalise); the bound on nodes is counted as the copy is made
  (`Halt::Nodes`), the step not taken. Each node made is a unit of
  `limits.work` and `NODE_BYTES` against `limits.memory_bytes`.
- **The strategies** (`Strategy::pick`): normal order is the first redex
  in pre-order (leftmost outermost), applicative order the first in
  post-order, which holds no other redex (leftmost innermost). The
  differential test compares normal order's normal forms and step counts
  with a de Bruijn reducer on random terms whose few names force
  captures.
- **The principal type** (`infer.rs`) is computed on the term copied as
  a tree (two places of one subterm may need different types, which a
  DAG would force equal), by unification with the occurs check over
  type variables and arrows (`Unifier`, union-find with path
  compression, the occurs check a marked walk over the type DAG, every
  step a unit of `limits.work`); no recursion. Only variables are linked
  in the union-find: two arrows unified stay apart (`unified` keeps the
  pair, so shared types are unified once), since an arrow linked away
  before its operands were unified hid them from the occurs check, and
  `\z. \x. \k. k (z x) (z z)` then made a cyclic type that the
  solution's walk followed forever (the review's witness, a test now). A failure is the occurs
  check at an application, said with the equation (`a = a → b`) and the
  application's text, as `Error::IllTyped` at the source's node. The
  solution's type variables are atoms `a`, `b`, … named in the order the
  judgement writes them, the term returned is in `Calculus::Simple` and
  checked (`ClaimFails` otherwise). `untyped_tests.rs` holds every
  typing the checker accepts over a small set of types to be an instance
  of it, and finds none for a term without one.
- **The text the parser reads is what the printer writes with `types`
  on**, in both notations (`agrees` in `tests.rs` holds every read-back
  term to it). A change of a spelling or of a bracket is made in both.
  `inl`, `inr`, `abort` and `false` take their type after `_`; a word
  that starts with one of them and `_` is therefore no variable's name
  (`SUBSCRIPTED`), in the parser and in the JSON reader alike.
- **Types are parsed here, not by the sequents' parsers** (one cursor,
  exact places), with their tokens and strengths copied from
  `ordinary/parse.rs` and the manual's "Syntax"; a change there is made
  here too. A variable in scope is found by its name in a table, each
  name's innermost last: a scan of the scope made a text of `n` binders
  and `n` uses cost `n²`.
- **No rule file for STLC**: step 60's `fresh` reads any occurrence in
  the conclusion and a connective cannot bind, so the abstraction's side
  condition cannot be said (the design note, section 6). The typing
  derivation is the checked term (`Typing`); a view that draws it is to
  implement `linear::proofs::style::Drawn`.
- **The calculus is fixed and sourced** (the manual's "Proofs as
  programs" chapter, `docs/src/terms.md`, says it to users): Bierman's
  thesis (UCAM-CL-TR-346, ch. 3, fig. 3.2), the BBdPH calculus (TLCA
  1993) completed with the additives, `true(x⃗)` for `⊤R` and
  `false_A(x⃗; z)` for `0L`. A left rule substitutes (`N[x := f M]`), so
  a cut-free proof is a normal term; a cut is the redex `(λx:A. N) M`.
  A change of a rule's term is a change of the calculus: change the
  manual's table, the module docs' table and the reviewer's criteria
  with it.
- **Two checkers agree on every read-back** (`tests.rs`): the tests'
  `Checker` (recursive over small terms, written before `check.rs` and
  kept as its reference) infers a term's type and counts every
  variable's uses, and compares the hypotheses' types with the
  reading's own printer (`round_trip`) and with an ordinary converter
  written apart (`of_ordinary`); `agrees` then runs `Term::check` and
  both round trips (text in two notations, JSON) on the same term.
- **Matching premises by member is sound because equal members are one
  formula** (`linear.rs`): a premise's hypotheses take the conclusion's
  terms by member from a pool, the rule's new members first; a split
  context (`⊗R`, `⊸L`, cut) takes each term once, an additive rule
  (`&R`, `⊕L`) gives both premises a copy of the pool. Which of two equal
  members gets which term only swaps two copies of one `!` formula. The
  premises are in the one-sided rule's order (`Inference::premises`), so
  the left operand's premise is the first; the read-back relies on it
  for `⊗R`, `&R`, `⊕L`, `⊸L` and the cut (whose first premise holds `A`
  in output position, `core-proofs.md`).
- **Structural rules are read by multiset difference**, so a compact
  derivation (a run drawn as one inference) reads back alike: `!w`
  discards what the premise lacks, `wk` drops it (the affine calculus),
  `!c` copies a chain for what the premise has more of. A term with a
  `wk` is `Calculus::Affine`, one without is linear whatever the mode.
- **The LJ read-back keeps hypotheses per formula, innermost last**, as
  the Rocq certificate does (`ordinary/rocq/mod.rs`): equal formulas are one
  arena id, a contraction is a variable used twice, a weakening one left
  unused, and the admissible view (`Derivation::shown`) reads back like
  the explicit one (`simple` in `tests.rs` checks both). `WR` and `⊥L`
  into `⊥` write no `abort` (`abort_⊥(M)` is `M`). **It never panics on
  a derivation**: what it rests on (an inference at all, every premise
  before its inference, a hypothesis in scope, a rule's premises and
  operands, an equivalence's implications and `⊥` in the arena, a rule
  of LJ) is checked as it reads, and a break is
  `Error::ReadBack` naming the inference (`unreadable`), a defect, since
  the checker refuses each and the crate makes none
  (`a_derivation_the_read_back_cannot_read_is_refused`). Only the
  quantifier arms stay `unreachable!`, behind `propositional`'s door.
- **The DAG and the bound.** A left rule's term is one node every place
  the hypothesis goes names, so the nodes stay within twice the
  inferences, one per formula a compact run weakens or copies (a run of
  `!w` is one inference and a `Discard` per formula), and one per
  variable, so never more than the derivation's members; but the text
  unfolds the DAG: a pair
  under a `⊸L` doubles it (`a_term_past_the_bound_is_refused`). The
  printer computes an upper bound over the DAG, each node once with its
  own pieces and two brackets, saturating, and refuses past
  `limits.derivation_bytes` (`Refusal::Output { what: "term" }`) before
  writing; the exports use the default limits, since their `write` takes
  none. `Builder::finish` drops the nodes the root does not reach (a
  weakened variable's).
- **`Display` for `Term` never fails**: it writes under the default
  `Options` and `Limits`, and a term past the bound writes the refusal's
  message in angle brackets (refused before anything is written), since
  a `Display` that returned `fmt::Error` would make `to_string` panic
  (`a_term_displays_as_written`, a tensor of a tensor 64 deep).
- **The types are a DAG too**: equal types are one id, so a type's
  text is not bounded by the arena (the simple calculus's `A ↔ B` is `(A
  → B) ∧ (B → A)` with `A` and `B` shared, and nested `k` deep writes
  `2ᵏ` atoms). `Printer::new` bounds every type's length over the arena,
  each type once, saturating, and builds no text to measure one; the
  review found the first printer writing every type out to measure it,
  which a session's `↔` nested 27 deep drove out of 2 GiB before the
  refusal (`a_type_past_the_bound_is_refused_unwritten`).
- **No recursion**: both read-backs walk with a task stack, the types
  are built with a stack of their own, the printer is one loop over
  pieces; `any_height_reads_back_on_a_small_stack` (120 000 inferences
  on 256 KiB) and `core/tests/depth.rs` (a type nested 50 000 deep)
  pin it. The stop is asked once per inference (`Phase::ReadBack`) and
  once per node written (`Phase::Write`).
- **Refusals** (`Error::NoTerm { reason }`, kind unsupported, code
  `no_term`): checked before the walk, in this order: an ordered mode
  or reading (`Ordered`), Mix in the mode or a `mix` inference (`Mix`),
  binders or a quantifier rule (`Quantified`), no reading (`Classical`:
  one-sided), an open goal (`Open`); LK is `Classical`.
- **The cut-elimination test keeps to `⊸`, `&` and `⊤`**
  (`cut_elimination_keeps_the_normal_form`): there the normal forms are
  β with the projections and `true` absorbing its arguments' free
  variables (Bierman's commuting conversion for `true`). With `⊗`, `⊕`,
  `1` or `!` the term of the eliminated proof differs from the reduct by
  commuting conversions (a `let` or `case` moved past a cut), which the
  test's normaliser does not implement; extend it with those
  conversions before widening the fragment.
- **System F is the typing axis's third value** (`Calculus::SystemF`,
  `SystemFLinear`, `SystemFAffine`; `is_polymorphic`): the simple or the
  linear calculus's nodes and types (`is_ordinary` says which, where
  code once compared with `Calculus::Simple`) plus `Type::Forall`,
  `Bound`, `Variable` and `Node::TypeLambda`, `TypeApply`. Church style
  only: no inference (Wells, undecidable) and no inhabitation search, by
  the author's word; `NoTerm::Quantified` stays for first-order
  quantifiers, which are no type quantifiers.
- **A `∀`'s variable is a de Bruijn index** (`Type::Bound`, counted from
  the nearest `∀`), and the name a `∀` keeps (`Forall::variable`, an
  entry of `Term::type_variables`) is for printing alone: the arena
  keys a type by `Type::key`, which erases it, so types equal up to
  bound names are one id and the rules still compare ids. The first name
  interned wins: a type prints with the names of the first equal type
  made, which the manual says. Every type a term writes is closed
  (`from_parts` refuses a loose index with `poly::free`'s `needs`), so
  `poly::open` substitutes a closed argument and nothing can be captured
  by construction; the capture the tests hold is the printed one.
- **A `ΛX`'s variable is a `TypeVarId` of its own** (`Type::Variable` in
  the types inside `M`); `poly::close` turns it into indices for the
  abstraction's type. Its side condition (`X` free in no hypothesis) is
  `check::scope`: a node's free type variables are those of the types it
  writes (`check::written`: a binder's variables', a sum, `false`'s
  goal, a type argument) and of its subterms, less its own at a `ΛX`;
  the root's, with the context's and the goal's, must be none. A term
  variable's type is written where it is bound, so a variable bound
  outside `ΛX` whose type names `X` leaves `X` free at the root. Sets are
  per node, so a shared node is scoped once. That argument needs each
  `TypeVarId` bound by one `ΛX` node at most (`TypeTwice`): a document
  with `ΛX. λx:X. ΛX. x` under one id passed with the type `∀X. X → ∀X.
  X`, the inner `ΛX` generalizing the outer `x` and the outer taking `X`
  out (the review's witness, closed at `a → ⊥`; a test now). The parser
  and the stepper's copy give every `ΛX` its own. `scope` also refuses a
  `ΛX` named as an atom, or one that would take in another type variable
  of its name (`TypeNamed`, `TypeCaptured`), so its text reads back.
- **Free type variables are found per written type, on demand**
  (`poly::Walker`, a mark per type and nothing else, each walk's types
  ticked and each result held against the limits): a list per type of
  the arena, made before any limit was asked, took 344 MB for 218 KB of
  text under a bound of 50 MB (the review's witness, a chain of arrows
  over as many type variables). The reader's check of loose indices is
  `poly::needs`, a number per type.
- **The printer renames a `∀` only where its name is free in its body**
  (`print::free_in`: per type its atoms' and variables' names and the
  indices that point past it): a name equal to one of those, or to the
  printed name of an outer `∀` the body refers to, takes the least index
  free (`fresh`, `X₁`). A rule that renamed against every name of the
  whole printed type renamed `∀Y` in one half of a judgement because an
  atom `Y` stood in the other.
- **System F's rules make types and count their work** (`Types::work`,
  which `poly`'s walks raise, a unit per type met at a depth): the
  checker ticks its meter by the difference after each `rule` and holds
  the types made past one per node; the parser counts each against the
  occurrence bound.
- **A type β step copies with a fresh `TypeVarId` per `ΛX` copied**
  (`Copier::type_standing`, the type each source variable stands for at
  the place: its copy's `Variable`, or a redex's argument), as term
  binders get fresh variables, so no `ΛX` is bound twice and a type put
  in place cannot meet an abstraction of its own variable;
  `needs_copy` copies a term whose `ΛX`s share a variable first.
  `rename_types` then renames a copied `ΛX` whose name would take in a
  free type variable of its name or is an atom's, as `rename` does for
  term variables.
- **Inhabitation goes through the ordinary engines** (`inhabit.rs`):
  `inhabit` is `ordinary::decide_within` of the sequent in
  intuitionistic logic, LJT again where the default method gave no
  derivation, the derivation read back (`simple::read`) and checked. The
  type is the ordinary formula, converted by `simple::type_of`, the
  read-back's own conversion, so `¬A` and `↔` are typed alike.
- **`enumerate` counts long normal forms** (Ben-Yelles; Hindley 1997,
  ch. 8) of the negative fragment only: atoms, `→`, `∧`, `⊤` (`↔` is a
  pair of `→`). With `∨` or `⊥`, βη-equality is not the identity of long
  normal forms (Scherer and Rémy, ICFP 2015), so a count would be
  wrong: `Error::NotEnumerable`. Widening the fragment needs their
  focused normal forms, not a new arm.
- **Why its count is right**: every goal's shape (the sorted set of the
  types in scope and the goal) is decided by the engines first
  (`Search::decided`, memoized), and a head is a candidate only when
  every argument's shape has a term, so every goal the search enters
  lies in some inhabitant. Then (a) a shape met again inside itself
  (`on_way`) means infinitely many terms: the outer goal's term can
  stand for the inner goal, as the inner context holds the outer's
  variables, pumping a longer term each time; (b) without such a repeat
  the pruned tree is finite (contexts only grow, shapes are finitely
  many), so a search that met no depth bound and no undecided shape met
  every term, and the count is exact. Distinct derivations of the
  grammar are distinct terms (a head is a `VarId`, a path a list of
  eliminations). A shape the engines leave unknown is not entered, and
  the count becomes "at least" (`undecided`). The engines' verdicts are
  the trusted part of the count; the terms listed are checked by
  `Term::check` one by one.
- **The counts saturate** at `u64::MAX` (`Found::overflow`), which
  makes the answer "at least"; the lists stop at `max_terms` while the
  counts go on (`product`, `Join`). The search is a task stack
  (`Task`), no recursion; work and the bytes held (`Search::held`:
  the arena, the variables and contexts, the lists and tasks waiting,
  the terms built, the ways to a head's atom and the tables' keys) are
  held to `limits.work` and `limits.memory_bytes`. The ways are counted
  by capacity and kept up as they grow: a type shared as a DAG has
  exponentially many ways to an atom (the reviewer's 621-byte document
  held 2.3 GB under 1 GiB before), and a count by length missed the
  doubling, 1.8 times the bound.
  The arena alone was counted at first, and the panel's witness took
  1.1 GB under a bound of 100 MB; a new growing structure joins `held`. Each engine call takes the caller's whole
  `limits`, so the work of a call is bounded per shape decided, not in
  total.
- **A cut list keeps the shortest terms**: the search fills a list in
  the order it meets terms, the first candidate's (the outermost
  variable's) deepest first, so where the count passes `max_terms`
  `enumerate` finds the deepest depth whose terms all fit by halving
  (a shallower search meets fewer terms; `Search::reset` keeps the
  engines' decisions), so a depth of `u32::MAX` costs some 32 searches,
  and lists its terms, then the full run's others. A loop down from the
  depth took one search a step, 25 minutes at `u32::MAX`. The count is
  the full run's.
- **`inhabit` holds its term to the sequent** by `has_type` in a
  `debug_assert!` (the read-back keeps hypotheses and goal); the
  protocol's checks of `lambda.inhabit` and `lambda.enumerate` hold it
  for real, and ask for the simple calculus.
- **Variable names** follow the read-back's letters per kind of type,
  counted along the context (`Ctx::named`), so names on one path are
  distinct and the capture condition of `check` never fires on a term
  listed; siblings reuse names.
- **`has_type`** (feature `protocol`) interns the sequent's formulas
  into a builder seeded with the term's own arena and atoms, so a type
  equal to one of the term's gets its id; a type made anew is none of
  the term's.
