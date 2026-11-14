// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Why a sequent is unprovable. A [`Refutation`] is the reason a search
//! or a test of the sequent gives. A [`Disproof`] holds it with the
//! sequent, the goal and the mode it is about.

use crate::linear::fragment::Mode;
use crate::linear::occurrences::{Member, OccId, Sign};
use crate::linear::sequents::{Atom, Sequent, Symbol, Term};
use std::fmt::{Display, Formatter, Result as FmtResult};

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
///   the predicate by its index into the sequent's symbols.
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
        }
    }

    /// Writes the refutation as a phrase. Each atom and predicate is
    /// written by its name in `sequent`, or by its number where there is
    /// no sequent. The literals of a count are spoken of in the `sides`
    /// the sequent is read with.
    fn write(&self, f: &mut Formatter<'_>, sequent: Option<&Sequent>, sides: Sides) -> FmtResult {
        let intuitionistic = sides == Sides::Intuitionistic;
        // The wording of a count: two-sided where the sequent was written
        // with hypotheses, with where the counted literals lie.
        let words = |counted: &dyn Fn(Atom) -> bool, least: i32| match (sides, sequent) {
            (Sides::Two, Some(sequent)) => Words::Two(placed(sequent, counted, least > 0)),
            (Sides::Intuitionistic, _) => Words::Intuitionistic,
            (Sides::One | Sides::Two, _) => Words::One,
        };
        let of_symbol = |symbol: Symbol| {
            move |a: Atom| {
                sequent
                    .and_then(|s| s.atom_predicate(a))
                    .is_some_and(|(s, _)| s == symbol)
            }
        };
        let names = sequent.map(Sequent::atom_names);
        let name = |atom: Atom| match names.and_then(|names| names.get(atom.index())) {
            Some(name) => name.clone(),
            None => format!("#{}", atom.get()),
        };
        // A predicate as its literals read: its name, with its arguments
        // left out.
        let predicate = |symbol: Symbol| match sequent.and_then(|s| s.symbols().get(symbol.index()))
        {
            Some((name, _)) => format!("{name}(…)"),
            None => format!("#{}(…)", symbol.get()),
        };
        match self {
            Refutation::UnbalancedSymbol(UnbalancedSymbol {
                symbol,
                least,
                most,
            }) => write_unbalanced(
                f,
                &predicate(*symbol),
                *least,
                *most,
                (None, true),
                words(&of_symbol(*symbol), *least),
            ),
            Refutation::OneSidedSymbol(OneSidedSymbol {
                symbol,
                least,
                most,
                copied,
            }) => write_unbalanced(
                f,
                &predicate(*symbol),
                *least,
                *most,
                (Some(*copied), true),
                words(&of_symbol(*symbol), *least),
            ),
            Refutation::Exhausted => f.write_str("the search was exhaustive"),
            Refutation::Unbalanced(Unbalanced { atom, least, most }) => write_unbalanced(
                f,
                &name(*atom),
                *least,
                *most,
                (None, false),
                words(&|a| a == *atom, *least),
            ),
            Refutation::OneSided(OneSided {
                atom,
                least,
                most,
                copied,
            }) => write_unbalanced(
                f,
                &name(*atom),
                *least,
                *most,
                (Some(*copied), false),
                words(&|a| a == *atom, *least),
            ),
            Refutation::Equation(Equation {
                formulas,
                needed,
                tensors,
                pars,
                ones,
                bottoms,
                mix,
            }) => {
                // A count below zero takes the minus sign the expression uses.
                let needed = if *needed < 0 {
                    format!("−{}", needed.unsigned_abs())
                } else {
                    needed.to_string()
                };
                let exactly = if *mix { "at least " } else { "exactly " };
                if intuitionistic {
                    // A one-sided `⊗` is a `⊗` right of `⊢` or a `⊸` left
                    // of it, a `⅋` the other two, a `1` a `1` right and a
                    // `⊥` a `1` left.
                    write!(
                        f,
                        "the count equation fails: a provable sequent of IMLL has {exactly}#⊗ᵣ + \
                         #⊸ₗ − #⊗ₗ − #⊸ᵣ − #1ᵣ + #1ₗ + 2 formulas (ᵣ right of ⊢, ₗ left of it, an \
                         implication's antecedent on the other side), here {tensors} − {pars} − \
                         {ones} + {bottoms} + 2 = {needed}, and this one has {formulas}",
                    )
                } else {
                    write!(
                        f,
                        "the count equation fails: a provable one-sided sequent of MLL has \
                         {exactly}#⊗ − #⅋ − #1 + #⊥ + 2 formulas, here {tensors} − {pars} − \
                         {ones} + {bottoms} + 2 = {needed}, and this one has {formulas}",
                    )
                }
            }
            Refutation::StateEquation(StateEquation { atoms, clauses, .. }) => {
                f.write_str(
                    "the state equation of the Petri net has no solution, so no firing of its \
                     clauses yields the goal's atoms: weighting",
                )?;
                let once = !clauses.is_empty();
                for (i, &(atom, weight)) in atoms.iter().enumerate() {
                    let sign = if weight < 0 { "−" } else { "" };
                    let between = match i {
                        0 => " each",
                        _ if i + 1 == atoms.len() && !once => " and",
                        _ => ",",
                    };
                    write!(
                        f,
                        "{between} {} by {sign}{}",
                        name(atom),
                        weight.unsigned_abs()
                    )?;
                }
                if once {
                    let between = if atoms.is_empty() { "" } else { " and" };
                    f.write_str(between)?;
                    f.write_str(" the clauses used once by weights of their own")?;
                }
                f.write_str(
                    ", no clause that can fire raises the weighted count of the atoms, and the goal \
                     asks it raised",
                )
            }
            Refutation::Classical(Classical { assignment }) => {
                f.write_str("the classical reading is false")?;
                let truths: Vec<String> = (0..assignment.len())
                    .filter(|&a| assignment[a])
                    .map(|a| name(Atom::new(a as u32)))
                    .collect();
                match truths.len() {
                    _ if assignment.is_empty() => {}
                    0 => f.write_str(" where every atom is false")?,
                    n if n == assignment.len() => f.write_str(" where every atom is true")?,
                    n => write!(
                        f,
                        " where {} {} true and every other atom false",
                        list(&truths),
                        if n == 1 { "is" } else { "are" }
                    )?,
                }
                f.write_str(if intuitionistic {
                    " (! erased, ⊗ and & as and, ⊕ as or, ⊸ as implies, 1 and ⊤ true, 0 false), \
                     and every provable sequent reads as a tautology"
                } else {
                    " (! and ? erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), \
                     and every provable sequent reads as a tautology"
                })
            }
            Refutation::Saturated(Saturated { sequents, .. }) => write!(
                f,
                "the search saturated: its {} {} closed under the rules, and {} the goal",
                sequents.len(),
                if sequents.len() == 1 {
                    "sequent is"
                } else {
                    "sequents are"
                },
                if sequents.len() == 1 {
                    "it does not yield"
                } else {
                    "none of them yields"
                },
            ),
        }
    }
}

/// Returns names as an English list: `a`, `a and b`, `a, b and c`.
fn list(names: &[String]) -> String {
    match names {
        [] => String::new(),
        [one] => one.clone(),
        [init @ .., last] => format!("{} and {last}", init.join(", ")),
    }
}

/// How a refutation's phrase speaks of the sides of `⊢`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sides {
    /// Of the one-sided sequent: an atom and its negation.
    One,
    /// Of a classical sequent written with formulas left of `⊢`: an atom
    /// left and right of it.
    Two,
    /// Of an intuitionistic sequent: an atom's positive and negative
    /// occurrences.
    Intuitionistic,
}

/// The wording of a count of literals, as [`Sides`] picks it.
#[derive(Clone, Copy, Debug)]
enum Words {
    /// One-sided.
    One,
    /// Two-sided, with where the counted literals lie.
    Two(Placed),
    /// Intuitionistic.
    Intuitionistic,
}

/// Where the literals of a count lie in a classical sequent written with
/// formulas left of `⊢`, a positive literal counting as right of it and a
/// negative one as left.
#[derive(Clone, Copy, Debug)]
struct Placed {
    /// The side with fewer of them has none.
    fewer_absent: bool,
    /// One counts on the other side than the formula it stands in: it is
    /// negated, or an implication's antecedent.
    crossed: bool,
}

/// Returns where the literals of the atoms `counted` picks lie among the
/// roots of `sequent`, the positive ones being the more where
/// `more_right` is set.
fn placed(sequent: &Sequent, counted: &dyn Fn(Atom) -> bool, more_right: bool) -> Placed {
    /// A positive literal below a term: right of `⊢`.
    const RIGHT: u8 = 1;
    /// A negative literal below a term: left of `⊢`.
    const LEFT: u8 = 2;
    // A pass over the arena, whose terms refer only to terms before them:
    // the sides of the counted literals below each term.
    let mut below: Vec<u8> = Vec::with_capacity(sequent.terms().len());
    for term in sequent.terms() {
        let bits = match *term {
            Term::Atom(a) => {
                if counted(a) {
                    RIGHT
                } else {
                    0
                }
            }
            Term::DualAtom(a) => {
                if counted(a) {
                    LEFT
                } else {
                    0
                }
            }
            Term::One | Term::Bot | Term::Top | Term::Zero => 0,
            Term::Tensor(a, b) | Term::Par(a, b) | Term::With(a, b) | Term::Plus(a, b) => {
                below[a.index()] | below[b.index()]
            }
            Term::Bang(a) | Term::Quest(a) | Term::Forall(a) | Term::Exists(a) => below[a.index()],
        };
        below.push(bits);
    }
    let antecedents = sequent.antecedents().map_or(0, |n| n as usize);
    let (mut all, mut crossed) = (0, false);
    for (i, root) in sequent.roots().iter().enumerate() {
        let bits = below[root.index()];
        all |= bits;
        // A hypothesis is held negated, so a literal of its own side is
        // negative; a positive one below it was negated or an antecedent.
        crossed |= bits & if i < antecedents { RIGHT } else { LEFT } != 0;
    }
    Placed {
        fewer_absent: all & if more_right { LEFT } else { RIGHT } == 0,
        crossed,
    }
}

/// Writes why the literals of the atom `name` cannot all meet in axioms:
/// one literal outnumbers the other between `least` and `most` times.
/// Where `copied` is given, the count is outside the exponentials, and
/// every literal below one has that sign. Where `predicate` is set, the
/// count is over every atom of a predicate. The `words` say how: of the
/// one-sided sequent; of a two-sided classical one, with the atom left and
/// right of `⊢`; or of an intuitionistic one, its positive occurrences
/// counted right of `⊢` and its negative ones left of it.
fn write_unbalanced(
    f: &mut Formatter<'_>,
    name: &str,
    least: i32,
    most: i32,
    (copied, predicate): (Option<Sign>, bool),
    words: Words,
) -> FmtResult {
    let over = if predicate {
        ", every atom of the predicate counted, since one has a bound variable"
    } else {
        ""
    };
    let (a, b) = (least.unsigned_abs(), most.unsigned_abs());
    let (a, b) = (a.min(b), a.max(b));
    let excess = if a == b {
        format!("{a}")
    } else {
        format!("{a} to {b}")
    };
    let times = if a == 1 && b == 1 { "time" } else { "times" };
    let alternatives = if a == b {
        ""
    } else {
        ", whichever additive alternatives a proof takes"
    };
    if let Words::Two(placed) = words {
        // A positive literal counts as right of `⊢`.
        let (more, fewer) = if least > 0 {
            ("right", "left")
        } else {
            ("left", "right")
        };
        let crossed = if placed.crossed {
            " (a negated formula or an implication's antecedent counts on the other side)"
        } else {
            ""
        };
        if copied.is_none() && placed.fewer_absent {
            return write!(
                f,
                "{name} on the {more} has no {name} on the {fewer} to meet in an axiom{over}{crossed}"
            );
        }
        let (place, copies) = match copied {
            Some(sign) => (
                " outside the exponentials",
                format!(
                    ", and every occurrence of it below one, which copies add, is on the {}",
                    match sign {
                        Sign::Atom => "right",
                        Sign::Dual => "left",
                    }
                ),
            ),
            None => ("", String::new()),
        };
        return write!(
            f,
            "{name} occurs {excess} more {times} on the {more} than on the {fewer}{place}\
             {alternatives}{copies}{over}, so they cannot all meet in axioms{crossed}",
        );
    }
    if matches!(words, Words::Intuitionistic) {
        // An atom in output position is the positive literal, which
        // counts as right of `⊢`.
        let (more, fewer) = if least > 0 {
            ("right", "left")
        } else {
            ("left", "right")
        };
        let (place, copies) = match copied {
            Some(sign) => (
                " outside the exponentials",
                format!(
                    ", and every occurrence of it below one, which copies add, is on the {}",
                    match sign {
                        Sign::Atom => "right",
                        Sign::Dual => "left",
                    }
                ),
            ),
            None => ("", String::new()),
        };
        return write!(
            f,
            "{name} occurs {excess} more {times} on the {more} than on the {fewer}{place}\
             {alternatives}{copies}{over}, so they cannot all meet in axioms (an implication's \
             antecedent counts on the other side)",
        );
    }
    let (more, fewer) = if least > 0 {
        (name.to_owned(), format!("~{name}"))
    } else {
        (format!("~{name}"), name.to_owned())
    };
    // The copied literal is the one `copied` names. The engine's
    // refutations make it the one that outnumbers the other; a refutation
    // read from JSON says what its fields say.
    let (place, copies) = match copied {
        Some(sign) => {
            let literal = match sign {
                Sign::Atom => name.to_owned(),
                Sign::Dual => format!("~{name}"),
            };
            (
                "outside the exponentials of the one-sided sequent",
                format!(
                    ", and every literal of the atom below one, which copies add, is {literal}"
                ),
            )
        }
        None => ("in the one-sided sequent", String::new()),
    };
    write!(
        f,
        "{more} occurs {excess} more {times} than {fewer} {place}{alternatives}{copies}{over}, so \
         they cannot all meet in axioms",
    )
}

impl Display for Refutation {
    /// Writes the refutation as a phrase, such as `the search was
    /// exhaustive`, with every atom by its number, as in `#0`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.write(f, None, Sides::One)
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

impl Display for Disproof {
    /// Writes the refutation as a phrase, with every atom by its name. In
    /// an intuitionistic mode it uses the sequent's two-sided terms, and
    /// so does a count of a classical sequent written with formulas left
    /// of `⊢`, refuted as a whole: an atom left and right of `⊢`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let sides = if self.mode.is_intuitionistic() {
            Sides::Intuitionistic
        } else if self.goal.is_none() && self.sequent.antecedents().is_some_and(|n| n > 0) {
            Sides::Two
        } else {
            Sides::One
        };
        self.refutation.write(f, Some(&self.sequent), sides)
    }
}

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
        }
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
    /// These six kinds are certificates. Every derivation keeps the
    /// invariant that each of the five others breaks, and the set holds a
    /// sequent for every stable one of every proof.
    ///
    /// # Cost
    ///
    /// Time and memory are linear in the goal's occurrences, a member
    /// repeated counted each time, for every kind but the saturated set.
    /// The set's check applies every rule to every choice of its sequents.
    /// For a set a search gave, that takes at most as long as the search
    /// that saturated. The check has no stop, though. For a set read from
    /// elsewhere it can take time exponential in the `&` under `⅋` of the
    /// goal, and polynomial in the set's size, of the degree of a rule's
    /// holes (a set that repeats one sequent). So a caller checks such a
    /// set within a time limit of its own.
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
        let fault = |fault| crate::Error::Refutation(fault);
        let forest = crate::linear::occurrences::Forest::new(&self.sequent)?;
        // The counts carry over to quantifiers, since a quantifier's rule
        // changes none of them. An assignment and a saturated set are
        // propositional.
        if forest.has_binders()
            && matches!(
                self.refutation,
                Refutation::Classical(_) | Refutation::Saturated(_)
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
                match super::saturated::check(&forest, &goal, self.mode, saturated, &mut |_| false)
                {
                    Ok(()) => Ok(()),
                    Err(super::saturated::Failed::Fault(fault)) => Err(fault),
                    Err(super::saturated::Failed::Stopped) => {
                        unreachable!("a check without a stop runs to its end")
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
            refutation @ Refutation::Exhausted => Err(RefutationFault::Uncertified {
                kind: refutation.name(),
            }),
        }
        .map_err(fault)
    }
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
    if let Some((symbol, _)) = sequent.atom_predicate(atom)
        && (0..sequent.atom_names().len()).any(|a| {
            let a = Atom::new(a as u32);
            sequent.atom_predicate(a).map(|(s, _)| s) == Some(symbol) && sequent.atom_is_open(a)
        })
    {
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
    use crate::linear::sequents::Kind::*;
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
                Bang | Quest => l.filter(|&s| s == (0, 0)),
                Forall | Exists => l,
            };
        }
        let Some((lo, hi)) = span[member.index()] else {
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
    if let Some((symbol, _)) = sequent.atom_predicate(atom)
        && (0..sequent.atom_names().len()).any(|a| {
            let a = Atom::new(a as u32);
            sequent.atom_predicate(a).map(|(s, _)| s) == Some(symbol) && sequent.atom_is_open(a)
        })
    {
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
                Bang | Quest => l.map(|_| (0, 0)),
                Forall | Exists => l,
            };
        }
        let Some((a, b)) = span[member.index()] else {
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
        ] {
            assert_eq!(refuted(input, classical).to_string(), phrase, "{input}");
        }
        let goal = refuted("a |- b", classical).of_goal(Box::new([]));
        assert!(goal.to_string().contains("one-sided"), "{goal}");
    }
}
