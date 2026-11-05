//! The types of `aimcost`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: RoundLeg, src/game/aimcost.ts:72)
#[derive(Clone, Copy, Debug)]
pub struct RoundLeg {
    pub o: V3,
    pub d: V3,
    pub end: f64,
}
impl RoundLeg {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoundLeg {
            o: [0.0; 3],
            d: [0.0; 3],
            end: 0.0,
        }
    }
}
impl Default for RoundLeg { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct AimcostStats {
    pub lamp_hits: i32,
    pub lamp_near: i32,
}
