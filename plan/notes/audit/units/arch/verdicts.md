# Unit arch: verdicts

Verifier: Opus 5.5, high effort. Every place below was opened at the
audit's commit in `/home/tux/Projects/own/deduxit-wt-audit`. Counts:
24 confirmed (several with corrections), 2 candidates, 0 rejected.

## F-arch-01: confirmed

`pub enum Reason` is at `core/src/linear/search/mod.rs:3481`, and the
quoted `PathLength` variant is at `:3557-3562`. `ModelSize` (`:3566`) and
`KripkeSize` (`:3576`) are there too. `modal/decide.rs:14` imports it.
It stands in `ordinary::Verdict::Unknown`, `modal::Verdict::Unknown` and
`lambda::Inhabitation::Unknown` (`public-api.txt`). `ordinary/decide.rs:354`
(`ended`) maps a `Refusal` to a `Reason`. `core.md` (Layout) says an item
that a second suite comes to share "moves to the root then". No rules
file gives a reason to keep `Reason` in `linear`: `core-modal.md:270`
only records that the modal suite took it "as it is". Aside: the enum's
JSON doc (`:3470-3477`) does not list `model_size` or `kripke_size`
among the kinds. Fix that when the enum moves.

## F-arch-02: confirmed

The fields are at `linear/search/mod.rs:3016`, `:3022`, `:3029` and
`:3037`, and each doc says only the ordinary prover or search reads it.
`decide_within` (`ordinary/decide.rs:748-756`) takes `search:
&search::Options` beside its own options. `core-search.md:478-489` and
`core-ordinary.md:909` record which engine owns each field. Neither
gives a reason for keeping the fields on the linear options.
Correction to the cost: renaming the settings keys also breaks stored
documents. A workspace recipe keeps its settings, and a record's key is
the projection of those settings on `reads`
(`core-workspace.md:25-32`). The page's links carry the settings too.
So old documents need an upgrade path (a wire level that maps
`search.path_length` and the other three keys to their new names), or
they stop reading under `deny_unknown_fields`. The cost stays large.

## F-arch-03: confirmed, with a correction

The `Refusal` half holds. Every variant of `Refusal` is
`#[non_exhaustive]` (`limits.rs:593-705`). The public API has no
function that returns a `Refusal`: it offers only `From<Refusal> for
Error`, `code`, `setting` and `is_stop`. So an outside engine cannot
refuse in the crate's family.

The `Progress` half is wrong. `Progress` derives `Default`
(`limits.rs:405`), and all five of its fields are `pub`
(`:410-423`). `#[non_exhaustive]` forbids a struct expression outside
the crate. It does not forbid
`let mut p = Progress::default(); p.work = n; p.done = d; p.phase = Phase::Search;`.
So an outside engine can make a `Progress` and call any stop today. The
consumer test's point 2 tried only a struct expression. The same error
is in `architecture.md` section 5 ("poll the stop … no").

Corrected change: add `Refusal::stopped`, `work` and `memory` (the gap).
A public `Progress::new` is only a convenience. Moving the meter to
`limits` is also recorded in `plan/later.md:2735`. Making it public
gives outside engines the same counting. The cost stays small.

## F-arch-04: confirmed

`pub(crate) trait Decide` is at `linear/search/mod.rs:1477`. `Task`
(`:1362`) and `Answer` (`:1416`) are `pub(crate)`. `Outcome` (`:3389`)
is `#[non_exhaustive]`, has no constructor and has no `Default`, so it
cannot be built from nothing. Points the finder did not weigh:

- `Engine` is the list that `Options::engine`, the command's `--engine`
  and the outcome's JSON read. An `Engine::External` variant must be
  refused as a forced engine, kept out of the documented dispatch table
  (`the_documented_dispatch_is_the_dispatch`), and added last, since
  `Engine` is fieldless (`core.md`, "The break against the last
  release").
- `core.md` (Decisions) says "An item without a caller waits for one".
  The caller here is the manual's page of X8, so the interface should
  land with that page.
- `Outcome`'s fields are `pub`, `checked` among them
  (`linear/search/mod.rs:3408`). A crate can take any outcome a call
  returned and set `verdict` and `checked` itself. So `prove_with` makes
  the checker the trust boundary only once `checked` is no longer a
  writable field (see F-arch-23).

The cost stays medium.

## F-arch-05: confirmed, with corrections

The witness holds. `Sequent::from_parts` is `pub(crate)` and behind
`parse` (`linear/sequents/mod.rs:414-420`), and `Sequent`'s only
`&mut self` builders are `add`, `add_cut` and `optimize`.
`ordinary::Derivation::from_parts` is `pub(crate)`
(`ordinary/derivation.rs:605`). The `lambda` ids have only `index()`
(`lambda/mod.rs:212-251`).

Corrections:

- A `lambda::Term` is also made by read-back
  (`linear::Derivation::term`, `ordinary::Derivation::term`) and by
  `inhabit` and `enumerate`, not "only from text or JSON". The claim
  holds for a term built by hand, and so does the finding.
- A design for the linear builder is already on record:
  `plan/notes/api.md:354` (R59), `Sequent::builder()` with `atom`,
  `term`, `dual` and `finish(&Limits)`. It hash-conses as the parser
  does, so a built sequent equals the parsed one. The recorded reason it
  waits ("additive, so it waits for its first caller") is answered by
  X1. The change should follow R59. A bare `push` would need
  `optimize` afterwards to keep `core-sequents.md`'s invariants:
  distinct atom names, and equal sequents for equal text.
- `Sequent::check` is compiled only under `any(test, parse, serialize)`
  (`:585`). The builder must work under `default-features = false`, so
  its check must not carry that gate.
- `ordinary::Derivation::from_parts` makes a trusted derivation
  (`core-ordinary.md:678-686`), and `Derivation::term` relies on that
  flag to avoid panics. A public constructor must make an untrusted one,
  as the finder says.

## F-arch-06: confirmed

The places are as quoted. `Account` is at `linear/search/memory.rs:21`
and `Drawn` at `linear/proofs/style.rs:231`.
`linear/sequents/notation.rs:20` imports `crate::ordinary::{Formulas,
NodeId, Symbols}`. `Stop` and `Stops` are imported at
`ordinary/mod.rs:154` and `modal/mod.rs:71`. `Account` is named by
`ordinary/ljt.rs`, five files of `ordinary/sat/`,
`ordinary/resolution/outside.rs` and `modal/search.rs`. `Drawn` is
implemented in `ordinary/derivation.rs:1149`, `lambda/draw.rs:597` and
`rules/draw.rs:276`. `plan/later.md:2730` records the move as due, and
it has not happened. One correction: `Stops` is defined in
`linear/sequents/fmt.rs:57`, not in `notation.rs`. The finding's
"Value" leans on "a feature per suite", which the author ruled out
(`plan/README.md:5574`). The move stands without that argument.

## F-arch-07: confirmed

All seven `public-api.txt` lines are as quoted (`:17`, `:395`, `:5285`,
`:5287`, `:5369`, `:5458`, `:5492`). The re-exports are at
`linear/sequents/mod.rs:21` and `linear/proofs/mod.rs:47-52`.
`ordinary/parse.rs:15` imports `crate::linear::lltp::{Clauses, Status,
clauses}`. `core.md` (Layout) records the terms' public path under
`linear::sequents::fo`, and `kernel-boundary.md` 6.1 kept the other
paths until a split. Both reasons rested on changing no public path,
which D2 and D4 now override. Cost large, as stated.

## F-arch-08: confirmed

`duplicates.md` lists 5 pairs between the arenas, 6 between the
parsers and 6 between the serialize files: 17 in all. The two `add`
bodies (`modal/mod.rs:209-214`, `ordinary/mod.rs:864-871`) are the same
code. `core-modal.md:263-272` records the copy as made "for the kernel
crate". That crate ended at its gate (`plan/later.md`, "The kernel
crate: ended at the gate"). The trigger in `later.md:2731` (step 74's
quantifiers) has landed: `ordinary::Node::Forall` and `Exists` are at
`ordinary/mod.rs:742-744`. A shared `Arena<N>` must leave room for the
ordinary atom's first-order arguments and the symbol table, which modal
lacks. The `operands` trait the finder proposes allows that.

## F-arch-09: confirmed

The four files hold 2 213 lines (`wc -l`). `rules/search.rs:14-15`,
`rules/ordinary.rs:131` and `ordinary/written/mod.rs:32` are as quoted.
`rules/mod.rs` also names `ordinary`, in docs and through `mod
ordinary`. `core-rules.md:25-29` records where the files are, but gives
no reason for keeping them in `rules`. C-CONV-SPECIFIC fits:
`rules::Derivation::from_ordinary(&ordinary::Derivation)` converts from
the more specific type. Moving `close` and `hint` keeps them as
inherent methods of `rules::Interactive`, written in an `impl` block
inside `ordinary`. That needs the session's fields made `pub(crate)`.
`core.md`'s API list names `Derivation::from_ordinary` and
`Derivation::natural`, so those docs change too. The gain is mostly
layering. The cost of medium is fair.

## F-arch-10: confirmed, with a correction

The doc at `protocol/mod.rs:354-355` and the types are as stated.
`Kind`, `Operation`, `Parameter`, `Promise` and `Check` are
`#[non_exhaustive]` without constructors, `Produced::done` is private
(`:259-262`), and `SUITES` is a fixed static (`:418`). An outside impl
compiles, but it can return only empty kinds and errors, and nothing
runs it.

Correction: the trait is unsealed on purpose. `plan/notes/63g-design.md:814-816`
says "The `Suite` trait is not sealed … a suite from another crate is
one of the cases step 63's prompt names." So "seal it for 0.9.0"
reverses a recorded decision. It goes to the author, and the record
favours the finder's second option: constructors and a `Registry`
value. The mismatch between the doc and the types is a finding either
way.

## F-arch-11: confirmed

`OPERATIONS` (`:432`), `KINDS` (`:505-529`), `SUITES` (`:418`) and
`statement`'s `match` with its own `known` list (`:706-734`) are as
stated. `KINDS` names no `lambda.*` kind, while `protocol/lambda.rs:29`
has `static KINDS: [Kind; 5]`. `core-protocol.md:173-175` records this
as a gap and gives no reason to keep it. `KINDS` is only the `known`
list of `UnknownName` (`:576`). The lookup reads the suites, so the
gap affects only the error message. There are 13 duplicate rows among
`protocol/*.rs`, as stated (2 named plus 11). Adding `statement` to
the unsealed `Suite` with a default body is additive.

## F-arch-12: confirmed

The places are as stated:

- in `web/src/lib.rs`, `start` (`:97`), `candidates` (`:109`),
  `ordinary_start` (`:296`), `ordinary_candidates` (`:308`) and
  `rules_candidates` (`:557`);
- in `web/src/protocol.rs`, `pub const SUITES: [Speaks; 5]` (`:151`),
  `offered("candidates", "rules_candidates", GOAL)` (`:332`) and
  `lacks("candidates")` (`:378`);
- the answer form at `web/src/calls.rs:640-645`;
- `is_complete` at `linear/proofs/interactive/mod.rs:905` against
  `is_finished` at `rules/interactive.rs:430`, and `goals()` as an
  iterator at `:882` against a `Vec` at `:416`.

Nothing on record stands against it. `web.md:87-90` expects the step
forms to move into the library ("When the library gives them one …
the bindings take it"), and `web.md:38-44` (nothing kept between
calls) fits a session over JSON. One cost the finder did not name: the
command's `interact` works on typed sessions (`cli/src/interact/`).
Making it read a JSON trait is a larger change than the bindings'
switch. Cost large, as stated.

## F-arch-13: confirmed

The fields are at `ordinary/decide.rs:72`, `:79` and `:85`, and
`modal/decide.rs:106` has the modal one. The reader refuses a
countermodel beside another verdict (`:50`), and the crate itself
clears the fields when it changes a verdict (`decide.rs:797-801`,
`interactive.rs:2637-2639`). The linear `Unprovable(Box<Disproof>)`
carries its evidence in the variant. Since `Outcome`'s fields are
`pub`, a user can also write a model beside `Valid` today, and the
proposed enum removes that state. `ordinary::Verdict` is in `core.md`'s
marked list, so a changed variant is a 0.9.0 break, which D2 allows.

## F-arch-14: confirmed

`Settings` is at `settings.rs:129-153`. `search`, `view`, `batch`,
`interactive`, `input` and `cut` have no suite prefix, `ordinary` and
`lambda` have one, and no modal field exists. `Error::Step` (`:302`)
sits beside `OrdinaryStep` (`:309`). The same correction as for
F-arch-02 applies, and more strongly: every settings key moves, so
stored workspace documents and page links need an upgrade of their
settings. The protocol's `reads` lists, which are cache keys
(`core-protocol.md:34-42`), move as well. The value is consistency
under D4 alone, at a large cost.

## F-arch-15: confirmed

`web/src/lib.rs:97` is `start` and `:296` is `ordinary_start`.
`protocol/mod.rs:417-420` lists `&linear::LINEAR` first, and `:732`
has `known: &["linear", "ordinary", "modal", "lambda"]`. No rules file
records a reason for this order. The doc says it is "the order a front
end shows them", which D4 sets as ordinary first. B6 makes the
renaming of exports a joint change with the client, as stated.

## F-arch-16: confirmed, with a correction

`lib.rs:350-351` and `modal/mod.rs:42-64` are as quoted.
`modal/mod.rs:20-21` says deciding needs `serialize`. Correction: the
finding does not answer the recorded reason in `core-rules.md:33-37`:
"a calculus is made only by reading a rule file … so without serde
there is nothing to check against". That reason has partly lapsed,
since `modal::calculus` builds its calculus in code
(`modal/calculus.rs:169-170`, `Calculus::from_form(form(system), …)`).
The forms it builds are still the serde types of `serialize/rules.rs`,
though, and the built-in files `LK` to `NK_LEM` are JSON text
(`rules/mod.rs:102-152`). So the change has to:

- move the rule forms out of the private `serialize` module, with
  `cfg_attr` on their derives;
- leave the built-in rule files unavailable without `serialize`;
- say both in the crate docs.

The cost is medium to large, not medium.

## F-arch-17: candidate

`default-features = false` does keep many engines. `linear::search`
(`linear/mod.rs:25`) and the ordinary `connection`, `ljt`, `normal` and
`resolution` modules carry no `cfg`. `timings.md` gives 40 s and 16 s.

Correction: it does not keep "every engine of every suite". The SAT
engine, grounding and the finite model search need `sat`, the modal
tableau needs `serialize`, and the signed tableau needs `interactive`
(`core.md` Layout). Two more things stand in the way. The
`interactive` sessions' `close` and `hint` and the protocol's
operations call the searches, so `interactive` and `protocol` would
have to imply `search`. And without F-arch-05 the cut leaves no linear
sequent to work on.

To measure before proposing: the share of the 16 s that the engine
modules take (`cargo build --timings` on a copy with them compiled
out, or `cargo llvm-lines` grouped by module path).

## F-arch-18: candidate

The arithmetic holds: 2 987 284 − 2 438 116 = 549 168. D29 is
`plan/README.md:1287` ("one wasm module until a measured size or a
suite from another repository asks for a module per suite loaded
lazily"), not `plan/web/design.md`'s D29, which is about autosave.

Corrections:

- **The bytes were attributed before.** `twiggy` ran at 54x6
  (`plan/later.md`, "The web module's size after the workspace"). It
  found about 1.35 MB raw from the protocol's dispatch and the serde
  code that only the workspace's reads reach. Its conclusion was that
  "a module per suite (web-front-end-4) gains little", while a module
  for the renderers was the larger gain. That module was then done.
  The same note names the trim already on the author's list: "a
  registry the bindings build from the page's list". That is the
  finder's step D.
- **"One obstacle" is too few.** `kernel-boundary.md:308-311` says a
  module per suite "asks for a cargo feature per suite and meets the
  same sums", and `plan/README.md:5574` records "No feature per suite".
  `architecture.md` 6.1 itself lists three obstacles.
- **The first paint does not wait for it.** The page loads the module
  in its worker after first paint (`web.md:632-634`). It is still
  downloaded before the first proof.

To measure: `twiggy` again on today's module (step A, ten days after
54x6), and `-Oz` or `opt-level = "s"` against the calls' time in Node
(step B).

## F-arch-19: confirmed

The file has 5 552 lines (`wc -l`). Its item heads are at the lines
given: `Task` at `:1362`, `Decide` at `:1477`, `dispatch` at `:1515`,
`Engine` at `:1976`, `Bias` at `:2400`, `Options` at `:2855`, `Outcome`
at `:3389`, `Reason` at `:3481`, `Statistics` at `:3729` and the tests
at `:3846`. The module doc (`:4-16`) describes the front door. The gain
is for readers. A pure move inside one crate should leave the target
sets' counters unchanged, as stated.

## F-arch-20: confirmed, with a correction

The seams are as stated. `ordinary/mod.rs` holds the vocabulary (`Logic`
at `:167` to `Options` at `:596-707`), then the arena (`NodeId` at
`:711` to `Sequent` at `:1670`), then the tests (`:1773`). The
`Interactive` impl begins at `ordinary/interactive.rs:1069`.

Correction: `ordinary/interactive.rs` has 5 900 lines at the audit's
commit (`lines.md`, `baselines.md:300`). The 5 258 in the finding is
the 0.8.0 count. `ordinary/mod.rs` has 2 374 lines, not 2 379. The gain
is for readers, at a cost of medium.

## F-arch-21: confirmed

`modal/model.rs:18` is `use crate::ordinary::MODEL_CELLS;`. The model
has `impl Model` blocks in both suites (`ordinary/model.rs:175`, with
`forcing` and `refutes`; `modal/model.rs:176`, with `frame_fault` and
`truth`). `ordinary::Countermodel::check` (`ordinary/model.rs:55`)
gives a second way to check. `core.md` (Layout) records the model as
"data only: each suite checks it by its own rules", and the change
follows that record. It does not go against it.

## F-arch-22: confirmed, with a small correction

`export/mod.rs:85` and `core/Cargo.toml` (`latex = []`, `svg = []`,
`typst = []`) are as quoted. The counts are 91, 82 and 85 `cfg` lines,
258 in all. There are 53 lines with `any(` that name one of them, not
51. 66 − 45 = 21 pairs.

`web-render/Cargo.toml:18` asks only for `pdf` and `png`, which imply
`svg`. But it depends on `deduxit-web-tab`, which asks for all three,
so in effect every consumer builds all three. No rules file records a
reason for three separate features. The promise "No feature is removed
within a 0.y version" (`lib.rs:309`) allows the change at 0.9.0.

## F-arch-23: confirmed, with a correction

The quotes are at `ordinary/derivation.rs:507-510` (a private flag with
no getter) and `linear/search/mod.rs:3404-3408`.

Correction to the proposed doc sentence. It says "a value in a verdict
the crate returned has passed the checker unless the outcome's
`checked` is false". That is not true of a linear outcome read from a
document. Its `checked` is whatever the document says
(`serialize/search.rs:929`, and the field's own doc). Because the field
is `pub`, any holder of an outcome can also set it. Under X3 ("No public
path makes a value that reads as checked without the check"), the
change should:

- make `checked` false, or private with a getter, for an outcome that
  was read rather than returned by a search;
- say on `Outcome` that a read outcome is unchecked whatever the field
  says.

No consumer in `cli/src`, `web/src` or `core/src/protocol` trusts the
read flag, so no verdict is wrong today. The kind becomes text plus a
small API change.

## F-arch-24: confirmed

`duplicates.md` has 13 rows between the two `prove.rs` files. The
`on_path` bodies at `connection/prove.rs:652` and
`intuitionistic/prove.rs:707` are word for word the same.
`core-ordinary.md:924-925` records `prove.rs` as "the search (a copy
of the classical one, changed)", with no reason to keep two copies. By
P13, `connection::Statistics` must stay equal on the comparison rows,
or this is a search change. The ratchet has no journey through these
provers, as stated.

## F-arch-25: confirmed, narrowed

The 90-line pair (`latex.rs:480` ↔ `typst.rs:550`) is real code:
`sequent`, both model tables and `PER_OCCURRENCE`, which differ only in
doc words and one bracket comment (checked with `diff`). The dispatch
at `latex.rs:677-684` repeats as well.

Two of the six pairs are not code to share:

- `:22` ↔ `:22` is the two modules' doc examples, with different
  expected outputs;
- `:265` ↔ `:305` is the docs of two public `Options` types.

The claim "the same code over two spellings" also goes too far for the
trees. `core-export.md:168-221` records that ebproof's postfix order,
curryst's nesting and Typst's own layout above `CURRYST_HEIGHT` are
different emitters. Corrected change: share the sequent writer, the
model tables and the `Drawable` dispatch in one module over a table of
spellings, and keep the tree emitters per format. The cost is small to
medium.

## F-arch-26: confirmed, kind corrected

`Drawable` (`export/mod.rs:233-254`) is a closed list of the crate's
derivations. Each arm but Fitch calls `tree` over the crate-private
`Drawn` (`latex.rs:677-684`). X5 is met for a linear proof built
outside (`Proof::new`, then `derivation()`) and for a derivation in a
rule file. The gap is the docs' silence, so the kind is text, not
refactor. The trait's generic `write_sequent(&self, out: &mut impl
Write, …)` (`style.rs:266`) is not object-safe. That condition for a
later `Drawable::Other` is stated correctly.

## F-arch-27: confirmed

`lib.rs:18` is "A sequent of ordinary logic, here intuitionistic logic,
the default:". The crate docs import only `linear` in the sections at
`:111`, `:143`, `:167` and `:198`, plus the first example at `:41`.
They have no `modal::` or `lambda::` example. "Errors" (`:253-262`)
lists the linear suite's error types. `ordinary::Logic` has `#[default]
Intuitionistic` (`ordinary/mod.rs:171-172`).

## architecture.md

### Factual claims that are wrong at the audit's commit

- **"One crate, 207 688 lines"** (§1), and the per-module counts
  "`ordinary` (55 577)", "`lambda` (9 670)" and "`rules` (7 905)". These
  are the 0.8.0 figures. At the audit's commit
  (`baselines.md:285-293`, `lines.md`) the crate has 214 915 lines,
  `ordinary` 56 540, `lambda` 13 705 and `rules` 9 824. The §8 figure
  "13 665 of 207 688 lines" needs the same correction.
- **"`ordinary/interactive.rs` (5 258)"** (§2) is the 0.8.0 count. It
  has 5 900 lines now. `ordinary/mod.rs` has 2 374 lines, not 2 379.
- **"poll the stop … no (`Progress::new` is `pub(crate)`)"** (§5 table)
  is wrong. `Progress` derives `Default` with public fields
  (`limits.rs:405-423`), so an outside crate makes one by
  `Progress::default()` and field assignment. Only "refuse at a limit"
  is missing. Part 1's `Progress::new` is a convenience, not a gap.
- **"The checkers are four, sharing no code"** (§1) leaves several
  out. Others are `lambda::Term::check`, the type checker
  (`core-lambda.md`), and the clausal checks
  (`ordinary::resolution::Refutation::check`, `check_proof`). The finite
  and growing-domain structures have their own checks too
  (`ordinary/structure.rs:603`, `kripke_structure.rs:1019`).
- **"the walk `Stops` (`linear/sequents/fmt.rs`)"** (§1) is right. But
  F-arch-06's "`Stops` and `Notation` to a root module `notation`"
  implies they sit together, and they live in two files.
- **"no inventory attributes the module's bytes"** (§6.1) overlooks
  54x6's `twiggy` attribution (`plan/later.md`, "The web module's size
  after the workspace"). It is older, but it exists and it answers
  option D.
- **"Every consumer in the workspace enables all three or none"** (§8)
  holds only through feature unification. `web-render`'s own manifest
  asks for `svg` alone, through `png` and `pdf`, and gets `latex` and
  `typst` from `deduxit-web-tab`.

Checked and right: the counts of what one suite names of another
(`deps.md`: 62, 19, 8, 4), `Settings`' six linear fields of eleven, the
eight methods in one `match` at `ordinary/decide.rs:821`, `Reason`'s
thirteen variants, `Sequent::from_parts` and
`ordinary::Derivation::from_parts` being `pub(crate)`, the public
`Proof::new`, `Disproof::new`, `Countermodel::new`, `Formulas::add` and
`atom`, `Sequent::new` and `rules::Derivation::new`, the
`protocol::run` calls at `workspace/mod.rs:1062`, `:1084` and `:1743`,
the timings, the cfg counts of `parse`, `serialize` and `interactive`
(404, 402, 164), the two meters, the `nets` graph duplicate, and
`rules::Interactive::goal` panicking on a foreign id
(`rules/interactive.rs:436-441`).

### What stands against the proposals

- **The engine interface (§5).**
  - Part 1 needs only the `Refusal` constructors and the meter.
    `Progress` is constructible already.
  - `core.md` (Decisions) says "An item without a caller waits for
    one", so the interface lands with its caller, the manual page of X8.
  - `core.md` also says "A front end therefore never builds an error of
    the library". The proposal must keep `Refusal`'s constructors for
    engines and leave that rule standing for front ends.
  - `Engine::External` shows up wherever `Engine` is read: the forced
    engine, `--engine`, the JSON names, and the documented dispatch
    test. It must be refused as a forced engine and added last.
  - `Outcome`'s fields are `pub`. Until `checked` and `verdict` stop
    being writable, `prove_with` alone does not keep the checker as the
    trust boundary.
  - Part 2 should follow the builder design recorded in
    `plan/notes/api.md:354` (R59): hash-consed, with `finish(&Limits)`.
- **The crate's shape (§6).** The "feature per suite" the reader
  proposes runs against the author's decision of 2026-11-09, "No
  feature per suite" (`plan/README.md:5574`). `kernel-boundary.md:308-311`
  and `plan/later.md:2700` restate it. The reader's case against a
  split into crates agrees with `kernel-boundary.md` 4.2 and 4.3.
- **Modules loaded apart in the browser (§6.1).**
  - 54x6's measurement concluded that a module per suite gains little
    (`plan/later.md`, `plan/README.md:7403`), and the renderers' split
    it recommended was done.
  - The trim it left for the author is the registry the bindings build
    from the page's list. That is the reader's option D without
    separate modules.
  - The module loads after first paint in a worker (`web.md:632-634`).
  - D29 is in `plan/README.md:1287`, not in `plan/web/design.md`.
- **The features (§8).**
  - For `serialize`, the reader did not see the recorded reason in
    `core-rules.md:33-37`, which still holds for the built-in JSON rule
    files.
  - For the proposed `search` feature, `interactive` (`close` and
    `hint`), the `rules` session's search and `protocol` all call the
    searches, so each would have to imply it.
  - Nothing recorded stands against one `export` feature for `latex`,
    `typst` and `svg`.
- **The session over JSON (§7).** `web.md:87-90` supports it. The
  command's `interact` works on typed sessions, so having it "read the
  same trait" is a larger change than the bindings'. Nothing recorded
  stands against it.
- **The protocol's registry (§7.3).** `plan/notes/63g-design.md:814-816`
  records that `Suite` is unsealed so that outside suites can join. The
  open registry follows that record. Sealing would reverse it.

## Beside the findings

None. The linear outcome's `checked`, read as the document says,
matters for X3 (see F-arch-23). But nothing in `cli/src`, `web/src`,
`core/src/protocol` or `core/src/workspace` trusts it, so no verdict is
wrong today.
