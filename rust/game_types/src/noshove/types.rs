//! The types of `noshove`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct Shoved {
    pub char: crate::noshove::ShovedChar,
    pub shoved: [f64; 2],
    pub shoved_t: f64,
    pub shoved_from: [f64; 2],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ShovedChar {
    pub pos: V3,
}

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
