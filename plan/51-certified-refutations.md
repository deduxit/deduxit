# Step 51: the one-sided and state-equation refutations, checked and certified

You are working in linlog (`sequentium/sequentium` after step 48) and in
the Rocq library `rocq-linlog` (`../rocq-linlog`, `sequentium/rocq-linlog`
after step 48, with its own CLAUDE.md and rules). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing. A
change of the library is made, checked and pushed there first, then the
lock moves here (CLAUDE.md, "Workspace crates"). The step has two
sessions. This prompt was written on 2026-11-08 by step 46's second
session, from the author's decisions of that day. It is complete.

Read `plan/later.md` ("Certified refutations" and "Follow-ups: certified
refutations"), `plan/notes/research/next/verification.md` (sections 1 and
candidate 2), `scope-semantics-decision.md`, `.claude/rules/core-search.md`
(the refutations, `Disproof::check`, "The refuters with quantifiers"),
`core-horn.md` (the state equation), `core-focus.md` (where the one-sided
count is found), `core-exports.md` (the Rocq certificates), the report of
step 31 (session 31d, "certified refutations") and the library's rules
on its invariants. `plan/README.md`'s D31 is the decision this step
starts.

## Why this step

Decision 8 of 2026-11-08 (D31): verification grows stepwise, and its
first step is to certify the two refutations that linlog states today
with neither a check nor a lemma. `Refutation::OneSided` (and its
first-order form `OneSidedSymbol`): an atom's literals below copies can
add only one sign, so the balance cannot be met. `Refutation::StateEquation`:
a Horn program whose state equation has no solution, by weights an exact
check confirmed in the Horn engine. Both answer
`RefutationFault::Uncertified` from `Disproof::check` today, so a
"not provable" resting on them is trusted to the search. After this step
each is checked without the search, in Rust, and proved once in Rocq,
with a certificate the exporter writes and the library checks, as the
balance, the count equation and the classical reading are since step 31.

## Requirements

1. **The invariants stated** (51a, in the library first): for the
   one-sided count, that in every derivation of the calculus (each mode
   the refutation is given in: classical and intuitionistic, linear and
   affine, with Mix where it applies, and the first-order form if its
   argument carries over) an atom whose literals beneath the copies are
   of one sign keeps the inequality the refutation computes; for the
   state equation, that a proof of a Horn program's goal gives a firing
   sequence whose counts solve the state equation, so a weighting that
   proves the equation unsolvable (a Farkas certificate, checked by
   computation in Rocq) refutes it. Each proved once, its assumptions
   closed under the global context (`Print Assumptions`, as the
   library's check does), with tests.
2. **The certificate** (51b, here): the exporter's second kernel
   (`--kernel linlog`) writes each refutation's certificate (the counts
   or the weights, and the lemma's application), refused only where the
   lemma does not reach (said by an `Unsupported` variant with its
   reason); the flake's `rocq-linlog` check compiles every new kind
   against the pinned library.
3. **The check without the search** (51b): `Disproof::check` gains the
   two arms, each recomputing the invariant from the goal alone with the
   arithmetic that cannot wrap (`conduct.md`: say at each sum why it
   stays within its type, or refuse), and a refutation that does not
   hold answers `invalid_refutation`, as the others do; a planted wrong
   refutation of each kind is refused by a test.
4. **`linlog check` reads a disproof** (`later.md`, "Follow-ups:
   certified refutations"): the JSON of an outcome's refutation is read
   and checked with the exit statuses a proof's check has, in the manual
   and the guide for AI models with a real example.
5. **The comparison's driver** counts a disproof as checked when
   `Disproof::check` accepts it (the same follow-up), so the next
   comparison credits linlog's checked refutations.

## Constraints

- The searches do not change: the target sets' counters equal against
  step 47's oracles, the ratchet within its ceilings.
- The library's version and changelog say what was added; its release
  follows linlog's 0.3.0 (step 53).
- `checks.x86_64-linux.rocq-linlog` and `rocq-linlog-library` under the
  machine lock, as CLAUDE.md's table says.

## How the step runs

Two sessions on Opus 5.5 at `high` (51a the library's lemmas, 51b the
exporter, the check, the command and the driver). The mechanised proofs
move to Fable 5.1 at `xhigh` only if Opus visibly falls short ("Why these
models and efforts", the role table). `Disproof::check` is a checker:
its two arms are reviewed by the panel of step 26's prompt (an argument
read on Fable 5.1 at `high`, counterexamples and integers on Opus 5.5
and Sonnet 5.5 at `high`), each refutation backed by a witness the
session reproduces. Deliverables: the library's commits (pushed by the
planning session before linlog's lock moves), linlog's commits,
`plan/reports/51-certified-refutations.md` with "What the next sessions
must know" (what is certified now, and what stays uncertified:
`Exhausted`, `Saturated`, the first-order counts if they did not carry
over).
