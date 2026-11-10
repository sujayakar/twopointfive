//! The types of `darkfoe`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FoeState {
    pub knows: bool,
    pub blind: bool,
    pub ask: f64,
    pub lit: bool,
    pub against: f64,
    pub held: f64,
    pub blind_for: f64,
    pub moves: f64,
    pub since: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DarkFoeSpot {
    pub spot: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DarkfoeMod {
    pub state_of: JsMap<GuardId, crate::darkfoe::FoeState>,
}

pub const WAY_TRIES: f64 = 6.0;
