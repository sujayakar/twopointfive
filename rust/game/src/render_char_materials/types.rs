//! The types of `render_char_materials`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CharSlot {
    ClothPlain,
    Skin,
    ClothUniform,
    Webbing,
    Boot,
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
            CharSlot::ClothPlain => "cloth_plain",
            CharSlot::Skin => "skin",
            CharSlot::ClothUniform => "cloth_uniform",
            CharSlot::Webbing => "webbing",
            CharSlot::Boot => "boot",
            CharSlot::Helmet => "helmet",
            CharSlot::MetalDark => "metal_dark",
            CharSlot::Rubber => "rubber",
            CharSlot::Lens => "lens",
            CharSlot::Accent => "accent",
            CharSlot::Eye => "eye",
        }
    }
}

/// (TypeScript: CharSlotData, src/render/charMaterials.ts:45)
#[derive(Clone, Debug)]
pub struct CharSlotData {
    pub slot: Vec<u8>,
    pub cavity: Vec<f32>,
    pub authored: bool,
}
impl CharSlotData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CharSlotData {
            slot: Vec::new(),
            cavity: Vec::new(),
            authored: false,
        }
    }
}
impl Default for CharSlotData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SlotWeights, src/render/charMaterials.ts:61)
#[derive(Clone, Debug)]
pub struct SlotWeights {
    pub area: Vec<f32>,
    pub cap_weights: Vec<f32>,
}
impl SlotWeights {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotWeights {
            area: Vec::new(),
            cap_weights: Vec::new(),
        }
    }
}
impl Default for SlotWeights { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SlotMat, src/render/charMaterials.ts:218)
#[derive(Clone, Debug)]
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
impl SlotMat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotMat {
            albedo: None,
            src: None,
            k: None,
            mix: None,
            pick: None,
            rough: 0.0,
            spec: 0.0,
            metal: None,
            soak: 0.0,
            grime: 0.0,
            wear: 0.0,
        }
    }
}
impl Default for SlotMat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: SlotAlias, src/render/charMaterials.ts:219)
#[derive(Clone, Copy, Debug)]
pub struct SlotAlias {
    pub as_: crate::render_char_materials::CharSlot,
}
impl SlotAlias {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotAlias {
            as_: crate::render_char_materials::CharSlot::ClothPlain,
        }
    }
}
impl Default for SlotAlias { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SideTable, src/render/charMaterials.ts:220)
#[derive(Clone, Debug)]
pub struct SideTable {
    pub slots: JsMap<crate::render_char_materials::CharSlot, crate::render_char_materials::SideTableSlotsValue>,
    pub grime: crate::render_char_materials::SideTableGrime,
    pub cavity: f64,
    pub ridge: f64,
    pub stand_in: Option<crate::render_char_materials::SideTableStandIn>,
}
impl SideTable {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideTable {
            slots: JsMap::new(),
            grime: crate::render_char_materials::SideTableGrime::zeroed(),
            cavity: 0.0,
            ridge: 0.0,
            stand_in: None,
        }
    }
}
impl Default for SideTable { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SideTableSlotsValue {
    SlotMat(crate::render_char_materials::SlotMat),
    SlotAlias(crate::render_char_materials::SlotAlias),
}

/// (TypeScript: inline at src/render/charMaterials.ts:223 SideTable.grime, src/render/charMaterials.ts:223)
#[derive(Clone, Copy, Debug)]
pub struct SideTableGrime {
    pub color: V3,
    pub height: f64,
    pub band: f64,
    pub k: f64,
}
impl SideTableGrime {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideTableGrime {
            color: [0.0; 3],
            height: 0.0,
            band: 0.0,
            k: 0.0,
        }
    }
}
impl Default for SideTableGrime { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/charMaterials.ts:228 SideTable.standIn, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct SideTableStandIn {
    pub cloth_plain: Option<crate::render_char_materials::CharSlot>,
    pub skin: Option<crate::render_char_materials::CharSlot>,
    pub cloth_uniform: Option<crate::render_char_materials::CharSlot>,
    pub webbing: Option<crate::render_char_materials::CharSlot>,
    pub boot: Option<crate::render_char_materials::CharSlot>,
    pub helmet: Option<crate::render_char_materials::CharSlot>,
    pub metal_dark: Option<crate::render_char_materials::CharSlot>,
    pub rubber: Option<crate::render_char_materials::CharSlot>,
    pub lens: Option<crate::render_char_materials::CharSlot>,
    pub accent: Option<crate::render_char_materials::CharSlot>,
    pub eye: Option<crate::render_char_materials::CharSlot>,
}
impl SideTableStandIn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideTableStandIn {
            cloth_plain: None,
            skin: None,
            cloth_uniform: None,
            webbing: None,
            boot: None,
            helmet: None,
            metal_dark: None,
            rubber: None,
            lens: None,
            accent: None,
            eye: None,
        }
    }
}
impl Default for SideTableStandIn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/charMaterials.ts:345 CHAR_MATS_LOOK, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct RenderCharMaterialsModCHARMATSLOOK {
    pub civilian: Option<crate::render_char_materials::SideTable>,
    pub operator: Option<crate::render_char_materials::SideTable>,
    pub defender: Option<crate::render_char_materials::SideTable>,
    pub responder: Option<crate::render_char_materials::SideTable>,
}
impl RenderCharMaterialsModCHARMATSLOOK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderCharMaterialsModCHARMATSLOOK {
            civilian: None,
            operator: None,
            defender: None,
            responder: None,
        }
    }
}
impl Default for RenderCharMaterialsModCHARMATSLOOK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/charMaterials.ts:425 orderOf, src/render/charMaterials.ts:425)
#[derive(Clone, Copy, Debug)]
pub struct RenderCharMaterialsModOrderOfValue {
    pub ver: f64,
    pub bits: f64,
}
impl RenderCharMaterialsModOrderOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderCharMaterialsModOrderOfValue {
            ver: 0.0,
            bits: 0.0,
        }
    }
}
impl Default for RenderCharMaterialsModOrderOfValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/charMaterials.ts:466 charLookSides, src/render/charMaterials.ts:466)
#[derive(Clone, Copy, Debug)]
pub struct RenderCharMaterialsModCharLookSides {
    pub on: bool,
}
impl RenderCharMaterialsModCharLookSides {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderCharMaterialsModCharLookSides {
            on: false,
        }
    }
}
impl Default for RenderCharMaterialsModCharLookSides { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/charMaterials.ts:411 resolveSlot ->, src/render/charMaterials.ts:411)
#[derive(Clone, Debug)]
pub struct ResolveSlot {
    pub name: crate::render_char_materials::CharSlot,
    pub m: crate::render_char_materials::SlotMat,
}
impl ResolveSlot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResolveSlot {
            name: crate::render_char_materials::CharSlot::ClothPlain,
            m: crate::render_char_materials::SlotMat::zeroed(),
        }
    }
}
impl Default for ResolveSlot { fn default() -> Self { Self::zeroed() } }

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
    /// (src/render/charMaterials.ts:466 charLookSides)
    pub char_look_sides: crate::render_char_materials::RenderCharMaterialsModCharLookSides,
}
impl RenderCharMaterialsMod {
    pub fn blank() -> Self {
        RenderCharMaterialsMod {
            cache: JsMap::new(),
            char_mats: JsMap::new(),
            char_mats_look: crate::render_char_materials::RenderCharMaterialsModCHARMATSLOOK::zeroed(),
            sneak_suit: crate::render_char_materials::SideTable::zeroed(),
            order_of: JsMap::new(),
            mats_version: 1.0,
            char_look_sides: crate::render_char_materials::RenderCharMaterialsModCharLookSides { on: false },
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
