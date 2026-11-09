//! The types of `math_dice`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Die {
    pub dice: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DiceHome {
    pub s: f64,
    pub n: f64,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct SimDice {
    pub home: Option<crate::math_dice::DiceHome>,
    pub s0: f64,
    pub n0: f64,
    pub seed: f64,
}
impl SimDice {
    /// what a new one holds
    pub fn blank() -> Self {
        SimDice {
            home: None,
            s0: 0.0,
            n0: 0.0,
            seed: 0.0,
        }
    }
}
impl Default for SimDice { fn default() -> Self { Self::blank() } }
