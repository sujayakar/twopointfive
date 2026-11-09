//! The types of `scene_cut`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CutPlane {
    pub n: V3,
    pub d: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CutResult {
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub empty: bool,
    pub dropped: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CutGlass {
    pub see: f64,
    pub rim: f64,
    pub anim: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CutBounds {
    pub min: V3,
    pub max: V3,
}

#[derive(Clone, Debug, Default)]
pub struct CutFit {
    pub off: V3,
    pub h: V3,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
}

#[derive(Clone, Debug, Default)]
pub struct CutPrism {
    pub off: V3,
    pub h: V3,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub dropped: f64,
    pub empty: bool,
}

pub const MAX_CUT_PLANES: f64 = 8.0;

pub const CUT_STRIDE: f64 = 9.0;
