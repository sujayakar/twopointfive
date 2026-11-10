//! The types of `lighting`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum SetBlackoutCause {
    Shot,
    Ocp,
    Debug,
    Hand,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LightEventCause {
    Shot,
    Ocp,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct LightingMod {
    pub lens_from: JsMap<LightId, V3>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct LightingStats {
    pub asked: i32,
    pub pulled: i32,
    pub max_back: f64,
}

pub const LENS_SLANT: f64 = 0.3;
