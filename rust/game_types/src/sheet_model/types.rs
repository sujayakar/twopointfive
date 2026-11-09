//! The types of `sheet_model`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct ImpulseResponse {
    pub gain: f64,
    pub up_k: f64,
    pub v_max: f64,
    pub wake: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ImpulseLike {
    pub pos: V3,
    pub radius: f64,
    pub strength: f64,
    pub up: f64,
    pub bias: V3,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct IMPJITTER {
    pub mag: f64,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ImpulseKick {
    pub dv: V3,
    pub s: f64,
}

/// (src/game/sheetModel.ts:27 SHEET: a table nothing writes)
pub fn sheet() -> crate::sheet_model::SHEET { crate::sheet_model::SHEET { gravity: 9.8, kt: 0.5, kn_lin: 1.5, kn_q: 9.0, sleep_v: 0.28, fade: 4.0, flut: 25.0, trans: 0.3, ang_damp: 1.8, spin_max: 3.0, life: 60.0 } }

/// (src/game/sheetModel.ts:115 IMP_MIN_KICK)
pub const IMP_MIN_KICK: f64 = 0.03;

/// (src/game/sheetModel.ts:134 IMP_JITTER: a table nothing writes)
pub fn imp_jitter() -> crate::sheet_model::IMPJITTER { crate::sheet_model::IMPJITTER { mag: 0.6, side: 0.35 } }
