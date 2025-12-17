use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TriptyxId { pub sch: String, pub uuid: String, pub cuid: String }

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Position { pub lat: f64, pub lon: f64, pub alt_m: f64, pub sigma_m: f64 }

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OodaPhase { Sense, Orient, Decide, Act }

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActivityState { Investigating, AtRest, Converging, OutBelow, OutAbove }

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Convergence { pub inst: f64, pub ema: f64, pub snr: f64 }
impl Convergence { pub fn new() -> Self { Self { inst: 0.0, ema: 0.0, snr: 0.0 } } }

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Timestamp { pub last_update: OffsetDateTime }

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Scales { pub graph_scale: f64, pub gis_scale_m: f64, pub time_scale_s: f64, pub speed_ref_mps: f64 }
impl Default for Scales {
    fn default() -> Self { Self { graph_scale: 8.0, gis_scale_m: 1000.0, time_scale_s: 60.0, speed_ref_mps: 5.0 } }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Velocity { pub mps: f64 }

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CorePrimitive { Actor, Object, Event, Concept, Attribute }

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PolePrimitive { Person, Object, Location, Event }

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OntologyMode { AgnosticCore, POLE }

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeKind { Actor, Object, Location, Event, Concept, Attribute }

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NodeMeta { pub mode: OntologyMode, pub core: Option<CorePrimitive>, pub pole: Option<PolePrimitive> }
impl NodeMeta {
    pub fn kind(&self) -> NodeKind {
        match self.mode {
            OntologyMode::AgnosticCore => match self.core.unwrap_or(CorePrimitive::Object) {
                CorePrimitive::Actor => NodeKind::Actor,
                CorePrimitive::Object => NodeKind::Object,
                CorePrimitive::Event => NodeKind::Event,
                CorePrimitive::Concept => NodeKind::Concept,
                CorePrimitive::Attribute => NodeKind::Attribute,
            },
            OntologyMode::POLE => match self.pole.unwrap_or(PolePrimitive::Object) {
                PolePrimitive::Person => NodeKind::Actor,
                PolePrimitive::Object => NodeKind::Object,
                PolePrimitive::Location => NodeKind::Location,
                PolePrimitive::Event => NodeKind::Event,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ConceptualAtom { pub p: f64, pub t: f64, pub e: f64, pub s: f64, pub r: f64, pub phi: f64 }
impl ConceptualAtom {
    pub fn snr(&self) -> f64 {
        let signal = self.p + self.e + self.s;
        let noise = self.t + self.r + self.phi + 1e-6;
        signal / noise
    }
}

#[derive(Clone, Debug, Default)]
pub struct DetectionSummary {
    pub counts: std::collections::HashMap<ActivityState, usize>,
    pub counts_by_kind: std::collections::HashMap<NodeKind, std::collections::HashMap<ActivityState, usize>>,
}
