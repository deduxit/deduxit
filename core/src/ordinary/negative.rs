// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The negative translations of classical logic into intuitionistic and
//! minimal logic, each with its claim checked.
//!
//! A negative translation turns a classical sequent into one that is
//! valid in a weaker logic exactly when the first is valid classically:
//!
//! | translation | what it does | into |
//! |---|---|---|
//! | [`Kolmogorov`](Translation::Kolmogorov) | `¬¬` before every subformula | minimal logic |
//! | [`GodelGentzen`](Translation::GodelGentzen) | `¬¬` before every atom, `A ∨ B` as `¬(¬A ∧ ¬B)`, `∃x. A` as `¬∀x. ¬A` | minimal logic |
//! | [`Kuroda`](Translation::Kuroda) | `¬¬` before every formula of the sequent and after every `∀` | intuitionistic logic |
//! | [`Glivenko`](Translation::Glivenko) | `¬¬` before the formula right of `⊢` | intuitionistic logic |
//!
//! In propositional logic Kuroda's translation puts `¬¬` before each
//! formula, since it adds more only after a `∀`; Glivenko's theorem says
//! that `Γ ⊢ A` is classically valid exactly when `Γ ⊢ ¬¬A` is
//! intuitionistically valid. Several formulas right of `⊢` are read as
//! their disjunction, which is translated.
//!
//! The first three translate first-order sequents too. Glivenko's
//! refuses them: its theorem fails at first order, where
//! `⊢ ¬¬∀x. (p(x) ∨ ¬p(x))` is not intuitionistically valid.
//!
//! [`translate`] returns the translated sequent. [`compare`] decides the
//! sequent classically and its translation in the target logic, and
//! returns both outcomes with their evidence; verdicts that differ are
//! [`Error::ClaimFails`], a defect, never an answer.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::negative::{self, Agreement, Options, Translation};
//! use deduxit::ordinary::Sequent;
//!
//! let sequent: Sequent = "a \\/ ~a".parse()?;
//! let options = Options::default().with_translation(Translation::GodelGentzen);
//! let translated = negative::translate(&sequent, &options, &deduxit::Limits::default())?;
//! assert_eq!(translated.to_string(), "⊢ ¬(¬¬¬a ∧ ¬¬¬¬a)");
//! let comparison = negative::compare_default(&sequent, &options)?;
//! assert_eq!(comparison.agreement, Agreement::Valid);
//! # Ok::<(), deduxit::Error>(())
//! ```

use super::{Formulas, Logic, Node, NodeId, Outcome, Sequent, Verdict, decide_within};
use crate::Error;
use crate::limits::{Limits, Progress};
use crate::linear::search;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A negative translation of classical logic, by its name.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "kebab-case"))]
pub enum Translation {
    /// Kolmogorov's: `¬¬` before every subformula. Into minimal logic.
    #[default]
    Kolmogorov,
    /// Gödel and Gentzen's: `¬¬` before every atom, `A ∨ B` as
    /// `¬(¬A ∧ ¬B)` and `∃x. A` as `¬∀x. ¬A`. Into minimal logic.
    GodelGentzen,
    /// Kuroda's: `¬¬` before every formula of the sequent and after
    /// every `∀`. Into intuitionistic logic.
    Kuroda,
    /// Glivenko's theorem: `¬¬` before the formula right of `⊢`, the
    /// hypotheses as they are. Into intuitionistic logic, in
    /// propositional logic only.
    Glivenko,
}

impl Translation {
    /// Every translation, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[
        Self::Kolmogorov,
        Self::GodelGentzen,
        Self::Kuroda,
        Self::Glivenko,
    ];

    /// The translations' names, which [`name`](Self::name) writes and
    /// `FromStr` reads.
    pub const NAMES: &'static [&'static str] =
        &["kolmogorov", "godel-gentzen", "kuroda", "glivenko"];

    /// Returns the translation's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Kolmogorov => "kolmogorov",
            Self::GodelGentzen => "godel-gentzen",
            Self::Kuroda => "kuroda",
            Self::Glivenko => "glivenko",
        }
    }

    /// Returns what a sentence calls it: "Kolmogorov's translation".
    pub const fn title(self) -> &'static str {
        match self {
            Self::Kolmogorov => "Kolmogorov's translation",
            Self::GodelGentzen => "Gödel and Gentzen's translation",
            Self::Kuroda => "Kuroda's translation",
            Self::Glivenko => "Glivenko's translation",
        }
    }

    /// Returns the weakest logic its claim holds in: minimal logic for
    /// Kolmogorov's and Gödel and Gentzen's, intuitionistic logic for
    /// Kuroda's and Glivenko's.
    pub const fn target(self) -> Logic {
        match self {
            Self::Kolmogorov | Self::GodelGentzen => Logic::Minimal,
            Self::Kuroda | Self::Glivenko => Logic::Intuitionistic,
        }
    }

    /// Returns whether its claim holds in `logic`: in its
    /// [`target`](Self::target), and in intuitionistic logic, which
    /// proves what minimal logic proves and no classical sequent that
    /// minimal logic does not prove of these translations.
    pub const fn reaches(self, logic: Logic) -> bool {
        match logic {
            Logic::Classical => false,
            Logic::Intuitionistic => true,
            Logic::Minimal => matches!(self.target(), Logic::Minimal),
        }
    }
}

impl Display for Translation {
    /// Writes the translation's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for Translation {
    type Err = Error;

    /// Reads a translation's [`name`](Self::name), one of
    /// [`NAMES`](Self::NAMES).
    fn from_str(name: &str) -> Result<Self, Error> {
        Self::ALL
            .iter()
            .copied()
            .find(|t| t.name() == name)
            .ok_or_else(|| Error::UnknownName {
                what: "negative translation",
                name: name.into(),
                known: Self::NAMES,
            })
    }
}

/// Which negative translation to take, and into which logic.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// The translation.
    pub translation: Translation,
    /// The logic translated into, or `None` for the translation's own
    /// [`target`](Translation::target).
    pub target: Option<Logic>,
}

impl Options {
    /// Returns the options with the translation given.
    #[must_use]
    pub const fn with_translation(self, translation: Translation) -> Self {
        Self {
            translation,
            ..self
        }
    }

    /// Returns the options with the target logic given, `None` for the
    /// translation's own.
    #[must_use]
    pub const fn with_target(self, target: Option<Logic>) -> Self {
        Self { target, ..self }
    }

    /// Returns the logic these options translate into.
    pub const fn target(&self) -> Logic {
        match self.target {
            Some(logic) => logic,
            None => self.translation.target(),
        }
    }
}

/// How a node of a classical formula is translated: its connective kept
/// or a disjunction turned into a negated conjunction, and how many `¬`
/// go before the result.
#[derive(Clone, Copy)]
struct Row {
    /// What becomes of the connective.
    shape: Shape,
    /// The `¬` written before the translated node.
    negations: u8,
}

/// What a translation makes of a node's connective.
#[derive(Clone, Copy)]
enum Shape {
    /// The same connective over the translated operands.
    Same,
    /// `A ∨ B` as `¬(¬A ∧ ¬B)` over the translated operands.
    DeMorgan,
    /// `∃x. A` as `¬∀x. ¬A` over the translated operand.
    ExistsAsForall,
    /// `∀x. A` as `∀x. ¬¬A` over the translated operand.
    ForallDoubled,
}

/// Returns the row of `node` in the table of `translation`. Quantifiers
/// get their rows here: Kuroda's translation puts `¬¬` after every `∀`,
/// and Gödel and Gentzen's writes `∃x A` as `¬∀x ¬A`.
const fn row(translation: Translation, node: Node) -> Row {
    const KEEP: Row = Row {
        shape: Shape::Same,
        negations: 0,
    };
    const DOUBLE: Row = Row {
        shape: Shape::Same,
        negations: 2,
    };
    match (translation, node) {
        (Translation::Kolmogorov, _) => DOUBLE,
        (Translation::GodelGentzen, Node::Atom(_)) => DOUBLE,
        (Translation::GodelGentzen, Node::Or(..)) => Row {
            shape: Shape::DeMorgan,
            negations: 0,
        },
        (Translation::GodelGentzen, Node::Exists(_)) => Row {
            shape: Shape::ExistsAsForall,
            negations: 0,
        },
        (Translation::Kuroda, Node::Forall(_)) => Row {
            shape: Shape::ForallDoubled,
            negations: 0,
        },
        (
            Translation::GodelGentzen,
            Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Forall(_),
        )
        | (
            Translation::Kuroda,
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Exists(_),
        )
        | (Translation::Glivenko, _) => KEEP,
    }
}

/// Returns the `¬` a translation puts before each translated hypothesis
/// and before the translated formula right of `⊢`.
const fn sides(translation: Translation) -> (u8, u8) {
    match translation {
        Translation::Kolmogorov | Translation::GodelGentzen => (0, 0),
        Translation::Kuroda => (2, 2),
        Translation::Glivenko => (0, 2),
    }
}

/// Returns the sequent translated by `options`, every formula built from
/// its table over the arena, its subformulas first, without recursion.
/// Several formulas right of `⊢` are translated as their disjunction, the
/// first leftmost; none stays none, which intuitionistic and minimal logic
/// read as `⊥`.
///
/// # Errors
///
/// [`Error::NegativeTarget`] for a target logic where the translation's
/// claim does not hold; [`Refusal::Occurrences`](crate::Refusal) for a
/// translation that unfolds past `limits.occurrences`;
/// [`Refusal::Index`](crate::Refusal) when the arena is full.
pub fn translate(sequent: &Sequent, options: &Options, limits: &Limits) -> Result<Sequent, Error> {
    let translation = options.translation;
    // Glivenko's theorem fails at first order: `¬¬∀x. (p(x) ∨ ¬p(x))` is
    // the double negation of a classical theorem that intuitionistic logic
    // does not prove.
    let first_order = sequent.formulas().is_first_order();
    if first_order && translation == Translation::Glivenko {
        sequent.formulas().propositional("Glivenko's translation")?;
    }
    let target = options.target();
    if !translation.reaches(target) {
        return Err(Error::NegativeTarget {
            translation,
            logic: target,
        });
    }
    // The formulas right of `⊢` as one disjunction, in a copy of the arena.
    let mut source = sequent.formulas().clone();
    let mut goal = None;
    for &d in sequent.right() {
        goal = Some(match goal {
            None => d,
            Some(g) => source.add(Node::Or(g, d))?,
        });
    }
    // The nodes the sequent reaches: every operand precedes its formula.
    let mut reached = vec![false; source.len()];
    for &root in sequent.left().iter().chain(&goal) {
        reached[root.index()] = true;
    }
    for k in (0..source.len()).rev() {
        if reached[k] {
            let (a, b) = source.nodes[k].operands();
            for id in [a, b].into_iter().flatten() {
                reached[id.index()] = true;
            }
        }
    }
    // A propositional sequent is translated into an arena of its own, its
    // atoms by name; a first-order one in a copy of its arena, where its
    // atoms and their terms stand as they are.
    let mut out = if first_order {
        source.clone()
    } else {
        Formulas::default()
    };
    // A node the sequent does not reach keeps a translation it never reads.
    let mut image: Vec<NodeId> = vec![NodeId(0); source.len()];
    for (k, &node) in source.nodes.iter().enumerate() {
        if !reached[k] {
            continue;
        }
        // An operand is reached when its formula is, and precedes it.
        let of = |id: NodeId| image[id.index()];
        let Row { shape, negations } = row(translation, node);
        let here = NodeId(k as u32);
        let mut id = match (shape, node) {
            (Shape::DeMorgan, Node::Or(a, b)) => {
                let na = out.add(Node::Not(of(a)))?;
                let nb = out.add(Node::Not(of(b)))?;
                let both = out.add(Node::And(na, nb))?;
                out.add(Node::Not(both))?
            }
            (Shape::ExistsAsForall, Node::Exists(a)) => {
                let na = out.add(Node::Not(of(a)))?;
                let all = out.add_like(Node::Forall(na), here)?;
                out.add(Node::Not(all))?
            }
            (Shape::ForallDoubled, Node::Forall(a)) => {
                let nna = negated(&mut out, of(a), 2)?;
                out.add_like(Node::Forall(nna), here)?
            }
            (_, Node::Atom(_)) if first_order => here,
            (_, Node::Atom(a)) => out.atom(source.atom_name(a))?,
            (_, Node::True) => out.add(Node::True)?,
            (_, Node::False) => out.add(Node::False)?,
            (_, Node::Not(a)) => out.add(Node::Not(of(a)))?,
            (_, Node::And(a, b)) => out.add(Node::And(of(a), of(b)))?,
            (_, Node::Or(a, b)) => out.add(Node::Or(of(a), of(b)))?,
            (_, Node::Implies(a, b)) => out.add(Node::Implies(of(a), of(b)))?,
            (_, Node::Iff(a, b)) => out.add(Node::Iff(of(a), of(b)))?,
            (_, Node::Forall(a)) => out.add_like(Node::Forall(of(a)), here)?,
            (_, Node::Exists(a)) => out.add_like(Node::Exists(of(a)), here)?,
        };
        id = negated(&mut out, id, negations)?;
        image[k] = id;
    }
    let (hypotheses, goals) = sides(translation);
    let mut left = Vec::with_capacity(sequent.left().len());
    for &h in sequent.left() {
        let id = image[h.index()];
        left.push(negated(&mut out, id, hypotheses)?);
    }
    let right = match goal {
        Some(g) => {
            let id = image[g.index()];
            vec![negated(&mut out, id, goals)?]
        }
        None => Vec::new(),
    };
    super::bound(&out, &left, &right, limits)?;
    Sequent::new(out, left, right)
}

/// Returns `id` under `count` negations, added to the arena.
fn negated(out: &mut Formulas, mut id: NodeId, count: u8) -> Result<NodeId, Error> {
    for _ in 0..count {
        id = out.add(Node::Not(id))?;
    }
    Ok(id)
}

/// What the two verdicts of a [`Comparison`] are, which agree.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "snake_case"))]
pub enum Agreement {
    /// The sequent is valid classically and its translation in the target
    /// logic.
    Valid,
    /// Neither is valid.
    NotValid,
    /// One of the two searches stopped before it decided, so whether they
    /// agree is not known.
    Unknown,
}

/// A classical sequent, its negative translation and both decided: what
/// [`compare`] returns.
///
/// # JSON
///
/// With the feature `serialize` a comparison is the document
/// `{"version": 16, "deduxit": "0.7.0", "translation": "kolmogorov",
/// "target": "minimal", "agreement": "valid", "sequent": {…},
/// "translated": {…}, "classical": {…}, "decided": {…}}`
/// ([`wire`](crate::wire)), written only: the two sequents in the form of
/// a [`Sequent`] and the two outcomes in the form of an
/// [`Outcome`], each without a `version`.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Comparison {
    /// The translation taken.
    pub translation: Translation,
    /// The classical sequent.
    pub sequent: Sequent,
    /// Its translation.
    pub translated: Sequent,
    /// The classical sequent decided in classical logic.
    pub classical: Outcome,
    /// The translation decided in the target logic, which is its `logic`.
    pub decided: Outcome,
    /// The verdicts, which agree.
    pub agreement: Agreement,
}

/// Translates `sequent` by `options`, decides it in classical logic and
/// its translation in the target logic, each by that logic's default
/// method under `search`, `limits` and `stop`, and compares the verdicts.
///
/// # Errors
///
/// [`translate`]'s and [`decide_within`]'s errors, and
/// [`Error::ClaimFails`] where one verdict is valid and the other not
/// valid, a defect.
pub fn compare(
    sequent: &Sequent,
    options: &Options,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Comparison, Error> {
    let translated = translate(sequent, options, limits)?;
    let classical = decide_within(
        sequent,
        &super::Options::default().with_logic(Logic::Classical),
        search,
        limits,
        &mut stop,
    )?;
    let target = options.target();
    let decided = decide_within(
        &translated,
        &super::Options::default().with_logic(target),
        search,
        limits,
        &mut stop,
    )?;
    let fails = |was: &str, is: &str| Error::ClaimFails {
        claim: options.translation.title().to_owned(),
        reason: format!(
            "the sequent is {was} in classical logic and its translation {is} in {target} logic"
        ),
    };
    let agreement = match (&classical.verdict, &decided.verdict) {
        (Verdict::Valid(_), Verdict::Valid(_)) => Agreement::Valid,
        (Verdict::NotValid, Verdict::NotValid) => Agreement::NotValid,
        (Verdict::Unknown(_), _) | (_, Verdict::Unknown(_)) => Agreement::Unknown,
        (Verdict::Valid(_), Verdict::NotValid) => return Err(fails("valid", "not valid")),
        (Verdict::NotValid, Verdict::Valid(_)) => return Err(fails("not valid", "valid")),
    };
    Ok(Comparison {
        translation: options.translation,
        sequent: sequent.clone(),
        translated,
        classical,
        decided,
        agreement,
    })
}

/// Compares as [`compare`] does within the default [`Limits`] and with the
/// linear search's default options.
///
/// # Errors
///
/// As [`compare`].
pub fn compare_default(sequent: &Sequent, options: &Options) -> Result<Comparison, Error> {
    compare(
        sequent,
        options,
        &search::Options::default(),
        &Limits::default(),
        |_| false,
    )
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;

    /// Each translation writes the formulas its table says, and keeps
    /// the hypotheses where its table keeps them.
    #[test]
    fn each_table_writes_its_formulas() {
        let sequent: Sequent = "a, a -> b |- b \\/ c".parse().unwrap();
        let limits = Limits::default();
        let written: Vec<String> = Translation::ALL
            .iter()
            .map(|&t| {
                let options = Options::default().with_translation(t);
                translate(&sequent, &options, &limits).unwrap().to_string()
            })
            .collect();
        assert_eq!(
            written,
            [
                "¬¬a, ¬¬(¬¬a → ¬¬b) ⊢ ¬¬(¬¬b ∨ ¬¬c)",
                "¬¬a, ¬¬a → ¬¬b ⊢ ¬(¬¬¬b ∧ ¬¬¬c)",
                "¬¬a, ¬¬(a → b) ⊢ ¬¬(b ∨ c)",
                "a, a → b ⊢ ¬¬(b ∨ c)",
            ]
        );
    }

    /// At first order Kuroda's translation puts `¬¬` after each `∀`, Gödel
    /// and Gentzen's writes `∃x. A` as `¬∀x. ¬A`, Kolmogorov's puts `¬¬`
    /// before every subformula, and each keeps the classical verdict;
    /// Glivenko's, whose theorem fails there, refuses.
    #[test]
    fn the_translations_at_first_order() {
        let sequent: Sequent = "forall x. p(x) |- exists y. p(y)".parse().unwrap();
        let limits = Limits::default();
        let written = |t: Translation| {
            let options = Options::default().with_translation(t);
            translate(&sequent, &options, &limits).map(|s| s.to_string())
        };
        assert_eq!(
            written(Translation::Kolmogorov).unwrap(),
            "¬¬(∀x. ¬¬p(x)) ⊢ ¬¬(∃y. ¬¬p(y))"
        );
        assert_eq!(
            written(Translation::GodelGentzen).unwrap(),
            "∀x. ¬¬p(x) ⊢ ¬(∀y. ¬¬¬p(y))"
        );
        assert_eq!(
            written(Translation::Kuroda).unwrap(),
            "¬¬(∀x. ¬¬p(x)) ⊢ ¬¬(∃y. p(y))"
        );
        assert!(matches!(
            written(Translation::Glivenko),
            Err(Error::FirstOrder { .. })
        ));
        // The intuitionistic search needs more than the default three
        // copies of a quantified hypothesis under the double negations.
        let lifted = search::Options::default().with_copies(None);
        for (text, valid) in [
            ("forall x. p(x) |- exists y. p(y)", true),
            ("|- exists x. (p(x) -> forall y. p(y))", true),
            ("exists x. p(x) |- forall y. p(y)", false),
        ] {
            let sequent: Sequent = text.parse().unwrap();
            for t in [
                Translation::Kolmogorov,
                Translation::GodelGentzen,
                Translation::Kuroda,
            ] {
                let options = Options::default().with_translation(t);
                let compared = compare(&sequent, &options, &lifted, &limits, |_| false).unwrap();
                let wanted = if valid {
                    Agreement::Valid
                } else {
                    Agreement::NotValid
                };
                assert_eq!(compared.agreement, wanted, "{text} by {t}");
            }
        }
    }

    /// Every translation keeps the classical verdict of sequents that
    /// minimal or intuitionistic logic alone would answer otherwise, and
    /// of sequents that are not valid at all: `compare` decides both and
    /// refuses a disagreement.
    #[test]
    fn every_translation_keeps_the_verdict() {
        let cases = [
            ("a \\/ ~a", true),
            ("~~a -> a", true),
            ("((a -> b) -> a) -> a", true),
            ("false -> a", true),
            ("a -> b |- ~a \\/ b", true),
            ("~(a /\\ b) |- ~a \\/ ~b", true),
            ("a <-> b, b |- a", true),
            ("a |- b, a", true),
            ("a \\/ b |- a", false),
            ("a -> b", false),
            ("~a, b |-", false),
            ("true |- a <-> ~a", false),
        ];
        for (text, valid) in cases {
            let sequent: Sequent = text.parse().unwrap();
            for &t in Translation::ALL {
                let options = Options::default().with_translation(t);
                let comparison = compare_default(&sequent, &options)
                    .unwrap_or_else(|e| panic!("{text} by {t}: {e}"));
                let expected = if valid {
                    Agreement::Valid
                } else {
                    Agreement::NotValid
                };
                assert_eq!(comparison.agreement, expected, "{text} by {t}");
                assert_eq!(comparison.decided.logic, t.target());
            }
        }
    }

    /// Generated sequents keep their verdict under every translation.
    #[test]
    fn generated_sequents_keep_their_verdicts() {
        use crate::linear::search::generate::Rng;
        let mut rng = Rng::new(63);
        for _ in 0..200 {
            let sequent = super::super::tests::generated_sequent(&mut rng);
            for &t in Translation::ALL {
                let options = Options::default().with_translation(t);
                if let Err(error) = compare_default(&sequent, &options) {
                    panic!("{sequent} by {t}: {error}");
                }
            }
        }
    }

    /// A translation is refused into a logic where its claim does not
    /// hold, and past the occurrence bound.
    #[test]
    fn targets_and_bounds_are_kept() {
        let sequent: Sequent = "a -> a".parse().unwrap();
        let limits = Limits::default();
        for (t, logic, ok) in [
            (Translation::Kuroda, Logic::Minimal, false),
            (Translation::Glivenko, Logic::Minimal, false),
            (Translation::Kolmogorov, Logic::Classical, false),
            (Translation::Kolmogorov, Logic::Intuitionistic, true),
        ] {
            let options = Options::default()
                .with_translation(t)
                .with_target(Some(logic));
            assert_eq!(
                translate(&sequent, &options, &limits).is_ok(),
                ok,
                "{t} into {logic}"
            );
        }
        let small = Limits::default().with_occurrences(Some(5));
        assert!(matches!(
            translate(&sequent, &Options::default(), &small),
            Err(Error::Refused(crate::Refusal::Occurrences { .. }))
        ));
    }
}
