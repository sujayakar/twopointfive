//! The types of `render_char_materials`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub enum CharSlot {
    #[default] Boot,
    ClothPlain,
    Skin,
    ClothUniform,
    Webbing,
    Helmet,
    MetalDark,
    Rubber,
    Lens,
    Accent,
    Eye,
}
impl CharSlot {
    pub const fn as_str(self) -> &'static str {
        match self {
            CharSlot::Boot => "boot",
            CharSlot::ClothPlain => "cloth_plain",
            CharSlot::Skin => "skin",
            CharSlot::ClothUniform => "cloth_uniform",
            CharSlot::Webbing => "webbing",
            CharSlot::Helmet => "helmet",
            CharSlot::MetalDark => "metal_dark",
            CharSlot::Rubber => "rubber",
            CharSlot::Lens => "lens",
            CharSlot::Accent => "accent",
            CharSlot::Eye => "eye",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CharSlotData {
    pub slot: Vec<u8>,
    pub cavity: Vec<f32>,
    pub authored: bool,
    pub cap_weights: Vec<f32>,
    pub area: Vec<f32>,
    pub unmatched: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SlotWeights {
    pub area: Vec<f32>,
    pub cap_weights: Vec<f32>,
}

#[derive(Clone, Debug, Default)]
pub struct SlotMat {
    pub albedo: Option<V3>,
    pub src: Option<crate::render_char_materials::SlotMatSrc>,
    pub k: Option<f64>,
    pub mix: Option<f64>,
    pub pick: Option<Vec<V3>>,
    pub rough: f64,
    pub spec: f64,
    pub metal: Option<f64>,
    pub soak: f64,
    pub grime: f64,
    pub wear: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SlotMatSrc {
    Tint,
    Tint2,
}
impl SlotMatSrc {
    pub const fn as_str(self) -> &'static str {
        match self {
            SlotMatSrc::Tint => "tint",
            SlotMatSrc::Tint2 => "tint2",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SlotAlias {
    pub as_: crate::render_char_materials::CharSlot,
}

#[derive(Clone, Debug, Default)]
pub struct SideTable {
    pub slots: JsMap<crate::render_char_materials::CharSlot, crate::render_char_materials::SideTableSlotsValue>,
    pub grime: crate::render_char_materials::SideTableGrime,
    pub cavity: f64,
    pub ridge: f64,
    pub stand_in: Option<crate::render_char_materials::SideTableStandIn>,
}

#[derive(Clone, Debug)]
pub enum SideTableSlotsValue {
    SlotMat(crate::render_char_materials::SlotMat),
    SlotAlias(crate::render_char_materials::SlotAlias),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SideTableGrime {
    pub color: V3,
    pub height: f64,
    pub band: f64,
    pub k: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SideTableStandIn {
    pub boot: Option<crate::render_char_materials::CharSlot>,
    pub cloth_plain: Option<crate::render_char_materials::CharSlot>,
    pub skin: Option<crate::render_char_materials::CharSlot>,
    pub cloth_uniform: Option<crate::render_char_materials::CharSlot>,
    pub webbing: Option<crate::render_char_materials::CharSlot>,
    pub helmet: Option<crate::render_char_materials::CharSlot>,
    pub metal_dark: Option<crate::render_char_materials::CharSlot>,
    pub rubber: Option<crate::render_char_materials::CharSlot>,
    pub lens: Option<crate::render_char_materials::CharSlot>,
    pub accent: Option<crate::render_char_materials::CharSlot>,
    pub eye: Option<crate::render_char_materials::CharSlot>,
}

#[derive(Clone, Debug, Default)]
pub struct RenderCharMaterialsModCHARMATSLOOK {
    pub civilian: Option<crate::render_char_materials::SideTable>,
    pub operator: Option<crate::render_char_materials::SideTable>,
    pub defender: Option<crate::render_char_materials::SideTable>,
    pub responder: Option<crate::render_char_materials::SideTable>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderCharMaterialsModOrderOfValue {
    pub ver: f64,
    pub bits: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ResolveSlot {
    pub name: crate::render_char_materials::CharSlot,
    pub m: crate::render_char_materials::SlotMat,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderCharMaterialsMod {
    /// (src/render/charMaterials.ts:159 cache)
    pub cache: JsMap<AnimGltfMeshDataId, crate::render_char_materials::CharSlotData>,
    /// (src/render/charMaterials.ts:256 CHAR_MATS)
    pub char_mats: JsMap<crate::anim_gltf::CastSide, crate::render_char_materials::SideTable>,
    /// (src/render/charMaterials.ts:345 CHAR_MATS_LOOK)
    pub char_mats_look: crate::render_char_materials::RenderCharMaterialsModCHARMATSLOOK,
    /// (src/render/charMaterials.ts:388 SNEAK_SUIT)
    pub sneak_suit: crate::render_char_materials::SideTable,
    /// (src/render/charMaterials.ts:425 orderOf)
    pub order_of: JsMap<crate::anim_gltf::CastSide, crate::render_char_materials::RenderCharMaterialsModOrderOfValue>,
    /// (src/render/charMaterials.ts:438 matsVersion)
    pub mats_version: f64,
}
impl RenderCharMaterialsMod {
    pub fn blank() -> Self {
        RenderCharMaterialsMod {
            cache: JsMap::new(),
            char_mats: JsMap::new(),
            char_mats_look: crate::render_char_materials::RenderCharMaterialsModCHARMATSLOOK::default(),
            sneak_suit: crate::render_char_materials::SideTable::default(),
            order_of: JsMap::new(),
            mats_version: 1.0,
        }
    }
}

/// (src/render/charMaterials.ts:24 CHAR_SLOTS: a table nothing writes)
pub fn char_slots_table() -> [String; 11] { [String::from("skin"), String::from("cloth_uniform"), String::from("cloth_plain"), String::from("webbing"), String::from("boot"), String::from("helmet"), String::from("metal_dark"), String::from("rubber"), String::from("lens"), String::from("accent"), String::from("eye")] }

/// (src/render/charMaterials.ts:26 CHAR_SLOT_COUNT)
pub const CHAR_SLOT_COUNT: f64 = 11.0;

/// (src/render/charMaterials.ts:28 CHAR_MAT_ROWS)
pub const CHAR_MAT_ROWS: f64 = 24.0;

/// (src/render/charMaterials.ts:29 CHAR_MAT_FLOATS)
pub const CHAR_MAT_FLOATS: f64 = 96.0;

/// (src/render/charMaterials.ts:30 SLOT_ID: a table nothing writes)
pub fn slot_id() -> JsMap<String, f64> { todo!() }

/// (src/render/charMaterials.ts:32 PROXIES)
pub const PROXIES: f64 = 15.0;

/// (src/render/charMaterials.ts:34 NO_MATERIAL)
pub const NO_MATERIAL: f64 = 255.0;

/// (src/render/charMaterials.ts:144 CAVITY_BROAD)
pub const CAVITY_BROAD: f64 = 6.0;

/// (src/render/charMaterials.ts:144 CAVITY_PCT)
pub const CAVITY_PCT: f64 = 0.9;

/// (src/render/charMaterials.ts:144 CAVITY_AT)
pub const CAVITY_AT: f64 = 0.6;

/// (src/render/charMaterials.ts:232 SUIT: a table nothing writes)
pub fn suit() -> V3 { [0.05, 0.06, 0.07] }

/// (src/render/charMaterials.ts:233 SKIN_A: a table nothing writes)
pub fn skin_a() -> Vec<V3> { alloc::vec![[0.42, 0.27, 0.19], [0.33, 0.2, 0.135], [0.25, 0.15, 0.1], [0.47, 0.32, 0.24]] }

/// (src/render/charMaterials.ts:242 SHIRTS: a table nothing writes)
pub fn shirts() -> Vec<V3> { alloc::vec![[0.3, 0.3, 0.29], [0.27, 0.22, 0.25], [0.13, 0.09, 0.06], [0.3, 0.33, 0.25], [0.31, 0.24, 0.14], [0.2, 0.26, 0.25], [0.37, 0.355, 0.3], [0.21, 0.125, 0.07], [0.34, 0.31, 0.21], [0.2, 0.22, 0.1], [0.15, 0.14, 0.13]] }

/// (src/render/charMaterials.ts:243 HATS: a table nothing writes)
pub fn hats() -> Vec<V3> { alloc::vec![[0.25, 0.28, 0.2], [0.09, 0.06, 0.04], [0.17, 0.18, 0.1], [0.28, 0.15, 0.09], [0.42, 0.38, 0.27], [0.15, 0.19, 0.19]] }

/// (src/render/charMaterials.ts:245 HAIR: a table nothing writes)
pub fn hair() -> Vec<V3> { alloc::vec![[0.022, 0.02, 0.019], [0.042, 0.028, 0.02], [0.075, 0.047, 0.03], [0.085, 0.083, 0.08]] }

/// (src/render/charMaterials.ts:248 NATURALS: a table nothing writes)
pub fn naturals() -> Vec<V3> { alloc::vec![[0.6, 0.54, 0.38], [0.63, 0.59, 0.48], [0.6, 0.57, 0.49], [0.6, 0.49, 0.3]] }

/// (src/render/charMaterials.ts:250 EYE_WHITE: a table nothing writes)
pub fn eye_white() -> crate::render_char_materials::SlotMat { crate::render_char_materials::SlotMat { albedo: Some([0.46, 0.445, 0.42]), src: None, k: None, mix: None, pick: None, rough: 0.25, spec: 1.0, metal: None, soak: 0.0, grime: 0.0, wear: 0.0 } }

/// (src/render/charMaterials.ts:251 SKIN_B: a table nothing writes)
pub fn skin_b() -> Vec<V3> { alloc::vec![[0.31, 0.19, 0.125], [0.36, 0.225, 0.15], [0.26, 0.155, 0.1], [0.4, 0.26, 0.18]] }

/// (src/render/charMaterials.ts:339 MULTICAM: a table nothing writes)
pub fn multicam() -> V3 { [0.25, 0.225, 0.15] }

/// (src/render/charMaterials.ts:340 SUITS: a table nothing writes)
pub fn suits() -> Vec<V3> { alloc::vec![[0.6, 0.57, 0.5], [0.25, 0.2, 0.14], [0.58, 0.56, 0.51], [0.3, 0.36, 0.42], [0.62, 0.58, 0.49], [0.22, 0.17, 0.12], [0.3, 0.3, 0.22], [0.27, 0.21, 0.155], [0.24, 0.27, 0.31], [0.64, 0.59, 0.49], [0.27, 0.28, 0.22]] }

/// (src/render/charMaterials.ts:341 WAISTCOATS: a table nothing writes)
pub fn waistcoats() -> Vec<V3> { alloc::vec![[0.085, 0.07, 0.055], [0.065, 0.07, 0.075], [0.115, 0.12, 0.085], [0.15, 0.11, 0.07], [0.075, 0.06, 0.05]] }

/// (src/render/charMaterials.ts:342 WOOL_CAPS: a table nothing writes)
pub fn wool_caps() -> Vec<V3> { alloc::vec![[0.2, 0.15, 0.1], [0.15, 0.115, 0.08], [0.235, 0.23, 0.215], [0.13, 0.11, 0.09], [0.105, 0.095, 0.085]] }

/// (src/render/charMaterials.ts:343 HEAD_CLOTH: a table nothing writes)
pub fn head_cloth() -> Vec<V3> { alloc::vec![[0.7, 0.68, 0.62], [0.14, 0.125, 0.11], [0.21, 0.21, 0.21], [0.72, 0.7, 0.67], [0.38, 0.36, 0.32]] }

/// (src/render/charMaterials.ts:344 WRAPS: a table nothing writes)
pub fn wraps() -> Vec<V3> { alloc::vec![[0.17, 0.135, 0.1], [0.33, 0.31, 0.28], [0.2, 0.18, 0.12], [0.62, 0.59, 0.52], [0.3, 0.24, 0.17], [0.22, 0.16, 0.1]] }

/// (src/render/charMaterials.ts:387 SNEAK: a table nothing writes)
pub fn sneak() -> V3 { [0.032, 0.034, 0.04] }

/// (src/render/charMaterials.ts:424 TINTS_AS_IS)
pub const TINTS_AS_IS: f64 = 2.0;

/// (src/render/charMaterials.ts:467 LOOK_SIDE: a table nothing writes)
pub fn look_side() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("operator"), 0.0); t.set(String::from("defender"), 1.0); t.set(String::from("responder"), 2.0); t.set(String::from("civilian"), 3.0); t } }
