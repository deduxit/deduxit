// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A derivation of LK or LJ written as a derivation in the rule files
//! `rules::LK`, `rules::LJ` and `rules::LM`.

use super::FORMULA;
use super::derivation::{Derivation, Node, Step};
use super::terms::TermId;
use crate::Error;
use crate::ordinary::{self, Inference, Logic, Rule, Side};

/// Returns the name of the rule file of `logic`.
const fn calculus(logic: Logic) -> &'static str {
    match logic {
        Logic::Classical => "LK",
        Logic::Intuitionistic => "LJ",
        Logic::Minimal => "LM",
    }
}

/// Adds every formula of `formulas` to `terms`, in the arena's order,
/// and returns the term of each.
pub(super) fn terms_of(
    formulas: &ordinary::Formulas,
    terms: &mut super::Terms,
) -> Result<Vec<TermId>, Error> {
    let mut ids: Vec<TermId> = Vec::with_capacity(formulas.len());
    for &node in formulas.nodes() {
        use ordinary::Node::*;
        let id = match node {
            Atom(atom) => terms.atom(formulas.atom_name(atom), FORMULA)?,
            True => terms.apply("⊤", &[])?,
            False => terms.apply("⊥", &[])?,
            Not(a) => terms.apply("¬", &[ids[a.index()]])?,
            And(a, b) => terms.apply("∧", &[ids[a.index()], ids[b.index()]])?,
            Or(a, b) => terms.apply("∨", &[ids[a.index()], ids[b.index()]])?,
            Implies(a, b) => terms.apply("→", &[ids[a.index()], ids[b.index()]])?,
            Iff(a, b) => terms.apply("↔", &[ids[a.index()], ids[b.index()]])?,
        };
        ids.push(id);
    }
    Ok(ids)
}

/// Returns how many formulas a sequent holds.
fn size(inference: &Inference) -> usize {
    inference.left().len() + inference.right().len()
}

/// Returns the rule file's name for the rule of `inference`, and the
/// side its principal formula stands on, if it has one. A rule of two
/// premises has two names there, one for each way to take the context,
/// and `¬` in LJ two forms; the premises' sizes tell them apart, and only
/// one of them fits whenever either does.
fn name(logic: Logic, inference: &Inference, premises: &[&Inference]) -> (String, Option<Side>) {
    use Rule::*;
    use Side::{Left, Right};
    let rule = inference.rule();
    // How many formulas each premise holds beyond the context where the
    // context is in both.
    let shared = |first: usize, second: usize| match premises {
        // Where the conclusion is empty neither form fits.
        [p, q] => size(p) + 1 == size(inference) + first && size(q) + 1 == size(inference) + second,
        _ => true,
    };
    let split = |shared: bool| if shared { "" } else { " split" };
    let lj = logic != Logic::Classical;
    let (name, side) = match rule {
        Axiom | Cut | Open => (rule.name().to_owned(), None),
        WeakenLeft | ContractLeft | AndLeft | AndLeft1 | AndLeft2 | IffLeft | IffLeft1
        | IffLeft2 | FalseLeft | TrueLeft => (rule.name().to_owned(), Some(Left)),
        WeakenRight | ContractRight | OrRight | OrRight1 | OrRight2 | ImpliesRight | FalseRight
        | TrueRight => (rule.name().to_owned(), Some(Right)),
        AndRight | IffRight => (format!("{rule}{}", split(shared(1, 1))), Some(Right)),
        OrLeft | ImpliesLeft => (format!("{rule}{}", split(shared(1, 1))), Some(Left)),
        NotLeft if lj && premises.len() != 1 => {
            (format!("¬L ⊥{}", split(shared(1, 1))), Some(Left))
        }
        NotLeft => (rule.name().to_owned(), Some(Left)),
        // `A → ⊥`'s form keeps as many formulas right of `⊢`.
        NotRight
            if lj
                && premises
                    .first()
                    .is_some_and(|p| p.right().len() == inference.right().len()) =>
        {
            ("¬R ⊥".to_owned(), Some(Right))
        }
        NotRight => (rule.name().to_owned(), Some(Right)),
    };
    // A cut takes the context whole only where nothing stands right of
    // `⊢`; with no context the two forms are one.
    if rule == Cut
        && let [p, q] = premises
        && inference.right().is_empty()
        && !inference.left().is_empty()
        && p.left().len() == inference.left().len()
        && q.left().len() == inference.left().len() + 1
    {
        return ("cut shared".to_owned(), None);
    }
    (name, side)
}

impl Derivation {
    /// Returns a derivation of LK or LJ as a derivation in the rule file
    /// of its logic: [`LK`](super::LK) for classical,
    /// [`LJ`](super::LJ) for intuitionistic and [`LM`](super::LM) for
    /// minimal logic. Its terms are the derivation's formulas and its
    /// nodes the inferences, in their order. An open goal stays one.
    ///
    /// Nothing is checked beyond what the form cannot say. The result
    /// passes [`check`](Self::check) against that rule file exactly when
    /// [`ordinary::Derivation::check`] accepts the derivation.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidDerivation`] where the root does not conclude the
    /// derivation's sequent, a principal formula stands on the side its
    /// rule does not act on, a premise does not come before its
    /// conclusion, or the derivation's formulas lack an implication of an
    /// equivalence a rule acts on: this form has no place for any of
    /// them.
    pub fn from_ordinary(derivation: &ordinary::Derivation) -> Result<Self, Error> {
        let logic = derivation.logic();
        let invalid = |reason: String| Error::InvalidDerivation {
            calculus: calculus(logic).into(),
            reason,
        };
        let formulas = derivation.formulas();
        let mut out = Self::new(calculus(logic), 2);
        let ids = terms_of(formulas, out.terms_mut())?;
        let zone = |side: &[ordinary::NodeId]| -> Vec<TermId> {
            side.iter().map(|formula| ids[formula.index()]).collect()
        };
        let sorted = |side: &[ordinary::NodeId]| {
            let mut side = side.to_vec();
            side.sort_unstable();
            side
        };
        let inferences = derivation.inferences();
        let (left, right) = derivation.sequent();
        let concluded = inferences.last().is_some_and(|root| {
            sorted(root.left()) == sorted(left) && sorted(root.right()) == sorted(right)
        });
        if !concluded {
            return Err(invalid("the root does not conclude the sequent".to_owned()));
        }
        for (index, inference) in inferences.iter().enumerate() {
            if inference.premises().iter().any(|p| p.index() >= index) {
                return Err(invalid(format!(
                    "inference {index}: a premise does not precede it"
                )));
            }
            let premises: Vec<&Inference> = inference
                .premises()
                .iter()
                .map(|p| &inferences[p.index()])
                .collect();
            // The ordinary checker asks that the arena hold both
            // implications of an equivalence a rule acts on, whichever
            // the rule writes out. A rule file cannot ask that.
            if let Some((side, at)) = inference.principal()
                && let Some(&formula) = inference.side(side).get(at)
                && let ordinary::Node::Iff(a, b) = formulas.node(formula)
                && matches!(
                    inference.rule(),
                    Rule::IffLeft | Rule::IffLeft1 | Rule::IffLeft2 | Rule::IffRight
                )
                && (formulas.find(ordinary::Node::Implies(a, b)).is_none()
                    || formulas.find(ordinary::Node::Implies(b, a)).is_none())
            {
                return Err(invalid(format!(
                    "inference {index} ({}): the implications of the equivalence are missing",
                    inference.rule()
                )));
            }
            let (name, side) = name(logic, inference, &premises);
            // The position of the principal formula on its rule's side.
            // One on the other side has no place in this form; one an
            // axiom or a cut should not have is passed on, and refused
            // by the check.
            let position = match (inference.principal(), side) {
                (Some((on, _)), Some(side)) if on != side => {
                    return Err(invalid(format!(
                        "inference {index} ({}): the principal formula stands on the side \
                         the rule does not act on",
                        inference.rule()
                    )));
                }
                (Some((_, at)), _) => u32::try_from(at).ok(),
                (None, _) => None,
            };
            let sequent = [zone(inference.left()), zone(inference.right())];
            let premises: Vec<u32> = inference
                .premises()
                .iter()
                .map(|p| p.index() as u32)
                .collect();
            let step = if inference.rule() == Rule::Open {
                Step::Open
            } else {
                Step::Rule(&name)
            };
            out.push(
                Node::new(&sequent, step)
                    .premises(&premises)
                    .positions(position.as_slice()),
            )?;
        }
        Ok(out)
    }
}
