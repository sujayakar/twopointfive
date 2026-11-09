//! The types of `wallwear`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct WallWear {
    pub on: bool,
    pub stats: crate::wallwear::WallWearStats,
    pub log: Vec<String>,
    pub cells: JsMap<crate::Num, Vec<(f64, f64, f64)>>,
    pub taken: f64,
}
impl WallWear {
    /// what a new one holds
    pub fn blank() -> Self {
        WallWear {
            on: true,
            stats: crate::wallwear::WallWearStats { rounds: 0.0, marks: 0.0, through: 0.0, grown: 0.0, chips: 0.0, puffs: 0.0 },
            log: Vec::new(),
            cells: JsMap::new(),
            taken: 0.0,
        }
    }
}
impl Default for WallWear { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallWearStats {
    pub rounds: f64,
    pub marks: f64,
    pub through: f64,
    pub grown: f64,
    pub chips: f64,
    pub puffs: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WearDecal {
    pub kind: crate::render_decals::DecalKind,
    pub pos: V3,
    pub n: V3,
    pub t: V3,
    pub radius: f64,
    pub param: f64,
    pub seed: f64,
    pub serial: f64,
    pub pane: f64,
    pub box_: Option<BoxId>,
    pub local: Option<crate::render_decals::DecalLocal>,
    pub look: Option<bool>,
    pub box_idx: f64,
    pub hidden: bool,
    pub wear_half: Option<f64>,
    pub wear_mat: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WEAR {
    pub curve: f64,
    pub vary: f64,
    pub exit_chips_from: f64,
    pub max_births: f64,
    pub max_cells: f64,
}

/// (src/game/wallwear.ts:32 WEAR: a table nothing writes)
pub fn wear() -> crate::wallwear::WEAR { crate::wallwear::WEAR { curve: 0.4, vary: 0.25, exit_chips_from: 0.3, max_births: 16.0, max_cells: 256.0 } }

/// (src/game/wallwear.ts:42 PALE: a table nothing writes)
pub fn pale() -> V3 { [0.86, 0.85, 0.82] }
