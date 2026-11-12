//! The types of `air_dust`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct AirDust {
    #[is(true)] pub dirty: bool,
    #[bytes(skip)]
    pub drift: f64,
    #[bytes(skip)]
    #[is(alloc::vec![0.0; 4480])] pub gpu: Vec<f32>,
    #[bytes(skip)]
    pub seeded: bool,
    #[bytes(skip)]
    #[is(crate::air_dust::AirDustStats { roofed: 0.0, edges: 0.0 })] pub stats: crate::air_dust::AirDustStats,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AirDustStats {
    pub roofed: f64,
    pub edges: f64,
}

pub fn n() -> f64 { 4480.0 }

pub const SEND: f64 = 0.0002;
