//! The types of `smokebreak`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SmokeShown, src/game/smokebreak.ts:65)
#[derive(Clone, Debug)]
pub struct SmokeShown {
    pub men: JsMap<crate::Num, crate::smokebreak::SmokeShownMenValue>,
    pub frame: f64,
    pub threads: f64,
    pub records: f64,
}
impl SmokeShown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeShown {
            men: JsMap::new(),
            frame: 0.0,
            threads: 0.0,
            records: 0.0,
        }
    }
}
impl Default for SmokeShown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/smokebreak.ts:65 SmokeShown.men, src/game/smokebreak.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct SmokeShownMenValue {
    pub thread: bool,
    pub gen_: f64,
    pub beat: f64,
    pub left: f64,
    pub gap: f64,
}
impl SmokeShownMenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeShownMenValue {
            thread: false,
            gen_: 0.0,
            beat: 0.0,
            left: 0.0,
            gap: 0.0,
        }
    }
}
impl Default for SmokeShownMenValue { fn default() -> Self { Self::zeroed() } }

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

/// (TypeScript: inline at src/game/smokebreak.ts:67 smokeShown ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Debug)]
pub struct SmokeShown2 {
    pub men: JsMap<crate::Num, crate::smokebreak::SmokeShownMenValue>,
    pub frame: f64,
    pub threads: f64,
    pub records: f64,
}
impl SmokeShown2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeShown2 {
            men: JsMap::new(),
            frame: 0.0,
            threads: 0.0,
            records: 0.0,
        }
    }
}
impl Default for SmokeShown2 { fn default() -> Self { Self::zeroed() } }

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
