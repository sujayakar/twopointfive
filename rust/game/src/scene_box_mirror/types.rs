//! The types of `scene_box_mirror`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Furniture, src/scene/boxMirror.ts:69)
#[derive(Clone, Debug)]
pub struct Furniture {
    pub all: Vec<crate::scene_box_mirror::FurnitureThing>,
    pub props: Vec<crate::scene_box_mirror::FurnitureThing>,
}
impl Furniture {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Furniture {
            all: Vec::new(),
            props: Vec::new(),
        }
    }
}
impl Default for Furniture { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FurnitureThing, src/scene/boxMirror.ts:70)
#[derive(Clone, Debug)]
pub struct FurnitureThing {
    pub id: f64,
    pub x: f64,
    pub z: f64,
    pub yaw: f64,
    pub radius: f64,
    pub boxes: Vec<BoxId>,
}
impl FurnitureThing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureThing {
            id: 0.0,
            x: 0.0,
            z: 0.0,
            yaw: 0.0,
            radius: 0.0,
            boxes: Vec::new(),
        }
    }
}
impl Default for FurnitureThing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneSite, src/scene/boxMirror.ts:73)
#[derive(Clone, Debug)]
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
impl PaneSite {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneSite {
            u: 0.0,
            v: 0.0,
            rot: 0.0,
            n: 0.0,
            seed: 0.0,
            rho: Vec::new(),
            rz: 0.0,
            merge: 0.0,
        }
    }
}
impl Default for PaneSite { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneDice, src/scene/boxMirror.ts:75)
#[derive(Clone, Debug)]
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
impl PaneDice {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneDice {
            u: 0.0,
            v: 0.0,
            cols: 0.0,
            rows: 0.0,
            seed: 0.0,
            hu: 0.0,
            hv: 0.0,
            next: 0.0,
            out: Vec::new(),
            clumps: Vec::new(),
        }
    }
}
impl Default for PaneDice { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:75 PaneDice.clumps, src/scene/boxMirror.ts:75)
#[derive(Clone, Copy, Debug)]
pub struct PaneDiceClumpsItem {
    pub k: f64,
    pub key: f64,
}
impl PaneDiceClumpsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneDiceClumpsItem {
            k: 0.0,
            key: 0.0,
        }
    }
}
impl Default for PaneDiceClumpsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneRow, src/scene/boxMirror.ts:76)
#[derive(Clone, Debug)]
pub struct PaneRow {
    pub bi: f64,
    pub gone: bool,
    pub run: bool,
    pub sites: Vec<crate::scene_box_mirror::PaneSite>,
    pub open: Vec<u8>,
    pub pending: Vec<crate::scene_box_mirror::PaneRowPendingItem>,
    pub dice: Option<crate::scene_box_mirror::PaneDice>,
}
impl PaneRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneRow {
            bi: 0.0,
            gone: false,
            run: false,
            sites: Vec::new(),
            open: Vec::new(),
            pending: Vec::new(),
            dice: None,
        }
    }
}
impl Default for PaneRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:76 PaneRow.pending, src/scene/boxMirror.ts:76)
#[derive(Clone, Copy, Debug)]
pub struct PaneRowPendingItem {
    pub fid: f64,
}
impl PaneRowPendingItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneRowPendingItem {
            fid: 0.0,
        }
    }
}
impl Default for PaneRowPendingItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneView, src/scene/boxMirror.ts:86)
#[derive(Clone, Debug)]
pub struct PaneView {
    pub kind: f64,
    pub bi: Vec<f64>,
    pub at: Vec<f64>,
}
impl PaneView {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneView {
            kind: 0.0,
            bi: Vec::new(),
            at: Vec::new(),
        }
    }
}
impl Default for PaneView { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneSource, src/scene/boxMirror.ts:89)
#[derive(Clone, Debug)]
pub struct PaneSource {
    pub view: Option<crate::scene_box_mirror::PaneView>,
}
impl PaneSource {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneSource {
            view: None,
        }
    }
}
impl Default for PaneSource { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BoxMirror, src/scene/boxMirror.ts:143)
#[derive(Clone, Debug)]
pub struct BoxMirror {
    pub x: crate::sim_sim::SimExports,
    pub slot: f64,
    pub live: bool,
    pub lines: bool,
    pub rays: bool,
    pub circles: bool,
    pub sight: bool,
    pub senses: f64,
    pub owes: bool,
    pub glass: Option<crate::scene_box_mirror::PaneSource>,
    pub owed: f64,
    pub glass_busy: f64,
    pub glass_angles: bool,
    pub view_lengths: bool,
    pub glass_d: f64,
    pub glass_w: f64,
    pub glass_of: f64,
    pub view_d: f64,
    pub view_bi: f64,
    pub glass_n: f64,
    pub glass_bis: Vec<f64>,
    pub row_lines: bool,
    pub row_rays: bool,
    pub row_circles: bool,
    pub row_sight: bool,
    pub row_senses: f64,
    pub walk: Option<crate::FnField>,
    pub eyes: f64,
    pub pt: f64,
    pub list: f64,
    pub list_cap: f64,
    pub scratch: f64,
    pub scratch_len: f64,
    pub lamps: f64,
    pub lamp_cap: f64,
    pub blobs: f64,
    pub blob_cap: f64,
    pub wind: f64,
    pub wind_cap: f64,
    pub nav_shut: f64,
    pub nav_cap: f64,
    pub way: f64,
    pub nav_want: f64,
    pub nav_price: f64,
    pub lamp_set: f64,
    pub lamp_n: f64,
    pub blob_set: f64,
    pub blob_n: f64,
    pub copy: f64,
    pub grid_over: bool,
    pub cells_wrote: bool,
    pub grid_of: Option<crate::scene_box_mirror::BoxMirrorGridOf>,
    pub used: f64,
    pub caps: crate::scene_box_mirror::BoxMirrorCaps,
    pub thing: f64,
    pub thing_live: f64,
    pub things_at: f64,
    pub things_of: Option<crate::scene_box_mirror::Furniture>,
    pub things_ok: bool,
    pub things_all: bool,
    pub snow_at0: f64,
    pub snow_len: f64,
    pub rec: f64,
    pub leaf: f64,
    pub cut: f64,
    pub pane_t: f64,
    pub flags: f64,
    pub leaf_flags: f64,
    pub start: f64,
    pub items: f64,
    pub tilted: f64,
    pub pane_bi: f64,
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
}
impl BoxMirror {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        BoxMirror {
            x: crate::sim_sim::SimExports::zeroed(),
            slot: -1.0,
            live: false,
            lines: false,
            rays: false,
            circles: false,
            sight: false,
            senses: 0.0,
            owes: false,
            glass: None,
            owed: 0.0,
            glass_busy: 0.0,
            glass_angles: false,
            view_lengths: false,
            glass_d: 0.0,
            glass_w: 0.0,
            glass_of: 0.0,
            view_d: 0.0,
            view_bi: 0.0,
            glass_n: 0.0,
            glass_bis: Vec::new(),
            row_lines: false,
            row_rays: false,
            row_circles: false,
            row_sight: false,
            row_senses: 0.0,
            walk: None,
            eyes: 0.0,
            pt: 0.0,
            list: 0.0,
            list_cap: 0.0,
            scratch: 0.0,
            scratch_len: 0.0,
            lamps: 0.0,
            lamp_cap: 0.0,
            blobs: 0.0,
            blob_cap: 0.0,
            wind: 0.0,
            wind_cap: 0.0,
            nav_shut: 0.0,
            nav_cap: 0.0,
            way: 0.0,
            nav_want: 0.0,
            nav_price: 0.0,
            lamp_set: -2.0,
            lamp_n: 0.0,
            blob_set: -2.0,
            blob_n: 0.0,
            copy: 0.0,
            grid_over: false,
            cells_wrote: true,
            grid_of: None,
            used: 0.0,
            caps: crate::scene_box_mirror::BoxMirrorCaps { boxes: 0.0, leaves: 0.0, cuts: 0.0, panes: 0.0, cells: 0.0, items: 0.0, tilted: 0.0, things: 0.0, glass: 0.0, view: 0.0 },
            thing: 0.0,
            thing_live: 0.0,
            things_at: -1.0,
            things_of: None,
            things_ok: false,
            things_all: true,
            snow_at0: 0.0,
            snow_len: -1.0,
            rec: 0.0,
            leaf: 0.0,
            cut: 0.0,
            pane_t: 0.0,
            flags: 0.0,
            leaf_flags: 0.0,
            start: 0.0,
            items: 0.0,
            tilted: 0.0,
            pane_bi: 0.0,
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
        }
    }
}
impl Default for BoxMirror { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:185 BoxMirror.gridOf, src/scene/boxMirror.ts:185)
#[derive(Clone, Debug)]
pub struct BoxMirrorGridOf {
    pub cells: Vec<Vec<f64>>,
    pub tilted_at: Vec<f64>,
    pub w: f64,
    pub h: f64,
    pub cs: f64,
}
impl BoxMirrorGridOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxMirrorGridOf {
            cells: Vec::new(),
            tilted_at: Vec::new(),
            w: 0.0,
            h: 0.0,
            cs: 0.0,
        }
    }
}
impl Default for BoxMirrorGridOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:187 BoxMirror.caps, src/scene/boxMirror.ts:187)
#[derive(Clone, Copy, Debug)]
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
impl BoxMirrorCaps {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxMirrorCaps {
            boxes: 0.0,
            leaves: 0.0,
            cuts: 0.0,
            panes: 0.0,
            cells: 0.0,
            items: 0.0,
            tilted: 0.0,
            things: 0.0,
            glass: 0.0,
            view: 0.0,
        }
    }
}
impl Default for BoxMirrorCaps { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:56 CLOCK, src/scene/boxMirror.ts:56)
#[derive(Clone, Copy, Debug)]
pub struct SceneBoxMirrorModCLOCK {
    pub n: f64,
}
impl SceneBoxMirrorModCLOCK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneBoxMirrorModCLOCK {
            n: 0.0,
        }
    }
}
impl Default for SceneBoxMirrorModCLOCK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:58 COPIES, src/scene/boxMirror.ts:58)
#[derive(Clone, Copy, Debug)]
pub struct SceneBoxMirrorModCOPIES {
    pub n: f64,
}
impl SceneBoxMirrorModCOPIES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneBoxMirrorModCOPIES {
            n: 0.0,
        }
    }
}
impl Default for SceneBoxMirrorModCOPIES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:60 LEAVES, src/scene/boxMirror.ts:60)
#[derive(Clone, Copy, Debug)]
pub struct SceneBoxMirrorModLEAVES {
    pub n: f64,
}
impl SceneBoxMirrorModLEAVES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneBoxMirrorModLEAVES {
            n: 0.0,
        }
    }
}
impl Default for SceneBoxMirrorModLEAVES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:66 THINGS, src/scene/boxMirror.ts:66)
#[derive(Clone, Copy, Debug)]
pub struct SceneBoxMirrorModTHINGS {
    pub n: f64,
}
impl SceneBoxMirrorModTHINGS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneBoxMirrorModTHINGS {
            n: 0.0,
        }
    }
}
impl Default for SceneBoxMirrorModTHINGS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:99 SENSES, src/scene/boxMirror.ts:99)
#[derive(Clone, Copy, Debug)]
pub struct SENSES {
    pub sight: f64,
    pub light: f64,
    pub smoke: f64,
    pub sound: f64,
    pub paths: f64,
}
impl SENSES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SENSES {
            sight: 0.0,
            light: 0.0,
            smoke: 0.0,
            sound: 0.0,
            paths: 0.0,
        }
    }
}
impl Default for SENSES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:106 NAV, src/scene/boxMirror.ts:106)
#[derive(Clone, Copy, Debug)]
pub struct NAV {
    pub no_way: f64,
    pub ask_host: f64,
    pub need: f64,
}
impl NAV {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NAV {
            no_way: 0.0,
            ask_host: 0.0,
            need: 0.0,
        }
    }
}
impl Default for NAV { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/boxMirror.ts:107 FACE, src/scene/boxMirror.ts:107)
#[derive(Clone, Copy, Debug)]
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
impl FACE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FACE {
            lamp: 0.0,
            max_lamps: 0.0,
            blob: 0.0,
            max_blobs: 0.0,
            graze_a: 0.0,
            graze_b: 0.0,
            over: 0.0,
            far: 0.0,
            walled: 0.0,
            sealed: 0.0,
            routed: 0.0,
            routed_door: 0.0,
            q_lens: 0.0,
            q_unseen: 0.0,
            q_smoked: 0.0,
            a_visible: 0.0,
            a_in_cone: 0.0,
        }
    }
}
impl Default for FACE { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneBoxMirrorMod {
    /// (src/scene/boxMirror.ts:55 HELD)
    pub held: Vec<Option<crate::scene_box_mirror::BoxMirror>>,
    /// (src/scene/boxMirror.ts:56 CLOCK)
    pub clock: crate::scene_box_mirror::SceneBoxMirrorModCLOCK,
    /// (src/scene/boxMirror.ts:58 COPIES)
    pub copies: crate::scene_box_mirror::SceneBoxMirrorModCOPIES,
    /// (src/scene/boxMirror.ts:60 LEAVES)
    pub leaves: crate::scene_box_mirror::SceneBoxMirrorModLEAVES,
    /// (src/scene/boxMirror.ts:66 THINGS)
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

/// (src/scene/boxMirror.ts:37 REC)
pub const REC: f64 = 16.0;

/// (src/scene/boxMirror.ts:37 R_COS)
pub const R_COS: f64 = 6.0;

/// (src/scene/boxMirror.ts:37 R_SIN)
pub const R_SIN: f64 = 7.0;

/// (src/scene/boxMirror.ts:37 R_YAW)
pub const R_YAW: f64 = 8.0;

/// (src/scene/boxMirror.ts:37 R_ROT)
pub const R_ROT: f64 = 9.0;

/// (src/scene/boxMirror.ts:37 R_CUT)
pub const R_CUT: f64 = 13.0;

/// (src/scene/boxMirror.ts:37 R_CUTN)
pub const R_CUTN: f64 = 14.0;

/// (src/scene/boxMirror.ts:37 R_KIND)
pub const R_KIND: f64 = 15.0;

/// (src/scene/boxMirror.ts:37 K_ROT)
pub const K_ROT: f64 = 1.0;

/// (src/scene/boxMirror.ts:37 K_ROUND)
pub const K_ROUND: f64 = 2.0;

/// (src/scene/boxMirror.ts:37 K_CUT)
pub const K_CUT: f64 = 4.0;

/// (src/scene/boxMirror.ts:37 K_WALK)
pub const K_WALK: f64 = 8.0;

/// (src/scene/boxMirror.ts:37 F_GLASS)
pub const F_GLASS: f64 = 131072.0;

/// (src/scene/boxMirror.ts:38 T_REC)
pub const T_REC: f64 = 0.0;

/// (src/scene/boxMirror.ts:38 T_LEAF)
pub const T_LEAF: f64 = 1.0;

/// (src/scene/boxMirror.ts:38 T_CUT)
pub const T_CUT: f64 = 2.0;

/// (src/scene/boxMirror.ts:38 T_PANE_T)
pub const T_PANE_T: f64 = 3.0;

/// (src/scene/boxMirror.ts:38 T_FLAGS)
pub const T_FLAGS: f64 = 4.0;

/// (src/scene/boxMirror.ts:38 T_LEAF_FLAGS)
pub const T_LEAF_FLAGS: f64 = 5.0;

/// (src/scene/boxMirror.ts:38 T_START)
pub const T_START: f64 = 6.0;

/// (src/scene/boxMirror.ts:38 T_ITEMS)
pub const T_ITEMS: f64 = 7.0;

/// (src/scene/boxMirror.ts:38 T_TILTED)
pub const T_TILTED: f64 = 8.0;

/// (src/scene/boxMirror.ts:38 T_PANE_BI)
pub const T_PANE_BI: f64 = 9.0;

/// (src/scene/boxMirror.ts:38 T_WORLDS)
pub const T_WORLDS: f64 = 10.0;

/// (src/scene/boxMirror.ts:38 T_PT)
pub const T_PT: f64 = 12.0;

/// (src/scene/boxMirror.ts:38 T_LIST)
pub const T_LIST: f64 = 13.0;

/// (src/scene/boxMirror.ts:38 T_THING)
pub const T_THING: f64 = 14.0;

/// (src/scene/boxMirror.ts:38 T_THING_LIVE)
pub const T_THING_LIVE: f64 = 15.0;

/// (src/scene/boxMirror.ts:39 T_GLASS)
pub const T_GLASS: f64 = 16.0;

/// (src/scene/boxMirror.ts:39 T_GLASS_W)
pub const T_GLASS_W: f64 = 17.0;

/// (src/scene/boxMirror.ts:39 T_GLASS_OF)
pub const T_GLASS_OF: f64 = 18.0;

/// (src/scene/boxMirror.ts:39 T_VIEW)
pub const T_VIEW: f64 = 19.0;

/// (src/scene/boxMirror.ts:39 T_VIEW_BI)
pub const T_VIEW_BI: f64 = 20.0;

/// (src/scene/boxMirror.ts:39 T_SNOW)
pub const T_SNOW: f64 = 21.0;

/// (src/scene/boxMirror.ts:41 GD)
pub const GD: f64 = 80.0;

/// (src/scene/boxMirror.ts:41 GW)
pub const GW: f64 = 64.0;

/// (src/scene/boxMirror.ts:41 G_SITE)
pub const G_SITE: f64 = 9.0;

/// (src/scene/boxMirror.ts:41 G_SITES)
pub const G_SITES: f64 = 8.0;

/// (src/scene/boxMirror.ts:41 G_DICE_AT)
pub const G_DICE_AT: f64 = 72.0;

/// (src/scene/boxMirror.ts:41 GW_FLAGS)
pub const GW_FLAGS: f64 = 1.0;

/// (src/scene/boxMirror.ts:41 GW_COLS)
pub const GW_COLS: f64 = 2.0;

/// (src/scene/boxMirror.ts:41 GW_ROWS)
pub const GW_ROWS: f64 = 3.0;

/// (src/scene/boxMirror.ts:41 GW_DSEED)
pub const GW_DSEED: f64 = 4.0;

/// (src/scene/boxMirror.ts:41 GW_N)
pub const GW_N: f64 = 5.0;

/// (src/scene/boxMirror.ts:41 GW_SEED)
pub const GW_SEED: f64 = 13.0;

/// (src/scene/boxMirror.ts:41 GW_OPEN)
pub const GW_OPEN: f64 = 21.0;

/// (src/scene/boxMirror.ts:41 GW_GOING)
pub const GW_GOING: f64 = 42.0;

/// (src/scene/boxMirror.ts:41 GW_CRATER)
pub const GW_CRATER: f64 = 63.0;

/// (src/scene/boxMirror.ts:41 GW_BITS)
pub const GW_BITS: f64 = 21.0;

/// (src/scene/boxMirror.ts:42 G_GONE)
pub const G_GONE: f64 = 1.0;

/// (src/scene/boxMirror.ts:42 G_DICE)
pub const G_DICE: f64 = 2.0;

/// (src/scene/boxMirror.ts:42 G_RUN)
pub const G_RUN: f64 = 4.0;

/// (src/scene/boxMirror.ts:42 VD)
pub const VD: f64 = 4.0;

/// (src/scene/boxMirror.ts:42 FRAGS_PER_SITE)
pub const FRAGS_PER_SITE: f64 = 84.0;

/// (src/scene/boxMirror.ts:42 SECTORS)
pub const SECTORS: f64 = 12.0;

/// (src/scene/boxMirror.ts:44 THING_SET)
pub const THING_SET: f64 = 2.0;

/// (src/scene/boxMirror.ts:44 TH)
pub const TH: f64 = 4.0;

/// (src/scene/boxMirror.ts:44 TL)
pub const TL: f64 = 3.0;

/// (src/scene/boxMirror.ts:46 OWE_BOXES)
pub const OWE_BOXES: f64 = 1.0;

/// (src/scene/boxMirror.ts:46 OWE_GLASS)
pub const OWE_GLASS: f64 = 2.0;

/// (src/scene/boxMirror.ts:46 OWE_VIEW)
pub const OWE_VIEW: f64 = 4.0;

/// (src/scene/boxMirror.ts:49 PANE_CAP)
pub const PANE_CAP: f64 = 256.0;

/// (src/scene/boxMirror.ts:52 PLANE)
pub const PLANE: f64 = 4.0;

/// (src/scene/boxMirror.ts:52 CUT_ROW)
pub const CUT_ROW: f64 = 32.0;

/// (src/scene/boxMirror.ts:80 VIEW_NONE)
pub const VIEW_NONE: f64 = 0.0;

/// (src/scene/boxMirror.ts:80 VIEW_BLOWN)
pub const VIEW_BLOWN: f64 = 1.0;

/// (src/scene/boxMirror.ts:80 VIEW_GIVES)
pub const VIEW_GIVES: f64 = 2.0;

/// (src/scene/boxMirror.ts:97 MIRRORED: a table nothing writes)
pub fn mirrored() -> f64 { 4294901770.0 }

/// (src/scene/boxMirror.ts:99 SENSES: a table nothing writes)
pub fn senses() -> crate::scene_box_mirror::SENSES { crate::scene_box_mirror::SENSES { sight: 1.0, light: 2.0, smoke: 4.0, sound: 8.0, paths: 16.0 } }

/// (src/scene/boxMirror.ts:106 NAV: a table nothing writes)
pub fn nav() -> crate::scene_box_mirror::NAV { crate::scene_box_mirror::NAV { no_way: -1.0, ask_host: -2.0, need: -3.0 } }

/// (src/scene/boxMirror.ts:107 FACE: a table nothing writes)
pub fn face() -> crate::scene_box_mirror::FACE { crate::scene_box_mirror::FACE { lamp: 15.0, max_lamps: 256.0, blob: 6.0, max_blobs: 64.0, graze_a: 1.0, graze_b: 2.0, over: 4.0, far: 8.0, walled: 16.0, sealed: 0.0, routed: 1.0, routed_door: 3.0, q_lens: 1.0, q_unseen: 2.0, q_smoked: 4.0, a_visible: 1.0, a_in_cone: 2.0 } }
