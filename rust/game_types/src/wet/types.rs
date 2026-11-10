//! The types of `wet`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WetMem {
    pub sky_t: f64,
    pub open: bool,
    pub prints_left: f64,
    pub last_out_wet_t: f64,
    pub drip_acc: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WetMod {
    pub mem: JsMap<CharId, crate::wet::WetMem>,
    pub drip_n: f64,
}

