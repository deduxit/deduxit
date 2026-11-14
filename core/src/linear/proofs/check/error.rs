// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What a failed check reports: the fault at a node with the sequents its
//! premises derived, or the refusal that ended the check without a
//! verdict.

use crate::errors::describe::{Budget, Limit, Names, cut, occurrence};
use crate::errors::{Described, Owner, Subject};
use crate::limits::Refusal;
use crate::linear::occurrences::{Member, ShapeError, Side};
use crate::linear::proofs::{Node, NodeId};
use crate::linear::sequents::Eigen;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A sequent as the checker derived it at a node, its formulas named by
/// member ids.
///
/// The sequent is dyadic, `⊢ Θ ; Γ`: it has two zones. `Θ`, the
/// unrestricted zone, holds subformulas of `?` formulas, which the
/// subproof may use any number of times. `Γ`, the linear zone, holds the
/// formulas used once each, and may hold one member more than once.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dyadic {
    /// The unrestricted zone, ascending.
    pub theta: Vec<Member>,
    /// The linear zone, ascending, with repeats.
    pub gamma: Vec<Member>,
    /// Whether a `⊤` above lets the linear zone hold any further formulas.
    pub any: bool,
}

impl Display for Dyadic {
    /// Writes `⊢ Θ ; Γ` with member ids. `Θ ;` is left out when `Θ` is
    /// empty, and `…` follows `Γ` when `any` is set.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.write(f, None, None)
    }
}

impl Dyadic {
    /// Writes the sequent as [`Display`] does, or with formulas when
    /// `names` are given: two-sided under an intuitionistic reading, every
    /// list cut at `limit`.
    fn write(&self, f: &mut Formatter<'_>, names: Option<Names<'_>>, limit: Limit) -> FmtResult {
        let list = |f: &mut Formatter<'_>, ids: &mut dyn Iterator<Item = &Member>, len: usize| {
            let mut budget = Budget::new(f, limit);
            for (i, &o) in ids.enumerate() {
                let written =
                    std::fmt::Write::write_str(&mut budget, if i == 0 { " " } else { ", " })
                        .and_then(|()| occurrence(&mut budget, names, o));
                if written.is_err() && budget.cut {
                    return write!(budget.f, "… ({len} formulas)");
                }
                written?;
            }
            Ok(())
        };
        let any =
            |f: &mut Formatter<'_>, empty: bool| f.write_str(if empty { " …" } else { ", …" });
        // Under a reading the sequent is two-sided, `Θ ; Γ ⊢ C`: the
        // unrestricted zone holds hypotheses under a `!`.
        if let Some(Names {
            reading: Some(reading),
            framing,
            ..
        }) = names
        {
            let occ = move |m: Member| framing.map_or(m.occ(), |f| f.occurrence(m));
            let side = |side: Side| move |o: &&Member| reading.position(occ(**o)) == side;
            let inputs = self.gamma.iter().filter(side(Side::Input)).count();
            if !self.theta.is_empty() {
                list(f, &mut self.theta.iter(), self.theta.len())?;
                f.write_str(" ;")?;
            }
            list(f, &mut self.gamma.iter().filter(side(Side::Input)), inputs)?;
            if self.any {
                any(f, inputs == 0)?;
            }
            f.write_str(if self.theta.is_empty() && inputs == 0 && !self.any {
                "⊢"
            } else {
                " ⊢"
            })?;
            let outputs = self.gamma.len() - inputs;
            return list(
                f,
                &mut self.gamma.iter().filter(side(Side::Output)),
                outputs,
            );
        }
        f.write_str("⊢")?;
        if !self.theta.is_empty() {
            list(f, &mut self.theta.iter(), self.theta.len())?;
            f.write_str(" ;")?;
        }
        list(f, &mut self.gamma.iter(), self.gamma.len())?;
        if self.any {
            any(f, self.gamma.is_empty())?;
        }
        Ok(())
    }
}

/// Why a check did not accept a proof: a verdict that the term is no
/// proof, or a check that ended before it decided.
///
/// Keep the two variants apart:
/// - [`Invalid`](Self::Invalid) is a verdict. The term does not prove its
///   conclusion in the mode, and [`Invalid`] says at which node and why.
/// - [`Refused`](Self::Refused) is no verdict. The check reached a bound
///   of its [`Limits`](crate::Limits), or the caller's stop, before it
///   decided. Never report it as an invalid proof. To get a verdict,
///   check again with a larger bound.
///
/// [`describe`](Self::describe) writes either with formulas in place of
/// member ids.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::Limits;
/// use deduxit::linear::proofs::Fault;
/// use deduxit::linear::{CheckError, Forest, Member, Mode, Node, NodeId, Proof, Sequent};
///
/// // ⊢ ~A, A ⊗ ~B, B, with the members 0: ~A, 1: A ⊗ ~B, 2: A,
/// // 3: ~B, 4: B, and the premises of the ⊗ the wrong way round.
/// let sequent: Sequent = "A, A -o B |- B".parse()?;
/// let (o, n) = (Member::new, NodeId::new);
/// let nodes = vec![
///     Node::Ax(o(0), o(2)),
///     Node::Ax(o(3), o(4)),
///     Node::Tensor(o(1), n(1), n(0)),
/// ];
/// let proof = Proof::new(Forest::new(&sequent)?, nodes, n(2))?;
///
/// let error = proof.check(Mode::CLASSICAL).unwrap_err();
/// let CheckError::Invalid(invalid) = &error else {
///     panic!("a verdict was expected: {error}");
/// };
/// assert!(matches!(invalid.fault, Fault::Missing { premise: 0, .. }));
/// assert_eq!(
///     error.describe(&proof).to_string(),
///     "node 2 (⊗ on A ⊗ ~B from 1, 0) with premises ⊢ ~B, B and ⊢ ~A, A: \
///      premise 0 lacks A"
/// );
///
/// // Within one byte of memory the check ends before it decides.
/// let limits = Limits::default().with_memory_bytes(Some(1));
/// let error = proof.check_within(Mode::CLASSICAL, &limits, |_| false).unwrap_err();
/// assert!(matches!(error, CheckError::Refused(_)));
/// # Ok::<(), deduxit::Error>(())
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckError {
    /// The term is not a proof of its conclusion in the mode.
    Invalid(Box<Invalid>),
    /// The check ended without a verdict, at a bound or at the caller's
    /// stop.
    Refused(Refused),
}

/// Why a term is no proof: the node at fault, the first in arena order,
/// the sequents its premises derived, and the fault.
///
/// [`CheckError::describe`] writes it with formulas.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invalid {
    /// The node at fault. It is the root when the proof concludes another
    /// sequent, or when the sequent has no intuitionistic reading.
    pub node: NodeId,
    /// The node's rule instance.
    pub rule: Node,
    /// The sequents the node's premises derived, in the node's order.
    /// Empty when the sequent has no intuitionistic reading.
    pub premises: Vec<Dyadic>,
    /// What the rule required and did not get.
    pub fault: Fault,
}

/// Where a check ended without a verdict, and why.
///
/// It says nothing about the proof. A check within a larger bound, or
/// without the stop, may decide.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refused {
    /// The node the check had come to: node 0 when the check was refused
    /// before its first node.
    pub node: NodeId,
    /// The bound that was reached, or the stop.
    pub refusal: Refusal,
}

/// What was wrong at the node at fault. Every fault is a verdict: the
/// term is no proof in the mode.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault {
    /// The mode does not have the rule:
    /// - weakening of a formula that is not a `?` formula, outside affine
    ///   mode;
    /// - Mix, unless the mode has Mix and is not intuitionistic;
    /// - in an ordered mode, every rule but the axiom, `⊗` and `⅋`.
    Forbidden,
    /// The mode is intuitionistic and the sequent has no intuitionistic
    /// reading, so no proof of it is checked.
    Shape(ShapeError),
    /// The mode is intuitionistic and a sequent has `count` formulas right
    /// of `⊢` instead of one. The sequent is the one the node derives, a
    /// premise's sequent with a formula a `⊤` would absorb, the premise of
    /// a weakening of the goal (`count` 0), or the goal of the proof.
    #[non_exhaustive]
    Succedents {
        /// How many formulas stand right of `⊢`.
        count: usize,
    },
    /// The member is not of the kind the rule acts on: a `⅋` step on a
    /// `⊗` formula, or an axiom on a formula that is no literal.
    #[non_exhaustive]
    Kind {
        /// The member the rule names.
        member: Member,
    },
    /// The two literals of an axiom are not an atom and its negation.
    NotDual,
    /// A premise lacks the member the rule takes from it.
    #[non_exhaustive]
    Missing {
        /// Which premise, by its index in the node: 0 for the first.
        premise: usize,
        /// What it lacks.
        member: Member,
    },
    /// The premise of a promotion (`!`) has linear formulas besides the
    /// promoted subformula.
    NotEmpty,
    /// The premises of a `&` do not have the same linear context.
    Differ,
    /// In an ordered mode, the premises of a `⊗` do not split its
    /// conclusion in order. Going round the conclusion's cyclic order
    /// from the `⊗`, the right premise's context must come first and the
    /// left premise's after it. Any other split needs an exchange, which
    /// an ordered mode does not have.
    Order,
    /// In the Lambek calculus without empty antecedents, a sequent has no
    /// formula left of `⊢`. Every sequent of such a proof keeps a
    /// hypothesis.
    EmptyAntecedent,
    /// A copy of a member that is not the subformula of a `?`.
    #[non_exhaustive]
    NotUnderQuest {
        /// The member copied.
        member: Member,
    },
    /// A cut names a member outside the cut formulas `A` of the sequent:
    /// a member of the conclusion, or of a cut formula's dual.
    #[non_exhaustive]
    NotACut {
        /// The member the cut names.
        member: Member,
    },
    /// The node's linear zone holds more formulas than the rest of the
    /// proof can consume: each later rule takes two at most, and the root
    /// must conclude the sequent. No engine builds such a term. With
    /// counters of unbounded width it would fail at the root instead.
    Surplus,
    /// The node names an occurrence under binders (`∀`, `∃`) by itself,
    /// without an instance: a term for each binder above it. Only a member
    /// of the proof's instance table has those terms.
    #[non_exhaustive]
    Instance {
        /// The member the node names.
        member: Member,
    },
    /// An eigenvariable, the fresh variable a `∀` step introduces, occurs
    /// outside that step: a witness, or the frame of a member a cut is
    /// on, names it below the step or beside it. The fault is at the
    /// root, which still carries the eigenvariable.
    ///
    /// An eigenvariable that no `∀` step introduces is no fault: it is a
    /// constant outside the signature, which any witness may name.
    #[non_exhaustive]
    Eigenvariable {
        /// The eigenvariable.
        eigen: Eigen,
    },
    /// A second `∀` step introduces the same eigenvariable: each `∀` step
    /// needs its own. The fault is at the later step in arena order.
    #[non_exhaustive]
    Reintroduced {
        /// The eigenvariable.
        eigen: Eigen,
    },
    /// The root derives a sequent that is not the proof's conclusion: its
    /// linear zone differs from the conclusion's formulas, or its
    /// unrestricted zone holds a copied member that no `?` step below
    /// moved there.
    #[non_exhaustive]
    Conclusion {
        /// What the root derives.
        derived: Dyadic,
    },
}

impl Invalid {
    /// Returns the fault at `node`, with the premises derived for it.
    pub(crate) fn new(node: NodeId, rule: Node, premises: Vec<Dyadic>, fault: Fault) -> Self {
        Self {
            node,
            rule,
            premises,
            fault,
        }
    }
}

impl CheckError {
    /// Returns the node at fault, or for a refusal the node the check had
    /// come to: node 0 when it was refused before its first node.
    pub fn node(&self) -> NodeId {
        match self {
            Self::Invalid(invalid) => invalid.node,
            Self::Refused(refused) => refused.node,
        }
    }

    /// Returns the fault, or `None` for a refusal.
    pub fn fault(&self) -> Option<&Fault> {
        match self {
            Self::Invalid(invalid) => Some(&invalid.fault),
            Self::Refused(_) => None,
        }
    }

    /// Returns the error of a fault at `node`.
    pub(crate) fn invalid(node: NodeId, rule: Node, premises: Vec<Dyadic>, fault: Fault) -> Self {
        Self::Invalid(Box::new(Invalid::new(node, rule, premises, fault)))
    }
}

impl Display for CheckError {
    /// Writes the node, its premises and the fault with member ids, as in
    /// `node 2 (⊗ on 1 from 0, 1) with premises ⊢ 0, 3 and ⊢ 3, 4: premise 0
    /// lacks formula 2`. A refusal says that the check ended without a
    /// verdict, and why.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.write(f, None, None)
    }
}

impl CheckError {
    /// Returns the error for display with formulas in place of member ids,
    /// read from `owner`: the proof that failed, or its forest. Nodes keep
    /// their ids, as in `node 2 (⊗ on A ⊗ ~B from 0, 1) with premises
    /// ⊢ ~A, ~B and ⊢ ~B, B: premise 0 lacks A`.
    pub fn describe<'a>(&'a self, owner: &'a impl Owner) -> Described<'a> {
        Described::new(Subject::Check(self), owner)
    }

    /// Writes the error as [`Display`] does, or with formulas when `names`
    /// are given, each formula cut at `limit`.
    pub(crate) fn write(
        &self,
        f: &mut Formatter<'_>,
        names: Option<Names<'_>>,
        limit: Limit,
    ) -> FmtResult {
        let invalid = match self {
            Self::Invalid(invalid) => invalid,
            Self::Refused(refused) => {
                return write!(
                    f,
                    "the check stopped at node {} without a verdict on the proof: {}",
                    refused.node.get(),
                    refused.refusal
                );
            }
        };
        invalid.write(f, names, limit)
    }
}

impl Invalid {
    /// Writes the fault as [`CheckError`]'s `Display` does, or with
    /// formulas when `names` are given, each formula cut at `limit`.
    fn write(&self, f: &mut Formatter<'_>, names: Option<Names<'_>>, limit: Limit) -> FmtResult {
        let reading = names.and_then(|n| n.reading);
        let occ = |m: Member| {
            names
                .and_then(|n| n.framing)
                .map_or(m.occ(), |framing| framing.occurrence(m))
        };
        write!(
            f,
            "node {} ({}",
            self.node.get(),
            name(self.rule, reading, occ)
        )?;
        for (i, o) in self.rule.members().enumerate() {
            f.write_str(if i == 0 { " on " } else { ", " })?;
            cut(f, names, o, limit)?;
        }
        for (i, p) in self.rule.premises().enumerate() {
            write!(f, "{}{}", if i == 0 { " from " } else { ", " }, p.get())?;
        }
        f.write_str(")")?;
        for (i, p) in self.premises.iter().enumerate() {
            f.write_str(if i == 0 { " with premises " } else { " and " })?;
            p.write(f, names, limit)?;
        }
        f.write_str(": ")?;
        use Fault::*;
        match &self.fault {
            Forbidden => f.write_str("the rule is not allowed in this mode"),
            Shape(e) => {
                f.write_str("not an intuitionistic sequent: ")?;
                e.write(f, names, limit)
            }
            Succedents { count } => write!(
                f,
                "a sequent of the rule has {count} formulas right of ⊢ instead of one"
            ),
            Kind { member: o } => {
                f.write_str("the rule does not act on ")?;
                if names.is_none() {
                    f.write_str("formula ")?;
                }
                cut(f, names, *o, limit)
            }
            NotDual => f.write_str(if reading.is_some() {
                "the axiom's formulas are not one atom on either side of ⊢"
            } else {
                "the literals are not an atom and its negation"
            }),
            Missing { premise, member: o } => {
                write!(f, "premise {premise} lacks ")?;
                if names.is_none() {
                    f.write_str("formula ")?;
                }
                cut(f, names, *o, limit)
            }
            // Intuitionistically a promotion is `!R`, and an input `?` a
            // hypothesis under a `!`; a `&` node is `&R` or `⊕L`.
            NotEmpty => f.write_str(if reading.is_some() {
                "the premise of !R has a hypothesis that is not a ! formula"
            } else {
                "the premise of ! has formulas besides the promoted one that are not ? formulas"
            }),
            Differ => f.write_str("the two premises do not have the same context"),
            Order => f.write_str("the premises' contexts are not the arcs on either side of the ⊗"),
            EmptyAntecedent => f.write_str(
                "the sequent has no formula left of ⊢, which the Lambek calculus without empty antecedents forbids",
            ),
            NotACut { member: o } => {
                if names.is_none() {
                    f.write_str("formula ")?;
                }
                cut(f, names, *o, limit)?;
                f.write_str(" is not in a cut formula")
            }
            NotUnderQuest { member: o } => {
                if names.is_none() {
                    f.write_str("formula ")?;
                }
                cut(f, names, *o, limit)?;
                // An input `?` is a hypothesis under a `!`.
                f.write_str(if reading.is_some() {
                    " is not under the ! of a hypothesis"
                } else {
                    " is not under a ?"
                })
            }
            Surplus => f.write_str(
                "the linear zone holds more formulas than the rest of the proof can consume",
            ),
            Instance { member: o } => {
                if names.is_none() {
                    f.write_str("formula ")?;
                }
                cut(f, names, *o, limit)?;
                f.write_str(" lies under a quantifier but is named without the terms of its instance")
            }
            Eigenvariable { eigen } => {
                f.write_str("eigenvariable ")?;
                eigenvariable(f, names, *eigen)?;
                f.write_str(" occurs outside the ∀ step that introduces it")
            }
            Reintroduced { eigen } => {
                f.write_str("eigenvariable ")?;
                eigenvariable(f, names, *eigen)?;
                f.write_str(" is introduced by a second ∀ step")
            }
            Conclusion { derived: d } => {
                f.write_str("the proof concludes ")?;
                d.write(f, names, limit)?;
                f.write_str(", not the sequent")
            }
        }
    }
}

impl std::error::Error for CheckError {}

/// Returns the name of a node's rule: its tag, or under a reading the
/// two-sided name of the rule on its principal formula (`⊸L` for a `⊗`
/// on a hypothesis). `occ` gives the occurrence of a member.
fn name(
    node: Node,
    reading: Option<&crate::linear::Reading<'_>>,
    occ: impl Fn(Member) -> crate::linear::OccId,
) -> &'static str {
    use crate::linear::proofs::{Branch, Named, Rule};
    let (Some(reading), Some(o)) = (reading, node.principal()) else {
        return node.name();
    };
    let rule = match node {
        Node::Tensor(..) => Rule::Tensor,
        Node::Par(..) => Rule::Par,
        Node::One(_) => Rule::One,
        Node::Bot(..) => Rule::Bot,
        Node::With(..) => Rule::With,
        Node::Plus(_, Branch::Left, _) => Rule::PlusLeft,
        Node::Plus(_, Branch::Right, _) => Rule::PlusRight,
        Node::Top(_) => Rule::Top,
        Node::Bang(..) => Rule::Promotion,
        Node::Quest(..) => Rule::Dereliction,
        Node::Forall(..) => Rule::Forall,
        Node::Exists(..) => Rule::Exists,
        // Their tags name no connective.
        Node::Ax(..) | Node::Copy(..) | Node::Weaken(..) | Node::Mix(..) | Node::Cut(..) => {
            return node.name();
        }
    };
    Named::applied(rule, Some(reading), occ(o)).name()
}

/// Writes an eigenvariable by the name the proof's derivation gives it
/// when `names` are given, else by its number.
fn eigenvariable(f: &mut Formatter<'_>, names: Option<Names<'_>>, eigen: Eigen) -> FmtResult {
    match names.and_then(|n| n.framing) {
        Some(framing) => f.write_str(framing.name(eigen)),
        None => write!(f, "{}", eigen.get()),
    }
}
