// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Essential-net search for unit-free IMLL: the net engine's linking
//! search under the directed criterion of essential nets, in place of the
//! switchings.
//!
//! The sequent's reading orients every edge
//! ([`ProofStructure::is_essential`] says how). The search keeps the
//! reflexive transitive closure of the directed graph as a bit matrix:
//! row `u` holds the occurrences `u` reaches. Every link widens it, and an
//! undo log narrows it again when the link is taken back.
//!
//! A candidate link from an input literal `x` to an output literal `y` is
//! rejected in two cases:
//! - `y` reaches `x`. The link would close a directed cycle, which every
//!   completion keeps.
//! - `x` lies in the antecedent of an implication `l` by input premises
//!   alone, `y` reaches the goal, and `y` does not reach `l`. Along the
//!   link the antecedent would reach the goal around `l`, in every
//!   completion.
//!
//! At the cadence of the net engine's exact test, and on every complete
//! linking, the search tests the *dominator condition*: every path from
//! an implication's antecedent to the goal passes through the
//! implication. It tests it for each implication `l` in output position
//! other than the goal, with antecedent `s`. The test fails in two cases:
//! - `s` does not reach `l`, but reaches the goal or an occurrence `l`
//!   reaches;
//! - some input `⅋` `p` (the one kind of vertex with two edges out) is
//!   reached by `s` and reaches `l`, and a premise `q` of `p` does not
//!   reach `l`, but reaches the goal or an occurrence `l` reaches.
//!
//! Each such failure is a path from `s` around `l` to an occurrence that
//! can never reach `l` again without a cycle, so no completion is
//! correct. On a complete acyclic linking every occurrence reaches the
//! goal, and the test is exactly the dominator condition: a path that
//! leaves the occurrences reaching `l` leaves them at an input `⅋`, or at
//! `s`.
//!
//! A complete linking that passes is an essential net. For an
//! intuitionistic sequent that is a proof net, and it is sequentialized
//! as the net engine's is.

use super::memory::{Account, Charged, bytes_of};
use super::net::{self, Correctness, Linker, Prunes};
use super::planar::Planar;
use super::{Answer, Decide, Engine, NotTaken, Options, Reason, Statistics, Stop, Task, Work};
use crate::Error;
use crate::limits::Limits;
use crate::linear::fragment::{Fragment, Mode, Order};
use crate::linear::nets::essential::{fixed_successors, is_implication};
use crate::linear::nets::{Criterion, ProofStructure, VertexId};
use crate::linear::occurrences::{Forest, OccId, Reading, ShapeError, Side, Sign};
use crate::linear::sequents::{Atom, Kind};

/// The raw index that stands for "no occurrence".
const NONE: u32 = u32::MAX;

/// The bytes per occurrence of the criterion's tables beside the closure:
/// 4 each for its partner and binder, at most 8 for an implication with
/// its antecedent, and 4 each for its share of the links and their marks,
/// of which there is one per two literals. The bits of the branching
/// connectives come on top, a word per 64 occurrences.
const LINEAR_BYTES: usize = 24;

/// The bytes per occurrence of the test against empty antecedents, in the
/// Lambek calculus without them: 4 each for an implication's body and
/// antecedent, and for a source and its binder.
const NONEMPTY_BYTES: usize = 16;

/// The rows of the closure's set-up between two polls of the stop. A row
/// costs a word per 64 occurrences, so a sequent of a hundred thousand
/// occurrences is polled every few milliseconds.
const SETUP_ROWS: usize = 1024;

/// The essential-net engine as the front door calls it.
pub(crate) struct Essentials;

impl Decide for Essentials {
    /// Refuses classical and affine mode, a fragment beyond unit-free MLL
    /// (in intuitionistic mode, unit-free IMLL), and a forest too large for
    /// a structure. In a commutative mode it also refuses a goal other than
    /// the roots, since an essential net's conclusions are the forest's
    /// roots. In the Lambek calculus it takes a goal of distinct formulas,
    /// none inside another, and searches it as a sequent of its own, as the
    /// net engine does in the cyclic mode.
    fn admits(&self, task: &Task<'_>) -> Result<(), Error> {
        let refused = |because| Error::EngineRefused {
            engine: Engine::Essential,
            because,
        };
        let mode = task.mode;
        if !mode.is_intuitionistic() || mode.is_affine() {
            return Err(refused(NotTaken::Mode { mode }));
        }
        if !task.fragment.has_nets() {
            return Err(refused(NotTaken::Fragment {
                decides: Fragment::MLL,
                goal: task.fragment,
            }));
        }
        if !task.roots && !(mode.is_ordered() && net::apart(task.forest, task.goal)) {
            return Err(refused(NotTaken::Goal));
        }
        crate::linear::nets::fits(task.forest.len())
    }

    /// Links the roots on the calling thread, whatever
    /// [`Options::jobs`] says, planar in the Lambek calculus.
    fn decide(
        &self,
        task: &Task<'_>,
        options: &Options,
        limits: &Limits,
        account: &Account,
        _work: &Work,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Result<Answer, Error> {
        let mode = task.mode;
        if !task.roots {
            return net::goal(task, limits, |own| {
                // A goal without a hypothesis has no proof without empty
                // antecedents; any other has the reading the goal had.
                match Reading::of_mode(own, mode) {
                    Ok(reading) => Ok(search(own, &reading, mode, options, limits, account, stop)),
                    Err(ShapeError::EmptyAntecedent) => Ok(net::answer(
                        Ok(false),
                        Statistics::default(),
                        None,
                        limits,
                        stop,
                    )),
                    Err(shape) => Err(Error::NotIntuitionistic(shape)),
                }
            });
        }
        let reading = task
            .reading
            .expect("intuitionistic mode reads the sequent first");
        Ok(search(
            task.forest,
            reading,
            mode,
            options,
            limits,
            account,
            stop,
        ))
    }
}

/// Runs the essential-net engine on the forest of a sequent of unit-free
/// IMLL read by `reading` in `mode`, and returns its answer. For a proved
/// sequent the answer holds the proof net the proof was read off.
///
/// The closure is charged to `account`. `stop` is polled during the
/// closure's set-up, once per literal chosen and once per exact test that
/// fails. In the Lambek calculus the links are planar ([`Planar`]), under
/// the prunes of a planar search.
pub(crate) fn search(
    forest: &Forest,
    reading: &Reading<'_>,
    mode: Mode,
    options: &Options,
    limits: &Limits,
    account: &Account,
    stop: &mut dyn FnMut(u64) -> bool,
) -> Answer {
    // Intuitionistic mode has no Mix.
    if !net::counts_admit(forest, false) {
        return net::answer(Ok(false), Statistics::default(), None, limits, stop);
    }
    let (result, statistics, structure) = match Essential::new(forest, reading, mode, account, stop)
    {
        Err(reason) => (Err(reason), Statistics::default(), None),
        Ok(criterion) if mode.is_ordered() => {
            let criterion = Planar::new(criterion);
            let mut engine = Linker::with(criterion, options, Stop::Closure(stop), Prunes::PLANAR);
            let result = engine.run();
            let structure =
                matches!(result, Ok(true)).then(|| engine.criterion.inner().structure());
            (result, engine.statistics, structure)
        }
        Ok(criterion) => {
            let mut engine = Linker::with(criterion, options, Stop::Closure(stop), Prunes::ALL);
            let result = engine.run();
            let structure = matches!(result, Ok(true)).then(|| engine.criterion.structure());
            (result, engine.statistics, structure)
        }
    };
    debug_assert!(
        structure
            .as_ref()
            .is_none_or(|net| net.is_essential(|_| false).is_ok()),
        "the search's test is the essential criterion"
    );
    net::answer(result, statistics, structure, limits, stop)
}

/// A change of the closure, which the undo log takes back: the row, the
/// word in it, and what the word held before.
#[derive(Clone, Copy, Debug)]
struct Change {
    /// The row: an occurrence, below the forest's length.
    row: u32,
    /// The word in the row, below the row's length in words.
    word: u32,
    /// The word's bits before the change.
    old: u64,
}

/// The essential criterion over a forest of unit-free IMLL: the links
/// made, the reflexive transitive closure of the essential net they make,
/// and its undo log.
pub(super) struct Essential<'a> {
    /// The forest linked.
    forest: &'a Forest,
    /// Per occurrence, the literal it is linked to, or `NONE`.
    partner: Box<[u32]>,
    /// The links made, in order, each as the search gave it.
    links: Vec<(VertexId, VertexId)>,
    /// The words of a row of the closure.
    words: usize,
    /// The closure, row by row: bit `v` of row `u` says that `u` reaches
    /// `v`. Every occurrence reaches itself.
    reach: Box<[u64]>,
    /// The words the links made have changed, in the order they changed.
    log: Vec<Change>,
    /// Per link made, the length of the log before it.
    marks: Vec<usize>,
    /// The bits of the input `⅋`s, the occurrences with two edges out, as
    /// a row.
    branching: Box<[u64]>,
    /// The implications in output position other than the goal, each with
    /// its antecedent.
    implications: Box<[(u32, u32)]>,
    /// Per occurrence in input position, the implication whose antecedent
    /// it lies in by input premises alone. `NONE` inside a hypothesis or
    /// the goal's antecedent.
    binder: Box<[u32]>,
    /// The goal.
    goal: u32,
    /// In the Lambek calculus without empty antecedents, what its test
    /// of a complete linking reads; `None` in every other mode.
    nonempty: Option<Nonempty>,
    /// The order of the structure the links make.
    order: Order,
    /// What the closure and the log charge to the search's account.
    charged: Charged<'a>,
}

/// What the test against empty antecedents reads: every implication but
/// the goal, with its consequent and antecedent, and every *source*, with
/// the implication that binds it.
///
/// A source is an occurrence with no edge in: a hypothesis of the
/// sequent, or the antecedent of an implication, which binds it. In an
/// essential net the term at an output occurrence is what reaches it. Its
/// free hypotheses are the sources that reach it, less those bound inside
/// it, whose implication reaches it too. A proof of the Lambek calculus
/// keeps a hypothesis in every sequent exactly when every subterm has a
/// free one. That holds exactly when the body of every implication but
/// the goal has a free hypothesis besides the one its implication binds.
/// The goal's body has the sequent's hypotheses, and the reading asks for
/// at least one.
struct Nonempty {
    /// For each implication but the goal, its consequent and its
    /// antecedent.
    bodies: Box<[(u32, u32)]>,
    /// Each source, with the implication whose antecedent it is, or `NONE`
    /// for a hypothesis.
    sources: Box<[(u32, u32)]>,
}

impl<'a> Essential<'a> {
    /// Returns the criterion over a forest of unit-free IMLL without a
    /// link, its closure that of the edges every structure has. The
    /// closure is charged before it is allocated.
    ///
    /// Fails with [`Reason::MemoryLimit`] when the account cannot hold the
    /// closure or the allocator cannot give it, and with
    /// [`Reason::Stopped`] when `stop` fires. `stop` is polled every
    /// [`SETUP_ROWS`] rows of the set-up.
    fn new(
        forest: &'a Forest,
        reading: &Reading<'_>,
        mode: Mode,
        account: &'a Account,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Result<Self, Reason> {
        let n = forest.len();
        let words = n.div_ceil(64);
        // Count in `u64` first: a matrix of more than 4 GB does not fit a
        // 32-bit `usize`. The forest has fewer than 2³² occurrences, so the
        // product is below 2⁵⁹. Once the whole fits a `usize`, so does every
        // row index `u · words + w` below it. Every step is checked, and a
        // charge that does not fit is the memory limit.
        let cells = n as u64 * words as u64;
        let limit = Reason::MemoryLimit {
            limit_bytes: account.limit(),
        };
        let linear = if mode.allows_empty_antecedents() {
            LINEAR_BYTES
        } else {
            LINEAR_BYTES + NONEMPTY_BYTES
        };
        let held = usize::try_from(cells)
            .ok()
            .and_then(|cells| cells.checked_mul(size_of::<u64>()))
            .and_then(|matrix| matrix.checked_add(n.checked_mul(linear)?))
            .and_then(|bytes| bytes.checked_add(words * size_of::<u64>()))
            .filter(|&bytes| bytes <= isize::MAX as usize && account.fits(bytes))
            .ok_or(limit)?;
        let mut charged = Charged::new(account);
        charged.charge(held);
        // `cells` fits a `usize`, since `held` does. Without a bound the
        // charge always fits, so a matrix the allocator cannot give is the
        // memory limit too, never an abort.
        let mut reach = Vec::new();
        reach.try_reserve_exact(cells as usize).map_err(|_| limit)?;
        reach.resize(cells as usize, 0u64);
        let mut reach = reach.into_boxed_slice();
        let mut rows = 0usize;
        let mut poll = |stop: &mut dyn FnMut(u64) -> bool| {
            rows += 1;
            rows.is_multiple_of(SETUP_ROWS) && stop(0)
        };

        let position = |o: OccId| reading.position(o);
        let set = |row: &mut [u64], v: usize| row[v / 64] |= 1 << (v % 64);
        // The fixed edges go from an input connective to its input
        // premises, which come after it, and from an output occurrence to
        // its parent, which comes before it. So the rows are filled for the
        // inputs from the last up, then for the outputs from the first
        // down, each from rows already complete.
        for o in forest.ids().rev().filter(|&o| position(o) == Side::Input) {
            if poll(stop) {
                return Err(Reason::Stopped);
            }
            let (u, row) = (o.index(), o.index() * words);
            set(&mut reach[row..row + words], u);
            for s in fixed_successors(forest, reading, o) {
                let from = s.index() * words;
                for w in 0..words {
                    reach[row + w] |= reach[from + w];
                }
            }
        }
        for o in forest.ids().filter(|&o| position(o) == Side::Output) {
            if poll(stop) {
                return Err(Reason::Stopped);
            }
            let (u, row) = (o.index(), o.index() * words);
            set(&mut reach[row..row + words], u);
            for s in fixed_successors(forest, reading, o) {
                let from = s.index() * words;
                for w in 0..words {
                    reach[row + w] |= reach[from + w];
                }
            }
        }

        let goal = reading.goal();
        let mut branching = vec![0u64; words].into_boxed_slice();
        let mut implications = Vec::new();
        let mut binder = vec![NONE; n].into_boxed_slice();
        for o in forest.ids() {
            if position(o) == Side::Input {
                if forest.kind(o) == Kind::Par {
                    set(&mut branching, o.index());
                }
                binder[o.index()] = match forest.parent(o) {
                    // An input occurrence under an output one is the
                    // antecedent of an implication.
                    Some(p) if position(p) == Side::Output && p != goal => p.get(),
                    Some(p) if position(p) == Side::Input => binder[p.index()],
                    _ => NONE,
                };
            } else if is_implication(forest, reading, o) && o != goal {
                let (antecedent, _) = reading.implication(o).expect("an implication");
                implications.push((o.get(), antecedent.get()));
            }
        }
        let nonempty = (!mode.allows_empty_antecedents()).then(|| {
            let bodies = forest
                .ids()
                .filter(|&o| o != goal)
                .filter_map(|o| {
                    reading
                        .implication(o)
                        .filter(|_| is_implication(forest, reading, o))
                })
                .map(|(antecedent, consequent)| (consequent.get(), antecedent.get()))
                .collect();
            let hypotheses = reading.hypotheses().map(|h| (h.get(), NONE));
            let antecedents = forest
                .ids()
                .filter(|&o| is_implication(forest, reading, o))
                .map(|o| {
                    (
                        reading.implication(o).expect("an implication").0.get(),
                        o.get(),
                    )
                });
            Nonempty {
                bodies,
                sources: hypotheses.chain(antecedents).collect(),
            }
        });
        Ok(Self {
            forest,
            partner: vec![NONE; n].into_boxed_slice(),
            links: Vec::with_capacity(forest.all_literals().len() / 2),
            words,
            reach,
            log: Vec::new(),
            marks: Vec::with_capacity(forest.all_literals().len() / 2),
            branching,
            implications: implications.into_boxed_slice(),
            binder,
            goal: goal.get(),
            nonempty,
            order: mode.order(),
            charged,
        })
    }

    /// Returns whether the body of every implication but the goal has a
    /// free hypothesis besides the one it binds ([`Nonempty`]): exact on
    /// a complete linking, which is the only one it is asked of.
    fn keeps_antecedents(&self, nonempty: &Nonempty) -> bool {
        nonempty.bodies.iter().all(|&(body, bound)| {
            nonempty.sources.iter().any(|&(source, binder)| {
                source != bound
                    && self.reaches(source, body)
                    && (binder == NONE || !self.reaches(binder, body))
            })
        })
    }

    /// Returns whether `u` reaches `v`.
    fn reaches(&self, u: u32, v: u32) -> bool {
        let (u, v) = (u as usize, v as usize);
        self.reach[u * self.words + v / 64] >> (v % 64) & 1 == 1
    }

    /// Returns the row of `u`: the occurrences it reaches.
    fn row(&self, u: u32) -> &[u64] {
        let start = u as usize * self.words;
        &self.reach[start..start + self.words]
    }

    /// Returns whether `q`, which does not reach `l`, reaches the goal or
    /// an occurrence that `l` reaches. Such a path from `q` never comes
    /// back to `l` in an acyclic completion, where it ends at the goal.
    fn escapes(&self, q: u32, l: u32) -> bool {
        self.reaches(q, self.goal) || self.row(q).iter().zip(self.row(l)).any(|(a, b)| a & b != 0)
    }

    /// Returns whether no path from the antecedent `s` of the implication
    /// `l` is known to pass around `l`, by the test the module's
    /// documentation describes.
    fn dominated(&self, l: u32, s: u32) -> bool {
        if !self.reaches(s, l) {
            return !self.escapes(s, l);
        }
        for (w, (&reached, &branching)) in self.row(s).iter().zip(&self.branching).enumerate() {
            let mut bits = reached & branching;
            while bits != 0 {
                let p = (w * 64) as u32 + bits.trailing_zeros();
                bits &= bits - 1;
                if !self.reaches(p, l) {
                    continue;
                }
                let p = OccId::new(p);
                for q in [self.forest.left(p), self.forest.right(p)] {
                    let q = q.expect("a `⅋` has two premises").get();
                    if !self.reaches(q, l) && self.escapes(q, l) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Returns the link's literals as the edge runs: from the input
    /// literal `~a` to the output literal `a`.
    fn oriented(&self, x: OccId, y: OccId) -> (u32, u32) {
        match self.forest.sign(x) {
            Some(Sign::Dual) => (x.get(), y.get()),
            _ => (y.get(), x.get()),
        }
    }

    /// Makes room for one more change in the log, charging any growth to
    /// the account. Fails with [`Reason::MemoryLimit`] when the account or
    /// the allocator refuses it.
    fn reserve(&mut self) -> Result<(), Reason> {
        if self.log.len() < self.log.capacity() {
            return Ok(());
        }
        let before = bytes_of(&self.log);
        let more = self.log.capacity().max(1024);
        if !self
            .charged
            .account()
            .fits(more.saturating_mul(size_of::<Change>()))
        {
            return Err(Reason::MemoryLimit {
                limit_bytes: self.charged.account().limit(),
            });
        }
        self.log
            .try_reserve_exact(more)
            .map_err(|_| Reason::MemoryLimit {
                limit_bytes: self.charged.account().limit(),
            })?;
        self.charged.resize(before, bytes_of(&self.log));
        Ok(())
    }

    /// Returns the proof structure of the links made, in the mode's
    /// order.
    pub(super) fn structure(&self) -> ProofStructure {
        let criterion = Criterion {
            order: self.order,
            ..Criterion::MLL
        };
        let mut net = ProofStructure::new(self.forest.clone(), criterion)
            .expect("the engine admits unit-free IMLL only");
        for &(x, y) in &self.links {
            net.link_unchecked(x, y);
        }
        net
    }
}

impl Correctness for Essential<'_> {
    fn forest(&self) -> &Forest {
        self.forest
    }

    fn mate(&self, x: OccId) -> Option<OccId> {
        let p = self.partner[x.index()];
        (p != NONE).then(|| OccId::new(p))
    }

    fn links(&self) -> &[(VertexId, VertexId)] {
        &self.links
    }

    fn is_complete(&self) -> bool {
        2 * self.links.len() == self.forest.all_literals().len()
    }

    /// Rejects a link whose output literal reaches its input literal: it
    /// would close a directed cycle. Rejects one whose input literal lies
    /// in the antecedent of an implication by input premises alone, while
    /// its output literal reaches the goal and not the implication.
    fn admits(&self, x: OccId, y: OccId) -> bool {
        let (i, o) = self.oriented(x, y);
        if self.reaches(o, i) {
            return false;
        }
        let l = self.binder[i as usize];
        l == NONE || self.reaches(o, l) || !self.reaches(o, self.goal)
    }

    /// Adds the edge from the input literal to the output literal: ORs the
    /// output literal's row into every row that reaches the input literal.
    /// The link closes no cycle, so the output literal's row does not reach
    /// the input literal, and the row read is never one written.
    fn link(&mut self, x: OccId, y: OccId) -> Result<(), Reason> {
        let (i, o) = self.oriented(x, y);
        debug_assert!(!self.reaches(o, i), "a link closes no cycle");
        let mark = self.log.len();
        let words = self.words;
        let (column, bit) = (i as usize / 64, i % 64);
        let from = o as usize * words;
        for u in 0..self.forest.len() {
            let row = u * words;
            if self.reach[row + column] >> bit & 1 == 0 {
                continue;
            }
            for w in 0..words {
                let (old, new) = (
                    self.reach[row + w],
                    self.reach[row + w] | self.reach[from + w],
                );
                if old != new {
                    if let Err(reason) = self.reserve() {
                        // Leave the structure as it was.
                        while self.log.len() > mark {
                            let change = self.log.pop().unwrap();
                            self.reach[change.row as usize * words + change.word as usize] =
                                change.old;
                        }
                        return Err(reason);
                    }
                    // `u` is below the forest's length and `w` below the
                    // words of a row, both below 2³².
                    self.log.push(Change {
                        row: u as u32,
                        word: w as u32,
                        old,
                    });
                    self.reach[row + w] = new;
                }
            }
        }
        self.marks.push(mark);
        self.partner[x.index()] = y.get();
        self.partner[y.index()] = x.get();
        self.links.push((VertexId::of(x), VertexId::of(y)));
        Ok(())
    }

    fn unlink(&mut self) -> OccId {
        let (x, y) = self.links.pop().expect("a link to take back");
        let mark = self.marks.pop().expect("a mark per link");
        while self.log.len() > mark {
            let change = self.log.pop().unwrap();
            self.reach[change.row as usize * self.words + change.word as usize] = change.old;
        }
        self.partner[x.index()] = NONE;
        self.partner[y.index()] = NONE;
        x.occ()
    }

    /// Tests the dominator condition of every implication but the goal.
    /// The closure keeps the graph acyclic by itself. In the Lambek
    /// calculus without empty antecedents, a complete linking must also
    /// have a free hypothesis in the body of each implication.
    fn test(&mut self) -> bool {
        self.implications.iter().all(|&(l, s)| self.dominated(l, s))
            && (self.nonempty.is_none() || !self.is_complete() || {
                let nonempty = self.nonempty.as_ref().expect("asked");
                self.keeps_antecedents(nonempty)
            })
    }

    /// Returns true: the criterion keeps no skeleton.
    fn balanced(&self, _atom: Atom, _remaining: u32, _tally: &mut [u32]) -> bool {
        true
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::Essential;
    use crate::limits::Limits;
    use crate::linear::fragment::Mode;
    use crate::linear::nets::{Criterion, ProofStructure};
    use crate::linear::occurrences::{Forest, Reading};
    use crate::linear::search::generate::{self, IllRules, Rng};
    use crate::linear::search::memory::Account;
    use crate::linear::search::net::Correctness;
    use crate::linear::search::{
        Engine, Error, NotTaken, Options, Outcome, Reason, Verdict, prove,
    };
    use crate::linear::search::{Statistics, prove_within};
    use crate::linear::sequents::Sequent;

    /// Unit-free IMLL.
    const IMLL: IllRules = IllRules {
        units: false,
        additives: false,
        zero: false,
        exponentials: false,
    };

    /// Parses `input`.
    fn sequent(input: &str) -> Sequent {
        input.parse().unwrap_or_else(|e| panic!("{input:?}: {e}"))
    }

    /// Options that force the engine.
    fn essential() -> Options {
        Options::default().with_engine(Some(Engine::Essential))
    }

    /// The verdict of an outcome as provable or not, after checking a
    /// proof against the checker and, for the essential engine, its net
    /// against the proof: the links the proof's axioms make are the net's.
    fn verdict(text: &str, outcome: Outcome) -> bool {
        let i = Mode::INTUITIONISTIC;
        match outcome.verdict {
            Verdict::Proved(proof) => {
                assert!(outcome.checked);
                assert_eq!(proof.check(i), Ok(()), "{text:?} by {}", outcome.engine);
                if outcome.engine == Engine::Essential {
                    let net = outcome.net.expect("the net found");
                    let read = ProofStructure::from_proof(
                        &proof,
                        Criterion::MLL,
                        &Limits::default(),
                        |_| false,
                    )
                    .unwrap();
                    let sorted = |net: &ProofStructure| {
                        let mut links: Vec<_> = net
                            .links()
                            .iter()
                            .map(|&(x, y)| (x.min(y), x.max(y)))
                            .collect();
                        links.sort_unstable();
                        links
                    };
                    assert_eq!(sorted(&read), sorted(&net), "{text:?}");
                    assert_eq!(net.is_essential(|_| false), Ok(()), "{text:?}");
                }
                true
            }
            Verdict::Unprovable(_) => false,
            Verdict::Unknown(reason) => panic!("{text:?} by {}: {reason}", outcome.engine),
        }
    }

    /// The most copies of one literal (atom and sign) in the sequent.
    fn most_copies(s: &Sequent) -> usize {
        let forest = Forest::new(s).unwrap();
        forest
            .all_literals()
            .chunk_by(|x, y| {
                forest.atom(*x) == forest.atom(*y) && forest.sign(*x) == forest.sign(*y)
            })
            .map(<[_]>::len)
            .max()
            .unwrap_or(0)
    }

    /// On generated intuitionistic sequents and their mutants with no
    /// literal more than three times, the essential-net engine gives the
    /// verdict of the two-sided engine. Where no literal is there more than
    /// twice, it also gives the verdict of the net engine on the embedding.
    /// Every proof passes the checker, and the net it returns is the net of
    /// its proof.
    #[test]
    fn agrees_with_the_two_sided_engine() {
        let i = Mode::INTUITIONISTIC;
        let mut rng = Rng::new(35);
        let (mut compared, mut provable, mut embedded) = (0, 0, 0);
        for _ in 0..300 {
            let budget = 2 + rng.below(10);
            let generate::Ill {
                mut hypotheses,
                mut goal,
                ..
            } = generate::ill(&mut rng, IMLL, 3, budget);
            for mutated in [false, true] {
                if mutated && !generate::mutate_two_sided(&mut rng, &mut hypotheses, &mut goal, 3) {
                    continue;
                }
                let text = generate::two_sided(&hypotheses, &goal);
                let s = sequent(&text);
                // Linking searches are exponential in a literal's copies:
                // the essential engine is asked where none has more than
                // three, the net engine where none has more than two, which
                // a debug build decides quickly.
                let copies = most_copies(&s);
                if copies > 3 {
                    continue;
                }
                let by_essential = prove(&s, i, &essential()).unwrap();
                assert_eq!(by_essential.engine, Engine::Essential);
                let two_sided = Options::default().with_engine(Some(Engine::TwoSided));
                let by_two_sided = prove(&s, i, &two_sided).unwrap();
                let (e, t) = (verdict(&text, by_essential), verdict(&text, by_two_sided));
                assert_eq!(e, t, "{text:?}: essential {e}, two-sided {t}");
                if copies <= 2 {
                    let net = Options::default().with_engine(Some(Engine::Net));
                    let n = verdict(&text, prove(&s, i, &net).unwrap());
                    assert_eq!(e, n, "{text:?}: essential {e}, net {n}");
                    embedded += 1;
                }
                assert!(mutated || e, "{text:?} is provable");
                compared += 1;
                provable += usize::from(e);
            }
        }
        assert!(
            compared > 300 && provable > 150 && provable < compared - 100 && embedded > 200,
            "{provable} of {compared}, {embedded} on the embedding"
        );
    }

    /// Taking links back restores the closure word for word, in stack
    /// order, after links that widen many rows.
    #[test]
    fn unlinking_restores_the_closure() {
        let s = sequent("a -o b, b -o c, c -o d, d -o e |- a -o e");
        let forest = Forest::new(&s).unwrap();
        let reading = Reading::new(&forest).unwrap();
        let account = Account::new(None);
        let mut criterion = Essential::new(
            &forest,
            &reading,
            Mode::INTUITIONISTIC,
            &account,
            &mut |_| false,
        )
        .unwrap();
        let mut snapshots = vec![criterion.reach.clone()];
        let mut linked = 0;
        loop {
            // The first admissible pair, input literal first.
            let next = forest.all_literals().iter().find_map(|&x| {
                let free = |o| criterion.mate(o).is_none();
                (free(x) && forest.sign(x) == Some(crate::linear::occurrences::Sign::Dual))
                    .then(|| {
                        forest
                            .literals(
                                forest.atom(x).unwrap(),
                                crate::linear::occurrences::Sign::Atom,
                            )
                            .iter()
                            .copied()
                            .find(|&y| free(y) && criterion.admits(x, y))
                            .map(|y| (x, y))
                    })
                    .flatten()
            });
            let Some((x, y)) = next else { break };
            criterion.link(x, y).unwrap();
            linked += 1;
            snapshots.push(criterion.reach.clone());
        }
        assert!(criterion.is_complete() && linked == 5, "{linked} links");
        assert!(criterion.test(), "the composition is a net");
        assert!(criterion.log.len() > 10, "{} changes", criterion.log.len());
        while !criterion.links().is_empty() {
            snapshots.pop();
            criterion.unlink();
            assert_eq!(&criterion.reach, snapshots.last().unwrap());
        }
        assert!(criterion.log.is_empty() && criterion.marks.is_empty());
    }

    /// The closure is charged before it is allocated: under a bound it
    /// does not fit, the engine answers unknown at the memory limit, and
    /// a bound that holds it decides.
    #[test]
    fn the_closure_is_charged() {
        let s = sequent(&format!(
            "a0{} |- a40",
            (0..40)
                .map(|k| format!(", a{k} -o a{}", k + 1))
                .collect::<String>()
        ));
        let i = Mode::INTUITIONISTIC;
        let within = |bytes| {
            let limits = Limits {
                memory_bytes: Some(bytes),
                ..Limits::default()
            };
            prove_within(&s, i, &essential(), &limits, |_| false).unwrap()
        };
        // 162 occurrences: 162 rows of three words.
        let outcome = within(2_000);
        assert!(
            matches!(
                outcome.verdict,
                Verdict::Unknown(Reason::MemoryLimit { limit_bytes: 2_000 })
            ),
            "{:?}",
            outcome.verdict
        );
        assert_eq!(outcome.statistics, Statistics::default());
        assert!(matches!(within(1 << 20).verdict, Verdict::Proved(_)));
    }

    /// In the Lambek calculus the essential engine is the only one: at its
    /// memory bound the net engine, whose planar nets may need an empty
    /// antecedent, does not take the goal over, and forced it is refused.
    #[test]
    fn lambek_has_no_handover() {
        let text = format!(
            "a0{} |- a40",
            (0..40)
                .map(|k| format!(", a{k} \\ a{}", k + 1))
                .collect::<String>()
        );
        let s = Sequent::parse_ordered(&text, crate::linear::Order::Cyclic, &Limits::default())
            .unwrap();
        let limits = Limits {
            memory_bytes: Some(2_000),
            ..Limits::default()
        };
        let outcome =
            prove_within(&s, Mode::LAMBEK, &Options::default(), &limits, |_| false).unwrap();
        assert_eq!(outcome.engine, Engine::Essential);
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::MemoryLimit { .. })
        ));
        let net = Options::default().with_engine(Some(Engine::Net));
        assert!(matches!(
            prove(&s, Mode::LAMBEK, &net),
            Err(Error::EngineRefused {
                engine: Engine::Net,
                ..
            })
        ));
    }

    /// Two sequents that weaker rejections would get wrong. The first is
    /// unprovable: its only linking takes a hypothesis's tensor apart
    /// around an implication, which a dominator test without the input
    /// `⅋`s would call a net. The second is provable: its links run into an
    /// implication's body from inside its antecedent, which a binder
    /// rejection without its last condition would refute.
    #[test]
    fn the_review_s_witnesses() {
        let i = Mode::INTUITIONISTIC;
        let decided = |text: &str| verdict(text, prove(&sequent(text), i, &essential()).unwrap());
        assert!(!decided("a -o (a * a) |- (a -o a) * a"));
        assert!(decided("|- b -o (a -o (b * a))"));
    }

    /// The engine refuses what has no essential net: classical and affine
    /// mode, units, and a goal other than the roots.
    #[test]
    fn refusals() {
        let refused = |text: &str, mode: Mode| match prove(&sequent(text), mode, &essential()) {
            Err(Error::EngineRefused {
                engine: Engine::Essential,
                because,
            }) => because,
            other => panic!("{text:?}: {other:?}"),
        };
        assert!(matches!(
            refused("|- a, ~a", Mode::CLASSICAL),
            NotTaken::Mode { .. }
        ));
        let affine = Mode::INTUITIONISTIC.with_affine();
        assert!(matches!(refused("a |- a", affine), NotTaken::Mode { .. }));
        let units = refused("a * 1 |- a", Mode::INTUITIONISTIC);
        assert_eq!(
            units.explained(Engine::Essential).to_string(),
            "essential nets exist for IMLL without units only, not for MLL with units"
        );
    }
}
