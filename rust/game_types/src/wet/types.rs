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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PrintRec {
    pub d: crate::render_decals::Decal,
    pub t0: f64,
    pub k0: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WetModPrintsValue {
    pub list: Vec<crate::wet::PrintRec>,
    pub age_t: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct WetMod {
    /// (src/game/wet.ts:53 mem)
    pub mem: JsMap<CharId, crate::wet::WetMem>,
    /// (src/game/wet.ts:57 prints)
    pub prints: Option<crate::wet::WetModPrintsValue>,
    /// (src/game/wet.ts:127 dripN)
    pub drip_n: f64,
}
impl WetMod {
    pub fn blank() -> Self {
        WetMod {
            mem: JsMap::new(),
            prints: None,
            drip_n: 0.0,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WetStats {
    pub sky_tests: i32,
    pub prints: i32,
    pub drips: i32,
}

