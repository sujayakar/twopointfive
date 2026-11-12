//! The types of `friends`.
use crate::ids::*;

#[derive(Clone, Copy, Debug)]
pub enum SamOffFriend {
    Bool(bool),
    Guard(GuardId),
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FriendsStats {
    pub lowered: i32,
    pub lowered_s: f64,
}
