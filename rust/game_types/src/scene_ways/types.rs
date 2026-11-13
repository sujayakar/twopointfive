//! The types of `scene_ways`.
use crate::bytes::ToBytes;
use alloc::string::String;

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WayHint {
    pub from: String,
    pub to: String,
    pub weight: Option<f64>,
}

