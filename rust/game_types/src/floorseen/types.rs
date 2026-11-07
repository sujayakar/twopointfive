//! The types of `floorseen`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: FloorSeen, src/game/floorseen.ts:18)
#[derive(Clone, Debug)]
pub struct FloorSeen {
    pub at: JsMap<GuardId, Vec<f32>>,
    pub asked: JsMap<GuardId, Vec<f32>>,
    pub frame: f64,
}
impl FloorSeen {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        FloorSeen {
            at: JsMap::new(),
            asked: JsMap::new(),
            frame: 0.0,
        }
    }
}
impl Default for FloorSeen { fn default() -> Self { Self::blank() } }

/// (src/game/floorseen.ts:16 EVERY)
pub const EVERY: f64 = 12.0;

/// (src/game/floorseen.ts:16 REFRESH)
pub const REFRESH: f64 = 1.0;

/// (src/game/floorseen.ts:16 NEVER)
pub const NEVER: f64 = -1000000000.0;
