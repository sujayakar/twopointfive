//! The types of `noshove`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Shoved, src/game/noshove.ts:54)
#[derive(Clone, Debug)]
pub struct Shoved {
    pub char: crate::noshove::ShovedChar,
    pub shoved: [f64; 2],
    pub shoved_t: f64,
    pub shoved_from: [f64; 2],
}
impl Shoved {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Shoved {
            char: crate::noshove::ShovedChar::zeroed(),
            shoved: [0.0; 2],
            shoved_t: 0.0,
            shoved_from: [0.0; 2],
        }
    }
}
impl Default for Shoved { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/noshove.ts:54 Shoved.char, src/game/noshove.ts:54)
#[derive(Clone, Debug)]
pub struct ShovedChar {
    pub pos: Vec<f64>,
}
impl ShovedChar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShovedChar {
            pos: Vec::new(),
        }
    }
}
impl Default for ShovedChar { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct NoshoveStats {
    pub round: i32,
    pub short: i32,
    pub through: i32,
    pub back: i32,
    pub not_walked: i32,
}

/// (src/game/noshove.ts:44 BREAK)
pub const BREAK: bool = false;
