// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The proof of a Horn program read off a firing sequence.
//!
//! Read from the root up, the proof does three things. It moves the
//! clauses under `?` into the unrestricted zone (the formulas that may be
//! used any number of times). It decomposes the markings into tokens,
//! one literal each. Then it fires the clauses in order. A firing is a
//! copy of a clause under `?`, or the clause used once itself. Its
//! tensors give each body literal the token it takes, by an axiom, and
//! the rest of the linear zone to the head. The head's `⅋` and `⊥` leave
//! its literals beside the rest as new tokens.
//!
//! At the top, the goal's tensors meet the tokens left, an axiom each. In
//! affine mode the tokens beyond the goal's, and the clauses used once
//! that no firing used, are weakened there.

use super::{Clause, Program, clause_head};
use crate::linear::occurrences::{Forest, Member, OccId, Sign};
use crate::linear::proofs::{Node, NodeId};
use crate::linear::search::Reason;
use crate::linear::search::memory::{Account, Charged};
use crate::linear::sequents::Kind;

/// The most nodes a proof may have, since its ids are `u32`.
///
/// A larger proof is [`Reason::IndexLimit`]. With a memory bound the
/// bound comes first, since a node takes sixteen bytes.
pub(super) const MOST_NODES: u64 = u32::MAX as u64;

/// Builds the proof of the program's goal that fires `firings` from the
/// initial marking. The firings reach the target, or in affine mode cover
/// it.
///
/// Returns the proof's root and its arena of at most `most` nodes, which
/// is charged to `account`. Fails with [`Reason::IndexLimit`] when the
/// proof has more nodes, and with the memory limit when the account has
/// no room.
pub(super) fn build(
    forest: &Forest,
    program: &Program,
    firings: &[u32],
    affine: bool,
    account: &Account,
    most: u64,
) -> Result<(NodeId, Vec<Node>), Reason> {
    let mut charged = Charged::new(account);
    // The clause each firing fires, and whether it is under `?`. Each
    // occurrence of the clause is at most one node, and a copy is one
    // more.
    let mut once = program.once.clone();
    let clauses: Vec<(OccId, bool)> = firings
        .iter()
        .map(|&t| match program.transitions[t as usize].clause {
            Clause::Reusable(clause) => (clause, true),
            Clause::Once(class) => (
                once[class as usize]
                    .pop()
                    .expect("the target leaves no clause unused, and a firing takes one"),
                false,
            ),
        })
        .collect();
    // The clauses used once that no firing used, which only affine mode
    // leaves.
    let unused: Vec<OccId> = once.into_iter().flatten().collect();
    debug_assert!(affine || unused.is_empty());
    let size = |o: OccId| u64::from(forest.size(o));
    let nodes = clauses.iter().map(|&(c, _)| size(c) + 1).sum::<u64>()
        + size(program.goal)
        + program.markings.iter().map(|&m| size(m)).sum::<u64>()
        + program.quests.len() as u64;
    if nodes > most {
        return Err(Reason::IndexLimit);
    }
    // A weakening for each token left beside the goal's: the tokens given
    // and made less those taken. Each token is a literal, and so a node
    // counted above. So none of these sums passes twice the bound.
    let atoms = program.places - program.once.len();
    let literals = |arcs: &[(u32, u32)]| {
        arcs.iter()
            .filter(|&&(p, _)| (p as usize) < atoms)
            .map(|&(_, w)| u64::from(w))
            .sum::<u64>()
    };
    let (mut made, mut taken) = (
        literals_of(&program.initial[..atoms]),
        literals_of(&program.target[..atoms]),
    );
    for &t in firings {
        let transition = &program.transitions[t as usize];
        taken += literals(program.inputs(transition));
        made += literals(program.outputs(transition));
    }
    let nodes = nodes + (made - taken) + unused.len() as u64;
    if nodes > most {
        return Err(Reason::IndexLimit);
    }
    // The charge covers:
    // - the nodes;
    // - a pair of a body literal and its token per node at most;
    // - the tokens the replay keeps, a literal and so a node each;
    // - the clauses fired, with where each one's pairs start.
    let per_node = size_of::<Node>() + size_of::<(OccId, OccId)>() + size_of::<OccId>();
    let per_firing = size_of::<(OccId, bool)>() + size_of::<usize>();
    let bytes = usize::try_from(nodes)
        .ok()
        .and_then(|nodes| nodes.checked_mul(per_node))
        .and_then(|bytes| bytes.checked_add(clauses.len() * per_firing))
        .filter(|&bytes| account.fits(bytes));
    let Some(bytes) = bytes else {
        return Err(Reason::MemoryLimit {
            limit_bytes: account.limit(),
        });
    };
    charged.charge(bytes);
    let (pairs, starts, left) = replay(forest, program, &clauses, nodes as usize);
    let mut builder = Builder {
        forest,
        body: program.body,
        nodes: Vec::with_capacity(nodes as usize),
        values: Vec::new(),
    };
    // The goal's pairs come last in the replay, and each firing's in its
    // place.
    let mut goal = &pairs[starts[clauses.len()]..];
    let mut root = builder.clause(program.goal, None, &mut goal, NodeId::new(0));
    debug_assert_eq!(goal, []);
    for &weakened in left.iter().chain(&unused) {
        root = builder.push(Node::Weaken(Member::from(weakened), root));
    }
    for (i, &(clause, reusable)) in clauses.iter().enumerate().rev() {
        let head = clause_head(forest, program.body, clause).expect("a clause the program read");
        let mut fired = &pairs[starts[i]..starts[i + 1]];
        root = builder.clause(clause, head, &mut fired, root);
        debug_assert_eq!(fired, []);
        if reusable {
            root = builder.push(Node::Copy(Member::from(clause), root));
        }
    }
    for &marking in &program.markings {
        root = builder.head(marking, root);
    }
    for &quest in &program.quests {
        root = builder.push(Node::Quest(Member::from(quest), root));
    }
    Ok((root, builder.nodes))
}

/// Returns the tokens of a marking.
fn literals_of(marking: &[u32]) -> u64 {
    marking.iter().map(|&c| u64::from(c)).sum()
}

/// Replays the firings on the tokens, starting from the markings' tokens.
/// A token is an occurrence of a head literal.
///
/// Every body literal of a clause fired takes a token of its atom, and
/// the clause's head literals become tokens. Returns three things:
/// - the pairs of a body literal and its token, each firing's and then
///   the goal's, each in decreasing order of the body literals' ids,
///   which is the order the proof takes them in;
/// - where each firing's pairs start, and the goal's last;
/// - the tokens the goal leaves, which only a firing sequence that covers
///   the target without reaching it leaves.
fn replay(
    forest: &Forest,
    program: &Program,
    clauses: &[(OccId, bool)],
    capacity: usize,
) -> (Vec<(OccId, OccId)>, Vec<usize>, Vec<OccId>) {
    let place = |literal: OccId| {
        program.place_of[forest.atom(literal).expect("a literal").index()] as usize
    };
    let mut tokens: Vec<Vec<OccId>> = vec![Vec::new(); program.places];
    for &marking in &program.markings {
        for x in forest.subtree(marking).filter(|&x| forest.is_literal(x)) {
            tokens[place(x)].push(x);
        }
    }
    // Each firing's pairs and the goal's, and where each list starts.
    let mut pairs = Vec::with_capacity(capacity);
    let mut starts = Vec::with_capacity(clauses.len() + 1);
    let take =
        |clause: OccId, head: Option<OccId>, tokens: &mut Vec<Vec<OccId>>, pairs: &mut Vec<_>| {
            for x in forest.subtree(clause).rev() {
                if !head.is_some_and(|h| forest.is_below(x, h)) && forest.is_literal(x) {
                    let token = tokens[place(x)].pop().expect("the firing is enabled");
                    pairs.push((x, token));
                }
            }
        };
    for &(clause, _) in clauses {
        starts.push(pairs.len());
        let head = clause_head(forest, program.body, clause).expect("a clause the program read");
        take(clause, head, &mut tokens, &mut pairs);
        if let Some(h) = head {
            for x in forest.subtree(h).filter(|&x| forest.is_literal(x)) {
                tokens[place(x)].push(x);
            }
        }
    }
    starts.push(pairs.len());
    take(program.goal, None, &mut tokens, &mut pairs);
    (pairs, starts, tokens.into_iter().flatten().collect())
}

/// The builder of the proof's nodes: the forest, the sign of the bodies,
/// the arena, and a stack of the nodes that prove a tensor's factors.
struct Builder<'a> {
    /// The forest.
    forest: &'a Forest,
    /// The sign of the bodies' atoms.
    body: Sign,
    /// The arena of the proof's nodes.
    nodes: Vec<Node>,
    /// The proofs of the factors of a clause whose tensors are not built
    /// yet.
    values: Vec<NodeId>,
}

impl Builder<'_> {
    /// Pushes a node and returns its id.
    fn push(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        NodeId::new(self.nodes.len() as u32 - 1)
    }

    /// Wraps `above`, whose conclusion holds the literals of a head or a
    /// marking, in that formula's `⅋` and `⊥` rules, and returns the new
    /// root.
    ///
    /// The rules go in decreasing order of ids, so both subformulas of a
    /// `⅋` are in the conclusion when its rule comes.
    fn head(&mut self, head: OccId, mut above: NodeId) -> NodeId {
        for x in self.forest.subtree(head).rev() {
            match self.forest.kind(x) {
                Kind::Par => above = self.push(Node::Par(Member::from(x), above)),
                Kind::Bot => above = self.push(Node::Bot(Member::from(x), above)),
                _ => {}
            }
        }
        above
    }

    /// Builds the proof of a clause fired, or of the goal, and returns its
    /// root. The proof `above` takes the head's literals as tokens.
    ///
    /// The proof has an axiom for each body literal with its token (the
    /// next of `pairs`), a `1` rule for each `1`, and the tensors over
    /// them. The nodes go in decreasing order of ids, so both factors of a
    /// tensor are proved when it comes.
    fn clause(
        &mut self,
        clause: OccId,
        head: Option<OccId>,
        pairs: &mut &[(OccId, OccId)],
        above: NodeId,
    ) -> NodeId {
        let forest = self.forest;
        for x in forest.subtree(clause).rev() {
            let node = match head {
                Some(h) if x == h => self.head(h, above),
                Some(h) if forest.is_below(x, h) => continue,
                _ => match forest.kind(x) {
                    Kind::Tensor => {
                        let left = self.values.pop().expect("the left factor is proved");
                        let right = self.values.pop().expect("the right factor is proved");
                        self.push(Node::Tensor(Member::from(x), left, right))
                    }
                    Kind::One => self.push(Node::One(Member::from(x))),
                    _ => {
                        let ((literal, token), rest) =
                            pairs.split_first().expect("a token per body literal");
                        debug_assert_eq!(*literal, x);
                        debug_assert_eq!(forest.sign(x), Some(self.body));
                        *pairs = rest;
                        self.push(Node::Ax(x.into(), (*token).into()))
                    }
                },
            };
            self.values.push(node);
        }
        self.values.pop().expect("the clause is proved")
    }
}
