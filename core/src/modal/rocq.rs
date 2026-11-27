// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The certificates of modal logic for deduxit's Rocq library, its module
//! `Deduxit.Modal`: a derivation of a labelled calculus as a term that
//! the library's checker accepts, applied to `modal_soundness` or
//! `labelled_soundness`, and a countermodel as a finite model applied to
//! `modal_refutation`. Each states Kripke semantics, never derivability:
//! the rule table is not part of what a reader trusts.

use super::{Condition, Countermodel, Node, NodeId, Sequent, System};
use crate::Error;
use crate::export::Form;
use crate::export::rocq::library::{self, Bounded, stopped, string};
use crate::export::rocq::{
    DEDUXIT_MODAL_COUNTERMODEL_PRELUDE, DEDUXIT_MODAL_PRELUDE, Options, Unsupported,
};
use crate::limits::{Limits, Phase, Progress};
use crate::linear::sequents::{Stop, Stops};
use crate::rules::{Derivation, NodeId as Place, NodeRef, Step, Term, TermId, Terms};
use std::fmt::Write;

/// Returns the record of a system's frame conditions, as the library's
/// `conditions` writes it: every condition the system holds, those that
/// follow from its axioms too, so the record is the system's frame class.
fn conditions(system: System) -> String {
    let of = |condition| system.has(condition);
    format!(
        "{{| serial := {}; reflexive := {}; symmetric := {}; transitive := {}; euclidean := {} |}}",
        of(Condition::Serial),
        of(Condition::Reflexive),
        of(Condition::Symmetric),
        of(Condition::Transitive),
        of(Condition::Euclidean),
    )
}

/// Returns the refusal of a derivation the library's calculus has no
/// term for. The checker refused it first, so this is a defect.
fn shapeless(why: &str) -> Error {
    Error::ReadBack {
        calculus: "G3",
        reason: format!("the derivation has no term of Deduxit.Modal: {why}"),
    }
}

/// Writes the formula `id` of a derivation's terms in the constructors of
/// `Deduxit.Modal.Formulas`, in brackets if `argument` is set and it is
/// an application.
fn formula(terms: &Terms, id: TermId, argument: bool, out: &mut Bounded) -> Result<(), Error> {
    let operands = |k: TermId| match terms.term(k) {
        Term::Apply { arguments, .. } => (arguments.first().copied(), arguments.get(1).copied()),
        Term::Atom { .. } => (None, None),
    };
    for visit in Stops::new(id, argument, operands) {
        match visit {
            Stop::Enter(k, argument) => match terms.term(k) {
                Term::Atom { name, .. } => {
                    let atom = format!("{} {}", library::modal_node(Node::Atom(0)), string(name));
                    // A leaf has no exit, so its bracket closes here.
                    if argument {
                        out.push(&format!("({atom})"))?;
                    } else {
                        out.push(&atom)?;
                    }
                }
                Term::Apply {
                    connective,
                    arguments,
                } => {
                    let name = library::modal(connective).ok_or_else(|| {
                        shapeless(&format!("{connective} is no connective of a formula"))
                    })?;
                    if arguments.is_empty() {
                        out.push(name)?;
                    } else {
                        if argument {
                            out.push("(")?;
                        }
                        out.push(name)?;
                        out.push(" ")?;
                    }
                }
            },
            Stop::Between(_) => out.push(" ")?,
            Stop::Exit(_, argument) => {
                if argument {
                    out.push(")")?;
                }
            }
        }
    }
    Ok(())
}

/// An item of a labelled sequent: a formula at a label, or an edge.
#[derive(Clone, Copy)]
enum Item<'a> {
    /// `x : A`.
    At(&'a str, TermId),
    /// `x R y`.
    Edge(&'a str, &'a str),
}

/// Returns the label a term is.
fn label(terms: &Terms, id: TermId) -> Result<&str, Error> {
    match terms.term(id) {
        Term::Atom { name, .. } => Ok(name),
        Term::Apply { .. } => Err(shapeless("a label that is no atom")),
    }
}

/// Returns the item a member of a sequent is.
fn item(terms: &Terms, id: TermId) -> Result<Item<'_>, Error> {
    match terms.term(id) {
        Term::Apply {
            connective: ":",
            arguments: &[x, a],
        } => Ok(Item::At(label(terms, x)?, a)),
        Term::Apply {
            connective: "R",
            arguments: &[x, y],
        } => Ok(Item::Edge(label(terms, x)?, label(terms, y)?)),
        Term::Atom { .. } | Term::Apply { .. } => {
            Err(shapeless("a member that is neither x : A nor x R y"))
        }
    }
}

/// Writes an item in the constructors of `Deduxit.Modal.Labelled`.
fn write_item(terms: &Terms, item: Item<'_>, out: &mut Bounded) -> Result<(), Error> {
    match item {
        Item::At(x, a) => {
            out.push(&format!("lab {} ", string(x)))?;
            formula(terms, a, true, out)
        }
        Item::Edge(x, y) => out.push(&format!("rel {} {}", string(x), string(y))),
    }
}

/// Where a position of a node of the labelled calculus points: a zone of
/// the node's own sequent, or the left zone of its first premise (a new
/// edge), as the rule compiler numbers them.
#[derive(Clone, Copy)]
enum At {
    /// The conclusion's left zone.
    Left,
    /// The conclusion's right zone.
    Right,
    /// The first premise's left zone.
    Premise,
}

/// Returns where each position of a node of `rule` points, in the
/// order the node gives them (its constructor is
/// [`library::modal_proof`]'s).
fn shape(rule: &str) -> Option<&'static [At]> {
    use At::{Left, Premise, Right};
    Some(match rule {
        "ax" => &[Left, Right],
        "L⊥" => &[Left],
        "R⊤" => &[Right],
        "L¬" => &[Left],
        "R¬" => &[Right],
        "L∧" => &[Left],
        "R∧" => &[Right],
        "L∨" => &[Left],
        "R∨" => &[Right],
        "L→" => &[Left],
        "R→" => &[Right],
        "L↔" => &[Left],
        "R↔" => &[Right],
        "L□" => &[Left, Left],
        "R□" => &[Right, Premise],
        "L◇" => &[Left, Premise],
        "R◇" => &[Left, Right],
        "Ref" => &[Premise],
        "Ser" => &[Premise],
        "Sym" => &[Left],
        "Trans" => &[Left, Left],
        "Eucl" => &[Left, Left],
        _ => return None,
    })
}

/// Writes a node's constructor and its labels and formulas, without its
/// premises: `RBox "w0" "w1" (var "p")`.
fn head(derivation: &Derivation, node: NodeRef<'_>, out: &mut Bounded) -> Result<(), Error> {
    let terms = derivation.terms();
    let Step::Rule(rule) = node.step() else {
        return Err(Unsupported::Open.into());
    };
    let (name, places) = library::modal_proof(rule)
        .zip(shape(rule))
        .ok_or_else(|| shapeless(&format!("the rule {rule}")))?;
    let positions = node.positions();
    if positions.len() != places.len() {
        return Err(shapeless(&format!(
            "{rule} gives {} positions",
            positions.len()
        )));
    }
    let mut items = Vec::with_capacity(places.len());
    for (&place, &position) in places.iter().zip(positions) {
        let zone = match place {
            At::Left => node.zone(0),
            At::Right => node.zone(1),
            At::Premise => {
                let premise = node
                    .premises()
                    .first()
                    .ok_or_else(|| shapeless(&format!("{rule} without a premise")))?;
                derivation.node(*premise).zone(0)
            }
        };
        let member = zone
            .get(position as usize)
            .ok_or_else(|| shapeless(&format!("a position of {rule} past its zone")))?;
        items.push(item(terms, *member)?);
    }
    out.push(name)?;
    // The principal formula's label, the other label of an edge or the
    // new label, then the principal formula's parts.
    let parts = |a: TermId| -> Vec<TermId> {
        match terms.term(a) {
            Term::Apply { arguments, .. } => arguments.to_vec(),
            Term::Atom { .. } => Vec::new(),
        }
    };
    let mut labels: Vec<&str> = Vec::new();
    let mut formulas: Vec<TermId> = Vec::new();
    match (name, items.as_slice()) {
        ("Ax", [Item::At(x, a), _]) => {
            labels.push(*x);
            formulas.push(*a);
        }
        ("LBot" | "RTop", [Item::At(x, _)]) => labels.push(*x),
        ("LBox" | "RBox" | "LDia", [Item::At(x, a), Item::Edge(_, y)])
        | ("RDia", [Item::Edge(_, y), Item::At(x, a)]) => {
            labels.extend([*x, *y]);
            formulas.extend(parts(*a));
        }
        ("Ref", [Item::Edge(x, _)]) => labels.push(*x),
        ("Ser" | "Sym", [Item::Edge(x, y)]) => labels.extend([*x, *y]),
        ("Trans", [Item::Edge(x, y), Item::Edge(_, z)])
        | ("Eucl", [Item::Edge(x, y), Item::Edge(_, z)]) => labels.extend([*x, *y, *z]),
        (_, [Item::At(x, a)]) => {
            labels.push(*x);
            formulas.extend(parts(*a));
        }
        _ => {
            return Err(shapeless(&format!(
                "{rule} acts on members of another kind"
            )));
        }
    }
    for x in labels {
        out.push(" ")?;
        out.push(&string(x))?;
    }
    for a in formulas {
        out.push(" ")?;
        formula(terms, a, true, out)?;
    }
    Ok(())
}

/// A piece of a term still to be written.
enum Piece {
    /// The term of a node.
    Node(Place),
    /// Text.
    Text(&'static str),
}

/// Writes the term of `root`: each node's constructor and its premises'
/// terms, a node that several premises share once for each, as the
/// library's checker walks it (a `let` would share the text and hide the
/// checker's work, exponential in the text for a chain of shared nodes).
/// A loop over a stack, so that a deep derivation needs no stack of the
/// caller's.
fn term(
    derivation: &Derivation,
    root: Place,
    out: &mut Bounded,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<(), Error> {
    let mut written = 0u64;
    let mut pieces = vec![Piece::Node(root)];
    while let Some(piece) = pieces.pop() {
        match piece {
            Piece::Text(text) => out.push(text)?,
            Piece::Node(id) => {
                written += 1;
                if stop(Progress::new(Phase::Write, 1, written)) {
                    return Err(stopped());
                }
                let node = derivation.node(id);
                // Every node's term is an argument, the root's of the
                // theorem the proof applies.
                out.push("(")?;
                pieces.push(Piece::Text(")"));
                head(derivation, node, out)?;
                for &premise in node.premises().iter().rev() {
                    pieces.push(Piece::Node(premise));
                    pieces.push(Piece::Text(" "));
                }
            }
        }
    }
    Ok(())
}

/// Writes the certificate of a derivation of a labelled modal calculus
/// (`G3` and a system's name), after checking it in that system's
/// calculus. A root at one label without relational atoms is stated as
/// `valid` and proved by `modal_soundness`; any other as `lvalid` and
/// `labelled_soundness`.
pub(crate) fn derivation(
    derivation: &Derivation,
    options: &Options,
    limits: &Limits,
    out: &mut impl Write,
    stop: impl FnMut(Progress) -> bool,
) -> Result<(), Error> {
    labelled(derivation, None, options, limits, out, stop)
}

/// Writes the certificate of a modal outcome: a valid one's derivation,
/// stated as the sequent the outcome decided, or a countermodel.
pub(crate) fn outcome(
    outcome: &super::Outcome,
    options: &Options,
    limits: &Limits,
    out: &mut impl Write,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<(), Error> {
    match (&outcome.verdict, &outcome.countermodel) {
        (super::Verdict::Valid(super::Evidence::Derivation(derivation)), _) => {
            // The root is the sequent at w0, which the check asks.
            outcome.check(limits, &mut stop)?;
            labelled(
                derivation,
                Some(&outcome.sequent),
                options,
                limits,
                out,
                stop,
            )
        }
        (super::Verdict::NotValid, Some(model)) => countermodel(model, options, limits, out, stop),
        _ => Err(Error::NotApplicable {
            operation: "certificate".into(),
            reason: "the outcome holds no evidence: a derivation or a countermodel".to_owned(),
        }),
    }
}

/// Writes the certificate of a derivation, as [`derivation`] does; with
/// `sequent`, the sequent its root is at `w0`, stated as written.
fn labelled(
    derivation: &Derivation,
    sequent: Option<&Sequent>,
    options: &Options,
    limits: &Limits,
    out: &mut impl Write,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<(), Error> {
    let system = System::ALL
        .iter()
        .copied()
        .find(|&system| super::calculus::name(system) == derivation.calculus())
        .ok_or(Unsupported::Calculus)?;
    derivation.check(&super::calculus(system, limits)?, limits, &mut stop)?;
    // The checker takes a hypothesis of the outermost scope as an
    // assumption, which the library's statement has no place for.
    if derivation.assumptions().next().is_some() {
        return Err(Unsupported::Assumption.into());
    }
    let root = derivation.root().ok_or(Unsupported::Open)?;
    let terms = derivation.terms();
    let (left, right): (Vec<Item<'_>>, Vec<Item<'_>>) = (
        root.zone(0)
            .iter()
            .map(|&m| item(terms, m))
            .collect::<Result<_, _>>()?,
        root.zone(1)
            .iter()
            .map(|&m| item(terms, m))
            .collect::<Result<_, _>>()?,
    );
    // The label every item of the root is at, if there is one.
    let mut at: Option<&str> = None;
    let one_label = left.iter().chain(&right).all(|&i| match i {
        Item::At(x, _) => *at.get_or_insert(x) == x,
        Item::Edge(..) => false,
    });
    let record = conditions(system);
    let mut text = Bounded::new(limits);
    if options.form == Form::Standalone {
        let prelude = options.prelude.as_deref().unwrap_or(DEDUXIT_MODAL_PRELUDE);
        text.push(prelude.trim_end())?;
        text.push("\n\n")?;
    }
    text.push(&format!("Lemma {} : ", options.lemma))?;
    let list = |items: &[Item<'_>], text: &mut Bounded, formulas: bool| -> Result<(), Error> {
        text.push("[")?;
        for (n, &i) in items.iter().enumerate() {
            if n > 0 {
                text.push("; ")?;
            }
            match (formulas, i) {
                (true, Item::At(_, a)) => formula(terms, a, false, text)?,
                _ => write_item(terms, i, text)?,
            }
        }
        text.push("]")
    };
    if let Some(sequent) = sequent {
        // The sequent decided, at w0, repeated formulas included.
        text.push(&format!("valid (atom := string) {record} "))?;
        for (k, side) in [sequent.left(), sequent.right()].into_iter().enumerate() {
            text.push(if k == 0 { "[" } else { " [" })?;
            for (n, &id) in side.iter().enumerate() {
                if n > 0 {
                    text.push("; ")?;
                }
                modal_formula(sequent, id, false, &mut text)?;
            }
            text.push("]")?;
        }
        text.push(&format!(
            ".\nProof.\n  apply (modal_soundness String.eqb String.eqb_spec {record} \"w0\"\n    "
        ))?;
    } else if one_label {
        text.push(&format!("valid (atom := string) {record} "))?;
        list(&left, &mut text, true)?;
        text.push(" ")?;
        list(&right, &mut text, true)?;
        text.push(&format!(
            ".\nProof.\n  apply (modal_soundness String.eqb String.eqb_spec {record} {}\n    ",
            string(at.unwrap_or("w0"))
        ))?;
    } else {
        text.push(&format!("lvalid (atom := string) {record} "))?;
        list(&left, &mut text, false)?;
        text.push(" ")?;
        list(&right, &mut text, false)?;
        text.push(&format!(
            ".\nProof.\n  apply (labelled_soundness String.eqb String.eqb_spec {record}\n    "
        ))?;
    }
    term(derivation, root.id(), &mut text, &mut stop)?;
    text.push(").\n  vm_compute. reflexivity.\nQed.")?;
    out.write_str(&text.text)?;
    Ok(())
}

/// Writes the modal formula `id` in the constructors of
/// `Deduxit.Modal.Formulas`, in brackets if `argument` is set and it is
/// an application.
fn modal_formula(
    sequent: &Sequent,
    id: NodeId,
    argument: bool,
    out: &mut Bounded,
) -> Result<(), Error> {
    let formulas = sequent.formulas();
    for visit in Stops::new(id, argument, |k| formulas.node(k).operands()) {
        match visit {
            Stop::Enter(k, argument) => {
                let node = formulas.node(k);
                let name = library::modal_node(node);
                match node {
                    Node::Atom(a) => {
                        let atom = format!("{name} {}", string(&formulas.atom_names()[a as usize]));
                        // A leaf has no exit, so its bracket closes here.
                        out.push(&if argument { format!("({atom})") } else { atom })?;
                        continue;
                    }
                    Node::True | Node::False => {
                        out.push(name)?;
                        continue;
                    }
                    Node::Not(_)
                    | Node::And(..)
                    | Node::Or(..)
                    | Node::Implies(..)
                    | Node::Iff(..)
                    | Node::Box(_)
                    | Node::Diamond(_) => {}
                }
                if argument {
                    out.push("(")?;
                }
                out.push(name)?;
                out.push(" ")?;
            }
            Stop::Between(_) => out.push(" ")?,
            Stop::Exit(_, argument) => {
                if argument {
                    out.push(")")?;
                }
            }
        }
    }
    Ok(())
}

/// Writes the certificate that a modal sequent is not valid, after
/// checking its countermodel: the model's worlds in its order, each with
/// the worlds it sees as written and the atoms it forces, applied to
/// `modal_refutation` at the model's world.
pub(crate) fn countermodel(
    countermodel: &Countermodel,
    options: &Options,
    limits: &Limits,
    out: &mut impl Write,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<(), Error> {
    countermodel.check(limits, &mut stop)?;
    let (sequent, model) = (&countermodel.sequent, &countermodel.model);
    let record = conditions(countermodel.system);
    let mut text = Bounded::new(limits);
    if options.form == Form::Standalone {
        let prelude = options
            .prelude
            .as_deref()
            .unwrap_or(DEDUXIT_MODAL_COUNTERMODEL_PRELUDE);
        text.push(prelude.trim_end())?;
        text.push("\n\n")?;
    }
    text.push(&format!(
        "Lemma {} : ~ valid (atom := string) {record} ",
        options.lemma
    ))?;
    for (k, side) in [sequent.left(), sequent.right()].into_iter().enumerate() {
        text.push(if k == 0 { "[" } else { " [" })?;
        for (n, &id) in side.iter().enumerate() {
            if n > 0 {
                text.push("; ")?;
            }
            modal_formula(sequent, id, false, &mut text)?;
        }
        text.push("]")?;
    }
    text.push(".\nProof.\n  apply (modal_refutation String.eqb\n    {| arrows := [")?;
    for (w, world) in model.worlds.iter().enumerate() {
        if stop(Progress::new(Phase::Write, 1, w as u64 + 1)) {
            return Err(stopped());
        }
        if w > 0 {
            text.push("; ")?;
        }
        let arrows: Vec<String> = world.arrows.iter().map(u32::to_string).collect();
        text.push(&format!("[{}]", arrows.join("; ")))?;
    }
    text.push("];\n       atoms := [")?;
    for (w, world) in model.worlds.iter().enumerate() {
        if w > 0 {
            text.push("; ")?;
        }
        let atoms: Vec<String> = world.atoms.iter().map(|a| string(a)).collect();
        text.push(&format!("[{}]", atoms.join("; ")))?;
    }
    text.push(&format!(
        "] |}}\n    {record} {}).\n  vm_compute. reflexivity.\nQed.",
        model.world
    ))?;
    out.write_str(&text.text)?;
    Ok(())
}
