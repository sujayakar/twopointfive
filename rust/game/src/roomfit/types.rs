//! The types of `roomfit`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Fit, src/game/roomfit.ts:78)
#[derive(Clone, Debug)]
pub struct Fit {
    pub at: V3,
    pub pos: V3,
    pub watch: Option<V3>,
    pub face: f64,
    pub words: String,
    pub key: Option<String>,
    pub w: f64,
    pub front: f64,
    pub on: bool,
    pub next_t: f64,
    pub sig: String,
}
impl Fit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Fit {
            at: [0.0; 3],
            pos: [0.0; 3],
            watch: None,
            face: 0.0,
            words: String::new(),
            key: None,
            w: 0.0,
            front: 0.0,
            on: false,
            next_t: 0.0,
            sig: String::new(),
        }
    }
}
impl Default for Fit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Threat, src/game/roomfit.ts:79)
#[derive(Clone, Debug)]
pub struct Threat {
    pub at: V3,
    pub w: f64,
    pub way: Option<crate::roomread::RoomWay>,
    pub key: String,
    pub words: String,
}
impl Threat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Threat {
            at: [0.0; 3],
            w: 0.0,
            way: None,
            key: String::new(),
            words: String::new(),
        }
    }
}
impl Default for Threat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Watchable, src/game/roomfit.ts:130)
#[derive(Clone, Debug)]
pub struct Watchable {
    pub at: V3,
    pub w: f64,
    pub key: String,
    pub yaw: f64,
    pub dist: f64,
    pub way: bool,
}
impl Watchable {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Watchable {
            at: [0.0; 3],
            w: 0.0,
            key: String::new(),
            yaw: 0.0,
            dist: 0.0,
            way: false,
        }
    }
}
impl Default for Watchable { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct RoomfitStats {
    pub deals: i32,
    pub way: i32,
    pub sector: i32,
    pub plain: i32,
    pub moved: i32,
}

/// (src/game/roomfit.ts:81 EYE)
pub const EYE: f64 = 1.55;

/// (src/game/roomfit.ts:81 CHEST)
pub const CHEST: f64 = 1.1;

/// (src/game/roomfit.ts:81 DEG)
pub const DEG: f64 = 0.017453292519943295;

/// (src/game/roomfit.ts:83 COMPASS: a table nothing writes)
pub fn compass_table() -> Vec<String> { alloc::vec![String::from("south"), String::from("south-east"), String::from("east"), String::from("north-east"), String::from("north"), String::from("north-west"), String::from("west"), String::from("south-west")] }
