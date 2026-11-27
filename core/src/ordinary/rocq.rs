// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The certificate of an ordinary sequent: a lemma over `Prop` whose
//! proof is a term made from a checked derivation of LK or LJ, one
//! construction per rule. An LJ sequent `Γ ⊢ C` is a term of type `C`
//! (`False` when nothing is right of `⊢`) with a variable per hypothesis.
//! An LK sequent `Γ ⊢ Δ` is a term of type `False` with a variable per
//! hypothesis and one of type `~ D` per formula `D` of `Δ`; the rules that
//! move a formula to the right use double-negation elimination (`NNPP`
//! from the standard library's `Classical_Prop`). The terms are written by
//! a walk with a stack of its own, so a derivation of any height is
//! written.

use super::derivation::Side;
use super::{
    Countermodel, Derivation, Formulas, Image, Logic, Node, NodeId, Rule, Sequent, Symbols,
};
use crate::Error;
use crate::export::Form;
use crate::export::rocq::{
    CLASSICAL_PRELUDE, DEDUXIT_KRIPKE_PRELUDE, Options, Unsupported, identifiers,
};
use crate::export::rocq::library::{self, stopped, string};
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::proofs::InfId;
use crate::linear::search::{Classical, Disproof, Refutation};
use crate::linear::sequents::{Stop, Stops};
use std::fmt::Write;

/// The names the terms use. An atom's identifier that is one of them gets
/// `'` appended until it is none.
const USED: &[&str] = &[
    "conj",
    "or_introl",
    "or_intror",
    "proj1",
    "proj2",
    "I",
    "True",
    "False",
    "False_ind",
    "NNPP",
    "iff",
    "and",
    "or",
    "not",
];

/// How Rocq spells the connectives.
const ROCQ: Symbols = Symbols {
    not: "~ ",
    and: "/\\",
    or: "\\/",
    implies: "->",
    iff: "<->",
    truth: "True",
    falsity: "False",
    forall: "forall ",
    exists: "exists ",
    dot: ", ",
};

/// What the walk has yet to do.
enum Task {
    /// Write the term of an inference.
    Enter(InfId),
    /// Write text.
    Text(String),
    /// Give formulas names for the premise that follows.
    Bind(Vec<((NodeId, Side), String)>),
    /// Take back the names these formulas were last given.
    Unbind(Vec<(NodeId, Side)>),
}

/// The writer of one certificate.
struct Certificate<'a> {
    /// The derivation.
    derivation: &'a Derivation,
    /// The Rocq identifier of every atom.
    atoms: Vec<String>,
    /// The variables in scope, per formula and side, the innermost last.
    names: HashMap<(NodeId, Side), Vec<String>>,
    /// How many variables were made.
    fresh: u64,
}

impl Certificate<'_> {
    /// Returns a formula in Rocq's syntax, in brackets if binary or a
    /// negation, so that it stands as an argument.
    fn formula(&self, id: NodeId) -> String {
        let mut out = String::new();
        let formulas: &Formulas = self.derivation.formulas();
        // A negation is an application, which an argument needs in
        // brackets as much as a binary formula.
        let negation = matches!(formulas.node(id), super::Node::Not(_));
        if negation {
            out.push('(');
        }
        formulas
            .write(
                &mut out,
                id,
                true,
                &ROCQ,
                |o, a| o.write_str(&self.atoms[a as usize]),
                |o, name| o.write_str(name),
            )
            .unwrap();
        if negation {
            out.push(')');
        }
        out
    }

    /// Returns a new variable's name, `h'n` or `k'n` with a new number
    /// `n`, which no atom's identifier is.
    fn name(&mut self, letter: char) -> String {
        self.fresh += 1;
        format!("{letter}'{}", self.fresh)
    }

    /// Returns the variable of a formula on a side.
    fn of(&self, node: NodeId, side: Side) -> String {
        self.names
            .get(&(node, side))
            .and_then(|v| v.last())
            .cloned()
            .expect("a checked derivation names only formulas in scope")
    }

    /// Returns the type of an LJ sequent's term: its goal, or `False`.
    fn goal(&self, id: InfId) -> String {
        match self.derivation.inference(id).right.first() {
            Some(&c) => self.formula(c),
            None => "False".to_owned(),
        }
    }

    /// Returns the tasks that write the term of an inference: text and
    /// premises, each premise between the names it binds and their end.
    fn tasks(&mut self, id: InfId) -> Vec<Task> {
        use Rule::*;
        use Side::{Left, Right};
        let d = self.derivation;
        let inference = d.inference(id);
        let classical = d.logic() == Logic::Classical;
        let f = d.formulas();
        let principal = inference
            .principal
            .map(|(side, at)| inference.side(side)[at]);
        let p = principal.unwrap_or(NodeId(0));
        let operands = f.node(p).operands();
        let (a, b) = (operands.0.unwrap_or(p), operands.1.unwrap_or(p));
        let premises = &inference.premises;
        let text = |s: String| Task::Text(s);
        // A premise with the formulas given bound to new names.
        let premise = |n: usize, binds: Vec<((NodeId, Side), String)>| {
            let keys = binds.iter().map(|(k, _)| *k).collect();
            [
                Task::Bind(binds),
                Task::Enter(premises[n]),
                Task::Unbind(keys),
            ]
        };
        let fa = self.formula(a);
        let fb = self.formula(b);
        let mut tasks = Vec::new();
        // The term of a premise whose goal moves to the right, classically:
        // `NNPP A (fun k : ~ A => premise)`.
        let excluded = |this: &mut Self, n: usize, node: NodeId, tasks: &mut Vec<Task>| {
            let k = this.name('k');
            let shown = this.formula(node);
            tasks.push(text(format!("(NNPP {shown} (fun {k} : ~ {shown} => ")));
            tasks.extend(premise(n, vec![((node, Right), k)]));
            tasks.push(text("))".to_owned()));
        };
        match inference.rule {
            // `write` refuses a derivation with an open goal.
            Open => unreachable!("an open goal has no term"),
            ForallLeft | ForallRight | ExistsLeft | ExistsRight => {
                unreachable!("{}", crate::ordinary::REFUSED)
            }
            Axiom => {
                let (x, y) = (inference.left[0], inference.right[0]);
                let h = self.of(x, Left);
                tasks.push(text(if classical {
                    format!("({} {h})", self.of(y, Right))
                } else {
                    h
                }));
            }
            WeakenLeft | ContractLeft | ContractRight | TrueLeft | FalseRight => {
                tasks.extend(premise(0, vec![]));
            }
            WeakenRight if classical => tasks.extend(premise(0, vec![])),
            WeakenRight => {
                tasks.push(text(format!("(False_ind {} ", self.goal(id))));
                tasks.extend(premise(0, vec![]));
                tasks.push(text(")".to_owned()));
            }
            TrueRight if classical => tasks.push(text(format!("({} I)", self.of(p, Right)))),
            TrueRight => tasks.push(text("I".to_owned())),
            FalseLeft if classical => tasks.push(text(self.of(p, Left))),
            FalseLeft => tasks.push(text(format!(
                "(False_ind {} {})",
                self.goal(id),
                self.of(p, Left)
            ))),
            AndLeft | AndLeft1 | AndLeft2 | IffLeft | IffLeft1 | IffLeft2 => {
                let h = self.of(p, Left);
                let iff = matches!(inference.rule, IffLeft | IffLeft1 | IffLeft2);
                let (x, y) = if iff { implications(f, a, b) } else { (a, b) };
                let pair = if iff {
                    format!("({h} : {} /\\ {})", self.formula(x), self.formula(y))
                } else {
                    h
                };
                let mut binds = Vec::new();
                let mut lets = String::from("(");
                for (keep, node, projection) in [
                    (!matches!(inference.rule, AndLeft2 | IffLeft2), x, "proj1"),
                    (!matches!(inference.rule, AndLeft1 | IffLeft1), y, "proj2"),
                ] {
                    if keep {
                        let n = self.name('h');
                        write!(
                            lets,
                            "let {n} : {} := {projection} {pair} in ",
                            self.formula(node)
                        )
                        .unwrap();
                        binds.push(((node, Left), n));
                    }
                }
                tasks.push(text(lets));
                tasks.extend(premise(0, binds));
                tasks.push(text(")".to_owned()));
            }
            AndRight | IffRight => {
                let iff = inference.rule == IffRight;
                let (x, y) = if iff { implications(f, a, b) } else { (a, b) };
                let cast = format!(" : {} /\\ {}", self.formula(x), self.formula(y));
                if classical {
                    tasks.push(text(format!("({} (conj ", self.of(p, Right))));
                    excluded(self, 0, x, &mut tasks);
                    tasks.push(text(" ".to_owned()));
                    excluded(self, 1, y, &mut tasks);
                    tasks.push(text(format!("{cast}))")));
                } else {
                    tasks.push(text("(conj ".to_owned()));
                    tasks.extend(premise(0, vec![]));
                    tasks.push(text(" ".to_owned()));
                    tasks.extend(premise(1, vec![]));
                    tasks.push(text(format!("{cast})")));
                }
            }
            OrLeft => {
                let h = self.of(p, Left);
                let (h1, h2) = (self.name('h'), self.name('h'));
                let goal = if classical {
                    "False".to_owned()
                } else {
                    self.goal(id)
                };
                tasks.push(text(format!(
                    "(match {h} return {goal} with or_introl {h1} => "
                )));
                tasks.extend(premise(0, vec![((a, Left), h1)]));
                tasks.push(text(format!(" | or_intror {h2} => ")));
                tasks.extend(premise(1, vec![((b, Left), h2)]));
                tasks.push(text(" end)".to_owned()));
            }
            OrRight => {
                let k = self.of(p, Right);
                let (k1, k2) = (self.name('k'), self.name('k'));
                tasks.push(text(format!(
                    "(let {k1} : ~ {fa} := fun x' => {k} (or_introl x') in \
                     let {k2} : ~ {fb} := fun x' => {k} (or_intror x') in "
                )));
                tasks.extend(premise(0, vec![((a, Right), k1), ((b, Right), k2)]));
                tasks.push(text(")".to_owned()));
            }
            OrRight1 | OrRight2 => {
                let (constructor, x) = if inference.rule == OrRight1 {
                    ("or_introl", a)
                } else {
                    ("or_intror", b)
                };
                if classical {
                    tasks.push(text(format!("({} ({constructor} ", self.of(p, Right))));
                    excluded(self, 0, x, &mut tasks);
                    tasks.push(text("))".to_owned()));
                } else {
                    tasks.push(text(format!("({constructor} ")));
                    tasks.extend(premise(0, vec![]));
                    tasks.push(text(")".to_owned()));
                }
            }
            ImpliesRight | NotRight => {
                let h = self.name('h');
                if classical {
                    tasks.push(text(format!("({} (fun {h} : {fa} => ", self.of(p, Right))));
                } else {
                    tasks.push(text(format!("(fun {h} : {fa} => ")));
                }
                if classical && inference.rule == ImpliesRight {
                    let k = self.name('k');
                    tasks.push(text(format!("NNPP {fb} (fun {k} : ~ {fb} => ")));
                    tasks.extend(premise(0, vec![((a, Left), h), ((b, Right), k)]));
                    tasks.push(text(")".to_owned()));
                } else {
                    tasks.extend(premise(0, vec![((a, Left), h)]));
                }
                tasks.push(text(if classical { "))" } else { ")" }.to_owned()));
            }
            // LK's `¬L` in LJ: `Γ, ¬A ⊢` from `Γ ⊢ A`, so the premise is
            // a term of `A` and `¬A` applied to it a term of `False`.
            NotLeft if premises.len() == 1 && !classical => {
                tasks.push(text(format!("({} (", self.of(p, Left))));
                tasks.extend(premise(0, vec![]));
                tasks.push(text("))".to_owned()));
            }
            NotLeft if premises.len() == 1 => {
                let k = self.name('k');
                tasks.push(text(format!(
                    "(let {k} : ~ {fa} := {} in ",
                    self.of(p, Left)
                )));
                tasks.extend(premise(0, vec![((a, Right), k)]));
                tasks.push(text(")".to_owned()));
            }
            ImpliesLeft | NotLeft => {
                let h = self.of(p, Left);
                let (consequent, shown) = if inference.rule == NotLeft {
                    let falsity = f.find(super::Node::False);
                    (falsity.expect("a checked ¬L has ⊥"), "False".to_owned())
                } else {
                    (b, fb.clone())
                };
                let h2 = self.name('h');
                tasks.push(text(format!("(let {h2} : {shown} := {h} ")));
                if classical {
                    excluded(self, 0, a, &mut tasks);
                } else {
                    tasks.push(text("(".to_owned()));
                    tasks.extend(premise(0, vec![]));
                    tasks.push(text(")".to_owned()));
                }
                tasks.push(text(" in ".to_owned()));
                tasks.extend(premise(1, vec![((consequent, Left), h2)]));
                tasks.push(text(")".to_owned()));
            }
            // The checker takes a cut in LJ only.
            Cut => {
                let cut = d.inference(premises[0]).right[0];
                let h = self.name('h');
                tasks.push(text(format!("(let {h} : {} := (", self.formula(cut))));
                tasks.extend(premise(0, vec![]));
                tasks.push(text(") in ".to_owned()));
                tasks.extend(premise(1, vec![((cut, Left), h)]));
                tasks.push(text(")".to_owned()));
            }
        }
        tasks
    }
}

/// Returns the implications `A → B` and `B → A` an equivalence of `a` and
/// `b` stands for, which a checked derivation's arena holds.
fn implications(formulas: &Formulas, a: NodeId, b: NodeId) -> (NodeId, NodeId) {
    let id = |node| {
        formulas
            .find(node)
            .expect("a checked derivation holds the implications of its equivalences")
    };
    (
        id(super::Node::Implies(a, b)),
        id(super::Node::Implies(b, a)),
    )
}

/// Writes the certificate of a derivation: the lemma `options.lemma`
/// stating the sequent over `Prop`, with a variable for every atom, and
/// its proof term. The derivation is checked first
/// ([`Derivation::check`]), and one with an open goal is refused before
/// that. A standalone certificate starts with `options.prelude`; by
/// default that is the import of `Classical_Prop` for a classical
/// certificate and nothing for one of LJ. Asks `stop` during the check
/// and after each inference.
pub(crate) fn write(
    derivation: &Derivation,
    options: &Options,
    out: &mut impl Write,
    mut stop: impl FnMut() -> bool,
) -> Result<(), Error> {
    derivation.propositional("a certificate over Prop")?;
    // A proof in progress, or a derivation read from a document, may break
    // the rules of its logic, and the terms are sound only for one that
    // keeps them.
    if derivation
        .inferences()
        .iter()
        .any(|i| i.rule() == Rule::Open)
    {
        return Err(Unsupported::Open.into());
    }
    let unbounded = crate::Limits {
        work: None,
        ..crate::Limits::default()
    };
    derivation.check(&unbounded, |_| stop())?;
    let classical = derivation.logic() == Logic::Classical;
    let formulas = derivation.formulas();
    let mut atoms = identifiers(formulas.atom_names(), options.lemma.as_str());
    for atom in &mut atoms {
        while USED.contains(&atom.as_str()) {
            atom.push('\'');
        }
    }
    let mut certificate = Certificate {
        derivation,
        atoms,
        names: HashMap::default(),
        fresh: 0,
    };
    if options.form == Form::Standalone {
        let prelude = match &options.prelude {
            Some(prelude) => Some(prelude.as_str()),
            None if classical => Some(CLASSICAL_PRELUDE),
            None => None,
        };
        if let Some(prelude) = prelude {
            write!(out, "{}\n\n", prelude.trim_end())?;
        }
    }
    let root = derivation.inference(derivation.root());
    let mut statement = String::new();
    let mut binders = String::new();
    if !certificate.atoms.is_empty() {
        let all = certificate.atoms.join(" ");
        write!(statement, "forall {all} : Prop, ").unwrap();
        write!(binders, "({all} : Prop) ").unwrap();
    }
    for &h in &root.left {
        let name = certificate.name('h');
        let shown = certificate.formula(h);
        write!(statement, "{shown} -> ").unwrap();
        write!(binders, "({name} : {shown}) ").unwrap();
        certificate
            .names
            .entry((h, Side::Left))
            .or_default()
            .push(name);
    }
    let mut body = String::new();
    let mut closing = String::new();
    let right: Vec<String> = root.right.iter().map(|&d| certificate.formula(d)).collect();
    match right.len() {
        0 => statement.push_str("False"),
        _ if !classical => statement.push_str(&right[0]),
        n => {
            let disjunction = right.join(" \\/ ");
            statement.push_str(&disjunction);
            if n == 1 {
                let k = certificate.name('k');
                write!(body, "NNPP {} (fun {k} : ~ {} => ", right[0], right[0]).unwrap();
                certificate
                    .names
                    .entry((root.right[0], Side::Right))
                    .or_default()
                    .push(k);
            } else {
                let k = certificate.name('k');
                write!(
                    body,
                    "NNPP ({disjunction}) (fun {k} : ~ ({disjunction}) => "
                )
                .unwrap();
                for (i, &d) in root.right.iter().enumerate() {
                    let ki = certificate.name('k');
                    let mut injection = "x'".to_owned();
                    if i + 1 < n {
                        injection = format!("(or_introl {injection})");
                    }
                    for _ in 0..i {
                        injection = format!("(or_intror {injection})");
                    }
                    write!(
                        body,
                        "let {ki} : ~ {} := fun x' => {k} {injection} in ",
                        right[i]
                    )
                    .unwrap();
                    certificate
                        .names
                        .entry((d, Side::Right))
                        .or_default()
                        .push(ki);
                }
            }
            closing.push(')');
        }
    }
    // A statement without atoms or hypotheses binds nothing, and `fun =>`
    // is no term.
    let lambda = if binders.is_empty() {
        String::new()
    } else {
        format!("fun {binders}=> ")
    };
    write!(
        out,
        "Lemma {} : {statement}.\nProof.\n  exact ({lambda}{body}",
        options.lemma
    )?;
    let mut stack = vec![Task::Enter(derivation.root())];
    while let Some(task) = stack.pop() {
        match task {
            Task::Enter(id) => {
                let tasks = certificate.tasks(id);
                stack.extend(tasks.into_iter().rev());
                if stop() {
                    return Err(stopped());
                }
            }
            Task::Text(text) => out.write_str(&text)?,
            Task::Bind(binds) => {
                for (key, name) in binds {
                    certificate.names.entry(key).or_default().push(name);
                }
            }
            Task::Unbind(keys) => {
                for key in keys {
                    if let Some(names) = certificate.names.get_mut(&key) {
                        names.pop();
                    }
                }
            }
        }
    }
    write!(out, "{closing}).\nQed.\n")?;
    Ok(())
}

/// Writes the certificate that an ordinary sequent is not valid, from the
/// classical refutation of its image, as [`falsified`] does with the
/// image's assignment (an atom the image lacks false).
pub(crate) fn not_valid(
    image: &Image,
    disproof: &Disproof,
    options: &Options,
    out: &mut impl Write,
) -> Result<(), Error> {
    let Refutation::Classical(Classical { assignment }) = disproof.refutation() else {
        return Err(Unsupported::NotValid.into());
    };
    disproof.check()?;
    let linear = image.sequent();
    let value = |name: &str| linear.atom(name).is_some_and(|a| assignment[a.index()]);
    falsified(image.ordinary(), value, options, out)
}

/// Writes the certificate that an ordinary sequent is false under the
/// assignment `value` gives its atoms by name: the lemma `options.lemma`
/// stating `~ (forall a b : Prop, Γ -> Δ)`, with `Γ` and `Δ` written as a
/// certificate of validity writes them. The proof instantiates every atom
/// with `True` or `False`, as `value` gives it, and `tauto` decides the
/// closed formula left. The assignment is checked first: it must make
/// every hypothesis true and every formula right of `⊢` false. A
/// standalone certificate starts with `options.prelude` only where one is
/// given, since it needs no library.
fn falsified(
    sequent: &Sequent,
    value: impl Fn(&str) -> bool,
    options: &Options,
    out: &mut impl Write,
) -> Result<(), Error> {
    let formulas = sequent.formulas();
    // Every subformula precedes the formulas built from it.
    let mut truth: Vec<bool> = Vec::with_capacity(formulas.len());
    for id in 0..formulas.len() {
        let of = |k: NodeId| truth[k.index()];
        let node = formulas.node(NodeId(id as u32));
        let t = match node {
            Node::Atom(a) => value(&formulas.atom_names()[a as usize]),
            Node::True => true,
            Node::False => false,
            Node::Not(a) => !of(a),
            Node::And(a, b) => of(a) && of(b),
            Node::Or(a, b) => of(a) || of(b),
            Node::Implies(a, b) => !of(a) || of(b),
            Node::Iff(a, b) => of(a) == of(b),
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        };
        truth.push(t);
    }
    let holds = |id: NodeId| truth[id.index()];
    if !sequent.left().iter().all(|&h| holds(h)) || sequent.right().iter().any(|&d| holds(d)) {
        return Err(Unsupported::NotValid.into());
    }
    let mut atoms = identifiers(formulas.atom_names(), options.lemma.as_str());
    for atom in &mut atoms {
        while USED.contains(&atom.as_str()) {
            atom.push('\'');
        }
    }
    if options.form == Form::Standalone
        && let Some(prelude) = &options.prelude
    {
        write!(out, "{}\n\n", prelude.trim_end())?;
    }
    let shown = |id: NodeId| {
        let mut text = String::new();
        let negation = matches!(formulas.node(id), Node::Not(_));
        if negation {
            text.push('(');
        }
        formulas
            .write(
                &mut text,
                id,
                true,
                &ROCQ,
                |o, a| o.write_str(&atoms[a as usize]),
                |o, name| o.write_str(name),
            )
            .expect("a string takes the text");
        if negation {
            text.push(')');
        }
        text
    };
    let mut statement = String::new();
    if !atoms.is_empty() {
        write!(statement, "forall {} : Prop, ", atoms.join(" ")).expect("a string");
    }
    for &h in sequent.left() {
        write!(statement, "{} -> ", shown(h)).expect("a string");
    }
    let right: Vec<String> = sequent.right().iter().map(|&d| shown(d)).collect();
    if right.is_empty() {
        statement.push_str("False");
    } else {
        statement.push_str(&right.join(" \\/ "));
    }
    let instances: Vec<&str> = (0..atoms.len())
        .map(|a| {
            if value(&formulas.atom_names()[a]) {
                "True"
            } else {
                "False"
            }
        })
        .collect();
    let specialize = if instances.is_empty() {
        String::new()
    } else {
        format!(" specialize (H {}).", instances.join(" "))
    };
    write!(
        out,
        "Lemma {} : ~ ({statement}).\nProof.\n  intro H.{specialize} tauto.\nQed.",
        options.lemma
    )?;
    Ok(())
}

/// Writes the certificate that an ordinary sequent is not valid, from a
/// countermodel, after checking it in `logic` under `limits`: in
/// intuitionistic and minimal logic the lemma `options.lemma` stating
/// `~ lj (atom := string) R [Γ] C` over deduxit's library (`R`
/// `intuitionistic` or `minimal`, `C` `bot` where nothing is right of
/// `⊢`), proved by `kripke_refutation` applied to the model as data and
/// its world, which the library's `refutes` checks by `vm_compute`; in
/// classical logic, where the model is one world, the certificate over
/// `Prop` of that world's assignment ([`falsified`]). The text is refused
/// past `limits.derivation_bytes` before anything is written, and the
/// order's closure, which it lists, within `limits.work` steps; `stop` is
/// asked once per world and every [`Closure::POLL`] steps of the closure.
pub(crate) fn countermodel(
    countermodel: &Countermodel,
    logic: Logic,
    options: &Options,
    limits: &Limits,
    out: &mut impl Write,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<(), Error> {
    countermodel.check(logic, limits, &mut stop)?;
    let (sequent, model) = (&countermodel.sequent, &countermodel.model);
    let rules = match logic {
        Logic::Classical => {
            let world = &model.worlds[model.world as usize];
            let value = |name: &str| world.atoms.iter().any(|a| a == name);
            return falsified(sequent, value, options, out);
        }
        Logic::Intuitionistic => "intuitionistic",
        Logic::Minimal => "minimal",
    };
    let mut closure = Closure::new(
        model.worlds.iter().map(|w| w.arrows.as_slice()).collect(),
        limits,
    );
    // The bytes of the text, counted before any of it is written.
    let formulas = sequent.formulas();
    let mut bytes: Vec<u64> = Vec::with_capacity(formulas.len());
    for id in 0..formulas.len() {
        let of = |k: NodeId| bytes[k.index()];
        let b = match formulas.node(NodeId(id as u32)) {
            Node::Atom(a) => (formulas.atom_names()[a as usize].len() as u64)
                .saturating_mul(2)
                .saturating_add(9),
            Node::True | Node::False => 5,
            Node::Not(a) => of(a).saturating_add(7),
            Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                of(a).saturating_add(of(b)).saturating_add(10)
            }
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        };
        bytes.push(b);
    }
    let mut estimate = sequent
        .left()
        .iter()
        .chain(sequent.right())
        .fold(400u64, |sum, id| {
            sum.saturating_add(bytes[id.index()].saturating_add(2))
        });
    let digits = model.worlds.len().to_string().len() as u64 + 2;
    for (w, world) in model.worlds.iter().enumerate() {
        if stop(Progress::new(Phase::Write, 1, w as u64 + 1)) {
            return Err(stopped());
        }
        let above = closure.above(w, &mut stop)?.len() as u64;
        let atoms = world
            .atoms
            .iter()
            .fold(4u64, |sum, a| sum.saturating_add(a.len() as u64 * 2 + 4));
        estimate = estimate
            .saturating_add(above.saturating_mul(digits))
            .saturating_add(atoms)
            .saturating_add(digits + 4);
        if let Some(limit) = limits.derivation_bytes
            && estimate > limit
        {
            return Err(Refusal::Output {
                what: "certificate",
                estimate_bytes: estimate,
                limit_bytes: limit,
                least_bytes: None,
            }
            .into());
        }
    }
    // The estimate's searches took every step the writing takes again,
    // within the bound: the text is not cut short by it.
    closure.left = closure.limit;
    if options.form == Form::Standalone {
        let prelude = options.prelude.as_deref().unwrap_or(DEDUXIT_KRIPKE_PRELUDE);
        write!(out, "{}\n\n", prelude.trim_end())?;
    }
    let mut text = String::new();
    let hypotheses: Vec<String> = sequent
        .left()
        .iter()
        .map(|&h| {
            text.clear();
            formula(sequent, &mut text, h, false);
            text.clone()
        })
        .collect();
    text.clear();
    match sequent.right().first() {
        Some(&c) => formula(sequent, &mut text, c, true),
        None => text.push_str("bot"),
    }
    write!(
        out,
        "Lemma {} : ~ lj (atom := string) {rules} [{}] {text}.\nProof.\n  apply (kripke_refutation \
         String.eqb String.eqb_spec {rules}\n    {{| later := [",
        options.lemma,
        hypotheses.join("; "),
    )?;
    for w in 0..model.worlds.len() {
        if stop(Progress::new(Phase::Write, 1, w as u64 + 1)) {
            return Err(stopped());
        }
        if w > 0 {
            out.write_str("; ")?;
        }
        out.write_char('[')?;
        for (n, v) in closure.above(w, &mut stop)?.iter().enumerate() {
            if n > 0 {
                out.write_str("; ")?;
            }
            write!(out, "{v}")?;
        }
        out.write_char(']')?;
    }
    out.write_str("];\n       atoms := [")?;
    for (w, world) in model.worlds.iter().enumerate() {
        if w > 0 {
            out.write_str("; ")?;
        }
        out.write_char('[')?;
        for (n, a) in world.atoms.iter().enumerate() {
            if n > 0 {
                out.write_str("; ")?;
            }
            out.write_str(&string(a))?;
        }
        out.write_char(']')?;
    }
    out.write_str("];\n       absurd := [")?;
    let absurd: Vec<String> = (0..model.worlds.len())
        .filter(|&w| model.worlds[w].falsity)
        .map(|w| w.to_string())
        .collect();
    write!(
        out,
        "{}] |}} {}).\n  vm_compute. reflexivity.\nQed.",
        absurd.join("; "),
        model.world
    )?;
    Ok(())
}

/// The worlds above each world of a model, by a search along its arrows
/// from that world, within a bound of steps.
pub(super) struct Closure<'a> {
    /// Each world's arrows.
    arrows: Vec<&'a [u32]>,
    /// The search that last reached each world.
    seen: Vec<u32>,
    /// The current search's mark, never 0.
    mark: u32,
    /// The worlds to search from.
    stack: Vec<u32>,
    /// The worlds above the last world asked, in order.
    above: Vec<u32>,
    /// The steps left, or `None` without a bound.
    pub(super) left: Option<u64>,
    /// The bound of steps.
    pub(super) limit: Option<u64>,
    /// The steps taken.
    steps: u64,
}

impl<'a> Closure<'a> {
    /// The steps between two polls of the stop.
    const POLL: u64 = 1024;

    /// Returns the search over the worlds with these arrows, bounded by
    /// `limits.work`.
    pub(super) fn new(arrows: Vec<&'a [u32]>, limits: &Limits) -> Self {
        Self {
            seen: vec![0; arrows.len()],
            arrows,
            mark: 0,
            stack: Vec::new(),
            above: Vec::new(),
            left: limits.work,
            limit: limits.work,
            steps: 0,
        }
    }

    /// Returns the worlds strictly above `w`, in increasing order; the
    /// model's arrows were checked to go round nowhere. `stop` is asked
    /// every [`Self::POLL`] steps, since one world's search may be long.
    pub(super) fn above(
        &mut self,
        w: usize,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<&[u32], Error> {
        // Each search has its own mark, so a world reached by an earlier
        // search counts as unreached.
        if self.mark == u32::MAX {
            self.seen.fill(0);
            self.mark = 0;
        }
        self.mark += 1;
        let mark = self.mark;
        self.above.clear();
        self.stack.clear();
        self.stack.push(w as u32);
        while let Some(u) = self.stack.pop() {
            for &v in self.arrows[u as usize] {
                self.steps += 1;
                if self.steps.is_multiple_of(Self::POLL)
                    && stop(Progress::new(Phase::Write, Self::POLL, self.steps))
                {
                    return Err(stopped());
                }
                if let Some(left) = &mut self.left {
                    *left = left.checked_sub(1).ok_or(Refusal::Work {
                        limit: self.limit.unwrap_or(0),
                    })?;
                }
                if self.seen[v as usize] != mark {
                    self.seen[v as usize] = mark;
                    self.above.push(v);
                    self.stack.push(v);
                }
            }
        }
        self.above.sort_unstable();
        Ok(&self.above)
    }
}

/// Writes the formula `id` in the constructors of the library's
/// `Deduxit.Intuitionistic`, in brackets if `argument` is set and it is
/// an application.
fn formula(sequent: &Sequent, out: &mut String, id: NodeId, argument: bool) {
    let formulas = sequent.formulas();
    for visit in Stops::new(id, argument, |k| formulas.node(k).operands()) {
        match visit {
            Stop::Enter(k, argument) => {
                let node = formulas.node(k);
                if argument && !matches!(node, Node::True | Node::False) {
                    out.push('(');
                }
                assert!(
                    !matches!(node, Node::Forall(_) | Node::Exists(_)),
                    "{}",
                    crate::ordinary::REFUSED
                );
                out.push_str(library::ordinary(node));
                match node {
                    // A leaf has no exit, so its bracket closes here.
                    Node::Atom(a) => {
                        out.push(' ');
                        out.push_str(&string(&formulas.atom_names()[a as usize]));
                        if argument {
                            out.push(')');
                        }
                    }
                    Node::True | Node::False => {}
                    Node::Not(_)
                    | Node::And(..)
                    | Node::Or(..)
                    | Node::Implies(..)
                    | Node::Iff(..)
                    | Node::Forall(_)
                    | Node::Exists(_) => out.push(' '),
                }
            }
            Stop::Between(_) => out.push(' '),
            Stop::Exit(_, argument) => {
                if argument {
                    out.push(')');
                }
            }
        }
    }
}


