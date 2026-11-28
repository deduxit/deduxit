// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The instance table of a first-order proof: what the proof's members
//! past the forest's length stand for.
//!
//! An occurrence under binders is a formula with loose bound variables.
//! A sequent inside the proof holds such an occurrence together with a
//! *frame*: the terms its binders were opened with. The table gives each
//! pair of an occurrence and a frame its own member number.
//!
//! A frame holds closed terms of the sequent, or terms of the proof's
//! own. These extend the sequent's term arena past its end: the
//! witnesses of `∃` steps and the eigenvariables of `∀` steps.

use crate::hash::HashMap;
use crate::linear::occurrences::{Forest, Member, OccId};
use crate::linear::sequents::fo::FirstOrder;
use crate::linear::sequents::{Args, Eigen, FoTerm, FoTermId, Symbol};
use std::fmt::{Result as FmtResult, Write};

/// The index of a frame in a proof's instance table. A frame is a term
/// pushed onto a parent frame, so the innermost binder's term comes
/// first. Frames are linked lists, so the members under nested binders
/// share the tails of their frames.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FrameId(u32);

impl FrameId {
    /// Wraps a raw table index.
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    /// Returns the raw table index.
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Returns the index as a `usize`, for indexing the table.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The instance table of a first-order proof: what the proof's members
/// past its forest's length stand for.
///
/// It holds:
///
/// - each member: an occurrence under `n` binders, with a frame of `n`
///   terms, the innermost binder's first;
/// - the frames, each a term pushed onto a parent frame;
/// - the proof's own terms: the witnesses and eigenvariables the frames
///   hold, numbered on from the sequent's own terms and argument lists.
///
/// Build a table for one forest with [`new`](Self::new), then hand it to
/// [`Proof::with_instances`](crate::linear::Proof::with_instances). That
/// checks it and makes it canonical: equal terms, frames and members
/// become one entry each, so the checker can compare instances by their
/// numbers.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::{Eigen, Forest, Instances, Member, Mode, Node, NodeId, OccId, Proof, Sequent};
///
/// // ⊢ ∀x. (~p(x) ⅋ p(x)): occurrence 0 is the binder, 1 the `⅋`, 2 and
/// // 3 the literals.
/// let sequent: Sequent = "|- forall x. p(x) -o p(x)".parse()?;
/// let forest = Forest::new(&sequent)?;
/// let mut instances = Instances::new(&forest);
/// let a = instances.eigen(Eigen::new(0));
/// let frame = instances.frame(None, a);
/// let [par, l, r] = [1, 2, 3].map(|o| instances.member(OccId::new(o), frame));
/// let nodes = vec![
///     Node::Ax(l, r),
///     Node::Par(par, NodeId::new(0)),
///     Node::Forall(Member::new(0), Eigen::new(0), NodeId::new(1)),
/// ];
/// let proof = Proof::with_instances(forest, instances, nodes, NodeId::new(2))?;
/// proof.check(Mode::CLASSICAL)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Instances {
    /// The forest's length, which is the first member the table numbers.
    pub(crate) occurrences: u32,
    /// The number of terms in the sequent's arena, which is the id of the
    /// extension's first term.
    pub(crate) base_terms: u32,
    /// The number of argument lists of the sequent, which is the id of the
    /// extension's first list.
    pub(crate) base_lists: u32,
    /// Per member past the forest, its occurrence and frame.
    pub(crate) members: Vec<(OccId, FrameId)>,
    /// Per frame, its parent and the term it pushes.
    pub(crate) frames: Vec<(Option<FrameId>, FoTermId)>,
    /// The extension of the term arena, each term referring only to terms
    /// before it. In a canonical table every term is closed.
    pub(crate) terms: Vec<FoTerm>,
    /// The arguments of every list of the extension, one after the other.
    pub(crate) args: Vec<FoTermId>,
    /// The extension's argument lists, each a start and a length into
    /// `args`.
    pub(crate) lists: Vec<(u32, u32)>,
}

impl Instances {
    /// Returns the empty table of a proof over `forest`.
    pub fn new(forest: &Forest) -> Self {
        let fo = &forest.sequent().fo;
        Self {
            // These fit a `u32`: a forest has fewer than 2³² occurrences,
            // and a sequent's arenas have no more entries than its text or
            // document.
            occurrences: forest.len() as u32,
            base_terms: fo.terms.len() as u32,
            base_lists: fo.lists.len() as u32,
            ..Self::default()
        }
    }

    /// Adds the term of the eigenvariable `a` to the table and returns its
    /// id.
    pub fn eigen(&mut self, a: Eigen) -> FoTermId {
        self.push_term(FoTerm::Eigen(a))
    }

    /// Adds the term `symbol(args…)` and its argument list to the table
    /// and returns its id. `args` are terms of the sequent or of the
    /// table.
    pub fn apply(&mut self, symbol: Symbol, args: &[FoTermId]) -> FoTermId {
        // The ids saturate. A table with more ids than a `u32` counts is
        // refused when a proof takes it (`canonical`), before any id is
        // read.
        let start = saturated(self.args.len());
        self.args.extend_from_slice(args);
        self.lists.push((start, saturated(args.len())));
        let list = Args::new(
            self.base_lists
                .saturating_add(saturated(self.lists.len() - 1)),
        );
        self.push_term(FoTerm::App(symbol, list))
    }

    /// Adds the frame that pushes `term` onto `parent` and returns it.
    /// `parent` is the frame of the binders further out, or `None` for an
    /// outermost binder.
    pub fn frame(&mut self, parent: Option<FrameId>, term: FoTermId) -> FrameId {
        self.frames.push((parent, term));
        FrameId::new(saturated(self.frames.len() - 1))
    }

    /// Adds the member that stands for `occurrence` under `frame` and
    /// returns it. `frame` must hold a term for every binder above the
    /// occurrence.
    pub fn member(&mut self, occurrence: OccId, frame: FrameId) -> Member {
        self.members.push((occurrence, frame));
        Member::new(
            self.occurrences
                .saturating_add(saturated(self.members.len() - 1)),
        )
    }

    /// Returns the number of members the table numbers.
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns whether the table numbers no member.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Returns the occurrence and frame `member` stands for, or `None` for
    /// a member that is an occurrence of the forest or that the table does
    /// not number.
    pub fn instance(&self, member: Member) -> Option<(OccId, FrameId)> {
        let at = member.get().checked_sub(self.occurrences)?;
        self.members.get(at as usize).copied()
    }

    /// Returns the parent frame and the term of `frame`.
    ///
    /// # Panics
    ///
    /// For a frame the table does not hold.
    pub fn frame_parts(&self, frame: FrameId) -> (Option<FrameId>, FoTermId) {
        self.frames[frame.index()]
    }

    /// Returns the terms of `frame`, the innermost binder's first.
    ///
    /// # Panics
    ///
    /// For a frame the table does not hold.
    pub fn frame_terms(&self, frame: FrameId) -> Vec<FoTermId> {
        let mut terms = Vec::new();
        let mut next = Some(frame);
        while let Some(f) = next {
            let (parent, term) = self.frames[f.index()];
            terms.push(term);
            next = parent;
        }
        terms
    }

    /// Returns the table's own terms, whose ids follow the sequent's.
    pub fn terms(&self) -> &[FoTerm] {
        &self.terms
    }

    /// Returns the id of the table's first term, which is the number of
    /// the sequent's own terms.
    pub fn first_term(&self) -> FoTermId {
        FoTermId::new(self.base_terms)
    }

    /// Returns the eigenvariables among the table's terms, ascending and
    /// each once.
    pub fn eigenvariables(&self) -> Vec<Eigen> {
        let mut eigens: Vec<Eigen> = self
            .terms
            .iter()
            .filter_map(|t| match *t {
                FoTerm::Eigen(a) => Some(a),
                FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
            })
            .collect();
        eigens.sort_unstable();
        eigens.dedup();
        eigens
    }

    /// Adds `term` to the table and returns its id.
    pub(crate) fn push_term(&mut self, term: FoTerm) -> FoTermId {
        self.terms.push(term);
        FoTermId::new(
            self.base_terms
                .saturating_add(saturated(self.terms.len() - 1)),
        )
    }
}

/// Returns `n` as a `u32`, or `u32::MAX` where `n` is larger.
fn saturated(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// The terms of a proof: its sequent's arena and the proof's extension of
/// it, under one range of ids.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Terms<'a> {
    /// The sequent's tables.
    pub(crate) fo: &'a FirstOrder,
    /// The proof's table.
    pub(crate) table: &'a Instances,
}

impl Terms<'_> {
    /// Returns the number of terms of both arenas.
    pub(crate) fn len(&self) -> usize {
        self.fo.terms.len() + self.table.terms.len()
    }

    /// Returns the term `id`.
    pub(crate) fn term(&self, id: FoTermId) -> FoTerm {
        match id.index().checked_sub(self.fo.terms.len()) {
            None => self.fo.terms[id.index()],
            Some(at) => self.table.terms[at],
        }
    }

    /// Returns the terms of the argument list `args`.
    pub(crate) fn list(&self, args: Args) -> &[FoTermId] {
        match args.index().checked_sub(self.fo.lists.len()) {
            None => self.fo.list(args),
            Some(at) => {
                let (start, len) = self.table.lists[at];
                &self.table.args[start as usize..(start + len) as usize]
            }
        }
    }

    /// Returns the number of argument lists of both tables.
    pub(crate) fn lists(&self) -> usize {
        self.fo.lists.len() + self.table.lists.len()
    }
}

/// The key [`Hashcons`] numbers a term by: its tag and its children's
/// numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Key {
    /// A bound variable.
    Bound(u32),
    /// A symbol applied to the argument list of that number.
    App(Symbol, u64),
    /// An eigenvariable.
    Eigen(Eigen),
}

/// Numbers first-order terms and argument lists so that equal ones get the
/// same number. A term is numbered by its symbol and its arguments'
/// numbers, and a list by its items' numbers.
///
/// A number is the count of distinct terms or lists numbered before it,
/// each held in a table. So a `u64` cannot overflow: no memory holds 2⁶⁴
/// entries.
#[derive(Default)]
pub(crate) struct Hashcons {
    /// Every term numbered, by its key.
    terms: HashMap<Key, u64>,
    /// Every list numbered, by its items.
    lists: HashMap<Box<[u64]>, u64>,
}

impl Hashcons {
    /// Returns the number of a term and whether it is new.
    fn term(&mut self, key: Key) -> (u64, bool) {
        let next = self.terms.len() as u64;
        match self.terms.entry(key) {
            std::collections::hash_map::Entry::Occupied(e) => (*e.get(), false),
            std::collections::hash_map::Entry::Vacant(e) => (*e.insert(next), true),
        }
    }

    /// Returns the number of a list of term numbers and whether it is new.
    fn list(&mut self, items: &[u64]) -> (u64, bool) {
        if let Some(&n) = self.lists.get(items) {
            return (n, false);
        }
        let n = self.lists.len() as u64;
        self.lists.insert(items.into(), n);
        (n, true)
    }
}

/// The numbers of a proof's terms, and the means to number the instances
/// of its atoms. The checker compares an axiom's two literals by these
/// numbers.
pub(crate) struct Numbering<'a> {
    /// The proof's terms.
    terms: Terms<'a>,
    /// The hash-consing.
    cons: Hashcons,
    /// Per term of both arenas, its number.
    numbers: Vec<u64>,
    /// Per term, one more than its largest loose bound index, or `0` for a
    /// closed term.
    loose: Vec<u32>,
    /// Per argument list, its number once computed.
    list_numbers: Vec<Option<u64>>,
}

impl<'a> Numbering<'a> {
    /// Numbers every term of both arenas, in one pass, since every term
    /// comes after its arguments.
    pub(crate) fn new(terms: Terms<'a>) -> Self {
        let mut this = Self {
            terms,
            cons: Hashcons::default(),
            numbers: Vec::with_capacity(terms.len()),
            loose: Vec::with_capacity(terms.len()),
            list_numbers: vec![None; terms.lists()],
        };
        for id in 0..terms.len() {
            let id = FoTermId::new(id as u32);
            let (key, loose) = match terms.term(id) {
                FoTerm::Bound(i) => (Key::Bound(i), i.saturating_add(1)),
                FoTerm::Eigen(a) => (Key::Eigen(a), 0),
                // `canonical` refuses a table with a metavariable before
                // it numbers the terms.
                FoTerm::Meta(_) => unreachable!("a proof's table holds no metavariable"),
                FoTerm::App(f, args) => {
                    let loose = terms
                        .list(args)
                        .iter()
                        .map(|t| this.loose[t.index()])
                        .max()
                        .unwrap_or(0);
                    (Key::App(f, this.list_number(args)), loose)
                }
            };
            this.numbers.push(this.cons.term(key).0);
            this.loose.push(loose);
        }
        this
    }

    /// Returns the number of the argument list `args`, whose items must
    /// already be numbered.
    pub(crate) fn list_number(&mut self, args: Args) -> u64 {
        if let Some(n) = self.list_numbers[args.index()] {
            return n;
        }
        let items: Vec<u64> = self
            .terms
            .list(args)
            .iter()
            .map(|t| self.numbers[t.index()])
            .collect();
        let n = self.cons.list(&items).0;
        self.list_numbers[args.index()] = Some(n);
        n
    }

    /// Returns the terms numbered.
    pub(crate) fn terms(&self) -> Terms<'a> {
        self.terms
    }

    /// Returns the number of the eigenvariable `a`'s term, numbering it
    /// where no term of the arenas is it.
    pub(crate) fn eigen(&mut self, a: Eigen) -> u64 {
        self.cons.term(Key::Eigen(a)).0
    }

    /// Returns the number of the term `id`.
    pub(crate) fn number(&self, id: FoTermId) -> u64 {
        self.numbers[id.index()]
    }

    /// Returns whether the term `id` is closed: it has no loose bound
    /// variable.
    pub(crate) fn is_closed(&self, id: FoTermId) -> bool {
        self.loose[id.index()] == 0
    }

    /// Returns the number of the argument list `args` with every loose
    /// bound variable `i` replaced by the term numbered `frame[i]`: the
    /// arguments of an atom's instance. Each shared subterm is visited
    /// once, on an explicit stack, so deep terms cannot overflow the call
    /// stack.
    pub(crate) fn instance(&mut self, args: Args, frame: &[u64]) -> u64 {
        let mut memo: HashMap<FoTermId, u64> = HashMap::default();
        let terms = self.terms;
        let list = terms.list(args);
        let mut items = Vec::with_capacity(list.len());
        for &t in list {
            items.push(self.substitute(t, frame, &mut memo));
        }
        self.cons.list(&items).0
    }

    /// Returns the number of the term `t` under `frame`, as
    /// [`instance`](Self::instance) takes its arguments.
    fn substitute(&mut self, t: FoTermId, frame: &[u64], memo: &mut HashMap<FoTermId, u64>) -> u64 {
        if self.loose[t.index()] == 0 {
            return self.numbers[t.index()];
        }
        let mut stack = vec![(t, false)];
        while let Some((u, expanded)) = stack.pop() {
            if memo.contains_key(&u) {
                continue;
            }
            match self.terms.term(u) {
                // The sequent's check keeps every bound index of an atom
                // below the number of its occurrence's binders, and the
                // frame has a term for each binder.
                FoTerm::Bound(i) => {
                    memo.insert(u, frame[i as usize]);
                }
                FoTerm::App(f, args) if expanded => {
                    let items: Vec<u64> = self
                        .terms
                        .list(args)
                        .iter()
                        .map(|&c| {
                            if self.loose[c.index()] == 0 {
                                self.numbers[c.index()]
                            } else {
                                memo[&c]
                            }
                        })
                        .collect();
                    let list = self.cons.list(&items).0;
                    memo.insert(u, self.cons.term(Key::App(f, list)).0);
                }
                FoTerm::App(_, args) => {
                    stack.push((u, true));
                    for &c in self.terms.list(args) {
                        if self.loose[c.index()] > 0 && !memo.contains_key(&c) {
                            stack.push((c, false));
                        }
                    }
                }
                // An eigenvariable is closed, so the check at the top
                // returned its number.
                FoTerm::Eigen(_) => unreachable!("an eigenvariable is a closed term"),
                // `canonical` refuses a table with a metavariable before
                // it numbers the terms.
                FoTerm::Meta(_) => unreachable!("a proof's table holds no metavariable"),
            }
        }
        memo[&t]
    }
}

/// Returns, per occurrence of `forest`, how many binders stand above it.
pub(crate) fn binders(forest: &Forest) -> Vec<u32> {
    let mut depth = vec![0u32; forest.len()];
    for o in forest.ids() {
        if let Some(p) = forest.parent(o) {
            // At most one binder per ancestor, so the depth stays below the
            // forest's length and cannot overflow.
            depth[o.index()] = depth[p.index()] + u32::from(forest.kind(p).is_binder());
        }
    }
    depth
}

/// Returns the names the eigenvariables `eigens` are written with: `a`,
/// `b`, … `z`, then `a_1`, `b_1`, …. Names of the sequent's symbols are
/// skipped, so a sequent of a derivation reads as a sequent with those
/// eigenvariables as new constants.
pub(crate) fn eigen_names(fo: &FirstOrder, eigens: &[Eigen]) -> HashMap<Eigen, String> {
    let taken: crate::hash::HashSet<&str> =
        fo.symbols.iter().map(|(s, _)| s.as_str()).collect();
    let mut names = HashMap::default();
    let mut candidates = (0u64..).map(|n| {
        let letter = char::from(b'a' + (n % 26) as u8);
        match n / 26 {
            0 => letter.to_string(),
            k => format!("{letter}_{k}"),
        }
    });
    for &a in eigens {
        let name = candidates
            .by_ref()
            .find(|c| !taken.contains(c.as_str()))
            .expect("the candidates never end");
        names.insert(a, name);
    }
    names
}

/// Returns the name an open witness is written with: the metavariable `k`
/// of an interactive session, written `$1` for the first. No reader of
/// either syntax accepts a `$`, so the name cannot be mistaken for a
/// formula, term or symbol, or for a renamed variable such as `x_1`. A
/// `?` would not do: `?1` reads as the formula `?(1)`.
pub(crate) fn meta_name(k: u32) -> String {
    format!("${}", u64::from(k) + 1)
}

/// What a printer needs to write a proof's instances: the table, the
/// terms of both arenas and the eigenvariables' names.
#[derive(Clone, Debug)]
pub(crate) struct Framing<'a> {
    /// The proof's table.
    pub(crate) table: &'a Instances,
    /// The terms of both arenas.
    terms: Terms<'a>,
    /// The name of every eigenvariable the table holds.
    names: HashMap<Eigen, String>,
}

impl<'a> Framing<'a> {
    /// Returns the framing of `table`, a proof's table over a sequent
    /// whose first-order tables are `fo`.
    pub(crate) fn new(fo: &'a FirstOrder, table: &'a Instances) -> Self {
        Self {
            table,
            terms: Terms { fo, table },
            names: eigen_names(fo, &table.eigenvariables()),
        }
    }

    /// Returns the occurrence `m` stands for.
    pub(crate) fn occurrence(&self, m: Member) -> OccId {
        self.table.instance(m).map_or(m.occ(), |(o, _)| o)
    }

    /// Returns the terms of both arenas.
    #[cfg(feature = "rocq")]
    pub(crate) const fn terms(&self) -> Terms<'a> {
        self.terms
    }

    /// Returns the terms of the argument list `args` of either arena.
    #[cfg(feature = "rocq")]
    pub(crate) fn list(&self, args: Args) -> &[FoTermId] {
        self.terms.list(args)
    }

    /// Returns the eigenvariables' names.
    pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
        self.names.values().map(String::as_str)
    }

    /// Returns the name eigenvariable `a` is written with, or `?` for one
    /// the table does not hold.
    pub(crate) fn name(&self, a: Eigen) -> &str {
        self.names.get(&a).map_or("?", String::as_str)
    }

    /// Writes the closed term `term` as `f(t, …)`, writing every symbol's
    /// and eigenvariable's name through `atom`. It uses an explicit stack,
    /// so deep terms cannot overflow the call stack.
    pub(crate) fn write_term<W: Write + ?Sized>(
        &self,
        out: &mut W,
        term: FoTermId,
        atom: &mut impl FnMut(&mut W, &str) -> FmtResult,
    ) -> FmtResult {
        let mut open: Vec<(Args, u32)> = Vec::new();
        let mut next = Some(term);
        loop {
            if let Some(t) = next.take() {
                match self.terms.term(t) {
                    FoTerm::App(f, args) => {
                        atom(out, &self.terms.fo.symbols[f.index()].0)?;
                        if !self.terms.list(args).is_empty() {
                            out.write_char('(')?;
                            open.push((args, 0));
                        }
                    }
                    FoTerm::Eigen(a) => atom(out, self.name(a))?,
                    FoTerm::Meta(k) => atom(out, &meta_name(k))?,
                    // Frames and witnesses hold closed terms, so this is
                    // only a fallback.
                    FoTerm::Bound(index) => write!(out, "#{index}")?,
                }
            }
            let Some(&mut (args, ref mut i)) = open.last_mut() else {
                return Ok(());
            };
            let list = self.terms.list(args);
            if (*i as usize) < list.len() {
                if *i > 0 {
                    out.write_str(", ")?;
                }
                next = Some(list[*i as usize]);
                *i += 1;
            } else {
                out.write_char(')')?;
                open.pop();
            }
        }
    }
}
