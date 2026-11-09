//! The types of `floorseen`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct FloorSeen {
    pub at: JsMap<GuardId, Vec<f32>>,
    pub asked: JsMap<GuardId, Vec<f32>>,
    pub frame: f64,
}
impl FloorSeen {
    /// what a new one holds
    pub fn blank() -> Self {
        FloorSeen {
            at: JsMap::new(),
            asked: JsMap::new(),
            frame: 0.0,
        }
    }
}
impl Default for FloorSeen { fn default() -> Self { Self::blank() } }

pub const EVERY: f64 = 12.0;

pub const REFRESH: f64 = 1.0;

pub const NEVER: f64 = -1000000000.0;
