//! The types of `aimweight`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct AimWeight {
    pub vel: f64,
    pub r: f64,
    pub y: f64,
    pub live: bool,
    pub was: bool,
    pub landed: bool,
    pub on: bool,
    pub err: f64,
    pub launch_t: f64,
}
impl AimWeight {
    /// what a new one holds
    pub fn blank() -> Self {
        AimWeight {
            vel: 0.0,
            r: 0.0,
            y: 0.0,
            live: false,
            was: false,
            landed: true,
            on: true,
            err: 0.0,
            launch_t: 0.0,
        }
    }
}
impl Default for AimWeight { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct AimSwingW {
    pub vel: f64,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct AimweightStats {
    pub steps: i32,
    pub off_shots: i32,
}
