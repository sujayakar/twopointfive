//! The types of `wounds`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WoundZone {
    Head,
    Side,
    Leg,
    Centre,
}
impl WoundZone {
    pub const fn as_str(self) -> &'static str {
        match self {
            WoundZone::Head => "head",
            WoundZone::Side => "side",
            WoundZone::Leg => "leg",
            WoundZone::Centre => "centre",
        }
    }
}

/// (TypeScript: Wound, src/game/wounds.ts:30)
#[derive(Clone, Copy, Debug)]
pub struct Wound {
    pub zone: crate::wounds::WoundZone2,
    pub pose: crate::wounds::WoundPose,
    pub t: f64,
}
impl Wound {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Wound {
            zone: crate::wounds::WoundZone2::Side,
            pose: crate::wounds::WoundPose::Knee,
            t: 0.0,
        }
    }
}
impl Default for Wound { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WoundZone2 {
    Side,
    Leg,
}
impl WoundZone2 {
    pub const fn as_str(self) -> &'static str {
        match self {
            WoundZone2::Side => "side",
            WoundZone2::Leg => "leg",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WoundPose {
    Knee,
    Back,
}
impl WoundPose {
    pub const fn as_str(self) -> &'static str {
        match self {
            WoundPose::Knee => "knee",
            WoundPose::Back => "back",
        }
    }
}

/// (TypeScript: inline at src/game/wounds.ts:72 hitZone ->, src/game/wounds.ts:72)
#[derive(Clone, Copy, Debug)]
pub struct HitZone {
    pub zone: crate::wounds::WoundZone,
    pub head: f64,
    pub trunk: f64,
    pub leg: f64,
}
impl HitZone {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HitZone {
            zone: crate::wounds::WoundZone::Head,
            head: 0.0,
            trunk: 0.0,
            leg: 0.0,
        }
    }
}
impl Default for HitZone { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoundDoes {
    Shot,
    None,
    Side,
    Leg,
}
impl RoundDoes {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoundDoes::Shot => "shot",
            RoundDoes::None => "none",
            RoundDoes::Side => "side",
            RoundDoes::Leg => "leg",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WoundDownZone {
    Side,
    Leg,
}
impl WoundDownZone {
    pub const fn as_str(self) -> &'static str {
        match self {
            WoundDownZone::Side => "side",
            WoundDownZone::Leg => "leg",
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WoundsStats {
    pub dead: i32,
    pub wounded: i32,
    pub by_zone: [i32; 4],
    pub again: i32,
    pub finished: i32,
}
