// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The Horn engine, which decides a goal that is a Horn program by a
//! search over the markings of a Petri net instead of over sequents.
//!
//! A Horn program has atoms, clauses used once, clauses under `!` that
//! may be used any number of times, and one goal of atoms. It reads as a
//! Petri net: the atoms are places, a marking is how many tokens (atoms)
//! each place holds, and a clause is a transition, which takes tokens
//! from its input places and puts tokens on its output places when it
//! fires. The goal asks whether the net can reach a target marking, and
//! in affine mode whether it can cover one (reach a marking at least the
//! target).
//!
//! A firing of a clause is one step of a proof. It copies the clause,
//! splits the linear zone so that the clause's body gets exactly the
//! tokens it takes, and decomposes the clause's head into new tokens. So
//! the firing sequence found is the proof. Beside the search, the net's
//! state equation may refute the goal.

/// Coverability by the backward algorithm, for affine mode.
mod cover;
/// The state equation, and the weights that refute it.
mod equation;
/// The proof read off a firing sequence.
mod proof;
/// The search over markings.
mod reach;
#[cfg(test)]
mod tests;

use super::memory::{Account, Charged};
use super::{
    Answer, Decide, Engine, Finished, NotTaken, Options, Reason, Refutation, StateEquation,
    Statistics, Task, Work,
};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::Limits;
use crate::linear::fragment::Fragment;
use crate::linear::occurrences::{Forest, OccId, Side, Sign};
use crate::linear::sequents::{Atom, Kind};
use equation::Equation;

/// The engine, as [`Engine::Horn`] names it.
pub(crate) struct Horn;

impl Decide for Horn {
    /// Refuses a goal beyond MELL and one that is no Horn program.
    fn admits(&self, task: &Task<'_>) -> Result<(), Error> {
        let because = if !Fragment::MELL.contains(task.fragment) {
            NotTaken::Fragment {
                decides: Fragment::MELL,
                goal: task.fragment,
            }
        } else if Program::of(task).is_none() {
            NotTaken::Shape
        } else {
            return Ok(());
        };
        Err(Error::EngineRefused {
            engine: Engine::Horn,
            because,
        })
    }

    /// Decides the goal by a search over its net's markings, and reads the
    /// proof off the firing sequence found.
    ///
    /// The search runs on the calling thread, whatever [`Options::jobs`]
    /// says. It goes forward to the target in linear mode and backward
    /// from it in affine mode, with the state equation beside it.
    fn decide(
        &self,
        task: &Task<'_>,
        _options: &Options,
        _limits: &Limits,
        account: &Account,
        _work: &Work,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Result<Answer, Error> {
        // What the counts of the goal's literals rule out needs no
        // search. A goal that never balances an atom may have an infinite
        // net, which the search would explore until the memory bound. The
        // counts are charged to an account of their own and give it back
        // when they go, so the search has the whole bound.
        let counted = super::focus::counts::refutation(
            task.forest,
            task.goal,
            task.fragment,
            task.mode,
            &account.fork(),
            stop,
        );
        if counted != Refutation::Exhausted {
            // The front door takes this refutation as it is and does not
            // count again. A second count would cost as much again, and a
            // stop between the two would report an exhausted search that
            // never ran.
            return Ok(Answer {
                refutation: Some(counted),
                ..Answer::of_arena(
                    task.forest,
                    Finished {
                        result: Ok(None),
                        nodes: Vec::new(),
                        statistics: Statistics::default(),
                        instances: None,
                    },
                    stop,
                )
            });
        }
        let program = Program::of(task).expect("the engine admitted the goal");
        let affine = task.mode.affine;
        let mut equation = Equation::new(affine, account);
        let most = reach::MOST_MARKINGS;
        let (mut found, statistics) = if affine {
            cover::search(program, account, most, &mut equation, stop)
        } else {
            reach::search(program, account, most, &mut equation, stop)
        };
        // A search that ran out of room has given its memory back. The
        // state equation may still refute, with the rest of the time.
        if let Err(Reason::MemoryLimit { .. } | Reason::IndexLimit) = found
            && let Ok(true) = equation.finish(program, stop)
        {
            found = Ok(None);
        }
        // The simplex gives its memory back, so the proof has the whole
        // bound.
        if let Ok(Some(_)) = found {
            equation.release();
        }
        let (result, nodes) = match found {
            Ok(Some(firings)) => {
                match proof::build(
                    task.forest,
                    program,
                    &firings,
                    affine,
                    account,
                    proof::MOST_NODES,
                ) {
                    Ok((root, nodes)) => (Ok(Some(root)), nodes),
                    Err(reason) => (Err(reason), Vec::new()),
                }
            }
            Ok(None) => (Ok(None), Vec::new()),
            Err(reason) => (Err(reason), Vec::new()),
        };
        let mut answer = Answer::of_arena(
            task.forest,
            Finished {
                result,
                nodes,
                statistics,
                instances: None,
            },
            stop,
        );
        answer.refutation = equation
            .certificate()
            .map(|weights| program.refutation(weights));
        Ok(answer)
    }
}

/// Whether a goal is a Horn program with a clause under `!`, which makes
/// it a Petri net. This is the feature of the engine's row in the
/// dispatch.
pub(crate) fn is_net(task: &Task<'_>) -> bool {
    Program::of(task).is_some_and(|program| !program.quests.is_empty())
}

/// A list of arcs: places with their weights.
type Arcs = Vec<(u32, u32)>;

impl Program {
    /// A transition's inputs, each a place and a weight, by place.
    fn inputs(&self, t: &Transition) -> &[(u32, u32)] {
        t.inputs_in(&self.arcs)
    }

    /// A transition's outputs, each a place and a weight, by place.
    fn outputs(&self, t: &Transition) -> &[(u32, u32)] {
        t.outputs_in(&self.arcs)
    }

    /// Returns the program with every transition reversed, and the initial
    /// and the target marking swapped.
    ///
    /// A firing sequence of one program, read backward, is a firing
    /// sequence of the other. Only the net is kept, for a search.
    fn reversed(&self) -> Program {
        let mut arcs = Vec::with_capacity(self.arcs.len());
        let transitions = self
            .transitions
            .iter()
            .map(|t| {
                let inputs = arcs.len() as u32;
                arcs.extend_from_slice(self.outputs(t));
                let outputs = arcs.len() as u32;
                arcs.extend_from_slice(self.inputs(t));
                Transition {
                    clause: t.clause,
                    inputs,
                    outputs,
                    end: arcs.len() as u32,
                }
            })
            .collect();
        Program {
            places: self.places,
            place_of: Vec::new(),
            transitions,
            arcs,
            initial: self.target.clone(),
            target: self.initial.clone(),
            body: self.body,
            quests: Vec::new(),
            markings: Vec::new(),
            goal: self.goal,
            once: Vec::new(),
            alike: Vec::new(),
            dropped: Vec::new(),
        }
    }

    /// Returns, for each place, the most tokens it holds in any marking
    /// that a firing sequence from the initial marking passes.
    ///
    /// That is the place's initial count where no transition raises it,
    /// and no bound (`u32::MAX`) otherwise.
    fn caps(&self) -> Vec<u32> {
        let mut raised = vec![false; self.places];
        for t in &self.transitions {
            // The inputs and the outputs are each sorted by place.
            let mut inputs = self.inputs(t).iter().peekable();
            for &(p, w) in self.outputs(t) {
                while inputs.next_if(|&&(q, _)| q < p).is_some() {}
                let taken = inputs.next_if(|&&(q, _)| q == p).map_or(0, |&(_, v)| v);
                if w > taken {
                    raised[p as usize] = true;
                }
            }
        }
        (0..self.places)
            .map(|p| if raised[p] { u32::MAX } else { self.initial[p] })
            .collect()
    }

    /// Returns the refutation that weights for the places give, each
    /// given as a place and its weight.
    ///
    /// The refutation lists every weight that is not zero: an atom's
    /// place weighs the atom, and a class's place weighs each clause of
    /// the class, since the place counts the class's tickets (one token
    /// per unused clause). It also lists the clauses that can never fire.
    fn refutation(&self, weights: &[(u32, i64)]) -> Refutation {
        let mut weight = vec![0; self.places];
        for &(p, w) in weights {
            weight[p as usize] = w;
        }
        let tickets = &weight[self.places - self.once.len()..];
        let atoms = self
            .place_of
            .iter()
            .enumerate()
            .filter(|&(_, &p)| p != u32::MAX && weight[p as usize] != 0)
            .map(|(a, &p)| (Atom::new(a as u32), weight[p as usize]))
            .collect();
        let mut clauses: Vec<(OccId, i64)> = self
            .once
            .iter()
            .zip(tickets)
            .filter(|&(_, &w)| w != 0)
            .flat_map(|(class, &w)| class.iter().map(move |&clause| (clause, w)))
            .collect();
        clauses.sort_unstable();
        let mut dropped = self.dropped.clone();
        dropped.sort_unstable();
        Refutation::StateEquation(StateEquation {
            atoms,
            clauses,
            dropped,
        })
    }
}

/// The clause a transition fires: one under a `?`, whose occurrence is
/// copied at every firing, or one of a class of interchangeable clauses
/// used once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Clause {
    /// The occurrence of the clause under its `?`.
    Reusable(OccId),
    /// The index of the class in [`Program::once`].
    Once(u32),
}

/// The parents of a search's markings or elements. For each entry, the
/// entry it came from and the transition between them, packed as
/// `parent << 32 | transition`. The first entry's parent is
/// [`Parents::ROOT`].
#[derive(Default)]
struct Parents(Vec<u64>);

impl Parents {
    /// The parent recorded for the first entry, which has none.
    const ROOT: u32 = u32::MAX;

    /// How many entries have their parent recorded.
    fn len(&self) -> usize {
        self.0.len()
    }

    /// Makes room for one more entry and charges what the list grew by.
    /// Returns false when the bound has no room.
    fn room(&mut self, charged: &mut Charged<'_>) -> bool {
        reach::room(&mut self.0, 1, charged)
    }

    /// Records the next entry's parent and the transition from it.
    fn push(&mut self, parent: u32, transition: u32) {
        self.0.push(u64::from(parent) << 32 | u64::from(transition));
    }

    /// Returns the transitions on the way from entry `i` up to the first
    /// entry, the one into `i` first.
    fn up(&self, mut i: u32) -> Vec<u32> {
        let mut transitions = Vec::new();
        loop {
            let parent = self.0[i as usize];
            if (parent >> 32) as u32 == Self::ROOT {
                return transitions;
            }
            transitions.push(parent as u32);
            i = (parent >> 32) as u32;
        }
    }
}

/// A transition of the net: the clause it fires and its arcs.
#[derive(Clone, Copy, Debug)]
struct Transition {
    /// The clause it fires.
    clause: Clause,
    /// Where its inputs start in [`Program::arcs`].
    inputs: u32,
    /// Where its outputs start, and its inputs end.
    outputs: u32,
    /// Where its outputs end.
    end: u32,
}

impl Transition {
    /// Its inputs among a program's `arcs`, each a place and a weight, by
    /// place.
    fn inputs_in(self, arcs: &[(u32, u32)]) -> &[(u32, u32)] {
        &arcs[self.inputs as usize..self.outputs as usize]
    }

    /// Its outputs among a program's `arcs`, each a place and a weight, by
    /// place.
    fn outputs_in(self, arcs: &[(u32, u32)]) -> &[(u32, u32)] {
        &arcs[self.outputs as usize..self.end as usize]
    }
}

/// A Horn program read off a goal, as a Petri net.
///
/// Every count below is at most the forest's occurrences, which are
/// fewer than 2³², so none of them overflows its `u32`. A weight counts
/// literals of one clause, a marking's count literals of the goal, and a
/// place an atom or a class of clauses.
pub(super) struct Program {
    /// How many places there are: one per atom of the goal, then one per
    /// class of clauses used once, which holds a token per unused clause
    /// of the class.
    places: usize,
    /// The place of each atom of the sequent, `u32::MAX` for an atom the
    /// goal does not name.
    place_of: Vec<u32>,
    /// The transitions: the clauses under `?` with distinct arcs, then
    /// the classes of clauses used once.
    transitions: Vec<Transition>,
    /// The arcs of the transitions, each a place and a weight, sorted by
    /// place within the inputs and within the outputs of a transition.
    arcs: Vec<(u32, u32)>,
    /// The marking the search starts from.
    initial: Vec<u32>,
    /// The marking to reach: the goal's atoms, with no clause used once
    /// left unused.
    target: Vec<u32>,
    /// The sign of the atoms in bodies and in the goal; heads and markings
    /// have the other.
    body: Sign,
    /// The goal's members under a `?`.
    quests: Vec<OccId>,
    /// The goal's members that are markings: `⅋` and `⊥` over head
    /// literals.
    markings: Vec<OccId>,
    /// The goal's member to reach: a tensor of body literals and `1`.
    goal: OccId,
    /// The occurrences of each class of clauses used once.
    once: Vec<Vec<OccId>>,
    /// Per transition under `?`, the other clauses under `?` with its
    /// arcs; emptied once the dead transitions are dropped.
    alike: Vec<Vec<OccId>>,
    /// The clauses whose transitions can never fire, which [`live`]
    /// dropped.
    dropped: Vec<OccId>,
}

impl Program {
    /// Returns the Horn program a goal is, without the transitions that
    /// can never fire ([`live`]), or `None` when the goal is no Horn
    /// program.
    ///
    /// The first call on a task reads the program, and the task keeps it.
    fn of<'t>(task: &'t Task<'_>) -> Option<&'t Self> {
        task.program
            .get_or_init(|| {
                Self::read(task).map(|mut program| {
                    live(&mut program);
                    program
                })
            })
            .as_ref()
    }

    /// Reads the Horn program a goal is, or returns `None` when the goal
    /// is none.
    ///
    /// The atoms of the bodies are all written `a`, or else all written
    /// `~a`. Every member under a `?` must be a clause with exactly one
    /// head. Every other member is one of these:
    /// - a marking;
    /// - a clause with exactly one head, used once;
    /// - the goal, a clause without a head, of which there is exactly one.
    ///
    /// A clause is a tensor of body literals and `1`, with at most one
    /// factor a head. A head is a `⅋` of head literals and `⊥`.
    ///
    /// In intuitionistic mode the reading must put the goal and the
    /// clauses' bodies on the right of `⊢` and everything else on the
    /// left, as the proof built for a firing sequence does.
    fn read(task: &Task<'_>) -> Option<Self> {
        [Sign::Atom, Sign::Dual]
            .into_iter()
            .find_map(|body| Self::read_with(task, body))
    }

    /// [`read`](Self::read) with the sign of the bodies' atoms given.
    fn read_with(task: &Task<'_>, body: Sign) -> Option<Self> {
        let forest = task.forest;
        let atoms = forest.sequent().atom_names().len();
        let mut reader = Reader {
            forest,
            body,
            place_of: vec![u32::MAX; atoms],
            places: 0,
        };
        let mut reusable = Vec::new();
        let mut once = Vec::new();
        let mut quests = Vec::new();
        let mut markings = Vec::new();
        let mut goal = None;
        let mut tokens = Vec::new();
        for &member in task.goal {
            if forest.kind(member) == Kind::Quest {
                let clause = forest.left(member)?;
                let (inputs, outputs, head) = reader.clause(clause)?;
                head?;
                reusable.push((clause, inputs, outputs));
                quests.push(member);
            } else if is_head(forest, reader.body, member) {
                tokens.extend(reader.literals(member));
                markings.push(member);
            } else {
                let (inputs, outputs, head) = reader.clause(member)?;
                if head.is_some() {
                    once.push((member, inputs, outputs));
                } else if goal.replace(member).is_some() {
                    return None;
                }
            }
        }
        let goal = goal?;
        if let Some(reading) = task.reading
            && !task.goal.iter().all(|&member| {
                let head = if member == goal {
                    None
                } else if forest.kind(member) == Kind::Quest {
                    Some(reader.head_of(forest.left(member).unwrap()))
                } else {
                    Some(reader.head_of(member))
                };
                forest.subtree(member).all(|x| {
                    // An occurrence is on the left exactly when it holds
                    // the head or lies in it. A marking is its own head.
                    let left = head.is_some_and(|h| forest.is_below(h, x) || forest.is_below(x, h));
                    reading.position(x) == if left { Side::Input } else { Side::Output }
                })
            })
        {
            return None;
        }
        let target_tokens = reader.clause(goal)?.0;
        Some(reader.program(
            Parts {
                reusable,
                once,
                tokens,
                target: target_tokens,
            },
            quests,
            markings,
            goal,
        ))
    }
}

/// The members of a goal read as a net, before the transitions are
/// gathered. Each clause, under `?` or used once, comes with its input and
/// output places; then the tokens of the markings and of the goal.
struct Parts {
    /// The clauses under `?`.
    reusable: Vec<(OccId, Vec<u32>, Vec<u32>)>,
    /// The clauses used once.
    once: Vec<(OccId, Vec<u32>, Vec<u32>)>,
    /// A place per head literal of the markings.
    tokens: Vec<u32>,
    /// A place per body literal of the goal.
    target: Vec<u32>,
}

/// The reader of a goal's members: the forest, the sign of the bodies,
/// and the places given to the atoms read so far.
struct Reader<'a> {
    /// The forest.
    forest: &'a Forest,
    /// The sign of the bodies' atoms.
    body: Sign,
    /// The place of each atom, `u32::MAX` before it is met.
    place_of: Vec<u32>,
    /// The places given out.
    places: u32,
}

impl Reader<'_> {
    /// Returns the place of a literal's atom, given out when the atom is
    /// new.
    fn place(&mut self, literal: OccId) -> u32 {
        let atom = self.forest.atom(literal).expect("a literal").index();
        if self.place_of[atom] == u32::MAX {
            self.place_of[atom] = self.places;
            self.places += 1;
        }
        self.place_of[atom]
    }

    /// Returns the places of the literals of a subformula, one per
    /// literal.
    fn literals(&mut self, o: OccId) -> Vec<u32> {
        let forest = self.forest;
        forest
            .subtree(o)
            .filter(|&x| forest.is_literal(x))
            .map(|x| self.place(x))
            .collect()
    }

    /// Reads a clause. Returns the places of its body literals, the
    /// places of its head's literals, and its head.
    ///
    /// Returns `None` when it is no clause: it has a factor that is
    /// neither a body literal, `1`, a tensor nor a head, or it has two
    /// heads.
    fn clause(&mut self, clause: OccId) -> Option<(Vec<u32>, Vec<u32>, Option<OccId>)> {
        let forest = self.forest;
        let (mut inputs, mut head) = (Vec::new(), None);
        let mut factors = vec![clause];
        while let Some(x) = factors.pop() {
            match forest.kind(x) {
                Kind::Tensor => factors.extend(forest.children(x)),
                Kind::One => {}
                _ if forest.sign(x) == Some(self.body) => inputs.push(self.place(x)),
                _ if head.is_none() && is_head(forest, self.body, x) => head = Some(x),
                _ => return None,
            }
        }
        let outputs = head.map_or_else(Vec::new, |h| self.literals(h));
        Some((inputs, outputs, head))
    }

    /// Returns the head of a clause already read, or the clause itself
    /// for a marking.
    fn head_of(&self, clause: OccId) -> OccId {
        clause_head(self.forest, self.body, clause)
            .flatten()
            .unwrap_or(clause)
    }

    /// Gathers the transitions and returns the program.
    ///
    /// The clauses under `?` give a transition per distinct set of arcs
    /// that changes the marking. The clauses used once give a transition
    /// per class of equal arcs. Each class has a place of its own that
    /// holds a token per clause, and a firing takes one.
    fn program(
        self,
        parts: Parts,
        quests: Vec<OccId>,
        markings: Vec<OccId>,
        goal: OccId,
    ) -> Program {
        /// Returns the arcs of a list of places, one per literal: each
        /// place with its multiplicity, sorted by place.
        fn weights(mut places: Vec<u32>) -> Vec<(u32, u32)> {
            places.sort_unstable();
            let mut arcs: Vec<(u32, u32)> = Vec::new();
            for p in places {
                match arcs.last_mut() {
                    Some((q, w)) if *q == p => *w += 1,
                    _ => arcs.push((p, 1)),
                }
            }
            arcs
        }
        let Reader {
            place_of,
            places,
            body,
            ..
        } = self;
        let mut seen: HashMap<(Arcs, Arcs), u32> = HashMap::default();
        let mut transitions = Vec::new();
        let mut arcs = Vec::new();
        let mut push = |clause, inputs: &[(u32, u32)], outputs: &[(u32, u32)]| {
            let at = |arcs: &Vec<(u32, u32)>| u32::try_from(arcs.len()).expect("arcs are literals");
            let inputs_at = at(&arcs);
            arcs.extend_from_slice(inputs);
            let outputs_at = at(&arcs);
            arcs.extend_from_slice(outputs);
            transitions.push(Transition {
                clause,
                inputs: inputs_at,
                outputs: outputs_at,
                end: at(&arcs),
            });
        };
        let mut alike: Vec<Vec<OccId>> = Vec::new();
        for (clause, inputs, outputs) in parts.reusable {
            let (inputs, outputs) = (weights(inputs), weights(outputs));
            if inputs == outputs {
                continue;
            }
            match seen.get(&(inputs.clone(), outputs.clone())) {
                Some(&t) => alike[t as usize].push(clause),
                None => {
                    push(Clause::Reusable(clause), &inputs, &outputs);
                    seen.insert((inputs, outputs), alike.len() as u32);
                    alike.push(Vec::new());
                }
            }
        }
        seen.clear();
        let mut once: Vec<Vec<OccId>> = Vec::new();
        let mut classes = Vec::new();
        for (clause, inputs, outputs) in parts.once {
            let key = (weights(inputs), weights(outputs));
            let class = *seen.entry(key.clone()).or_insert_with(|| {
                once.push(Vec::new());
                classes.push(key);
                once.len() as u32 - 1
            });
            once[class as usize].push(clause);
        }
        let ticket = |class: usize| places + class as u32;
        for (class, (inputs, outputs)) in classes.iter().enumerate() {
            let mut inputs = inputs.clone();
            inputs.push((ticket(class), 1));
            push(Clause::Once(class as u32), &inputs, outputs);
        }
        let width = places as usize + once.len();
        let mut initial = vec![0; width];
        for p in parts.tokens {
            initial[p as usize] += 1;
        }
        for (class, clauses) in once.iter().enumerate() {
            initial[ticket(class) as usize] = clauses.len() as u32;
        }
        let mut target = vec![0; width];
        for p in parts.target {
            target[p as usize] += 1;
        }
        Program {
            places: width,
            place_of,
            transitions,
            arcs,
            initial,
            target,
            body,
            quests,
            markings,
            goal,
            once,
            alike,
            dropped: Vec::new(),
        }
    }
}

/// Drops the program's transitions that no firing sequence from the
/// initial marking can fire: those with an input place that no marking
/// on the way marks.
///
/// The places that can be marked lie within the least set that holds the
/// places the initial marking marks (the tokens given and the class
/// places of the clauses used once) and the outputs of every transition
/// whose inputs are all in it. This holds by induction on the firing
/// sequence. A transition with an input outside that set is never
/// enabled (able to fire).
///
/// The dropped transitions' clauses go to [`Program::dropped`], with the
/// clauses under `?` that share their arcs. It is one pass, made where
/// the program is read ([`Program::of`]).
fn live(program: &mut Program) {
    let Program {
        transitions,
        arcs,
        initial,
        once,
        alike,
        dropped,
        ..
    } = program;
    let width = initial.len();
    let mut marked: Vec<bool> = initial.iter().map(|&count| count > 0).collect();
    // The inputs each transition still waits for, and the transitions
    // waiting for each place.
    let mut waiting: Vec<u32> = Vec::with_capacity(transitions.len());
    let mut by_input: Vec<Vec<u32>> = vec![Vec::new(); width];
    let mut ready = Vec::new();
    for (t, transition) in transitions.iter().enumerate() {
        let unmarked = transition
            .inputs_in(arcs)
            .iter()
            .filter(|&&(p, _)| !marked[p as usize])
            .inspect(|&&(p, _)| by_input[p as usize].push(t as u32))
            .count();
        waiting.push(unmarked as u32);
        if unmarked == 0 {
            ready.push(t as u32);
        }
    }
    let mut enabled = vec![false; transitions.len()];
    while let Some(t) = ready.pop() {
        enabled[t as usize] = true;
        let transition = transitions[t as usize];
        for &(p, _) in transition.outputs_in(arcs) {
            if !marked[p as usize] {
                marked[p as usize] = true;
                for &u in &by_input[p as usize] {
                    waiting[u as usize] -= 1;
                    if waiting[u as usize] == 0 {
                        ready.push(u);
                    }
                }
            }
        }
    }
    for (t, transition) in transitions.iter().enumerate() {
        if enabled[t] {
            continue;
        }
        match transition.clause {
            Clause::Reusable(clause) => {
                dropped.push(clause);
                dropped.extend_from_slice(&alike[t]);
            }
            Clause::Once(class) => dropped.extend_from_slice(&once[class as usize]),
        }
    }
    *alike = Vec::new();
    let mut t = 0;
    transitions.retain(|_| {
        t += 1;
        enabled[t - 1]
    });
}

/// Whether a subformula is a head for bodies of the sign `body`: `⅋` and
/// `⊥` over literals of the other sign.
pub(crate) fn is_head(forest: &Forest, body: Sign, o: OccId) -> bool {
    forest.subtree(o).all(|x| match forest.kind(x) {
        Kind::Par | Kind::Bot => true,
        _ => forest.sign(x) == Some(!body),
    })
}

/// Returns the head of a clause whose bodies have the sign `body`.
///
/// Returns `Some(None)` for a clause without a head, and `None` for no
/// clause: one with a factor that is neither a body literal, `1`, a
/// tensor nor a head, or with two heads.
#[expect(
    clippy::option_option,
    reason = "no clause, or a clause with or without a head: the goal has none"
)]
pub(crate) fn clause_head(forest: &Forest, body: Sign, clause: OccId) -> Option<Option<OccId>> {
    let mut head = None;
    let mut factors = vec![clause];
    while let Some(x) = factors.pop() {
        match forest.kind(x) {
            Kind::Tensor => factors.extend(forest.children(x)),
            Kind::One => {}
            _ if forest.sign(x) == Some(body) => {}
            _ if head.is_none() && is_head(forest, body, x) => head = Some(x),
            _ => return None,
        }
    }
    Some(head)
}
