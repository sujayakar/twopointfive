//! The types of `peeksight`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct PeeksightStats {
    pub frames: i32,
    pub short: i32,
}
