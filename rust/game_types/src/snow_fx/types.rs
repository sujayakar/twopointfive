//! The types of `snow_fx`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct SnowState {
    pub seed: f64,
    pub serial: f64,
    pub birth_acc: f64,
    pub pop: f64,
    pub last_emitted: f64,
    pub fill_at: f64,
    pub was_active: bool,
    pub band_off: [f64; 2],
    pub stats: crate::snow_fx::SnowStats,
    pub w_serial: f64,
    pub w_acc: f64,
    pub flake_filed: f64,
    pub wisps: crate::snow_fx::WispStats,
    pub g_serial: f64,
    pub g_acc: f64,
    pub g_fill: f64,
    pub g_hold: f64,
    pub g_seen: bool,
    pub grains: crate::snow_fx::GrainStats,
    pub one_air: bool,
    pub kick_seen: f64,
    pub kick_q: Vec<crate::snow_tracks::SnowKick>,
}

#[derive(Clone, Debug, Default)]
pub struct SnowStats {
    pub active: bool,
    pub why: String,
    pub radius: f64,
    pub centre: [f64; 2],
    pub area_out: f64,
    pub target: f64,
    pub capped: bool,
    pub pop: f64,
    pub births: f64,
    pub total_births: f64,
    pub frames: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WispStats {
    pub active: bool,
    pub why: String,
    pub target: f64,
    pub blow: f64,
    pub births: f64,
    pub total_births: f64,
    pub records: f64,
}

#[derive(Clone, Debug, Default)]
pub struct GrainStats {
    pub active: bool,
    pub why: String,
    pub target: f64,
    pub pop: f64,
    pub births: f64,
    pub total_births: f64,
    pub records: f64,
    pub radius: f64,
    pub air: bool,
    pub u0: f64,
    pub wind: [f64; 2],
}

