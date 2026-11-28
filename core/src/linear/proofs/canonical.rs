// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The check of a proof's instance table against its forest and nodes,
//! and the table in canonical form, with the nodes renumbered into it.

use super::Node;
use crate::Error;
use crate::hash::HashMap;
use crate::limits::Space;
use crate::linear::occurrences::instances::{Numbering, Terms, binders};
use crate::linear::occurrences::{Forest, FrameId, Instances, Member, OccId};
use crate::linear::sequents::{Args, Eigen, FoTerm, FoTermId};

/// Returns the error of a malformed instance table.
fn malformed(message: String) -> Error {
    Error::InstanceTable { message }
}

/// Checks `table` against `forest` and the proof's `nodes`, whose
/// indices are already within bounds. Returns the table in canonical
/// form, with the nodes' members and witnesses renumbered into it.
///
/// In canonical form:
///
/// - equal terms, lists, frames and members are one entry each;
/// - the sequent's own terms keep their ids;
/// - every member the checker derives from a member a node names has a
///   number: the subformulas of a named member under its frame, the body
///   of a `∀` or `∃` step under its frame with the eigenvariable or
///   witness pushed, and the dual of a cut's member. So every member a
///   check or a derivation of the proof uses is in the table.
///
/// # Errors
///
/// [`Error::InstanceTable`] for:
///
/// - a table made for another forest;
/// - an index out of range, or an entry that refers to a later one;
/// - a table term that is a bound variable or a metavariable;
/// - a symbol applied to the wrong number of arguments, or a predicate
///   applied as a function;
/// - a frame term with a loose bound variable;
/// - a member whose frame does not have one term for each binder above
///   its occurrence;
/// - a witness out of range or with a loose bound variable.
///
/// [`Error::IndexOutOfBounds`] for a member past the table, and
/// [`Refusal::Index`](crate::Refusal::Index) for a table whose ids would
/// not fit a `u32`.
pub(crate) fn canonical(
    forest: &Forest,
    table: &Instances,
    nodes: &mut [Node],
) -> Result<Instances, Error> {
    let fo = &forest.sequent().fo;
    let made_for = Instances::new(forest);
    if (table.occurrences, table.base_terms, table.base_lists)
        != (
            made_for.occurrences,
            made_for.base_terms,
            made_for.base_lists,
        )
    {
        return Err(malformed("the table was made for another sequent".into()));
    }
    // Every id of the table made must fit a `u32`, counting the members,
    // frames and terms that the derived members add: at most three
    // members, one frame and one term per node. Past that the table is
    // refused.
    let count = nodes.len() as u64;
    let most = u64::from(u32::MAX);
    let ids = [
        (
            Space::Member,
            u64::from(table.occurrences) + table.members.len() as u64 + 3 * count,
        ),
        (
            Space::Term,
            u64::from(table.base_terms) + table.terms.len() as u64 + count,
        ),
        (
            Space::Term,
            u64::from(table.base_lists) + table.lists.len() as u64 + count,
        ),
        (Space::Term, table.args.len() as u64),
        (Space::Term, table.frames.len() as u64 + count),
    ];
    if let Some(&(what, count)) = ids.iter().find(|&&(_, count)| count > most) {
        return Err(Error::Refused(crate::limits::Refusal::Index {
            what,
            count,
            most,
        }));
    }
    let terms = Terms { fo, table };
    let all_terms = terms.len();
    let all_lists = terms.lists();
    let mut predicate = vec![false; fo.symbols.len()];
    for &(p, _) in &fo.atoms {
        predicate[p.index()] = true;
    }
    for (&(start, len), n) in table.lists.iter().zip(0..) {
        if (start as usize).saturating_add(len as usize) > table.args.len() {
            return Err(malformed(format!("list {n} runs past the arguments")));
        }
    }
    for (i, &term) in table.terms.iter().enumerate() {
        let id = fo.terms.len() + i;
        match term {
            FoTerm::Bound(_) => {
                return Err(malformed(format!("term {id} is a bound variable")));
            }
            FoTerm::Meta(_) => {
                return Err(malformed(format!(
                    "term {id} is an open witness, which a proof cannot hold"
                )));
            }
            FoTerm::Eigen(_) => {}
            FoTerm::App(f, args) => {
                let Some(&(_, arity)) = fo.symbols.get(f.index()) else {
                    return Err(malformed(format!("term {id} applies no symbol")));
                };
                if predicate[f.index()] {
                    return Err(malformed(format!("term {id} applies a predicate")));
                }
                if args.index() >= all_lists {
                    return Err(malformed(format!("term {id} has no argument list")));
                }
                let list = terms.list(args);
                if list.len() != arity as usize {
                    return Err(malformed(format!(
                        "term {id} applies a symbol of {arity} arguments to {}",
                        list.len()
                    )));
                }
                if list.iter().any(|t| t.index() >= id) {
                    return Err(malformed(format!("term {id} refers to a later term")));
                }
            }
        }
    }
    let numbering = Numbering::new(terms);
    // A table term is never a bound variable itself, but its arguments may
    // be open terms of the sequent. So the frames' terms and the witnesses
    // are checked for loose bound variables through the numbering.
    let mut lengths: Vec<u32> = Vec::with_capacity(table.frames.len());
    for (n, &(parent, term)) in table.frames.iter().enumerate() {
        if term.index() >= all_terms || !numbering.is_closed(term) {
            return Err(malformed(format!("frame {n} adds no closed term")));
        }
        let length = match parent {
            None => 1,
            Some(p) if p.index() < n => lengths[p.index()] + 1,
            Some(_) => return Err(malformed(format!("frame {n} refers to a later frame"))),
        };
        lengths.push(length);
    }
    let depth = binders(forest);
    for (n, &(o, frame)) in table.members.iter().enumerate() {
        let member = u64::from(table.occurrences) + n as u64;
        if o.index() >= forest.len() || frame.index() >= table.frames.len() {
            return Err(malformed(format!("member {member} names no instance")));
        }
        if lengths[frame.index()] != depth[o.index()] {
            return Err(malformed(format!(
                "member {member} stands under {} binders with a frame of {} terms",
                depth[o.index()],
                lengths[frame.index()]
            )));
        }
    }
    for node in nodes.iter() {
        if let Node::Exists(_, w, _) = *node {
            if w.index() >= all_terms {
                return Err(malformed(format!(
                    "the witness {} is past the {all_terms} terms",
                    w.get()
                )));
            }
            if !numbering.is_closed(w) {
                return Err(malformed(format!(
                    "the witness {} holds a loose bound variable",
                    w.get()
                )));
            }
        }
    }
    Canonical::new(forest, table, numbering).finish(nodes)
}

/// The canonical table that [`canonical`] builds, while it builds it.
struct Canonical<'a> {
    /// The forest.
    forest: &'a Forest,
    /// The table read.
    table: &'a Instances,
    /// The numbers of the read table's terms.
    numbering: Numbering<'a>,
    /// The table made.
    out: Instances,
    /// Per term number, its id in the table made.
    term_at: HashMap<u64, FoTermId>,
    /// Per list number, its id in the table made.
    list_at: HashMap<u64, Args>,
    /// The frames of the table made, by their parent and term.
    frame_at: HashMap<(Option<FrameId>, FoTermId), FrameId>,
    /// Per frame of the table read, its frame in the table made, once
    /// made. So a chain of frames is made once, not once per member.
    made: Vec<Option<FrameId>>,
    /// The members of the table made, by their occurrence and frame.
    member_at: HashMap<(OccId, FrameId), Member>,
}

impl<'a> Canonical<'a> {
    /// Returns the empty table, with the sequent's terms and lists in
    /// place.
    fn new(forest: &'a Forest, table: &'a Instances, numbering: Numbering<'a>) -> Self {
        let fo = &forest.sequent().fo;
        let mut this = Self {
            forest,
            table,
            numbering,
            out: Instances::new(forest),
            term_at: HashMap::default(),
            list_at: HashMap::default(),
            frame_at: HashMap::default(),
            made: vec![None; table.frames.len()],
            member_at: HashMap::default(),
        };
        for id in 0..fo.terms.len() {
            let id = FoTermId::new(id as u32);
            this.term_at.entry(this.numbering.number(id)).or_insert(id);
        }
        for args in 0..fo.lists.len() {
            let args = Args::new(args as u32);
            let n = this.numbering.list_number(args);
            this.list_at.entry(n).or_insert(args);
        }
        this
    }

    /// Returns the id in the table made of the closed term `id` of the
    /// table read, adding it and its arguments where they are new. It
    /// walks the term on a stack of its own, arguments first, since a
    /// search's witness may be nested as deep as its bindings go.
    fn term(&mut self, id: FoTermId) -> FoTermId {
        let mut walk = vec![(id, false)];
        while let Some((t, expanded)) = walk.pop() {
            let n = self.numbering.number(t);
            if self.term_at.contains_key(&n) {
                continue;
            }
            // Not among the terms made so far, so a table term, whose
            // arguments come before it.
            let term = match self.numbering.terms().term(t) {
                FoTerm::App(f, args) => {
                    let ln = self.numbering.list_number(args);
                    let list = match self.list_at.get(&ln) {
                        Some(&list) => list,
                        None if !expanded => {
                            walk.push((t, true));
                            walk.extend(
                                self.numbering
                                    .terms()
                                    .list(args)
                                    .iter()
                                    .map(|&a| (a, false)),
                            );
                            continue;
                        }
                        None => {
                            let items: Vec<FoTermId> = self
                                .numbering
                                .terms()
                                .list(args)
                                .iter()
                                .map(|&a| self.term_at[&self.numbering.number(a)])
                                .collect();
                            let start = self.out.args.len() as u32;
                            self.out.args.extend_from_slice(&items);
                            self.out.lists.push((start, items.len() as u32));
                            let list =
                                Args::new(self.out.base_lists + self.out.lists.len() as u32 - 1);
                            self.list_at.insert(ln, list);
                            list
                        }
                    };
                    FoTerm::App(f, list)
                }
                other @ (FoTerm::Eigen(_) | FoTerm::Bound(_) | FoTerm::Meta(_)) => other,
            };
            let at = self.out.push_term(term);
            self.term_at.insert(n, at);
        }
        self.term_at[&self.numbering.number(id)]
    }

    /// Returns the id in the table made of the eigenvariable `a`'s term.
    fn eigen(&mut self, a: Eigen) -> FoTermId {
        let n = self.numbering.eigen(a);
        if let Some(&at) = self.term_at.get(&n) {
            return at;
        }
        let at = self.out.push_term(FoTerm::Eigen(a));
        self.term_at.insert(n, at);
        at
    }

    /// Returns the frame of the table made that pushes `term`, an id of
    /// the table made, onto `parent`.
    fn push(&mut self, parent: Option<FrameId>, term: FoTermId) -> FrameId {
        *self.frame_at.entry((parent, term)).or_insert_with(|| {
            self.out.frames.push((parent, term));
            // Fits a `u32`: `canonical` checked the bound first.
            FrameId::new(self.out.frames.len() as u32 - 1)
        })
    }

    /// Returns the frame of the table made for the frame `f` of the table
    /// read.
    fn frame(&mut self, f: FrameId) -> FrameId {
        // Collect the frames out to the first one already made, then make
        // them from the outermost in.
        let mut chain = Vec::new();
        let mut next = Some(f);
        let mut below = None;
        while let Some(f) = next {
            if let Some(made) = self.made[f.index()] {
                below = Some(made);
                break;
            }
            chain.push(f);
            next = self.table.frames[f.index()].0;
        }
        for f in chain.into_iter().rev() {
            let term = self.term(self.table.frames[f.index()].1);
            let made = self.push(below, term);
            self.made[f.index()] = Some(made);
            below = Some(made);
        }
        below.expect("a frame holds a term")
    }

    /// Returns the member of the table made for `o` under `frame`, a frame
    /// of the table made, or the occurrence itself when there is no frame.
    fn member(&mut self, o: OccId, frame: Option<FrameId>) -> Member {
        let Some(frame) = frame else {
            return Member::from(o);
        };
        *self.member_at.entry((o, frame)).or_insert_with(|| {
            self.out.members.push((o, frame));
            Member::new(self.out.occurrences + self.out.members.len() as u32 - 1)
        })
    }

    /// Returns the occurrence and frame of a member of the table made.
    fn resolve(&self, m: Member) -> (OccId, Option<FrameId>) {
        match self.out.instance(m) {
            Some((o, f)) => (o, Some(f)),
            None => (m.occ(), None),
        }
    }

    /// Renumbers the nodes' members and witnesses into the table made, adds
    /// the members the checker derives from them, and returns the table.
    fn finish(mut self, nodes: &mut [Node]) -> Result<Instances, Error> {
        let read: Vec<Member> = (0..self.table.members.len())
            .map(|n| {
                let (o, f) = self.table.members[n];
                let f = self.frame(f);
                self.member(o, Some(f))
            })
            .collect();
        let occurrences = self.table.occurrences;
        let renumber = |m: Member| -> Result<Member, Error> {
            match m.get().checked_sub(occurrences) {
                None => Ok(m),
                Some(at) => read
                    .get(at as usize)
                    .copied()
                    .ok_or(Error::IndexOutOfBounds {
                        space: Space::Member,
                        index: m.index(),
                        len: occurrences as usize + read.len(),
                    }),
            }
        };
        for node in nodes.iter_mut() {
            *node = node.renamed(renumber)?;
            if let Node::Exists(m, w, p) = *node {
                *node = Node::Exists(m, self.term(w), p);
            }
        }
        let forest = self.forest;
        for node in nodes.iter() {
            for m in node.members() {
                let (o, frame) = self.resolve(m);
                if frame.is_some() && !forest.kind(o).is_binder() {
                    for c in forest.children(o) {
                        self.member(c, frame);
                    }
                }
            }
            match *node {
                Node::Forall(m, a, _) => {
                    let (o, frame) = self.resolve(m);
                    if forest.kind(o).is_binder() {
                        let term = self.eigen(a);
                        let frame = self.push(frame, term);
                        self.member(OccId::new(o.get() + 1), Some(frame));
                    }
                }
                Node::Exists(m, w, _) => {
                    let (o, frame) = self.resolve(m);
                    if forest.kind(o).is_binder() {
                        let frame = self.push(frame, w);
                        self.member(OccId::new(o.get() + 1), Some(frame));
                    }
                }
                Node::Cut(m, ..) => {
                    let (o, frame) = self.resolve(m);
                    if let Some(dual) = forest.dual(o) {
                        self.member(dual, frame);
                    }
                }
                Node::Ax(..)
                | Node::Tensor(..)
                | Node::Par(..)
                | Node::One(_)
                | Node::Bot(..)
                | Node::With(..)
                | Node::Plus(..)
                | Node::Top(_)
                | Node::Bang(..)
                | Node::Quest(..)
                | Node::Copy(..)
                | Node::Weaken(..)
                | Node::Mix(..) => {}
            }
        }
        Ok(self.out)
    }
}
