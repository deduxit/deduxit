// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Classical logic decided by the SAT solver: the sequent `Γ ⊢ Δ` is
//! falsifiable exactly when the clauses of its negation, every formula of
//! `Γ` true and every formula of `Δ` false, have a model.
//!
//! The clauses are Tseitin's encoding with polarities (D. A. Plaisted and
//! S. Greenbaum, *A structure-preserving clause form translation*, J.
//! Symbolic Computation 2, 1986): every subformula is a variable, and a
//! formula gets the clauses of one direction of its definition, the one
//! its polarity needs. A formula that must be true gets "the formula
//! implies its definition", one that must be false "its definition
//! implies the formula".
//!
//! The answers are checked. A model of the clauses gives an assignment of
//! the atoms, a Kripke model of one world, which must falsify the sequent
//! by evaluation ([`Model::refutes`]). A refutation comes with the clauses
//! the solver learnt and deleted, a clausal proof that the checker of
//! [`drup`] accepts against the encoder's clauses before the
//! sequent is called valid; the derivation of LK is then the embedding's.

use super::drup::{self, Rejection, Step};
use super::solver::{Answer, Batsat, Lit, Solver};
use super::{Model, Outcome, Statistics, World};
use crate::Error;
use crate::limits::{Limits, Phase, Progress};
use crate::linear::search::memory::Account;
use crate::linear::search::{self, Reason};
use crate::ordinary::{
    Derivation, Formulas, Logic, Method, Node, NodeId, Options, Sequent, Verdict,
};

/// The polarity of a formula that must be true where the sequent fails,
/// as a hypothesis must.
const TRUE: u8 = 1;
/// The polarity of a formula that must be false where the sequent fails,
/// as a formula right of `⊢` must.
const FALSE: u8 = 2;

/// The clauses of a sequent's negation.
pub(super) struct Encoding {
    /// The variable of every formula of the arena that has one.
    pub(super) vars: Vec<Option<u32>>,
    /// The variables, numbered from 0.
    pub(super) variables: u32,
    /// The clauses.
    pub(super) clauses: Vec<Vec<Lit>>,
}

/// Returns the polarity of every formula of the arena where `left ⊢
/// right` fails: `TRUE` for the hypotheses and the parts of a formula
/// that must be true and that make it true, `FALSE` likewise, both for
/// the parts of an equivalence; `0` for a formula the sequent does not
/// hold.
fn polarities(formulas: &Formulas, left: &[NodeId], right: &[NodeId]) -> Vec<u8> {
    let mut polarity = vec![0u8; formulas.len()];
    for h in left {
        polarity[h.index()] |= TRUE;
    }
    for c in right {
        polarity[c.index()] |= FALSE;
    }
    // Every formula comes after its parts, so a pass from the last formula
    // to the first meets each before its parts.
    for id in (0..formulas.len()).rev() {
        let p = polarity[id];
        if p == 0 {
            continue;
        }
        let flipped = ((p & TRUE) << 1) | ((p & FALSE) >> 1);
        match formulas.node(NodeId(id as u32)) {
            Node::Atom(_) | Node::True | Node::False => {}
            Node::Not(a) => polarity[a.index()] |= flipped,
            Node::And(a, b) | Node::Or(a, b) => {
                polarity[a.index()] |= p;
                polarity[b.index()] |= p;
            }
            Node::Implies(a, b) => {
                polarity[a.index()] |= flipped;
                polarity[b.index()] |= p;
            }
            Node::Iff(a, b) => {
                polarity[a.index()] |= TRUE | FALSE;
                polarity[b.index()] |= TRUE | FALSE;
            }
        }
    }
    polarity
}

/// How many formulas the encoder takes, or clauses the solver is given,
/// between two polls of the stop.
const POLL: usize = 1 << 12;

/// How a clause of the encoding is charged to the memory account besides
/// its literals: its vector and the header of its allocation.
const CLAUSE_BYTES: usize = size_of::<Vec<Lit>>() + 16;

/// Returns the clauses of the negation of `left ⊢ right`, a variable for
/// every formula the sequent holds, at most `most` variables, each clause
/// charged to the account; refuses once the account is over, past
/// `most`, or when the stop, asked every [`POLL`] formulas, fires.
///
/// Every clause holds when each variable takes its formula's value under
/// an assignment of the atoms, except the unit clauses of the sequent's
/// formulas, which hold exactly where the assignment falsifies the
/// sequent. So the clauses have a model wherever the sequent is
/// falsifiable, and a refutation of them shows it valid.
pub(super) fn encode(
    formulas: &Formulas,
    left: &[NodeId],
    right: &[NodeId],
    most: u32,
    account: &Account,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<Encoding, Reason> {
    let over = || {
        account.over().then(|| Reason::MemoryLimit {
            limit_bytes: account.limit(),
        })
    };
    let polarity = polarities(formulas, left, right);
    let mut vars = vec![None; formulas.len()];
    let mut variables = 0u32;
    for (var, &p) in vars.iter_mut().zip(&polarity) {
        if p != 0 {
            if variables >= most {
                return Err(Reason::IndexLimit);
            }
            *var = Some(variables);
            variables += 1;
        }
    }
    let var = |x: NodeId| vars[x.index()].unwrap_or_else(|| unreachable!("a polarity"));
    let (t, f) = (|x| Lit::positive(var(x)), |x| Lit::negative(var(x)));
    let mut clauses: Vec<Vec<Lit>> = Vec::new();
    let mut add = |clause: Vec<Lit>| {
        account.charge(clause.len() * size_of::<Lit>() + CLAUSE_BYTES);
        clauses.push(clause);
        over()
    };
    for h in left {
        add(vec![t(*h)]).map_or(Ok(()), Err)?;
    }
    for c in right {
        add(vec![f(*c)]).map_or(Ok(()), Err)?;
    }
    for (id, &p) in polarity.iter().enumerate() {
        if id % POLL == 0 && stop(Progress::new(Phase::Search, POLL as u64, 0)) {
            return Err(Reason::Stopped);
        }
        if p == 0 {
            continue;
        }
        let x = NodeId(id as u32);
        // `x` true implies its definition, its definition implies `x`.
        let (forward, backward) = (p & TRUE != 0, p & FALSE != 0);
        let mut definition: Vec<Vec<Lit>> = Vec::new();
        match formulas.node(x) {
            Node::Atom(_) => {}
            Node::True => {
                if backward {
                    definition.push(vec![t(x)]);
                }
            }
            Node::False => {
                if forward {
                    definition.push(vec![f(x)]);
                }
            }
            Node::Not(a) => {
                if forward {
                    definition.push(vec![f(x), f(a)]);
                }
                if backward {
                    definition.push(vec![t(a), t(x)]);
                }
            }
            Node::And(a, b) => {
                if forward {
                    definition.push(vec![f(x), t(a)]);
                    definition.push(vec![f(x), t(b)]);
                }
                if backward {
                    definition.push(vec![f(a), f(b), t(x)]);
                }
            }
            Node::Or(a, b) => {
                if forward {
                    definition.push(vec![f(x), t(a), t(b)]);
                }
                if backward {
                    definition.push(vec![f(a), t(x)]);
                    definition.push(vec![f(b), t(x)]);
                }
            }
            Node::Implies(a, b) => {
                if forward {
                    definition.push(vec![f(x), f(a), t(b)]);
                }
                if backward {
                    definition.push(vec![t(a), t(x)]);
                    definition.push(vec![f(b), t(x)]);
                }
            }
            Node::Iff(a, b) => {
                if forward {
                    definition.push(vec![f(x), f(a), t(b)]);
                    definition.push(vec![f(x), t(a), f(b)]);
                }
                if backward {
                    definition.push(vec![t(a), t(b), t(x)]);
                    definition.push(vec![f(a), f(b), t(x)]);
                }
            }
        }
        for clause in definition {
            add(clause).map_or(Ok(()), Err)?;
        }
    }
    Ok(Encoding {
        vars,
        variables,
        clauses,
    })
}

/// Decides a classical sequent as [`super::decide`] does, with at most
/// `most` variables, which the tests lower.
pub(super) fn decide(
    sequent: &Sequent,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
    most: u32,
) -> Result<Outcome, Error> {
    let logic = Logic::Classical;
    let formulas = sequent.formulas();
    let account = Account::holding(limits.memory_bytes, 0);
    let mut statistics = Statistics::default();
    let encoded = encode(
        formulas,
        sequent.left(),
        sequent.right(),
        most,
        &account,
        &mut stop,
    );
    let found = encoded.and_then(|clauses| {
            statistics.clauses = clauses.clauses.len() as u64;
            let work = limits.work.unwrap_or(u64::MAX);
            let mut solver = Batsat::recording(work, &account, &mut stop);
            for _ in 0..clauses.variables {
                solver.new_var();
            }
            // The solver copies the clauses: it is charged for them, and
            // refused once over, before it holds them all.
            for (n, clause) in clauses.clauses.iter().enumerate() {
                solver.add_clause(clause);
                if n % POLL == 0
                    && let Some(reason) = solver.poll(POLL as u64)
                {
                    return Err(reason);
                }
            }
            let answer = solver.solve(&[]);
            statistics.calls = solver.calls();
            statistics.conflicts = solver.conflicts();
            match answer {
                Answer::Satisfiable => {
                    let atoms = assignment(formulas, &clauses, &solver);
                    Ok(Found::Assignment(atoms))
                }
                Answer::Unsatisfiable => {
                    let mut proof = solver.proof().unwrap_or_default();
                    account.charge(Step::bytes(0));
                    proof.push(Step::Add(Box::new([])));
                    Ok(Found::Proof(clauses, proof))
                }
                Answer::Stopped(reason) => Err(reason),
            }
        });
    let (verdict, model) = match found {
        Err(reason) => (Verdict::Unknown(reason), None),
        Ok(Found::Assignment(atoms)) => {
            statistics.worlds = 1;
            let model = Model {
                worlds: vec![World {
                    atoms,
                    falsity: false,
                    above: Vec::new(),
                }],
            };
            model.refutes(sequent, logic, &mut stop)?;
            (Verdict::NotValid, Some(model))
        }
        Ok(Found::Proof(clauses, proof)) => {
            statistics.proof_steps = proof.len() as u64;
            // The solver is gone; the clauses and the proof stay.
            let held = clauses
                .clauses
                .iter()
                .map(|c| c.len() * size_of::<Lit>() + size_of::<Vec<Lit>>())
                .chain(proof.iter().map(|step| {
                    let (Step::Add(c) | Step::Delete(c)) = step;
                    Step::bytes(c.len())
                }))
                .sum::<usize>();
            let account = Account::holding(limits.memory_bytes, held as u64);
            match drup::check(
                clauses.variables,
                &clauses.clauses,
                &proof,
                limits,
                &account,
                &mut stop,
            ) {
                Ok(()) => {}
                Err(Rejection::Refused(refusal)) => return Err(Error::Refused(refusal)),
                Err(Rejection::Flaw(flaw)) => {
                    return Err(Error::ReadBack {
                        calculus: logic.calculus(),
                        reason: format!("the solver's clausal proof fails its check: {flaw}"),
                    });
                }
            }
            drop((clauses, proof));
            match by_embedding(sequent, limits, &mut stop)? {
                Ok(derivation) => (Verdict::Valid(Box::new(derivation)), None),
                Err(reason) => (Verdict::Unknown(reason), None),
            }
        }
    };
    Ok(Outcome {
        verdict,
        logic,
        statistics,
        model,
    })
}

/// What the solver found.
enum Found {
    /// The atoms true in a model of the clauses, by index, ascending.
    Assignment(Vec<u32>),
    /// The clauses have none: the clauses, and the solver's proof, ending
    /// in the empty clause.
    Proof(Encoding, Vec<Step>),
}

/// Returns the atoms true in the solver's model, by index among the atom
/// names, ascending; an atom without a variable is false.
fn assignment(formulas: &Formulas, clauses: &Encoding, solver: &impl Solver) -> Vec<u32> {
    let mut atoms: Vec<u32> = (0..formulas.len())
        .filter_map(|id| match formulas.node(NodeId(id as u32)) {
            Node::Atom(a) => clauses.vars[id]
                .filter(|&v| solver.value(Lit::positive(v)))
                .map(|_| a),
            Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
        })
        .collect();
    atoms.sort_unstable();
    atoms
}

/// Returns the derivation of LK of a sequent the checked proof showed
/// valid, by the embedding within the same limits, or why the embedding's
/// search stopped before it found one.
fn by_embedding(
    sequent: &Sequent,
    limits: &Limits,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<Result<Derivation, Reason>, Error> {
    let options = Options::default()
        .with_logic(Logic::Classical)
        .with_method(Some(Method::Embedding));
    // A trait object, since `decide` reaches this function again through
    // `Method::Sat` and a generic stop would wrap itself without end.
    let stop: &mut dyn FnMut(Progress) -> bool = stop;
    let outcome =
        super::super::decide(sequent, &options, &search::Options::default(), limits, stop)?;
    match outcome.verdict {
        Verdict::Valid(derivation) => Ok(Ok(*derivation)),
        Verdict::NotValid => Err(Error::ReadBack {
            calculus: Logic::Classical.calculus(),
            reason: "the embedding refutes a sequent whose clausal proof passed its check"
                .to_owned(),
        }),
        Verdict::Unknown(reason) => Ok(Err(reason)),
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::MOST_VARIABLES;
    use super::*;
    use crate::limits::Refusal;
    use crate::linear::search::generate::Rng;
    use crate::ordinary::tests::{generated_sequent, valid_by_truth_table};

    /// Returns the pigeonhole sequent of `n + 1` pigeons and `n` holes:
    /// every pigeon in a hole and no two in one, which cannot all hold.
    /// Its refutation takes the solver thousands of conflicts from
    /// `n = 7`, and clauses deleted on the way.
    fn pigeons(n: usize) -> Sequent {
        let mut hypotheses = Vec::new();
        for i in 0..=n {
            let holes: Vec<String> = (0..n).map(|j| format!("p{i}h{j}")).collect();
            hypotheses.push(holes.join(" \\/ "));
        }
        for j in 0..n {
            for i in 0..=n {
                for k in i + 1..=n {
                    hypotheses.push(format!("~(p{i}h{j} /\\ p{k}h{j})"));
                }
            }
        }
        format!("{} |-", hypotheses.join(", ")).parse().unwrap()
    }

    /// Encodes the sequent and refutes its clauses, returning them and the
    /// solver's proof ending in the empty clause.
    fn refuted(sequent: &Sequent) -> (Encoding, Vec<Step>) {
        let account = Account::holding(None, 0);
        let clauses = encode(
            sequent.formulas(),
            sequent.left(),
            sequent.right(),
            MOST_VARIABLES,
            &account,
            &mut |_| false,
        )
        .unwrap();
        let mut stop = |_| false;
        let mut solver = Batsat::recording(u64::MAX, &account, &mut stop);
        for _ in 0..clauses.variables {
            solver.new_var();
        }
        for clause in &clauses.clauses {
            solver.add_clause(clause);
        }
        assert_eq!(solver.solve(&[]), Answer::Unsatisfiable);
        let mut proof = solver.proof().unwrap();
        proof.push(Step::Add(Box::new([])));
        (clauses, proof)
    }

    /// Checks the proof against the clauses within the default limits.
    fn checked(clauses: &Encoding, proof: &[Step]) -> Result<(), Rejection> {
        let limits = Limits::default();
        let account = Account::holding(limits.memory_bytes, 0);
        drup::check(
            clauses.variables,
            &clauses.clauses,
            proof,
            &limits,
            &account,
            |_| false,
        )
    }

    /// On generated sequents and a few written ones, some with several
    /// formulas right of `⊢`, the solver agrees with the truth table and
    /// with the embedding; each derivation passes the checker again and
    /// each model refutes its sequent again.
    #[test]
    fn agrees_with_the_truth_table_and_the_embedding() {
        let limits = Limits::default();
        let mut rng = Rng::new(54);
        let written = [
            "a \\/ ~a",
            "((a -> b) -> a) -> a",
            "a \\/ b |- a, b",
            "a |- b, c",
            "|-",
            "a, ~a |-",
            "false |-",
            "|- true",
            "true |- false",
            "(a <-> b) <-> (b <-> a)",
            "a <-> b, b |- a",
            "a |- a",
        ]
        .map(|text| text.parse::<Sequent>().unwrap());
        let generated = (0..400).map(|_| generated_sequent(&mut rng));
        let (mut proved, mut refuted) = (0, 0);
        for sequent in written.into_iter().chain(generated) {
            let outcome = decide(&sequent, &limits, |_| false, MOST_VARIABLES)
                .unwrap_or_else(|e| panic!("{sequent}: {e}"));
            let valid = match &outcome.verdict {
                Verdict::Valid(derivation) => {
                    assert_eq!(derivation.check(&limits, |_| false), Ok(()), "{sequent}");
                    assert!(outcome.statistics.proof_steps > 0, "{sequent}");
                    proved += 1;
                    true
                }
                Verdict::NotValid => {
                    let model = outcome.model.as_ref().unwrap();
                    assert_eq!(model.worlds.len(), 1, "{sequent}");
                    assert_eq!(model.refutes(&sequent, Logic::Classical, |_| false), Ok(()));
                    refuted += 1;
                    false
                }
                Verdict::Unknown(reason) => panic!("{sequent}: {reason:?}"),
            };
            assert_eq!(valid, valid_by_truth_table(&sequent), "{sequent}");
            let options = Options::default().with_logic(Logic::Classical);
            let embedded = crate::ordinary::decide(
                &sequent,
                &options,
                &search::Options::default(),
                &limits,
                |_| false,
            )
            .unwrap();
            assert_eq!(
                valid,
                matches!(embedded.verdict, Verdict::Valid(_)),
                "{sequent}"
            );
        }
        assert!(proved > 50 && refuted > 50, "{proved} proved, {refuted} refuted");
    }

    /// A proof of the pigeonhole sequent with a step planted wrong is
    /// refused: the learnt clauses taken away, a learnt clause replaced by
    /// a unit no propagation gives, and a model with a conclusion true.
    #[test]
    fn a_planted_wrong_proof_step_is_refused() {
        let sequent = pigeons(4);
        let (clauses, proof) = refuted(&sequent);
        assert_eq!(checked(&clauses, &proof), Ok(()));
        let last = proof.len() - 1;
        assert!(last > 1, "{proof:?}");
        assert_eq!(
            checked(&clauses, &proof[last..]),
            Err(Rejection::Flaw(drup::Flaw::NotRup { step: 0 }))
        );
        // The first pigeon in the first hole follows from no propagation.
        let mut planted = proof.clone();
        let first = Lit::positive(clauses.vars[0].unwrap());
        assert!(matches!(sequent.formulas().node(NodeId(0)), Node::Atom(_)));
        planted.insert(0, Step::Add(Box::new([first])));
        assert_eq!(
            checked(&clauses, &planted),
            Err(Rejection::Flaw(drup::Flaw::NotRup { step: 0 }))
        );
        // A one-world model that makes a formula right of `⊢` true.
        let sequent: Sequent = "a |- a, b".parse().unwrap();
        let model = Model {
            worlds: vec![World {
                atoms: vec![0],
                falsity: false,
                above: Vec::new(),
            }],
        };
        let Err(Error::Countermodel { reason }) =
            model.refutes(&sequent, Logic::Classical, |_| false)
        else {
            panic!("a true conclusion accepted");
        };
        assert!(reason.contains("forces the conclusion"), "{reason}");
        let two = Model {
            worlds: vec![model.worlds[0].clone(), World::default()],
        };
        assert!(two.refutes(&sequent, Logic::Classical, |_| false).is_err());
    }

    /// The work bound, the stop, the memory bound and the variables'
    /// bound end the search, each as its reason, before the embedding is
    /// asked; the bound on the work also refuses the check of a proof
    /// longer than the solver's work, which deletions make it.
    #[test]
    fn the_search_and_the_check_keep_to_their_bounds() {
        let sequent = pigeons(7);
        let (_, proof) = refuted(&sequent);
        let adds = proof.iter().filter(|s| matches!(s, Step::Add(_))).count() as u64;
        let deletions = proof.len() as u64 - adds;
        assert!(deletions > 0 && adds > 100, "{adds} added, {deletions} deleted");
        let limits = Limits::default();
        let at = |work| limits.with_work(Some(work));
        let outcome = decide(&sequent, &at(adds / 2), |_| false, MOST_VARIABLES).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::WorkLimit { .. })
        ));
        let outcome = decide(&sequent, &limits, |_| true, MOST_VARIABLES).unwrap();
        assert!(matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
        let tight = limits.with_memory_bytes(Some(1024));
        let outcome = decide(&sequent, &tight, |_| false, MOST_VARIABLES).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::MemoryLimit { .. })
        ));
        let outcome = decide(&sequent, &limits, |_| false, 3).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::IndexLimit)
        ));
        // The solver's work is a unit per clause learnt and one for its
        // call, the clauses added but the empty one; the check's is a unit
        // per step, deletions included.
        assert!(matches!(
            decide(&sequent, &at(adds), |_| false, MOST_VARIABLES),
            Err(Error::Refused(Refusal::Work { limit })) if limit == adds
        ));
    }

    /// Where the embedding stops before it finds the derivation of a
    /// sequent the clausal proof showed valid, the outcome is the
    /// embedding's unknown with its reason, the proof's steps counted.
    #[test]
    fn the_embedding_s_bound_is_its_unknown() {
        let sequent: Sequent = "|- ((a -> b) -> a) -> a".parse().unwrap();
        let shallow = Limits::default().with_recursion_depth(1);
        let outcome = decide(&sequent, &shallow, |_| false, MOST_VARIABLES).unwrap();
        assert!(
            matches!(
                outcome.verdict,
                Verdict::Unknown(Reason::RecursionLimit { depth: 1 })
            ),
            "{:?}",
            outcome.verdict
        );
        assert!(outcome.statistics.proof_steps > 0);
    }

    /// A sequent of many formulas asks the stop while it is encoded,
    /// before the solver is called.
    #[test]
    fn the_encoder_asks_the_stop() {
        let atoms: Vec<String> = (0..3 * POLL).map(|i| format!("a{i}")).collect();
        let sequent: Sequent = format!("{} |-", atoms.join(" /\\ ")).parse().unwrap();
        let mut asked = 0;
        let outcome = decide(
            &sequent,
            &Limits::default(),
            |_| {
                asked += 1;
                asked > 1
            },
            MOST_VARIABLES,
        )
        .unwrap();
        assert!(matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
        assert_eq!(outcome.statistics.calls, 0);
    }

    /// The proof of the pigeonhole sequent of eight pigeons, written as
    /// DIMACS and DRUP text, is accepted by drat-trim, an outside checker,
    /// which must be on the `PATH` (`nix shell nixpkgs#drat-trim`). The
    /// proof deletes clauses, so the deletions are checked too.
    #[test]
    #[ignore = "needs drat-trim on the PATH: nix shell nixpkgs#drat-trim"]
    fn drat_trim_accepts_the_proof() {
        let directory = std::env::temp_dir().join(format!("deduxit-drup-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        for (name, sequent) in [
            ("peirce", "|- ((a -> b) -> a) -> a".parse().unwrap()),
            ("pigeons-7", pigeons(7)),
        ] {
            let (clauses, proof) = refuted(&sequent);
            assert_eq!(checked(&clauses, &proof), Ok(()), "{name}");
            let deletions = proof
                .iter()
                .filter(|s| matches!(s, Step::Delete(_)))
                .count();
            let (cnf, drup) = drup::dimacs(clauses.variables, &clauses.clauses, &proof);
            let (cnf_path, drup_path) = (
                directory.join(format!("{name}.cnf")),
                directory.join(format!("{name}.drup")),
            );
            std::fs::write(&cnf_path, cnf).unwrap();
            std::fs::write(&drup_path, drup).unwrap();
            let output = std::process::Command::new("drat-trim")
                .arg(&cnf_path)
                .arg(&drup_path)
                .output()
                .expect("drat-trim on the PATH");
            let text = String::from_utf8_lossy(&output.stdout);
            println!(
                "{name}: {} clauses, {} steps, {deletions} deletions\n{text}",
                clauses.clauses.len(),
                proof.len()
            );
            assert!(
                text.lines().any(|l| l.trim() == "s VERIFIED"),
                "{name}: {text}"
            );
            if name == "pigeons-7" {
                assert!(deletions > 0, "{name} deletes no clause");
            }
        }
        std::fs::remove_dir_all(&directory).unwrap();
    }
}
