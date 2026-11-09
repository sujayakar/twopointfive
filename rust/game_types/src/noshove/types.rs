//! The types of `noshove`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct NoshoveStats {
    pub round: i32,
    pub short: i32,
    pub through: i32,
    pub back: i32,
    pub not_walked: i32,
}

