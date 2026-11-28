// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Herbrand's theorem: a first-order sequent is valid in classical logic
//! exactly when finitely many ground instances of the clauses of its
//! negation have no model.
//!
//! [`herbrand`] makes the sequent's clausal form
//! ([`normal::Form::Clausal`], every step shown), then puts closed terms
//! for the clauses' variables, the shallowest first: the constants, then
//! each function symbol applied to them, and so on to a depth. At each
//! depth the SAT solver decides the ground clauses, whose atoms it reads
//! as propositional atoms; its clausal proof is checked. The first depth
//! whose instances have no model gives the certificate: those instances,
//! cut down to a set the solver still refutes.
//!
//! From the instances [`herbrand`] then builds a derivation of LK of the
//! sequent as written, which [`Derivation::check`] accepts. Each Skolem
//! term of the clausal form becomes an eigenvariable again, the least
//! nested first: `∀R` and `∃L` introduce the eigenvariable of the Skolem
//! term the quantifier was replaced by, and `∀L` and `∃R` take the terms
//! of the instances as witnesses. So a valid verdict rests on the checker
//! alone, not on the clausal form's construction.
//!
//! A sequent that is not valid has no such set, and the search ends
//! unknown at its depth: [`decide`](fn@super::decide) decides such a
//! sequent where grounding or the model search refutes it.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::herbrand::{self, Options};
//! use deduxit::ordinary::{Sequent, Verdict};
//!
//! let sequent: Sequent = "forall x. p(x) |- p(f(a))".parse()?;
//! let found = herbrand::herbrand_default(&sequent, &Options::default())?;
//! assert_eq!(found.ground.to_string(), "p(f(a)), ¬p(f(a)) ⊢");
//! assert!(matches!(found.verdict, Verdict::Valid(_)));
//! # Ok::<(), deduxit::Error>(())
//! ```

use super::derivation::ArenaSize;
use super::normal::{self, NormalForm, Rule as NormalRule};
use super::{Derivation, Evidence, Node, NodeId, Outcome, Sequent, Verdict};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId, Symbol};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Progress};
use crate::linear::search::Reason;
use std::cell::RefCell;

mod build;
mod matching;
mod search;

use self::build::{Branch, Build, Item, Skolemized};
use self::matching::{ground_atoms, witnesses_of};
use self::search::{Search, Universe, clauses_of, fresh_constant};

/// How deep the terms of the instances go by default.
pub const DEFAULT_DEPTH: u32 = 3;

/// The most instances cut down one by one; a larger set is kept whole.
const MOST_CUT: usize = 256;

/// How [`herbrand`] searches.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// The deepest term put for a variable, a constant being of depth 0;
    /// `None` deepens until a bound of the limits refuses.
    pub depth: Option<u32>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            depth: Some(DEFAULT_DEPTH),
        }
    }
}

impl Options {
    /// Returns the options with the depth given.
    #[must_use]
    pub const fn with_depth(self, depth: Option<u32>) -> Self {
        Self { depth }
    }
}

/// A ground instance of a clause.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instance {
    /// The clause's position among the clausal form's hypotheses.
    pub clause: usize,
    /// The closed terms put for the clause's variables, the outermost's
    /// first, in the arena of [`Herbrand::ground`].
    pub terms: Vec<FoTermId>,
    /// The instance, in that arena.
    pub formula: NodeId,
}

/// What the search cost.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Statistics {
    /// The closed terms of the deepest level tried.
    pub terms: u64,
    /// The instances decided at that level.
    pub instances: u64,
    /// The instances the certificate keeps.
    pub kept: u64,
    /// The calls of the SAT solver.
    pub calls: u64,
    /// The inferences of the derivation built.
    pub inferences: u64,
}

/// What [`herbrand`] found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Herbrand {
    /// The sequent decided.
    pub sequent: Sequent,
    /// Its clausal form, with the steps that made it.
    pub clausal: NormalForm,
    /// The depth of the terms of the last level tried.
    pub depth: u32,
    /// The ground instances of the last level tried, cut down to those the
    /// certificate keeps where the solver refuted them.
    pub instances: Vec<Instance>,
    /// The instances as a sequent, `C₁σ₁, …, Cₙσₙ ⊢`, whose arena holds the
    /// clausal form's.
    pub ground: Sequent,
    /// The SAT solver's outcome on the instances read as propositional
    /// clauses: valid with a checked clausal proof where they have no
    /// model.
    pub refutation: Option<Outcome>,
    /// The verdict: valid with the derivation of LK built from the
    /// instances, checked, or with the instances' clausal proof where a
    /// bound refused the derivation; unknown at the depth or a bound.
    pub verdict: Verdict,
    /// What the search cost.
    pub statistics: Statistics,
}

impl Herbrand {
    /// Returns what `instance` puts for its clause's variables: each
    /// variable's name, the outermost's first, with its term as text.
    pub fn bindings(&self, instance: &Instance) -> Vec<(String, String)> {
        let formulas = self.ground.formulas();
        let mut names = Vec::new();
        if let Some(&root) = self.clausal.result.left().get(instance.clause) {
            let mut node = root;
            while let Node::Forall(body) = formulas.node(node) {
                let name = formulas.first_order().binder_name(node);
                names.push(name.map_or_else(|| format!("x{}", names.len() + 1), str::to_owned));
                node = body;
            }
        }
        names
            .into_iter()
            .zip(&instance.terms)
            .map(|(name, &t)| (name, formulas.term_text(t)))
            .collect()
    }
}

/// Searches for ground instances of the clausal form of `sequent` that
/// have no model, within `options`, `limits` and `stop`, and builds the
/// derivation of LK they give.
///
/// # Errors
///
/// [`Refusal::Form`](crate::Refusal::Form), [`Refusal::Terms`](crate::Refusal::Terms) and
/// [`Refusal::Occurrences`](crate::Refusal::Occurrences)
/// where the clausal form or the instances pass the bounds of `limits`;
/// [`Error::ReadBack`] where the derivation built from a refutation is
/// rejected, a defect; the SAT solver's errors. A stop, the memory and
/// the work end the search as unknown.
pub fn herbrand(
    sequent: &Sequent,
    options: &Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Herbrand, Error> {
    herbrand_dyn(sequent, options, limits, &mut stop)
}

/// Searches as [`herbrand`] does within the default [`Limits`] and with no
/// stop.
///
/// # Errors
///
/// As [`herbrand`].
pub fn herbrand_default(sequent: &Sequent, options: &Options) -> Result<Herbrand, Error> {
    herbrand(sequent, options, &Limits::default(), |_| false)
}

/// [`herbrand`], with the stop as a trait object.
fn herbrand_dyn(
    sequent: &Sequent,
    options: &Options,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Herbrand, Error> {
    let clausal = normal::rewrite(
        sequent,
        &normal::Options::default().with_form(normal::Form::Clausal),
        limits,
        &mut *stop,
    )?;
    let mut formulas = clausal.result.formulas().clone();
    let clauses = clauses_of(&formulas, clausal.result.left());
    let universe = Universe::of(&mut formulas, &clauses)?;
    let mut search = Search {
        formulas,
        clauses,
        universe,
        limits,
        statistics: Statistics::default(),
        charged: ArenaSize::default(),
        held: 0,
    };
    let mut depth = 0;
    let (instances, refutation, reason) = loop {
        let instances = match search.instances(depth, &mut *stop) {
            Ok(instances) => instances,
            Err(error) => {
                let reason = super::decide::ended(error)?;
                break (Vec::new(), None, Some(reason));
            }
        };
        let decided = search.decide(&instances, &mut *stop);
        let decided = match decided {
            Ok(decided) => decided,
            Err(error) => {
                let reason = super::decide::ended(error)?;
                break (instances, None, Some(reason));
            }
        };
        match decided.verdict {
            Verdict::Valid(_) => {
                let (kept, decided) = search.cut(instances, decided, &mut *stop)?;
                break (kept, Some(decided), None);
            }
            // A bound or the stop ended the solver: the instances may have
            // no model.
            Verdict::Unknown(reason) => break (instances, Some(decided), Some(reason)),
            Verdict::NotValid => {}
        }
        let complete = search.universe.functions.is_empty();
        let last = options.depth.is_some_and(|most| depth >= most);
        if complete || last {
            let reason = if complete {
                Reason::Incomplete
            } else {
                Reason::WitnessDepth { depth }
            };
            break (instances, Some(decided), Some(reason));
        }
        depth += 1;
    };
    search.statistics.kept = instances.len() as u64;
    let rest = search.rest();
    let Search {
        formulas,
        universe,
        mut statistics,
        ..
    } = search;
    let ground = Sequent::new(
        formulas,
        instances.iter().map(|i| i.formula).collect(),
        Vec::new(),
    )?;
    let verdict = match (reason, &refutation) {
        (Some(reason), _) => Verdict::Unknown(reason),
        (None, Some(decided)) => {
            let constants = universe.levels.first().map_or(&[][..], Vec::as_slice);
            match deskolemized(sequent, &clausal, &ground, constants, &rest, &mut *stop) {
                Ok(derivation) => {
                    statistics.inferences = derivation.inferences().len() as u64;
                    Verdict::Valid(Evidence::Derivation(Box::new(derivation)))
                }
                Err(error) => {
                    // A bound refused the derivation: the instances' proof
                    // stands for it.
                    super::decide::ended(error)?;
                    decided.verdict.clone()
                }
            }
        }
        (None, None) => unreachable!("a refutation is kept with its verdict"),
    };
    Ok(Herbrand {
        sequent: sequent.clone(),
        clausal,
        depth,
        instances,
        ground,
        refutation: refutation.map(Outcome::from),
        verdict,
        statistics,
    })
}

/// Returns the derivation of LK of `sequent` that the instances of
/// `ground` give, each Skolem term of `clausal` an eigenvariable, checked.
/// The first of `constants` that is no Skolem term, or a new constant
/// where none is, is the witness a first `∀L` or `∃R` may take before any
/// eigenvariable exists.
pub(super) fn deskolemized(
    sequent: &Sequent,
    clausal: &NormalForm,
    ground: &Sequent,
    constants: &[FoTermId],
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Derivation, Error> {
    // The clausal arena began as a copy of the input's, so the input's
    // ids are its ids.
    let mut formulas = ground.formulas().clone();
    let skolems: Vec<Skolemized> = clausal
        .steps
        .iter()
        .filter(|s| s.rule == NormalRule::Skolemize)
        .filter_map(|s| {
            s.term.map(|term| Skolemized {
                before: s.before,
                after: s.after,
                term,
            })
        })
        .collect();
    let symbols: HashSet<Symbol> = skolems
        .iter()
        .filter_map(|k| match formulas.first_order().terms[k.term.index()] {
            FoTerm::App(f, _) => Some(f),
            FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => None,
        })
        .collect();
    let mut witnesses = witnesses_of(&formulas, ground.left());
    // A first `∀L` or `∃R` may need a witness before any `∀R` or `∃L` has
    // introduced a Skolem term: a constant of the universe that is none.
    let skolem = |t: FoTermId| match formulas.first_order().terms[t.index()] {
        FoTerm::App(f, _) => symbols.contains(&f),
        FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => false,
    };
    let plain = constants.iter().copied().find(|&t| !skolem(t));
    let plain = match plain {
        Some(t) => Some(t),
        None if sequent.formulas().is_first_order() => Some(fresh_constant(&mut formulas)?),
        None => None,
    };
    if let Some(plain) = plain
        && !witnesses.contains(&plain)
    {
        witnesses.insert(0, plain);
    }
    let clauses = ground_atoms(&formulas, ground.left());
    let mut build = Build {
        charged: ArenaSize::default(),
        formulas,
        skolems,
        symbols,
        witnesses,
        clauses,
        given: HashMap::default(),
        keys: HashMap::default(),
        instances: HashMap::default(),
        made: Vec::new(),
        inferences: Vec::new(),
        bytes: 0,
        expanded: 0,
        skolem_memo: RefCell::default(),
        nesting_memo: RefCell::default(),
        limits,
    };
    // The arena is a copy of the instances': it is charged whole first.
    build.charge_arena()?;
    let mut root = Branch::default();
    for &node in sequent.left() {
        root.items.push(Item::new(node, true));
    }
    for &node in sequent.right() {
        root.items.push(Item::new(node, false));
    }
    build.derive(root, stop)?;
    let derivation = build.deskolemized(sequent)?;
    derivation
        .check(limits, &mut *stop)
        .map_err(|e| derivation.own_fault(e))?;
    Ok(derivation)
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::limits::Phase;
    use crate::ordinary::Logic;

    /// Returns what Herbrand's theorem finds for `text` by default.
    fn found(text: &str) -> Herbrand {
        let sequent: Sequent = text.parse().unwrap();
        herbrand_default(&sequent, &Options::default()).unwrap()
    }

    /// A valid sequent gets instances the solver refutes and a checked
    /// derivation of LK of the sequent as written, its Skolem terms
    /// eigenvariables again.
    #[test]
    fn a_valid_sequent_gets_its_instances_and_a_derivation() {
        for text in [
            "forall x. p(x) |- p(f(a))",
            "|- exists x. (p(x) -> forall y. p(y))",
            "forall x. exists y. r(x, y) |- forall x. exists y. exists z. (r(x, y) /\\ r(x, z))",
            "exists x. forall y. r(x, y) |- forall y. exists x. r(x, y)",
            "forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(a)))",
            "|- (forall x. p(x)) <-> ~(exists x. ~p(x))",
            "p |- p \\/ q",
            // An instance of the first hypothesis is the second: the kept
            // instances' term decides which Skolem term it stands for.
            "forall y. exists x. p(x, y), exists x. p(x, a) |- exists x. ~~p(x, a)",
            // The clausal form Skolemizes the hypothesis's `∃x. p(x)` first,
            // inside the goal's quantifier too.
            "~((exists x. p(x)) -> (r \\/ p(a))) |- forall y. ((exists x. p(x)) -> (exists x. p(x)))",
            // A closed instance of an open step's quantifier is another
            // node, which the clausal form left alone.
            "forall y. exists x. p(x, y), exists z. (q(z) /\\ exists x. p(x, a)) |- exists z. q(z)",
        ] {
            let found = found(text);
            let Verdict::Valid(Evidence::Derivation(derivation)) = &found.verdict else {
                panic!("{text}: {:?}", found.verdict);
            };
            let (left, right) = derivation.sequent();
            assert_eq!(left, found.sequent.left(), "{text}");
            assert_eq!(right, found.sequent.right(), "{text}");
            derivation.check(&Limits::default(), |_| false).unwrap();
            assert!(
                found
                    .refutation
                    .as_ref()
                    .is_some_and(|r| matches!(r.verdict, Verdict::Valid(_))),
                "{text}"
            );
            assert!(!found.instances.is_empty(), "{text}");
        }
    }

    /// Two steps of the rule need the terms up to depth 1, whose instances
    /// are six: the certificate keeps the four the solver needs, none of
    /// `q`.
    #[test]
    fn the_certificate_keeps_the_instances_it_needs() {
        let found = found("forall x. (p(x) -> p(f(x))), forall x. q(x), p(a) |- p(f(f(a)))");
        assert_eq!(found.depth, 1);
        assert_eq!(found.statistics.instances, 6);
        assert_eq!(found.instances.len(), 4);
        assert_eq!(found.statistics.kept, 4);
    }

    /// A sequent that is not valid ends unknown: at the depth where
    /// function symbols make the universe grow, and as incomplete where
    /// its universe is finite and every instance was tried.
    #[test]
    fn a_sequent_not_valid_ends_unknown() {
        let deep = found("forall x. (p(x) -> p(f(x))) |- p(a)");
        assert!(matches!(
            deep.verdict,
            Verdict::Unknown(Reason::WitnessDepth {
                depth: DEFAULT_DEPTH
            })
        ));
        let finite = found("exists x. p(x) |- forall y. p(y)");
        assert!(matches!(
            finite.verdict,
            Verdict::Unknown(Reason::Incomplete)
        ));
    }

    /// Two different quantifiers of one variable get two eigenvariables:
    /// the witness of one is no witness of the other.
    #[test]
    fn different_quantifiers_get_different_eigenvariables() {
        let valid =
            found("(exists x. p(x)) /\\ (exists x. q(x)) |- exists x. p(x) /\\ exists y. q(y)");
        assert!(matches!(
            valid.verdict,
            Verdict::Valid(Evidence::Derivation(_))
        ));
        let not = found("(exists x. p(x)) /\\ (exists x. q(x)) |- exists x. (p(x) /\\ q(x))");
        assert!(matches!(not.verdict, Verdict::Unknown(_)));
    }

    /// A propositional sequent keeps an arena without first-order tables,
    /// so its ground sequent and derivation read back as written.
    #[test]
    fn a_propositional_sequent_stays_propositional() {
        let found = found("p |- p \\/ q");
        assert!(!found.ground.formulas().is_first_order());
        let Verdict::Valid(Evidence::Derivation(derivation)) = &found.verdict else {
            panic!("{:?}", found.verdict);
        };
        assert!(!derivation.formulas().is_first_order());
    }

    /// A bound that ends the solver, or the stop while the instances are
    /// made, ends the search unknown for that reason, never as instances
    /// with a model.
    #[test]
    fn a_bound_or_the_stop_ends_the_search_for_its_reason() {
        let sequent: Sequent = "forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(a)))"
            .parse()
            .unwrap();
        let options = Options::default().with_depth(None);
        // Whichever phase meets it, the bound named is the one given, never
        // the share a phase had.
        let mut refused = 0;
        for bound in (10..24).map(|k| 1u64 << k) {
            let small = Limits::default().with_memory_bytes(Some(bound));
            let found = herbrand(&sequent, &options, &small, |_| false).unwrap();
            if let Verdict::Unknown(Reason::MemoryLimit { limit_bytes }) = found.verdict {
                assert_eq!(limit_bytes, bound);
                refused += 1;
            }
        }
        assert!(refused > 0);
        let deep: Sequent = "forall x y z. (q(x, y, z) -> q(g(x, y), z, x)) |- q(a, a, a) /\\ r"
            .parse()
            .unwrap();
        let found = herbrand(&deep, &options, &Limits::default(), |p| {
            p.phase == Phase::Search && p.done >= 4096
        })
        .unwrap();
        assert!(
            matches!(found.verdict, Verdict::Unknown(Reason::Stopped)),
            "{:?}",
            found.verdict
        );
    }

    /// The read-back charges the arena it grows: the instances of many
    /// nested quantifiers over an atom of as many arguments are refused at
    /// the memory bound, and the verdict keeps the instances' clausal
    /// proof, where without the charge the derivation was built past the
    /// bound.
    #[test]
    fn the_read_back_keeps_to_the_memory_bound() {
        let n = 300;
        let variables: Vec<String> = (0..n).map(|i| format!("x{i}")).collect();
        let text = format!(
            "forall {}. p({}) |- p({})",
            variables.join(" "),
            variables.join(", "),
            vec!["a"; n].join(", ")
        );
        let sequent: Sequent = text.parse().unwrap();
        let options = Options::default();
        let roomy = herbrand(&sequent, &options, &Limits::default(), |_| false).unwrap();
        assert!(matches!(
            &roomy.verdict,
            Verdict::Valid(Evidence::Derivation(_))
        ));
        // Four MiB holds the clausal form and the instances, not the arena
        // the derivation's instances grow (about 11 MiB charged here).
        let tight = Limits::default().with_memory_bytes(Some(4 << 20));
        let found = herbrand(&sequent, &options, &tight, |_| false).unwrap();
        let kind = match &found.verdict {
            Verdict::Valid(Evidence::Clausal(_)) => "clausal proof",
            Verdict::Valid(_) => "derivation",
            Verdict::Unknown(_) => "unknown",
            Verdict::NotValid => "not valid",
        };
        assert_eq!(kind, "clausal proof");
    }

    /// Each quantifier takes first the witnesses its kept instances give:
    /// three nested `∀` over three constants take one each, where every
    /// witness in every `∀` made a derivation of 175 inferences.
    #[test]
    fn each_quantifier_takes_the_witnesses_its_instances_give() {
        let found = found(
            "p(c1), p(c2), p(c3), forall x1 x2 x3. (p(x1) /\\ p(x2) /\\ p(x3) -> q(x1, x2, x3)) \
             |- q(c3, c2, c1)",
        );
        let Verdict::Valid(Evidence::Derivation(derivation)) = &found.verdict else {
            panic!("a derivation");
        };
        assert!(
            derivation.inferences().len() <= 40,
            "{}",
            derivation.inferences().len()
        );
    }

    /// Generated first-order sequents: a valid verdict always has its
    /// derivation checked and is never refuted classically, and most
    /// sequents the default prover finds valid are found valid here.
    #[test]
    fn generated_sequents_agree_with_the_default_prover() {
        use crate::linear::search::generate::Rng;
        use crate::ordinary::normal::first_order_tests::formula;
        let mut rng = Rng::new(77);
        let limits = Limits::default().with_work(Some(500_000));
        let options = Options::default().with_depth(Some(2));
        let (mut sequents, mut valid, mut found) = (0, 0, 0);
        while sequents < 300 {
            let (left, right) = (
                formula(&mut rng, 3, &mut Vec::new()),
                formula(&mut rng, 3, &mut Vec::new()),
            );
            let Ok(sequent) = format!("{left} |- {right}").parse::<Sequent>() else {
                continue;
            };
            if !sequent.formulas().is_first_order() {
                continue;
            }
            sequents += 1;
            let classical = crate::ordinary::decide_within(
                &sequent,
                &crate::ordinary::Options::default().with_logic(Logic::Classical),
                &crate::linear::search::Options::default(),
                &limits,
                |_| false,
            );
            let classical = classical.map(|o| o.verdict);
            let herbrand = match herbrand(&sequent, &options, &limits, |_| false) {
                Ok(herbrand) => herbrand,
                Err(Error::Refused(_)) => continue,
                Err(error) => panic!("{sequent}: {error}"),
            };
            if matches!(classical, Ok(Verdict::Valid(_))) {
                valid += 1;
            }
            if let Verdict::Valid(evidence) = &herbrand.verdict {
                assert!(!matches!(classical, Ok(Verdict::NotValid)), "{sequent}");
                if let Evidence::Derivation(derivation) = evidence {
                    derivation.check(&Limits::default(), |_| false).unwrap();
                    found += 1;
                }
            }
        }
        assert!(found * 4 >= valid * 3, "{found} of {valid} valid found");
    }
}
