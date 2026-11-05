//! The types of `condition`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum Man {
    Player(PlayerId),
    Guard(GuardId),
}

/// (TypeScript: Condition, src/game/condition.ts:43)
#[derive(Clone, Copy, Debug)]
pub struct Condition {
    pub shaken: bool,
    pub blind: bool,
    pub folded: bool,
    pub hurt: bool,
    pub down: bool,
}
impl Condition {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Condition {
            shaken: false,
            blind: false,
            folded: false,
            hurt: false,
            down: false,
        }
    }
}
impl Default for Condition { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ConditionStats {
    pub hurt: i32,
}
