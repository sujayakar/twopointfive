//! The types of `scene_markings`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MarkFace {
    X,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: MarkPal, src/scene/markings.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct MarkPal {
    pub albedo: Option<V3>,
    pub mul: Option<f64>,
    pub rough: Option<f64>,
    pub emit: Option<V3>,
}
impl MarkPal {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MarkPal {
            albedo: None,
            mul: None,
            rough: None,
            emit: None,
        }
    }
}
impl Default for MarkPal { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: MarkingDef, src/scene/markings.ts:38)
#[derive(Clone, Debug)]
pub struct MarkingDef {
    pub face: crate::scene_markings::MarkFace,
    pub res: Option<crate::scene_markings::MarkRes>,
    pub at: [f64; 2],
    pub size: [f64; 2],
    pub pal: Vec<crate::scene_markings::MarkPal>,
    pub cells: Vec<String>,
}
impl MarkingDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MarkingDef {
            face: crate::scene_markings::MarkFace::X,
            res: None,
            at: [0.0; 2],
            size: [0.0; 2],
            pal: Vec::new(),
            cells: Vec::new(),
        }
    }
}
impl Default for MarkingDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CellCanvas, src/scene/markings.ts:119)
#[derive(Clone, Debug)]
pub struct CellCanvas {
    pub cells: Vec<u8>,
    pub w: f64,
    pub h: f64,
}
impl CellCanvas {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        CellCanvas {
            cells: Vec::new(),
            w: 0.0,
            h: 0.0,
        }
    }
}
impl Default for CellCanvas { fn default() -> Self { Self::blank() } }

/// (TypeScript: MarkStats, src/scene/markings.ts:316)
#[derive(Clone, Copy, Debug)]
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
impl MarkStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MarkStats {
            texel: 0.0,
            w: 0.0,
            h: 0.0,
            f: [0.0; 3],
            alb: [[0.0; 3]; 3],
            mean_l: [0.0; 3],
            area: 0.0,
            d_albedo: [0.0; 3],
            emask: 0.0,
        }
    }
}
impl Default for MarkStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/markings.ts:53 FACE_AXES, src/scene/markings.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct FACEAXESItem {
    pub u: V3,
    pub v: V3,
    pub n: V3,
}
impl FACEAXESItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FACEAXESItem {
            u: [0.0; 3],
            v: [0.0; 3],
            n: [0.0; 3],
        }
    }
}
impl Default for FACEAXESItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/markings.ts:58 faceHalf ->, src/scene/markings.ts:58)
#[derive(Clone, Copy, Debug)]
pub struct FaceHalf {
    pub hu: f64,
    pub hv: f64,
    pub hn: f64,
}
impl FaceHalf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FaceHalf {
            hu: 0.0,
            hv: 0.0,
            hn: 0.0,
        }
    }
}
impl Default for FaceHalf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/markings.ts:63 texelOf(m), src/scene/markings.ts:63)
#[derive(Clone, Copy, Debug)]
pub struct TexelOfM {
    pub res: Option<crate::scene_markings::MarkRes>,
}
impl TexelOfM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TexelOfM {
            res: None,
        }
    }
}
impl Default for TexelOfM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/markings.ts:370 faceAreas ->, src/scene/markings.ts:370)
#[derive(Clone, Debug)]
pub struct FaceAreas {
    pub areas: Vec<f64>,
    pub total: f64,
}
impl FaceAreas {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FaceAreas {
            areas: Vec::new(),
            total: 0.0,
        }
    }
}
impl Default for FaceAreas { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/markings.ts:383 markWorldRect ->, src/scene/markings.ts:383)
#[derive(Clone, Copy, Debug)]
pub struct MarkWorldRect {
    pub centre: V3,
    pub n: V3,
    pub hu: f64,
    pub hv: f64,
}
impl MarkWorldRect {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MarkWorldRect {
            centre: [0.0; 3],
            n: [0.0; 3],
            hu: 0.0,
            hv: 0.0,
        }
    }
}
impl Default for MarkWorldRect { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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

/// (src/scene/markings.ts:25 MARK_HEAD_ROWS)
pub const MARK_HEAD_ROWS: f64 = 8.0;

/// (src/scene/markings.ts:25 CELLS_PER_FLOAT)
pub const CELLS_PER_FLOAT: f64 = 12.0;

/// (src/scene/markings.ts:25 CELLS_PER_ROW)
pub const CELLS_PER_ROW: f64 = 48.0;

/// (src/scene/markings.ts:26 MARK_TABLE_ROWS)
pub const MARK_TABLE_ROWS: f64 = 6144.0;

/// (src/scene/markings.ts:30 MARK_FACES: a table nothing writes)
pub fn mark_faces() -> Vec<String> { alloc::vec![String::from("+x"), String::from("-x"), String::from("+y"), String::from("-y"), String::from("+z"), String::from("-z")] }

/// (src/scene/markings.ts:32 MARK_RES: a table nothing writes)
pub fn mark_res() -> Vec<String> { alloc::vec![String::from("fine"), String::from("base"), String::from("coarse")] }

/// (src/scene/markings.ts:33 RES_K: a table nothing writes)
pub fn res_k() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("fine"), 0.5); t.set(String::from("base"), 1.0); t.set(String::from("coarse"), 2.0); t } }

/// (src/scene/markings.ts:50 CELL_CHARS)
pub const CELL_CHARS: &str = ".abc";

/// (src/scene/markings.ts:53 FACE_AXES: a table nothing writes)
pub fn face_axes() -> Vec<crate::scene_markings::FACEAXESItem> { alloc::vec![crate::scene_markings::FACEAXESItem { u: [0.0, 0.0, -1.0], v: [0.0, 1.0, 0.0], n: [1.0, 0.0, 0.0] }, crate::scene_markings::FACEAXESItem { u: [0.0, 0.0, 1.0], v: [0.0, 1.0, 0.0], n: [-1.0, 0.0, 0.0] }, crate::scene_markings::FACEAXESItem { u: [1.0, 0.0, 0.0], v: [0.0, 0.0, -1.0], n: [0.0, 1.0, 0.0] }, crate::scene_markings::FACEAXESItem { u: [1.0, 0.0, 0.0], v: [0.0, 0.0, 1.0], n: [0.0, -1.0, 0.0] }, crate::scene_markings::FACEAXESItem { u: [1.0, 0.0, 0.0], v: [0.0, 1.0, 0.0], n: [0.0, 0.0, 1.0] }, crate::scene_markings::FACEAXESItem { u: [-1.0, 0.0, 0.0], v: [0.0, 1.0, 0.0], n: [0.0, 0.0, -1.0] }] }
