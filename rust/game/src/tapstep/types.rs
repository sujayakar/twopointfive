//! The types of `tapstep`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: TapMove, src/game/tapstep.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct TapMove {
    pub mx: f64,
    pub mz: f64,
    pub speed: f64,
}
impl TapMove {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TapMove {
            mx: 0.0,
            mz: 0.0,
            speed: 0.0,
        }
    }
}
impl Default for TapMove { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: St, src/game/tapstep.ts:48)
#[derive(Clone, Copy, Debug)]
pub struct St {
    pub st: f64,
    pub t: f64,
    pub at: f64,
    pub keys: bool,
    pub dir: [f64; 2],
    pub v: f64,
    pub a: f64,
    pub go: f64,
    pub left: f64,
    pub age: f64,
    pub secs: f64,
    pub done: f64,
    pub len: f64,
    pub least: f64,
    pub rush: f64,
    pub u: f64,
    pub n: f64,
    pub way: f64,
    pub stuck: f64,
    pub prev: Option<V3>,
    pub asked: f64,
    pub man: Option<GuardId>,
    pub mask_t: f64,
    pub set: bool,
}
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            st: 0.0,
            t: 0.0,
            at: 0.0,
            keys: false,
            dir: [0.0; 2],
            v: 0.0,
            a: 0.0,
            go: 0.0,
            left: 0.0,
            age: 0.0,
            secs: 0.0,
            done: 0.0,
            len: 0.0,
            least: 0.0,
            rush: 0.0,
            u: 0.0,
            n: 0.0,
            way: 0.0,
            stuck: 0.0,
            prev: None,
            asked: 0.0,
            man: None,
            mask_t: 0.0,
            set: false,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tapstep.ts:61 backOf ->, src/game/tapstep.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct BackOf {
    pub gd: GuardId,
    pub x: f64,
    pub z: f64,
    pub d: f64,
}
impl BackOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BackOf {
            gd: GuardId(0),
            x: 0.0,
            z: 0.0,
            d: 0.0,
        }
    }
}
impl Default for BackOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tapstep.ts:69 spotOf ->, src/game/tapstep.ts:69)
#[derive(Clone, Copy, Debug)]
pub struct SpotOf {
    pub x: f64,
    pub z: f64,
}
impl SpotOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpotOf {
            x: 0.0,
            z: 0.0,
        }
    }
}
impl Default for SpotOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct TapstepMod {
    /// (src/game/tapstep.ts:51 STATE)
    pub state: JsMap<PlayerId, crate::tapstep::St>,
}
impl TapstepMod {
    pub fn blank() -> Self {
        TapstepMod {
            state: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct TapstepStats {
    pub steps: i32,
    pub scuffs: i32,
    pub beats: i32,
    pub masked: i32,
    pub lands: i32,
    pub last: String,
}
