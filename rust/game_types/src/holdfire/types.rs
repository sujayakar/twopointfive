//! The types of `holdfire`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum LiftWhy {
    #[default] Shot,
    Order,
    Drill,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub lift_t: f64,
    pub why: Option<crate::holdfire::LiftWhy>,
    pub cast: Vec<GuardId>,
    pub by: Option<PlayerId>,
    pub loud_t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HoldFireOrderO {
    pub waits: Option<bool>,
    pub blown: Option<bool>,
    pub why: Option<crate::holdfire::LiftWhy>,
    pub by: Option<PlayerId>,
    pub silent: Option<bool>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct HoldfireMod {
    /// (src/game/holdfire.ts:74 states)
    pub states: JsMap<PlayerId, crate::holdfire::State>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct HoldfireStats {
    pub held: i32,
    pub held_glass: i32,
    pub lifts: [i32; 3],
}

