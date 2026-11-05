//! The types of `scene_boxes`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Box, src/scene/boxes.ts:51)
#[derive(Clone, Debug)]
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
    pub was: Option<crate::scene_boxes::BoxWas>,
    pub shape: Option<crate::scene_shapes::BoxShape>,
    pub shaped: Option<bool>,
    pub piece: Option<bool>,
    pub of: Option<f64>,
    pub dress: Option<bool>,
}
impl Box {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Box {
            c: [0.0; 3],
            h: [0.0; 3],
            yaw: 0.0,
            albedo: [0.0; 3],
            emissive: [0.0; 3],
            flags: 0.0,
            owner: 0.0,
            cut_height: 0.0,
            wet_k: None,
            rot: None,
            dc: None,
            dh: None,
            name: None,
            rough: None,
            metal: None,
            stuff: None,
            does: None,
            aniso: None,
            marks: None,
            mark_gain: None,
            fog_emit: None,
            fog_mark: None,
            mkey: None,
            cut: None,
            cut_core: None,
            cut_glass: None,
            cut_round: None,
            cut_arc: None,
            cut_old: None,
            cut_core_stuff: None,
            round: None,
            was: None,
            shape: None,
            shaped: None,
            piece: None,
            of: None,
            dress: None,
        }
    }
}
impl Default for Box { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:140 Box.was, src/scene/boxes.ts:140)
#[derive(Clone, Debug)]
pub struct BoxWas {
    pub c: V3,
    pub h: V3,
    pub yaw: f64,
    pub flags: f64,
    pub marks: Option<Vec<crate::scene_markings::MarkingDef>>,
}
impl BoxWas {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWas {
            c: [0.0; 3],
            h: [0.0; 3],
            yaw: 0.0,
            flags: 0.0,
            marks: None,
        }
    }
}
impl Default for BoxWas { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BoxWorld, src/scene/boxes.ts:247)
#[derive(Clone, Debug)]
pub struct BoxWorld {
    pub statics: Vec<BoxId>,
    pub dynamics: Vec<BoxId>,
    pub hide_primary: Option<JsSet<BoxId>>,
    pub motion_ids: bool,
    pub mkey_prev: JsMap<crate::Num, f64>,
    pub mkey_cur: JsMap<crate::Num, f64>,
    pub mkey_stats: crate::scene_boxes::BoxWorldMkeyStats,
    pub all: Vec<BoxId>,
    pub max_boxes: f64,
    pub geo_data: crate::Opaque,
    pub geo_f32: Vec<f32>,
    pub geo_u32: Vec<u32>,
    pub mat_data: Vec<f32>,
    pub geo_buf: crate::Opaque,
    pub mat_buf: crate::Opaque,
    pub cell_start: Vec<u32>,
    pub items: Vec<u32>,
    pub item_count: f64,
    pub cell_buf: crate::Opaque,
    pub item_buf: crate::Opaque,
    pub globals: Vec<f64>,
    pub grid_stats: crate::scene_boxes::BoxWorldGridStats,
    pub grid_ranges: Vec<i32>,
    pub grid_fill: Vec<u32>,
    pub grid_mask: Vec<u32>,
    pub grid_dist: Vec<u8>,
    pub info_buf: crate::Opaque,
    pub info_data: Vec<u32>,
    pub glass_enabled: bool,
    pub glass_focus: V3,
    pub glass_list: Vec<f64>,
    pub glass_statics: Vec<f64>,
    pub glass_scratch: Vec<crate::scene_boxes::BoxWorldGlassScratchItem>,
    pub glass_statics_n: f64,
    pub glass_cell_fill: Option<crate::FnField>,
    pub gb_data: Vec<f32>,
    pub gb_live: bool,
    pub gb_sent: Vec<u32>,
    pub ground_on: bool,
    pub rows_stat_n: f64,
    pub visible_sent: Vec<u32>,
    pub capsule_data: Vec<f32>,
    pub capsule_count: f64,
    pub char_count: f64,
    pub capsule_buf: crate::Opaque,
    pub rig: Option<crate::scene_boxes::BoxWorldRig>,
    pub visible: Vec<u32>,
    pub visible_count: f64,
    pub visible_buf: crate::Opaque,
    pub draw_top: Vec<f32>,
    pub draw_rank: Vec<u32>,
    pub draw_slot: Vec<u32>,
    pub draw_rank_n: f64,
    pub statics_dirty: bool,
    pub geom_serial: f64,
    pub stat_shadow: Vec<f64>,
    pub stat_shadow_marks: Vec<Option<Vec<crate::scene_markings::MarkingDef>>>,
    pub stat_marked: Vec<u8>,
    pub stat_shadow_n: f64,
    pub pack_epoch: f64,
    pub stat_epoch: f64,
    pub stat_marks_on: bool,
    pub grid_enabled: bool,
    pub grid_stat_ranges: Vec<i32>,
    pub grid_stat_bits: Vec<u32>,
    pub grid_stat_slack: f64,
    pub grid_stat_n: f64,
    pub grid_stat_globals: Vec<f64>,
    pub grid_dyn_bits: Vec<u32>,
    pub grid_stat_counts: Vec<u32>,
    pub grid_stat_mask: Vec<u32>,
    pub grid_last: Vec<i32>,
    pub grid_last_globals: Vec<f64>,
    pub grid_live_n: f64,
    pub grid_builds: crate::scene_boxes::BoxWorldGridBuilds,
    pub rg_start: Vec<u32>,
    pub rg_items: Vec<u32>,
    pub rg_globals: Vec<f64>,
    pub rg_n: f64,
    pub rg_shape: Vec<f64>,
    pub rg_fill: Vec<u32>,
    pub rg_ranges: Vec<i32>,
    pub rg_stamp: Vec<u32>,
    pub rg_ray: f64,
    pub rc_best: f64,
    pub rc_bi: f64,
    pub rc_bn: V3,
    pub ray_grid_stats: crate::scene_boxes::BoxWorldRayGridStats,
    pub markings_enabled: bool,
    pub marking_texel: f64,
    pub mark_data: Vec<f32>,
    pub mark_rows: f64,
    pub mark_dirty: bool,
    pub mark_by_key: JsMap<String, crate::scene_boxes::BoxWorldMarkByKeyValue>,
    pub mark_of_box: JsMap<BoxId, crate::scene_boxes::BoxWorldMarkOfBoxValue>,
    pub mark_stats_info: crate::scene_boxes::BoxWorldMarkStatsInfo,
    pub cut_on: bool,
    pub round_on: bool,
    pub cut_data: Vec<f32>,
    pub cut_lo: f64,
    pub cut_hi: f64,
    pub cut_lo_d: f64,
    pub cut_hi_d: f64,
    pub cut_split: f64,
    pub visible_cut_count: f64,
    pub visible_cut_traced: f64,
    pub cut_traced: f64,
    pub cut_visible: Vec<u32>,
    pub stat_shadow_cut: Vec<Option<Vec<crate::scene_cut::CutPlane>>>,
    pub device: crate::Opaque,
    pub draw_ties: f64,
    pub draw_ranked: f64,
    pub pack_behind: bool,
    pub stuff_of: Option<crate::FnField>,
    pub core_stuff_of: Option<crate::FnField>,
    pub glass_open: Option<crate::FnField>,
}
impl BoxWorld {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        BoxWorld {
            statics: Vec::new(),
            dynamics: Vec::new(),
            hide_primary: None,
            motion_ids: true,
            mkey_prev: JsMap::new(),
            mkey_cur: JsMap::new(),
            mkey_stats: crate::scene_boxes::BoxWorldMkeyStats { matched: 0.0, fresh: 0.0, unkeyed: 0.0 },
            all: Vec::new(),
            max_boxes: 0.0,
            geo_data: crate::Opaque,
            geo_f32: Vec::new(),
            geo_u32: Vec::new(),
            mat_data: Vec::new(),
            geo_buf: crate::Opaque,
            mat_buf: crate::Opaque,
            cell_start: Vec::new(),
            items: Vec::new(),
            item_count: 0.0,
            cell_buf: crate::Opaque,
            item_buf: crate::Opaque,
            globals: Vec::new(),
            grid_stats: crate::scene_boxes::BoxWorldGridStats { cells: 1120.0, empty: 0.0, mean_skip: 0.0, max_per_cell: 0.0, slack: 0.0 },
            grid_ranges: alloc::vec![0; 0],
            grid_fill: alloc::vec![0; 1120],
            grid_mask: alloc::vec![0; 1120],
            grid_dist: alloc::vec![0; 1120],
            info_buf: crate::Opaque,
            info_data: alloc::vec![0; 640],
            glass_enabled: true,
            glass_focus: [20.0, 0.0, 14.0],
            glass_list: Vec::new(),
            glass_statics: Vec::new(),
            glass_scratch: Vec::new(),
            glass_statics_n: -1.0,
            glass_cell_fill: None,
            gb_data: alloc::vec![0.0; 4608],
            gb_live: false,
            gb_sent: alloc::vec![0; 4608],
            ground_on: false,
            rows_stat_n: -1.0,
            visible_sent: Vec::new(),
            capsule_data: alloc::vec![0.0; 1920],
            capsule_count: 0.0,
            char_count: 0.0,
            capsule_buf: crate::Opaque,
            rig: None,
            visible: Vec::new(),
            visible_count: 0.0,
            visible_buf: crate::Opaque,
            draw_top: alloc::vec![0.0; 0],
            draw_rank: alloc::vec![0; 0],
            draw_slot: alloc::vec![0; 0],
            draw_rank_n: -1.0,
            statics_dirty: true,
            geom_serial: 0.0,
            stat_shadow: alloc::vec![0.0; 0],
            stat_shadow_marks: Vec::new(),
            stat_marked: alloc::vec![0; 0],
            stat_shadow_n: 0.0,
            pack_epoch: 0.0,
            stat_epoch: -1.0,
            stat_marks_on: true,
            grid_enabled: true,
            grid_stat_ranges: alloc::vec![0; 0],
            grid_stat_bits: alloc::vec![0; 0],
            grid_stat_slack: 0.0,
            grid_stat_n: -1.0,
            grid_stat_globals: Vec::new(),
            grid_dyn_bits: alloc::vec![0; 1],
            grid_stat_counts: alloc::vec![0; 1120],
            grid_stat_mask: alloc::vec![0; 1120],
            grid_last: alloc::vec![0; 0],
            grid_last_globals: Vec::new(),
            grid_live_n: -1.0,
            grid_builds: crate::scene_boxes::BoxWorldGridBuilds { built: 0.0, kept: 0.0 },
            rg_start: alloc::vec![0; 1121],
            rg_items: alloc::vec![0; 4096],
            rg_globals: Vec::new(),
            rg_n: -1.0,
            rg_shape: alloc::vec![0.0; 0],
            rg_fill: alloc::vec![0; 1120],
            rg_ranges: alloc::vec![0; 0],
            rg_stamp: alloc::vec![0; 0],
            rg_ray: 0.0,
            rc_best: 0.0,
            rc_bi: -1.0,
            rc_bn: [0.0, 1.0, 0.0],
            ray_grid_stats: crate::scene_boxes::BoxWorldRayGridStats { statics: 0.0, globals: 0.0, items: 0.0 },
            markings_enabled: true,
            marking_texel: 0.025,
            mark_data: alloc::vec![0.0; 24576],
            mark_rows: 0.0,
            mark_dirty: false,
            mark_by_key: JsMap::new(),
            mark_of_box: JsMap::new(),
            mark_stats_info: crate::scene_boxes::BoxWorldMarkStatsInfo { lists: 0.0, records: 0.0, rows: 0.0, boxes: 0.0, dropped: 0.0, uploads: 0.0 },
            cut_on: false,
            round_on: false,
            cut_data: alloc::vec![0.0; 0],
            cut_lo: 1073741824.0,
            cut_hi: -1.0,
            cut_lo_d: 1073741824.0,
            cut_hi_d: -1.0,
            cut_split: 0.0,
            visible_cut_count: 0.0,
            visible_cut_traced: 0.0,
            cut_traced: 0.0,
            cut_visible: alloc::vec![0; 0],
            stat_shadow_cut: Vec::new(),
            device: crate::Opaque,
            draw_ties: 0.0,
            draw_ranked: 0.0,
            pack_behind: false,
            stuff_of: None,
            core_stuff_of: None,
            glass_open: None,
        }
    }
}
impl Default for BoxWorld { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/scene/boxes.ts:259 BoxWorld.mkeyStats, src/scene/boxes.ts:259)
#[derive(Clone, Copy, Debug)]
pub struct BoxWorldMkeyStats {
    pub matched: f64,
    pub fresh: f64,
    pub unkeyed: f64,
}
impl BoxWorldMkeyStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldMkeyStats {
            matched: 0.0,
            fresh: 0.0,
            unkeyed: 0.0,
        }
    }
}
impl Default for BoxWorldMkeyStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:275 BoxWorld.gridStats, src/scene/boxes.ts:275)
#[derive(Clone, Copy, Debug)]
pub struct BoxWorldGridStats {
    pub cells: f64,
    pub empty: f64,
    pub mean_skip: f64,
    pub max_per_cell: f64,
    pub slack: f64,
}
impl BoxWorldGridStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldGridStats {
            cells: 0.0,
            empty: 0.0,
            mean_skip: 0.0,
            max_per_cell: 0.0,
            slack: 0.0,
        }
    }
}
impl Default for BoxWorldGridStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:301 BoxWorld.glassScratch, src/scene/boxes.ts:301)
#[derive(Clone, Copy, Debug)]
pub struct BoxWorldGlassScratchItem {
    pub i: f64,
    pub d: f64,
}
impl BoxWorldGlassScratchItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldGlassScratchItem {
            i: 0.0,
            d: 0.0,
        }
    }
}
impl Default for BoxWorldGlassScratchItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:341 BoxWorld.rig, src/scene/boxes.ts:341)
#[derive(Clone, Debug)]
pub struct BoxWorldRig {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
    pub k_pad: f64,
    pub by_owner: Option<JsMap<crate::Num, crate::scene_boxes::BoxWorldRigByOwnerValue>>,
}
impl BoxWorldRig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldRig {
            r0: Vec::new(),
            r1: Vec::new(),
            k_pad: 0.0,
            by_owner: None,
        }
    }
}
impl Default for BoxWorldRig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:341 BoxWorldRig.byOwner, src/scene/boxes.ts:341)
#[derive(Clone, Debug)]
pub struct BoxWorldRigByOwnerValue {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
    pub pad: Option<f64>,
}
impl BoxWorldRigByOwnerValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldRigByOwnerValue {
            r0: Vec::new(),
            r1: Vec::new(),
            pad: None,
        }
    }
}
impl Default for BoxWorldRigByOwnerValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:384 BoxWorld.gridBuilds, src/scene/boxes.ts:384)
#[derive(Clone, Copy, Debug)]
pub struct BoxWorldGridBuilds {
    pub built: f64,
    pub kept: f64,
}
impl BoxWorldGridBuilds {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldGridBuilds {
            built: 0.0,
            kept: 0.0,
        }
    }
}
impl Default for BoxWorldGridBuilds { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:412 BoxWorld.rayGridStats, src/scene/boxes.ts:412)
#[derive(Clone, Copy, Debug)]
pub struct BoxWorldRayGridStats {
    pub statics: f64,
    pub globals: f64,
    pub items: f64,
}
impl BoxWorldRayGridStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldRayGridStats {
            statics: 0.0,
            globals: 0.0,
            items: 0.0,
        }
    }
}
impl Default for BoxWorldRayGridStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:422 BoxWorld.markByKey, src/scene/boxes.ts:422)
#[derive(Clone, Debug)]
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
impl BoxWorldMarkByKeyValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldMarkByKeyValue {
            slot: 0.0,
            n: 0.0,
            rows: 0.0,
            sec: [0.0; 3],
            any_emit: false,
            marks: Vec::new(),
            base: [0.0; 3],
            h: [0.0; 3],
        }
    }
}
impl Default for BoxWorldMarkByKeyValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:423 BoxWorld.markOfBox, src/scene/boxes.ts:423)
#[derive(Clone, Debug)]
pub struct BoxWorldMarkOfBoxValue {
    pub key: String,
    pub slot: f64,
    pub sec: V3,
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub marks: Vec<crate::scene_markings::MarkingDef>,
}
impl BoxWorldMarkOfBoxValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldMarkOfBoxValue {
            key: String::new(),
            slot: 0.0,
            sec: [0.0; 3],
            a0: 0.0,
            a1: 0.0,
            a2: 0.0,
            marks: Vec::new(),
        }
    }
}
impl Default for BoxWorldMarkOfBoxValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:424 BoxWorld.markStatsInfo, src/scene/boxes.ts:424)
#[derive(Clone, Copy, Debug)]
pub struct BoxWorldMarkStatsInfo {
    pub lists: f64,
    pub records: f64,
    pub rows: f64,
    pub boxes: f64,
    pub dropped: f64,
    pub uploads: f64,
}
impl BoxWorldMarkStatsInfo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldMarkStatsInfo {
            lists: 0.0,
            records: 0.0,
            rows: 0.0,
            boxes: 0.0,
            dropped: 0.0,
            uploads: 0.0,
        }
    }
}
impl Default for BoxWorldMarkStatsInfo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:13 BoxFlag, src/scene/boxes.ts:13)
#[derive(Clone, Copy, Debug)]
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
impl BoxFlag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxFlag {
            no_primary: 0.0,
            no_shadow: 0.0,
            cutaway: 0.0,
            dynamic: 0.0,
            emissive: 0.0,
            area_lit: 0.0,
            rot: 0.0,
            gloss: 0.0,
            sound_seal: 0.0,
            glass: 0.0,
            thin: 0.0,
            bar: 0.0,
        }
    }
}
impl Default for BoxFlag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:162 MKEY, src/scene/boxes.ts:162)
#[derive(Clone, Copy, Debug)]
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
}
impl MKEY {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MKEY {
            door: 0.0,
            prop: 0.0,
            item: 0.0,
            rig: 0.0,
            device: 0.0,
            ring: 0.0,
            test: 0.0,
            debris: 0.0,
            look: 0.0,
        }
    }
}
impl Default for MKEY { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:182 makeBox(p), src/scene/boxes.ts:182 makeBox(p))
#[derive(Clone, Debug)]
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
    pub was: Option<crate::scene_boxes::BoxWas>,
    pub shape: Option<crate::scene_shapes::BoxShape>,
    pub shaped: Option<bool>,
    pub piece: Option<bool>,
    pub of: Option<f64>,
    pub dress: Option<bool>,
}
impl MakeBoxP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MakeBoxP {
            c: [0.0; 3],
            h: [0.0; 3],
            yaw: None,
            albedo: None,
            emissive: None,
            flags: None,
            owner: None,
            cut_height: None,
            wet_k: None,
            rot: None,
            dc: None,
            dh: None,
            name: None,
            rough: None,
            metal: None,
            stuff: None,
            does: None,
            aniso: None,
            marks: None,
            mark_gain: None,
            fog_emit: None,
            fog_mark: None,
            mkey: None,
            cut: None,
            cut_core: None,
            cut_glass: None,
            cut_round: None,
            cut_arc: None,
            cut_old: None,
            cut_core_stuff: None,
            round: None,
            was: None,
            shape: None,
            shaped: None,
            piece: None,
            of: None,
            dress: None,
        }
    }
}
impl Default for MakeBoxP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:198 wordOf(b), src/scene/boxes.ts:198)
#[derive(Clone, Copy, Debug)]
pub struct WordOfB {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}
impl WordOfB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WordOfB {
            stuff: None,
            does: None,
        }
    }
}
impl Default for WordOfB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:198 wordOf ->, src/scene/boxes.ts:198)
#[derive(Clone, Copy, Debug)]
pub struct WordOf {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}
impl WordOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WordOf {
            stuff: None,
            does: None,
        }
    }
}
impl Default for WordOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:205 solidGlassBox(b), src/scene/boxes.ts:205)
#[derive(Clone, Copy, Debug)]
pub struct SolidGlassBoxB {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub flags: f64,
}
impl SolidGlassBoxB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SolidGlassBoxB {
            stuff: None,
            flags: 0.0,
        }
    }
}
impl Default for SolidGlassBoxB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:403 rayAside(own), src/scene/boxes.ts:403)
#[derive(Clone, Copy, Debug)]
pub struct BoxWorldRayAsideOwn {
    pub w: Option<crate::scene_boxes::BoxWorldRayAsideOwnW>,
}
impl BoxWorldRayAsideOwn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldRayAsideOwn {
            w: None,
        }
    }
}
impl Default for BoxWorldRayAsideOwn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:403 BoxWorldRayAsideOwn.w, src/scene/boxes.ts:403 BoxWorldRayAsideOwn.w)
#[derive(Clone, Copy, Debug)]
pub struct BoxWorldRayAsideOwnW {
}
impl BoxWorldRayAsideOwnW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldRayAsideOwnW {
        }
    }
}
impl Default for BoxWorldRayAsideOwnW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:501 packList(ent), src/scene/boxes.ts:501)
#[derive(Clone, Debug)]
pub struct BoxWorldPackListEnt {
    pub slot: f64,
    pub n: f64,
    pub sec: V3,
    pub any_emit: bool,
    pub marks: Vec<crate::scene_markings::MarkingDef>,
    pub base: V3,
    pub h: V3,
}
impl BoxWorldPackListEnt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldPackListEnt {
            slot: 0.0,
            n: 0.0,
            sec: [0.0; 3],
            any_emit: false,
            marks: Vec::new(),
            base: [0.0; 3],
            h: [0.0; 3],
        }
    }
}
impl Default for BoxWorldPackListEnt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum BoxWorldAddCharacterCapsulesAlbedo {
    List(V3),
    Array(Vec<V3>),
}

/// (TypeScript: inline at src/scene/boxes.ts:1029 raycast ->, src/scene/boxes.ts:1029)
#[derive(Clone, Copy, Debug)]
pub struct BoxWorldRaycast {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}
impl BoxWorldRaycast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxWorldRaycast {
            t: 0.0,
            index: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for BoxWorldRaycast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:1107 rayBox ->, src/scene/boxes.ts:1107)
#[derive(Clone, Copy, Debug)]
pub struct RayBox {
    pub t: f64,
    pub n: V3,
}
impl RayBox {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RayBox {
            t: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for RayBox { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:1133 rayBoxRot ->, src/scene/boxes.ts:1133)
#[derive(Clone, Copy, Debug)]
pub struct RayBoxRot {
    pub t: f64,
    pub n: V3,
}
impl RayBoxRot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RayBoxRot {
            t: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for RayBoxRot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:1154 rayBoxCut ->, src/scene/boxes.ts:1154)
#[derive(Clone, Copy, Debug)]
pub struct RayBoxCut {
    pub t: f64,
    pub n: V3,
}
impl RayBoxCut {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RayBoxCut {
            t: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for RayBoxCut { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxes.ts:1184 rayBoxRound ->, src/scene/boxes.ts:1184)
#[derive(Clone, Copy, Debug)]
pub struct RayBoxRound {
    pub t: f64,
    pub n: V3,
}
impl RayBoxRound {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RayBoxRound {
            t: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for RayBoxRound { fn default() -> Self { Self::zeroed() } }

/// (src/scene/boxes.ts:13 BoxFlag: a table nothing writes)
pub fn box_flag() -> crate::scene_boxes::BoxFlag { crate::scene_boxes::BoxFlag { no_primary: 1.0, no_shadow: 2.0, cutaway: 4.0, dynamic: 8.0, emissive: 16.0, area_lit: 32.0, rot: 64.0, gloss: 128.0, sound_seal: 65536.0, glass: 131072.0, thin: 262144.0, bar: 524288.0 } }

/// (src/scene/boxes.ts:41 SHEET_MASK)
pub const SHEET_MASK: f64 = 131083.0;

/// (src/scene/boxes.ts:43 MAX_GLASS)
pub const MAX_GLASS: f64 = 32.0;

/// (src/scene/boxes.ts:162 MKEY: a table nothing writes)
pub fn mkey() -> crate::scene_boxes::MKEY { crate::scene_boxes::MKEY { door: 16777216.0, prop: 33554432.0, item: 50331648.0, rig: 67108864.0, device: 83886080.0, ring: 100663296.0, test: 117440512.0, debris: 134217728.0, look: 150994944.0 } }

/// (src/scene/boxes.ts:222 GEO_STRIDE)
pub const GEO_STRIDE: f64 = 8.0;

/// (src/scene/boxes.ts:228 GB_STRIDE)
pub const GB_STRIDE: f64 = 36.0;

/// (src/scene/boxes.ts:228 GB_ROWS)
pub const GB_ROWS: f64 = 1152.0;

/// (src/scene/boxes.ts:229 MAT_STRIDE)
pub const MAT_STRIDE: f64 = 16.0;
