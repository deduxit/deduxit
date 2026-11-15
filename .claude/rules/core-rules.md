---
paths:
  - "core/src/rules/**"
  - "core/src/serialize/rules.rs"
  - "core/tests/rules.rs"
---

# deduxit core: calculi given as data

Loaded, beside `core.md`, when a file of `core/src/rules/` is read or
changed: the rule file, the generic derivation and its checker. The
design, with the reasons and the alternatives weighed, is
`plan/notes/rule-schemas.md`; this file holds what the code relies on.

- **Layout.** `mod.rs`: the public names, `FORMULA`, the rule files'
  text `LK`, `LJ`, `LM` (`calculi/*.json`) and `SCHEMA`
  (`calculus.schema.json`); `terms.rs`: `Terms`, the hash-consed arena
  of a derivation's terms; `calculus.rs`: `Calculus`, read from a
  `CalculusForm` and compiled (`Rule`, `Shape`, `Entry`, `Find`,
  `Equation`); `derivation.rs`: `Derivation`, its flat storage, `Node`
  (what `push` takes), `NodeRef`, `Step`, scopes; `check.rs`:
  `Derivation::check`; `ordinary.rs`: `Derivation::from_ordinary`. The
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
  `Value` for a premise's bound member in a set or multiset. The order
  is the conclusion's zones, then each premise's, and `Rule::positions`
  counts the `Position`s; a node gives exactly that many (`at`).
  Changing the order or the rule changes every stored derivation: it is
  a wire-level change.
- **Contexts** (`Checker::equations`, `sets`): multiset and sequence
  zones are solved in the order `Compiler::equations` found (each
  equation has at most one context not yet defined; a rule with none
  such is refused, "cannot be told apart"); set zones by the largest
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
  (compared per zone, sorted unless a sequence). A hypothesis of the
  root scope is an assumption. The invariant: a node's sequent follows
  from the hypotheses of its scope and the scopes around it. It is a
  statement per node, so `check` also asks that the root lie in the
  root scope: a last node inside a scope passed every node's check and
  showed `b` from no assumption (the review of 60a).
- **Freshness** (`Checker::conditions`): the variable is bound to an
  atom that occurs neither in the node's sequent nor in a hypothesis of
  its scope or a scope around it. `occurs` walks the term DAG once per
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
- **A connective takes `ARITY` (255) arguments at most**: the number
  comes from the file and sizes a vector, and 2³² of them reserved
  16 GiB under a 1 MiB memory bound.
- **Errors**: `Error::InvalidCalculus` (malformed, `invalid_calculus`),
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
  and it uses `linear::sequents::name` (`normalized`, `is_identifier`)
  for atom names.
