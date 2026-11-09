//! The types of `math_dice`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
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

#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct SimDice {
    pub home: Option<crate::math_dice::DiceHome>,
    pub s0: f64,
    pub n0: f64,
    pub seed: f64,
}
