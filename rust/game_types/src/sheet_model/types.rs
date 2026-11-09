//! The types of `sheet_model`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct ImpulseResponse {
    pub gain: f64,
    pub up_k: f64,
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
pub struct IMPJITTER {
    pub mag: f64,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ImpulseKick {
    pub dv: V3,
    pub s: f64,
}

/// (src/game/sheetModel.ts:115 IMP_MIN_KICK)
pub const IMP_MIN_KICK: f64 = 0.03;

/// (src/game/sheetModel.ts:134 IMP_JITTER: a table nothing writes)
pub fn imp_jitter() -> crate::sheet_model::IMPJITTER { crate::sheet_model::IMPJITTER { mag: 0.6, side: 0.35 } }
