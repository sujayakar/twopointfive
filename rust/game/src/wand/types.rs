//! The types of `wand`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Wanding, src/game/wand.ts:86)
#[derive(Clone, Copy, Debug)]
pub struct Wanding {
    pub door: DoorId,
    pub side: f64,
    pub t: f64,
    pub pan: f64,
    pub live: bool,
    pub off: f64,
    pub age: f64,
}
impl Wanding {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Wanding {
            door: DoorId(0),
            side: 0.0,
            t: 0.0,
            pan: 0.0,
            live: false,
            off: 0.0,
            age: 0.0,
        }
    }
}
impl Default for Wanding { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wand.ts:108 wandLens ->, src/game/wand.ts:108)
#[derive(Clone, Copy, Debug)]
pub struct WandLens {
    pub pos: V3,
    pub yaw: f64,
}
impl WandLens {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WandLens {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for WandLens { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WandStats {
    pub looks: i32,
    pub live: i32,
    pub marked: i32,
}
