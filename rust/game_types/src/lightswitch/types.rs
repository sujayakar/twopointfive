//! The types of `lightswitch`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallSwitch {
    pub id: f64,
    pub pos: V3,
    pub n: V3,
    pub stand: V3,
    pub floor: V3,
    pub door: DoorId,
    pub targets: Vec<FixtureId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Hand {
    pub c: CharId,
    pub sw: crate::lightswitch::WallSwitch,
    pub at: f64,
    pub dir: f64,
    pub dur: f64,
    pub plant: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct St {
    pub all: Vec<crate::lightswitch::WallSwitch>,
    pub list: Vec<crate::lightswitch::WallSwitch>,
    pub hands: Vec<crate::lightswitch::Hand>,
    pub serial: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Plate {
    pub door: DoorId,
    pub n: V3,
    pub pos: V3,
    pub floor: V3,
    pub stand: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SwitchHand {
    pub p: V3,
    pub n: V3,
}

#[derive(Clone, Debug, Default)]
pub struct SwitchRowsRowOver {
    pub kind: Option<crate::lightswitch::SwitchRowsRowOverKind>,
    pub pos: Option<V3>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub in_reach: Option<bool>,
    pub off: Option<bool>,
    pub progress: Option<f64>,
    pub target: Option<FixtureId>,
    pub door: Option<DoorId>,
    pub guard: Option<GuardId>,
    pub verb: Option<crate::game::Verb>,
    pub running: Option<bool>,
    pub close: Option<crate::close::CloseKind>,
    pub past: Option<bool>,
    pub framed: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum SwitchRowsRowOverKind {
    Cam,
    Guard,
    Breaker,
    Door,
    Light,
    Body,
    Objective,
    Held,
    Wall,
    Switch,
    Charge,
    Cancel,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct LightswitchMod {
    /// (src/game/lightswitch.ts:90 states)
    pub states: Option<crate::lightswitch::St>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct LightswitchStats {
    pub plates: i32,
    pub flips: i32,
    pub guard_on: i32,
    pub events: i32,
    pub gone: i32,
}

/// (src/game/lightswitch.ts:121 WALL_TOP)
pub const WALL_TOP: f64 = 2.2;

/// (src/game/lightswitch.ts:303 PLATE: a table nothing writes)
pub fn plate() -> V3 { [0.62, 0.6, 0.55] }

/// (src/game/lightswitch.ts:303 ROCKER: a table nothing writes)
pub fn rocker() -> V3 { [0.74, 0.72, 0.66] }

/// (src/game/lightswitch.ts:303 GLOW: a table nothing writes)
pub fn glow() -> V3 { [1.6, 0.8, 0.16] }

/// (src/game/lightswitch.ts:304 SW_KEY)
pub const SW_KEY: f64 = 4194304.0;
