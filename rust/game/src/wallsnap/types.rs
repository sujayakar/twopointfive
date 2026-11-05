//! The types of `wallsnap`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/wallsnap.ts:125 wsCornerAngle ->, src/game/wallsnap.ts:125)
#[derive(Clone, Copy, Debug)]
pub struct WsCornerAngle {
    pub deg: f64,
    pub dist: f64,
}
impl WsCornerAngle {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WsCornerAngle {
            deg: 0.0,
            dist: 0.0,
        }
    }
}
impl Default for WsCornerAngle { fn default() -> Self { Self::zeroed() } }

/// (src/game/wallsnap.ts:26 LEAN_GUN_M)
pub const LEAN_GUN_M: f64 = 0.42;
