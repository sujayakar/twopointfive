//! The types of `falling`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// WHAT A FALLING THING LANDS ON (falling.ts): the statics (shards.ts:568), or the statics and the props (debris.ts:107). A pane's glass does not land on furniture.
#[derive(Clone, Debug)]
pub enum FallWorld {
    Statics,
    StaticsAndProps,
}

/// (TypeScript: inline at src/game/falling.ts:12 fallOf ->, src/game/falling.ts:12)
#[derive(Clone, Copy, Debug)]
pub struct FallOf {
    pub t: f64,
    pub pos: V3,
}
impl FallOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FallOf {
            t: 0.0,
            pos: [0.0; 3],
        }
    }
}
impl Default for FallOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/falling.ts:29 flightOf ->, src/game/falling.ts:29)
#[derive(Clone, Copy, Debug)]
pub struct FlightOf {
    pub t: f64,
    pub pos: V3,
}
impl FlightOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlightOf {
            t: 0.0,
            pos: [0.0; 3],
        }
    }
}
impl Default for FlightOf { fn default() -> Self { Self::zeroed() } }

/// (src/game/falling.ts:24 CHORDS)
pub const CHORDS: f64 = 6.0;
