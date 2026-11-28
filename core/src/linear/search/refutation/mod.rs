// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Why a sequent is unprovable, and the checks that verify it. A
//! [`Refutation`] is the reason a search or a test of the sequent gives.
//! A [`Disproof`] holds it with the sequent, the goal and the mode it is
//! about, and [`Disproof::check`] verifies it.
//!
//! The checks are the linear suite's second trust boundary beside the
//! proof checker: the counts', the assignment's and the count equation's
//! (`check`), the saturated set's (`saturated`), the basis's (`basis`) and
//! the weighted count's (`weighted`). Nothing here shares code with a
//! search: each check reads the goal again by code of its own. `phrase`
//! holds the wording a refutation is written in.

/// The check of a refutation by a basis, apart from the backward search
/// that found it.
pub(super) mod basis;
/// [`Disproof::check`], the faults it answers, and the checks of the
/// counts, an assignment and the count equation.
mod check;
/// The phrases a refutation and a disproof are written in.
mod phrase;
/// The check of a refutation by a saturated set, apart from the search.
pub(super) mod saturated;
/// The weighted count that certifies a one-sided count and a Horn
/// program's state equation, apart from the search.
pub(crate) mod weighted;

pub(crate) use check::Failed;
pub use check::RefutationFault;

use crate::linear::fragment::Mode;
use crate::linear::occurrences::{Member, OccId, Sign};
use crate::linear::sequents::{Atom, Sequent, Symbol};
#[cfg(doc)]
use std::fmt::Display;

/// Why a sequent is unprovable.
///
/// Most refutations rest on a search that tried every way to a proof.
/// Others rest on the sequent alone: its literals or its connectives rule
/// out a proof, and the refutation says which. The focused engine tests
/// these on every sequent it searches. The Horn engine may refute a Horn
/// program by its state equation, whatever its search found.
///
/// A refutation means something only with its sequent, goal and mode,
/// which a [`Disproof`] holds. Its [`Display`] writes atoms by number
/// (`#0`); a disproof's writes them by name. For a goal other than the
/// sequent's roots, the "one-sided sequent" a count speaks of is the goal.
/// Its members are counted as written, even where they lie below an
/// exponential of the sequent. A disproof of a classical sequent written
/// with formulas left of `⊢` speaks of its sides instead: `a` counts
/// right of `⊢` and `~a` left of it, as in `a on the left has no a on the
/// right to meet in an axiom`.
///
/// In JSON (feature `serialize`), a refutation is tagged by `kind`. It is
/// the `refutation` of an unprovable outcome and of a disproof:
///
/// - `{"kind": "exhausted"}`;
/// - `{"kind": "unbalanced", "atom", "least", "most"}`, the atom by its
///   index into the sequent beside it;
/// - `{"kind": "one_sided", "atom", "least", "most", "copied"}`, with
///   `copied` the literal `"atom"` or `"dual"`;
/// - `{"kind": "equation", "formulas", "needed", "tensors", "pars",
///   "ones", "bottoms", "mix"}`;
/// - `{"kind": "state_equation", "atoms": [[atom, weight], …], "clauses":
///   [[occurrence, weight], …], "dropped": [occurrence, …]}`;
/// - `{"kind": "classical", "assignment": [true, false, …]}`, a value per
///   atom of the sequent, by index;
/// - `{"kind": "saturated", "positive": ["atom", "dual", …], "sequents":
///   [{"linear": [occurrence, …], "unrestricted": [occurrence, …],
///   "absorbs": false}, …]}`;
/// - `{"kind": "unbalanced_symbol", "symbol", "least", "most"}` and
///   `{"kind": "one_sided_symbol", "symbol", "least", "most", "copied"}`,
///   the predicate by its index into the sequent's symbols;
/// - `{"kind": "basis", "body": "atom" | "dual", "classes": [[occurrence,
///   …], …], "markings": [{"atoms": [[atom, count], …], "tickets":
///   [[class, count], …]}, …]}`.
///
/// It reads back the same way. A reader refuses a kind it does not know:
/// read as another kind, it would claim a certificate it does not hold.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Refutation {
    /// Every way to prove the sequent was tried and failed.
    Exhausted,
    /// The literals of an atom cannot all meet their duals in axioms.
    Unbalanced(Unbalanced),
    /// The literals of an atom cannot all meet their duals in axioms, even
    /// though copies may add some, all of one sign.
    OneSided(OneSided),
    /// The count equation of the multiplicatives fails.
    Equation(Equation),
    /// The goal is a Horn program whose state equation has no solution.
    StateEquation(StateEquation),
    /// The classical reading of the goal is false under an assignment.
    Classical(Classical),
    /// A forward search saturated. Its sequents are closed under the
    /// rules, and the goal needs one they do not give.
    Saturated(Saturated),
    /// The literals of a predicate cannot all meet their duals in axioms,
    /// counted over all its atoms. One of its atoms has a bound variable.
    UnbalancedSymbol(UnbalancedSymbol),
    /// The literals of a predicate cannot all meet their duals in axioms,
    /// even though copies may add some, all of one sign.
    OneSidedSymbol(OneSidedSymbol),
    /// The goal is a Horn program, and a backward search from its target
    /// saturated: the markings from which a firing sequence covers the
    /// target are those at least one of a finite basis, and the initial
    /// marking is not one of them.
    Basis(Basis),
}

/// The literals of a predicate, which cannot all meet their duals in
/// axioms. They are counted as [`Unbalanced`] counts an atom's, but over
/// every atom of the predicate together.
///
/// One of the predicate's atoms has a bound variable, so an instance of
/// it may be any atom of the predicate. An axiom joins only literals of
/// one predicate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UnbalancedSymbol {
    /// The predicate, whose name the sequent has.
    pub symbol: Symbol,
    /// The least excess of positive over negative literals.
    pub least: i32,
    /// The greatest excess.
    pub most: i32,
}

/// The literals of a predicate, counted as [`UnbalancedSymbol`] counts
/// them, some of which lie below a `?` or `!`. Copies may add any number
/// of those, all of one sign, as [`OneSided`] says of an atom.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OneSidedSymbol {
    /// The predicate, whose name the sequent has.
    pub symbol: Symbol,
    /// The least excess of positive over negative literals outside the
    /// exponentials.
    pub least: i32,
    /// The greatest excess.
    pub most: i32,
    /// The sign of every literal of the predicate below an exponential.
    pub copied: Sign,
}

/// The literals of an atom, which cannot all meet their duals in axioms.
///
/// Whichever additive alternatives a proof takes, the positive literal
/// occurs between `least` and `most` times more often than the negative
/// one (less often, where these are negative), and never equally often.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Unbalanced {
    /// The atom, whose name the sequent has.
    pub atom: Atom,
    /// The least excess of positive over negative literals.
    pub least: i32,
    /// The greatest excess.
    pub most: i32,
}

/// The literals of an atom, which cannot all meet their duals in axioms
/// although some lie below a `?` or `!`, where copies may add any number
/// of them.
///
/// Every literal of the atom below an exponential has the sign `copied`,
/// so copies add only that literal. Outside the exponentials, that
/// literal already outnumbers the other one. Its excess, counted as
/// [`Unbalanced`] counts it, lies between `least` and `most`: `least` is
/// above zero where `copied` is the atom, and `most` is below zero where
/// it is the negation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OneSided {
    /// The atom, whose name the sequent has.
    pub atom: Atom,
    /// The least excess of positive over negative literals outside the
    /// exponentials.
    pub least: i32,
    /// The greatest excess.
    pub most: i32,
    /// The sign of every literal of the atom below an exponential.
    pub copied: Sign,
}

/// The count equation of the multiplicatives, which the sequent fails.
///
/// A provable one-sided sequent of MLL with units has exactly `tensors −
/// pars − ones + bottoms + 2` formulas, and at least that many with Mix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Equation {
    /// How many formulas the sequent has.
    pub formulas: u64,
    /// How many formulas the equation asks for. It is below zero where the
    /// `⅋` and the `1` outnumber the rest.
    pub needed: i64,
    /// How many `⊗` the sequent has.
    pub tensors: u64,
    /// How many `⅋` it has.
    pub pars: u64,
    /// How many `1` it has.
    pub ones: u64,
    /// How many `⊥` it has.
    pub bottoms: u64,
    /// Whether the mode allows Mix.
    pub mix: bool,
}

/// Weights under which a Horn program's state equation fails, so that no
/// sequence of firings reaches the goal.
///
/// The weights are for the program's Petri net, every place included.
/// Weigh the tokens of each atom, and the ticket of each clause used
/// once, as given, zero where not listed. Then no clause that can fire
/// raises the weighted count, while the goal asks it raised (Farkas'
/// lemma). In affine mode, with weights that are not negative, no
/// sequence of firings covers the goal either. A clause with an input
/// that no marking reachable from the start holds can never fire, and is
/// dropped from the inequalities.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct StateEquation {
    /// The atoms whose tokens weigh something, in the order of the
    /// sequent's atoms, with their weights.
    pub atoms: Vec<(Atom, i64)>,
    /// The clauses used once whose ticket weighs something, by
    /// occurrence, with their weights.
    pub clauses: Vec<(OccId, i64)>,
    /// The clauses that can never fire, by occurrence.
    pub dropped: Vec<OccId>,
}

/// An assignment of truth values to the atoms under which the classical
/// reading of the goal is false.
///
/// The classical reading erases `!` and `?`, reads `⊗` and `&` as "and",
/// `⅋` and `⊕` as "or", `1` and `⊤` as true, `⊥` and `0` as false, `~a`
/// as "not a", and the goal as the disjunction of its formulas. In every
/// mode, a provable sequent's reading is true under every assignment.
/// Each rule, Mix and weakening included, derives a tautology from
/// tautologies, and an intuitionistic sequent's one-sided form reads as
/// its implication. So the assignment proves the goal unprovable.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Classical {
    /// The value of each atom of the sequent, by index. An atom the goal
    /// does not contain has one too.
    pub assignment: Vec<bool>,
}

/// The sequents a forward search kept when it saturated, which show that
/// the goal has no proof.
///
/// The sequents are stable ([`Stable`]) and closed under the rules of the
/// focused calculus. They leave underived a stable sequent of the goal's
/// asynchronous phase, the part of a proof where only invertible rules
/// apply. A rule is a focus on a positive formula, or a copy of a `?`
/// formula's subformula, with the sides of its `⊕`, the partners of its
/// axioms and the asynchronous phases of what it releases. Applied to any
/// of the sequents, every rule derives a sequent that a sequent of the set
/// stands for, or one that no proof holds: a member twice, or two members
/// that a `⊗`, `&` or `⊕` separates, outside the subformulas of `?`. Mix
/// does the same where the mode has it. So, by induction, every stable
/// sequent of a focused proof is one that a sequent of the set stands for,
/// and some stable sequent of the goal's phase is one that none stands
/// for.
///
/// The rules are those of the polarity `positive` names. Focusing is as
/// complete under it as under any other. The certificate is as large as
/// the set, which the check reads whole and applies every rule to.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Saturated {
    /// The positive literal of each atom of the sequent, by index:
    /// [`Sign::Atom`] where the atom is positive, [`Sign::Dual`] where its
    /// negation is.
    pub positive: Vec<Sign>,
    /// The sequents.
    pub sequents: Vec<Stable>,
}

/// A stable sequent of a saturated set, as occurrences of the sequent's
/// forest. Its linear zone holds positive formulas and negative literals,
/// and its unrestricted zone holds subformulas of `?`.
///
/// It stands for the sequents with exactly this linear zone and an
/// unrestricted zone that holds this one. Where it `absorbs` (a `⊤`
/// above), or in affine mode, it also stands for those with a larger
/// linear zone. A member in output position is then absorbed only where
/// the zone has none, and never weakened in. Every sequent of a proof
/// holds the subformulas of the `?` the goal reaches through `⅋` alone,
/// so they may be left out of the unrestricted zone.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Stable {
    /// The linear zone, sorted, with repeats.
    pub linear: Vec<OccId>,
    /// The unrestricted zone, sorted, without repeats.
    pub unrestricted: Vec<OccId>,
    /// Whether a `⊤` above absorbs any further linear member.
    pub absorbs: bool,
}

impl Stable {
    /// Returns the stable sequent of the zones given.
    pub fn new(linear: Vec<OccId>, unrestricted: Vec<OccId>, absorbs: bool) -> Self {
        Self {
            linear,
            unrestricted,
            absorbs,
        }
    }
}

impl Saturated {
    /// Returns the saturated set of `sequents` under the polarity
    /// `positive`.
    pub fn new(positive: Vec<Sign>, sequents: Vec<Stable>) -> Self {
        Self { positive, sequents }
    }
}

/// The basis of the markings from which a Horn program's target can be
/// covered, as a backward search left it, which shows the goal
/// unprovable.
///
/// The goal is read as a Petri net. A place is an atom, or a class of
/// interchangeable clauses used once, which holds a ticket per unused
/// clause. A clause is a transition: it takes its body's literals (and a
/// ticket of its class, when it is used once) and gives its head's. The
/// initial marking is the goal's markings with every ticket; the target
/// is the goal's own body. A proof of the goal fires a sequence of
/// clauses from the initial marking to a marking at least the target, in
/// every mode: weakening only drops tokens.
///
/// The markings at least one of the basis hold every marking at least the
/// target, and with a marking `m` every marking from which a clause leads
/// to `m`. So they hold every marking of such a sequence, the initial one
/// included, unless none is at most it. The basis is all the backward
/// search kept, smaller markings and the ones they made redundant.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Basis {
    /// The sign of the bodies' literals and the goal's: [`Sign::Atom`]
    /// where they are atoms, [`Sign::Dual`] where they are negated. Heads
    /// and markings have the other.
    pub body: Sign,
    /// The goal's clauses used once, by occurrence, in classes of clauses
    /// with the same body and head. A marking counts each class's
    /// tickets.
    pub classes: Vec<Vec<OccId>>,
    /// The markings of the basis.
    pub markings: Vec<Marking>,
}

/// A marking of a [`Basis`]: tokens on atoms and tickets of classes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Marking {
    /// The atoms with tokens, by increasing index, each with its count,
    /// above zero.
    pub atoms: Vec<(Atom, u32)>,
    /// The classes with tickets, by increasing index into
    /// [`Basis::classes`], each with its count, above zero.
    pub tickets: Vec<(u32, u32)>,
}

impl Basis {
    /// Returns the basis of `markings`, over the clauses used once in
    /// `classes`, for bodies of the sign `body`.
    pub fn new(body: Sign, classes: Vec<Vec<OccId>>, markings: Vec<Marking>) -> Self {
        Self {
            body,
            classes,
            markings,
        }
    }
}

impl Marking {
    /// Returns the marking of the tokens `atoms` and the tickets
    /// `tickets`.
    pub fn new(atoms: Vec<(Atom, u32)>, tickets: Vec<(u32, u32)>) -> Self {
        Self { atoms, tickets }
    }
}

impl Refutation {
    /// The words of the refutations, as [`Refutation::name`] gives them and
    /// the JSON form's `kind` writes them.
    pub const NAMES: &'static [&'static str] = &[
        "exhausted",
        "unbalanced",
        "one_sided",
        "equation",
        "state_equation",
        "classical",
        "saturated",
        "unbalanced_symbol",
        "one_sided_symbol",
        "basis",
    ];

    /// Returns the refutation's word, its `kind` in the JSON form.
    pub const fn name(&self) -> &'static str {
        match self {
            Refutation::Exhausted => "exhausted",
            Refutation::Unbalanced(_) => "unbalanced",
            Refutation::OneSided(_) => "one_sided",
            Refutation::Equation(_) => "equation",
            Refutation::StateEquation(_) => "state_equation",
            Refutation::Classical(_) => "classical",
            Refutation::Saturated(_) => "saturated",
            Refutation::UnbalancedSymbol(_) => "unbalanced_symbol",
            Refutation::OneSidedSymbol(_) => "one_sided_symbol",
            Refutation::Basis(_) => "basis",
        }
    }
}

/// A refutation with what it refutes: the sequent, the goal within it if
/// that is not the sequent's roots, and the mode. A search's
/// [`Verdict::Unprovable`](super::Verdict::Unprovable) carries one, as a
/// [`Verdict::Proved`](super::Verdict::Proved) carries a proof.
///
/// Its [`Display`] says why the goal is unprovable, with every atom by
/// its name. [`check`](Self::check) verifies a disproof read from
/// elsewhere.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::{Mode, Options, Refutation, Sequent, Verdict, prove};
///
/// let sequent: Sequent = "a |- b".parse()?;
/// let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
/// let Verdict::Unprovable(disproof) = outcome.verdict else {
///     panic!("a |- b has no proof");
/// };
/// // `~a` has no `a` to meet in an axiom.
/// assert!(matches!(disproof.refutation(), Refutation::Unbalanced(_)));
/// println!("{disproof}");
/// disproof.check()?;
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # JSON
///
/// With the feature `serialize`, a disproof is the document `{"version":
/// 1, "sequent": …, "mode": …, "refutation": …, "goal": […]}`. The
/// refutation has the form [`Refutation`] describes, and `goal` lists the
/// goal's members only for a goal other than the roots. An unprovable
/// outcome writes the same keys, so it reads back as its disproof.
///
/// Reading counts the sequent against the limits. It checks that every
/// atom and occurrence the refutation and the goal name is in the
/// sequent (`Error::IndexOutOfBounds` otherwise), but not the refutation
/// itself: that is what [`check`](Self::check) does. A refutation kind
/// the reader does not know is refused (`Error::Json`, kind malformed).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Disproof {
    /// The sequent.
    sequent: Sequent,
    /// The goal refuted, `None` for the sequent's roots.
    goal: Option<Box<[Member]>>,
    /// The mode it is refuted in.
    mode: Mode,
    /// Why it is unprovable.
    refutation: Refutation,
}

impl Disproof {
    /// Returns the disproof of a sequent in a mode.
    pub fn new(sequent: Sequent, mode: Mode, refutation: Refutation) -> Self {
        Self {
            sequent,
            goal: None,
            mode,
            refutation,
        }
    }

    /// Returns the disproof as one of a goal within the sequent.
    #[must_use]
    pub(crate) fn of_goal(self, goal: Box<[Member]>) -> Self {
        Self {
            goal: Some(goal),
            ..self
        }
    }

    /// Returns the sequent.
    pub fn sequent(&self) -> &Sequent {
        &self.sequent
    }

    /// Returns the goal refuted, `None` for the sequent's roots.
    pub fn goal(&self) -> Option<&[Member]> {
        self.goal.as_deref()
    }

    /// Returns the mode it is refuted in.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Returns why the goal is unprovable.
    pub fn refutation(&self) -> &Refutation {
        &self.refutation
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::linear::search::{Options, Verdict, prove};

    /// Returns the disproof the default search gives `input` in `mode`.
    fn refuted(input: &str, mode: Mode) -> Disproof {
        let sequent: Sequent = input.parse().unwrap();
        match prove(&sequent, mode, &Options::default()).unwrap().verdict {
            Verdict::Unprovable(disproof) => *disproof,
            other => panic!("{input}: {other:?}"),
        }
    }

    /// Returns the disproof of `input` in `mode` for `refutation`.
    fn claimed(input: &str, mode: Mode, refutation: Refutation) -> Disproof {
        Disproof::new(input.parse().unwrap(), mode, refutation)
    }

    /// Returns the fault the check finds, `None` where it accepts.
    fn fault(disproof: &Disproof) -> Option<RefutationFault> {
        match disproof.check() {
            Ok(()) => None,
            Err(crate::Error::Refutation(fault)) => Some(fault),
            Err(other) => panic!("{other:?}"),
        }
    }

    /// The check accepts the search's assignments, balances and equations,
    /// and rejects every one that does not hold where the claim is: a
    /// satisfying or short assignment, numbers not the goal's or that
    /// allow a proof, a mode or connectives that defeat the invariant, and
    /// the refutations that rest on the search.
    #[test]
    fn a_wrong_refutation_is_rejected() {
        let classical = Mode::CLASSICAL;
        let assignment = |values: &[bool]| {
            Refutation::Classical(Classical {
                assignment: values.to_vec(),
            })
        };
        let found = refuted("|- a & b, ~a", classical);
        assert_eq!(fault(&found), None);
        assert_eq!(fault(&refuted("a, a -o b |- b * a", classical)), None);
        let wrong = claimed("|- a & b, ~a", classical, assignment(&[true, true]));
        assert_eq!(fault(&wrong), Some(RefutationFault::Satisfied));
        let short = claimed("|- a & b, ~a", classical, assignment(&[true]));
        assert_eq!(
            fault(&short),
            Some(RefutationFault::Assignment {
                atoms: 2,
                values: 1
            })
        );
        // In every mode the reading holds, intuitionistic too.
        let ill = refuted("a + b |- a", Mode::INTUITIONISTIC);
        assert!(matches!(ill.refutation(), Refutation::Classical(_)));
        assert_eq!(fault(&ill), None);

        let unbalanced = |atom: u32, least: i32, most: i32| {
            Refutation::Unbalanced(Unbalanced {
                atom: Atom::new(atom),
                least,
                most,
            })
        };
        let found = refuted("|- (a * a) + (a * a * a), ~a", classical);
        assert_eq!(found.refutation(), &unbalanced(0, 1, 2));
        assert_eq!(fault(&found), None);
        let mix = refuted("a |- b", classical.with_mix());
        assert!(matches!(mix.refutation(), Refutation::Unbalanced(_)));
        assert_eq!(fault(&mix), None);
        for (input, mode, refutation, expected) in [
            (
                "|- a, ~a",
                classical,
                unbalanced(0, 0, 0),
                RefutationFault::Holds,
            ),
            (
                "|- a, ~a",
                classical,
                unbalanced(0, 1, 1),
                RefutationFault::Mismatch,
            ),
            (
                "|- a, a",
                classical,
                unbalanced(1, 2, 2),
                RefutationFault::Mismatch,
            ),
            (
                "|- a + ~a, a",
                classical,
                unbalanced(0, 1, 1),
                RefutationFault::Mismatch,
            ),
            (
                "|- a + ~a, a",
                classical,
                unbalanced(0, 0, 2),
                RefutationFault::Holds,
            ),
        ] {
            assert_eq!(
                fault(&claimed(input, mode, refutation)),
                Some(expected),
                "{input}"
            );
        }
        for (input, mode) in [
            ("|- a, a", classical.with_affine()),
            ("|- a, a, top", classical),
            ("|- ?a, a", classical),
        ] {
            assert!(
                matches!(
                    fault(&claimed(input, mode, unbalanced(0, 2, 2))),
                    Some(RefutationFault::NotApplicable { .. })
                ),
                "{input}"
            );
        }

        let found = refuted("|- a par b, ~a, ~b", classical);
        let Refutation::Equation(equation) = *found.refutation() else {
            panic!("{found:?}");
        };
        assert_eq!(fault(&found), None);
        let stated = |input: &str, mode: Mode, equation: Equation| {
            fault(&claimed(input, mode, Refutation::Equation(equation)))
        };
        let with_mix = Equation {
            mix: true,
            ..equation
        };
        assert_eq!(
            stated("|- a par b, ~a, ~b", classical, with_mix),
            Some(RefutationFault::Mismatch)
        );
        assert_eq!(
            stated("|- a par b, ~a, ~b", classical.with_mix(), with_mix),
            Some(RefutationFault::Holds)
        );
        let holds = Equation {
            formulas: 2,
            needed: 2,
            tensors: 0,
            pars: 0,
            ones: 0,
            bottoms: 0,
            mix: false,
        };
        assert_eq!(
            stated("|- a, ~a", classical, holds),
            Some(RefutationFault::Holds)
        );
        for (input, mode) in [
            ("|- a par b, ~a, ~b", classical.with_affine()),
            ("|- a & b, ~a, ~b", classical),
            ("|- ?(a par b), ~a, ~b", classical),
        ] {
            assert!(
                matches!(
                    stated(input, mode, equation),
                    Some(RefutationFault::NotApplicable { .. })
                ),
                "{input}"
            );
        }

        let exhausted = claimed("|- a & b, ~a", classical, Refutation::Exhausted);
        let error = exhausted.check().unwrap_err();
        assert_eq!(
            (error.code(), error.kind()),
            ("uncertified", crate::ErrorKind::Unsupported)
        );
        let error = wrong.check().unwrap_err();
        assert_eq!(
            (error.code(), error.kind()),
            ("invalid_refutation", crate::ErrorKind::Invalid)
        );
    }

    /// The check accepts the search's one-sided counts and state
    /// equations, in each mode they are given in, and refuses a planted
    /// wrong one of each kind: numbers not the goal's, copies of the other
    /// sign, an interval that copies can bring to zero, weights that leave
    /// a contribution at zero, a goal that is no Horn program, and the
    /// modes where the invariant does not hold.
    #[test]
    fn a_wrong_count_with_copies_or_state_equation_is_refused() {
        let (classical, ill) = (Mode::CLASSICAL, Mode::INTUITIONISTIC);
        for (input, mode, kind) in [
            ("!d, !((c * d * d) -o 1) |- c * c", classical, "one_sided"),
            ("!d, !((c * d * d) -o 1) |- c * c", ill, "one_sided"),
            ("|- ~a * ~a, ?(~a * b), ~b", classical, "one_sided"),
            (
                "!(a -o a * a), !(a * c -o b * c), a |- b",
                ill,
                "state_equation",
            ),
            (
                "!(a -o a * a), !(a * c -o b * c), a |- b",
                classical.with_mix(),
                "state_equation",
            ),
            // Bodies written negative, with a clause that never fires.
            (
                "|- a, ?(~a * (a par a)), ?((~a * ~c) * b), ~b",
                classical,
                "state_equation",
            ),
            // A clause used once, whose ticket weighs.
            (
                "a, a -o b, !(b -o b * c), !(b -o d) |- b * d",
                ill,
                "state_equation",
            ),
            (
                "!(a -o a * c), !(a -o b), a |- a * b",
                ill.with_affine(),
                "state_equation",
            ),
            (
                "|- a, ?(~a * (a par c)), ?(~a * b), ~a * ~b",
                classical.with_affine(),
                "state_equation",
            ),
        ] {
            let found = refuted(input, mode);
            assert_eq!(found.refutation().name(), kind, "{input}");
            assert_eq!(fault(&found), None, "{input}");
        }

        let one_sided = |atom: u32, least: i32, most: i32, copied: Sign| {
            Refutation::OneSided(OneSided {
                atom: Atom::new(atom),
                least,
                most,
                copied,
            })
        };
        // The atom `c` is the second: its copies add `c`.
        let counted = "!d, !((c * d * d) -o 1) |- c * c";
        for (input, mode, refutation, expected) in [
            (
                counted,
                classical,
                one_sided(1, 1, 1, Sign::Atom),
                RefutationFault::Mismatch,
            ),
            (
                counted,
                classical,
                one_sided(1, 2, 2, Sign::Dual),
                RefutationFault::Mismatch,
            ),
            // The copies add `~c`: only a deficit of `~c` would refute.
            (
                "|- c * c, ?~c",
                classical,
                one_sided(0, 2, 2, Sign::Dual),
                RefutationFault::Holds,
            ),
            (
                "|- ~c, ~c, ?c",
                classical,
                one_sided(0, -2, -2, Sign::Dual),
                RefutationFault::Mismatch,
            ),
            (
                "|- ~c, ~c, ?c",
                classical,
                one_sided(0, -2, -2, Sign::Atom),
                RefutationFault::Holds,
            ),
        ] {
            let planted = claimed(input, mode, refutation);
            assert_eq!(fault(&planted), Some(expected), "{input}");
        }
        for (input, mode, atom) in [
            (counted, classical.with_affine(), 1),
            ("|- c * c, ?~c, ?c", classical, 0),
            ("|- c * c, ?~c, top", classical, 0),
        ] {
            assert!(
                matches!(
                    fault(&claimed(input, mode, one_sided(atom, 2, 2, Sign::Atom))),
                    Some(RefutationFault::NotApplicable { .. })
                ),
                "{input}"
            );
        }

        let weighed = |atoms: &[(u32, i64)]| {
            Refutation::StateEquation(StateEquation {
                atoms: atoms.iter().map(|&(a, w)| (Atom::new(a), w)).collect(),
                clauses: Vec::new(),
                dropped: Vec::new(),
            })
        };
        let net = "!(a -o a * a), !(a * c -o b * c), a |- b";
        let found = refuted(net, ill);
        let Refutation::StateEquation(equation) = found.refutation() else {
            panic!("{found:?}");
        };
        assert_eq!(equation.atoms, [(Atom::new(0), -1), (Atom::new(2), 1)]);
        for (input, mode, atoms, expected) in [
            // Found with `a ↦ 2, b ↦ 1, d ↦ 1`; the copies of `b ⊸ d` raise
            // the count with `d ↦ 2`, those of `b ⊸ b ⊗ c` with `c ↦ 1`.
            (
                "a, a -o b, !(b -o b * c), !(b -o d) |- b * d",
                ill,
                &[(0, 1), (1, 1), (3, 2)][..],
                RefutationFault::Holds,
            ),
            (
                "a, a -o b, !(b -o b * c), !(b -o d) |- b * d",
                ill,
                &[(0, 2), (1, 1), (2, 1), (3, 1)][..],
                RefutationFault::Holds,
            ),
            (net, ill, &[(2, 1), (0, -1)][..], RefutationFault::Mismatch),
            (net, ill, &[(0, -1), (7, 1)][..], RefutationFault::Mismatch),
            // The bodies negative, found with `a ↦ 1, b ↦ 1`: with `c ↦ 1`
            // the copies of the first clause raise the count.
            (
                "|- a, ?(~a * (a par c)), ?(~a * b), ~a * ~b",
                classical.with_affine(),
                &[(0, 1), (1, 1), (2, 1)][..],
                RefutationFault::Holds,
            ),
        ] {
            let planted = claimed(input, mode, weighed(atoms));
            assert_eq!(fault(&planted), Some(expected), "{input}");
        }
        for (input, mode) in [
            ("|- a & b, ~a", classical),
            // With weakening the goal has no output, or two.
            ("|- ~a, ~a", classical.with_affine()),
            ("forall x. p(x) |- p(c)", classical),
        ] {
            assert!(
                matches!(
                    fault(&claimed(input, mode, weighed(&[(0, 1)]))),
                    Some(RefutationFault::NotApplicable { .. })
                ),
                "{input}"
            );
        }
    }

    /// A classical disproof of a sequent written with formulas left of
    /// `⊢` speaks of its sides: an atom with no partner on the other side,
    /// an excess counted left and right, and a note where a literal counts
    /// on the other side than its formula. One written `⊢ Γ`, a goal
    /// within the sequent and the JSON form keep the one-sided wording.
    #[test]
    fn a_two_sided_sequent_is_refuted_by_its_sides() {
        let classical = Mode::CLASSICAL;
        for (input, phrase) in [
            (
                "a |- b",
                "a on the left has no a on the right to meet in an axiom",
            ),
            (
                "a, a, a |- a",
                "a occurs 2 more times on the left than on the right, so they cannot all meet in \
                 axioms",
            ),
            (
                "b, ~a |- b",
                "a on the right has no a on the left to meet in an axiom (a negated formula or an \
                 implication's antecedent counts on the other side)",
            ),
            (
                "|- ~a, b",
                "~a occurs 1 more time than a in the one-sided sequent, so they cannot all meet in \
                 axioms",
            ),
            // An equality is named as it is written.
            (
                "a = b, p(a) |- p(b)",
                "a = b on the left has no a = b on the right to meet in an axiom",
            ),
        ] {
            assert_eq!(refuted(input, classical).to_string(), phrase, "{input}");
        }
        let goal = refuted("a |- b", classical).of_goal(Box::new([]));
        assert!(goal.to_string().contains("one-sided"), "{goal}");
    }

    /// An intuitionistic count names the side that has none of the atom,
    /// as the two-sided classical one does, and counts the excess where
    /// both sides have some.
    #[test]
    fn an_intuitionistic_count_names_the_empty_side() {
        let intuitionistic = Mode::INTUITIONISTIC;
        for (input, phrase) in [
            (
                "a -o b |- b -o a",
                "a on the right has no a on the left to meet in an axiom (an implication's \
                 antecedent counts on the other side)",
            ),
            (
                "a |- a * a",
                "a occurs 1 more time on the right than on the left, so they cannot all meet in \
                 axioms (an implication's antecedent counts on the other side)",
            ),
        ] {
            assert_eq!(
                refuted(input, intuitionistic).to_string(),
                phrase,
                "{input}"
            );
        }
    }

    /// The check of a saturated set asks the stop and holds to the work
    /// bound, refusing with no verdict, and builds the forest within the
    /// caller's bound; within the defaults it passes, as `check` does.
    #[test]
    fn the_check_within_asks_the_stop_and_the_bounds() {
        use crate::limits::{Phase, Refusal};
        use crate::linear::search::Engine;
        use crate::{Error, Limits};
        let problem = crate::linear::families::find("contraction")
            .unwrap()
            .instance(8, 1)
            .unwrap();
        let options = Options::default()
            .with_copies(problem.copies)
            .with_engine(Some(Engine::Inverse));
        let Verdict::Unprovable(disproof) = prove(&problem.sequent, problem.mode, &options)
            .unwrap()
            .verdict
        else {
            panic!("contraction/8#1 is unprovable");
        };
        assert!(matches!(disproof.refutation(), Refutation::Saturated(_)));
        disproof.check().unwrap();
        disproof
            .check_within(&Limits::default(), |_| false)
            .unwrap();
        let refused = |limits: &Limits, stop: bool| match disproof.check_within(limits, |_| stop) {
            Err(Error::Refused(refusal)) => refusal,
            other => panic!("{other:?}"),
        };
        assert!(matches!(
            refused(&Limits::default(), true),
            Refusal::Stopped {
                phase: Phase::Check
            }
        ));
        assert!(matches!(
            refused(&Limits::default().with_work(Some(1)), false),
            Refusal::Work { limit: 1 }
        ));
        assert!(matches!(
            refused(&Limits::default().with_occurrences(Some(2)), false),
            Refusal::Occurrences { .. }
        ));
    }

    /// The goal's phase of a saturated set is written under the stop: an
    /// `&` under each `⅋` doubles its holes, `2ⁿ` for `n` pairs, before
    /// any sequent of the set is read (a set with none, as a document may
    /// give, wrote 2²³ holes past a time limit of 1 s).
    #[test]
    fn a_saturated_set_s_goal_phase_asks_the_stop() {
        use crate::limits::Refusal;
        use crate::{Error, Limits};
        let pairs = ["(a & a)"; 18].join(" | ");
        let sequent: Sequent = format!("|- {pairs}").parse().unwrap();
        let set = Saturated::new(vec![Sign::Dual], vec![]);
        let disproof = Disproof::new(sequent, Mode::CLASSICAL, Refutation::Saturated(set));
        let result = disproof.check_within(&Limits::default(), |_| true);
        assert!(
            matches!(result, Err(Error::Refused(Refusal::Stopped { .. }))),
            "{result:?}"
        );
    }

    /// The partners of a focus's axioms are counted work: a tensor of `n`
    /// literals beside `n` duals has `n²` of them, which the check asks its
    /// stop over. With a `b` that has no partner the focus closes nothing,
    /// so nothing after the writing polls.
    #[test]
    fn a_focus_s_partners_are_counted_work() {
        let n = 300;
        let tensor = vec!["a"; n].join(" * ");
        let duals = vec!["~a"; n].join(", ");
        let sequent: Sequent = format!("|- b * {tensor}, {duals}").parse().unwrap();
        let set = Saturated::new(vec![Sign::Atom, Sign::Atom], vec![]);
        let disproof = Disproof::new(sequent, Mode::CLASSICAL, Refutation::Saturated(set));
        let mut polls = 0;
        let result = disproof.check_within(&crate::Limits::default(), |_| {
            polls += 1;
            false
        });
        assert!(polls >= 1, "{polls} polls over 90 000 partners: {result:?}");
    }
}
