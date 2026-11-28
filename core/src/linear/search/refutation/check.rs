// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! [`Disproof::check`] and the checks of the counts, an assignment and
//! the count equation, and the faults a check answers. They share no code
//! with the search: each evaluates the goal over a forest of its own.

use super::{
    Classical, Disproof, Equation, OneSided, OneSidedSymbol, Refutation, Unbalanced,
    UnbalancedSymbol,
};
use crate::linear::fragment::Mode;
use crate::linear::occurrences::{OccId, Sign};
use crate::linear::sequents::{Atom, Sequent};

/// Why [`Disproof::check`] does not accept a refutation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum RefutationFault {
    /// The refutation is no certificate. An exhausted search rests on the
    /// search itself, which the check cannot verify.
    #[error("a refutation of kind `{kind}` rests on the search and has no check")]
    Uncertified {
        /// The refutation's word ([`Refutation::name`]).
        kind: &'static str,
    },
    /// The refutation's invariant does not hold for this goal and mode.
    /// Weakening discards literals and connectives, a `⊤` absorbs them,
    /// copies add them, and the count equation holds in the
    /// multiplicative fragment only.
    #[error("the {invariant} is no invariant here: {why}")]
    NotApplicable {
        /// The invariant, such as `balance of the atom` or `count
        /// equation`.
        invariant: &'static str,
        /// What defeats it.
        why: &'static str,
    },
    /// The refutation's numbers are not the goal's. It names an atom or an
    /// occurrence the sequent lacks, or states counts that differ from the
    /// goal's.
    #[error("the refutation's numbers are not those of the sequent")]
    Mismatch,
    /// The numbers are the goal's, and they allow a proof. The atom's
    /// excess can be zero, the count equation holds, or some contribution
    /// to the weighted count is zero or below.
    #[error("the counts allow a proof")]
    Holds,
    /// The assignment has another number of values than the sequent has
    /// atoms.
    #[error("the assignment gives {values} values for {atoms} atoms")]
    Assignment {
        /// The atoms of the sequent.
        atoms: usize,
        /// The values given.
        values: usize,
    },
    /// The assignment makes the classical reading true.
    #[error("the classical reading is true under the assignment")]
    Satisfied,
    /// A sequent of a saturated set is not a sequent of the mode. Under
    /// the intuitionistic reading it has other than one member in output
    /// position, or more than one where it absorbs.
    #[error("a sequent of the saturated set is no sequent of the mode")]
    Unshaped,
    /// A rule applied to sequents of a saturated set derives one that
    /// could stand in a proof and that none of them stands for.
    #[error("the saturated set is not closed under the rules")]
    NotClosed,
    /// The sequents of a saturated set stand for every stable sequent of
    /// the goal's asynchronous phase, so they do not rule a proof out.
    #[error("the saturated set gives every stable sequent the goal needs")]
    Filled,
    /// A clause fired backward from a marking of a basis needs a marking
    /// that no marking of the basis is at most.
    #[error("the basis is not closed under the clauses fired backward")]
    Open,
    /// A marking of a basis is at most the initial marking, so the basis
    /// does not rule a firing sequence out.
    #[error("a marking of the basis is at most the initial marking")]
    Reached,
    /// No marking of a basis is at most the target, so the basis says
    /// nothing of the target.
    #[error("no marking of the basis is at most the target")]
    Missed,
}

impl RefutationFault {
    /// Returns the fault's word, as an error's details write it.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Uncertified { .. } => "uncertified",
            Self::NotApplicable { .. } => "not_applicable",
            Self::Mismatch => "mismatch",
            Self::Holds => "holds",
            Self::Assignment { .. } => "assignment",
            Self::Satisfied => "satisfied",
            Self::Unshaped => "unshaped",
            Self::NotClosed => "not_closed",
            Self::Filled => "filled",
            Self::Open => "open",
            Self::Reached => "reached",
            Self::Missed => "missed",
        }
    }
}

/// Why a check of a saturated set or a basis did not accept it: it does
/// not hold, or the caller's stop ended the check.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Failed {
    /// The refutation does not hold.
    Fault(RefutationFault),
    /// The caller's stop ended the check.
    Stopped,
}

impl Failed {
    /// Returns the fault, or `None` where the stop ended the check.
    pub(crate) const fn fault(self) -> Option<RefutationFault> {
        match self {
            Self::Fault(fault) => Some(fault),
            Self::Stopped => None,
        }
    }
}

impl From<RefutationFault> for Failed {
    /// The fault as a failure.
    fn from(fault: RefutationFault) -> Self {
        Self::Fault(fault)
    }
}

impl Disproof {
    /// Checks that the refutation proves the goal unprovable in the mode,
    /// and returns `Ok` when it does.
    ///
    /// The check shares no code with the search. Each kind is checked by
    /// the invariant it rests on:
    ///
    /// - An assignment ([`Refutation::Classical`]): the goal's classical
    ///   reading, evaluated under it, must be false.
    /// - An unbalanced atom ([`Refutation::Unbalanced`]): its excess of
    ///   positive over negative literals is computed as an interval over
    ///   the additive alternatives. `⊗` and `⅋` add their sides', and `&`
    ///   and `⊕` take the hull of theirs. The interval must be the one
    ///   stated and exclude zero. The mode must not be affine, and the goal
    ///   must have no `⊤` and no `!` or `?` over the atom.
    /// - The count equation ([`Refutation::Equation`]): the goal's
    ///   formulas, `⊗`, `⅋`, `1` and `⊥` must be the ones stated and fail
    ///   it. The goal must be in the multiplicative fragment with units,
    ///   the mode must not be affine, and Mix must be stated exactly where
    ///   the mode allows it.
    /// - A saturated set ([`Refutation::Saturated`]): the rules of the
    ///   focused calculus are read off the forest again, under the set's
    ///   polarity. Each is applied to every choice of premises among the
    ///   set's sequents, and so is Mix where the mode has it. Each sequent
    ///   derived must be one that a sequent of the set stands for, or one
    ///   that no proof holds. The set's sequents must be shaped as the
    ///   mode's are, and some stable sequent of the goal's phase must be
    ///   one that none of them stands for.
    /// - A one-sided count ([`Refutation::OneSided`]): its interval is
    ///   computed outside the goal's exponentials, as an unbalanced atom's,
    ///   with the side that copies reach left open. The sign of the atom's
    ///   literals below the exponentials must be the one stated. Then the
    ///   weighted test with the weight `±1` on the atom must accept the
    ///   goal.
    /// - A state equation ([`Refutation::StateEquation`]): the weighted
    ///   test with the stated weights must accept the goal, read as a Horn
    ///   program with positive bodies, else negative ones (the weights
    ///   negated). The atoms that no firing reaches are declared dead. In
    ///   affine mode the test counts the formulas with one output.
    /// - A basis ([`Refutation::Basis`]): the goal is read as a Petri net
    ///   again, with the bodies' sign stated. The classes must be its
    ///   clauses used once, each class of one body and head. Some marking
    ///   of the basis must be at most the target, none at most the initial
    ///   marking, and for every marking `m` and every clause that can ever
    ///   fire with a head on `m`'s places, some marking must be at most
    ///   `max(m − head, 0) + body`, the least marking from which the clause
    ///   leads to one at least `m`.
    ///
    /// The predicate forms, [`Refutation::UnbalancedSymbol`] and
    /// [`Refutation::OneSidedSymbol`], are checked as their atom forms,
    /// over every atom of the predicate.
    ///
    /// The weighted test is the one deduxit's Rocq library proves sound for
    /// any weights. No formula of the goal may absorb, and every
    /// contribution to the weighted count of its literals must be above
    /// zero, a `?` formula's copies included.
    ///
    /// These seven kinds are certificates. Every derivation keeps the
    /// invariant that each of the five others breaks, the set holds a
    /// sequent for every stable one of every proof, and the basis's upward
    /// closure every marking of every firing sequence a proof gives.
    ///
    /// # Cost
    ///
    /// Time and memory are linear in the goal's occurrences, a member
    /// repeated counted each time, for every kind but the saturated set
    /// and the basis. A basis's check compares, for every marking and
    /// clause, a marking with the basis's markings: quadratic in the basis
    /// times the clauses, over the markings' places.
    /// The set's check applies every rule to every choice of its sequents.
    /// For a set a search gave, that takes at most as long as the search
    /// that saturated. For a set read from elsewhere it can take time
    /// exponential in the `&` under `⅋` of the goal, and polynomial in the
    /// set's size, of the degree of a rule's holes (a set that repeats one
    /// sequent). This call has no stop and no bound on its work: check
    /// such a set with [`check_within`](Self::check_within).
    ///
    /// # Errors
    ///
    /// [`Error::Refutation`](crate::Error::Refutation) with the
    /// [`RefutationFault`]. The error's kind is invalid for a refutation
    /// that does not hold, and unsupported for an exhausted search
    /// ([`RefutationFault::Uncertified`]). The errors of
    /// [`Forest::new`](crate::linear::Forest::new) for a sequent past the
    /// default limits.
    ///
    /// # Examples
    ///
    /// A disproof sent as JSON, read back and checked, and a changed one
    /// that the check refuses:
    ///
    #[cfg_attr(all(feature = "parse", feature = "serialize"), doc = "```")]
    #[cfg_attr(not(all(feature = "parse", feature = "serialize")), doc = "```ignore")]
    /// use deduxit::linear::{Disproof, Mode, Options, Refutation, Sequent, Verdict, prove};
    ///
    /// let sequent: Sequent = "|- a & b, ~a".parse()?;
    /// let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
    /// let Verdict::Unprovable(disproof) = outcome.verdict else {
    ///     panic!("the sequent has no proof");
    /// };
    /// // False where `a` is true and `b` false.
    /// assert!(matches!(disproof.refutation(), Refutation::Classical(_)));
    /// let json = serde_json::to_string(&disproof)?;
    ///
    /// let read: Disproof = serde_json::from_str(&json)?;
    /// read.check()?;
    ///
    /// // Under this assignment the reading is true.
    /// let changed = json.replace("[true,false]", "[true,true]");
    /// let read: Disproof = serde_json::from_str(&changed)?;
    /// assert_eq!(read.check().unwrap_err().code(), "invalid_refutation");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn check(&self) -> Result<(), crate::Error> {
        // The default limits bound no work, so this is the check without
        // a stop that it always was.
        self.check_dyn(&crate::Limits::default(), &mut |_| false)
    }

    /// Checks the refutation as [`check`](Self::check) does, within
    /// `limits` and until `stop` fires: the forest is built within
    /// `limits.occurrences`, and the checks of a saturated set and of a
    /// basis, the two whose time is not linear, ask `stop` as they go
    /// ([`Phase::Check`](crate::Phase::Check)) and end at the first poll
    /// past `limits.work`, a unit being a step of their own count (the
    /// set's check polls every 2¹⁶ units, the basis's per clause, so a set
    /// checked in fewer units than a poll's is never refused). The other
    /// kinds are linear in the goal and ask nothing.
    ///
    /// # Errors
    ///
    /// Those of [`check`](Self::check), the forest's within `limits`,
    /// and [`Error::Refused`](crate::Error::Refused) with
    /// [`Refusal::Stopped`](crate::Refusal::Stopped) where `stop` ended
    /// the check, or [`Refusal::Work`](crate::Refusal::Work) past
    /// `limits.work`: no verdict on the refutation.
    pub fn check_within(
        &self,
        limits: &crate::Limits,
        mut stop: impl FnMut(crate::Progress) -> bool,
    ) -> Result<(), crate::Error> {
        self.check_dyn(limits, &mut stop)
    }

    /// The body of [`check_within`](Self::check_within).
    fn check_dyn(
        &self,
        limits: &crate::Limits,
        stop: &mut dyn FnMut(crate::Progress) -> bool,
    ) -> Result<(), crate::Error> {
        use crate::limits::{Phase, Progress};
        let fault = |fault| crate::Error::Refutation(fault);
        let forest = crate::linear::occurrences::Forest::within(&self.sequent, limits)?;
        // The work done, against `limits.work`, and whether it or the
        // stop ended the check.
        let mut done = 0u64;
        let mut polled = |units: u64| {
            done = done.saturating_add(units);
            stop(Progress::new(Phase::Check, units, done))
                || limits.work.is_some_and(|limit| done > limit)
        };
        // The counts carry over to quantifiers, since a quantifier's rule
        // changes none of them. An assignment and a saturated set are
        // propositional.
        if forest.has_binders()
            && matches!(
                self.refutation,
                Refutation::Classical(_) | Refutation::Saturated(_) | Refutation::Basis(_)
            )
        {
            return Err(fault(RefutationFault::NotApplicable {
                invariant: "this refutation",
                why: "it is propositional, and the sequent has a quantifier",
            }));
        }
        let goal: Vec<OccId> = match &self.goal {
            None => forest.roots().to_vec(),
            Some(members) => members
                .iter()
                .map(|m| {
                    m.occurrence(&forest)
                        .ok_or(fault(RefutationFault::Mismatch))
                })
                .collect::<Result<_, _>>()?,
        };
        match &self.refutation {
            Refutation::Classical(classical) => check_classical(&forest, &goal, classical),
            Refutation::Unbalanced(unbalanced) => {
                check_unbalanced(&forest, &goal, self.mode, unbalanced)
            }
            Refutation::UnbalancedSymbol(unbalanced) => {
                check_unbalanced_symbol(&forest, &goal, self.mode, unbalanced)
            }
            Refutation::Equation(equation) => check_equation(&forest, &goal, self.mode, equation),
            Refutation::Saturated(saturated) => {
                match super::saturated::check(&forest, &goal, self.mode, saturated, &mut polled) {
                    Ok(()) => Ok(()),
                    Err(Failed::Fault(fault)) => Err(fault),
                    Err(Failed::Stopped) => {
                        return Err(stopped(done, limits));
                    }
                }
            }
            Refutation::OneSided(one_sided) => {
                check_one_sided(&forest, &goal, self.mode, one_sided)
            }
            Refutation::OneSidedSymbol(one_sided) => {
                check_one_sided_symbol(&forest, &goal, self.mode, one_sided)
            }
            Refutation::StateEquation(equation) => {
                super::weighted::state_equation(&forest, &goal, self.mode, equation).map(|_| ())
            }
            Refutation::Basis(basis) => {
                match super::basis::check(&forest, &goal, basis, &mut polled) {
                    Ok(()) => Ok(()),
                    Err(Failed::Fault(fault)) => Err(fault),
                    Err(Failed::Stopped) => return Err(stopped(done, limits)),
                }
            }
            refutation @ Refutation::Exhausted => Err(RefutationFault::Uncertified {
                kind: refutation.name(),
            }),
        }
        .map_err(fault)
    }
}

/// Returns the refusal of a check that its poll ended after `done` units:
/// the work bound where they passed it, else the stop.
fn stopped(done: u64, limits: &crate::Limits) -> crate::Error {
    use crate::limits::{Phase, Refusal};
    crate::Error::Refused(match limits.work {
        Some(limit) if done > limit => Refusal::Work { limit },
        _ => Refusal::stopped(Phase::Check),
    })
}

/// Checks that `classical` falsifies the classical reading of the goal.
fn check_classical(
    forest: &crate::linear::occurrences::Forest,
    goal: &[OccId],
    classical: &Classical,
) -> Result<(), RefutationFault> {
    use crate::linear::sequents::Kind::*;
    let (atoms, values) = (
        forest.sequent().atom_names().len(),
        classical.assignment.len(),
    );
    if atoms != values {
        return Err(RefutationFault::Assignment { atoms, values });
    }
    let value = |o: OccId| {
        let atom = forest.atom(o).map(|a| classical.assignment[a.index()]);
        atom.unwrap_or(false)
    };
    let mut truth = vec![false; forest.len()];
    for &member in goal {
        // The forest is in preorder: a subtree's occurrences follow its
        // root. Walked from the end, operands come before their connective.
        for o in forest.subtree(member).rev() {
            let side = |side: Option<OccId>| side.is_some_and(|s| truth[s.index()]);
            let (l, r) = (side(forest.left(o)), side(forest.right(o)));
            truth[o.index()] = match forest.kind(o) {
                Atom => value(o),
                DualAtom => !value(o),
                One | Top => true,
                Bot | Zero => false,
                Tensor | With => l && r,
                Par | Plus => l || r,
                Bang | Quest => l,
                Forall | Exists => {
                    unreachable!("an assignment of a sequent with a binder is refused")
                }
            };
        }
        if truth[member.index()] {
            return Err(RefutationFault::Satisfied);
        }
    }
    Ok(())
}

/// Checks that the atom of `unbalanced` has the stated excess interval in
/// the goal, and that the interval excludes zero. In a sequent with a
/// quantifier, the balance is an invariant only where no atom of the
/// atom's predicate has a bound variable, since an instance of such an
/// atom could be this one.
fn check_unbalanced(
    forest: &crate::linear::occurrences::Forest,
    goal: &[OccId],
    mode: Mode,
    unbalanced: &Unbalanced,
) -> Result<(), RefutationFault> {
    let atom = unbalanced.atom;
    let sequent = forest.sequent();
    if atom.index() >= sequent.atom_names().len() {
        return Err(RefutationFault::Mismatch);
    }
    if predicate_open(sequent, atom) {
        return Err(RefutationFault::NotApplicable {
            invariant: "balance of the atom",
            why: "an atom of its predicate has a bound variable, whose instance may be it",
        });
    }
    check_balance(
        forest,
        goal,
        mode,
        |o| forest.atom(o) == Some(atom),
        (unbalanced.least, unbalanced.most),
    )
}

/// Checks that the literals of the predicate of `unbalanced`, every atom
/// of it counted, have the stated excess interval in the goal, and that
/// it excludes zero, where the balance is an invariant.
fn check_unbalanced_symbol(
    forest: &crate::linear::occurrences::Forest,
    goal: &[OccId],
    mode: Mode,
    unbalanced: &UnbalancedSymbol,
) -> Result<(), RefutationFault> {
    let sequent = forest.sequent();
    if unbalanced.symbol.index() >= sequent.symbols().len() {
        return Err(RefutationFault::Mismatch);
    }
    check_balance(
        forest,
        goal,
        mode,
        |o| {
            forest
                .atom(o)
                .and_then(|a| sequent.atom_predicate(a))
                .is_some_and(|(s, _)| s == unbalanced.symbol)
        },
        (unbalanced.least, unbalanced.most),
    )
}

/// Returns whether an atom of `atom`'s predicate has a bound variable,
/// whose instance may be `atom`: a count of `atom` alone is then no
/// invariant. An atom of no predicate has none.
fn predicate_open(sequent: &Sequent, atom: Atom) -> bool {
    sequent.atom_predicate(atom).is_some_and(|(symbol, _)| {
        (0..sequent.atom_names().len()).any(|a| {
            let a = Atom::new(a as u32);
            sequent.atom_predicate(a).map(|(s, _)| s) == Some(symbol) && sequent.atom_is_open(a)
        })
    })
}

/// Returns the least and the greatest excess of the literals `counted`
/// picks in `member`: positive ones count one, negative ones minus one,
/// `⊗` and `⅋` add their sides', `&` and `⊕` take the hull, a quantifier
/// keeps its body's, and a `!` or `?` gives `exponential` of its operand's.
/// `None` where a `⊤` absorbs. `span` holds an entry per occurrence of the
/// forest, written for the member's subtree. The sums are of ±1 over a
/// subtree of fewer than 2³² occurrences, within an `i64`.
fn excess(
    forest: &crate::linear::occurrences::Forest,
    member: OccId,
    counted: &impl Fn(OccId) -> bool,
    span: &mut [Option<(i64, i64)>],
    exponential: impl Fn(Option<(i64, i64)>) -> Option<(i64, i64)>,
) -> Option<(i64, i64)> {
    use crate::linear::sequents::Kind::*;
    // The forest is in preorder: walked from the end, a subtree's
    // operands come before their connective.
    for o in forest.subtree(member).rev() {
        let side = |side: Option<OccId>| side.and_then(|s| span[s.index()]);
        let (l, r) = (side(forest.left(o)), side(forest.right(o)));
        let literal = |e| Some(if counted(o) { (e, e) } else { (0, 0) });
        span[o.index()] = match forest.kind(o) {
            Atom => literal(1),
            DualAtom => literal(-1),
            One | Bot | Zero => Some((0, 0)),
            Top => None,
            Tensor | Par => l.zip(r).map(|((a, b), (c, d))| (a + c, b + d)),
            With | Plus => l.zip(r).map(|((a, b), (c, d))| (a.min(c), b.max(d))),
            Bang | Quest => exponential(l),
            Forall | Exists => l,
        };
    }
    span[member.index()]
}

/// Checks that the literals `counted` picks have the excess interval
/// `stated` in the goal, and that it excludes zero, where the balance is
/// an invariant. It is one because every axiom joins one positive and one
/// negative literal of a group (an atom, or a predicate). `⊗` and `⅋` add
/// their sides' excesses, `&` and `⊕` take the hull of theirs, and a
/// quantifier's rule changes none.
fn check_balance(
    forest: &crate::linear::occurrences::Forest,
    goal: &[OccId],
    mode: Mode,
    counted: impl Fn(OccId) -> bool,
    stated: (i32, i32),
) -> Result<(), RefutationFault> {
    const INVARIANT: &str = "balance of the atom";
    if mode.affine {
        return Err(RefutationFault::NotApplicable {
            invariant: INVARIANT,
            why: "weakening discards literals",
        });
    }
    // Per occurrence, the least and the greatest excess, or `None` where a
    // `⊤` absorbs or copies of the counted literals can add any number.
    // They are sums of ±1 over the goal's members, each a subtree of the
    // forest's fewer than 2³² occurrences. Leaving an `i64` would take a
    // goal of more than 2³¹ members, which no memory holds.
    let mut span: Vec<Option<(i64, i64)>> = vec![None; forest.len()];
    let (mut least, mut most) = (0, 0);
    for &member in goal {
        let excess = excess(forest, member, &counted, &mut span, |l| {
            l.filter(|&s| s == (0, 0))
        });
        let Some((lo, hi)) = excess else {
            return Err(RefutationFault::NotApplicable {
                invariant: INVARIANT,
                why: "a ⊤ absorbs, or copies of the atom's literals add any number",
            });
        };
        (least, most) = (least + lo, most + hi);
    }
    if (least, most) != (i64::from(stated.0), i64::from(stated.1)) {
        Err(RefutationFault::Mismatch)
    } else if least <= 0 && 0 <= most {
        Err(RefutationFault::Holds)
    } else {
        Ok(())
    }
}

/// Checks that the atom of `one_sided` has the stated excess interval
/// outside the goal's exponentials, that its literals below them all have
/// the stated sign, and that copies of that sign cannot bring the excess
/// to zero. In a sequent with a quantifier, the count is an invariant
/// only where no atom of the atom's predicate has a bound variable.
fn check_one_sided(
    forest: &crate::linear::occurrences::Forest,
    goal: &[OccId],
    mode: Mode,
    one_sided: &OneSided,
) -> Result<(), RefutationFault> {
    let atom = one_sided.atom;
    let sequent = forest.sequent();
    if atom.index() >= sequent.atom_names().len() {
        return Err(RefutationFault::Mismatch);
    }
    if predicate_open(sequent, atom) {
        return Err(RefutationFault::NotApplicable {
            invariant: "one-sided count of the atom",
            why: "an atom of its predicate has a bound variable, whose instance may be it",
        });
    }
    check_copies(
        forest,
        goal,
        mode,
        |o| forest.atom(o) == Some(atom),
        (one_sided.least, one_sided.most, one_sided.copied),
    )
}

/// Checks that the literals of the predicate of `one_sided`, every atom of
/// it counted, have the stated excess interval outside the goal's
/// exponentials and the stated sign below them, and that copies cannot
/// bring the excess to zero.
fn check_one_sided_symbol(
    forest: &crate::linear::occurrences::Forest,
    goal: &[OccId],
    mode: Mode,
    one_sided: &OneSidedSymbol,
) -> Result<(), RefutationFault> {
    let sequent = forest.sequent();
    if one_sided.symbol.index() >= sequent.symbols().len() {
        return Err(RefutationFault::Mismatch);
    }
    check_copies(
        forest,
        goal,
        mode,
        |o| {
            forest
                .atom(o)
                .and_then(|a| sequent.atom_predicate(a))
                .is_some_and(|(s, _)| s == one_sided.symbol)
        },
        (one_sided.least, one_sided.most, one_sided.copied),
    )
}

/// Checks a one-sided count of the literals `counted` picks.
///
/// Outside the goal's exponentials, their excess interval must be the
/// stated one: `⊗` and `⅋` add their sides', `&` and `⊕` take the hull,
/// and a `!` or `?` counts nothing. Every one of them below an exponential
/// of the goal must have the stated sign `copied`. The interval must
/// exclude zero on the side that copies of that sign cannot reach: the
/// least above zero for `a`, the greatest below zero for `~a`. Then the
/// weighted test, with the weight `1` on `a` (`−1` for `~a`) and nothing
/// dead, must accept the goal: that is the theorem the count rests on.
/// The mode must not be affine, and the goal must have no `⊤`.
fn check_copies(
    forest: &crate::linear::occurrences::Forest,
    goal: &[OccId],
    mode: Mode,
    counted: impl Fn(OccId) -> bool,
    (least, most, copied): (i32, i32, Sign),
) -> Result<(), RefutationFault> {
    use crate::linear::sequents::Kind::*;
    const INVARIANT: &str = "one-sided count of the atom";
    if mode.affine {
        return Err(RefutationFault::NotApplicable {
            invariant: INVARIANT,
            why: "weakening discards literals",
        });
    }
    // Per occurrence, the least and the greatest excess outside the
    // exponentials, or `None` where a `⊤` absorbs. They are sums of ±1
    // over a member's subtree of fewer than 2³² occurrences, so within an
    // `i64` per member. The sum over the members is checked.
    let mut span: Vec<Option<(i64, i64)>> = vec![None; forest.len()];
    // Which signs of the counted literals lie below an exponential.
    let mut below = [false; 2];
    let (mut lo, mut hi) = (0i64, 0i64);
    for &member in goal {
        // An occurrence's subtree is the ids that follow it, so the end of
        // the outermost exponential met so far says what lies below one.
        let mut inside = 0;
        for o in forest.subtree(member) {
            if o.index() < inside
                && let Some(sign) = forest.sign(o)
                && counted(o)
            {
                below[sign as usize] = true;
            }
            if matches!(forest.kind(o), Bang | Quest) {
                inside = inside.max(o.index() + forest.size(o) as usize);
            }
        }
        let excess = excess(forest, member, &counted, &mut span, |l| l.map(|_| (0, 0)));
        let Some((a, b)) = excess else {
            return Err(RefutationFault::NotApplicable {
                invariant: INVARIANT,
                why: "a ⊤ absorbs",
            });
        };
        let (Some(a), Some(b)) = (lo.checked_add(a), hi.checked_add(b)) else {
            return Err(RefutationFault::Mismatch);
        };
        (lo, hi) = (a, b);
    }
    match below {
        [true, true] => {
            return Err(RefutationFault::NotApplicable {
                invariant: INVARIANT,
                why: "copies add literals of both signs",
            });
        }
        [positive, negative]
            if positive && copied == Sign::Dual || negative && copied == Sign::Atom =>
        {
            return Err(RefutationFault::Mismatch);
        }
        _ => {}
    }
    if (lo, hi) != (i64::from(least), i64::from(most)) {
        return Err(RefutationFault::Mismatch);
    }
    let excluded = match copied {
        Sign::Atom => lo > 0,
        Sign::Dual => hi < 0,
    };
    if !excluded {
        return Err(RefutationFault::Holds);
    }
    let weight = match copied {
        Sign::Atom => 1,
        Sign::Dual => -1,
    };
    let weights = super::weighted::Weights {
        weight: &|o| if counted(o) { weight } else { 0 },
        dead: &|_| false,
        sign: Sign::Atom,
        weakening: false,
    };
    super::weighted::overweight(forest, goal, &weights)
}

/// Checks that the goal has the formulas and connectives `equation`
/// states, which fail the count equation, where it is an invariant.
fn check_equation(
    forest: &crate::linear::occurrences::Forest,
    goal: &[OccId],
    mode: Mode,
    equation: &Equation,
) -> Result<(), RefutationFault> {
    use crate::linear::sequents::Kind::*;
    const INVARIANT: &str = "count equation";
    if mode.affine {
        return Err(RefutationFault::NotApplicable {
            invariant: INVARIANT,
            why: "weakening adds formulas",
        });
    }
    let (mut tensors, mut pars, mut ones, mut bottoms) = (0u64, 0u64, 0u64, 0u64);
    for &member in goal {
        for o in forest.subtree(member) {
            match forest.kind(o) {
                Tensor => tensors += 1,
                Par => pars += 1,
                One => ones += 1,
                Bot => bottoms += 1,
                // A quantifier's rule changes no count.
                Atom | DualAtom | Forall | Exists => {}
                Top | Zero | With | Plus | Bang | Quest => {
                    return Err(RefutationFault::NotApplicable {
                        invariant: INVARIANT,
                        why: "it holds in the multiplicative fragment with units alone",
                    });
                }
            }
        }
    }
    // Counts over the goal's members, each a subtree of fewer than 2³²
    // occurrences. For any goal a memory holds they are far within an
    // `i64`.
    let needed = tensors as i64 - pars as i64 - ones as i64 + bottoms as i64 + 2;
    let formulas = goal.len() as u64;
    let stated = Equation {
        formulas,
        needed,
        tensors,
        pars,
        ones,
        bottoms,
        mix: mode.mix,
    };
    if *equation != stated {
        return Err(RefutationFault::Mismatch);
    }
    let holds = if mode.mix {
        formulas as i64 >= needed
    } else {
        formulas as i64 == needed
    };
    if holds {
        Err(RefutationFault::Holds)
    } else {
        Ok(())
    }
}
