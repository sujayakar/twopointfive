//! The types of `carlamps`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: LampSig, src/game/carlamps.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct LampSig {
    pub what: crate::carlamps::LampSigWhat,
    pub h: V3,
    pub albedo: Option<V3>,
    pub emissive: Option<V3>,
    pub gap: f64,
}
impl LampSig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LampSig {
            what: crate::carlamps::LampSigWhat::HeadLamp,
            h: [0.0; 3],
            albedo: None,
            emissive: None,
            gap: 0.0,
        }
    }
}
impl Default for LampSig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LampSigWhat {
    HeadLamp,
    TailLamp,
}
impl LampSigWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            LampSigWhat::HeadLamp => "head lamp",
            LampSigWhat::TailLamp => "tail lamp",
        }
    }
}

/// (TypeScript: FoundLamp, src/game/carlamps.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct FoundLamp {
    pub box_: f64,
    pub what: crate::carlamps::FoundLampWhat,
    pub lit: bool,
}
impl FoundLamp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FoundLamp {
            box_: 0.0,
            what: crate::carlamps::FoundLampWhat::HeadLamp,
            lit: false,
        }
    }
}
impl Default for FoundLamp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FoundLampWhat {
    HeadLamp,
    TailLamp,
}
impl FoundLampWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            FoundLampWhat::HeadLamp => "head lamp",
            FoundLampWhat::TailLamp => "tail lamp",
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CarlampsStats {
    pub found: i32,
    pub head: i32,
    pub tail: i32,
    pub lit: i32,
    pub shot: i32,
    pub painted: i32,
}

/// (src/game/carlamps.ts:39 VEHICLE_GROUP)
pub const VEHICLE_GROUP: &str = "vehicle";
