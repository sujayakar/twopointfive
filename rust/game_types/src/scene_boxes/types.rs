//! The types of `scene_boxes`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct PlanBox {
    pub c: V3,
    pub h: V3,
    pub yaw: f64,
    pub flags: f64,
    pub rot: Option<Quat>,
    pub cut: Option<crate::scene_boxes::PlanBoxCut>,
    pub round: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlanBoxCut {
    pub length: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Box {
    pub c: V3,
    pub h: V3,
    pub yaw: f64,
    pub albedo: V3,
    pub emissive: V3,
    pub flags: f64,
    pub owner: f64,
    pub cut_height: f64,
    pub wet_k: Option<f64>,
    pub rot: Option<Quat>,
    pub dc: Option<V3>,
    pub dh: Option<V3>,
    pub name: Option<String>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
    pub aniso: Option<bool>,
    pub marks: Option<Vec<crate::scene_markings::MarkingDef>>,
    pub mark_gain: Option<f64>,
    pub fog_emit: Option<[f64; 3]>,
    pub fog_mark: Option<f64>,
    pub mkey: Option<f64>,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub cut_core: Option<V3>,
    pub cut_glass: Option<crate::scene_cut::CutGlass>,
    pub cut_round: Option<f64>,
    pub cut_arc: Option<f64>,
    pub cut_old: Option<bool>,
    pub cut_core_stuff: Option<crate::scene_stuff::BoxStuff>,
    pub round: Option<bool>,
    pub shape: Option<crate::scene_shapes::BoxShape>,
    pub shaped: Option<bool>,
    pub piece: Option<bool>,
    pub of: Option<f64>,
    pub dress: Option<bool>,
}

#[derive(Clone, Debug, ToBytes)]
pub struct BoxWorld {
    pub statics: Vec<BoxId>,
    pub dynamics: Vec<AnyBoxId>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub undrawn: Option<crate::scene_boxes::BoxWorldUndrawn>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub motion_ids: bool,
    pub max_boxes: f64,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub geo_f32: Vec<f32>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub geo_u32: Vec<u32>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub mat_data: Vec<f32>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub geo_buf: crate::Opaque,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub mat_buf: crate::Opaque,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub cell_start: Vec<u32>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub items: Vec<u32>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub cell_buf: crate::Opaque,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub item_buf: crate::Opaque,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub globals: Vec<f64>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub info_buf: crate::Opaque,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub info_data: Vec<u32>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub visible_sent: Vec<u32>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub capsule_data: Vec<f32>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub capsule_count: f64,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub char_count: f64,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub capsule_buf: crate::Opaque,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub rig: Option<crate::scene_boxes::BoxWorldRig>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub visible: Vec<u32>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub visible_buf: crate::Opaque,
    pub geom_serial: f64,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub cut_on: bool,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub cut_data: Vec<f32>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub cut_split: f64,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub visible_cut_traced: f64,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub cut_traced: f64,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub device: crate::Opaque,
    pub glass_open: Option<crate::scene_collision::PaneRule>,
}
impl BoxWorld {
    /// what a new one holds
    pub fn blank() -> Self {
        BoxWorld {
            statics: Vec::new(),
            dynamics: Vec::new(),
            undrawn: None,
            motion_ids: true,
            max_boxes: 0.0,
            geo_f32: Vec::new(),
            geo_u32: Vec::new(),
            mat_data: Vec::new(),
            geo_buf: crate::Opaque,
            mat_buf: crate::Opaque,
            cell_start: Vec::new(),
            items: Vec::new(),
            cell_buf: crate::Opaque,
            item_buf: crate::Opaque,
            globals: Vec::new(),
            info_buf: crate::Opaque,
            info_data: alloc::vec![0; 640],
            visible_sent: Vec::new(),
            capsule_data: alloc::vec![0.0; 1920],
            capsule_count: 0.0,
            char_count: 0.0,
            capsule_buf: crate::Opaque,
            rig: None,
            visible: Vec::new(),
            visible_buf: crate::Opaque,
            geom_serial: 0.0,
            cut_on: false,
            cut_data: alloc::vec![0.0; 0],
            cut_split: 0.0,
            visible_cut_traced: 0.0,
            cut_traced: 0.0,
            device: crate::Opaque,
            glass_open: None,
        }
    }
}
impl Default for BoxWorld { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxWorldUndrawn {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxWorldMkeyStats {
    pub matched: f64,
    pub fresh: f64,
    pub unkeyed: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxWorldGridStats {
    pub cells: f64,
    pub empty: f64,
    pub mean_skip: f64,
    pub max_per_cell: f64,
    pub slack: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxWorldGlassScratchItem {
    pub i: f64,
    pub d: f64,
}

#[derive(Clone, Debug, Default)]
pub struct BoxWorldRig {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
    pub k_pad: f64,
    pub by_owner: Option<JsMap<crate::Num, crate::scene_boxes::BoxWorldRigByOwnerValue>>,
}

#[derive(Clone, Debug, Default)]
pub struct BoxWorldRigByOwnerValue {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
    pub pad: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxWorldGridBuilds {
    pub built: f64,
    pub kept: f64,
}

#[derive(Clone, Debug, Default)]
pub struct BoxWorldMarkByKeyValue {
    pub slot: f64,
    pub n: f64,
    pub rows: f64,
    pub sec: V3,
    pub any_emit: bool,
    pub marks: Vec<crate::scene_markings::MarkingDef>,
    pub base: V3,
    pub h: V3,
}

#[derive(Clone, Debug, Default)]
pub struct BoxWorldMarkOfBoxValue {
    pub key: String,
    pub slot: f64,
    pub sec: V3,
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub marks: Vec<crate::scene_markings::MarkingDef>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxWorldMarkStatsInfo {
    pub lists: f64,
    pub records: f64,
    pub rows: f64,
    pub boxes: f64,
    pub dropped: f64,
    pub uploads: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxFlag {
    pub no_primary: f64,
    pub no_shadow: f64,
    pub cutaway: f64,
    pub dynamic: f64,
    pub emissive: f64,
    pub area_lit: f64,
    pub rot: f64,
    pub gloss: f64,
    pub sound_seal: f64,
    pub glass: f64,
    pub thin: f64,
    pub bar: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MKEY {
    pub door: f64,
    pub prop: f64,
    pub item: f64,
    pub rig: f64,
    pub device: f64,
    pub ring: f64,
    pub test: f64,
    pub debris: f64,
    pub look: f64,
    pub skin: f64,
}

#[derive(Clone, Debug, Default)]
pub struct MakeBoxP {
    pub c: V3,
    pub h: V3,
    pub yaw: Option<f64>,
    pub albedo: Option<V3>,
    pub emissive: Option<V3>,
    pub flags: Option<f64>,
    pub owner: Option<f64>,
    pub cut_height: Option<f64>,
    pub wet_k: Option<f64>,
    pub rot: Option<Quat>,
    pub dc: Option<V3>,
    pub dh: Option<V3>,
    pub name: Option<String>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
    pub aniso: Option<bool>,
    pub marks: Option<Vec<crate::scene_markings::MarkingDef>>,
    pub mark_gain: Option<f64>,
    pub fog_emit: Option<[f64; 3]>,
    pub fog_mark: Option<f64>,
    pub mkey: Option<f64>,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub cut_core: Option<V3>,
    pub cut_glass: Option<crate::scene_cut::CutGlass>,
    pub cut_round: Option<f64>,
    pub cut_arc: Option<f64>,
    pub cut_old: Option<bool>,
    pub cut_core_stuff: Option<crate::scene_stuff::BoxStuff>,
    pub round: Option<bool>,
    pub shape: Option<crate::scene_shapes::BoxShape>,
    pub shaped: Option<bool>,
    pub piece: Option<bool>,
    pub of: Option<f64>,
    pub dress: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WordOfB {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WordOf {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SolidGlassBoxB {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub flags: f64,
}

#[derive(Clone, Debug, Default)]
pub struct BoxWorldPackListEnt {
    pub slot: f64,
    pub n: f64,
    pub sec: V3,
    pub any_emit: bool,
    pub marks: Vec<crate::scene_markings::MarkingDef>,
    pub base: V3,
    pub h: V3,
}

#[derive(Clone, Debug)]
pub enum BoxWorldAddCharacterCapsulesAlbedo {
    List(V3),
    Array(Vec<V3>),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxWorldRaycast {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBox {
    pub t: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBoxRot {
    pub t: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBoxCut {
    pub t: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBoxRound {
    pub t: f64,
    pub n: V3,
}

/// (src/scene/boxes.ts:303 BOX_WORLD_MAX_CAST)
pub const BOX_WORLD_MAX_CAST: f64 = 16.0;

/// (src/scene/boxes.ts:306 BOX_WORLD_castCap)
pub const BOX_WORLD_castCap: f64 = 16.0;

/// (src/scene/boxes.ts:307 BOX_WORLD_CAPS_PER_CHAR)
pub const BOX_WORLD_CAPS_PER_CHAR: f64 = 15.0;

/// (src/scene/boxes.ts:307 BOX_WORLD_MAX_CAPSULES)
pub const BOX_WORLD_MAX_CAPSULES: f64 = 240.0;

/// (src/scene/boxes.ts:309 BOX_WORLD_INFO_BOUNDS)
pub const BOX_WORLD_INFO_BOUNDS: f64 = 24.0;

/// (src/scene/boxes.ts:309 BOX_WORLD_INFO_GLOBALS_Y)
pub const BOX_WORLD_INFO_GLOBALS_Y: f64 = 88.0;

/// (src/scene/boxes.ts:309 BOX_WORLD_INFO_GRID)
pub const BOX_WORLD_INFO_GRID: f64 = 120.0;

/// (src/scene/boxes.ts:309 BOX_WORLD_INFO_GLOBALS_B)
pub const BOX_WORLD_INFO_GLOBALS_B: f64 = 124.0;

/// (src/scene/boxes.ts:310 BOX_WORLD_GLASS_WORD)
pub const BOX_WORLD_GLASS_WORD: f64 = 252.0;

/// (src/scene/boxes.ts:347 BOX_WORLD_GLOBAL_PAD)
pub const BOX_WORLD_GLOBAL_PAD: f64 = 0.005;

/// (src/scene/boxes.ts:352 BOX_WORLD_ITEM_PAD_KEEP)
pub const BOX_WORLD_ITEM_PAD_KEEP: f64 = 0.005;

/// (src/scene/boxes.ts:362 BOX_WORLD_RB_QUANT)
pub const BOX_WORLD_RB_QUANT: f64 = 0.001;

/// (src/scene/boxes.ts:393 BOX_WORLD_SS)
pub const BOX_WORLD_SS: f64 = 24.0;

/// (src/scene/boxes.ts:422 BOX_WORLD_RAY_PAD)
pub const BOX_WORLD_RAY_PAD: f64 = 0.001;

/// (src/scene/boxes.ts:426 BOX_WORLD_RS)
pub const BOX_WORLD_RS: f64 = 12.0;

/// (src/scene/boxes.ts:671 BOX_WORLD_TIE_PLANE)
pub const BOX_WORLD_TIE_PLANE: f64 = 0.00025;

/// (src/scene/boxes.ts:673 BOX_WORLD_TIE_PATCH)
pub const BOX_WORLD_TIE_PATCH: f64 = 0.0005;

/// (src/scene/boxes.ts:13 BoxFlag: a table nothing writes)
pub fn box_flag() -> crate::scene_boxes::BoxFlag { crate::scene_boxes::BoxFlag { no_primary: 1.0, no_shadow: 2.0, cutaway: 4.0, dynamic: 8.0, emissive: 16.0, area_lit: 32.0, rot: 64.0, gloss: 128.0, sound_seal: 65536.0, glass: 131072.0, thin: 262144.0, bar: 524288.0 } }

/// (src/scene/boxes.ts:41 SHEET_MASK)
pub const SHEET_MASK: f64 = 131083.0;

/// (src/scene/boxes.ts:43 MAX_GLASS)
pub const MAX_GLASS: f64 = 32.0;

/// (src/scene/boxes.ts:176 MKEY: a table nothing writes)
pub fn mkey() -> crate::scene_boxes::MKEY { crate::scene_boxes::MKEY { door: 16777216.0, prop: 33554432.0, item: 50331648.0, rig: 67108864.0, device: 83886080.0, ring: 100663296.0, test: 117440512.0, debris: 134217728.0, look: 150994944.0, skin: 167772160.0 } }

/// (src/scene/boxes.ts:236 GEO_STRIDE)
pub const GEO_STRIDE: f64 = 8.0;

/// (src/scene/boxes.ts:242 GB_STRIDE)
pub const GB_STRIDE: f64 = 36.0;

/// (src/scene/boxes.ts:242 GB_ROWS)
pub const GB_ROWS: f64 = 1152.0;

/// (src/scene/boxes.ts:243 MAT_STRIDE)
pub const MAT_STRIDE: f64 = 16.0;
