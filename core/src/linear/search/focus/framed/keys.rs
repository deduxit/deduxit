// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The keys of a first-order search's stable sequents, which its memo and
//! loop check read.
//!
//! A member is written as its occurrence followed by the terms of its
//! frame, innermost first. Each term is written in prefix order, as the
//! bindings resolve it:
//! - a ground term by its id in the search's table of ground terms;
//! - an application holding a variable by its symbol, followed by its
//!   arguments;
//! - a metavariable or an eigenvariable by a number.
//!
//! The table of ground terms holds the arena's closed terms and every
//! ground term a binding built, each once. So equal ids are equal terms
//! however they were reached, and a witness built over many bindings is
//! one word, not its length. The occurrence fixes how many binders lie
//! above it, and a symbol its arity, so the words of a member end where
//! its last term does.
//!
//! **The instance's own key** names every variable as the search does,
//! with every metavariable's level beside it. Two stable sequents share it
//! exactly when they are the same instance: the same members over the same
//! unbound metavariables and eigenvariables. The loop check compares it. A
//! sequent equal to an ancestor whose metavariables are still unbound is
//! that ancestor, and a smallest proof of the ancestor never passes
//! through it. A failure cut by a bound is recorded under the own key, as
//! the propositional engine records one under the sequent's own key.
//!
//! **The canonical key** is the same sequent up to a renaming of its
//! variables. The members are sorted with every metavariable alike. The
//! metavariables are then numbered as they first appear, and the
//! eigenvariables ranked by number. Every metavariable's level is written
//! as how many of the sequent's eigenvariables lie below it. That is all
//! of its level a search of the sequent reads: a binding there only ever
//! meets the sequent's own eigenvariables and younger ones, which no
//! metavariable of the sequent may take. Two sequents with one canonical
//! key are one sequent up to a renaming that keeps the levels' order, so
//! a complete failure of either is one of the other.

use super::{FMember, Framed, Head, Members, NONE, Name, Named, Naming, Set, TRef, Undone, Var};
use crate::hash::HashMap;
use crate::linear::occurrences::{Forest, OccId};
use crate::linear::sequents::fo::FirstOrder;
use crate::linear::sequents::{FoTerm, Symbol};
use crate::memory::bytes_of;
use std::collections::hash_map::Entry;
use std::sync::atomic::Ordering;

/// The bits of a term's word that say what it is.
const TAG: u32 = 3 << 30;
/// A ground term, by its id in the table of ground terms.
const GROUND: u32 = 0;
/// An application, by its symbol; its arguments follow.
const APP: u32 = 1 << 30;
/// An eigenvariable, by its number or rank.
const EIGEN: u32 = 2 << 30;
/// A metavariable, by its index or number.
const META: u32 = 3 << 30;
/// The bits of a term's word that hold its value. A value past them is
/// refused: such a sequent is decided without a key.
const VALUE: u32 = !TAG;

/// The most words the members of a key may take. A sequent whose terms,
/// written out, pass it is decided without the memo and the loop check,
/// which would spend more on its key than on most searches. A witness
/// shared many times over is written as often as it is reached.
const KEY_WORDS: usize = 1 << 16;

/// The buffers the keys are written with, kept between stable sequents.
#[derive(Debug, Default)]
pub(super) struct KeyScratch {
    /// The members' words, one member after another.
    words: Vec<u32>,
    /// Per member, where its words start, `Γ`'s first.
    starts: Vec<usize>,
    /// Per member, how many copies of it the zone holds.
    copies: Vec<u32>,
    /// The members' order in a key.
    order: Vec<usize>,
    /// The eigenvariables' numbers, sorted, each once.
    eigens: Vec<u32>,
    /// The metavariables by index, in the order a key names them.
    metas: Vec<u32>,
    /// The number of each metavariable in the canonical key.
    numbers: HashMap<u32, u32>,
    /// The terms to write.
    walk: Vec<TRef>,
}

impl KeyScratch {
    /// Returns the bytes its buffers hold, a hash table's as its slots with a
    /// control byte each.
    pub(super) fn bytes(&self) -> usize {
        bytes_of(&self.words)
            + bytes_of(&self.starts)
            + bytes_of(&self.copies)
            + bytes_of(&self.order)
            + bytes_of(&self.eigens)
            + bytes_of(&self.metas)
            + bytes_of(&self.walk)
            + self.numbers.capacity() * (size_of::<(u32, u32)>() + 1)
    }

    /// Where the words of member `i` lie.
    fn range(&self, i: usize) -> std::ops::Range<usize> {
        let end = self.starts.get(i + 1).copied().unwrap_or(self.words.len());
        self.starts[i]..end
    }

    /// The words of member `i`.
    fn member(&self, i: usize) -> &[u32] {
        &self.words[self.range(i)]
    }

    /// Orders members `x` and `y` as the canonical key does: by their
    /// words, a metavariable as any other and an eigenvariable by its
    /// rank.
    fn abstract_order(&self, x: usize, y: usize) -> std::cmp::Ordering {
        let (x, y) = (self.member(x), self.member(y));
        x[0].cmp(&y[0]).then_with(|| {
            x[1..]
                .iter()
                .map(|&w| self.abstracted(w))
                .cmp(y[1..].iter().map(|&w| self.abstracted(w)))
        })
    }

    /// The word as the canonical key's order reads it: a metavariable as
    /// any other, an eigenvariable by its rank.
    fn abstracted(&self, word: u32) -> u32 {
        match word & TAG {
            META => META,
            EIGEN => EIGEN | self.rank(word & VALUE),
            _ => word,
        }
    }

    /// The rank of an eigenvariable among the sequent's.
    fn rank(&self, number: u32) -> u32 {
        // Fewer eigenvariables than words.
        self.eigens.partition_point(|&n| n < number) as u32
    }
}

impl Framed {
    /// Writes the keys of the stable sequent `⊢ Θ ; Γ`: its own and its
    /// canonical one, as the comment above [`KeyScratch`] says. Returns false
    /// where a variable, a term or a symbol is too large for a word's value,
    /// or the words pass [`KEY_WORDS`]. The sequent is then decided without a
    /// key.
    pub(super) fn write_keys(
        &mut self,
        forest: &Forest,
        theta: &Set,
        gamma: &Members,
        own: &mut Vec<OccId>,
        canonical: &mut Vec<OccId>,
    ) -> bool {
        let fo = &forest.sequent().fo;
        let mut s = std::mem::take(&mut self.key);
        s.words.clear();
        s.starts.clear();
        s.copies.clear();
        s.eigens.clear();
        // A member present more than once is written once, with its number of
        // copies. A clause copied a hundred times leaves a hundred copies of one
        // instance, which written out would make every key as long.
        let written = gamma
            .0
            .chunk_by(|x, y| x == y)
            .chain(theta.0.chunks(1))
            .all(|copies| {
                u32::try_from(copies.len()).is_ok_and(|n| {
                    s.copies.push(n);
                    self.write_member(fo, copies[0], &mut s)
                })
            });
        if written {
            // Writing the keys reads every word once per key and sorts the
            // members: counted as the terms a walk visits.
            let sorting = usize::BITS - s.starts.len().leading_zeros();
            self.steps.fetch_add(
                (s.words.len() as u64).saturating_mul(2 * u64::from(1 + sorting)),
                Ordering::Relaxed,
            );
            s.eigens.sort_unstable();
            s.eigens.dedup();
            let n = gamma.0.chunk_by(|x, y| x == y).count();
            self.own_key(&mut s, n, own);
            self.canonical_key(&mut s, n, canonical);
        }
        self.key = s;
        written
    }

    /// Writes a member's words, collecting its eigenvariables.
    fn write_member(&mut self, fo: &FirstOrder, m: FMember, s: &mut KeyScratch) -> bool {
        let (o, mut frame) = self.instance(m);
        s.starts.push(s.words.len());
        s.words.push(o.get());
        while frame != NONE {
            let (parent, term) = self.frames[frame as usize];
            s.walk.clear();
            s.walk.push(term);
            while let Some(t) = s.walk.pop() {
                self.step();
                if s.words.len() >= KEY_WORDS {
                    return false;
                }
                let (tag, value) = match self.name(fo, t) {
                    Name::Ground(id) => (GROUND, id),
                    Name::Past => return false,
                    Name::Open(_) => {
                        let t = self.deref(fo, t);
                        if let Some(v) = self.var(t) {
                            match self.vars[v as usize] {
                                Var::Meta { .. } => (META, v),
                                Var::Eigen(number) => {
                                    s.eigens.push(number);
                                    (EIGEN, number)
                                }
                            }
                        } else {
                            let (symbol, args) = Self::app(fo, t);
                            s.walk.extend(args.iter().rev().map(|a| TRef {
                                id: a.get(),
                                frame: t.frame,
                            }));
                            (APP, symbol.get())
                        }
                    }
                };
                if value > VALUE {
                    return false;
                }
                s.words.push(tag | value);
            }
            frame = parent;
        }
        true
    }

    /// Returns the name of the term `t` stands for: its id where it is
    /// ground, a variable it holds where it is not, or `Name::Past` where the
    /// id is too large for a word.
    ///
    /// Every bound metavariable the walk passes keeps its name, on the trail.
    /// So a key names a witness built over many bindings in one step. A name
    /// rests only on bindings made before it, which an undo takes back only
    /// after the name. An open name is asked again once its variable is
    /// bound.
    fn name(&mut self, fo: &FirstOrder, t: TRef) -> Name {
        // A frame's term is a variable: an unbound one, an eigenvariable
        // or a kept name needs no walk.
        if let Some(v) = self.var(t) {
            match self.vars[v as usize] {
                Var::Meta {
                    binding: Some(_), ..
                } => {
                    if let Some(name) = self.kept(v) {
                        return name;
                    }
                }
                _ => return Name::Open(v),
            }
        }
        let mut steps = std::mem::take(&mut self.naming);
        let mut names = std::mem::take(&mut self.names);
        steps.clear();
        names.clear();
        steps.push(Naming::Term(t));
        while let Some(step) = steps.pop() {
            self.step();
            match step {
                Naming::Term(mut t) => {
                    while t.id < self.base
                        && let FoTerm::Bound(i) = fo.terms[t.id as usize]
                    {
                        t = self.frame_term(t.frame, i);
                    }
                    if let Some(v) = self.var(t) {
                        steps.push(Naming::Var(v));
                    } else if self.closed[t.id as usize] {
                        names.push(match self.arena[t.id as usize] {
                            NONE => Name::Past,
                            id => Name::Ground(id),
                        });
                    } else {
                        let (symbol, args) = Self::app(fo, t);
                        // An arity is below a sequent's length.
                        steps.push(Naming::Apply(symbol, args.len() as u32));
                        steps.extend(args.iter().rev().map(|a| {
                            Naming::Term(TRef {
                                id: a.get(),
                                frame: t.frame,
                            })
                        }));
                    }
                }
                Naming::Var(v) => match self.vars[v as usize] {
                    Var::Meta {
                        binding: Some(b), ..
                    } => match self.kept(v) {
                        Some(name) => names.push(name),
                        None => {
                            steps.push(Naming::Keep(v));
                            steps.push(Naming::Term(b));
                        }
                    },
                    _ => names.push(Name::Open(v)),
                },
                Naming::Apply(symbol, arity) => {
                    let at = names.len() - arity as usize;
                    let name = self.ground.apply(symbol, &names[at..]);
                    names.truncate(at);
                    names.push(name);
                }
                Naming::Keep(v) => {
                    let kept = match names.last() {
                        Some(&Name::Ground(id)) => Named::Ground(id),
                        Some(&Name::Open(w)) => Named::Open(w),
                        _ => continue,
                    };
                    self.named[v as usize] = kept;
                    self.trail.push(Undone::Named(v));
                }
            }
        }
        let name = names.pop().expect("a walk names its term");
        self.naming = steps;
        self.names = names;
        name
    }

    /// The name the bound metavariable `v` keeps, where it still holds.
    fn kept(&self, v: u32) -> Option<Name> {
        match self.named[v as usize] {
            Named::Unknown => None,
            Named::Ground(id) => Some(Name::Ground(id)),
            Named::Open(w) => match self.vars[w as usize] {
                Var::Meta {
                    binding: Some(_), ..
                } => None,
                _ => Some(Name::Open(w)),
            },
        }
    }

    /// Returns the name of the application of `symbol` to terms of the names
    /// `args`. Where all are ground, it is the id of the term in `ground`, a
    /// new one for a new term. The table takes the arguments one at a time,
    /// and a constant as applied to [`NONE`]. Otherwise it is the first open
    /// name, or past.
    pub(super) fn apply(
        ground: &mut HashMap<(Head, u32), u32>,
        symbol: Symbol,
        args: &[Name],
    ) -> Name {
        let mut past = false;
        for &arg in args {
            match arg {
                Name::Open(v) => return Name::Open(v),
                Name::Past => past = true,
                Name::Ground(_) => {}
            }
        }
        if past {
            return Name::Past;
        }
        let ids = args.iter().map(|&arg| match arg {
            Name::Ground(id) => id,
            Name::Open(_) | Name::Past => unreachable!("every argument is ground"),
        });
        let mut head = Head::Symbol(symbol.get());
        let mut id = NONE;
        for arg in ids.chain(args.is_empty().then_some(NONE)) {
            let next = u32::try_from(ground.len())
                .ok()
                .filter(|&next| next <= VALUE);
            id = match (ground.entry((head, arg)), next) {
                (Entry::Occupied(entry), _) => *entry.get(),
                (Entry::Vacant(entry), Some(next)) => *entry.insert(next),
                (Entry::Vacant(_), None) => return Name::Past,
            };
            head = Head::Applied(id);
        }
        Name::Ground(id)
    }

    /// The level of a metavariable, by index.
    fn meta_level(&self, v: u32) -> u32 {
        match self.vars[v as usize] {
            Var::Meta { level, .. } => level,
            Var::Eigen(_) => unreachable!("a metavariable's word names a metavariable"),
        }
    }

    /// Writes the instance's own key into `key`. It holds the members of `Γ`
    /// (the first `n`, each once) and of `Θ`, each zone's sorted by their
    /// words. Each member comes after its number of copies, and each zone
    /// after its number of members. Then comes the level of every
    /// metavariable, in the order they appear.
    fn own_key(&self, s: &mut KeyScratch, n: usize, key: &mut Vec<OccId>) {
        let total = s.starts.len();
        s.order.clear();
        s.order.extend(0..total);
        self.write_key::<false>(s, n, key);
    }

    /// Writes the canonical key into `key`, over the order the own key left
    /// in `s`. It holds the members of `Γ` (the first `n`, each once, after
    /// its number of copies) and of `Θ`. Each zone's members are sorted with
    /// every metavariable alike and every eigenvariable by its rank. The
    /// metavariables are numbered as they first appear in that order. Then
    /// comes, per metavariable, how many of the sequent's eigenvariables lie
    /// below its level.
    fn canonical_key(&self, s: &mut KeyScratch, n: usize, key: &mut Vec<OccId>) {
        self.write_key::<true>(s, n, key);
    }

    /// Writes the own key, or the canonical one where `CANONICAL` is set,
    /// as [`own_key`](Self::own_key) and
    /// [`canonical_key`](Self::canonical_key) describe them; a constant, so
    /// that each is a loop of its own.
    fn write_key<const CANONICAL: bool>(&self, s: &mut KeyScratch, n: usize, key: &mut Vec<OccId>) {
        let mut order = std::mem::take(&mut s.order);
        if CANONICAL {
            order[..n].sort_by(|&x, &y| s.abstract_order(x, y));
            order[n..].sort_by(|&x, &y| s.abstract_order(x, y));
        } else {
            order[..n].sort_by(|&x, &y| s.member(x).cmp(s.member(y)));
            order[n..].sort_by(|&x, &y| s.member(x).cmp(s.member(y)));
        }
        s.metas.clear();
        s.numbers.clear();
        key.clear();
        for members in [&order[..n], &order[n..]] {
            // A zone's distinct members are fewer than a word counts: each
            // is a member id, a `u32`.
            key.push(OccId::new(members.len() as u32));
            for &i in members {
                let range = s.range(i);
                key.push(OccId::new(s.copies[i]));
                key.push(OccId::new(s.words[range.start]));
                for j in range.start + 1..range.end {
                    let w = s.words[j];
                    let word = if CANONICAL {
                        match w & TAG {
                            META => {
                                let v = w & VALUE;
                                // Fewer metavariables than words.
                                let next = s.metas.len() as u32;
                                let number = *s.numbers.entry(v).or_insert(next);
                                if number == next {
                                    s.metas.push(v);
                                }
                                META | number
                            }
                            EIGEN => EIGEN | s.rank(w & VALUE),
                            _ => w,
                        }
                    } else {
                        if w & TAG == META && s.numbers.insert(w & VALUE, 0).is_none() {
                            s.metas.push(w & VALUE);
                        }
                        w
                    };
                    key.push(OccId::new(word));
                }
            }
        }
        if CANONICAL {
            key.extend(
                s.metas
                    .iter()
                    .map(|&v| OccId::new(s.rank(self.meta_level(v)))),
            );
        } else {
            key.extend(s.metas.iter().map(|&v| OccId::new(self.meta_level(v))));
        }
        s.order = order;
    }
}
