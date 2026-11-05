//! The types of `calmtorch`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: TorchState, src/game/calmtorch.ts:45)
#[derive(Clone, Copy, Debug)]
pub struct TorchState {
    pub dark: bool,
    pub read: f64,
    pub against: f64,
    pub ask: f64,
    pub held: f64,
}
impl TorchState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TorchState {
            dark: false,
            read: 0.0,
            against: 0.0,
            ask: 0.0,
            held: 0.0,
        }
    }
}
impl Default for TorchState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/calmtorch.ts:105 calmTorchOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct CalmTorchOf {
    pub dark: bool,
    pub read: f64,
    pub against: f64,
    pub ask: f64,
    pub held: f64,
}
impl CalmTorchOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CalmTorchOf {
            dark: false,
            read: 0.0,
            against: 0.0,
            ask: 0.0,
            held: 0.0,
        }
    }
}
impl Default for CalmTorchOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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
