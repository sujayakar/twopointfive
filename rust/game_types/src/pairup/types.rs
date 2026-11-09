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
impl PairUpRule {
    pub const fn as_str(self) -> &'static str {
        match self {
            PairUpRule::Stay => "stay",
            PairUpRule::Word => "word",
            PairUpRule::OneEye => "oneEye",
            PairUpRule::Hunted => "hunted",
            PairUpRule::Rejoin => "rejoin",
            PairUpRule::Hands => "hands",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PAIRUPDEFAULTS {
    pub on: bool,
    pub one_eye: bool,
    pub hunted: bool,
    pub rejoin: bool,
    pub stay: bool,
    pub word: bool,
    pub hands: bool,
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

/// (src/game/pairup.ts:62 PAIRUP_DEFAULTS: a table nothing writes)
pub fn pairup_defaults() -> crate::pairup::PAIRUPDEFAULTS { crate::pairup::PAIRUPDEFAULTS { on: false, one_eye: true, hunted: true, rejoin: true, stay: true, word: true, hands: true } }

/// (src/game/pairup.ts:69 LS)
pub const LS: &str = "twopointsix.pair";

/// (src/game/pairup.ts:73 RULES: a table nothing writes)
pub fn rules() -> [String; 6] { [String::from("oneEye"), String::from("hunted"), String::from("rejoin"), String::from("stay"), String::from("word"), String::from("hands")] }

/// (src/game/pairup.ts:73 KEPT: a table nothing writes)
pub fn kept() -> [String; 7] { [String::from("on"), String::from("oneEye"), String::from("hunted"), String::from("rejoin"), String::from("stay"), String::from("word"), String::from("hands")] }
