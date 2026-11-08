//! The types of `glassbreak`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a burst's view of the glass for one judgment (GlassBreak.seeThrough): which panes a walk also passes
#[derive(Clone, Debug)]
pub enum GlassBreakAlso {
    Blown,
    Wary { from: V3, size: crate::glassbreak::GlassBlastOpts },
    Counted { from: V3, size: crate::glassbreak::GlassBlastOpts },
}

/// (TypeScript: GlassBreak, src/game/glassbreak.ts:221)
#[derive(Clone, Debug, ToBytes)]
pub struct GlassBreak {
    pub log: Vec<String>,
    pub sheet_list: Option<Vec<f64>>,
    pub view_of: Option<crate::glassbreak::GlassBreakViewOf>,
    pub gust_dirty: bool,
    pub gusts: JsMap<crate::Num, WindWindSourceId>,
}
impl GlassBreak {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        GlassBreak {
            log: Vec::new(),
            sheet_list: None,
            view_of: None,
            gust_dirty: false,
            gusts: JsMap::new(),
        }
    }
}
impl Default for GlassBreak { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/glassbreak.ts:390 GlassBreak.viewOf, src/game/glassbreak.ts:390)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct GlassBreakViewOf {
    pub kind: crate::glassbreak::GlassBreakViewOfKind,
    pub from: V3,
    pub size: crate::glassbreak::GlassBlastOpts,
}
impl GlassBreakViewOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassBreakViewOf {
            kind: crate::glassbreak::GlassBreakViewOfKind::Blown,
            from: [0.0; 3],
            size: crate::glassbreak::GlassBlastOpts::zeroed(),
        }
    }
}
impl Default for GlassBreakViewOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum GlassBreakViewOfKind {
    Blown,
    Wary,
    Counted,
}
impl GlassBreakViewOfKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassBreakViewOfKind::Blown => "blown",
            GlassBreakViewOfKind::Wary => "wary",
            GlassBreakViewOfKind::Counted => "counted",
        }
    }
}

/// (TypeScript: GlassBlastOpts, src/game/glassbreak.ts:178)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct GlassBlastOpts {
    pub radius: Option<f64>,
    pub power: Option<f64>,
    pub who: Option<f64>,
}
impl GlassBlastOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassBlastOpts {
            radius: None,
            power: None,
            who: None,
        }
    }
}
impl Default for GlassBlastOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GbTune, src/game/glassbreak.ts:125)
#[derive(Clone, Copy, Debug)]
pub struct GbTune {
    pub glass_break: Option<bool>,
}
impl GbTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GbTune {
            glass_break: None,
        }
    }
}
impl Default for GbTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GlassImpact, src/game/glassbreak.ts:147)
#[derive(Clone, Copy, Debug)]
pub struct GlassImpact {
    pub pane: bool,
    pub live: bool,
    pub type_: crate::glasspen::PaneType,
    pub thickness: f64,
    pub was_open: bool,
    pub punched: bool,
    pub diced: bool,
    pub released: f64,
    pub cracked: bool,
}
impl GlassImpact {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassImpact {
            pane: false,
            live: false,
            type_: crate::glasspen::PaneType::Annealed,
            thickness: 0.0,
            was_open: false,
            punched: false,
            diced: false,
            released: 0.0,
            cracked: false,
        }
    }
}
impl Default for GlassImpact { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GlassFragment, src/game/glassbreak.ts:159)
#[derive(Clone, Debug)]
pub struct GlassFragment {
    pub pane: f64,
    pub type_: crate::glasspen::PaneType,
    pub kind: crate::glassbreak::GlassFragmentKind,
    pub outline: Vec<V3>,
    pub centre: V3,
    pub normal: V3,
    pub thickness: f64,
    pub area: f64,
    pub velocity: V3,
    pub spin: V3,
    pub tint: V3,
    pub grain: f64,
    pub cause: crate::glassbreak::GlassFragmentCause,
    pub who: f64,
    pub seed: f64,
    pub fall: Option<crate::glassbreak::GlassFragmentFall>,
    pub landed: Option<bool>,
}
impl GlassFragment {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassFragment {
            pane: 0.0,
            type_: crate::glasspen::PaneType::Annealed,
            kind: crate::glassbreak::GlassFragmentKind::Piece,
            outline: Vec::new(),
            centre: [0.0; 3],
            normal: [0.0; 3],
            thickness: 0.0,
            area: 0.0,
            velocity: [0.0; 3],
            spin: [0.0; 3],
            tint: [0.0; 3],
            grain: 0.0,
            cause: crate::glassbreak::GlassFragmentCause::Blast,
            who: 0.0,
            seed: 0.0,
            fall: None,
            landed: None,
        }
    }
}
impl Default for GlassFragment { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassFragmentKind {
    Piece,
    Sheet,
    Dagger,
    Clump,
}
impl GlassFragmentKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassFragmentKind::Piece => "piece",
            GlassFragmentKind::Sheet => "sheet",
            GlassFragmentKind::Dagger => "dagger",
            GlassFragmentKind::Clump => "clump",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassFragmentCause {
    Blast,
    Hit,
    Dice,
    Support,
    Rake,
}
impl GlassFragmentCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassFragmentCause::Blast => "blast",
            GlassFragmentCause::Hit => "hit",
            GlassFragmentCause::Dice => "dice",
            GlassFragmentCause::Support => "support",
            GlassFragmentCause::Rake => "rake",
        }
    }
}

/// (TypeScript: inline at src/game/glassbreak.ts:176 GlassFragment.fall, src/game/glassbreak.ts:176)
#[derive(Clone, Copy, Debug)]
pub struct GlassFragmentFall {
    pub t: f64,
    pub pos: V3,
}
impl GlassFragmentFall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassFragmentFall {
            t: 0.0,
            pos: [0.0; 3],
        }
    }
}
impl Default for GlassFragmentFall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Clump, src/game/glassbreak.ts:180)
#[derive(Clone, Copy, Debug)]
pub struct Clump {
    pub k: f64,
    pub c: f64,
    pub r: f64,
    pub key: f64,
}
impl Clump {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Clump {
            k: 0.0,
            c: 0.0,
            r: 0.0,
            key: 0.0,
        }
    }
}
impl Default for Clump { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Dice, src/game/glassbreak.ts:181)
#[derive(Clone, Debug)]
pub struct Dice {
    pub t0: f64,
    pub clumps: Vec<crate::glassbreak::Clump>,
    pub next: f64,
    pub out: Vec<u8>,
    pub u: f64,
    pub v: f64,
    pub cols: f64,
    pub rows: f64,
    pub seed: f64,
    pub hu: f64,
    pub hv: f64,
}
impl Dice {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Dice {
            t0: 0.0,
            clumps: Vec::new(),
            next: 0.0,
            out: Vec::new(),
            u: 0.0,
            v: 0.0,
            cols: 0.0,
            rows: 0.0,
            seed: 0.0,
            hu: 0.0,
            hv: 0.0,
        }
    }
}
impl Default for Dice { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Pending, src/game/glassbreak.ts:182)
#[derive(Clone, Copy, Debug)]
pub struct Pending {
    pub fid: f64,
    pub at: f64,
    pub cause: crate::glassbreak::PendingCause,
    pub from: Option<V3>,
    pub k: f64,
}
impl Pending {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Pending {
            fid: 0.0,
            at: 0.0,
            cause: crate::glassbreak::PendingCause::Blast,
            from: None,
            k: 0.0,
        }
    }
}
impl Default for Pending { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PendingCause {
    Blast,
    Hit,
    Dice,
    Support,
    Rake,
}
impl PendingCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            PendingCause::Blast => "blast",
            PendingCause::Hit => "hit",
            PendingCause::Dice => "dice",
            PendingCause::Support => "support",
            PendingCause::Rake => "rake",
        }
    }
}

/// (TypeScript: PaneState, src/game/glassbreak.ts:183)
#[derive(Clone, Debug)]
pub struct PaneState {
    pub bi: f64,
    pub box_: BoxId,
    pub key: f64,
    pub type_: crate::glasspen::PaneType,
    pub u_axis: f64,
    pub hu: f64,
    pub hv: f64,
    pub ht: f64,
    pub exterior: bool,
    pub sites: Vec<crate::glass_pattern::GpSite>,
    pub run: bool,
    pub open: Vec<u8>,
    pub n_open: f64,
    pub hits: f64,
    pub energy: f64,
    pub damage: f64,
    pub frags: Option<Vec<crate::glass_pattern::GpFrag>>,
    pub pending: Vec<crate::glassbreak::Pending>,
    pub next_think: f64,
    pub dice: Option<crate::glassbreak::Dice>,
    pub gone: bool,
    pub last_dir: V3,
    pub last_who: f64,
    pub last_uv: [f64; 2],
    pub sound_at: f64,
    pub walkable: bool,
    pub log: Vec<String>,
}
impl PaneState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneState {
            bi: 0.0,
            box_: BoxId(0),
            key: 0.0,
            type_: crate::glasspen::PaneType::Annealed,
            u_axis: 0.0,
            hu: 0.0,
            hv: 0.0,
            ht: 0.0,
            exterior: false,
            sites: Vec::new(),
            run: false,
            open: Vec::new(),
            n_open: 0.0,
            hits: 0.0,
            energy: 0.0,
            damage: 0.0,
            frags: None,
            pending: Vec::new(),
            next_think: 0.0,
            dice: None,
            gone: false,
            last_dir: [0.0; 3],
            last_who: 0.0,
            last_uv: [0.0; 2],
            sound_at: 0.0,
            walkable: false,
            log: Vec::new(),
        }
    }
}
impl Default for PaneState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneFrame, src/game/glassbreak.ts:583)
#[derive(Clone, Copy, Debug)]
pub struct PaneFrame {
    pub key: f64,
    pub u_axis: f64,
    pub hu: f64,
    pub hv: f64,
    pub ht: f64,
    pub box_: BoxId,
}
impl PaneFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneFrame {
            key: 0.0,
            u_axis: 0.0,
            hu: 0.0,
            hv: 0.0,
            ht: 0.0,
            box_: BoxId(0),
        }
    }
}
impl Default for PaneFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassbreak.ts:121 GLASSBREAK_DEFAULTS, src/game/glassbreak.ts:121)
#[derive(Clone, Copy, Debug)]
pub struct GLASSBREAKDEFAULTS {
    pub break_e: crate::glassbreak::GLASSBREAKDEFAULTSBreakE,
    pub hang_area: f64,
    pub blast_m: f64,
    pub blast_run_k: f64,
    pub blast_dice_k: f64,
    pub blast_sheet_k: f64,
    pub rake_open: f64,
}
impl GLASSBREAKDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GLASSBREAKDEFAULTS {
            break_e: crate::glassbreak::GLASSBREAKDEFAULTSBreakE::zeroed(),
            hang_area: 0.0,
            blast_m: 0.0,
            blast_run_k: 0.0,
            blast_dice_k: 0.0,
            blast_sheet_k: 0.0,
            rake_open: 0.0,
        }
    }
}
impl Default for GLASSBREAKDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassbreak.ts:121 GLASSBREAKDEFAULTS.breakE, src/game/glassbreak.ts:121)
#[derive(Clone, Copy, Debug)]
pub struct GLASSBREAKDEFAULTSBreakE {
    pub annealed: f64,
    pub tempered: f64,
}
impl GLASSBREAKDEFAULTSBreakE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GLASSBREAKDEFAULTSBreakE {
            annealed: 0.0,
            tempered: 0.0,
        }
    }
}
impl Default for GLASSBREAKDEFAULTSBreakE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassbreak.ts:193 GB_HOST, src/game/glassbreak.ts:193)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct GlassbreakModGBHOST {
    pub panes: f64,
    pub impacts: f64,
    pub sites: f64,
    pub runs: f64,
    pub dropped: f64,
    pub shards: f64,
    pub chips: f64,
    pub refused: f64,
    pub diced: f64,
    pub clumps: f64,
    pub sheets: f64,
    pub blasts: f64,
    pub thinks: f64,
    pub hooked: f64,
    pub raked: f64,
}
impl GlassbreakModGBHOST {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassbreakModGBHOST {
            panes: 0.0,
            impacts: 0.0,
            sites: 0.0,
            runs: 0.0,
            dropped: 0.0,
            shards: 0.0,
            chips: 0.0,
            refused: 0.0,
            diced: 0.0,
            clumps: 0.0,
            sheets: 0.0,
            blasts: 0.0,
            thinks: 0.0,
            hooked: 0.0,
            raked: 0.0,
        }
    }
}
impl Default for GlassbreakModGBHOST { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassbreak.ts:195 TALLY_AT, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct TALLYAT {
    pub panes: Option<f64>,
    pub dropped: Option<f64>,
    pub impacts: Option<f64>,
    pub sites: Option<f64>,
    pub runs: Option<f64>,
    pub shards: Option<f64>,
    pub chips: Option<f64>,
    pub refused: Option<f64>,
    pub diced: Option<f64>,
    pub clumps: Option<f64>,
    pub sheets: Option<f64>,
    pub blasts: Option<f64>,
    pub thinks: Option<f64>,
    pub hooked: Option<f64>,
    pub raked: Option<f64>,
}
impl TALLYAT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TALLYAT {
            panes: None,
            dropped: None,
            impacts: None,
            sites: None,
            runs: None,
            shards: None,
            chips: None,
            refused: None,
            diced: None,
            clumps: None,
            sheets: None,
            blasts: None,
            thinks: None,
            hooked: None,
            raked: None,
        }
    }
}
impl Default for TALLYAT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassbreak.ts:293 letGo(f), src/game/glassbreak.ts:293 letGo(f))
#[derive(Clone, Debug)]
pub struct GlassBreakLetGoF {
    pub pane: f64,
    pub type_: crate::glasspen::PaneType,
    pub kind: crate::glassbreak::GlassBreakLetGoFKind,
    pub outline: Vec<V3>,
    pub centre: V3,
    pub normal: V3,
    pub thickness: f64,
    pub area: f64,
    pub velocity: V3,
    pub spin: V3,
    pub tint: V3,
    pub grain: f64,
    pub cause: crate::glassbreak::GlassBreakLetGoFCause,
    pub who: f64,
    pub seed: f64,
    pub fall: Option<crate::glassbreak::GlassFragmentFall>,
    pub landed: Option<bool>,
    pub key: f64,
    pub heard: bool,
    pub bi: f64,
}
impl GlassBreakLetGoF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassBreakLetGoF {
            pane: 0.0,
            type_: crate::glasspen::PaneType::Annealed,
            kind: crate::glassbreak::GlassBreakLetGoFKind::Piece,
            outline: Vec::new(),
            centre: [0.0; 3],
            normal: [0.0; 3],
            thickness: 0.0,
            area: 0.0,
            velocity: [0.0; 3],
            spin: [0.0; 3],
            tint: [0.0; 3],
            grain: 0.0,
            cause: crate::glassbreak::GlassBreakLetGoFCause::Blast,
            who: 0.0,
            seed: 0.0,
            fall: None,
            landed: None,
            key: 0.0,
            heard: false,
            bi: 0.0,
        }
    }
}
impl Default for GlassBreakLetGoF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassBreakLetGoFKind {
    Piece,
    Sheet,
    Dagger,
    Clump,
}
impl GlassBreakLetGoFKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassBreakLetGoFKind::Piece => "piece",
            GlassBreakLetGoFKind::Sheet => "sheet",
            GlassBreakLetGoFKind::Dagger => "dagger",
            GlassBreakLetGoFKind::Clump => "clump",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassBreakLetGoFCause {
    Blast,
    Hit,
    Dice,
    Support,
    Rake,
}
impl GlassBreakLetGoFCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassBreakLetGoFCause::Blast => "blast",
            GlassBreakLetGoFCause::Hit => "hit",
            GlassBreakLetGoFCause::Dice => "dice",
            GlassBreakLetGoFCause::Support => "support",
            GlassBreakLetGoFCause::Rake => "rake",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassBreakSeeThroughView {
    Blown,
    Wary,
    Counted,
}
impl GlassBreakSeeThroughView {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassBreakSeeThroughView::Blown => "blown",
            GlassBreakSeeThroughView::Wary => "wary",
            GlassBreakSeeThroughView::Counted => "counted",
        }
    }
}

/// (TypeScript: inline at src/game/glassbreak.ts:465 paneInfo ->, src/game/glassbreak.ts:465)
#[derive(Clone, Copy, Debug)]
pub struct GlassBreakPaneInfo {
    pub type_: crate::glasspen::PaneType,
    pub thickness: f64,
    pub exterior: bool,
    pub open_fraction: f64,
    pub walkable: bool,
    pub gone: bool,
    pub run: bool,
    pub sites: f64,
    pub energy: f64,
}
impl GlassBreakPaneInfo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassBreakPaneInfo {
            type_: crate::glasspen::PaneType::Annealed,
            thickness: 0.0,
            exterior: false,
            open_fraction: 0.0,
            walkable: false,
            gone: false,
            run: false,
            sites: 0.0,
            energy: 0.0,
        }
    }
}
impl Default for GlassBreakPaneInfo { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct GlassbreakMod {
    /// (src/game/glassbreak.ts:193 GB_HOST)
    pub gb_host: crate::glassbreak::GlassbreakModGBHOST,
    /// (src/game/glassbreak.ts:211 F)
    /// (scratch, read by nobody who did not just write it: in no bytes)
    #[bytes(skip)]
    pub f: Option<Vec<f64>>,
}
impl GlassbreakMod {
    pub fn blank() -> Self {
        GlassbreakMod {
            gb_host: crate::glassbreak::GlassbreakModGBHOST { panes: 0.0, impacts: 0.0, sites: 0.0, runs: 0.0, dropped: 0.0, shards: 0.0, chips: 0.0, refused: 0.0, diced: 0.0, clumps: 0.0, sheets: 0.0, blasts: 0.0, thinks: 0.0, hooked: 0.0, raked: 0.0 },
            f: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct GlassbreakStats {
    pub panes: i32,
    pub impacts: i32,
    pub sites: i32,
    pub runs: i32,
    pub dropped: i32,
    pub shards: i32,
    pub chips: i32,
    pub refused: i32,
    pub diced: i32,
    pub clumps: i32,
    pub sheets: i32,
    pub blasts: i32,
    pub thinks: i32,
    pub hooked: i32,
    pub raked: i32,
}

/// (src/game/glassbreak.ts:121 GLASSBREAK_DEFAULTS: a table nothing writes)
pub fn glassbreak_defaults() -> crate::glassbreak::GLASSBREAKDEFAULTS { crate::glassbreak::GLASSBREAKDEFAULTS { break_e: crate::glassbreak::GLASSBREAKDEFAULTSBreakE { annealed: 3.0, tempered: 1.0 }, hang_area: 0.12, blast_m: 2.6, blast_run_k: 0.2, blast_dice_k: 0.45, blast_sheet_k: 0.75, rake_open: 0.5 } }

/// (src/game/glassbreak.ts:123 GB_KEPT: a table nothing writes)
pub fn gb_kept() -> [String; 4] { [String::from("blastRunK"), String::from("blastDiceK"), String::from("blastSheetK"), String::from("rakeOpen")] }

/// (src/game/glassbreak.ts:124 LS_KEY)
pub const LS_KEY: &str = "twopointsix.glassfracture.on";

/// (src/game/glassbreak.ts:124 LS_KNOBS)
pub const LS_KNOBS: &str = "twopointsix.glassfracture";

/// (src/game/glassbreak.ts:195 TALLY_AT: a table nothing writes)
pub fn tally_at() -> crate::glassbreak::TALLYAT { crate::glassbreak::TALLYAT { panes: Some(0.0), dropped: Some(4.0), impacts: Some(1.0), sites: Some(2.0), runs: Some(3.0), shards: None, chips: None, refused: None, diced: Some(5.0), clumps: Some(6.0), sheets: Some(7.0), blasts: Some(8.0), thinks: Some(9.0), hooked: None, raked: Some(10.0) } }

/// (src/game/glassbreak.ts:200 GB_HEADER_ROWS)
pub const GB_HEADER_ROWS: f64 = 4.0;

/// (src/game/glassbreak.ts:200 GB_SITE_ROWS)
pub const GB_SITE_ROWS: f64 = 4.0;

/// (src/game/glassbreak.ts:200 GB_RECORD_ROWS)
pub const GB_RECORD_ROWS: f64 = 36.0;

/// (src/game/glassbreak.ts:204 TYPES: a table nothing writes)
pub fn types() -> Vec<String> { alloc::vec![String::from("annealed"), String::from("tempered"), String::from("laminated")] }

/// (src/game/glassbreak.ts:205 CAUSES: a table nothing writes)
pub fn causes() -> Vec<String> { alloc::vec![String::from("hit"), String::from("support"), String::from("blast"), String::from("dice"), String::from("rake")] }

/// (src/game/glassbreak.ts:206 KINDS: a table nothing writes)
pub fn kinds() -> Vec<String> { alloc::vec![String::from("piece"), String::from("dagger"), String::from("clump"), String::from("sheet")] }

/// (src/game/glassbreak.ts:207 NO_TYPE)
pub const NO_TYPE: f64 = 9.0;

/// (src/game/glassbreak.ts:207 NO_OVER)
pub const NO_OVER: f64 = 3.0;

/// (src/game/glassbreak.ts:207 SKY_UNKNOWN)
pub const SKY_UNKNOWN: f64 = 2.0;

/// (src/game/glassbreak.ts:209 OUT_LEN)
pub const OUT_LEN: f64 = 32.0;

/// (src/game/glassbreak.ts:209 O_PANE)
pub const O_PANE: f64 = 0.0;

/// (src/game/glassbreak.ts:209 O_TYPE)
pub const O_TYPE: f64 = 1.0;

/// (src/game/glassbreak.ts:209 O_KIND)
pub const O_KIND: f64 = 2.0;

/// (src/game/glassbreak.ts:209 O_PTS)
pub const O_PTS: f64 = 3.0;

/// (src/game/glassbreak.ts:209 O_NPTS)
pub const O_NPTS: f64 = 4.0;

/// (src/game/glassbreak.ts:209 O_CX)
pub const O_CX: f64 = 5.0;

/// (src/game/glassbreak.ts:209 O_NX)
pub const O_NX: f64 = 8.0;

/// (src/game/glassbreak.ts:209 O_THICK)
pub const O_THICK: f64 = 11.0;

/// (src/game/glassbreak.ts:209 O_AREA)
pub const O_AREA: f64 = 12.0;

/// (src/game/glassbreak.ts:209 O_VX)
pub const O_VX: f64 = 13.0;

/// (src/game/glassbreak.ts:209 O_SX)
pub const O_SX: f64 = 16.0;

/// (src/game/glassbreak.ts:209 O_GRAIN)
pub const O_GRAIN: f64 = 19.0;

/// (src/game/glassbreak.ts:209 O_CAUSE)
pub const O_CAUSE: f64 = 20.0;

/// (src/game/glassbreak.ts:209 O_WHO)
pub const O_WHO: f64 = 21.0;

/// (src/game/glassbreak.ts:209 O_SEED)
pub const O_SEED: f64 = 22.0;

/// (src/game/glassbreak.ts:209 O_KEY)
pub const O_KEY: f64 = 23.0;

/// (src/game/glassbreak.ts:209 O_HEARD)
pub const O_HEARD: f64 = 24.0;

/// (src/game/glassbreak.ts:209 O_FALL_T)
pub const O_FALL_T: f64 = 25.0;

/// (src/game/glassbreak.ts:209 O_FALL_X)
pub const O_FALL_X: f64 = 26.0;

/// (src/game/glassbreak.ts:209 O_LANDED)
pub const O_LANDED: f64 = 29.0;

/// (src/game/glassbreak.ts:212 FIELDS)
pub const FIELDS: f64 = 32.0;

/// (src/game/glassbreak.ts:213 F_D)
pub const F_D: f64 = 0.0;

/// (src/game/glassbreak.ts:213 F_W)
pub const F_W: f64 = 1.0;

/// (src/game/glassbreak.ts:213 F_E)
pub const F_E: f64 = 2.0;

/// (src/game/glassbreak.ts:213 F_HITS)
pub const F_HITS: f64 = 3.0;

/// (src/game/glassbreak.ts:213 F_C)
pub const F_C: f64 = 4.0;

/// (src/game/glassbreak.ts:213 F_HU)
pub const F_HU: f64 = 5.0;

/// (src/game/glassbreak.ts:213 F_HV)
pub const F_HV: f64 = 6.0;

/// (src/game/glassbreak.ts:213 F_HT)
pub const F_HT: f64 = 7.0;

/// (src/game/glassbreak.ts:213 F_ENERGY)
pub const F_ENERGY: f64 = 8.0;

/// (src/game/glassbreak.ts:213 F_DAMAGE)
pub const F_DAMAGE: f64 = 9.0;

/// (src/game/glassbreak.ts:213 F_NEXT)
pub const F_NEXT: f64 = 10.0;

/// (src/game/glassbreak.ts:213 F_DIR)
pub const F_DIR: f64 = 11.0;

/// (src/game/glassbreak.ts:213 F_UV)
pub const F_UV: f64 = 12.0;

/// (src/game/glassbreak.ts:213 F_SOUND)
pub const F_SOUND: f64 = 13.0;

/// (src/game/glassbreak.ts:213 F_WHO)
pub const F_WHO: f64 = 14.0;

/// (src/game/glassbreak.ts:213 F_KEY)
pub const F_KEY: f64 = 15.0;

/// (src/game/glassbreak.ts:213 F_TY)
pub const F_TY: f64 = 16.0;

/// (src/game/glassbreak.ts:213 F_AXIS)
pub const F_AXIS: f64 = 17.0;

/// (src/game/glassbreak.ts:213 F_EXT)
pub const F_EXT: f64 = 18.0;

/// (src/game/glassbreak.ts:213 F_HITN)
pub const F_HITN: f64 = 19.0;

/// (src/game/glassbreak.ts:213 F_NOPEN)
pub const F_NOPEN: f64 = 20.0;

/// (src/game/glassbreak.ts:213 F_DNEXT)
pub const F_DNEXT: f64 = 21.0;

/// (src/game/glassbreak.ts:213 F_WALK)
pub const F_WALK: f64 = 22.0;

/// (src/game/glassbreak.ts:213 F_Q0)
pub const F_Q0: f64 = 23.0;

/// (src/game/glassbreak.ts:213 F_QN)
pub const F_QN: f64 = 24.0;

/// (src/game/glassbreak.ts:213 F_QLEN)
pub const F_QLEN: f64 = 26.0;

/// (src/game/glassbreak.ts:213 F_QAT)
pub const F_QAT: f64 = 27.0;

/// (src/game/glassbreak.ts:213 F_QFROM)
pub const F_QFROM: f64 = 28.0;

/// (src/game/glassbreak.ts:213 F_QK)
pub const F_QK: f64 = 29.0;

/// (src/game/glassbreak.ts:213 F_QFID)
pub const F_QFID: f64 = 30.0;

/// (src/game/glassbreak.ts:213 F_QCAUSE)
pub const F_QCAUSE: f64 = 31.0;

/// (src/game/glassbreak.ts:216 GW_COLS)
pub const GW_COLS: f64 = 2.0;

/// (src/game/glassbreak.ts:216 GW_ROWS)
pub const GW_ROWS: f64 = 3.0;

/// (src/game/glassbreak.ts:216 GW_DSEED)
pub const GW_DSEED: f64 = 4.0;

/// (src/game/glassbreak.ts:216 GW_N)
pub const GW_N: f64 = 5.0;

/// (src/game/glassbreak.ts:216 GW_SEED)
pub const GW_SEED: f64 = 13.0;

/// (src/game/glassbreak.ts:216 GW_OPEN)
pub const GW_OPEN: f64 = 21.0;

/// (src/game/glassbreak.ts:216 GW_FLAGS)
pub const GW_FLAGS: f64 = 1.0;

/// (src/game/glassbreak.ts:216 G_GONE)
pub const G_GONE: f64 = 1.0;

/// (src/game/glassbreak.ts:216 G_DICE)
pub const G_DICE: f64 = 2.0;

/// (src/game/glassbreak.ts:216 G_RUN)
pub const G_RUN: f64 = 4.0;

/// (src/game/glassbreak.ts:216 G_SITE)
pub const G_SITE: f64 = 9.0;

/// (src/game/glassbreak.ts:216 G_DICE_AT)
pub const G_DICE_AT: f64 = 72.0;
