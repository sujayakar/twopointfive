//! The types of `stature`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct STATURETUNE {
    pub jitter: f64,
    pub min: f64,
    pub max: f64,
    pub ease_per_sec: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct StatureMod {
    pub shown: JsMap<CharId, f64>,
    #[is(alloc::vec![None; 256])] pub by_id: Vec<Option<CharId>>,
}

pub fn stature() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("civilian_1"), 0.93); t.set(String::from("defender_2"), 0.97); t.set(String::from("defender_3"), 1.04); t.set(String::from("operator_2"), 1.02); t.set(String::from("operator_3"), 0.99); t.set(String::from("operator_4"), 1.03); t.set(String::from("operator_5"), 1.0); t } }

pub fn stature_tune() -> crate::stature::STATURETUNE { crate::stature::STATURETUNE { jitter: 0.015, min: 0.92, max: 1.05, ease_per_sec: 0.25 } }

