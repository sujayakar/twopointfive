//! The types of `wind`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WindWhy {
    #[default] Door,
    Glass,
    Storm,
    Vent,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WindSource {
    pub why: crate::wind::WindWhy,
    pub base: f64,
    pub on: f64,
    pub gust: f64,
    pub seed: f64,
    pub vent: Option<crate::scene_level::VentDef>,
    pub door: Option<DoorId>,
    pub kind: crate::wind::WindSourceKind,
    pub name: String,
    pub pos: V3,
    pub dir: V3,
    pub speed: f64,
    pub reach: f64,
    pub tan_spread: f64,
    pub r0: f64,
    pub d_max: f64,
    pub floor_y: Option<f64>,
    pub interior: Option<[f64; 4]>,
    pub annex: Option<[f64; 4]>,
    pub roofs: Option<Vec<Vec<f64>>>,
    pub field: Option<bool>,
    pub bound: crate::smoke_types::WindGpuSourceBound,
    pub kinds: Option<f64>,
    pub heat: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WindSourceKind {
    #[default] Jet,
    Storm,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Wind {
    pub all: Vec<WindWindSourceId>,
    pub active: Vec<WindWindSourceId>,
    #[is(WindWindSourceId(0))] pub storm: WindWindSourceId,
    pub lv_roofs: Option<Vec<Vec<f64>>>,
    pub annexed: Vec<DoorId>,
    pub extra: Vec<WindWindSourceId>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WindMod {
    pub shape: Option<Vec<f64>>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WindStats {
    pub frames: i32,
    pub active: i32,
    pub to_smoke: i32,
    pub door_gusts: i32,
    pub storm_frames: i32,
}

pub const DEG: f64 = 0.017453292519943295;

pub fn unit_x() -> V3 { [1.0, 0.0, 0.0] }
