//! The types of `air_sim`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct AirHandle {
    pub w: f64,
    pub h: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AirStatePuffsItem {
    pub x: f64,
    pub z: f64,
    pub out: f64,
    pub sigma: f64,
    pub rate: f64,
    pub t: f64,
    pub secs: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
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
