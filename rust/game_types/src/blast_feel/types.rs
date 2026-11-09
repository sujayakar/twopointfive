//! The types of `blast_feel`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct Face {
    pub o: V3,
    pub d: V3,
    pub mask: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Kick {
    pub t0: f64,
    pub at: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub seen: Vec<WallChargeId>,
    pub queue: Vec<crate::blast_feel::StateQueueItem>,
    pub kicks: Vec<crate::blast_feel::Kick>,
    pub serial: f64,
    pub last: f64,
    pub list: Option<u32>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateQueueItem {
    pub at: f64,
    pub run: crate::FnField,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BlastFeelModFEEL {
    pub on: bool,
    pub kick: f64,
    pub kick_m: f64,
    pub kick_secs: f64,
    pub kick_tau: f64,
    pub kick_hz: f64,
    pub kick_reach: f64,
    pub sound: crate::blast_feel::BlastFeelModFEELSound,
    pub ring: crate::blast_feel::BlastFeelModFEELRing,
    pub push: Vec<[f64; 3]>,
    pub push_up: f64,
    pub ceil: crate::blast_feel::BlastFeelModFEELCeil,
    pub fire: crate::blast_feel::BlastFeelModFEELFire,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BlastFeelModFEELSound {
    pub crack: f64,
    pub thump: f64,
    pub debris: f64,
    pub debris_after: f64,
    pub thump_after: f64,
    pub carry: f64,
    pub carry_from: f64,
    pub carry_over: f64,
    pub c: f64,
    pub late_max: f64,
    pub dull_m: f64,
    pub ring: f64,
    pub ring_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BlastFeelModFEELRing {
    pub stages: f64,
    pub sectors: f64,
    pub arc: f64,
    pub r0: f64,
    pub step: f64,
    pub every: f64,
    pub y: f64,
    pub speed: f64,
    pub life: f64,
    pub size: f64,
    pub color: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BlastFeelModFEELCeil {
    pub spots: f64,
    pub reach: f64,
    pub after: f64,
    pub speed: f64,
    pub puffs: f64,
    pub crumbs: f64,
    pub color: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BlastFeelModFEELFire {
    pub core: crate::blast_feel::BlastFeelModFEELFireCore,
    pub roll: crate::blast_feel::BlastFeelModFEELFireRoll,
    pub soot: Vec<crate::blast_feel::BlastFeelModFEELFireSootItem>,
    pub soot_color: V3,
    pub chunks: crate::blast_feel::BlastFeelModFEELFireChunks,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BlastFeelModFEELFireCore {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub life: f64,
    pub size: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BlastFeelModFEELFireRoll {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub life: f64,
    pub size: f64,
    pub after: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BlastFeelModFEELFireSootItem {
    pub after: f64,
    pub count: Vec<f64>,
    pub speed: f64,
    pub size: f64,
    pub life: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BlastFeelModFEELFireChunks {
    pub count: Vec<f64>,
    pub speed: Vec<f64>,
    pub life: f64,
    pub size: f64,
    pub color: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct BlastFeelMod {
    /// (src/game/blastFeel.ts:47 FEEL)
    pub feel: crate::blast_feel::BlastFeelModFEEL,
    /// (src/game/blastFeel.ts:82 loaded)
    pub loaded: bool,
    /// (src/game/blastFeel.ts:92 STATE)
    pub state: JsMap<PlayerId, crate::blast_feel::State>,
}
impl BlastFeelMod {
    pub fn blank() -> Self {
        BlastFeelMod {
            feel: crate::blast_feel::BlastFeelModFEEL { on: true, kick: 1.0, kick_m: 0.45, kick_secs: 0.45, kick_tau: 0.14, kick_hz: 7.0, kick_reach: 28.0, sound: crate::blast_feel::BlastFeelModFEELSound { crack: 1.0, thump: 1.0, debris: 0.6, debris_after: 0.1, thump_after: 0.016666666666666666, carry: 0.8, carry_from: 3.0, carry_over: 7.0, c: 343.0, late_max: 0.25, dull_m: 14.0, ring: 0.3, ring_m: 7.0 }, ring: crate::blast_feel::BlastFeelModFEELRing { stages: 9.0, sectors: 13.0, arc: 1.4, r0: 0.5, step: 0.55, every: 0.025, y: 0.14, speed: 5.0, life: 0.5, size: 0.3, color: [0.74, 0.72, 0.68] }, push: alloc::vec![[0.0, 3.0, 7.0], [0.075, 5.5, 4.5], [0.15, 8.0, 3.0]], push_up: 0.5, ceil: crate::blast_feel::BlastFeelModFEELCeil { spots: 13.0, reach: 6.5, after: 0.1, speed: 14.0, puffs: 3.0, crumbs: 3.0, color: [0.82, 0.81, 0.78] }, fire: crate::blast_feel::BlastFeelModFEELFire { core: crate::blast_feel::BlastFeelModFEELFireCore { count: alloc::vec![30.0, 16.0], speed: alloc::vec![4.2, 3.0], life: 0.3, size: 0.11 }, roll: crate::blast_feel::BlastFeelModFEELFireRoll { count: alloc::vec![14.0, 8.0], speed: alloc::vec![2.2, 1.6], life: 0.5, size: 0.15, after: 2.0 }, soot: alloc::vec![crate::blast_feel::BlastFeelModFEELFireSootItem { after: 2.0, count: alloc::vec![12.0, 6.0], speed: 2.8, size: 0.3, life: 1.4 }, crate::blast_feel::BlastFeelModFEELFireSootItem { after: 5.0, count: alloc::vec![9.0, 5.0], speed: 2.2, size: 0.38, life: 1.7 }, crate::blast_feel::BlastFeelModFEELFireSootItem { after: 8.0, count: alloc::vec![8.0, 4.0], speed: 1.7, size: 0.44, life: 2.0 }, crate::blast_feel::BlastFeelModFEELFireSootItem { after: 13.0, count: alloc::vec![6.0, 3.0], speed: 1.1, size: 0.54, life: 2.6 }], soot_color: [0.05, 0.044, 0.038], chunks: crate::blast_feel::BlastFeelModFEELFireChunks { count: alloc::vec![28.0, 14.0], speed: alloc::vec![8.0, 5.5], life: 7.0, size: 0.05, color: [0.05, 0.046, 0.042] } } },
            loaded: false,
            state: JsMap::new(),
        }
    }
}

/// (src/game/blastFeel.ts:81 LS_KEY)
pub const LS_KEY: &str = "twopointsix.blastfeel";

/// (src/game/blastFeel.ts:97 FRAME)
pub const FRAME: f64 = 0.016666666666666666;
