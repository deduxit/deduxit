# Unit 7b: verdicts

21 confirmed.

## F-7b-01: confirmed

Opened cli/src/workspace.rs:431-449: the quote is there and only the top-level strings `not_valid`, `unprovable` and `unknown` leave exit 0. Ran the witness with target/debug/deduxit (0.8.0): `workspace apply w.json 0 ordinary.resolve` prints `Satisfiable` and exits 0, `ordinary.interpolate` prints `No interpolant` and exits 0, while `resolve 'a |- b'` and `interpolate 'a |- b'` exit 1 and `ordinary.decide` on the same node exits 1. docs/src/workspace.md:53 promises the worst verdict run. cli.md:633-635 only describes `verdict_status` ("an outcome's verdict") and gives no reason for leaving the other kinds at 0. D3 fits: the statuses are a contract.

Correction: The interpolant node is numbered 2 in the run, not 3. The kind is nearer a bug fix of the exit status than an API break of the library; the proposed `protocol::verdict` is an addition.

## F-7b-02: confirmed

cli/src/eliminate.rs:441-443 holds the quote, and `grep println! cli/src` finds no other site outside `eprintln!`. Ran `deduxit eliminate --net --format svg -o net.svg cut.json | true` on the manual's cut.json (docs/src/cut.md:9): the thread `search` panics with "failed printing to stdout: Broken pipe" and the status is 101, which is none of D3's statuses. Every neighbouring path writes through `io::Output` or `io::write`.

Correction: The kind is a bug fix (a panic and exit 101 on a closed pipe go away), not a behaviour-keeping refactor. The criterion D3/L1 fits.

## F-7b-03: confirmed

cli/src/eliminate.rs:43-51 refuses only `--logic`, `--sequent` and `--calculus`. target/audit/help/eliminate.txt lists `--sequent`, `--world`, `--tableau`, `--dimacs`, `--proof-format`, `--calculus`, `--logic`, `--system`, `--layout` and `--lambda` at the lines given. Ran `eliminate --tableau`, `--dimacs x.cnf` (x.cnf does not exist), `--system S4` and `--lambda` on cut.json: each exits 0 with nothing on standard error. cli.md:671-672 records the flatten with the reason that the bounds, mode and output are check's; that reason covers those groups and not check's document selectors, as the finding says.

Correction: `--lambda` is an output flag of check that eliminate ignores, so it belongs to the output group the finding would keep: either eliminate honours it or the shared output group must leave it out. The cost is medium rather than small: `CheckArgs` in argument_parsing.rs has to be split into flattened groups, and `CheckArgs::settings`, `source()` and the help tests follow.

## F-7b-04: confirmed

cli/src/term.rs:253-257 passes `|_| interrupted()` alone as the stop, with no `Deadline`. Ran `timeout 5 deduxit term reduce --steps none '(\x. x x) (\x. x x)'`: status 124; `--timeout 1s` is refused as an unexpected argument. help/term-reduce.txt:17 says the settings' `clock` is read and not used; help.md:95 and :132 list `--memory-limit` and `--timeout` for `term inhabit` and `term enumerate` but not for check, infer or reduce. term.rs:334-335 advises `--settings (limits.memory_bytes)`, against cli.md:612 ("a refusal at a bound is an `unknown:` line naming the flag"). No recorded reason for the missing flags was found in cli.md:599-616. The default `--steps` bound keeps the default call bounded, so the gap shows only with `--steps none` or a large bound; Ctrl-C still stops it.

Correction: The kind is an additive change (new flags), not a break.

## F-7b-05: confirmed

Opened cli/src/ordinary.rs:168-575: the grounding branch (:358-391) repeats `by_grounding`'s bound (`grounding::DEFAULT_NODES`, core/src/ordinary/decide.rs:230-242), `by_connection` (:445-575) runs the prover without `CONNECTION_WORK` (decide.rs:403-429) and adds a scoped model-search thread, and web/src/ordinary.rs:1078 calls `ordinary::decide_within`. cli.md:1402-1408 itself says the command does "as the library's `by_grounding` does", and cli.md:1544-1546 says the command never re-implements what core computes. The shares are recorded choices with ILTP measurements (cli.md:1447-1493); the finding keeps them in the command and moves only the order of methods, so it does not contradict them. Two limits of the change the finder did not state: the race needs a second thread, which the library has only behind `parallel` (and not on wasm), so a staged library entry must take the race as a feature-gated path or leave it to the caller; and `gave_way`'s grace is measured by the clock, so it must be a stage's stop too. With different budgets per front end the answers still differ at the bounds; what becomes one is the order of methods and the grounding and connection hand-over.

Correction: The value is narrower than stated: the same method order in the command, the page and a library call, not the same answer on every sequent. The cost large and the kind search change are right; the net must include `--jobs 1` and `--deterministic` runs and the batch, which take the sequential path.

## F-7b-06: confirmed

cli/src/interact/rules.rs:433-437 writes `serde_json::to_string(&derivation)` to any path. Ran the witness: `proof c.tex` in `interact --calculus LK 'a |- a'` writes `{"version":15,"calculus":"LK",...` into c.tex. `show_target`, `so_far`, `proof_target` and `proof_written` exist at interact/mod.rs:1022-1117 and the ordinary session uses them (cli.md:442-443). cli.md:422-426 describes the rules session's `proof` (node list, JSON into a FILE, `--rocq`) without a reason against drawings, and `calculus::shown` already draws such derivations for `prove --system`.

Correction: A script that writes `proof FILE` with an extension other than .json and reads JSON back would change; the change must keep JSON for .json and for `proof --json`. Otherwise as written.

## F-7b-07: confirmed

cli/src/calculus.rs:151-164 refuses PNG and PDF in `check_rules`, while `shown` (:442) renders them; the two `write` closures (:335-344, :401-410) and the two size estimates (:266-278, :413-424) are as quoted, with the differing advice ("writes the verdict alone" against "writes it"). cli.md:317-326 describes `drawing` and the refusal of another format but gives no reason why PNG and PDF are left out for check. The two writers differ in where the verdict goes when nothing is drawn (`drawing` puts verdict and reason on standard error and writes nothing; `shown` returns `Shown::LeftOut` for `Show::close`), so the merge must keep check's behaviour there or the behaviour lock and the manual's examples change.

Correction: The kind is not a pure refactor: PNG and PDF for `check` of a rules derivation is new behaviour (additive). `shown` also lacks the Rocq-before-layout order and the Fitch branch that `drawing` has, as the finding says.

## F-7b-08: confirmed

Compared cli/src/interact/mod.rs:594-670 with cli/src/interact/ordinary.rs:255-324: `undo`, `invertible on|off` and `show` are the same token for token but a variable name, and `close` differs only in how the lines are joined and in `verdict_line`. The `Commands` trait (mod.rs:183-189) has only `command` and `finished`. cli.md:442-444 records that the ordinary session shares the loop and four helpers and "the rest is its own", without a reason for the copies. The session tests pin exact output (cli.md:1512-1516), which is the net.

## F-7b-09: confirmed

`grep 'raise' cli/src` shows the match on `error.setting()` at tableau.rs:135-138, eliminate.rs:757-761, term.rs:333-335 and :467-469, resolve.rs:95-97 and :166-168, transform.rs:127-129 and :349-351, calculus.rs:191-193 and :296-298, check.rs:432-434, :761-763 and :1398-1400, interact/mod.rs:397-401, with the spellings the finding quotes ("--memory-limit" against "--memory-limit SIZE", "; raise --derivation-limit", term.rs's "--settings (limits.memory_bytes)") and modal.rs:137-139 naming three flags at once. The flag-to-key table is pinned by `each_flag_overrides_its_key_in_a_file` (cli/src/settings.rs:813). A flag exists only on some commands (term reduce has no `--memory-limit` until F-7b-04), so the shared function must know the command or F-7b-04 must land first.

Correction: The place is tableau.rs:135 for the `match`, the quoted arms are :136-138. prove.rs:893-929 has its own advice by `Reason`, which is a different mapping and stays.

## F-7b-10: confirmed

cli/src/calculus.rs:28-42 and web/src/rules.rs:37-38 hold the same table of 13 names, the bindings' comment saying it is "the command's"; `modal` (calculus.rs:50-56) reads `G3` names in the command alone (`grep G3 web/src/rules.rs` finds nothing). A library `Calculus::built_in` would serve both. The third place is misread: core/src/protocol/rules.rs:207-213 maps the calculus of an ordinary logic's derivation (`from.logic().calculus()`, :369), which can only be LK, LJ or LM; it is no table of names a user gives, and the protocol takes a calculus as a `rules.calculus` document, not by name.

Correction: Two tables, not three: the command's and the bindings'. The stated value that `NK-DNE` and `G3S4` then work in the protocol does not follow, since the protocol has no operation that names a calculus; that would be a new operation. The kind is an addition to the library, not a break.

## F-7b-11: confirmed

cli/src/show.rs:448-451, :461-465, :495-499 and :505-509 hold the fixed advice `--format json` (and `prove --format json`). Ran the witness: `interact --derivation-limit 10 '|- A par ~A'` with `proof` prints "...; --format json writes the proof itself, --derivation-limit SIZE raises the limit...", and `--format` is no word of the session, where the command is `proof --json`. `Show::session` exists (show.rs:148), so the field has a place. `--derivation-limit` in the same line is a flag of `interact` and stays right.

## F-7b-12: confirmed

cli/src/interact/mod.rs:205-220: on a terminal the loop writes only `> `, and `first` is set only under `--invertible` (:169-177). Ran the session under `script` with `quit` as input: the whole output is the echoed `quit` and `> `. `grep -i 'prompt|greet|banner' cli.md` finds no recorded choice. L7 asks that the session says how to leave; the long help does, the session does not.

Correction: The kind is an additive change of what a terminal session prints (pipes and scripts unchanged), not an API break.

## F-7b-13: confirmed

docs/src/interactive.md:266-268 says the ordinary session has "the same commands but `cut` and `invertible`"; cli/src/interact/ordinary.rs:276-290 implements `invertible on|off` and docs/src/ordinary-logic.md:1136 and :1150 list it, so the sentence is wrong. `grep -i 'calculus|natural' docs/src/interactive.md` finds nothing. target/audit/help/interact.txt:1-10 describes goals, positions and the linear rule names only; the options below do name `--logic` and `--calculus` (:84-90, :114), so the help names the selectors as flags but its lead describes one session.

Correction: The witness on the help is narrower than the claim: `--calculus` and `--logic` are described as options (interact.txt:84-90, :21), only the long help's lead paragraphs are the linear session's alone.

## F-7b-14: confirmed

cli/src/show.rs:1473-1496 and web/src/calls.rs:2295-2315 make the same choice with the same `Placement::Cut` fallback, the bindings' doc saying "as the command makes it". core/src/protocol/linear.rs:553-564 has only the `has_exponentials` branch and no fallback, and the protocol has an `enum Net` of its own (:362), as the command has (show.rs:1314). No `Net` exists in core/src/linear. X5 fits. The command's `Net` holds a `Cow<ProofStructure>` so that the net engine's found net is borrowed (show.rs:1469-1470); a library type must allow that or the command keeps a thin wrapper.

## F-7b-15: confirmed

cli/src/ordinary.rs:1127-1135 begins the hand-written table of labels by `ordinary::Search` variant, and cli/src/modal.rs:166-171 formats the modal counters inline. core/src/linear/search/mod.rs:2465-2469 documents `Engine::counters` as what "a front end shows". `grep 'sequents expanded|worlds made' core/src web/src` finds the words only in doc comments of core/src/modal, never as labels. cli.md:1377-1381 records the library's counters as the extension point for an engine, and cli.md:1398 and :1414 only say `native_statistics` prints the counters, without a reason for keeping the labels in the command. D4 fits.

Correction: The place is the function at ordinary.rs:1127, the table starts at :1129. The kind is an addition to the library, not a break.

## F-7b-16: confirmed

The write-or-render tail is at cli/src/show.rs:673-691, :735-753 and :1537-1553, cli/src/modal.rs:278-296 and cli/src/calculus.rs:455-473. The three in show.rs say `why()`; modal.rs:293 and calculus.rs:470 say the fixed "it was stopped". The copies differ slightly in which formats are written as text (show.rs:673 takes SVG, LaTeX and Typst; the others SVG alone; calculus.rs writes every non-binary format), which a parameter covers. modal.rs and calculus.rs have only `halt` and no `why` in scope, so their callers must pass one.

## F-7b-17: confirmed

cli/src/eliminate.rs:183-196 is a closure with the body of `sequent_of` (:283-296), which the same file calls at :422, :525, :643 and :651. :199-244 builds the two messages and the `Checked` record that `unchecked` builds at :638-685. The MLL/MELL match around `nets::cut::eliminate` is at :492-515 and again at :569-590. One limit: the JSON path serializes the library's whole elimination record (`e`), while `run` returns only the net, the lines and the end, so "call `run` and serialize what it returns" would change the JSON; the dispatch has to return the record, or go to the library's `Net` of F-7b-14.

Correction: The third part of the change needs the record kept for JSON (cli.md:682: "the record of the library with `--format json`").

## F-7b-18: confirmed

cli/src/resolve.rs:72 and :163 and cli/src/transform.rs:346 send the user to a settings file for `limits.form_occurrences`, while transform.rs:120 and :124 name `--form-limit`. target/audit/help.md:72 lists `--form-limit` for `seq normalize` alone; the flag's field and its key are at cli/src/argument_parsing.rs:1804-1809 and cli/src/settings.rs:490-491. cli.md:538 records the flag for `limits.form_occurrences` on that command and gives no reason against it elsewhere.

Correction: The kind is an addition (a flag on two more commands), not a break.

## F-7b-19: confirmed

The wordings are as quoted: interact/mod.rs:473 and ordinary.rs:185 against rules.rs:294 for a goal that is no number; mod.rs:488 and ordinary.rs:201 against rules.rs:308 for no rule; mod.rs:755 against rules.rs:469 for an unknown command; mod.rs:725-729, ordinary.rs:373-377 and rules.rs:455-457 for `load` of another kind. The `load` lines of the first two name the kinds the file may hold, which is information the rules session's line lacks; one wording should keep it. The session tests pin the exact text, so they change with it.

## F-7b-20: confirmed

cli/src/show.rs:1513 is `#[allow(clippy::too_many_arguments)]` with no reason, and it is the only `#[allow` in cli/src; the other sites of the lint in the unit use `#[expect(..., reason = ...)]` (eliminate.rs:152-155, :304-307, :462-465; calculus.rs:311-314). E7 says a single site is no finding, and `allow_attributes` and `allow_attributes_without_reason` are allowed in Cargo.toml:108-109 because they fire. But they fire at five sites in all (target/audit/clippy.md:42-43): this one and core/src/linear/search/focus/split.rs:606 and :660, focus/mod.rs:1698 and focus/scratch.rs:218. Fixing the five turns both lints to warn, which is the kind of finding E7 admits; this site alone would not.

Correction: Widen the place to the five sites (cli/src/show.rs:1513; core/src/linear/search/focus/split.rs:606, :660; focus/mod.rs:1698; focus/scratch.rs:218) and the change to turning `allow_attributes` and `allow_attributes_without_reason` on in Cargo.toml once they are `#[expect]` with a reason. Taken as the one site it is no finding under E7.

## F-7b-21: confirmed

`fn counted(count, one, many)` is at cli/src/eliminate.rs:689-691, cli/src/calculus.rs:605 and cli/src/modal.rs:533; `plural` at cli/src/resolve.rs:424-425; the inline form at cli/src/transform.rs:387, :427 and :507; cli/src/grade.rs:647 calls `crate::eliminate::counted`. The gain is tidiness alone, at small cost.

Correction: The library has a `pub(crate) fn counted` of its own at core/src/errors/mod.rs:1213, which the command cannot reach; the shared copy stays in the command.
