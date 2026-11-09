//! The types of `wet`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
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
#[derive(Clone, Debug, ToBytes)]
pub struct WetMod {
    /// (src/game/wet.ts:53 mem)
    pub mem: JsMap<CharId, crate::wet::WetMem>,
    /// (src/game/wet.ts:127 dripN)
    pub drip_n: f64,
}
impl WetMod {
    pub fn blank() -> Self {
        WetMod {
            mem: JsMap::new(),
            drip_n: 0.0,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WetStats {
    pub sky_tests: i32,
    pub drips: i32,
}

