use legion::*;
use std::collections::HashMap;
use time::OffsetDateTime;

use crate::components::*;
use crate::gis;
use crate::graph::SlotGraphView;

fn estimate_speed(_prev: &Position, _curr: &Position, dt: f64) -> f64 { if dt <= 0.0 { 0.0 } else { 0.0 } }

pub fn update_convergence_system() -> impl Schedulable {
    SystemBuilder::new("update_convergence")
        .read_resource::<Scales>()
        .with_query(<(Entity, &Position, &mut Convergence, Option<&mut Velocity>, Option<&mut Timestamp>, Option<&ConceptualAtom>)>::query())
        .build(move |cmd, world, scales, query| {
            let view = SlotGraphView::build(world);
            for (e, pos, mut conv, vel, ts, atom) in query.iter_mut(world) {
                let now = OffsetDateTime::now_utc();
                let idx = view.ent_to_idx.get(e).copied();
                let degree_norm = idx.map(|i| view.degree_norm(i, scales.graph_scale)).unwrap_or(0.0);
                let speed_norm = vel.map(|v| v.mps / scales.speed_ref_mps).unwrap_or(0.0).clamp(0.0, 1.0);
                let inst = (0.5 * degree_norm) + (0.5 * (1.0 - speed_norm));
                conv.inst = inst.clamp(0.0, 1.0);
                conv.ema = 0.8 * conv.ema + 0.2 * conv.inst;
                conv.snr = atom.map(|a| a.snr()).unwrap_or(conv.inst / (1.0 - conv.inst + 1e-6));
                if ts.is_none() { cmd.add_component(*e, Timestamp { last_update: now }); }
            }
        })
}

pub fn classify_state_system() -> impl Schedulable {
    SystemBuilder::new("classify_state")
        .with_query(<(&mut ActivityState, &Convergence)>::query())
        .build(move |_cmd, _world, _res, query| {
            for (state, conv) in query.iter_mut(_world) {
                let ema = conv.ema;
                *state = if ema >= 0.7 { ActivityState::OutAbove }
                else if ema <= 0.35 { ActivityState::OutBelow }
                else if ema > 0.55 { ActivityState::Converging }
                else if ema < 0.45 { ActivityState::Investigating }
                else { ActivityState::AtRest };
            }
        })
}

pub fn aggregate_detection_system() -> impl Schedulable {
    SystemBuilder::new("aggregate_detection")
        .write_resource::<DetectionSummary>()
        .with_query(<(&ActivityState, &NodeMeta)>::query())
        .build(move |_cmd, _world, summary, query| {
            let mut counts: HashMap<ActivityState, usize> = HashMap::new();
            let mut by_kind: HashMap<NodeKind, HashMap<ActivityState, usize>> = HashMap::new();
            for (state, meta) in query.iter(_world) {
                *counts.entry(*state).or_default() += 1;
                let kind = meta.kind();
                *by_kind.entry(kind).or_default().entry(*state).or_default() += 1;
            }
            summary.counts = counts;
            summary.counts_by_kind = by_kind;
        })
}
