# Unit 4f: findings

Read in full: 9 files. Not read in full: none.

The finder's remarks: I agree with F-arch-12 and F-4e-03: F-4f-06 gives the linear column with signatures, and F-4f-09 adds `interactive::Options` to what should move to the shared root module. F-4e-07 (no stop on `invert`/`invert_all`) was confirmed with the linear session's calls included, so I did not raise it again, and F-4e-04/F-4e-11 (the witness tokenizer, `derivation_within`'s estimate and renumbering) hold on the linear side as quoted there (`witness.rs:43-57`, `mod.rs:963-1008`). I could not judge any session's performance by measurement, because no journey runs a session; F-4f-10 names the journey it needs. Unit 8 should look at `web/src/calls.rs:1154-1348` (the linear hint lives in the bindings, F-4f-01), at the view building two unbounded derivations per call (`:478`, `:504`), and at whether `Candidate`, `Status` and `Step` should derive serde, since the bindings write their own forms for them. `text.md`'s two hits in this unit ("this session", `invert.rs:113`, `mod.rs:1018`) are false positives: the type is a session. `derivation_within(view, …)` ignores its `view` (`mod.rs:960`, `let _ = view;`) and `Back::term` ignores its `forest` (`handover.rs:807`), and `handover.rs:517` names a private struct `Copy`, which shadows the prelude trait in a file that derives `Copy`; I left these out for lack of a criterion that fits, but a refactor step could tidy them.

## F-4f-01

- dimension: architecture
- criterion: D4
- place: web/src/calls.rs:504; web/src/calls.rs:1184; web/src/calls.rs:478; core/src/ordinary/interactive.rs:1190; core/src/ordinary/interactive.rs:2542
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The linear session's invertible steps, its hint and its list of cuts are computed in the bindings, while the ordinary session has `invertible_steps` and `hint` in the library, and to read what the session already holds the bindings build two whole derivations per view with the derivation bound switched off.

**Witness.** `web/src/calls.rs:504-516`, run on every view of a session (`:455`):

```rust
fn invertible_steps(state: &Interactive) -> Vec<GoalId> {
    let Ok(derivation) = state.derivation_within(
        &deduxit::linear::ViewOptions::default(),
        &deduxit::Limits::default().with_derivation_bytes(None),
        never,
    ) else {
        return Vec::new();
    };
    let mut found: Vec<GoalId> = (derivation.inferences().iter())
        .zip(state.derivation_ids())
        .filter(|(inference, _)| {
            let step = Step::new(inference.principal().unwrap_or(0), inference.rule());
            inference.rule().rule != Rule::Open && state.is_invertible(inference.sequent(), &step)
```

`cuts_made` (`:478-484`) builds the same unbounded derivation again to read each cut. The hint is 190 lines of the bindings (`hint`, `first_order_hint`, `first_steps`, `read_steps`, `:1154-1348`), including the rule that keeps hints from contracting and weakening forever (`:1176-1179`). The other suites have both in the library:

```rust
    pub fn invertible_steps(&self) -> Vec<GoalId> {
```

(`ordinary/interactive.rs:1190`), `pub fn hint(` (`ordinary/interactive.rs:2542`, `rules/search.rs:126`). The session keeps its inferences private (`mod.rs:877`, `pub(crate) fn inferences`), so a front end has no other way to read them.

**Change.** Add `Interactive::invertible_steps(&self) -> Vec<GoalId>` (reading `self.inferences` and `self.history` directly), `Interactive::hint(&self, goal, options, view, limits, stop) -> Result<Hint, Error>` with the bindings' first-steps logic, and a way to list the cuts made (`cuts(&self) -> impl Iterator<Item = (GoalId, Member)>`); the bindings call them.

**Value.** A user of the library and the command get the linear session's hint and invertible list as ordinary logic's users already do (D4). The page's view stops building two unbounded derivations per call, so a session whose derivation the page's bound refuses no longer gets built whole twice (S1).

**Net.** The bindings' tests under Node (the view's `invertible` and `cuts`, the hint calls) and `cargo test --workspace`; the hint's steps must stay those the bindings return today. No checker, search or certificate is touched (the hint still runs `prove_goal` and `close`).

## F-4f-02

- dimension: text
- criterion: T1
- place: core/src/linear/proofs/interactive/mod.rs:253; core/src/linear/proofs/interactive/expand.rs:660
- kind: text
- cost: small
- candidate: no

**Claim.** An axiom on a goal that is not exactly two formulas is refused with `NotAlone`, whose message tells a learner the axiom needs the formula "without other formulas in the goal", which is wrong for the axiom and contradicts itself on a goal of one formula.

**Witness.** `expand.rs:659-662`:

```rust
            Ax => {
                let [x, y] = sequent else {
                    return Err(StepError::NotAlone { rule, position });
                };
```

`mod.rs:253-256`:

```rust
            StepError::NotAlone { rule, position } => write!(
                f,
                "the rule {rule} needs formula {position} without other formulas in the goal"
            ),
```

So `ax` on formula 0 of `⊢ a, ~a, b` prints "the rule ax needs formula 0 without other formulas in the goal" (the axiom needs `~a` beside it), and on the goal `⊢ a`, which has no other formula, prints the same. The variant's own doc (`mod.rs:127-128`) says "alone with its dual (`ax`). In affine mode, weaken the other formulas first", which the message leaves out. The page shows it as a candidate's refusal (`Status::Refused`).

**Change.** Write the axiom's case apart: "the axiom needs formula {position} with its dual and nothing else", adding "weaken the other formulas first" in affine mode; or return `NoDual` for the axiom, whose message already says this. Keep "without other formulas" for `1`.

**Value.** A learner on the Practice page or in `interact` who tries the axiom too early reads what the axiom needs, not a rule that cannot be met.

**Net.** The test `rejections` (`mod.rs:1346`) pins the variant, not the text; any lock or manual example that prints the sentence changes with it. No checker is touched.

## F-4f-03

- dimension: api
- criterion: C-VALIDATE
- place: core/src/linear/proofs/interactive/expand.rs:750; cli/src/interact/mod.rs:524
- kind: API break
- cost: small
- candidate: no

**Claim.** `split_passes` takes a `Step` but ignores its rule and judges a `⊗` on a `⊗` formula and a Mix on any other, so the command repeats that choice before every call to avoid a wrong answer.

**Witness.** `expand.rs:743-760`:

```rust
    /// It is a cheap test to run before trying a split; a split that
    /// passes may still fail. The step's rule is not read: the step is a
    /// `⊗` on a `⊗` formula, and a Mix on any other.
...
        let rule = if self.forest.kind(o) == Kind::Tensor {
            Rule::Tensor
        } else {
            Rule::Mix
        };
```

`cli/src/interact/mod.rs:526-536`:

```rust
                // The test reads a `⊗` formula's split as the `⊗`'s, so a Mix
                // is judged only at another formula.
                let at_tensor = self.state.goal(goal).ok().and_then(|members| {
                    let member = *members.get(position)?;
                    Some(self.state.forest().kind(self.state.occurrence(member)) == Kind::Tensor)
                });
                let splits = matches!(
                    (step.rule.rule, at_tensor),
                    (Rule::Tensor, Some(true)) | (Rule::Mix, Some(false))
                );
                let lost = splits && !self.state.split_passes(goal, &step).unwrap_or(true);
```

A Mix step on a `⊗` formula, or a `⅋` step, is judged as something else. This adds to F-2b-01 (the same call is not cheap).

**Change.** Judge the step's own rule: refuse a rule other than `⊗` and Mix, and a `⊗` on a formula that is no `⊗`, with `apply`'s `StepError::Rule`, and judge a Mix on a `⊗` formula as a Mix; or take `(goal, position, left)` and a `Split` kind. The command drops its guard.

**Value.** A library user and the command get the answer for the step they pass; the command's copy of the rule (twelve lines) goes. The page's greyed splits (`web/src/calls.rs:815`) cannot be wrong for a Mix on a `⊗` formula.

**Net.** The test `split_helper` (`mod.rs:1844`) and `cli/tests/lock.rs` for `interact`. No checker or search is touched: `focus::counts::split_passes` is called as before.

## F-4f-04

- dimension: text
- criterion: T1
- place: core/src/linear/proofs/interactive/mod.rs:265; core/src/linear/proofs/interactive/mod.rs:287
- kind: text
- cost: small
- candidate: no

**Claim.** Two refusals a learner meets list every cause they could have, though each place that raises them knows which one holds: a split position is "out of range, repeated, or the formula the rule acts on", and an eigenvariable is out of scope "either … or" made too late.

**Witness.** `mod.rs:265-268` and `:287-292`:

```rust
            StepError::Split { position } => write!(
                f,
                "position {position} cannot go to the left premise: it is out of range, repeated, or the formula the rule acts on"
            ),
```
```rust
            StepError::Eigenvariable { name } => write!(
                f,
                "the eigenvariable {name} cannot be used here: either no step below this goal \
                 introduced it (a ∀ step, or ∀R or ∃L two-sided), or it was made after the \
                 open witness it would fill"
            ),
```

The raising places each know the cause: `expand.rs:623-624` (`if p >= sequent.len() || used[p]`, the principal being marked used at :621), `mod.rs:430` (a cut point past the goal), `cut.rs:263-264` (past the goal or repeated); `witness.rs:161-167` and `cut.rs:525-528` (not in the goal's scope), `expand.rs:387-392` (`Clash::Eigen`, made after the open witness).

**Change.** Add a cause to each variant (both are `#[non_exhaustive]`, so a field is additive): `Split { position, cause: Past | Twice | Principal }`, `Eigenvariable { name, cause: OutOfScope | AfterWitness }`, and write one sentence per cause: "the goal has no formula 5", "formula 2 is named twice", "formula 1 is the one the rule acts on"; "no ∀ step below this goal introduced a", "a was made after the open witness $1".

**Value.** A learner on the Practice page or in `interact` reads the one thing to fix. A program reading the JSON error can branch on the cause.

**Net.** The tests `rejections`, `cut::tests::refusals` and the first-order witness tests pin the variants; the written error form gains a field, so the behaviour lock and the manual's examples that print these sentences move in a commit of their own (S19). No checker is touched.

## F-4f-05

- dimension: api
- criterion: S15
- place: core/src/linear/proofs/interactive/mod.rs:617
- kind: text
- cost: small
- candidate: no

**Claim.** The public docs of a session's JSON form leave out two keys the session writes and reads: an inference's `cut` and a first-order session's `instances`, which are documented only on private types.

**Witness.** `mod.rs:619-631`:

```rust
/// With the feature `serialize`, a session is written as
/// `{"version": 1, "sequent": …, "mode": "intuitionistic", "inferences":
/// […], "history": […]}` ([`wire`](crate::wire)), the mode by its name.
///
/// - `inferences` are in the session's own order, the conclusion first.
///   Each is `{"sequent": [ids], "rule": name, "principal": position,
///   "premises": [indices]}`, with the rule's [`name`](Rule::name). An open
///   goal is its sequent alone.
```

against the private form in `core/src/serialize/interactive.rs:33-35` and `:61-64`:

```rust
    /// For a cut, the occurrence of its cut formula.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cut: Option<u32>,
```
```rust
    #[serde(default, skip_serializing_if = "Option::is_none")]
    instances: Option<InstancesForm>,
```

The keys of `instances` (`members`, `frames`, `terms`, `metas`, `eigens`, `constants`, `bindings`, `marks`) are listed only at `serialize/interactive.rs:67-81`, a private module. `core.md` asks every JSON form on its type, `Interactive` named.

**Change.** Add `cut` to the inference line and a bullet for `instances` with its keys to the `# JSON` section of `Interactive`, moved from the private type's comment; link the manual's first-order chapter for what a mark means.

**Value.** A front end other than the page (or a script that saves sessions) learns the whole form from the rustdoc, which is the library's manual; today a cut or a first-order session writes keys it never saw described.

**Net.** Docs only; `cargo doc` and the `doc` check. No checker is touched.

## F-4f-06

- dimension: api
- criterion: C-WORD-ORDER
- place: core/src/linear/proofs/interactive/mod.rs:905; core/src/linear/proofs/interactive/terms.rs:38; core/src/linear/proofs/interactive/expand.rs:781; core/src/linear/proofs/interactive/mod.rs:1022
- kind: API break
- cost: medium
- candidate: no

**Claim.** The linear session's column of F-4e-01's table, which F-arch-12 asks for: it shares the ordinary session's names for goals, steps and undo, but calls its result `proof` where the other two say `finished`, and has five operations neither of the others has and lacks two they have.

**Witness.** The linear session's signatures, beside F-4e-01's columns:

| operation | `linear::Interactive` |
|---|---|
| open goals | `pub fn goals(&self) -> impl Iterator<Item = GoalId> + '_` (`mod.rs:882`) |
| one goal | `pub fn goal(&self, id: GoalId) -> Result<&[Member], StepError>` (`mod.rs:896`) |
| finished? | `pub fn is_complete(&self) -> bool` (`mod.rs:905`) |
| steps | `pub fn steps(&self) -> usize` (`mod.rs:911`) |
| rules | `pub fn rules(&self, goal: GoalId, position: usize) -> Result<Vec<Applicable>, StepError>` (`expand.rs:69`) |
| candidates | `pub fn candidates(&self, goal: GoalId, position: usize) -> Result<Vec<Candidate>, StepError>` (`expand.rs:127`) |
| apply | `pub fn apply(&mut self, goal: GoalId, step: &Step) -> Result<Vec<GoalId>, StepError>` (`expand.rs:261`) |
| undo | `pub fn undo(&mut self) -> Option<GoalId>` (`expand.rs:781`) |
| close | `close(&mut self, goal, options: &Options, view: &ViewOptions, limits: &Limits, stop) -> Result<Closed, Error>` (`expand.rs:837`) |
| hint | none (in the bindings, F-4f-01) |
| invertible steps | none (in the bindings, F-4f-01); `invert(&mut self, goal, &Limits) -> Result<Inverted, StepError>`, `invert_all(&mut self, &Limits) -> Inverted`, `is_invertible(&self, &[Member], &Step) -> bool` (`invert.rs:101-128`) |
| drawn inference to goal | `pub fn derivation_ids(&self) -> Vec<GoalId>` (`mod.rs:1022`) |
| the result | `pub fn proof(&self, limits: &Limits, stop: impl FnMut(Progress) -> bool) -> Result<Proof, Error>` (`terms.rs:38`) |
| linear only | `close_all` (`expand.rs:949`), `close_with` (`:889`), `fill` (`:930`), `cut` (`cut.rs:246`), `split_passes` (`expand.rs:750`) |

`ordinary::Interactive::finished` and `rules::Interactive::finished` return the result; `rules` calls the test `is_finished`.

**Change.** With F-4e-01's renames: `is_finished` for `is_complete`, and `finished` for `proof` (keeping `Proof` as its type), so that the three sessions share `goals`, `goal`, `is_finished`, `steps`, `rules`, `candidates`, `apply`, `undo`, `close`, `hint`, `invertible_steps`, `derivation_ids` and `finished`. `close_all` is worth giving the others, since a front end loops over goals the same way in every suite.

**Value.** A library user and the authors of `cli/src/interact/` and `web/src/` learn one vocabulary for the three sessions, and the JSON interface F-arch-12 proposes names operations that exist under one name in every suite.

**Net.** Renames only: `cargo test --workspace`, the bindings' tests under Node, `breaks 0.8.0`, and the manual and guide where they name `proof()` on a session. No checker, search or certificate is touched.

## F-4f-07

- dimension: api
- criterion: S11
- place: core/src/linear/proofs/interactive/invert.rs:182
- kind: API break
- cost: small
- candidate: no

**Claim.** In a session with quantifiers, a `∀` step the invertible run cannot take (past the session's bound on terms) is skipped silently, and the run reports `finished: Ok(())`, which its docs say means no invertible rule applies to any goal left open.

**Witness.** `invert.rs:182-191`:

```rust
            let premises = if self.fo.is_some() {
                match self.framed_premises(goal, position, rule.rule, &[], &Witness::Open) {
                    Ok(premises) => premises,
                    // Past the eigenvariables a `u32` numbers, or past
                    // the session's bound on terms: the goal stays open.
                    Err(_) => {
                        open.push(goal);
                        continue;
                    }
                }
```

against `Inverted::finished` (`invert.rs:79-82`):

```rust
    /// `Ok` when no invertible rule applies to any goal of
    /// [`open`](Self::open). Otherwise the memory bound stopped it, and the
    /// steps it took stand.
    pub finished: Result<(), Refusal>,
```

`framed_premises` refuses with `WitnessFault::Table { limit }` when `q.too_large()` (`expand.rs:331-334`). The command prints `inverted_line(steps, finished.err())` (`cli/src/interact/mod.rs:836`), so the user sees the run end with a `∀` still open and no word why; `held` has already counted the skipped step's bytes (`:181`).

**Change.** End the run there with `finished: Err(Refusal::Terms { terms, limit })` (the refusal `close` already gives for the same bound, `handover.rs:472`), the steps taken standing, as the memory bound does; or keep going and record the first refusal. Count `held` only for steps taken.

**Value.** A user of the command or the page learns that a bound stopped the invertible steps, as they do for the memory bound, instead of an unexplained open `∀`; a program reading `Inverted` can trust `Ok`.

**Net.** A test of a first-order session at a small `limits.terms` with `invert_all`; `inverting_keeps_every_proof_and_ends` and `inverting_stops_at_the_memory_bound` unchanged. The bindings' `Automatic::add` already writes any refusal. No checker is touched.

## F-4f-08

- dimension: architecture
- criterion: E6
- place: core/src/linear/proofs/interactive/cut.rs:431; core/src/linear/proofs/interactive/expand.rs:526
- kind: refactor
- cost: small
- candidate: no

**Claim.** Inside the unit six small blocks are written two to five times: the open goal's `Inference` literal and the bookkeeping of a step (`record` and `cut`), the step's number with its cast, the mapping of a graft's error, the order of a goal's formulas for the search, the check of a split's positions, and the list of predicates.

**Witness.** The step bookkeeping, `expand.rs:526-546` (`record`) and again `cut.rs:431-449`:

```rust
        let base = self.inferences.len() as u32;
        let ids = [InfId::new(base), InfId::new(base + 1)];
        for premise in premises {
            self.inferences.push(Inference {
                sequent: premise,
                rule: Rule::Open.into(),
                principal: None,
                premises: vec![],
                times: 1,
                cut: None,
                node: None,
            });
        }
```

The same `Inference` literal of an open goal is at `mod.rs:719-727`. `let step = self.history.len() as u32;` with the same comment at `expand.rs:325`, `:368`, `:798`, `handover.rs:268`, `cut.rs:396`. The graft's error mapping, `expand.rs:858-863` and `handover.rs:92-97`:

```rust
                Err(Error::Refused(refusal)) => Err(refusal),
                Err(Error::Check(CheckError::Refused(refused))) => Err(refused.refusal),
                Err(error) => return Err(error),
```

The goal's order, hypotheses first, `handover.rs:110-115` and `:317-322` (`order.sort_by_key(|&m| { reading.position(self.occurrence(m)) == crate::linear::Side::Output })`). The split check, `expand.rs:620-628` and `cut.rs:261-266`. `let predicates: Vec<Symbol> = fo.atoms.iter().map(|&(p, _)| p).collect();` at `witness.rs:42`, `:156`, `handover.rs:161`, `quantified.rs:576`.

**Change.** `Inference::open(sequent)`; `cut` calls `record` and sets the inference's `cut` after; one `fn step_number(&self) -> u32` with the comment once; one `fn graft_refusal(Error) -> Result<Result<(), Refusal>, Error>` for `close` and `close_framed`; `Written` keeps the goal's `order` so `carry_back_from` reads it; one `fn chosen(len, position, left) -> Result<Vec<bool>, StepError>`; a `FirstOrder::is_predicate(symbol)`.

**Value.** A reader of the session sees each piece once; a change to how a step is recorded (cut is the step most often forgotten) or how the goal is ordered for the search is made in one place, and `carry_back_from` cannot read the goal in another order than `written` wrote it.

**Net.** The session's tests (`every_rule_once`, `undo_restores_goals`, `a_cut_opens_two_goals_and_undo_takes_it_back`, the first-order tests), `core/tests/serialize.rs` round trips and the behaviour lock. No checker is touched.

## F-4f-09

- dimension: architecture
- criterion: D4
- place: core/src/linear/proofs/interactive/invert.rs:36; core/src/settings.rs:146
- kind: API break
- cost: small
- candidate: no

**Claim.** The setting that says whether a front end applies the invertible rules, `interactive::Options { invertible }`, lives in the linear session's module, though the command and the page read it for ordinary logic's sessions as well.

**Witness.** `core/src/settings.rs:11-12` and `:144-146`:

```rust
#[cfg(feature = "interactive")]
use crate::linear::proofs::interactive;
```
```rust
    /// How an interactive session takes steps of its own.
    #[cfg(feature = "interactive")]
    pub interactive: interactive::Options,
```

Read for ordinary logic at `cli/src/interact/ordinary.rs:129` (`let invertible = settings.interactive.invertible;`) and `web/src/ordinary.rs:188` (`if settings.interactive.invertible {`). Its type is `linear::proofs::interactive::Options`, a third `Options` in the linear suite beside `search::Options` and the cut's.

**Change.** Move it to the root module F-4e-03 proposes for what the sessions share (with `GoalId` and `Inverted`), named for what it is (`settings::Interactive` or `interactive::Options` at the root), and re-export it from each suite's session.

**Value.** A user of ordinary logic sets the invertible steps without naming the linear suite, and the root's settings name no suite's module (D4).

**Net.** A type move: `cargo test --workspace`, the bindings' tests, `breaks 0.8.0`; the JSON form `{"invertible": …}` is unchanged. No checker is touched.

## F-4f-10

- dimension: performance
- criterion: P1
- place: core/src/linear/proofs/interactive/quantified.rs:1006; core/src/linear/proofs/interactive/witness.rs:206
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** In a session with quantifiers every step remakes the whole shown table, and every step with a given witness or a cut builds a parent list of every inference, so a session of n such steps costs time quadratic in n.

**Witness.** `quantified.rs:1006-1011`, called after every first-order step (`expand.rs:399`, `:435`), every failed one (`:347`), every undo (`:807`) and every graft (`handover.rs:467`):

```rust
    pub(super) fn show(&mut self, forest: &Forest) {
        let base = self.table.base_terms;
        let mut shown = Instances::new(forest);
        let mut at: HashMap<FoTermId, FoTermId> = HashMap::default();
        let terms = self.terms(forest);
        for n in 0..self.table.terms.len() {
```

and it clones the members and recounts every term's size (`:1057-1084`). `witness.rs:206-212`, per `∃` step with a given witness (`:152`) and per cut naming an eigenvariable (`cut.rs:516`):

```rust
    pub(super) fn scope(&self, goal: InfId) -> Vec<Eigen> {
        let mut parent = vec![None; self.inferences.len()];
        for (i, inference) in self.inferences.iter().enumerate() {
```

The reader already replaced the same walk by intervals because it was quadratic (`replay.rs:211-213`; `core-derivations.md:707-709`: 100 000 `∃` steps took 38 s). No ratchet or DHAT journey runs a session.

**Change.** To measure first: a callgrind journey that applies n `∀`/`∃` steps through `apply` (n = 1 000 and 10 000, with and without given witnesses) on a session of nested quantifiers. If quadratic, update the shown table incrementally (append the new entries; remake only where a binding changed) and keep a parent per inference in the session.

**Value.** The page and `interact` stay responsive on long first-order sessions; a front end that replays a saved session through `apply` stays linear.

**Net.** The first-order tests (`a_session_holds_deep_witnesses_and_bounds_its_table`, `a_session_read_back_undoes_as_it_was_written`, the witness tests), `core/tests/serialize.rs`, and the new journey's counts before and after. The proof is still checked at the end; no checker is touched.

## F-4f-11

- dimension: text
- criterion: T1
- place: core/src/linear/proofs/interactive/replay.rs:124; core/src/linear/proofs/interactive/cut.rs:616
- kind: text
- cost: small
- candidate: no

**Claim.** Several refusals of a stored session state a rule of the form instead of what is wrong with the document, so the user reads "not a proof in progress: an open goal has no principal formula and no premise".

**Witness.** With `Error::InconsistentSession`'s prefix "not a proof in progress: " (`errors/mod.rs:280`), `replay.rs:122-125`, `:129-132`, `:347-351`:

```rust
                if inference.principal.is_some() || !inference.premises.is_empty() {
                    return Err(Error::InconsistentSession {
                        reason: "an open goal has no principal formula and no premise",
```
```rust
                    reason: "a cut names its cut formula, every other rule none",
```
```rust
                reason: "every rule but the axiom and Mix names its principal formula, and those two name none",
```

and `cut.rs:615-620`:

```rust
            return Err(inconsistent("an ordered mode has no cut"));
...
            return Err(inconsistent("a cut has two premises"));
```

The same pattern as F-4e-18 in the rule-file session's reader.

**Change.** Say the fault: "an open goal names a principal formula or a premise"; "a cut names no cut formula, or another rule names one"; "a rule names a principal formula it does not have, or lacks one it has"; "a session of an ordered mode has a cut"; "a cut does not have two premises".

**Value.** A user whose saved session (the page's stored proof, a file given to `interact`) is refused reads what is wrong with it.

**Net.** Any test or lock entry matching these sentences changes with them (`core/tests/serialize.rs`'s forged sessions). No checker is touched.

## F-4f-12

- dimension: performance
- criterion: P6
- place: core/src/linear/proofs/interactive/quantified.rs:959; core/src/linear/proofs/interactive/quantified.rs:984
- kind: refactor
- cost: small
- candidate: no

**Claim.** The linear session's first-order part uses the standard library's `HashSet` (SipHash) in two walks beside the crate's `hash` module, which the same file imports for its maps.

**Witness.** `quantified.rs:959-960` (`same_atom`) and `:984` (`variables`):

```rust
        let mut met: std::collections::HashSet<((FoTermId, bool), (FoTermId, bool))> =
            Default::default();
```
```rust
        let mut seen: std::collections::HashSet<FoTermId> = Default::default();
```

while `quantified.rs:16` is `use crate::hash::HashMap;`. Neither is on a search's hot path: `same_atom` runs per axiom the invertible run or the reader checks, `variables` per term the search's handover writes and per given witness the reader checks. These are the unit's only SipHash sites (the every-unit list of F-1a-12, F-1b-13, F-4e-14).

**Change.** Use `crate::hash::HashSet` at both, and keep new sites out with `disallowed-types` in a `clippy.toml`, as F-1a-12 and F-4e-14 propose.

**Value.** The crate keeps one hasher, and the lint that would hold it can be turned on once the other units' sites go too.

**Net.** The first-order session tests and `core/tests/serialize.rs`; only `insert` is read, so iteration order does not matter. No checker is touched.
