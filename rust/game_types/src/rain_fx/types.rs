//! The types of `rain_fx`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct RainState {
    pub seed: f64,
    pub birth_acc: f64,
    pub drip_acc: f64,
    pub was_active: bool,
    pub edges: Option<Vec<crate::rain_fx::DripPoint>>,
    pub edge_boxes: f64,
    pub job: Option<crate::rain_fx::DripJob>,
    pub kind: Option<String>,
    pub stats: crate::rain_fx::RainStatsCpu,
    pub serial: f64,
    pub pop: f64,
    pub pop_log: Vec<crate::rain_fx::RainStatePopLogItem>,
    pub clock: f64,
    pub last_c: Option<[f64; 2]>,
    pub drip_life_max: f64,
    pub shear: [f64; 2],
    pub band_off: [f64; 2],
    pub air: [f64; 2],
    pub shear_init: bool,
    pub aside: crate::scene_collision::AsideOwn,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DripPoint {
    pub pos: V3,
    pub fall: f64,
    pub box_: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DripJob {
    pub boxes: f64,
    pub serial: f64,
    pub left: f64,
    pub hold: f64,
    pub bi: f64,
    pub ei: f64,
    pub k: f64,
    pub paid: bool,
    pub out: Vec<crate::rain_fx::DripPoint>,
}

#[derive(Clone, Debug, Default)]
pub struct RainStatsCpu {
    pub active: bool,
    pub why: String,
    pub radius: f64,
    pub centre: [f64; 2],
    pub area_out: f64,
    pub lattice_per_m2: f64,
    pub target: f64,
    pub capped: bool,
    pub keep: f64,
    pub fill: f64,
    pub births_per_sec: f64,
    pub births: f64,
    pub drips: f64,
    pub edges_active: f64,
    pub edges_total: f64,
    pub frames: f64,
    pub total_births: f64,
    pub total_drips: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RainStatePopLogItem {
    pub t: f64,
    pub dt: f64,
    pub n: f64,
}

