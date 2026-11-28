// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The first-order search's zone: the engine's questions answered over
//! members with frames.

#[cfg(feature = "parallel")]
use super::SharedGrounds;
use super::closing::Closing;
use super::keys::KeyScratch;
use super::{
    Answers, FMember, Framed, Grounds, Head, Mark, Members, NONE, Name, Named, Pat, Pattern, Set,
    TRef, Undone, Unified, Var,
};
use crate::hash::{HashMap, HashSet};
use crate::linear::Instances;
use crate::linear::occurrences::{Forest, Member, OccId};
use crate::linear::proofs::Node;
use crate::linear::search::focus::choices::Choices;
use crate::linear::search::focus::classes::Classes;
use crate::linear::search::focus::scratch::{Pooled, Pools};
use crate::linear::search::focus::zone::{Cursors, Linear, Unrestricted, Zone};
use crate::linear::search::{Options, Reason};
use crate::linear::sequents::{Eigen, FoTerm, FoTermId};
use crate::memory::{Charged, bytes_of};
use std::cell::RefCell;
#[cfg(feature = "parallel")]
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

impl Zone for Framed {
    const FIRST_ORDER: bool = true;

    type Member = FMember;
    type Linear = Members;
    type Unrestricted = Set;
    type Duals = usize;

    fn new(forest: &Forest, options: &Options) -> Self {
        let fo = &forest.sequent().fo;
        let mut closed = vec![true; fo.terms.len()];
        let mut depth = vec![0; fo.terms.len()];
        // Every term names only terms before it.
        for (i, term) in fo.terms.iter().enumerate() {
            (closed[i], depth[i]) = match *term {
                FoTerm::Bound(_) => (false, 0),
                FoTerm::App(_, args) => {
                    let args = fo.list(args);
                    (
                        args.iter().all(|a| closed[a.index()]),
                        args.iter().map(|a| depth[a.index()] + 1).max().unwrap_or(0),
                    )
                }
                FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                    unreachable!("a sequent's arena holds no eigenvariable or metavariable")
                }
            };
        }
        // A forest has fewer than 2³² occurrences and a sequent fewer
        // terms than its text or document.
        let occurrences = forest.len() as u32;
        let mut ground = Grounds::Own(HashMap::default());
        let mut arena = vec![NONE; fo.terms.len()];
        // Every term names only terms before it.
        let mut names = Vec::new();
        for (i, term) in fo.terms.iter().enumerate() {
            if let FoTerm::App(symbol, args) = *term
                && closed[i]
            {
                names.clear();
                names.extend(fo.list(args).iter().map(|a| match arena[a.index()] {
                    NONE => Name::Past,
                    id => Name::Ground(id),
                }));
                if let Name::Ground(id) = ground.apply(symbol, &names) {
                    arena[i] = id;
                }
            }
        }
        Self {
            occurrences,
            base: fo.terms.len() as u32,
            spans: Vec::new(),
            frames: Vec::new(),
            vars: Vec::new(),
            eigens: 0,
            trail: Vec::new(),
            introduced: Vec::new(),
            answers: Vec::new(),
            choices: Choices::default(),
            witness_depth: options.witness_depth,
            instances: options.instances,
            cut: None,
            closed: closed.into(),
            depth: depth.into(),
            marks: Vec::new(),
            stamp: 0,
            steps: AtomicU64::new(0),
            unifications: 0,
            bindings: 0,
            charged: 0,
            pairs: Vec::new(),
            met: HashSet::default(),
            walk: Vec::new(),
            depths: HashMap::default(),
            key: KeyScratch::default(),
            ground,
            arena: arena.into(),
            named: Vec::new(),
            naming: Vec::new(),
            names: Vec::new(),
        }
    }

    /// Returns a copy of the spans, frames, variables, bindings and trail,
    /// with the sequent's tables and the table of ground terms the pool
    /// shares ([`Zone::share`]), so that the worker's keys name terms as every
    /// engine's do. The counters are at zero and the scratch is empty. No
    /// rule is open that asks for another answer ([`Zone::divides`]), so
    /// there is no answer and no choice to log.
    #[cfg(feature = "parallel")]
    fn fork(&self) -> Self {
        Self {
            occurrences: self.occurrences,
            base: self.base,
            spans: self.spans.clone(),
            frames: self.frames.clone(),
            vars: self.vars.clone(),
            eigens: self.eigens,
            trail: self.trail.clone(),
            introduced: self.introduced.clone(),
            answers: Vec::new(),
            choices: Choices::default(),
            witness_depth: self.witness_depth,
            instances: self.instances,
            cut: None,
            closed: Arc::clone(&self.closed),
            depth: Arc::clone(&self.depth),
            marks: Vec::new(),
            stamp: 0,
            steps: AtomicU64::new(0),
            unifications: 0,
            bindings: 0,
            charged: 0,
            pairs: Vec::new(),
            met: HashSet::default(),
            walk: Vec::new(),
            depths: HashMap::default(),
            key: KeyScratch::default(),
            ground: self.ground.clone(),
            arena: Arc::clone(&self.arena),
            named: self.named.clone(),
            naming: Vec::new(),
            names: Vec::new(),
        }
    }

    #[cfg(feature = "parallel")]
    fn share(&mut self) {
        if let Grounds::Own(table) = &mut self.ground {
            let table = std::mem::take(table);
            self.ground = Grounds::Shared(Arc::new(SharedGrounds {
                slots: std::sync::atomic::AtomicUsize::new(table.capacity()),
                table: std::sync::Mutex::new(table),
            }));
        }
    }

    /// While a rule may ask a premise for another answer, its choices are
    /// logged and replayed in order, and a failure may rest on an excluded
    /// answer. So only a choice outside every such rule is divided.
    #[cfg(feature = "parallel")]
    fn divides(&self) -> bool {
        self.answers.is_empty()
    }

    #[cfg(feature = "parallel")]
    fn adopt(&mut self, worker: Self) {
        let (unifications, bindings, cut, charged) =
            (self.unifications, self.bindings, self.cut, self.charged);
        let steps = *self.steps.get_mut();
        *self = Self {
            unifications,
            bindings,
            cut,
            charged,
            steps: AtomicU64::new(steps),
            ..worker
        };
    }

    #[cfg(feature = "parallel")]
    fn cut_by(&mut self, reason: Reason) {
        self.cut.get_or_insert(reason);
    }

    fn root(&self, o: OccId) -> FMember {
        FMember(o.get())
    }

    fn occurrence(&self, m: FMember) -> OccId {
        self.instance(m).0
    }

    fn child(&self, m: FMember, child: OccId) -> FMember {
        FMember(m.0 + (child.get() - self.occurrence(m).get()))
    }

    /// Finds the dual of a literal in `within`: a member with the literal's
    /// atom and the other sign. The literal's predicate has only closed
    /// atoms, so its dual has the same atom.
    fn dual_in(&self, forest: &Forest, literal: FMember, within: &Set) -> Option<FMember> {
        let o = self.occurrence(literal);
        let (atom, sign) = (forest.atom(o)?, forest.sign(o)?);
        within.iter().find(|&d| {
            let d = self.occurrence(d);
            forest.atom(d) == Some(atom) && forest.sign(d) == Some(!sign)
        })
    }

    /// As [`Self::dual_in`], over the members of `rest` in their order.
    fn dual_from(
        &self,
        forest: &Forest,
        literal: FMember,
        rest: &Members,
        _cursors: &mut Cursors,
    ) -> Option<FMember> {
        let o = self.occurrence(literal);
        let (atom, sign) = (forest.atom(o)?, forest.sign(o)?);
        rest.iter().find(|&d| {
            let d = self.occurrence(d);
            forest.atom(d) == Some(atom) && forest.sign(d) == Some(!sign)
        })
    }

    /// Marks the predicates and signs of the members' literals.
    fn mark_literals(&mut self, forest: &Forest, members: &[FMember], scratch: &mut Charged) {
        if self.marks.is_empty() {
            let fo = &forest.sequent().fo;
            let keys = 2 * fo.symbols.len().max(forest.sequent().atom_names().len());
            scratch.charge(keys * size_of::<u64>());
            self.marks = vec![0; keys];
        }
        self.stamp += 1;
        for &m in members {
            if let Some(key) = Self::key(forest, self.occurrence(m), false) {
                self.marks[key] = self.stamp;
            }
        }
    }

    fn duals(&self, _: &Forest, _: FMember) -> Option<usize> {
        None
    }

    fn marked(&self, _: usize) -> Option<FMember> {
        None
    }

    fn first_lookup(&mut self, _: usize) -> bool {
        false
    }

    /// Whether a literal below the formula has a predicate whose dual
    /// literal the last marking found.
    fn meets(&self, forest: &Forest, formula: FMember) -> bool {
        forest
            .subtree(self.occurrence(formula))
            .filter_map(|l| Self::key(forest, l, true))
            .any(|key| self.marks[key] == self.stamp)
    }

    /// Returns a member's class: an occurrence's class, or, for a member in a
    /// span, the member itself.
    fn class(&self, classes: &Classes, m: FMember) -> FMember {
        if m.0 < self.occurrences {
            FMember(classes.of(OccId::new(m.0)).get())
        } else {
            m
        }
    }

    fn same(&self, classes: &Classes, x: FMember, y: FMember) -> bool {
        if x.0 < self.occurrences && y.0 < self.occurrences {
            classes.same(OccId::new(x.0), OccId::new(y.0))
        } else {
            x == y
        }
    }

    /// A first-order search keys its sequents by instance
    /// ([`Zone::keys`]), not by occurrence.
    fn memoizes(&self, _: &Set, _: &Members) -> bool {
        false
    }

    /// A pool of its own, since the members are not words.
    fn word_lists(pools: &mut Pools<Self>) -> &mut Vec<Pooled<OccId>> {
        &mut pools.words
    }

    fn listed(_: &[OccId]) -> Option<&[FMember]> {
        None
    }

    fn mark(&self) -> Mark {
        Mark {
            spans: self.spans.len(),
            frames: self.frames.len(),
            vars: self.vars.len(),
            eigens: self.eigens,
            trail: self.trail.len(),
            introduced: self.introduced.len(),
        }
    }

    fn undo(&mut self, mark: Mark) {
        while self.trail.len() > mark.trail {
            match self.trail.pop().expect("longer than the mark") {
                Undone::Bound(v) => {
                    if let Var::Meta { binding, .. } = &mut self.vars[v as usize] {
                        *binding = None;
                    }
                }
                Undone::Lowered(v, old) => {
                    if let Var::Meta { level, .. } = &mut self.vars[v as usize] {
                        *level = old;
                    }
                }
                Undone::Named(v) => self.named[v as usize] = Named::Unknown,
            }
        }
        self.spans.truncate(mark.spans);
        self.frames.truncate(mark.frames);
        self.vars.truncate(mark.vars);
        self.named.truncate(mark.vars);
        self.eigens = mark.eigens;
        // The nodes built since took theirs already.
        self.introduced.truncate(mark.introduced);
    }

    fn begin_level(&mut self) {
        self.cut = None;
    }

    fn cut(&self) -> Option<Reason> {
        self.cut
    }

    fn forall(&mut self, forest: &Forest, binder: FMember) -> Result<Option<FMember>, Reason> {
        if self.full() {
            return Ok(None);
        }
        let number = self.eigens;
        let term = self.push_var(Var::Eigen(number))?;
        self.eigens += 1;
        self.introduced.push(Eigen::new(number));
        self.open(forest, binder, term).map(Some)
    }

    fn introduced(&mut self) -> Eigen {
        self.introduced
            .pop()
            .expect("an eigenvariable per `∀` step")
    }

    fn exists(
        &mut self,
        forest: &Forest,
        binder: FMember,
    ) -> Result<Option<(FMember, FoTermId)>, Reason> {
        if self.full() {
            return Ok(None);
        }
        let term = self.push_var(Var::Meta {
            binding: None,
            level: self.eigens,
        })?;
        let body = self.open(forest, binder, term)?;
        Ok(Some((body, FoTermId::new(term.id))))
    }

    fn opposite(&self, forest: &Forest, p: FMember, q: FMember) -> bool {
        let (p, q) = (self.occurrence(p), self.occurrence(q));
        let (Some(a), Some(s), Some(b), Some(t)) = (
            forest.atom(p),
            forest.sign(p),
            forest.atom(q),
            forest.sign(q),
        ) else {
            return false;
        };
        if s == t {
            return false;
        }
        let fo = &forest.sequent().fo;
        match (fo.atoms.get(a.index()), fo.atoms.get(b.index())) {
            (Some(&(x, _)), Some(&(y, _))) => x == y,
            _ => a == b,
        }
    }

    fn unify(&mut self, forest: &Forest, p: FMember, q: FMember) -> Unified {
        self.unifications += 1;
        let fo = &forest.sequent().fo;
        let ((o, f), (r, g)) = (self.instance(p), self.instance(q));
        let (Some(a), Some(b)) = (forest.atom(o), forest.atom(r)) else {
            return Unified::No;
        };
        if let (Some(&(_, xs)), Some(&(_, ys))) = (fo.atoms.get(a.index()), fo.atoms.get(b.index()))
        {
            self.pairs.clear();
            self.pairs
                .extend(fo.list(xs).iter().zip(fo.list(ys)).map(|(&x, &y)| {
                    (
                        TRef {
                            id: x.get(),
                            frame: f,
                        },
                        TRef {
                            id: y.get(),
                            frame: g,
                        },
                    )
                }));
            let found = self.unify_pairs(fo);
            if found != Unified::Yes {
                return found;
            }
        } else if a != b {
            return Unified::No;
        }
        // A replay takes the alternatives the first search took, which
        // passed the answers excluded then.
        if self.choices.replaying() || self.passes(fo) {
            Unified::Yes
        } else {
            Unified::No
        }
    }

    fn open_answers(
        &mut self,
        forest: &Forest,
        members: &mut dyn Iterator<Item = FMember>,
    ) -> bool {
        let fo = &forest.sequent().fo;
        let mut metas = Vec::new();
        let mut frames = HashSet::default();
        let mut met = HashSet::default();
        let mut walk = Vec::new();
        for m in members {
            let mut frame = self.instance(m).1;
            while frame != NONE && frames.insert(frame) {
                let (parent, term) = self.frames[frame as usize];
                walk.push(term);
                frame = parent;
            }
        }
        while let Some(t) = walk.pop() {
            self.step();
            let t = self.deref(fo, t);
            if !met.insert(t) {
                continue;
            }
            match self.var(t) {
                Some(v) => {
                    if matches!(self.vars[v as usize], Var::Meta { .. }) {
                        metas.push(v);
                    }
                }
                None if !self.closed[t.id as usize] => {
                    walk.extend(Self::app(fo, t).1.iter().map(|&a| TRef {
                        id: a.get(),
                        frame: t.frame,
                    }));
                }
                None => {}
            }
        }
        if metas.is_empty() {
            return false;
        }
        metas.sort_unstable();
        self.answers.push(Answers {
            metas,
            patterns: Vec::new(),
            first: true,
        });
        true
    }

    fn searching_first(&mut self, first: bool) {
        self.answers.last_mut().expect("answers open").first = first;
    }

    fn record(&mut self, forest: &Forest) -> usize {
        let fo = &forest.sequent().fo;
        let answers = self.answers.last().expect("answers open");
        let pattern = self.pattern(fo, &answers.metas);
        let bytes = size_of::<Pattern>()
            + pattern.nodes.len() * size_of::<Pat>()
            + (pattern.kids.len() + pattern.roots.len() + pattern.levels.len()) * size_of::<u32>();
        self.answers
            .last_mut()
            .expect("answers open")
            .patterns
            .push(pattern);
        bytes
    }

    fn exhausted(&mut self, forest: &Forest) -> bool {
        let fo = &forest.sequent().fo;
        let answers = self.answers.last().expect("answers open");
        answers
            .patterns
            .iter()
            .any(|p| self.instance_of(fo, &answers.metas, p))
    }

    fn close_answers(&mut self) {
        self.answers.pop();
    }

    fn keys(
        &mut self,
        forest: &Forest,
        theta: &Set,
        gamma: &Members,
        own: &mut Vec<OccId>,
        canonical: &mut Vec<OccId>,
    ) -> bool {
        self.write_keys(forest, theta, gamma, own, canonical)
    }

    /// Whether an answer is excluded while its first premise is searched.
    fn excluding(&self) -> bool {
        self.answers
            .iter()
            .any(|answers| answers.first && !answers.patterns.is_empty())
    }

    fn take_steps(&mut self) -> u64 {
        std::mem::take(self.steps.get_mut())
    }

    fn counts(&self) -> (u64, u64) {
        (self.unifications, self.bindings)
    }

    /// Returns what the zone was charged when last asked, and what it holds
    /// now: the tables and the walks' buffers by their capacities, a hash
    /// table's as its slots with a control byte each.
    fn held(&mut self) -> (usize, usize) {
        let entries = |slots: usize, size: usize| slots * (size + 1);
        let held = bytes_of(&self.spans)
            + bytes_of(&self.frames)
            + bytes_of(&self.vars)
            + bytes_of(&self.trail)
            + bytes_of(&self.introduced)
            + bytes_of(&self.pairs)
            + bytes_of(&self.walk)
            + self.key.bytes()
            + entries(self.met.capacity(), size_of::<(TRef, TRef)>())
            + entries(self.depths.capacity(), size_of::<(TRef, u32)>())
            + entries(self.ground.capacity(), size_of::<((Head, u32), u32)>())
            + bytes_of(&self.named)
            + bytes_of(&self.naming)
            + bytes_of(&self.names)
            + self.choices.bytes();
        (std::mem::replace(&mut self.charged, held), held)
    }

    fn close(&self, forest: &Forest, nodes: &mut [Node]) -> Result<Instances, Reason> {
        let closing = RefCell::new(Closing {
            framed: self,
            fo: &forest.sequent().fo,
            table: Instances::new(forest),
            terms: HashMap::default(),
            frames: HashMap::default(),
            members: HashMap::default(),
            fresh: HashMap::default(),
        });
        for node in nodes.iter_mut() {
            let renamed = node.renamed(|m| -> Result<Member, Reason> {
                Ok(closing.borrow_mut().member(FMember(m.get())))
            })?;
            *node = match renamed {
                Node::Exists(m, w, p) => Node::Exists(
                    m,
                    closing.borrow_mut().term(TRef {
                        id: w.get(),
                        frame: NONE,
                    }),
                    p,
                ),
                node => node,
            };
        }
        Ok(closing.into_inner().table)
    }

    fn choices(&mut self) -> &mut Choices {
        &mut self.choices
    }
}
