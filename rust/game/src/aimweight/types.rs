//! The types of `aimweight`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: AimWeight, src/game/aimweight.ts:42)
#[derive(Clone, Copy, Debug)]
pub struct AimWeight {
    pub vel: f64,
    pub r: f64,
    pub y: f64,
    pub live: bool,
    pub was: bool,
    pub landed: bool,
    pub on: bool,
    pub err: f64,
    pub launch_t: f64,
}
impl AimWeight {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        AimWeight {
            vel: 0.0,
            r: 0.0,
            y: 0.0,
            live: false,
            was: false,
            landed: true,
            on: true,
            err: 0.0,
            launch_t: 0.0,
        }
    }
}
impl Default for AimWeight { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/aimweight.ts:65 aimSwing(w), src/game/aimweight.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct AimSwingW {
    pub vel: f64,
}
impl AimSwingW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AimSwingW {
            vel: 0.0,
        }
    }
}
impl Default for AimSwingW { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct AimweightStats {
    pub steps: i32,
    pub off_shots: i32,
}
