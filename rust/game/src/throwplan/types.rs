//! The types of `throwplan`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ThrowFlight, src/game/throwplan.ts:27)
#[derive(Clone, Debug)]
pub struct ThrowFlight {
    pub end: V3,
    pub land: V3,
    pub points: Vec<V3>,
    pub landed: bool,
}
impl ThrowFlight {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowFlight {
            end: [0.0; 3],
            land: [0.0; 3],
            points: Vec::new(),
            landed: false,
        }
    }
}
impl Default for ThrowFlight { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwplan.ts:96 throwTo(lim), src/game/throwplan.ts:96)
#[derive(Clone, Copy, Debug)]
pub struct ThrowToLim {
    pub n: f64,
}
impl ThrowToLim {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowToLim {
            n: 0.0,
        }
    }
}
impl Default for ThrowToLim { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwplan.ts:96 throwTo ->, src/game/throwplan.ts:96)
#[derive(Clone, Debug)]
pub struct ThrowTo {
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}
impl ThrowTo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowTo {
            from: [0.0; 3],
            sol: crate::throwables::ThrowSolution::zeroed(),
            rest: [0.0; 3],
        }
    }
}
impl Default for ThrowTo { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ThrowplanStats {
    pub sims: i32,
}
