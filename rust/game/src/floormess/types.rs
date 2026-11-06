//! The types of `floormess`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Spread, src/game/floormess.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct Spread {
    pub core: f64,
    pub lobes: f64,
    pub reach: f64,
    pub lobe: f64,
    pub strength: f64,
}
impl Spread {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Spread {
            core: 0.0,
            lobes: 0.0,
            reach: 0.0,
            lobe: 0.0,
            strength: 0.0,
        }
    }
}
impl Default for Spread { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/floormess.ts:24 MESS_LOOK, src/game/floormess.ts:24)
#[derive(Clone, Copy, Debug)]
pub struct MESSLOOK {
    pub film: f64,
    pub grit: f64,
    pub splinter: f64,
}
impl MESSLOOK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MESSLOOK {
            film: 0.0,
            grit: 0.0,
            splinter: 0.0,
        }
    }
}
impl Default for MESSLOOK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/floormess.ts:36 lookOf(d), src/game/floormess.ts:36)
#[derive(Clone, Debug)]
pub struct LookOfD {
    pub kind: String,
    pub param: f64,
}
impl LookOfD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LookOfD {
            kind: String::new(),
            param: 0.0,
        }
    }
}
impl Default for LookOfD { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FloormessStats {
    pub films: i32,
    pub tops: i32,
    pub rubble: i32,
    pub crowded: i32,
    pub shut: i32,
    pub full: i32,
}

/// (src/game/floormess.ts:24 MESS_LOOK: a table nothing writes)
pub fn mess_look() -> crate::floormess::MESSLOOK { crate::floormess::MESSLOOK { film: 6.0, grit: 7.0, splinter: 8.0 } }

/// (src/game/floormess.ts:28 DIRS: a table nothing writes)
pub fn dirs() -> Vec<[f64; 2]> { alloc::vec![[1.0, 0.0], [0.7071, 0.7071], [0.0, 1.0], [-0.7071, 0.7071], [-1.0, 0.0], [-0.7071, -0.7071], [0.0, -1.0], [0.7071, -0.7071]] }

/// (src/game/floormess.ts:29 UP: a table nothing writes)
pub fn up() -> V3 { [0.0, 1.0, 0.0] }

/// (src/game/floormess.ts:29 DOWN: a table nothing writes)
pub fn down() -> V3 { [0.0, -1.0, 0.0] }

/// (src/game/floormess.ts:40 near: a table nothing writes)
pub fn near() -> Vec<i32> { alloc::vec![0; 192] }
