// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The certificates of first-order countermodels for deduxit's library,
//! its first-order development of `Deduxit.Intuitionistic`: a finite
//! structure as data applied to `fo_classical_refutation`, which states
//! `~ lk true [Γ] [Δ]`, and a Kripke model with growing domains as data
//! applied to `fo_kripke_refutation`, which states `~ lj R [Γ] C`, each
//! of the calculus with cut; in each the library's checker accepts the
//! data by `vm_compute`.
//!
//! The model is written as the library reads it: per world its domain's
//! size, a table `("f", arity, values)` per function symbol and constant
//! the sequent applies (a constant at every world), its true facts
//! `("p", [elements])` (with `("=", [e; e])` for each element where the
//! sequent has `=`, which the checks read as identity and the library
//! as a predicate), the worlds above it (the closure of the arrows)
//! and the worlds that force `⊥`. Only the tables of the sequent's
//! symbols are written: the library compares every table it is given
//! along the order, and the check compared only those.

use super::rocq::Closure;
use super::{
    FiniteCountermodel, KripkeCountermodel, KripkeStructure, Logic, Node, NodeId, Sequent,
};
use crate::Error;
use crate::export::Form;
use crate::export::rocq::library::{self, stopped, string};
use crate::export::rocq::{
    DEDUXIT_FIRST_ORDER_MODEL_PRELUDE, MOST_DOMAIN, MOST_ENTRIES, Options, Unsupported,
};
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress};
use crate::linear::sequents::{Stop, Stops};
use std::fmt::Write;

/// The most the numbers of a certificate's model add up to, each counted
/// as its value plus one: Rocq's numbers are unary, so reading and
/// comparing a number costs its value (a table of 20 000 entries of
/// 19 999 took 7.4 GB to compile, one of 8 192 entries of 500 3.2 GB).
const MOST_WEIGHT: u64 = 1 << 20;

/// Text that refuses to grow past `limits.derivation_bytes`, so that a
/// certificate is never held or written past the bound, and that counts
/// the numbers of the model it writes.
struct Bounded {
    /// The text so far, bounded.
    text: library::Bounded,
    /// What the model's numbers add up to so far.
    weight: u64,
}

impl Bounded {
    /// Returns empty text under `limits.derivation_bytes`.
    const fn new(limits: &Limits) -> Self {
        Self {
            text: library::Bounded::new(limits),
            weight: 0,
        }
    }

    /// Appends a number of the model, or refuses where the model's numbers
    /// would add up past [`MOST_WEIGHT`].
    fn number(&mut self, n: u32) -> Result<(), Error> {
        self.weight = self.weight.saturating_add(u64::from(n) + 1);
        if self.weight > MOST_WEIGHT {
            return Err(Unsupported::LargeModel.into());
        }
        self.push(&n.to_string())
    }

    /// Refuses a list of the model of more than [`MOST_ENTRIES`] entries.
    fn entries(count: usize) -> Result<(), Error> {
        if count > MOST_ENTRIES {
            return Err(Unsupported::LargeModel.into());
        }
        Ok(())
    }

    /// Appends `piece`, or refuses where the text would pass the bound.
    fn push(&mut self, piece: &str) -> Result<(), Error> {
        self.text.push(piece)
    }

    /// Appends `pieces` as one piece, the bound compared after the last.
    fn push_all(&mut self, pieces: &[&str]) -> Result<(), Error> {
        self.text.push_all(pieces)
    }
}

/// Writes the certificate that a finite structure refutes its sequent in
/// `logic`, after checking it: in classical logic `~ lk` by the
/// structure, in intuitionistic and minimal logic `~ lj` by the
/// structure as the Kripke model of one world.
pub(crate) fn structure(
    countermodel: &FiniteCountermodel,
    logic: Logic,
    options: &Options,
    limits: &Limits,
    out: &mut impl Write,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<(), Error> {
    countermodel.check(logic, limits, &mut stop)?;
    let model = KripkeStructure::from(countermodel.structure.clone());
    written(
        &countermodel.sequent,
        &model,
        logic,
        options,
        limits,
        out,
        &mut stop,
    )
}

/// Writes the certificate that a Kripke model with growing domains
/// refutes its sequent in `logic`, after checking it: `~ lj` in
/// intuitionistic and minimal logic, and in classical logic, where the
/// model is one world, `~ lk` by that world as a finite structure.
pub(crate) fn kripke_structure(
    countermodel: &KripkeCountermodel,
    logic: Logic,
    options: &Options,
    limits: &Limits,
    out: &mut impl Write,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<(), Error> {
    countermodel.check(logic, limits, &mut stop)?;
    written(
        &countermodel.sequent,
        &countermodel.model,
        logic,
        options,
        limits,
        out,
        &mut stop,
    )
}

/// Writes the certificate of a checked model, built in a bounded text
/// before anything reaches `out`.
fn written(
    sequent: &Sequent,
    model: &KripkeStructure,
    logic: Logic,
    options: &Options,
    limits: &Limits,
    out: &mut impl Write,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<(), Error> {
    // Rocq's numbers are unary and its reader of a list recursive: a
    // domain of billions of elements, which the check accepts where no
    // quantifier ranges over it, or a table of tens of thousands of
    // values would not compile.
    if model.worlds.iter().any(|w| w.size > MOST_DOMAIN) {
        return Err(Unsupported::LargeModel.into());
    }
    Bounded::entries(model.worlds.len())?;
    let mut text = Bounded::new(limits);
    if options.form == Form::Standalone {
        let prelude = options
            .prelude
            .as_deref()
            .unwrap_or(DEDUXIT_FIRST_ORDER_MODEL_PRELUDE);
        text.push(prelude.trim_end())?;
        text.push("\n\n")?;
    }
    let list = |ids: &[NodeId], text: &mut Bounded| -> Result<(), Error> {
        text.push("[")?;
        for (k, &id) in ids.iter().enumerate() {
            if k > 0 {
                text.push("; ")?;
            }
            formula(sequent, id, false, text)?;
        }
        text.push("]")
    };
    let symbols = applied(sequent);
    // `=` is identity at every world, which the library reads from facts.
    let identity = sequent.formulas().has_equality();
    let constants: HashMap<&str, u32> = model
        .constants
        .iter()
        .map(|(name, e)| (name.as_str(), *e))
        .collect();
    write!(text.text.text, "Lemma {} : ~ ", options.lemma).map_err(|_| Error::WriteFailed)?;
    if stop(Progress::new(Phase::Write, 1, 0)) {
        return Err(stopped());
    }
    if logic == Logic::Classical {
        text.push("lk (symbol := string) true ")?;
        list(sequent.left(), &mut text)?;
        text.push(" ")?;
        list(sequent.right(), &mut text)?;
        text.push(".\nProof.\n  apply (fo_classical_refutation String.eqb true\n    {| size := ")?;
        // The check allows one world alone in classical logic.
        let world = &model.worlds[model.world as usize];
        text.number(world.size)?;
        text.push(";\n       functions := ")?;
        tables(&world.functions, &symbols, &constants, &mut text)?;
        text.push(";\n       predicates := ")?;
        facts(&world.predicates, identity.then_some(world.size), &mut text)?;
        text.push(" |}).\n  vm_compute. reflexivity.\nQed.")?;
        out.write_str(&text.text.text)?;
        return Ok(());
    }
    // The calculus with cut: the theorem holds for every choice of
    // rules, and deduxit's own derivations of LJ and LM may hold a cut.
    let explosion = logic != Logic::Minimal;
    write!(
        text.text.text,
        "lj (symbol := string) {{| Calculus.explosion := {explosion}; Calculus.cut := true |}} "
    )
    .map_err(|_| Error::WriteFailed)?;
    list(sequent.left(), &mut text)?;
    text.push(" ")?;
    match sequent.right().first() {
        Some(&c) => formula(sequent, c, true, &mut text)?,
        None => text.push(library::ordinary(Node::False))?,
    }
    write!(
        text.text.text,
        ".\nProof.\n  apply (fo_kripke_refutation String.eqb String.eqb_spec _\n    {{| later := ["
    )
    .map_err(|_| Error::WriteFailed)?;
    let mut closure = Closure::new(
        model.worlds.iter().map(|w| w.arrows.as_slice()).collect(),
        limits,
    );
    let mut stop = |progress| stop(progress);
    for w in 0..model.worlds.len() {
        if stop(Progress::new(Phase::Write, 1, w as u64 + 1)) {
            return Err(stopped());
        }
        if w > 0 {
            text.push("; ")?;
        }
        text.push("[")?;
        for (k, &v) in closure.above(w, &mut stop)?.iter().enumerate() {
            if k > 0 {
                text.push("; ")?;
            }
            text.number(v)?;
        }
        text.push("]")?;
    }
    text.push("];\n       sizes := [")?;
    for (w, world) in model.worlds.iter().enumerate() {
        if w > 0 {
            text.push("; ")?;
        }
        text.number(world.size)?;
    }
    text.push("];\n       tables := [")?;
    for (w, world) in model.worlds.iter().enumerate() {
        if stop(Progress::new(Phase::Write, 1, w as u64 + 1)) {
            return Err(stopped());
        }
        if w > 0 {
            text.push("; ")?;
        }
        tables(&world.functions, &symbols, &constants, &mut text)?;
    }
    text.push("];\n       facts := [")?;
    for (w, world) in model.worlds.iter().enumerate() {
        if stop(Progress::new(Phase::Write, 1, w as u64 + 1)) {
            return Err(stopped());
        }
        if w > 0 {
            text.push("; ")?;
        }
        facts(&world.predicates, identity.then_some(world.size), &mut text)?;
    }
    text.push("];\n       absurd := [")?;
    let mut first = true;
    for (w, world) in model.worlds.iter().enumerate() {
        if world.falsity {
            if !first {
                text.push("; ")?;
            }
            first = false;
            text.number(w as u32)?;
        }
    }
    text.push("] |} ")?;
    text.number(model.world)?;
    text.push(").\n  vm_compute. reflexivity.\nQed.")?;
    out.write_str(&text.text.text)?;
    Ok(())
}

/// Returns each function symbol and constant the sequent's formulas
/// apply, by name, with its arity, in the order of the symbol table: the
/// terms its formulas reach, as the model's check walks them (a JSON
/// sequent's arena may hold terms no formula uses, whose tables the check
/// does not compare).
fn applied(sequent: &Sequent) -> Vec<(&str, u32)> {
    let formulas = sequent.formulas();
    let tables = formulas.first_order();
    let mut seen = vec![false; formulas.len()];
    let mut stack: Vec<NodeId> = sequent
        .left()
        .iter()
        .chain(sequent.right())
        .copied()
        .collect();
    let mut terms: Vec<FoTermId> = Vec::new();
    while let Some(n) = stack.pop() {
        if std::mem::replace(&mut seen[n.index()], true) {
            continue;
        }
        match formulas.node(n) {
            Node::Atom(a) => {
                if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                    terms.extend_from_slice(tables.list(args));
                }
            }
            Node::True | Node::False => {}
            Node::Not(a) | Node::Forall(a) | Node::Exists(a) => stack.push(a),
            Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                stack.push(a);
                stack.push(b);
            }
        }
    }
    let mut used = vec![false; tables.symbols.len()];
    let mut met = vec![false; tables.terms.len()];
    while let Some(t) = terms.pop() {
        if std::mem::replace(&mut met[t.index()], true) {
            continue;
        }
        if let FoTerm::App(f, args) = tables.terms[t.index()] {
            used[f.index()] = true;
            terms.extend_from_slice(tables.list(args));
        }
    }
    tables
        .symbols
        .iter()
        .zip(used)
        .filter(|(_, used)| *used)
        .map(|((name, arity), _)| (name.as_str(), *arity))
        .collect()
}

/// Writes a world's tables of the sequent's function symbols and
/// constants: `[("a", 0, [0]); ("f", 1, [1; 0])]`.
fn tables(
    functions: &[(String, Vec<u32>)],
    symbols: &[(&str, u32)],
    constants: &HashMap<&str, u32>,
    text: &mut Bounded,
) -> Result<(), Error> {
    text.push("[")?;
    // A symbol listed twice has its last table, as in the check.
    let listed: HashMap<&str, &[u32]> = functions
        .iter()
        .map(|(f, values)| (f.as_str(), values.as_slice()))
        .collect();
    Bounded::entries(symbols.len())?;
    let mut first = true;
    for &(name, arity) in symbols {
        let constant;
        let values: &[u32] = if arity == 0 {
            match constants.get(name) {
                Some(&e) => {
                    constant = [e];
                    &constant
                }
                None => continue,
            }
        } else {
            match listed.get(name) {
                Some(values) => values,
                None => continue,
            }
        };
        if !first {
            text.push("; ")?;
        }
        first = false;
        Bounded::entries(values.len())?;
        text.push("(")?;
        text.push(&string(name))?;
        text.push(", ")?;
        text.number(arity)?;
        text.push(", [")?;
        for (k, &v) in values.iter().enumerate() {
            if k > 0 {
                text.push("; ")?;
            }
            text.number(v)?;
        }
        text.push("])")?;
    }
    text.push("]")
}

/// Writes a world's true facts: `[("p", [0]); ("r", [])]`, and where
/// `identity` gives the world's size, `=` of each element and itself.
/// The library reads `=` as any predicate, and the checks read it as
/// identity, which a model never lists: without these facts a structure
/// refuting a sequent with the equality axioms made reflexivity false in
/// Rocq, and the certificate did not compile.
fn facts(
    predicates: &[(String, Vec<Vec<u32>>)],
    identity: Option<u32>,
    text: &mut Bounded,
) -> Result<(), Error> {
    text.push("[")?;
    let mut count = 0usize;
    let mut fact = |name: &str, tuple: &[u32], text: &mut Bounded| -> Result<(), Error> {
        count = count.saturating_add(1);
        Bounded::entries(count)?;
        if count > 1 {
            text.push("; ")?;
        }
        Bounded::entries(tuple.len())?;
        text.push("(")?;
        text.push(&string(name))?;
        text.push(", [")?;
        for (k, &e) in tuple.iter().enumerate() {
            if k > 0 {
                text.push("; ")?;
            }
            text.number(e)?;
        }
        text.push("])")
    };
    for (name, tuples) in predicates {
        for tuple in tuples {
            fact(name, tuple, text)?;
        }
    }
    for e in 0..identity.unwrap_or(0) {
        fact(crate::first_order::EQUALS, &[e, e], text)?;
    }
    text.push("]")
}

/// Writes the formula `id` in the constructors of the library's
/// first-order formulas (`var`, `bot`, `wedge`, `vee`, `imp`, `fall`,
/// `fex`, and `top`, `neg`, `equiv` defined), in brackets if `argument`
/// is set and it is an application.
fn formula(sequent: &Sequent, id: NodeId, argument: bool, text: &mut Bounded) -> Result<(), Error> {
    let formulas = sequent.formulas();
    let tables = formulas.first_order();
    for visit in Stops::new(id, argument, |k| formulas.node(k).operands()) {
        match visit {
            Stop::Enter(k, argument) => {
                let node = formulas.node(k);
                let nullary = matches!(node, Node::True | Node::False);
                if argument && !nullary {
                    text.push("(")?;
                }
                // The name and its space in one piece, so that a refusal
                // counts them together.
                text.push_all(&[library::ordinary(node), if nullary { "" } else { " " }])?;
                // A leaf has no exit, so its bracket closes here.
                if let Node::Atom(a) = node {
                    match tables.atoms.get(a as usize) {
                        Some(&(symbol, args)) => {
                            text.push(&string(&tables.symbols[symbol.index()].0))?;
                            text.push(" ")?;
                            terms(sequent, tables.list(args), text)?;
                        }
                        None => {
                            text.push(&string(formulas.atom_name(a)))?;
                            text.push(" []")?;
                        }
                    }
                    if argument {
                        text.push(")")?;
                    }
                }
            }
            Stop::Between(_) => text.push(" ")?,
            Stop::Exit(_, argument) => {
                if argument {
                    text.push(")")?;
                }
            }
        }
    }
    Ok(())
}

/// What is left to write of a list of terms.
#[derive(Clone, Copy)]
enum Piece {
    /// A term.
    Term(FoTermId),
    /// The separator between two terms of a list.
    Separator,
    /// The end of the list.
    Close,
    /// The end of an application: its list of arguments and its bracket.
    CloseApplication,
}

/// Writes a list of terms in the library's terms (`bound`, `eigen`,
/// `tapp`), `[t₁; …]`, on a stack of its own: a term may nest as deep as
/// it is long.
fn terms(sequent: &Sequent, list: &[FoTermId], text: &mut Bounded) -> Result<(), Error> {
    let tables = sequent.formulas().first_order();
    let push_list = |stack: &mut Vec<Piece>, list: &[FoTermId]| {
        for (k, &t) in list.iter().enumerate().rev() {
            stack.push(Piece::Term(t));
            if k > 0 {
                stack.push(Piece::Separator);
            }
        }
    };
    let mut stack: Vec<Piece> = vec![Piece::Close];
    push_list(&mut stack, list);
    text.push("[")?;
    while let Some(piece) = stack.pop() {
        match piece {
            Piece::Separator => text.push("; ")?,
            Piece::Close => text.push("]")?,
            Piece::CloseApplication => text.push("])")?,
            Piece::Term(t) => match tables.terms[t.index()] {
                FoTerm::Bound(n) => {
                    text.push_all(&[library::BOUND, " "])?;
                    text.push(&n.to_string())?;
                }
                FoTerm::Eigen(a) => {
                    text.push_all(&[library::EIGEN, " "])?;
                    text.push(&a.get().to_string())?;
                }
                FoTerm::App(f, args) => {
                    text.push_all(&["(", library::TAPP, " "])?;
                    text.push(&string(&tables.symbols[f.index()].0))?;
                    text.push(" [")?;
                    stack.push(Piece::CloseApplication);
                    push_list(&mut stack, tables.list(args));
                }
                FoTerm::Meta(_) => {
                    return Err(Error::Structure {
                        reason: "it cannot interpret an open witness".to_owned(),
                    });
                }
            },
        }
    }
    Ok(())
}
