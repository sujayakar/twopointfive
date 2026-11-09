//! The types of `spotted`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum SpottedMode {
    #[default] Calm,
    Wary,
    Caught,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Man {
    pub mode: crate::spotted::SpottedMode,
    pub since: f64,
    pub wary: Vec<GuardId>,
    pub by: Option<GuardId>,
    pub eyes: Option<GuardId>,
    pub wary_t: f64,
    pub caught_t: f64,
    pub spot: Option<V3>,
    pub what: Option<crate::spotted::ManWhat>,
    pub ask_t: f64,
    pub exposed: bool,
    pub trail: bool,
    pub said_t: f64,
    pub real: bool,
    pub told: bool,
    pub take: Option<crate::spotted::ManTake>,
    pub take_no: Option<crate::spotted::ManTakeNo>,
    pub real_t: f64,
    pub tick_t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum ManWhat {
    Step,
    Freeze,
    Slip,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ManTake {
    pub foe: GuardId,
    pub t0: f64,
    pub born: f64,
    pub spot: V3,
    pub asked: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ManTakeNo {
    pub foe: GuardId,
    pub until: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SpottedModSPOTTEDLOGItem {
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SpottedModDropsValue {
    pub down: JsSet<GuardId>,
    pub at: Vec<crate::spotted::SpottedModDropsValueAtItem>,
    pub seen: bool,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SpottedModDropsValueAtItem {
    pub pos: V3,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SpottedFeet {
    pub goal: Option<V3>,
    pub speed: Option<f64>,
    pub crouch: bool,
    pub watch: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FreshItem {
    pub pos: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SpottedMod {
    /// (src/game/spotted.ts:119 men)
    pub men: JsMap<GuardId, crate::spotted::Man>,
    /// (src/game/spotted.ts:122 SPOTTED_LOG)
    pub spotted_log: Vec<crate::spotted::SpottedModSPOTTEDLOGItem>,
    /// (src/game/spotted.ts:312 drops)
    pub drops: Option<crate::spotted::SpottedModDropsValue>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SpottedStats {
    pub wary: i32,
    pub caught: i32,
    pub calm: i32,
    pub slips: i32,
    pub freezes: i32,
    pub steps: i32,
    pub held: i32,
    pub held_unaware: i32,
    pub keen: i32,
    pub cues: i32,
    pub braces: i32,
    pub takes: i32,
    pub taken: i32,
}

