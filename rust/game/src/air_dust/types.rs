//! The types of `air_dust`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: AirDustSnap, src/game/airDust.ts:55)
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

/// (TypeScript: AirDust, src/game/airDust.ts:57)
#[derive(Clone, Debug)]
pub struct AirDust {
    pub s: Vec<f64>,
    pub on: bool,
    pub dirty: bool,
    pub drift: f64,
    pub indoor: Vec<u8>,
    pub cells: Vec<i32>,
    pub edges: Vec<i32>,
    pub stat: Vec<u8>,
    pub leaf: Vec<u8>,
    pub next: Vec<f64>,
    pub gpu: Vec<f32>,
    pub leak: f64,
    pub acc: f64,
    pub cur: f64,
    pub leaf_sig: f64,
    pub seeded: bool,
    pub pa: V3,
    pub pb: V3,
    pub stats: crate::air_dust::AirDustStats,
}
impl AirDust {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        AirDust {
            s: alloc::vec![1.0; 4480],
            on: false,
            dirty: true,
            drift: 0.0,
            indoor: alloc::vec![0; 4480],
            cells: alloc::vec![0; 0],
            edges: alloc::vec![0; 0],
            stat: alloc::vec![0; 8960],
            leaf: alloc::vec![0; 8960],
            next: alloc::vec![0.0; 4480],
            gpu: alloc::vec![0.0; 4480],
            leak: 1.0,
            acc: 0.0,
            cur: 0.0,
            leaf_sig: f64::NAN,
            seeded: false,
            pa: [0.0, 0.0, 0.0],
            pb: [0.0, 0.0, 0.0],
            stats: crate::air_dust::AirDustStats { steps: 0.0, probes: 0.0, sends: 0.0, roofed: 0.0, edges: 0.0, open: 0.0 },
        }
    }
}
impl Default for AirDust { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/airDust.ts:72 AirDust.stats, src/game/airDust.ts:72)
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

/// (src/game/airDust.ts:48 PROBE_Y: a table nothing writes)
pub fn probe_y() -> Vec<f64> { alloc::vec![1.6, 0.4, 2.05] }

/// (src/game/airDust.ts:49 N: a table nothing writes)
pub fn n() -> f64 { 4480.0 }

/// (src/game/airDust.ts:51 SNAP)
pub const SNAP: f64 = 0.000001;

/// (src/game/airDust.ts:53 SEND)
pub const SEND: f64 = 0.0002;
