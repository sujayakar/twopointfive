//! The types of `flinch`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: FlinchSnap, src/game/flinch.ts:114)
#[derive(Clone, Debug)]
pub struct FlinchSnap {
    pub men: Vec<Option<[f64; 2]>>,
}
impl FlinchSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlinchSnap {
            men: Vec::new(),
        }
    }
}
impl Default for FlinchSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/flinch.ts:51 stateOf, src/game/flinch.ts:51)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct FlinchModStateOfValue {
    pub under: f64,
    pub at: f64,
}
impl FlinchModStateOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlinchModStateOfValue {
            under: 0.0,
            at: 0.0,
        }
    }
}
impl Default for FlinchModStateOfValue { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct FlinchMod {
    /// (src/game/flinch.ts:51 stateOf)
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
