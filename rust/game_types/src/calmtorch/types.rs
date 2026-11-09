//! The types of `calmtorch`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TorchState {
    pub dark: bool,
    pub read: f64,
    pub against: f64,
    pub ask: f64,
    pub held: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CalmTorchOf {
    pub dark: bool,
    pub read: f64,
    pub against: f64,
    pub ask: f64,
    pub held: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct CalmtorchMod {
    /// (src/game/calmtorch.ts:46 stateOf)
    pub state_of: JsMap<GuardId, crate::calmtorch::TorchState>,
    /// (src/game/calmtorch.ts:48 posing)
    pub posing: bool,
}
impl CalmtorchMod {
    pub fn blank() -> Self {
        CalmtorchMod {
            state_of: JsMap::new(),
            posing: false,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CalmtorchStats {
    pub clicks_on: i32,
    pub clicks_off: i32,
    pub asks: i32,
}
