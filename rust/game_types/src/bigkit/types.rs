//! The types of `bigkit`.
#![allow(unused_imports)]
use crate::{Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct BigSpot {
    pub pos: V3,
    pub yaw: f64,
    pub at: V3,
    pub hinge: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SPOTSValue {
    pub jamb: Vec<[f64; 2]>,
    pub shoulder: Vec<[f64; 2]>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SamBigSpot {
    pub pos: V3,
    pub yaw: f64,
    pub at: Option<V3>,
    pub via: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BigCarriedC {
    pub id: f64,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct BigkitStats {
    pub sam_shells: i32,
    pub sam_blows: i32,
    pub mate_blows: i32,
    pub mate_hammered: i32,
}

pub const TAIL: f64 = 0.3;

pub const BREAK: bool = false;

pub const MOVE_SLACK: f64 = 1.2;

pub const MOVE_CLEAR: f64 = 0.45;

pub const MOVE_WALK: f64 = 1.5;

pub const MOVE_CAP: f64 = 3.0;

pub const MOVE_LINE: f64 = 0.3;

pub const MOVE_ROUND: f64 = 0.75;

pub const PROUD: f64 = 0.06;

pub const HINGES_IN: f64 = 0.22;

pub fn spots() -> JsMap<String, crate::bigkit::SPOTSValue> { { let mut t = JsMap::new(); t.set(String::from("hammer"), crate::bigkit::SPOTSValue { jamb: alloc::vec![[0.3, 0.55], [0.18, 0.8]], shoulder: alloc::vec![[0.12, 1.12]] }); t.set(String::from("shotgun"), crate::bigkit::SPOTSValue { jamb: alloc::vec![[0.35, 0.7], [0.22, 0.88]], shoulder: alloc::vec![[0.12, 1.15]] }); t } }

pub const CARRY_TAIL: f64 = 0.6;

