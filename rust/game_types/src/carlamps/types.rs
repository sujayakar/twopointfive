//! The types of `carlamps`.
#![allow(unused_imports)]
use crate::{Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct LampSig {
    pub what: crate::carlamps::CarLamp,
    pub h: V3,
    pub albedo: Option<V3>,
    pub emissive: Option<V3>,
    pub gap: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FoundLamp {
    pub box_: f64,
    pub what: crate::carlamps::CarLamp,
    pub lit: bool,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CarlampsStats {
    pub found: i32,
}

pub const VEHICLE_GROUP: &str = "vehicle";

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Words)]
pub enum CarLamp {
    #[word("head lamp")] #[default] HeadLamp,
    #[word("tail lamp")] TailLamp,
}
