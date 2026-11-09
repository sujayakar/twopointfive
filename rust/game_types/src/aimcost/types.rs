//! The types of `aimcost`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct RoundLeg {
    pub o: V3,
    pub d: V3,
    pub end: f64,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct AimcostStats {
    pub lamp_hits: i32,
    pub lamp_near: i32,
}
