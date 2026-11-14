// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use super::sequents::Sequent;
use crate::limits::Limits;
use crate::linear::fragment::Order;
use crate::linear::nets::{Criterion, MellStructure, Parts, Placed, ProofStructure, VertexId};
use crate::linear::occurrences::{Forest, OccId};
use crate::wire::{self, Readable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The serialized form of a proof structure: its level, its sequent, the
/// criterion's fields, its cut links where it has some, and its axiom
/// links in the order they were made. The criterion's fields are whether
/// Mix is allowed, and the order where it is not the commutative one. A
/// link is a pair of vertex ids.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Net {
    /// The wire level.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The sequent.
    sequent: Sequent,
    /// Whether Mix is allowed.
    mix: bool,
    /// The order the criterion keeps, written where it is not the
    /// commutative one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    order: Option<OrderForm>,
    /// The cut links, written where there are some.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    cuts: Vec<(u32, u32)>,
    /// The axiom links.
    links: Vec<(u32, u32)>,
    /// The vertices of a structure with boxes. This form refuses them,
    /// since the links of such a structure name its vertices, not
    /// occurrences.
    #[serde(default, skip_serializing)]
    vertices: Option<serde::de::IgnoredAny>,
}

/// An order other than the commutative one, by its name.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum OrderForm {
    /// The commutative order, which a writer leaves out.
    Commutative,
    /// The cyclic order.
    Cyclic,
}

impl From<OrderForm> for Order {
    /// Returns the order the name names.
    fn from(form: OrderForm) -> Self {
        match form {
            OrderForm::Commutative => Order::Commutative,
            OrderForm::Cyclic => Order::Cyclic,
        }
    }
}

impl Serialize for ProofStructure {
    /// Serializes the structure as a document: its level, sequent,
    /// criterion, cut links and links.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        form(self).serialize(serializer)
    }
}

/// Returns the form of a proof structure, with its level.
fn form(net: &ProofStructure) -> Net {
    let order = match net.criterion().order {
        Order::Commutative => None,
        Order::Cyclic => Some(OrderForm::Cyclic),
    };
    let pair = |&(x, y): &(VertexId, VertexId)| (x.get(), y.get());
    Net {
        // The cyclic order needs level 2. The sequent's own level is 6
        // with cut formulas and 7 when it is first-order.
        version: Some(if order.is_some() {
            wire::level_of_sequent(net.sequent()).max(2)
        } else {
            wire::level_of_sequent(net.sequent())
        }),
        sequent: Sequent::from(net.sequent()),
        mix: net.criterion().mix,
        order,
        vertices: None,
        cuts: net.cuts().iter().map(pair).collect(),
        links: net.links().iter().map(pair).collect(),
    }
}

impl<'a> Deserialize<'a> for ProofStructure {
    /// Deserializes a structure within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for ProofStructure {}

impl Readable for ProofStructure {
    const FORM: &'static str = "proof structure";

    /// Reads a structure. Builds the forest of its sequent within the
    /// limits, and validates the cut links and links as
    /// [`ProofStructure::with_cuts`] does. Whether it is a proof net is
    /// for [`ProofStructure::is_correct`] to say.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let net = Net::deserialize(deserializer)?;
        if net.vertices.is_some() {
            return Err(serde::de::Error::custom(
                "a proof structure with boxes, whose links name its vertices: read it as one",
            ));
        }
        let build = || {
            let sequent = crate::linear::Sequent::try_from(net.sequent)?;
            let forest = Forest::from_owned(sequent, limits)?;
            let pair = |&(x, y): &(u32, u32)| (VertexId::new(x), VertexId::new(y));
            let links: Vec<(VertexId, VertexId)> = net.links.iter().map(pair).collect();
            let cuts: Vec<(VertexId, VertexId)> = net.cuts.iter().map(pair).collect();
            let criterion = Criterion {
                mix: net.mix,
                order: net.order.map_or(Order::Commutative, Order::from),
            };
            ProofStructure::with_cuts(forest, criterion, &cuts, &links)
        };
        build().map_err(wire::fail)
    }
}

/// The serialized form of a proof structure with boxes. Its keys:
/// - `version`, its level;
/// - `sequent`;
/// - `mix`, whether Mix is allowed;
/// - `vertices`, the vertices after the roots, each `[occurrence, parent,
///   box]`. `parent` is `null` for an end of a cut link. `box` is the `!`
///   vertex of the innermost box around the vertex, or `null` outside
///   every box;
/// - `cuts`, the cut links, where there are some;
/// - `links`, the axiom links;
/// - `weakenings`, each a `?` vertex without copies and the literal it
///   hangs on.
///
/// Links and weakenings are pairs of vertex ids.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Boxed {
    /// The wire level.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The sequent.
    sequent: Sequent,
    /// Whether Mix is allowed.
    mix: bool,
    /// The vertices after the roots.
    vertices: Vec<(u32, Option<u32>, Option<u32>)>,
    /// The cut links, written where there are some.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    cuts: Vec<(u32, u32)>,
    /// The axiom links.
    links: Vec<(u32, u32)>,
    /// The weakenings.
    weakenings: Vec<(u32, u32)>,
}

impl Serialize for MellStructure {
    /// Serializes the structure as a document: its level, sequent,
    /// whether Mix is allowed, vertices, cut links, links and weakenings.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        boxed(self).serialize(serializer)
    }
}

/// Returns the form of a proof structure with boxes, with its level.
fn boxed(net: &MellStructure) -> Boxed {
    let parts = net.parts();
    let pair = |&(x, y): &(VertexId, VertexId)| (x.get(), y.get());
    Boxed {
        version: Some(wire::BOXES.max(wire::level_of_sequent(net.forest().sequent()))),
        sequent: Sequent::from(net.forest().sequent()),
        mix: net.criterion().mix,
        vertices: parts
            .vertices
            .iter()
            .map(|p| {
                (
                    p.occurrence.get(),
                    p.parent.map(VertexId::get),
                    p.inside.map(VertexId::get),
                )
            })
            .collect(),
        cuts: parts.cuts.iter().map(pair).collect(),
        links: parts.links.iter().map(pair).collect(),
        weakenings: parts.weakenings.iter().map(pair).collect(),
    }
}

/// A step of a cut elimination on a net, as its record writes it.
#[derive(Serialize)]
struct StepForm {
    /// The cut link reduced, from its end in a cut formula's tree.
    cut: (u32, u32),
    /// The case.
    case: crate::linear::proofs::cut::Case,
    /// The vertex the step acted at besides the cut link.
    #[serde(skip_serializing_if = "Option::is_none")]
    above: Option<u32>,
}

/// A cut elimination on a net, as its record writes it.
#[derive(Serialize)]
struct EliminationForm<'a, F> {
    /// The wire level.
    version: u32,
    /// The net reached, nested.
    net: F,
    /// The steps.
    steps: Vec<StepForm>,
    /// `cut_free`, or `refused` with the reason.
    end: &'static str,
    /// Why the elimination was refused, in an error's form.
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'a crate::Error>,
}

/// Writes the record of a cut elimination. `net` is the form of the net
/// it reached, without its level, which is `version`.
fn record<S: Serializer, F: Serialize>(
    serializer: S,
    net: F,
    version: u32,
    steps: &[crate::linear::nets::cut::Step],
    end: &crate::linear::proofs::cut::End,
) -> Result<S::Ok, S::Error> {
    use crate::linear::proofs::cut::End;
    let reason = match end {
        End::CutFree => None,
        End::Refused(refusal) => Some(crate::Error::Refused(refusal.clone())),
    };
    EliminationForm {
        version,
        net,
        steps: steps
            .iter()
            .map(|step| StepForm {
                cut: (step.cut.0.get(), step.cut.1.get()),
                case: step.case,
                above: step.above.map(VertexId::get),
            })
            .collect(),
        end: if reason.is_some() {
            "refused"
        } else {
            "cut_free"
        },
        reason: reason.as_ref(),
    }
    .serialize(serializer)
}

impl Serialize for crate::linear::nets::cut::Elimination<ProofStructure> {
    /// Serializes the elimination as a document: the net reached, at the
    /// level of its form, the steps and how it ended.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut net = form(&self.net);
        let version = net.version.take().unwrap_or(wire::BASE);
        record(serializer, net, version, &self.steps, &self.end)
    }
}

impl Serialize for crate::linear::nets::cut::Elimination<MellStructure> {
    /// Serializes the elimination as a document: the net reached, at the
    /// level of its form, the steps and how it ended.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut net = boxed(&self.net);
        let version = net.version.take().unwrap_or(wire::BOXES);
        record(serializer, net, version, &self.steps, &self.end)
    }
}

impl<'a> Deserialize<'a> for MellStructure {
    /// Deserializes a structure within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for MellStructure {}

impl Readable for MellStructure {
    const FORM: &'static str = "proof structure with boxes";

    /// Reads a structure. Builds the forest of its sequent within the
    /// limits, and validates its vertices, links and weakenings as
    /// [`MellStructure::from_parts`] does, its graph within
    /// `limits.memory_bytes`. Whether it is a proof net is for
    /// [`MellStructure::is_correct`] to say.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let boxed = Boxed::deserialize(deserializer)?;
        let build = || {
            let sequent = crate::linear::Sequent::try_from(boxed.sequent)?;
            let forest = Forest::from_owned(sequent, limits)?;
            let pair = |&(x, y): &(u32, u32)| (VertexId::new(x), VertexId::new(y));
            let parts = Parts {
                vertices: boxed
                    .vertices
                    .iter()
                    .map(|&(o, p, b)| match p {
                        Some(p) => {
                            Placed::new(OccId::new(o), VertexId::new(p), b.map(VertexId::new))
                        }
                        None => Placed::cut(OccId::new(o), b.map(VertexId::new)),
                    })
                    .collect(),
                cuts: boxed.cuts.iter().map(pair).collect(),
                links: boxed.links.iter().map(pair).collect(),
                weakenings: boxed.weakenings.iter().map(pair).collect(),
            };
            let criterion = Criterion {
                mix: boxed.mix,
                order: Order::Commutative,
            };
            MellStructure::from_parts(forest, criterion, &parts, limits)
        };
        build().map_err(wire::fail)
    }
}
