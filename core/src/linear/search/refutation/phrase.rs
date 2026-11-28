// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The phrases a refutation and a disproof are written in: each kind's
//! wording, the sides of `⊢` a count speaks of, and the atoms by name.

use super::{
    Basis, Classical, Disproof, Equation, OneSided, OneSidedSymbol, Refutation, Saturated,
    StateEquation, Unbalanced, UnbalancedSymbol,
};
use crate::linear::occurrences::Sign;
use crate::linear::sequents::{Atom, Sequent, Symbol, Term};
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Refutation {
    /// Writes the refutation as a phrase. Each atom and predicate is
    /// written by its name in `sequent`, or by its number where there is
    /// no sequent. The literals of a count are spoken of in the `sides`
    /// the sequent is read with.
    fn write(&self, f: &mut Formatter<'_>, sequent: Option<&Sequent>, sides: Sides) -> FmtResult {
        let intuitionistic = matches!(sides, Sides::Intuitionistic { .. });
        // The wording of a count: two-sided where the sequent was written
        // with hypotheses, with where the counted literals lie.
        let words = |counted: &dyn Fn(Atom) -> bool, least: i32| match (sides, sequent) {
            (Sides::Two, Some(sequent)) => Words::Two(placed(sequent, counted, least > 0)),
            // Under the reading an atom in output position is the
            // positive literal, so the literals' signs are their sides.
            (Sides::Intuitionistic { whole }, sequent) => Words::Intuitionistic {
                fewer_absent: whole
                    && sequent.is_some_and(|s| placed(s, counted, least > 0).fewer_absent),
            },
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
        let name = |atom: Atom| match (sequent, names.and_then(|names| names.get(atom.index()))) {
            // An equality is written between its terms, as the printer does.
            (Some(sequent), Some(_)) if sequent.is_first_order() => {
                crate::linear::sequents::notation::Notation::atom_text(sequent, atom)
            }
            (_, Some(name)) => name.clone(),
            (_, None) => format!("#{}", atom.get()),
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
            Refutation::Basis(Basis { markings, .. }) => write!(
                f,
                "read as a Petri net, the goal's target cannot be covered: a backward search \
                 from it saturated, and none of the {} {} it kept is at most the initial marking",
                markings.len(),
                if markings.len() == 1 {
                    "marking"
                } else {
                    "markings"
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
    Intuitionistic {
        /// Whether the sequent is refuted as a whole, not a goal within
        /// it, so that its literals are the counted ones.
        whole: bool,
    },
}

/// The wording of a count of literals, as [`Sides`] picks it.
#[derive(Clone, Copy, Debug)]
enum Words {
    /// One-sided.
    One,
    /// Two-sided, with where the counted literals lie.
    Two(Placed),
    /// Intuitionistic, with whether the side with fewer of the counted
    /// literals has none.
    Intuitionistic {
        /// The side with fewer of them has none.
        fewer_absent: bool,
    },
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
    // Two-sided, whether the side with fewer of the literals has none, and
    // the note that closes the phrase.
    let sided = match words {
        Words::One => None,
        Words::Two(placed) => Some((
            placed.fewer_absent,
            if placed.crossed {
                " (a negated formula or an implication's antecedent counts on the other side)"
            } else {
                ""
            },
        )),
        Words::Intuitionistic { fewer_absent } => Some((
            fewer_absent,
            " (an implication's antecedent counts on the other side)",
        )),
    };
    if let Some((fewer_absent, note)) = sided {
        // A positive literal counts as right of `⊢`; under a reading it is
        // the atom in output position.
        let (more, fewer) = if least > 0 {
            ("right", "left")
        } else {
            ("left", "right")
        };
        if copied.is_none() && fewer_absent {
            return write!(
                f,
                "{name} on the {more} has no {name} on the {fewer} to meet in an axiom{over}{note}"
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
             {alternatives}{copies}{over}, so they cannot all meet in axioms{note}",
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

impl Display for Disproof {
    /// Writes the refutation as a phrase, with every atom by its name. In
    /// an intuitionistic mode it uses the sequent's two-sided terms, and
    /// so does a count of a classical sequent written with formulas left
    /// of `⊢`, refuted as a whole: an atom left and right of `⊢`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let sides = if self.mode.is_intuitionistic() {
            Sides::Intuitionistic {
                whole: self.goal.is_none(),
            }
        } else if self.goal.is_none() && self.sequent.antecedents().is_some_and(|n| n > 0) {
            Sides::Two
        } else {
            Sides::One
        };
        self.refutation.write(f, Some(&self.sequent), sides)
    }
}
