# Step 80: the release of 0.8.0

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`) and the Rocq library rocq-deduxit
(`~/Projects/own/rocq-deduxit`). CLAUDE.md applies throughout; tagging
and publishing are the author's acts. This prompt was written on
2026-11-23 by the planning session at the review of step 78 (step 79,
the page at first order, moved after 0.8.0: the author holds the
website).

Read `plan/73-release-0.7.0.md` and its report (the last release, the
pattern), `.claude/rules/ci.md`, `CHANGELOG.md` and the reports of steps
74 to 78 and of fixups 13 to 25 (`plan/reports/fixups.md`).

## Fixed now

0.8.0 is "first-order ordinary logic" (the author, 2026-11-15): steps 74
to 78. As the releases before it:
- `breaks 0.7.0` and every line it lists covered (under "Changed" or
  "Removed", or explained);
- the versions (the behaviour lock's workspace input blessed in the
  bump's commit, as 64, 69 and 73 did);
- `status.md` true; the guide within its budget; README, the
  installation chapter, `CITATION.cff`;
- the changelog's 0.8.0 section dated on the day the session finishes,
  every entry of `[Unreleased]` in it;
- `nix flake check`; the release workflow's dry run (its tag check, the
  crate selection, `cargo package`);
- the release notes; the author's checklist.

**rocq-deduxit releases too** (its first since v0.6.0): 76c added the
first-order Kripke semantics, LJ's and LM's soundness for it, the
classical structures, `fo_kripke_refutation`, and the infinite chain's
lemmas (`chain_excluded_middle`, `chain_double_negation_shift`); fixup-24
fixed the shift's spelling in its comments. Its version, its CHANGELOG
dated, its README and opam file as its own rules ask, its `nix flake
check`; its tag goes first (deduxit's `--kernel deduxit` certificates
import it), then deduxit's lock points at the tagged commit.

Its report ends with the state of every item `plan/later.md` keeps for
after 0.7.0 or 0.8.0, and those steps 74 to 78 and the fixups added,
for the planning step after the release that re-surveys `later.md` and
research (D30, D31), and with step 79's state (the page at first order,
waiting for the author's redesign).

One session on Opus 5.5 at `high`, `fresh-reviewer` once (Fable).
Deliverable: `plan/reports/80-release-0.8.0.md`.

## What steps 74 to 78 left you

- **What 0.8.0 holds**: first-order terms, predicates and quantifiers,
  TPTP `fof` and `cnf`, ILTP's first-order problems, LK, LJ and LM at
  first order (74; wire 25); the search: the embedding into first-order
  MALL and ILL, the classical and intuitionistic connection provers,
  the decidable fragments, the defaults (75; fixup-23 bounded the
  library's default prover and gave the batch the race); finite models
  by the model search and finite Kripke models with growing domains,
  certified in Rocq and drawn (76; wires 26, 27); prenex form,
  miniscoping, Skolem and Herbrand forms, clausal form, Herbrand's
  theorem by the SAT engine, the negative translations at first order,
  interpolants, Glivenko's failure shown (77; wire 28); equality by its
  axioms, first-order resolution with paramodulation, `prove --szs`
  (78; wire 29).
- **The wire levels 25 to 29 are new**: the manual's JSON Schema of the
  wire forms holds them (the `wire-schema` check validates the command's
  forms); the changelog names them.
- **The ceilings moved** in commits of their own (read-tptp, read-lltp,
  ordinary-pigeons, read-text 12 591 991; the web module 2 875 000 bytes
  at gzip): the changelog says nothing of them unless a user sees it.
- **The website is frozen until 0.8.0 is out**: nothing goes to
  `deduxit.github.io`, neither page work nor a lock bump. The web
  bindings (`web/`) are in the workspace and released with it as
  before; the client takes them after 0.8.0.
- **The tags are held for the author** (unattended: no yes covers a
  tag). Prepare everything up to them: rocq-deduxit's first, then
  deduxit's, then the crates.io approval of the release workflow.
- **The opam submission** of rocq-deduxit waits for the author ("when
  everything is done, 0.8.0 or later"): say in the checklist that it can
  go now, with what it needs, and leave it to the author.

## The session (planned 2026-11-23)

- **80**, when 78c and fixup-25 have landed: everything but the tags
  and the publish.
