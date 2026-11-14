# Step 55: Kripke countermodels, checked and certified

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and in the Rocq
library (`../rocq-deduxit`, `deduxit/rocq-deduxit`). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing; a
change of the library is made, checked and pushed there first. This
prompt was written on 2026-11-08 by step 46's second session, from the
author's decisions of that day, and is finished at the review of step
54: it says what is fixed.

Read `plan/notes/research/next/ordinary-sequents.md` (candidate 2),
`scope-semantics-decision.md` (semantics-decision-11),
`modal-logics.md` (section 5: what the modal suite will ask of the same
output), `plan/later.md` ("Certified refutations": the Kripke models
escalation), the reports of steps 31 (31d) and 54, and `plan/README.md`'s
D15, D24, D27 and D31.

## Goal

Every intuitionistic or minimal sequent that is not valid gets a reason
a student can look at and a researcher can cite: a finite Kripke model
(worlds, a preorder, a monotone valuation) with a world that forces the
antecedent and not the succedent, extracted from step 54's failed
search, checked by computation, drawn, exported, and certified by a
lemma of the Rocq library.

Since 2026-11-09 the model's source is step 54s: the SAT-based engine
(intuitR's algorithm) returns a Kripke model for every sequent it finds
not valid. Where 54s's measurement makes it the default, the model
comes from its search; where LJT stays the default, it is extracted
from LJT's failed search (read Pinto and Dyckhoff, *Loop-free
construction of counter-models for intuitionistic propositional logic*,
1995, first). Classical logic's falsifying assignment, from 54s's CDCL
engine or the embedding, is the model's one-world case.

## Fixed now

- **A countermodel output kind shared by suites** (semantics-decision-11):
  its data (worlds, relation, valuation, the refuting world) and its
  check are written so that the modal suite (step 65) reuses them with
  its frame conditions, and the classical falsifying assignment is its
  one-world case; it goes in the JSON forms at a new wire level and in
  `Disproof`'s family as a checked refutation.
- **The check is a checker** (`conduct.md`): forcing computed on a
  finite model within a size bound said at its declaration; the panel of
  step 26's prompt reviews it.
- **The certificate** (decision 8, D31): the library proves once that a
  finite Kripke model refuting a sequent shows it underivable in LJ (and
  the minimal variant), and the exporter's second kernel writes the
  model and the lemma's application, compiled by the flake's
  `rocq-deduxit` check.
- **The drawing**: the model as SVG (worlds as nodes with their atoms,
  the order as arrows, the refuting world marked), on the command and,
  in a session in the client's repository, on the page beside the
  ordinary tab's "not valid".
- The model's size is bounded (D16); a model past the bound is reported
  as "not valid, model too large to show", never truncated silently.
- **A model the user gives** (the author, 2026-11-08: "a way to check
  whether one particular Kripke/fan model satisfies a certain
  minimal/intuitionistic/modal sequent"): the same checker takes a
  finite model as input, in the JSON form of the output kind. It says
  whether the model satisfies the sequent at a world named or at every
  world, and where it does not, which world fails and why (the forcing
  of each subformula there). It first checks that the model is one of
  the logic's: a preorder with a monotone valuation for intuitionistic
  logic, and for minimal logic ⊥ read as an atom some worlds may force,
  monotone too. This works in the command and in the bindings; on the
  page the model editor of step 68 uses it. Beth models (the fans the
  author named) are weighed here from the literature: on a finite tree
  their forcing through bars is decidable and differs from Kripke's for
  disjunction, so they can be a second semantics of the same checker;
  an infinite fan needs a finite presentation and stays out unless one
  is standard. The report says which was built and why.

## First: ordinary logic's shape (the author, 2026-11-09)

The author accepted two breaking changes from the usability review
(`plan/notes/usability.md` §6, Q5 and Q12). They come first, in a
session of their own, so that the countermodel joins the new shape and
the JSON forms move to one new wire level, not two.
- **Q5, ordinary logic in one call.** `ordinary::decide(&sequent,
  &options)` beside `ordinary::decide_within(&sequent, &options,
  &search, &limits, stop)`, the linear suite's shape (`prove` and
  `prove_within`). The module doc's example shrinks to three lines.
- **Q12, one JSON shape for ordinary logic.** Since step 54b,
  `prove --logic L --format json` writes the ordinary outcome under LJT
  and the linear image's outcome under the embedding, told apart by
  `engine`. Every ordinary run writes the ordinary outcome:
  `ordinary::Outcome` with its `search` tagged (`Embedding {
  translation, linear }`, the image's outcome nested, or `Ljt`). The
  wire level rises; earlier documents stay readable. 54u-a makes the
  linear `Outcome` readable (Q3); the ordinary outcome gets the same
  here.
- **`check` reads the ordinary outcome**, LJ derivations included
  (54b's follow-up in `plan/later.md`), so that every file `prove`
  writes, `check` reads.
- The bindings (`web/`) and the client follow the shape in the same
  step: the page reads the ordinary results through them.
- Both are in the changelog's breaks for 0.4.0, with the behaviour
  lock's JSON lines blessed again in a commit of their own.

## How the step runs

Four sessions on Opus 5.5 at `high`: first, ordinary logic's shape
(above, with the bindings; the client's part may join the fourth
session); then the model, its extraction and check; the library's lemma
and the exporter (Fable 5.2 at `high` only if Opus visibly falls
short); the drawing and the page. The panel for the check.
Deliverables: the commits in the three repositories,
`plan/reports/55-kripke-countermodels.md` with what step 65 must reuse.
