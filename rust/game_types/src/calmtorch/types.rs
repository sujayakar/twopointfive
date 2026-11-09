//! The types of `calmtorch`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TorchState {
    pub dark: bool,
    pub read: f64,
    pub against: f64,
    pub ask: f64,
    pub held: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct CalmtorchMod {
    pub state_of: JsMap<GuardId, crate::calmtorch::TorchState>,
    pub posing: bool,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CalmtorchStats {
    pub clicks_on: i32,
    pub clicks_off: i32,
    pub asks: i32,
}
