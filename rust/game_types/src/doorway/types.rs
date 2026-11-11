//! The types of `doorway`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a walk's price list, nav.findPath's penalty: a cell in the lane of a door of `open` costs doormanners.lane_dear, any other 1. The doors and the knobs are read live from g when priced; `memo` keeps each cell's first answer, by nav.idx, for this one value
#[derive(Clone, Debug)]
pub enum DoorwayLanePrice {
    Lanes { open: Vec<DoorId>, memo: JsMap<crate::Num, f64> },
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct St {
    pub t: f64,
    pub held_t: f64,
    pub pass: f64,
    pub by: Option<String>,
    pub spot: Option<V3>,
    pub door: Option<DoorId>,
    pub side: f64,
    pub path: Option<u32>,
    pub no_spot: f64,
    pub round_t: f64,
    pub funnel_t: f64,
    pub step: Option<V3>,
    pub step_by: f64,
    pub step_rest: f64,
    pub leaf_x: f64,
    pub leaf_z: f64,
    pub leaf_t: f64,
    pub leaf_seen: f64,
    pub leaf_by: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorwayModSTACKOWN {
    pub leaf: f64,
    pub mark: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rel {
    pub along: f64,
    pub perp: f64,
    pub f: crate::doorway::RelF,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RelF {
    pub hx: f64,
    pub hz: f64,
    pub ux: f64,
    pub uz: f64,
    pub nx: f64,
    pub nz: f64,
    pub w: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorAhead {
    pub d: DoorId,
    pub side: f64,
    pub dist: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FriendsItem {
    pub pos: V3,
    pub vel: V3,
    pub gd: Option<GuardId>,
    pub id: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Through {
    pub dist: f64,
    pub side: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DoorwayMod {
    #[is(crate::doorway::DoorwayModSTACKOWN { leaf: 0.0, mark: 0.0 })] pub stack_own: crate::doorway::DoorwayModSTACKOWN,
    pub states: JsMap<GuardId, crate::doorway::St>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DoorwayStats {
    pub marks: i32,
}

pub const ROUND_PACE: f64 = 0.5;
