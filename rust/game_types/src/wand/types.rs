//! The types of `wand`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Wanding {
    pub door: DoorId,
    pub side: f64,
    pub t: f64,
    pub pan: f64,
    pub live: bool,
    pub off: f64,
    pub age: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WandLens {
    pub pos: V3,
    pub yaw: f64,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WandStats {
    pub looks: i32,
    pub live: i32,
    pub marked: i32,
}
