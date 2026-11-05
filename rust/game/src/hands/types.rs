//! The types of `hands`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Haul, src/game/hands.ts:103)
#[derive(Clone, Copy, Debug)]
pub struct Haul {
    pub end: crate::hands::HaulEnd,
    pub phase: crate::hands::HaulPhase,
    pub t: f64,
    pub i_l: f64,
    pub i_r: f64,
    pub half: f64,
    pub from: [V3; 2],
    pub trunk: f64,
    pub carry: Option<crate::hands::HaulCarry>,
}
impl Haul {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Haul {
            end: crate::hands::HaulEnd::Head,
            phase: crate::hands::HaulPhase::Set,
            t: 0.0,
            i_l: 0.0,
            i_r: 0.0,
            half: 0.0,
            from: [[0.0; 3]; 2],
            trunk: 0.0,
            carry: None,
        }
    }
}
impl Default for Haul { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HaulEnd {
    Head,
    Feet,
}
impl HaulEnd {
    pub const fn as_str(self) -> &'static str {
        match self {
            HaulEnd::Head => "head",
            HaulEnd::Feet => "feet",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HaulPhase {
    Set,
    Reach,
    Rise,
    Carry,
}
impl HaulPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            HaulPhase::Set => "set",
            HaulPhase::Reach => "reach",
            HaulPhase::Rise => "rise",
            HaulPhase::Carry => "carry",
        }
    }
}

/// (TypeScript: inline at src/game/hands.ts:106 Haul.carry, src/game/hands.ts:106)
#[derive(Clone, Copy, Debug)]
pub struct HaulCarry {
    pub dim: [f64; 4],
    pub lie: crate::hands::Lie,
    pub to: Option<crate::hands::Lie>,
    pub yaw: f64,
    pub at: Option<[f64; 3]>,
    pub spot: Option<V3>,
}
impl HaulCarry {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HaulCarry {
            dim: [0.0; 4],
            lie: crate::hands::Lie::zeroed(),
            to: None,
            yaw: 0.0,
            at: None,
            spot: None,
        }
    }
}
impl Default for HaulCarry { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Lie, src/game/hands.ts:108)
#[derive(Clone, Copy, Debug)]
pub struct Lie {
    pub p: V3,
    pub u: V3,
    pub t: V3,
    pub kl: V3,
    pub kr: V3,
}
impl Lie {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Lie {
            p: [0.0; 3],
            u: [0.0; 3],
            t: [0.0; 3],
            kl: [0.0; 3],
            kr: [0.0; 3],
        }
    }
}
impl Default for Lie { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DoorMove, src/game/hands.ts:110)
#[derive(Clone, Copy, Debug)]
pub struct DoorMove {
    pub d: DoorId,
    pub open: bool,
    pub target: f64,
    pub t: f64,
    pub stage: crate::hands::DoorMoveStage,
    pub goal: Option<V3>,
    pub stall_t: f64,
    pub best: f64,
    pub step_best: f64,
    pub step_stall: f64,
    pub round: Option<bool>,
}
impl DoorMove {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorMove {
            d: DoorId(0),
            open: false,
            target: 0.0,
            t: 0.0,
            stage: crate::hands::DoorMoveStage::Hand,
            goal: None,
            stall_t: 0.0,
            best: 0.0,
            step_best: 0.0,
            step_stall: 0.0,
            round: None,
        }
    }
}
impl Default for DoorMove { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorMoveStage {
    Hand,
    Swing,
}
impl DoorMoveStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorMoveStage::Hand => "hand",
            DoorMoveStage::Swing => "swing",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EndDistEnd {
    Head,
    Feet,
}
impl EndDistEnd {
    pub const fn as_str(self) -> &'static str {
        match self {
            EndDistEnd::Head => "head",
            EndDistEnd::Feet => "feet",
        }
    }
}

/// (TypeScript: inline at src/game/hands.ts:189 carryRides(c), src/game/hands.ts:189)
#[derive(Clone, Copy, Debug)]
pub struct CarryRidesC {
    pub pos: V3,
    pub body_yaw: f64,
}
impl CarryRidesC {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CarryRidesC {
            pos: [0.0; 3],
            body_yaw: 0.0,
        }
    }
}
impl Default for CarryRidesC { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/hands.ts:212 layLie ->, src/game/hands.ts:212)
#[derive(Clone, Copy, Debug)]
pub struct LayLie {
    pub lie: crate::hands::Lie,
    pub yaw: f64,
}
impl LayLie {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LayLie {
            lie: crate::hands::Lie::zeroed(),
            yaw: 0.0,
        }
    }
}
impl Default for LayLie { fn default() -> Self { Self::zeroed() } }

/// (src/game/hands.ts:165 Y: a table nothing writes)
pub fn y() -> V3 { [0.0, 1.0, 0.0] }
