//! The types of `marks`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/marks.ts:14 Surface, src/game/marks.ts:14)
#[derive(Clone, Copy, Debug)]
pub struct Surface2 {
    pub box_: BoxId,
    pub dynamic: bool,
}
impl Surface2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Surface2 {
            box_: BoxId(0),
            dynamic: false,
        }
    }
}
impl Default for Surface2 { fn default() -> Self { Self::zeroed() } }
