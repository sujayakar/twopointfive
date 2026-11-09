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

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct TakedownMod {
    pub breathing: bool,
}
impl TakedownMod {
    pub fn blank() -> Self {
        TakedownMod {
            breathing: false,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct TakedownStats {
    pub blows: i32,
    pub falls: i32,
    pub partner: i32,
    pub breathing: i32,
}

pub const PARTNER_SECS: f64 = 0.6;

pub const SAM_HIT: f64 = 0.28;

pub const LAY_REACH: f64 = 0.95;

pub const LAY_HALF: f64 = 0.3;
