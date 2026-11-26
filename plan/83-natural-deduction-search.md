# Step 83: natural deduction proofs found, from sequent proofs

You are working in deduxit (`deduxit/deduxit`) and its web client
(`deduxit.github.io`). CLAUDE.md applies throughout. This prompt was
written on 2026-11-26 by the planning session, after the author's review
of the redesigned page (`plan/79-page-redesign.md`, "The author's
review").

Read the rules files of calculi given as data (the rules suite, its
natural deduction rule files `core/src/rules/calculi/`, its interactive
state and its Fitch layout), of the ordinary suite and of the bindings;
the section of 79b in `plan/reports/79-page-redesign.md` ("Departures":
natural deduction's `?` with Prove apart alone); and the client's
`.claude/rules/web.md`.

## Goal

Natural deduction runs on the rules suite, which checks a derivation in
a calculus given as a rule file but has no search, so the page's natural
deduction view offers no close by the search, no hint, no marking and
no Go back. The author's word (2026-11-26): "add a search to the library
(or a translation from sequent proofs to natural deduction/Fitch
proofs)".

The standard route is the translation: a cut-free sequent derivation
becomes a normal natural deduction proof (Gentzen's and Prawitz's
correspondence, as Troelstra and Schwichtenberg give it for LJ and NJ,
LM and NM, and LK and NK with a classical rule). With it, the sequent
search the ordinary suite has becomes natural deduction's search: search
in LJ, LM or LK, translate, check the result against the rule file. Take
that route unless the step finds it cannot cover a rule file deduxit
ships; then say why to the planning session before building a search of
the rules suite's own.

## Fixed now

- **Every translated proof is checked** by the rules suite's checker
  against its rule file before it is answered or grafted.
- The proof is normal (no introduction followed by an elimination of the
  same connective); a test holds it.
- Natural deduction's panel (`Natural.tsx` in the client) lists only
  the rules that apply, by `rules_rules`; step 81 added
  `rules_candidates` (every rule with its status and reason), which the
  linear and ordinary rule popover takes. The panel takes it too, so
  that a rule that does not apply is greyed with its reason, as in the
  other calculi.
- Each natural deduction rule file deduxit ships gets the translation of
  its sequent calculus: the intuitionistic and minimal ones from LJ and
  LM, the classical ones from LK with the file's classical rule (`nk`,
  `nk-dne`, …). First order where the rule files have quantifiers.
- The rules suite's goal state gets `close` and `hint` for those
  calculi through it, and the page's marking runs on it.
- Library, bindings (rows in `plan/web/README.md`, tests), the CLI's
  `prove` for a natural deduction calculus if it has a place for it (the
  manual's chapter, the guide for AI models), the changelog's
  `[Unreleased]`, `breaks 0.8.0`.

## How the step runs

One session, Opus 5.5 at `high`, `fresh-reviewer` (Fable 5.2 at
`medium`). The library and the bindings first, "83: deduxit ready" to
the planning session, which pushes deduxit's `main` after its review and
the full check; then the client's input moved and the page: natural
deduction's `?` gets the goal's menu the other calculi have (close by
the search, hint, the marking, Go back), tree and Fitch, with Playwright
tests. The report, `plan/reports/83-natural-deduction-search.md`.
