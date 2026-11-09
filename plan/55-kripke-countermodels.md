# Step 55: Kripke countermodels, checked and certified

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and in the Rocq
library (`../rocq-linlog`, `deduxit/rocq-deduxit`). CLAUDE.md applies
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

## How the step runs

Three sessions on Opus 5.5 at `high`: the model, its extraction and
check; the library's lemma and the exporter (Fable 5.1 at `xhigh` only
if Opus visibly falls short); the drawing and the page. The panel for
the check. Deliverables: the commits in the three repositories,
`plan/reports/55-kripke-countermodels.md` with what step 65 must reuse.
