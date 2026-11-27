# Unit 1a: verdicts

Verifier: Fable 5.2, medium effort. Every place was opened in
`/home/tux/Projects/own/deduxit-wt-audit`. 18 confirmed, 2 candidates,
none rejected.

## F-1a-01: confirmed

The witnesses are at their lines: `Sequent::new` (`sequents/mod.rs:217`)
is the empty sequent, `from_parts` (:414-415) is `pub(crate)` behind
`parse`, `raw` (:991) is test-only; `ordinary/translate.rs:587` writes
the private fields, `families.rs:1110` parses text, and ordinary and
modal logic have `Formulas::add`, `Formulas::atom` and
`Sequent::new(formulas, left, right)` at the lines given. `Term`'s
variants and `TermId::new` are public, so the constructor of the sequent
is the one missing piece. X1 says exactly this.

A recorded choice stands against it: `core.md:495`, "An item without a
caller waits for one (the checked builder, …): it is additive later".
Its reason no longer holds: the consumer test (point 1) is the caller.

Corrections: `protocol/linear.rs:455` calls the crate-private
`from_parts`, which is a use, not a workaround. If `Sequent::new` takes
arguments, `new` and `Default` no longer agree (C-CTOR); ordinary and
modal `Sequent` have no `Default`, so the proposal must say which way
the linear one goes. Cost medium is honest only if the families and
`translate.rs` are rerouted later; with them it is large, and the
families' arenas must stay byte-identical (the finder says so).

## F-1a-02: confirmed

`fo.rs:9` is the quoted re-export, `first_order` is a private root
module (`lib.rs:333`), and `public-api.txt` lists no accessor of
`ordinary::Formulas` for a symbol, a term or an argument list
(`Node::Atom(u32)`, `atom_name(u32)` only), while ordinary's own public
items already carry `linear::sequents::FoTermId` (`Inference::term`,
`Step::witness`, `herbrand::Instance::terms`). `core.md:41-44` records
the layout ("the linear suite re-exporting the terms' public types under
`linear::sequents::fo`") without a reason, and D4 decides against it.
The types are also reachable as `linear::sequents::X` and
`linear::Eigen`, so "only under `linear::sequents::fo`" is loose, but
every path is the linear suite's. Cost large is honest.

## F-1a-03: confirmed

`notation.rs:20` imports `ordinary::{Formulas, NodeId, Symbols}`,
`Notation` holds `necessary`, `possible` and `ordinary: Symbols`
(:81-89), `Notation::ordinary` is at :1209, and ordinary, modal, the
exports, lambda and rules import `Stop`, `Stops` or `notation` from
`linear::sequents` (27 files name `sequents::notation`, `Walk`, `Visit`
or `fmt`). No rules file gives a reason for the place.

Corrections. Kind: `Stop`, `Stops`, `Notation` and `Atoms` are
`pub(crate)`, so their move is a refactor; only the paths of `Walk` and
`Visit` break, and the two new `walk` methods are additions. "Imports
ordinary only to hold its symbols" understates it: the file also writes
an ordinary sequent (`Formulas`, `NodeId`). `ordinary/written/mod.rs:673`
has a `Walk` of its own, a different type, which a root `walk` module
must not collide with.

## F-1a-04: confirmed

The 33 types are the unit's rows of `surface.md` ("Items reachable under
several paths"), counted again: 16 with three paths, 17 with two. The
`pub mod` lines and the re-exports are as quoted. Consumer-test point 9
is the same matter. `core.md:56-58` records that `linear/mod.rs`
re-exports its public types, with no reason for keeping the inner paths
public too.

Correction to the cause: `Forest`, `Member`, `OccId`, `Sign`, `Sequent`
and `Fragment` have two paths because `linear` re-exports from the
public modules `occurrences`, `sequents` and `fragment`, not because a
leaf module is public; the change ("one public path per type") covers
them. C-HIDDEN fits loosely; X9 is as close. Cost: medium in the crate,
but the paths are written in `cli`, `web`, `bench`, the manual and the
guide, so large across the workspace.

## F-1a-05: confirmed

Every fact holds in `public-api.txt`: `linear::Side {Input, Output}` and
`ordinary::Side {Left, Right}`; `linear::Sign {Atom, Dual}` and
`ordinary::Sign {False, True}`; `linear::NodeId` a proof node and
`ordinary`/`modal::NodeId` a formula node; `new`/`get`/`index` on the
linear ids, `index` alone on ordinary, modal and lambda ids, `new` and
`index` on `rules::TermId`; `Sequent::atom(&self)` a lookup (:449) and
`ordinary::Formulas::atom(&mut self)` a constructor (:926).
`Reading::position` exists (`reading.rs:563`).

Corrections. C-WORD-ORDER as `criteria.md` words it is about the order
of words in a name; the finding uses it as consumer-test point 8 does,
and X1 is the better tag. A `new` on `ordinary::NodeId` and
`modal::NodeId` lets a caller make an id of no node: `Sequent::new`
checks its ids (`formulas.holds`), but every other taker of a `NodeId`
must be checked before `new` is added, so that part is a question for
the API proposal, not a mechanical change. The value is real; the cost
is large and a wide break.

## F-1a-06: candidate

The quoted lines are at `occurrences/mod.rs:738-741`, and
`Term::operands` (`term.rs:273`) gives the two subterms without a
collection, as `parse/mod.rs:1038-1040` uses it. It is a `collect`
whose result is only iterated (P2). `dhat.md` has a frame that fits:
`with_capacity_in<TermId>` with 6 150 blocks (8 %) in
`check-wide-m1-2048` and 389 (11 %) in `search-chain-128`.

To measure: DHAT on those two journeys before and after (that frame's
blocks should go), and the ratchet's instruction counts. The gain in
instructions is likely small: `check-wide-m1-2048` has one block per
220 990 instructions. The figure of 27.8 million occurrences was not
checked.

## F-1a-07: confirmed

`malformed` is at `instances.rs:513-518` as quoted, it has 15 call
sites in the file, the message is `errors/mod.rs:167`, and
`Proof::with_instances`'s own docs (`proofs/mod.rs:467`) promise
"`Error::Json` (form `instance table`)" for a table built in Rust. No
rules file records the choice.

Corrections. Criterion: C-GOOD-ERR (`Error`, `Send`, `Sync`, lower case)
is met by `Error`; the defect is X3's, a public constructor that
validates a shape and reports it as a wire form. The change as written
is unclear on one point: if the JSON reader "passes it through", a
malformed table in a proof file changes its code from `json` to the new
one, which is a change of what the command reports and of the
behaviour lock; the reader should map the new variant back to
`Error::Json`, or the change is made knowingly. `Error::CODES` and the
manual's list of codes grow by one.

## F-1a-08: confirmed

`dual_literals` is `pub(crate)` at `occurrences/mod.rs:595`, the
consumer's copy is at `consumer-test.md:219-222`, and
`core-forest.md:72-78` says a new site that pairs literals asks this
function. `core.md:495` ("an item without a caller waits for one") is
answered by the consumer test. Kind: an addition to the public API, not
a refactor. Its parts (`is_literal`, `atom`, `sign`) are public already,
so nothing new is exposed.

## F-1a-09: confirmed

`instances.rs:20` is `use crate::linear::proofs::Node;`, `canonical` is
at :551 and `Canonical` at :693-940, and the one caller is
`proofs/mod.rs:479`. `core-proofs.md:394` files the first-order proofs
under `occurrences/instances.rs` without a reason for the place.

Correction to the cost: `Canonical` reads and fills `Instances`'s
private fields (it builds `out: Instances` in place), which works
because it is in the same file. After the move those fields or a set of
crate-private builders must be visible to `proofs`, so the change is
more than a cut and paste; medium stands. The gain is for a reader
only.

## F-1a-10: confirmed

Both loops were read side by side: `instances.rs:1036-1069` and
`first_order.rs:299-329` are the same explicit-stack walk and differ in
how they read a term and a list (which `unify::Terms`, `unify.rs:19-45`,
already abstracts for both stores) and in what they do with `Eigen`,
`Meta` and `Bound` (closures in one, fixed in the other). One generic
writer with the three closures serves both; both take the symbols' names
from a `FirstOrder`. `unify` is behind `interactive`, so the trait must
move to `first_order.rs` for the writer to exist without that feature,
as the finding says.

## F-1a-11: confirmed

The two-sided layout is at `notation.rs:998-1017` (`classical`),
:1110-1136 (`framed`), :1179-1202 (`two_sided`) and :1231-1248
(`ordinary`); the one-sided loop at :1022-1029, :1063-1071 and
:1097-1105; the mark pair at :987, :1054 and :1217. The four differ in
the type of an item, in whether the right side is a list or one goal,
and in how one formula is written, which a closure carries.

Caution for the change: `framed` is `#[cold] #[inline(never)]` on
purpose and the writers are generic over `W`, so the shared function
must not pull the cold path into the hot one; the ratchet's `render-*`
and `read-text` counts and the web module's bytes decide whether the
shape is kept. The gain is for a reader.

## F-1a-12: candidate

The sites exist: `notation.rs:236`, `first_order.rs:598`,
`instances.rs:947`, `unify.rs:15`, `modal/model.rs:273`,
`ordinary/model.rs:299`, `ordinary/normal.rs:71`,
`ordinary/instance.rs:771`, `proofs/interactive/quantified.rs:959` and
:984. The finder missed one: `core/src/linear/proofs/size.rs:30`
(`use std::collections::{HashMap, HashSet};`, library code, `pub mod
size`). `search/reference.rs` is test-only, as said. There is no
`clippy.toml`, and `hash.rs`'s module doc says "The hasher every table
in this crate uses", which these sites contradict.

The sites and the missing lint are facts; that the machine gains is
not: P6 speaks of hot tables and the finder calls most of these cold.
To measure: the ratchet's `ordinary-pigeons`, `render-*` and
`derivation-chain-64` counts before and after. Two things to check in
the change: a site that iterates its table changes its order from
random per process to fixed (an improvement, but the lock must be
read), and `disallowed-types` must not fire on uses of the aliases
`crate::hash::{HashMap, HashSet}` (it should fire only where the
aliases are defined; confirm with the lint run).

## F-1a-13: confirmed

`ill.rs:188-190` and `written.rs:252-254` are
`self.forest.sequent().clone()` under `into_sequent(self)`. `Forest`
owns `sequent: Sequent` (`occurrences/mod.rs:254`), has no `Drop` and no
`into_sequent`, so moving the field out is sound. `core-forest.md:79`
("the forest owns a clone of its `Sequent`") is about the forest's own
copy and is untouched. Kind: `Forest::into_sequent` is a public
addition. The saving is one arena copy per call, at seven call sites.

## F-1a-14: confirmed

`sequents/mod.rs:449-454` compares the raw name; the parser
(`parse/mod.rs:347`), the JSON reader (`serialize/sequents.rs:274`) and
`ordinary::Formulas::atom` (:927) compose first, and
`core-sequents.md:285-292` and :610 say names are in NFC "on every
path". No caller in `cli`, `web`, `bench`, the manual or the guide
passes a decomposed name; the callers found are tests and doc examples.

Correction to the kind: composing before the scan breaks nothing, it is
a fix of behaviour (a decomposed name finds its atom). Only the rename
to `find_atom` is a break, and that belongs to F-1a-05.

## F-1a-15: confirmed

`parse/mod.rs:1002-1006` is the quoted match with `other => other`
over `Term`, and the file has no `deny`. `core.md:318-325` gives the
rule and its reason (F85) for `sequents`, `occurrences`, `nets`,
`export` and `ordinary`; `parse` is not in the list, and nothing says it
was left out on purpose. A new binary connective would keep its
operands' order in the cyclic dual without a compile error.

Corrections. The criterion is the recorded rule of `core.md` more than
E7 (which is about the workspace's allowed lints). The cost: a `deny` at
the top of `parse/mod.rs` also meets the file's other catch-all arms
(:228, :527, :550, :607, :652, :780, :883, :971, :1220, :1223), most of
them over tokens and options, where a wildcard is right; each needs a
look or an `#[expect]`. The cheap form is the `deny` on `dualise` alone,
or spelling out the arms there.

## F-1a-16: confirmed

`fragment.rs:190-213` and :220-246 are the same scrutinee and twelve
arms, each name of the second with an `I` in front;
`duplicates.md` lists the pair. `NAMED` and `NAMES` exist (:89, :106).
Both functions are `const fn`, so the shared index must be one too,
which the change says. The gain is small and for a reader.

## F-1a-17: confirmed

The four lines are at `parse/mod.rs:556-559` and :659-662 in full.

Correction: the first place is only partly the same. At :478-479 the
arity and the list are made for both branches; `self.symbol(…, false,
…)` and `FoTerm::App` follow only when an equality follows (:482-483),
and otherwise the symbol is read as a predicate (`true`, :486) and
the list goes to `predicate_atom`. One helper serves two places whole
and the third in one branch. The value is tidiness alone, at small
cost; the parser's cold functions are `#[inline(never)]` by a recorded
choice (`core-sequents.md:441`), which the helper should follow.

## F-1a-18: confirmed

The comment is at `sequents/mod.rs:325-326` as quoted, and it does not
say what "it" is or why the parser's path matters here. No rules file
explains it.

Correction to the proposed text: "the only caller of `entry` on this
map" is wrong, since `known` is a local of `push_cuts`. The other user
is `optimize_terms` (:699, :708) with its own map of the same type,
`HashMap<Term, TermId>`. Say: "`insert`, not `entry`: `optimize_terms`
is the only user of `entry` on a map from `Term` to `TermId`, so the
compiler inlines it there, on the parser's path; a second user here
would make it a call." Whoever writes it should confirm that reading
with the author or a count of `read-text`.

## F-1a-19: confirmed

`fmt.rs:210` is `"⊢ a ⊗ !b".parse().unwrap()` in the example of
`Sequent::walk`, and it is the only doc example of the unit that
unwraps a `Result`; the unit's other `unwrap`s in examples are on
`Option`s (`occurrences/mod.rs:239`, :246, `reading.rs:302`, :719).
`Reading::walk`'s example shows the form to use. C-QUESTION-MARK says
this.

## F-1a-20: confirmed

`fmt.rs:165` is the quoted alias, `Walk::new` (:170) is `pub(crate)`
with two callers (`fmt.rs:234`, `reading.rs:734`) whose closures capture
`&Sequent` and `&Reading` (a `&Forest`, a `Cow<[Side]>`, two scalars),
and the one `Send + Sync` assertion in the crate is `Error`'s
(`errors/mod.rs:1300`). I did not compile the bound; the build is the
check, and the `const` assertion the change adds keeps it. The value is
small: a walk is short-lived and made from a `Sync` value on the thread
that needs it.

## Beside the findings

- `Sequent::add` and `Sequent::add_cut`
  (`core/src/linear/sequents/mod.rs:851-852`, :863, :869, :294-295,
  :305) offset term and atom indices with `len() as u32` and an
  unchecked `+`. Two sequents whose arenas together pass 2³² terms give
  wrapped indices in a release build (a wrong sequent, silently) and a
  panic with overflow checks on. It needs an arena of about 48 GB, so it
  is out of reach in practice, but `add` returns no `Result` and no
  rules file records a bound. Seen in the finder's notes and confirmed
  by reading; not run.
