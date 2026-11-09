//! The types of `fireline`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct St {
    pub since: f64,
    pub t: f64,
    pub way: Option<Vec<V3>>,
    pub look_t: f64,
    pub go_t: f64,
    pub said: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Fired {
    pub gun: String,
    pub shots: f64,
    pub round_t: f64,
    pub gap: f64,
    pub live_t: f64,
    pub rel_t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Lane {
    pub m: V3,
    pub dx: f64,
    pub dz: f64,
    pub back: f64,
    pub k: f64,
    pub half: f64,
    pub energy: Option<f64>,
    pub thin: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FireLineFeet {
    pub goal: Option<V3>,
    pub speed: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct FirelineMod {
    /// (src/game/fireline.ts:79 states)
    pub states: JsMap<GuardId, crate::fireline::St>,
    /// (src/game/fireline.ts:83 fired)
    pub fired: JsMap<PlayerId, crate::fireline::Fired>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FirelineStats {
    pub holds: i32,
    pub held_s: f64,
    pub cues: i32,
    pub ways: i32,
    pub no_way: i32,
    pub fight: i32,
    pub enemy: i32,
}
