//! The types of `render_passes`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct FireRecord {
    pub pos: Vec<f64>,
    pub r: f64,
    pub h: f64,
    pub heat: f64,
    pub seed: f64,
    pub time: f64,
    pub wind: Vec<f64>,
    pub flick: f64,
    pub bed: Option<f64>,
    pub fog: Option<f64>,
    pub soot: Option<f64>,
}
