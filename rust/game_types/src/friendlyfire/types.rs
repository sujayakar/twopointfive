//! The types of `friendlyfire`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum FriendInCone {
    Guard(GuardId),
    Player,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FriendlyfireStats {
    pub sam_on_mate: i32,
    pub mate_on_mate: i32,
    pub mate_on_sam: i32,
    pub unpressed: i32,
    pub held: i32,
    pub said: i32,
}
