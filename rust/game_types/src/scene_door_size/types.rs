//! The types of `scene_door_size`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct DOORSIZE {
    pub gap: f64,
    pub head: f64,
    pub slot_max: f64,
    pub cell: f64,
    pub reach: f64,
}

/// (tools/levels/lib/doorSize.ts:17 DOOR_SIZE: a table nothing writes)
pub fn door_size() -> crate::scene_door_size::DOORSIZE { crate::scene_door_size::DOORSIZE { gap: 0.9, head: 2.05, slot_max: 1.3, cell: 0.5, reach: 0.1 } }

/// (tools/levels/lib/doorSize.ts:31 LEAF_H)
pub const LEAF_H: f64 = 2.01;
