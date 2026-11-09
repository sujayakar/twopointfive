//! The types of `samstun`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SamstunStats {
    pub staggered: i32,
    pub dazzled: i32,
}
