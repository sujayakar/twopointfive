//! The types of `destruction_switch`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/destructionSwitch.ts:47 BLAST_PACE, src/game/destructionSwitch.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct DestructionSwitchModBLASTPACE {
    pub on: bool,
    pub reach_hold: f64,
    pub drip_samples: f64,
    pub drip_hold: f64,
}
impl DestructionSwitchModBLASTPACE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DestructionSwitchModBLASTPACE {
            on: false,
            reach_hold: 0.0,
            drip_samples: 0.0,
            drip_hold: 0.0,
        }
    }
}
impl Default for DestructionSwitchModBLASTPACE { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DestructionSwitchMod {
    /// (src/game/destructionSwitch.ts:47 BLAST_PACE)
    pub blast_pace: crate::destruction_switch::DestructionSwitchModBLASTPACE,
}
impl DestructionSwitchMod {
    pub fn blank() -> Self {
        DestructionSwitchMod {
            blast_pace: crate::destruction_switch::DestructionSwitchModBLASTPACE { on: true, reach_hold: 1.0, drip_samples: 16.0, drip_hold: 8.0 },
        }
    }
}

/// (src/game/destructionSwitch.ts:32 DRESS_GROUP)
pub const DRESS_GROUP: &str = "dressing";
