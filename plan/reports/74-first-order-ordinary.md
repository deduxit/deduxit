# Step 74: first-order ordinary logic, its language and calculi

## Session 74a: the design and the data model

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc` on
`main@origin`. (departs from the plan: Opus in place of Fable,
unattended) Nothing was pushed.

### Outcome

The design is written: `plan/notes/first-order-design.md`. It decides
the language, the data model, the calculi and their checker, the
sessions' witnesses, the text, JSON and TPTP forms, the exports, the
Rocq certificates and the protocol, and what steps 75 to 79 build on.
Each decision gives its sources and the alternatives weighed.

**No code landed.** (departs from the plan: the brief asks for the
arena, the terms and the parser "as far as the session reaches")

- The first unit of code is `ordinary::Node`'s two variants. The
  compiler names 67 `match`es in 30 files that each need an arm for them.
- Most of those arms hold only behind every deciding method's refusal
  at its door. So the variants, the arms, the doors and their tests are
  one change, which must not land half done.
- This session ran 71a, 71b's review, 71c and 74a in one conversation,
  and that change did not fit in what was left of it. It is 74b's,
  first; the figures it compares against are below.
- A smaller slice was weighed and set aside: terms and ground atoms
  without binders, which every propositional engine already decides
  correctly. It would still need the JSON form of terms, since the JSON
  reader refuses an atom named `p(a)`. Without that, a sequent the
  parser reads would be written to JSON and refused when read back.

The commits:

| change | subject |
|---|---|
| `zzqkrqxr` | Design first-order ordinary logic: the language, the arena and the calculi |

This report is the change after it.

### The design's decisions

The note argues each; in brief:

- **One arena.** `ordinary::Node` gains `Forall(NodeId)` and
  `Exists(NodeId)`. An atom stays an index, its predicate and arguments
  in step 42's first-order table beside the nodes, so the node stays 12
  bytes.
- **Bound variables are de Bruijn indices** (locally nameless, as step
  42's linear terms), so formulas equal up to bound names are one node
  and the checker's comparison by id holds through quantifiers.
- **Instances are nodes**: `Formulas::instantiate` builds `A[t/x]`,
  hash-consed, memoised per node and binder depth. The linear suite's
  frames are not taken: they exist because its checker numbers the
  occurrences of one forest, and the ordinary derivation writes every
  sequent whole.
- **The terms are step 42's** (`FoTerm`, `FirstOrder`, `Interner`), not
  a third copy, as the prompt fixes. Their move from `linear::sequents`
  to the crate's root is kernel-boundary item 1's case, a commit of its
  own.
- **The calculi are G3c, G3i and G3m's quantifier rules** (Troelstra and
  Schwichtenberg, *Basic Proof Theory*, 2nd ed., §3.1). The checker also
  takes G1's forms. Every inference records its term. The eigenvariable
  condition is local: the eigenvariable occurs in no formula of the
  conclusion, since every inference holds its whole sequent.
- **Sessions take a witness as the linear session does**: a typed term,
  or an open witness `$1` that a later axiom fills by unification with
  levels. The linear session's unifier is lifted into the shared terms
  module.
- **Rule files do not carry the quantifier rules yet**: a connective of
  step 60 cannot bind, which is 71a's reason too. The plan's item "rule
  files with binders" is where they come.
- **The forms**:
  - the native syntax is the linear suite's (`forall x. A`, a quantifier
    binding loosest);
  - the TPTP reader follows TPTP's own grammar (a quantified formula is
    a unit);
  - JSON at wire level 25 with step 42's keys;
  - ILTP's first-order problems come as a package of their own.
- **The Rocq certificates are terms over `Prop`**, as the propositional
  ones are, with no module of the library: a domain `(D : Type) (d :
  D)`, predicates and functions over it, `∀` and `∃` as Rocq's own.
  (departs from the design: the plan's default was a new module of
  rocq-deduxit; the note says why a shallow statement certifies the same
  with less to trust)
- **The deciding methods refuse a first-order sequent at their door**
  with one error, `Error::FirstOrder { method }`, until step 75's
  searches and step 77's transformations.
- **What 75 needs**: the embedding's pattern table gains Girard's
  quantifier rows, and the read-back the quantifier rules. Then the
  focused engine's first-order search (42d) is a semi-decision of
  first-order LK and LJ from the start.

A question the prompt does not settle, decided by the literature: **the
scope of a quantifier in the native syntax**. It reaches as far right as
it can, as in the linear suite and in the texts that write a dot after
the variable; TPTP's own reader keeps TPTP's grammar.

### The measurements (D17)

The ratchet before any change of `ordinary::Node`, at `zzqkrqxr` (capped,
cores 10 to 12, callgrind's instruction counts, which do not depend on
the machine's load):

| journey | instructions |
|---|--:|
| `read-text` | 12 433 368 |
| `read-json` | 12 226 550 |
| `read-tptp` | 33 313 038 |
| `ordinary-pigeons` | 31 715 255 |
| `render-latex` | 5 780 834 |
| `render-svg` | 79 508 723 |

The other journeys are linear; the run's table is the ratchet's output
of that day. 74b measures again after the change of `Node`, and runs the
propositional ordinary suite's rows of the target set and ILTP's
propositional problems against these.

### What was checked, and by what

- The design's claims about the code were read in the code. Two reading
  sub-agents (Sonnet 5.5 at `high`, `deduxit-read-74a-plan` and
  `deduxit-read-74a-code`) mapped the plan's material and the linear
  suite's first-order code, with file and line citations. The session
  checked the points the design rests on itself: the ordinary node and
  arena, the checker's inference form, the parser's operand loop, the
  first-order table and its interner, and the sequent's JSON form.
- The compiler listed the 67 `match`es the two variants reach (the run
  is in this report's first section).
- No code changed, so the gate has nothing to judge beyond the last
  one's; `nix flake check` runs at the commit that holds this report,
  so its exit status cannot be in it: it is in the session's last
  message to `deduxit-orchestrator`.

### The reviewer's round

The fresh-context reviewer (`deduxit-review-74a`, Fable 5.2, as the
brief asked, since the writer is Opus) read the note against the prompt,
the brief, kernel-boundary section 6, the rules and the code it makes
claims about. It ran nothing.

It found that these held:
- the data model;
- the G3 rules for all three logics;
- the local eigenvariable condition, with the principal formula kept or
  dropped, shared or split contexts, and LJ's cut;
- the Rocq statement over a non-empty domain;
- the syntax;
- D17's account;
- that steps 75 to 79 are possible on this model.

It reported eight findings. All hold, and the note is amended for each
in the change that holds this report:

1. **An open witness cannot be filled in place** when instances are
   nodes: `p($1)` and `p(a)` are two ids, and the axiom compares ids.
   The note now has a bind substitute through every stored sequent of
   the session, with the replaced ids recorded for `undo`. The checker
   refuses `Meta` in a derivation read from JSON.
2. **An eigenvariable that no rule binds had no Rocq term.** Such an
   eigenvariable is used in a sibling branch, below its `∀R`, or as the
   constant an open witness became. It is now `d` outside the subtree
   of the inference that introduces it, with the soundness argument.
3. **The checker cannot build nodes** (`&self`). `instantiate` has a
   find-only form for it, and the interner's maps stay in `Formulas`.
4. **The term printer, the atom key and the sequent reader** take an
   arena to hold no `Eigen` or `Meta` (`unreachable!`). They are now
   listed as code 74 extends.
5. **Binder names kept per node print wrong formulas without renaming.**
   Every target's printer now renames as the linear one does,
   eigenvariables' names included.
6. **Kernel-boundary items 2 and 3 were not decided.** The note now
   decides them: `Arena<N>` is deferred to its own trigger, and the
   parser over tables is not shared, since the loops no longer share
   their shape. Item 5 is 74c's.
7. **TPTP's unbound upper-case word** is a variable, not a constant:
   the TPTP reader refuses it.
8. **The admissible view could draw a `∀R` with its eigenvariable in
   its conclusion.** It now keeps such a weakening.

### What the next sessions must know

- **74b first: the node change, whole.**
  - Add `Node::Forall` and `Node::Exists`.
  - Give every `match` its arm. The compiler names them: 67 in 30 files,
    `normal.rs` with 18, `ljt.rs` with 6 and `sat/` with 11.
  - Refuse a first-order sequent at the door of every method that
    decides or transforms: `decide`, `ljt::decide`, `sat::decide`,
    `truth_table`, the tableau, `normal`, `resolution`, `interpolant`,
    `negative`, `translate`, `Interactive::within`, `WrittenTableau`'s
    review and `Model::forcing`. Only then does an inner arm's
    `unreachable!` hold.
  - Test each door.
  - Measure the ratchet against the table above.
- **Then, in this order**: the arena's first-order table (step 42's,
  moved to the root), the native parser's binders and terms (out of
  line, `#[cold]`, as the linear parser's are), the printer with binder
  names (rename a binder whose name an enclosing one has), the JSON
  form at level 25, the TPTP dialect. Then 74c's checker and sessions,
  and 74d's exports and certificates.
- **The printer must rename**: a node is shared by every formula equal
  up to bound names, and its binder keeps the first name written, so two
  nested binders can carry one name. Print a fresh name where an
  enclosing binder has it.
- **The JSON reader refuses an atom named by its canonical text**
  (`p(a)`): ground first-order atoms need the level-25 form before any
  first-order sequent reaches a writer.
