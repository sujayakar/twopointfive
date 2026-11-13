//! The types of `fire`.
use crate::{Blank, bytes::ToBytes, V3};
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FireStats {
    pub lo: f64,
    pub hi: f64,
}

#[derive(Clone, Debug, Default)]
pub struct FireShown {
    pub n: f64,
    pub time: f64,
    pub stir: Option<Vec<f64>>,
    pub stir_at: Option<Vec<f64>>,
    pub smoke: Vec<f64>,
    pub ember: Vec<f64>,
    pub embers: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FirePopsItem {
    pub at: f64,
    pub hard: f64,
    pub seed: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MouthOf {
    pub pos: V3,
    pub r: f64,
    pub lid: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WindAt {
    pub wv: V3,
    pub wl: f64,
    pub wind: [f64; 2],
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct FireMod {
    pub states: Option<crate::fire::FireStats>,
}

pub const TAU: f64 = 6.283185307179586;

