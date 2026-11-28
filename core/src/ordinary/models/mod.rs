// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A search for a finite structure that refutes a first-order sequent,
//! in the manner of MACE and Paradox: for domains of 1, 2, … elements up
//! to a bound, the sequent's falsity in a structure of that size is
//! written as a propositional formula, and the SAT solver looks for a
//! model of it.
//!
//! A sequent `Γ ⊢ Δ` fails in a structure exactly when `⋀Γ → ⋁Δ` is false
//! there. Over the domain `{0, …, n−1}` the search takes that formula in
//! negation normal form, miniscoped (grounding's normal form, function
//! symbols kept), and writes `∀x. A` as the conjunction of `A` at every
//! element and `∃x. A` as the disjunction: no Skolem function is needed.
//! A function symbol `f` gets an atom `f(d̄) ↦ e` for each argument tuple
//! `d̄` the formula reaches and each value `e`, a constant `c ↦ e`; the
//! hypotheses say each such term has exactly one value (Paradox's
//! totality and functionality), and a term denotes `e` where its
//! arguments denote some `ē` and `f(ē) ↦ e` holds. A model of the
//! hypotheses that falsifies the formula is a structure of `n` elements
//! that refutes the sequent. Clauses after Paradox's put the constants in
//! the order they first name elements, which loses no structure up to
//! isomorphism.
//!
//! The sources: W. McCune, *MACE 2.0 reference manual and guide*
//! (arXiv cs/0106042) and *Mace4 reference manual and guide* (arXiv
//! cs/0310055); G. Reger, M. Suda and A. Voronkov, *Finding finite models
//! in multi-sorted first-order logic* (SAT 2016, arXiv 1604.08040), whose
//! §3 describes K. Claessen and N. Sörensson's Paradox.
//!
//! The search is untrusted. The structure read off the solver's model is
//! checked by [`FiniteCountermodel::check`] against the sequent as
//! written before it is an answer: one that fails is
//! [`Error::Countermodel`], a defect. A classical countermodel is a Kripke
//! model of one world, so "not valid" holds in every logic. No domain
//! without a model proves anything: past the bound the answer is unknown
//! ([`Reason::ModelSize`]).

/// What the SAT encodings of first-order models share, which grounding
/// shares too.
pub(super) mod encode;
/// The formula that a finite structure of a given size refutes the
/// sequent, and the structure read off its model.
mod finite;
/// The propositional sequent of a Kripke model's search.
mod kripke;

use self::finite::{Expansion, Signature};
use super::grounding;
use super::sat;
use super::{FiniteCountermodel, KripkeCountermodel, Logic, Sequent, Verdict};
use crate::Error;
use crate::limits::{Limits, Progress, Refusal};
use crate::linear::search::{self, Reason};

/// What the finite model search cost.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct Statistics {
    /// The largest domain tried, in elements.
    pub size: u64,
    /// The most worlds of the last Kripke model tried, in intuitionistic
    /// and minimal logic; 0 where only finite structures were tried (and
    /// then left out of the JSON form).
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "is_zero"))]
    pub worlds: u64,
    /// The atoms of the last domain's formula: the predicates' and the
    /// terms' values.
    pub atoms: u64,
    /// The formulas of the last domain's arena.
    pub nodes: u64,
    /// What the SAT solver's search on the last domain cost.
    pub sat: sat::Statistics,
}

/// Whether a count is zero, which the JSON form leaves out.
#[cfg(feature = "serialize")]
#[cfg_attr(
    target_pointer_width = "64",
    expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "serde's skip_serializing_if passes a reference"
    )
)]
const fn is_zero(count: &u64) -> bool {
    *count == 0
}

/// What the finite model search found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict: not valid, or unknown (past the bound, at another
    /// bound, or where the sequent names a term no structure interprets).
    /// Never valid.
    pub verdict: Verdict,
    /// The logic decided.
    pub logic: Logic,
    /// The finite structure that refutes a sequent that is not valid,
    /// checked by [`FiniteCountermodel::check`].
    pub structure: Option<FiniteCountermodel>,
    /// The Kripke model with growing domains that refutes a sequent that is
    /// not valid in intuitionistic or minimal logic where no finite
    /// structure does, checked by [`KripkeCountermodel::check`].
    pub kripke: Option<KripkeCountermodel>,
    /// What the search cost.
    pub statistics: Statistics,
}

/// Looks for a finite structure that refutes `sequent`, on domains of 1,
/// 2, … elements up to `options.model_size`, and in intuitionistic and
/// minimal logic for a Kripke model with growing domains beside it, of
/// up to `options.model_worlds` worlds, within `limits`, asking `stop` as
/// it goes. For L = 1, 2, … it tries the structures of L elements, then
/// the Kripke models of L worlds (or the bound) of up to L elements a
/// world (or the bound): a model fits every larger pair.
///
/// The verdict is not valid with a checked [`Structure`](super::Structure)
/// (in every logic: a classical countermodel is a Kripke model of one
/// world) or a checked [`KripkeStructure`](super::KripkeStructure), or
/// unknown:
/// [`Reason::ModelSize`] past the bound in classical logic and
/// [`Reason::KripkeSize`] past both in the others, the reason of the
/// bound or the stop that ended the search, and [`Reason::Incomplete`] for
/// a sequent over a derivation's arena that names an eigenvariable or an
/// open witness, which no structure interprets. Each domain is written within
/// `limits.occurrences` formulas and `limits.memory_bytes`, and the
/// solver gets `limits.work` on each.
///
/// # Errors
///
/// [`Error::Succedents`] for more than one formula right of `⊢` outside
/// classical logic; [`Error::Countermodel`] where the structure read off
/// does not refute the sequent, a defect.
pub fn decide(
    sequent: &Sequent,
    logic: Logic,
    options: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    // A sequent with `=` is searched with the equality axioms, which make
    // a structure's `=` a congruence that it is divided by.
    let bounds = (options.model_size, options.model_worlds);
    match super::equality::with_axioms(sequent, limits)? {
        Some(with) => run(&with, logic, bounds, limits, &mut stop),
        None => run(sequent, logic, bounds, limits, &mut stop),
    }
}

/// [`decide`] with the bounds alone and the stop as a trait object.
fn run(
    sequent: &Sequent,
    logic: Logic,
    (bound, worlds_bound): (Option<u32>, Option<u32>),
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    if logic != Logic::Classical && sequent.right().len() > 1 {
        return Err(Error::Succedents {
            count: sequent.right().len(),
        });
    }
    let mut statistics = Statistics::default();
    let unknown = |reason, statistics| {
        Ok(Outcome {
            verdict: Verdict::Unknown(reason),
            logic,
            structure: None,
            kripke: None,
            statistics,
        })
    };
    let (nnf, root) = match grounding::normal_form(sequent, limits, &mut *stop, true) {
        Ok(Some(prepared)) => prepared,
        Ok(None) => return unknown(Reason::Incomplete, statistics),
        Err(error) => return unknown(unknown_for(error, limits)?, statistics),
    };
    let signature = Signature::of(&nnf, root);
    let most = bound.unwrap_or(u32::MAX);
    // A bound of no element leaves nothing to try.
    if most == 0 {
        let reason = match logic {
            Logic::Classical => Reason::ModelSize { size: 0 },
            Logic::Intuitionistic | Logic::Minimal => Reason::KripkeSize {
                worlds: worlds_bound.unwrap_or(u32::MAX),
                size: 0,
            },
        };
        return unknown(reason, statistics);
    }
    // With `=` no Kripke model is tried, so the levels are the structures'
    // sizes alone: the Kripke encoding reads `=` as a predicate, and the
    // check reads it as identity at every world and refuses a world that
    // lists it. (With the worlds unbounded the loop once ran to
    // `u32::MAX` levels without a poll, then asked for a model of that
    // many worlds and overflowed.)
    let equality = sequent.formulas().has_equality();
    let worlds = match logic {
        Logic::Classical => 1,
        Logic::Intuitionistic | Logic::Minimal if equality => 1,
        Logic::Intuitionistic | Logic::Minimal => worlds_bound.unwrap_or(u32::MAX).max(1),
    };
    // Minimal logic's `⊥` makes a model of one world more than a finite
    // structure, which forces it nowhere.
    let least = if equality {
        u32::MAX
    } else {
        match logic {
            Logic::Minimal => 1,
            Logic::Classical | Logic::Intuitionistic => 2,
        }
    };
    for level in 1..=most.max(worlds) {
        // A Kripke model, on a pair not tried yet.
        let (k, n) = (level.min(worlds), level.min(most));
        if k >= least && level > most {
            match kripke(sequent, logic, (k, n), &mut statistics, limits, &mut *stop) {
                Ok(Some(outcome)) => return Ok(outcome),
                Ok(None) => continue,
                Err(Ok(reason)) => return unknown(reason, statistics),
                Err(Err(error)) => return Err(error),
            }
        }
        if level > most {
            continue;
        }
        let size = level;
        statistics.size = u64::from(size);
        let expansion = match Expansion::new(&nnf, root, &signature, size, limits, &mut *stop) {
            Ok(expansion) => expansion,
            Err(error) => return unknown(unknown_for(error, limits)?, statistics),
        };
        statistics.atoms = expansion.entries.len() as u64;
        statistics.nodes = expansion.sequent.formulas().len() as u64;
        let rest = limits.with_memory_bytes(
            limits
                .memory_bytes
                .map(|m| m.saturating_sub(expansion.bytes)),
        );
        let (found, solved) = sat::falsify(&expansion.sequent, &rest, &mut *stop);
        statistics.sat = solved;
        let true_atoms = match found {
            Ok(Some(atoms)) => atoms,
            Ok(None) if k >= least => {
                match kripke(sequent, logic, (k, n), &mut statistics, limits, &mut *stop) {
                    Ok(Some(outcome)) => return Ok(outcome),
                    Ok(None) => continue,
                    Err(Ok(reason)) => return unknown(reason, statistics),
                    Err(Err(error)) => return Err(error),
                }
            }
            Ok(None) => continue,
            Err(Reason::MemoryLimit { limit_bytes }) => {
                let limit_bytes = limits.memory_bytes.unwrap_or(limit_bytes);
                return unknown(Reason::MemoryLimit { limit_bytes }, statistics);
            }
            Err(reason) => return unknown(reason, statistics),
        };
        let structure = match expansion.structure(sequent, &signature, &true_atoms, limits) {
            Ok(structure) => structure,
            Err(error) => return unknown(unknown_for(error, limits)?, statistics),
        };
        drop(expansion);
        // The structure satisfies the equality axioms, so its `=` is a
        // congruence; divided by it, `=` is identity, as a structure
        // reads it.
        let structure = if sequent.formulas().has_equality() {
            super::equality::identified(structure)
        } else {
            structure
        };
        let countermodel = FiniteCountermodel::new(sequent.clone(), structure);
        return match countermodel.check(logic, limits, &mut *stop) {
            Ok(()) => Ok(Outcome {
                verdict: Verdict::NotValid,
                logic,
                structure: Some(countermodel),
                kripke: None,
                statistics,
            }),
            // The verdict rests on the structure alone, so a check that a
            // bound or the stop gives up leaves the sequent unknown.
            Err(Error::Structure { reason }) => Err(Error::Countermodel {
                reason: format!("the finite structure the model search found is wrong: {reason}"),
            }),
            Err(error) => unknown(unknown_for(error, limits)?, statistics),
        };
    }
    let reason = if logic != Logic::Classical && !equality {
        Reason::KripkeSize { worlds, size: most }
    } else {
        Reason::ModelSize { size: most }
    };
    unknown(reason, statistics)
}

/// Looks for a Kripke model of `worlds` worlds with domains of at most
/// `size` elements that refutes the sequent, and checks the one found:
/// the outcome where one refutes it, `None` where none does, the reason
/// (`Err(Ok(…))`) where a bound or the stop ends the search, and an error
/// (`Err(Err(…))`) for a model that fails its check, a defect, or another
/// refusal.
fn kripke(
    sequent: &Sequent,
    logic: Logic,
    (worlds, size): (u32, u32),
    statistics: &mut Statistics,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Option<Outcome>, Result<Reason, Error>> {
    statistics.worlds = u64::from(worlds);
    statistics.size = u64::from(size);
    let written = kripke::write(sequent, logic, worlds, size, limits, &mut *stop)
        .map_err(|error| unknown_for(error, limits))?;
    statistics.atoms = written.atoms();
    statistics.nodes = written.sequent.formulas().len() as u64;
    let rest =
        limits.with_memory_bytes(limits.memory_bytes.map(|m| m.saturating_sub(written.bytes)));
    let (found, solved) = sat::falsify(&written.sequent, &rest, &mut *stop);
    statistics.sat = solved;
    let true_atoms = match found {
        Ok(Some(atoms)) => atoms,
        Ok(None) => return Ok(None),
        Err(Reason::MemoryLimit { limit_bytes }) => {
            let limit_bytes = limits.memory_bytes.unwrap_or(limit_bytes);
            return Err(Ok(Reason::MemoryLimit { limit_bytes }));
        }
        Err(reason) => return Err(Ok(reason)),
    };
    let model = written
        .model(sequent, &true_atoms, limits)
        .map_err(|error| unknown_for(error, limits))?;
    drop(written);
    let countermodel = KripkeCountermodel::new(sequent.clone(), model);
    match countermodel.check(logic, limits, &mut *stop) {
        Ok(()) => Ok(Some(Outcome {
            verdict: Verdict::NotValid,
            logic,
            structure: None,
            kripke: Some(countermodel),
            statistics: *statistics,
        })),
        Err(Error::Structure { reason }) => Err(Err(Error::Countermodel {
            reason: format!("the Kripke model the model search found is wrong: {reason}"),
        })),
        Err(error) => Err(unknown_for(error, limits)),
    }
}

/// Returns the reason the stop or a bound on work or memory leaves the
/// search unknown for, and any other error as it is (a domain whose
/// formula passes `limits.occurrences` among them).
fn unknown_for(error: Error, limits: &Limits) -> Result<Reason, Error> {
    let Error::Refused(refusal) = &error else {
        return Err(error);
    };
    if let Refusal::Stopped { .. } = refusal {
        return Ok(Reason::Stopped);
    }
    if let Refusal::Memory { limit_bytes, .. } = refusal {
        return Ok(Reason::MemoryLimit {
            limit_bytes: limits.memory_bytes.unwrap_or(*limit_bytes),
        });
    }
    if let Refusal::Work { limit } = refusal {
        return Ok(Reason::WorkLimit { limit: *limit });
    }
    Err(error)
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::decide;
    use crate::Limits;
    use crate::linear::search::generate::Rng;
    use crate::linear::search::{self, Reason};
    use crate::ordinary::{Evaluation, Logic, Sequent, Verdict};

    /// Searches `text` in `logic` within the default limits and a bound
    /// of `size`.
    fn search(text: &str, logic: Logic, size: u32) -> super::Outcome {
        let sequent: Sequent = text.parse().unwrap();
        let options = search::Options::default().with_model_size(Some(size));
        decide(&sequent, logic, &options, &Limits::default(), |_| false)
            .unwrap_or_else(|e| panic!("{text}: {e}"))
    }

    /// Sequents decided by hand: those that a finite structure refutes get
    /// one of the size expected, checked, in every logic; the valid ones,
    /// and one that only infinite structures refute, are unknown past the
    /// bound.
    #[test]
    fn sequents_decided_by_hand() {
        let refuted = [
            ("p(a) |- p(f(a))", 2),
            ("exists x. p(x) |- forall x. p(x)", 2),
            (
                "forall x. exists y. r(x, y) |- exists y. forall x. r(x, y)",
                2,
            ),
            ("forall x. r(x, f(x)) |- exists x. r(f(x), x)", 3),
            ("forall x. p(f(x)) |- p(a)", 2),
            ("forall x. p(g(x, x)) |- p(g(a, b))", 2),
            ("forall x. p(g(x, f(x))) |- forall x. p(x)", 2),
            ("p(a), ~p(b), ~p(c) |- p(f(b)) \\/ p(f(c)) \\/ ~p(f(a))", 2),
            ("forall x. (p(x) -> p(f(x))) |- forall x. p(x)", 1),
        ];
        for (text, size) in refuted {
            for &logic in Logic::ALL {
                let outcome = search(text, logic, 4);
                assert!(
                    matches!(outcome.verdict, Verdict::NotValid),
                    "{text}: {:?}",
                    outcome.verdict
                );
                let countermodel = outcome.structure.expect("a structure");
                assert_eq!(countermodel.structure.size, size, "{text}");
                assert_eq!(outcome.statistics.size, u64::from(size));
                let sequent: Sequent = text.parse().unwrap();
                assert_eq!(
                    countermodel
                        .structure
                        .refutes(&sequent, &Limits::default(), |_| false)
                        .unwrap(),
                    Evaluation::Refutes,
                    "{text}"
                );
            }
        }
        let unknown = [
            "forall x. p(x) |- p(f(a))",
            "forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(a)))",
            "|- exists x. (p(x) -> forall y. p(y))",
            // Only an infinite structure refutes it: a strict order with
            // no greatest element.
            "forall x. ~r(x, x), forall x y z. (r(x, y) /\\ r(y, z) -> r(x, z)), \
             forall x. exists y. r(x, y) |-",
        ];
        for text in unknown {
            let outcome = search(text, Logic::Classical, 3);
            assert!(
                matches!(
                    outcome.verdict,
                    Verdict::Unknown(Reason::ModelSize { size: 3 })
                ),
                "{text}: {:?}",
                outcome.verdict
            );
        }
        // A table no allocation holds is the memory bound's refusal, an
        // unknown, with or without a bound.
        let wide = format!(
            "p(a), ~p(b), forall x. r(g({})) |- r(a)",
            vec!["x"; 70].join(", ")
        );
        for memory in [None, Some(1 << 26)] {
            let sequent: Sequent = wide.parse().unwrap();
            let limits = Limits::default().with_memory_bytes(memory);
            let options = search::Options::default().with_model_size(Some(3));
            let outcome = decide(&sequent, Logic::Classical, &options, &limits, |_| false).unwrap();
            assert!(
                matches!(
                    outcome.verdict,
                    Verdict::Unknown(Reason::MemoryLimit { .. })
                ),
                "{:?}",
                outcome.verdict
            );
        }
    }

    /// The default of a first-order sequent the first attempts of the
    /// provers and the embedding leave open asks the model search, in every
    /// logic, and the method named runs alone.
    #[test]
    fn the_default_asks_the_model_search() {
        use crate::ordinary::{Method, Options, Search};
        let sequent: Sequent = "forall x. (p(x) -> p(f(x))), p(a) |- p(b)".parse().unwrap();
        for &logic in Logic::ALL {
            let outcome = crate::ordinary::decide(&sequent, &Options::default().with_logic(logic))
                .unwrap_or_else(|e| panic!("{logic}: {e}"));
            assert!(matches!(outcome.verdict, Verdict::NotValid), "{logic}");
            assert!(matches!(outcome.search, Search::Models(_)), "{logic}");
            assert!(outcome.structure.is_some());
        }
        // A theorem keeps the prover's derivation.
        let valid: Sequent = "forall x. p(x) |- p(f(a))".parse().unwrap();
        let outcome = crate::ordinary::decide(&valid, &Options::default()).unwrap();
        assert!(matches!(outcome.verdict, Verdict::Valid(_)));
        let named = Options::default().with_method(Some(Method::Models));
        let outcome = crate::ordinary::decide(&valid, &named.with_logic(Logic::Classical)).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::ModelSize { size: 6 })
        ));
        let outcome =
            crate::ordinary::decide(&valid, &named.with_logic(Logic::Intuitionistic)).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::KripkeSize { worlds: 4, size: 6 })
        ));
    }

    /// Searches `text` in `logic` with bounds on elements and worlds.
    fn kripke(text: &str, logic: Logic, size: u32, worlds: u32) -> super::Outcome {
        let sequent: Sequent = text.parse().unwrap();
        let options = search::Options::default()
            .with_model_size(Some(size))
            .with_model_worlds(Some(worlds));
        decide(&sequent, logic, &options, &Limits::default(), |_| false)
            .unwrap_or_else(|e| panic!("{text}: {e}"))
    }

    /// Sequents valid classically and not intuitionistically get a checked
    /// Kripke model of at most the worlds and elements expected; minimal logic's
    /// `⊥` is forced where it must be; the double-negation shift and the
    /// double negation of excluded middle at first order, which only
    /// infinite models refute, are unknown at the bounds.
    #[test]
    fn kripke_models_decided_by_hand() {
        let refuted = [
            ("|- p(a) \\/ ~p(a)", Logic::Intuitionistic, 2, 1),
            ("|- forall x. (p(x) \\/ ~p(x))", Logic::Intuitionistic, 2, 2),
            ("~~p(a) |- p(a)", Logic::Intuitionistic, 2, 1),
            ("forall x. ~~p(x) |- p(f(a))", Logic::Intuitionistic, 2, 1),
            (
                "(forall x. p(x)) -> q |- exists x. (p(x) -> q)",
                Logic::Intuitionistic,
                2,
                2,
            ),
            ("false |- p(a)", Logic::Minimal, 1, 1),
            ("|- ~p(a) \\/ ~~p(a)", Logic::Minimal, 3, 3),
        ];
        for (text, logic, worlds, size) in refuted {
            let outcome = kripke(text, logic, 4, 3);
            assert!(
                matches!(outcome.verdict, Verdict::NotValid),
                "{text}: {:?}",
                outcome.verdict
            );
            let countermodel = outcome.kripke.expect("a Kripke model");
            assert!(countermodel.model.worlds.len() <= worlds, "{text}");
            let most = countermodel.model.worlds.iter().map(|w| w.size).max();
            assert!(most <= Some(size), "{text}");
            countermodel
                .check(logic, &Limits::default(), |_| false)
                .unwrap();
            if logic == Logic::Intuitionistic {
                countermodel
                    .check(Logic::Minimal, &Limits::default(), |_| false)
                    .unwrap();
            }
        }
        for text in [
            "forall x. ~~p(x) |- ~~forall x. p(x)",
            "|- ~~forall x. (p(x) \\/ ~p(x))",
        ] {
            let outcome = kripke(text, Logic::Intuitionistic, 3, 3);
            assert!(
                matches!(
                    outcome.verdict,
                    Verdict::Unknown(Reason::KripkeSize { worlds: 3, size: 3 })
                ),
                "{text}: {:?}",
                outcome.verdict
            );
        }
    }

    /// With `=` and no bound on worlds the search tries the structures up
    /// to the size bound and ends there, unknown: it once ran to
    /// `u32::MAX` levels without a poll and then overflowed asking for a
    /// Kripke model of that many worlds.
    #[test]
    fn equality_ends_at_the_size_bound() {
        let sequent: Sequent = "|- a = b \\/ ~a = b".parse().unwrap();
        let options = search::Options::default()
            .with_model_size(Some(2))
            .with_model_worlds(None);
        for logic in [Logic::Intuitionistic, Logic::Minimal] {
            let outcome = decide(&sequent, logic, &options, &Limits::default(), |_| false).unwrap();
            assert!(
                matches!(
                    outcome.verdict,
                    Verdict::Unknown(Reason::ModelSize { size: 2 })
                ),
                "{logic:?}: {:?}",
                outcome.verdict
            );
        }
    }

    /// The stop ends the search as unknown, and a sequent with two goals
    /// outside classical logic is refused.
    #[test]
    fn the_stop_and_the_goals() {
        let sequent: Sequent = "forall x. p(x) |- p(f(a))".parse().unwrap();
        let options = search::Options::default().with_model_size(None);
        let outcome = decide(
            &sequent,
            Logic::Classical,
            &options,
            &Limits::default(),
            |p| p.done > 10_000,
        )
        .unwrap();
        assert!(matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
        // A bound of no element is unknown at once, in every logic.
        let none = search::Options::default().with_model_size(Some(0));
        for &logic in Logic::ALL {
            let outcome = decide(&sequent, logic, &none, &Limits::default(), |_| false).unwrap();
            assert!(matches!(
                outcome.verdict,
                Verdict::Unknown(
                    Reason::ModelSize { size: 0 } | Reason::KripkeSize { size: 0, .. }
                )
            ));
        }
        let two: Sequent = "|- p(a), p(b)".parse().unwrap();
        assert!(
            decide(
                &two,
                Logic::Intuitionistic,
                &options,
                &Limits::default(),
                |_| false
            )
            .is_err()
        );
    }

    /// Returns a closed term over `a`, `b`, `f/1`, `g/2` and the bound
    /// variables in scope.
    fn term(rng: &mut Rng, depth: usize, scope: &[String]) -> String {
        if depth == 0 || rng.one_in(2) {
            return match rng.below(scope.len() + 2) {
                0 => "a".to_owned(),
                1 => "b".to_owned(),
                k => scope[k - 2].clone(),
            };
        }
        if rng.one_in(2) {
            format!("f({})", term(rng, depth - 1, scope))
        } else {
            format!(
                "g({}, {})",
                term(rng, depth - 1, scope),
                term(rng, depth - 1, scope)
            )
        }
    }

    /// Returns a formula over `p/1`, `q/2`, `r/0` and those terms.
    fn formula(rng: &mut Rng, depth: usize, scope: &mut Vec<String>) -> String {
        if depth == 0 || rng.one_in(4) {
            return match rng.below(5) {
                0 => "r".to_owned(),
                1 | 2 => format!("p({})", term(rng, 2, scope)),
                _ => format!("q({}, {})", term(rng, 1, scope), term(rng, 1, scope)),
            };
        }
        match rng.below(7) {
            0 => format!("~{}", formula(rng, depth - 1, scope)),
            1 | 2 if scope.len() < 2 => {
                let name = ["x", "y"][scope.len()].to_owned();
                let quantifier = if rng.one_in(2) { "forall" } else { "exists" };
                scope.push(name.clone());
                let body = formula(rng, depth - 1, scope);
                scope.pop();
                format!("({quantifier} {name}. {body})")
            }
            k => {
                let a = formula(rng, depth - 1, scope);
                let b = formula(rng, depth - 1, scope);
                let op = ["/\\", "\\/", "->", "<->"][k % 4];
                format!("({a} {op} {b})")
            }
        }
    }

    /// On generated sequents with function symbols, the model search never
    /// refutes what the connection prover proves, and every structure it
    /// gives refutes its sequent.
    #[test]
    fn the_model_search_agrees_with_the_connection_prover() {
        let mut rng = Rng::new(7_601);
        let limits = Limits {
            work: Some(1 << 16),
            ..Limits::default()
        };
        let options = search::Options::default().with_model_size(Some(3));
        let (mut refuted, mut proved) = (0, 0);
        for _ in 0..300 {
            let hypotheses: Vec<String> = (0..rng.below(3))
                .map(|_| formula(&mut rng, 3, &mut Vec::new()))
                .collect();
            let goal = formula(&mut rng, 3, &mut Vec::new());
            let text = format!("{} |- {goal}", hypotheses.join(", "));
            let sequent: Sequent = text.parse().unwrap();
            let ours = decide(&sequent, Logic::Classical, &options, &limits, |_| false)
                .unwrap_or_else(|e| panic!("{text}: {e}"));
            let theirs = crate::ordinary::connection::decide(
                &sequent,
                Logic::Classical,
                &search::Options::default(),
                &limits,
                |_| false,
            );
            let valid = matches!(theirs.map(|o| o.verdict), Ok(Verdict::Valid(_)));
            proved += usize::from(valid);
            if matches!(ours.verdict, Verdict::NotValid) {
                assert!(!valid, "{text}");
                refuted += 1;
                let countermodel = ours.structure.expect("a structure");
                // The structure as a Kripke model of one world says the same.
                assert_eq!(
                    crate::ordinary::KripkeStructure::from(countermodel.structure.clone())
                        .refutes(&sequent, Logic::Classical, &limits, |_| false)
                        .unwrap(),
                    Evaluation::Refutes,
                    "{text}"
                );
                assert_eq!(
                    countermodel
                        .structure
                        .refutes(&sequent, &limits, |_| false)
                        .unwrap(),
                    Evaluation::Refutes,
                    "{text}"
                );
            }
        }
        assert!(refuted > 200 && proved > 10, "{refuted} {proved}");
    }

    /// On generated intuitionistic sequents with function symbols, the
    /// model search never refutes what the embedding proves, and every
    /// Kripke model it gives refutes its sequent.
    #[test]
    fn kripke_models_agree_with_the_embedding() {
        use crate::ordinary::{Method, Options};
        let mut rng = Rng::new(7_602);
        let limits = Limits {
            work: Some(1 << 16),
            ..Limits::default()
        };
        let options = search::Options::default()
            .with_model_size(Some(2))
            .with_model_worlds(Some(3));
        let (mut kripke, mut proved) = (0, 0);
        for _ in 0..200 {
            let hypotheses: Vec<String> = (0..rng.below(3))
                .map(|_| formula(&mut rng, 3, &mut Vec::new()))
                .collect();
            let goal = formula(&mut rng, 3, &mut Vec::new());
            // Half the goals are classically valid: excluded middle or
            // double negation elimination of a generated formula.
            let goal = match rng.below(4) {
                0 => format!("({goal}) \\/ ~({goal})"),
                1 => format!("~~({goal}) -> ({goal})"),
                _ => goal,
            };
            let text = format!("{} |- {goal}", hypotheses.join(", "));
            let sequent: Sequent = text.parse().unwrap();
            let ours = decide(&sequent, Logic::Intuitionistic, &options, &limits, |_| {
                false
            })
            .unwrap_or_else(|e| panic!("{text}: {e}"));
            let embedded = crate::ordinary::decide_within(
                &sequent,
                &Options::default()
                    .with_logic(Logic::Intuitionistic)
                    .with_method(Some(Method::Embedding)),
                &search::Options::default(),
                &limits,
                |_| false,
            );
            let valid = matches!(embedded.map(|o| o.verdict), Ok(Verdict::Valid(_)));
            proved += usize::from(valid);
            if matches!(ours.verdict, Verdict::NotValid) {
                assert!(!valid, "{text}");
            }
            if let Some(countermodel) = ours.kripke {
                kripke += 1;
                countermodel
                    .check(Logic::Intuitionistic, &limits, |_| false)
                    .unwrap_or_else(|e| panic!("{text}: {e}"));
            }
        }
        assert!(kripke > 50 && proved > 5, "{kripke} {proved}");
    }
}
