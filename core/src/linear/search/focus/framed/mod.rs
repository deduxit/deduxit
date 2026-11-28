// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The zone of a first-order search. Its members are occurrences under
//! binders, with the terms their binders were opened with: a fresh
//! eigenvariable for a `∀`, and a metavariable for the witness of an `∃`.
//! An *eigenvariable* is a fresh constant that a `∀` step introduces. A
//! *metavariable* is a placeholder for a term that unification fills in
//! later. The zone's identities join literals whose terms unify.
//!
//! **Members.** An occurrence outside every binder is its own member. An
//! instance of a binder's body is a *span* of member ids, one per
//! occurrence of the body's subtree. The span has the body's *frame*: the
//! binder's own frame and the new term. So a subformula's member is the
//! member's id moved by as much as the occurrence is. The engine's paths
//! look up no table except where an occurrence is read, which finds the
//! span by its first id. A binder below another is never reached through
//! its parent's span: the rules open it with a span of its own.
//!
//! **Terms** are shared, never built. A term is a term of the sequent's
//! arena read under a frame, whose bound variables are the frame's terms.
//! Or it is a variable of the search: a metavariable, which a binding may
//! point at another term, or an eigenvariable. Unification binds
//! metavariables to such terms with the occurs check, iteratively, each
//! pair of terms once.
//!
//! **Levels** keep the eigenvariable condition: an eigenvariable never
//! occurs in the conclusion of its own `∀`. Eigenvariables are numbered in
//! the order they are made, and an eigenvariable's level is its number
//! plus one. A metavariable's level is the number of eigenvariables made
//! before it. A metavariable is bound only to a term whose eigenvariables
//! are below its level, and the metavariables of that term are lowered to
//! its level, on the trail. An eigenvariable made after a metavariable
//! lies in the subtree of a `∀` above the `∃` that made the metavariable,
//! or on another branch. There none of its terms ever meets the
//! metavariable except through metavariables of lower levels. So a binding
//! the levels allow puts no eigenvariable into the conclusion of its own
//! `∀`, and the checker's condition holds.
//!
//! **Undo.** Every binding and every lowering is on the *trail*, and the
//! spans, frames and variables only grow. A mark is where each stands, and
//! undoing to it pops the trail and truncates the rest. The engine undoes
//! exactly where it releases pending nodes, and where a binder rule or an
//! identity it tried failed.
//!
//! **Answers.** A rule of two premises whose second premise shares an
//! unbound metavariable with the first may need another answer of the
//! first: one that binds it otherwise. The first premise's answer is the
//! tuple of the shared metavariables' terms. When the second premise fails
//! under an answer, it fails under every instance of it (with
//! metavariables no more free and no eigenvariable of a higher level). So
//! the answer is recorded as a pattern, and the first premise is searched
//! again with every binding refused that makes the tuple an instance of a
//! recorded pattern. Bindings only make the tuple more specific. So the
//! refusal at the binding prunes exactly the proofs whose answer is such
//! an instance, and the search of the first premise finds every other
//! answer. It ends when the bindings before the first premise are an
//! instance themselves, or when the first premise has no proof left.

mod closing;
mod keys;
mod zone;

use self::keys::KeyScratch;
use super::choices::Choices;
use super::context::Context;
use super::zone::{Linear, Unrestricted};
use crate::hash::{HashMap, HashSet};
use crate::linear::occurrences::{Forest, Member, OccId, OccSet};
use crate::linear::search::Reason;
use crate::linear::sequents::fo::FirstOrder;
use crate::linear::sequents::{Eigen, FoTerm, FoTermId, Symbol};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// The frame of no binder: an occurrence outside every binder, or a
/// closed term, which reads no frame.
const NONE: u32 = u32::MAX;

#[cfg(test)]
thread_local! {
    /// The first member id a test's search may not give, lowered below the
    /// default's. Only a search far larger than a test runs reaches the
    /// default.
    pub(super) static MEMBER_LIMIT: std::cell::Cell<u32> = const { std::cell::Cell::new(u32::MAX) };
}

/// Returns the first member id a search may not give: `u32::MAX`, which
/// stands for no frame among the frames' parents. In a test it is what
/// the test's thread set.
fn member_limit() -> u32 {
    #[cfg(test)]
    {
        MEMBER_LIMIT.with(std::cell::Cell::get)
    }
    #[cfg(not(test))]
    {
        u32::MAX
    }
}

/// A member of a first-order search. An id below the forest's length is
/// an occurrence of the forest; an id past it is an occurrence in a span.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct FMember(u32);

impl From<FMember> for Member {
    fn from(m: FMember) -> Self {
        Self::new(m.0)
    }
}

/// A term of a search. An id below the arena's length is a term of the
/// sequent's arena under a frame. An id past it, by the variable's index,
/// is a variable of the search, whose frame is [`NONE`], as a closed
/// term's is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TRef {
    /// The term or the variable.
    id: u32,
    /// The frame its bound variables are read in.
    frame: u32,
}

/// A variable of the search.
#[derive(Clone, Copy, Debug)]
enum Var {
    /// The witness of an `∃` step, unbound or bound to a term, with its
    /// level. Every eigenvariable of a term it is bound to lies below the
    /// level.
    Meta {
        /// What it is bound to.
        binding: Option<TRef>,
        /// Its level.
        level: u32,
    },
    /// The eigenvariable of a `∀` step, by its number, one less than its
    /// level.
    Eigen(u32),
}

/// What undoing a step of the trail takes back.
#[derive(Clone, Copy, Debug)]
enum Undone {
    /// The binding of the metavariable of this index.
    Bound(u32),
    /// The lowering of the metavariable of this index from that level.
    Lowered(u32, u32),
    /// What the metavariable of this index was found to stand for.
    Named(u32),
}

/// What a bound metavariable was found to stand for, kept until an undo
/// takes back a binding it rests on.
#[derive(Clone, Copy, Debug)]
enum Named {
    /// Not looked at since it was bound.
    Unknown,
    /// The ground term of this id.
    Ground(u32),
    /// A term that holds the variable of this index, an unbound metavariable
    /// or an eigenvariable. The name holds while that variable stays unbound.
    Open(u32),
}

/// The head of an application in the table of ground terms: a symbol
/// before its first argument, else the application to the arguments
/// before.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Head {
    /// The symbol.
    Symbol(u32),
    /// The application of this id.
    Applied(u32),
}

/// The table of ground terms: a zone's own, or the one every engine of a
/// search on a pool shares. Shared, it gives a ground term one id in all
/// their keys, so that their memo is one.
#[derive(Clone, Debug)]
enum Grounds {
    /// The zone's own.
    Own(HashMap<(Head, u32), u32>),
    /// The pool's.
    #[cfg(feature = "parallel")]
    Shared(Arc<SharedGrounds>),
}

/// The table of ground terms a pool's engines share, and its slots. Every
/// engine's account reads the slots at each poll, so they are an atomic,
/// and the poll takes no lock.
#[cfg(feature = "parallel")]
#[derive(Debug)]
struct SharedGrounds {
    /// The table.
    table: std::sync::Mutex<HashMap<(Head, u32), u32>>,
    /// Its capacity after the last insertion.
    slots: std::sync::atomic::AtomicUsize,
}

impl Grounds {
    /// Returns the name of the application of `symbol` to terms of the names
    /// `args`, as [`Framed::apply`] gives it.
    fn apply(&mut self, symbol: Symbol, args: &[Name]) -> Name {
        match self {
            Self::Own(table) => Framed::apply(table, symbol, args),
            #[cfg(feature = "parallel")]
            Self::Shared(shared) => {
                let mut table = crate::linear::search::lock(&shared.table);
                let name = Framed::apply(&mut table, symbol, args);
                shared.slots.store(table.capacity(), Ordering::Relaxed);
                name
            }
        }
    }

    /// Returns the slots of the table. Every engine that reads a shared table
    /// counts it, so the count is more than the table holds; the memory bound
    /// errs on that side.
    fn capacity(&self) -> usize {
        match self {
            Self::Own(table) => table.capacity(),
            #[cfg(feature = "parallel")]
            Self::Shared(shared) => shared.slots.load(Ordering::Relaxed),
        }
    }
}

/// A step of the walk that names a metavariable's term.
#[derive(Clone, Copy, Debug)]
enum Naming {
    /// Name a term.
    Term(TRef),
    /// Name a variable, by index.
    Var(u32),
    /// Apply the symbol to the last names found, as many as its arity.
    Apply(Symbol, u32),
    /// Keep the last name found as the metavariable's of this index.
    Keep(u32),
}

/// A name a walk found for a term.
#[derive(Clone, Copy, Debug)]
enum Name {
    /// The ground term of this id.
    Ground(u32),
    /// A term holding the variable of this index.
    Open(u32),
    /// A ground term whose id would be too large for a word of a key.
    Past,
}

/// The members of one instance of a binder's body: the ids from `base`
/// to `end`, one per occurrence from `first` on, under `frame`.
#[derive(Clone, Copy, Debug)]
struct Span {
    /// The first id.
    base: u32,
    /// The first id past the span.
    end: u32,
    /// The occurrence of the first id: the body's.
    first: u32,
    /// The frame of every member of the span.
    frame: u32,
}

/// Where the trail and the tables of a first-order search stand.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Mark {
    /// The spans.
    spans: usize,
    /// The frames.
    frames: usize,
    /// The variables.
    vars: usize,
    /// The eigenvariables made.
    eigens: u32,
    /// The trail.
    trail: usize,
    /// The eigenvariables whose `∀` node is not built.
    introduced: usize,
}

/// What a unification found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Unified {
    /// The terms unify, with the bindings made.
    Yes,
    /// They do not, or the bindings make an excluded answer.
    No,
    /// A binding would pass the witness depth: the branch is cut.
    Cut,
}

/// A node of a recorded answer's pattern.
#[derive(Clone, Copy, Debug)]
enum Pat {
    /// A metavariable of the answer, numbered within it.
    Var(u32),
    /// An eigenvariable, by number.
    Eigen(u32),
    /// A closed term of the sequent's arena.
    Closed(u32),
    /// A symbol applied to the patterns `kids[start..start + len]`.
    App(Symbol, u32, u32),
}

/// A recorded answer: per shared metavariable the pattern of its term,
/// and per metavariable of the patterns its level.
#[derive(Clone, Debug, Default)]
struct Pattern {
    /// The nodes.
    nodes: Vec<Pat>,
    /// The children of the applications.
    kids: Vec<u32>,
    /// Per shared metavariable, its term's node.
    roots: Vec<u32>,
    /// Per metavariable of the patterns, its level when recorded.
    levels: Vec<u32>,
}

/// The answers of one rule's first premise.
#[derive(Clone, Debug)]
struct Answers {
    /// The metavariables the second premise shares, by index.
    metas: Vec<u32>,
    /// The answers under which the second premise failed.
    patterns: Vec<Pattern>,
    /// Whether the first premise is being searched. The patterns are excluded
    /// only then.
    first: bool,
}

/// The zone whose members are instances of occurrences under binders:
/// the spans, frames and variables, the trail, the answers excluded, the
/// first-order bounds, and the tables read off the sequent's terms.
pub(super) struct Framed {
    /// The forest's length: the first id of a span.
    occurrences: u32,
    /// The sequent's terms: the first id of a variable.
    base: u32,
    /// The spans, one per instance held. The forest's occurrences are the
    /// members below them.
    spans: Vec<Span>,
    /// Per frame, its parent ([`NONE`] for none) and its term.
    frames: Vec<(u32, TRef)>,
    /// The variables.
    vars: Vec<Var>,
    /// How many eigenvariables were made: the next one's number.
    eigens: u32,
    /// The bindings and lowerings, to be undone.
    trail: Vec<Undone>,
    /// The eigenvariables of the `∀` steps whose nodes are not built.
    introduced: Vec<Eigen>,
    /// The answers of the rules open on the branch, the innermost last.
    answers: Vec<Answers>,
    /// The choices made while a rule may ask for another answer, and the
    /// replay that resumes its first premise.
    choices: Choices,
    /// The deepest term a metavariable may be bound to.
    witness_depth: u32,
    /// The most instances of binders the search may hold at once.
    instances: u32,
    /// The bound that cut a branch of the level, if one did.
    cut: Option<Reason>,
    /// Per term of the sequent's arena, whether no bound variable lies
    /// below it. This table and the two below never change once the zone
    /// is made: the forks of a pool share them.
    closed: Arc<[bool]>,
    /// Per term of the sequent's arena, its depth: a constant's is 0.
    depth: Arc<[u32]>,
    /// Per predicate and sign, the stamp of the marking that last found
    /// a literal of it among the members.
    marks: Vec<u64>,
    /// The stamp of the last marking.
    stamp: u64,
    /// The terms the walks visited since the engine last took them. That is
    /// the work a unification, a pattern or a match does, which grows with
    /// the terms' size and not with the search's steps. Atomic, so that the
    /// walks that only read the zone count too; one thread uses it.
    steps: AtomicU64,
    /// The unifications of two literals tried.
    unifications: u64,
    /// The metavariables bound.
    bindings: u64,
    /// The bytes of the tables and the walks' buffers the engine was last
    /// told the zone holds.
    charged: usize,
    /// Scratch: pairs of terms to unify or compare.
    pairs: Vec<(TRef, TRef)>,
    /// Scratch: the pairs met in one walk.
    met: HashSet<(TRef, TRef)>,
    /// Scratch: terms to walk, and whether their children were.
    walk: Vec<(TRef, bool)>,
    /// Scratch: the depth of every term a binding's walk met.
    depths: HashMap<TRef, u32>,
    /// Scratch: the buffers the keys of stable sequents are written with.
    key: KeyScratch,
    /// The ground terms the keys name, by their heads and last arguments:
    /// each term met gets one id for the whole search, so equal ids are
    /// equal terms however they were reached.
    ground: Grounds,
    /// Per term of the sequent's arena, its id in `ground` if the term is
    /// closed and the id fits a word of a key, else [`NONE`].
    arena: Arc<[u32]>,
    /// Per variable, what it was found to stand for, on the trail.
    named: Vec<Named>,
    /// Scratch: the steps of a naming walk.
    naming: Vec<Naming>,
    /// Scratch: the names it found.
    names: Vec<Name>,
}

impl Framed {
    /// Counts a term visited by a walk.
    fn step(&self) {
        self.steps.fetch_add(1, Ordering::Relaxed);
    }

    /// The occurrence and frame of a member.
    fn instance(&self, m: FMember) -> (OccId, u32) {
        if m.0 < self.occurrences {
            return (OccId::new(m.0), NONE);
        }
        let span = self.spans[self.spans.partition_point(|s| s.base <= m.0) - 1];
        (OccId::new(span.first + (m.0 - span.base)), span.frame)
    }

    /// The term the bound variable `i` of `frame` stands for.
    fn frame_term(&self, mut frame: u32, i: u32) -> TRef {
        for _ in 0..i {
            frame = self.frames[frame as usize].0;
        }
        self.frames[frame as usize].1
    }

    /// Returns the term `t` stands for: its bound variables read in its
    /// frame, and its metavariables' bindings followed. A closed term comes
    /// back without its frame.
    fn deref(&self, fo: &FirstOrder, mut t: TRef) -> TRef {
        loop {
            if t.id < self.base {
                if let FoTerm::Bound(i) = fo.terms[t.id as usize] {
                    t = self.frame_term(t.frame, i);
                    continue;
                }
                if self.closed[t.id as usize] {
                    t.frame = NONE;
                }
                return t;
            }
            match self.vars[(t.id - self.base) as usize] {
                Var::Meta {
                    binding: Some(b), ..
                } => t = b,
                _ => return t,
            }
        }
    }

    /// The variable a term is, by index, if it is one.
    fn var(&self, t: TRef) -> Option<u32> {
        t.id.checked_sub(self.base)
    }

    /// The level of a variable.
    fn level(&self, v: u32) -> u32 {
        match self.vars[v as usize] {
            Var::Meta { level, .. } => level,
            Var::Eigen(number) => number + 1,
        }
    }

    /// The symbol and the arguments of a term of the arena that is an
    /// application.
    fn app(fo: &FirstOrder, t: TRef) -> (Symbol, &[FoTermId]) {
        match fo.terms[t.id as usize] {
            FoTerm::App(symbol, args) => (symbol, fo.list(args)),
            term => unreachable!("{term:?} is no application after a deref"),
        }
    }

    /// Adds a variable and returns its term.
    fn push_var(&mut self, var: Var) -> Result<TRef, Reason> {
        let id = u32::try_from(self.vars.len())
            .ok()
            .and_then(|v| self.base.checked_add(v))
            .filter(|&id| id < u32::MAX)
            .ok_or(Reason::IndexLimit)?;
        self.vars.push(var);
        self.named.push(Named::Unknown);
        Ok(TRef { id, frame: NONE })
    }

    /// Opens the body of the binder `binder` with `term`: a span of the
    /// body's occurrences under the binder's frame extended by `term`.
    fn open(&mut self, forest: &Forest, binder: FMember, term: TRef) -> Result<FMember, Reason> {
        let (o, frame) = self.instance(binder);
        let frames = u32::try_from(self.frames.len())
            .ok()
            .filter(|&f| f < NONE)
            .ok_or(Reason::IndexLimit)?;
        let base = self.spans.last().map_or(self.occurrences, |s| s.end);
        // A binder's body is one occurrence at the least.
        let end = base
            .checked_add(forest.size(o) - 1)
            .filter(|&end| end <= member_limit())
            .ok_or(Reason::IndexLimit)?;
        self.frames.push((frame, term));
        self.spans.push(Span {
            base,
            end,
            first: o.get() + 1,
            frame: frames,
        });
        Ok(FMember(base))
    }

    /// Returns whether the search holds as many instances as it may. Then the
    /// next binder step cuts its branch.
    fn full(&mut self) -> bool {
        if self.spans.len() >= self.instances as usize {
            self.cut = Some(Reason::Instances {
                instances: self.instances,
            });
            return true;
        }
        false
    }

    /// Binds the metavariable `x` to `t`, and lowers the metavariables of `t`
    /// above `x`'s level to it. Refuses where `x` occurs in `t` (the occurs
    /// check), or where `t` holds an eigenvariable at or past `x`'s level.
    /// Cuts the branch where `t` is deeper than the witness depth.
    fn bind(&mut self, fo: &FirstOrder, x: u32, t: TRef) -> Unified {
        let level = self.level(x);
        self.depths.clear();
        self.walk.clear();
        self.walk.push((t, false));
        while let Some((u, expanded)) = self.walk.pop() {
            self.step();
            let u = self.deref(fo, u);
            if !expanded && self.depths.contains_key(&u) {
                continue;
            }
            let depth = if let Some(v) = self.var(u) {
                match self.vars[v as usize] {
                    Var::Meta { level: l, .. } => {
                        if v == x {
                            return Unified::No;
                        }
                        if l > level {
                            self.trail.push(Undone::Lowered(v, l));
                            self.vars[v as usize] = Var::Meta {
                                binding: None,
                                level,
                            };
                        }
                    }
                    Var::Eigen(number) => {
                        if number + 1 > level {
                            return Unified::No;
                        }
                    }
                }
                0
            } else if self.closed[u.id as usize] {
                self.depth[u.id as usize]
            } else {
                let (_, args) = Self::app(fo, u);
                if !expanded {
                    self.walk.push((u, true));
                    self.walk.extend(args.iter().map(|&a| {
                        (
                            TRef {
                                id: a.get(),
                                frame: u.frame,
                            },
                            false,
                        )
                    }));
                    continue;
                }
                let deepest = args
                    .iter()
                    .map(|&a| {
                        let child = self.deref(
                            fo,
                            TRef {
                                id: a.get(),
                                frame: u.frame,
                            },
                        );
                        self.depths[&child]
                    })
                    .max();
                deepest.map_or(0, |d| d + 1)
            };
            if depth > self.witness_depth {
                self.cut = Some(Reason::WitnessDepth {
                    depth: self.witness_depth,
                });
                return Unified::Cut;
            }
            self.depths.insert(u, depth);
        }
        self.trail.push(Undone::Bound(x));
        self.bindings += 1;
        self.vars[x as usize] = Var::Meta {
            binding: Some(t),
            level,
        };
        Unified::Yes
    }

    /// Unifies the pairs of terms on `pairs`.
    fn unify_pairs(&mut self, fo: &FirstOrder) -> Unified {
        self.met.clear();
        while let Some((a, b)) = self.pairs.pop() {
            self.step();
            let (a, b) = (self.deref(fo, a), self.deref(fo, b));
            if a == b || !self.met.insert((a, b)) {
                continue;
            }
            let found = match (self.var(a), self.var(b)) {
                (Some(x), Some(y)) => match (self.vars[x as usize], self.vars[y as usize]) {
                    // The younger of two metavariables is bound to the
                    // older, which lowers nothing.
                    (Var::Meta { level: l, .. }, Var::Meta { level: m, .. }) => {
                        if l >= m {
                            self.bind(fo, x, b)
                        } else {
                            self.bind(fo, y, a)
                        }
                    }
                    (Var::Meta { .. }, Var::Eigen(_)) => self.bind(fo, x, b),
                    (Var::Eigen(_), Var::Meta { .. }) => self.bind(fo, y, a),
                    (Var::Eigen(_), Var::Eigen(_)) => Unified::No,
                },
                (Some(x), None) => match self.vars[x as usize] {
                    Var::Meta { .. } => self.bind(fo, x, b),
                    Var::Eigen(_) => Unified::No,
                },
                (None, Some(y)) => match self.vars[y as usize] {
                    Var::Meta { .. } => self.bind(fo, y, a),
                    Var::Eigen(_) => Unified::No,
                },
                (None, None) => {
                    let ((s, xs), (r, ys)) = (Self::app(fo, a), Self::app(fo, b));
                    if s != r {
                        Unified::No
                    } else {
                        self.pairs.extend(xs.iter().zip(ys).map(|(&x, &y)| {
                            (
                                TRef {
                                    id: x.get(),
                                    frame: a.frame,
                                },
                                TRef {
                                    id: y.get(),
                                    frame: b.frame,
                                },
                            )
                        }));
                        Unified::Yes
                    }
                }
            };
            if found != Unified::Yes {
                self.pairs.clear();
                return found;
            }
        }
        Unified::Yes
    }

    /// Whether two terms are equal, as they stand.
    fn equal(&self, fo: &FirstOrder, a: TRef, b: TRef) -> bool {
        let mut pairs = vec![(a, b)];
        let mut met = HashSet::default();
        while let Some((a, b)) = pairs.pop() {
            self.step();
            let (a, b) = (self.deref(fo, a), self.deref(fo, b));
            if a == b || !met.insert((a, b)) {
                continue;
            }
            if self.var(a).is_some() || self.var(b).is_some() {
                return false;
            }
            let ((s, xs), (r, ys)) = (Self::app(fo, a), Self::app(fo, b));
            if s != r {
                return false;
            }
            pairs.extend(xs.iter().zip(ys).map(|(&x, &y)| {
                (
                    TRef {
                        id: x.get(),
                        frame: a.frame,
                    },
                    TRef {
                        id: y.get(),
                        frame: b.frame,
                    },
                )
            }));
        }
        true
    }

    /// Whether every variable of `t` is below `level`, or at it for a
    /// metavariable. Such a term is what a metavariable of that level could be
    /// bound to, and be read as.
    fn fits(&self, fo: &FirstOrder, t: TRef, level: u32) -> bool {
        let mut walk = vec![t];
        let mut met = HashSet::default();
        while let Some(u) = walk.pop() {
            self.step();
            let u = self.deref(fo, u);
            if !met.insert(u) {
                continue;
            }
            if let Some(v) = self.var(u) {
                let fits = match self.vars[v as usize] {
                    Var::Meta { level: l, .. } => l <= level,
                    Var::Eigen(number) => number < level,
                };
                if !fits {
                    return false;
                }
            } else if !self.closed[u.id as usize] {
                walk.extend(Self::app(fo, u).1.iter().map(|&a| TRef {
                    id: a.get(),
                    frame: u.frame,
                }));
            }
        }
        true
    }

    /// Whether the terms of the metavariables `metas` are an instance of
    /// `pattern`: its metavariables replaced by terms that fit their
    /// levels, consistently.
    fn instance_of(&self, fo: &FirstOrder, metas: &[u32], pattern: &Pattern) -> bool {
        let mut theta: Vec<Option<TRef>> = vec![None; pattern.levels.len()];
        let mut walk: Vec<(u32, TRef)> = metas
            .iter()
            .zip(&pattern.roots)
            .map(|(&m, &root)| {
                (
                    root,
                    TRef {
                        id: self.base + m,
                        frame: NONE,
                    },
                )
            })
            .collect();
        let mut met = HashSet::default();
        while let Some((node, t)) = walk.pop() {
            self.step();
            let t = self.deref(fo, t);
            if !met.insert((node, t)) {
                continue;
            }
            let fits = match pattern.nodes[node as usize] {
                Pat::Var(k) => match theta[k as usize] {
                    None => {
                        theta[k as usize] = Some(t);
                        self.fits(fo, t, pattern.levels[k as usize])
                    }
                    Some(u) => self.equal(fo, u, t),
                },
                Pat::Eigen(number) => self
                    .var(t)
                    .is_some_and(|v| matches!(self.vars[v as usize], Var::Eigen(n) if n == number)),
                Pat::Closed(id) => self.equal(fo, TRef { id, frame: NONE }, t),
                Pat::App(symbol, start, len) => {
                    if self.var(t).is_some() {
                        false
                    } else {
                        let (s, args) = Self::app(fo, t);
                        let kids = &pattern.kids[start as usize..(start + len) as usize];
                        walk.extend(kids.iter().zip(args).map(|(&kid, &a)| {
                            (
                                kid,
                                TRef {
                                    id: a.get(),
                                    frame: t.frame,
                                },
                            )
                        }));
                        s == symbol
                    }
                }
            };
            if !fits {
                return false;
            }
        }
        true
    }

    /// Whether the bindings as they stand make no answer excluded while
    /// its first premise is searched.
    fn passes(&self, fo: &FirstOrder) -> bool {
        self.answers
            .iter()
            .filter(|answers| answers.first)
            .all(|answers| {
                answers
                    .patterns
                    .iter()
                    .all(|p| !self.instance_of(fo, &answers.metas, p))
            })
    }

    /// The pattern of the terms of `metas` as they stand.
    fn pattern(&self, fo: &FirstOrder, metas: &[u32]) -> Pattern {
        let mut pattern = Pattern::default();
        let mut nodes: HashMap<TRef, u32> = HashMap::default();
        let mut numbered: HashMap<u32, u32> = HashMap::default();
        for &m in metas {
            let root = TRef {
                id: self.base + m,
                frame: NONE,
            };
            let mut walk = vec![(root, false)];
            while let Some((u, expanded)) = walk.pop() {
                self.step();
                let u = self.deref(fo, u);
                if !expanded && nodes.contains_key(&u) {
                    continue;
                }
                let node = if let Some(v) = self.var(u) {
                    match self.vars[v as usize] {
                        Var::Meta { level, .. } => {
                            let k = *numbered.entry(v).or_insert_with(|| {
                                pattern.levels.push(level);
                                (pattern.levels.len() - 1) as u32
                            });
                            Pat::Var(k)
                        }
                        Var::Eigen(number) => Pat::Eigen(number),
                    }
                } else if self.closed[u.id as usize] {
                    Pat::Closed(u.id)
                } else {
                    let (symbol, args) = Self::app(fo, u);
                    let child = |a: &FoTermId| TRef {
                        id: a.get(),
                        frame: u.frame,
                    };
                    if !expanded {
                        walk.push((u, true));
                        walk.extend(args.iter().map(|a| (child(a), false)));
                        continue;
                    }
                    let start = pattern.kids.len() as u32;
                    for a in args {
                        let kid = nodes[&self.deref(fo, child(a))];
                        pattern.kids.push(kid);
                    }
                    Pat::App(symbol, start, args.len() as u32)
                };
                pattern.nodes.push(node);
                nodes.insert(u, (pattern.nodes.len() - 1) as u32);
            }
            pattern.roots.push(nodes[&self.deref(fo, root)]);
        }
        pattern
    }

    /// The marks' index of a literal's predicate, or of its atom where
    /// the sequent's atoms have no predicates, and sign.
    fn key(forest: &Forest, o: OccId, dual: bool) -> Option<usize> {
        let (atom, sign) = (forest.atom(o)?, forest.sign(o)?);
        let fo = &forest.sequent().fo;
        let index = match fo.atoms.get(atom.index()) {
            Some(&(symbol, _)) => symbol.index(),
            None => atom.index(),
        };
        Some(2 * index + (sign as usize ^ usize::from(dual)))
    }
}

/// A linear zone of a first-order search: its members in ascending
/// order, with repeats.
#[derive(Clone, Debug, Default)]
pub(super) struct Members(Vec<FMember>);

impl Linear<FMember> for Members {
    fn empty(_: &Forest) -> Self {
        Self(Vec::new())
    }

    fn insert(&mut self, m: FMember) {
        let at = self.0.partition_point(|&x| x <= m);
        self.0.insert(at, m);
    }

    fn remove(&mut self, m: FMember) {
        let at = self.0.binary_search(&m).expect("a member");
        self.0.remove(at);
    }

    fn contains(&self, m: FMember) -> bool {
        self.0.binary_search(&m).is_ok()
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn copy_from(&mut self, other: &Self) {
        self.0.clone_from(&other.0);
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = FMember> + '_ {
        self.0.iter().copied()
    }

    fn list_into(&self, list: &mut Vec<FMember>) {
        list.extend_from_slice(&self.0);
    }

    fn ground(&self) -> &Context {
        unreachable!("a first-order search keys no sequent by occurrence")
    }
}

/// An unrestricted zone of a first-order search: its members in
/// ascending order, each once, and the empty set of occurrences its
/// interning reads.
#[derive(Clone, Debug)]
pub(super) struct Set(Vec<FMember>, OccSet);

impl Unrestricted<FMember> for Set {
    fn empty(_: &Forest) -> Self {
        Self(Vec::new(), OccSet::empty(0))
    }

    fn insert(&mut self, m: FMember) {
        if let Err(at) = self.0.binary_search(&m) {
            self.0.insert(at, m);
        }
    }

    fn contains(&self, m: FMember) -> bool {
        self.0.binary_search(&m).is_ok()
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn copy_from(&mut self, other: &Self) {
        self.0.clone_from(&other.0);
    }

    fn iter(&self) -> impl Iterator<Item = FMember> + '_ {
        self.0.iter().copied()
    }

    /// The empty set: a first-order search keys its sequents by instance,
    /// and its table of zones keeps none, so it gives every zone a new id
    /// without reading it.
    fn ground(&self) -> &OccSet {
        &self.1
    }
}
