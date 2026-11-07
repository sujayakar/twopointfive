//! The types of `wallwear`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WallWear, src/game/wallwear.ts:44)
#[derive(Clone, Debug)]
pub struct WallWear {
    pub on: bool,
    pub stats: crate::wallwear::WallWearStats,
    pub log: Vec<String>,
    pub cells: JsMap<crate::Num, Vec<(f64, f64, f64)>>,
    pub taken: f64,
}
impl WallWear {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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

/// (TypeScript: inline at src/game/wallwear.ts:47 WallWear.stats, src/game/wallwear.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct WallWearStats {
    pub rounds: f64,
    pub marks: f64,
    pub through: f64,
    pub grown: f64,
    pub chips: f64,
    pub puffs: f64,
}
impl WallWearStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallWearStats {
            rounds: 0.0,
            marks: 0.0,
            through: 0.0,
            grown: 0.0,
            chips: 0.0,
            puffs: 0.0,
        }
    }
}
impl Default for WallWearStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WearDecal, src/game/wallwear.ts:131)
#[derive(Clone, Copy, Debug)]
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
impl WearDecal {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WearDecal {
            kind: crate::render_decals::DecalKind::Bullet,
            pos: [0.0; 3],
            n: [0.0; 3],
            t: [0.0; 3],
            radius: 0.0,
            param: 0.0,
            seed: 0.0,
            serial: 0.0,
            pane: 0.0,
            box_: None,
            local: None,
            look: None,
            box_idx: 0.0,
            hidden: false,
            wear_half: None,
            wear_mat: None,
        }
    }
}
impl Default for WearDecal { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallwear.ts:32 WEAR, src/game/wallwear.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct WEAR {
    pub curve: f64,
    pub vary: f64,
    pub exit_chips_from: f64,
    pub max_births: f64,
    pub max_cells: f64,
}
impl WEAR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WEAR {
            curve: 0.0,
            vary: 0.0,
            exit_chips_from: 0.0,
            max_births: 0.0,
            max_cells: 0.0,
        }
    }
}
impl Default for WEAR { fn default() -> Self { Self::zeroed() } }

/// (src/game/wallwear.ts:32 WEAR: a table nothing writes)
pub fn wear() -> crate::wallwear::WEAR { crate::wallwear::WEAR { curve: 0.4, vary: 0.25, exit_chips_from: 0.3, max_births: 16.0, max_cells: 256.0 } }

/// (src/game/wallwear.ts:42 PALE: a table nothing writes)
pub fn pale() -> V3 { [0.86, 0.85, 0.82] }
