//! The types of `snow_marks`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct Seen {
    pub x: f64,
    pub z: f64,
    pub vy: f64,
    pub sp: f64,
    pub n: f64,
    pub rest: bool,
    pub mx: f64,
    pub mz: f64,
    pub marked: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SNOWMARKS {
    pub min_v: f64,
    pub step: f64,
    pub jump: f64,
    pub reach: f64,
    pub leg: f64,
    pub leg_min: f64,
    pub wide: f64,
    pub deep_kg: f64,
    pub mess: f64,
    pub clods: f64,
    pub force0: f64,
    pub force_kg: f64,
    pub force_v: f64,
    pub quick: f64,
    pub kick_every: f64,
    pub land_v: f64,
    pub roll: f64,
    pub roll_deep: f64,
    pub dent: f64,
    pub least: f64,
    pub most: f64,
    pub skid: f64,
    pub rest: f64,
    pub land_force: f64,
    pub land_force_v: f64,
    pub land_force_r: f64,
    pub item_mess: f64,
    pub dent_clods: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SnowMarksMod {
    /// (src/game/snowMarks.ts:63 propSeen)
    pub prop_seen: JsMap<PropId, Vec<f32>>,
    /// (src/game/snowMarks.ts:65 itemSeen)
    pub item_seen: JsMap<ItemId, crate::snow_marks::Seen>,
    /// (src/game/snowMarks.ts:67 chunkSeen)
    pub chunk_seen: JsMap<DebrisChunkId, crate::snow_marks::Seen>,
}
impl SnowMarksMod {
    pub fn blank() -> Self {
        SnowMarksMod {
            prop_seen: JsMap::new(),
            item_seen: JsMap::new(),
            chunk_seen: JsMap::new(),
        }
    }
}

/// (src/game/snowMarks.ts:28 SNOWMARKS: a table nothing writes)
pub fn snowmarks() -> crate::snow_marks::SNOWMARKS { crate::snow_marks::SNOWMARKS { min_v: 0.08, step: 0.05, jump: 0.7, reach: 0.1, leg: 0.07, leg_min: 0.028, wide: 0.3, deep_kg: 14.0, mess: 0.7, clods: 1.6, force0: 0.15, force_kg: 0.45, force_v: 0.2, quick: 0.35, kick_every: 0.14, land_v: 0.7, roll: 0.15, roll_deep: 0.35, dent: 1.5, least: 0.02, most: 0.4, skid: 0.6, rest: 1.15, land_force: 0.15, land_force_v: 0.13, land_force_r: 2.5, item_mess: 0.5, dent_clods: 4.0 } }
