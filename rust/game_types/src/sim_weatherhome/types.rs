//! The types of `sim_weatherhome`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct WX {
    pub storm: f64,
    pub roofs: f64,
    pub air_acc: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RoofsCountedBoxesItem {
    pub c: Vec<f64>,
    pub h: Vec<f64>,
}

pub fn wx() -> crate::sim_weatherhome::WX { crate::sim_weatherhome::WX { storm: 1.0, roofs: 13.0, air_acc: 22.0 } }

pub const WX_BIT: f64 = 1024.0;

