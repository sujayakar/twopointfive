//! The types of `scene_level_data`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PropKindName {
    #[default] Cardboard,
    Plant,
    Chair,
    Bin,
}
impl PropKindName {
    pub const fn as_str(self) -> &'static str {
        match self {
            PropKindName::Cardboard => "cardboard",
            PropKindName::Plant => "plant",
            PropKindName::Chair => "chair",
            PropKindName::Bin => "bin",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SurfacesData {
    pub building: crate::scene_level_data::Rect,
    pub grass_edge: f64,
    pub inside: crate::floor::FloorMat,
    pub outside: crate::floor::FloorMat,
    pub verge: crate::floor::FloorMat,
    pub zones: Vec<crate::scene_level_data::SurfacesDataZonesItem>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Rect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SurfacesDataZonesItem {
    pub mat: crate::floor::FloorMat,
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DEVICEDEFAULTS {
    pub pitch: f64,
    pub fov_deg: f64,
    pub range: f64,
    pub sweep_period: f64,
    pub group: String,
    pub alarm: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct VENTDEFAULTS {
    pub size: f64,
    pub duty: f64,
    pub mains: bool,
}

/// (tools/levels/lib/levelData.ts:186 DEVICE_DEFAULTS: a table nothing writes)
pub fn device_defaults() -> crate::scene_level_data::DEVICEDEFAULTS { crate::scene_level_data::DEVICEDEFAULTS { pitch: -0.35, fov_deg: 70.0, range: 14.0, sweep_period: 9.0, group: String::from("security"), alarm: String::from("suspicion") } }

/// (tools/levels/lib/levelData.ts:194 VENT_DEFAULTS: a table nothing writes)
pub fn vent_defaults() -> crate::scene_level_data::VENTDEFAULTS { crate::scene_level_data::VENTDEFAULTS { size: 0.6, duty: 1.0, mains: true } }
