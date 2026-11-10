//! The types of `destruction`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// who is told of a blast, in the order they subscribed, before the doors and walls are judged: Glass adds its count to ev.res.panes, Bars, Snow
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum BlastListener {
    Glass,
    Bars,
    Snow,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Destruction {
    pub debris: crate::debris::Debris,
    #[is(alloc::boxed::Box::new(crate::wallbreak::WallBreak::blank()))] pub walls: alloc::boxed::Box<crate::wallbreak::WallBreak>,
    pub edge_look: crate::edge_look::EdgeLook,
    pub furn: crate::furnblast::Furniture,
    pub breaks: crate::break_table::Breaks,
    pub heat: Vec<f32>,
    pub hw: f64,
    pub hh: f64,
    pub items: JsMap<crate::Num, crate::destruction::Destructible>,
    pub cells: Vec<Vec<f64>>,
    pub hidden: JsMap<crate::Num, f64>,
    pub tiles_down: JsSet<String>,
    #[is(-100.0)] pub last_tile_t: f64,
    pub tile_count: f64,
    pub broke: JsSet<FixtureId>,
    pub fixture_count: f64,
    #[is(-100.0)] pub last_fixture_t: f64,
    pub roofs: JsMap<crate::Num, f64>,
    pub target_of_box: JsMap<crate::Num, FixtureId>,
    #[is(crate::destruction::DestructionCaps { puff: crate::destruction::RateCap { ts: alloc::vec::Vec::new(), head: 0.0, n: crate::destruction::RateCapN::Puff }, paper: crate::destruction::RateCap { ts: alloc::vec::Vec::new(), head: 0.0, n: crate::destruction::RateCapN::Paper }, knock: crate::destruction::RateCap { ts: alloc::vec::Vec::new(), head: 0.0, n: crate::destruction::RateCapN::Knock }, decal: crate::destruction::RateCap { ts: alloc::vec::Vec::new(), head: 0.0, n: crate::destruction::RateCapN::Decal } })] pub caps: crate::destruction::DestructionCaps,
    pub puffs_this_frame: f64,
    pub free_knocks: f64,
    #[is(1.0)] pub serial: f64,
    #[is(40.0)] pub w: f64,
    #[is(1.0)] pub cs: f64,
    pub knocks: f64,
    pub any_heat: bool,
    pub thuds_this_frame: f64,
    pub dust_queue: Vec<crate::destruction::DestructionDustQueueItem>,
    pub blast_listeners: Vec<crate::destruction::BlastListener>,
    pub pall_at: Vec<f64>,
}

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

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct RateCap {
    pub ts: Vec<f64>,
    pub head: f64,
    #[is(crate::destruction::RateCapN::Puff)] pub n: crate::destruction::RateCapN,
}

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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DestructionMod {
    pub destr_log: Vec<String>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DestructionStats {
    pub destr_puffs_refused: i32,
    pub destr_tiles_refused: i32,
}

pub fn pall_dirs() -> Vec<[f64; 2]> { alloc::vec![[1.0, 0.0], [0.7071, 0.7071], [0.0, 1.0], [-0.7071, 0.7071], [-1.0, 0.0], [-0.7071, -0.7071], [0.0, -1.0], [0.7071, -0.7071]] }

pub fn lane_mark() -> JsMap<String, crate::destruction::LaneMark> { { let mut t = JsMap::new(); t.set(String::from("dent"), crate::destruction::LaneMark { look: 0.0, r: [0.036, 0.045], s: [0.7, 0.9] }); t.set(String::from("shred"), crate::destruction::LaneMark { look: 1.0, r: [0.06, 0.075], s: [0.85, 0.85] }); t.set(String::from("crack"), crate::destruction::LaneMark { look: 2.0, r: [0.085, 0.11], s: [0.9, 0.9] }); t } }

pub fn roof_rays() -> Vec<[f64; 2]> { alloc::vec![[0.0, 0.0], [0.29, 0.0], [-0.29, 0.0], [0.0, 0.29], [0.0, -0.29]] }

/// which CHAOS row a rate cap reads, live at each take: puffsPerSec, papersPerSec, knocksPerSec, decalsPerSec
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum RateCapN {
    Puff,
    Paper,
    Knock,
    Decal,
}
