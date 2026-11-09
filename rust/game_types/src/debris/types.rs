//! The types of `debris`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FootBook {
    pub shoves: f64,
    pub stats: crate::debris::FootBookStats,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FootBookStats {
    pub kicked: f64,
    pub off_man: f64,
    pub walled: f64,
    pub gave_up: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Debris {
    pub list: Vec<DebrisChunkId>,
    #[is(1.0)] pub serial: f64,
    pub frame: f64,
    #[is(crate::debris::DebrisStats { spawned: 0.0, refused: 0.0, evicted: 0.0, landed: 0.0, peak_live: 0.0, peak_moving: 0.0, kicked: 0.0, told: 0.0 })] pub stats: crate::debris::DebrisStats,
    pub fling: Option<V3>,
    #[is(1.0)] pub serial0: f64,
    pub impulses: f64,
    #[is(0.3)] pub foot_mu: f64,
    pub shoves: f64,
    #[is(-1.0)] pub scuff_at: f64,
    #[is(crate::debris::DebrisFeetStats { kicked: 0.0, off_man: 0.0, walled: 0.0, gave_up: 0.0 })] pub feet_stats: crate::debris::DebrisFeetStats,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DebrisStats {
    pub spawned: f64,
    pub refused: f64,
    pub evicted: f64,
    pub landed: f64,
    pub peak_live: f64,
    pub peak_moving: f64,
    pub kicked: f64,
    pub told: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DebrisFeetStats {
    pub kicked: f64,
    pub off_man: f64,
    pub walled: f64,
    pub gave_up: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum ChunkKind {
    Tile,
    Shard,
    Housing,
    Clutter,
    Binder,
    Chunk,
}

#[derive(Clone, Debug, ToBytes)]
pub struct Chunk {
    pub kind: crate::debris::ChunkKind,
    pub box_: AnyBoxId,
    pub v: V3,
    pub w: V3,
    pub rest: bool,
    pub still_t: f64,
    pub age: f64,
    pub sounded: bool,
    pub level: f64,
    pub sfx: Vec<crate::debris::ChunkSfxItem>,
    pub serial: f64,
    pub who: f64,
    pub on: Option<PropId>,
    pub on_pose: f64,
    pub glass: Option<crate::shards::PlateMeta>,
    pub contact: Option<crate::debris::ChunkContact>,
    pub mat: Option<crate::break_table::BreakMeta>,
    pub mate: Option<BoxId>,
    pub foot: Option<bool>,
    pub foot_at: Option<f64>,
    pub foot_n: Option<f64>,
    pub foot_side: Option<f64>,
    pub dress: Option<bool>,
    pub lift: Option<f64>,
}
impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            kind: crate::debris::ChunkKind::Tile,
            box_: AnyBoxId::World(BoxId(0)),
            v: [0.0; 3],
            w: [0.0; 3],
            rest: false,
            still_t: 0.0,
            age: 0.0,
            sounded: false,
            level: 0.0,
            sfx: Vec::new(),
            serial: 0.0,
            who: 0.0,
            on: None,
            on_pose: 0.0,
            glass: None,
            contact: None,
            mat: None,
            mate: None,
            foot: None,
            foot_at: None,
            foot_n: None,
            foot_side: None,
            dress: None,
            lift: None,
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ChunkSfxItem {
    pub name: String,
    pub rate: f64,
    pub gain: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ChunkContact {
    pub keep_floor: f64,
    pub keep_wall: f64,
    pub rest: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FootMan {
    pub pos: V3,
    pub vel: V3,
    /// a place on a man down, not a man on his feet: at his floor's height, under one of his bones
    pub body: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FootKnobs {
    pub radius: f64,
    pub reach: f64,
    pub lead: f64,
    pub slide: f64,
    pub inside: f64,
    pub floor: f64,
    pub heavy: f64,
    pub walk: f64,
    pub base: f64,
    pub gain: f64,
    pub aside: f64,
    pub v_max: f64,
    pub off: f64,
    pub spin: f64,
    pub per_frame: f64,
    pub cooldown: f64,
    pub give_up: f64,
    pub give_up_secs: f64,
    pub scuff: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DEBRISIMP {
    pub gain: f64,
    pub up_k: f64,
    pub v_max: f64,
    pub wake: f64,
    pub tumble: f64,
}

#[derive(Clone, Debug, Default)]
pub struct LANDINGValue {
    pub level: f64,
    pub sfx: Vec<crate::debris::LANDINGValueSfxItem>,
}

#[derive(Clone, Debug, Default)]
pub struct LANDINGValueSfxItem {
    pub name: String,
    pub rate: f64,
    pub gain: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DebrisFootGap {
    pub gap: f64,
    pub dx: f64,
    pub dz: f64,
    pub lx: f64,
    pub lz: f64,
}

/// (src/game/debris.ts:62 G)
pub const G: f64 = 9.81;

/// (src/game/debris.ts:63 SLEEP_V)
pub const SLEEP_V: f64 = 0.22;

/// (src/game/debris.ts:63 SLEEP_W)
pub const SLEEP_W: f64 = 1.2;

/// (src/game/debris.ts:63 SLEEP_T)
pub const SLEEP_T: f64 = 0.25;

/// (src/game/debris.ts:67 DEBRIS_IMP: a table nothing writes)
pub fn debris_imp() -> crate::debris::DEBRISIMP { crate::debris::DEBRISIMP { gain: 0.35, up_k: 0.7, v_max: 3.0, wake: 0.1, tumble: 0.5 } }

/// (src/game/debris.ts:69 LANDING: a table nothing writes)
pub fn landing() -> JsMap<String, crate::debris::LANDINGValue> { { let mut t = JsMap::new(); t.set(String::from("tile"), crate::debris::LANDINGValue { level: 0.4, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 0.75, gain: 0.9 }, crate::debris::LANDINGValueSfxItem { name: String::from("magDrop"), rate: 0.5, gain: 0.35 }] }); t.set(String::from("housing"), crate::debris::LANDINGValue { level: 0.5, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 0.6, gain: 1.0 }, crate::debris::LANDINGValueSfxItem { name: String::from("lightBreak"), rate: 0.8, gain: 0.45 }] }); t.set(String::from("clutter"), crate::debris::LANDINGValue { level: 0.25, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 1.45, gain: 0.5 }] }); t.set(String::from("binder"), crate::debris::LANDINGValue { level: 0.25, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 1.1, gain: 0.55 }] }); t.set(String::from("chunk"), crate::debris::LANDINGValue { level: 0.2, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 1.3, gain: 0.4 }] }); t.set(String::from("shard"), crate::debris::LANDINGValue { level: 0.3, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("glassCrack"), rate: 1.9, gain: 0.3 }, crate::debris::LANDINGValueSfxItem { name: String::from("casingTink"), rate: 0.6, gain: 0.35 }] }); t } }
