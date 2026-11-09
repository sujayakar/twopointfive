//! The types of `flinch`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FlinchModStateOfValue {
    pub under: f64,
    pub at: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct FlinchMod {
    /// (src/game/flinch.ts:51 stateOf)
    pub state_of: JsMap<GuardId, crate::flinch::FlinchModStateOfValue>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FlinchStats {
    pub ducks: i32,
    pub broke: i32,
    pub last: Option<Vec<f64>>,
}
