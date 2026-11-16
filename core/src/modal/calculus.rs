// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The labelled sequent calculus of a modal system, as a calculus given
//! as data ([`rules::Calculus`](crate::rules::Calculus)): Negri's G3K with the frame rules of the
//! system's conditions. A modal proof is a derivation in it, which
//! [`rules::Derivation::check`](crate::rules::Derivation::check) checks.

use super::{Condition, System};
use crate::Error;
use crate::limits::Limits;
use crate::rules::{self, Associativity, Discipline, Fixity};
use crate::serialize::rules::{
    CalculusForm, ConnectiveForm, PatternForm, PremiseForm, RuleForm, SequentForm, ZoneForm,
};
use std::collections::BTreeMap;

/// The sort of a label, a world of the calculus.
pub(crate) const LABEL: &str = "label";

/// The sort of what a sequent holds: a labelled formula `x : A` or a
/// relational atom `x R y`.
pub(crate) const ITEM: &str = "item";

/// The connective of a labelled formula.
pub(crate) const COLON: &str = ":";

/// The connective of a relational atom.
pub(crate) const RELATION: &str = "R";

/// The left zone's name.
const LEFT: &str = "left";

/// The right zone's name.
const RIGHT: &str = "right";

/// A rule of the labelled calculus, by its name in the rule file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Rule {
    /// `x : A` on both sides.
    Axiom,
    /// `x : ⊥` left.
    FalseLeft,
    /// `x : ⊤` right.
    TrueRight,
    /// `¬` left.
    NotLeft,
    /// `¬` right.
    NotRight,
    /// `∧` left.
    AndLeft,
    /// `∧` right.
    AndRight,
    /// `∨` left.
    OrLeft,
    /// `∨` right.
    OrRight,
    /// `→` left.
    ImpliesLeft,
    /// `→` right.
    ImpliesRight,
    /// `↔` left.
    IffLeft,
    /// `↔` right.
    IffRight,
    /// `□` left: along an edge.
    BoxLeft,
    /// `□` right: a new world.
    BoxRight,
    /// `◇` left: a new world.
    DiamondLeft,
    /// `◇` right: along an edge.
    DiamondRight,
    /// Reflexivity: `x R x`.
    Reflexive,
    /// Seriality: a new world `y` with `x R y`.
    Serial,
    /// Symmetry: `y R x` from `x R y`.
    Symmetric,
    /// Transitivity: `x R z` from `x R y` and `y R z`.
    Transitive,
    /// Euclideanness: `y R z` from `x R y` and `x R z`.
    Euclidean,
}

impl Rule {
    /// Returns the rule's name in the rule file.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Axiom => "ax",
            Self::FalseLeft => "L⊥",
            Self::TrueRight => "R⊤",
            Self::NotLeft => "L¬",
            Self::NotRight => "R¬",
            Self::AndLeft => "L∧",
            Self::AndRight => "R∧",
            Self::OrLeft => "L∨",
            Self::OrRight => "R∨",
            Self::ImpliesLeft => "L→",
            Self::ImpliesRight => "R→",
            Self::IffLeft => "L↔",
            Self::IffRight => "R↔",
            Self::BoxLeft => "L□",
            Self::BoxRight => "R□",
            Self::DiamondLeft => "L◇",
            Self::DiamondRight => "R◇",
            Self::Reflexive => "Ref",
            Self::Serial => "Ser",
            Self::Symmetric => "Sym",
            Self::Transitive => "Trans",
            Self::Euclidean => "Eucl",
        }
    }

    /// Returns the frame rule of a condition.
    pub(crate) const fn of(condition: Condition) -> Self {
        match condition {
            Condition::Serial => Self::Serial,
            Condition::Reflexive => Self::Reflexive,
            Condition::Symmetric => Self::Symmetric,
            Condition::Transitive => Self::Transitive,
            Condition::Euclidean => Self::Euclidean,
        }
    }
}

/// Returns the name of the calculus of `system`: `G3` and the system's
/// name, `G3K`, `G3S4`.
pub(crate) const fn name(system: System) -> &'static str {
    /// The names, in the order of [`System::ALL`].
    const NAMES: [&str; 15] = [
        "G3K", "G3D", "G3T", "G3KB", "G3KDB", "G3B", "G3K4", "G3KD4", "G3S4", "G3K5", "G3KD5",
        "G3K45", "G3KD45", "G3KB5", "G3S5",
    ];
    // A system is one of `ALL`: the last where it is none before.
    let mut index = 0;
    while index + 1 < System::ALL.len()
        && !(System::ALL[index].extends(system) && system.extends(System::ALL[index]))
    {
        index += 1;
    }
    NAMES[index]
}

/// Returns the labelled sequent calculus of `system` as a calculus given
/// as data, which [`rules::Derivation::check`] checks a modal proof in.
///
/// A sequent holds labelled formulas `x : A` and relational atoms
/// `x R y` left of `⊢`, and labelled formulas right of it, each zone a
/// set. The rules are those of Negri's G3K ("Proof analysis in modal
/// logic", J. Philos. Logic 34, 2005, Table II), with the frame rule of
/// each condition of the system (Ref, Ser, Sym, Trans, Eucl, p. 515).
/// Its departures, all sound: every rule keeps its principal formula in
/// its premises, as a tableau keeps the formula it expands; the axiom
/// closes on any formula at one label, not only an atom; `L¬`, `R¬`,
/// `R⊤`, `L↔` and `R↔` are added for the connectives Table II has no rules
/// for; and the initial sequent on a relational atom is left out, since
/// no frame condition here needs it. Labels are atoms of sort `label`;
/// `R□`, `L◇` and `Ser` ask theirs to be new.
///
/// [`Calculus`](rules::Calculus) writes itself back as a rule file, the
/// text a front end shows or saves.
///
/// # Errors
///
/// [`Error::InvalidCalculus`] would be a defect of this table, and
/// [`Refusal::Memory`](crate::Refusal::Memory) where `limits.memory_bytes`
/// is too small for the calculus's tables.
pub fn calculus(system: System, limits: &Limits) -> Result<rules::Calculus, Error> {
    rules::Calculus::from_form(form(system), limits)
}

/// Returns a pattern of a name.
fn n(name: &str) -> PatternForm {
    PatternForm::Name(name.to_owned())
}

/// Returns a pattern of a connective applied to patterns.
fn app(connective: &str, arguments: Vec<PatternForm>) -> PatternForm {
    PatternForm::Apply(connective.to_owned(), arguments)
}

/// Returns `x : A` for a label variable and a formula pattern.
fn at(label: &str, formula: PatternForm) -> PatternForm {
    app(COLON, vec![n(label), formula])
}

/// Returns `x R y`.
fn r(x: &str, y: &str) -> PatternForm {
    app(RELATION, vec![n(x), n(y)])
}

/// Returns a connective's pattern over `A` and `B`, or `A` alone.
fn over(connective: &str, arity: u32) -> PatternForm {
    match arity {
        1 => app(connective, vec![n("A")]),
        _ => app(connective, vec![n("A"), n("B")]),
    }
}

/// Returns a schematic sequent `Γ, left ⊢ Δ, right`.
fn seq(left: Vec<PatternForm>, right: Vec<PatternForm>) -> SequentForm {
    let mut sequent = BTreeMap::new();
    sequent.insert(LEFT.to_owned(), [vec![n("Γ")], left].concat());
    sequent.insert(RIGHT.to_owned(), [vec![n("Δ")], right].concat());
    sequent
}

/// Returns a premise without discharges.
fn premise(sequent: SequentForm) -> PremiseForm {
    PremiseForm {
        sequent,
        discharges: Vec::new(),
    }
}

/// Returns a rule.
fn rule(
    rule: Rule,
    premises: Vec<SequentForm>,
    conclusion: SequentForm,
    fresh: &[&str],
) -> RuleForm {
    RuleForm {
        name: rule.name().to_owned(),
        premises: premises.into_iter().map(premise).collect(),
        conclusion,
        fresh: fresh.iter().map(|&v| v.to_owned()).collect(),
        atomic: Vec::new(),
    }
}

/// Returns a connective of formulas.
fn connective(
    name: &str,
    arity: u32,
    fixity: Option<Fixity>,
    precedence: Option<u32>,
) -> ConnectiveForm {
    ConnectiveForm {
        name: name.to_owned(),
        arity,
        sorts: None,
        symbol: None,
        fixity,
        precedence,
        associativity: (arity == 2 && name != "↔").then_some(Associativity::Right),
    }
}

/// Returns the rule file of `system`.
pub(crate) fn form(system: System) -> CalculusForm {
    let prefix = Some(Fixity::Prefix);
    let infix = Some(Fixity::Infix);
    let mut connectives = vec![
        connective("⊤", 0, None, None),
        connective("⊥", 0, None, None),
        connective("¬", 1, prefix, Some(5)),
        connective("□", 1, prefix, Some(5)),
        connective("◇", 1, prefix, Some(5)),
        connective("∧", 2, infix, Some(4)),
        connective("∨", 2, infix, Some(3)),
        connective("→", 2, infix, Some(2)),
        connective("↔", 2, infix, Some(1)),
    ];
    if let Some(iff) = connectives.last_mut() {
        iff.associativity = Some(Associativity::None);
    }
    for (name, first) in [(COLON, "formula"), (RELATION, LABEL)] {
        connectives.push(ConnectiveForm {
            name: name.to_owned(),
            arity: 2,
            sorts: Some(vec![LABEL.to_owned(), first.to_owned(), ITEM.to_owned()]),
            symbol: None,
            fixity: infix,
            precedence: Some(0),
            associativity: Some(Associativity::None),
        });
    }
    let zone = |name: &str, mark: Option<&str>| ZoneForm {
        name: name.to_owned(),
        sort: Some(ITEM.to_owned()),
        discipline: Discipline::Set,
        mark: mark.map(str::to_owned),
        least: None,
        most: None,
    };
    let mut variables = BTreeMap::new();
    variables.insert("formula".to_owned(), vec!["A".to_owned(), "B".to_owned()]);
    variables.insert(
        LABEL.to_owned(),
        vec!["x".to_owned(), "y".to_owned(), "z".to_owned()],
    );
    let x = |p: PatternForm| at("x", p);
    let (a, b) = (|| x(n("A")), || x(n("B")));
    let mut rules = vec![
        rule(Rule::Axiom, vec![], seq(vec![a()], vec![a()]), &[]),
        rule(Rule::FalseLeft, vec![], seq(vec![x(n("⊥"))], vec![]), &[]),
        rule(Rule::TrueRight, vec![], seq(vec![], vec![x(n("⊤"))]), &[]),
    ];
    let not = || x(over("¬", 1));
    rules.push(rule(
        Rule::NotLeft,
        vec![seq(vec![not()], vec![a()])],
        seq(vec![not()], vec![]),
        &[],
    ));
    rules.push(rule(
        Rule::NotRight,
        vec![seq(vec![a()], vec![not()])],
        seq(vec![], vec![not()]),
        &[],
    ));
    let and = || x(over("∧", 2));
    rules.push(rule(
        Rule::AndLeft,
        vec![seq(vec![and(), a(), b()], vec![])],
        seq(vec![and()], vec![]),
        &[],
    ));
    rules.push(rule(
        Rule::AndRight,
        vec![seq(vec![], vec![and(), a()]), seq(vec![], vec![and(), b()])],
        seq(vec![], vec![and()]),
        &[],
    ));
    let or = || x(over("∨", 2));
    rules.push(rule(
        Rule::OrLeft,
        vec![seq(vec![or(), a()], vec![]), seq(vec![or(), b()], vec![])],
        seq(vec![or()], vec![]),
        &[],
    ));
    rules.push(rule(
        Rule::OrRight,
        vec![seq(vec![], vec![or(), a(), b()])],
        seq(vec![], vec![or()]),
        &[],
    ));
    let implies = || x(over("→", 2));
    rules.push(rule(
        Rule::ImpliesLeft,
        vec![
            seq(vec![implies()], vec![a()]),
            seq(vec![implies(), b()], vec![]),
        ],
        seq(vec![implies()], vec![]),
        &[],
    ));
    rules.push(rule(
        Rule::ImpliesRight,
        vec![seq(vec![a()], vec![implies(), b()])],
        seq(vec![], vec![implies()]),
        &[],
    ));
    let iff = || x(over("↔", 2));
    rules.push(rule(
        Rule::IffLeft,
        vec![
            seq(vec![iff(), a(), b()], vec![]),
            seq(vec![iff()], vec![a(), b()]),
        ],
        seq(vec![iff()], vec![]),
        &[],
    ));
    rules.push(rule(
        Rule::IffRight,
        vec![
            seq(vec![a()], vec![iff(), b()]),
            seq(vec![b()], vec![iff(), a()]),
        ],
        seq(vec![], vec![iff()]),
        &[],
    ));
    let boxed = || x(over("□", 1));
    let diamond = || x(over("◇", 1));
    let ya = || at("y", n("A"));
    rules.push(rule(
        Rule::BoxLeft,
        vec![seq(vec![boxed(), r("x", "y"), ya()], vec![])],
        seq(vec![boxed(), r("x", "y")], vec![]),
        &[],
    ));
    rules.push(rule(
        Rule::BoxRight,
        vec![seq(vec![r("x", "y")], vec![boxed(), ya()])],
        seq(vec![], vec![boxed()]),
        &["y"],
    ));
    rules.push(rule(
        Rule::DiamondLeft,
        vec![seq(vec![diamond(), r("x", "y"), ya()], vec![])],
        seq(vec![diamond()], vec![]),
        &["y"],
    ));
    rules.push(rule(
        Rule::DiamondRight,
        vec![seq(vec![r("x", "y")], vec![diamond(), ya()])],
        seq(vec![r("x", "y")], vec![diamond()]),
        &[],
    ));
    for condition in system.conditions() {
        let frame = Rule::of(condition);
        rules.push(match condition {
            Condition::Reflexive => rule(
                frame,
                vec![seq(vec![r("x", "x")], vec![])],
                seq(vec![], vec![]),
                &[],
            ),
            Condition::Serial => rule(
                frame,
                vec![seq(vec![r("x", "y")], vec![])],
                seq(vec![], vec![]),
                &["y"],
            ),
            Condition::Symmetric => rule(
                frame,
                vec![seq(vec![r("x", "y"), r("y", "x")], vec![])],
                seq(vec![r("x", "y")], vec![]),
                &[],
            ),
            Condition::Transitive => rule(
                frame,
                vec![seq(vec![r("x", "y"), r("y", "z"), r("x", "z")], vec![])],
                seq(vec![r("x", "y"), r("y", "z")], vec![]),
                &[],
            ),
            Condition::Euclidean => rule(
                frame,
                vec![seq(vec![r("x", "y"), r("x", "z"), r("y", "z")], vec![])],
                seq(vec![r("x", "y"), r("x", "z")], vec![]),
                &[],
            ),
        });
    }
    let conditions: Vec<&str> = system.conditions().map(Condition::name).collect();
    CalculusForm {
        version: None,
        name: name(system).to_owned(),
        description: format!(
            "Negri's labelled sequent calculus G3K for the modal system {}, with the frame \
             rules of its conditions{}. A sequent holds labelled formulas x : A and relational \
             atoms x R y; every rule keeps its principal formula, as a tableau does, and the \
             axiom closes on any formula at one label.",
            system.name(),
            if conditions.is_empty() {
                String::new()
            } else {
                format!(" ({})", conditions.join(", "))
            }
        ),
        sorts: vec![LABEL.to_owned(), ITEM.to_owned()],
        connectives,
        sequent: vec![zone(LEFT, None), zone(RIGHT, Some("⊢"))],
        variables,
        contexts: vec!["Γ".to_owned(), "Δ".to_owned()],
        rules,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every system's calculus compiles, has the frame rules of its
    /// conditions and no other, and names its rules as the search does.
    #[test]
    fn every_system_has_its_calculus() {
        for &system in System::ALL {
            let calculus = calculus(system, &Limits::default()).unwrap();
            assert_eq!(calculus.name(), format!("G3{system}"));
            let names: Vec<&str> = calculus.rules().collect();
            for &condition in Condition::ALL {
                assert_eq!(
                    names.contains(&Rule::of(condition).name()),
                    system.has(condition),
                    "{system}: {condition}"
                );
            }
            // A new label's rule takes the edge's position too.
            assert_eq!(calculus.positions(Rule::BoxRight.name()), Some(2));
            assert_eq!(calculus.positions(Rule::BoxLeft.name()), Some(2));
            assert_eq!(calculus.positions(Rule::AndLeft.name()), Some(1));
        }
    }
}
