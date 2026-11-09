//! The types of `bigkit`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SamBig {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SamBigSpotTool {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SamBigSpot {
    pub pos: V3,
    pub yaw: f64,
    pub at: Option<V3>,
    pub via: Option<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum BigSpotsTool {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SamBigFar {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigEquipKind {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigOutByKind {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BigCarriedC {
    pub id: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum BigCarried {
    Shotgun,
    Hammer,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct BigkitStats {
    pub sam_shells: i32,
    pub sam_shot: i32,
    pub sam_blows: i32,
    pub sam_hammered: i32,
    pub drawn: i32,
    pub slung: i32,
    pub mate_blows: i32,
    pub mate_hammered: i32,
    pub hammers_out: i32,
    pub hammers_in: i32,
    pub sam_walks: i32,
}

/// (src/game/bigkit.ts:63 TAIL)
pub const TAIL: f64 = 0.3;

/// (src/game/bigkit.ts:66 BREAK)
pub const BREAK: bool = false;

/// (src/game/bigkit.ts:109 MOVE_SLACK)
pub const MOVE_SLACK: f64 = 1.2;

/// (src/game/bigkit.ts:109 MOVE_CLEAR)
pub const MOVE_CLEAR: f64 = 0.45;

/// (src/game/bigkit.ts:109 MOVE_WALK)
pub const MOVE_WALK: f64 = 1.5;

/// (src/game/bigkit.ts:109 MOVE_CAP)
pub const MOVE_CAP: f64 = 3.0;

/// (src/game/bigkit.ts:111 MOVE_LINE)
pub const MOVE_LINE: f64 = 0.3;

/// (src/game/bigkit.ts:113 MOVE_ROUND)
pub const MOVE_ROUND: f64 = 0.75;

/// (src/game/bigkit.ts:115 PROUD)
pub const PROUD: f64 = 0.06;

/// (src/game/bigkit.ts:115 HINGES_IN)
pub const HINGES_IN: f64 = 0.22;

/// (src/game/bigkit.ts:120 SPOTS: a table nothing writes)
pub fn spots() -> JsMap<String, crate::bigkit::SPOTSValue> { { let mut t = JsMap::new(); t.set(String::from("hammer"), crate::bigkit::SPOTSValue { jamb: alloc::vec![[0.3, 0.55], [0.18, 0.8]], shoulder: alloc::vec![[0.12, 1.12]] }); t.set(String::from("shotgun"), crate::bigkit::SPOTSValue { jamb: alloc::vec![[0.35, 0.7], [0.22, 0.88]], shoulder: alloc::vec![[0.12, 1.15]] }); t } }

/// (src/game/bigkit.ts:189 CARRY_TAIL)
pub const CARRY_TAIL: f64 = 0.6;

