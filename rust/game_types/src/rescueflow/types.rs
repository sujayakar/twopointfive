//! The types of `rescueflow`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum OpenState {
    #[default] Held,
    Unknown,
    Cleared,
    Hot,
    Looked,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Region {
    pub id: String,
    pub rect: crate::scene_scenarios::Rect,
    pub pts: Vec<V3>,
    pub seen_t: Vec<f64>,
    pub cleared: bool,
    pub hot: JsMap<GuardId, crate::rescueflow::RegionHotValue>,
    pub was_in: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RegionHotValue {
    pub t: f64,
    pub at: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Known {
    pub o: crate::scene_scenarios::RouteOpening,
    pub region: RescueflowRegionId,
    pub lookable: Vec<f64>,
    pub built_open: bool,
    pub share: f64,
    pub state: crate::rescueflow::OpenState,
    pub held: Option<GuardId>,
    pub hot: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Job {
    pub kind: crate::rescueflow::JobKind,
    pub o: crate::scene_scenarios::RouteOpening,
    pub a: OrdersAssignmentId,
    pub t0: f64,
    pub on_t: f64,
    pub chain: Vec<crate::scene_scenarios::RouteOpening>,
    pub said_hot: bool,
    pub said_wait: bool,
    pub leap: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum JobKind {
    #[default] Cover,
    Corridor,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct RescueFlow {
    #[is(crate::rescueflow::RescueFlowKnobs { abreast: 0.6, unpass: 2.0, past: 1.5, max_cover: 2.5, give_up: 7.0, want_secs: 6.0, hot_hold: 6.0, hot_secs: 20.0, hot_empty: 0.6, look_share: 0.6, look_secs: 10.0, clear_share: 0.7, clear_secs: 25.0, eye_m: 16.0, sense_hz: 6.0, sample_y: 0.9, hold_secs: 3.0, wait_m: 9.0, glow: 1.0 })] pub knobs: crate::rescueflow::RescueFlowKnobs,
    pub known: Vec<crate::rescueflow::Known>,
    pub jobs: JsMap<GuardId, crate::rescueflow::Job>,
    pub passed: JsSet<String>,
    pub want: JsMap<String, f64>,
    #[is(crate::rescueflow::RescueFlowStats { takes: 0.0, chained: 0.0 })] pub stats: crate::rescueflow::RescueFlowStats,
    pub regions: JsMap<String, RescueflowRegionId>,
    pub mine: JsSet<OrdersAssignmentId>,
    pub sense_at: f64,
    pub turn: f64,
    pub heard: JsSet<u32>,
    pub built: bool,
    #[is(crate::scene_scenarios::Rect { x0: 0.0, x1: 0.0, z0: 0.0, z1: 0.0 })] pub room: crate::scene_scenarios::Rect,
    pub corr: Option<crate::rescueflow::RescueFlowCorr>,
    pub came_in: Option<crate::rescueflow::RescueFlowCameIn>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueFlowKnobs {
    pub abreast: f64,
    pub unpass: f64,
    pub past: f64,
    pub max_cover: f64,
    pub give_up: f64,
    pub want_secs: f64,
    pub hot_hold: f64,
    pub hot_secs: f64,
    pub hot_empty: f64,
    pub look_share: f64,
    pub look_secs: f64,
    pub clear_share: f64,
    pub clear_secs: f64,
    pub eye_m: f64,
    pub sense_hz: f64,
    pub sample_y: f64,
    pub hold_secs: f64,
    pub wait_m: f64,
    pub glow: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueFlowStats {
    pub takes: f64,
    pub chained: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueFlowCorr {
    pub x: f64,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RescueFlowCameIn {
    pub region: String,
    pub o: crate::scene_scenarios::RouteOpening,
    pub t: f64,
}

pub fn colour() -> JsMap<String, V3> { { let mut t = JsMap::new(); t.set(String::from("unknown"), [1.0, 0.55, 0.08]); t.set(String::from("looked"), [0.6, 0.62, 0.5]); t.set(String::from("hot"), [2.6, 0.12, 0.05]); t.set(String::from("held"), [0.12, 0.5, 1.6]); t.set(String::from("cleared"), [0.1, 1.0, 0.3]); t } }

pub const EYE: f64 = 1.55;

