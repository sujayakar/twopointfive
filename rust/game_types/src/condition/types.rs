//! The types of `condition`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Hash)]
pub enum Man {
    Player(PlayerId),
    Guard(GuardId),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Condition {
    pub shaken: bool,
    pub blind: bool,
    pub folded: bool,
    pub hurt: bool,
    pub down: bool,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ConditionStats {
    pub hurt: i32,
}
