//! The types of `packside`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: PackExt, src/game/packside.ts:43)
#[derive(Clone, Copy, Debug)]
pub struct PackExt {
    pub part: f64,
    pub x0: f64,
    pub x1: f64,
    pub top0: f64,
    pub top1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl PackExt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PackExt {
            part: 0.0,
            x0: 0.0,
            x1: 0.0,
            top0: 0.0,
            top1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for PackExt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BackPt, src/game/packside.ts:45)
#[derive(Clone, Copy, Debug)]
pub struct BackPt {
    pub j: f64,
    pub y: f64,
    pub z: f64,
}
impl BackPt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BackPt {
            j: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
impl Default for BackPt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Memo, src/game/packside.ts:46)
#[derive(Clone, Debug)]
pub struct Memo {
    pub j3: f64,
    pub ext: Vec<Option<crate::packside::PackExt>>,
    pub kit: f64,
    pub ram: f64,
    pub pack: f64,
    pub body: f64,
    pub back: JsMap<crate::Num, Vec<crate::packside::BackPt>>,
}
impl Memo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Memo {
            j3: 0.0,
            ext: Vec::new(),
            kit: 0.0,
            ram: 0.0,
            pack: 0.0,
            body: 0.0,
            back: JsMap::new(),
        }
    }
}
impl Default for Memo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PackChar, src/game/packside.ts:48)
#[derive(Clone, Debug)]
pub struct PackChar {
    pub cast: Option<crate::packside::PackCharCast>,
    pub mesh: crate::packside::PackCharMesh,
    pub skin: crate::packside::PackCharSkin,
    pub node_by_name: JsMap<String, f64>,
    pub nodes: Vec<crate::packside::PackCharNodesItem>,
}
impl PackChar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PackChar {
            cast: None,
            mesh: crate::packside::PackCharMesh::zeroed(),
            skin: crate::packside::PackCharSkin::zeroed(),
            node_by_name: JsMap::new(),
            nodes: Vec::new(),
        }
    }
}
impl Default for PackChar { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/packside.ts:48 PackChar.cast, src/game/packside.ts:48)
#[derive(Clone, Debug)]
pub struct PackCharCast {
    pub parts: Vec<String>,
    pub variants: Option<Vec<crate::packside::PackCharCastVariantsItem>>,
}
impl PackCharCast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PackCharCast {
            parts: Vec::new(),
            variants: None,
        }
    }
}
impl Default for PackCharCast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/packside.ts:48 PackCharCast.variants, src/game/packside.ts:48)
#[derive(Clone, Debug)]
pub struct PackCharCastVariantsItem {
    pub side: String,
    pub mask: f64,
}
impl PackCharCastVariantsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PackCharCastVariantsItem {
            side: String::new(),
            mask: 0.0,
        }
    }
}
impl Default for PackCharCastVariantsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/packside.ts:48 PackChar.mesh, src/game/packside.ts:48)
#[derive(Clone, Debug)]
pub struct PackCharMesh {
    pub positions: Vec<f32>,
    pub part: Option<Vec<u8>>,
    pub vertex_count: f64,
    pub joints: Vec<f64>,
    pub weights: Vec<f64>,
}
impl PackCharMesh {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PackCharMesh {
            positions: Vec::new(),
            part: None,
            vertex_count: 0.0,
            joints: Vec::new(),
            weights: Vec::new(),
        }
    }
}
impl Default for PackCharMesh { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/packside.ts:48 PackChar.skin, src/game/packside.ts:48)
#[derive(Clone, Debug)]
pub struct PackCharSkin {
    pub joints: Vec<f64>,
    pub inverse_bind: Vec<f32>,
}
impl PackCharSkin {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PackCharSkin {
            joints: Vec::new(),
            inverse_bind: Vec::new(),
        }
    }
}
impl Default for PackCharSkin { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/packside.ts:48 PackChar.nodes, src/game/packside.ts:48)
#[derive(Clone, Debug)]
pub struct PackCharNodesItem {
    pub name: String,
}
impl PackCharNodesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PackCharNodesItem {
            name: String::new(),
        }
    }
}
impl Default for PackCharNodesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SidePlace, src/game/packside.ts:90)
#[derive(Clone, Copy, Debug)]
pub struct SidePlace {
    pub o: V3,
    pub x: V3,
    pub y: V3,
    pub z: V3,
    pub pivot: Option<V3>,
    pub len: Option<f64>,
    pub r: Option<f64>,
}
impl SidePlace {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SidePlace {
            o: [0.0; 3],
            x: [0.0; 3],
            y: [0.0; 3],
            z: [0.0; 3],
            pivot: None,
            len: None,
            r: None,
        }
    }
}
impl Default for SidePlace { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SideBindWhat {
    Rifle,
    Shotgun,
    Hammer,
}
impl SideBindWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            SideBindWhat::Rifle => "rifle",
            SideBindWhat::Shotgun => "shotgun",
            SideBindWhat::Hammer => "hammer",
        }
    }
}

/// (TypeScript: inline at src/game/packside.ts:95 sideBind(dims), src/game/packside.ts:95)
#[derive(Clone, Copy, Debug)]
pub struct SideBindDims {
    pub grip: Option<f64>,
    pub thick: f64,
    pub head: Option<V3>,
    pub rear: Option<f64>,
    pub top: Option<f64>,
    pub len: Option<f64>,
}
impl SideBindDims {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideBindDims {
            grip: None,
            thick: 0.0,
            head: None,
            rear: None,
            top: None,
            len: None,
        }
    }
}
impl Default for SideBindDims { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct PacksideMod {
    /// (src/game/packside.ts:49 memo)
    pub memo: JsMap<crate::ObjKey, crate::packside::Memo>,
}
impl PacksideMod {
    pub fn blank() -> Self {
        PacksideMod {
            memo: JsMap::new(),
        }
    }
}

/// (src/game/packside.ts:51 STRIP)
pub const STRIP: f64 = 0.075;

/// (src/game/packside.ts:52 Y0)
pub const Y0: f64 = 0.9;

/// (src/game/packside.ts:52 Y1)
pub const Y1: f64 = 1.62;

/// (src/game/packside.ts:52 ZB)
pub const ZB: f64 = -0.15;

/// (src/game/packside.ts:106 BAND)
pub const BAND: f64 = 0.04;

/// (src/game/packside.ts:106 TRUNK)
pub const TRUNK: &str = "/^DEF-(hips|spine\\.00[1-3]|thigh\\.[LR])$/";

/// (src/game/packside.ts:106 REACH)
pub const REACH: f64 = 0.14;
