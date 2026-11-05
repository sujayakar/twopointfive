//! The types of `air_sim`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: AirSimStats, src/game/airSim.ts:48)
#[derive(Clone, Debug)]
pub struct AirSimStats {
    pub active: bool,
    pub why: String,
    pub w: f64,
    pub h: f64,
    pub solid: f64,
    pub roofed: f64,
    pub open: f64,
    pub leaves: f64,
    pub rebuilt: f64,
    pub restamped: f64,
    pub sources: f64,
    pub wind: [f64; 2],
    pub speed: f64,
    pub calm: f64,
}
impl AirSimStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AirSimStats {
            active: false,
            why: String::new(),
            w: 0.0,
            h: 0.0,
            solid: 0.0,
            roofed: 0.0,
            open: 0.0,
            leaves: 0.0,
            rebuilt: 0.0,
            restamped: 0.0,
            sources: 0.0,
            wind: [0.0; 2],
            speed: 0.0,
            calm: 0.0,
        }
    }
}
impl Default for AirSimStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AirHandle, src/game/airSim.ts:50)
#[derive(Clone, Copy, Debug)]
pub struct AirHandle {
    pub w: f64,
    pub h: f64,
}
impl AirHandle {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AirHandle {
            w: 0.0,
            h: 0.0,
        }
    }
}
impl Default for AirHandle { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AirState, src/game/airSim.ts:52)
#[derive(Clone, Debug)]
pub struct AirState {
    pub w: f64,
    pub h: f64,
    pub base: Vec<u8>,
    pub mask: Vec<u8>,
    pub roof: Vec<u8>,
    pub key: f64,
    pub leaf_key: f64,
    pub sent: bool,
    pub last_t: f64,
    pub quiet: f64,
    pub fresh: bool,
    pub want: bool,
    pub src: Vec<f32>,
    pub cand: Vec<f32>,
    pub stats: crate::air_sim::AirSimStats,
    pub puffs: Vec<crate::air_sim::AirStatePuffsItem>,
}
impl AirState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AirState {
            w: 0.0,
            h: 0.0,
            base: Vec::new(),
            mask: Vec::new(),
            roof: Vec::new(),
            key: 0.0,
            leaf_key: 0.0,
            sent: false,
            last_t: 0.0,
            quiet: 0.0,
            fresh: false,
            want: false,
            src: Vec::new(),
            cand: Vec::new(),
            stats: crate::air_sim::AirSimStats::zeroed(),
            puffs: Vec::new(),
        }
    }
}
impl Default for AirState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/airSim.ts:54 AirState.puffs, src/game/airSim.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct AirStatePuffsItem {
    pub x: f64,
    pub z: f64,
    pub out: f64,
    pub sigma: f64,
    pub rate: f64,
    pub t: f64,
    pub secs: f64,
}
impl AirStatePuffsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AirStatePuffsItem {
            x: 0.0,
            z: 0.0,
            out: 0.0,
            sigma: 0.0,
            rate: 0.0,
            t: 0.0,
            secs: 0.0,
        }
    }
}
impl Default for AirStatePuffsItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct AirSimMod {
    /// (src/game/airSim.ts:55 states)
    pub states: Option<crate::air_sim::AirState>,
    /// (src/game/airSim.ts:67 offStats)
    pub off_stats: Option<crate::air_sim::AirSimStats>,
}
impl AirSimMod {
    pub fn blank() -> Self {
        AirSimMod {
            states: None,
            off_stats: None,
        }
    }
}

/// (src/game/airSim.ts:30 AIR_CELLS)
pub const AIR_CELLS: f64 = 5.0;

/// (src/game/airSim.ts:32 AIR_SOURCES)
pub const AIR_SOURCES: f64 = 16.0;

/// (src/game/airSim.ts:51 CAND)
pub const CAND: f64 = 8.0;

/// (src/game/airSim.ts:90 HIDDEN)
pub const HIDDEN: f64 = 3.0;
