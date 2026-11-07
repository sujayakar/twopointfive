//! The types of `takedown`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Half {
    Blow,
    Kill,
    Partner,
    Victim,
    Fall,
    Breathe,
    Lower,
    ChokeBeats,
    Haul,
}
impl Half {
    pub const fn as_str(self) -> &'static str {
        match self {
            Half::Blow => "blow",
            Half::Kill => "kill",
            Half::Partner => "partner",
            Half::Victim => "victim",
            Half::Fall => "fall",
            Half::Breathe => "breathe",
            Half::Lower => "lower",
            Half::ChokeBeats => "chokeBeats",
            Half::Haul => "haul",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TakeFallHow {
    Choke,
    Whip,
    Kill,
}
impl TakeFallHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            TakeFallHow::Choke => "choke",
            TakeFallHow::Whip => "whip",
            TakeFallHow::Kill => "kill",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct TakedownMod {
    /// (src/game/takedown.ts:162 BREATHING)
    pub breathing: bool,
}
impl TakedownMod {
    pub fn blank() -> Self {
        TakedownMod {
            breathing: false,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct TakedownStats {
    pub blows: i32,
    pub falls: i32,
    pub partner: i32,
    pub breathing: i32,
}

/// (src/game/takedown.ts:53 PARTNER_SECS)
pub const PARTNER_SECS: f64 = 0.6;

/// (src/game/takedown.ts:55 SAM_HIT)
pub const SAM_HIT: f64 = 0.28;

/// (src/game/takedown.ts:114 LAY_REACH)
pub const LAY_REACH: f64 = 0.95;

/// (src/game/takedown.ts:114 LAY_HALF)
pub const LAY_HALF: f64 = 0.3;
