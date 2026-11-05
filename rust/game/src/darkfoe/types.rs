//! The types of `darkfoe`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: FoeState, src/game/darkfoe.ts:76)
#[derive(Clone, Copy, Debug)]
pub struct FoeState {
    pub knows: bool,
    pub word: bool,
    pub blind: bool,
    pub ask: f64,
    pub lit: bool,
    pub against: f64,
    pub held: f64,
    pub blind_for: f64,
    pub moves: f64,
    pub since: f64,
    pub to: Option<V3>,
}
impl FoeState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FoeState {
            knows: false,
            word: false,
            blind: false,
            ask: 0.0,
            lit: false,
            against: 0.0,
            held: 0.0,
            blind_for: 0.0,
            moves: 0.0,
            since: 0.0,
            to: None,
        }
    }
}
impl Default for FoeState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/darkfoe.ts:162 darkFoeSpot ->, src/game/darkfoe.ts:162)
#[derive(Clone, Copy, Debug)]
pub struct DarkFoeSpot {
    pub spot: V3,
    pub mate: bool,
}
impl DarkFoeSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DarkFoeSpot {
            spot: [0.0; 3],
            mate: false,
        }
    }
}
impl Default for DarkFoeSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/darkfoe.ts:236 darkFoeOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct DarkFoeOf {
    pub knows: bool,
    pub word: bool,
    pub blind: bool,
    pub ask: f64,
    pub lit: bool,
    pub against: f64,
    pub held: f64,
    pub blind_for: f64,
    pub moves: f64,
    pub since: f64,
    pub to: Option<V3>,
}
impl DarkFoeOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DarkFoeOf {
            knows: false,
            word: false,
            blind: false,
            ask: 0.0,
            lit: false,
            against: 0.0,
            held: 0.0,
            blind_for: 0.0,
            moves: 0.0,
            since: 0.0,
            to: None,
        }
    }
}
impl Default for DarkFoeOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DarkfoeMod {
    /// (src/game/darkfoe.ts:77 stateOf)
    pub state_of: JsMap<GuardId, crate::darkfoe::FoeState>,
}
impl DarkfoeMod {
    pub fn blank() -> Self {
        DarkfoeMod {
            state_of: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DarkfoeStats {
    pub asks: i32,
    pub clicks_on: i32,
    pub clicks_off: i32,
    pub walks: i32,
    pub to_mate: i32,
    pub no_spot: i32,
}

/// (src/game/darkfoe.ts:160 WAY_TRIES)
pub const WAY_TRIES: f64 = 6.0;
