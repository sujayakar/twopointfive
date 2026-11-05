//! The types of `friends`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum SamOffFriend {
    Bool(bool),
    Guard(GuardId),
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FriendsStats {
    pub lowered: i32,
    pub lowered_s: i32,
}
