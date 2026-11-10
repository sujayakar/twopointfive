//! The types of `doorblast`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SlabMet {
    pub landed: bool,
    pub lx: f64,
    pub lz: f64,
    pub rested: bool,
    pub nudged: f64,
    pub nudge_max: f64,
    pub laid: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorBlastReachItem {
    pub d: DoorId,
    pub k: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DoorblastMod {
    pub doorblast_log: Vec<String>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DoorblastStats {
    pub doorblast_blasts: i32,
    pub doorblast_torn: i32,
    pub doorblast_landed: i32,
    pub doorblast_back: i32,
    pub doorblast_nudged: i32,
    pub doorblast_nudge_max: f64,
    pub doorblast_laid: i32,
    pub doorshot_rounds: i32,
    pub doorshot_splintered: i32,
    pub doorshot_chunks: i32,
    pub doorshot_demoted: i32,
}

pub const OFF_ANGLE: f64 = 1.5707963267948966;

pub const HALF_PI: f64 = 1.5707963267948966;

pub const LIFT: f64 = 0.005;

pub fn ov() -> Vec<i32> { alloc::vec![0; 512] }
