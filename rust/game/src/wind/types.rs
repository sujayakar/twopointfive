//! The types of `wind`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WindTune, src/game/wind.ts:50)
#[derive(Clone, Copy, Debug)]
pub struct WindTune {
    pub wind: Option<f64>,
    pub moths: Option<bool>,
}
impl WindTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WindTune {
            wind: None,
            moths: None,
        }
    }
}
impl Default for WindTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WindWhy {
    Door,
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

/// (TypeScript: WindSource, src/game/wind.ts:132)
#[derive(Clone, Debug)]
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
}
impl WindSource {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WindSource {
            why: crate::wind::WindWhy::Door,
            base: 0.0,
            on: 0.0,
            gust: 0.0,
            seed: 0.0,
            vent: None,
            door: None,
            kind: crate::wind::WindSourceKind::Jet,
            name: String::new(),
            pos: [0.0; 3],
            dir: [0.0; 3],
            speed: 0.0,
            reach: 0.0,
            tan_spread: 0.0,
            r0: 0.0,
            d_max: 0.0,
            floor_y: None,
            interior: None,
            annex: None,
            roofs: None,
            field: None,
            bound: crate::smoke_types::WindGpuSourceBound::zeroed(),
        }
    }
}
impl Default for WindSource { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WindSourceKind {
    Jet,
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

/// (TypeScript: PoolList, src/game/wind.ts:137)
#[derive(Clone, Debug)]
pub struct PoolList {
    pub list: Vec<crate::wind::WindSource>,
}
impl PoolList {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        PoolList {
            list: Vec::new(),
        }
    }
}
impl Default for PoolList { fn default() -> Self { Self::blank() } }

/// (TypeScript: Wind, src/game/wind.ts:142)
#[derive(Clone, Debug)]
pub struct Wind {
    pub all: Vec<crate::wind::WindSource>,
    pub active: Vec<crate::wind::WindSource>,
    pub to_pool: crate::wind::PoolList,
    pub storm: crate::wind::WindSource,
    pub lv_roofs: Option<Vec<Vec<f64>>>,
    pub annexed: Vec<DoorId>,
    pub extra: Vec<crate::wind::WindSource>,
}
impl Wind {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Wind {
            all: Vec::new(),
            active: Vec::new(),
            to_pool: crate::wind::PoolList::blank(),
            storm: crate::wind::WindSource::zeroed(),
            lv_roofs: None,
            annexed: Vec::new(),
            extra: Vec::new(),
        }
    }
}
impl Default for Wind { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/wind.ts:35 WIND_FIELD, src/game/wind.ts:35)
#[derive(Clone, Copy, Debug)]
pub struct WINDFIELD {
    pub deflect_h: f64,
    pub deflect_loss: f64,
    pub support_sigma: f64,
    pub storm_top: f64,
    pub storm_lee: f64,
    pub storm_low: f64,
    pub storm_gain: f64,
    pub storm_full_at: f64,
    pub cut_k: f64,
    pub core_k: f64,
}
impl WINDFIELD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WINDFIELD {
            deflect_h: 0.0,
            deflect_loss: 0.0,
            support_sigma: 0.0,
            storm_top: 0.0,
            storm_lee: 0.0,
            storm_low: 0.0,
            storm_gain: 0.0,
            storm_full_at: 0.0,
            cut_k: 0.0,
            core_k: 0.0,
        }
    }
}
impl Default for WINDFIELD { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WindStats {
    pub frames: i32,
    pub active: i32,
    pub to_particles: i32,
    pub to_smoke: i32,
    pub door_gusts: i32,
    pub storm_frames: i32,
    pub at_calls: i32,
}

/// (src/game/wind.ts:35 WIND_FIELD: a table nothing writes)
pub fn wind_field() -> crate::wind::WINDFIELD { crate::wind::WINDFIELD { deflect_h: 0.55, deflect_loss: 0.35, support_sigma: 2.6, storm_top: 8.0, storm_lee: 1.5, storm_low: 0.55, storm_gain: 0.45, storm_full_at: 2.5, cut_k: 2.5, core_k: 0.4 } }

/// (src/game/wind.ts:48 WIND_DEFAULTS: a table nothing writes)
pub fn wind_defaults() -> crate::knobs::WindK { crate::knobs::page_wind() }

/// (src/game/wind.ts:49 LS_KEY)
pub const LS_KEY: &str = "twopointsix.wind";

/// (src/game/wind.ts:51 PERSISTED: a table nothing writes)
pub fn persisted() -> [String; 12] { [String::from("vents"), String::from("ventK"), String::from("storm"), String::from("stormK"), String::from("doors"), String::from("doorK"), String::from("gusts"), String::from("gustAmp"), String::from("smokeRelax"), String::from("sheetWake"), String::from("sheetKick"), String::from("debrisK")] }

/// (src/game/wind.ts:135 DEG)
pub const DEG: f64 = 0.017453292519943295;

/// (src/game/wind.ts:140 SCRATCH_P: a table nothing writes)
pub fn scratch_p() -> V3 { [0.0, 0.0, 0.0] }

/// (src/game/wind.ts:140 SCRATCH_Q: a table nothing writes)
pub fn scratch_q() -> V3 { [0.0, 0.0, 0.0] }

/// (src/game/wind.ts:140 SCRATCH_W: a table nothing writes)
pub fn scratch_w() -> V3 { [0.0, 0.0, 0.0] }

/// (src/game/wind.ts:332 KNOBS0: a table nothing writes)
pub fn knobs0() -> [f64; 3] { [0.35, 2.0, 0.15] }

/// (src/game/wind.ts:333 UNIT_X: a table nothing writes)
pub fn unit_x() -> V3 { [1.0, 0.0, 0.0] }
