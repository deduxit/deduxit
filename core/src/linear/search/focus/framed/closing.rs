// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The closing of a first-order proof: the search's terms, frames and
//! members written into the proof's instance table.

use super::{FMember, Framed, NONE, TRef, Var};
use crate::hash::HashMap;
use crate::linear::Instances;
use crate::linear::occurrences::{FrameId, Member};
use crate::linear::sequents::fo::FirstOrder;
use crate::linear::sequents::{Eigen, FoTermId};

/// The closing of a proof: its instance table as it is built, and what
/// it gave each term, frame, member and unbound metavariable.
pub(super) struct Closing<'a> {
    /// The search.
    pub(super) framed: &'a Framed,
    /// The sequent's tables.
    pub(super) fo: &'a FirstOrder,
    /// The table.
    pub(super) table: Instances,
    /// What each term became.
    pub(super) terms: HashMap<TRef, FoTermId>,
    /// What each frame became.
    pub(super) frames: HashMap<u32, FrameId>,
    /// What each member past the forest became.
    pub(super) members: HashMap<u32, Member>,
    /// The fresh constant of each unbound metavariable.
    pub(super) fresh: HashMap<u32, Eigen>,
}

impl Closing<'_> {
    /// Returns the table's term for a term, as the search's bindings left it.
    pub(super) fn term(&mut self, t: TRef) -> FoTermId {
        let framed = self.framed;
        let mut walk = vec![(t, false)];
        while let Some((u, expanded)) = walk.pop() {
            let u = framed.deref(self.fo, u);
            if !expanded && self.terms.contains_key(&u) {
                continue;
            }
            let id = if let Some(v) = framed.var(u) {
                let eigen = match framed.vars[v as usize] {
                    Var::Eigen(number) => Eigen::new(number),
                    Var::Meta { .. } => {
                        // An unbound metavariable becomes a constant outside
                        // the signature that no `∀` step introduces: an
                        // eigenvariable numbered past every one the search
                        // made.
                        let next = framed.eigens.saturating_add(self.fresh.len() as u32);
                        *self.fresh.entry(v).or_insert(Eigen::new(next))
                    }
                };
                self.table.eigen(eigen)
            } else if framed.closed[u.id as usize] {
                FoTermId::new(u.id)
            } else {
                let (symbol, args) = Framed::app(self.fo, u);
                let child = |a: &FoTermId| TRef {
                    id: a.get(),
                    frame: u.frame,
                };
                if !expanded {
                    walk.push((u, true));
                    walk.extend(args.iter().map(|a| (child(a), false)));
                    continue;
                }
                let ids: Vec<FoTermId> = args
                    .iter()
                    .map(|a| self.terms[&framed.deref(self.fo, child(a))])
                    .collect();
                self.table.apply(symbol, &ids)
            };
            self.terms.insert(u, id);
        }
        self.terms[&framed.deref(self.fo, t)]
    }

    /// The table's frame for a frame of the search.
    fn frame(&mut self, frame: u32) -> FrameId {
        let mut chain = Vec::new();
        let mut f = frame;
        while f != NONE && !self.frames.contains_key(&f) {
            chain.push(f);
            f = self.framed.frames[f as usize].0;
        }
        for &f in chain.iter().rev() {
            let (parent, term) = self.framed.frames[f as usize];
            let parent = (parent != NONE).then(|| self.frames[&parent]);
            let term = self.term(term);
            let id = self.table.frame(parent, term);
            self.frames.insert(f, id);
        }
        self.frames[&frame]
    }

    /// The table's member for a member of the search.
    pub(super) fn member(&mut self, m: FMember) -> Member {
        if m.0 < self.framed.occurrences {
            return Member::new(m.0);
        }
        if let Some(&member) = self.members.get(&m.0) {
            return member;
        }
        let (o, frame) = self.framed.instance(m);
        let frame = self.frame(frame);
        let member = self.table.member(o, frame);
        self.members.insert(m.0, member);
        member
    }
}
