//! The types of `gunlook`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ModelCol {
    Lens,
    Metal,
    Torch,
    Shell,
    Steel,
    Cloth,
    Black,
    Poly,
    Furn,
    Brass,
}
impl ModelCol {
    pub const fn as_str(self) -> &'static str {
        match self {
            ModelCol::Lens => "lens",
            ModelCol::Metal => "metal",
            ModelCol::Torch => "torch",
            ModelCol::Shell => "shell",
            ModelCol::Steel => "steel",
            ModelCol::Cloth => "cloth",
            ModelCol::Black => "black",
            ModelCol::Poly => "poly",
            ModelCol::Furn => "furn",
            ModelCol::Brass => "brass",
        }
    }
}

/// (TypeScript: ModelPart, src/game/gunlook.ts:59)
#[derive(Clone, Copy, Debug)]
pub struct ModelPart {
    pub s: crate::gunlook::Seg,
    pub col: crate::gunlook::ModelCol,
    pub is: Option<crate::gunlook::ModelPartIs>,
}
impl ModelPart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ModelPart {
            s: crate::gunlook::Seg::zeroed(),
            col: crate::gunlook::ModelCol::Lens,
            is: None,
        }
    }
}
impl Default for ModelPart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Seg, src/game/gunlook.ts:68)
#[derive(Clone, Copy, Debug)]
pub struct Seg {
    pub l0: f64,
    pub l1: f64,
    pub thick: f64,
    pub tall: f64,
    pub adj: Option<crate::gunlook::SegAdj>,
}
impl Seg {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Seg {
            l0: 0.0,
            l1: 0.0,
            thick: 0.0,
            tall: 0.0,
            adj: None,
        }
    }
}
impl Default for Seg { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunlook.ts:68 Seg.adj, src/game/gunlook.ts:68)
#[derive(Clone, Copy, Debug)]
pub struct SegAdj {
    pub off: V3,
    pub rot: V3,
}
impl SegAdj {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SegAdj {
            off: [0.0; 3],
            rot: [0.0; 3],
        }
    }
}
impl Default for SegAdj { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ModelPartIs {
    Lens,
    Cover,
    Mag,
    Grip,
}
impl ModelPartIs {
    pub const fn as_str(self) -> &'static str {
        match self {
            ModelPartIs::Lens => "lens",
            ModelPartIs::Cover => "cover",
            ModelPartIs::Mag => "mag",
            ModelPartIs::Grip => "grip",
        }
    }
}

/// (TypeScript: LongModel, src/game/gunlook.ts:64)
#[derive(Clone, Debug)]
pub struct LongModel {
    pub parts: Vec<crate::gunlook::ModelPart>,
    pub ahead: f64,
    pub grip_l: crate::gunlook::LongModelGripL,
    pub post: bool,
    pub hinge: Option<V3>,
    pub lens: Option<crate::gunlook::Seg>,
    pub lens_ahead: Option<f64>,
}
impl LongModel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LongModel {
            parts: Vec::new(),
            ahead: 0.0,
            grip_l: crate::gunlook::LongModelGripL::zeroed(),
            post: false,
            hinge: None,
            lens: None,
            lens_ahead: None,
        }
    }
}
impl Default for LongModel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunlook.ts:64 LongModel.gripL, src/game/gunlook.ts:64)
#[derive(Clone, Copy, Debug)]
pub struct LongModelGripL {
    pub off: V3,
    pub fingers: V3,
    pub back: V3,
}
impl LongModelGripL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LongModelGripL {
            off: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for LongModelGripL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PistolModel, src/game/gunlook.ts:67)
#[derive(Clone, Copy, Debug)]
pub struct PistolModel {
    pub slide: crate::gunlook::Seg,
    pub grip: crate::gunlook::Seg,
    pub can: Option<crate::gunlook::Seg>,
    pub mag: Option<crate::gunlook::Seg>,
    pub frame: crate::gunlook::ModelCol,
    pub can_col: Option<crate::gunlook::ModelCol>,
}
impl PistolModel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PistolModel {
            slide: crate::gunlook::Seg::zeroed(),
            grip: crate::gunlook::Seg::zeroed(),
            can: None,
            mag: None,
            frame: crate::gunlook::ModelCol::Lens,
            can_col: None,
        }
    }
}
impl Default for PistolModel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GunLook, src/game/gunlook.ts:196)
#[derive(Clone, Copy, Debug)]
pub struct GunLook {
    pub hand: Option<crate::weapons::GunModelId>,
    pub long: Option<crate::weapons::GunModelId>,
    pub side: Option<crate::weapons::GunModelId>,
    pub bare_long: Option<bool>,
    pub bare_side: Option<bool>,
}
impl GunLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunLook {
            hand: None,
            long: None,
            side: None,
            bare_long: None,
            bare_side: None,
        }
    }
}
impl Default for GunLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunlook.ts:42 MODEL_POINTS, src/game/gunlook.ts:42)
#[derive(Clone, Copy, Debug)]
pub struct MODELPOINTSValue {
    pub well: V3,
    pub cover: Option<V3>,
    pub box_: Option<V3>,
    pub charge: Option<V3>,
    pub port: Option<V3>,
    pub saddle: Option<V3>,
}
impl MODELPOINTSValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MODELPOINTSValue {
            well: [0.0; 3],
            cover: None,
            box_: None,
            charge: None,
            port: None,
            saddle: None,
        }
    }
}
impl Default for MODELPOINTSValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunlook.ts:83 LONG_MODELS, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct LONGMODELS {
    pub m870: Option<crate::gunlook::LongModel>,
    pub mk12: Option<crate::gunlook::LongModel>,
    pub mp7: Option<crate::gunlook::LongModel>,
    pub mk46: Option<crate::gunlook::LongModel>,
    pub m1014: Option<crate::gunlook::LongModel>,
    pub hk45: Option<crate::gunlook::LongModel>,
    pub g18: Option<crate::gunlook::LongModel>,
}
impl LONGMODELS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LONGMODELS {
            m870: None,
            mk12: None,
            mp7: None,
            mk46: None,
            m1014: None,
            hk45: None,
            g18: None,
        }
    }
}
impl Default for LONGMODELS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunlook.ts:182 PISTOL_MODELS, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct PISTOLMODELS {
    pub m870: Option<crate::gunlook::PistolModel>,
    pub mk12: Option<crate::gunlook::PistolModel>,
    pub mp7: Option<crate::gunlook::PistolModel>,
    pub mk46: Option<crate::gunlook::PistolModel>,
    pub m1014: Option<crate::gunlook::PistolModel>,
    pub hk45: Option<crate::gunlook::PistolModel>,
    pub g18: Option<crate::gunlook::PistolModel>,
}
impl PISTOLMODELS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PISTOLMODELS {
            m870: None,
            mk12: None,
            mp7: None,
            mk46: None,
            m1014: None,
            hk45: None,
            g18: None,
        }
    }
}
impl Default for PISTOLMODELS { fn default() -> Self { Self::zeroed() } }

/// (src/game/gunlook.ts:24 SHELLS_BREAK)
pub const SHELLS_BREAK: bool = false;

/// (src/game/gunlook.ts:25 BREAK)
pub const BREAK: bool = false;

/// (src/game/gunlook.ts:42 MODEL_POINTS: a table nothing writes)
pub fn model_points() -> JsMap<String, crate::gunlook::MODELPOINTSValue> { { let mut t = JsMap::new(); t.set(String::from("mk12"), crate::gunlook::MODELPOINTSValue { well: [0.0, -0.06, 0.08], cover: None, box_: None, charge: None, port: None, saddle: None }); t.set(String::from("mp7"), crate::gunlook::MODELPOINTSValue { well: [0.0, -0.135, -0.045], cover: None, box_: None, charge: None, port: None, saddle: None }); t.set(String::from("mk46"), crate::gunlook::MODELPOINTSValue { well: [0.02, -0.085, 0.06], cover: Some([0.0, 0.04, 0.07]), box_: Some([0.02, -0.085, 0.06]), charge: Some([-0.03, 0.0, 0.0]), port: None, saddle: None }); t.set(String::from("m1014"), crate::gunlook::MODELPOINTSValue { well: [0.0, -0.047, 0.06], cover: None, box_: None, charge: None, port: Some([-0.02, 0.005, 0.03]), saddle: Some([0.024, 0.0, 0.0]) }); t.set(String::from("m870"), crate::gunlook::MODELPOINTSValue { well: [0.0, -0.047, 0.06], cover: None, box_: None, charge: None, port: Some([-0.02, 0.005, 0.03]), saddle: None }); t.set(String::from("hk45"), crate::gunlook::MODELPOINTSValue { well: [0.0, -0.1, -0.03], cover: None, box_: None, charge: None, port: None, saddle: None }); t.set(String::from("g18"), crate::gunlook::MODELPOINTSValue { well: [0.0, -0.1, -0.03], cover: None, box_: None, charge: None, port: None, saddle: None }); t } }

/// (src/game/gunlook.ts:71 GRIP: a table nothing writes)
pub fn grip() -> crate::gunlook::Seg { crate::gunlook::Seg { l0: 0.0, l1: 0.09, thick: 0.013, tall: 0.016, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.045, -0.015], rot: [108.0, 0.0, 0.0] }) } }

/// (src/game/gunlook.ts:73 LIGHT: a table nothing writes)
pub fn light() -> crate::gunlook::Seg { crate::gunlook::Seg { l0: 0.19, l1: 0.27, thick: 0.011, tall: 0.011, adj: Some(crate::gunlook::SegAdj { off: [-0.034, -0.008, 0.0], rot: [0.0, 0.0, 0.0] }) } }

/// (src/game/gunlook.ts:74 LENS: a table nothing writes)
pub fn lens() -> crate::gunlook::Seg { crate::gunlook::Seg { l0: 0.27, l1: 0.285, thick: 0.012, tall: 0.012, adj: Some(crate::gunlook::SegAdj { off: [-0.034, -0.008, 0.0], rot: [0.0, 0.0, 0.0] }) } }

/// (src/game/gunlook.ts:76 LIGHT_C: a table nothing writes)
pub fn light_c() -> crate::gunlook::Seg { crate::gunlook::Seg { l0: 0.18, l1: 0.265, thick: 0.011, tall: 0.011, adj: Some(crate::gunlook::SegAdj { off: [-0.032, -0.006, 0.0], rot: [0.0, 0.0, 0.0] }) } }

/// (src/game/gunlook.ts:77 LENS_C: a table nothing writes)
pub fn lens_c() -> crate::gunlook::Seg { crate::gunlook::Seg { l0: 0.265, l1: 0.28, thick: 0.012, tall: 0.012, adj: Some(crate::gunlook::SegAdj { off: [-0.032, -0.006, 0.0], rot: [0.0, 0.0, 0.0] }) } }

/// (src/game/gunlook.ts:83 LONG_MODELS: a table nothing writes)
pub fn long_models() -> crate::gunlook::LONGMODELS { crate::gunlook::LONGMODELS { m870: Some(crate::gunlook::LongModel { parts: alloc::vec![crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.09, l1: 0.12, thick: 0.018, tall: 0.03, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.01, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Steel, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.31, l1: -0.07, thick: 0.015, tall: 0.026, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.028, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Poly, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::grip(), col: crate::gunlook::ModelCol::Poly, is: Some(crate::gunlook::ModelPartIs::Grip) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.12, l1: 0.37, thick: 0.0095, tall: 0.0095, adj: None }, col: crate::gunlook::ModelCol::Steel, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.37, l1: 0.41, thick: 0.013, tall: 0.013, adj: None }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.12, l1: 0.34, thick: 0.0105, tall: 0.0105, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.024, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Steel, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.17, l1: 0.29, thick: 0.02, tall: 0.021, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.02, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Poly, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::light_c(), col: crate::gunlook::ModelCol::Torch, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::lens_c(), col: crate::gunlook::ModelCol::Lens, is: Some(crate::gunlook::ModelPartIs::Lens) }], ahead: 0.41, grip_l: crate::gunlook::LongModelGripL { off: [0.099, -0.059, 0.149], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] }, post: false, hinge: None, lens: None, lens_ahead: None }), mk12: Some(crate::gunlook::LongModel { parts: alloc::vec![crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.075, l1: 0.115, thick: 0.017, tall: 0.036, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.31, l1: -0.075, thick: 0.016, tall: 0.029, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.022, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.0, l1: 0.09, thick: 0.011, tall: 0.03, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.045, 0.075], rot: [80.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: Some(crate::gunlook::ModelPartIs::Mag) }, crate::gunlook::ModelPart { s: crate::gunlook::grip(), col: crate::gunlook::ModelCol::Furn, is: Some(crate::gunlook::ModelPartIs::Grip) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.115, l1: 0.4, thick: 0.021, tall: 0.021, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.002, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Furn, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.4, l1: 0.47, thick: 0.009, tall: 0.009, adj: None }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.47, l1: 0.61, thick: 0.019, tall: 0.019, adj: None }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.07, l1: 0.4, thick: 0.009, tall: 0.005, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.027, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.02, l1: 0.11, thick: 0.011, tall: 0.007, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.039, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.05, l1: 0.16, thick: 0.013, tall: 0.013, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.058, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.16, l1: 0.215, thick: 0.019, tall: 0.019, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.058, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.04, l1: 0.07, thick: 0.022, tall: 0.012, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.074, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.27, l1: 0.42, thick: 0.017, tall: 0.006, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.031, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.3, l1: 0.375, thick: 0.017, tall: 0.011, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.043, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Furn, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::light(), col: crate::gunlook::ModelCol::Torch, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::lens(), col: crate::gunlook::ModelCol::Lens, is: Some(crate::gunlook::ModelPartIs::Lens) }], ahead: 0.61, grip_l: crate::gunlook::LongModelGripL { off: [0.1, -0.056, 0.2], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] }, post: false, hinge: None, lens: None, lens_ahead: None }), mp7: Some(crate::gunlook::LongModel { parts: alloc::vec![crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.11, l1: 0.16, thick: 0.021, tall: 0.031, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.008, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.255, l1: -0.11, thick: 0.004, tall: 0.005, adj: Some(crate::gunlook::SegAdj { off: [0.014, 0.004, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.255, l1: -0.11, thick: 0.004, tall: 0.005, adj: Some(crate::gunlook::SegAdj { off: [-0.014, 0.004, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.27, l1: -0.255, thick: 0.018, tall: 0.034, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.0, l1: 0.095, thick: 0.015, tall: 0.019, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.045, -0.015], rot: [108.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: Some(crate::gunlook::ModelPartIs::Grip) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.09, l1: 0.2, thick: 0.011, tall: 0.014, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.045, -0.015], rot: [108.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: Some(crate::gunlook::ModelPartIs::Mag) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.16, l1: 0.31, thick: 0.017, tall: 0.017, adj: None }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.09, l1: 0.15, thick: 0.009, tall: 0.004, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.027, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.01, l1: 0.04, thick: 0.014, tall: 0.014, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.045, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.0, l1: 0.075, thick: 0.011, tall: 0.012, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.04, 0.13], rot: [90.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.075, l1: 0.14, thick: 0.01, tall: 0.01, adj: Some(crate::gunlook::SegAdj { off: [-0.031, -0.004, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Torch, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.14, l1: 0.153, thick: 0.011, tall: 0.011, adj: Some(crate::gunlook::SegAdj { off: [-0.031, -0.004, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Lens, is: Some(crate::gunlook::ModelPartIs::Lens) }], ahead: 0.31, grip_l: crate::gunlook::LongModelGripL { off: [0.06, -0.082, 0.075], fingers: [-0.3, 0.05, 0.95], back: [0.95, -0.15, 0.25] }, post: true, hinge: None, lens: Some(crate::gunlook::Seg { l0: 0.14, l1: 0.153, thick: 0.011, tall: 0.011, adj: Some(crate::gunlook::SegAdj { off: [-0.031, -0.004, 0.0], rot: [0.0, 0.0, 0.0] }) }), lens_ahead: Some(0.158) }), mk46: Some(crate::gunlook::LongModel { parts: alloc::vec![crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.1, l1: 0.14, thick: 0.027, tall: 0.034, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.06, l1: 0.13, thick: 0.03, tall: 0.011, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.033, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: Some(crate::gunlook::ModelPartIs::Cover) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.02, l1: 0.07, thick: 0.015, tall: 0.016, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.06, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: Some(crate::gunlook::ModelPartIs::Cover) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.31, l1: -0.1, thick: 0.015, tall: 0.032, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.02, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Poly, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::grip(), col: crate::gunlook::ModelCol::Poly, is: Some(crate::gunlook::ModelPartIs::Grip) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.005, l1: 0.125, thick: 0.034, tall: 0.045, adj: Some(crate::gunlook::SegAdj { off: [0.022, -0.092, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Cloth, is: Some(crate::gunlook::ModelPartIs::Mag) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.03, l1: 0.09, thick: 0.01, tall: 0.034, adj: Some(crate::gunlook::SegAdj { off: [0.034, -0.018, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Brass, is: Some(crate::gunlook::ModelPartIs::Mag) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.14, l1: 0.33, thick: 0.024, tall: 0.022, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.33, l1: 0.5, thick: 0.009, tall: 0.009, adj: None }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.33, l1: 0.43, thick: 0.007, tall: 0.007, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.022, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.5, l1: 0.53, thick: 0.012, tall: 0.012, adj: None }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.17, l1: 0.37, thick: 0.019, tall: 0.006, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.042, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.2, l1: 0.275, thick: 0.017, tall: 0.011, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.021, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Furn, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::light(), col: crate::gunlook::ModelCol::Torch, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::lens(), col: crate::gunlook::ModelCol::Lens, is: Some(crate::gunlook::ModelPartIs::Lens) }], ahead: 0.53, grip_l: crate::gunlook::LongModelGripL { off: [0.103, -0.062, 0.23], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] }, post: false, hinge: Some([0.0, 0.022, -0.06]), lens: None, lens_ahead: None }), m1014: Some(crate::gunlook::LongModel { parts: alloc::vec![crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.09, l1: 0.13, thick: 0.018, tall: 0.03, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.01, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.31, l1: -0.09, thick: 0.012, tall: 0.017, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.016, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.325, l1: -0.305, thick: 0.016, tall: 0.045, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.022, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Poly, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::grip(), col: crate::gunlook::ModelCol::Poly, is: Some(crate::gunlook::ModelPartIs::Grip) }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.13, l1: 0.47, thick: 0.0095, tall: 0.0095, adj: None }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.13, l1: 0.41, thick: 0.0105, tall: 0.0105, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.024, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.13, l1: 0.31, thick: 0.019, tall: 0.027, adj: Some(crate::gunlook::SegAdj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.06, l1: 0.08, thick: 0.008, tall: 0.004, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.024, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Black, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: 0.435, l1: 0.455, thick: 0.004, tall: 0.012, adj: Some(crate::gunlook::SegAdj { off: [0.0, 0.018, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Metal, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::Seg { l0: -0.05, l1: 0.07, thick: 0.007, tall: 0.012, adj: Some(crate::gunlook::SegAdj { off: [0.025, -0.004, 0.0], rot: [0.0, 0.0, 0.0] }) }, col: crate::gunlook::ModelCol::Shell, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::light_c(), col: crate::gunlook::ModelCol::Torch, is: None }, crate::gunlook::ModelPart { s: crate::gunlook::lens_c(), col: crate::gunlook::ModelCol::Lens, is: Some(crate::gunlook::ModelPartIs::Lens) }], ahead: 0.47, grip_l: crate::gunlook::LongModelGripL { off: [0.099, -0.059, 0.149], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] }, post: false, hinge: None, lens: None, lens_ahead: None }), hk45: None, g18: None } }

/// (src/game/gunlook.ts:182 PISTOL_MODELS: a table nothing writes)
pub fn pistol_models() -> crate::gunlook::PISTOLMODELS { crate::gunlook::PISTOLMODELS { m870: None, mk12: None, mp7: None, mk46: None, m1014: None, hk45: Some(crate::gunlook::PistolModel { slide: crate::gunlook::Seg { l0: 0.02, l1: 0.215, thick: 0.0175, tall: 0.027, adj: None }, grip: crate::gunlook::Seg { l0: 0.0, l1: 0.1, thick: 0.016, tall: 0.019, adj: None }, can: Some(crate::gunlook::Seg { l0: 0.215, l1: 0.4, thick: 0.0215, tall: 0.0215, adj: None }), mag: None, frame: crate::gunlook::ModelCol::Furn, can_col: Some(crate::gunlook::ModelCol::Black) }), g18: Some(crate::gunlook::PistolModel { slide: crate::gunlook::Seg { l0: 0.02, l1: 0.22, thick: 0.016, tall: 0.025, adj: None }, grip: crate::gunlook::Seg { l0: 0.0, l1: 0.095, thick: 0.014, tall: 0.017, adj: None }, can: None, mag: Some(crate::gunlook::Seg { l0: 0.09, l1: 0.2, thick: 0.011, tall: 0.0135, adj: None }), frame: crate::gunlook::ModelCol::Steel, can_col: None }) } }
