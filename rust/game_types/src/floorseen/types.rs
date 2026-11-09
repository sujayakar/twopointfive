//! The types of `floorseen`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct FloorSeen {
    pub at: JsMap<GuardId, Vec<f32>>,
    pub asked: JsMap<GuardId, Vec<f32>>,
    pub frame: f64,
}

pub const EVERY: f64 = 12.0;

pub const REFRESH: f64 = 1.0;

pub const NEVER: f64 = -1000000000.0;
