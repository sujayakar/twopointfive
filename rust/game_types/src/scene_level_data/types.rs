//! The types of `scene_level_data`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PropKindName {
    Cardboard,
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

/// (TypeScript: BoxData, src/scene/levelData.ts:79)
#[derive(Clone, Debug)]
pub struct BoxData {
    pub id: String,
    pub floor: Option<String>,
    pub group: Option<String>,
    pub name: Option<String>,
    pub c: V3,
    pub h: V3,
    pub yaw: Option<f64>,
    pub rot: Option<Quat>,
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
impl BoxData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxData {
            id: String::new(),
            floor: None,
            group: None,
            name: None,
            c: [0.0; 3],
            h: [0.0; 3],
            yaw: None,
            rot: None,
            material: None,
            albedo: None,
            emissive: None,
            rough: None,
            metal: None,
            stuff: None,
            does: None,
            flags: None,
            cut_height: None,
            markings: None,
            shape: None,
        }
    }
}
impl Default for BoxData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FlagName {
    NoPrimary,
    NoShadow,
    Cutaway,
    Gloss,
    Glass,
}
impl FlagName {
    pub const fn as_str(self) -> &'static str {
        match self {
            FlagName::NoPrimary => "NoPrimary",
            FlagName::NoShadow => "NoShadow",
            FlagName::Cutaway => "Cutaway",
            FlagName::Gloss => "Gloss",
            FlagName::Glass => "Glass",
        }
    }
}

/// (TypeScript: ShapeData, src/scene/levelData.ts:124)
#[derive(Clone, Debug)]
pub struct ShapeData {
    pub with: Option<Vec<crate::scene_level_data::ShapePieceData>>,
    pub round: Option<f64>,
    pub arc: Option<crate::scene_shapes::ShapeFormArc>,
    pub cut: Option<Vec<[f64; 4]>>,
    pub old: Option<bool>,
}
impl ShapeData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShapeData {
            with: None,
            round: None,
            arc: None,
            cut: None,
            old: None,
        }
    }
}
impl Default for ShapeData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ShapePieceData, src/scene/levelData.ts:123)
#[derive(Clone, Debug)]
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
impl ShapePieceData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShapePieceData {
            off: [0.0; 3],
            h: [0.0; 3],
            yaw: None,
            material: None,
            albedo: None,
            rough: None,
            metal: None,
            does: None,
            stuff: None,
            round: None,
            arc: None,
            cut: None,
            old: None,
        }
    }
}
impl Default for ShapePieceData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LightData, src/scene/levelData.ts:131)
#[derive(Clone, Debug)]
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
impl LightData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LightData {
            id: String::new(),
            kind: crate::scene_level_data::LightDataKind::Spot,
            pos: [0.0; 3],
            dir: [0.0; 3],
            color: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            inner_deg: 0.0,
            outer_deg: 0.0,
            group: String::new(),
            enabled: false,
            volumetric: 0.0,
            radius: None,
            mains: None,
            broad: None,
            fixture: None,
        }
    }
}
impl Default for LightData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LightDataKind {
    Spot,
    Dir,
    Point,
}
impl LightDataKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LightDataKind::Spot => "spot",
            LightDataKind::Dir => "dir",
            LightDataKind::Point => "point",
        }
    }
}

/// (TypeScript: DoorData, src/scene/levelData.ts:154)
#[derive(Clone, Debug)]
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
impl DoorData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorData {
            id: String::new(),
            floor: None,
            hinge: [0.0; 2],
            closed_dir: 0.0,
            width: 0.0,
            angle: None,
            exterior: None,
            closer: None,
            min_angle: None,
            max_angle: None,
            locked: None,
            lock: None,
            key_side: None,
            swing: None,
            steel: None,
        }
    }
}
impl Default for DoorData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SurfacesData, src/scene/levelData.ts:75)
#[derive(Clone, Debug)]
pub struct SurfacesData {
    pub building: crate::scene_level_data::Rect,
    pub grass_edge: f64,
    pub inside: crate::floor::FloorMat,
    pub outside: crate::floor::FloorMat,
    pub verge: crate::floor::FloorMat,
    pub zones: Vec<crate::scene_level_data::SurfacesDataZonesItem>,
}
impl SurfacesData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SurfacesData {
            building: crate::scene_level_data::Rect::zeroed(),
            grass_edge: 0.0,
            inside: crate::floor::FloorMat::Carpet,
            outside: crate::floor::FloorMat::Carpet,
            verge: crate::floor::FloorMat::Carpet,
            zones: Vec::new(),
        }
    }
}
impl Default for SurfacesData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Rect, src/scene/levelData.ts:59)
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl Rect {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rect {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for Rect { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/levelData.ts:75 SurfacesData.zones, src/scene/levelData.ts:75)
#[derive(Clone, Debug)]
pub struct SurfacesDataZonesItem {
    pub id: String,
    pub mat: crate::floor::FloorMat,
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
    pub notes: Option<String>,
}
impl SurfacesDataZonesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SurfacesDataZonesItem {
            id: String::new(),
            mat: crate::floor::FloorMat::Carpet,
            x0: 0.0,
            z0: 0.0,
            x1: 0.0,
            z1: 0.0,
            notes: None,
        }
    }
}
impl Default for SurfacesDataZonesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/levelData.ts:186 DEVICE_DEFAULTS, src/scene/levelData.ts:186)
#[derive(Clone, Debug)]
pub struct DEVICEDEFAULTS {
    pub pitch: f64,
    pub fov_deg: f64,
    pub range: f64,
    pub sweep_period: f64,
    pub group: String,
    pub alarm: String,
}
impl DEVICEDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DEVICEDEFAULTS {
            pitch: 0.0,
            fov_deg: 0.0,
            range: 0.0,
            sweep_period: 0.0,
            group: String::new(),
            alarm: String::new(),
        }
    }
}
impl Default for DEVICEDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/levelData.ts:194 VENT_DEFAULTS, src/scene/levelData.ts:194)
#[derive(Clone, Copy, Debug)]
pub struct VENTDEFAULTS {
    pub size: f64,
    pub duty: f64,
    pub phase: f64,
    pub mains: bool,
}
impl VENTDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        VENTDEFAULTS {
            size: 0.0,
            duty: 0.0,
            phase: 0.0,
            mains: false,
        }
    }
}
impl Default for VENTDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (src/scene/levelData.ts:186 DEVICE_DEFAULTS: a table nothing writes)
pub fn device_defaults() -> crate::scene_level_data::DEVICEDEFAULTS { crate::scene_level_data::DEVICEDEFAULTS { pitch: -0.35, fov_deg: 70.0, range: 14.0, sweep_period: 9.0, group: String::from("security"), alarm: String::from("suspicion") } }

/// (src/scene/levelData.ts:194 VENT_DEFAULTS: a table nothing writes)
pub fn vent_defaults() -> crate::scene_level_data::VENTDEFAULTS { crate::scene_level_data::VENTDEFAULTS { size: 0.6, duty: 1.0, phase: 0.0, mains: true } }
