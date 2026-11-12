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
    pub a: TaskId,
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
    pub known: Vec<crate::rescueflow::Known>,
    pub jobs: JsMap<GuardId, crate::rescueflow::Job>,
    pub passed: JsSet<String>,
    pub want: JsMap<String, f64>,
    #[is(crate::rescueflow::RescueFlowStats { takes: 0.0, chained: 0.0 })] pub stats: crate::rescueflow::RescueFlowStats,
    pub regions: JsMap<String, RescueflowRegionId>,
    pub mine: JsSet<TaskId>,
    pub sense_at: f64,
    pub turn: f64,
    pub heard: JsSet<u32>,
    pub built: bool,
    #[is(crate::scene_scenarios::Rect { x0: 0.0, x1: 0.0, z0: 0.0, z1: 0.0 })] pub room: crate::scene_scenarios::Rect,
    pub corr: Option<crate::rescueflow::RescueFlowCorr>,
    pub came_in: Option<crate::rescueflow::RescueFlowCameIn>,
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

