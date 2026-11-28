// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The search of a natural deduction session: a goal decided as a sequent
//! of ordinary logic (the lines in force at it left of `⊢`, the goal
//! right) through the embedding into linear logic, whose derivations have
//! no cut, the derivation found translated into the session's file and
//! checked against it, then taken step by step, each step checked again.

use super::interactive::{Apply, Done, GoalId, Interactive};
use super::natural::{Profile, ordinary_of};
use super::{Derivation, NodeId, Step, TermId};
use crate::hash::HashMap;
use crate::linear::search;
use crate::ordinary::{self, Evidence, Formulas, Method, Verdict};
use crate::{Error, Limits, Progress, Refusal};

/// What [`Interactive::close`] found: the outcome of deciding the goal,
/// the proof grafted, and whether a bound refused the graft.
///
/// Needs the cargo features `interactive` and `parse` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Closed {
    /// The outcome of deciding the goal as a sequent of ordinary logic,
    /// with the lines in force at it as its hypotheses.
    pub outcome: ordinary::Outcome,
    /// The natural deduction proof of the goal found, checked against the
    /// session's file; `None` where the goal is not valid or undecided.
    pub derivation: Option<Derivation>,
    /// `Ok` when the proof was taken or there was none; the refusal of a
    /// bound otherwise, the goal then still open.
    pub grafted: Result<(), Refusal>,
}

/// What [`Interactive::hint`] found: the outcome of deciding the goal and
/// the first step of the proof found.
///
/// Needs the cargo features `interactive` and `parse` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Hint {
    /// The outcome, as [`Closed::outcome`].
    pub outcome: ordinary::Outcome,
    /// The proof's first step on the goal, which the session takes as it
    /// is; empty where the goal is not valid or undecided.
    pub steps: Vec<Hinted>,
}

/// A step of a proof found, as a session takes it.
///
/// Needs the cargo features `interactive` and `parse` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hinted {
    /// The rule to apply ([`Interactive::apply`]), or `None` where the goal
    /// is a line in force ([`Interactive::assume`]).
    pub rule: Option<String>,
    /// The values the rule needs given, each a variable's name and its
    /// formula as text: what the goal leaves open.
    pub given: Vec<(String, String)>,
}

impl Interactive {
    /// Decides the open goal `goal` of a session in one of the natural
    /// deduction files deduxit ships, and takes the proof found: the goal
    /// as a sequent of ordinary logic in the file's logic (the lines in
    /// force at it left of `⊢`), searched through the embedding into linear
    /// logic with `search`, within `limits` and until `stop` fires; the
    /// derivation found translated into a normal proof of the file
    /// ([`Derivation::natural`]), checked against it, and taken step by
    /// step, each step checked as [`apply`](Self::apply) and
    /// [`assume`](Self::assume) check one, so that [`undo`](Self::undo)
    /// takes it back a step at a time. A goal not valid or undecided stays
    /// open, and the outcome says why.
    ///
    /// # Errors
    ///
    /// [`Error::NotApplicable`] in a calculus that is no shipped natural
    /// deduction file; a goal that is not open; the errors of
    /// [`ordinary::decide_within`] and [`Derivation::natural`]; and a step
    /// of the proof the session refuses, a defect, after which the session
    /// is as it was.
    pub fn close(
        &mut self,
        goal: GoalId,
        search: &search::Options,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Closed, Error> {
        self.closed(goal, search, limits, &mut stop)
    }

    /// [`close`](Self::close) with the stop as a trait object, so that it
    /// is compiled once, not once per caller's closure.
    fn closed(
        &mut self,
        goal: GoalId,
        search: &search::Options,
        limits: &Limits,
        mut stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Closed, Error> {
        let (outcome, derivation) = self.found(goal, search, limits, &mut stop)?;
        let grafted = match &derivation {
            Some(derivation) => match self.take(goal, derivation) {
                Ok(()) => Ok(()),
                Err(Error::Refused(refusal)) => Err(refusal),
                Err(error) => return Err(error),
            },
            None => Ok(()),
        };
        Ok(Closed {
            outcome,
            derivation,
            grafted,
        })
    }

    /// Decides the open goal `goal` as [`close`](Self::close) does and
    /// returns the outcome with the proof's first step on the goal, which
    /// the session takes as it is: [`assume`](Self::assume) where the goal
    /// is a line in force. The session does not change.
    ///
    /// The hints of a session follow one proof: the search starts at the
    /// session's root goal, and its proof is followed down the steps the
    /// session took while each is the proof's (the same rule, the same
    /// premises); where one is not, the search starts again from the goal
    /// the session left the proof at. So hints taken in turn follow a
    /// proof found once and end, where each goal searched alone could lead
    /// back to a goal it came from (a classical rule's `¬A` met again).
    /// The search runs on one thread, so that a goal's proof is the same
    /// each time it is asked. The outcome is the search's whose proof the
    /// step follows: valid exactly when the goal has a proof.
    ///
    /// # Errors
    ///
    /// As [`close`](Self::close).
    pub fn hint(
        &self,
        goal: GoalId,
        search: &search::Options,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Hint, Error> {
        self.hinted(goal, search, limits, &mut stop)
    }

    /// [`hint`](Self::hint) with the stop as a trait object, so that it
    /// is compiled once, not once per caller's closure.
    fn hinted(
        &self,
        goal: GoalId,
        search: &search::Options,
        limits: &Limits,
        mut stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Hint, Error> {
        self.open(goal)?;
        let search = search.clone().with_jobs(1);
        let mut state = self.clone();
        let mut terms = HashMap::default();
        let path = self.path_to(goal);
        let mut start = 0;
        let (outcome, derivation, node) = loop {
            let top = path[start];
            let (outcome, derivation) = self.found_at(top, &search, limits, &mut stop)?;
            let Some(derivation) = derivation else {
                if start + 1 == path.len() {
                    return Ok(Hint {
                        outcome,
                        steps: Vec::new(),
                    });
                }
                start += 1;
                continue;
            };
            let Some(root) = derivation.root() else {
                return Ok(Hint {
                    outcome,
                    steps: Vec::new(),
                });
            };
            // Down the steps taken while each is the proof's.
            let mut node = root.id();
            let mut at = start;
            while at + 1 < path.len() {
                let taken = &self.nodes[path[at] as usize];
                let proof = derivation.node(node);
                let next = path[at + 1];
                let follows = match (&taken.done, proof.step()) {
                    (Done::Rule(name, _), Step::Rule(rule)) if name == rule => {
                        proof.premises().len() == taken.premises.len()
                            && proof
                                .premises()
                                .iter()
                                .zip(&taken.premises)
                                .all(|(&p, &q)| {
                                    state
                                        .carried(
                                            &derivation,
                                            derivation.node(p).zone(0)[0],
                                            &mut terms,
                                        )
                                        .is_ok_and(|t| self.nodes[q as usize].sequent[0][0] == t)
                                })
                    }
                    _ => false,
                };
                let Some(k) = taken.premises.iter().position(|&p| p == next) else {
                    break;
                };
                if !follows {
                    break;
                }
                node = proof.premises()[k];
                at += 1;
            }
            if at + 1 == path.len() {
                break (outcome, derivation, node);
            }
            // The session left the proof at `path[at]`: start there, or below
            // where it left it at once.
            start = if at == start { start + 1 } else { at };
            terms.clear();
        };
        // A goal that is a line in force is closed by it: the proof found
        // may take it apart first.
        if state.clone().assume(goal).is_ok() {
            return Ok(Hint {
                outcome,
                steps: vec![Hinted {
                    rule: None,
                    given: Vec::new(),
                }],
            });
        }
        let root = derivation.node(node);
        let step = match root.step() {
            Step::Rule(rule) => {
                let premises: Vec<TermId> = root
                    .premises()
                    .iter()
                    .map(|&p| state.carried(&derivation, derivation.node(p).zone(0)[0], &mut terms))
                    .collect::<Result<_, _>>()?;
                let needs = match state.clone().apply(goal, &Apply::new(rule)) {
                    Err(Error::StepNeeds { needs }) => needs,
                    _ => Vec::new(),
                };
                let bound = state.premise_bindings(rule, &premises)?;
                let given = bound
                    .into_iter()
                    .filter(|(name, _)| needs.contains(name))
                    .map(|(name, term)| {
                        let text = state
                            .calculus()
                            .show(state.terms(), term)
                            .limit(TEXT)
                            .to_string();
                        (name, text)
                    })
                    .collect();
                Hinted {
                    rule: Some(rule.to_owned()),
                    given,
                }
            }
            Step::Hypothesis | Step::Open => Hinted {
                rule: None,
                given: Vec::new(),
            },
        };
        Ok(Hint {
            outcome,
            steps: vec![step],
        })
    }

    /// Returns the session's nodes from the root to `goal`, each the
    /// conclusion of the next.
    fn path_to(&self, goal: GoalId) -> Vec<u32> {
        let mut parent: HashMap<u32, u32> = HashMap::default();
        for (n, node) in self.nodes.iter().enumerate() {
            if node.line {
                continue;
            }
            for &p in &node.premises {
                // As many nodes as a `u32` numbers.
                parent.entry(p).or_insert(n as u32);
            }
        }
        let mut path = vec![goal.index() as u32];
        let mut at = goal.index() as u32;
        while let Some(&up) = parent.get(&at) {
            path.push(up);
            at = up;
        }
        path.reverse();
        path
    }

    /// Decides the open goal as a sequent of ordinary logic, and translates
    /// a derivation found into the session's file, checked.
    fn found(
        &self,
        goal: GoalId,
        search: &search::Options,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(ordinary::Outcome, Option<Derivation>), Error> {
        self.open(goal)?;
        self.found_at(goal.index() as u32, search, limits, stop)
    }

    /// Decides the node `at`'s sequent from the lines in force at it, open
    /// or closed, as a sequent of ordinary logic, and translates a
    /// derivation found into the session's file, checked.
    fn found_at(
        &self,
        at: u32,
        search: &search::Options,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(ordinary::Outcome, Option<Derivation>), Error> {
        let goal = GoalId::new(at);
        let Some(profile) = Profile::of(self.calculus()) else {
            return Err(Error::NotApplicable {
                operation: "the search".into(),
                reason: format!(
                    "{} is not one of the natural deduction files deduxit ships",
                    self.calculus().name()
                ),
            });
        };
        let mut formulas = Formulas::default();
        let mut memo = HashMap::default();
        let mut left = Vec::new();
        for (_, line) in self.lines(goal) {
            left.push(ordinary_of(
                self.terms(),
                line[0][0],
                &mut formulas,
                &mut memo,
            )?);
        }
        let right = vec![ordinary_of(
            self.terms(),
            self.nodes[at as usize].sequent[0][0],
            &mut formulas,
            &mut memo,
        )?];
        let sequent = ordinary::Sequent::new(formulas, left, right)?;
        // The embedding's derivations have no cut.
        let options = ordinary::Options::default()
            .with_logic(profile.logic)
            .with_method(Some(Method::Embedding));
        let outcome = ordinary::decide_within(&sequent, &options, search, limits, &mut *stop)?;
        let derivation = match &outcome.verdict {
            Verdict::Valid(Evidence::Derivation(found)) => Some(Derivation::natural(
                found,
                self.calculus(),
                limits,
                &mut *stop,
            )?),
            Verdict::Valid(_) | Verdict::NotValid | Verdict::Unknown(_) => None,
        };
        Ok((outcome, derivation))
    }

    /// Takes the checked proof `derivation` of the open goal `goal` step by
    /// step from its root: a rule by [`apply`](Self::apply) with its
    /// premises' formulas, a hypothesis by [`assume`](Self::assume). A step
    /// refused takes back every step before it.
    fn take(&mut self, goal: GoalId, derivation: &Derivation) -> Result<(), Error> {
        let before = self.steps();
        let mut terms = HashMap::default();
        let root = derivation.root().expect("a checked derivation has a root");
        let mut stack: Vec<(GoalId, NodeId)> = vec![(goal, root.id())];
        let taken = (|| {
            while let Some((at, node)) = stack.pop() {
                let node = derivation.node(node);
                match node.step() {
                    Step::Rule(rule) => {
                        let premises: Vec<TermId> = node
                            .premises()
                            .iter()
                            .map(|&p| {
                                self.carried(derivation, derivation.node(p).zone(0)[0], &mut terms)
                            })
                            .collect::<Result<_, _>>()?;
                        let opened = self.apply(at, &Apply::new(rule).premises(&premises))?;
                        stack.extend(opened.into_iter().zip(node.premises().iter().copied()));
                    }
                    Step::Hypothesis | Step::Open => self.assume(at)?,
                }
            }
            Ok(())
        })();
        if taken.is_err() {
            while self.steps() > before {
                self.undo();
            }
        }
        taken
    }

    /// Returns the session's term for the term `id` of `derivation`, added
    /// where the session lacks it; `memo` keeps those found.
    fn carried(
        &mut self,
        derivation: &Derivation,
        id: TermId,
        memo: &mut HashMap<TermId, TermId>,
    ) -> Result<TermId, Error> {
        let terms = derivation.terms();
        let mut stack = vec![(id, false)];
        while let Some((at, left)) = stack.pop() {
            if memo.contains_key(&at) {
                continue;
            }
            match terms.term(at) {
                super::Term::Atom { name, sort } => {
                    let term = self.terms.atom(name, sort)?;
                    memo.insert(at, term);
                }
                super::Term::Apply { arguments, .. } if !left => {
                    stack.push((at, true));
                    stack.extend(arguments.iter().map(|&a| (a, false)));
                }
                super::Term::Apply {
                    connective,
                    arguments,
                } => {
                    let arguments: Vec<TermId> = arguments.iter().map(|a| memo[a]).collect();
                    let term = self.terms.apply(connective, &arguments)?;
                    memo.insert(at, term);
                }
            }
        }
        Ok(memo[&id])
    }
}

/// The most characters of a hinted formula's text: one far longer is no
/// hint a person types.
const TEXT: usize = 1 << 16;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Calculus;

    /// Reads a rule file.
    fn calculus(text: &str) -> Calculus {
        let mut document = serde_json::Deserializer::from_str(text);
        crate::wire::upgrade(&mut document, &Limits::default()).unwrap()
    }

    /// The search closes a goal in NJ with a proof taken step by step, and
    /// the finished session checks; a goal under a hypothesis a step made
    /// is closed from it; `undo` takes the proof back a step at a time.
    #[test]
    fn close_takes_a_proof_step_by_step() {
        let limits = Limits::default();
        let options = search::Options::default();
        let nj = calculus(crate::rules::NJ);
        let mut session =
            Interactive::within(nj, "(a -> b) -> (b -> c) -> a -> c", &[], &limits).unwrap();
        let goal = session.goals()[0];
        let [inner] = session.apply(goal, &Apply::new("→I")).unwrap()[..] else {
            panic!()
        };
        let closed = session.close(inner, &options, &limits, |_| false).unwrap();
        assert!(matches!(closed.outcome.verdict, Verdict::Valid(_)));
        assert_eq!(closed.grafted, Ok(()));
        assert!(session.is_finished());
        session.finished(|_| false).unwrap();
        let steps = session.steps();
        assert!(steps > 2);
        session.undo();
        assert_eq!(session.steps(), steps - 1);
        while session.steps() > 1 {
            session.undo();
        }
        assert_eq!(session.goals(), [inner]);
    }

    /// A goal the file's logic does not prove stays open with the outcome
    /// saying so; the classical files prove it, each by its own rule.
    #[test]
    fn the_file_s_logic_decides() {
        let limits = Limits::default();
        let options = search::Options::default();
        let peirce = "((a -> b) -> a) -> a";
        let mut nj = Interactive::within(calculus(crate::rules::NJ), peirce, &[], &limits).unwrap();
        let goal = nj.goals()[0];
        let closed = nj.close(goal, &options, &limits, |_| false).unwrap();
        assert!(matches!(closed.outcome.verdict, Verdict::NotValid));
        assert_eq!(nj.goals(), [goal]);
        for text in [
            crate::rules::NK,
            crate::rules::NK_DNE,
            crate::rules::NK_LEM,
            crate::rules::NK_DEFINED,
        ] {
            let mut session = Interactive::within(calculus(text), peirce, &[], &limits).unwrap();
            let goal = session.goals()[0];
            let closed = session.close(goal, &options, &limits, |_| false).unwrap();
            assert!(
                matches!(closed.outcome.verdict, Verdict::Valid(_)),
                "{text}"
            );
            session.finished(|_| false).unwrap();
        }
    }

    /// The hint is the proof's first step on the goal, with what it needs
    /// given, and taking it as it is leaves goals the search closes;
    /// following the hints to the end proves the goal.
    #[test]
    fn following_the_hints_proves_a_goal() {
        let limits = Limits::default();
        let options = search::Options::default();
        let nj = calculus(crate::rules::NJ);
        let mut session =
            Interactive::within(nj, "c", &["a -> b", "b -> c", "a"], &limits).unwrap();
        for _ in 0..20 {
            let Some(&goal) = session.goals().first() else {
                break;
            };
            let hint = session.hint(goal, &options, &limits, |_| false).unwrap();
            let [step] = &hint.steps[..] else {
                panic!("{:?}", hint.outcome.verdict)
            };
            match &step.rule {
                Some(rule) => {
                    let given: Vec<(&str, &str)> = step
                        .given
                        .iter()
                        .map(|(n, t)| (n.as_str(), t.as_str()))
                        .collect();
                    session
                        .apply(goal, &Apply::new(rule).given(&given))
                        .unwrap_or_else(|e| panic!("{rule} {given:?}: {e}"));
                }
                None => session.assume(goal).unwrap(),
            }
        }
        assert!(session.is_finished());
        session.finished(|_| false).unwrap();
    }

    /// Following the hints ends in the classical files too: no hint on a
    /// goal `⊥` is the classical rule again, nor one that discharges a
    /// negation already in force.
    #[test]
    fn following_classical_hints_proves_a_goal() {
        let limits = Limits::default();
        let options = search::Options::default();
        for text in [
            crate::rules::NK,
            crate::rules::NK_DNE,
            crate::rules::NK_LEM,
            crate::rules::NK_DEFINED,
        ] {
            for goal in [
                "((a -> b) -> a) -> a",
                "a \\/ (a -> false)",
                "((a -> false) -> false) -> a",
            ] {
                let mut session = Interactive::within(calculus(text), goal, &[], &limits).unwrap();
                for _ in 0..60 {
                    let Some(&at) = session.goals().first() else {
                        break;
                    };
                    let hint = session.hint(at, &options, &limits, |_| false).unwrap();
                    let [step] = &hint.steps[..] else {
                        panic!("{goal}: {:?}", hint.outcome.verdict)
                    };
                    match &step.rule {
                        Some(rule) => {
                            let given: Vec<(&str, &str)> = step
                                .given
                                .iter()
                                .map(|(n, t)| (n.as_str(), t.as_str()))
                                .collect();
                            session.apply(at, &Apply::new(rule).given(&given)).unwrap();
                        }
                        None => session.assume(at).unwrap(),
                    }
                }
                assert!(
                    session.is_finished(),
                    "{goal} in {}",
                    session.calculus().name()
                );
                session.finished(|_| false).unwrap();
            }
        }
    }

    /// A calculus that is no shipped natural deduction file has no search.
    #[test]
    fn another_calculus_has_no_search() {
        let limits = Limits::default();
        let lj = calculus(crate::rules::LJ);
        let mut session = Interactive::within(lj, "a |- a", &[], &limits).unwrap();
        let goal = session.goals()[0];
        assert!(matches!(
            session.close(goal, &search::Options::default(), &limits, |_| false),
            Err(Error::NotApplicable { .. })
        ));
    }
}
