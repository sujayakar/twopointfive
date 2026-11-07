//! The types of `wet_ground`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WetGround, src/game/wetGround.ts:92)
#[derive(Clone, Debug)]
pub struct WetGround {
    pub wetted: Vec<crate::wet_ground::WetGroundWettedItem>,
    pub stood: f64,
    pub roofs: Option<Vec<[f64; 4]>>,
    pub picked: bool,
    pub lot: Option<BoxId>,
}
impl WetGround {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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

/// (TypeScript: inline at src/game/wetGround.ts:94 WetGround.wetted, src/game/wetGround.ts:94)
#[derive(Clone, Copy, Debug)]
pub struct WetGroundWettedItem {
    pub box_: BoxId,
    pub flags: f64,
    pub rough: Option<f64>,
    pub aniso: Option<bool>,
    pub film: bool,
}
impl WetGroundWettedItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WetGroundWettedItem {
            box_: BoxId(0),
            flags: 0.0,
            rough: None,
            aniso: None,
            film: false,
        }
    }
}
impl Default for WetGroundWettedItem { fn default() -> Self { Self::zeroed() } }

/// (src/game/wetGround.ts:45 WET_OVER)
pub const WET_OVER: f64 = 0.02;

/// (src/game/wetGround.ts:48 FLAT_TOP)
pub const FLAT_TOP: f64 = 0.06;
