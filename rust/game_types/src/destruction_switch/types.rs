//! The types of `destruction_switch`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DestructionSwitchModBLASTPACE {
    pub on: bool,
    pub reach_hold: f64,
    pub drip_samples: f64,
    pub drip_hold: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
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
