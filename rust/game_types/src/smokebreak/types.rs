//! The types of `smokebreak`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SmokeBreakStats, src/game/smokebreak.ts:52)
#[derive(Clone, Copy, Debug)]
pub struct SmokeBreakStats {
    pub men: f64,
    pub lit: f64,
    pub draws: f64,
    pub flames: f64,
    pub embers: f64,
    pub breaths: f64,
    pub frames: f64,
}
impl SmokeBreakStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeBreakStats {
            men: 0.0,
            lit: 0.0,
            draws: 0.0,
            flames: 0.0,
            embers: 0.0,
            breaths: 0.0,
            frames: 0.0,
        }
    }
}
impl Default for SmokeBreakStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Man, src/game/smokebreak.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct Man {
    pub t: f64,
    pub flamed: bool,
    pub draws: f64,
    pub outs: f64,
    pub ember_t: f64,
    pub live: bool,
    /// INVENTED by a review: see the proposals
    pub out_at: Option<f64>,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            t: 0.0,
            flamed: false,
            draws: 0.0,
            outs: 0.0,
            ember_t: 0.0,
            live: false,
            out_at: None,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/smokebreak.ts:68)
#[derive(Clone, Debug)]
pub struct State {
    pub men: JsMap<CharId, crate::smokebreak::Man>,
    pub stats: crate::smokebreak::SmokeBreakStats,
    pub seed: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            men: JsMap::new(),
            stats: crate::smokebreak::SmokeBreakStats::zeroed(),
            seed: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/smokebreak.ts:82 handAt ->, src/game/smokebreak.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct HandAt {
    pub up: f64,
    pub drawn: bool,
    pub cycle: f64,
    pub into: f64,
}
impl HandAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandAt {
            up: 0.0,
            drawn: false,
            cycle: 0.0,
            into: 0.0,
        }
    }
}
impl Default for HandAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/smokebreak.ts:89 mouthOf ->, src/game/smokebreak.ts:89)
#[derive(Clone, Copy, Debug)]
pub struct MouthOf {
    pub pos: V3,
    pub dir: V3,
}
impl MouthOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MouthOf {
            pos: [0.0; 3],
            dir: [0.0; 3],
        }
    }
}
impl Default for MouthOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SmokebreakMod {
    /// (src/game/smokebreak.ts:69 states)
    pub states: Option<crate::smokebreak::State>,
}
impl SmokebreakMod {
    pub fn blank() -> Self {
        SmokebreakMod {
            states: None,
        }
    }
}
