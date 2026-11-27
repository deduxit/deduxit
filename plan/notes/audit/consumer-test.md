# The API consumer test (step 86, phase A)

A scratch crate outside the workspace that builds two toy engines on
deduxit's public API alone, written as a researcher would write them:
from the crate docs, the doc comments of the items used, and the listing
of public items (`baselines.md`). It was built against `core/` at main
(`urvzlzxl`) with the default features, on 2026-11-27. This file records
what worked and each place the API got in the way; it is evidence for
phase B, cited by the criteria X1 to X9 of `criteria.md`. The crate's
source is at the end.

## What was built

- **A naive backward search for MLL with units** (`mll.rs`, 150 lines):
  reads a `linear::Sequent`, numbers its occurrences with a `Forest`,
  searches over contexts of `OccId`s (par and bottom first, then the
  axiom and one, then every tensor with every split), builds
  `proofs::Node`s, and hands them to `Proof::new`. deduxit's
  `Proof::check` decides the proof, `Proof::derivation` prints it and
  `export::latex::to_string` exports it. The answer is compared with
  `linear::prove`'s.
- **A truth table for classical propositional logic** (`truth.rs`, 55
  lines): reads an `ordinary::Sequent`, evaluates its `Node`s under
  every valuation, and for a falsifying one builds a one-world
  `kripke::Model` that `Model::refutes` and `Countermodel::check`
  accept. The answer is compared with `ordinary::decide`'s.

Both ran on their first or second compile and agreed with deduxit on
every input: seven linear sequents, `families::three_partition_mll`
(proved and checked, 20 occurrences), and five ordinary sequents.

## What worked well

- **The forest is enough for a search.** `Forest::roots`, `kind`,
  `left`, `right`, `atom`, `sign` and `ids` were all the toy engine
  needed, and `Member::from(OccId)` turns an occurrence into what a
  node names.
- **The proof term is documented where one looks.** The doc comment of
  each `Node` variant states its rule with premises and conclusion;
  the search compiled from it at the first try.
- **`Proof::new` drops what the root does not reach**, so a
  backtracking search may leave its failed branches in the vector.
- **The checker is the trust boundary, and it is one call**:
  `proof.check(Mode::CLASSICAL)?`. Everything after it (the derivation,
  the LaTeX export) took the outside proof as it takes deduxit's own.
- **A countermodel is checked the same way**: `Model::refutes`.

## Where the API got in the way

Each has its witness. The compile errors are those of
`src/bin/attempts.rs` below, on Rust 1.100.

1. **A linear sequent cannot be built without text or JSON** (X1).
   `Sequent::new()` is empty, and its only `&mut self` methods are
   `add(Self)`, `add_cut(Self)` and `optimize`; the constructors
   `from_parts` and `raw` are `pub(crate)`
   (`core/src/linear/sequents/mod.rs:415`, `:991`). An engine that
   makes a formula (a cut formula, an interpolant, a translation's
   image, a generated family) must print it and parse it. Witness:
   attempt 4, `E0599: no method named push found for struct Sequent`.
   The ordinary suite has what is missing here: `Formulas::atom(&str)`,
   `Formulas::add(Node)` and `Sequent::new(formulas, left, right)`.
   With `default-features = false` there is no way left at all, since
   `FromStr` is behind `parse` and the JSON reader behind `serialize`.
2. **An outside engine cannot honour the stop convention** (X2). Every
   long call takes `stop: impl FnMut(Progress) -> bool`, but `Progress`
   is `#[non_exhaustive]` with public fields and no constructor, so
   only the crate can call a stop. Witness: attempt 1, `E0639: cannot
   create non-exhaustive struct using struct expression`.
3. **An outside engine cannot refuse in the crate's error family**
   (X2). `Error::from(Refusal)` exists, but every variant of `Refusal`
   is a non-exhaustive struct variant without a constructor. Witness:
   attempt 3, `E0639: cannot create non-exhaustive variant using struct
   expression`. The engine needs an error type of its own, which a
   front end cannot sort into refusal (exit status 3) and fault (2) by
   `Error::kind`.
4. **An outside engine cannot answer with an `Outcome`** (X4).
   `linear::Outcome` is `#[non_exhaustive]` without a constructor
   (attempt 2, `E0639`), and its field `engine` is the crate's own list
   `Engine`, which `Options::engine` selects from. There is no trait or
   registry by which the front door, the batch, the command or the
   bindings would run an engine from another crate. `Verdict` is
   closed and can be built, so the toy engine returns an enum of its
   own.
5. **What a value promises is not on the value** (X3).
   `Verdict::Proved(Box<Proof>)` holds any proof, checked or not
   (`Outcome::checked` says which, where there is an `Outcome`), and
   attempt 5 compiles: any crate can build
   `Disproof::new(sequent, mode, Refutation::Exhausted)`. That is sound
   as long as readers treat `check` as the boundary; the docs should
   say so on these types.
6. **The first call in ordinary logic answers for intuitionistic
   logic** (D1). `ordinary::decide(&sequent, &Options::default())`
   says that `a \/ ~a` is not valid, because `Logic::Intuitionistic` is
   the `#[default]` (`core/src/ordinary/mod.rs:166`). The toy engine's
   comparison failed on it until the logic was set. D1 removes the
   default in the command; the library's `Default` has the same
   question.
7. **Two styles of options values** (C-BUILDER, C-CTOR).
   `linear::Options` and `Limits` are changed by `with_…` methods;
   `ordinary::Options` has public fields and no method, so the logic is
   set by `let mut options = Options::default(); options.logic = …`.
8. **The suites name the same things differently** (C-WORD-ORDER, X1).
   A formula's node is `linear::sequents::Term`, read by
   `Sequent::term(id)`, and `ordinary::Node`, read by
   `Formulas::node(id)`; `linear::proofs::Node` is a node of a proof.
   Ids are newtypes with `new`, `get` and `index` in the linear suite
   (`Atom`, `TermId`, `OccId`, `NodeId`), while `ordinary::Node::Atom`
   holds a bare `u32`, `ordinary::NodeId` has `index` only, and a
   Kripke world is a bare `u32` whose atoms are names in a
   `Vec<String>`. The text syntaxes differ too (`*`, `par`, `-o`
   against `/\`, `\/`, `->`), which is the logics' own notation, but a
   reader finds it only on each `Sequent`.
9. **One type under several paths** (C-HIDDEN). `Derivation` is
   `deduxit::linear::Derivation`, `…::linear::proofs::Derivation` and
   `…::linear::proofs::derivation::Derivation`; `Kind` is
   `linear::sequents::Kind` and `linear::sequents::term::Kind`;
   `Fault` is listed under `proofs` and `proofs::check`. The listing
   counts an item once per path (5 128 lines), and a reader of an
   error message or of rustdoc cannot tell which path is the one to
   write.
10. **The ordinary suite's public types lean on the linear one**
    (D4). `ordinary::Verdict::Unknown` holds a
    `linear::search::Reason`, and `ordinary::decide_within` takes
    `&linear::search::Options` beside its own. `kripke::Model`, the
    shared model, has `forcing` and `refutes` for ordinary sequents and
    `truth` and `frame_fault` for modal ones as inherent methods.
11. **Two calls check a countermodel**: `Model::refutes(&sequent,
    logic, …)` and `Countermodel::new(sequent, model).check(logic, …)`.
    Both accepted the toy engine's model; which one a user should call
    is not said.
12. **Nothing says where an engine writer starts** (X8). The crate
    docs list what the crate decides, with the linear suite first and
    in every leading example (D4); the way from a `Forest` to a checked
    `Proof` is found only by reading `Node` and `Proof::new`.
13. **The data model alone still brings the engines' cost** (X7). The
    default features compile 19 crates before the consumer (batsat,
    serde and its derive with syn among them; 41 s in the dev profile
    on three efficiency cores, the core crate most of it). `cargo tree`
    lists 27 normal dependency lines with the defaults and 14 without,
    and without them point 1 leaves no sequent to work on. No feature
    selects "sequents, forest, proofs and the checker" by themselves.

## What the test did not cover

- The interactive states, nets, the modal suite, lambda-terms, calculi
  given as data, the protocol and the workspace.
- First-order sequents: binders, instances and witnesses.
- A derivation of ordinary logic built by hand (`ordinary::Derivation`,
  `rules::Derivation`); the truth table has no certificate.
- The bindings in `web/` as a consumer, which phase B reads as area 8.

## The crate

`Cargo.toml` names one dependency, `deduxit = { path = "…/core" }`, and
an empty `[workspace]` table.

### `src/mll.rs`

```rust
use deduxit::linear::occurrences::{Forest, Member, OccId};
use deduxit::linear::proofs::{Node, NodeId, Proof};
use deduxit::linear::sequents::Kind;
use deduxit::linear::{Mode, Options, Sequent, Verdict, prove};
use std::error::Error;

pub enum Answer {
    Proved(Proof),
    Unprovable,
    Unsupported(Kind),
}

struct Search<'a> {
    forest: &'a Forest,
    nodes: Vec<Node>,
    steps: u64,
}

impl Search<'_> {
    fn push(&mut self, node: Node) -> NodeId {
        let id = NodeId::new(u32::try_from(self.nodes.len()).expect("a toy proof is small"));
        self.nodes.push(node);
        id
    }

    fn prove(&mut self, context: &[OccId]) -> Option<NodeId> {
        self.steps += 1;
        let kept = self.nodes.len();
        let found = self.rules(context);
        if found.is_none() {
            self.nodes.truncate(kept);
        }
        found
    }

    fn rules(&mut self, context: &[OccId]) -> Option<NodeId> {
        let forest = self.forest;
        let without = |i: usize| {
            let mut rest = context.to_vec();
            rest.swap_remove(i);
            rest
        };
        if let Some(i) = context.iter().position(|&o| forest.kind(o) == Kind::Par) {
            let o = context[i];
            let mut rest = without(i);
            rest.extend([forest.left(o)?, forest.right(o)?]);
            let premise = self.prove(&rest)?;
            return Some(self.push(Node::Par(Member::from(o), premise)));
        }
        if let Some(i) = context.iter().position(|&o| forest.kind(o) == Kind::Bot) {
            let premise = self.prove(&without(i))?;
            return Some(self.push(Node::Bot(Member::from(context[i]), premise)));
        }
        if let &[o] = context
            && forest.kind(o) == Kind::One
        {
            return Some(self.push(Node::One(Member::from(o))));
        }
        if let &[a, b] = context
            && forest.atom(a).is_some()
            && forest.atom(a) == forest.atom(b)
            && forest.sign(a) != forest.sign(b)
        {
            return Some(self.push(Node::Ax(Member::from(a), Member::from(b))));
        }
        for i in (0..context.len()).filter(|&i| forest.kind(context[i]) == Kind::Tensor) {
            let o = context[i];
            let rest = without(i);
            for mask in 0..1_u64 << rest.len() {
                let (mut gamma, mut delta) = (vec![forest.left(o)?], vec![forest.right(o)?]);
                for (j, &other) in rest.iter().enumerate() {
                    if mask >> j & 1 == 1 { gamma.push(other) } else { delta.push(other) }
                }
                let Some(left) = self.prove(&gamma) else { continue };
                let Some(right) = self.prove(&delta) else { continue };
                return Some(self.push(Node::Tensor(Member::from(o), left, right)));
            }
        }
        None
    }
}

pub fn decide(sequent: &Sequent) -> Result<Answer, Box<dyn Error>> {
    let forest = Forest::new(sequent)?;
    for o in forest.ids() {
        let kind = forest.kind(o);
        if !matches!(
            kind,
            Kind::Atom | Kind::DualAtom | Kind::Tensor | Kind::Par | Kind::One | Kind::Bot
        ) {
            return Ok(Answer::Unsupported(kind));
        }
    }
    let mut search = Search { forest: &forest, nodes: Vec::new(), steps: 0 };
    let roots = forest.roots().to_vec();
    let Some(root) = search.prove(&roots) else {
        return Ok(Answer::Unprovable);
    };
    let nodes = search.nodes;
    Ok(Answer::Proved(Proof::new(forest, nodes, root)?))
}

pub fn run(text: &str) -> Result<(), Box<dyn Error>> {
    let sequent: Sequent = text.parse()?;
    let own = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
    match decide(&sequent)? {
        Answer::Proved(proof) => {
            proof.check(Mode::CLASSICAL)?;
            println!("{sequent}: proved, checked\n{}", proof.derivation()?);
            let latex = deduxit::export::latex::to_string(
                &proof.derivation()?,
                &deduxit::export::latex::Options::default(),
            )?;
            println!("  ({} bytes of LaTeX)", latex.len());
            assert!(matches!(own.verdict, Verdict::Proved(_)), "{text}");
        }
        Answer::Unprovable => {
            println!("{sequent}: unprovable");
            assert!(matches!(own.verdict, Verdict::Unprovable(_)), "{text}");
        }
        Answer::Unsupported(kind) => println!("{sequent}: outside MLL ({kind:?})"),
    }
    Ok(())
}
```

`main.rs` calls `run` on `A, A -o B |- B`, `|- A * B, ~A par ~B`,
`|- A * B, ~A, ~B`, `|- (A * B) * C, ~A par (~B par ~C)`,
`|- A par ~A, B`, `|- 1, bot` and `|- A, ~A, B, ~B`, and `decide` on
`families::three_partition_mll(&[1, 1, 1], 1, 3)` and `families::mix(2)`
(outside MLL: it has `⊕`).

### `src/truth.rs`

```rust
use deduxit::Limits;
use deduxit::kripke::{Model, World};
use deduxit::ordinary::{Countermodel, Logic, Node, NodeId, Options, Sequent, Verdict, decide};
use std::error::Error;

fn eval(sequent: &Sequent, id: NodeId, valuation: u64) -> Option<bool> {
    let go = |x| eval(sequent, x, valuation);
    Some(match sequent.formulas().node(id) {
        Node::Atom(a) => valuation >> a & 1 == 1,
        Node::True => true,
        Node::False => false,
        Node::Not(x) => !go(x)?,
        Node::And(x, y) => go(x)? & go(y)?,
        Node::Or(x, y) => go(x)? | go(y)?,
        Node::Implies(x, y) => !go(x)? | go(y)?,
        Node::Iff(x, y) => go(x)? == go(y)?,
        Node::Forall(_) | Node::Exists(_) => return None,
    })
}

pub fn run(text: &str) -> Result<(), Box<dyn Error>> {
    let sequent: Sequent = text.parse()?;
    let names = sequent.formulas().atom_names().to_vec();
    let mut options = Options::default();
    options.logic = Logic::Classical;
    let own = decide(&sequent, &options)?;
    for valuation in 0..1_u64 << names.len() {
        let mut left = sequent.left().iter().map(|&l| eval(&sequent, l, valuation));
        let mut right = sequent.right().iter().map(|&r| eval(&sequent, r, valuation));
        if left.all(|v| v == Some(true)) && right.all(|v| v == Some(false)) {
            let true_atoms: Vec<String> = names
                .iter()
                .enumerate()
                .filter(|&(a, _)| valuation >> a & 1 == 1)
                .map(|(_, name)| name.clone())
                .collect();
            let model = Model::new(vec![World::new(true_atoms, false, Vec::new())], 0);
            model.refutes(&sequent, Logic::Classical, &Limits::default(), |_| false)?;
            let countermodel = Countermodel::new(sequent.clone(), model);
            countermodel.check(Logic::Classical, &Limits::default(), |_| false)?;
            println!("{text}: not valid, countermodel checked");
            assert!(matches!(own.verdict, Verdict::NotValid), "{text}");
            return Ok(());
        }
    }
    println!("{text}: valid by its truth table (no certificate to check)");
    assert!(matches!(own.verdict, Verdict::Valid(_)), "{text}");
    Ok(())
}
```

`main.rs` calls it on `a -> b, a |- b`, `|- a \/ ~a`,
`a \/ b |- a /\ b`, `|- ((a -> b) -> a) -> a` and `a <-> b, b |- a`.

### `src/bin/attempts.rs`, which does not compile

```rust
use deduxit::limits::{Phase, Progress, Refusal};
use deduxit::linear::search::{Engine, Outcome, Statistics};
use deduxit::linear::sequents::{Atom, Term, TermId};
use deduxit::linear::{Disproof, Fragment, Mode, Refutation, Sequent, Verdict};

// 1. Tell the caller's stop the progress made: E0639.
fn poll(stop: &mut impl FnMut(Progress) -> bool, work: u64) -> bool {
    stop(Progress { phase: Phase::Search, done: 0, work, held_bytes: 0, item: 0 })
}

// 2. Answer with the crate's outcome: E0639.
fn outcome(verdict: Verdict, mode: Mode) -> Outcome {
    Outcome {
        verdict,
        mode,
        fragment: Fragment::MLL,
        engine: Engine::Focus,
        statistics: Statistics::default(),
        checked: true,
        net: None,
    }
}

// 3. Refuse as the crate's engines do: E0639.
fn refuse(limit: u64) -> deduxit::Error {
    deduxit::Error::from(Refusal::Work { limit })
}

// 4. Build a sequent without the text parser: E0599, no such methods.
fn build() -> Sequent {
    let mut sequent = Sequent::new();
    let a: Atom = sequent.intern("A");
    let left: TermId = sequent.push(Term::Atom(a));
    let right: TermId = sequent.push(Term::DualAtom(a));
    sequent.push_root(sequent.push(Term::Par(left, right)));
    sequent
}

// 5. Say that a search of one's own found no proof: this compiles.
fn unprovable(sequent: Sequent) -> Verdict {
    Verdict::Unprovable(Box::new(Disproof::new(sequent, Mode::CLASSICAL, Refutation::Exhausted)))
}
```
