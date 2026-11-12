//! The types of `stature`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*};
use alloc::vec::Vec;

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct StatureMod {
    pub shown: JsMap<CharId, f64>,
    #[is(alloc::vec![None; 256])] pub by_id: Vec<Option<CharId>>,
}

/// how tall the men of a variant are; 1 for any other
pub fn variant_stature(variant: &str) -> f64 {
    match variant {
        "civilian_1" => 0.93,
        "defender_2" => 0.97,
        "defender_3" => 1.04,
        "operator_2" => 1.02,
        "operator_3" => 0.99,
        "operator_4" => 1.03,
        "operator_5" => 1.0,
        _ => 1.0,
    }
}

