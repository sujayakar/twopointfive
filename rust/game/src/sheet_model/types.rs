//! The types of `sheet_model`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ImpulseResponse, src/game/sheetModel.ts:117)
#[derive(Clone, Copy, Debug)]
pub struct ImpulseResponse {
    pub gain: f64,
    pub up_k: f64,
    pub v_max: f64,
    pub wake: f64,
}
impl ImpulseResponse {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ImpulseResponse {
            gain: 0.0,
            up_k: 0.0,
            v_max: 0.0,
            wake: 0.0,
        }
    }
}
impl Default for ImpulseResponse { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ImpulseLike, src/game/sheetModel.ts:131)
#[derive(Clone, Copy, Debug)]
pub struct ImpulseLike {
    pub pos: V3,
    pub radius: f64,
    pub strength: f64,
    pub up: f64,
    pub bias: V3,
}
impl ImpulseLike {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ImpulseLike {
            pos: [0.0; 3],
            radius: 0.0,
            strength: 0.0,
            up: 0.0,
            bias: [0.0; 3],
        }
    }
}
impl Default for ImpulseLike { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sheetModel.ts:27 SHEET, src/game/sheetModel.ts:27)
#[derive(Clone, Copy, Debug)]
pub struct SHEET {
    pub gravity: f64,
    pub kt: f64,
    pub kn_lin: f64,
    pub kn_q: f64,
    pub sleep_v: f64,
    pub fade: f64,
    pub flut: f64,
    pub trans: f64,
    pub ang_damp: f64,
    pub spin_max: f64,
    pub life: f64,
}
impl SHEET {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SHEET {
            gravity: 0.0,
            kt: 0.0,
            kn_lin: 0.0,
            kn_q: 0.0,
            sleep_v: 0.0,
            fade: 0.0,
            flut: 0.0,
            trans: 0.0,
            ang_damp: 0.0,
            spin_max: 0.0,
            life: 0.0,
        }
    }
}
impl Default for SHEET { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sheetModel.ts:134 IMP_JITTER, src/game/sheetModel.ts:134)
#[derive(Clone, Copy, Debug)]
pub struct IMPJITTER {
    pub mag: f64,
    pub side: f64,
}
impl IMPJITTER {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IMPJITTER {
            mag: 0.0,
            side: 0.0,
        }
    }
}
impl Default for IMPJITTER { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sheetModel.ts:138 impulseKick ->, src/game/sheetModel.ts:138)
#[derive(Clone, Copy, Debug)]
pub struct ImpulseKick {
    pub dv: V3,
    pub s: f64,
}
impl ImpulseKick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ImpulseKick {
            dv: [0.0; 3],
            s: 0.0,
        }
    }
}
impl Default for ImpulseKick { fn default() -> Self { Self::zeroed() } }

/// (src/game/sheetModel.ts:27 SHEET: a table nothing writes)
pub fn sheet() -> crate::sheet_model::SHEET { crate::sheet_model::SHEET { gravity: 9.8, kt: 0.5, kn_lin: 1.5, kn_q: 9.0, sleep_v: 0.28, fade: 4.0, flut: 25.0, trans: 0.3, ang_damp: 1.8, spin_max: 3.0, life: 60.0 } }

/// (src/game/sheetModel.ts:115 IMP_MIN_KICK)
pub const IMP_MIN_KICK: f64 = 0.03;

/// (src/game/sheetModel.ts:134 IMP_JITTER: a table nothing writes)
pub fn imp_jitter() -> crate::sheet_model::IMPJITTER { crate::sheet_model::IMPJITTER { mag: 0.6, side: 0.35 } }
