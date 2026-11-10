//! The types of `lighting`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct LightingMod {
    pub lens_from: JsMap<LightId, V3>,
}

pub const LENS_SLANT: f64 = 0.3;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum LightEventCause {
    Shot,
    Ocp,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum SetBlackoutCause {
    Shot,
    Ocp,
    Debug,
    Hand,
}
