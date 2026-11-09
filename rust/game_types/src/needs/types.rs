//! The types of `needs`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum NeedKind {
    #[default] Smoke,
    Coffee,
    Water,
    Stretch,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NeedSpot {
    pub name: String,
    pub pos: V3,
    pub face: f64,
    pub loop_: Option<Vec<V3>>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Need {
    pub kind: crate::needs::NeedKind,
    pub level: f64,
    pub fill_min: f64,
    pub wp: f64,
    pub trip_m: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum TripPhase {
    #[default] Going,
    Telegraph,
    Dwell,
    Returning,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Trip {
    pub kind: crate::needs::NeedKind,
    pub spot: crate::needs::NeedSpot,
    pub phase: crate::needs::TripPhase,
    pub t0: f64,
    pub depart_at: f64,
    pub left_at: f64,
    pub arrived_at: f64,
    pub dwell_until: f64,
    pub back_at: f64,
    pub wp: f64,
    pub out_yaw: f64,
    pub watched: bool,
    pub told: bool,
    pub partner: Option<GuardId>,
    pub radio: bool,
    pub reply_at: f64,
    pub replied: bool,
    pub necked: bool,
    pub loop_i: f64,
    pub line: String,
    pub gap_before: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NeedMem {
    pub needs: Vec<crate::needs::Need>,
    pub trip: Option<crate::needs::Trip>,
    pub last_back_at: f64,
    pub trips: f64,
    pub cancels: f64,
    pub shaken: bool,
    pub dealt: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct NeedsModFloorOfValue {
    pub last_depart_at: f64,
}

#[derive(Clone, Debug, Default)]
pub struct OwnSpotS {
    pub name: String,
    pub pos: V3,
    pub face: f64,
    pub loop_: Option<Vec<V3>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DeparturePoint {
    pub wp: f64,
    pub m: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct NeedsMod {
    /// (src/game/needs.ts:111 memOf)
    pub mem_of: JsMap<GuardId, crate::needs::NeedMem>,
    /// (src/game/needs.ts:121 floorOf)
    pub floor_of: Option<crate::needs::NeedsModFloorOfValue>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct NeedsStats {
    pub dealt: i32,
    pub scheduled: i32,
    pub forced: i32,
    pub departed: i32,
    pub arrived: i32,
    pub completed: i32,
    pub cancelled: i32,
    pub radioed: i32,
    pub told: i32,
    pub replies: i32,
    pub stuck: i32,
}

/// (src/game/needs.ts:35 NEED_KINDS: a table nothing writes)
pub fn need_kinds() -> Vec<crate::needs::NeedKind> { alloc::vec![crate::needs::NeedKind::Coffee, crate::needs::NeedKind::Smoke, crate::needs::NeedKind::Water, crate::needs::NeedKind::Stretch] }

/// (src/game/needs.ts:39 NEED_SPOTS: a table nothing writes)
pub fn need_spots() -> JsMap<String, crate::needs::NeedSpot> { { let mut t = JsMap::new(); t.set(String::from("coffee"), crate::needs::NeedSpot { name: String::from("the break-room counter"), pos: [34.6, 0.0, 14.4], face: 1.5707963267948966, loop_: None }); t.set(String::from("smoke"), crate::needs::NeedSpot { name: String::from("outside the fire exit"), pos: [37.6, 0.0, 22.4], face: 1.1, loop_: None }); t.set(String::from("water"), crate::needs::NeedSpot { name: String::from("the water cooler"), pos: [26.9, 0.0, 22.8], face: 0.7853981633974483, loop_: None }); t.set(String::from("stretch"), crate::needs::NeedSpot { name: String::from("the lobby"), pos: [8.4, 0.0, 17.0], face: 0.0, loop_: Some(alloc::vec![[10.8, 0.0, 18.6], [9.6, 0.0, 21.4], [7.4, 0.0, 22.8], [5.4, 0.0, 18.4], [8.4, 0.0, 17.0]]) }); t } }

/// (src/game/needs.ts:85 FILL_MIN: a table nothing writes)
pub fn fill_min() -> JsMap<String, [f64; 2]> { { let mut t = JsMap::new(); t.set(String::from("coffee"), [6.0, 9.0]); t.set(String::from("smoke"), [8.0, 12.0]); t.set(String::from("water"), [6.0, 10.0]); t.set(String::from("stretch"), [5.0, 7.0]); t } }

/// (src/game/needs.ts:86 LEVEL0: a table nothing writes)
pub fn level0() -> Vec<f64> { alloc::vec![0.3, 0.7] }

/// (src/game/needs.ts:88 LEVEL_CAP)
pub const LEVEL_CAP: f64 = 1.5;

/// (src/game/needs.ts:88 CALM_AWARENESS)
pub const CALM_AWARENESS: f64 = 0.3;

/// (src/game/needs.ts:88 SCHEDULE_AWARENESS)
pub const SCHEDULE_AWARENESS: f64 = 0.15;

/// (src/game/needs.ts:90 GLANCE_AT)
pub const GLANCE_AT: f64 = 1.1;

/// (src/game/needs.ts:90 GLANCE_SECS)
pub const GLANCE_SECS: f64 = 2.2;

