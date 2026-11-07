//! The types of `cleanrestart`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CleanrestartStats {
    pub restarts: i32,
    pub relit: i32,
    pub lines: i32,
    pub cuts: i32,
    pub pictures: i32,
}
