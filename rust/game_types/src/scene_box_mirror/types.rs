//! The types of `scene_box_mirror`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct PaneSite {
    pub u: f64,
    pub v: f64,
    pub rot: f64,
    pub n: f64,
    pub seed: f64,
    pub rho: Vec<f64>,
    pub rz: f64,
    pub merge: f64,
}

#[derive(Clone, Debug, Default)]
pub struct PaneDice {
    pub u: f64,
    pub v: f64,
    pub cols: f64,
    pub rows: f64,
    pub seed: f64,
    pub hu: f64,
    pub hv: f64,
    pub next: f64,
    pub out: Vec<u8>,
    pub clumps: Vec<crate::scene_box_mirror::PaneDiceClumpsItem>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PaneDiceClumpsItem {
    pub k: f64,
    pub key: f64,
}

#[derive(Clone, Debug, Default)]
pub struct PaneRow {
    pub bi: f64,
    pub gone: bool,
    pub run: bool,
    pub sites: Vec<crate::scene_box_mirror::PaneSite>,
    pub open: Vec<u8>,
    pub pending: Vec<crate::scene_box_mirror::PaneRowPendingItem>,
    pub dice: Option<crate::scene_box_mirror::PaneDice>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PaneRowPendingItem {
    pub fid: f64,
}

#[derive(Clone, Debug, Default)]
pub struct PaneView {
    pub kind: f64,
    pub bi: Vec<f64>,
    pub at: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct PaneSource {
    pub view: Option<crate::scene_box_mirror::PaneView>,
    pub home: Option<crate::Opaque>,
    pub sheets: Option<Vec<f64>>,
}

#[derive(Clone, Debug)]
pub struct BoxMirror {
    pub x: crate::sim_sim::SimExports,
    pub slot: f64,
    pub live: bool,
    pub lines: bool,
    pub rays: bool,
    pub circles: bool,
    pub owes: bool,
    pub glass: Option<crate::scene_box_mirror::PaneSource>,
    pub sheets_in: Option<Vec<f64>>,
    pub block: Option<crate::Opaque>,
    pub owed: f64,
    pub glass_busy: f64,
    pub glass_angles: bool,
    pub view_lengths: bool,
    pub glass_n: f64,
    pub glass_bis: Vec<f64>,
    pub row_lines: bool,
    pub row_rays: bool,
    pub row_circles: bool,
    pub walk: Option<crate::FnField>,
    pub list_cap: f64,
    pub scratch_len: f64,
    pub lamp_cap: f64,
    pub wind_cap: f64,
    pub nav_cap: f64,
    pub lamp_set: f64,
    pub lamp_n: f64,
    pub rule_set: f64,
    pub rule_panes: f64,
    pub copy: f64,
    pub grid_over: bool,
    pub cells_wrote: bool,
    pub grid_of: Option<crate::scene_box_mirror::BoxMirrorGridOf>,
    pub used: f64,
    pub caps: crate::scene_box_mirror::BoxMirrorCaps,
    pub things_at: f64,
    pub things_ok: bool,
    pub things_all: bool,
    pub snow_len: f64,
    pub boxes: f64,
    pub n_tilted: f64,
    pub epoch: f64,
    pub put_backs: f64,
    pub cut_of: Vec<i32>,
    pub cut_free: Vec<f64>,
    pub cut_next: f64,
    pub own: Option<Vec<BoxId>>,
    pub own_len: f64,
    pub own_at: f64,
    pub played: bool,
    pub of: Option<String>,
    /// the module's world as this game wrote it last (world.rs `stood`): a number of this process's, on no wire and in no word of the state
    pub stood: u32,
}
impl BoxMirror {
    /// what a new one holds
    pub fn blank() -> Self {
        BoxMirror {
            x: crate::sim_sim::SimExports::default(),
            slot: -1.0,
            live: false,
            lines: false,
            rays: false,
            circles: false,
            owes: false,
            glass: None,
            sheets_in: None,
            block: None,
            owed: 0.0,
            glass_busy: 0.0,
            glass_angles: false,
            view_lengths: false,
            glass_n: 0.0,
            glass_bis: Vec::new(),
            row_lines: false,
            row_rays: false,
            row_circles: false,
            walk: None,
            list_cap: 0.0,
            scratch_len: 0.0,
            lamp_cap: 0.0,
            wind_cap: 0.0,
            nav_cap: 0.0,
            lamp_set: -2.0,
            lamp_n: 0.0,
            rule_set: -1.0,
            rule_panes: 0.0,
            copy: 0.0,
            grid_over: false,
            cells_wrote: true,
            grid_of: None,
            used: 0.0,
            caps: crate::scene_box_mirror::BoxMirrorCaps { boxes: 0.0, leaves: 0.0, cuts: 0.0, panes: 0.0, cells: 0.0, items: 0.0, tilted: 0.0, things: 0.0, glass: 0.0, view: 0.0 },
            things_at: -1.0,
            things_ok: false,
            things_all: true,
            snow_len: -1.0,
            boxes: 0.0,
            n_tilted: 0.0,
            epoch: -1.0,
            put_backs: -1.0,
            cut_of: Vec::new(),
            cut_free: Vec::new(),
            cut_next: 0.0,
            own: None,
            own_len: 0.0,
            own_at: -1.0,
            played: false,
            of: None,
            stood: 0,
        }
    }
}
impl Default for BoxMirror { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, Default)]
pub struct BoxMirrorGridOf {
    pub cells: Vec<Vec<f64>>,
    pub tilted_at: Vec<f64>,
    pub w: f64,
    pub h: f64,
    pub cs: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxMirrorCaps {
    pub boxes: f64,
    pub leaves: f64,
    pub cuts: f64,
    pub panes: f64,
    pub cells: f64,
    pub items: f64,
    pub tilted: f64,
    pub things: f64,
    pub glass: f64,
    pub view: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SceneBoxMirrorModCLOCK {
    pub n: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SceneBoxMirrorModCOPIES {
    pub n: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SceneBoxMirrorModLEAVES {
    pub n: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SceneBoxMirrorModTHINGS {
    pub n: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NAV {
    pub no_way: f64,
    pub ask_host: f64,
    pub need: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FACE {
    pub lamp: f64,
    pub max_lamps: f64,
    pub blob: f64,
    pub max_blobs: f64,
    pub graze_a: f64,
    pub graze_b: f64,
    pub over: f64,
    pub far: f64,
    pub walled: f64,
    pub sealed: f64,
    pub routed: f64,
    pub routed_door: f64,
    pub q_lens: f64,
    pub q_unseen: f64,
    pub q_smoked: f64,
    pub a_visible: f64,
    pub a_in_cone: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneBoxMirrorMod {
    /// (src/scene/boxMirror.ts:54 HELD)
    pub held: Vec<Option<crate::scene_box_mirror::BoxMirror>>,
    /// (src/scene/boxMirror.ts:55 CLOCK)
    pub clock: crate::scene_box_mirror::SceneBoxMirrorModCLOCK,
    /// (src/scene/boxMirror.ts:57 COPIES)
    pub copies: crate::scene_box_mirror::SceneBoxMirrorModCOPIES,
    /// (src/scene/boxMirror.ts:59 LEAVES)
    pub leaves: crate::scene_box_mirror::SceneBoxMirrorModLEAVES,
    /// (src/scene/boxMirror.ts:65 THINGS)
    pub things: crate::scene_box_mirror::SceneBoxMirrorModTHINGS,
}
impl SceneBoxMirrorMod {
    pub fn blank() -> Self {
        SceneBoxMirrorMod {
            held: Vec::new(),
            clock: crate::scene_box_mirror::SceneBoxMirrorModCLOCK { n: 0.0 },
            copies: crate::scene_box_mirror::SceneBoxMirrorModCOPIES { n: 0.0 },
            leaves: crate::scene_box_mirror::SceneBoxMirrorModLEAVES { n: 0.0 },
            things: crate::scene_box_mirror::SceneBoxMirrorModTHINGS { n: 0.0 },
        }
    }
}

/// (src/scene/boxMirror.ts:38 REC)
pub const REC: f64 = 16.0;

/// (src/scene/boxMirror.ts:38 R_COS)
pub const R_COS: f64 = 6.0;

/// (src/scene/boxMirror.ts:38 R_SIN)
pub const R_SIN: f64 = 7.0;

/// (src/scene/boxMirror.ts:38 R_YAW)
pub const R_YAW: f64 = 8.0;

/// (src/scene/boxMirror.ts:38 R_ROT)
pub const R_ROT: f64 = 9.0;

/// (src/scene/boxMirror.ts:38 R_CUT)
pub const R_CUT: f64 = 13.0;

/// (src/scene/boxMirror.ts:38 R_CUTN)
pub const R_CUTN: f64 = 14.0;

/// (src/scene/boxMirror.ts:38 R_KIND)
pub const R_KIND: f64 = 15.0;

/// (src/scene/boxMirror.ts:38 K_ROT)
pub const K_ROT: f64 = 1.0;

/// (src/scene/boxMirror.ts:38 K_ROUND)
pub const K_ROUND: f64 = 2.0;

/// (src/scene/boxMirror.ts:38 K_CUT)
pub const K_CUT: f64 = 4.0;

/// (src/scene/boxMirror.ts:38 K_WALK)
pub const K_WALK: f64 = 8.0;

/// (src/scene/boxMirror.ts:38 F_GLASS)
pub const F_GLASS: f64 = 131072.0;

/// (src/scene/boxMirror.ts:39 T_REC)
pub const T_REC: f64 = 0.0;

/// (src/scene/boxMirror.ts:39 T_LEAF)
pub const T_LEAF: f64 = 1.0;

/// (src/scene/boxMirror.ts:39 T_CUT)
pub const T_CUT: f64 = 2.0;

/// (src/scene/boxMirror.ts:39 T_PANE_T)
pub const T_PANE_T: f64 = 3.0;

/// (src/scene/boxMirror.ts:39 T_FLAGS)
pub const T_FLAGS: f64 = 4.0;

/// (src/scene/boxMirror.ts:39 T_LEAF_FLAGS)
pub const T_LEAF_FLAGS: f64 = 5.0;

/// (src/scene/boxMirror.ts:39 T_START)
pub const T_START: f64 = 6.0;

/// (src/scene/boxMirror.ts:39 T_ITEMS)
pub const T_ITEMS: f64 = 7.0;

/// (src/scene/boxMirror.ts:39 T_TILTED)
pub const T_TILTED: f64 = 8.0;

/// (src/scene/boxMirror.ts:39 T_PANE_BI)
pub const T_PANE_BI: f64 = 9.0;

/// (src/scene/boxMirror.ts:39 T_WORLDS)
pub const T_WORLDS: f64 = 10.0;

/// (src/scene/boxMirror.ts:39 T_PT)
pub const T_PT: f64 = 12.0;

/// (src/scene/boxMirror.ts:39 T_LIST)
pub const T_LIST: f64 = 13.0;

/// (src/scene/boxMirror.ts:39 T_THING)
pub const T_THING: f64 = 14.0;

/// (src/scene/boxMirror.ts:39 T_THING_LIVE)
pub const T_THING_LIVE: f64 = 15.0;

/// (src/scene/boxMirror.ts:40 T_GLASS)
pub const T_GLASS: f64 = 16.0;

/// (src/scene/boxMirror.ts:40 T_GLASS_W)
pub const T_GLASS_W: f64 = 17.0;

/// (src/scene/boxMirror.ts:40 T_GLASS_OF)
pub const T_GLASS_OF: f64 = 18.0;

/// (src/scene/boxMirror.ts:40 T_VIEW)
pub const T_VIEW: f64 = 19.0;

/// (src/scene/boxMirror.ts:40 T_VIEW_BI)
pub const T_VIEW_BI: f64 = 20.0;

/// (src/scene/boxMirror.ts:40 T_SNOW)
pub const T_SNOW: f64 = 21.0;

/// (src/scene/boxMirror.ts:42 GD)
pub const GD: f64 = 80.0;

/// (src/scene/boxMirror.ts:42 GW)
pub const GW: f64 = 64.0;

/// (src/scene/boxMirror.ts:42 G_SITE)
pub const G_SITE: f64 = 9.0;

/// (src/scene/boxMirror.ts:42 G_SITES)
pub const G_SITES: f64 = 8.0;

/// (src/scene/boxMirror.ts:42 G_DICE_AT)
pub const G_DICE_AT: f64 = 72.0;

/// (src/scene/boxMirror.ts:42 GW_FLAGS)
pub const GW_FLAGS: f64 = 1.0;

/// (src/scene/boxMirror.ts:42 GW_COLS)
pub const GW_COLS: f64 = 2.0;

/// (src/scene/boxMirror.ts:42 GW_ROWS)
pub const GW_ROWS: f64 = 3.0;

/// (src/scene/boxMirror.ts:42 GW_DSEED)
pub const GW_DSEED: f64 = 4.0;

/// (src/scene/boxMirror.ts:42 GW_N)
pub const GW_N: f64 = 5.0;

/// (src/scene/boxMirror.ts:42 GW_SEED)
pub const GW_SEED: f64 = 13.0;

/// (src/scene/boxMirror.ts:42 GW_OPEN)
pub const GW_OPEN: f64 = 21.0;

/// (src/scene/boxMirror.ts:42 GW_GOING)
pub const GW_GOING: f64 = 42.0;

/// (src/scene/boxMirror.ts:42 GW_CRATER)
pub const GW_CRATER: f64 = 63.0;

/// (src/scene/boxMirror.ts:42 GW_BITS)
pub const GW_BITS: f64 = 21.0;

/// (src/scene/boxMirror.ts:43 G_GONE)
pub const G_GONE: f64 = 1.0;

/// (src/scene/boxMirror.ts:43 G_DICE)
pub const G_DICE: f64 = 2.0;

/// (src/scene/boxMirror.ts:43 G_RUN)
pub const G_RUN: f64 = 4.0;

/// (src/scene/boxMirror.ts:43 VD)
pub const VD: f64 = 4.0;

/// (src/scene/boxMirror.ts:43 FRAGS_PER_SITE)
pub const FRAGS_PER_SITE: f64 = 84.0;

/// (src/scene/boxMirror.ts:43 SECTORS)
pub const SECTORS: f64 = 12.0;

/// (src/scene/boxMirror.ts:45 THING_SET)
pub const THING_SET: f64 = 2.0;

/// (src/scene/boxMirror.ts:45 TH)
pub const TH: f64 = 4.0;

/// (src/scene/boxMirror.ts:45 TL)
pub const TL: f64 = 3.0;

/// (src/scene/boxMirror.ts:47 OWE_BOXES)
pub const OWE_BOXES: f64 = 1.0;

/// (src/scene/boxMirror.ts:47 OWE_GLASS)
pub const OWE_GLASS: f64 = 2.0;

/// (src/scene/boxMirror.ts:47 OWE_VIEW)
pub const OWE_VIEW: f64 = 4.0;

/// (src/scene/boxMirror.ts:50 PANE_CAP)
pub const PANE_CAP: f64 = 256.0;

/// (src/scene/boxMirror.ts:51 PLANE)
pub const PLANE: f64 = 4.0;

/// (src/scene/boxMirror.ts:51 CUT_ROW)
pub const CUT_ROW: f64 = 32.0;

/// (src/scene/boxMirror.ts:79 VIEW_NONE)
pub const VIEW_NONE: f64 = 0.0;

/// (src/scene/boxMirror.ts:79 VIEW_BLOWN)
pub const VIEW_BLOWN: f64 = 1.0;

/// (src/scene/boxMirror.ts:79 VIEW_GIVES)
pub const VIEW_GIVES: f64 = 2.0;

/// (src/scene/boxMirror.ts:93 NO_SHEETS: a table nothing writes)
pub fn no_sheets() -> Vec<f64> { Vec::new() }

/// (src/scene/boxMirror.ts:101 MIRRORED: a table nothing writes)
pub fn mirrored() -> f64 { 4294901770.0 }

/// (src/scene/boxMirror.ts:108 NAV: a table nothing writes)
pub fn nav() -> crate::scene_box_mirror::NAV { crate::scene_box_mirror::NAV { no_way: -1.0, ask_host: -2.0, need: -3.0 } }

/// (src/scene/boxMirror.ts:109 FACE: a table nothing writes)
pub fn face() -> crate::scene_box_mirror::FACE { crate::scene_box_mirror::FACE { lamp: 16.0, max_lamps: 256.0, blob: 6.0, max_blobs: 64.0, graze_a: 1.0, graze_b: 2.0, over: 4.0, far: 8.0, walled: 16.0, sealed: 0.0, routed: 1.0, routed_door: 3.0, q_lens: 1.0, q_unseen: 2.0, q_smoked: 4.0, a_visible: 1.0, a_in_cone: 2.0 } }
