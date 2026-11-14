// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Proof nets of unit-free MELL with `!`-boxes: the proof structure, its
//! correctness criterion, and the conversions to and from proof terms.
//!
//! A [`MellStructure`] has a vertex for every root formula. For each
//! `⊗`, `⅋` and `!` vertex it has one vertex per premise, as the forest's
//! trees do. So a vertex is an instance of an occurrence of the forest.
//!
//! For a `?` vertex it has any number of *copies* of the subformula.
//! This is the generalised `?` node: one copy is a dereliction, several
//! are a contraction, none is a weakening. It records neither the order
//! of these rules nor the boxes they happen in.
//!
//! Each `!` vertex opens a box. Its premise and everything above it lie
//! in the box, up to the copies of `?` vertices outside. A copy may lie in
//! any box inside its `?` vertex's own box. The wire to it then crosses
//! each box border on the way, at an auxiliary door.
//!
//! A `?` vertex without copies hangs on an axiom link inside its box, as a
//! weakening formula is a conclusion of an axiom (Guerrini and Masini,
//! *Parsing MELL proof nets*, TCS 254, 2001). That keeps every proof net
//! connected, so the Danos–Regnier criterion decides it. Where weakenings
//! hang free, counting components does not decide it.

// A new variant must not fall into an existing arm.
#![deny(clippy::wildcard_enum_match_arm)]

/// From a proof term to its structure.
mod desequentialize;
/// The structure graph of every box depth with boxes contracted, and the
/// criterion's tests on it.
mod graph;
/// From a proof net to a proof term.
mod sequentialize;
#[cfg(all(test, feature = "parse"))]
pub(in crate::linear::nets) mod tests;

use super::{Criterion, NONE, NetError, VertexId, afford};
use crate::Error;
use crate::limits::{Limits, Progress, Refusal, Space};
use crate::linear::fragment::{Fragment, Mode, Order};
use crate::linear::occurrences::{Forest, OccId, Reading, ShapeError};
use crate::linear::sequents::Kind;
use graph::Graph;

/// The most vertices a structure has, counting the door vertices of its
/// graph. A vertex has one slot in the graph per edge, and one more where
/// its parent edge is absent; every edge has two ends. The slots' offsets
/// are `u32`, so the structure also refuses a graph of `u32::MAX` slots or
/// more.
const MOST: u64 = (u32::MAX / 2 - 1) as u64;

/// The bytes a structure holds per vertex beside its forest: 20 for the
/// occurrence, the parent, the box, the partner and the hanging; 8 for
/// the premises' offset and slot; 16 for the graph's level, offset, two
/// slots and flags.
const VERTEX_BYTES: u64 = 20 + 8 + 16;

/// The bytes the construction holds per vertex while it builds the
/// structure, beside the structure: 16 for the premises' counts, offsets
/// and fill; 40 for the box nesting's depths, children and intervals; 40
/// for the graph's builder (a parent, a stamp, a degree, a fill and two
/// edges).
const BUILD_BYTES: u64 = 16 + 40 + 40;

/// The bytes a door vertex of the graph holds: 16 for its level, its `?`
/// vertex, its offset and its slots.
const DOOR_BYTES: u64 = 16;

/// The bytes the criterion's working memory takes per vertex of the
/// graph: 16 for the search's four arrays, 2 for its bridge flag and the
/// deleted sets, 2 for a removed flag per slot, and 8 for the stack.
const SCRATCH_BYTES: u64 = 16 + 2 + 2 + 8;

/// A vertex of a [`MellStructure`] other than a root, as
/// [`MellStructure::from_parts`] takes it.
///
/// It names the occurrence the vertex stands for, the vertex it is a
/// premise of (none for an end of a cut link), and the `!` vertex of the
/// innermost box it lies in (`None` outside every box).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Placed {
    /// The occurrence the vertex stands for.
    pub occurrence: OccId,
    /// The vertex it is a premise of: a `⊗` or `⅋` vertex for its left or
    /// right subformula, a `!` vertex for its subformula, or a `?` vertex
    /// for a copy of its subformula. `None` for an end of a cut link, which
    /// is an occurrence of a cut formula's tree or of its dual.
    pub parent: Option<VertexId>,
    /// The `!` vertex whose box is the innermost one around the vertex, or
    /// `None` outside every box.
    pub inside: Option<VertexId>,
}

impl Placed {
    /// Returns the vertex of `occurrence` that is a premise of `parent`,
    /// in the box of the `!` vertex `inside` (`None` outside every box).
    pub const fn new(occurrence: OccId, parent: VertexId, inside: Option<VertexId>) -> Self {
        Self {
            occurrence,
            parent: Some(parent),
            inside,
        }
    }

    /// Returns the vertex of `occurrence` at an end of a cut link, in the
    /// box of the `!` vertex `inside` (`None` outside every box).
    pub const fn cut(occurrence: OccId, inside: Option<VertexId>) -> Self {
        Self {
            occurrence,
            parent: None,
            inside,
        }
    }
}

/// The parts of a [`MellStructure`] beside its forest and criterion, as
/// [`MellStructure::from_parts`] takes them and [`MellStructure::parts`]
/// returns them.
///
/// The root formulas are vertices `0` to `r − 1`, in the forest's order;
/// `vertices[i]` is vertex `r + i`.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parts {
    /// The vertices after the roots.
    pub vertices: Vec<Placed>,
    /// The cut links. Each joins two vertices without a parent in one box:
    /// an occurrence of a cut formula's tree and its dual.
    pub cuts: Vec<(VertexId, VertexId)>,
    /// The axiom links, each between two dual literal vertices of one box.
    pub links: Vec<(VertexId, VertexId)>,
    /// The weakenings, each a `?` vertex without copies and the literal on
    /// whose axiom link it hangs. The literal lies in the `?` vertex's box
    /// or in a box inside it.
    pub weakenings: Vec<(VertexId, VertexId)>,
}

/// Why parts are no structure over their forest, as
/// [`NetError::Placement`] reports it with the vertex at fault.
///
/// It covers the vertices, cut links, links and weakenings of a structure
/// with boxes, and the cut links and links of a proof structure without
/// boxes.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Placement {
    /// The vertex names an occurrence outside the forest, or a parent or
    /// a box that is no vertex of the structure.
    Outside,
    /// The vertex's occurrence is no premise of its parent's: not the left
    /// or right subformula of a `⊗` or `⅋`, not the subformula of a `!` or
    /// a `?`.
    Premise,
    /// The vertex, a `⊗`, `⅋` or `!`, has a premise missing or twice.
    Premises,
    /// The vertex lies in another box than its place allows, or the box it
    /// names is no `!` vertex's. A root lies outside every box. A premise
    /// of a `⊗` or `⅋` lies in its parent's box, the premise of a `!` in
    /// the `!`'s own box, and a copy in its `?` vertex's box or in one
    /// inside it.
    Box,
    /// The boxes around the vertex's box nest in a cycle.
    Nesting,
    /// The link from this literal leaves its box.
    Link,
    /// The vertex is no weakening: it is no `?` without copies, or it hangs
    /// twice, or what it hangs on is no literal of its box or of a box
    /// inside it.
    Weakening,
    /// The cut link at this vertex is none. Either the link does not join,
    /// in one box, an occurrence of a cut formula's tree to its dual,
    /// neither of them a premise of another vertex; or the vertex ends two
    /// cut links; or it is no premise and no root, and ends no cut link.
    Cut,
    /// A link names an occurrence of a cut formula's tree that lies above
    /// no cut link's end, so it is no vertex of the structure.
    Uncut,
}

impl Placement {
    /// Returns what is wrong with the vertex, as a clause after its name.
    pub(crate) const fn clause(self) -> &'static str {
        match self {
            Self::Outside => "names an occurrence, a parent or a box outside the structure",
            Self::Premise => "stands for no premise of its parent",
            Self::Premises => "has a premise missing or twice",
            Self::Box => "lies in another box than its parent allows",
            Self::Nesting => "lies in boxes that nest in a cycle",
            Self::Link => "is linked out of its box",
            Self::Weakening => {
                "is not a weakening: a ? without copies that hangs once on a literal of its box"
            }
            Self::Cut => {
                "is not the end of one cut link between a cut formula's occurrence and its dual in \
                 one box"
            }
            Self::Uncut => {
                "lies in a cut formula above no cut link, so it is not a vertex, yet a link names it"
            }
        }
    }
}

/// A proof structure of unit-free MELL with `!`-boxes, over the occurrence
/// forest of its sequent, and the [`Criterion`] that
/// [`is_correct`](Self::is_correct) checks it under.
///
/// Every vertex stands for an occurrence of the forest,
/// [`occurrence(v)`](Self::occurrence). The vertices are numbered from the
/// root formulas, `0` to `r − 1` in the forest's order. Each `⊗`, `⅋` and
/// `!` vertex has one vertex per premise. A `?` vertex has any number of
/// *copies* of its subformula as premises: one copy is a dereliction,
/// several are a contraction, none is a weakening. So an occurrence under
/// a `?` can have several vertices.
///
/// A box is named by its `!` vertex. The `!` vertex's premise and
/// everything above it lie in the box, up to the copies of `?` vertices
/// outside it. A copy may lie in any box inside its `?` vertex's own box;
/// its wire then crosses each box border at an auxiliary door
/// ([`auxiliary_doors`](Self::auxiliary_doors)). A `?` vertex without
/// copies hangs on an axiom link of its box or of a box inside it
/// ([`hangs_on`](Self::hangs_on)), so that every proof net is connected.
///
/// Over a sequent with cut formulas, the vertices without a parent other
/// than the roots are the ends of *cut links*. A cut link joins an
/// instance of a cut formula's occurrence to an instance of its dual, in
/// one box. The criterion takes it as a `⊗` without a conclusion, and
/// [`cut`](crate::linear::nets::cut) eliminates it. A structure of an MLL
/// sequent has one vertex per occurrence and no box.
///
/// The structure is built whole and does not change: from its [`Parts`]
/// with [`from_parts`](Self::from_parts), or from a proof term with
/// [`from_proof`](Self::from_proof). [`sequentialize`](Self::sequentialize)
/// turns a proof net back into a term.
///
/// A term names occurrences, not vertices. So it fixes a net only up to
/// two choices: which axiom link a weakening hangs on, and, where a
/// sequent of the term holds two instances of one occurrence, which of
/// them a rule acts on (and so which of two `?` vertices of one occurrence
/// a copy belongs to). Reading a term back may therefore give another
/// proof net of the same sequent than the one it came from. Where no
/// occurrence has two vertices, it gives the same net up to the
/// weakenings' links.
///
/// # Examples
///
/// The net of `⊢ ?~a, !a`: a box around `a`, and the `?~a` outside it
/// with one copy `~a` inside the box, linked to `a`.
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::Limits;
/// use deduxit::linear::{
///     Criterion, Forest, MellStructure, Mode, OccId, Parts, Placed, Sequent, VertexId,
/// };
///
/// // The occurrences are 0: ?~a, 1: ~a, 2: !a, 3: a. The roots are the
/// // vertices 0: ?~a and 1: !a. Vertex 2 is a, in the box of 1, and
/// // vertex 3 is the copy ~a of 0, in that box too.
/// let sequent: Sequent = "|- ?~a, !a".parse()?;
/// let (o, v) = (OccId::new, VertexId::new);
/// let mut parts = Parts::default();
/// parts.vertices = vec![
///     Placed::new(o(3), v(1), Some(v(1))),
///     Placed::new(o(1), v(0), Some(v(1))),
/// ];
/// parts.links = vec![(v(2), v(3))];
/// let limits = Limits::default();
/// let forest = Forest::new(&sequent)?;
/// let net = MellStructure::from_parts(forest, Criterion::MLL, &parts, &limits)?;
/// assert_eq!(net.auxiliary_doors(v(1)), [v(0)]);
/// assert_eq!(net.is_correct(|_| false), Ok(()));
///
/// // A proof net sequentializes into a proof the checker accepts, and the
/// // net of that proof is a proof net again.
/// let proof = net.sequentialize(&limits, |_| false)?;
/// assert_eq!(proof.check(Mode::CLASSICAL), Ok(()));
/// let back = MellStructure::from_proof(&proof, Criterion::MLL, &limits, |_| false)?;
/// assert_eq!(back.len(), 4);
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # JSON
///
/// With the feature `serialize` a structure is `{"version": 4, "sequent":
/// …, "mix": …, "vertices": [[occurrence, parent, box], …], "links":
/// [[x, y], …], "weakenings": [[q, l], …]}` ([`wire`](crate::wire)). The
/// vertices are those after the roots, as in [`Parts`]: `parent` is `null`
/// for an end of a cut link, and `box` is the `!` vertex of the innermost
/// box around the vertex, or `null` outside every box. A structure with
/// cut links has them as `"cuts": [[x, y], …]`, at level 6. Reading
/// validates the parts as [`from_parts`](Self::from_parts) does and takes
/// a structure that is no proof net, since whether it is one is
/// [`is_correct`](Self::is_correct)'s question.
#[derive(Clone, Debug)]
pub struct MellStructure {
    /// The forest of the sequent.
    forest: Forest,
    /// The rules the structure is checked under.
    criterion: Criterion,
    /// Per vertex, the occurrence it stands for.
    occurrence: Box<[OccId]>,
    /// Per vertex, the vertex it is a premise of, or `NONE` for a root.
    parent: Box<[u32]>,
    /// Per vertex, the `!` vertex of its innermost box, or `NONE`.
    inside: Box<[u32]>,
    /// Per vertex, where its premises start in `premises`, and one more
    /// entry for the total, which is below the vertex count.
    start: Box<[u32]>,
    /// The premises of every vertex: of a `⊗` or `⅋` the left one first,
    /// of a `?` its copies in the order they were given.
    premises: Box<[u32]>,
    /// Per vertex, the literal it is linked to, or `NONE`.
    partner: Box<[u32]>,
    /// The links, as given.
    links: Vec<(VertexId, VertexId)>,
    /// The cut links, each from its end in a cut formula's tree, in the
    /// order given.
    cuts: Vec<(VertexId, VertexId)>,
    /// Per vertex, the literal a weakening hangs on, or `NONE`.
    hangs: Box<[u32]>,
    /// The graph of every box depth with boxes contracted.
    graph: Graph,
}

impl MellStructure {
    /// Checks that structures with boxes exist for a sequent of `fragment`
    /// in `mode`. They exist for unit-free MELL in classical mode, with or
    /// without Mix, and in intuitionistic mode.
    ///
    /// In intuitionistic mode the net is the net of the one-sided sequent,
    /// and the criterion without Mix decides it: every sequent of a
    /// classical proof of an intuitionistic sequent without units holds one
    /// formula in output position. The affine modes have no structures: a
    /// proof there weakens formulas that are no `?`, and a structure has no
    /// vertex for that. The ordered modes have none either, since they
    /// refuse exponentials.
    ///
    /// # Errors
    ///
    /// [`NetError::NotMell`] for any other fragment or mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use deduxit::linear::{Fragment, MellStructure, Mode};
    ///
    /// let unit_free = Fragment::MLL.union(Fragment::EXPONENTIALS);
    /// assert!(MellStructure::exist(unit_free, Mode::INTUITIONISTIC).is_ok());
    /// // `Fragment::MELL` has the units `1` and `⊥`, which a structure has
    /// // no vertex for.
    /// assert!(MellStructure::exist(Fragment::MELL, Mode::CLASSICAL).is_err());
    /// ```
    pub fn exist(fragment: Fragment, mode: Mode) -> Result<(), NetError> {
        let unit_free = Fragment::MLL.union(Fragment::EXPONENTIALS);
        if unit_free.contains(fragment) && !mode.is_affine() && !mode.is_ordered() {
            Ok(())
        } else {
            Err(NetError::NotMell { fragment, mode })
        }
    }

    /// Returns the structure of `parts` over the forest, validated.
    ///
    /// Every vertex must be in its place in the formula trees and the
    /// boxes. Every vertex without a parent other than the roots must end
    /// one cut link, which joins an occurrence of a cut formula's tree to
    /// its dual in one box. Every link must join two dual literals of one
    /// box. Every weakening must be a `?` vertex without copies, on a
    /// literal of its box or of a box inside it. The structure's graph is
    /// held within `limits.memory_bytes`.
    ///
    /// Whether the structure is a proof net is
    /// [`is_correct`](Self::is_correct)'s question, so a literal without a
    /// link and a weakening that hangs on nothing are allowed here.
    ///
    /// # Errors
    ///
    /// [`NetError::NotMell`] for a sequent outside unit-free MELL;
    /// [`NetError::Mode`] for a criterion of the cyclic order, which keeps
    /// no exponentials; [`NetError::Placement`] for a vertex out of place;
    /// [`NetError::NoVertex`], [`NetError::NotLiteral`],
    /// [`NetError::NotDual`] and [`NetError::LinkedTwice`] for a link that
    /// is none; [`Refusal::Index`] past the vertices a structure holds and
    /// [`NetError::Refused`] past the memory bound.
    pub fn from_parts(
        forest: Forest,
        criterion: Criterion,
        parts: &Parts,
        limits: &Limits,
    ) -> Result<Self, Error> {
        let mode = if criterion.mix {
            Mode::CLASSICAL.with_mix()
        } else {
            Mode::CLASSICAL
        };
        Self::exist(forest.sequent().fragment(), mode)?;
        if criterion.order != Order::Commutative {
            return Err(NetError::Mode { mode: Mode::CYCLIC }.into());
        }
        let roots = forest.roots().len();
        let n = roots as u64 + parts.vertices.len() as u64;
        if n > MOST {
            return Err(Error::Refused(Refusal::Index {
                what: Space::Vertex,
                count: n,
                most: MOST,
            }));
        }
        afford(n * (VERTEX_BYTES + BUILD_BYTES), limits)?;
        let n = n as usize;
        let mut occurrence = Vec::with_capacity(n);
        let mut parent = Vec::with_capacity(n);
        let mut inside = Vec::with_capacity(n);
        occurrence.extend(forest.roots().iter().copied());
        parent.resize(roots, NONE);
        inside.resize(roots, NONE);
        for (i, placed) in parts.vertices.iter().enumerate() {
            let v = VertexId::new((roots + i) as u32);
            let outside = |w: VertexId| w.index() >= n;
            if placed.occurrence.index() >= forest.len()
                || placed.parent.is_some_and(outside)
                || placed.inside.is_some_and(outside)
            {
                return Err(misplaced(v, Placement::Outside));
            }
            occurrence.push(placed.occurrence);
            parent.push(placed.parent.map_or(NONE, VertexId::get));
            inside.push(placed.inside.map_or(NONE, VertexId::get));
        }
        let (start, premises) = premises(&forest, &occurrence, &parent)?;
        let cuts = place_cuts(&forest, &occurrence, &parent, &inside, roots, &parts.cuts)?;
        check_boxes(&forest, &occurrence, &parent, &inside, roots)?;
        let mut structure = Self {
            criterion,
            occurrence: occurrence.into_boxed_slice(),
            parent: parent.into_boxed_slice(),
            inside: inside.into_boxed_slice(),
            start,
            premises,
            partner: vec![NONE; n].into_boxed_slice(),
            links: Vec::with_capacity(parts.links.len()),
            cuts,
            hangs: vec![NONE; n].into_boxed_slice(),
            graph: Graph::default(),
            forest,
        };
        let boxes = Nesting::new(&structure);
        structure.check_copies(&boxes)?;
        for &(x, y) in &parts.links {
            structure.link(x, y, &boxes)?;
        }
        for &(q, l) in &parts.weakenings {
            structure.hang(q, l, &boxes)?;
        }
        structure.graph = Graph::new(&structure, &boxes, limits)?;
        Ok(structure)
    }

    /// Links two dual literals of one box, refusing what is no link.
    fn link(&mut self, x: VertexId, y: VertexId, boxes: &Nesting) -> Result<(), NetError> {
        let n = self.len();
        for v in [x, y] {
            if v.index() >= n {
                return Err(NetError::NoVertex {
                    vertex: v.get(),
                    vertices: n as u32,
                });
            }
            if !self.forest.is_literal(self.occurrence[v.index()]) {
                return Err(NetError::NotLiteral { vertex: v });
            }
        }
        let (ox, oy) = (self.occurrence[x.index()], self.occurrence[y.index()]);
        if !self.forest.dual_literals(ox, oy) {
            return Err(NetError::NotDual { x, y });
        }
        for v in [x, y] {
            if self.partner[v.index()] != NONE {
                return Err(NetError::LinkedTwice { vertex: v });
            }
        }
        if boxes.of(x) != boxes.of(y) {
            return Err(NetError::Placement {
                vertex: x,
                fault: Placement::Link,
            });
        }
        self.partner[x.index()] = y.get();
        self.partner[y.index()] = x.get();
        self.links.push((x, y));
        Ok(())
    }

    /// Hangs the weakening `q` on the literal `l`, refusing what is none.
    fn hang(&mut self, q: VertexId, l: VertexId, boxes: &Nesting) -> Result<(), NetError> {
        let n = self.len();
        let fault = NetError::Placement {
            vertex: q,
            fault: Placement::Weakening,
        };
        if q.index() >= n || l.index() >= n {
            return Err(fault);
        }
        let weakening = self.forest.kind(self.occurrence[q.index()]) == Kind::Quest
            && self.premises(q).next().is_none()
            && self.hangs[q.index()] == NONE;
        let literal = self.forest.is_literal(self.occurrence[l.index()]);
        if !weakening || !literal || !boxes.within(boxes.of(l), boxes.of(q)) {
            return Err(fault);
        }
        self.hangs[q.index()] = l.get();
        Ok(())
    }

    /// Returns the forest of the sequent.
    pub fn forest(&self) -> &Forest {
        &self.forest
    }

    /// Returns the rules the structure is checked under.
    pub fn criterion(&self) -> Criterion {
        self.criterion
    }

    /// Returns the intuitionistic reading of the structure's sequent: the
    /// side of `⊢` every occurrence, and so every vertex, stands on. It is
    /// recomputed on each call, in time linear in the forest.
    ///
    /// # Errors
    ///
    /// The [`ShapeError`] of a sequent with no intuitionistic reading.
    pub fn reading(&self) -> Result<Reading<'_>, ShapeError> {
        Reading::new(&self.forest)
    }

    /// Returns the number of vertices.
    pub fn len(&self) -> usize {
        self.occurrence.len()
    }

    /// Returns whether the structure has no vertex, which only the empty
    /// sequent gives.
    pub fn is_empty(&self) -> bool {
        self.occurrence.is_empty()
    }

    /// Returns every vertex, in order.
    pub fn vertices(&self) -> impl ExactSizeIterator<Item = VertexId> {
        (0..self.len() as u32).map(VertexId::new)
    }

    /// Returns the occurrence a vertex stands for, `None` for a vertex
    /// outside the structure.
    pub fn occurrence(&self, v: VertexId) -> Option<OccId> {
        self.occurrence.get(v.index()).copied()
    }

    /// Returns the occurrence of every vertex, in vertex order.
    pub(crate) fn occurrences(&self) -> &[OccId] {
        &self.occurrence
    }

    /// Returns the vertex a vertex is a premise of, `None` for a root, an
    /// end of a cut link and a vertex outside the structure.
    pub fn parent(&self, v: VertexId) -> Option<VertexId> {
        some(*self.parent.get(v.index())?)
    }

    /// Returns the premises of a vertex: of a `⊗` or `⅋` its left and its
    /// right one, of a `!` its subformula's, of a `?` its copies; none for
    /// a literal and for a vertex outside the structure.
    pub fn premises(&self, v: VertexId) -> impl Iterator<Item = VertexId> + '_ {
        let range = if v.index() < self.len() {
            self.start[v.index()] as usize..self.start[v.index() + 1] as usize
        } else {
            0..0
        };
        self.premises[range].iter().map(|&w| VertexId::new(w))
    }

    /// Returns the `!` vertex of the innermost box a vertex lies in, `None`
    /// outside every box and for a vertex outside the structure.
    pub fn inside(&self, v: VertexId) -> Option<VertexId> {
        some(*self.inside.get(v.index())?)
    }

    /// Returns the `!` vertices in order, each the principal door of its
    /// box.
    pub fn boxes(&self) -> impl Iterator<Item = VertexId> + '_ {
        self.vertices()
            .filter(|&v| self.forest.kind(self.occurrence[v.index()]) == Kind::Bang)
    }

    /// Returns, in order, the `?` vertices outside the box of the `!`
    /// vertex `door` that have a copy, or a weakening's literal, inside it.
    /// Each has an auxiliary door on the box's border. Empty for a vertex
    /// that is no `!`.
    pub fn auxiliary_doors(&self, door: VertexId) -> Vec<VertexId> {
        self.graph.doors(door)
    }

    /// Returns the literal a vertex is linked to, or `None`.
    pub fn partner(&self, v: VertexId) -> Option<VertexId> {
        some(*self.partner.get(v.index())?)
    }

    /// Returns the links, as given.
    pub fn links(&self) -> &[(VertexId, VertexId)] {
        &self.links
    }

    /// Returns the cut links, each from its end in a cut formula's tree to
    /// its dual end, in the order given.
    pub fn cuts(&self) -> &[(VertexId, VertexId)] {
        &self.cuts
    }

    /// Returns the other end of the cut link at a vertex, `None` for a
    /// vertex at no cut link.
    pub fn cut_partner(&self, v: VertexId) -> Option<VertexId> {
        self.cuts.iter().find_map(|&(x, y)| match v {
            v if v == x => Some(y),
            v if v == y => Some(x),
            _ => None,
        })
    }

    /// Returns whether the structure has no cut link.
    pub fn is_cut_free(&self) -> bool {
        self.cuts.is_empty()
    }

    /// Returns the literal whose axiom link the weakening `v` hangs on, or
    /// `None`.
    pub fn hangs_on(&self, v: VertexId) -> Option<VertexId> {
        some(*self.hangs.get(v.index())?)
    }

    /// Returns the structure's parts, from which
    /// [`from_parts`](Self::from_parts) builds it again.
    pub fn parts(&self) -> Parts {
        let roots = self.forest.roots().len();
        let vertices = (roots..self.len())
            .map(|v| Placed {
                occurrence: self.occurrence[v],
                parent: some(self.parent[v]),
                inside: some(self.inside[v]),
            })
            .collect();
        let weakenings = self
            .vertices()
            .filter_map(|q| Some((q, self.hangs_on(q)?)))
            .collect();
        Parts {
            vertices,
            cuts: self.cuts.clone(),
            links: self.links.clone(),
            weakenings,
        }
    }

    /// Returns the kind of a vertex's occurrence.
    fn kind(&self, v: u32) -> Kind {
        self.forest.kind(self.occurrence[v as usize])
    }

    /// Decides whether the structure is a proof net, independently of any
    /// search and of the proof checker.
    ///
    /// A proof net has every literal linked and every weakening hung on a
    /// link. The criterion then looks at every box depth: outside every
    /// box, and inside each box. At a depth, each box inside it counts as
    /// its `!` vertex, and each `?` vertex outside it as its auxiliary
    /// door. A switching keeps one premise edge of every `⅋` and one wire
    /// of every `?` vertex and door. No switching may have a cycle and,
    /// unless the criterion allows Mix, every switching must be connected.
    ///
    /// It holds working memory linear in the graph. It asks `stop` once per
    /// round of its test for cycles; finding the witness of a cycle runs
    /// that test once per edge.
    ///
    /// # Errors
    ///
    /// [`NetError::Empty`], [`NetError::Unlinked`],
    /// [`NetError::Unattached`], and [`NetError::SwitchingCycle`] or
    /// [`NetError::Disconnected`] with a witness, wrapped in
    /// [`NetError::InBox`] when it lies inside a box;
    /// [`NetError::Refused`] when `stop` ended it.
    pub fn is_correct(&self, stop: impl FnMut(Progress) -> bool) -> Result<(), NetError> {
        if self.is_empty() {
            return Err(NetError::Empty);
        }
        for v in self.vertices() {
            let kind = self.kind(v.get());
            if matches!(kind, Kind::Atom | Kind::DualAtom) && self.partner[v.index()] == NONE {
                return Err(NetError::Unlinked { vertex: v });
            }
            if kind == Kind::Quest
                && self.premises(v).next().is_none()
                && self.hangs[v.index()] == NONE
            {
                return Err(NetError::Unattached { vertex: v });
            }
        }
        self.graph.correct(self.criterion.mix, stop)
    }
}

impl std::fmt::Display for MellStructure {
    /// Writes the structure as text, each vertex as its formula with its
    /// id (`~a[3]`), in this order:
    /// - the sequent;
    /// - a line per `?` vertex with its copies, each followed by the box it
    ///   lies in where that is not the `?` vertex's own
    ///   (`?~a[0]: ~a[3] in !a[1], ~a[5]`), or with `no copy` and the
    ///   literal it hangs on (`?~b[1]: no copy, hangs on a[4]`);
    /// - a line per cut link, in their order (`cut !a[4] — ?~a[7]`);
    /// - a line per link, each with its smaller id first and ordered by it
    ///   (`a[2] — ~a[3]`);
    /// - the criterion's verdict: `proof net (the correctness criterion
    ///   holds)`, the same with Mix, or `not a proof net: ` and the reason
    ///   with formulas.
    ///
    /// No trailing newline.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f, None)
    }
}

impl MellStructure {
    /// Returns the structure for display as `mode` reads it. That is the
    /// [`Display`](std::fmt::Display) text, except in an intuitionistic
    /// mode where the sequent has a reading: there the sequent is two-sided
    /// and every formula is written as the reading writes it
    /// (`!A[0]: A[3] in !A[1]`).
    pub fn written_in(&self, mode: crate::linear::Mode) -> impl std::fmt::Display + '_ {
        crate::linear::nets::In {
            structure: self,
            mode,
        }
    }

    /// Writes the structure as its `Display` does or, given a `mode`, as
    /// [`written_in`](Self::written_in) does.
    pub(crate) fn write(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        mode: Option<crate::linear::Mode>,
    ) -> std::fmt::Result {
        let reading = mode
            .filter(|mode| mode.is_intuitionistic())
            .and_then(|_| self.reading().ok());
        let vertex = |f: &mut std::fmt::Formatter<'_>, v: VertexId| {
            let o = self.occurrence[v.index()];
            match &reading {
                Some(reading) => write!(f, "{}[{}]", reading.formula(o), v.get()),
                None => write!(f, "{}[{}]", self.forest.formula(o), v.get()),
            }
        };
        match &reading {
            Some(reading) => writeln!(f, "{reading}")?,
            None => writeln!(f, "{}", self.forest.sequent())?,
        }

        for q in self
            .vertices()
            .filter(|&q| self.kind(q.get()) == Kind::Quest)
        {
            vertex(f, q)?;
            f.write_str(":")?;
            let mut copies = self.premises(q).peekable();
            if copies.peek().is_none() {
                f.write_str(" no copy")?;
                if let Some(l) = self.hangs_on(q) {
                    f.write_str(", hangs on ")?;
                    vertex(f, l)?;
                }
            }
            let mut first = true;
            for c in copies {
                f.write_str(if first { " " } else { ", " })?;
                first = false;
                vertex(f, c)?;
                if let Some(d) = self.inside(c).filter(|&d| Some(d) != self.inside(q)) {
                    f.write_str(" in ")?;
                    vertex(f, d)?;
                }
            }
            f.write_str("\n")?;
        }
        for &(x, y) in &self.cuts {
            f.write_str("cut ")?;
            vertex(f, x)?;
            f.write_str(" — ")?;
            vertex(f, y)?;
            f.write_str("\n")?;
        }
        let mut links: Vec<(VertexId, VertexId)> = self
            .links
            .iter()
            .map(|&(x, y)| (x.min(y), x.max(y)))
            .collect();
        links.sort_unstable();
        for (x, y) in links {
            vertex(f, x)?;
            f.write_str(" — ")?;
            vertex(f, y)?;
            f.write_str("\n")?;
        }
        match self.is_correct(|_| false) {
            Ok(()) if self.criterion.mix => {
                f.write_str("proof net with Mix (the correctness criterion with Mix holds)")
            }
            Ok(()) => f.write_str("proof net (the correctness criterion holds)"),
            Err(e) => match mode {
                Some(mode) => write!(f, "not a proof net: {}", e.describe(self).in_mode(mode)),
                None => write!(f, "not a proof net: {}", e.describe(self)),
            },
        }
    }
}

/// Returns a raw index as a vertex, `None` for `NONE`.
fn some(raw: u32) -> Option<VertexId> {
    (raw != NONE).then_some(VertexId::new(raw))
}

/// Returns the error of a vertex out of place.
fn misplaced(vertex: VertexId, fault: Placement) -> Error {
    NetError::Placement { vertex, fault }.into()
}

/// Offsets per vertex, with the total after them, and the list they index.
type Csr = (Box<[u32]>, Box<[u32]>);

/// Checks that every vertex stands for a premise of its parent's
/// occurrence, and that every `⊗`, `⅋` and `!` has each premise once.
/// Returns the premises in CSR layout: offsets per vertex into one list.
///
/// A parent's occurrence is the parent of its premise's occurrence, one
/// level nearer the root, so parents cannot form a cycle.
fn premises(forest: &Forest, occurrence: &[OccId], parent: &[u32]) -> Result<Csr, Error> {
    let n = occurrence.len();
    let mut count = vec![0u32; n + 1];
    for (v, &p) in parent.iter().enumerate() {
        if p == NONE {
            continue;
        }
        let o = occurrence[v];
        if forest.parent(o) != Some(occurrence[p as usize]) {
            return Err(misplaced(VertexId::new(v as u32), Placement::Premise));
        }
        count[p as usize] += 1;
    }
    // The offsets fit a `u32`: there are fewer premises than vertices.
    let mut start = Vec::with_capacity(n + 1);
    let mut total = 0;
    for &c in &count[..n] {
        start.push(total);
        total += c;
    }
    start.push(total);
    let mut fill = start.clone();
    let mut list = vec![0u32; total as usize];
    for (v, &p) in parent.iter().enumerate() {
        if p != NONE {
            list[fill[p as usize] as usize] = v as u32;
            fill[p as usize] += 1;
        }
    }
    for p in 0..n {
        let o = occurrence[p];
        let premises = &mut list[start[p] as usize..start[p + 1] as usize];
        let expected = match forest.kind(o) {
            Kind::Tensor | Kind::Par => 2,
            Kind::Bang => 1,
            Kind::Quest | Kind::Atom | Kind::DualAtom => continue,
            Kind::One
            | Kind::Bot
            | Kind::Top
            | Kind::Zero
            | Kind::With
            | Kind::Plus
            | Kind::Forall
            | Kind::Exists => {
                unreachable!("a unit-free MELL sequent has no such occurrence")
            }
        };
        // The left premise first; each premise once.
        if expected == 2
            && premises.len() == 2
            && occurrence[premises[0] as usize] != forest.left(o).unwrap()
        {
            premises.swap(0, 1);
        }
        let distinct = expected == 1
            || premises.len() != 2
            || occurrence[premises[0] as usize] != occurrence[premises[1] as usize];
        if premises.len() != expected || !distinct {
            return Err(misplaced(VertexId::new(p as u32), Placement::Premises));
        }
    }
    Ok((start.into_boxed_slice(), list.into_boxed_slice()))
}

/// Checks the cut links and returns them, each from its end in a cut
/// formula's tree.
///
/// A cut link joins, in one box, two vertices without a parent that are no
/// roots: an occurrence of a cut formula's tree and its dual. Each such
/// vertex ends exactly one cut link.
fn place_cuts(
    forest: &Forest,
    occurrence: &[OccId],
    parent: &[u32],
    inside: &[u32],
    roots: usize,
    cuts: &[(VertexId, VertexId)],
) -> Result<Vec<(VertexId, VertexId)>, Error> {
    let n = occurrence.len();
    let mut ended = vec![false; n];
    let mut placed = Vec::with_capacity(cuts.len());
    for &(x, y) in cuts {
        let end = |v: VertexId| v.index() < n && v.index() >= roots && parent[v.index()] == NONE;
        if !end(x) || !end(y) || inside[x.index()] != inside[y.index()] {
            return Err(misplaced(x, Placement::Cut));
        }
        let (ox, oy) = (occurrence[x.index()], occurrence[y.index()]);
        let (a, d) = match (forest.dual(ox), forest.dual(oy)) {
            (Some(dx), _) if dx == oy && ox < oy => (x, y),
            (_, Some(dy)) if dy == ox && oy < ox => (y, x),
            _ => return Err(misplaced(x, Placement::Cut)),
        };
        for v in [a, d] {
            if std::mem::replace(&mut ended[v.index()], true) {
                return Err(misplaced(v, Placement::Cut));
            }
        }
        placed.push((a, d));
    }
    if let Some(v) = (roots..n).find(|&v| parent[v] == NONE && !ended[v]) {
        return Err(misplaced(VertexId::new(v as u32), Placement::Cut));
    }
    Ok(placed)
}

/// The mark, in place of a depth, of a box on the chain of enclosing
/// boxes being walked.
const VISITING: u32 = NONE - 1;

/// Checks that every vertex lies in the box its parent allows and that
/// boxes nest without a cycle.
fn check_boxes(
    forest: &Forest,
    occurrence: &[OccId],
    parent: &[u32],
    inside: &[u32],
    roots: usize,
) -> Result<(), Error> {
    let n = occurrence.len();
    let is_bang = |b: u32| b == NONE || forest.kind(occurrence[b as usize]) == Kind::Bang;
    for v in 0..n {
        let (p, b) = (parent[v], inside[v]);
        let fits = match p {
            // The roots lie outside every box, a cut link's end in any.
            NONE if v < roots => b == NONE,
            NONE => is_bang(b),
            p => match forest.kind(occurrence[p as usize]) {
                Kind::Bang => b == p,
                // A copy's box is checked once the boxes are known to nest.
                Kind::Quest => is_bang(b),
                Kind::Tensor | Kind::Par => b == inside[p as usize],
                Kind::Atom
                | Kind::DualAtom
                | Kind::One
                | Kind::Bot
                | Kind::Top
                | Kind::Zero
                | Kind::With
                | Kind::Plus
                | Kind::Forall
                | Kind::Exists => false,
            },
        };
        if !fits {
            return Err(misplaced(VertexId::new(v as u32), Placement::Box));
        }
    }
    // Every box's chain of enclosing boxes must end. Walk it from each `!`
    // vertex, marking the boxes on the way, then give them their depths.
    let mut depth = vec![NONE; n];
    let mut path = Vec::new();
    for d in 0..n as u32 {
        if !is_bang(d) || depth[d as usize] != NONE {
            continue;
        }
        let mut b = d;
        while b != NONE && depth[b as usize] == NONE {
            depth[b as usize] = VISITING;
            path.push(b);
            b = inside[b as usize];
        }
        if b != NONE && depth[b as usize] == VISITING {
            return Err(misplaced(VertexId::new(d), Placement::Nesting));
        }
        let below = if b == NONE { 0 } else { depth[b as usize] + 1 };
        for (d, &c) in (below..).zip(path.iter().rev()) {
            depth[c as usize] = d;
        }
        path.clear();
    }
    Ok(())
}

/// The boxes of a structure as a tree, each with the interval of its
/// subtree in a depth-first order, so that whether one box lies inside
/// another takes two comparisons. The box outside every box is `NONE`.
pub(super) struct Nesting {
    /// Per vertex, the `!` vertex of its innermost box, or `NONE`.
    inside: Box<[u32]>,
    /// Per `!` vertex, when the walk entered its box and when it left it.
    interval: Box<[(u32, u32)]>,
}

impl Nesting {
    /// Returns the nesting of a structure whose boxes nest without a
    /// cycle.
    fn new(s: &MellStructure) -> Self {
        let n = s.len();
        let mut children = vec![Vec::new(); n + 1];
        for d in s.boxes() {
            let outer = s.inside[d.index()];
            let slot = if outer == NONE { n } else { outer as usize };
            children[slot].push(d.get());
        }
        let mut interval = vec![(0, 0); n];
        let mut time = 0;
        let mut stack = vec![(n as u32, 0usize)];
        while let Some(&mut (b, ref mut next)) = stack.last_mut() {
            let slot = if b as usize == n { n } else { b as usize };
            if let Some(&c) = children[slot].get(*next) {
                *next += 1;
                interval[c as usize].0 = time;
                time += 1;
                stack.push((c, 0));
            } else {
                if (b as usize) < n {
                    interval[b as usize].1 = time;
                }
                stack.pop();
            }
        }
        Self {
            inside: s.inside.clone(),
            interval: interval.into_boxed_slice(),
        }
    }

    /// Returns the box a vertex lies in.
    pub(super) fn of(&self, v: VertexId) -> u32 {
        self.inside[v.index()]
    }

    /// Returns whether box `b` lies in box `outer` or is it.
    pub(super) fn within(&self, b: u32, outer: u32) -> bool {
        if outer == NONE || b == outer {
            return true;
        }
        if b == NONE {
            return false;
        }
        let (i, o) = (self.interval[b as usize], self.interval[outer as usize]);
        o.0 < i.0 && i.0 < o.1
    }
}

impl MellStructure {
    /// Checks that every copy lies in its `?` vertex's box or in one inside
    /// it. [`check_boxes`] leaves this until the boxes are known to nest.
    fn check_copies(&self, boxes: &Nesting) -> Result<(), Error> {
        for v in self.vertices() {
            let p = self.parent[v.index()];
            if p != NONE
                && self.kind(p) == Kind::Quest
                && !boxes.within(self.inside[v.index()], self.inside[p as usize])
            {
                return Err(misplaced(v, Placement::Box));
            }
        }
        Ok(())
    }
}
