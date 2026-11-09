// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Reachability: whether the net's transitions, fired one after the
//! other from the initial marking (a firing sequence), can reach the
//! target marking. A marking is a count of tokens per place.
//!
//! The search visits the markings one by one. It keeps every marking it
//! expands once, as its counts written sparsely in a few bytes. The
//! frontier holds the successors not yet expanded, each as the marking
//! and the transition it comes from, nearest to the target first. A
//! successor is written out only when it is expanded, and dropped then if
//! it was kept before. A finite set of reachable markings runs out, which
//! refutes the goal. An infinite one runs until the stop or the memory
//! bound.
//!
//! Once the forward search has done some work without deciding, a second
//! search runs beside it, a share of the work at a time. It is the same
//! search on the reversed net, from the target toward the initial
//! marking. It caps the places that no transition raises at their initial
//! count, since no marking that a firing sequence from the initial one
//! passes has more. When it finds the initial marking, its firings read
//! backward are a firing sequence. When it runs out of markings, that
//! refutes. Nets whose markings grow without end forward often have few
//! backward.

use super::equation::Equation;
use super::{Parents, Program};
use crate::linear::search::memory::{Account, Charged, bytes_of};
use crate::linear::search::{Reason, Statistics};
use std::cell::OnceCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::BuildHasher;

/// The most markings the search keeps. Their indices are `u32`, and the
/// table stores an index plus one.
///
/// Reaching it is [`Reason::IndexLimit`]. With a memory bound the bound
/// comes first, since a marking kept takes more than thirty bytes.
pub(super) const MOST_MARKINGS: usize = u32::MAX as usize - 1;

/// How many markings the forward search keeps alone before the backward
/// search starts beside it.
///
/// A net decided within them is searched exactly as without the backward
/// search, and most nets met in practice are. A count of work would start
/// the backward search after a few expansions of the LLTP library's
/// largest nets.
const BACKWARD_AFTER: usize = 1 << 14;

/// The backward search's share: it may do one unit of work for every
/// this many of the forward search's.
const FORWARD_PER_BACKWARD: u64 = 4;

/// Searches the program's markings from the initial one for the target.
///
/// Returns the transitions of a firing sequence that reaches the target,
/// `None` when the markings of either direction run out or the state
/// equation refutes, or the reason the search stopped. Also returns the
/// counters of both directions:
/// - `nodes`: the initial marking and the successors taken from the
///   frontier;
/// - `memo_hits`: those of them kept already or above a cap;
/// - `memo_entries`: the markings kept, each expanded once.
///
/// It polls `stop` at every successor taken from the frontier, keeps at
/// most `most` markings in each direction, and gives `equation` and the
/// backward search their shares of the work.
pub(super) fn search(
    program: &Program,
    account: &Account,
    most: usize,
    equation: &mut Equation<'_>,
    stop: &mut dyn FnMut(u64) -> bool,
) -> (Result<Option<Vec<u32>>, Reason>, Statistics) {
    // The reversed program and the backward search are made only when the
    // backward search starts, since most nets are decided before.
    let reversed = OnceCell::new();
    let mut forward = Reach::new(program, account, Vec::new());
    let mut backward = None;
    let result = both(&mut forward, &mut backward, &reversed, most, equation, stop);
    let (nodes, hits, kept) = backward
        .as_ref()
        .map_or((0, 0, 0), |b: &Reach<'_>| (b.nodes(), b.repeated, b.kept));
    let statistics = Statistics {
        nodes: forward.nodes() + nodes,
        memo_hits: forward.repeated + hits,
        memo_entries: (forward.kept + kept) as u64,
        ..Statistics::default()
    };
    (result, statistics)
}

/// Runs the forward search with the state equation and the backward
/// search beside it, as [`search`] describes.
///
/// The backward search is made in `backward` when it starts, on the
/// program reversed in `reversed`.
fn both<'a>(
    forward: &mut Reach<'a>,
    backward: &mut Option<Reach<'a>>,
    reversed: &'a OnceCell<Program>,
    most: usize,
    equation: &mut Equation<'_>,
    stop: &mut dyn FnMut(u64) -> bool,
) -> Result<Option<Vec<u32>>, Reason> {
    let program = forward.program;
    let account = forward.charged.account();
    // Out of room, the forward search takes back the simplex's memory and
    // the backward search's, and tries once more.
    let release = |equation: &mut Equation<'_>, backward: &mut Option<Reach<'a>>| {
        // Both give back at once, whichever of them held something.
        let gave = equation.release();
        backward.as_mut().is_some_and(Reach::give_back) || gave
    };
    if let Some(firings) = forward.start(most, &mut || release(equation, backward))? {
        return Ok(Some(firings));
    }
    loop {
        if stop(1) {
            return Err(Reason::Stopped);
        }
        let work = forward.work;
        let budget = equation.budget(work);
        if equation.wants(budget) && equation.run(program, budget, stop)? {
            return Ok(None);
        }
        if forward.kept >= BACKWARD_AFTER {
            let behind = backward.get_or_insert_with(|| {
                let reversed = reversed.get_or_init(|| program.reversed());
                Reach::new(reversed, account, program.caps())
            });
            if behind.alive {
                match behind.advance(work / FORWARD_PER_BACKWARD, most, stop) {
                    Ok(Slice::Found(mut firings)) => {
                        firings.reverse();
                        return Ok(Some(firings));
                    }
                    Ok(Slice::Exhausted) => return Ok(None),
                    Ok(Slice::Paused) => {}
                    Err(Reason::MemoryLimit { .. } | Reason::IndexLimit) => {
                        behind.give_back();
                    }
                    Err(reason) => return Err(reason),
                }
            }
        }
        match forward.step(most, &mut || release(equation, backward))? {
            Slice::Paused => {}
            Slice::Found(firings) => return Ok(Some(firings)),
            Slice::Exhausted => return Ok(None),
        }
    }
}

/// The outcome of a step or a slice of a search.
enum Slice {
    /// The target was reached, by these firings from the initial marking.
    Found(Vec<u32>),
    /// The frontier is empty: every reachable marking was expanded.
    Exhausted,
    /// Neither yet.
    Paused,
}

/// An entry of the frontier: a key and the transition that leads to the
/// successor.
///
/// The key has the successor's distance to the target in its high bits
/// and its parent's index complemented in its low bits. So the nearest
/// successor comes first and, among equals, a successor of the latest
/// marking expanded.
type Successor = Reverse<(u64, u32)>;

/// A search in one direction: the markings kept, the frontier, the
/// transitions indexed by their first input place, the marking at hand,
/// and the counters.
struct Reach<'a> {
    /// The program searched.
    program: &'a Program,
    /// The tokens of the target, which is the empty marking's distance to
    /// it. At most the forest's occurrences.
    target_total: u64,
    /// The markings kept, one after the other.
    ///
    /// Each is its marked places in increasing order. A place is written
    /// as its distance from the one before less one, then its count less
    /// one, both in LEB128. The bytes are a function of the marking, so
    /// two markings are equal exactly when their bytes are.
    bytes: Vec<u8>,
    /// Where each marking's bytes end.
    ends: Vec<u64>,
    /// The hash of each marking's bytes.
    hashes: Vec<u64>,
    /// For each marking kept, the marking it was reached from and the
    /// transition fired.
    parents: Parents,
    /// The hash table of markings: a power of two of slots, each zero or
    /// a marking's index plus one. At most half of them are used.
    slots: Vec<u32>,
    /// The successors to expand.
    frontier: BinaryHeap<Successor>,
    /// The transitions whose first input is each place: those of place
    /// `p` are `by_place[starts[p]..starts[p + 1]]`.
    starts: Vec<u32>,
    /// The transitions by their first input place.
    by_place: Vec<u32>,
    /// The transitions without inputs, which can always fire.
    free: Vec<u32>,
    /// The marking at hand, as a count per place: zero except on
    /// `marked`.
    counts: Vec<u32>,
    /// The places the marking at hand marks, in increasing order.
    marked: Vec<u32>,
    /// A buffer for the places of a successor.
    next: Vec<u32>,
    /// A buffer for a marking's bytes.
    written: Vec<u8>,
    /// The memory charged for the buffers that grow.
    charged: Charged<'a>,
    /// The successors taken from the frontier that were kept already or
    /// above a cap.
    repeated: u64,
    /// The markings kept.
    kept: usize,
    /// The work done, in transitions examined and markings written.
    work: u64,
    /// The most tokens a marking may hold on each place. A marking above
    /// a cap is dropped. Empty for no caps.
    caps: Vec<u32>,
    /// Whether the search has started.
    started: bool,
    /// Whether the search still holds its markings, that is, has not
    /// given its memory back.
    alive: bool,
}

impl<'a> Reach<'a> {
    /// Returns a search of the program with nothing kept yet, with caps
    /// per place, or none when `caps` is empty.
    fn new(program: &'a Program, account: &'a Account, caps: Vec<u32>) -> Self {
        let width = program.places;
        let mut starts = vec![0u32; width + 1];
        let mut free = Vec::new();
        for (t, transition) in program.transitions.iter().enumerate() {
            if transition.inputs == transition.outputs {
                free.push(t as u32);
            } else {
                starts[program.inputs(transition)[0].0 as usize + 1] += 1;
            }
        }
        for p in 0..width {
            starts[p + 1] += starts[p];
        }
        let mut next = starts.clone();
        let mut by_place = vec![0; starts[width] as usize];
        for (t, transition) in program.transitions.iter().enumerate() {
            if transition.inputs != transition.outputs {
                let p = program.inputs(transition)[0].0 as usize;
                by_place[next[p] as usize] = t as u32;
                next[p] += 1;
            }
        }
        Self {
            program,
            target_total: program.target.iter().map(|&t| u64::from(t)).sum(),
            bytes: Vec::new(),
            ends: Vec::new(),
            hashes: Vec::new(),
            parents: Parents::default(),
            slots: Vec::new(),
            frontier: BinaryHeap::new(),
            starts,
            by_place,
            free,
            counts: vec![0; width],
            marked: Vec::new(),
            next: Vec::new(),
            written: Vec::new(),
            charged: Charged::new(account),
            repeated: 0,
            kept: 0,
            work: 0,
            caps,
            started: false,
            alive: true,
        }
    }

    /// Returns how many markings were taken: the kept and the dropped.
    fn nodes(&self) -> u64 {
        self.kept as u64 + self.repeated
    }

    /// Gives back the memory the markings hold, for a search beside this
    /// one that has no room left. Returns whether there was any to give.
    ///
    /// This search then ends without deciding.
    fn give_back(&mut self) -> bool {
        if !self.alive || !self.started {
            return false;
        }
        self.alive = false;
        self.bytes = Vec::new();
        self.ends = Vec::new();
        self.hashes = Vec::new();
        self.parents = Parents::default();
        self.slots = Vec::new();
        self.frontier = BinaryHeap::new();
        self.charged = Charged::new(self.charged.account());
        true
    }

    /// Starts at the initial marking.
    ///
    /// Returns the empty firing sequence if the initial marking is the
    /// target, and the firing that reaches the target from it if one does.
    /// Otherwise returns `None`, with its successors on the frontier.
    fn start(
        &mut self,
        most: usize,
        release: &mut dyn FnMut() -> bool,
    ) -> Result<Option<Vec<u32>>, Reason> {
        self.started = true;
        for (p, &count) in self.program.initial.iter().enumerate() {
            if count > 0 {
                self.counts[p] = count;
                self.marked.push(p as u32);
            }
        }
        if self.distance() == 0 {
            return Ok(Some(Vec::new()));
        }
        self.take(Parents::ROOT, 0, most, release)
    }

    /// Takes the next successor from the frontier.
    fn step(&mut self, most: usize, release: &mut dyn FnMut() -> bool) -> Result<Slice, Reason> {
        let Some(Reverse((key, t))) = self.frontier.pop() else {
            return Ok(Slice::Exhausted);
        };
        let parent = u32::MAX - key as u32;
        self.successor(parent, t)?;
        Ok(match self.take(parent, t, most, release)? {
            Some(firings) => Slice::Found(firings),
            None => Slice::Paused,
        })
    }

    /// Runs until the work reaches `until`, polling `stop` at every
    /// successor. Out of room, it fails with the reason, and the caller
    /// ends it (`give_back`).
    fn advance(
        &mut self,
        until: u64,
        most: usize,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Result<Slice, Reason> {
        if !self.started
            && let Some(firings) = self.start(most, &mut || false)?
        {
            return Ok(Slice::Found(firings));
        }
        while self.work < until {
            if stop(1) {
                return Err(Reason::Stopped);
            }
            match self.step(most, &mut || false)? {
                Slice::Paused => {}
                decided => return Ok(decided),
            }
        }
        Ok(Slice::Paused)
    }

    /// Takes the marking at hand, reached from `parent` by `transition`.
    ///
    /// Drops it if it is kept already or above a cap, and otherwise keeps
    /// and expands it. Returns the firing sequence when one of its
    /// successors is the target. Out of room, it calls `release` and tries
    /// once more if that gave memory back.
    fn take(
        &mut self,
        parent: u32,
        transition: u32,
        most: usize,
        release: &mut dyn FnMut() -> bool,
    ) -> Result<Option<Vec<u32>>, Reason> {
        self.write();
        let capped = !self.caps.is_empty()
            && self
                .marked
                .iter()
                .any(|&p| self.counts[p as usize] > self.caps[p as usize]);
        if capped || self.find().is_some() {
            self.repeated += 1;
        } else {
            let index = match self.keep(parent, transition, most) {
                Err(Reason::MemoryLimit { .. }) if release() => {
                    self.keep(parent, transition, most)?
                }
                kept => kept?,
            };
            let found = match self.expand(index) {
                Err(Reason::MemoryLimit { .. }) if release() => self.expand(index)?,
                expanded => expanded?,
            };
            if let Some(t) = found {
                let mut firings = self.path(index);
                firings.push(t);
                return Ok(Some(firings));
            }
        }
        self.clear();
        Ok(None)
    }

    /// Returns the distance of the marking at hand to the target: the
    /// tokens by which the two differ.
    fn distance(&self) -> u64 {
        let target = &self.program.target;
        let unmarked: u64 = self.target_total
            - self
                .marked
                .iter()
                .map(|&p| u64::from(target[p as usize]))
                .sum::<u64>();
        unmarked
            + self
                .marked
                .iter()
                .map(|&p| u64::from(self.counts[p as usize].abs_diff(target[p as usize])))
                .sum::<u64>()
    }

    /// Puts every successor of the marking at hand, kept as `index`, on
    /// the frontier.
    ///
    /// Returns a transition that reaches the target from it, if one does.
    /// Fails with the memory limit when the frontier has no room.
    fn expand(&mut self, index: u32) -> Result<Option<u32>, Reason> {
        let program = self.program;
        let distance = self.distance();
        let marked = std::mem::take(&mut self.marked);
        let candidates = self
            .free
            .iter()
            .copied()
            .chain(marked.iter().flat_map(|&p| {
                self.by_place
                    [self.starts[p as usize] as usize..self.starts[p as usize + 1] as usize]
                    .iter()
                    .copied()
            }));
        let counts = &self.counts;
        let mut successors = Vec::new();
        let mut examined = 0;
        for t in candidates {
            examined += 1;
            let transition = &program.transitions[t as usize];
            let inputs = program.inputs(transition);
            if inputs.iter().any(|&(p, w)| counts[p as usize] < w) {
                continue;
            }
            let outputs = program.outputs(transition);
            let d = moved(distance, inputs, outputs, counts, &program.target);
            if d == 0 {
                self.marked = marked;
                return Ok(Some(t));
            }
            // The key holds 32 bits of distance, so a distance of 2³² or
            // more ranks with the farthest.
            let rank = d.min(u64::from(u32::MAX));
            successors.push(Reverse((rank << 32 | u64::from(u32::MAX - index), t)));
        }
        self.work += examined;
        self.marked = marked;
        let before = self.frontier.capacity() * size_of::<Successor>();
        if self.frontier.capacity() - self.frontier.len() < successors.len() {
            let more = successors.len().max(self.frontier.capacity());
            let fits = more
                .checked_mul(size_of::<Successor>())
                .is_some_and(|bytes| self.charged.account().fits(bytes));
            if !fits {
                return Err(Reason::MemoryLimit {
                    limit_bytes: self.charged.account().limit(),
                });
            }
            self.frontier.reserve_exact(more);
        }
        self.charged
            .resize(before, self.frontier.capacity() * size_of::<Successor>());
        self.frontier.extend(successors);
        Ok(None)
    }

    /// Makes the marking at hand the marking kept as `parent`, with
    /// `transition` fired on it.
    ///
    /// Fails with [`Reason::IndexLimit`] when a count would pass
    /// `u32::MAX`.
    fn successor(&mut self, parent: u32, transition: u32) -> Result<(), Reason> {
        let start = match parent {
            0 => 0,
            _ => self.ends[parent as usize - 1] as usize,
        };
        let bytes = &self.bytes[start..self.ends[parent as usize] as usize];
        let (mut at, mut place) = (0, 0u32);
        while at < bytes.len() {
            place += get(bytes, &mut at);
            self.counts[place as usize] = get(bytes, &mut at) + 1;
            self.marked.push(place);
            place += 1;
        }
        let program = self.program;
        let transition = &program.transitions[transition as usize];
        for &(p, w) in program.inputs(transition) {
            self.counts[p as usize] -= w;
        }
        // The places marked before and those the transition marks, in
        // order. The places it empties are removed below.
        self.next.clear();
        let outputs = program.outputs(transition);
        let (mut i, mut o) = (0, 0);
        while i < self.marked.len() || o < outputs.len() {
            let p = match (self.marked.get(i), outputs.get(o)) {
                (Some(&a), Some(&(b, _))) if a == b => {
                    (i, o) = (i + 1, o + 1);
                    a
                }
                (Some(&a), Some(&(b, _))) if a < b => {
                    i += 1;
                    a
                }
                (Some(&a), None) => {
                    i += 1;
                    a
                }
                (_, Some(&(b, _))) => {
                    o += 1;
                    b
                }
                (None, None) => unreachable!("the loop's condition"),
            };
            self.next.push(p);
        }
        for &(p, w) in outputs {
            let Some(count) = self.counts[p as usize].checked_add(w) else {
                return Err(Reason::IndexLimit);
            };
            self.counts[p as usize] = count;
        }
        let counts = &self.counts;
        self.next.retain(|&p| counts[p as usize] > 0);
        std::mem::swap(&mut self.marked, &mut self.next);
        Ok(())
    }

    /// Writes the marking at hand into `written`.
    fn write(&mut self) {
        self.work += 1;
        self.written.clear();
        let mut from = 0;
        for &p in &self.marked {
            put(&mut self.written, p - from);
            put(&mut self.written, self.counts[p as usize] - 1);
            from = p + 1;
        }
    }

    /// Clears the marking at hand.
    fn clear(&mut self) {
        for &p in &self.marked {
            self.counts[p as usize] = 0;
        }
        self.marked.clear();
    }

    /// Returns the hash of the marking written.
    fn hash(&self) -> u64 {
        crate::hash::BuildHasher::default().hash_one(&self.written)
    }

    /// Returns the index of the marking written, if it is kept.
    fn find(&self) -> Option<u32> {
        if self.slots.is_empty() {
            return None;
        }
        let hash = self.hash();
        let mask = self.slots.len() - 1;
        let mut slot = hash as usize & mask;
        loop {
            let e = self.slots[slot].checked_sub(1)? as usize;
            let start = if e == 0 { 0 } else { self.ends[e - 1] as usize };
            if self.hashes[e] == hash && self.bytes[start..self.ends[e] as usize] == *self.written {
                return Some(e as u32);
            }
            slot = (slot + 1) & mask;
        }
    }

    /// Keeps the marking written, reached from `parent` by `transition`.
    /// Returns its index, or the reason it cannot be kept.
    fn keep(&mut self, parent: u32, transition: u32, most: usize) -> Result<u32, Reason> {
        let index = self.parents.len();
        if index >= most {
            return Err(Reason::IndexLimit);
        }
        let more = self.written.len();
        if (self.slots.len() / 2 <= index && !self.grow_table())
            || !room(&mut self.bytes, more, &mut self.charged)
            || !room(&mut self.ends, 1, &mut self.charged)
            || !room(&mut self.hashes, 1, &mut self.charged)
            || !self.parents.room(&mut self.charged)
        {
            return Err(Reason::MemoryLimit {
                limit_bytes: self.charged.account().limit(),
            });
        }
        let hash = self.hash();
        self.kept += 1;
        self.bytes.extend_from_slice(&self.written);
        self.ends.push(self.bytes.len() as u64);
        self.hashes.push(hash);
        self.parents.push(parent, transition);
        let index = index as u32;
        self.place(index, hash);
        Ok(index)
    }

    /// Puts a marking's index into the table.
    fn place(&mut self, index: u32, hash: u64) {
        let mask = self.slots.len() - 1;
        let mut slot = hash as usize & mask;
        while self.slots[slot] != 0 {
            slot = (slot + 1) & mask;
        }
        self.slots[slot] = index + 1;
    }

    /// Doubles the table, or returns false when the bound has no room for
    /// it.
    fn grow_table(&mut self) -> bool {
        let Some(size) = self.slots.len().checked_mul(2).map(|size| size.max(16)) else {
            return false;
        };
        let before = bytes_of(&self.slots);
        let fits = size
            .checked_mul(size_of::<u32>())
            .is_some_and(|bytes| self.charged.account().fits(bytes.saturating_sub(before)));
        if !fits {
            return false;
        }
        self.slots = vec![0; size];
        self.charged.resize(before, bytes_of(&self.slots));
        for index in 0..self.hashes.len() {
            self.place(index as u32, self.hashes[index]);
        }
        true
    }

    /// Returns the transitions fired from the initial marking to marking
    /// `j`.
    fn path(&self, j: u32) -> Vec<u32> {
        let mut firings = self.parents.up(j);
        firings.reverse();
        firings
    }
}

/// Returns the distance to the target after a transition with these arcs
/// fires on `counts`, whose distance was `distance`.
///
/// Both lists of arcs are sorted, so a place that both touch is counted
/// once.
fn moved(
    distance: u64,
    inputs: &[(u32, u32)],
    outputs: &[(u32, u32)],
    counts: &[u32],
    target: &[u32],
) -> u64 {
    let (mut i, mut o) = (0, 0);
    let mut distance = distance;
    while i < inputs.len() || o < outputs.len() {
        let (p, taken, given) = match (inputs.get(i), outputs.get(o)) {
            (Some(&(a, w)), Some(&(b, v))) if a == b => {
                (i, o) = (i + 1, o + 1);
                (a, w, v)
            }
            (Some(&(a, w)), Some(&(b, _))) if a < b => {
                i += 1;
                (a, w, 0)
            }
            (Some(&(a, w)), None) => {
                i += 1;
                (a, w, 0)
            }
            (_, Some(&(b, v))) => {
                o += 1;
                (b, 0, v)
            }
            (None, None) => unreachable!("the loop's condition"),
        };
        let (count, goal) = (u64::from(counts[p as usize]), u64::from(target[p as usize]));
        // In u64 a count past u32::MAX is still exact. The successor
        // refuses it when it is written out.
        let after = count - u64::from(taken) + u64::from(given);
        distance = distance - count.abs_diff(goal) + after.abs_diff(goal);
    }
    distance
}

/// Appends a number in LEB128: seven bits a byte, the lowest first, the
/// top bit set on every byte but the last.
fn put(bytes: &mut Vec<u8>, mut value: u32) {
    while value >= 0x80 {
        bytes.push(value as u8 | 0x80);
        value >>= 7;
    }
    bytes.push(value as u8);
}

/// Reads a number [`put`] wrote at `at`, and moves `at` past it.
fn get(bytes: &[u8], at: &mut usize) -> u32 {
    let (mut value, mut shift) = (0, 0);
    loop {
        let byte = bytes[*at];
        *at += 1;
        value |= u32::from(byte & 0x7f) << shift;
        if byte < 0x80 {
            return value;
        }
        shift += 7;
    }
}

/// Makes room in a buffer for `more` elements, doubling it, and charges
/// what it grew by. Returns false when the bound has no room.
pub(super) fn room<T>(buffer: &mut Vec<T>, more: usize, charged: &mut Charged<'_>) -> bool {
    if buffer.capacity() - buffer.len() >= more {
        return true;
    }
    let before = bytes_of(buffer);
    // On a 32-bit target these could pass `usize::MAX`. Such a buffer
    // cannot fit the bound either.
    let capacity = buffer
        .len()
        .checked_add(more)
        .zip(buffer.capacity().checked_mul(2))
        .map(|(needed, doubled)| needed.max(doubled).max(16));
    let fits = capacity
        .and_then(|capacity| capacity.checked_mul(size_of::<T>()))
        .is_some_and(|bytes| charged.account().fits(bytes.saturating_sub(before)));
    let (true, Some(capacity)) = (fits, capacity) else {
        return false;
    };
    buffer.reserve_exact(capacity - buffer.len());
    charged.resize(before, bytes_of(buffer));
    true
}
