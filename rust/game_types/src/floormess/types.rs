//! The types of `floormess`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct Spread {
    pub core: f64,
    pub lobes: f64,
    pub reach: f64,
    pub lobe: f64,
    pub strength: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MESSLOOK {
    pub film: f64,
    pub grit: f64,
    pub splinter: f64,
}

#[derive(Clone, Debug, Default)]
pub struct LookOfD {
    pub kind: String,
    pub param: f64,
}

/// (src/game/floormess.ts:24 MESS_LOOK: a table nothing writes)
pub fn mess_look() -> crate::floormess::MESSLOOK { crate::floormess::MESSLOOK { film: 6.0, grit: 7.0, splinter: 8.0 } }

/// (src/game/floormess.ts:28 DIRS: a table nothing writes)
pub fn dirs() -> Vec<[f64; 2]> { alloc::vec![[1.0, 0.0], [0.7071, 0.7071], [0.0, 1.0], [-0.7071, 0.7071], [-1.0, 0.0], [-0.7071, -0.7071], [0.0, -1.0], [0.7071, -0.7071]] }

/// (src/game/floormess.ts:29 UP: a table nothing writes)
pub fn up() -> V3 { [0.0, 1.0, 0.0] }

/// (src/game/floormess.ts:29 DOWN: a table nothing writes)
pub fn down() -> V3 { [0.0, -1.0, 0.0] }

/// (src/game/floormess.ts:40 near: a table nothing writes)
pub fn near() -> Vec<i32> { alloc::vec![0; 192] }
