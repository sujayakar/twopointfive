//! The types of `scene_ways`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WayHint {
    pub from: String,
    pub to: String,
    pub weight: Option<f64>,
}

