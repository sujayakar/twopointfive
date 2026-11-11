//! The types of `holdfire`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub lift_t: f64,
    pub cast: Vec<GuardId>,
    pub loud_t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HoldFireOrderO {
    pub blown: Option<bool>,
    pub silent: Option<bool>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct HoldfireMod {
    pub states: JsMap<GroupId, crate::holdfire::State>,
}

