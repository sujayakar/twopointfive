//! The types of `stature`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Fit {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
    pub pad: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct STATURETUNE {
    pub jitter: f64,
    pub min: f64,
    pub max: f64,
    pub ease_per_sec: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StatureModFitsValue {
    pub of: crate::stature::Fit,
    pub s: f64,
    pub fit: crate::stature::Fit,
}

#[derive(Clone, Debug, Default)]
pub struct DrawnCapsulesP {
    pub caps: Vec<Vec<f64>>,
    pub bound: [f64; 4],
}

#[derive(Clone, Debug, Default)]
pub struct DrawnCapsules {
    pub caps: Vec<Vec<f64>>,
    pub bound: [f64; 4],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DrawnBeam {
    pub from: V3,
    pub dir: V3,
    pub k: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct StatureMod {
    /// (src/game/stature.ts:47 shown)
    pub shown: JsMap<CharId, f64>,
    /// (src/game/stature.ts:85 fits)
    pub fits: JsMap<CharId, crate::stature::StatureModFitsValue>,
    /// (src/game/stature.ts:103 byId)
    pub by_id: Vec<Option<CharId>>,
}
impl StatureMod {
    pub fn blank() -> Self {
        StatureMod {
            shown: JsMap::new(),
            fits: JsMap::new(),
            by_id: alloc::vec![None; 256],
        }
    }
}

/// (src/game/stature.ts:26 STATURE: a table nothing writes)
pub fn stature() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("civilian_1"), 0.93); t.set(String::from("defender_2"), 0.97); t.set(String::from("defender_3"), 1.04); t.set(String::from("operator_2"), 1.02); t.set(String::from("operator_3"), 0.99); t.set(String::from("operator_4"), 1.03); t.set(String::from("operator_5"), 1.0); t } }

/// (src/game/stature.ts:31 STATURE_TUNE: a table nothing writes)
pub fn stature_tune() -> crate::stature::STATURETUNE { crate::stature::STATURETUNE { jitter: 0.015, min: 0.92, max: 1.05, ease_per_sec: 0.25 } }

/// (src/game/stature.ts:65 palette: a table nothing writes)
pub fn palette() -> Vec<f32> { alloc::vec![0.0; 1024] }
