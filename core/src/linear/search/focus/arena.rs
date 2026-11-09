// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The proof arena of a run: the nodes of the branch being searched,
//! pending on the engine's own stack, and the nodes kept for the memo's
//! entries and the final proof. The memo is the table of the sequents the
//! search has decided.

use crate::linear::proofs::{Node, NodeId};
use crate::linear::search::Reason;
use crate::linear::search::memory::{Account, Charged, bytes_of};
use std::sync::Mutex;

/// Where the nodes of a run's proofs are kept: the engine's own arena in
/// a sequential search, or the one arena of a parallel search behind its
/// lock.
pub(crate) enum Kept<'a> {
    /// The engine's own arena.
    Own(Vec<Node>),
    /// The arena of a parallel search.
    #[cfg_attr(
        not(feature = "parallel"),
        expect(dead_code, reason = "only a pool shares its kept arena")
    )]
    Shared(&'a Mutex<Vec<Node>>),
}

/// The flag in the id of a node that is still pending.
pub(super) const PENDING: u32 = 1 << 31;

/// How many nodes [`Arena::keep_all`] renames between two polls of its
/// stop: some tenths of a millisecond.
const ROOT_POLL: usize = 1 << 16;

/// The proof arena of a run, in two parts.
///
/// A node is first *pending*. It belongs to the branch being searched and
/// lives on a stack in the engine's own buffer. It goes when the branch
/// fails ([`release`](Self::release)). A pending node's id carries the
/// [`PENDING`] flag and is valid only until its branch is released or
/// kept. In a parallel search no pending id leaves its worker.
///
/// Once the stable sequent a node helps to prove is proved, its nodes are
/// *kept* ([`keep`](Self::keep)). (A stable sequent is one with no rule
/// left that applies without a choice; the memo holds only those.) They
/// move to the arena that the memo's entries and the final proof refer
/// to, where a node's id is its position and a premise precedes its
/// conclusion. So the nodes of failed branches never reach the kept
/// arena, and a refutation's memory is bounded by what the memo holds,
/// however many branches it tried.
///
/// When an engine's own memo is emptied, the kept nodes that only its
/// entries referred to are dropped, and the others move up
/// ([`collect`](Self::collect)). What still refers to a kept node then is
/// a pending node, or a rule that holds the proof of one premise while it
/// searches the other. Such a rule *holds* the id ([`hold`](Self::hold))
/// for the time of that search, and reads it back afterwards, since it
/// may have moved.
pub(crate) struct Arena<'a> {
    /// The nodes kept.
    pub(super) kept: Kept<'a>,
    /// The nodes of the branch being searched.
    pending: Vec<Node>,
    /// The ids that rules of the branch hold across a search: the proof
    /// of a first premise while the second is searched.
    held: Vec<NodeId>,
    /// The most nodes either part may hold, at most [`PENDING`]. An id
    /// beyond would read as the flag.
    most: usize,
    /// Whether a node was pushed that the pending part had no id for. No
    /// proof resting on it is kept.
    overflowed: bool,
    /// Who is charged the memory of the kept part.
    pub(super) account: &'a Account,
    /// What the pending part was charged, which goes with the engine.
    own: Charged<'a>,
}

impl<'a> Arena<'a> {
    /// An arena with nothing pending, charging `account`.
    pub(crate) fn new(kept: Kept<'a>, account: &'a Account) -> Self {
        Self {
            kept,
            pending: Vec::new(),
            held: Vec::new(),
            most: PENDING as usize,
            overflowed: false,
            account,
            own: Charged::new(account),
        }
    }

    /// Appends a pending node and returns its id.
    ///
    /// When the pending part is full, the node is dropped and the arena
    /// remembers it. The id returned is then no node's, and
    /// [`keep`](Self::keep) refuses whatever would rest on it.
    pub(super) fn push(&mut self, node: Node) -> NodeId {
        if self.pending.len() >= self.most {
            self.overflowed = true;
            return NodeId::new(PENDING);
        }
        let id = NodeId::new(PENDING | self.pending.len() as u32);
        let before = bytes_of(&self.pending);
        self.pending.push(node);
        if bytes_of(&self.pending) != before {
            self.own.resize(before, bytes_of(&self.pending));
        }
        id
    }

    /// Returns the point of the pending nodes to come back to.
    pub(super) fn mark(&self) -> usize {
        self.pending.len()
    }

    /// Drops the nodes pending since the mark, since their branch failed.
    pub(super) fn release(&mut self, mark: usize) {
        self.pending.truncate(mark);
    }

    /// Holds an id across a search that may empty the memo, and returns
    /// the point to read it back from.
    pub(super) fn hold(&mut self, node: NodeId) -> usize {
        self.held.push(node);
        self.held.len() - 1
    }

    /// Reads back the id held at the point given, which may have moved,
    /// and lets go of it and of everything held after it.
    pub(super) fn unhold(&mut self, point: usize) -> NodeId {
        let node = self.held[point];
        self.held.truncate(point);
        node
    }

    /// Keeps the nodes pending since the mark, and returns the id `node`
    /// has from now on. The nodes since the mark must hold everything
    /// pending that `node` rests on. A node that was kept already keeps
    /// its id.
    ///
    /// Fails with [`Reason::IndexLimit`] when the kept part would outgrow
    /// its ids, or when a node of the branch was dropped because the
    /// pending part had outgrown them.
    pub(super) fn keep(&mut self, mark: usize, node: NodeId) -> Result<NodeId, Reason> {
        if self.overflowed {
            return Err(Reason::IndexLimit);
        }
        let (most, account) = (self.most, self.account);
        match &mut self.kept {
            Kept::Own(kept) => Self::append(kept, &mut self.pending, mark, node, most, account),
            Kept::Shared(kept) => {
                let mut kept = crate::linear::search::lock(kept);
                Self::append(&mut kept, &mut self.pending, mark, node, most, account)
            }
        }
    }

    /// Keeps every pending node, all of which `node` rests on, and returns
    /// the id `node` has from then on. This keeps the root's proof once
    /// the search ends.
    ///
    /// Where nothing is kept yet, the pending nodes become the kept ones in
    /// place. Their premises are renamed in one pass that asks `stopped`
    /// every [`ROOT_POLL`] nodes, so a proof of hundreds of millions of
    /// nodes is not copied, and does not hold off the stop. Otherwise it
    /// works as [`keep`](Self::keep) does.
    ///
    /// Fails as `keep` does, or with [`Reason::Stopped`].
    pub(super) fn keep_all(
        &mut self,
        node: NodeId,
        stopped: &mut dyn FnMut() -> bool,
    ) -> Result<NodeId, Reason> {
        let Kept::Own(kept) = &mut self.kept else {
            return self.keep(0, node);
        };
        if self.overflowed || !kept.is_empty() {
            return self.keep(0, node);
        }
        let pending = bytes_of(&self.pending);
        std::mem::swap(kept, &mut self.pending);
        self.own.resize(pending, 0);
        self.account.resize(0, pending);
        let renamed = |id: NodeId| NodeId::new(id.get() & !PENDING);
        for (i, n) in kept.iter_mut().enumerate() {
            if i % ROOT_POLL == 0 && stopped() {
                return Err(Reason::Stopped);
            }
            *n = n.map_premises(renamed);
        }
        Ok(renamed(node))
    }

    /// Moves the nodes pending since the mark to the end of the kept ones,
    /// with their premises renamed, and returns the new id of `node`.
    /// Fails where that would make more than `most` kept nodes.
    fn append(
        kept: &mut Vec<Node>,
        pending: &mut Vec<Node>,
        mark: usize,
        node: NodeId,
        most: usize,
        account: &Account,
    ) -> Result<NodeId, Reason> {
        let base = kept.len();
        if base + (pending.len() - mark) > most {
            return Err(Reason::IndexLimit);
        }
        let renamed = |id: NodeId| {
            if id.get() & PENDING == 0 {
                return id;
            }
            let index = (id.get() & !PENDING) as usize;
            debug_assert!(index >= mark, "a pending node below the mark");
            NodeId::new((base + index - mark) as u32)
        };
        let before = bytes_of(kept);
        kept.extend(pending.drain(mark..).map(|n| n.map_premises(renamed)));
        if bytes_of(kept) != before {
            account.resize(before, bytes_of(kept));
        }
        Ok(renamed(node))
    }

    /// Drops every kept node that nothing refers to any more, once the
    /// engine's own memo is emptied, and returns the new id of `root`.
    ///
    /// What stays is what the pending nodes, the ids held and `root` rest
    /// on. The nodes that stay move up in their order, so a premise still
    /// precedes its conclusion. The pending nodes and the ids held are
    /// renamed in place.
    ///
    /// The memory freed is given back when it is three quarters of the
    /// allocation, down to twice what stays. So the nodes to come have
    /// room, and a collection never costs a reallocation at the next node.
    /// With `tight`, all of it is given back.
    ///
    /// The arena of a parallel search is left alone: its workers hold ids
    /// that nobody could rename.
    pub(super) fn collect(&mut self, root: Option<NodeId>, tight: bool) -> Option<NodeId> {
        /// A node that goes.
        const DEAD: u32 = u32::MAX;
        /// A node that stays, before its place is known.
        const LIVE: u32 = 0;
        let Kept::Own(kept) = &mut self.kept else {
            return root;
        };
        let before = bytes_of(kept);
        // Where each kept node moves to. This is four bytes a node for the
        // time of the collection, which the account does not see.
        let mut moved = vec![DEAD; kept.len()];
        let is_kept = |id: &NodeId| id.get() & PENDING == 0;
        for id in self
            .pending
            .iter()
            .flat_map(|node| node.premises())
            .chain(self.held.iter().copied())
            .chain(root)
            .filter(is_kept)
        {
            moved[id.index()] = LIVE;
        }
        // A premise has a smaller id than its conclusion, so one pass
        // down marks everything the live nodes rest on.
        for i in (0..kept.len()).rev() {
            if moved[i] == LIVE {
                for premise in kept[i].premises() {
                    moved[premise.index()] = LIVE;
                }
            }
        }
        let mut next = 0;
        for i in 0..kept.len() {
            if moved[i] == LIVE {
                kept[next] = kept[i].map_premises(|p| NodeId::new(moved[p.index()]));
                moved[i] = next as u32;
                next += 1;
            }
        }
        kept.truncate(next);
        let renamed = |id: NodeId| {
            if is_kept(&id) {
                NodeId::new(moved[id.index()])
            } else {
                id
            }
        };
        for node in &mut self.pending {
            *node = node.map_premises(renamed);
        }
        for id in &mut self.held {
            *id = renamed(*id);
        }
        let root = root.map(renamed);
        if tight {
            kept.shrink_to_fit();
        } else if kept.capacity() > 4 * kept.len().max(1024) {
            kept.shrink_to(2 * kept.len().max(1024));
        }
        self.account.resize(before, bytes_of(kept));
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear::occurrences::Member;

    /// A released branch leaves no node, a kept one moves to the kept
    /// arena with its premises renamed, and a node kept before keeps its
    /// id.
    #[test]
    fn arena() {
        let o = Member::new;
        let account = Account::new(None);
        let mut arena = Arena::new(Kept::Own(Vec::new()), &account);
        let ax = arena.push(Node::Ax(o(0), o(1)));
        let mark = arena.mark();
        let one = arena.push(Node::One(o(2)));
        arena.push(Node::Tensor(o(3), ax, one));
        arena.release(mark);
        let top = arena.push(Node::Top(o(4)));
        let top = arena.keep(mark, top).unwrap();
        assert_eq!(top, NodeId::new(0));
        let tensor = arena.push(Node::Tensor(o(5), ax, top));
        let root = arena.keep(0, tensor).unwrap();
        assert_eq!(root, NodeId::new(2));
        let Kept::Own(kept) = arena.kept else {
            unreachable!()
        };
        assert_eq!(
            kept,
            [
                Node::Top(o(4)),
                Node::Ax(o(0), o(1)),
                Node::Tensor(o(5), NodeId::new(1), NodeId::new(0))
            ]
        );
    }

    /// A collection drops the kept nodes that neither a pending node, nor
    /// an id held, nor the root given rests on. The others move up in
    /// their order, whoever refers to them is renamed, and the memory
    /// freed is no longer charged.
    #[test]
    fn collection() {
        let o = Member::new;
        let account = Account::new(None);
        let mut arena = Arena::new(Kept::Own(Vec::new()), &account);
        let kept = |arena: &mut Arena, node| {
            let id = arena.push(node);
            arena.keep(0, id).unwrap()
        };
        let dead = kept(&mut arena, Node::One(o(0)));
        let below = kept(&mut arena, Node::Top(o(1)));
        let held = kept(&mut arena, Node::Bot(o(2), below));
        kept(&mut arena, Node::Bot(o(3), dead));
        let rooted = kept(&mut arena, Node::One(o(4)));
        let under = kept(&mut arena, Node::Top(o(5)));
        let pending = arena.push(Node::Bot(o(6), under));
        let point = arena.hold(held);
        let before = account.used();
        let rooted = arena.collect(Some(rooted), true).unwrap();
        assert!(account.used() < before);
        assert_eq!(arena.unhold(point), NodeId::new(1));
        assert_eq!(rooted, NodeId::new(2));
        let root = arena.keep(0, pending).unwrap();
        assert_eq!(root, NodeId::new(4));
        let Kept::Own(kept) = arena.kept else {
            unreachable!()
        };
        assert_eq!(
            kept,
            [
                Node::Top(o(1)),
                Node::Bot(o(2), NodeId::new(0)),
                Node::One(o(4)),
                Node::Top(o(5)),
                Node::Bot(o(6), NodeId::new(3))
            ]
        );
    }

    /// An arena whose kept part is full refuses to keep more, and one
    /// whose pending part is full refuses whatever rests on the node it
    /// had no id for. Both answer [`Reason::IndexLimit`] and do not panic.
    #[test]
    fn full_arena() {
        let o = Member::new;
        let account = Account::new(None);
        let mut arena = Arena::new(Kept::Own(Vec::new()), &account);
        arena.most = 2;
        for id in 0..2 {
            let node = arena.push(Node::One(o(id)));
            assert!(arena.keep(0, node).is_ok());
        }
        let third = arena.push(Node::One(o(2)));
        assert_eq!(arena.keep(0, third), Err(Reason::IndexLimit));
        let mut arena = Arena::new(Kept::Own(Vec::new()), &account);
        arena.most = 2;
        let first = arena.push(Node::One(o(0)));
        arena.push(Node::One(o(1)));
        arena.push(Node::One(o(2)));
        assert_eq!(arena.keep(0, first), Err(Reason::IndexLimit));
    }
}
