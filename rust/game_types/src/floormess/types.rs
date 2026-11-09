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

/// (src/game/floormess.ts:24 MESS_LOOK: a table nothing writes)
pub fn mess_look() -> crate::floormess::MESSLOOK { crate::floormess::MESSLOOK { film: 6.0, grit: 7.0, splinter: 8.0 } }

