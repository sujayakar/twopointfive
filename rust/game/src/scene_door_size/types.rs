//! The types of `scene_door_size`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/scene/doorSize.ts:17 DOOR_SIZE, src/scene/doorSize.ts:17)
#[derive(Clone, Copy, Debug)]
pub struct DOORSIZE {
    pub gap: f64,
    pub head: f64,
    pub slot_max: f64,
    pub cell: f64,
    pub reach: f64,
}
impl DOORSIZE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DOORSIZE {
            gap: 0.0,
            head: 0.0,
            slot_max: 0.0,
            cell: 0.0,
            reach: 0.0,
        }
    }
}
impl Default for DOORSIZE { fn default() -> Self { Self::zeroed() } }

/// (src/scene/doorSize.ts:17 DOOR_SIZE: a table nothing writes)
pub fn door_size() -> crate::scene_door_size::DOORSIZE { crate::scene_door_size::DOORSIZE { gap: 0.9, head: 2.05, slot_max: 1.3, cell: 0.5, reach: 0.1 } }

/// (src/scene/doorSize.ts:31 LEAF_H)
pub const LEAF_H: f64 = 2.01;
