//! The types of `walkahead`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Ahead, src/game/walkahead.ts:30)
#[derive(Clone, Copy, Debug)]
pub struct Ahead {
    pub pos: V3,
    pub vel: V3,
    pub aim_yaw: f64,
    pub body_yaw: f64,
    pub crouch: bool,
    pub follow: V3,
    pub aim: Option<V3>,
    pub free: bool,
}
impl Ahead {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Ahead {
            pos: [0.0; 3],
            vel: [0.0; 3],
            aim_yaw: 0.0,
            body_yaw: 0.0,
            crouch: false,
            follow: [0.0; 3],
            aim: None,
            free: false,
        }
    }
}
impl Default for Ahead { fn default() -> Self { Self::zeroed() } }
