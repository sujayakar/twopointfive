//! The types of `carlamps`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct LampSig {
    pub what: crate::carlamps::LampSigWhat,
    pub h: V3,
    pub albedo: Option<V3>,
    pub emissive: Option<V3>,
    pub gap: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum LampSigWhat {
    #[default] HeadLamp,
    TailLamp,
}
impl LampSigWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            LampSigWhat::HeadLamp => "head lamp",
            LampSigWhat::TailLamp => "tail lamp",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FoundLamp {
    pub box_: f64,
    pub what: crate::carlamps::FoundLampWhat,
    pub lit: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum FoundLampWhat {
    #[default] HeadLamp,
    TailLamp,
}
impl FoundLampWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            FoundLampWhat::HeadLamp => "head lamp",
            FoundLampWhat::TailLamp => "tail lamp",
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CarlampsStats {
    pub found: i32,
    pub head: i32,
    pub tail: i32,
    pub lit: i32,
    pub shot: i32,
    pub painted: i32,
}

/// (src/game/carlamps.ts:39 VEHICLE_GROUP)
pub const VEHICLE_GROUP: &str = "vehicle";
