//! The types of `air_dust`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: AirDustSnap, src/game/airDust.ts:54)
#[derive(Clone, Debug)]
pub struct AirDustSnap {
    pub acc: f64,
    pub cur: f64,
    pub s: Vec<f64>,
    pub stat: String,
}
impl AirDustSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AirDustSnap {
            acc: 0.0,
            cur: 0.0,
            s: Vec::new(),
            stat: String::new(),
        }
    }
}
impl Default for AirDustSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AirDust, src/game/airDust.ts:61)
#[derive(Clone, Debug)]
pub struct AirDust {
    pub dirty: bool,
    pub drift: f64,
    pub gpu: Vec<f32>,
    pub seeded: bool,
    pub stats: crate::air_dust::AirDustStats,
}
impl AirDust {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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

/// (TypeScript: inline at src/game/airDust.ts:80 AirDust.stats, src/game/airDust.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct AirDustStats {
    pub steps: f64,
    pub probes: f64,
    pub sends: f64,
    pub roofed: f64,
    pub edges: f64,
    pub open: f64,
}
impl AirDustStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AirDustStats {
            steps: 0.0,
            probes: 0.0,
            sends: 0.0,
            roofed: 0.0,
            edges: 0.0,
            open: 0.0,
        }
    }
}
impl Default for AirDustStats { fn default() -> Self { Self::zeroed() } }

/// (src/game/airDust.ts:50 N: a table nothing writes)
pub fn n() -> f64 { 4480.0 }

/// (src/game/airDust.ts:52 SEND)
pub const SEND: f64 = 0.0002;
