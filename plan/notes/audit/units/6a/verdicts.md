# Unit 6a: verdicts

Verifier: Fable 5.2, medium effort. Every place was opened in
`/home/tux/Projects/own/deduxit-wt-audit`; the two command witnesses
(F-6a-01, F-6a-02) were run again with `target/debug/deduxit`.

## F-6a-01: confirmed

`serialize/sequents.rs:571-581` is the `TryFrom` with
`Limits::default()`, and `proofs.rs:303`, `nets.rs:123`, `nets.rs:313`
and `interactive.rs:259` call it while holding the caller's `limits`.
The witness reproduces: `prove 'p(f(f(a))) |- p(f(f(a)))' --term-limit 1`
is refused (exit 2), and `check --file` of its JSON outcome with
`--term-limit 1` answers "valid proof" (exit 0). The default bound
still holds on those readers, so the growth is bounded; what is lost
is the caller's lower bound. `core-sequents.md:363` records the default
`Limits` only for a plain `Deserialize`, not for `upgrade`. The change
is sound; the proxy `Sequent` is private, so dropping the `TryFrom` is
no API break.

## F-6a-02: confirmed

`wire.rs:434-441` is as quoted, and `cli/src/io.rs:402-406` is the one
caller outside the crate (with `:411` and a test in
`core/tests/serialize.rs:947`). The witness reproduces: an
intuitionistic JSON sequent of 5 occurrences under
`--occurrence-limit 2` prints "not an intuitionistic sequent in JSON:
…more than the limit of 2", without the "raise it with" hint the
one-sided path gives. `core-sequents.md:365-367` records that a
deserializer's error is text only and that `upgrade` answers the kept
cause, which is the finding's point, not a choice against it.
Correction: the kind is `API break` only if `Within` is removed; moving
the command to `upgrade` and documenting the seed's text-only error is
a refactor plus text. `cli.md:869` names `Within` for a stream from
standard input, so removal needs that caller looked at first.

## F-6a-03: confirmed

`Progress::new` is `pub(crate)` (correct place: `limits.rs:429`; the
`Refusal` enum is at `:593` with every variant marked
`#[non_exhaustive]`), and `consumer-test.md` attempts 1 and 3 fail with
E0639. The pattern `if let Some(limit) = limits.X && … > limit` occurs
37 times in `core/src`. `core.md` (Decisions) records "a front end
therefore never builds an error of the library: it asks the call that
does"; the proposed `Limits::admit_…` and `Refusal::stopped` are such
calls and keep the variants' fields hidden, so the recorded reason
stands. Additive for users (new public functions), so "API break"
overstates the kind: it is an addition plus a refactor inside the
crate.

## F-6a-04: confirmed

`lib.rs:350-351`, `rules/calculus.rs:12` and `:267`, and
`modal/mod.rs:61-64` are as quoted; `timings.md` gives 40 s and 16 s.
`core-rules.md:33` records the choice with a reason ("a calculus is
made only by reading a rule file … so without serde there is nothing to
check against"). That reason no longer holds: `modal/calculus.rs:252`
(`form(system)`) builds a `CalculusForm` in Rust and
`Calculus::from_form` at `:170` takes it with no deserializer, so a
calculus exists without a document, and the modal decision is gated
only because the proxy type lives in `serialize`. The finder did not
name the recorded choice; this is its answer. Cost large is honest; the
gain (modal logic and rule-checking without serde) is for a library
user who turns the defaults off, a small audience, and the exact cut is
the architecture proposal's.

## F-6a-05: confirmed

`lib.rs:107-225` and `:253-263` are as described: the sections at
`:111`, `:143`, `:167` and `:198` use `linear::…` alone, and the Errors
list names three linear types and `ParseError`. Correction: "every
walk-through section" overstates it; "What each suite offers, over
JSON" (`lib.rs:227-251`) runs an ordinary translation through
`protocol`, so four of the five sections after the two opening examples
are linear. The keywords are as quoted.

## F-6a-06: confirmed

Confirmed in part, with the change narrowed.
- The two paths hold: `surface.md:9-14` lists `Clock`, `Limits`,
  `Phase`, `Progress`, `Refusal` and `Settings` at the root and under
  `limits::`/`settings::`, while `errors` is private (`lib.rs:330`).
  One path per item is sound; the gain is tidiness.
- `Owner` is implemented by linear types only
  (`errors/describe.rs:126-206`), so moving `Owner` and `Described`
  under `linear` is possible, though `core.md:58-60` lists both as root
  re-exports and `Error::describe` is a method of the root's error.
- Moving `StepFault` and `Backwards` to `rules` is unsound and that
  part is rejected: `StepFault` is the payload of `Error::StepRefused`
  (`errors/mod.rs:724`), which `ordinary/resolution/refutation.rs:146-160`
  builds without the feature `serialize`, while `rules` exists only
  with it (`lib.rs:350`). The type would be unnameable in a build
  without `serialize`, and `core.md`'s rule sends an item two suites
  share to the root, which is where it is. `Backwards` is a field of a
  `StepFault` variant and stays with it.

## F-6a-07: confirmed

`serialize/errors.rs:238-274` is as quoted and `Why`
(`serialize/search.rs:53-175`) is a field-for-field copy of `Reason`
with two converters, which a derive on `Reason` would replace byte for
byte. Corrections:
- "a derive would write the same bytes" is not true of every arm:
  `Fault::Shape(e)` nests a tagged enum under the key `shape` (an
  internally tagged derive would merge the two `kind`s),
  `ShapeError::Formula` drops a field, `NetError::NoVertex` and
  `NoCut` write a raw and an optional id, `Crossing` an array of pairs.
  Those need serde attributes or stay by hand, so the 600 lines are an
  upper bound; the sure part is `Why`, `Literal`, `Placement` and
  `NotTaken`.
- `WhyNot` is also the reader's check of the atoms and occurrences a
  refutation names (`core-sequents.md:522-537`); a derived
  `Deserialize` on `Refutation` must keep that check after reading.
- `core.md` ("The written form") and `core-sequents.md:534` describe
  the proxies but give no reason for preferring them to a derive, so no
  recorded choice stands against it.
The gain is mostly tidiness, with serde on the ids for library users,
at large cost; the lock and `core/tests/wire/` pin the bytes.

## F-6a-08: confirmed

`errors/mod.rs:1186-1199` is as quoted. `workspace/mod.rs:1565-1571`
reports a missing workspace node as `Space::Node` with `len: 0`,
`rules/terms.rs:195-200`, `ordinary/bruijn.rs:94-99`,
`first_order.rs:457-489` and `serialize/rules.rs:559-565` give
`Space::Term` for terms that are no root formula, and
`serialize/resolution.rs:153-159` gives `Space::Node` for a parent
clause. `Space`'s own docs (`limits.rs:508-511`) say "the terms of a
sequent's arena" and "the nodes of a proof", so the enum is used wider
than it is documented. New spaces go last in the fieldless enum
(`core.md`, "The break against the last release").

## F-6a-09: confirmed

`settings.rs:129-153` is as quoted, `:163-164` clears
`path_length` and `model_size` on `search`, and
`protocol/ordinary.rs:969`, `:1015` and `:1071` pass `&settings.search`
to ordinary calls. `core-ordinary.md:909` and `core-search.md:478`
record that the bounds live in `linear::search::Options` as fact,
without a reason that D4 would have to overcome; consumer test point 10
names the same lean. The cost is large and honest: the settings JSON,
the keys the command's flags name (`docs/src/settings.md`), `web-tab`
and the client all follow (B6). `Settings` has no modal key today, so
the proposed `modal` group is new, not a move.

## F-6a-10: confirmed

The table is `limits.rs:18-39` (20 rows). No row names `modal`,
`lambda`, `rules`, `protocol`, the workspace, `ljt`, `sat`,
`resolution`, `herbrand` or `normal`, each of which takes `&Limits` and
a stop per `core.md` ("The public API at a glance"). Text only.

## F-6a-11: confirmed

Both places were opened. `serialize/modal.rs`'s `FormulaForm`, its
`From<Node>` and the reader's eight shared arms are the same text as
`serialize/ordinary.rs:41-91` and `:193-222`, over two different arenas
(`modal::{Node, NodeId, Formulas}` and `ordinary`'s). One reader
generic over the arena would serve both. Corrections: the ordinary
reader also carries the first-order tables and binders, so the shared
part is the form, the conversion and the connective arms, nearer 100
lines than 200; and a merged tag set changes what an ordinary reader
says of `□` (today serde's "unknown variant"), so each suite's refusal
of the other's tags needs its own message and test. `core-modal.md:65`
records that the level is read from the content, which the change
keeps.

## F-6a-12: confirmed

`errors/parse.rs:31-38` is as quoted; the field is set only at `:73`
(`reserved: false`) and read at `:94`, `linear/lltp.rs:531` and
`core/tests/parse.rs:289`. No parser of the crate sets it (the modal
and lambda parsers go through `ParseError::new` and `spanning`). The
JSON never writes it. `core-sequents.md:611-613` still says a parse
error "names the word as reserved", which is stale against the field's
own doc and changes with it. A small API break with a small gain.

## F-6a-13: confirmed

The codes are at `errors/mod.rs:1070`, `:1102`, `:1116-1122` and
`:1132` as quoted, and the variants at `:292` (`Refutation`, linear)
and `:731` (`InvalidRefutation`, clauses). C-WORD-ORDER asks one naming
pattern, which fits. Corrections: not every variant can take its
code's name (`Check` has two codes by its content), so the rename is of
the listed ones; the pair `Refutation`/`InvalidRefutation` is the one
with a real cost to a reader, the rest is consistency. Cost large is
honest (the crate, the command, the bindings, `breaks`); the codes
stay.

## F-6a-14: rejected

The quote is right (`errors/mod.rs:460-470`), but the message's content
is a recorded choice with its reason: `core-ordinary.md:2219-2226` says
the message "names the formulas, the infinite chain and rocq-deduxit's
`chain_excluded_middle` and `chain_double_negation_shift`: its theorem
fails there, and no finite Kripke model shows it, so no search of
deduxit can", and ties the message to the manual's transformations
chapter (which prints it, `docs/src/transformations.md:64`) and the
guide. The finding does not name the choice or say why its reason no
longer holds. It comes back only with that: for example the author's
word that the explanation belongs in the manual alone now that
`transformations.md:59-80` carries it.

## F-6a-15: confirmed

Of the thirteen `reason: String` fields in `errors/mod.rs`, `Model`
(`:694`), `Countermodel` (`:686`), `Structure` (`:779`),
`InvalidCalculus` (`:703`) and `NotFitch` (`:761`) are as cited, and
`IllTyped` (`:625`) does carry a node and span beside its reason. No
rules file records prose reasons as a choice. The change is additive on
the wire (`details` gains keys) and keeps `Display`; `Error` must stay
within 64 bytes (`core.md`), so a fault type larger than a `String`
is boxed. Cost large is honest; the gain is real for a front end that
points at the place of a fault.

## F-6a-16: candidate

`serialize/proofs.rs:260-264` is as quoted and builds an owned proxy;
`bench/ceilings.csv` has `read-json` and no journey that writes JSON,
and `dhat.md` has no writing journey either. `core.md` (Decisions,
"Owned values, borrowed views") says "no journey shows the clone",
which is true only because none writes. To measure: a journey that
writes a large proof and an ordinary outcome as JSON, its blocks and
bytes under DHAT and its instructions under callgrind, before and after
one proxy (the proof's) is written from borrowed slices.

## F-6a-17: confirmed

`core/Cargo.toml:65` is `serialize = ["dep:serde"]`, and `criteria.md`
(C-SERDE) names the feature `serde`. No rules file gives a reason for
the name. Correction of the cost: not small. `feature = "serialize"`
occurs 402 times in `core/src`, and the manifests of `cli`, `bench`,
`web`, `web-tab`, `fuzz` and `compat/linlog` name it, with README, the
manual and the guide; the change is mechanical but across crates
(large). The gain is the conventional name, no more, and it is worth
doing only together with F-6a-04, when the feature comes to add serde
alone.

## F-6a-18: confirmed

`wire.rs:620-630` is as quoted; bare levels at `wire.rs:612`, `:626`,
`:627` and `serialize/nets.rs:85`; the constants run 31, 23, 30, 31,
24, 15, 17, 18, 19, 20, 1, 4, 5 at `:350-394` and 6, 7, 8, 9, 25, 26,
27, 12, 13, 14, 16, 21, 22, 28, 29, 11 at `:529-587`. `LEVEL` and
`POLYMORPHIC` are both 31 with nothing tying them. Small and sound;
`wire_levels` keeps the bytes.

## F-6a-19: confirmed

The file has 2 065 lines; `SequentForm::of(x.formulas(), x.left(),
x.right())` on a whole sequent is at `:292`, `:585`, `:605`, `:625`,
`:1278`, `:1656`, `:1661`, `:1672`, `:1733`, `:1891`, `:1982` and
`:2037` (twelve, the finder missed `:292`); the three at `:429`,
`:1136` and `:1430` pass sides of their own and stay. The seams are
where the finder says (`TableForm` at `:1256`, `ComparisonForm` at
`:1604`). One correction: the tableau and the table are read back too
(`core-sequents.md:387-391`), so the split is by subject, not by
"read back or written only". A move only; the gain is tidiness at
small cost.

## Beside the findings

None.
