//! The types of `doorpath`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Walk {
    pub gd: GuardId,
    pub op: DoorOpId,
    pub door: DoorId,
    pub pod: V3,
    pub n: f64,
    pub t0: f64,
    pub pts: Vec<V3>,
    pub len: Vec<f64>,
    pub s: f64,
    pub laid_t: f64,
    pub near: Vec<crate::doorpath::WalkNearItem>,
    pub lane: f64,
    pub via: Vec<V3>,
    pub arr: Option<u32>,
    pub arr_n: f64,
    pub yield_t: f64,
    pub yields: f64,
    pub held: f64,
    pub off_t: f64,
    pub round: bool,
    pub wait: bool,
    pub waited: f64,
    pub in_m: f64,
    pub pod0: V3,
    pub body_t: f64,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WalkNearItem {
    pub d: DoorId,
    pub angle: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub struct DoorPath {
    pub walks: JsMap<GuardId, crate::doorpath::Walk>,
    pub last_of: JsMap<DoorOpId, crate::doorpath::DoorPathLastOfValue>,
    pub stats: crate::doorpath::DoorPathStats,
}
impl DoorPath {
    /// what a new one holds
    pub fn blank() -> Self {
        DoorPath {
            walks: JsMap::new(),
            last_of: JsMap::new(),
            stats: crate::doorpath::DoorPathStats { sent: 0.0, walked: 0.0, round: 0.0, no_round: 0.0, no_lane: 0.0, late: 0.0, held_s: 0.0, capped: 0.0, yields: 0.0, gave_up: 0.0, taken: 0.0, relaid: 0.0, turned: 0.0, moved: 0.0, wait_s: 0.0, nav: 0.0, stepped: 0.0 },
        }
    }
}
impl Default for DoorPath { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorPathLastOfValue {
    pub w: Option<(GuardId, u32)>,
    pub n: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorPathStats {
    pub sent: f64,
    pub walked: f64,
    pub round: f64,
    pub no_round: f64,
    pub no_lane: f64,
    pub late: f64,
    pub held_s: f64,
    pub capped: f64,
    pub yields: f64,
    pub gave_up: f64,
    pub taken: f64,
    pub relaid: f64,
    pub turned: f64,
    pub moved: f64,
    pub wait_s: f64,
    pub nav: f64,
    pub stepped: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DoorPathRoundLeaves {
    pub pts: Vec<V3>,
    pub round: bool,
}

#[derive(Clone, Debug, Default)]
pub struct DoorPathWay {
    pub pts: Vec<V3>,
    pub round: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorPathMayGoQ {
    pub gd: GuardId,
    pub pod: crate::doorpath::DoorPathMayGoQPod,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorPathMayGoQPod {
    pub pos: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorPathSentPod {
    pub pos: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorPathDropWhy {
    Taken,
    GaveUp,
}
impl DoorPathDropWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorPathDropWhy::Taken => "taken",
            DoorPathDropWhy::GaveUp => "gaveUp",
        }
    }
}
