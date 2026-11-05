//! The types of `doorblast`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SlabMet, src/game/doorblast.ts:160)
#[derive(Clone, Copy, Debug)]
pub struct SlabMet {
    pub landed: bool,
    pub lx: f64,
    pub lz: f64,
    pub rested: bool,
    pub nudged: f64,
    pub nudge_max: f64,
    pub laid: f64,
}
impl SlabMet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlabMet {
            landed: false,
            lx: 0.0,
            lz: 0.0,
            rested: false,
            nudged: 0.0,
            nudge_max: 0.0,
            laid: 0.0,
        }
    }
}
impl Default for SlabMet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SlabSnap, src/game/doorblast.ts:489)
#[derive(Clone, Copy, Debug)]
pub struct SlabSnap {
    pub x: f64,
    pub z: f64,
    pub a: f64,
    pub side: f64,
    pub who: f64,
    pub bits: Option<[f64; 4]>,
}
impl SlabSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlabSnap {
            x: 0.0,
            z: 0.0,
            a: 0.0,
            side: 0.0,
            who: 0.0,
            bits: None,
        }
    }
}
impl Default for SlabSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorblast.ts:123 hitAt ->, src/game/doorblast.ts:123)
#[derive(Clone, Copy, Debug)]
pub struct HitAt {
    pub t: f64,
    pub n: V3,
}
impl HitAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HitAt {
            t: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for HitAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorblast.ts:194 doorBlastReach ->, src/game/doorblast.ts:194)
#[derive(Clone, Copy, Debug)]
pub struct DoorBlastReachItem {
    pub d: DoorId,
    pub k: f64,
}
impl DoorBlastReachItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorBlastReachItem {
            d: DoorId(0),
            k: 0.0,
        }
    }
}
impl Default for DoorBlastReachItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorblast.ts:360 slabThere ->, src/game/doorblast.ts:360)
#[derive(Clone, Copy, Debug)]
pub struct SlabThere {
    pub but: f64,
    pub ask: f64,
}
impl SlabThere {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlabThere {
            but: 0.0,
            ask: 0.0,
        }
    }
}
impl Default for SlabThere { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorblast.ts:370 stepThere(W), src/game/doorblast.ts:370)
#[derive(Clone, Copy, Debug)]
pub struct StepThereW {
    pub but: f64,
    pub ask: f64,
}
impl StepThereW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StepThereW {
            but: 0.0,
            ask: 0.0,
        }
    }
}
impl Default for StepThereW { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DoorblastMod {
    /// (src/game/doorblast.ts:70 DOORBLAST_LOG)
    pub doorblast_log: Vec<String>,
}
impl DoorblastMod {
    pub fn blank() -> Self {
        DoorblastMod {
            doorblast_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DoorblastStats {
    pub doorblast_blasts: i32,
    pub doorblast_torn: i32,
    pub doorblast_landed: i32,
    pub doorblast_back: i32,
    pub doorblast_nudged: i32,
    pub doorblast_nudge_max: i32,
    pub doorblast_laid: i32,
    pub doorshot_rounds: i32,
    pub doorshot_splintered: i32,
    pub doorshot_chunks: i32,
    pub doorshot_demoted: i32,
}

/// (src/game/doorblast.ts:40 OFF_ANGLE)
pub const OFF_ANGLE: f64 = 1.5707963267948966;

/// (src/game/doorblast.ts:41 HALF_PI)
pub const HALF_PI: f64 = 1.5707963267948966;

/// (src/game/doorblast.ts:114 LIFT)
pub const LIFT: f64 = 0.005;

/// (src/game/doorblast.ts:133 OV: a table nothing writes)
pub fn ov() -> Vec<i32> { alloc::vec![0; 512] }
