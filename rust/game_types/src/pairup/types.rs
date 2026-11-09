//! The types of `pairup`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PairUpRule {
    Stay,
    Word,
    OneEye,
    Hunted,
    Rejoin,
    Hands,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PairupModPAIRHELD {
    pub row: Option<bool>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct PairupMod {
    /// (src/game/pairup.ts:72 PAIR_HELD)
    pub pair_held: crate::pairup::PairupModPAIRHELD,
}
impl PairupMod {
    pub fn blank() -> Self {
        PairupMod {
            pair_held: crate::pairup::PairupModPAIRHELD { row: None },
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct PairupStats {
    pub eye_asks: i32,
    pub ways_kept: i32,
    pub rejoins: i32,
    pub caught_off: i32,
    pub stays: i32,
    pub handoffs: i32,
    pub finishes: i32,
}

