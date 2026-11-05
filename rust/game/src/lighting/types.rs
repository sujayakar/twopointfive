//! The types of `lighting`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: DayState, src/game/lighting.ts:180)
#[derive(Clone, Debug)]
pub struct DayState {
    pub hour: crate::hours::HourId,
    pub hour_sun_k: f64,
    pub hour_sun_given: bool,
    pub targets: JsMap<FixtureId, bool>,
    pub lights_off: Vec<LightId>,
    pub sky: crate::lighting::DayStateSky,
    pub haze: f64,
    pub exposure: f64,
    pub sun: LightId,
    pub blackout: Option<crate::lighting::DayStateBlackout>,
}
impl DayState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DayState {
            hour: crate::hours::HourId::Overcast,
            hour_sun_k: 0.0,
            hour_sun_given: false,
            targets: JsMap::new(),
            lights_off: Vec::new(),
            sky: crate::lighting::DayStateSky::zeroed(),
            haze: 0.0,
            exposure: 0.0,
            sun: LightId(0),
            blackout: None,
        }
    }
}
impl Default for DayState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lighting.ts:180 DayState.sky, src/game/lighting.ts:180)
#[derive(Clone, Copy, Debug)]
pub struct DayStateSky {
    pub zenith: V3,
    pub horizon: V3,
}
impl DayStateSky {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DayStateSky {
            zenith: [0.0; 3],
            horizon: [0.0; 3],
        }
    }
}
impl Default for DayStateSky { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lighting.ts:180 DayState.blackout, src/game/lighting.ts:180)
#[derive(Clone, Copy, Debug)]
pub struct DayStateBlackout {
    pub active: bool,
    pub permanent: bool,
    pub until: f64,
    pub since: f64,
}
impl DayStateBlackout {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DayStateBlackout {
            active: false,
            permanent: false,
            until: 0.0,
            since: 0.0,
        }
    }
}
impl Default for DayStateBlackout { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SetBlackoutCause {
    Shot,
    Ocp,
    Debug,
    Hand,
}
impl SetBlackoutCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            SetBlackoutCause::Shot => "shot",
            SetBlackoutCause::Ocp => "ocp",
            SetBlackoutCause::Debug => "debug",
            SetBlackoutCause::Hand => "hand",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LightEventCause {
    Shot,
    Ocp,
}
impl LightEventCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            LightEventCause::Shot => "shot",
            LightEventCause::Ocp => "ocp",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct LightingMod {
    /// (src/game/lighting.ts:33 LENS_FROM)
    pub lens_from: JsMap<LightId, V3>,
    /// (src/game/lighting.ts:181 dayStates)
    pub day_states: Option<crate::lighting::DayState>,
}
impl LightingMod {
    pub fn blank() -> Self {
        LightingMod {
            lens_from: JsMap::new(),
            day_states: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct LightingStats {
    pub asked: i32,
    pub pulled: i32,
    pub max_back: i32,
}

/// (src/game/lighting.ts:34 LENS_SLANT)
pub const LENS_SLANT: f64 = 0.3;
