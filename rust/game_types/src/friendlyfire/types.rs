//! The types of `friendlyfire`.

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FriendlyfireStats {
    pub sam_on_mate: i32,
    pub unpressed: i32,
}
