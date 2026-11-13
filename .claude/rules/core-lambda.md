---
paths:
  - "core/src/lambda/**"
---

# deduxit core: proofs read back as lambda-terms

Loaded, beside `core.md`, when a file of `core/src/lambda/` is read or
changed: the read-back of a two-sided ILL derivation as a term of the
linear lambda calculus and of an LJ derivation as a simply typed term,
the term's data, its printer, and the tests' own type checker.

- **Layout.** `mod.rs`: the data (`Term`, `Node`, `Type`, `Variable`,
  the ids), `Calculus`, the print `Options` and `Notation`, `NoTerm`,
  and the crate-private `Builder` both read-backs fill; `linear.rs`:
  `read` from a `proofs::Derivation` (`Derivation::term`); `simple.rs`:
  `read` from an `ordinary::Derivation` (`ordinary::Derivation::term`);
  `print.rs`: the `Spelling` tables (Unicode, ASCII, and LaTeX, Typst
  and SVG behind their features) and the one printer (`Term::write`,
  `write_judgement`, the crate-private `write_in` the exports call);
  `tests.rs`: the round trips against a checker of the tests' own.
- **The calculus is fixed and sourced** (the manual's "Proofs as
  programs" chapter, `docs/src/terms.md`, says it to users): Bierman's
  thesis (UCAM-CL-TR-346, ch. 3, fig. 3.2), the BBdPH calculus (TLCA
  1993) completed with the additives, `true(x⃗)` for `⊤R` and
  `false_A(x⃗; z)` for `0L`. A left rule substitutes (`N[x := f M]`), so
  a cut-free proof is a normal term; a cut is the redex `(λx:A. N) M`.
  A change of a rule's term is a change of the calculus: change the
  manual's table, the module docs' table and the reviewer's criteria
  with it.
- **The term is printed, never trusted.** It is made from a derivation
  the checker accepted and checks nothing itself; `tests.rs`'s
  `Checker` (test-only, recursive over small terms) infers its type and
  counts every variable's uses (linear: exactly once, an additive pair's
  and a case's branches sharing; affine: at most once; simple: any) and
  compares the hypotheses' types with the reading's own printer
  (`round_trip`) and with an ordinary converter written apart
  (`of_ordinary`). Step 71's checker is the one meant to replace it.
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
  the Rocq certificate does (`ordinary/rocq.rs`): equal formulas are one
  arena id, a contraction is a variable used twice, a weakening one left
  unused, and the admissible view (`Derivation::shown`) reads back like
  the explicit one (`simple` in `tests.rs` checks both). `WR` and `⊥L`
  into `⊥` write no `abort` (`abort_⊥(M)` is `M`).
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
- **Where quantifiers go**: a `Node` variant for type abstraction and
  application and a `Type` variant with a bound type variable, in the
  arena's operand-first order; `NoTerm::Quantified` stays for
  first-order quantifiers, which are no type quantifiers.
