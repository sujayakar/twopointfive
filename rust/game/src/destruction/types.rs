//! The types of `destruction`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// which CHAOS row a rate cap reads, live at each take (destruction.ts:235): puffsPerSec, papersPerSec, knocksPerSec, decalsPerSec
#[derive(Clone, Debug)]
pub enum RateCapN {
    Puff,
    Paper,
    Knock,
    Decal,
}
/// who is told of a blast, in the order they subscribed, before the doors and walls are judged: Glass adds its count to ev.res.panes (glassbreak.ts:219), Bars (winbars.ts:80), Snow (snowBlast.ts:119)
#[derive(Clone, Debug)]
pub enum BlastListener {
    Glass,
    Bars,
    Snow,
}

/// (TypeScript: Destruction, src/game/destruction.ts:207)
#[derive(Clone, Debug)]
pub struct Destruction {
    pub debris: crate::debris::Debris,
    pub walls: alloc::boxed::Box<crate::wallbreak::WallBreak>,
    pub edge_look: crate::edge_look::EdgeLook,
    pub furn: crate::furnblast::Furniture,
    pub breaks: crate::break_table::Breaks,
    pub show_chaos: Option<f64>,
    pub heat: Vec<f32>,
    pub hw: f64,
    pub hh: f64,
    pub items: JsMap<crate::Num, crate::destruction::Destructible>,
    pub cells: Vec<Vec<f64>>,
    pub hidden: JsMap<crate::Num, f64>,
    pub tiles_down: JsSet<String>,
    pub last_tile_t: f64,
    pub tile_count: f64,
    pub broke: JsSet<FixtureId>,
    pub fixture_count: f64,
    pub last_fixture_t: f64,
    pub roofs: JsMap<crate::Num, f64>,
    pub target_of_box: JsMap<crate::Num, FixtureId>,
    pub gather_scratch: Vec<i32>,
    pub caps: crate::destruction::DestructionCaps,
    pub puffs_this_frame: f64,
    pub free_knocks: f64,
    pub serial: f64,
    pub w: f64,
    pub cs: f64,
    pub knocks: f64,
    pub any_heat: bool,
    pub thuds_this_frame: f64,
    pub dust_queue: Vec<crate::destruction::DestructionDustQueueItem>,
    pub blast_listeners: Vec<crate::destruction::BlastListener>,
    pub pall_at: Vec<f64>,
}
impl Destruction {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Destruction {
            debris: crate::debris::Debris::blank(),
            walls: alloc::boxed::Box::new(crate::wallbreak::WallBreak::blank()),
            edge_look: crate::edge_look::EdgeLook::blank(),
            furn: crate::furnblast::Furniture::blank(),
            breaks: crate::break_table::Breaks::blank(),
            show_chaos: None,
            heat: Vec::new(),
            hw: 0.0,
            hh: 0.0,
            items: JsMap::new(),
            cells: Vec::new(),
            hidden: JsMap::new(),
            tiles_down: JsSet::new(),
            last_tile_t: -100.0,
            tile_count: 0.0,
            broke: JsSet::new(),
            fixture_count: 0.0,
            last_fixture_t: -100.0,
            roofs: JsMap::new(),
            target_of_box: JsMap::new(),
            gather_scratch: alloc::vec![0; 32],
            caps: crate::destruction::DestructionCaps { puff: crate::destruction::RateCap { ts: alloc::vec::Vec::new(), head: 0.0, n: crate::destruction::RateCapN::Puff }, paper: crate::destruction::RateCap { ts: alloc::vec::Vec::new(), head: 0.0, n: crate::destruction::RateCapN::Paper }, knock: crate::destruction::RateCap { ts: alloc::vec::Vec::new(), head: 0.0, n: crate::destruction::RateCapN::Knock }, decal: crate::destruction::RateCap { ts: alloc::vec::Vec::new(), head: 0.0, n: crate::destruction::RateCapN::Decal } },
            puffs_this_frame: 0.0,
            free_knocks: 0.0,
            serial: 1.0,
            w: 40.0,
            cs: 1.0,
            knocks: 0.0,
            any_heat: false,
            thuds_this_frame: 0.0,
            dust_queue: Vec::new(),
            blast_listeners: Vec::new(),
            pall_at: Vec::new(),
        }
    }
}
impl Default for Destruction { fn default() -> Self { Self::blank() } }

/// (TypeScript: Destructible, src/game/destruction.ts:178)
#[derive(Clone, Debug)]
pub struct Destructible {
    pub i: f64,
    pub kind: crate::destruction::DKind,
    pub box_: BoxId,
    pub flags0: f64,
    pub mate: Option<f64>,
    pub target: Option<FixtureId>,
    pub group: Option<Vec<f64>>,
}
impl Destructible {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Destructible {
            i: 0.0,
            kind: crate::destruction::DKind::Screen,
            box_: BoxId(0),
            flags0: 0.0,
            mate: None,
            target: None,
            group: None,
        }
    }
}
impl Default for Destructible { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DKind {
    Screen,
    Paper,
    Housing,
    Clutter,
    Binder,
    Tv,
}
impl DKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DKind::Screen => "screen",
            DKind::Paper => "paper",
            DKind::Housing => "housing",
            DKind::Clutter => "clutter",
            DKind::Binder => "binder",
            DKind::Tv => "tv",
        }
    }
}

/// (TypeScript: inline at src/game/destruction.ts:235 Destruction.caps, src/game/destruction.ts:235)
#[derive(Clone, Debug)]
pub struct DestructionCaps {
    pub puff: crate::destruction::RateCap,
    pub paper: crate::destruction::RateCap,
    pub knock: crate::destruction::RateCap,
    pub decal: crate::destruction::RateCap,
}
impl DestructionCaps {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DestructionCaps {
            puff: crate::destruction::RateCap::blank(),
            paper: crate::destruction::RateCap::blank(),
            knock: crate::destruction::RateCap::blank(),
            decal: crate::destruction::RateCap::blank(),
        }
    }
}
impl Default for DestructionCaps { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RateCap, src/game/destruction.ts:195)
#[derive(Clone, Debug)]
pub struct RateCap {
    pub ts: Vec<f64>,
    pub head: f64,
    pub n: crate::destruction::RateCapN,
}
impl RateCap {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        RateCap {
            ts: Vec::new(),
            head: 0.0,
            n: crate::destruction::RateCapN::Puff,
        }
    }
}
impl Default for RateCap { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/destruction.ts:370 Destruction.dustQueue, src/game/destruction.ts:370)
#[derive(Clone, Copy, Debug)]
pub struct DestructionDustQueueItem {
    pub p: V3,
    pub n: V3,
    pub scale: f64,
    pub surf: crate::scene_materials::Surf,
    pub t: f64,
}
impl DestructionDustQueueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DestructionDustQueueItem {
            p: [0.0; 3],
            n: [0.0; 3],
            scale: 0.0,
            surf: crate::scene_materials::Surf::Carpet,
            t: 0.0,
        }
    }
}
impl Default for DestructionDustQueueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ChaosTune, src/game/destruction.ts:155)
#[derive(Clone, Copy, Debug)]
pub struct ChaosTune {
    pub chaos: Option<f64>,
    pub debris_impulses: Option<bool>,
}
impl ChaosTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChaosTune {
            chaos: None,
            debris_impulses: None,
        }
    }
}
impl Default for ChaosTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LaneMark, src/game/destruction.ts:186)
#[derive(Clone, Copy, Debug)]
pub struct LaneMark {
    pub look: f64,
    pub r: [f64; 2],
    pub s: [f64; 2],
}
impl LaneMark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaneMark {
            look: 0.0,
            r: [0.0; 2],
            s: [0.0; 2],
        }
    }
}
impl Default for LaneMark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DestructionTakeK {
    Paper,
    Puff,
    Knock,
    Decal,
}
impl DestructionTakeK {
    pub const fn as_str(self) -> &'static str {
        match self {
            DestructionTakeK::Paper => "paper",
            DestructionTakeK::Puff => "puff",
            DestructionTakeK::Knock => "knock",
            DestructionTakeK::Decal => "decal",
        }
    }
}

/// (TypeScript: inline at src/game/destruction.ts:391 round(hit), src/game/destruction.ts:391)
#[derive(Clone, Copy, Debug)]
pub struct DestructionRoundHit {
    pub n: V3,
    pub index: Option<f64>,
    pub prop: Option<PropId>,
    pub box_: Option<BoxId>,
}
impl DestructionRoundHit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DestructionRoundHit {
            n: [0.0; 3],
            index: None,
            prop: None,
            box_: None,
        }
    }
}
impl Default for DestructionRoundHit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:458 doorsFor(opts), src/game/destruction.ts:458)
#[derive(Clone, Copy, Debug)]
pub struct DestructionDoorsForOpts {
    pub radius: Option<f64>,
    pub power: Option<f64>,
}
impl DestructionDoorsForOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DestructionDoorsForOpts {
            radius: None,
            power: None,
        }
    }
}
impl Default for DestructionDoorsForOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:468 wallsFor(opts), src/game/destruction.ts:468)
#[derive(Clone, Copy, Debug)]
pub struct DestructionWallsForOpts {
    pub radius: Option<f64>,
    pub power: Option<f64>,
}
impl DestructionWallsForOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DestructionWallsForOpts {
            radius: None,
            power: None,
        }
    }
}
impl Default for DestructionWallsForOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DestructionTestImpulseKind {
    Bang,
    Thud,
    Impact,
}
impl DestructionTestImpulseKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DestructionTestImpulseKind::Bang => "bang",
            DestructionTestImpulseKind::Thud => "thud",
            DestructionTestImpulseKind::Impact => "impact",
        }
    }
}

/// (TypeScript: inline at src/game/destruction.ts:663 burst(hit), src/game/destruction.ts:663)
#[derive(Clone, Copy, Debug)]
pub struct DestructionBurstHit {
    pub index: Option<f64>,
    pub prop: Option<PropId>,
    pub box_: Option<BoxId>,
}
impl DestructionBurstHit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DestructionBurstHit {
            index: None,
            prop: None,
            box_: None,
        }
    }
}
impl Default for DestructionBurstHit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/destruction.ts:724 pallEmit(R), src/game/destruction.ts:724)
#[derive(Clone, Copy, Debug)]
pub struct DestructionPallEmitR {
    pub radius: f64,
    pub density: f64,
    pub ttl: f64,
    pub speed: f64,
    pub temp: f64,
    pub push: f64,
    pub expand: f64,
}
impl DestructionPallEmitR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DestructionPallEmitR {
            radius: 0.0,
            density: 0.0,
            ttl: 0.0,
            speed: 0.0,
            temp: 0.0,
            push: 0.0,
            expand: 0.0,
        }
    }
}
impl Default for DestructionPallEmitR { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DestructionMod {
    /// (src/game/destruction.ts:189 DESTR_LOG)
    pub destr_log: Vec<String>,
}
impl DestructionMod {
    pub fn blank() -> Self {
        DestructionMod {
            destr_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DestructionStats {
    pub pall_palls: i32,
    pub pall_emitters: i32,
    pub pall_refused: i32,
    pub pall_starved: i32,
    pub pall_shut: i32,
    pub pall_dust: i32,
    pub destr_rounds: i32,
    pub destr_bangs: i32,
    pub destr_decals: i32,
    pub destr_papers: i32,
    pub destr_sheets: i32,
    pub destr_puffs: i32,
    pub destr_puffs_refused: i32,
    pub destr_bursts: i32,
    pub destr_knocks: i32,
    pub destr_knocks_demoted: i32,
    pub destr_screens: i32,
    pub destr_tiles: i32,
    pub destr_tiles_refused: i32,
    pub destr_chair_spins: i32,
    pub destr_imp_bang: i32,
    pub destr_imp_impact: i32,
    pub destr_imp_thud: i32,
    pub destr_imp_thud_dropped: i32,
    pub destr_debris_kicked: i32,
    pub destr_fixtures: i32,
    pub destr_fixtures_refused: i32,
}

/// (src/game/destruction.ts:152 PALL_DIRS: a table nothing writes)
pub fn pall_dirs() -> Vec<[f64; 2]> { alloc::vec![[1.0, 0.0], [0.7071, 0.7071], [0.0, 1.0], [-0.7071, 0.7071], [-1.0, 0.0], [-0.7071, -0.7071], [0.0, -1.0], [0.7071, -0.7071]] }

/// (src/game/destruction.ts:153 CHAOS_DEFAULTS: a table nothing writes)
pub fn chaos_defaults() -> crate::knobs::ChaosK { crate::knobs::page_chaos() }

/// (src/game/destruction.ts:154 LS_KEY)
pub const LS_KEY: &str = "twopointsix.chaos";

/// (src/game/destruction.ts:187 LANE_MARK: a table nothing writes)
pub fn lane_mark() -> JsMap<String, crate::destruction::LaneMark> { { let mut t = JsMap::new(); t.set(String::from("dent"), crate::destruction::LaneMark { look: 0.0, r: [0.036, 0.045], s: [0.7, 0.9] }); t.set(String::from("shred"), crate::destruction::LaneMark { look: 1.0, r: [0.06, 0.075], s: [0.85, 0.85] }); t.set(String::from("crack"), crate::destruction::LaneMark { look: 2.0, r: [0.085, 0.11], s: [0.9, 0.9] }); t } }

/// (src/game/destruction.ts:194 ROOF_RAYS: a table nothing writes)
pub fn roof_rays() -> Vec<[f64; 2]> { alloc::vec![[0.0, 0.0], [0.29, 0.0], [-0.29, 0.0], [0.0, 0.29], [0.0, -0.29]] }
