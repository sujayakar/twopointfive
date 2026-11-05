//! The types of `glassbreak`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// who takes a fragment as it lets go (GlassBreak.onRelease; shards.ts takes and gives it back): None = the stand-in runs
#[derive(Clone, Debug)]
pub enum GlassBreakOnRelease {
    Shards,
}
/// a burst's view of the glass for one judgment (GlassBreak.seeThrough): which panes a walk also passes
#[derive(Clone, Debug)]
pub enum GlassBreakAlso {
    Blown,
    Wary { from: V3, size: crate::glassbreak::GlassBlastOpts },
    Counted { from: V3, size: crate::glassbreak::GlassBlastOpts },
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassType {
    Annealed,
    Tempered,
    Laminated,
}
impl GlassType {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassType::Annealed => "annealed",
            GlassType::Tempered => "tempered",
            GlassType::Laminated => "laminated",
        }
    }
}

/// (TypeScript: GbTune, src/game/glassbreak.ts:122)
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

/// (TypeScript: GlassImpact, src/game/glassbreak.ts:144)
#[derive(Clone, Copy, Debug)]
pub struct GlassImpact {
    pub pane: bool,
    pub live: bool,
    pub type_: crate::glassbreak::GlassType,
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
            type_: crate::glassbreak::GlassType::Annealed,
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

/// (TypeScript: GlassFragment, src/game/glassbreak.ts:156)
#[derive(Clone, Debug)]
pub struct GlassFragment {
    pub pane: f64,
    pub type_: crate::glassbreak::GlassType,
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
}
impl GlassFragment {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassFragment {
            pane: 0.0,
            type_: crate::glassbreak::GlassType::Annealed,
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

/// (TypeScript: GlassBlastOpts, src/game/glassbreak.ts:175)
#[derive(Clone, Copy, Debug)]
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

/// (TypeScript: Clump, src/game/glassbreak.ts:183)
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

/// (TypeScript: Dice, src/game/glassbreak.ts:184)
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

/// (TypeScript: Pending, src/game/glassbreak.ts:185)
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

/// (TypeScript: PaneState, src/game/glassbreak.ts:186)
#[derive(Clone, Debug)]
pub struct PaneState {
    pub bi: f64,
    pub box_: BoxId,
    pub key: f64,
    pub type_: crate::glassbreak::GlassType,
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
            type_: crate::glassbreak::GlassType::Annealed,
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

/// (TypeScript: GlassBreak, src/game/glassbreak.ts:200)
#[derive(Clone, Debug)]
pub struct GlassBreak {
    pub panes: JsMap<crate::Num, crate::glassbreak::PaneState>,
    pub log: Vec<String>,
    pub on_release: Option<crate::glassbreak::GlassBreakOnRelease>,
    pub types: JsMap<crate::Num, crate::glassbreak::GlassType>,
    pub ext: JsMap<crate::Num, f64>,
    pub blasted: Vec<crate::glassbreak::GlassBreakBlastedItem>,
    pub released_this_frame: f64,
    pub also: Option<crate::glassbreak::GlassBreakAlso>,
    pub view_of: Option<crate::glassbreak::GlassBreakViewOf>,
    pub gust_dirty: bool,
    pub gusts: JsMap<crate::Num, crate::wind::WindSource>,
}
impl GlassBreak {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        GlassBreak {
            panes: JsMap::new(),
            log: Vec::new(),
            on_release: None,
            types: JsMap::new(),
            ext: JsMap::new(),
            blasted: Vec::new(),
            released_this_frame: 0.0,
            also: None,
            view_of: None,
            gust_dirty: false,
            gusts: JsMap::new(),
        }
    }
}
impl Default for GlassBreak { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/glassbreak.ts:207 GlassBreak.blasted, src/game/glassbreak.ts:207)
#[derive(Clone, Debug)]
pub struct GlassBreakBlastedItem {
    pub t: f64,
    pub at: V3,
    pub k: JsMap<crate::Num, f64>,
}
impl GlassBreakBlastedItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassBreakBlastedItem {
            t: 0.0,
            at: [0.0; 3],
            k: JsMap::new(),
        }
    }
}
impl Default for GlassBreakBlastedItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassbreak.ts:477 GlassBreak.viewOf, src/game/glassbreak.ts:477)
#[derive(Clone, Debug)]
pub struct GlassBreakViewOf {
    pub kind: crate::glassbreak::GlassBreakViewOfKind,
    pub from: V3,
    pub size: crate::glassbreak::GlassBlastOpts,
    pub rows: Option<crate::scene_box_mirror::PaneView>,
}
impl GlassBreakViewOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassBreakViewOf {
            kind: crate::glassbreak::GlassBreakViewOfKind::Blown,
            from: [0.0; 3],
            size: crate::glassbreak::GlassBlastOpts::zeroed(),
            rows: None,
        }
    }
}
impl Default for GlassBreakViewOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: inline at src/game/glassbreak.ts:118 GLASSBREAK_DEFAULTS, src/game/glassbreak.ts:118)
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

/// (TypeScript: inline at src/game/glassbreak.ts:118 GLASSBREAKDEFAULTS.breakE, src/game/glassbreak.ts:118)
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassBreakReleaseCraterCause {
    Blast,
    Hit,
    Dice,
    Support,
    Rake,
}
impl GlassBreakReleaseCraterCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassBreakReleaseCraterCause::Blast => "blast",
            GlassBreakReleaseCraterCause::Hit => "hit",
            GlassBreakReleaseCraterCause::Dice => "dice",
            GlassBreakReleaseCraterCause::Support => "support",
            GlassBreakReleaseCraterCause::Rake => "rake",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassBreakReleaseFragCause {
    Blast,
    Hit,
    Dice,
    Support,
    Rake,
}
impl GlassBreakReleaseFragCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassBreakReleaseFragCause::Blast => "blast",
            GlassBreakReleaseFragCause::Hit => "hit",
            GlassBreakReleaseFragCause::Dice => "dice",
            GlassBreakReleaseFragCause::Support => "support",
            GlassBreakReleaseFragCause::Rake => "rake",
        }
    }
}

/// (TypeScript: inline at src/game/glassbreak.ts:814 paneInfo ->, src/game/glassbreak.ts:814)
#[derive(Clone, Copy, Debug)]
pub struct GlassBreakPaneInfo {
    pub type_: crate::glassbreak::GlassType,
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
            type_: crate::glassbreak::GlassType::Annealed,
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

/// (src/game/glassbreak.ts:118 GLASSBREAK_DEFAULTS: a table nothing writes)
pub fn glassbreak_defaults() -> crate::glassbreak::GLASSBREAKDEFAULTS { crate::glassbreak::GLASSBREAKDEFAULTS { break_e: crate::glassbreak::GLASSBREAKDEFAULTSBreakE { annealed: 3.0, tempered: 1.0 }, hang_area: 0.12, blast_m: 2.6, blast_run_k: 0.2, blast_dice_k: 0.45, blast_sheet_k: 0.75, rake_open: 0.5 } }

/// (src/game/glassbreak.ts:120 GB_KEPT: a table nothing writes)
pub fn gb_kept() -> [String; 4] { [String::from("blastRunK"), String::from("blastDiceK"), String::from("blastSheetK"), String::from("rakeOpen")] }

/// (src/game/glassbreak.ts:121 LS_KEY)
pub const LS_KEY: &str = "twopointsix.glassfracture.on";

/// (src/game/glassbreak.ts:121 LS_KNOBS)
pub const LS_KNOBS: &str = "twopointsix.glassfracture";

/// (src/game/glassbreak.ts:174 EXT_SKY_TOP)
pub const EXT_SKY_TOP: f64 = 12.0;

/// (src/game/glassbreak.ts:178 BLAST_MIN_K)
pub const BLAST_MIN_K: f64 = 0.05;

/// (src/game/glassbreak.ts:182 DICE_TIE)
pub const DICE_TIE: f64 = 0.0001;

/// (src/game/glassbreak.ts:198 GB_HEADER_ROWS)
pub const GB_HEADER_ROWS: f64 = 4.0;

/// (src/game/glassbreak.ts:198 GB_SITE_ROWS)
pub const GB_SITE_ROWS: f64 = 4.0;

/// (src/game/glassbreak.ts:198 GB_RECORD_ROWS)
pub const GB_RECORD_ROWS: f64 = 36.0;
