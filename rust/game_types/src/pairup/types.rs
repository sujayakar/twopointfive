//! The types of `pairup`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PairupMod {
    #[is(crate::pairup::PairupModPAIRHELD { row: None })] pub pair_held: crate::pairup::PairupModPAIRHELD,
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

