// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The SAT solver the engine asks, behind a trait of this crate's own, so
//! that another solver or a fork replaces it in one place: variables,
//! clauses, solving under assumptions, the model, the failed assumptions,
//! and a stop that the solver polls at every decision. The one
//! implementation is batsat, MiniSat 2.2 in Rust.

use std::cell::{Cell, RefCell};

use batsat::interface::SolverInterface;
use batsat::{Callbacks, ClauseKind, Lit as BatLit, SolverOpts, Var, lbool};

use super::drup::Step;
use crate::limits::{Phase, Progress};
use crate::linear::search::Reason;
use crate::linear::search::memory::Account;

/// A literal: a variable, by its index, and its sign.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Lit {
    /// The variable.
    var: u32,
    /// Whether the literal is the variable itself rather than its
    /// negation.
    positive: bool,
}

impl Lit {
    /// The variable itself.
    pub(crate) const fn positive(var: u32) -> Self {
        Self {
            var,
            positive: true,
        }
    }

    /// The negation of the variable.
    pub(crate) const fn negative(var: u32) -> Self {
        Self {
            var,
            positive: false,
        }
    }

    /// Returns the variable.
    pub(crate) const fn var(self) -> u32 {
        self.var
    }

    /// Returns whether the literal is the variable itself.
    pub(crate) const fn is_positive(self) -> bool {
        self.positive
    }
}

/// What a call of [`Solver::solve`] found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Answer {
    /// The clauses and the assumptions have a model, which
    /// [`Solver::value`] reads.
    Satisfiable,
    /// They have none; [`Solver::failed`] lists the assumptions that
    /// suffice for that.
    Unsatisfiable,
    /// The solver stopped before it could tell, for the reason given.
    Stopped(Reason),
}

/// An incremental SAT solver: clauses are only ever added, between calls
/// of [`solve`](Self::solve).
pub(crate) trait Solver {
    /// Returns a new variable, the variables numbered from 0 in the order
    /// they are made.
    fn new_var(&mut self) -> u32;
    /// Adds a clause, a disjunction of its literals.
    fn add_clause(&mut self, clause: &[Lit]);
    /// Decides the clauses under the assumptions, each literal taken as
    /// true, asking the stop at every decision.
    fn solve(&mut self, assumptions: &[Lit]) -> Answer;
    /// Returns the value of the literal in the model the last
    /// satisfiable answer found.
    fn value(&self, literal: Lit) -> bool;
    /// Returns the assumptions of the last unsatisfiable answer that
    /// make the clauses unsatisfiable by themselves.
    fn failed(&self) -> Vec<Lit>;
    /// Asks the stop, the work bound and the memory bound between calls,
    /// with `work` units of progress since the last ask, returning why
    /// the solver must stop, if it must.
    fn poll(&mut self, work: u64) -> Option<Reason>;
    /// Returns the calls of [`solve`](Self::solve) so far.
    fn calls(&self) -> u64;
    /// Returns the conflicts met so far.
    fn conflicts(&self) -> u64;
}

/// How a learnt clause is charged to the memory account: its literals,
/// and what the solver keeps beside them (a header, two watches).
const fn clause_bytes(len: usize) -> usize {
    len * size_of::<BatLit>() + 32
}

/// How a variable is charged to the memory account: what the solver
/// keeps per variable (its value, reason, level, activity, polarity,
/// heap entry, the model's copy and two watch lists' headers).
const VARIABLE_BYTES: usize = 128;

/// The callbacks batsat calls: they count the clauses learnt against the
/// work bound, charge the learnt clauses to the memory account and ask the
/// caller's stop, which batsat polls at every decision.
pub(crate) struct Bridge<'s> {
    /// The caller's stop, asked at every decision; a `RefCell` since
    /// batsat polls it through a shared reference.
    stop: RefCell<&'s mut dyn FnMut(Progress) -> bool>,
    /// The memory account the clauses are charged to.
    account: &'s Account,
    /// The calls of `solve` so far.
    calls: Cell<u64>,
    /// The work done: a unit per clause learnt (at a conflict, or at the
    /// end of a call that refutes the assumptions) and per call of
    /// `solve`.
    done: Cell<u64>,
    /// The most work the bound allows.
    work: u64,
    /// Why the solver was stopped, once it was.
    why: Cell<Option<Reason>>,
    /// The clauses learnt and deleted, in order, where a proof is
    /// recorded.
    proof: Option<Vec<Step>>,
}

impl Callbacks for Bridge<'_> {
    fn on_new_clause(&mut self, clause: &[BatLit], _kind: ClauseKind) {
        self.done.set(self.done.get().saturating_add(1));
        self.account.charge(clause_bytes(clause.len()));
        self.record(clause, Step::Add);
    }

    fn on_delete_clause(&mut self, clause: &[BatLit]) {
        self.account.release(clause_bytes(clause.len()));
        self.record(clause, Step::Delete);
    }

    fn stop(&self) -> bool {
        self.ask(1)
    }
}

impl Bridge<'_> {
    /// Appends the clause to the proof as `step` says, where a proof is
    /// recorded, charging it to the memory account.
    fn record(&mut self, clause: &[BatLit], step: fn(Box<[Lit]>) -> Step) {
        if let Some(proof) = &mut self.proof {
            self.account.charge(Step::bytes(clause.len()));
            proof.push(step(clause.iter().map(|&l| Batsat::ours(l)).collect()));
        }
    }

    /// Returns whether the solver must stop: past the work bound, past the
    /// memory bound, or at the caller's stop, asked with `work` units of
    /// progress since the last ask; records why.
    fn ask(&self, work: u64) -> bool {
        if self.why.get().is_some() {
            return true;
        }
        let done = self.done.get();
        let why = if done > self.work {
            Some(Reason::WorkLimit { limit: self.work })
        } else if self.account.over() {
            Some(Reason::MemoryLimit {
                limit_bytes: self.account.limit(),
            })
        } else if (self.stop.borrow_mut())(Progress::new(Phase::Search, work, done)) {
            Some(Reason::Stopped)
        } else {
            None
        };
        self.why.set(why);
        why.is_some()
    }
}

/// batsat behind [`Solver`].
pub(crate) struct Batsat<'s> {
    /// The solver, with the callbacks.
    solver: batsat::Solver<Bridge<'s>>,
}

impl<'s> Batsat<'s> {
    /// A solver that stops past `work` units of work (a conflict, or a
    /// call of `solve`), past the memory account's bound, or when `stop`
    /// fires.
    pub(crate) fn new(
        work: u64,
        account: &'s Account,
        stop: &'s mut dyn FnMut(Progress) -> bool,
    ) -> Self {
        let bridge = Bridge {
            stop: RefCell::new(stop),
            account,
            calls: Cell::new(0),
            done: Cell::new(0),
            work,
            why: Cell::new(None),
            proof: None,
        };
        Self {
            solver: batsat::Solver::new(SolverOpts::default(), bridge),
        }
    }

    /// [`new`](Self::new), recording every clause learnt and deleted as a
    /// clausal proof, which [`proof`](Self::proof) takes.
    pub(crate) fn recording(
        work: u64,
        account: &'s Account,
        stop: &'s mut dyn FnMut(Progress) -> bool,
    ) -> Self {
        let mut solver = Self::new(work, account, stop);
        solver.solver.cb_mut().proof = Some(Vec::new());
        solver
    }

    /// Takes the clauses learnt and deleted so far, in order, where the
    /// solver records them.
    pub(crate) fn proof(&mut self) -> Option<Vec<Step>> {
        let proof = self.solver.cb_mut().proof.as_mut()?;
        Some(std::mem::take(proof))
    }

    /// Returns batsat's literal of ours.
    fn lit(literal: Lit) -> BatLit {
        BatLit::new(Var::unsafe_from_idx(literal.var), literal.positive)
    }

    /// Returns our literal of batsat's, whose sign is `true` for the
    /// variable itself.
    fn ours(literal: BatLit) -> Lit {
        Lit {
            var: literal.var().idx(),
            positive: literal.sign(),
        }
    }
}

/// The most variables a solver holds: batsat keeps a literal as twice its
/// variable's index plus its sign in a `u32`, and two values at the top
/// are its markers.
pub(crate) const MOST_VARIABLES: u32 = u32::MAX / 2 - 1;

impl Solver for Batsat<'_> {
    fn new_var(&mut self) -> u32 {
        self.solver.cb().account.charge(VARIABLE_BYTES);
        self.solver.new_var_default().idx()
    }

    fn add_clause(&mut self, clause: &[Lit]) {
        let mut literals: Vec<BatLit> = clause.iter().map(|&l| Self::lit(l)).collect();
        self.solver
            .cb()
            .account
            .charge(clause_bytes(literals.len()));
        // `false` says the clauses are unsatisfiable already, which the
        // next call of `solve` answers.
        self.solver.add_clause_reuse(&mut literals);
    }

    fn solve(&mut self, assumptions: &[Lit]) -> Answer {
        let bridge = self.solver.cb();
        bridge.calls.set(bridge.calls.get().saturating_add(1));
        bridge.done.set(bridge.done.get().saturating_add(1));
        if bridge.stop() {
            return Answer::Stopped(self.why());
        }
        let assumptions: Vec<BatLit> = assumptions.iter().map(|&l| Self::lit(l)).collect();
        let answer = self.solver.solve_limited(&assumptions);
        if answer == lbool::TRUE {
            Answer::Satisfiable
        } else if answer == lbool::FALSE {
            // A refutation of the assumptions reports its final conflict as
            // a clause learnt, which the solver does not keep: it is the
            // core, and given back here.
            // It is no consequence of the clauses alone, so a proof does not
            // record it.
            let core = self.solver.unsat_core().len();
            if core > 0 {
                let bridge = self.solver.cb_mut();
                bridge.account.release(clause_bytes(core));
                if let Some(proof) = &mut bridge.proof {
                    proof.pop();
                    bridge.account.release(Step::bytes(core));
                }
            }
            Answer::Unsatisfiable
        } else {
            Answer::Stopped(self.why())
        }
    }

    fn poll(&mut self, work: u64) -> Option<Reason> {
        self.solver.cb().ask(work).then(|| self.why())
    }

    fn value(&self, literal: Lit) -> bool {
        self.solver.value_lit(Self::lit(literal)) == lbool::TRUE
    }

    fn failed(&self) -> Vec<Lit> {
        // batsat's core is MiniSat's final conflict: the negations of the
        // failed assumptions.
        self.solver
            .unsat_core()
            .iter()
            .map(|&l| Self::ours(!l))
            .collect()
    }

    fn calls(&self) -> u64 {
        self.solver.cb().calls.get()
    }

    fn conflicts(&self) -> u64 {
        self.solver.num_conflicts()
    }
}

impl Batsat<'_> {
    /// Returns why the solver stopped: the reason its callbacks recorded.
    fn why(&self) -> Reason {
        self.solver.cb().why.get().unwrap_or(Reason::Stopped)
    }
}
