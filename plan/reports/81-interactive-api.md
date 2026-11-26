# Step 81: every candidate rule with its reason, and a step's term written out

Opus 5.5 at `high`, 2026-11-26, in the deduxit workspace
`deduxit-wt-step-79a-a0a84e` on `main@origin` (`oxwtukll`) and, after
"deduxit pushed", the client workspace `step79a`
(`/home/tux/Projects/own/deduxit.github.io-79a`), cores 4 to 9. Nothing
pushed.

## Outcome

### deduxit

- **Every candidate rule with its status, in the three suites.**
  `Interactive::candidates` (linear: `goal, position`; ordinary: `goal,
  side, position`; `rules::Interactive`: `goal`) returns every rule of
  the calculus that acts on the formula (in the rules suite, every rule
  at every choice of members), each `Status::Applies`,
  `Status::NeedsInput { needs }` (a split, a context, a witness, or the
  rules suite's names to give) or `Status::Refused(error)` with the
  error `apply` returns for the same step. Legal steps only: nothing
  judges whether a goal can close.
- **Cost.** At most one `apply` per rule, and less where the suite can
  say it without a step:
  - linear, propositional: `expand` alone per rule (complete for
    `apply` there); a `⊗` or Mix is judged by one split found by a
    linear scan of the cut points round from the formula;
  - linear and ordinary at first order, and the ordinary axiom once
    witnesses exist: `apply` on one clone, then `undo`;
  - ordinary propositional: `expand` alone (it is `&self`);
  - rules suite: one `plan` and `commit` on a clone per choice, as
    `rules` did before (`rules` is now `candidates` less the refused).
- **The refusals that carried a sentence have fields.** `witness` (both
  suites) holds a `WitnessFault`, the ordinary `binding` a
  `BindingFault`, `step_refused` (the rules suite and the refutation by
  hand) a `StepFault`; each `Display` is the sentence it was, and in
  JSON `details` has `fault` and its fields. One sentence is left: the
  calculus checker's own reason (`StepFault::Check { reason }`), since
  `rules::Derivation::check` writes its faults as text.
- **A step's term written out.** `ordinary::Interactive::opened_term`
  gives a quantifier step's term as taken (eigenvariable by its printed
  name, a given witness as text `witness` reads, `$1` for one left
  open), as the linear session's did; the bindings' `ordinary_view`
  answers it as `opened`, as the linear view does.
- **Bindings.** `candidates`, `ordinary_candidates`,
  `rules_candidates`, one form for all (`{"rule", "at", "status",
  "needs", "refusal"}`, `refusal` the error `apply` throws for that
  step, the linear one described with formulas); `describe` lists
  `candidates` as the protocol's twelfth call (`lacks` in modal and
  lambda).

### The client

- The rule popover takes the call: one `candidates` per press in place
  of an `apply` per rule. The press to the popover with its greyed rules
  went from a median of 13.9 ms (90th 16.2, slowest 30.4) to 12.9 ms
  (90th 13.7, slowest 18.7) over the stored problems.
- An ordinary proof replayed onto a goal gives its witnesses exactly.
- Head `osruxkzq` (`ddfe1645`) on the client's `main`, its full
  `nix flake check` passed; nothing pushed.

## The call's shape and why

- **What current tools do** (read 2026-11-26 by a Sonnet 5.5 agent at
  `high`, from the sources):
  - the Language Server Protocol 3.17, `CodeAction.disabled: { reason }`
    (since 3.16): a server lists every action and marks one that cannot
    apply now with a human-readable reason; clients show it faded with
    the reason (the specification's `codeAction.md`). Its `data` and
    `codeAction/resolve` are the idiom for an action that needs more
    before it runs;
  - Lean 4's reference manual (tactic language): a failure is a message
    with no structured reason ("there is no operator to distinguish
    between reasons for failure"), and `exact?`, `apply?`, `rw?` list
    successes only;
  - Rocq's reference manual: `fail` gives "Error: Tactic failure." and
    leaves the proof unchanged;
  - KeYmaera X's proof IDE paper (arXiv 1701.08469): a formula's menu
    lists the applicable tactics and the input they need, the rest left
    out;
  - the Proof Tree Builder (arXiv 2303.05865) hides rules whose
    connective does not match and warns when a rule cannot apply.
  - Isabelle's manuals came back unreadable; Click & coLLecT's paper was
    blocked.
- **What I took**: LSP's shape, every candidate listed with an optional
  refusal, plus what none of these has, a stable code and fields
  (the library's error form: `code`, `kind`, `message`, `details`), and
  a third state between enabled and disabled for a step that takes
  input, as KeYmaera X shows "required input". The status names
  (`applies`, `needs_input`, `refused`) are the prompt's.
- **Candidates are the calculus's rules for the formula**: the linear
  ones `rules` lists (connective and mode; wk only affine, Mix only with
  Mix), the ordinary axiom and every rule of the logic `shape` admits
  (a rule the logic lacks, `WR` in LJ, is no candidate, as no calculus
  of that logic has it), and every rule of a rule file. I read "every
  candidate rule of the calculus" as the rules of the session's
  calculus, not of every calculus.
- **A refusal is that of the step without input**: every other formula
  to the right premise for `⊗` and Mix, a shared context, a witness left
  open; in the rules suite the first choice of members. A split rule is
  refused only where no split is legal: the scan over the cut points is
  complete, since what a split must meet counts formulas of each kind on
  each side and an arc reaches every count a split can (checked by
  trying every split of goals up to nine formulas in
  `candidates_agree_with_apply`).
- **Where quantifiers go** (D17): `Status` and `Candidate` are
  `#[non_exhaustive]`; a witness rule is `NeedsInput { needs: Witness }`
  and judged with its witness open, and the rules suite's `at` already
  carries positions; a binder in a rule file would need nothing here.
- **Options** (D15): none. The call has no choice a user might vary;
  the bounds are the session's own (`limits` it was started under).

## Departures

- **Two refusal texts changed** (the eigenvariable-number refusals of
  both sessions now say "the proof has made as many eigenvariables as
  it can number", and a given ordinary witness outside the scope names
  its eigenvariable), so that one `WitnessFault` variant has one
  wording. Neither is reachable by a session of fewer than 2³²
  eigenvariables, but the second.

## Checks (deduxit)

All on the chain's head `urlqqnuv`, cores 4 to 9:
- `cargo clippy --workspace --all-targets -- --deny warnings`: clean.
- `cargo test --workspace`: every test passed (core lib 704, one ignored
  as before; the new `candidates_agree_with_apply` in the linear and the
  ordinary session, `a_step_s_term_written_out_replays_the_proof`, and
  `candidates_give_every_rule_its_status` in `core/tests/rules.rs`).
- The bindings' tests natively and under Node: `cargo test -p
  deduxit-web` and `nix build .#checks.x86_64-linux.web-wasm-tests`
  (three new `candidates_give_every_rule_its_status`,
  `a_step_s_term_is_written_out`, and the linear call joined to
  `no_intuitionistic_text_writes_a_classical_symbol`).
- `cargo test -p deduxit-cli --test readme` (the guide's names): 7
  passed; `nix build .#checks.x86_64-linux.manual
  .#checks.x86_64-linux.manual-links`: built.
- `cargo doc -p deduxit -p deduxit-web --no-deps --all-features` with
  `RUSTDOCFLAGS=-D warnings`: clean, after `breaks` showed a link to a
  private item, fixed.
- The gate (`systemd-run … taskset -c 4-9 nix develop -c gate`):
  passed. Its first run failed on an import used only behind
  `interactive` (`errors/step.rs`), fixed in the first commit. The
  ratchet's note that cache misses moved on `search-chain-128` and
  `check-qbf-20-2` is informational; no commit touches the search.
- `breaks 0.8.0`: cargo-semver-checks fails one lint,
  `enum_struct_variant_field_missing`, for the six `reason` fields
  removed (both `StepError::Witness`, `ordinary::StepError::Binding`,
  `Error::StepRefused`); cargo-public-api lists the same six as removed,
  nothing as changed, and the new types and calls as added. Each removal
  is the changelog's breaking bullet.

### What the full check on GitHub's runners found, and the fixes

The planning session's run at `urlqqnuv` failed three checks; the fixes
are commits of their own on top (`muutryzo`, `tyrkospu`, `xunrokxs`):
- **features-tests**: the replay test of a step's term needs `serialize`
  (it compares documents), and the rules suite's test kept a helper
  outside its `cfg`. Fixed by a `cfg(feature = "serialize")` on the test
  and the helper moved inside; `cargo hack check --feature-powerset
  --depth 2 --tests -p deduxit` with warnings denied passes.
- **core-wasm32**: `WitnessFault` holds a `Refusal`, so inline in
  `StepError` it made both step errors 56 bytes and `Error` 64 (measured
  with `size_of` on x86_64); the larger `Result` frames on the recursive
  read-back of the intuitionistic connection prover ran its memory-bound
  test out of stack on wasm32-wasip1. Both faults are now boxed, as
  `StepFault` already was: `linear::StepError` 32 bytes,
  `ordinary::StepError` 24, `Error` 56. `nix build
  .#checks.x86_64-linux.web-core-wasm32` passes.
- **budget**: the module was 2 923 027 bytes at gzip -9 against
  `main`'s 2 867 073 (measured by building `main`'s budget check).
  `twiggy diff` of the two modules built for wasm32 with names showed,
  besides the new calls, a second copy of the error forms for
  `serde_json::Value` (`Judged::refused` used `to_value`) and the moves
  of 56-byte errors. With the faults boxed and the refusal written as
  text and read back, it is 2 885 650 (+18 577); the ceiling is raised
  to 2 900 000 in `xunrokxs`, whose body says what the bytes buy.
- The gate passes again at `xunrokxs`.

## The client

On the client's `main` (`xxslynwr`, with fixup-29), in the workspace
`step79a`:
- `xtmuzsqu`, "flake.lock: Update": the input `deduxit` moved to
  `4c460de5` (`xunrokxs`).
- `stkuqzuo`: the rule popover takes `candidates` (`ordinary_candidates`
  in an ordinary proof) in one call in place of `rules` and an `apply`
  per rule (`rules` in `state.tsx`; the worker and `api.ts` export the
  new calls and drop the two old ones, which nothing else called). A
  refused rule's words are the page's as before (`refusal`,
  `ordinaryRefusal`), the witness's and the binding's now worded from
  `details.fault` (`witnessFault` in `words.ts`) where they quoted the
  library's free `reason`. What changes on the page: a split rule is
  greyed where deduxit finds no legal split (before, a split rule was
  never greyed); the ordinary axiom is listed on every formula and
  greyed where it has no partner, so a formula of an ordinary goal
  always shows at least one rule.
- `osruxkzq`: an ordinary proof replayed onto a goal ("Use its proof",
  a change of logic) gives `∀L` and `∃R` the witness the source took,
  from the view's `opened`, each eigenvariable renamed to the one the
  replay's own `∀R` or `∃L` made; a witness left open in the source
  stays open. Before, every witness replayed open and only an axiom that
  used it filled it again; one weakened away stayed `$1`.
- Tests: `tests/ordinary.spec.ts` "an ordinary rule that is no legal
  step is greyed with its reason" and "an ordinary proof made apart
  closes its goal with the witness it was given" (∀L with `c`, which no
  axiom fills; the drawing after "Use its proof" shows `p(c)` and no
  `$1`); `tests/states.ts`'s `ordinary` walk gains "an ordinary greyed
  rule's reason", which the target-size and accessibility walks visit.
  The client's `.claude/rules/web.md` says how the popover and the
  replay work now.

## The measurement

The press on a formula to the popover with its greyed rules
(`deduxit:rules`, the page's User Timing measure), on every stored
problem (`src/content/linear/problems`, 452, and
`src/content/ordinary/problems`, 62), the built site served as Pages
serves it, one Chromium on cores 4 to 9 at 1280 by 800, the first three
formulas of the first goal pressed and each popover left only once its
rule buttons showed. Before: the client's `main` with deduxit at
`7e974ff9` (the trials). After: the client's head with deduxit at
`4c460de5`.

| | presses | median | 90th | slowest |
|---|---|---|---|---|
| linear, before | 1 075 | 13.9 ms | 15.6 ms | 30.4 ms |
| linear, after | 1 075 | 12.9 ms | 13.6 ms | 18.7 ms |
| ordinary, before | 100 | 15.3 ms | 17.1 ms | 28.4 ms |
| ordinary, after | 123 | 13.2 ms | 13.8 ms | 16.1 ms |

The ordinary rows differ in count because before a formula with no rule
the logic offers (nine problems) showed an empty popover, which the
script could not wait out; now its axiom is listed. Seven linear
problems gave no row in either run (the formula `0`, which no rule acts
on, and four whose first formula's button the script could not press).
A first run of the script, which pressed Escape as soon as the popover
showed, recorded only the presses whose rules came in time (568 and
1 030 rows); it is discarded. The calls per press went from one per rule
to one.

## Checks (client)

- `lint` (Biome, `astro check`): clean; `tests` (Vitest): 736 passed.
- `e2e` in the dev shell, the whole suite: 364 passed, no retry; the two
  new tests also three times each in a row.
- The full `nix flake check` at the head `osruxkzq` (`ddfe1645`, the
  review's regular expression folded in), under the machine lock:
  passed, its unit tests 736 and its e2e 364 passed, none flaky. Its
  first run failed `treefmt` alone (the appended tests were unformatted;
  `nix fmt`, folded into `osruxkzq`).

## The review

`fresh-reviewer` (Fable 5.2 at `medium`) read both chains and the
prompt, ran `candidates_agree_with_apply`, and found the statuses sound:
a refused `⊗` or Mix has no legal split (in ordered modes the scan tries
every cut point, which are all the splits there; in commutative modes
only the output's side matters, which `k = 0` or `k = len` reaches), the
refusal is `apply`'s for the default input, no call changes the session,
`opened_term` reads back through `witness`, the replay passes a witness
only to `∀L` and `∃R` and renames by the replay's own `opened`, and the
page reads the faults' fields. Its three points and the answers:
1. **The changelog said "at most one `apply` per rule" for all three
   suites**; in a calculus given as data the call takes one step per
   choice of the members a rule acts on (natural deduction has no such
   choice, so one per rule there). Holds: the changelog now says so
   (`wuqrrxuy`); the doc comment of `rules::Interactive::candidates`
   said it already.
2. **The page does not call `rules_candidates`**: natural deduction's
   panel still lists the applicable steps by `rules_rules`. A question of
   reading, and I read the prompt's "through the bindings to the page"
   with its step 2, which names the rule popover's trials, so the
   popover takes the call and the natural-deduction panel, which had no
   trials, is left as it was. Left open below.
3. **The replay's renaming read names as letters and digits**, narrower
   than deduxit's identifiers (Unicode's identifier classes, combining
   marks included): a constant `a` with U+0324 would have been renamed
   in part and the step refused, never a wrong proof. Holds: the regular
   expression is now `[\p{ID_Start}_]\p{ID_Continue}*`, folded into
   `osruxkzq`.

## Left open

- Natural deduction's panel (`Natural.tsx`) could grey the rules that
  do not apply, with their reasons, from `rules_candidates`; it lists
  only those that do. A small change of the client, if the author wants
  it.
- The calculus checker's own reasons (`StepFault::Check`) are still a
  sentence: `rules::Derivation::check` writes its faults as text, and
  giving them fields is a change of the checker.
- A rule of a calculus given as data that asks positions of more than
  `RULE_CHOICES` (4 096) choices of members is left out of `candidates`,
  as `rules` left it out.
