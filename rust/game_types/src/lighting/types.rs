//! The types of `lighting`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LightEventCause {
    Shot,
    Ocp,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct LightingMod {
    pub lens_from: JsMap<LightId, V3>,
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
    pub max_back: f64,
}

pub const LENS_SLANT: f64 = 0.3;
