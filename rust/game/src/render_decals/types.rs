//! The types of `render_decals`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DecalKind {
    Bullet,
    Scorch,
    Scuff,
    Chip,
    GlassHit,
    Wetprint,
    Damage,
}
impl DecalKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DecalKind::Bullet => "bullet",
            DecalKind::Scorch => "scorch",
            DecalKind::Scuff => "scuff",
            DecalKind::Chip => "chip",
            DecalKind::GlassHit => "glassHit",
            DecalKind::Wetprint => "wetprint",
            DecalKind::Damage => "damage",
        }
    }
}

/// (TypeScript: Decal, src/render/decals.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct Decal {
    pub kind: crate::render_decals::DecalKind,
    pub pos: V3,
    pub n: V3,
    pub t: V3,
    pub radius: f64,
    pub param: f64,
    pub seed: f64,
    pub serial: f64,
    pub pane: f64,
    pub box_: Option<BoxId>,
    pub local: Option<crate::render_decals::DecalLocal>,
    pub look: Option<bool>,
    pub box_idx: f64,
    pub hidden: bool,
}
impl Decal {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Decal {
            kind: crate::render_decals::DecalKind::Bullet,
            pos: [0.0; 3],
            n: [0.0; 3],
            t: [0.0; 3],
            radius: 0.0,
            param: 0.0,
            seed: 0.0,
            serial: 0.0,
            pane: 0.0,
            box_: None,
            local: None,
            look: None,
            box_idx: 0.0,
            hidden: false,
        }
    }
}
impl Default for Decal { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/decals.ts:49 Decal.local, src/render/decals.ts:49)
#[derive(Clone, Copy, Debug)]
pub struct DecalLocal {
    pub p: V3,
    pub n: V3,
    pub t: V3,
}
impl DecalLocal {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DecalLocal {
            p: [0.0; 3],
            n: [0.0; 3],
            t: [0.0; 3],
        }
    }
}
impl Default for DecalLocal { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DecalOpts, src/render/decals.ts:58)
#[derive(Clone, Copy, Debug)]
pub struct DecalOpts {
    pub tangent: Option<V3>,
    pub param: Option<f64>,
    pub look: Option<bool>,
    pub attach: Option<BoxId>,
    pub clamp_to: Option<BoxId>,
    pub seed: Option<f64>,
    pub pane: Option<f64>,
}
impl DecalOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DecalOpts {
            tangent: None,
            param: None,
            look: None,
            attach: None,
            clamp_to: None,
            seed: None,
            pane: None,
        }
    }
}
impl Default for DecalOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Decals, src/render/decals.ts:130)
#[derive(Clone, Debug)]
pub struct Decals {
    pub list: Vec<crate::render_decals::Decal>,
    pub enabled: bool,
    pub buf: crate::Opaque,
    pub dirty: bool,
    pub serial: f64,
    pub device: crate::Opaque,
}
impl Decals {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Decals {
            list: Vec::new(),
            enabled: true,
            buf: crate::Opaque,
            dirty: true,
            serial: 0.0,
            device: crate::Opaque,
        }
    }
}
impl Default for Decals { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/render/decals.ts:96 WEAR_MARK, src/render/decals.ts:96)
#[derive(Clone, Copy, Debug)]
pub struct WEARMARK {
    pub look: f64,
    pub materials: f64,
    pub reach0: f64,
    pub reach1: f64,
    pub min_half: f64,
    pub max_half: f64,
    pub face_tol: f64,
}
impl WEARMARK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WEARMARK {
            look: 0.0,
            materials: 0.0,
            reach0: 0.0,
            reach1: 0.0,
            min_half: 0.0,
            max_half: 0.0,
            face_tol: 0.0,
        }
    }
}
impl Default for WEARMARK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/decals.ts:103 DEFAULTS, src/render/decals.ts:103)
#[derive(Clone, Copy, Debug)]
pub struct DEFAULTSValue {
    pub radius: f64,
    pub param: f64,
}
impl DEFAULTSValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DEFAULTSValue {
            radius: 0.0,
            param: 0.0,
        }
    }
}
impl Default for DEFAULTSValue { fn default() -> Self { Self::zeroed() } }

/// (src/render/decals.ts:38 DECAL_ANY_BOX)
pub const DECAL_ANY_BOX: f64 = 65535.0;

/// (src/render/decals.ts:77 CELLS)
pub const CELLS: f64 = 1120.0;

/// (src/render/decals.ts:78 CELLS_V4: a table nothing writes)
pub fn cells_v4() -> f64 { todo!() }

/// (src/render/decals.ts:79 DECAL_CAP)
pub const DECAL_CAP: f64 = 512.0;

/// (src/render/decals.ts:80 DECAL_LIST_CAP)
pub const DECAL_LIST_CAP: f64 = 4096.0;

/// (src/render/decals.ts:82 LIST_V4)
pub const LIST_V4: f64 = 1024.0;

/// (src/render/decals.ts:83 CELL0)
pub const CELL0: f64 = 1.0;

/// (src/render/decals.ts:83 LIST0: a table nothing writes)
pub fn list0() -> f64 { todo!() }

/// (src/render/decals.ts:83 REC0: a table nothing writes)
pub fn rec0() -> f64 { todo!() }

/// (src/render/decals.ts:84 TOTAL_V4: a table nothing writes)
pub fn total_v4() -> f64 { todo!() }

/// (src/render/decals.ts:85 DECAL_BYTES: a table nothing writes)
pub fn decal_bytes() -> f64 { todo!() }

/// (src/render/decals.ts:96 WEAR_MARK: a table nothing writes)
pub fn wear_mark() -> crate::render_decals::WEARMARK { crate::render_decals::WEARMARK { look: 3.0, materials: 3.0, reach0: 0.09, reach1: 0.21, min_half: 0.035, max_half: 0.09, face_tol: 0.015 } }

/// (src/render/decals.ts:103 DEFAULTS: a table nothing writes)
pub fn defaults() -> JsMap<String, crate::render_decals::DEFAULTSValue> { { let mut t = JsMap::new(); t.set(String::from("bullet"), crate::render_decals::DEFAULTSValue { radius: 0.03, param: 1.0 }); t.set(String::from("scorch"), crate::render_decals::DEFAULTSValue { radius: 0.75, param: 1.0 }); t.set(String::from("scuff"), crate::render_decals::DEFAULTSValue { radius: 0.1, param: 0.16 }); t.set(String::from("chip"), crate::render_decals::DEFAULTSValue { radius: 0.013, param: 1.0 }); t.set(String::from("glassHit"), crate::render_decals::DEFAULTSValue { radius: 0.12, param: 1.0 }); t.set(String::from("wetprint"), crate::render_decals::DEFAULTSValue { radius: 0.17, param: 1.0 }); t.set(String::from("damage"), crate::render_decals::DEFAULTSValue { radius: 0.05, param: 0.8 }); t } }

/// (src/render/decals.ts:112 SCUFF_HALF_W)
pub const SCUFF_HALF_W: f64 = 0.016;
