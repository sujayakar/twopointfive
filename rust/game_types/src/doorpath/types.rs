//! The types of `doorpath`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
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
    /// dealt as the walk is laid: what holds a walk's serial asks by it whether the man's live walk is still that one
    pub serial: u32,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WalkNearItem {
    pub d: DoorId,
    pub angle: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DoorPath {
    pub walks: JsMap<GuardId, crate::doorpath::Walk>,
    pub last_of: JsMap<DoorOpId, crate::doorpath::DoorPathLastOfValue>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorPathLastOfValue {
    pub w: Option<(GuardId, u32)>,
    pub n: f64,
    pub t: f64,
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
