//! The types of `lighting`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DayState {
    pub hour: crate::hours::HourId,
    pub hour_sun_k: f64,
    pub hour_sun_given: bool,
    pub targets: JsMap<FixtureId, bool>,
    pub lights_off: Vec<LightId>,
    pub sun: LightId,
    pub blackout: Option<crate::lighting::DayStateBlackout>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DayStateBlackout {
    pub until: f64,
    pub since: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum SetBlackoutCause {
    Shot,
    Ocp,
    Debug,
    Hand,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LightEventCause {
    Shot,
    Ocp,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct LightingMod {
    /// (src/game/lighting.ts:33 LENS_FROM)
    pub lens_from: JsMap<LightId, V3>,
    /// (src/game/lighting.ts:178 dayStates)
    pub day_states: Option<crate::lighting::DayState>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct LightingStats {
    pub asked: i32,
    pub pulled: i32,
    pub max_back: f64,
}

/// (src/game/lighting.ts:34 LENS_SLANT)
pub const LENS_SLANT: f64 = 0.3;
