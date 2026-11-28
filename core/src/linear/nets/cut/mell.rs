// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use super::{Step, within};
use crate::Error;
use crate::limits::{Limits, Progress};
use crate::linear::nets::{MellStructure, NONE, NetError, Parts, Placed, VertexId};
use crate::linear::occurrences::OccId;
use crate::linear::proofs::cut::{Case, Options};
use crate::linear::sequents::Kind;

/// A structure with boxes taken apart for a step, all by raw vertex ids:
/// per vertex its occurrence, its parent, its box and whether the step
/// erased it, and the structure's cut links, links and weakenings.
struct Work {
    /// Per vertex, its occurrence.
    occurrence: Vec<OccId>,
    /// Per vertex, the vertex it is a premise of, or `NONE`.
    parent: Vec<u32>,
    /// Per vertex, the `!` vertex of its innermost box, or `NONE`.
    inside: Vec<u32>,
    /// Per vertex, whether the step erased it.
    erased: Vec<bool>,
    /// The cut links, each from its end in a cut formula's tree.
    cuts: Vec<(u32, u32)>,
    /// The links.
    links: Vec<(u32, u32)>,
    /// The weakenings and the literals they hang on.
    hangs: Vec<(u32, u32)>,
    /// The number of roots, vertices `0` to `roots − 1`.
    roots: usize,
}

impl Work {
    /// Takes a structure apart.
    fn of(net: &MellStructure) -> Self {
        let raw = |v: Option<VertexId>| v.map_or(NONE, VertexId::get);
        let parts = net.parts();
        let pair = |&(x, y): &(VertexId, VertexId)| (x.get(), y.get());
        Self {
            occurrence: net.vertices().map(|v| net.occurrence(v).unwrap()).collect(),
            parent: net.vertices().map(|v| raw(net.parent(v))).collect(),
            inside: net.vertices().map(|v| raw(net.inside(v))).collect(),
            erased: vec![false; net.len()],
            cuts: parts.cuts.iter().map(pair).collect(),
            links: parts.links.iter().map(pair).collect(),
            hangs: parts.weakenings.iter().map(pair).collect(),
            roots: net.forest().roots().len(),
        }
    }

    /// Returns the number of vertices the step left.
    fn vertices(&self) -> usize {
        self.erased.iter().filter(|&&e| !e).count()
    }

    /// Returns the box of the `!` vertex `b`: `b` and every vertex inside
    /// it, at any depth, in vertex order.
    ///
    /// One pass over the vertices lists each box's own vertices, then a
    /// walk down the boxes inside gathers them. So it takes time linear in
    /// the structure, however deep the boxes nest.
    fn contents(&self, b: u32) -> Vec<u32> {
        let n = self.parent.len();
        let mut own: Vec<Vec<u32>> = vec![Vec::new(); n];
        for v in 0..n {
            let i = self.inside[v];
            if i != NONE && !self.erased[v] {
                own[i as usize].push(v as u32);
            }
        }
        let mut inner = vec![b];
        let mut boxes = vec![b];
        while let Some(c) = boxes.pop() {
            // A vertex whose box this is and which has a box of its own is
            // the `!` vertex of a box inside.
            for &v in &own[c as usize] {
                inner.push(v);
                if !own[v as usize].is_empty() {
                    boxes.push(v);
                }
            }
        }
        inner.sort_unstable();
        inner
    }

    /// Adds a vertex and returns it.
    fn push(&mut self, occurrence: OccId, parent: u32, inside: u32) -> u32 {
        self.occurrence.push(occurrence);
        self.parent.push(parent);
        self.inside.push(inside);
        self.erased.push(false);
        (self.occurrence.len() - 1) as u32
    }

    /// Erases the box of `b` and every link, cut link and weakening in it.
    fn erase(&mut self, b: u32) {
        let gone = self.contents(b);
        for &v in &gone {
            self.erased[v as usize] = true;
        }
        let erased = &self.erased;
        let kept = |&(x, y): &(u32, u32)| !erased[x as usize] && !erased[y as usize];
        self.links.retain(kept);
        self.cuts.retain(kept);
        self.hangs.retain(|&(q, _)| !erased[q as usize]);
    }

    /// Duplicates the box of `b`, whose contents are `inner`, and returns
    /// the duplicate's `!` vertex: every vertex in it, the links, cut links
    /// and weakenings inside, and for a copy in it of a `?` vertex outside,
    /// a copy of that vertex.
    fn duplicate(&mut self, b: u32, inner: &[u32]) -> u32 {
        let mut copy = vec![NONE; self.parent.len()];
        for &v in inner {
            copy[v as usize] = self.push(self.occurrence[v as usize], NONE, NONE);
        }
        let image = |v: u32| match copy[v as usize] {
            NONE => v,
            c => c,
        };
        for &v in inner {
            let c = copy[v as usize] as usize;
            let (p, i) = (self.parent[v as usize], self.inside[v as usize]);
            self.parent[c] = if p == NONE { NONE } else { image(p) };
            self.inside[c] = if i == NONE { NONE } else { image(i) };
        }
        let within = |&(x, y): &(u32, u32)| copy[x as usize] != NONE && copy[y as usize] != NONE;
        let twin = |&(x, y): &(u32, u32)| (image(x), image(y));
        let links: Vec<(u32, u32)> = self.links.iter().filter(|l| within(l)).map(twin).collect();
        let cuts: Vec<(u32, u32)> = self.cuts.iter().filter(|l| within(l)).map(twin).collect();
        let hangs: Vec<(u32, u32)> = self.hangs.iter().filter(|l| within(l)).map(twin).collect();
        self.links.extend(links);
        self.cuts.extend(cuts);
        self.hangs.extend(hangs);
        copy[b as usize]
    }

    /// Returns the parts of what the step left, renumbered: the roots,
    /// then depth first every root's tree and every cut link's two trees,
    /// a vertex's premises in vertex order.
    fn parts(&self) -> Parts {
        let n = self.parent.len();
        let mut premises = vec![Vec::new(); n];
        for v in 0..n {
            let p = self.parent[v];
            if p != NONE && !self.erased[v] {
                premises[p as usize].push(v as u32);
            }
        }
        let mut new = vec![NONE; n];
        let mut order: Vec<u32> = Vec::with_capacity(n);
        for r in 0..self.roots as u32 {
            new[r as usize] = r;
            order.push(r);
        }
        let ends = self.cuts.iter().flat_map(|&(x, y)| [x, y]);
        let mut stack = Vec::new();
        for top in (0..self.roots as u32).chain(ends) {
            stack.push(top);
            while let Some(v) = stack.pop() {
                if new[v as usize] == NONE {
                    new[v as usize] = order.len() as u32;
                    order.push(v);
                }
                stack.extend(premises[v as usize].iter().rev());
            }
        }
        let renamed = |v: u32| VertexId::new(new[v as usize]);
        let vertices = order[self.roots..]
            .iter()
            .map(|&v| {
                let (o, p, i) = (
                    self.occurrence[v as usize],
                    self.parent[v as usize],
                    self.inside[v as usize],
                );
                let inside = (i != NONE).then(|| renamed(i));
                match p {
                    NONE => Placed::cut(o, inside),
                    p => Placed::new(o, renamed(p), inside),
                }
            })
            .collect();
        let pairs = |list: &[(u32, u32)]| {
            list.iter()
                .map(|&(x, y)| (renamed(x), renamed(y)))
                .collect()
        };
        Parts {
            vertices,
            cuts: pairs(&self.cuts),
            links: pairs(&self.links),
            weakenings: pairs(&self.hangs),
        }
    }
}

impl crate::sealed::Net for MellStructure {
    fn cut_links(&self) -> &[(VertexId, VertexId)] {
        self.cuts()
    }

    fn check(&self, stop: &mut dyn FnMut(Progress) -> bool) -> Result<(), NetError> {
        self.is_correct(stop)
    }

    fn reduce(
        &self,
        (x, y): (VertexId, VertexId),
        options: &Options,
        limits: &Limits,
    ) -> Result<(Self, Step), Error> {
        let forest = self.forest();
        let mut work = Work::of(self);
        let at = work
            .cuts
            .iter()
            .position(|&cut| cut == (x.get(), y.get()))
            .expect("a cut link of the net");
        let (x, y) = (x.get(), y.get());
        let mut above = None;
        let case = match work.kind(self, x) {
            Kind::Atom | Kind::DualAtom => work.axiom(self, at, (x, y)),
            Kind::Tensor | Kind::Par => work.tensor_par(self, at, (x, y)),
            Kind::Bang | Kind::Quest => {
                let (bang, quest) = if work.kind(self, x) == Kind::Bang {
                    (x, y)
                } else {
                    (y, x)
                };
                let cut = Exponential {
                    at,
                    ends: (x, y),
                    bang,
                    quest,
                    depth: work.inside[quest as usize],
                };
                match premises(self, quest)[..] {
                    [] => work.weakening(self, &cut),
                    [c] if work.inside[c as usize] == cut.depth => {
                        above = Some(c);
                        work.dereliction(self, &cut, c)
                    }
                    [c] => {
                        let e = work.box_into_box(&cut, c);
                        above = Some(e);
                        Case::BoxIntoBox
                    }
                    [c, ..] => {
                        work.contraction(&cut, c, options)?;
                        above = Some(c);
                        Case::Contraction
                    }
                }
            }
            Kind::One
            | Kind::Bot
            | Kind::Top
            | Kind::Zero
            | Kind::With
            | Kind::Plus
            | Kind::Forall
            | Kind::Exists => {
                unreachable!("a structure with boxes is of unit-free MELL")
            }
        };
        within(work.vertices(), options)?;
        let parts = work.parts();
        let made = Self::from_parts(forest.clone(), self.criterion(), &parts, limits)?;
        let step = Step {
            cut: (VertexId::new(x), VertexId::new(y)),
            case,
            above: above.map(VertexId::new),
        };
        Ok((made, step))
    }
}

/// Returns a vertex's premises in the net the step is taken on.
fn premises(net: &MellStructure, v: u32) -> Vec<u32> {
    net.premises(VertexId::new(v)).map(VertexId::get).collect()
}

/// A cut link between a `!` vertex and a `?` vertex, as the exponential
/// cases read it.
struct Exponential {
    /// The cut link's place in the list.
    at: usize,
    /// Its two ends, the one in a cut formula's tree first.
    ends: (u32, u32),
    /// The `!` end.
    bang: u32,
    /// The `?` end.
    quest: u32,
    /// The box the cut link stands in, or `NONE`.
    depth: u32,
}

impl Work {
    /// Returns the kind of a vertex's occurrence.
    fn kind(&self, net: &MellStructure, v: u32) -> Kind {
        net.forest().kind(self.occurrence[v as usize])
    }

    /// The axiom case: the two literals' partners are linked, and the
    /// weakenings hung on either literal hang on that link.
    fn axiom(&mut self, net: &MellStructure, at: usize, (x, y): (u32, u32)) -> Case {
        // A proof net links every literal, and never a cut's two ends to
        // each other.
        let partner = |v: u32| {
            net.partner(VertexId::new(v))
                .expect("a proof net links every literal")
                .get()
        };
        let (px, py) = (partner(x), partner(y));
        super::relink(&mut self.links, (x, y), (px, py));
        // A weakening hung on either literal hangs on the link made.
        for hang in &mut self.hangs {
            if hang.1 == x || hang.1 == y {
                hang.1 = px;
            }
        }
        self.erased[x as usize] = true;
        self.erased[y as usize] = true;
        self.cuts.remove(at);
        Case::Axiom
    }

    /// The `⊗`/`⅋` case: the cut link becomes two, between the left
    /// premises and between the right ones.
    fn tensor_par(&mut self, net: &MellStructure, at: usize, (x, y): (u32, u32)) -> Case {
        let forest = net.forest();
        let (px, py) = (premises(net, x), premises(net, y));
        // The left premise first, as the structure keeps them.
        let left = |v: u32, list: &[u32]| {
            let l = forest.left(self.occurrence[v as usize]).unwrap();
            if self.occurrence[list[0] as usize] == l {
                (list[0], list[1])
            } else {
                (list[1], list[0])
            }
        };
        let ((xl, xr), (yl, yr)) = (left(x, &px), left(y, &py));
        for v in [xl, xr, yl, yr] {
            self.parent[v as usize] = NONE;
        }
        self.erased[x as usize] = true;
        self.erased[y as usize] = true;
        self.cuts.splice(at..=at, [(xl, yl), (xr, yr)]);
        Case::TensorPar
    }

    /// The weakening case: the box is erased, and what lost its copies or
    /// its literal in it hangs where the weakening hung.
    fn weakening(&mut self, net: &MellStructure, cut: &Exponential) -> Case {
        let (b, q) = (cut.bang, cut.quest);
        let hung = net
            .hangs_on(VertexId::new(q))
            .expect("a proof net hangs every weakening")
            .get();
        self.cuts.remove(cut.at);
        self.erase(b);
        debug_assert!(
            !self.erased[hung as usize],
            "a weakening cut against a box hangs outside it in a proof net"
        );
        self.erased[q as usize] = true;
        self.hangs.retain(|&(w, _)| w != q);
        // What lost its copies, or its literal, in the box hangs where the
        // erased weakening hung.
        for hang in &mut self.hangs {
            if self.erased[hang.1 as usize] {
                hang.1 = hung;
            }
        }
        let bare: Vec<u32> = (0..self.parent.len() as u32)
            .filter(|&r| {
                !self.erased[r as usize] && self.kind(net, r) == Kind::Quest && {
                    let mut copies = net.premises(VertexId::new(r)).peekable();
                    copies.peek().is_some() && copies.all(|c| self.erased[c.index()])
                }
            })
            .collect();
        self.hangs.extend(bare.into_iter().map(|r| (r, hung)));
        Case::Weakening
    }

    /// The dereliction case, the one copy `c` at the cut's depth: the box
    /// opens, and its premise is cut against the copy.
    fn dereliction(&mut self, net: &MellStructure, cut: &Exponential, c: u32) -> Case {
        let (b, q, depth) = (cut.bang, cut.quest, cut.depth);
        let premise = premises(net, b)[0];
        for v in 0..self.inside.len() {
            if self.inside[v] == b {
                self.inside[v] = depth;
            }
        }
        self.parent[premise as usize] = NONE;
        self.parent[c as usize] = NONE;
        self.erased[b as usize] = true;
        self.erased[q as usize] = true;
        self.cuts[cut.at] = if b == cut.ends.0 {
            (premise, c)
        } else {
            (c, premise)
        };
        Case::Dereliction
    }

    /// The box-into-box case, the one copy `c` inside a box: the cut link
    /// moves into the box at the cut's depth around the copy, which it
    /// returns.
    fn box_into_box(&mut self, cut: &Exponential, c: u32) -> u32 {
        // The box at the cut's depth around the copy.
        let mut e = self.inside[c as usize];
        while self.inside[e as usize] != cut.depth {
            e = self.inside[e as usize];
        }
        self.inside[cut.bang as usize] = e;
        self.inside[cut.quest as usize] = e;
        e
    }

    /// The contraction case, the first of several copies `c`: the box is
    /// copied, the copy cut against a `?` vertex of `c`'s own, within
    /// `options`' bound on vertices.
    fn contraction(&mut self, cut: &Exponential, c: u32, options: &Options) -> Result<(), Error> {
        let (b, q) = (cut.bang, cut.quest);
        // The bound on vertices before the box is copied.
        let inner = self.contents(b);
        within(self.vertices() + inner.len() + 1, options)?;
        let twin = self.duplicate(b, &inner);
        let own = self.push(self.occurrence[q as usize], NONE, cut.depth);
        self.parent[c as usize] = own;
        let made = if b == cut.ends.0 {
            (twin, own)
        } else {
            (own, twin)
        };
        self.cuts.insert(cut.at, made);
        Ok(())
    }
}
