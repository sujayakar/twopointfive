//! The types of `cleanrestart`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CleanrestartStats {
    pub restarts: i32,
}
