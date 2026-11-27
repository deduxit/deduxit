# Unit 7b: findings

Read in full: 15 files. Not read in full: none.

The finder's remarks: I could not settle F-3-01's suspected protocol defect by a run. It needs a proof that cuts one occurrence twice, and I could not build one through the command; unit 8 should run `linear.net` on the proof from `core/src/linear/nets/cut/tests.rs:556` (F-7b-14 cites the missing fallback only from reading). The handover's "exponential `binders` walk" (`interact/ordinary.rs:672`) cannot happen with the default settings: a saved session whose formulas share 60 levels is refused at load by the unfolded occurrence limit ("unfolds to at least 2305843009213693951 subformula occurrences, more than the limit of 50000000"); it needs `--occurrence-limit none` to bite. F-2b-01 (`split_passes` on every ⊗/Mix `apply`, `interact/mod.rs:536`) stays a candidate, since no journey runs a session. For F-4e-02, `rules 7` and `apply 7 a` in an LK session answer "there is no open goal 7" with no panic, so the command's own checks plus the library's Result-returning calls cover the ids users type. Unit 7a / the leader should know that `grade` reads each check's verdict from the first line of its text output (`grade.rs:248`, `:252-255`), so the answer value that F-7a-12 proposes should serve grade too. `--quiet` exists only on prove, check and eliminate (help.md:112), which falls under 7a's L4/L8.

## F-7b-01

- dimension: api
- criterion: D3
- place: cli/src/workspace.rs:441
- kind: API break
- cost: medium
- candidate: no

**Claim.** The workspace's exit status reads a node's verdict from the string at the JSON key `verdict`, so a resolution node whose clauses are satisfiable, and an interpolant node of a sequent that is not valid, both exit 0 where the commands `resolve` and `interpolate` exit 1 and the manual promises 1.

**Witness.** ```rust
    let verdict = value
        .and_then(|value| serde_json::from_str::<serde_json::Value>(value).ok())
        .and_then(|value| value["verdict"].as_str().map(str::to_owned));
    match verdict.as_deref() {
        Some("not_valid" | "unprovable") => Status::No,
        Some("unknown") => Status::Unknown,
        _ => Status::Yes,
    }
```

```text
$ deduxit workspace new w.json --logic classical 'a |- b'
$ deduxit workspace apply w.json 0 ordinary.resolve; echo $?
1  ordinary.resolution  Satisfiable
0
$ deduxit workspace apply w.json 0 ordinary.interpolate; echo $?
3  ordinary.interpolant  No interpolant
0
$ deduxit resolve 'a |- b' >/dev/null; echo $?
1
$ deduxit interpolate 'a |- b' >/dev/null; echo $?
1
```

docs/src/workspace.md:52-54: "The exit status is the worst verdict run: 0 valid, 1 not valid, 3 not decided". The resolution node's JSON has `"verdict":"satisfiable"`. The interpolant's has no top-level `verdict`: it sits under `outcome`.

**Change.** Let the library state each value's verdict (for example `protocol::verdict(kind, value) -> Option<Verdict>` beside `protocol::summary`, with each suite mapping its own kinds), and have `verdict_status` match on that enum instead of strings. Until then, map `satisfiable` and a nested `outcome.verdict` too.

**Value.** A script that runs `deduxit workspace apply|run` reads a sequent that is not valid as exit 1, as the manual and the matching commands say. The exit status of every workspace kind gets one source of truth.

**Net.** This is a fix of a documented contract, not a refactor. Add a case per kind to the workspace tests and `docs/src/workspace.md`'s examples (`cli/tests/readme.rs`). No checker, search or certificate is touched.

## F-7b-02

- dimension: idiom
- criterion: L1
- place: cli/src/eliminate.rs:442
- kind: refactor
- cost: small
- candidate: no

**Claim.** `eliminate --net` with a drawn format writes its verdict with `println!`, which bypasses the command's closed-pipe handling, so a closed pipe makes it panic and exit 101.

**Witness.** ```rust
    if show.verdict {
        println!("{line}");
    }
```

`io::Stdout` (cli/src/io.rs:747-750) exists to take "a closed pipe as a reader that has read all it wants". This is the one `println!` in the unit.

```text
$ deduxit eliminate --net --format svg -o net.svg cutp.json | true
thread 'search' panicked at …/std/src/io/stdio.rs:1209:9:
failed printing to stdout: Broken pipe (os error 32)
(PIPESTATUS[0] = 101)
```

**Change.** Write the line through `io::write(None, …)` or `io::Stdout::lock()`, as every other command does.

**Value.** A user who pipes `eliminate --net` into `head` gets one of the documented statuses (D3) and no panic message.

**Net.** Behaviour is kept except on the closed-pipe path. Add a case to `cli/tests/cli.rs`. No checker or search is touched.

## F-7b-03

- dimension: api
- criterion: L9
- place: cli/src/eliminate.rs:44
- kind: API break
- cost: small
- candidate: no

**Claim.** `eliminate` flattens all of `CheckArgs`, so its `--help` offers about ten of `check`'s flags. It refuses three of them and runs with four others silently ignored, exiting 0.

**Witness.** ```rust
    if check.logic.is_some() || check.sequent.is_some() {
        bail!("--logic and --sequent check a Kripke model, with `check`; eliminate reads proofs");
    }
    if check.calculus.is_some() {
```

help/eliminate.txt lists `--sequent` (:102), `--world` (:107), `--tableau` (:110), `--dimacs` (:115), `--proof-format` (:120), `--calculus` (:127), `--logic` (:155), `--system` (:165), `--layout` (:171) and `--lambda` (:268).

```text
$ deduxit eliminate --tableau cutp.json        # exit 0, the steps printed, --tableau ignored
$ deduxit eliminate --dimacs x.cnf cutp.json   # exit 0, x.cnf does not exist
$ deduxit eliminate --system S4 cutp.json      # exit 0
$ deduxit eliminate --lambda cutp.json         # exit 0, no lambda-term printed
```

**Change.** Give `EliminateArgs` the groups it reads (mode, the proof file, output, limits), flattened as `#[command(flatten)]` structs shared with `CheckArgs`, instead of the whole `CheckArgs`. clap then refuses the others with its usual error and leaves them out of the help. cli.md:671's reason ("its bounds, mode and output are check's") holds for those groups, not for check's document selectors.

**Value.** A user of `eliminate` sees only the flags it reads, and a mistyped or misplaced flag is an error instead of a run that ignores it.

**Net.** Use README's and the manual's `eliminate` examples, `each_flag_overrides_its_key_in_a_file` and the help tests in `cli/tests/cli.rs`. No checker or search is touched.

## F-7b-04

- dimension: api
- criterion: L9
- place: cli/src/term.rs:253
- kind: API break
- cost: small
- candidate: no

**Claim.** `term reduce`, `term check` and `term infer` take no `--timeout`, `--memory-limit` or `--work-limit`, and `reduce` ignores the settings' clock, so a reduction with `--steps none` runs until it is killed.

**Witness.** ```rust
    let reduced = on_large_stack(limits.stack_bytes(), || {
        term.reduce(
            &options,
            &limits,
            |_| interrupted(),
```

```text
$ timeout 15 deduxit term reduce --steps none '(\x. x x) (\x. x x)'; echo $?
124
$ deduxit term reduce --timeout 1s '(\x. x x) (\x. x x)'
error: unexpected argument '--timeout' found
```

help/term-reduce.txt says of `--settings` that `clock` and the other keys "are read and not used". term.rs:334 then advises "raise it with --settings (limits.memory_bytes)". `term inhabit` and `term enumerate` do take `--timeout`, `--memory-limit` and `--work-limit` (help.md:95, :133).

**Change.** Flatten the same limits group (`--timeout`, `--memory-limit`, `--work-limit`) into the three term commands. Start a `Deadline` in `reduce` and pass `interrupted() || deadline.passed()` as the stop. Pass a stop to `check` and `principal_type` and install `catch_interrupt` there too.

**Value.** A user of `term reduce` can bound a reduction by time the way every other command does, and the five `term` subcommands take the same limit flags (L8).

**Net.** The change is additive. Use README's and the manual's `term` examples (`readme.rs`) and the settings tests. The reduction itself is not touched.

## F-7b-05

- dimension: architecture
- criterion: E6
- place: cli/src/ordinary.rs:358
- kind: search change
- cost: large
- candidate: no

**Claim.** The command decides first-order ordinary sequents by its own copy of the library's default: a grounding branch copied from `by_grounding`, plus a connection-prover and model-search race with time shares. The library's `ordinary::decide_within` (which the bindings use) follows a different policy, so the command and the library answer one sequent by different paths.

**Witness.** cli/src/ordinary.rs:358-361:

```rust
        Method::Grounding => {
            let named = options.with_method(Some(Method::Grounding));
            // The default grounds within the library default's bound.
            let most = ordinary::grounding::DEFAULT_NODES;
```

core/src/ordinary/decide.rs:239-242:

```rust
    // The default grounds within a bound of its own and gives way past it.
    let bounded_grounding;
    let grounding_limits = if default {
        let most = super::grounding::DEFAULT_NODES;
```

The command's connection prover runs without the library's `CONNECTION_WORK` bound (`connection_limits`, decide.rs:425):

```rust
    let connected = ordinary::connection::decide(sequent, logic, search, limits, |p| {
        stop(p) || connection.passed()
    })
```

(ordinary.rs:551-553). It adds `FIRST_ORDER_SHARE` 90, `CONNECTION_SHARE` 70, `MODEL_GRACE` 5 and a scoped model-search thread (:173, :205, :213, :445-575). The library runs the model search after the embedding instead (decide.rs:906-922). cli.md:1408 ("as the library's `by_grounding` does") and :1545 ("the CLI never re-implements what `core` computes").

**Change.** Move the default's order of methods into the library behind stops given per stage, for example `ordinary::decide_staged(sequent, options, search, limits, stages: &mut dyn FnMut(Stage) -> Stop)`. The library still reads no clock: the command maps each stage to its share of the deadline, and the bindings to a work bound. `decide_natively`, `by_connection` and the grounding branch go. cli.md records the shares because the library has no clock; a stop per stage removes that reason.

**Value.** A user gets the same answer from `prove --logic`, the web page and a library call on one sequent. About 230 lines leave the command, and a change to the default is made once.

**Net.** Check the ILTP first-order verdicts (`deduxit-bench run --tptp bench/iltp-first-order/Problems`) against the baseline, plus the behaviour lock's `--logic` cases and the manual's first-order examples. This changes how the default searches (method order, bounds), so it is a search change, not a refactor.

## F-7b-06

- dimension: api
- criterion: L8
- place: cli/src/interact/rules.rs:434
- kind: API break
- cost: small
- candidate: no

**Claim.** In a session in a calculus given as data, `proof FILE` always writes JSON and `show` has no formats, where the other two sessions take `--latex`, `--typst`, `--svg`, `--png` and `--pdf` and pick the format from the file's extension. `proof c.tex` thus writes JSON into a `.tex` file.

**Witness.** ```rust
                    [path] => {
                        io::write(Some(Path::new(path)), &serde_json::to_string(&derivation)?)?;
                        format!("{line}; written to {path}")
                    }
```

```text
$ printf 'apply 0 ax\nproof c.tex\n' | deduxit interact --calculus LK 'a |- a'
valid in the calculus given (LK): a ⊢ a; written to c.tex
$ head -c 60 c.tex
{"version":15,"calculus":"LK","terms":["a"],"nodes":[{"sequ
$ printf 'apply 0 R0 ->R\napply 1 L0 ax\nproof p.tex\n' | deduxit interact --logic classical 'a |- a -> a'; head -1 p.tex
\begin{prooftree}
```

rules.rs:41 HELP: "show                the derivation so far, a line per node". The library draws such derivations (`rules::Drawing`, `rules::Fitch`, used by `check --calculus --format latex`).

**Change.** Use the shared `show_target`/`proof_target` and `so_far`/`proof_written` of `interact/mod.rs` in the rules session too, with `calculus::shown` (F-7b-07) doing the drawing.

**Value.** A learner of natural deduction or LK who writes `proof proof.tex` or `show --svg` gets a drawing as in the other sessions, and one command means one thing in every session.

**Net.** Use the session tests in `cli/tests/cli.rs` and the manual's natural-deduction session (`readme.rs`). No checker is touched.

## F-7b-07

- dimension: architecture
- criterion: E6
- place: cli/src/calculus.rs:155
- kind: refactor
- cost: small
- candidate: no

**Claim.** A derivation in a calculus given as data has two writers in one file, `check_rules` with `drawing` and `shown`. They support different formats: `check` refuses PNG and PDF for the same kind of derivation that `prove --system` renders, and only `check` has the Fitch layout.

**Witness.** ```rust
    if !(drawn || matches!(show.format, Format::Text | Format::Json))
        || show.net
        || show.lambda.is_some()
    {
        bail!(
            "a derivation in a calculus given as data is shown as text, json, latex, typst or \
```

(calculus.rs:155-160), against `shown`'s `Format::Latex | Format::Typst | Format::Svg | Format::Png | Format::Pdf =>` (:442).

```text
$ deduxit prove --system S4 '[]p -> p' --format png -o m.png     # exit 0, 17 391 bytes
$ deduxit check d.json --format png -o d.png                      # exit 2
error: a derivation in a calculus given as data is shown as text, json, latex, typst or svg, …
```

The `write` closure is written twice (:335-344, :401-410), and so is the list's size estimate (:266-278, :413-424), whose advice differs: "--format json writes the verdict alone" against "--format json writes it".

**Change.** Have `check_rules` call `shown` for every format after its verdict line. Give `shown` the Fitch branch, and delete `drawing` and the second estimate.

**Value.** A user can render a checked derivation of NJ or LK as PNG or PDF, as `prove --system` already does. About 90 lines go.

**Net.** Use README's and the manual's `check --calculus` examples, the natural-deduction chapter's drawings (`readme.rs`), and the `export` flake check. No checker is touched (`derivation.check` stays first).

## F-7b-08

- dimension: architecture
- criterion: E6
- place: cli/src/interact/mod.rs:594
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The linear and the ordinary sessions each implement `undo` with its step groups, `invertible on|off`, the `close` loop, `show`, `save`, `load`, `invert_all`, `stepped`, `goals` and `opened` (about 250 lines) as near-identical copies. The `Commands` trait shares only the read loop.

**Witness.** cli/src/interact/mod.rs:594-602 and cli/src/interact/ordinary.rs:255-263 are the same:

```rust
            "undo" => {
                let steps = self.state.steps();
                let back = match self.groups.last() {
                    Some(&(before, after)) if after == steps => {
                        self.groups.pop();
                        steps - before
                    }
                    _ => 1,
                };
```

duplicates.md lists 268 tokens at mod.rs:593 / ordinary.rs:254, 145 at mod.rs:851 / ordinary.rs:424, and 107 at mod.rs:871 / ordinary.rs:540. The `show` arms (mod.rs:651-670, ordinary.rs:305-324) differ only in a variable's name.

**Change.** Put the step-grouping state (`invertible`, `groups`) and the shared arms in `mod.rs`, generic over a small command-side trait the two sessions implement (`steps`, `undo`, `invert`, `invert_all`, `goals`, `is_complete`, `goal_line`, `derivation_within`). F-arch-12's shared library interface would make the trait a thin adapter.

**Value.** A reader of the code sees one session loop. A new session command, or a fix to undo's grouping, is written once for both sessions (and for the rules session where it applies).

**Net.** Use the session tests that pin exact output (`interactive_session`, `ordinary_session` in `cli/tests/cli.rs`), the manual's session examples and grade's script tests. No checker or search is touched.

## F-7b-09

- dimension: idiom
- criterion: E6
- place: cli/src/tableau.rs:135
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The mapping from a refusal's settings key to the flag that raises it is written about sixteen times, with different spellings and coverage, and one copy sends the user to a settings file.

**Witness.** ```rust
    match error.setting() {
        Some("limits.memory_bytes") => format!("{error}; raise it with --memory-limit"),
        Some("limits.work") => format!("{error}; raise it with --work-limit"),
        Some("limits.occurrences") => format!("{error}; raise it with --occurrence-limit"),
```

The same match appears at eliminate.rs:757, term.rs:333 ("raise it with --settings (limits.memory_bytes)"), term.rs:467, resolve.rs:95, :166, transform.rs:127, :349, calculus.rs:191 ("--memory-limit SIZE"), :296 ("; raise --derivation-limit"), check.rs:432, :761, :1398 and interact/mod.rs:397-402. modal.rs:137-140 names three flags without asking which: "raise the limit (--derivation-limit, --memory-limit, --work-limit)".

**Change.** Add one `settings::flag_of(setting: &str) -> Option<&'static str>` beside the flag-to-key table that `each_flag_overrides_its_key_in_a_file` already pins, and one `advice(&Error)` that writes "; raise it with FLAG". Every site calls it, and so does modal.rs.

**Value.** A user is always told the one flag that lifts the bound that stopped them, in one wording. A new limit gets its advice in every command at once.

**Net.** Check the behaviour lock's refusal lines and README's and the manual's examples that show a refusal. No checker or search is touched.

## F-7b-10

- dimension: api
- criterion: E6
- place: cli/src/calculus.rs:28
- kind: API break
- cost: medium
- candidate: no

**Claim.** Three front ends keep their own tables of the built-in calculi by name, with different contents: the command knows 13 names plus the modal `G3…` calculi, the bindings know 13, and the protocol knows 3.

**Witness.** ```rust
const BUILT_IN: [(&str, &str); 13] = [
    ("LK", rules::LK),
    ("LJ", rules::LJ),
```

The same table is at web/src/rules.rs:37-38 ("The built-in calculi by name, each its rule file: the command's."). core/src/protocol/rules.rs:207-213:

```rust
fn built_in(name: &str) -> Option<&'static str> {
    match name {
        "LK" => Some(crate::rules::LK),
        "LJ" => Some(crate::rules::LJ),
        "LM" => Some(crate::rules::LM),
        _ => None,
```

`G3S4` and the other modal calculi are read only by the command (`calculus.rs:50` `modal`). duplicates.md: 122 tokens at calculus.rs:28 / web/src/rules.rs:38.

**Change.** Add `rules::Calculus::built_in(name, limits) -> Option<Result<Calculus, Error>>` and `rules::BUILT_IN: &[&str]` to the library, covering the `G3`+system names through `modal::calculus`. The command's `BUILT_IN` and `modal`, the bindings' table and the protocol's `built_in` call it.

**Value.** A user names a calculus once and every front end takes the name: `NK-DNE` and `G3S4` work in the protocol and the workspace as they do in the command. A library user gets the list without copying it.

**Net.** Use README's and the manual's `--calculus` examples, the bindings' tests under Node and the protocol tests. The addition is additive for the library. No checker is touched.

## F-7b-11

- dimension: text
- criterion: L5
- place: cli/src/show.rs:450
- kind: text
- cost: small
- candidate: no

**Claim.** A session's `proof` past the derivation limit is advised to use `--format json`, which is no word of the session (there it is `proof --json`), because `show.rs`'s refusal lines are written for `prove`.

**Witness.** ```rust
        "the derivation is not written: its {} inferences with {} characters of sequents are \
         estimated at {}{compact}, over the limit of {}; --format json writes the proof itself, \
         --derivation-limit SIZE raises the limit and --derivation-limit none lifts it",
```

```text
$ printf 'apply 0 0 par\napply 1 0 ax\nproof\n' | deduxit interact --derivation-limit 10 '|- A par ~A'
…
valid proof (classical)
the derivation is not written: … over the limit of 10 B; --format json writes the proof itself, --derivation-limit SIZE raises the limit …
```

The same holds for `too_large_to_draw` (:464, "prove --format json writes it"), `proof_unread` (:498) and `over_memory` (:508), which `interact --logic`'s `proof` reaches through `show::shown`.

**Change.** Give `Show` the way to get the whole proof as a field (`--format json` for the commands, `proof --json` for `Show::session`), and write it into these lines instead of the fixed text.

**Value.** A learner in a session reads advice they can type.

**Net.** Check the session tests in `cli/tests/cli.rs` and the behaviour lock's refusal lines of `prove` (unchanged).

## F-7b-12

- dimension: api
- criterion: L7
- place: cli/src/interact/mod.rs:216
- kind: API break
- cost: small
- candidate: no

**Claim.** On a terminal, `interact` prints only the prompt `> `. It does not show the goal it starts from or say that `help` lists the commands and `quit` (or Ctrl-D) ends the session.

**Witness.** ```rust
    while !io::closed() {
        if prompt {
            write!(stdout, "> ")?;
            stdout.flush()?;
        }
```

```text
$ script -qec "deduxit interact '|- A, ~A'" /dev/null   # typed: quit
> quit
```

`first` is printed only when `--invertible` applied steps (mod.rs:169-177).

**Change.** When standard input is a terminal (`prompt`), print the open goals and one line such as "type `help` for the commands, `quit` or Ctrl-D to end" before the first prompt. A pipe or a script stays as it is.

**Value.** A newcomer at a terminal sees what to prove and how to go on and leave (clig: say how to leave; do not wait silently).

**Net.** The behaviour is unchanged for pipes and scripts, so `cli/tests/cli.rs`'s session tests and grade's scripts hold. A pseudo-terminal test would pin the new lines.

## F-7b-13

- dimension: text
- criterion: D4
- place: docs/src/interactive.md:266
- kind: text
- cost: small
- candidate: no

**Claim.** The chapter "Prove step by step" and `interact --help` describe the linear session alone. The chapter's one paragraph on the ordinary session wrongly says it has no `invertible`, and the session in a calculus given as data is not named.

**Witness.** ```text
`deduxit interact --logic classical`, `intuitionistic` or `minimal`
proves a sequent of ordinary logic with the rules of LK or LJ, with the
same commands but `cut` and `invertible`.
```

The ordinary session has `invertible` (cli/src/interact/ordinary.rs:276-290; docs/src/ordinary-logic.md:1136 lists it). `grep -i 'calculus\|natural' docs/src/interactive.md` finds nothing. help/interact.txt lists only linear rules ("ax, 1, bot, top, *, par, &, …") and positions, not `L0`/`R1` or `derive`/`assume`.

**Change.** Make "Prove step by step" lead with what every session shares (goals, apply, undo, close, show, proof, save), then one short section per session in the page's order (ordinary with `L0`/`R1`, linear with positions and `cut`, a calculus given as data with `derive` and `assume`), each linking its chapter. Correct the `invertible` sentence. `interact`'s long help names the three selectors and says `help` lists each session's own commands.

**Value.** A user of ordinary logic or natural deduction finds their session in the chapter and the help, and the text stops telling them a command is missing when it is not.

**Net.** Run `readme.rs` (the manual's session examples run) and the `manual` and `manual-links` checks.

## F-7b-14

- dimension: api
- criterion: X5
- place: cli/src/show.rs:1476
- kind: API break
- cost: medium
- candidate: no

**Claim.** The choice of a proof's net (a net with boxes for exponentials, else the MLL structure, falling back to boxes where the proof cuts one occurrence twice) is written by hand in the command and in the bindings, while the protocol's `linear.net` lacks the fallback.

**Witness.** ```rust
    let made = if proof.sequent().fragment().has_exponentials() {
        boxed(proof)
    } else {
        // A proof that cuts one occurrence twice, or an occurrence and one
        // of its subformulas, needs two vertices of one occurrence, which
        // only a structure with boxes has.
        match ProofStructure::from_proof(proof, criterion, limits, |_| stop()) {
```

The same is at web/src/calls.rs:2295-2312. core/src/protocol/linear.rs:557-562 has only `if has_exponentials() { MellStructure } else { ProofStructure }`, with no `Placement::Cut` arm. The command also defines its own `Net` enum (show.rs:1314) with `Display`, `Serialize`, `written_in`, `formula` and `cuts` dispatching to both.

**Change.** Give the library `linear::nets::Net` (MLL or MELL) with `Net::of(proof, mode, limits, stop)` doing this choice, its `Display`/`Serialize`, and dispatch for `cut::eliminate` and the SVG drawings. The command, the bindings and the protocol call it.

**Value.** A library user, the page and the protocol get a net for every checked proof the command draws a net for. Three copies of the choice go, and with them the protocol's gap (F-3-01).

**Net.** Run the nets' tests (`core/src/linear/nets/cut/tests.rs:556`), the README's and manual's `--net` examples and the bindings' tests under Node. No checker or search is touched; the net's own checks stay.

## F-7b-15

- dimension: api
- criterion: D4
- place: cli/src/ordinary.rs:1127
- kind: API break
- cost: medium
- candidate: no

**Claim.** The library labels the linear engines' statistics (`Engine::counters`), but the labels of ordinary logic's methods and of the modal tableau are hand-written in the command alone, so a front end other than the command has none.

**Witness.** ```rust
    let counters: &[(&str, u64)] = match &outcome.search {
        ordinary::Search::Ljt(s) => &[
            ("sequents expanded", s.sequents),
            ("memo hits", s.memo_hits),
```

The table covers seven methods (ordinary.rs:1129-1184). modal.rs:166-171 is the same for modal logic: `"worlds made: {}\nsplits: {}\nworlds blocked: {}\nunits of work: {}\ninferences: {}\ntime: {:.2?}"`. core/src/linear/search/mod.rs:2469: `pub const fn counters(self) -> &'static [Counter]`. `grep` finds "sequents expanded" and "worlds made" nowhere in `core/src` or `web/src`. cli.md:1380: "`--stats` prints the counters its `Engine::counters` lists, so the command needs nothing for them".

**Change.** Give `ordinary::Search` and `modal::Statistics` a `counters()` in the library's `Counter` form (label and JSON key), as `Engine::counters` does. `native_statistics` and modal's `--stats` print from it.

**Value.** The page and a library user can show any suite's statistics with the library's labels (no suite is the main one). A new counter needs no CLI edit.

**Net.** The change is additive. Run `prove --stats` in the behaviour lock and the manual's `--stats` examples. No search is touched.

## F-7b-16

- dimension: idiom
- criterion: E6
- place: cli/src/show.rs:673
- kind: refactor
- cost: small
- candidate: no

**Claim.** "Draw an SVG, write it or render it to PNG or PDF, and say why it is left out" is written five times, and the copies say different things when a stop ends the render.

**Witness.** show.rs:673-691 (`first_order_model_drawn`), :735-753 (`model_drawn`) and :1537-1553 (`net_into`) have the same tail. modal.rs:278-296 and calculus.rs:455-473 have it too, with a fixed reason:

```rust
                    Rendered::Stopped => {
                        Shown::LeftOut("the countermodel is not drawn: it was stopped".to_owned())
                    }
```

(modal.rs:292-294; calculus.rs:470 "the derivation is not drawn: it was stopped"), where show.rs says `why()` ("the time limit of 1s was reached"). duplicates.md: 105 tokens at show.rs:673 / :735.

**Change.** Add one `show::drawn_into(drawing: Result<String, Error>, what: &str, show, prefix, out, halt, why) -> Result<Shown>`, called at the five places.

**Value.** A user always learns what stopped a drawing (the time limit or Ctrl-C). About 80 lines go.

**Net.** Run the behaviour lock's drawn-format cases and the `export` check. No checker is touched.

## F-7b-17

- dimension: idiom
- criterion: E6
- place: cli/src/eliminate.rs:183
- kind: refactor
- cost: small
- candidate: no

**Claim.** `eliminate.rs` repeats itself three times: a closure equal to its own `sequent_of`, the invalid-or-unknown report written again in `unchecked`, and the MLL/MELL dispatch of `nets::cut::eliminate` written twice.

**Witness.** eliminate.rs:183-196 is a closure with the same body as `fn sequent_of` (:283-296) (duplicates.md: 112 tokens). It begins:

```rust
    let sequent = |proof: &Proof| {
        let formulas = proof.sequent().roots().len();
        match Forest::new(proof.sequent())
```

:199-244 builds the same message and `Checked` as `unchecked` (:638-685). :492-515 and :569-590 each match `Net::Mll`/`Net::Mell` around `deduxit::linear::nets::cut::eliminate`.

**Change.** Call `sequent_of(proof, mode, show)` in place of the closure. Have `eliminate_into` reuse `unchecked`'s report. Let the JSON path call `run` and serialize what it returns (or let F-7b-14's library `Net` dispatch).

**Value.** A reader of the code finds about 70 fewer lines, and one wording of each line.

**Net.** Run the behaviour lock's `eliminate` cases and README's `eliminate --net` example. No checker or search is touched.

## F-7b-18

- dimension: api
- criterion: L8
- place: cli/src/resolve.rs:72
- kind: API break
- cost: small
- candidate: no

**Claim.** The bound on a normal form's size has a flag on `seq normalize` (`--form-limit`), but `resolve` and `herbrand` refuse past the same bound and send the user to a settings file.

**Witness.** ```rust
                Err(error @ Error::Refused(Refusal::Form { .. })) => {
                    bail!(
                        "{error}; raise limits.form_occurrences in a --settings file, or take --clauses tseitin"
                    )
```

The same is at resolve.rs:163 and transform.rs:346 ("raise the limit with --settings and limits.form_occurrences"), against transform.rs:120 ("raise the limit with --form-limit"). help.md has no `--form-limit` row for `resolve` or `herbrand`.

**Change.** Put `--form-limit` on `resolve` and `herbrand` (the key `limits.form_occurrences`, applied in `TransformArgs::settings` or their own) and name it in the three refusals.

**Value.** A user who hits the clausal form's bound in `resolve` or `herbrand` raises it with the flag they met in `seq normalize`.

**Net.** The change is additive. Add rows to `each_flag_overrides_its_key_in_a_file`, and the guide's name check in `readme.rs` will see the flag. No search is touched.

## F-7b-19

- dimension: text
- criterion: L8
- place: cli/src/interact/rules.rs:294
- kind: text
- cost: small
- candidate: no

**Claim.** The three sessions word the same answers differently: a goal that is no number, a goal with no rule, an unknown command, and a saved file of another kind.

**Witness.** interact/mod.rs:473 and ordinary.rs:185: `"{word:?} is not a goal: a goal is a number, and `goals` lists them"`, against rules.rs:294: `format!("{word:?} names no goal: give its number, as `goals` shows it")`. mod.rs:488 and ordinary.rs:201: `"no rule acts on it"`, against rules.rs:308: `"no rule applies to the goal"`. mod.rs:755: `"unknown command {command:?}; `help` lists them"`, against rules.rs:469: `"unknown command {other:?}; `help` lists the commands"`. Each of mod.rs:725-729, ordinary.rs:373-377 and rules.rs:455-457 words `load` of a session of another kind its own way.

**Change.** Use one wording for each, written once in `interact/mod.rs` (as consts or small functions the three sessions call), as part of F-7b-08.

**Value.** A learner who moves between LK, linear logic and natural deduction reads the same answer for the same mistake.

**Net.** Use the session tests' expected outputs in `cli/tests/cli.rs` and the manual's session examples.

## F-7b-20

- dimension: idiom
- criterion: E7
- place: cli/src/show.rs:1513
- kind: refactor
- cost: small
- candidate: no

**Claim.** `net_into` silences a lint with `#[allow]` and no reason, where the workspace's convention (and every other site in the unit) is `#[expect(lint, reason = …)]`.

**Witness.** ```rust
#[allow(clippy::too_many_arguments)]
pub(crate) fn net_into(
```

Compare show.rs:768-771: `#[expect(clippy::too_many_arguments, reason = "what to draw, where, and how it may stop, which every caller has apart")]`. `grep -n '#\[allow' cli/src` finds only this site.

**Change.** Write `#[expect(clippy::too_many_arguments, reason = "…")]`. Better still, pass `show` and the stop pair as one value so the lint no longer fires.

**Value.** A reader learns why the exception exists, and Clippy reports it when it is no longer needed.

**Net.** Run Clippy (the gate). Nothing else changes.

## F-7b-21

- dimension: idiom
- criterion: E6
- place: cli/src/eliminate.rs:689
- kind: refactor
- cost: small
- candidate: no

**Claim.** The helper that writes a count with its noun is defined three times, plus a fourth variant, and `grade` borrows `eliminate`'s copy.

**Witness.** ```rust
pub(crate) fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}
```

This is eliminate.rs:689-691. The same is at calculus.rs:605-607 and modal.rs:533-535, `plural` at resolve.rs:424, and inline `if n == 1 { "" } else { "s" }` at transform.rs:387, :427 and :507. grade.rs:647 calls `crate::eliminate::counted`.

**Change.** Keep one `counted` in a shared module (`show.rs` or a small `text.rs`) and call it everywhere.

**Value.** A reader of the code finds one helper where `grade` does not have to reach into `eliminate`.

**Net.** Use the outputs pinned by the behaviour lock and README; the text is unchanged.
