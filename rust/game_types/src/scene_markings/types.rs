//! The types of `scene_markings`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum MarkFace {
    #[default] X,
    X2,
    Y,
    Y2,
    Z,
    Z2,
}
impl MarkFace {
    pub const fn as_str(self) -> &'static str {
        match self {
            MarkFace::X => "+x",
            MarkFace::X2 => "-x",
            MarkFace::Y => "+y",
            MarkFace::Y2 => "-y",
            MarkFace::Z => "+z",
            MarkFace::Z2 => "-z",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum MarkRes {
    Base,
    Fine,
    Coarse,
}
impl MarkRes {
    pub const fn as_str(self) -> &'static str {
        match self {
            MarkRes::Base => "base",
            MarkRes::Fine => "fine",
            MarkRes::Coarse => "coarse",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MarkPal {
    pub albedo: Option<V3>,
    pub mul: Option<f64>,
    pub rough: Option<f64>,
    pub emit: Option<V3>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MarkingDef {
    pub face: crate::scene_markings::MarkFace,
    pub res: Option<crate::scene_markings::MarkRes>,
    pub at: [f64; 2],
    pub size: [f64; 2],
    pub pal: Vec<crate::scene_markings::MarkPal>,
    pub cells: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MarkStats {
    pub texel: f64,
    pub w: f64,
    pub h: f64,
    pub f: [f64; 3],
    pub alb: [V3; 3],
    pub mean_l: V3,
    pub area: f64,
    pub d_albedo: V3,
    pub emask: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FACEAXESItem {
    pub u: V3,
    pub v: V3,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FaceHalf {
    pub hu: f64,
    pub hv: f64,
    pub hn: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MarkWorldRect {
    pub centre: V3,
    pub n: V3,
    pub hu: f64,
    pub hv: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SceneMarkingsMod {
    /// (src/scene/markings.ts:369 AREA_LIT_MARKS)
    pub area_lit_marks: JsSet<u32>,
}
impl SceneMarkingsMod {
    pub fn blank() -> Self {
        SceneMarkingsMod {
            area_lit_marks: JsSet::new(),
        }
    }
}

/// (src/scene/markings.ts:21 MARKING_TEXEL)
pub const MARKING_TEXEL: f64 = 0.025;

/// (src/scene/markings.ts:23 MAX_CELLS_W)
pub const MAX_CELLS_W: f64 = 32.0;

/// (src/scene/markings.ts:23 MAX_CELLS_H)
pub const MAX_CELLS_H: f64 = 32.0;

/// (src/scene/markings.ts:26 MARK_TABLE_ROWS)
pub const MARK_TABLE_ROWS: f64 = 6144.0;

/// (src/scene/markings.ts:33 RES_K: a table nothing writes)
pub fn res_k() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("fine"), 0.5); t.set(String::from("base"), 1.0); t.set(String::from("coarse"), 2.0); t } }

/// (src/scene/markings.ts:50 CELL_CHARS)
pub const CELL_CHARS: &str = ".abc";

/// (src/scene/markings.ts:53 FACE_AXES: a table nothing writes)
pub fn face_axes() -> Vec<crate::scene_markings::FACEAXESItem> { alloc::vec![crate::scene_markings::FACEAXESItem { u: [0.0, 0.0, -1.0], v: [0.0, 1.0, 0.0], n: [1.0, 0.0, 0.0] }, crate::scene_markings::FACEAXESItem { u: [0.0, 0.0, 1.0], v: [0.0, 1.0, 0.0], n: [-1.0, 0.0, 0.0] }, crate::scene_markings::FACEAXESItem { u: [1.0, 0.0, 0.0], v: [0.0, 0.0, -1.0], n: [0.0, 1.0, 0.0] }, crate::scene_markings::FACEAXESItem { u: [1.0, 0.0, 0.0], v: [0.0, 0.0, 1.0], n: [0.0, -1.0, 0.0] }, crate::scene_markings::FACEAXESItem { u: [1.0, 0.0, 0.0], v: [0.0, 1.0, 0.0], n: [0.0, 0.0, 1.0] }, crate::scene_markings::FACEAXESItem { u: [-1.0, 0.0, 0.0], v: [0.0, 1.0, 0.0], n: [0.0, 0.0, -1.0] }] }
