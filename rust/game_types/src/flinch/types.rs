//! The types of `flinch`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FlinchModStateOfValue {
    pub under: f64,
    pub at: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct FlinchMod {
    pub state_of: JsMap<GuardId, crate::flinch::FlinchModStateOfValue>,
}
impl FlinchMod {
    pub fn blank() -> Self {
        FlinchMod {
            state_of: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FlinchStats {
    pub ducks: i32,
    pub broke: i32,
    pub last: Option<Vec<f64>>,
}
