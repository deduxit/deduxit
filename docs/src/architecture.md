# Architecture

This chapter explains how deduxit is built: which crate does what, how
the library stores a sequent, and how proofs, search and outputs fit
together. It helps when reading the code or the library's API.

## The crates

All the logic is in the library. Everything else is a front end to it.

| package | directory | what it is |
|---|---|---|
| `deduxit` | `core/` | the library |
| `deduxit-cli` | `cli/` | the command `deduxit`, a thin front end |
| `deduxit-bench` | `bench/` | the benchmark harness, which times the library's engines on problem sets |
| `deduxit-web` | `web/` | the bindings that compile the library to WebAssembly for a web page |
| `linlog`, `linlog-cli` | `compat/` | the crates under their names up to 0.2.0, which point to `deduxit` |

The library must run in a browser. So it reads no clock, and it uses
threads only behind its `parallel` feature. The bindings leave that
feature off: the flake builds them for `wasm32-unknown-unknown` with
every other feature.

The library holds three suites side by side: `linear` (linear logic),
`ordinary` (propositional logic) and `lambda` (proofs read as
lambda-terms). What they share stays at the root: the one error type,
the limits a long call takes, the settings a front end holds, the
exports and the JSON forms.

## Sequents and occurrences

A sequent is a compact arena of its subformulas in negation normal
form, one-sided: `Γ ⊢ Δ` becomes `⊢ Γ^⊥, Δ`. There is no second
representation for the other readings:

- An intuitionistic sequent is lowered once into the same arena, with
  the side of every subformula recorded. Every output reads it back
  two-sided through those sides.
- A classical sequent's text keeps, beside the arena, one byte per
  subformula occurrence saying how it was written: its side of `⊢`, and
  `A ⊸ B` against `A^⊥ ⅋ B`. Derivations are drawn two-sided from it.

On top of the arena the library detects the fragment the sequent lives
in, and builds the occurrence forest. The forest numbers every place a
subformula occurs. Every engine, the checker and the proof nets work on
those numbers, and errors name occurrences by them.

## Proofs and the checker

A proof is a compact term over the occurrences, one node per rule
instance. An independent checker decides whether a term proves its
sequent. It shares no code with the search. A derivation view unfolds a
term into the tree of explicit sequents of the standard sequent
calculus, one-sided or two-sided, for printing and the exports.

A proof net is the same forest with axiom links. It has its own
correctness criterion, and converts to and from a proof term.

Interactive proving holds a derivation with open goals over the same
forest, with the same inferences as the derivation view. Once it is
finished, it turns back into a term for the checker.

## Search

The search decides a sequent, or any goal within one, with the engine
its fragment and mode call for: the focused sequent search, the net
search, the additive recursion, the inverse method or the search of a
Petri net's markings. [How deduxit searches](engines.md) describes them.
The search returns a checked proof, a refutation that a check of its own
confirms, or the reason it could not tell.

Every long call takes the limits it may use and a stop closure, which it
asks regularly and tells the progress made. The library has no clock: a
caller with one stops on time (`Settings::stop` builds the command's
stop), and a caller without one, such as a browser tab, stops on the
work done.

## The other suites

Ordinary logic has an arena and a syntax of its own. It decides
intuitionistic and minimal logic by LJT, a calculus whose search always
ends, and every logic through an embedding: it translates the sequent
into a linear one, searches that, and reads the linear proof back as a
derivation of LK or LJ. A checker of its own checks that derivation by
the rules of the logic. Behind the feature `sat`, a SAT-based engine
decides classical, intuitionistic and minimal logic too.

The lambda suite reads an intuitionistic proof back as a term: a proof
of intuitionistic linear logic as a linear lambda-term, a derivation of
LJ as a simply typed one.

## Outputs

The exports write sequents and derivations, finished or not, as LaTeX
and Typst source, one inference at a time, so they hold one inference's
text at once. A derivation of LK or LJ goes through the same writers as
a linear one.

SVG drawings come from layouts of deduxit's own. A tree is laid out by
the widths of its subtrees. A net is laid out by its formula trees under
the axiom links, each subformula written where it stands in its
conclusion. PNG and PDF render the SVG with fonts the caller gives.

A finished derivation becomes a Rocq certificate for one of two kernels.
For NanoYalla it is a proof script, one derived rule per inference. The
kernel's sequents are lists, so the writer tracks the order of each
goal's formulas, and adds an exchange only before a `⊗` whose goal is
not already in the order the rule needs. For deduxit's own Rocq library
it is the sequent, the proof term and one computation, in which the
library's checker accepts the term.
