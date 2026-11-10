//! The types of `scene_box_mirror`.
#![allow(unused_imports)]
use crate::{Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct PaneView {
    pub kind: f64,
    pub at: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct PaneSource {
}

#[derive(Clone, Debug, Blank)]
pub struct BoxMirror {
    #[is(-1.0)] pub slot: f64,
    pub live: bool,
    pub lines: bool,
    pub rays: bool,
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
    pub walk: Option<crate::FnField>,
    pub scratch_len: f64,
    pub lamp_cap: f64,
    pub wind_cap: f64,
    pub nav_cap: f64,
    #[is(-2.0)] pub lamp_set: f64,
    pub lamp_n: f64,
    #[is(-1.0)] pub rule_set: f64,
    pub rule_panes: f64,
    pub copy: f64,
    pub grid_over: bool,
    #[is(true)] pub cells_wrote: bool,
    pub grid_of: Option<crate::scene_box_mirror::BoxMirrorGridOf>,
    #[is(crate::scene_box_mirror::BoxMirrorCaps { boxes: 0.0, leaves: 0.0, cuts: 0.0, panes: 0.0, cells: 0.0, items: 0.0, tilted: 0.0, things: 0.0, glass: 0.0, view: 0.0 })] pub caps: crate::scene_box_mirror::BoxMirrorCaps,
    #[is(-1.0)] pub things_at: f64,
    pub things_ok: bool,
    #[is(true)] pub things_all: bool,
    #[is(-1.0)] pub snow_len: f64,
    pub boxes: f64,
    pub n_tilted: f64,
    #[is(-1.0)] pub epoch: f64,
    #[is(-1.0)] pub put_backs: f64,
    pub cut_of: Vec<i32>,
    pub cut_free: Vec<f64>,
    pub cut_next: f64,
    pub own: Option<Vec<BoxId>>,
    pub own_len: f64,
    #[is(-1.0)] pub own_at: f64,
    pub played: bool,
    pub of: Option<String>,
    /// the module's world as this game wrote it last (world.rs `stood`): a number of this process's, on no wire and in no word of the state
    pub stood: u32,
}

#[derive(Clone, Debug, Default)]
pub struct BoxMirrorGridOf {
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

/// the state this module keeps at its top level
#[derive(Clone, Debug, Blank)]
pub struct SceneBoxMirrorMod {
    #[is(crate::scene_box_mirror::SceneBoxMirrorModCOPIES { n: 0.0 })] pub copies: crate::scene_box_mirror::SceneBoxMirrorModCOPIES,
    #[is(crate::scene_box_mirror::SceneBoxMirrorModLEAVES { n: 0.0 })] pub leaves: crate::scene_box_mirror::SceneBoxMirrorModLEAVES,
    #[is(crate::scene_box_mirror::SceneBoxMirrorModTHINGS { n: 0.0 })] pub things: crate::scene_box_mirror::SceneBoxMirrorModTHINGS,
}

pub const REC: f64 = 16.0;

pub const R_COS: f64 = 6.0;

pub const R_SIN: f64 = 7.0;

pub const R_YAW: f64 = 8.0;

pub const R_ROT: f64 = 9.0;

pub const R_CUT: f64 = 13.0;

pub const R_CUTN: f64 = 14.0;

pub const R_KIND: f64 = 15.0;

pub const K_ROT: f64 = 1.0;

pub const K_ROUND: f64 = 2.0;

pub const K_CUT: f64 = 4.0;

pub const K_WALK: f64 = 8.0;

pub const F_GLASS: f64 = 131072.0;

pub const T_REC: f64 = 0.0;

pub const T_LEAF: f64 = 1.0;

pub const T_CUT: f64 = 2.0;

pub const T_PANE_T: f64 = 3.0;

pub const T_START: f64 = 6.0;

pub const T_ITEMS: f64 = 7.0;

pub const T_TILTED: f64 = 8.0;

pub const T_THING: f64 = 14.0;

pub const T_GLASS: f64 = 16.0;

pub const T_VIEW: f64 = 19.0;

pub const THING_SET: f64 = 2.0;

pub const TH: f64 = 4.0;

pub const TL: f64 = 3.0;

pub const OWE_BOXES: f64 = 1.0;

pub const OWE_GLASS: f64 = 2.0;

pub const OWE_VIEW: f64 = 4.0;

pub const PANE_CAP: f64 = 256.0;

pub const PLANE: f64 = 4.0;

pub const CUT_ROW: f64 = 32.0;

pub const VIEW_NONE: f64 = 0.0;

pub const VIEW_BLOWN: f64 = 1.0;

pub const VIEW_GIVES: f64 = 2.0;

pub fn mirrored() -> f64 { 4294901770.0 }

