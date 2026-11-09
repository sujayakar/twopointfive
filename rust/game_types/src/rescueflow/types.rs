//! The types of `rescueflow`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum OpenState {
    #[default] Held,
    Unknown,
    Cleared,
    Hot,
    Looked,
}
impl OpenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            OpenState::Held => "held",
            OpenState::Unknown => "unknown",
            OpenState::Cleared => "cleared",
            OpenState::Hot => "hot",
            OpenState::Looked => "looked",
        }
    }
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

#[derive(Clone, Debug, ToBytes)]
pub struct RescueFlow {
    pub knobs: crate::rescueflow::RescueFlowKnobs,
    pub known: Vec<crate::rescueflow::Known>,
    pub jobs: JsMap<GuardId, crate::rescueflow::Job>,
    pub passed: JsSet<String>,
    pub want: JsMap<String, f64>,
    pub stats: crate::rescueflow::RescueFlowStats,
    pub regions: JsMap<String, RescueflowRegionId>,
    pub mine: JsSet<OrdersAssignmentId>,
    pub sense_at: f64,
    pub turn: f64,
    pub heard: JsSet<u32>,
    pub built: bool,
    pub room: crate::scene_scenarios::Rect,
    pub corr: Option<crate::rescueflow::RescueFlowCorr>,
    pub came_in: Option<crate::rescueflow::RescueFlowCameIn>,
}
impl RescueFlow {
    /// what a new one holds
    pub fn blank() -> Self {
        RescueFlow {
            knobs: crate::rescueflow::RescueFlowKnobs { abreast: 0.6, unpass: 2.0, past: 1.5, max_cover: 2.5, give_up: 7.0, want_secs: 6.0, hot_hold: 6.0, hot_secs: 20.0, hot_empty: 0.6, look_share: 0.6, look_secs: 10.0, clear_share: 0.7, clear_secs: 25.0, eye_m: 16.0, sense_hz: 6.0, sample_y: 0.9, hold_secs: 3.0, wait_m: 9.0, glow: 1.0 },
            known: Vec::new(),
            jobs: JsMap::new(),
            passed: JsSet::new(),
            want: JsMap::new(),
            stats: crate::rescueflow::RescueFlowStats { takes: 0.0, chained: 0.0, corridor: 0.0, overridden: 0.0, hot_holds: 0.0, unmanned: 0.0 },
            regions: JsMap::new(),
            mine: JsSet::new(),
            sense_at: 0.0,
            turn: 0.0,
            heard: JsSet::new(),
            built: false,
            room: crate::scene_scenarios::Rect { x0: 0.0, x1: 0.0, z0: 0.0, z1: 0.0 },
            corr: None,
            came_in: None,
        }
    }
}
impl Default for RescueFlow { fn default() -> Self { Self::blank() } }

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
    pub corridor: f64,
    pub overridden: f64,
    pub hot_holds: f64,
    pub unmanned: f64,
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

/// (src/game/rescueflow.ts:68 COLOUR: a table nothing writes)
pub fn colour() -> JsMap<String, V3> { { let mut t = JsMap::new(); t.set(String::from("unknown"), [1.0, 0.55, 0.08]); t.set(String::from("looked"), [0.6, 0.62, 0.5]); t.set(String::from("hot"), [2.6, 0.12, 0.05]); t.set(String::from("held"), [0.12, 0.5, 1.6]); t.set(String::from("cleared"), [0.1, 1.0, 0.3]); t } }

/// (src/game/rescueflow.ts:69 EYE)
pub const EYE: f64 = 1.55;

