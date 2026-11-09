//! The types of `destruction`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// which CHAOS row a rate cap reads, live at each take: puffsPerSec, papersPerSec, knocksPerSec, decalsPerSec
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum RateCapN {
    Puff,
    Paper,
    Knock,
    Decal,
}
/// who is told of a blast, in the order they subscribed, before the doors and walls are judged: Glass adds its count to ev.res.panes, Bars, Snow
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum BlastListener {
    Glass,
    Bars,
    Snow,
}

#[derive(Clone, Debug, ToBytes)]
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
    /// what a new one holds
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Destructible {
    pub i: f64,
    pub kind: crate::destruction::DKind,
    pub box_: BoxId,
    pub mate: Option<f64>,
    pub target: Option<FixtureId>,
    pub group: Option<Vec<f64>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DKind {
    #[default] Screen,
    Paper,
    Housing,
    Clutter,
    Binder,
    Tv,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DestructionCaps {
    pub puff: crate::destruction::RateCap,
    pub paper: crate::destruction::RateCap,
    pub knock: crate::destruction::RateCap,
    pub decal: crate::destruction::RateCap,
}

#[derive(Clone, Debug, ToBytes)]
pub struct RateCap {
    pub ts: Vec<f64>,
    pub head: f64,
    pub n: crate::destruction::RateCapN,
}
impl RateCap {
    /// what a new one holds
    pub fn blank() -> Self {
        RateCap {
            ts: Vec::new(),
            head: 0.0,
            n: crate::destruction::RateCapN::Puff,
        }
    }
}
impl Default for RateCap { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DestructionDustQueueItem {
    pub p: V3,
    pub n: V3,
    pub scale: f64,
    pub surf: crate::scene_materials::Surf,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LaneMark {
    pub look: f64,
    pub r: [f64; 2],
    pub s: [f64; 2],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DestructionTakeK {
    Paper,
    Puff,
    Knock,
    Decal,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DestructionRoundHit {
    pub n: V3,
    pub index: Option<f64>,
    pub prop: Option<PropId>,
    pub box_: Option<BoxId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DestructionDoorsForOpts {
    pub radius: Option<f64>,
    pub power: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DestructionWallsForOpts {
    pub radius: Option<f64>,
    pub power: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DestructionBurstHit {
    pub index: Option<f64>,
    pub prop: Option<PropId>,
    pub box_: Option<BoxId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DestructionPallEmitR {
    pub radius: f64,
    pub density: f64,
    pub ttl: f64,
    pub speed: f64,
    pub temp: f64,
    pub push: f64,
    pub expand: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct DestructionMod {
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

pub fn pall_dirs() -> Vec<[f64; 2]> { alloc::vec![[1.0, 0.0], [0.7071, 0.7071], [0.0, 1.0], [-0.7071, 0.7071], [-1.0, 0.0], [-0.7071, -0.7071], [0.0, -1.0], [0.7071, -0.7071]] }

pub fn lane_mark() -> JsMap<String, crate::destruction::LaneMark> { { let mut t = JsMap::new(); t.set(String::from("dent"), crate::destruction::LaneMark { look: 0.0, r: [0.036, 0.045], s: [0.7, 0.9] }); t.set(String::from("shred"), crate::destruction::LaneMark { look: 1.0, r: [0.06, 0.075], s: [0.85, 0.85] }); t.set(String::from("crack"), crate::destruction::LaneMark { look: 2.0, r: [0.085, 0.11], s: [0.9, 0.9] }); t } }

pub fn roof_rays() -> Vec<[f64; 2]> { alloc::vec![[0.0, 0.0], [0.29, 0.0], [-0.29, 0.0], [0.0, 0.29], [0.0, -0.29]] }
