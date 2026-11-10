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

