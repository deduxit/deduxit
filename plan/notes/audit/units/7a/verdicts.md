# Unit 7a: verdicts

Verified against the tree at `/home/tux/Projects/own/deduxit-wt-audit`,
with runs of `target/debug/deduxit` (built 2026-11-27 08:04) on cores
13 to 15. 17 confirmed, 2 rejected (F-7a-18, F-7a-19).

## F-7a-01: confirmed

The witness holds at every place opened: `ModeArgs::mode` ends in
`Mode::CLASSICAL` (argument_parsing.rs:2629-2637), `proof_mode` returns
the flags' mode for a file without one (check.rs:276-285),
`StatementArgs::mode` is a `String` with `default_value = "classical"`
(:541-549), `InterpolateArgs::logic` has `default_value_t =
LogicArg::Classical` (:2289), and grade's help says "in linear logic by
default" (:171). D1 says what the finding says, and no rules file
records a reason for the default that D1 does not override.

The change is sound as a design for the API proposal. What it must
carry besides, which the finding does not name:
- inputs that state their own logic need no selector and must not be
  refused: a `.spec` file (always affine, cli.md "decided in affine
  mode whatever the flags say"), `interact --state FILE`, a batch whose
  records or problem lines name a mode, `workspace run`;
- the group differs per command (`prove` has no `--calculus`, `interact`
  no `--system`), so it is one `Args` struct per set, not one shared;
- `--calculus` as a member collides with `term … --calculus` (F-7a-05);
- `hint.rs` and `lib.rs` match library hints to suggest `--logic
  classical` and `|- `; they are rewritten with it;
- `compat/linlog-cli` runs `deduxit_cli::main`, so the `linlog` binary
  changes with it if it is ever built from this tree.
Cost large is honest: every example of README, the manual and the guide
names a selector afterwards.

## F-7a-02: confirmed

Every run reproduces: `prove --logic classical --cyclic 'a |- a'` and
`prove --logic intuitionistic --lambek 'a |- a'` exit 0 with the flag
dropped; `grade` and `seq print` refuse the same pair (exit 2); with
`{"batch":{"mode":"affine"}}`, `prove --settings s.json --cyclic
--input-format lines -f l.txt` answers `x: provable (MLL, classical
affine, focus engine)` where `--cyclic` alone answers unprovable, and
`--lambek` loses to the file the same way while `-i` wins. The code is
as quoted (prove.rs:748, interact/ordinary.rs:85, settings.rs:246).
cli.md's "(the three asked together)" describes the three flags of the
time and gives no reason to leave the later ones out.

Correction: the kind is not `refactor`. It is a fix that changes
behaviour (an exit 0 becomes exit 2; a batch's mode changes under a
settings file), so it is a correctness fix with a changelog line, and
the lock and `each_flag_overrides_its_key_in_a_file` gain rows.
`--empty-antecedents` belongs in the list too (`given()` has it).

## F-7a-03: confirmed

The runs reproduce: `--calculus system_f` and `--strategy normal_order`
are "invalid value" with a tip naming the kebab form, while
`lambda::Calculus::NAMES` (core/src/lambda/mod.rs:345) says "as the
JSON form and the command write them" and holds `system_f`;
`--fragment "MLL with units"` is refused while `seq fragment` prints
that name. I count 28 `ValueEnum`s and 19 `impl From<…>` mirrors in
argument_parsing.rs (the finding says 18; `SequentFormat` is the 19th).
No rules file records the mirrors as a choice.

Correction to the change and cost: the mirror enums carry each value's
help line (clap's "Possible values" list comes from their doc
comments). A parser built from `NAMES` as `engine_name` is loses those
lines unless the library gives a description per name, so "delete the
mirrors" costs help text the finding does not count. The cheap sound
form is to accept the library's spelling beside the kebab one
(`#[value(alias = …)]`, or the library's `FromStr` tried first) and to
correct the doc sentence at lambda/mod.rs:345; that is additive, not an
API break. One spelling across flags and JSON is the proposal's to
decide.

## F-7a-04: confirmed

The runs reproduce: `seq print --logic classical 'a -> b |- b'` prints
`⊢ a & ~b, b`; `seq print '[]p -> p'` is a parse error with no hint;
`seq json --logic …` is "unexpected argument"; a batch with `--system`
is refused at prove.rs:300-302. `SequentFormat` has no `json`, so `seq
json` is the only JSON a sequent gets. cli.md records "`seq print
--logic` prints the image" as a fact without a reason, so D4 is not
answered by a recorded choice. The change is an API break of `seq
print --logic` (its output changes meaning); the image moves under a
flag, which the finding says. Cost medium is honest only if the library
already writes ordinary and modal sequents in every format; that is the
exports' unit to confirm.

## F-7a-05: confirmed

Confirmed for the translation alone, with the claim corrected.
`prove --translation` names the embedding into linear logic (affine,
cbn, cbv, 01; argument_parsing.rs:2487-2491); `seq translate --by` and
the workspace's `--set translation=glivenko` (docs/src/workspace.md:28)
name the negative translation. So one name means two things and one
thing has two names, not "one thing has three names".

The other two parts I do not find to be defects: `--to` names the
target of the conversion in both commands (a logic for `translate`, a
form for `normalize`), and `--calculus` names the calculus to check in
on every command that has it; the value sets differ per command as
`--format`'s and `--method`'s do. `--calculus` becomes a real clash
only if F-7a-01 makes it a suite selector. Cost small holds for
renaming `--by`; renaming `--to` and `--calculus` is not worth a break
by itself.

## F-7a-06: confirmed

`grep -il "example\|https://" target/audit/help/*` finds no file;
`help/prove.txt` has 538 lines; line 76 is one paragraph of 453 words
(the doc at argument_parsing.rs:699-); "(but not by `interact`)" is in
thirteen commands' help, from the one doc at :2331 (the finding names
two). L2 says help leads with examples and links the web manual. No
rules file records a choice against examples. Cost medium is honest;
the examples should be run by `readme.rs` as the finding says.

## F-7a-07: confirmed

`prove -o - 'A |- A'` exits 0, prints nothing and leaves a file `-`
holding the verdict and the tree. `Output::open` (io.rs:906) has no
case for `-`, while the readers take `Path::new("-")` as standard input
(io.rs:86, :163, :195, :235). cli.md records `-` for `--file` only and
nothing against it for `--output`. The change must also cover the
format chosen from the path's extension (`Format::of_path`), the rule
that a file is made only when it holds a derivation (a `-` is standard
output, where the verdict is printed), and a batch's `--output DIR`.
Simplest is to map `-` to `None` where the arguments are read.

## F-7a-08: confirmed

The sentence is at argument_parsing.rs:2671-2673 as quoted, and
`prove -o out.tex 'A |- A'` writes LaTeX. The sentence means "and a
JSON output is always made" (cli.md: "JSON is always written"), so it
is ambiguous and reads false at first; the rewrite is right.

## F-7a-09: confirmed

The four texts are at :2469, :2496, :2498 and :2427 as quoted.
`prove --logic classical 'forall x. p(x) |- exists y. p(y)'` and a
`.p` file with `![X]` and `?[Y]` both answer `valid (classical logic by
the connection prover)`. The ordinary syntax block (:53-66) lists no
quantifier, predicate or equality for ordinary logic either, which the
rewrite should add.

## F-7a-10: confirmed

`const SYNTAX: &str = linear_syntax!();` is at :79, the ordinary block
is the linear syntax followed by "With --logic the input is ordinary
logic" (:53-66), the top-level help shows it (:101), no help text has
modal syntax although `prove`, `check` and `grade` take `--system`,
`prove`'s description is the fragment and engine sentence (:113-114),
and docs/src/commands.md:140 reads "By default a sequent is read in
classical linear logic". D4 and writing.md rule 3 say what the finding
says. Most of it falls out of F-7a-01; the text that remains is small.

## F-7a-11: confirmed

No `clap_mangen` or `clap_complete` in any `Cargo.toml`, nothing on man
pages or completions in cli.md, the manual or `plan/later.md`.
`modules/dist.nix` builds `.deb`, `.rpm` and Arch packages, so there is
a place to ship them. L3 is conditional ("if they serve users"); with
this many flags they do. To weigh: the lock's package count is 175
against a ceiling of 180 (B7), and both crates add packages; a hidden
`completions` subcommand adds code to the binary, a build-time
generator does not.

## F-7a-12: confirmed

Both functions run the same steps in the same order at the lines given
(prove.rs:336/360/404/498/548/560, batch.rs:743/752/766/806/834/847).
cli.md's invariant "A batch's results are those of single calls" is the
reason one implementation should serve both; it records no reason for
two. The differences one function must take as parameters: the batch's
extra stop (`self.batch.passed()`), no race of the model search beside
the embedding (it always runs `model_first`), the search within cores
by the plan, and what is written (a line or a record against prove's
output). Cost large is honest and the gain is to the code's readers
and to whoever changes a method's order; the behaviour lock and the
batch's verdicts are the net.

## F-7a-13: confirmed

`pub(crate) timeout:` is declared six times with identical attributes
(:559, :588, :912, :1306, :1498, :1745), `memory_limit` five times as a
struct field with identical attributes plus the `seq` variants' fields,
`work_limit` five times; the three-part apply block is at
settings.rs:342, :426, :458, :510 (and a fifth at :185-194, which the
finding leaves out); the three `settings()` of the term commands are
the same body. The doc texts differ per command (what the limit stops
there), so a flattened struct needs one wording or `long_about`
overrides, as the finding says. A flattened struct also fixes the
heading order (F-7a-16), since clap orders headings by the first
argument that names each.

## F-7a-14: confirmed

`Checked {` literals: check.rs 9, eliminate.rs 3, modal.rs 2,
calculus.rs 1, so 15 (the finding's list has 15 places and its text
says 16). The status match is seven times (check.rs:497, :792, :884,
:1029, :1376, modal.rs:384, calculus.rs:215).

Correction: two changes of two kinds. The constructor is a refactor
and sound. The tagged suite field is an API break of a wire form and
should not be decided here: the library's own documents name their
suite the same way (`mode`, `logic`, `system`, `calculus` as keys,
which `read_document` probes), so one tagged field in `Checked` alone
would make it the odd one. It belongs to the proposal for all wire
forms, with the manual's JSON Schema (`wire-schema`).

## F-7a-15: confirmed

All four are as quoted: docs/src/settings.md:3-5 names six takers while
twenty help pages list `--settings`; cli/src/lib.rs:4-5 names four
commands; `TransformArgs` (argument_parsing.rs:1725-1728) is flattened
by seven structs (:1782, :1799, :1864, :1916, :2228, :2236, :2287);
check.rs:1182 is a half sentence above its rewrite at :1183-1184.

## F-7a-16: confirmed

The orders are as said: prove Options, Logic, Input, Output, Search,
Limits, Batch; check and eliminate Options, Limits, Input, Logic,
Output; grade Options, Input, Logic, Output, Limits; interpolate ends
Limits, Logic. `--file` is under Options in check.txt:18.

Corrections: the place is the source, argument_parsing.rs:1297
(`CheckArgs` declares `memory_limit` first), not the help inventory.
`seq print` does have headings, for the groups it flattens (Input,
Logic, Output); its own flags (`-i`, `--cyclic`, `--lambek`,
`--format`, `--standalone`, `--output`, `--memory-limit`, `--settings`)
are under Options, so its mode flags and `--logic` sit under two
headings. cli.md:20-28 says every flag of `check` and `eliminate`
names a heading and that Limits comes after Search; check's `--file`
and its order already depart from that record. L2 fits better than L8.

## F-7a-17: confirmed

The five lines repeat at check.rs:343, :353, :365, :374 and :418 with
only the type and the noun changed; :395-407 is the same with two types
in an `if`. `read_proof` (:1278) shares the first two lines but keeps
the refusal apart to explain an outcome without a proof, so the helper
serves six places, not seven. Small gain at small cost.

## F-7a-18: rejected

The quotes are right, but the change does not write the same record
and the criterion does not apply. An outcome serializes its own
`version` first (`{"version":1,"deduxit":"0.8.0",…}`), so a record
struct with `version`, `name` and `#[serde(flatten)] body` writes
`version` twice; dropping the inner one needs a custom `Serialize`
adapter longer than the 17 lines it replaces. The other route,
`serde_json::Map` with `preserve_order`, turns a feature on for the
whole workspace (features unify), adds a package to a lock at 175 of
180, and changes the key order of every `Value` the core crate writes.
`seconds` is written by `{}` of an `f64`, which differs from
serde_json's form for small values (`0.00001` against `1e-5`).
cli.md records why the record is assembled as text ("the keys are
written in order, verdict first, since `serde_json::Value` would sort
them", and `record_json` "drops the outcome's own `version`"). The two
`expect`s cannot fire: the body is always `serde_json::to_string` of a
struct. B8 is about dependencies earning their place, which is not the
matter here.

## F-7a-19: rejected

The format strings are as quoted, but the claim does not hold where it
would matter. A time limit echoed in a message reads back:
`--timeout 1.5s` and `1500ms` are accepted and print as `1.5s`, and
limits print in `ms` or `s`. `µs` appears only in elapsed times
(`after 54.08µs`, `time: 451.04µs`), which no flag takes as input, and
it is a unit a user understands, so L4 ("nothing only a developer
understands") is not broken. What is left is that the text relies on
`Debug` of `Duration`, whose format the standard library does not
promise; a finding on that would cite an idiom criterion and show a
harm, and the cost is every timed example of the manual (27 lines with
`µs`).

## Beside the findings

None beyond what the findings themselves report. F-7a-02 is a defect
of correctness (a verdict in another logic than the flags ask, exit 0)
and should be treated as a fix, not a refactor.
