//! The types of `air_dust`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct AirDust {
    pub dirty: bool,
    pub drift: f64,
    pub gpu: Vec<f32>,
    pub seeded: bool,
    pub stats: crate::air_dust::AirDustStats,
}
impl AirDust {
    /// what a new one holds
    pub fn blank() -> Self {
        AirDust {
            dirty: true,
            drift: 0.0,
            gpu: alloc::vec![0.0; 4480],
            seeded: false,
            stats: crate::air_dust::AirDustStats { steps: 0.0, probes: 0.0, sends: 0.0, roofed: 0.0, edges: 0.0, open: 0.0 },
        }
    }
}
impl Default for AirDust { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AirDustStats {
    pub steps: f64,
    pub probes: f64,
    pub sends: f64,
    pub roofed: f64,
    pub edges: f64,
    pub open: f64,
}

pub fn n() -> f64 { 4480.0 }

pub const SEND: f64 = 0.0002;
