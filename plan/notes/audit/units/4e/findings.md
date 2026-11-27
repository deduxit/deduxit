# Unit 4e: findings

Read in full: 5 files. Not read in full: none.

The finder's remarks: I agree with F-arch-12: F-4e-01 lists the signatures it asks for. Each session addresses a formula its own way (side and position in ordinary logic, position in linear logic, member numbers across zones in rule files), so a common typed trait would need an associated step type; arch's interface over JSON fits better, with the names made to agree first. I agree with F-arch-20, and F-4e-05 gives the seams with line ranges; tests are a third of the file. For unit 4f: the linear session has the same missing stop on invert and invert_all (F-4e-07) and the other halves of F-4e-04 and F-4e-11; it also has no `hint` in the library, and its `invertible_steps` is computed in `web/src/calls.rs:455` while ordinary logic's is a library method (also for unit 8). For unit 6a: F-4e-06 (three refusal types and codes for one step refusal). I could not judge any session's performance by measurement, because no ratchet or DHAT journey runs a session; a journey for `rules::Interactive::candidates` on `rules::LK` and one for an ordinary first-order axiom with open witnesses would give F-4e-09 and F-4e-16 an instrument; `rules/search.rs` (close and hint) belongs to unit 5b.

## F-4e-01

- dimension: api
- criterion: C-WORD-ORDER
- place: core/src/rules/interactive.rs:430; core/src/ordinary/interactive.rs:1173; core/src/rules/interactive.rs:1484; core/src/ordinary/interactive.rs:3115
- kind: API break
- cost: medium
- candidate: no

**Claim.** The ordinary session and the rule-file session give different names and shapes to the operations they share (finished, goals, a goal, undo, the drawn inference's goal, the finished derivation), and the linear session differs again.

**Witness.** The operations each session shares, with their signatures (F-arch-12 asks for this list):

| operation | `ordinary::Interactive` | `rules::Interactive` | `linear::proofs::Interactive` (unit 4f) |
|---|---|---|---|
| open goals | `pub fn goals(&self) -> impl Iterator<Item = GoalId> + '_` (:1144) | `pub fn goals(&self) -> Vec<GoalId>` (:416) | iterator (`mod.rs:882`) |
| one goal | `pub fn goal(&self, id: GoalId) -> Result<(&[NodeId], &[NodeId]), StepError>` (:1159) | `pub fn goal(&self, goal: GoalId) -> &[Vec<TermId>]`, panics (:439) | `pub fn goal(&self, id: GoalId) -> Result<&[Member], StepError>` (`mod.rs:896`) |
| finished? | `pub fn is_complete(&self) -> bool` (:1173) | `pub fn is_finished(&self) -> bool` (:430) | `pub fn is_complete(&self) -> bool` (`mod.rs:905`) |
| candidates | `candidates(&self, goal: GoalId, side: Side, position: usize) -> Result<Vec<Candidate>, StepError>` (:1326) | `candidates(&self, goal: GoalId) -> Result<Vec<Candidate>, Error>` (:820) | `candidates(&self, goal: GoalId, position: usize) -> Result<Vec<Candidate>, StepError>` |
| rules | `rules(&self, goal, side, position) -> Result<Vec<Applicable>, StepError>` (:1274) | `rules(&self, goal: GoalId) -> Result<Vec<Applicable>, Error>` (:787) | `rules(&self, goal, position)` |
| apply | `apply(&mut self, goal: GoalId, step: &Step) -> Result<Vec<GoalId>, StepError>` (:1600) | `apply(&mut self, goal: GoalId, step: &Apply<'_>) -> Result<Vec<GoalId>, Error>` (:923) | as ordinary |
| undo | `pub fn undo(&mut self) -> Option<GoalId>` (:2394) | `pub fn undo(&mut self) -> Option<Undone>` (:1389) | `Option<GoalId>` (`expand.rs:781`) |
| close | `close(&mut self, goal, translation: Option<Translation>, search: &search::Options, limits: &Limits, stop)` (:2508) | `close(&mut self, goal, search: &search::Options, limits: &Limits, stop)` (`rules/search.rs:84`) | `close(&mut self, goal, options: &Options, view: &ViewOptions, limits, stop)` (`expand.rs:837`) |
| hint | `hint(&self, goal, translation, search, limits, stop) -> Result<Hint, Error>` (:2542) | `hint(&self, goal, search, limits, stop) -> Result<Hint, Error>` (`rules/search.rs:126`) | none in the library |
| drawn inference to goal | `pub fn derivation_ids(&self, structural: Structural) -> Vec<GoalId>` (:3115) | `pub fn derivation_goals(&self) -> Result<Vec<GoalId>, Error>` (:1484) | `pub fn derivation_ids(&self) -> Vec<GoalId>` (`mod.rs:1022`) |
| the result | `finished(&self, limits: &Limits, mut stop: impl FnMut(Progress) -> bool) -> Result<Derivation, Error>` (:3188) | `finished(&self, stop: impl FnMut(crate::Progress) -> bool) -> Result<Derivation, Error>` (:1504) | `proof(&self, limits: &Limits, stop) -> Result<Proof, Error>` (`terms.rs:38`) |
| steps | `steps(&self) -> usize` (:1179) | `steps(&self) -> usize` (:425) | same |

**Change.** Give the three sessions one name per operation: `is_finished` (it pairs with `finished`), `goals()` as an iterator, `goal(id)` returning a `Result`, `derivation_ids` (taking the view where the session has one), and one return type for `undo`. Keep the per-goal addressing that differs by calculus (side and position, position, member numbers). The JSON interface of F-arch-12 can then name one set of operations.

**Value.** A user of the library and the authors of the two front ends (`cli/src/interact/`, `web/src/`) learn one vocabulary for the three sessions, and the protocol F-arch-12 proposes names operations that exist under the same name in every suite.

**Net.** The renames change no behaviour. Instruments: `cargo test --workspace` (the sessions' tests, `cli/tests/lock.rs` for `interact`) and the bindings' tests under Node; the client's calls are unchanged unless the bindings' export names change. No checker, search or certificate is touched.

## F-4e-02

- dimension: api
- criterion: C-VALIDATE
- place: core/src/rules/interactive.rs:439
- kind: API break
- cost: small
- candidate: no

**Claim.** The rule-file session's `goal`, `members`, `hypotheses` and `lines` index the session directly and panic on an id that is not one of its nodes, so the command checks every goal itself and writes its own copy of the library's refusal.

**Witness.** ```rust
    /// Returns the members of each zone of the goal `goal`.
    ///
    /// # Panics
    ///
    /// For a goal of another session, past this one's nodes.
    pub fn goal(&self, goal: GoalId) -> &[Vec<TermId>] {
        &self.nodes[goal.index()].sequent
    }
```

`members` (:450), `hypotheses` (:465) and `lines` (:560) do the same. The command guards each call with its own check and message (`cli/src/interact/rules.rs:247-253`):

```rust
        if !self.state.goals().contains(&target) {
            bail!(
                "the step is refused: there is no open goal {}",
                target.index()
            );
        }
        let mut before = self.state.members(target);
```

The ordinary session's `goal` returns `Result<(&[NodeId], &[NodeId]), StepError>` (`ordinary/interactive.rs:1159`). `core.md` records that a front end "never builds an error of the library: it asks the call that does (`nets::exist`, `Interactive::goal`)".

**Change.** Return `Result<_, Error>` from `goal`, `members`, `hypotheses` and `lines`, refusing a goal that is not open with `StepFault::NoGoal`, which `open` (:595) already builds. The command then drops its guard and its copy of the message.

**Value.** Users of the library get an error instead of a panic for a stale or foreign goal id from a front end's input. The command and the bindings stop writing the library's message themselves, so the two cannot drift apart.

**Net.** `cargo test --workspace` (`core/tests/rules.rs`, `cli/tests/lock.rs` for `interact` with a calculus) and the bindings' tests under Node (`web/src/rules.rs:215` calls `goal` only on ids from `goals()`). No checker is touched.

## F-4e-03

- dimension: architecture
- criterion: D4
- place: core/src/ordinary/interactive.rs:28; core/src/ordinary/invert.rs:15; core/src/rules/interactive.rs:19; core/src/goal_tree.rs:13
- kind: API break
- cost: large
- candidate: no

**Claim.** The ordinary session's public signatures use the linear suite's `GoalId` and `Inverted`, which `ordinary` does not re-export, while the rule-file session defines a second `GoalId` of its own; even the shared `goal_tree` walks the linear `InfId`.

**Witness.** `core/src/ordinary/interactive.rs:28-29`:

```rust
use crate::linear::proofs::{GoalId, InfId, Size, Structural, ViewOptions};
use crate::linear::search;
```

`core/src/ordinary/invert.rs:15-16` (`invert` returns `Inverted`, whose docs link the linear `Interactive::invert` and `undo`):

```rust
use crate::linear::proofs::GoalId;
use crate::linear::proofs::interactive::Inverted;
```

The bindings drive the ordinary session through the linear path (`web/src/ordinary.rs:25`):

```rust
use deduxit::{Progress, Settings, linear::GoalId, wire};
```

Meanwhile `core/src/rules/interactive.rs:17-19` has its own type with `new` and `index` but no `get`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
#[serde(transparent)]
pub struct GoalId(u32);
```

`core/src/goal_tree.rs:13` is `use crate::linear::proofs::InfId;`, and `ordinary/interactive.rs:2636` writes `crate::linear::search::Reason::Incomplete`.

**Change.** Move `GoalId` (with `new`, `get`, `index`), `Inverted` and the inference id that `goal_tree` walks to one root module of what the sessions share. Replace `rules::GoalId` with it, and have `ordinary` and `rules` re-export it beside their `Interactive`. This is the session part of architecture.md's "move to the root" for `linear::proofs::{InfId, GoalId, …, interactive::Inverted}`; `close`'s `&search::Options` is F-arch-02.

**Value.** A user of ordinary logic or of rule files names the goal type from their own suite. Front ends handle one `GoalId` for all three sessions, and the docs of `Inverted` stop sending ordinary-logic readers to the linear session.

**Net.** Type moves only. Instruments: `cargo test --workspace`, the bindings' tests under Node, `breaks 0.8.0` for the moved paths. No checker, search or certificate is touched; the JSON of a goal id stays a bare number.

## F-4e-04

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/interactive.rs:2061; core/src/linear/proofs/interactive/witness.rs:43
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The reader of a witness term typed as text is written twice, in the ordinary session and the linear one: the same tokenizer, the same stack of open applications, and the same `WitnessFault` refusals.

**Witness.** `core/src/ordinary/interactive.rs:2061-2075`, which is identical, line for line, to `core/src/linear/proofs/interactive/witness.rs:43-56` (duplicates.md lists the pair at 115 tokens):

```rust
        // The tokens: names, brackets and commas.
        let mut tokens = Vec::new();
        let mut name = String::new();
        for c in text.chars().chain([' ']) {
            if c.is_whitespace() || matches!(c, '(' | ')' | ',') {
                if !name.is_empty() {
                    tokens.push(std::mem::take(&mut name));
                }
                if !c.is_whitespace() {
                    tokens.push(c.to_string());
                }
            } else {
                name.push(c);
            }
        }
```

The parsers after it (ordinary :2088-2137, linear :57-120) both keep `open: Vec<(…, Vec<…>)>` and `done: Option<…>` and return the same faults: `Missing`, `Bracket`, `Unopened`, `Comma`, `Applied`, `UnknownSymbol`, `Predicate`, `Arity`. They differ only in what a name resolves to (an interned term, or a `TermNode`) and in ordinary logic's bound on symbols.

**Change.** Write the reader once, beside `WitnessFault` at the root: a function over the text that takes two closures, one resolving a name (eigenvariable or symbol, applied or not) and one building an application, and returns `Result<T, WitnessFault>`. Each session passes its own closures.

**Value.** A reader of the code, and anyone who changes the witness syntax (a new token, a better message), changes one place; today a fix in one session can miss the other.

**Net.** Behaviour kept. Instruments: the witness tests of both sessions (`the_quantifier_rules_take_witnesses_and_eigenvariables`, `a_step_s_term_written_out_replays_the_proof`, and the linear session's in `witness.rs`), `cli/tests/lock.rs` for `interact`, and the examples of the manual. No checker is touched.

## F-4e-05

- dimension: architecture
- criterion: E5
- place: core/src/ordinary/interactive.rs:1069
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `ordinary/interactive.rs` (5 900 lines) holds nine separable parts, among them a 600-line session reader and 1 900 lines of tests, where the linear session it was modelled on is a directory of nine files.

**Witness.** The seams, read in full:

- errors and their text: `StepError`, `BindingFault`, `of`, `target` (:34-372);
- the step vocabulary: `Context`, `Step`, `Applicable`, `Needs`, `Candidate`, `Status`, `Closed`, `Hint`, `Shape` (:374-572);
- open witnesses and unification: `Made`, `Witnesses`, `Unbound`, `unify_axiom`, `arena_terms`, `unfolded`, `implications_from` (:658-960), plus the quantifier steps `unifying_axiom`, `bind`, `instance_step`, `instance_within`, `open_witness`, `fresh_eigenvariable`, `held`, `scope`, `witness`, `applied` (:1621-2162);
- the state and its queries: `new`, `within`, `goals`, `goal`, `rules`, `candidates`, `bindings`, `opened_term` (:1069-1470);
- expansion: `shape`, `apply`, `expand`, `run_within`, `checked`, `split`, `record` (:1472-1619, :2164-2389) and the free functions `premise`, `contracted`, `axiom` (:3204-3391);
- undo: `undo`, `reshow` (:2391-2473);
- the search's graft: `Decided`, `Carry`, `with_derivation`, `takes_open_witness` (:962-1056), then `close`, `hint`, `decide`, `ids_of`, `graft`, `graft_first_order`, `carried`, `first_steps`, `step_at` (:2475-2987);
- the views: `derivation`, `derivation_within`, `finishing`, `derivation_ids`, `finished`, `estimate` (:2989-3201, :3393-3404);
- the reader, behind `serialize`: `Parts`, `parts`, `from_parts`, `Replay`, `reached`, `Tree` (:3406-3999);
- tests (:4000-5900).

`linear/proofs/interactive/` has `mod.rs`, `expand.rs`, `witness.rs`, `terms.rs`, `replay.rs`, `invert.rs`, `cut.rs`, `quantified.rs` and `handover.rs`.

**Change.** Make `ordinary/interactive/` with `mod.rs` (the vocabulary, the state, its queries), `error.rs`, `expand.rs` (with undo), `witness.rs` (open witnesses, the quantifier steps), `close.rs` (decide, graft, hint), `derivation.rs` (the views), `replay.rs` (behind `serialize`) and `tests.rs`, and move `invert.rs` into it. This confirms F-arch-20 with exact seams.

**Value.** A reader, and the reviewer of any later change to the session, holds one part at a time. The `serialize` reader and the tests stop sharing a file with the steps.

**Net.** No public path changes. Instruments: `gate`, the session's tests, `core/tests/first_order.rs`, `cargo hack` (the `serialize` cfg moves to a module). No checker is touched.

## F-4e-06

- dimension: api
- criterion: S18
- place: core/src/errors/mod.rs:1121
- kind: API break
- cost: medium
- candidate: no

**Claim.** A refused step is three error types with two stable codes: the linear and ordinary sessions answer `Error::Step` or `Error::OrdinaryStep` with code `step`, and the rule-file session answers `Error::StepRefused` with code `step_refused`, so the same refusal (no open goal 3) differs by suite in type, code and field type.

**Witness.** `core/src/errors/mod.rs:302`, `:309`, `:722-726`:

```rust
    Step(#[from] crate::linear::proofs::StepError),
```
```rust
    OrdinaryStep(#[from] crate::ordinary::StepError),
```
```rust
    StepRefused {
        /// Why.
        fault: Box<StepFault>,
    },
```

Their codes, `:1073` and `:1121`:

```rust
            Self::Step(_) | Self::OrdinaryStep(_) => "step",
```
```rust
            Self::StepRefused { .. } => "step_refused",
```

No open goal is `ordinary::StepError::NoGoal { goal: GoalId }` (`ordinary/interactive.rs:48-51`) and `StepFault::NoGoal { goal: u32 }` (`errors/step.rs:212-215`).

**Change.** Give a refused step of any session one code (`step`), and one field type for the goal (the shared `GoalId` of F-4e-03). The simplest route keeps the three fault enums and changes `StepRefused`'s code; a fuller one makes `StepFault` a variant family of one session `StepError`.

**Value.** A program that branches on `Error::code()` (the command's exit statuses, the page's messages, a script reading the JSON error) treats a refused step alike whichever suite the session is in.

**Net.** The written error form's `code` changes for rule-file sessions, so the behaviour lock (`cli/tests/lock.rs`, `core/tests/lock.rs`) and the manual's error examples move in a commit of their own (S19). `ErrorKind` stays `invalid`. No checker is touched.

## F-4e-07

- dimension: api
- criterion: S1
- place: core/src/ordinary/invert.rs:109
- kind: API break
- cost: small
- candidate: no

**Claim.** `invert` and `invert_all` run every invertible step to the end with no stop, bounded only by `limits.memory_bytes` (1 GiB by default), though in LK a shared `∧R` doubles the goals at each step.

**Witness.** ```rust
    pub fn invert(&mut self, goal: GoalId, limits: &Limits) -> Result<Inverted, super::StepError> {
```
```rust
    pub fn invert_all(&mut self, limits: &Limits) -> Inverted {
```

The only exit before the end (`invert.rs:149-151`):

```rust
            if let Some(limit) = limits.memory_bytes
                && held > limit
            {
```

`Limits::DEFAULT_MEMORY_BYTES` is `1 << 30` (`limits.rs:125`). On `|- a1 /\ b1, …, an /\ bn` in LK every `∧R` is invertible with a shared context (`invertible`, `invert.rs:41`), so the run makes 2^n goals of n formulas each before the estimate reaches the bound. The command calls it after every step once `invertible on` is set (`cli/src/interact/mod.rs:617`), and the page through `ordinary_invert_all` (`web/src/lib.rs:405`). The linear session's `invert` and `invert_all` (`linear/proofs/interactive/invert.rs:101`, `:108`, unit 4f) have the same signatures.

**Change.** Add `stop: impl FnMut(Progress) -> bool` to `invert` and `invert_all` in both sessions, polled once per step, and end the run with `Inverted::finished = Err(Refusal::Stopped { phase })`, the steps taken standing, as the memory bound already does.

**Value.** A user of the command can end a long run with Ctrl-C (L11), and a page can bound it in time. The call then keeps `core.md`'s rule that every long call takes `&Limits` and a stop.

**Net.** The tests `inverting_keeps_every_valid_goal_valid_and_ends` and `inverting_stops_at_the_memory_bound` pass `|_| false`, plus one test of a stop. The command and the bindings pass their stop. No checker is touched; each step still goes through `apply`.

## F-4e-08

- dimension: text
- criterion: T1
- place: core/src/ordinary/interactive.rs:238
- kind: text
- cost: small
- candidate: no

**Claim.** A learner proving in minimal logic who tries `WR`, `CR`, `⊥R` or `∨R` reads that "every goal of LJ has exactly one" formula right of ⊢, which names another calculus than the one they chose.

**Witness.** `core/src/ordinary/interactive.rs:238-245`:

```rust
                    (Rule::ContractRight, _) => {
                        "its premise would have two formulas right of ⊢, and every goal of LJ \
                         has exactly one"
                    }
                    _ => {
                        "its premise would have no formula right of ⊢, and every goal of LJ has \
                         exactly one"
                    }
```

These arms fire in minimal logic too (`WeakenRight | ContractRight | OrRight | FalseRight => !classical`, :1517). So `WR` in LM prints "the rule WR is not available in minimal logic: its premise would have no formula right of ⊢, and every goal of LJ has exactly one". The same "of LJ" stands in the `∨R` arm (:235-236), in `Succedents` (:260, "every goal of LJ has exactly one: share the context, …"), and in the reader's "an open goal of LJ must have exactly one formula right of ⊢" (:3543).

**Change.** Name the session's logic: "…, and in {logic} logic every goal has exactly one formula there" (the `Logic` variant's `Display` is already in the message), or "every goal of LJ and LM". In `Succedents`, which carries no logic, say "in intuitionistic and minimal logic every goal has exactly one".

**Value.** A learner on the Practice page or in `interact --logic minimal` reads the rule of the logic they are in, not a calculus they did not pick.

**Net.** The test `a_rule_that_does_not_fit_is_refused_with_why` (:4326) and any lock or manual example that prints the sentence change with it. No checker is touched.

## F-4e-09

- dimension: performance
- criterion: P3
- place: core/src/rules/interactive.rs:868
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** Each choice `rules::Interactive::candidates` tries clones the whole session, `plan`, `give` and `fresh` each clone the whole `Calculus` again to escape a borrow, and every trial step rebuilds the session's whole derivation (cloning its `Terms`) to check one node, so one call and one session grow with the session's size times the choices.

**Witness.** `candidates`, per rule and per choice of members (up to `RULE_CHOICES = 4096`):

```rust
                let mut trial = self.clone();
```

(:868; `rules::interactive::Interactive` is 1 024 bytes inline, `rules::calculus::Calculus` 472, by `type-sizes.txt`, besides their heap). Each backward step then runs `let calculus = self.calculus.clone();` in `plan` (:1516), again in `give` (:1783) and in `fresh` (:1882), because `self.terms` is mutated while the calculus is read. `commit` runs `verify` (:750-751):

```rust
    fn verify(&self, node: u32) -> Result<(), Error> {
        let (derivation, places) = self.derived()?;
```

and `derived_with` starts with `out.adopt_terms(self.terms.clone());` (:665) and pushes every node. No ratchet or DHAT journey runs a session (`ratchet.out`).

**Change.** Borrow the fields apart instead of cloning (`let Self { calculus, terms, .. } = self;`, or the planning functions as free functions over `&Calculus` and `&mut Terms`). Try a candidate on a cheaper trial: an undo of the step instead of a whole clone, as the ordinary session does (`ordinary/interactive.rs:1358`). Measure whether keeping the derivation between steps pays for `verify`.

**Value.** The page's `rules_candidates` and `rules_apply` calls, and the command's `rules` listing, get faster on larger goals. On a sequent calculus file (`rules::LK`) a goal of ten members gives each rule ten or a hundred choices.

**Net.** To measure: a new DHAT and callgrind journey that starts a session in `rules::LK` on a goal of about ten members and calls `candidates` once, and one that applies a hundred NK steps, before and after. The rule-file tests (`random_labelled_steps_keep_a_checked_session` in `core/tests/rules.rs`, the session's candidate tests) must hold. The checker still runs on every step; only how its input is built changes.

## F-4e-10

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/interactive.rs:2654; core/src/ordinary/written/mod.rs:776
- kind: refactor
- cost: small
- candidate: no

**Claim.** The copy of a propositional arena into another, node by node, is written twice by hand (`Interactive::ids_of` and `WrittenTableau::differs`), though `transfer::copy` does it, every atom by name, for `roots: None`.

**Witness.** `core/src/ordinary/interactive.rs:2654-2668`:

```rust
    fn ids_of(formulas: &mut Formulas, other: &Formulas) -> Result<Vec<NodeId>, Error> {
        let mut ids: Vec<NodeId> = Vec::with_capacity(other.len());
        // Every subformula precedes the formulas built from it.
        for &node in &other.nodes {
            let at = |k: NodeId| ids[k.index()];
            let id = match node {
                Node::Atom(a) => formulas.atom(other.atom_name(a))?,
                Node::True | Node::False => formulas.add(node)?,
                Node::Not(a) => formulas.add(Node::Not(at(a)))?,
```

`core/src/ordinary/written/mod.rs:776-788` has the same loop and match (duplicates.md, 199 tokens). `core/src/ordinary/transfer.rs:62-66`: "Copies into `target` the formulas `roots` of `source` … or every node and term of `source` for `None` … otherwise every atom is copied by its name alone." `first_steps` already uses `transfer::copy`'s result for the first-order case (:2875-2882).

**Change.** Replace both loops with `transfer::copy(target, source, None, &Parameters::Constants)` and read ids through `Copied::node`. The `graft` and `first_steps` arms for the propositional case then share the first-order arm's path.

**Value.** A reader finds one way to carry formulas between arenas. A later node kind, or a fix in how atoms are matched, is made once.

**Net.** Instruments: `close_and_hint`, `following_the_hints_proves_a_sequent`, `undo_restores`, `random_steps_finish_as_checked_derivations`, and the tableau review tests of `written/`. The graft's inferences are those `decide` checked; no checker is touched.

## F-4e-11

- dimension: idiom
- criterion: E6
- place: core/src/ordinary/interactive.rs:3027; core/src/linear/proofs/interactive/mod.rs:984
- kind: refactor
- cost: small
- candidate: no

**Claim.** `derivation_within` is the same in the ordinary and linear sessions (the estimate refused past `derivation_bytes`, one stop, the postorder renumbered), and the ordinary file renumbers the postorder a second time in `derivation_ids`.

**Witness.** `core/src/ordinary/interactive.rs:3027-3035`:

```rust
        if stop(Progress::new(Phase::View, 0, 0)) {
            return Err(Error::Refused(Refusal::Stopped { phase: Phase::View }));
        }
        let order = self.postorder();
        let mut new_id = vec![InfId::new(u32::MAX); self.inferences.len()];
        for (i, id) in order.iter().enumerate() {
            // As many as the state's inferences, which a `u32` counts.
            new_id[id.index()] = InfId::new(i as u32);
        }
```

`core/src/linear/proofs/interactive/mod.rs:984-990` is the same without the comment, after the same `Refusal::Output { what: "derivation", … }` block (duplicates.md: 169 tokens, :971 and :3016). The renumbering is written again at `ordinary/interactive.rs:3123-3126`.

**Change.** Add `goal_tree::renumbered(order: &[InfId], len) -> Vec<InfId>` beside `goal_tree::postorder`, which both sessions already share, and one helper that refuses an estimate past `limits.derivation_bytes`. Call them from the three places.

**Value.** A reader of the code sees that the two sessions draw alike. A change to the refusal or the stop is made once.

**Net.** Instruments: `the_bounds_hold`, `structural_rules_explicit_or_admissible`, the linear session's view tests and the behaviour lock. No checker is touched.

## F-4e-12

- dimension: idiom
- criterion: E6
- place: core/src/ordinary/interactive.rs:2719; core/src/ordinary/interactive.rs:2784
- kind: refactor
- cost: small
- candidate: no

**Claim.** Inside `ordinary/interactive.rs` three blocks are written several times: the fitting of a graft's root onto the goal (twice), the mapping from a rule's shape to what it needs (twice), and the next eigenvariable number or its refusal (four times).

**Witness.** `graft` (:2719-2729) and `graft_first_order` (:2784-2795) both do (duplicates.md, 133 tokens):

```rust
        if !same(&top.left, &goal_inference.left) || !same(&top.right, &goal_inference.right) {
            return Err(Error::GoalMismatch);
        }
        top.principal = top.principal.map(|(side, position)| {
            let id = top.side(side)[position];
            let list = goal_inference.side(side);
            (side, list.iter().position(|&y| y == id).unwrap_or(position))
        });
        top.left.clone_from(&goal_inference.left);
        top.right.clone_from(&goal_inference.right);
```

`rules` (:1294-1300) and `candidates` (:1336-1342) both write:

```rust
                    Shape::Binary(_) => Needs::Context,
                    Shape::Instance if matches!(rule, Rule::ForallLeft | Rule::ExistsRight) => {
                        Needs::Witness
                    }
                    Shape::Leaf | Shape::Unary(..) | Shape::Instance => Needs::Nothing,
```

`.map_or(Some(0), |e| e.checked_add(1))` (or `|k|`) gives the next eigenvariable at :1953, :1975, :2833 and :3095, and the same `Refusal::Index { what: Space::Term, count: u64::from(u32::MAX) + 1, most: u64::from(u32::MAX) }` is built at :2834-2838 and :3082-3086.

**Change.** Add `fn fit_root(goal: &Inference, top: &mut Inference) -> Result<(), Error>`, `fn needs(rule: Rule, shape: &Shape) -> Needs`, and one helper for the next eigenvariable number with its refusal. Call each from its places.

**Value.** A reader of the session and the next change to how a graft's principal is found or how eigenvariables are numbered (S3: the numbering soundness rests on) touches one place.

**Net.** Instruments: `close_and_hint`, `the_search_closes_or_refutes_a_first_order_goal`, `a_derivation_s_own_symbols_are_renamed_apart`, `candidates_agree_with_apply`, `random_steps_finish_as_checked_derivations`. No checker is touched; `check_inference` still runs on every grafted inference.

## F-4e-13

- dimension: idiom
- criterion: E6
- place: core/src/rules/interactive.rs:925
- kind: refactor
- cost: small
- candidate: no

**Claim.** The rule-file session looks a rule up by name, and turns `Unplanned` into an `Error`, by the same lines in `apply`, `derive`, `extend` and `premise_bindings`, and maps a checker's refusal to `StepFault::Check` by the same closure in `verify` and `verify_all`.

**Witness.** `core/src/rules/interactive.rs:925-935` (`apply`):

```rust
        let Some(&index) = self.calculus.rule_ids.get(step.rule) else {
            return Err(refused(StepFault::NoRule {
                rule: step.rule.to_owned(),
            }));
        };
        let rule = self.calculus.rules[index as usize].clone();
        let plan = match self.plan(goal, &rule, step) {
            Ok(plan) => plan,
            Err(Unplanned::Needs(needs)) => return Err(Error::StepNeeds { needs }),
            Err(Unplanned::Error(error)) => return Err(error),
        };
```

The lookup is written again at :1032-1037, :1136-1141 and :1860-1865, and the mapping at :1059-1063, :1186-1190 and :1867-1871. `verify` (:759-762) and `verify_all` (:771-774) share:

```rust
            .map_err(|error| match error {
                Error::InvalidDerivation { reason, .. } => refused(StepFault::Check { reason }),
                other => other,
            })
```

**Change.** Add `fn rule(&self, name: &str) -> Result<&Rule, Error>`, `impl From<Unplanned> for Error` (`Needs` to `Error::StepNeeds`), and one `fn as_step(error: Error) -> Error` for the checker's refusal. Call them from the seven places.

**Value.** A reader of the session sees one path for every step kind, and a change to the refusal of an unknown rule is made once.

**Net.** Instruments: `core/tests/rules.rs`, the session's tests, and the behaviour lock for `interact` with a calculus. No checker is touched.

## F-4e-14

- dimension: performance
- criterion: P6
- place: core/src/unify.rs:15
- kind: refactor
- cost: small
- candidate: no

**Claim.** The unifier the sessions share uses the standard library's `HashSet` (SipHash) for its two sets of pairs and terms seen, beside the crate's `hash` module (foldhash), which the ordinary session uses for the same kind of set.

**Witness.** ```rust
use std::collections::HashSet;
```

used at `core/src/unify.rs:208` (`let mut met: HashSet<(FoTermId, FoTermId)> = HashSet::default();`, per `unify` call) and `:251` (`let mut seen: HashSet<FoTermId> = HashSet::default();`, per `bind`). `ordinary/interactive.rs` uses `crate::hash::HashSet` for its sets of terms seen (:869, :2001). These are the unit's only SipHash sites. They are not on a search's hot path: one call per axiom step with open witnesses, and per partner in `rules`, `unifies` and `candidates`.

**Change.** Use `crate::hash::HashSet` in `unify.rs`, and keep new SipHash sites out with `disallowed-types` in a `clippy.toml`, as units 1a and 1b propose.

**Value.** The crate keeps one hasher. The lint that would hold that cannot be turned on while this site remains.

**Net.** Instruments: the sessions' first-order tests (`an_axiom_fills_the_open_witnesses`, `a_binding_shown_through_its_holders_is_the_walk_s`) and the linear session's. Iteration order is not read (only `insert`), so behaviour is kept. No checker is touched.

## F-4e-15

- dimension: text
- criterion: T1
- place: core/src/ordinary/interactive.rs:1242
- kind: text
- cost: small
- candidate: no

**Claim.** The doc comment of `widths` says no step adds a formula, but every quantifier step adds its instance to the arena, and the invertible run re-computes the widths for that reason.

**Witness.** `core/src/ordinary/interactive.rs:1241-1243`:

```rust
    /// Returns the width of every formula of the state, as the
    /// derivation's estimate counts it. No step adds a formula.
    pub(super) fn widths(&self) -> Vec<u64> {
```

against `core/src/ordinary/invert.rs:144-147`:

```rust
            // A quantifier step adds its instance to the arena.
            if widths.len() < self.formulas().len() {
                widths = self.widths();
            }
```

and `instance_step`'s `self.formulas.instantiate(body, term)` (:1833-1835).

**Change.** Rewrite the sentence: "A quantifier step adds its instance, so a caller that keeps the widths takes them again when the arena grew."

**Value.** A reader of the bound on the invertible run (and of `added_bytes`, which takes these widths) is not misled into dropping the re-computation in `invert.rs`.

**Net.** Comment only; no instrument needed.

## F-4e-16

- dimension: performance
- criterion: S5
- place: core/src/rules/interactive.rs:2209
- kind: refactor
- cost: small
- candidate: yes

**Claim.** The reader of a rule-file session scans every scope once per scope and once per history entry, so a document with many scopes, which a session of many discharging steps writes, is read in time quadratic in its scopes.

**Witness.** `core/src/rules/interactive.rs:2209-2213`, inside a loop over every scope:

```rust
            let unique = scopes
                .iter()
                .filter(|other| other.owner == scope.owner)
                .count()
                == 1;
```

Once per history entry (:2319-2323):

```rust
                    let owned = session
                        .scopes
                        .iter()
                        .filter(|s| s.owner.is_some_and(|(o, _)| o == goal))
                        .count();
```

and once per node with a parent (:2179-2182, `scopes.iter().position(|s| s.owner == Some((p, k)) && s.parent == up.scope)`). Each `→I` or `∨E` step opens a scope (`commit`, :953-961). The ordinary and linear readers are linear in the document by their rules files (`core-derivations.md`, "Reading a session back is linear").

**Change.** Build once a map from each owner `(node, premise)` to its scope (and a count per owner node), and read it in the three loops.

**Value.** A page or the command reading a stored session (S5, a document that may come from anywhere) answers in linear time.

**Net.** To measure first: read, under callgrind, a session document of n discharging steps for n = 1 000 and 10 000 (written by applying `→I` repeatedly in `rules::NJ`) and compare the counts. The reader's refusals must stay those of `core/tests/rules.rs`'s forged documents. The checker run at the end (`check_open`) is unchanged.

## F-4e-17

- dimension: text
- criterion: T4
- place: core/src/ordinary/interactive.rs:4912
- kind: text
- cost: small
- candidate: no

**Claim.** A test comment is two comments interleaved line by line, so neither reads.

**Witness.** ```rust
        // `∃L` names `a`, the first name no symbol has; undone and taken
        // The right side's `∃y. p(y)` is the hypotheses' formula, which
        // prints with the name written first.
        // again, it names `a` again. The other branch cannot use it.
```

**Change.** Restore the two comments: "`∃L` names `a`, the first name no symbol has; undone and taken again, it names `a` again. The other branch cannot use it." Put the sentence on the right side's `∃y. p(y)` where it belongs, or drop it if it no longer applies.

**Value.** A reader of the test learns what it pins.

**Net.** Comment only; no instrument needed.

## F-4e-18

- dimension: text
- criterion: T1
- place: core/src/rules/interactive.rs:2078
- kind: text
- cost: small
- candidate: no

**Claim.** Some refusals of the rule-file session's reader state a rule as if it were a fact, or use "is no", so the message does not say what is wrong with the document.

**Witness.** With `Error::InconsistentSession`'s prefix "not a proof in progress: " (`errors/mod.rs:280`), `core/src/rules/interactive.rs:2078`:

```rust
            return Err(fault("a session has a node and a scope"));
```

prints "not a proof in progress: a session has a node and a scope". At :2151 and :2162:

```rust
                    return Err(fault("the nodes are no tree with its root first"));
```

**Change.** "it has no node or no scope"; "the nodes do not form a tree with the root first".

**Value.** A user whose stored session (the page's saved proof, a file given to `interact`) is refused reads what is wrong with it.

**Net.** Any test or lock entry that matches these sentences changes with them. No checker is touched.
