// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The weighted count of a goal's literals, which certifies a one-sided
//! count and a Horn program's state equation apart from the search.
//!
//! Each atom (or predicate) weighs an integer, and every formula has the
//! numbers it can contribute to the weighted count of a derivable
//! sequent's literals: a literal its weight (negated for the negative
//! one), `⊗` and `⅋` their sides' sums, `&` and `⊕` either side's, `!A`
//! nothing, and `?A` the sum of any number of `A`'s contributions, none
//! included. Every derivable sequent, without weakening and without cut,
//! Mix allowed, can contribute zero or has a formula that absorbs (a
//! `⊤`); so a goal none of whose formulas may absorb and every
//! contribution of which is above zero is not derivable. An atom may be
//! declared dead: its literal of one sign contributes nothing a sum can
//! cancel (as `0`), its other one absorbs (as `⊤`), so an axiom on it
//! still settles; a Horn program's atoms that no firing reaches die in
//! the bodies' sign. With weakening the count holds of the formulas that
//! have one output, those of the sign the bodies' literals have, and a
//! formula without an output may be dropped, contributing zero.
//!
//! This is the test `overweight` of deduxit's Rocq library (`Weights.v`,
//! `overweight_affine` with weakening, and `FirstOrderWeights.v` for a
//! goal with a quantifier, whose rule changes no literal's weight), which
//! proves it sound for any weights, dead atoms and sign: [`overweight`](crate::linear::search::weighted::overweight)
//! computes the same lower bound per occurrence, so a goal it accepts is
//! one the library's theorem refutes.

use super::refutation::RefutationFault;
use crate::linear::occurrences::{Forest, OccId, Sign};
use crate::linear::sequents::Kind;

/// The invariant the faults of the test name.
const INVARIANT: &str = "weighted count";

/// A lower bound of the numbers a formula can contribute: none at all
/// (`Never`, as `0` contributes), every one at least a number, or none
/// known (`Unbounded`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Low {
    /// The formula contributes no number.
    Never,
    /// Every number it contributes is at least this.
    From(i128),
    /// Its numbers have no lower bound.
    Unbounded,
}

impl Low {
    /// Returns the bound of a sum, `None` where it leaves an `i128`; every
    /// sum of the test is one of these, so none wraps.
    fn sum(self, other: Self) -> Option<Self> {
        Some(match (self, other) {
            (Low::Never, _) | (_, Low::Never) => Low::Never,
            (Low::From(a), Low::From(b)) => Low::From(a.checked_add(b)?),
            _ => Low::Unbounded,
        })
    }

    /// Returns the bound of either.
    fn min(self, other: Self) -> Self {
        match (self, other) {
            (Low::Never, z) | (z, Low::Never) => z,
            (Low::From(a), Low::From(b)) => Low::From(a.min(b)),
            _ => Low::Unbounded,
        }
    }

    /// Returns the bound of a sum of any number of copies, none included.
    const fn star(self) -> Self {
        match self {
            Low::Never => Low::From(0),
            Low::From(z) if z >= 0 => Low::From(0),
            Low::From(_) | Low::Unbounded => Low::Unbounded,
        }
    }

    /// Returns whether the bound rules zero out: every number is above
    /// zero, or there is none.
    const fn positive(self) -> bool {
        match self {
            Low::Never => true,
            Low::From(z) => z > 0,
            Low::Unbounded => false,
        }
    }

    /// Returns whether a formula with this bound, which may absorb or not,
    /// can contribute something a sum can cancel or absorb.
    const fn reaches(self, absorbs: bool) -> bool {
        absorbs || !matches!(self, Low::Never)
    }
}

/// What the test weighs: the weight of each literal's atom (or group),
/// which the literal's positive form contributes and its negation
/// negated, whether the atom is dead, the sign of a dead atom's literal
/// that contributes nothing (the other one absorbs), and whether
/// formulas without an output may be dropped (with weakening, where the
/// outputs are the literals of that sign).
pub(crate) struct Weights<'a> {
    /// The weight of a literal's atom, at most 2⁶³ in absolute value.
    pub(crate) weight: &'a dyn Fn(OccId) -> i128,
    /// Whether a literal's atom is dead.
    pub(crate) dead: &'a dyn Fn(OccId) -> bool,
    /// The sign of a dead atom's literal that contributes nothing, and with
    /// weakening the outputs' sign.
    pub(crate) sign: Sign,
    /// Whether the calculus has weakening.
    pub(crate) weakening: bool,
}

/// What the test knows of an occurrence: its bound, whether it may
/// absorb, its outputs and whether it and every subformula has outputs as
/// the count with weakening needs (none below zero, none below a `?`).
#[derive(Clone, Copy)]
struct Node {
    /// The lower bound of its contributions.
    least: Low,
    /// Whether it may absorb.
    absorbs: bool,
    /// Its outputs: a sum of ±1 over its subtree's occurrences, fewer
    /// than 2³², within an `i64`.
    outputs: i64,
    /// Whether it is shaped.
    shaped: bool,
}

/// Checks that the weighted test accepts the goal: no formula of it may
/// absorb, and every contribution it can make is above zero, with
/// weakening of the goal's formulas shaped and with exactly one output.
///
/// # Errors
///
/// [`RefutationFault::NotApplicable`] where a formula may absorb, where
/// with weakening the formulas are not shaped or have other than one
/// output, or where a sum leaves an `i128`; [`RefutationFault::Holds`]
/// where some contribution is zero or below.
pub(crate) fn overweight(
    forest: &Forest,
    goal: &[OccId],
    weights: &Weights<'_>,
) -> Result<(), RefutationFault> {
    let overflow = RefutationFault::NotApplicable {
        invariant: INVARIANT,
        why: "a sum leaves 128 bits",
    };
    let positive = weights.sign == Sign::Atom;
    let mut nodes: Vec<Option<Node>> = vec![None; forest.len()];
    let (mut total, mut outputs, mut shaped, mut absorbs) = (Low::From(0), 0i64, true, false);
    for &member in goal {
        // Preorder: a subtree's occurrences follow its root, so its
        // operands come first from the end.
        for o in forest.subtree(member).rev() {
            let side = |side: Option<OccId>| side.and_then(|s| nodes[s.index()]);
            let (l, r) = (side(forest.left(o)), side(forest.right(o)));
            let both = || l.zip(r).expect("a binary connective has two operands");
            let one = || l.expect("a unary connective has an operand");
            let kind = forest.kind(o);
            let outputs = match kind {
                Kind::Atom => i64::from(positive),
                Kind::DualAtom => i64::from(!positive),
                Kind::One | Kind::Top | Kind::Bang => 1,
                Kind::Bot | Kind::Zero | Kind::Quest => 0,
                Kind::Tensor => both().0.outputs + both().1.outputs - 1,
                Kind::Par => both().0.outputs + both().1.outputs,
                Kind::With | Kind::Plus => both().0.outputs.max(both().1.outputs),
                Kind::Forall | Kind::Exists => one().outputs,
            };
            let shaped = outputs >= 0
                && match kind {
                    Kind::Tensor | Kind::Par | Kind::With | Kind::Plus => {
                        both().0.shaped && both().1.shaped
                    }
                    Kind::Bang | Kind::Forall | Kind::Exists => one().shaped,
                    Kind::Quest => one().outputs == 0 && one().shaped,
                    Kind::Atom
                    | Kind::DualAtom
                    | Kind::One
                    | Kind::Bot
                    | Kind::Top
                    | Kind::Zero => true,
                };
            let literal = |negated: bool| {
                if (weights.dead)(o) {
                    Low::Never
                } else {
                    // A weight is at most 2⁶³ in absolute value, so its
                    // negation is an `i128` too.
                    let w = (weights.weight)(o);
                    Low::From(if negated { -w } else { w })
                }
            };
            let least = match kind {
                Kind::Atom => literal(false),
                Kind::DualAtom => literal(true),
                Kind::One | Kind::Bot | Kind::Bang => Low::From(0),
                Kind::Top | Kind::Zero => Low::Never,
                Kind::Tensor | Kind::Par => both().0.least.sum(both().1.least).ok_or(overflow)?,
                Kind::With | Kind::Plus => both().0.least.min(both().1.least),
                Kind::Quest => one().least.star(),
                Kind::Forall | Kind::Exists => one().least,
            };
            // A formula that may be dropped contributes zero too.
            let least = if weights.weakening && outputs == 0 {
                Low::From(0).min(least)
            } else {
                least
            };
            let absorbs = match kind {
                Kind::Atom => (weights.dead)(o) && !positive,
                Kind::DualAtom => (weights.dead)(o) && positive,
                Kind::Top => true,
                Kind::Tensor => {
                    let (a, b) = both();
                    a.absorbs && b.least.reaches(b.absorbs)
                        || b.absorbs && a.least.reaches(a.absorbs)
                }
                Kind::Par | Kind::With | Kind::Plus => both().0.absorbs || both().1.absorbs,
                Kind::Quest | Kind::Forall | Kind::Exists => one().absorbs,
                Kind::One | Kind::Bot | Kind::Zero | Kind::Bang => false,
            };
            nodes[o.index()] = Some(Node {
                least,
                absorbs,
                outputs,
                shaped,
            });
        }
        let node = nodes[member.index()].expect("the member was just visited");
        total = total.sum(node.least).ok_or(overflow)?;
        // A shaped member has no output below zero and an unshaped one
        // refuses whatever the sum, so a sum saturated past an `i64` is
        // more than one output and stays so.
        outputs = outputs.saturating_add(node.outputs);
        shaped &= node.shaped;
        absorbs |= node.absorbs;
    }
    if weights.weakening && !(shaped && outputs == 1) {
        return Err(RefutationFault::NotApplicable {
            invariant: INVARIANT,
            why: "with weakening it counts goals of shaped formulas with one output, which this one is not",
        });
    }
    if absorbs {
        return Err(RefutationFault::NotApplicable {
            invariant: INVARIANT,
            why: "a formula may absorb any count: a ⊤, or a dead atom's literal of the other sign",
        });
    }
    if total.positive() {
        Ok(())
    } else {
        Err(RefutationFault::Holds)
    }
}

/// Returns the atoms of the goal that no firing reaches when the goal is
/// read as a Horn program whose bodies' literals have the sign `body`, as
/// a value per atom of the sequent, or `None` where it is no such program.
/// A member under `?` is a clause, any other a marking (a `⅋` of `⊥` and
/// literals of the other sign) or a clause: a tensor of body literals and
/// `1` with at most one head, a marking's shape. The atoms reached are
/// the least set that holds the markings' atoms and the heads' atoms of
/// every clause whose body atoms it holds; the rest of the goal's atoms
/// are dead. Whatever the set, the weighted test is sound; this one makes
/// a clause that can never fire contribute nothing.
pub(crate) fn dead_atoms(forest: &Forest, goal: &[OccId], body: Sign) -> Option<Vec<bool>> {
    let atoms = forest.sequent().atom_names().len();
    let head = |o: OccId| {
        forest.subtree(o).all(|x| match forest.kind(x) {
            Kind::Par | Kind::Bot => true,
            _ => forest.sign(x) == Some(!body),
        })
    };
    let atom = |o: OccId| forest.atom(o).expect("a literal").index();
    let literals = |o: OccId| forest.subtree(o).filter(|&x| forest.sign(x).is_some());
    let mut named = vec![false; atoms];
    let mut live = vec![false; atoms];
    // Per clause the body literals not yet reached and its head; per atom
    // the clauses waiting for it, once per literal.
    let mut missing: Vec<usize> = Vec::new();
    let mut heads: Vec<Option<OccId>> = Vec::new();
    let mut waiting: Vec<Vec<usize>> = vec![Vec::new(); atoms];
    for &member in goal {
        for x in literals(member) {
            named[atom(x)] = true;
        }
        let clause = match forest.kind(member) {
            Kind::Quest => forest.left(member).expect("a `?` has an operand"),
            _ if head(member) => {
                for x in literals(member) {
                    live[atom(x)] = true;
                }
                continue;
            }
            _ => member,
        };
        let c = heads.len();
        let (mut inputs, mut found) = (Vec::new(), None);
        let mut factors = vec![clause];
        while let Some(x) = factors.pop() {
            match forest.kind(x) {
                Kind::Tensor => factors.extend(forest.left(x).into_iter().chain(forest.right(x))),
                Kind::One => {}
                _ if forest.sign(x) == Some(body) => inputs.push(atom(x)),
                _ if found.is_none() && head(x) => found = Some(x),
                _ => return None,
            }
        }
        for &a in &inputs {
            waiting[a].push(c);
        }
        missing.push(inputs.len());
        heads.push(found);
    }
    // The markings' atoms are reached; a clause fires once its body's are.
    let mut ready: Vec<usize> = Vec::new();
    for a in 0..atoms {
        if live[a] {
            for &c in &waiting[a] {
                missing[c] -= 1;
            }
        }
    }
    ready.extend((0..heads.len()).filter(|&c| missing[c] == 0));
    while let Some(c) = ready.pop() {
        let Some(h) = heads[c] else { continue };
        for x in literals(h) {
            let a = atom(x);
            if !live[a] {
                live[a] = true;
                for &d in &waiting[a] {
                    missing[d] -= 1;
                    if missing[d] == 0 {
                        ready.push(d);
                    }
                }
            }
        }
    }
    Some((0..atoms).map(|a| named[a] && !live[a]).collect())
}

/// The dead atoms and the sign they die in under which a state equation's
/// weights pass the weighted test: the bodies' sign of the Horn program
/// the goal is read as, and a value per atom of the sequent.
pub(crate) struct Deaths {
    /// The bodies' sign, whose literal of a dead atom contributes nothing;
    /// the weights are negated for [`Sign::Dual`], whose bodies and goal
    /// are negative literals.
    #[cfg_attr(
        not(feature = "rocq"),
        expect(dead_code, reason = "the Rocq certificate writes the reading")
    )]
    pub(crate) body: Sign,
    /// Whether each atom of the sequent is dead.
    #[cfg_attr(
        not(feature = "rocq"),
        expect(dead_code, reason = "the Rocq certificate writes the reading")
    )]
    pub(crate) dead: Vec<bool>,
}

/// Checks a state equation's weights by the weighted test: the goal read
/// as a Horn program with its bodies' literals positive, else negative,
/// the atoms no firing reaches dead in that sign, and the atoms weighed
/// as stated, negated for negative bodies, with weakening in affine mode;
/// returns the reading the test accepts. The ticket weights and the
/// clauses dropped are the engine's account of its inequalities, which
/// the test does not need: it sums each clause's own literals, and a
/// clause that can never fire has a dead atom in its body.
///
/// # Errors
///
/// [`RefutationFault::Mismatch`] for an atom or occurrence the sequent
/// lacks or atoms out of the sequent's order;
/// [`RefutationFault::NotApplicable`] for a sequent with a quantifier, a
/// goal that is no Horn program, and as [`overweight`] answers it; and
/// [`RefutationFault::Holds`] where neither reading passes the test.
pub(crate) fn state_equation(
    forest: &Forest,
    goal: &[OccId],
    mode: crate::linear::Mode,
    equation: &super::StateEquation,
) -> Result<Deaths, RefutationFault> {
    const INVARIANT: &str = "state equation";
    if forest.has_binders() {
        return Err(RefutationFault::NotApplicable {
            invariant: INVARIANT,
            why: "it is of propositional Horn programs, and the sequent has a quantifier",
        });
    }
    let atoms = forest.sequent().atom_names().len();
    let ordered = equation
        .atoms
        .windows(2)
        .all(|pair| pair[0].0.index() < pair[1].0.index());
    let occurrences = equation
        .clauses
        .iter()
        .map(|&(o, _)| o)
        .chain(equation.dropped.iter().copied());
    if !ordered
        || equation.atoms.iter().any(|&(a, _)| a.index() >= atoms)
        || occurrences.into_iter().any(|o| o.index() >= forest.len())
    {
        return Err(RefutationFault::Mismatch);
    }
    let mut weight = vec![0i128; atoms];
    for &(a, w) in &equation.atoms {
        weight[a.index()] = i128::from(w);
    }
    let atom = |o: OccId| forest.atom(o).expect("a literal").index();
    // The first reading's fault, or that there is none.
    let mut fault = None;
    for body in [Sign::Atom, Sign::Dual] {
        let Some(dead) = dead_atoms(forest, goal, body) else {
            continue;
        };
        let negated = body == Sign::Dual;
        let weights = Weights {
            weight: &|o| {
                let w = weight[atom(o)];
                if negated { -w } else { w }
            },
            dead: &|o| dead[atom(o)],
            sign: body,
            weakening: mode.affine,
        };
        match overweight(forest, goal, &weights) {
            Ok(()) => return Ok(Deaths { body, dead }),
            Err(found) => {
                fault.get_or_insert(found);
            }
        }
    }
    Err(fault.unwrap_or(RefutationFault::NotApplicable {
        invariant: INVARIANT,
        why: "the goal is no Horn program",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sum past an `i128` is refused, not wrapped: no goal gets there
    /// (its bounds stay within ±2⁹⁵ per member), so the bounds' own sum is
    /// tested where it does.
    #[test]
    fn a_sum_past_128_bits_is_refused() {
        let top = Low::From(i128::MAX);
        assert_eq!(top.sum(Low::From(1)), None);
        assert_eq!(Low::From(i128::MIN).sum(Low::From(-1)), None);
        assert_eq!(top.sum(Low::From(-1)), Some(Low::From(i128::MAX - 1)));
        assert_eq!(top.sum(Low::Never), Some(Low::Never));
    }

    /// The test refuses where a sum of its bounds would leave an `i128`,
    /// within a formula (a `⊗`) and across the goal's members, driven
    /// there by weights past any an `i64` gives.
    #[cfg(feature = "parse")]
    #[test]
    fn the_test_refuses_a_sum_past_128_bits() {
        let weights = Weights {
            weight: &|_| i128::MAX,
            dead: &|_| false,
            sign: Sign::Atom,
            weakening: false,
        };
        for input in ["|- a * a", "|- a, a"] {
            let sequent: crate::linear::Sequent = input.parse().unwrap();
            let forest = Forest::new(&sequent).unwrap();
            assert_eq!(
                overweight(&forest, forest.roots(), &weights),
                Err(RefutationFault::NotApplicable {
                    invariant: INVARIANT,
                    why: "a sum leaves 128 bits",
                }),
                "{input}"
            );
        }
    }
}
