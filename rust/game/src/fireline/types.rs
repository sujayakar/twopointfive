//! The types of `fireline`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: St, src/game/fireline.ts:77)
#[derive(Clone, Debug)]
pub struct St {
    pub since: f64,
    pub t: f64,
    pub way: Option<Vec<V3>>,
    pub look_t: f64,
    pub go_t: f64,
    pub said: bool,
}
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            since: 0.0,
            t: 0.0,
            way: None,
            look_t: 0.0,
            go_t: 0.0,
            said: false,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Fired, src/game/fireline.ts:82)
#[derive(Clone, Debug)]
pub struct Fired {
    pub gun: String,
    pub shots: f64,
    pub round_t: f64,
    pub gap: f64,
    pub live_t: f64,
    pub rel_t: f64,
}
impl Fired {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Fired {
            gun: String::new(),
            shots: 0.0,
            round_t: 0.0,
            gap: 0.0,
            live_t: 0.0,
            rel_t: 0.0,
        }
    }
}
impl Default for Fired { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Lane, src/game/fireline.ts:103)
#[derive(Clone, Copy, Debug)]
pub struct Lane {
    pub m: V3,
    pub dx: f64,
    pub dz: f64,
    pub back: f64,
    pub k: f64,
    pub half: f64,
    pub energy: Option<f64>,
    pub thin: f64,
}
impl Lane {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Lane {
            m: [0.0; 3],
            dx: 0.0,
            dz: 0.0,
            back: 0.0,
            k: 0.0,
            half: 0.0,
            energy: None,
            thin: 0.0,
        }
    }
}
impl Default for Lane { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fireline.ts:166 hold ->, src/game/fireline.ts:166)
#[derive(Clone, Copy, Debug)]
pub struct Hold {
    pub speed: f64,
    /// INVENTED: dealt when a hold starts; a new hold of the same man counts as new
    pub serial: u32,
}
impl Hold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Hold {
            speed: 0.0,
            serial: 0,
        }
    }
}
impl Default for Hold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fireline.ts:171 fireLineFeet ->, src/game/fireline.ts:171)
#[derive(Clone, Copy, Debug)]
pub struct FireLineFeet {
    pub goal: Option<V3>,
    pub speed: f64,
}
impl FireLineFeet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireLineFeet {
            goal: None,
            speed: 0.0,
        }
    }
}
impl Default for FireLineFeet { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct FirelineMod {
    /// (src/game/fireline.ts:79 states)
    pub states: JsMap<GuardId, crate::fireline::St>,
    /// (src/game/fireline.ts:83 fired)
    pub fired: JsMap<PlayerId, crate::fireline::Fired>,
}
impl FirelineMod {
    pub fn blank() -> Self {
        FirelineMod {
            states: JsMap::new(),
            fired: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FirelineStats {
    pub holds: i32,
    pub held_s: i32,
    pub cues: i32,
    pub ways: i32,
    pub no_way: i32,
    pub fight: i32,
    pub enemy: i32,
}
