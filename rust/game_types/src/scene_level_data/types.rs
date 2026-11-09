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
pub struct BoxData {
    pub id: String,
    pub floor: Option<String>,
    pub group: Option<String>,
    pub name: Option<String>,
    pub c: V3,
    pub h: V3,
    pub yaw: Option<f64>,
    pub rot: Option<Quat>,
    pub round: Option<bool>,
    pub material: Option<String>,
    pub albedo: Option<V3>,
    pub emissive: Option<V3>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<String>,
    pub does: Option<String>,
    pub flags: Option<Vec<crate::scene_level_data::FlagName>>,
    pub cut_height: Option<f64>,
    pub markings: Option<Vec<crate::scene_markings::MarkingDef>>,
    pub shape: Option<crate::scene_level_data::ShapeData>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum FlagName {
    NoPrimary,
    NoShadow,
    Cutaway,
    Gloss,
    Glass,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShapeData {
    pub with: Option<Vec<crate::scene_level_data::ShapePieceData>>,
    pub round: Option<f64>,
    pub arc: Option<crate::scene_shapes::ShapeFormArc>,
    pub cut: Option<Vec<[f64; 4]>>,
    pub old: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShapePieceData {
    pub off: V3,
    pub h: V3,
    pub yaw: Option<f64>,
    pub material: Option<String>,
    pub albedo: Option<V3>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub does: Option<String>,
    pub stuff: Option<String>,
    pub round: Option<f64>,
    pub arc: Option<crate::scene_shapes::ShapeFormArc>,
    pub cut: Option<Vec<[f64; 4]>>,
    pub old: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LightData {
    pub id: String,
    pub kind: crate::scene_level_data::LightDataKind,
    pub pos: V3,
    pub dir: V3,
    pub color: V3,
    pub intensity: f64,
    pub range: f64,
    pub inner_deg: f64,
    pub outer_deg: f64,
    pub group: String,
    pub enabled: bool,
    pub volumetric: f64,
    pub radius: Option<f64>,
    pub mains: Option<bool>,
    pub broad: Option<bool>,
    pub fixture: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum LightDataKind {
    #[default] Spot,
    Dir,
    Point,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorData {
    pub id: String,
    pub floor: Option<String>,
    pub hinge: [f64; 2],
    pub closed_dir: f64,
    pub width: f64,
    pub angle: Option<f64>,
    pub exterior: Option<bool>,
    pub closer: Option<bool>,
    pub min_angle: Option<f64>,
    pub max_angle: Option<f64>,
    pub locked: Option<bool>,
    pub lock: Option<String>,
    pub key_side: Option<f64>,
    pub swing: Option<f64>,
    pub steel: Option<bool>,
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
    pub id: String,
    pub mat: crate::floor::FloorMat,
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
    pub notes: Option<String>,
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
    pub phase: f64,
    pub mains: bool,
}

/// (tools/levels/lib/levelData.ts:186 DEVICE_DEFAULTS: a table nothing writes)
pub fn device_defaults() -> crate::scene_level_data::DEVICEDEFAULTS { crate::scene_level_data::DEVICEDEFAULTS { pitch: -0.35, fov_deg: 70.0, range: 14.0, sweep_period: 9.0, group: String::from("security"), alarm: String::from("suspicion") } }

/// (tools/levels/lib/levelData.ts:194 VENT_DEFAULTS: a table nothing writes)
pub fn vent_defaults() -> crate::scene_level_data::VENTDEFAULTS { crate::scene_level_data::VENTDEFAULTS { size: 0.6, duty: 1.0, phase: 0.0, mains: true } }
