//! The types of `doorthrow`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DoorthrowStats {
    pub startled: i32,
    pub pushed: i32,
    pub kicked: i32,
    pub floored: i32,
    pub carried: i32,
}
