//! The types of `wind`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct WindTune {
    pub wind: Option<f64>,
    pub moths: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WindWhy {
    #[default] Door,
    Glass,
    Storm,
    Vent,
}
impl WindWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            WindWhy::Door => "door",
            WindWhy::Glass => "glass",
            WindWhy::Storm => "storm",
            WindWhy::Vent => "vent",
        }
    }
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
impl WindSourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WindSourceKind::Jet => "jet",
            WindSourceKind::Storm => "storm",
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub struct Wind {
    pub all: Vec<WindWindSourceId>,
    pub active: Vec<WindWindSourceId>,
    pub storm: WindWindSourceId,
    pub lv_roofs: Option<Vec<Vec<f64>>>,
    pub annexed: Vec<DoorId>,
    pub extra: Vec<WindWindSourceId>,
}
impl Wind {
    /// what a new one holds
    pub fn blank() -> Self {
        Wind {
            all: Vec::new(),
            active: Vec::new(),
            storm: WindWindSourceId(0),
            lv_roofs: None,
            annexed: Vec::new(),
            extra: Vec::new(),
        }
    }
}
impl Default for Wind { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct WINDFIELD {
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct WindMod {
    /// (src/game/wind.ts:38 SHAPE)
    pub shape: Option<Vec<f64>>,
}
impl WindMod {
    pub fn blank() -> Self {
        WindMod {
            shape: None,
        }
    }
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

/// (src/game/wind.ts:37 WIND_FIELD: a table nothing writes)
pub fn wind_field() -> crate::wind::WINDFIELD { crate::wind::WINDFIELD {  } }

/// (src/game/wind.ts:53 WIND_DEFAULTS: a table nothing writes)
pub fn wind_defaults() -> crate::knobs::WindK { crate::knobs::page_wind() }

/// (src/game/wind.ts:54 LS_KEY)
pub const LS_KEY: &str = "twopointsix.wind";

/// (src/game/wind.ts:56 PERSISTED: a table nothing writes)
pub fn persisted() -> [String; 12] { [String::from("vents"), String::from("ventK"), String::from("storm"), String::from("stormK"), String::from("doors"), String::from("doorK"), String::from("gusts"), String::from("gustAmp"), String::from("smokeRelax"), String::from("sheetWake"), String::from("sheetKick"), String::from("debrisK")] }

/// (src/game/wind.ts:111 DEG)
pub const DEG: f64 = 0.017453292519943295;

/// (src/game/wind.ts:116 SCRATCH_P: a table nothing writes)
pub fn scratch_p() -> V3 { [0.0, 0.0, 0.0] }

/// (src/game/wind.ts:116 SCRATCH_Q: a table nothing writes)
pub fn scratch_q() -> V3 { [0.0, 0.0, 0.0] }

/// (src/game/wind.ts:116 SCRATCH_W: a table nothing writes)
pub fn scratch_w() -> V3 { [0.0, 0.0, 0.0] }

/// (src/game/wind.ts:309 UNIT_X: a table nothing writes)
pub fn unit_x() -> V3 { [1.0, 0.0, 0.0] }
