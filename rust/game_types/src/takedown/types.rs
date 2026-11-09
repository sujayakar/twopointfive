//! The types of `takedown`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Half {
    Blow,
    Kill,
    Partner,
    Victim,
    Fall,
    Breathe,
    Lower,
    ChokeBeats,
    Haul,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TakeFallHow {
    Choke,
    Whip,
    Kill,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct TakedownStats {
    pub blows: i32,
    pub falls: i32,
    pub partner: i32,
    pub breathing: i32,
}

/// (src/game/takedown.ts:53 PARTNER_SECS)
pub const PARTNER_SECS: f64 = 0.6;

/// (src/game/takedown.ts:55 SAM_HIT)
pub const SAM_HIT: f64 = 0.28;

/// (src/game/takedown.ts:114 LAY_REACH)
pub const LAY_REACH: f64 = 0.95;

/// (src/game/takedown.ts:114 LAY_HALF)
pub const LAY_HALF: f64 = 0.3;
