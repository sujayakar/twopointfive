//! The types of `workstow`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/workstow.ts:91 stowFight(A), src/game/workstow.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct StowFightA {
    pub yield_t: f64,
}
impl StowFightA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StowFightA {
            yield_t: 0.0,
        }
    }
}
impl Default for StowFightA { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workstow.ts:91 stowFight(op), src/game/workstow.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct StowFightOp {
    pub stow_off_t: Option<f64>,
}
impl StowFightOp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StowFightOp {
            stow_off_t: None,
        }
    }
}
impl Default for StowFightOp { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WorkstowStats {
    pub sam_down: i32,
    pub sam_up: i32,
    pub sam_kept: i32,
    pub mate_down: i32,
    pub mate_up: i32,
    pub mate_fired: i32,
}
