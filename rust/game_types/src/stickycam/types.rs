//! The types of `stickycam`.
use crate::{Blank, bytes::ToBytes, ids::*, V3};
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StickyCam {
    pub pos: V3,
    pub n: V3,
    pub vel: Option<V3>,
    pub age: f64,
    pub si: f64,
    pub id: f64,
}

#[derive(Clone, Debug, Default)]
pub struct StickyLivePl {
    pub cams: Vec<StickyCamId>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct StickycamMod {
    pub visit: Option<f64>,
}

pub const EYE_ID: f64 = 200.0;

pub fn none() -> Vec<crate::fogofwar::FogEye> { Vec::new() }

pub fn none_cams() -> Vec<StickyCamId> { Vec::new() }

pub const CAM_KEY: f64 = 8388608.0;

