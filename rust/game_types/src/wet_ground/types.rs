//! The types of `wet_ground`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug)]
pub struct WetGround {
    pub wetted: Vec<crate::wet_ground::WetGroundWettedItem>,
    pub stood: f64,
    pub roofs: Option<Vec<[f64; 4]>>,
    pub picked: bool,
    pub lot: Option<BoxId>,
}
impl WetGround {
    /// what a new one holds
    pub fn blank() -> Self {
        WetGround {
            wetted: Vec::new(),
            stood: -1.0,
            roofs: None,
            picked: false,
            lot: None,
        }
    }
}
impl Default for WetGround { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct WetGroundWettedItem {
    pub box_: BoxId,
    pub flags: f64,
    pub rough: Option<f64>,
    pub aniso: Option<bool>,
    pub film: bool,
}

/// (src/game/wetGround.ts:45 WET_OVER)
pub const WET_OVER: f64 = 0.02;

/// (src/game/wetGround.ts:48 FLAT_TOP)
pub const FLAT_TOP: f64 = 0.06;
